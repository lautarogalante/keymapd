# keymapd

Simple tool to map evdev events to commands.

## What it does

keymapd listens for input events from evdev devices (like keyboards) and executes commands in response. Currently supports volume control via media keys using wpctl (PipeWire).

## Building
```bash
cd keymapd
cargo build
```
## Move the binary
```bash
sudo cp /keymapd/target/debug/keymapd /usr/local/bin

```


## Running as a service

Create the file `~/.config/systemd/user/keymapd.service`:

```ini
[Unit]
Description=keymapd - evdev event mapper
After=graphical-session.target

[Service]
ExecStart=/path/to/keymapd
Restart=on-failure
RestartSec=5

[Install]
WantedBy=default.target
```

Then enable and start the service:

```bash
systemctl --user daemon-reload
systemctl --user enable keymapd.service
systemctl --user start keymapd.service
```

## Notes

Your user needs access to `/dev/input/` devices. Add yourself to the `input` group:

```bash
sudo usermod -aG input $USER
```

Logout and login again for changes to take effect.
