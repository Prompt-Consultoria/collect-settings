#!/usr/bin/env bash
# Build the npm package into bindings/js/pkg/ and fix its metadata.
# Publish with: cd pkg && npm publish --access public
set -euo pipefail
cd "$(dirname "$0")"

wasm-pack build --target web --release --out-dir pkg -- --config 'profile.release.opt-level="z"'

cp ../../LICENSE pkg/LICENSE

# wasm-pack names the package after the crate (collect-settings-wasm); the
# npm package is published as plain `collect-settings`.
node - <<'JS'
const fs = require('fs');
const p = JSON.parse(fs.readFileSync('pkg/package.json', 'utf8'));
p.name = 'collect-settings';
p.description = 'ODK Collect / KoboCollect configuration QR codes: build, encode, decode and render (WebAssembly)';
p.repository = { type: 'git', url: 'git+https://github.com/Prompt-Consultoria/collect-settings.git', directory: 'bindings/js' };
p.homepage = 'https://github.com/Prompt-Consultoria/collect-settings';
p.keywords = ['odk', 'odk-collect', 'kobotoolbox', 'kobocollect', 'qrcode', 'survey', 'wasm', 'webassembly'];
p.license = 'MIT';
p.author = { name: 'Prompt Consultoria' };
if (Array.isArray(p.files) && !p.files.includes('LICENSE')) p.files.push('LICENSE');
fs.writeFileSync('pkg/package.json', JSON.stringify(p, null, 2) + '\n');
console.log(`pkg ready: ${p.name}@${p.version}`);
JS
