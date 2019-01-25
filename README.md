# Word count plugin for xi-editor

## Installation

tldr; `make install`.

To install this plugin, the plugin manifest must be placed in a new directory under
$XI_CONFIG_DIR/plugins, where $XI_CONFIG_DIR is the path passed by your client
to xi core via the `client_started` RPC's `config_dir` field, on startup.
On MacOS, by default, $XI_CONFIG_DIR is located at ~/Library/Application Support/XiEditor.

Additionally, the compiled binary should be placed in a `bin/` subdir of the
directory containing the manifest. (This is the default; the location can be
changed in the manifest.)



