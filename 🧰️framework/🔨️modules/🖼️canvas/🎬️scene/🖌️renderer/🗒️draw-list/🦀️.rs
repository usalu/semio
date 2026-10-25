//! 🗒️ Draw list of the scene: the portable replay encoding of a scene command list as numbers a 2D canvas context can replay verb for verb. See [`super`].

use super::{BlendMode, Cap, FillRule, Paint, RecordedShape, Scene, SceneCommand};
use geometry::{Affine, PathEl};

/// 🔖️ Wire version of the encoding below; a replayer MUST refuse a version it does not know.
pub const DRAW_LIST_VERSION: u32 = 1;
/// 📐️ Flattening tolerance for the two non-primitive shapes (`Arc`, `BezPath` curves are
/// emitted exactly; only `Arc` is flattened), in the scene's own pre-transform units.
pub const DEFAULT_DRAW_LIST_TOLERANCE: f64 = 0.1;
/// 🔢️ Decimal places every coordinate is rounded to. Three is ~1/1000 of a device pixel at
/// unit scale — below any display's resolving power, and it is what makes two
/// implementations' draw lists comparable as text.
pub const DEFAULT_DRAW_LIST_DECIMALS: usize = 3;

/// ⚙️ Encoder knobs. `maximum_commands` is a hard ceiling, not a hint: the encoded list
/// travels a bounded ABI reply body, so a pathological scene must truncate loudly
/// (`truncated: true` in the payload) rather than overrun it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawListOptions {
    pub tolerance: f64,
    pub decimals: usize,
    pub maximum_commands: usize,
}

impl Default for DrawListOptions {
    fn default() -> Self {
        Self { tolerance: DEFAULT_DRAW_LIST_TOLERANCE, decimals: DEFAULT_DRAW_LIST_DECIMALS, maximum_commands: 65_536 }
    }
}

fn fill_rule_code(rule: FillRule) -> u8 {
    match rule {
        FillRule::NonZero => 0,
        FillRule::EvenOdd => 1,
    }
}

fn cap_code(cap: Cap) -> u8 {
    match cap {
        Cap::Butt => 0,
        Cap::Round => 1,
        Cap::Square => 2,
    }
}

fn blend_code(blend: BlendMode) -> u8 {
    match blend {
        BlendMode::Normal => 0,
        BlendMode::Multiply => 1,
        BlendMode::Screen => 2,
        BlendMode::Overlay => 3,
        BlendMode::Darken => 4,
        BlendMode::Lighten => 5,
        BlendMode::ColorDodge => 6,
        BlendMode::ColorBurn => 7,
        BlendMode::HardLight => 8,
        BlendMode::SoftLight => 9,
        BlendMode::Difference => 10,
        BlendMode::Exclusion => 11,
        BlendMode::Hue => 12,
        BlendMode::Saturation => 13,
        BlendMode::Color => 14,
        BlendMode::Luminosity => 15,
    }
}

fn push_number(out: &mut String, value: f64, decimals: usize) {
    if !value.is_finite() {
        out.push('0');
        return;
    }
    let text = format!("{value:.decimals$}");
    let trimmed = if text.contains('.') { text.trim_end_matches('0').trim_end_matches('.') } else { &text };
    out.push_str(if trimmed.is_empty() || trimmed == "-" { "0" } else { trimmed });
}

fn push_numbers(out: &mut String, values: impl IntoIterator<Item = f64>, decimals: usize) {
    let mut first = true;
    for value in values {
        if !first {
            out.push(',');
        }
        first = false;
        push_number(out, value, decimals);
    }
}

fn push_affine(out: &mut String, transform: Affine, decimals: usize) {
    out.push('[');
    push_numbers(out, transform.as_coeffs(), decimals);
    out.push(']');
}

fn push_paint(out: &mut String, paint: &Paint) {
    let Paint::Solid(color) = paint;
    let rgba = color.to_rgba8();
    out.push_str(&format!("[{},{},{},{}]", rgba.r, rgba.g, rgba.b, rgba.a));
}

fn push_path_elements(out: &mut String, elements: &[PathEl], decimals: usize) {
    out.push_str(r#"["p",["#);
    let mut first = true;
    for element in elements {
        if !first {
            out.push(',');
        }
        first = false;
        match element {
            PathEl::MoveTo(p) => {
                out.push('0');
                out.push(',');
                push_numbers(out, [p.x, p.y], decimals);
            }
            PathEl::LineTo(p) => {
                out.push('1');
                out.push(',');
                push_numbers(out, [p.x, p.y], decimals);
            }
            PathEl::QuadTo(c, p) => {
                out.push('2');
                out.push(',');
                push_numbers(out, [c.x, c.y, p.x, p.y], decimals);
            }
            PathEl::CurveTo(c1, c2, p) => {
                out.push('3');
                out.push(',');
                push_numbers(out, [c1.x, c1.y, c2.x, c2.y, p.x, p.y], decimals);
            }
            PathEl::ClosePath => out.push('4'),
        }
    }
    out.push_str("]]");
}

fn push_shape(out: &mut String, shape: &RecordedShape, options: DrawListOptions) {
    let decimals = options.decimals;
    match shape {
        RecordedShape::Rect(rect) => {
            out.push_str(r#"["r","#);
            push_numbers(out, [rect.x0(), rect.y0(), rect.x1(), rect.y1()], decimals);
            out.push(']');
        }
        RecordedShape::RoundedRect(rounded) => {
            let rect = rounded.rect();
            let [tl, tr, br, bl] = rounded.radii().as_clockwise();
            out.push_str(r#"["rr","#);
            push_numbers(out, [rect.x0(), rect.y0(), rect.x1(), rect.y1(), tl, tr, br, bl], decimals);
            out.push(']');
        }
        RecordedShape::Circle(circle) => {
            let center = circle.center();
            out.push_str(r#"["ci","#);
            push_numbers(out, [center.x, center.y, circle.radius()], decimals);
            out.push(']');
        }
        RecordedShape::Line(line) => {
            let (p0, p1) = (line.p0(), line.p1());
            out.push_str(r#"["ln","#);
            push_numbers(out, [p0.x, p0.y, p1.x, p1.y], decimals);
            out.push(']');
        }
        RecordedShape::CubicBez(cubic) => {
            out.push_str(r#"["cb","#);
            push_numbers(out, [cubic.p0.x, cubic.p0.y, cubic.p1.x, cubic.p1.y, cubic.p2.x, cubic.p2.y, cubic.p3.x, cubic.p3.y], decimals);
            out.push(']');
        }
        RecordedShape::Arc(arc) => push_path_elements(out, &arc.path_elements(options.tolerance), decimals),
        RecordedShape::BezPath(path) => push_path_elements(out, &path.elements(), decimals),
    }
}

const BASE64_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// 🔤️ First-party base64 for the one command that carries opaque bytes. Written here
/// rather than taken from a crate because this module must stay dependency-free on every
/// target it compiles for.
fn push_base64(out: &mut String, bytes: &[u8]) {
    out.push('"');
    for chunk in bytes.chunks(3) {
        let b0 = u32::from(chunk[0]);
        let b1 = chunk.get(1).copied().map_or(0, u32::from);
        let b2 = chunk.get(2).copied().map_or(0, u32::from);
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(BASE64_ALPHABET[((triple >> 18) & 63) as usize] as char);
        out.push(BASE64_ALPHABET[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 { BASE64_ALPHABET[((triple >> 6) & 63) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { BASE64_ALPHABET[(triple & 63) as usize] as char } else { '=' });
    }
    out.push('"');
}

fn push_command(out: &mut String, command: &SceneCommand, options: DrawListOptions) {
    let decimals = options.decimals;
    match command {
        SceneCommand::Fill { rule, transform, paint, shape, .. } => {
            out.push_str(r#"["f","#);
            out.push_str(&fill_rule_code(*rule).to_string());
            out.push(',');
            push_paint(out, paint);
            out.push(',');
            push_affine(out, *transform, decimals);
            out.push(',');
            push_shape(out, shape, options);
            out.push(']');
        }
        SceneCommand::Stroke { stroke, transform, paint, shape, .. } => {
            out.push_str(r#"["s","#);
            push_number(out, stroke.width, decimals);
            out.push_str(&format!(",{},{},", cap_code(stroke.start_cap), cap_code(stroke.end_cap)));
            out.push('[');
            push_numbers(out, stroke.dash_pattern.iter().copied(), decimals);
            out.push_str("],");
            push_number(out, stroke.dash_offset, decimals);
            out.push(',');
            push_paint(out, paint);
            out.push(',');
            push_affine(out, *transform, decimals);
            out.push(',');
            push_shape(out, shape, options);
            out.push(']');
        }
        SceneCommand::DrawImage { image, transform } => {
            out.push_str(&format!(r#"["i",{},{},"#, image.width(), image.height()));
            push_base64(out, &image.data);
            out.push(',');
            push_affine(out, *transform, decimals);
            out.push(']');
        }
        SceneCommand::PushLayer { rule, blend, alpha, transform, clip } => {
            out.push_str(r#"["pl","#);
            out.push_str(&format!("{},{},", fill_rule_code(*rule), blend_code(*blend)));
            push_number(out, f64::from(*alpha), decimals);
            out.push(',');
            push_affine(out, *transform, decimals);
            out.push(',');
            push_shape(out, clip, options);
            out.push(']');
        }
        SceneCommand::PushClipLayer { rule, transform, clip } => {
            out.push_str(r#"["pc","#);
            out.push_str(&format!("{},", fill_rule_code(*rule)));
            push_affine(out, *transform, decimals);
            out.push(',');
            push_shape(out, clip, options);
            out.push(']');
        }
        SceneCommand::PopLayer => out.push_str(r#"["po"]"#),
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        SceneCommand::VelloFragment { .. } => out.push_str(r#"["po"]"#),
    }
}

/// 🎬️ Encodes every replayable command of `scene` as one JSON draw list.
///
/// `{"version":1,"truncated":false,"commands":[…]}` — each command a tagged flat array:
/// `["f",rule,[r,g,b,a],[a,b,c,d,e,f],shape]` fill, `["s",width,capStart,capEnd,[dash…],
/// dashOffset,[r,g,b,a],[affine],shape]` stroke, `["i",w,h,"<base64 rgba8>",[affine]]`
/// image, `["pl",rule,blend,alpha,[affine],clip]` layer, `["pc",rule,[affine],clip]` clip,
/// `["po"]` pop. Rule `0` non-zero / `1` even-odd; cap `0` butt / `1` round / `2` square;
/// blend is [`BlendMode`]'s own declaration order. Shapes: `["r",x0,y0,x1,y1]`,
/// `["rr",x0,y0,x1,y1,tl,tr,br,bl]`, `["ci",cx,cy,r]`, `["ln",x0,y0,x1,y1]`,
/// `["cb",8 coords]`, `["p",[verbs]]` with verb `0` move / `1` line / `2` quad / `3` cubic
/// / `4` close.
///
/// A `VelloFragment` (the one host-only escape hatch, produced solely by
/// [`SvgDocument::append_to_scene`]) carries an opaque `vello` encoding with no
/// first-party commands to read, so it encodes as a bare `["po"]` — a no-op a replayer
/// skips — rather than silently shifting every later command's layer depth.
pub fn scene_draw_list_json(scene: &Scene, options: DrawListOptions) -> String {
    let mut out = String::new();
    write_scene_draw_list(&mut out, scene, options);
    out
}

/// 📝️ Appends the same encoding into a caller-owned buffer.
///
/// The buffer is the point: a guest paints EVERY frame through here, and dlmalloc never
/// returns freed pages to the module, so a fresh multi-kilobyte `String` per frame
/// fragments the guest heap until it aborts. One buffer, cleared and refilled, keeps the
/// allocation count per frame at zero once it has reached its working size.
pub fn write_scene_draw_list(out: &mut String, scene: &Scene, options: DrawListOptions) {
    let truncated = scene.0.len() > options.maximum_commands;
    out.reserve(scene.0.len().min(options.maximum_commands) * 96 + 64);
    out.push_str(r#"{"version":"#);
    out.push_str(&DRAW_LIST_VERSION.to_string());
    out.push_str(if truncated { r#","truncated":true,"commands":["# } else { r#","truncated":false,"commands":["# });
    for (index, command) in scene.0.iter().take(options.maximum_commands).enumerate() {
        if index != 0 {
            out.push(',');
        }
        push_command(out, command, options);
    }
    out.push_str("]}");
}
