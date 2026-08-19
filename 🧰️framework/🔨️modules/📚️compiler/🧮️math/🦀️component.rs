//! 🧮️ `compiler_math` — lays out a `crate::syntax::MathNode` into a backend-neutral tree of
//! positioned glyphs, rules, and images (a [`MathBox`]), driven by the loaded fonts' OpenType
//! `MATH` table via `compiler_text`. All layout arithmetic works in **em units** (`1.0` = one font
//! size, regardless of which physical font a given glyph came from — each font's own
//! `units_per_em` is divided out immediately after any `compiler_text` call) so callers scale to a
//! concrete point size only once, at the very end.
//!
//! Scope (Wave 2), honestly bounded — see the ticket for the full list:
//! - Stretchy delimiters/radicals use the font's `MathVariants` glyph list, falling back to a
//!   vertical outline scale ([`PlacedItem::Glyph::scale_y`]) when no variant is tall enough — no
//!   full non-linear glyph *assembly* (repeatable middle pieces) yet.
//! - Operator/relation spacing uses TeX's classic fixed thin/medium space constants, not a
//!   font-derived `MathClass` spacing table (the `MATH` table doesn't carry one).
//! - Fractions/scripts use the constants directly rather than the OpenType spec's display-style-
//!   adaptive branching (this crate always lays out at one style level).

use crate::syntax::{BinOp, MathNode};
use crate::text::Font;

//#region 🔖️Model
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FontKind {
    Math,
    Serif,
    Mono,
    Emoji,
}

/// @emoji 🖼️ One item to draw, already positioned in the containing [`MathBox`]'s own em-space:
/// `x` rightward from the box's left edge, `y` UPWARD from the box's baseline (so `y > 0` is above
/// the baseline, matching height/depth's own sign convention).
#[derive(Clone, Debug, PartialEq)]
pub enum PlacedItem {
    Glyph { font: FontKind, glyph_id: u16, x: f32, y: f32, scale_y: f32 },
    Rule { x: f32, y: f32, width: f32, height: f32 },
    Image { data: Vec<u8>, x: f32, y: f32, width: f32, height: f32 },
}

/// @emoji 📦️ A laid-out box: its own metrics (`width`, `height` above baseline, `depth` below) plus
/// every item placed inside it, in the box's own local em-space.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct MathBox {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub items: Vec<PlacedItem>,
}

impl MathBox {
    async fn empty() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Offsets every item in `self` by `(dx, dy)` — used when placing an already-built box
    /// inside a larger one.
    async fn translated(mut self, dx: f32, dy: f32) -> Self {
        for item in &mut self.items {
            match item {
                PlacedItem::Glyph { x, y, .. } | PlacedItem::Rule { x, y, .. } | PlacedItem::Image { x, y, .. } => {
                    *x += dx;
                    *y += dy;
                }
            }
        }
        self
    }
}

/// @emoji 🎯️ The fonts a formula lays out against. Borrowed, not owned — callers keep the
/// underlying `compiler_world` bytes (or their own) alive for the layout's lifetime.
pub struct FontContext<'a> {
    pub math: &'a Font<'a>,
    pub serif: &'a Font<'a>,
    pub mono: &'a Font<'a>,
    pub emoji: &'a Font<'a>,
}

/// @emoji 📐️ TeX-classic fixed spacing constants (eighteenths of an em) — the `MATH` table has no
/// dedicated inter-atom spacing table, so these are the same conventional values TeX itself hardcodes.
const THIN_SPACE_EM: f32 = 3.0 / 18.0;
const MEDIUM_SPACE_EM: f32 = 4.0 / 18.0;
/// Fixed matrix/cases column and row gap — likewise not a `MATH` table constant.
const GRID_GAP_EM: f32 = 0.3;
//#endregion 🔖️Model

//#region 🔖️Units
async fn em(font: &Font<'_>, units: i32) -> f32 {
    units as f32 / font.units_per_em() as f32
}

async fn mc_em(font: &Font<'_>, value: i16) -> f32 {
    em(font, value as i32)
}

async fn scale_down(percent: i16) -> f32 {
    (percent as f32 / 100.0).clamp(0.05, 1.0)
}
//#endregion 🔖️Units

//#region 🔖️Symbols
/// @emoji 🔤️ Multi-letter identifiers that resolve to a single symbol glyph, shaped via the Math
/// font (so they render in the font's own italic/symbol design, matching `sin`-style upright names'
/// *opposite* treatment below). Not exhaustive — grows as real usage needs more.
async fn named_symbol(name: &str) -> Option<char> {
    Some(match name {
        "alpha" => 'α',
        "beta" => 'β',
        "gamma" => 'γ',
        "delta" => 'δ',
        "epsilon" => 'ε',
        "zeta" => 'ζ',
        "eta" => 'η',
        "theta" => 'θ',
        "iota" => 'ι',
        "kappa" => 'κ',
        "lambda" => 'λ',
        "mu" => 'μ',
        "nu" => 'ν',
        "xi" => 'ξ',
        "pi" => 'π',
        "rho" => 'ρ',
        "sigma" => 'σ',
        "tau" => 'τ',
        "upsilon" => 'υ',
        "phi" => 'φ',
        "chi" => 'χ',
        "psi" => 'ψ',
        "omega" => 'ω',
        "Gamma" => 'Γ',
        "Delta" => 'Δ',
        "Theta" => 'Θ',
        "Lambda" => 'Λ',
        "Xi" => 'Ξ',
        "Pi" => 'Π',
        "Sigma" => 'Σ',
        "Upsilon" => 'Υ',
        "Phi" => 'Φ',
        "Psi" => 'Ψ',
        "Omega" => 'Ω',
        "infinity" => '∞',
        "partial" => '∂',
        "nabla" => '∇',
        "sum" => '∑',
        "int" => '∫',
        "prod" => '∏',
        "times" => '×',
        "cdot" => '⋅',
        "pm" => '±',
        "approx" => '≈',
        "equiv" => '≡',
        "in" => '∈',
        "notin" => '∉',
        "subset" => '⊂',
        "cup" => '∪',
        "cap" => '∩',
        "forall" => '∀',
        "exists" => '∃',
        "emptyset" => '∅',
        "hbar" => 'ℏ',
        "ell" => 'ℓ',
        "aleph" => 'ℵ',
        "prime" => '′',
        "dot_operator" => '⋅',
        _ => return None,
    })
}

/// @emoji 🔤️ Multi-letter identifiers that render upright via the text (serif) font, per math
/// typesetting convention for named functions.
async fn is_upright_function_name(name: &str) -> bool {
    matches!(name, "sin" | "cos" | "tan" | "cot" | "sec" | "csc" | "sinh" | "cosh" | "tanh" | "lim" | "log" | "ln" | "exp" | "min" | "max" | "det" | "gcd" | "arg" | "sup" | "inf" | "mod" | "dim" | "ker" | "hom")
}

/// @emoji 😀️ A curated common-name subset of emoji shortcodes — not exhaustive, grows with usage.
async fn emoji_codepoint(name: &str) -> Option<char> {
    Some(match name {
        "rocket" => '🚀',
        "star" => '⭐',
        "fire" => '🔥',
        "heart" => '❤',
        "check" => '✅',
        "cross" => '❌',
        "warning" => '⚠',
        "bulb" => '💡',
        "gear" => '⚙',
        "bug" => '🐛',
        "sparkles" => '✨',
        "tada" => '🎉',
        "eyes" => '👀',
        "thumbsup" => '👍',
        "smile" => '😀',
        _ => return None,
    })
}
//#endregion 🔖️Symbols

//#region 🔖️Atoms
/// @emoji ✍️ Shapes `text` against `font`'s own vocabulary and returns a box for the run, with
/// per-glyph bounding boxes (not blanket font ascender/descender) sizing `height`/`depth`.
async fn text_box(font: &Font<'_>, font_kind: FontKind, text: &str) -> MathBox {
    let run = crate::text::shape(font, text);
    let mut items = Vec::with_capacity(run.glyphs.len());
    let mut x = 0.0f32;
    let mut height = 0.0f32;
    let mut depth = 0.0f32;
    for glyph in &run.glyphs {
        let gx = x + em(font, glyph.x_offset);
        let gy = em(font, glyph.y_offset);
        if let Some((_, y_min, _, y_max)) = font.glyph_bounding_box(glyph.glyph_id) {
            height = height.max(em(font, y_max as i32) + gy);
            depth = depth.max(-em(font, y_min as i32) - gy);
        }
        items.push(PlacedItem::Glyph { font: font_kind, glyph_id: glyph.glyph_id, x: gx, y: gy, scale_y: 1.0 });
        x += em(font, glyph.x_advance);
    }
    MathBox { width: x, height: height.max(0.0), depth: depth.max(0.0), items }
}

async fn char_box(font: &Font<'_>, font_kind: FontKind, ch: char) -> MathBox {
    let mut buf = [0u8; 4];
    text_box(font, font_kind, ch.encode_utf8(&mut buf))
}

async fn number_box(fonts: &FontContext<'_>, text: &str) -> MathBox {
    text_box(fonts.serif, FontKind::Serif, text)
}

async fn quoted_text_box(fonts: &FontContext<'_>, text: &str) -> MathBox {
    text_box(fonts.serif, FontKind::Serif, text)
}

/// @emoji 🔤️ A `Symbol` atom: a single character always goes through the Math font (whose default
/// glyph shapes for Latin/Greek letters ARE italic — the standard OpenType MATH convention, not a
/// synthetic slant); a known multi-letter name resolves via [`named_symbol`] (also Math font); a
/// known function name renders upright via the Serif font; anything else falls back to the Math
/// font for the whole run (a reasonable, documented best effort for unknown multi-letter symbols).
async fn symbol_box(fonts: &FontContext<'_>, name: &str) -> MathBox {
    if name.chars().count() == 1 {
        let ch = name.chars().next().expect("checked count == 1");
        return char_box(fonts.math, FontKind::Math, ch);
    }
    if let Some(ch) = named_symbol(name) {
        return char_box(fonts.math, FontKind::Math, ch);
    }
    if is_upright_function_name(name) {
        return text_box(fonts.serif, FontKind::Serif, name);
    }
    text_box(fonts.math, FontKind::Math, name)
}

const EMOJI_PIXELS_PER_EM: u16 = 96;

async fn raster_glyph_box(raster: crate::text::RasterGlyph) -> MathBox {
    // CBDT strikes are square raster images meant to sit on the baseline at roughly cap-height —
    // approximate as a 1em x 1em box (matches how NotoColorEmoji is used elsewhere in this repo's
    // icon codec: a fixed square glyph slot), scaled from the strike's own pixel size.
    let size_em = raster.pixels_per_em as f32 / EMOJI_PIXELS_PER_EM as f32;
    MathBox { width: size_em, height: size_em * 0.8, depth: size_em * 0.2, items: vec![PlacedItem::Image { data: raster.data, x: 0.0, y: -size_em * 0.2, width: size_em, height: size_em }] }
}

async fn emoji_box(fonts: &FontContext<'_>, name: &str) -> MathBox {
    let Some(ch) = emoji_codepoint(name) else {
        // Unknown shortcode: render the `:name:` source literally via the serif font rather than
        // silently dropping it — a visible fallback beats a blank box.
        return text_box(fonts.serif, FontKind::Serif, &format!(":{name}:"));
    };
    let Some(glyph_id) = fonts.emoji.glyph_index(ch) else {
        return text_box(fonts.serif, FontKind::Serif, &format!(":{name}:"));
    };
    let Some(raster) = crate::text::glyph_raster_image(fonts.emoji, glyph_id, EMOJI_PIXELS_PER_EM) else {
        return text_box(fonts.serif, FontKind::Serif, &format!(":{name}:"));
    };
    raster_glyph_box(raster)
}
//#endregion 🔖️Atoms

//#region 🔖️Raw
/// @emoji 🔤️ Shapes arbitrary `text` via the Serif font and lays it out left to right — for callers
/// rendering a plain caller-supplied string (e.g. an icon label), not parsed math notation. Unlike
/// [`crate::syntax::parse_formula`]-driven layout, this never fails and never interprets `text`'s
/// characters as notation syntax (`_ ; < > !` included) — appropriate since arbitrary text is not
/// guaranteed to be valid math notation and shouldn't be rejected or misparsed as such.
pub async fn layout_raw_text(fonts: &FontContext<'_>, text: &str) -> MathBox {
    text_box(fonts.serif, FontKind::Serif, text)
}

/// @emoji 💻️ Shapes arbitrary `text` via the Mono font — for monospace code/source snippets, same
/// non-parsing guarantee as [`layout_raw_text`].
pub async fn layout_raw_code(fonts: &FontContext<'_>, text: &str) -> MathBox {
    text_box(fonts.mono, FontKind::Mono, text)
}

/// @emoji 😀️ Shapes arbitrary `text` (typically one emoji, possibly a multi-codepoint ZWJ/skin-tone
/// sequence — HarfBuzz shaping via [`crate::text::shape`] already collapses those into the
/// sequence's own ligature glyph where the font supports it) against the Emoji font, laying out
/// each resulting glyph as a raster image where the font provides one (every glyph in the vendored
/// Noto Color Emoji CBDT subset), falling back to an outline path for any glyph that doesn't
/// (defensive — not expected to trigger against this specific font). Unlike [`MathNode::Emoji`]
/// (a curated `:shortcode:` name resolved through [`emoji_codepoint`]), this accepts any text the
/// caller already resolved to a concrete Unicode emoji string.
pub async fn layout_raw_emoji(fonts: &FontContext<'_>, text: &str) -> MathBox {
    let run = crate::text::shape(fonts.emoji, text);
    let mut items = Vec::with_capacity(run.glyphs.len());
    let mut x = 0.0f32;
    let mut height = 0.0f32;
    let mut depth = 0.0f32;
    for glyph in &run.glyphs {
        if let Some(raster) = crate::text::glyph_raster_image(fonts.emoji, glyph.glyph_id, EMOJI_PIXELS_PER_EM) {
            let sized = raster_glyph_box(raster).translated(x, 0.0);
            height = height.max(sized.height);
            depth = depth.max(sized.depth);
            x += sized.width;
            items.extend(sized.items);
            continue;
        }
        let gx = x + em(fonts.emoji, glyph.x_offset);
        let gy = em(fonts.emoji, glyph.y_offset);
        if let Some((_, y_min, _, y_max)) = fonts.emoji.glyph_bounding_box(glyph.glyph_id) {
            height = height.max(em(fonts.emoji, y_max as i32) + gy);
            depth = depth.max(-em(fonts.emoji, y_min as i32) - gy);
        }
        items.push(PlacedItem::Glyph { font: FontKind::Emoji, glyph_id: glyph.glyph_id, x: gx, y: gy, scale_y: 1.0 });
        x += em(fonts.emoji, glyph.x_advance);
    }
    MathBox { width: x, height: height.max(0.0), depth: depth.max(0.0), items }
}
//#endregion 🔖️Raw

//#region 🔖️Combinators
/// @emoji ↔️ Lays `children` left to right along a shared baseline (each child's own `y = 0` is the
/// baseline), summing widths and combining `height`/`depth` as the max over all children.
async fn hbox(children: Vec<MathBox>) -> MathBox {
    let mut out = MathBox::empty();
    let mut x = 0.0f32;
    for child in children {
        let w = child.width;
        let h = child.height;
        let d = child.depth;
        out.items.extend(child.translated(x, 0.0).items);
        x += w;
        out.height = out.height.max(h);
        out.depth = out.depth.max(d);
    }
    out.width = x;
    out
}

/// @emoji ↕️ Stacks `top` above `bottom` on one shared center column, `gap` apart (vertical
/// whitespace between `top`'s depth and `bottom`'s height), returning the combined box with
/// `axis_y` as the resulting box's own baseline-relative placement of the stack's vertical center.
async fn vstack_centered(top: MathBox, bottom: MathBox, gap: f32, axis_y: f32) -> MathBox {
    let width = top.width.max(bottom.width);
    let top_x = (width - top.width) / 2.0;
    let bottom_x = (width - bottom.width) / 2.0;
    let top_depth = top.depth;
    let top_height = top.height;
    let bottom_height = bottom.height;
    let bottom_depth = bottom.depth;
    // Place bottom's baseline first (relative to the shared axis), then derive top's baseline from
    // the required gap between top's descent and bottom's ascent.
    let bottom_baseline_y = axis_y - gap / 2.0 - bottom_height;
    let top_baseline_y = bottom_baseline_y + bottom_height + gap + top_depth;
    let mut out = MathBox::empty();
    out.items.extend(top.translated(top_x, top_baseline_y).items);
    out.items.extend(bottom.translated(bottom_x, bottom_baseline_y).items);
    out.width = width;
    out.height = (top_baseline_y + top_height).max(0.0);
    out.depth = (-(bottom_baseline_y - bottom_depth)).max(0.0);
    out
}
//#endregion 🔖️Combinators

//#region 🔖️Scripts
async fn layout_scripted(fonts: &FontContext<'_>, base: MathBox, script: MathBox, superscript: bool) -> MathBox {
    let Some(constants) = crate::text::math_constants(fonts.math) else {
        // No MATH table (shouldn't happen with the vendored Math font, but never panic on a
        // missing optional table): fall back to a fixed 60% scale-down and a simple half-height shift.
        let scaled = scale_box(script, 0.6);
        let shift = if superscript { base.height * 0.6 } else { -base.depth * 0.4 };
        return hbox(vec![base, scaled.translated(0.0, shift).with_extent_from_shift(shift)]);
    };
    let percent = scale_down(constants.script_percent_scale_down);
    let scaled = scale_box(script, percent);
    let shift = if superscript {
        (mc_em(fonts.math, constants.superscript_shift_up)).max(base.height - mc_em(fonts.math, constants.superscript_bottom_min).min(0.0)).max(mc_em(fonts.math, constants.superscript_bottom_min) + scaled.depth)
    } else {
        (mc_em(fonts.math, constants.subscript_shift_down)).max(base.depth + mc_em(fonts.math, constants.subscript_top_max).min(0.0)).max(scaled.height - mc_em(fonts.math, constants.subscript_top_max))
    };
    let signed_shift = if superscript { shift } else { -shift };
    let mut out = base;
    let script_x = out.width;
    out.items.extend(scaled.clone().translated(script_x, signed_shift).items);
    out.width += scaled.width;
    if superscript {
        out.height = out.height.max(signed_shift + scaled.height);
    } else {
        out.depth = out.depth.max(-signed_shift + scaled.depth);
    }
    out
}

async fn scale_box(mut box_: MathBox, factor: f32) -> MathBox {
    for item in &mut box_.items {
        match item {
            PlacedItem::Glyph { x, y, scale_y, .. } => {
                *x *= factor;
                *y *= factor;
                *scale_y *= factor;
            }
            PlacedItem::Rule { x, y, width, height } => {
                *x *= factor;
                *y *= factor;
                *width *= factor;
                *height *= factor;
            }
            PlacedItem::Image { x, y, width, height, .. } => {
                *x *= factor;
                *y *= factor;
                *width *= factor;
                *height *= factor;
            }
        }
    }
    box_.width *= factor;
    box_.height *= factor;
    box_.depth *= factor;
    box_
}

trait WithExtentFromShift {
    async fn with_extent_from_shift(self, shift: f32) -> Self;
}
impl WithExtentFromShift for MathBox {
    async fn with_extent_from_shift(mut self, shift: f32) -> Self {
        if shift >= 0.0 {
            self.height += shift;
        } else {
            self.depth += -shift;
        }
        self
    }
}
//#endregion 🔖️Scripts

//#region 🔖️Stretch
/// @emoji 📏️ Picks the smallest declared vertical stretch variant of `base_glyph_id` that covers
/// `target_extent` (em units); falls back to `(base_glyph_id, natural_scale)` with a computed
/// `scale_y` when the font declares no variant tall enough (or none at all) — the documented
/// glyph-scale fallback in place of full non-linear assembly.
async fn pick_vertical_stretch(font: &Font<'_>, base_glyph_id: u16, target_extent: f32) -> (u16, f32) {
    let target_units = (target_extent * font.units_per_em() as f32) as i64;
    let variants = crate::text::math_stretch_variants(font, base_glyph_id, true);
    if let Some(variant) = variants.iter().find(|v| v.advance as i64 >= target_units) {
        return (variant.glyph_id, 1.0);
    }
    if let Some(largest) = variants.last() {
        let natural = largest.advance.max(1) as f32;
        let scale = (target_units.max(1) as f32 / natural).max(1.0);
        return (largest.glyph_id, scale);
    }
    let natural = font.glyph_bounding_box(base_glyph_id).map_or(font.units_per_em() as f32, |(_, y_min, _, y_max)| (y_max - y_min).max(1) as f32);
    let scale = (target_units.max(1) as f32 / natural).max(1.0);
    (base_glyph_id, scale)
}

/// @emoji 📏️ Places a vertically-stretched glyph so its own (possibly asymmetric — a radical sign
/// is almost all ascent, a paren has more ascent than descent too) bounding box is vertically
/// CENTERED at `center_y`, deriving `height`/`depth` from that real bounding box rather than
/// assuming the glyph splits evenly around its origin. Used for stretchy delimiters and radical
/// signs alike. A first version of this function placed the glyph's origin directly at `center_y`
/// with a naive symmetric height/depth split — visually wrong for every glyph here, since none of
/// them are vertically symmetric around their own origin (a radical's origin sits near its bottom
/// tip, not its middle).
async fn stretched_glyph_box(font: &Font<'_>, font_kind: FontKind, glyph_id: u16, target_extent: f32, center_y: f32) -> MathBox {
    let (variant_id, scale_y) = pick_vertical_stretch(font, glyph_id, target_extent);
    let natural_width = font.glyph_hor_advance(variant_id).map_or(0.5, |a| em(font, a as i32));
    let (_, y_min, _, y_max) = font.glyph_bounding_box(variant_id).unwrap_or((0, 0, 0, font.units_per_em() as i16));
    let glyph_bottom = em(font, y_min as i32) * scale_y;
    let glyph_top = em(font, y_max as i32) * scale_y;
    let glyph_center = (glyph_bottom + glyph_top) / 2.0;
    let origin_y = center_y - glyph_center;
    let height = (origin_y + glyph_top).max(0.0);
    let depth = (-(origin_y + glyph_bottom)).max(0.0);
    MathBox { width: natural_width, height, depth, items: vec![PlacedItem::Glyph { font: font_kind, glyph_id: variant_id, x: 0.0, y: origin_y, scale_y }] }
}
//#endregion 🔖️Stretch

//#region 🔖️Structures
async fn layout_group(fonts: &FontContext<'_>, inner: &MathNode) -> MathBox {
    layout(fonts, inner)
}

async fn layout_paren(fonts: &FontContext<'_>, open: char, inner: &MathNode) -> MathBox {
    let close = if open == '(' { ')' } else { ']' };
    let inner_box = layout(fonts, inner);
    delimited(fonts, open, close, inner_box)
}

/// @emoji 📎️ Wraps `inner` in a matching stretchy delimiter pair, sized to `inner`'s own
/// height+depth, vertically centered on the math axis.
async fn delimited(fonts: &FontContext<'_>, open: char, close: char, inner: MathBox) -> MathBox {
    let axis = crate::text::math_constants(fonts.math).map_or(0.25, |c| mc_em(fonts.math, c.axis_height));
    let target = (inner.height - axis).abs().max((inner.depth + axis).abs()) * 2.0;
    let target = target.max(inner.height + inner.depth).max(0.1);
    let open_id = fonts.math.glyph_index(open).unwrap_or(0);
    let close_id = fonts.math.glyph_index(close).unwrap_or(0);
    let left = stretched_glyph_box(fonts.math, FontKind::Math, open_id, target, axis);
    let right = stretched_glyph_box(fonts.math, FontKind::Math, close_id, target, axis);
    hbox(vec![left, inner, right])
}

async fn layout_call(fonts: &FontContext<'_>, name: &str, rows: &[Vec<MathNode>]) -> MathBox {
    match name {
        "frac" => layout_fraction(fonts, rows),
        "sqrt" => layout_radical(fonts, None, rows.first().and_then(|r| r.first())),
        "root" => layout_radical(fonts, rows.first().and_then(|r| r.first()), rows.first().and_then(|r| r.get(1))),
        "hat" | "bar" | "vec" | "dot" | "ddot" | "tilde" => layout_accent(fonts, name, rows.first().and_then(|r| r.first())),
        "abs" => layout_delimited_single(fonts, '|', '|', rows),
        "norm" => layout_delimited_single(fonts, '‖', '‖', rows),
        "brace" => layout_delimited_single(fonts, '{', '}', rows),
        "mat" => layout_matrix(fonts, rows, Some(('(', ')'))),
        "cases" => layout_cases(fonts, rows),
        _ => layout_generic_call(fonts, name, rows),
    }
}

async fn layout_delimited_single(fonts: &FontContext<'_>, open: char, close: char, rows: &[Vec<MathNode>]) -> MathBox {
    let inner = rows.first().and_then(|r| r.first()).map_or_else(MathBox::empty, |n| layout(fonts, n));
    delimited(fonts, open, close, inner)
}

async fn layout_fraction(fonts: &FontContext<'_>, rows: &[Vec<MathNode>]) -> MathBox {
    let numerator = rows.first().and_then(|r| r.first()).map_or_else(MathBox::empty, |n| layout(fonts, n));
    let denominator = rows.first().and_then(|r| r.get(1)).map_or_else(MathBox::empty, |n| layout(fonts, n));
    let Some(constants) = crate::text::math_constants(fonts.math) else {
        return vstack_centered(numerator, denominator, 0.1, 0.0);
    };
    let axis = mc_em(fonts.math, constants.axis_height);
    let rule_thickness = mc_em(fonts.math, constants.fraction_rule_thickness);
    let num_shift = mc_em(fonts.math, constants.fraction_numerator_shift_up);
    let den_shift = mc_em(fonts.math, constants.fraction_denominator_shift_down);
    let width = numerator.width.max(denominator.width);
    let num_x = (width - numerator.width) / 2.0;
    let den_x = (width - denominator.width) / 2.0;
    let mut out = MathBox::empty();
    out.items.extend(numerator.clone().translated(num_x, num_shift).items);
    out.items.push(PlacedItem::Rule { x: 0.0, y: axis - rule_thickness / 2.0, width, height: rule_thickness });
    out.items.extend(denominator.clone().translated(den_x, -den_shift).items);
    out.width = width;
    out.height = num_shift + numerator.height;
    out.depth = den_shift + denominator.depth;
    out
}

/// @emoji √ Radical: `degree` is `Some` only for `root(degree, radicand)`.
async fn layout_radical(fonts: &FontContext<'_>, degree: Option<&MathNode>, radicand: Option<&MathNode>) -> MathBox {
    let inner = radicand.map_or_else(MathBox::empty, |n| layout(fonts, n));
    let Some(constants) = crate::text::math_constants(fonts.math) else {
        return delimited(fonts, '(', ')', inner); // never reached with the vendored Math font
    };
    let gap = mc_em(fonts.math, constants.radical_vertical_gap);
    let rule_thickness = mc_em(fonts.math, constants.radical_rule_thickness);
    let extra_ascender = mc_em(fonts.math, constants.radical_extra_ascender);
    let target = inner.height + inner.depth + gap + rule_thickness;
    let radical_id = fonts.math.glyph_index('√').unwrap_or(0);
    let axis_y = (inner.height - inner.depth) / 2.0 - gap / 2.0;
    let sign = stretched_glyph_box(fonts.math, FontKind::Math, radical_id, target + extra_ascender, axis_y);
    let mut out = MathBox::empty();
    let mut x = 0.0f32;
    if let Some(degree_node) = degree {
        // `RadicalKernAfterDegree` (Libertinus Math: -0.38em) is tuned for the font's own precise
        // glyph-shape notch — trusting it naively here (a single-em-box approximation of the degree,
        // not the font's true ink-aware kerning model) pulled the sign back far enough to overlap
        // the degree outright. A small fixed positive gap is a robust v1 substitute: never
        // overlapping, at the cost of a slightly wider gap than a fully spec-accurate layout would use.
        const DEGREE_GAP_EM: f32 = 0.05;
        let raise_percent = constants.radical_degree_bottom_raise_percent as f32 / 100.0;
        let degree_box = scale_box(layout(fonts, degree_node), scale_down(constants.script_percent_scale_down));
        // Raised from the baseline toward the radicand's own height — always positive and scales
        // with the radicand, unlike a formula anchored to the independently-computed sign position.
        let degree_y = inner.height * raise_percent;
        out.items.extend(degree_box.clone().translated(x, degree_y).items);
        x += degree_box.width + DEGREE_GAP_EM;
    }
    out.items.extend(sign.clone().translated(x, 0.0).items);
    x += sign.width;
    out.items.push(PlacedItem::Rule { x, y: inner.height + extra_ascender - rule_thickness, width: inner.width, height: rule_thickness });
    out.items.extend(inner.clone().translated(x, 0.0).items);
    out.width = x + inner.width;
    out.height = inner.height + extra_ascender;
    out.depth = inner.depth;
    out
}

/// @emoji ˆ Accent (`hat`/`bar`/`vec`/`dot`/`ddot`/`tilde`): places a combining accent glyph over
/// `base`, centered on `base`'s own `MathTopAccentAttachment` (or its horizontal midpoint).
async fn layout_accent(fonts: &FontContext<'_>, kind: &str, base_node: Option<&MathNode>) -> MathBox {
    let base = base_node.map_or_else(MathBox::empty, |n| layout(fonts, n));
    let accent_ch = match kind {
        "hat" => '\u{0302}',
        "bar" => '\u{0304}',
        "vec" => '\u{20D7}',
        "dot" => '\u{0307}',
        "ddot" => '\u{0308}',
        "tilde" => '\u{0303}',
        _ => return base,
    };
    let Some(accent_id) = fonts.math.glyph_index(accent_ch) else { return base };
    // Combining accents conventionally have ZERO horizontal advance (`glyph_hor_advance`) and a
    // bounding box offset well away from their own origin — e.g. Libertinus Math's `hat` glyph has
    // advance `0` and bbox x in `[-266, -14]` font units. Centering/placing by advance (as an
    // earlier version of this function did) silently produces an invisible-or-mispositioned accent;
    // the glyph's real bounding box is the only correct basis for both axes.
    let (x_min, y_min, x_max, y_max) = fonts.math.glyph_bounding_box(accent_id).unwrap_or((0, 0, 0, 0));
    let accent_width = em(fonts.math, (x_max - x_min) as i32).max(0.05);
    let accent_center_offset = em(fonts.math, x_min as i32 + x_max as i32) / 2.0;
    let base_center = base_node.and_then(|n| base_glyph_id_for_top_accent(fonts, n)).and_then(|gid| crate::text::math_top_accent_attachment(fonts.math, gid)).map_or(base.width / 2.0, |v| em(fonts.math, v as i32));
    let gap = crate::text::math_constants(fonts.math).map_or(0.05, |c| mc_em(fonts.math, c.overbar_vertical_gap));
    let accent_x = base_center - accent_center_offset;
    let accent_bottom_offset = em(fonts.math, y_min as i32);
    let accent_top_offset = em(fonts.math, y_max as i32);
    // The glyph's own origin, not its bbox bottom, is what gets placed — so back out where the
    // origin must sit for the bbox's bottom edge to land exactly `gap` above the base.
    let origin_y = base.height + gap - accent_bottom_offset;
    let mut out = base;
    out.items.push(PlacedItem::Glyph { font: FontKind::Math, glyph_id: accent_id, x: accent_x, y: origin_y, scale_y: 1.0 });
    out.width = out.width.max(accent_x + accent_width);
    out.height = out.height.max(origin_y + accent_top_offset);
    out
}

/// @emoji 🔍️ Best-effort: only single-character `Symbol` bases have a resolvable glyph id for a
/// `MathTopAccentAttachment` lookup; anything else falls back to the horizontal-midpoint default.
async fn base_glyph_id_for_top_accent(fonts: &FontContext<'_>, node: &MathNode) -> Option<u16> {
    match node {
        MathNode::Symbol(name) if name.chars().count() == 1 => fonts.math.glyph_index(name.chars().next().expect("checked count == 1")),
        _ => None,
    }
}

/// @emoji 🔢️ Any `name(...)` call this crate doesn't special-case: render `name` upright, followed
/// by its rows/cells wrapped in stretchy parens, comma/semicolon separated — never a silent drop.
async fn layout_generic_call(fonts: &FontContext<'_>, name: &str, rows: &[Vec<MathNode>]) -> MathBox {
    let label = text_box(fonts.serif, FontKind::Serif, name);
    let comma = text_box(fonts.serif, FontKind::Serif, ", ");
    let semicolon = text_box(fonts.serif, FontKind::Serif, "; ");
    let mut row_boxes = Vec::new();
    for row in rows {
        let mut cells = Vec::new();
        for (i, cell) in row.iter().enumerate() {
            if i > 0 {
                cells.push(comma.clone());
            }
            cells.push(layout(fonts, cell));
        }
        row_boxes.push(hbox(cells));
    }
    let mut args = Vec::new();
    for (i, row) in row_boxes.into_iter().enumerate() {
        if i > 0 {
            args.push(semicolon.clone());
        }
        args.push(row);
    }
    let inner = hbox(args);
    hbox(vec![label, delimited(fonts, '(', ')', inner)])
}

/// @emoji ▦️ A grid of cells (rows × columns), each own-laid-out and column/row aligned by max
/// extent, `GRID_GAP_EM` apart — shared by `mat` (bracketed, optional delimiter pair) and `cases`.
async fn layout_grid(fonts: &FontContext<'_>, rows: &[Vec<MathNode>]) -> MathBox {
    let cell_boxes: Vec<Vec<MathBox>> = rows.iter().map(|row| row.iter().map(|cell| layout(fonts, cell)).collect()).collect();
    let column_count = cell_boxes.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut column_widths = vec![0.0f32; column_count];
    for row in &cell_boxes {
        for (c, cell) in row.iter().enumerate() {
            column_widths[c] = column_widths[c].max(cell.width);
        }
    }
    let mut out = MathBox::empty();
    let mut y = 0.0f32;
    // Lay out top row first, walking downward, then re-center on the vertical midpoint at the end —
    // simpler than tracking a running axis offset while row heights are still unknown.
    let mut row_tops = Vec::with_capacity(cell_boxes.len());
    for row in &cell_boxes {
        let row_height = row.iter().map(|c| c.height).fold(0.0f32, f32::max);
        let row_depth = row.iter().map(|c| c.depth).fold(0.0f32, f32::max);
        row_tops.push((y, row_height, row_depth));
        y += row_height + row_depth + GRID_GAP_EM;
    }
    let total_height = y - GRID_GAP_EM;
    for (row, (top, row_height, _row_depth)) in cell_boxes.iter().zip(row_tops.iter()) {
        let mut x = 0.0f32;
        for (c, cell) in row.iter().enumerate() {
            let cell_x = x + (column_widths[c] - cell.width) / 2.0;
            let baseline_y = total_height / 2.0 - (top + row_height);
            out.items.extend(cell.clone().translated(cell_x, baseline_y).items);
            x += column_widths[c] + GRID_GAP_EM;
        }
        out.width = out.width.max(x - GRID_GAP_EM);
    }
    out.height = total_height / 2.0;
    out.depth = total_height / 2.0;
    out
}

async fn layout_matrix(fonts: &FontContext<'_>, rows: &[Vec<MathNode>], delimiters: Option<(char, char)>) -> MathBox {
    let grid = layout_grid(fonts, rows);
    match delimiters {
        Some((open, close)) => delimited(fonts, open, close, grid),
        None => grid,
    }
}

async fn layout_cases(fonts: &FontContext<'_>, rows: &[Vec<MathNode>]) -> MathBox {
    let grid = layout_grid(fonts, rows);
    let brace_id = fonts.math.glyph_index('{').unwrap_or(0);
    let target = grid.height + grid.depth;
    let brace = stretched_glyph_box(fonts.math, FontKind::Math, brace_id, target, (grid.height - grid.depth) / 2.0);
    let gap = text_box(fonts.serif, FontKind::Serif, " ");
    hbox(vec![brace, gap, grid])
}
//#endregion 🔖️Structures

//#region 🔖️Sequence
async fn spacing_before(op: BinOp) -> f32 {
    match op {
        BinOp::Add | BinOp::Sub | BinOp::Div => THIN_SPACE_EM,
        BinOp::Eq | BinOp::Ne | BinOp::Le | BinOp::Ge | BinOp::Lt | BinOp::Gt | BinOp::Arrow => MEDIUM_SPACE_EM,
    }
}

async fn layout_binop(fonts: &FontContext<'_>, op: BinOp, lhs: &MathNode, rhs: &MathNode) -> MathBox {
    let lhs_box = layout(fonts, lhs);
    let op_box = char_box(fonts.math, FontKind::Math, op_char(op));
    let rhs_box = layout(fonts, rhs);
    let gap = spacing_before(op);
    let spacer = MathBox { width: gap, ..MathBox::empty() };
    hbox(vec![lhs_box, spacer.clone(), op_box, spacer, rhs_box])
}

async fn op_char(op: BinOp) -> char {
    match op {
        BinOp::Add => '+',
        BinOp::Sub => '−',
        BinOp::Div => '/',
        BinOp::Eq => '=',
        BinOp::Ne => '≠',
        BinOp::Le => '≤',
        BinOp::Ge => '≥',
        BinOp::Lt => '<',
        BinOp::Gt => '>',
        BinOp::Arrow => '→',
    }
}

async fn layout_sequence(fonts: &FontContext<'_>, items: &[crate::syntax::SeqItem]) -> MathBox {
    let mut children = Vec::with_capacity(items.len() * 2);
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            if item.dot {
                let spacer = MathBox { width: THIN_SPACE_EM, ..MathBox::empty() };
                children.push(spacer.clone());
                children.push(char_box(fonts.math, FontKind::Math, '⋅'));
                children.push(spacer);
            } else {
                children.push(MathBox { width: THIN_SPACE_EM * 0.6, ..MathBox::empty() });
            }
        }
        children.push(layout(fonts, &item.node));
    }
    hbox(children)
}
//#endregion 🔖️Sequence

//#region 🔖️Entry
/// @emoji 🎯️ Lays out any [`MathNode`] into a [`MathBox`] — the crate's main entry point, called
/// recursively by every structure above for sub-expressions.
pub async fn layout(fonts: &FontContext<'_>, node: &MathNode) -> MathBox {
    match node {
        MathNode::Number(text) => number_box(fonts, text),
        MathNode::Symbol(name) => symbol_box(fonts, name),
        MathNode::Emoji(name) => emoji_box(fonts, name),
        MathNode::Text(text) => quoted_text_box(fonts, text),
        MathNode::Group(inner) => layout_group(fonts, inner),
        MathNode::Paren(open, inner) => layout_paren(fonts, *open, inner),
        MathNode::Sup(base, exponent) => layout_scripted(fonts, layout(fonts, base), layout(fonts, exponent), true),
        MathNode::Sub(base, subscript) => layout_scripted(fonts, layout(fonts, base), layout(fonts, subscript), false),
        MathNode::Call(name, rows) => layout_call(fonts, name, rows),
        MathNode::BinOp(op, lhs, rhs) => layout_binop(fonts, *op, lhs, rhs),
        MathNode::Sequence(items) => layout_sequence(fonts, items),
    }
}
//#endregion 🔖️Entry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::parse_formula;

    async fn with_fonts<R>(f: impl FnOnce(&FontContext<'_>) -> R) -> R {
        let fonts = crate::world::embedded_fonts();
        let math = Font::from_bytes(fonts.math, 0).expect("parse math font");
        let serif = Font::from_bytes(fonts.serif, 0).expect("parse serif font");
        let mono = Font::from_bytes(fonts.mono, 0).expect("parse mono font");
        let emoji = Font::from_bytes(fonts.emoji, 0).expect("parse emoji font");
        let ctx = FontContext { math: &math, serif: &serif, mono: &mono, emoji: &emoji };
        f(&ctx)
    }

    async fn layout_src(fonts: &FontContext<'_>, src: &str) -> MathBox {
        let node = parse_formula(src).unwrap_or_else(|e| panic!("parse {src:?} failed: {e}"));
        layout(fonts, &node)
    }

    #[test]
    async fn simple_symbol_has_positive_width_and_at_least_one_glyph() {
        with_fonts(|fonts| {
            let node = MathNode::Symbol("x".to_string());
            let box_ = layout(fonts, &node);
            assert!(box_.width > 0.0);
            assert!(!box_.items.is_empty());
        });
    }

    #[test]
    async fn superscript_is_narrower_and_shifted_above_the_baseline() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, "x^2");
            assert!(box_.width > 0.0);
            assert!(box_.height > 0.0, "a superscript must raise the box's height");
            // Two items: the base glyph and the (smaller, higher) exponent glyph.
            assert_eq!(box_.items.len(), 2);
        });
    }

    #[test]
    async fn subscript_extends_the_depth_not_the_height() {
        with_fonts(|fonts| {
            let base_only = layout_src(fonts, "x");
            let subscripted = layout_src(fonts, "x_1");
            assert!(subscripted.depth > base_only.depth, "a subscript must extend depth: {} vs {}", subscripted.depth, base_only.depth);
        });
    }

    #[test]
    async fn fraction_stacks_numerator_over_denominator_around_the_axis() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, "frac(a, b)");
            assert!(box_.height > 0.0 && box_.depth > 0.0, "a fraction must have both height and depth: {box_:?}");
            let rule_count = box_.items.iter().filter(|item| matches!(item, PlacedItem::Rule { .. })).count();
            assert_eq!(rule_count, 1, "a fraction must draw exactly one rule");
        });
    }

    #[test]
    async fn sqrt_draws_a_radical_sign_and_a_top_rule() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, "sqrt(x)");
            let rule_count = box_.items.iter().filter(|item| matches!(item, PlacedItem::Rule { .. })).count();
            assert_eq!(rule_count, 1, "a radical must draw exactly one top rule");
            assert!(box_.width > layout_src(fonts, "x").width, "the radical sign must add width beyond the radicand alone");
        });
    }

    #[test]
    async fn matrix_lays_out_a_grid_wrapped_in_parens() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, "mat(1, 2; 3, 4)");
            // 4 number glyphs + 2 stretchy paren glyphs.
            assert_eq!(box_.items.iter().filter(|i| matches!(i, PlacedItem::Glyph { .. })).count(), 6);
        });
    }

    #[test]
    async fn stretchy_parens_are_taller_for_taller_content() {
        with_fonts(|fonts| {
            let short = layout_src(fonts, "(x)");
            let tall = layout_src(fonts, "(frac(a, b))");
            assert!(tall.height + tall.depth > short.height + short.depth, "parens around a fraction must be taller than parens around a bare symbol");
        });
    }

    #[test]
    async fn emoji_shortcode_places_an_image_item() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, ":rocket:");
            assert_eq!(box_.items.len(), 1);
            assert!(matches!(box_.items[0], PlacedItem::Image { .. }), "known shortcode must render as an image, got {:?}", box_.items[0]);
        });
    }

    #[test]
    async fn unknown_emoji_shortcode_falls_back_to_visible_text_not_a_blank_box() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, ":not-a-real-shortcode:");
            assert!(!box_.items.is_empty(), "an unresolved shortcode must still render something visible");
            assert!(box_.items.iter().all(|i| matches!(i, PlacedItem::Glyph { .. })), "fallback must be text glyphs: {:?}", box_.items);
        });
    }

    #[test]
    async fn binary_operator_inserts_visible_spacing_between_operands() {
        with_fonts(|fonts| {
            let plain_sum = hbox(vec![layout_src(fonts, "x"), layout_src(fonts, "y")]);
            let spaced_sum = layout_src(fonts, "x + y");
            assert!(spaced_sum.width > plain_sum.width, "an explicit `+` must take more width than bare juxtaposition");
        });
    }

    #[test]
    async fn accent_adds_height_above_the_base() {
        with_fonts(|fonts| {
            let base = layout_src(fonts, "x");
            let accented = layout_src(fonts, "hat(x)");
            assert!(accented.height > base.height, "an accent must raise the box's height above the bare base");
        });
    }

    #[test]
    async fn unknown_call_name_renders_the_name_and_wrapped_args_rather_than_silently_dropping() {
        with_fonts(|fonts| {
            let box_ = layout_src(fonts, "mystery(x, y)");
            assert!(!box_.items.is_empty());
            assert!(box_.items.len() >= 4, "expected the label glyphs plus at least two stretchy delimiters plus content: {}", box_.items.len());
        });
    }

    #[test]
    async fn layout_raw_text_shapes_arbitrary_strings_without_parsing_notation_syntax() {
        with_fonts(|fonts| {
            // `_ ; < > !` are all special characters in math notation — a raw-text caller must be
            // able to include them literally, which `layout_raw_text` (no parser involved) allows.
            let box_ = layout_raw_text(fonts, "a_b; c<d!");
            assert!(box_.width > 0.0);
            assert!(!box_.items.is_empty());
        });
    }

    #[test]
    async fn layout_raw_emoji_places_a_raster_image_for_a_known_glyph() {
        with_fonts(|fonts| {
            let box_ = layout_raw_emoji(fonts, "🚀");
            assert_eq!(box_.items.len(), 1);
            assert!(matches!(box_.items[0], PlacedItem::Image { .. }));
        });
    }

    #[test]
    async fn layout_raw_code_shapes_via_the_mono_font() {
        with_fonts(|fonts| {
            let box_ = layout_raw_code(fonts, "fn main() {}");
            assert!(box_.width > 0.0);
            assert!(box_.items.iter().all(|i| matches!(i, PlacedItem::Glyph { font: FontKind::Mono, .. })));
        });
    }
}
//#endregion 🧪️Tests
