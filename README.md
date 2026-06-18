# A perl extension for Zed

## Installation

Rust must be installed via [rustup](https://rustup.rs) for dev extensions to work.

Clone the repository, then open Zed's extensions page and click "Install Dev Extension". Select the cloned directory and Zed will build and load the extension automatically.

Currently, this is a work in progress.

The grammar is set up with our [tree-sitter parser](https://github.com/tree-sitter-perl/tree-sitter-perl); queries are constantly being improved.

## Using Perl Navigator

Until the next release of perlnavigator, the way you can use it is by installing either
from `npm` or from github releases, and having it available on your `$PATH` for zed to
find.

If you would like to pass settings in, you can base yourself off the following snippet.
See perlnavigator for configuration options.
```json
{
  ...
  "lsp": {
    "perlnavigator-server": {
      "settings": {
        "perlnavigator": {
          "includePaths": [
            "local/lib/perl5",
            "lib"
          ]
        }
      }
    }
  }
}
```

## Using perl-lsp

[`perl-lsp`](https://github.com/tree-sitter-perl/perl-tree-sitter-lsp) is an
opt-in alternative language server built on this grammar. It stays dormant
unless you opt in, so existing Perl Navigator users are unaffected.

First, make the binary available in one of two ways:

- **Install it yourself** so it's on your `$PATH` (recommended):
  ```sh
  cargo install perl-lsp
  ```
- **Let the extension download a prebuilt binary** by enabling the `download`
  setting (binaries are published for Linux x86_64, macOS Intel/Apple Silicon,
  and Windows x86_64):
  ```json
  {
    "lsp": {
      "perl-lsp": {
        "settings": { "download": true }
      }
    }
  }
  ```

Then select it over Perl Navigator for the Perl language:
```json
{
  "languages": {
    "Perl": {
      "language_servers": ["perl-lsp", "!perlnavigator-server", "..."]
    }
  }
}
```

## Text objects

In Zed's vim mode, the extension provides these tree-sitter text objects:

| Object | Around (`a`) | Inside (`i`) | Matches |
| ------ | ------------ | ------------ | ------- |
| function | `af` | `if` | named/anonymous `sub`s and `method`s |
| class | `ac` | `ic` | block-form `package` and `class` declarations |
| comment | `gc` | — | comments (adjacent line comments group together) |
