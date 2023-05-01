
# Lyr - a TUI display manager

## This project is a WIP
 - [X] Rendering everything
 - [ ] Pam auth working

## Dependencies
 - rust
 - tui (0.19.0)
 - pam
 - dbus 
 - shutdown

## Configuration
You can find all the configuration in `/etc/lyr/config.toml`.
The file is commented, and includes the default values.

## Controls
Use the up and down arrow keys to change the current field, and the
left and right arrow keys to change the target desktop environment
while on the desktop field (above the login field).

## Tips
The numlock and capslock state is printed in the top-right corner.
Use the F1 and F2 keys to respectively shutdown and reboot.

## PSX DOOM fire animation
Don't get your hopes up
