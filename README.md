# rusty-keys
Linux, mac, and windows rust keylogger

mac & windows rely on the [rdev](https://docs.rs/rdev/latest/rdev/) crate.
Linux uses the `/dev/input/eventX` device to read console and GUI keystrokes regardless of window manager.

_Keymappings in Linux need to be applied since /dev/input reads raw key values_
