# Automatic download

You can automatically download and patch RoPro within the command-line easily, or within the program itself.

## How it works

- Downloads the RoPro source, directly from the Chrome web store
- Extracts the extension to a folder called `RoPro`
- Patches the extension within the `RoPro` folder with the proxy at position `1` within [proxies](../proxies.txt)
  - `1` can be replaced with any index (when using command-line)
  - Indexes start at `0`
  - Indexes represent which line to use
  
## Command-line

`ropro-patcher.exe 1`
- `1` can be any index as explained [here](#automatic-download)

## GUI

Select `Download and Patch (uses default proxy)`
- Uses index `1` as explained [here](#automatic-download)