# collect-settings (npm)

Build, encode, decode and render the configuration QR codes of
[ODK Collect](https://docs.getodk.org/collect-import-export/) and KoboCollect,
in the browser or in Node. This is the
[`collect-settings`](https://crates.io/crates/collect-settings) Rust crate
compiled to WebAssembly (~270 kB, gzips to ~110 kB).

```js
import init, { render, decode, lockedInterviewerAdmin, autoSyncGeneral } from 'collect-settings';
await init();

const settings = {
  general: autoSyncGeneral('https://kc.kobotoolbox.org', 'interviewer01', 'secret'),
  admin: lockedInterviewerAdmin('admin-pw'),
  project: { name: 'Household survey', icon: 'H', color: '#6b63e6' },
};
const { svg, payload, modules, warnings } = render(settings, 4);
document.querySelector('#qr').innerHTML = svg;   // scalable SVG
const back = decode(payload);                      // settings object again
```

## API

| Function | What it does |
|---|---|
| `render(settings, quietZone?)` | `{ payload, svg, modules, json, json_bytes, warnings }` |
| `encode(settings)` | the QR string: `base64(zlib(json))` |
| `decode(payload)` | QR string → settings object |
| `warnings(settings)` | `string[]` of known field traps |
| `autoSyncGeneral(serverUrl, username, password)` | `general` preset: mirror the server, auto-update, auto-send |
| `lockedInterviewerAdmin(adminPw)` | `admin` preset: everything locked except send / view sent |
| `openAdmin(adminPw)` | `admin` preset: everything allowed |
| `adminSwitches()` | names of the 39 admin switches |
| `version()` | crate version |

`settings` is a plain object `{ general, admin, project? }` with the key
names ODK Collect reads; see the crate docs at
[docs.rs/collect-settings](https://docs.rs/collect-settings) for every key,
its allowed values and the presets.

For KoboToolbox point `server_url` at the KoboCAT host (`kc.kobotoolbox.org`,
`kc-eu.kobotoolbox.org`), never at the `kf.`/`eu.` web interface;
`warnings` flags that mistake.

## Node

```js
import init, { render } from 'collect-settings';
import { readFileSync } from 'node:fs';
await init({ module_or_path: readFileSync('node_modules/collect-settings/collect_settings_wasm_bg.wasm') });
```

## License

MIT.
