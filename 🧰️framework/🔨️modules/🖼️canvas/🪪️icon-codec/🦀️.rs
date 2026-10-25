//! 🪪️ Canvas icon identity and codec: the structured `Icon` union and its canonical wire string, the metabolism icon names, and the resolver that turns an encoded icon into paintable SVG or raster content through the compiler's installed fonts and the installed icon pack.
#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]

/// 📐️ First-party intrinsic-dimension reader (`🧰️framework/🔨️modules/📏️intrinsic-size`) — the `wasm32-wasip2` arm of
/// `icon_codec::decode_raster_icon_bytes` uses it instead of `image` so that dimension-only call site stops pulling the
/// raster pipeline onto the guest component. Native/browser keep the real `image` decode unchanged. Ticket
/// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`, `🔍️research/📓️intrinsic-size-wiring.md`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_intrinsic_size as intrinsic_size;

// #region 🔖️IconCodec
#[path = "../../🖼️assets/🌱️metabolism/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs"]
mod metabolism_icon_name_gen;

pub use semio_framework_icon_contract::IconName;
pub use metabolism_icon_name_gen::MetabolismIconName;

// 🌉️ Hand-written `ToValue`/`FromValue` for `MetabolismIconName` — see that file's own
// header docstring for why it lives here rather than as `#[derive(...)]` on the machine-generated
// sources themselves.
include!("../🌉️icon-name-value/🦀️.rs");

pub mod icon_codec {
    // #region icon_codec
    //! 🖼️ Generic icon encoding resolver for board nodes (url, shortcode, math, emoji, raster, inline SVG, catalog, themed, text).

    pub use super::{IconName, MetabolismIconName};

    use semio_framework_canvas_assets::{compiler_fonts, icon_pack};
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;

    mod icon_shortcodes {
        include!("../../🖼️assets/🔣️icons/🤖️generated/🔤️shortcodes/🦀️.rs");
    }

    /// 🔖️ What one shortcode body names: an emoji glyph from the compiled table, or an SVG source from the installed icon pack.
    enum ShortcodeResolved {
        Emoji(&'static str),
        SvgPlain(&'static str),
        SvgThemed(&'static str),
    }

    /// 🔍️ Resolves a shortcode body against the compiled emoji table first, then the installed pack's themed and plain SVG;
    /// `None` when the code is unknown or no host has installed the icon pack.
    fn icon_shortcode_resolve(code: &str) -> Option<ShortcodeResolved> {
        let code = code.trim();
        if let Some(glyph) = icon_shortcodes::icon_shortcode_emoji(code) {
            return Some(ShortcodeResolved::Emoji(glyph));
        }
        let pack = icon_pack()?;
        pack.themed(code).map(ShortcodeResolved::SvgThemed).or_else(|| pack.plain(code).map(ShortcodeResolved::SvgPlain))
    }

    /// 🔎️ Optional lookup for domain-themed SVG icons (e.g. puzzle metabolism table).
    pub type ThemedSvgLookup = fn(&str) -> Option<&'static str>;

    // #region 🏷️IconUnion

    /// 🟢️ Canonical structured icon payload shared across canvases and UI chrome.
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    #[value(tag = "kind", rename_all = "camelCase")]
    pub enum Icon {
        Url { url: String },
        Shortcode { code: String },
        Data { data: String },
        Emoji { emoji: String },
        Math { src: String },
        Text { text: String },
        Svg { svg: String },
        Catalog { key: IconName },
        Themed { key: MetabolismIconName },
    }

    fn is_raster_data_url_payload(s: &str) -> bool {
        let u = s.trim().to_ascii_lowercase();
        u.starts_with("data:image/png;base64,") || u.starts_with("data:image/jpeg;base64,") || u.starts_with("data:image/jpg;base64,") || u.starts_with("data:image/webp;base64,") || u.starts_with("data:image/gif;base64,")
    }

    fn is_svg_data_url_payload(s: &str) -> bool {
        s.trim().to_ascii_lowercase().starts_with("data:image/svg+xml")
    }

    fn looks_like_shortcode_token(t: &str) -> bool {
        t.len() >= 3 && t.starts_with(':') && t.ends_with(':') && t[1..t.len() - 1].chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '+' | '-'))
    }

    fn shortcode_inner(t: &str) -> Option<&str> {
        if !looks_like_shortcode_token(t) {
            return None;
        }
        Some(&t[1..t.len() - 1])
    }

    fn looks_like_ascii_catalogish_stem(s: &str) -> bool {
        let t = s.trim();
        if t.is_empty() || !t.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-')) {
            return false;
        }
        matches!(t.chars().next(), Some(c) if c.is_ascii_alphabetic() || c == '_') && (t.contains('.') || t.contains('_') || t.contains('-') || t.len() > 48)
    }

    fn is_extended_pictographic_char(c: char) -> bool {
        let cp = c as u32;
        matches!(cp, 0x1F1E6..=0x1F1FF | 0x1F300..=0x1FAFF | 0x2600..=0x27BF | 0x2300..=0x23FF) || matches!(c, '©' | '®' | '™' | '☺' | '☻' | '♥' | '♦' | '♣' | '♠' | '✓' | '✔' | '✕' | '✖' | '✗' | '✘') || c == '\u{FE0F}' || c == '\u{200D}'
    }

    fn looks_like_bare_emoji(s: &str) -> bool {
        let t = s.trim();
        !t.is_empty() && t.chars().any(is_extended_pictographic_char)
    }

    fn looks_like_bare_url(s: &str) -> bool {
        let lower = s.trim().to_ascii_lowercase();
        lower.starts_with("http://") || lower.starts_with("https://")
    }

    fn resolve_inline_svg(encoded: &str) -> Option<String> {
        let t = encoded.trim();
        if t.is_empty() {
            return None;
        }
        let lower = t.to_ascii_lowercase();
        if lower.starts_with("<?xml") || lower.contains("<svg") {
            return Some(t.to_string());
        }
        None
    }

    /// 🔤️ Decodes a canonical icon string into a structured {@link Icon}.
    ///
    /// `$formula$` decode sugar — generic math-notation culture, not a Typst leftover (the icon
    /// selector's own placeholder already teaches it). Unlike Typst's own `$...$` inline-math
    /// marker, the compiler's grammar has no delimiter syntax of its own, so the `$`s must be
    /// STRIPPED here rather than passed through verbatim.
    pub fn decode_icon(encoded: &str) -> Option<Icon> {
        let t = encoded.trim();
        if t.is_empty() {
            return None;
        }
        if let Some(url) = t.strip_prefix("url:") {
            let url = url.trim();
            return (!url.is_empty()).then(|| Icon::Url { url: url.to_string() });
        }
        if looks_like_bare_url(t) {
            return Some(Icon::Url { url: t.to_string() });
        }
        if let Some(code) = shortcode_inner(t) {
            return Some(Icon::Shortcode { code: code.to_string() });
        }
        if let Some(src) = t.strip_prefix("math:") {
            let src = src.trim();
            return (!src.is_empty()).then(|| Icon::Math { src: src.to_string() });
        }
        if t.len() >= 2 && t.starts_with('$') && t.ends_with('$') {
            let inner = t[1..t.len() - 1].trim();
            return (!inner.is_empty()).then(|| Icon::Math { src: inner.to_string() });
        }
        if let Some(em) = t.strip_prefix("emoji:") {
            let em = em.trim();
            return (!em.is_empty()).then(|| Icon::Emoji { emoji: em.to_string() });
        }
        if let Some(text) = t.strip_prefix("text:") {
            let text = text.trim();
            return (!text.is_empty()).then(|| Icon::Text { text: text.to_string() });
        }
        if is_raster_data_url_payload(t) || is_svg_data_url_payload(t) || t.to_ascii_lowercase().starts_with("data:") {
            return Some(Icon::Data { data: t.to_string() });
        }
        if let Some(svg) = resolve_inline_svg(t) {
            return Some(Icon::Svg { svg });
        }
        if let Some(key) = MetabolismIconName::parse(t) {
            return Some(Icon::Themed { key });
        }
        if let Some(key) = IconName::from_str(t) {
            return Some(Icon::Catalog { key });
        }
        if looks_like_ascii_catalogish_stem(t) {
            return None;
        }
        if looks_like_bare_emoji(t) {
            return Some(Icon::Emoji { emoji: t.to_string() });
        }
        if t.chars().count() <= 16 {
            return Some(Icon::Text { text: t.to_string() });
        }
        None
    }

    /// 📡️ Encodes a structured {@link Icon} into the canonical wire string.
    ///
    /// Always canonical, even for a `src` that was decoded from `$…$` sugar — the `$`
    /// delimiters are input sugar only, never round-tripped back out.
    pub fn encode_icon(icon: &Icon) -> String {
        match icon {
            Icon::Url { url } => format!("url:{}", url.trim()),
            Icon::Shortcode { code } => format!(":{code}:"),
            Icon::Data { data } => data.trim().to_string(),
            Icon::Emoji { emoji } => format!("emoji:{}", emoji.trim()),
            Icon::Math { src } => format!("math:{}", src.trim()),
            Icon::Text { text } => format!("text:{}", text.trim()),
            Icon::Svg { svg } => svg.trim().to_string(),
            Icon::Catalog { key } => key.as_str().to_string(),
            Icon::Themed { key } => key.as_str().to_string(),
        }
    }

    // #endregion 🏷️IconUnion

    #[derive(Debug)]
    pub enum ResolvedIcon {
        None,
        SvgThemed(String),
        SvgPlain(String),
        RasterRgba8 { rgba: Arc<[u8]>, w: u32, h: u32 },
    }

    struct RgbaImage {
        data: Arc<[u8]>,
        w: u32,
        h: u32,
    }

    fn decode_data_url_svg(s: &str) -> Option<String> {
        let t = s.trim();
        let lower = t.to_ascii_lowercase();
        if lower.starts_with("data:image/svg+xml;base64,") {
            let rest = t.split_once(',').map(|(_, b)| b.trim())?;
            let raw = base64_codec::base64_standard_decode(rest).ok()?;
            return String::from_utf8(raw).ok();
        }
        if lower.starts_with("data:image/svg+xml,") {
            let rest = t.split_once(',').map(|(_, b)| b.trim())?;
            return Some(percent_decode_utf8(rest));
        }
        None
    }

    fn percent_decode_utf8(input: &str) -> String {
        let bytes = input.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let (Some(h), Some(l)) = (hex_nibble(bytes[i + 1]), hex_nibble(bytes[i + 2])) {
                    out.push((h << 4) | l);
                    i += 3;
                    continue;
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8_lossy(&out).into_owned()
    }

    fn hex_nibble(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }

    fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
        let s = t.trim();
        let rest = s
            .strip_prefix("data:image/png;base64,")
            .or_else(|| s.strip_prefix("data:image/jpeg;base64,"))
            .or_else(|| s.strip_prefix("data:image/jpg;base64,"))
            .or_else(|| s.strip_prefix("data:image/webp;base64,"))
            .or_else(|| s.strip_prefix("data:image/gif;base64,"))?;
        let raw = base64_codec::base64_standard_decode(rest.trim()).ok()?;
        raster_icon_bytes_to_rgba(&raw)
    }

    /// 🖌️ Full pixel decode for icon *painting* — host/browser only, PNG and JPEG through the owned `semio-framework-pixels` decoders; a GIF or WebP data URL still classifies as a raster icon but yields no pixels here (the browser paints those natively).
    /// `IconPaintCache::get_or_build` (the only caller that ever reads `RgbaImage::data`) is unconditionally `None` on
    /// `wasm32-wasip2` (a WASI guest never paints pixels), so the real decode is never needed
    /// there. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn raster_icon_bytes_to_rgba(raw: &[u8]) -> Option<RgbaImage> {
        let decoded = semio_framework_pixels::image_decoding::decode_encoded_blocking(raw.to_vec(), semio_framework_pixels::image_decoding::EncodedDecodeLimits::MAXIMUM, &|| false).ok()?.image;
        if decoded.width == 0 || decoded.height == 0 {
            return None;
        }
        Some(RgbaImage { data: Arc::from(decoded.pixels.into_boxed_slice()), w: decoded.width, h: decoded.height })
    }

    /// 🖨️ `wasm32-wasip2` arm: header-only dimension read (`semio-framework-intrinsic-size`),
    /// never a full pixel decode. `data` is deliberately empty — the only consumer that would
    /// read real pixels (`IconPaintCache::get_or_build`, icon painting) is itself unconditionally
    /// `None` on this target, so nothing ever reads it; the only live consumer of this arm's
    /// result is `preview_media_natural_size`'s dimension query, which reads `w`/`h` only. Ticket
    /// `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    fn raster_icon_bytes_to_rgba(raw: &[u8]) -> Option<RgbaImage> {
        let (w, h) = super::intrinsic_size::raster_dimensions(raw).ok()?;
        if w == 0 || h == 0 {
            return None;
        }
        Some(RgbaImage { data: Arc::from([].as_slice()), w, h })
    }

    /// 🧮️ Compiles a semio math notation snippet ([`compiler::syntax::parse_formula`]) to SVG
    /// via the `compiler` module — replaces the former Typst-markup icon path. `None` on invalid
    /// notation syntax (graceful degradation, matching the old malformed-typst-markup behavior)
    /// and while the host has not installed the compiler fonts.
    fn resolve_math_src(src: &str) -> ResolvedIcon {
        let src = src.trim();
        if src.is_empty() {
            return ResolvedIcon::None;
        }
        let Some(fonts) = compiler_fonts() else {
            return ResolvedIcon::None;
        };
        let options = compiler::SnippetOptions { font_size_pt: ui_styling::metrics::math::ICON_FONT_SIZE_PT as f32, margin_pt: ui_styling::metrics::math::ICON_MARGIN_PT as f32 };
        match compiler::compile_snippet_to_svg(fonts, src, options) {
            Ok(snippet) => ResolvedIcon::SvgPlain(snippet.svg),
            Err(_) => ResolvedIcon::None,
        }
    }

    /// 😀️ Renders an arbitrary emoji string via `compiler::compile_emoji_to_svg` — `None` only
    /// while the host has not installed the compiler fonts.
    fn resolve_emoji_body(em: &str) -> ResolvedIcon {
        let em = em.trim();
        if em.is_empty() {
            return ResolvedIcon::None;
        }
        let Some(fonts) = compiler_fonts() else {
            return ResolvedIcon::None;
        };
        let options = compiler::SnippetOptions { font_size_pt: ui_styling::metrics::math::EMOJI_FONT_SIZE_PT as f32, margin_pt: ui_styling::metrics::math::EMOJI_MARGIN_PT as f32 };
        ResolvedIcon::SvgPlain(compiler::compile_emoji_to_svg(fonts, em, options).svg)
    }

    /// 📝️ Renders arbitrary text via `compiler::compile_text_to_svg` — `None` only while the host
    /// has not installed the compiler fonts.
    fn resolve_text_body(text: &str) -> ResolvedIcon {
        let text = text.trim();
        if text.is_empty() {
            return ResolvedIcon::None;
        }
        let Some(fonts) = compiler_fonts() else {
            return ResolvedIcon::None;
        };
        let options = compiler::SnippetOptions { font_size_pt: ui_styling::metrics::math::TEXT_FONT_SIZE_PT as f32, margin_pt: ui_styling::metrics::math::TEXT_MARGIN_PT as f32 };
        ResolvedIcon::SvgPlain(compiler::compile_text_to_svg(fonts, text, options).svg)
    }

    fn resolve_icon_data(data: &str) -> ResolvedIcon {
        if let Some(svg) = decode_data_url_svg(data) {
            return ResolvedIcon::SvgPlain(svg);
        }
        if let Some(img) = decode_raster_icon_bytes(data) {
            return ResolvedIcon::RasterRgba8 { rgba: img.data, w: img.w, h: img.h };
        }
        ResolvedIcon::None
    }

    fn resolve_structured_icon(icon: &Icon, themed_lookup: ThemedSvgLookup) -> ResolvedIcon {
        match icon {
            Icon::Url { .. } => ResolvedIcon::None,
            Icon::Shortcode { code } => match icon_shortcode_resolve(code) {
                Some(ShortcodeResolved::Emoji(em)) => resolve_emoji_body(em),
                Some(ShortcodeResolved::SvgPlain(svg)) => ResolvedIcon::SvgPlain(svg.to_string()),
                Some(ShortcodeResolved::SvgThemed(svg)) => ResolvedIcon::SvgThemed(svg.to_string()),
                None => themed_lookup(code).map_or(ResolvedIcon::None, |svg| ResolvedIcon::SvgThemed(svg.to_string())),
            },
            Icon::Data { data } => resolve_icon_data(data),
            Icon::Emoji { emoji } => resolve_emoji_body(emoji),
            Icon::Math { src } => resolve_math_src(src),
            Icon::Text { text } => resolve_text_body(text),
            Icon::Svg { svg } => {
                if themed_lookup(svg).is_some() {
                    ResolvedIcon::SvgThemed(svg.clone())
                } else {
                    ResolvedIcon::SvgPlain(svg.clone())
                }
            }
            Icon::Catalog { key } => match icon_shortcode_resolve(key.as_str()) {
                Some(ShortcodeResolved::SvgPlain(svg)) => ResolvedIcon::SvgPlain(svg.to_string()),
                Some(ShortcodeResolved::SvgThemed(svg)) => ResolvedIcon::SvgThemed(svg.to_string()),
                _ => themed_lookup(key.as_str()).map_or(ResolvedIcon::None, |svg| ResolvedIcon::SvgThemed(svg.to_string())),
            },
            Icon::Themed { key } => match icon_shortcode_resolve(key.as_str()) {
                Some(ShortcodeResolved::SvgThemed(svg)) => ResolvedIcon::SvgThemed(svg.to_string()),
                _ => themed_lookup(key.as_str()).map_or(ResolvedIcon::None, |svg| ResolvedIcon::SvgThemed(svg.to_string())),
            },
        }
    }

    /// 🎨️ Resolves an icon encoding to paintable content; `themed_lookup` marks SVG as themed when present.
    pub fn resolve_icon_kind(encoded: &str, themed_lookup: ThemedSvgLookup) -> ResolvedIcon {
        let Some(icon) = decode_icon(encoded) else {
            return ResolvedIcon::None;
        };
        resolve_structured_icon(&icon, themed_lookup)
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️icon-codec-unit/🦀️.rs");
    // #endregion icon_codec
}
pub use icon_codec::{resolve_icon_kind, decode_icon, encode_icon, ResolvedIcon, Icon, ThemedSvgLookup};
// #endregion 🔖️IconCodec
