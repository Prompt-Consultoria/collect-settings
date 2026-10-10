# collect-settings

Rust crate (`collect-settings` on crates.io) + WebAssembly npm package
(`collect-settings` on npm, built from `bindings/js` with wasm-pack). Encodes,
decodes and renders ODK Collect / KoboCollect configuration QR codes.

## Layout

- `src/` — the crate. `cargo test --all-features` and `--no-default-features`
  both have to pass; CI also runs `cargo fmt --check`, clippy with
  `-D warnings` and `cargo doc` with `RUSTDOCFLAGS=-D warnings`.
- `bindings/js/` — wasm crate `collect-settings-wasm` (path dependency on
  `../..`). `build.sh` runs wasm-pack into `bindings/js/pkg/` and rewrites
  `pkg/package.json` (name → `collect-settings`, license, repo). `pkg/` is
  generated, not versioned. `test.mjs` smoke-tests the built package.

## Release process

A push of a tag `v*` runs `.github/workflows/release.yml`, which publishes
to crates.io and npm with Trusted Publishing (OIDC, no stored tokens).

1. Bump the version in **four** places: `Cargo.toml`, `bindings/js/Cargo.toml`
   (both the `version` and the `collect-settings = { path = "../..",
   version = ... }` line), and `Cargo.lock` (`cargo update -w`). Add a
   `CHANGELOG.md` entry.
2. The `check versions` job fails if the two Cargo.toml versions diverge or
   the tag isn't `v<version>`.
3. Commit, `git tag -a vX.Y.Z`, `git push origin main vX.Y.Z`.
4. npm takes a minute or two to show the new version in the packument after
   the job prints `+ collect-settings@X.Y.Z`; crates.io is immediate.

Things learned the hard way (2026-10-09/10):

- Both registries match the trusted publisher on the **workflow filename**
  (`release.yml`) and the GitHub environment (`release`). Renaming the file
  breaks publishing until both registry configs are updated. The crates.io
  config can be read with the local cargo token:
  `GET https://crates.io/api/v1/trusted_publishing/github_configs?crate=collect-settings`.
  npm has no API for it; check on the package settings page.
- A version consumed by a failed or interrupted publish can never be reused
  on npm (0.1.1 and 0.1.2 were burned this way). Bump again instead.
- `npm publish --dry-run` is rejected when the committed version already
  exists on npm, which fails every commit between a release and the next
  bump. CI and the `workflow_dispatch` dry run therefore use
  `npm pack --dry-run`, which validates the tarball without the registry.
- `workflow_dispatch` on `release.yml` exercises the builds and dry runs but
  **not** OIDC; the only real test of the publishing path is a tag push.
- The npm job needs npm ≥ 11.5.1 (installed in the workflow) for OIDC.

## CI

`ci.yml` runs on push to `main` and on PRs. `actions/checkout` and
`actions/setup-node` are on v7. Both workflows were verified on the
`ubuntu-26.04` image (2026-10-10) before the `ubuntu-latest` migration;
nothing in them is distro-specific, so keep `ubuntu-latest`.
