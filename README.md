# PowerShell extension for Zed editor

## Setting up language server

Currently, the extension didn't support automatic installation (it'll soon land). You need
to set the bundle path through `lsp` configuration:

```json
{
  "lsp": {
    "powershell-es": {
      "binary": {
        "path": "<path to PowerShellEditorServices>"
      }
    }
  }
}
