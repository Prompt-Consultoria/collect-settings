# Changelog

## 0.1.3 — 2026-10-10

- CI: release workflow renamed to `release.yml` to match the Trusted Publishing
  configuration on crates.io and npm; no library changes.

## 0.1.2 — 2026-10-09

- Version bump only: the npm version 0.1.1 was consumed by an interrupted
  staged publish and cannot be reused.

## 0.1.1 — 2026-10-09

- Fix: the "looks like the KoboToolbox web interface" warning fired for the
  KoboCAT EU host `kc-eu.kobotoolbox.org`; the check now looks at the exact
  host (`kf.*` or `eu.kobotoolbox.org`).
- npm package `collect-settings` (WebAssembly build, `bindings/js`).

## 0.1.0 — 2026-10-09

- First release: typed model, codec, SVG renderer, presets and warnings.
