use crate::Settings;
use base64::Engine;
use flate2::{read::ZlibDecoder, write::ZlibEncoder, Compression};
use std::io::{Read, Write};

/// Errors from encoding, decoding or rendering.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The settings could not be (de)serialised as JSON.
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    /// The payload is not valid base64.
    #[error("base64: {0}")]
    Base64(#[from] base64::DecodeError),
    /// The payload could not be zlib-(de)compressed.
    #[error("zlib: {0}")]
    Zlib(#[from] std::io::Error),
    /// The decompressed payload is not UTF-8.
    #[error("payload is not utf-8")]
    Utf8(#[from] std::string::FromUtf8Error),
    /// The payload is too large for any QR code version.
    #[cfg(feature = "qr")]
    #[error("qr: {0}")]
    Qr(#[from] qrcode::types::QrError),
}

/// Compress and encode an already serialised JSON document.
pub fn encode_json(json: &str) -> Result<String, Error> {
    let mut z = ZlibEncoder::new(Vec::new(), Compression::best());
    z.write_all(json.as_bytes())?;
    let bytes = z.finish()?;
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

/// Serialise, compress and encode: the exact string a QR code must carry.
pub fn encode(settings: &Settings) -> Result<String, Error> {
    encode_json(&settings.to_json()?)
}

/// Decode a scanned payload back into [`Settings`].
///
/// Whitespace is ignored and both standard and URL-safe base64 alphabets are
/// accepted, because scanners and chat apps mangle long strings.
pub fn decode(payload: &str) -> Result<Settings, Error> {
    let cleaned: String = payload.chars().filter(|c| !c.is_whitespace()).collect();
    let std = base64::engine::general_purpose::STANDARD;
    let bytes = match std.decode(&cleaned) {
        Ok(b) => b,
        Err(e) => base64::engine::general_purpose::URL_SAFE
            .decode(&cleaned)
            .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(&cleaned))
            .map_err(|_| e)?,
    };
    let mut json = String::new();
    ZlibDecoder::new(bytes.as_slice()).read_to_string(&mut json)?;
    Ok(serde_json::from_str(&json)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Produced by the Python example in the ODK docs:
    /// `b64encode(zlib.compress(json.dumps(settings).encode()))`.
    #[test]
    fn decodes_python_payload() {
        let s = Settings::decode("eJwVzEsKgDAMRdG9vHHReTcjqcYq/UTSVgXp3tXh5cB94DmzUoR90AprpsSwIBh8dbJOTT/EVutR7DiGeQjipIpEJ/cg6tENaEl7/hdJzj37ydEcLtKlwK4UC/f+AqVRJcs=").unwrap();
        assert_eq!(s.general.username.as_deref(), Some("a"));
        assert_eq!(s.admin.moving_backwards, Some(false));
    }
}
