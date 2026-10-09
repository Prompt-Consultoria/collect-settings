//! WebAssembly bindings for `collect-settings` — the npm package `collect-settings`.
//!
//! ```js
//! import init, { render, decode, lockedInterviewerAdmin, autoSyncGeneral } from 'collect-settings';
//! await init();
//! const settings = {
//!   general: autoSyncGeneral('https://kc.kobotoolbox.org', 'interviewer01', 'secret'),
//!   admin: lockedInterviewerAdmin('admin-pw'),
//!   project: { name: 'Household survey', icon: 'H', color: '#6b63e6' },
//! };
//! const { svg, payload, modules, warnings } = render(settings, 4);
//! const back = decode(payload); // the settings object again
//! ```
//!
//! Settings are plain JS objects with the same shape as the crate's
//! [`Settings`](https://docs.rs/collect-settings): `general`, `admin` and
//! `project` sections whose keys are the ones ODK Collect reads.

use collect_settings::{Admin, General, Settings};
use serde::Serialize;
use wasm_bindgen::prelude::*;

fn erro(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&e.to_string())
}

/// JS object → `Settings`, going through JSON so Maps/objects/undefined all
/// behave the way `JSON.stringify` would.
fn ler(settings: &JsValue) -> Result<Settings, JsValue> {
    let json = js_sys::JSON::stringify(settings)
        .map_err(|_| JsValue::from_str("settings must be a JSON-serialisable object"))?;
    let json: String = json.into();
    if json == "undefined" || json == "null" {
        return Err(JsValue::from_str("settings is missing"));
    }
    serde_json::from_str(&json).map_err(|e| erro(format!("invalid settings: {e}")))
}

/// `Serialize` → plain JS object (objects, not Maps; no `undefined` keys).
fn escrever<T: Serialize>(v: &T) -> Result<JsValue, JsValue> {
    v.serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(erro)
}

/// Encode a settings object as the string a configuration QR code must
/// contain: `base64(zlib(json))`.
#[wasm_bindgen]
pub fn encode(settings: JsValue) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    ler(&settings)?.encode().map_err(erro)
}

/// Decode the string read from a configuration QR code back into a settings
/// object. Whitespace is ignored; standard and URL-safe base64 are accepted.
#[wasm_bindgen]
pub fn decode(payload: &str) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    escrever(&Settings::decode(payload).map_err(erro)?)
}

/// Human-readable warnings about combinations that usually surprise people
/// in the field (e.g. the KoboToolbox web host instead of the KoboCAT host,
/// locks without an admin password). Returns `string[]`.
#[wasm_bindgen]
pub fn warnings(settings: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    escrever(&ler(&settings)?.warnings())
}

/// Everything a UI needs in one call.
#[derive(Serialize)]
pub struct Rendered {
    /// The string inside the QR code.
    pub payload: String,
    /// Scalable SVG (`viewBox` only) with the quiet zone included.
    pub svg: String,
    /// Symbol width in modules, without the quiet zone.
    pub modules: usize,
    /// Pretty-printed JSON of the settings, for review.
    pub json: String,
    /// Bytes of the compact JSON before compression.
    pub json_bytes: usize,
    /// See [`warnings`].
    pub warnings: Vec<String>,
}

/// Render a settings object as a QR code. `quiet_zone` is in modules
/// (default 4). Returns `{ payload, svg, modules, json, json_bytes, warnings }`.
#[wasm_bindgen]
pub fn render(settings: JsValue, quiet_zone: Option<u32>) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let s = ler(&settings)?;
    let code = s
        .qr_code(collect_settings::EcLevel::M)
        .map_err(|e| erro(format!("settings too large for a QR code: {e}")))?;
    let opt = collect_settings::QrOptions {
        quiet_zone: quiet_zone.unwrap_or(4).min(16),
        ..Default::default()
    };
    let r = Rendered {
        payload: s.encode().map_err(erro)?,
        svg: collect_settings::render_svg(&code, opt),
        modules: code.width(),
        json: s.to_json_pretty().map_err(erro)?,
        json_bytes: s.to_json().map(|j| j.len()).map_err(erro)?,
        warnings: s.warnings(),
    };
    escrever(&r)
}

/// `admin` section for an interviewer-only device: every user setting, the
/// server, the QR scanner, form download and deletion are hidden behind
/// `admin_pw`; sending and viewing sent forms stay visible.
#[wasm_bindgen(js_name = lockedInterviewerAdmin)]
pub fn locked_interviewer_admin(admin_pw: &str) -> Result<JsValue, JsValue> {
    escrever(&Admin::locked_interviewer(admin_pw))
}

/// `admin` section with everything allowed and the given password.
#[wasm_bindgen(js_name = openAdmin)]
pub fn open_admin(admin_pw: &str) -> Result<JsValue, JsValue> {
    escrever(&Admin::open(admin_pw))
}

/// `general` section that mirrors the server (`match_exactly`), checks every
/// 15 minutes, downloads updates on its own, sends finalized forms over any
/// connection and keeps the local copy after sending.
#[wasm_bindgen(js_name = autoSyncGeneral)]
pub fn auto_sync_general(
    server_url: &str,
    username: &str,
    password: &str,
) -> Result<JsValue, JsValue> {
    escrever(&General::auto_sync(server_url, username, password))
}

/// Names of every boolean switch of the `admin` section, in the order ODK
/// Collect declares them. Returns `string[]`.
#[wasm_bindgen(js_name = adminSwitches)]
pub fn admin_switches() -> Result<JsValue, JsValue> {
    escrever(&Admin::SWITCHES)
}

/// Version of the underlying `collect-settings` crate.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
