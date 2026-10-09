# collect-settings

Build, encode and decode the configuration QR codes of
[ODK Collect](https://docs.getodk.org/collect-import-export/) and
KoboCollect, in Rust.

A configuration code is `base64(zlib(json))` where the JSON has three
sections: `general` (server, account, form management, interface), `admin`
(admin password and access-control switches) and `project` (name, icon,
colour). This crate gives you a typed model of every key the current app
declares, two presets that match how survey teams usually lock a device, a
codec that round-trips the official Python example, and an SVG renderer.

```rust
use collect_settings::{Settings, General, Admin, Project};

let settings = Settings {
    general: General::auto_sync("https://kc.kobotoolbox.org", "interviewer01", "secret"),
    admin: Admin::locked_interviewer("admin-pw"),
    project: Some(Project::new("Household survey").icon('H').color("#6b63e6")),
};

let payload = settings.encode()?;          // string to put in the QR code
let svg = settings.qr_svg(4)?;             // scalable SVG, 4-module quiet zone
for w in settings.warnings() { eprintln!("warning: {w}"); }

let decoded = Settings::decode(&payload)?; // read a code back
# Ok::<(), collect_settings::Error>(())
```

## Presets

* `General::auto_sync(server, user, password)` — the device mirrors the
  server (`match_exactly`), checks every 15 minutes, downloads updates on its
  own, sends finalized forms over any connection and keeps the local copy.
* `Admin::locked_interviewer(admin_pw)` — every user setting, the server, the
  QR scanner, form download and deletion are hidden behind the admin
  password; sending and viewing sent forms stay visible.
* `Admin::open(admin_pw)` — everything allowed.

Both are starting points: `Admin::set("moving_backwards", false)` and the
public fields let you tighten or loosen anything.

## KoboToolbox

Point `server_url` at the **KoboCAT** host (`https://kc.kobotoolbox.org`,
`https://kc-eu.kobotoolbox.org`, or `https://kc.your-server`), not at the
`kf.`/`eu.` web interface. `Settings::warnings()` flags that mistake.

## Features

* `qr` (default) — `Settings::qr_svg` and `Settings::qr_code` via the
  [`qrcode`](https://crates.io/crates/qrcode) crate. Disable it for a
  dependency-free codec.

## Minimum supported Rust version

1.75.

## License

MIT.

## Contributing

Issues and pull requests are welcome at
[github.com/Prompt-Consultoria/collect-settings](https://github.com/Prompt-Consultoria/collect-settings).
Please run `cargo fmt`, `cargo clippy --all-targets --all-features` and
`cargo test --all-features` before opening a PR.
