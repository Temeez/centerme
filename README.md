# centerme

Centers a window on the primary monitor. Windows only for now. (PR's always welcome.)

## Usage
`centerme.exe --title "Untitled - Notepad"`

This will center your empty notepad window (Windows 10) on your primary monitor when it is active.

```
Options:
  -t, --title <TITLE>  Title of the window to center when it is active (foreground)
  -d, --delay <DELAY>  How long to wait in milliseconds before centering the window. Some windows may change in size shortly after appearing so this delay will allow more flexibility
  -p, --print          Print the title of the currently active (foreground) window after 1 second
  -h, --help           Print help
  -V, --version        Print version

```