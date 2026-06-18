use std::env;
use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const SERVER_PATH: &str = "node_modules/.bin/perlnavigator";
const PACKAGE_NAME: &str = "perlnavigator-server";

const PERL_LSP_SERVER_ID: &str = "perl-lsp";
const PERL_LSP_REPO: &str = "tree-sitter-perl/perl-tree-sitter-lsp";

struct PerlExtension {
    did_find_server: bool,
    perl_lsp_path: Option<String>,
}

impl PerlExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();

        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }

    fn perlnavigator_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // prefer a user-installed perlnavigator on PATH
        if let Some(path) = worktree.which("perlnavigator") {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        let server_path = self.server_script_path(language_server_id)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    /// Resolve the `perl-lsp` binary. This server is opt-in: it stays dormant
    /// (returns an error) unless the user has either installed `perl-lsp` on
    /// their PATH or enabled the managed download via the `download` setting.
    fn perl_lsp_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = self.perl_lsp_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }

    fn perl_lsp_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // 1. prefer a user-installed perl-lsp on PATH (e.g. `cargo install perl-lsp`)
        if let Some(path) = worktree.which(PERL_LSP_SERVER_ID) {
            return Ok(path);
        }

        // 2. otherwise only fetch a prebuilt binary if the user opted in
        let download_opt_in = LspSettings::for_worktree(PERL_LSP_SERVER_ID, worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .and_then(|value| value.get("download").and_then(|d| d.as_bool()))
            .unwrap_or(false);

        if !download_opt_in {
            return Err(concat!(
                "perl-lsp is opt-in: it will not be downloaded or started unless you opt in, ",
                "so you can ignore this if you didn't ask for it.\n",
                "To use perl-lsp, either install it with `cargo install perl-lsp` so it's on your PATH, ",
                "or let the extension fetch a prebuilt binary by adding to your settings:\n",
                "  \"lsp\": { \"perl-lsp\": { \"settings\": { \"download\": true } } }\n",
                "To hide this message, disable perl-lsp for Perl in your settings:\n",
                "  \"languages\": { \"Perl\": { \"language_servers\": [\"!perl-lsp\", \"...\"] } }",
            )
            .to_string());
        }

        self.download_perl_lsp(language_server_id)
    }

    fn download_perl_lsp(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        if let Some(path) = &self.perl_lsp_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            PERL_LSP_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();
        let target = match (os, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "aarch64-apple-darwin",
            (zed::Os::Mac, zed::Architecture::X8664) => "x86_64-apple-darwin",
            (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-unknown-linux-gnu",
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => {
                return Err(format!(
                    "perl-lsp has no prebuilt binary for {os:?}/{arch:?}; install it with `cargo install perl-lsp` so it's on your PATH"
                ))
            }
        };

        let (archive_ext, file_type, bin_name) = match os {
            zed::Os::Windows => ("zip", zed::DownloadedFileType::Zip, "perl-lsp.exe"),
            _ => ("tar.gz", zed::DownloadedFileType::GzipTar, "perl-lsp"),
        };

        let asset_name = format!("perl-lsp-{target}.{archive_ext}");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "no asset named `{asset_name}` in perl-lsp release {}",
                    release.version
                )
            })?;

        let version_dir = format!("perl-lsp-{}", release.version);
        let binary_path = format!("{version_dir}/{bin_name}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|err| format!("failed to download perl-lsp: {err}"))?;
            zed::make_file_executable(&binary_path)?;

            // clean up any previously downloaded versions
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with("perl-lsp-") && name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }

        self.perl_lsp_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for PerlExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
            perl_lsp_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            PERL_LSP_SERVER_ID => self.perl_lsp_command(language_server_id, worktree),
            _ => self.perlnavigator_command(language_server_id, worktree),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(PerlExtension);
