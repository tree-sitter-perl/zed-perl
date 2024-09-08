use std::path::PathBuf;
use std::{env, fs};

use zed_extension_api::{self as zed, Result};
const SERVER_PATH: &str = "node_modules/.bin/perlnavigator";
const PACKAGE_NAME: &str = "perlnavigator-server";

struct PerlExtension {
    did_find_server: bool,
}

fn copy_dir_all(src: impl Into<PathBuf>, dst: impl Into<PathBuf>) -> std::io::Result<()> {
    let src = src.into();
    let dst = dst.into();

    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

// NOTE - this is very super duper evil
fn move_oniguruma_super_evil() -> std::io::Result<()> {
    let top_level_node_modules = PathBuf::from("node_modules");
    let nested_oniguruma = top_level_node_modules
        .join(PACKAGE_NAME)
        .join("node_modules")
        .join("vscode-oniguruma");
    let onig_dir = top_level_node_modules.join("vscode-oniguruma");

    copy_dir_all(onig_dir, nested_oniguruma)?;

    Ok(())
}

impl PerlExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
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
                    // TODO - don't be evil;
                    // Create the symlink for node_modules
                    let _ignored = move_oniguruma_super_evil();
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
}

impl zed::Extension for PerlExtension {
    fn new() -> Self {
        Self {
            did_find_server: true,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
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
}

zed::register_extension!(PerlExtension);
