//! 🎨️ CSS color parsing of the board model: hex, hsl and general CSS colors with the hue, saturation, alpha and byte conversions. See [`super`].

use super::*;

pub fn parse_css_hex_color(s: &str) -> Option<Color> {
    let s = s.trim();
    let hex = s.strip_prefix('#')?;
    match hex.len() {
        3 => {
            let mut full = String::new();
            for ch in hex.chars() {
                full.push(ch);
                full.push(ch);
            }
            let v = u32::from_str_radix(&full, 16).ok()?;
            let r = ((v >> 16) & 0xff) as u8;
            let g = ((v >> 8) & 0xff) as u8;
            let b = (v & 0xff) as u8;
            Some(Color::from_rgba8(r, g, b, 255))
        }
        6 => {
            let v = u32::from_str_radix(hex, 16).ok()?;
            let r = ((v >> 16) & 0xff) as u8;
            let g = ((v >> 8) & 0xff) as u8;
            let b = (v & 0xff) as u8;
            Some(Color::from_rgba8(r, g, b, 255))
        }
        8 => {
            let v = u32::from_str_radix(hex, 16).ok()?;
            let r = ((v >> 24) & 0xff) as u8;
            let g = ((v >> 16) & 0xff) as u8;
            let b = ((v >> 8) & 0xff) as u8;
            let a = (v & 0xff) as u8;
            Some(Color::from_rgba8(r, g, b, a))
        }
        _ => None,
    }
}

/// 🎨️ Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
pub fn parse_css_color(s: &str) -> Option<Color> {
    if let Some(c) = parse_css_hex_color(s) {
        return Some(c);
    }
    parse_css_hsl_color(s)
}

fn parse_css_hsl_color(s: &str) -> Option<Color> {
    let low = s.trim().to_ascii_lowercase();
    let (legacy_alpha_form, inner) = if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
        (true, inner)
    } else {
        let inner = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')'))?;
        (false, inner)
    };
    let inner = inner.trim();
    let (main, alpha_slash) = inner.split_once('/').map_or((inner, None), |(a, b)| (a.trim(), Some(b.trim())));
    let normalized = main.replace(',', " ");
    let parts: Vec<&str> = normalized.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    let h = parse_css_hsl_hue(parts[0])?;
    let sat = parse_css_hsl_sl(parts[1])?;
    let light = parse_css_hsl_sl(parts[2])?;
    let alpha = if let Some(a) = alpha_slash {
        parse_css_alpha_channel(a)?
    } else if legacy_alpha_form && parts.len() >= 4 {
        parse_css_alpha_channel(parts[3])?
    } else {
        1.0
    };
    let (r, g, b) = hsl_to_rgb_bytes(h, sat, light);
    let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    Some(Color::from_rgba8(r, g, b, a))
}

fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
    let t = tok.trim();
    let n = t.strip_suffix("deg").map_or(t, str::trim);
    let v: f64 = n.parse().ok()?;
    v.is_finite().then_some(v)
}

fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
    let t = tok.trim();
    if let Some(p) = t.strip_suffix('%') {
        let v: f64 = p.trim().parse().ok()?;
        Some((v / 100.0).clamp(0.0, 1.0))
    } else {
        let v: f64 = t.parse().ok()?;
        Some(v.clamp(0.0, 1.0))
    }
}

fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
    parse_css_hsl_sl(tok)
}

fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 0.5 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);
    if s <= f64::EPSILON {
        let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
        return (v, v, v);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let r = board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
    let g = board_hsl_hue_to_rgb_component(p, q, h_norm);
    let b = board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
    ((r * 255.0).round().clamp(0.0, 255.0) as u8, (g * 255.0).round().clamp(0.0, 255.0) as u8, (b * 255.0).round().clamp(0.0, 255.0) as u8)
}
