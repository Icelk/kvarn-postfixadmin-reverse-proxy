This is a simple configuration for using Kvarn as a reverse proxy for [PostfixAdmin](https://wiki.archlinux.org/title/PostfixAdmin).

Build the sources using:

```shell
$ cargo b --release
```

Then, run it as root.

# Systemd unit

```ini
[Unit]
Description=Kvarn PostfixAdmin proxy

[Service]
Type=exec
ExecStart=<path to the executable>

[Install]
WantedBy=default.target
```
