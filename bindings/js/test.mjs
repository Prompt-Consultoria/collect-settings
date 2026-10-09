// Smoke test of the built package (run after build.sh): node bindings/js/test.mjs
import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { gunzipSync, inflateSync } from 'node:zlib';
import init, { render, encode, decode, warnings, lockedInterviewerAdmin, autoSyncGeneral, openAdmin, adminSwitches, version } from './pkg/collect_settings_wasm.js';

await init({ module_or_path: readFileSync(new URL('./pkg/collect_settings_wasm_bg.wasm', import.meta.url)) });

const settings = {
  general: autoSyncGeneral('https://kc.kobotoolbox.org', 'interviewer01', 'secret'),
  admin: lockedInterviewerAdmin('admin-pw'),
  project: { name: 'Household survey', icon: 'H', color: '#6b63e6' },
};
const r = render(settings, 4);
assert.ok(r.svg.startsWith('<svg'), 'svg');
assert.ok(r.modules > 21, 'modules');
assert.equal(r.payload, encode(settings), 'payload == encode');
assert.deepEqual(r.warnings, [], 'no warnings for the presets');

// The payload is zlib(json) in base64 — check with Node's own zlib.
const json = JSON.parse(inflateSync(Buffer.from(r.payload, 'base64')).toString());
assert.equal(json.general.username, 'interviewer01');
assert.equal(json.admin.qr_code_scanner, false);
assert.deepEqual(decode(r.payload), json, 'decode == zlib roundtrip');

assert.ok(warnings({ general: { server_url: 'https://kf.kobotoolbox.org' }, admin: {} })[0].includes('KoboCAT'));
assert.equal(openAdmin('x').moving_backwards, true);
assert.ok(adminSwitches().includes('bulk_finalize'));
assert.throws(() => decode('not a qr'), /not a|base64|zlib/i);
assert.throws(() => render({ general: { font_size: 20 }, admin: {} }), /font_size/);
assert.match(version(), /^\d+\.\d+\.\d+$/);
console.log(`collect-settings wasm ${version()} — OK (${r.modules}×${r.modules} modules, ${r.payload.length} chars)`);
void gunzipSync;
