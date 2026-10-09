use crate::{Error, Settings};
use qrcode::{Color, EcLevel, QrCode};
use std::fmt::Write;

/// Rendering options for [`Settings::qr_svg_with`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QrOptions {
    /// Error-correction level. `M` (15 %) is what the ODK docs example uses
    /// and survives a laminated, slightly reflective card.
    pub ec_level: EcLevel,
    /// Quiet zone around the symbol, in modules. The QR spec asks for 4.
    pub quiet_zone: u32,
    /// Dark colour (CSS).
    pub dark: &'static str,
    /// Light colour (CSS).
    pub light: &'static str,
}

impl Default for QrOptions {
    fn default() -> Self {
        QrOptions {
            ec_level: EcLevel::M,
            quiet_zone: 4,
            dark: "#000000",
            light: "#ffffff",
        }
    }
}

impl Settings {
    /// Build the QR code symbol for these settings.
    pub fn qr_code(&self, ec_level: EcLevel) -> Result<QrCode, Error> {
        Ok(QrCode::with_error_correction_level(
            self.encode()?.as_bytes(),
            ec_level,
        )?)
    }

    /// Render the configuration as an SVG string with the given quiet zone
    /// and default colours / error correction.
    pub fn qr_svg(&self, quiet_zone: u32) -> Result<String, Error> {
        self.qr_svg_with(QrOptions {
            quiet_zone,
            ..QrOptions::default()
        })
    }

    /// Render the configuration as an SVG string.
    ///
    /// The SVG has no fixed pixel size (`viewBox` only), so it scales to any
    /// print or screen size, and all dark modules share one `<path>` so the
    /// file stays small.
    pub fn qr_svg_with(&self, opt: QrOptions) -> Result<String, Error> {
        Ok(render_svg(&self.qr_code(opt.ec_level)?, opt))
    }
}

/// Render any [`QrCode`] with the given options.
pub fn render_svg(code: &QrCode, opt: QrOptions) -> String {
    let n = code.width();
    let total = n as u32 + 2 * opt.quiet_zone;
    let colors = code.to_colors();
    let mut path = String::with_capacity(colors.len() * 6);
    for (i, c) in colors.iter().enumerate() {
        if *c == Color::Dark {
            let (x, y) = (i % n, i / n);
            let _ = write!(
                path,
                "M{}{}h1v1h-1z",
                x as u32 + opt.quiet_zone,
                fmt_y(y as u32 + opt.quiet_zone)
            );
        }
    }
    format!(
        concat!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {t} {t}\" ",
            "shape-rendering=\"crispEdges\" role=\"img\" aria-label=\"Collect settings QR code\">",
            "<rect width=\"{t}\" height=\"{t}\" fill=\"{light}\"/>",
            "<path d=\"{path}\" fill=\"{dark}\"/></svg>"
        ),
        t = total,
        light = opt.light,
        dark = opt.dark,
        path = path
    )
}

/// SVG path syntax needs a separator between two numbers: ` 12` after `M7`.
fn fmt_y(y: u32) -> String {
    format!(" {y}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Admin, General};

    #[test]
    fn renders_scalable_svg() {
        let s = Settings {
            general: General {
                server_url: Some("https://kc.kobotoolbox.org".into()),
                ..Default::default()
            },
            admin: Admin::locked_interviewer("pw"),
            project: None,
        };
        let svg = s.qr_svg(4).unwrap();
        assert!(svg.starts_with("<svg xmlns"));
        assert!(svg.contains("viewBox=\"0 0 "));
        assert!(
            svg.contains("M4 4h1v1h-1z"),
            "finder pattern starts at the quiet zone offset"
        );
    }
}
