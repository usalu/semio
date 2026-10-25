//! 🌈️ Cursor emitter and colour quantizing: the cursor emitter with its style codes, the colour cube and palette conversions and the sgr quantizing. See [`super`].

use super::*;

//#region 🖍️Cursor
/// 🖍️ Tracks what the terminal cursor shows and emits only the escapes that change it, wrapped around a frame.
#[derive(Default)]
pub struct CursorEmitter {
    visible: bool,
    style: Option<(CursorShape, bool)>,
    pub(super) pos: Option<Pos>,
}

fn cursor_style_code(shape: CursorShape, blink: bool) -> u8 {
    let base = match shape {
        CursorShape::Block => 1,
        CursorShape::Underline => 3,
        CursorShape::Bar => 5,
    };
    base + u8::from(!blink)
}

impl CursorEmitter {
    /// 🧼️ Forgets everything known about the terminal cursor, as after entering the alternate screen.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// 🎞️ Composes one frame: hide while drawing, paint `patch`, then place, shape and show (or hide) the cursor.
    pub fn frame(&mut self, patch: &str, cursor: Option<CursorSpec>, synchronized: bool) -> String {
        let mut body = String::with_capacity(patch.len() + 48);
        let draws = !patch.is_empty();
        if draws && self.visible {
            body.push_str("\x1b[?25l");
            self.visible = false;
        }
        body.push_str(patch);
        if draws {
            self.pos = None;
        }
        match cursor {
            Some(spec) => {
                if self.style != Some((spec.shape, spec.blink)) {
                    body.push_str(&format!("\x1b[{} q", cursor_style_code(spec.shape, spec.blink)));
                    self.style = Some((spec.shape, spec.blink));
                }
                if self.pos != Some(spec.pos) {
                    body.push_str(&format!("\x1b[{};{}H", spec.pos.y + 1, spec.pos.x + 1));
                    self.pos = Some(spec.pos);
                }
                if !self.visible {
                    body.push_str("\x1b[?25h");
                    self.visible = true;
                }
            }
            None if self.visible => {
                body.push_str("\x1b[?25l");
                self.visible = false;
            }
            None => {}
        }
        if body.is_empty() || !synchronized {
            return body;
        }
        format!("{SYNC_BEGIN}{body}{SYNC_END}")
    }
}
//#endregion 🖍️Cursor

//#region 🌈️Color
fn scale_to_cube(value: u8) -> u8 {
    (f64::from(value) / 255.0 * 5.0).round() as u8
}

/// 🎨️ Nearest xterm 256-colour palette index for a truecolor value (greys use the 24-step ramp).
pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return ((f64::from(r) - 8.0) / 247.0 * 24.0).round() as u8 + 232;
    }
    16 + 36 * scale_to_cube(r) + 6 * scale_to_cube(g) + scale_to_cube(b)
}

/// 🔅️ The SGR foreground code (30-37 or 90-97) nearest to a truecolor value.
pub fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8 {
    let brightness = (f64::from(r.max(g).max(b)) / 255.0 * 100.0 / 50.0).round() as u8;
    if brightness == 0 {
        return 30;
    }
    let half = |channel: u8| (f64::from(channel) / 255.0).round() as u8;
    let code = 30 + ((half(b) << 2) | (half(g) << 1) | half(r));
    if brightness == 2 {
        code + 60
    } else {
        code
    }
}

/// 🧊️ The RGB value of an xterm 256-colour palette index above the 16 system colours.
pub fn ansi256_to_rgb(index: u8) -> Rgb {
    if index >= 232 {
        let level = 8 + 10 * (index - 232);
        return [level, level, level];
    }
    const LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    let cube = index.saturating_sub(16);
    [LEVELS[usize::from(cube / 36)], LEVELS[usize::from(cube % 36 / 6)], LEVELS[usize::from(cube % 6)]]
}

fn ansi16_code(base: u8, background: bool) -> String {
    (base + if background { 10 } else { 0 }).to_string()
}

fn indexed_to_ansi16(index: u8, background: bool) -> String {
    let base = match index {
        0..=7 => 30 + index,
        8..=15 => 90 + index - 8,
        _ => {
            let [r, g, b] = ansi256_to_rgb(index);
            rgb_to_ansi16(r, g, b)
        }
    };
    ansi16_code(base, background)
}

fn push_token(out: &mut String, wrote: &mut bool, token: &str) {
    if *wrote {
        out.push(';');
    }
    out.push_str(token);
    *wrote = true;
}

fn rewrite_sgr(params: &str, depth: ColorDepth, out: &mut String) {
    let mut wrote = false;
    let mut parts = params.split(';');
    out.push_str("\x1b[");
    while let Some(token) = parts.next() {
        let background = match token {
            "38" => false,
            "48" => true,
            _ => {
                let number = token.parse::<u16>().unwrap_or(u16::MAX);
                let basic = matches!(number, 30..=37 | 40..=47 | 90..=97 | 100..=107);
                if !(basic && depth == ColorDepth::Monochrome) {
                    push_token(out, &mut wrote, token);
                }
                continue;
            }
        };
        let mode = parts.next();
        let channel = |part: Option<&str>| part.and_then(|value| value.parse::<u8>().ok());
        let rewritten = match mode {
            Some("2") => {
                let (r, g, b) = (channel(parts.next()), channel(parts.next()), channel(parts.next()));
                match (r, g, b) {
                    (Some(r), Some(g), Some(b)) => Some(match depth {
                        ColorDepth::Ansi256 => Some(format!("{};5;{}", if background { 48 } else { 38 }, rgb_to_ansi256(r, g, b))),
                        ColorDepth::Ansi16 => Some(ansi16_code(rgb_to_ansi16(r, g, b), background)),
                        _ => None,
                    }),
                    _ => None,
                }
            }
            Some("5") => channel(parts.next()).map(|index| match depth {
                ColorDepth::Ansi256 => Some(format!("{};5;{index}", if background { 48 } else { 38 })),
                ColorDepth::Ansi16 => Some(indexed_to_ansi16(index, background)),
                _ => None,
            }),
            _ => None,
        };
        if let Some(Some(text)) = rewritten {
            push_token(out, &mut wrote, &text);
        }
    }
    out.push('m');
}

/// 🔽️ Rewrites the truecolor SGR sequences of a patch down to the colours a terminal of `depth` renders.
pub fn quantize_patch(patch: &str, depth: ColorDepth) -> Cow<'_, str> {
    if depth == ColorDepth::TrueColor {
        return Cow::Borrowed(patch);
    }
    let mut out = String::with_capacity(patch.len());
    let mut rest = patch;
    while let Some(start) = rest.find("\x1b[") {
        out.push_str(&rest[..start]);
        let tail = &rest[start + 2..];
        match tail.bytes().position(|byte| (0x40..=0x7e).contains(&byte)) {
            Some(end) if tail.as_bytes()[end] == b'm' => {
                rewrite_sgr(&tail[..end], depth, &mut out);
                rest = &tail[end + 1..];
            }
            Some(end) => {
                out.push_str(&rest[start..start + 2 + end + 1]);
                rest = &tail[end + 1..];
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}
//#endregion 🌈️Color
