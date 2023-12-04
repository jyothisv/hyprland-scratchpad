# Hyprland Scratchpad

XMonad-like scratchpads for the Hyprland window manager.

## How to use

1) Build the project. The binary named `hyprland_scratchpad` would be created.
2) To run a command in a scratchpad, use the following syntax:

```shell
    hyprland_scratchpad workspace_name command args ...
```
3) This will invoke the command in the special workspace named `workspace_name`, and the resulting window(s) will be moved to the current workspace.
4) Invoking the command again will toggle the special workspace
