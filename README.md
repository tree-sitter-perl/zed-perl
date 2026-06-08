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
