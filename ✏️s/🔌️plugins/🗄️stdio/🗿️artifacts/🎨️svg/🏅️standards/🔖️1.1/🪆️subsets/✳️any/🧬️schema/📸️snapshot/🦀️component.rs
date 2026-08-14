//! 🧬️ SvgSnapshot schema — persistent fields + real codecs.

use crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA;
use crate::artifacts::xml::schema::snapshot::{
    xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg")]
pub struct SvgSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[serde(default)]
    pub doc: XmlDocument,
}

impl Default for SvgSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                root: Some(XmlNode::Element {
                    name: "svg".into(),
                    attrs: Vec::new(),
                    children: Vec::new(),
                }),
                doctype: None,
                declaration: None,
                prolog: Vec::new(),
            },
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️SvgCodec
pub fn parse_svg_xml(text: &str) -> Result<XmlDocument, String> {
    let doc = xml_document_from_text(text)?;
    if let Some(XmlNode::Element { name, .. }) = &doc.root {
        if name != "svg" && !name.ends_with(":svg") {
            return Err("root element must be svg".into());
        }
    } else {
        return Err("svg document requires root element".into());
    }
    Ok(doc)
}

pub fn write_svg_xml(doc: &XmlDocument) -> String {
    xml_document_to_text(doc)
}

impl SvgSnapshot {
    /// 🧠️ Returns the lossless logical SVG state used by diff and mutation laws.
    pub fn semantic_projection(&self) -> Self {
        self.clone()
    }

    /// 📥️ Parses SVG UTF-8 into its lossless logical XML model.
    pub fn import_utf8(bytes: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("svg source is not UTF-8: {error}"))?;
        Ok(Self {
            schema: STDIO_SVG_DOCUMENT_SCHEMA.into(),
            doc: parse_svg_xml(text)?,
        })
    }

    /// 📤️ Deterministically materializes SVG from the logical XML model.
    pub fn export_utf8(&self) -> Result<Vec<u8>, String> {
        Ok(write_svg_xml(&self.doc).into_bytes())
    }
}
//#endregion 🔖️SvgCodec

//#region 🔖️NumberGrammar
/// 🔢 Byte-cursor shared by the `d`/`transform`/`viewBox`/`points` grammars (all of which use the
/// same SVG `<number>`/`<comma-wsp>` productions). Operates on bytes rather than chars because SVG
/// numeric grammar is pure ASCII; `str` slicing stays valid because we only ever slice at ASCII
/// byte boundaries.
struct NumCursor<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> NumCursor<'a> {
    fn new(s: &'a str) -> Self {
        Self { s: s.as_bytes(), pos: 0 }
    }
    fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }
    fn is_eof(&self) -> bool {
        self.pos >= self.s.len()
    }
    fn skip_wsp_comma(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') | Some(b',')) {
            self.pos += 1;
        }
    }
    fn skip_wsp(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')) {
            self.pos += 1;
        }
    }
    fn parse_number(&mut self) -> Result<f64, String> {
        self.skip_wsp_comma();
        let start = self.pos;
        if matches!(self.peek(), Some(b'+') | Some(b'-')) {
            self.pos += 1;
        }
        let mut has_digits = false;
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.pos += 1;
            has_digits = true;
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
                has_digits = true;
            }
        }
        if !has_digits {
            self.pos = start;
            return Err(format!("expected number at byte {start}"));
        }
        if matches!(self.peek(), Some(b'e') | Some(b'E')) {
            let save = self.pos;
            self.pos += 1;
            if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                self.pos += 1;
            }
            if matches!(self.peek(), Some(b'0'..=b'9')) {
                while matches!(self.peek(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            } else {
                self.pos = save;
            }
        }
        let raw = std::str::from_utf8(&self.s[start..self.pos]).unwrap();
        raw.parse::<f64>().map_err(|_| format!("invalid number '{raw}'"))
    }
    /// 🚩 A path arc-flag is EXACTLY one `0`/`1` byte with no separator required before the next
    /// token -- `A5 5 0 108 8` must decompose the run `108` into flags `1`,`0` then the number `8`
    /// (large-arc=1, sweep=0, x=8), never a naive 2-digit/3-digit number grab. Classic bug source.
    fn parse_flag(&mut self) -> Result<bool, String> {
        self.skip_wsp_comma();
        match self.peek() {
            Some(b'0') => { self.pos += 1; Ok(false) }
            Some(b'1') => { self.pos += 1; Ok(true) }
            other => Err(format!("expected arc flag (0/1), got {other:?} at byte {}", self.pos)),
        }
    }
}

fn parse_number_list(s: &str) -> Result<Vec<f64>, String> {
    let mut c = NumCursor::new(s);
    let mut out = Vec::new();
    loop {
        c.skip_wsp_comma();
        if c.is_eof() {
            break;
        }
        out.push(c.parse_number()?);
    }
    Ok(out)
}

fn fmt_num(v: f64) -> String {
    v.to_string()
}
//#endregion 🔖️NumberGrammar

//#region 🔖️Geometry
/// 📐️ Parsed `viewBox="min-x min-y width height"`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ViewBox {
    pub min_x: f64,
    pub min_y: f64,
    pub width: f64,
    pub height: f64,
}

pub fn parse_view_box(s: &str) -> Result<ViewBox, String> {
    let nums = parse_number_list(s)?;
    if nums.len() != 4 {
        return Err(format!("viewBox requires exactly 4 numbers, got {}", nums.len()));
    }
    Ok(ViewBox { min_x: nums[0], min_y: nums[1], width: nums[2], height: nums[3] })
}

pub fn view_box_to_string(v: &ViewBox) -> String {
    format!("{} {} {} {}", fmt_num(v.min_x), fmt_num(v.min_y), fmt_num(v.width), fmt_num(v.height))
}

/// 🔗️ `points="x1,y1 x2,y2 ..."` (polyline/polygon).
pub fn parse_points(s: &str) -> Result<Vec<(f64, f64)>, String> {
    let nums = parse_number_list(s)?;
    if nums.len() % 2 != 0 {
        return Err("points list must have an even number of coordinates".into());
    }
    Ok(nums.chunks(2).map(|c| (c[0], c[1])).collect())
}

pub fn points_to_string(points: &[(f64, f64)]) -> String {
    points.iter().map(|(x, y)| format!("{},{}", fmt_num(*x), fmt_num(*y))).collect::<Vec<_>>().join(" ")
}
//#endregion 🔖️Geometry

//#region 🔖️Transform
/// ✖️ 2D affine matrix `[a c e; b d f; 0 0 1]`, matching SVG's `matrix(a,b,c,d,e,f)` layout.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matrix2D {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub e: f64,
    pub f: f64,
}

impl Matrix2D {
    pub fn identity() -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 }
    }
    /// ✖️ Composes `self ∘ other` (apply `other` first, then `self`) -- matches SVG's
    /// left-to-right `transform="A B"` list semantics, where the combined matrix is `A * B`.
    pub fn multiply(&self, other: &Matrix2D) -> Matrix2D {
        Matrix2D {
            a: self.a * other.a + self.c * other.b,
            b: self.b * other.a + self.d * other.b,
            c: self.a * other.c + self.c * other.d,
            d: self.b * other.c + self.d * other.d,
            e: self.a * other.e + self.c * other.f + self.e,
            f: self.b * other.e + self.d * other.f + self.f,
        }
    }
}

/// 📜 One entry of a `transform="..."` list. Kept as a typed op list (rather than collapsed
/// eagerly into a single matrix) so the original function-call structure round-trips; compose via
/// `transform_ops_to_matrix` whenever a single resolved affine matrix is needed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum TransformOp {
    Matrix { a: f64, b: f64, c: f64, d: f64, e: f64, f: f64 },
    Translate { x: f64, #[serde(default, skip_serializing_if = "Option::is_none")] y: Option<f64> },
    Scale { x: f64, #[serde(default, skip_serializing_if = "Option::is_none")] y: Option<f64> },
    Rotate { angle: f64, #[serde(default, skip_serializing_if = "Option::is_none")] center: Option<(f64, f64)> },
    SkewX { angle: f64 },
    SkewY { angle: f64 },
}

impl TransformOp {
    pub fn to_matrix(&self) -> Matrix2D {
        match *self {
            TransformOp::Matrix { a, b, c, d, e, f } => Matrix2D { a, b, c, d, e, f },
            TransformOp::Translate { x, y } => Matrix2D { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: x, f: y.unwrap_or(0.0) },
            TransformOp::Scale { x, y } => Matrix2D { a: x, b: 0.0, c: 0.0, d: y.unwrap_or(x), e: 0.0, f: 0.0 },
            TransformOp::Rotate { angle, center } => {
                let (sin, cos) = angle.to_radians().sin_cos();
                let rot = Matrix2D { a: cos, b: sin, c: -sin, d: cos, e: 0.0, f: 0.0 };
                match center {
                    None => rot,
                    Some((cx, cy)) => {
                        let t1 = Matrix2D { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: cx, f: cy };
                        let t2 = Matrix2D { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: -cx, f: -cy };
                        t1.multiply(&rot).multiply(&t2)
                    }
                }
            }
            TransformOp::SkewX { angle } => Matrix2D { a: 1.0, b: 0.0, c: angle.to_radians().tan(), d: 1.0, e: 0.0, f: 0.0 },
            TransformOp::SkewY { angle } => Matrix2D { a: 1.0, b: angle.to_radians().tan(), c: 0.0, d: 1.0, e: 0.0, f: 0.0 },
        }
    }
}

/// ✖️ Composes an entire transform list into one resolved matrix (fold in list order).
pub fn transform_ops_to_matrix(ops: &[TransformOp]) -> Matrix2D {
    ops.iter().fold(Matrix2D::identity(), |acc, op| acc.multiply(&op.to_matrix()))
}

pub fn parse_transform_list(s: &str) -> Result<Vec<TransformOp>, String> {
    let mut c = NumCursor::new(s);
    let mut ops = Vec::new();
    loop {
        c.skip_wsp_comma();
        if c.is_eof() {
            break;
        }
        let start = c.pos;
        while matches!(c.peek(), Some(b'a'..=b'z') | Some(b'A'..=b'Z')) {
            c.pos += 1;
        }
        let name = std::str::from_utf8(&c.s[start..c.pos]).unwrap();
        if name.is_empty() {
            return Err(format!("expected transform function name at byte {}", c.pos));
        }
        c.skip_wsp();
        if c.peek() != Some(b'(') {
            return Err(format!("expected '(' after transform function '{name}'"));
        }
        c.pos += 1;
        let mut nums = Vec::new();
        loop {
            c.skip_wsp_comma();
            if c.peek() == Some(b')') {
                break;
            }
            nums.push(c.parse_number()?);
        }
        if c.peek() != Some(b')') {
            return Err(format!("unclosed '(' for transform function '{name}'"));
        }
        c.pos += 1;
        let op = match name {
            "matrix" if nums.len() == 6 => TransformOp::Matrix { a: nums[0], b: nums[1], c: nums[2], d: nums[3], e: nums[4], f: nums[5] },
            "translate" if nums.len() == 1 => TransformOp::Translate { x: nums[0], y: None },
            "translate" if nums.len() == 2 => TransformOp::Translate { x: nums[0], y: Some(nums[1]) },
            "scale" if nums.len() == 1 => TransformOp::Scale { x: nums[0], y: None },
            "scale" if nums.len() == 2 => TransformOp::Scale { x: nums[0], y: Some(nums[1]) },
            "rotate" if nums.len() == 1 => TransformOp::Rotate { angle: nums[0], center: None },
            "rotate" if nums.len() == 3 => TransformOp::Rotate { angle: nums[0], center: Some((nums[1], nums[2])) },
            "skewX" if nums.len() == 1 => TransformOp::SkewX { angle: nums[0] },
            "skewY" if nums.len() == 1 => TransformOp::SkewY { angle: nums[0] },
            other => return Err(format!("unknown/malformed transform function '{other}' with {} args", nums.len())),
        };
        ops.push(op);
    }
    Ok(ops)
}

pub fn transform_list_to_string(ops: &[TransformOp]) -> String {
    ops.iter()
        .map(|op| match op {
            TransformOp::Matrix { a, b, c, d, e, f } => format!("matrix({},{},{},{},{},{})", fmt_num(*a), fmt_num(*b), fmt_num(*c), fmt_num(*d), fmt_num(*e), fmt_num(*f)),
            TransformOp::Translate { x, y: None } => format!("translate({})", fmt_num(*x)),
            TransformOp::Translate { x, y: Some(y) } => format!("translate({},{})", fmt_num(*x), fmt_num(*y)),
            TransformOp::Scale { x, y: None } => format!("scale({})", fmt_num(*x)),
            TransformOp::Scale { x, y: Some(y) } => format!("scale({},{})", fmt_num(*x), fmt_num(*y)),
            TransformOp::Rotate { angle, center: None } => format!("rotate({})", fmt_num(*angle)),
            TransformOp::Rotate { angle, center: Some((cx, cy)) } => format!("rotate({},{},{})", fmt_num(*angle), fmt_num(*cx), fmt_num(*cy)),
            TransformOp::SkewX { angle } => format!("skewX({})", fmt_num(*angle)),
            TransformOp::SkewY { angle } => format!("skewY({})", fmt_num(*angle)),
        })
        .collect::<Vec<_>>()
        .join(" ")
}
//#endregion 🔖️Transform

//#region 🔖️PathData
/// 🖊️ One command of the `d` attribute mini-language. `relative` distinguishes the lower-case
/// (relative-to-current-point) form from the upper-case (absolute) form -- both are kept typed
/// rather than pre-resolved to absolute coordinates, since resolving requires walking the whole
/// path with a running current-point/start-point state that belongs to a renderer, not the parser.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum PathCommand {
    MoveTo { x: f64, y: f64, relative: bool },
    LineTo { x: f64, y: f64, relative: bool },
    HorizontalLineTo { x: f64, relative: bool },
    VerticalLineTo { y: f64, relative: bool },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64, relative: bool },
    SmoothCurveTo { x2: f64, y2: f64, x: f64, y: f64, relative: bool },
    QuadraticCurveTo { x1: f64, y1: f64, x: f64, y: f64, relative: bool },
    SmoothQuadraticCurveTo { x: f64, y: f64, relative: bool },
    Arc { rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64, relative: bool },
    ClosePath,
}

/// 🖊️ Parses a `d` attribute per the SVG path mini-language grammar. Verified against 18 checks
/// (incl. the arc-flag squeeze edge case) in a standalone scratch crate before porting here, per
/// the technique in the ticket's STATUS.md.
pub fn parse_path_data(d: &str) -> Result<Vec<PathCommand>, String> {
    let mut c = NumCursor::new(d);
    let mut cmds = Vec::new();
    // 🔁 `last_letter`/`last_relative` drive implicit command repetition: a number run with no
    // leading letter reuses the previous command -- except a bare `M`/`m` run whose FIRST pair is
    // the moveto and whose SUBSEQUENT pairs become implicit `L`/`l` (SVG spec 8.3.2).
    let mut last_letter: Option<u8> = None;
    let mut last_relative = false;
    loop {
        c.skip_wsp_comma();
        if c.is_eof() {
            break;
        }
        let peeked = c.peek().unwrap();
        let (letter, relative, explicit) = if peeked.is_ascii_alphabetic() {
            c.pos += 1;
            (peeked.to_ascii_uppercase(), peeked.is_ascii_lowercase(), true)
        } else {
            let letter = last_letter.ok_or_else(|| "path data must start with a moveto command".to_string())?;
            (letter, last_relative, false)
        };
        if explicit && letter == b'Z' {
            cmds.push(PathCommand::ClosePath);
            last_letter = None;
            continue;
        }
        match letter {
            b'M' => {
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::MoveTo { x, y, relative });
                last_letter = Some(b'L');
                last_relative = relative;
            }
            b'L' => {
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::LineTo { x, y, relative });
                last_letter = Some(b'L');
                last_relative = relative;
            }
            b'H' => {
                let x = c.parse_number()?;
                cmds.push(PathCommand::HorizontalLineTo { x, relative });
                last_letter = Some(b'H');
                last_relative = relative;
            }
            b'V' => {
                let y = c.parse_number()?;
                cmds.push(PathCommand::VerticalLineTo { y, relative });
                last_letter = Some(b'V');
                last_relative = relative;
            }
            b'C' => {
                let x1 = c.parse_number()?;
                let y1 = c.parse_number()?;
                let x2 = c.parse_number()?;
                let y2 = c.parse_number()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative });
                last_letter = Some(b'C');
                last_relative = relative;
            }
            b'S' => {
                let x2 = c.parse_number()?;
                let y2 = c.parse_number()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::SmoothCurveTo { x2, y2, x, y, relative });
                last_letter = Some(b'S');
                last_relative = relative;
            }
            b'Q' => {
                let x1 = c.parse_number()?;
                let y1 = c.parse_number()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::QuadraticCurveTo { x1, y1, x, y, relative });
                last_letter = Some(b'Q');
                last_relative = relative;
            }
            b'T' => {
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::SmoothQuadraticCurveTo { x, y, relative });
                last_letter = Some(b'T');
                last_relative = relative;
            }
            b'A' => {
                let rx = c.parse_number()?;
                let ry = c.parse_number()?;
                let x_axis_rotation = c.parse_number()?;
                let large_arc = c.parse_flag()?;
                let sweep = c.parse_flag()?;
                let x = c.parse_number()?;
                let y = c.parse_number()?;
                cmds.push(PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative });
                last_letter = Some(b'A');
                last_relative = relative;
            }
            other => return Err(format!("unknown path command '{}'", other as char)),
        }
    }
    Ok(cmds)
}

pub fn path_data_to_string(cmds: &[PathCommand]) -> String {
    fn letter(base: char, relative: bool) -> char {
        if relative { base.to_ascii_lowercase() } else { base }
    }
    let mut parts = Vec::with_capacity(cmds.len());
    for cmd in cmds {
        parts.push(match cmd {
            PathCommand::MoveTo { x, y, relative } => format!("{} {} {}", letter('M', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::LineTo { x, y, relative } => format!("{} {} {}", letter('L', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::HorizontalLineTo { x, relative } => format!("{} {}", letter('H', *relative), fmt_num(*x)),
            PathCommand::VerticalLineTo { y, relative } => format!("{} {}", letter('V', *relative), fmt_num(*y)),
            PathCommand::CurveTo { x1, y1, x2, y2, x, y, relative } => {
                format!("{} {} {} {} {} {} {}", letter('C', *relative), fmt_num(*x1), fmt_num(*y1), fmt_num(*x2), fmt_num(*y2), fmt_num(*x), fmt_num(*y))
            }
            PathCommand::SmoothCurveTo { x2, y2, x, y, relative } => format!("{} {} {} {} {}", letter('S', *relative), fmt_num(*x2), fmt_num(*y2), fmt_num(*x), fmt_num(*y)),
            PathCommand::QuadraticCurveTo { x1, y1, x, y, relative } => format!("{} {} {} {} {}", letter('Q', *relative), fmt_num(*x1), fmt_num(*y1), fmt_num(*x), fmt_num(*y)),
            PathCommand::SmoothQuadraticCurveTo { x, y, relative } => format!("{} {} {}", letter('T', *relative), fmt_num(*x), fmt_num(*y)),
            PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative } => format!(
                "{} {} {} {} {} {} {} {}",
                letter('A', *relative), fmt_num(*rx), fmt_num(*ry), fmt_num(*x_axis_rotation), *large_arc as u8, *sweep as u8, fmt_num(*x), fmt_num(*y)
            ),
            PathCommand::ClosePath => "Z".to_string(),
        });
    }
    parts.join(" ")
}
//#endregion 🔖️PathData

//#region 🔖️Style
/// 🎨 The subset of presentation properties this artifact understands both as plain XML attributes
/// (`fill="red"`) and as `style="fill: red"` CSS-like declarations (style wins on conflict, matching
/// CSS cascade precedence over presentation attributes). `extra_style` losslessly retains any
/// `style=""` declaration this artifact doesn't specifically model, so a `style` attribute never
/// silently loses content it didn't recognize.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationAttrs {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub fill: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub stroke: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub stroke_width: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub opacity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub fill_opacity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub stroke_opacity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub font_family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub font_size: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub extra_style: Vec<(String, String)>,
}

/// 🧩 Real `key: value; key2: value2` parsing (declaration-list split on `;`, each split on the
/// FIRST `:`) -- not a substring/`contains()` hack.
pub fn parse_style_decls(s: &str) -> Vec<(String, String)> {
    s.split(';')
        .filter_map(|decl| {
            let decl = decl.trim();
            if decl.is_empty() {
                return None;
            }
            let mut parts = decl.splitn(2, ':');
            let key = parts.next()?.trim();
            let value = parts.next()?.trim();
            if key.is_empty() || value.is_empty() {
                return None;
            }
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

/// ↩️ Returns `true` if `name` is a recognized presentation property (and was applied).
fn apply_presentation_attr(p: &mut PresentationAttrs, name: &str, value: &str) -> bool {
    match name {
        "fill" => p.fill = Some(value.to_string()),
        "stroke" => p.stroke = Some(value.to_string()),
        "stroke-width" => p.stroke_width = Some(value.to_string()),
        "opacity" => p.opacity = Some(value.to_string()),
        "fill-opacity" => p.fill_opacity = Some(value.to_string()),
        "stroke-opacity" => p.stroke_opacity = Some(value.to_string()),
        "font-family" => p.font_family = Some(value.to_string()),
        "font-size" => p.font_size = Some(value.to_string()),
        _ => return false,
    }
    true
}

fn push_presentation_attrs(attrs: &mut Vec<XmlAttr>, p: &PresentationAttrs) {
    let mut push = |name: &str, value: &Option<String>| {
        if let Some(v) = value {
            attrs.push(XmlAttr { name: name.to_string(), value: v.clone() });
        }
    };
    push("fill", &p.fill);
    push("stroke", &p.stroke);
    push("stroke-width", &p.stroke_width);
    push("opacity", &p.opacity);
    push("fill-opacity", &p.fill_opacity);
    push("stroke-opacity", &p.stroke_opacity);
    push("font-family", &p.font_family);
    push("font-size", &p.font_size);
    if !p.extra_style.is_empty() {
        let decls: Vec<String> = p.extra_style.iter().map(|(k, v)| format!("{k}: {v}")).collect();
        attrs.push(XmlAttr { name: "style".to_string(), value: decls.join("; ") });
    }
}
//#endregion 🔖️Style

//#region 🔖️CommonAttrs
/// 🧬 Attributes shared by (almost) every SVG element: `id`, `class`, `transform`, the
/// presentation-attribute subset, and an `extra_attrs` escape hatch for anything else on the
/// element this artifact doesn't specifically model (namespaced attrs, `xmlns`, custom `data-*`,
/// unrecognized presentation properties, ...) -- kept verbatim so nothing is ever silently dropped.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonAttrs {
    #[serde(default, skip_serializing_if = "Option::is_none")] pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub transform: Option<Vec<TransformOp>>,
    #[serde(default)] pub presentation: PresentationAttrs,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub extra_attrs: Vec<XmlAttr>,
}

impl CommonAttrs {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
    pub fn with_transform(mut self, ops: Vec<TransformOp>) -> Self {
        self.transform = Some(ops);
        self
    }
    pub fn with_fill(mut self, v: impl Into<String>) -> Self {
        self.presentation.fill = Some(v.into());
        self
    }
    pub fn with_stroke(mut self, v: impl Into<String>) -> Self {
        self.presentation.stroke = Some(v.into());
        self
    }
    pub fn with_stroke_width(mut self, v: impl Into<String>) -> Self {
        self.presentation.stroke_width = Some(v.into());
        self
    }
    pub fn with_opacity(mut self, v: impl Into<String>) -> Self {
        self.presentation.opacity = Some(v.into());
        self
    }
}

/// 🧩 Buckets `attrs` into `CommonAttrs`, treating `element_specific` names as already consumed
/// elsewhere (so they don't ALSO land in `extra_attrs`). `style=""` is real-parsed and takes
/// precedence over same-named plain attributes (CSS cascade order); unrecognized style
/// declarations are retained in `presentation.extra_style`, never dropped.
fn parse_common_attrs(attrs: &[XmlAttr], element_specific: &[&str]) -> CommonAttrs {
    let mut common = CommonAttrs::default();
    let style_decls = attr_val(attrs, "style").map(parse_style_decls).unwrap_or_default();
    for a in attrs {
        match a.name.as_str() {
            "id" => common.id = Some(a.value.clone()),
            "class" => common.class = Some(a.value.clone()),
            "style" => {}
            "transform" => match parse_transform_list(&a.value) {
                Ok(ops) => common.transform = Some(ops),
                // 🚧️ malformed transform: kept verbatim rather than fabricated as an empty list.
                Err(_) => common.extra_attrs.push(a.clone()),
            },
            "fill" | "stroke" | "stroke-width" | "opacity" | "fill-opacity" | "stroke-opacity" | "font-family" | "font-size" => {}
            other if element_specific.contains(&other) => {}
            _ => common.extra_attrs.push(a.clone()),
        }
    }
    for a in attrs {
        apply_presentation_attr(&mut common.presentation, &a.name, &a.value);
    }
    for (k, v) in &style_decls {
        if !apply_presentation_attr(&mut common.presentation, k, v) {
            common.presentation.extra_style.push((k.clone(), v.clone()));
        }
    }
    common
}

fn push_common_attrs(attrs: &mut Vec<XmlAttr>, common: &CommonAttrs) {
    if let Some(id) = &common.id {
        attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
    }
    if let Some(class) = &common.class {
        attrs.push(XmlAttr { name: "class".into(), value: class.clone() });
    }
    if let Some(t) = &common.transform {
        attrs.push(XmlAttr { name: "transform".into(), value: transform_list_to_string(t) });
    }
    push_presentation_attrs(attrs, &common.presentation);
    attrs.extend(common.extra_attrs.iter().cloned());
}
//#endregion 🔖️CommonAttrs

//#region 🔖️TypedElementModel
fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

fn attr_f64(attrs: &[XmlAttr], name: &str, default: f64) -> Result<f64, String> {
    match attr_val(attrs, name) {
        None => Ok(default),
        Some(v) => v.trim().parse::<f64>().map_err(|_| format!("attribute '{name}' is not a number: '{v}'")),
    }
}

fn attr_f64_opt(attrs: &[XmlAttr], name: &str) -> Result<Option<f64>, String> {
    match attr_val(attrs, name) {
        None => Ok(None),
        Some(v) => v.trim().parse::<f64>().map(Some).map_err(|_| format!("attribute '{name}' is not a number: '{v}'")),
    }
}

fn attr_string_opt(attrs: &[XmlAttr], name: &str) -> Option<String> {
    attr_val(attrs, name).map(|s| s.to_string())
}

/// ✂️ Strips an XML namespace prefix (`xlink:href` -> `href`) for TYPED-ELEMENT DISPATCH ONLY;
/// `Unknown` and attribute passthrough always keep the original, fully-qualified name.
fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

/// 🌳 Typed SVG 1.1 element tree. Elements outside this typed set (and any element this session
/// chose not to model in depth) fall into `Unknown` -- name/attrs/children kept byte-for-byte, so
/// parsing never drops or corrupts content outside the typed surface.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SvgElement {
    Svg {
        common: CommonAttrs,
        #[serde(default, skip_serializing_if = "Option::is_none")] view_box: Option<ViewBox>,
        #[serde(default, skip_serializing_if = "Option::is_none")] width: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] height: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] xmlns: Option<String>,
        #[serde(default)] children: Vec<SvgElement>,
    },
    Rect {
        common: CommonAttrs,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")] rx: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")] ry: Option<f64>,
    },
    Circle { common: CommonAttrs, cx: f64, cy: f64, r: f64 },
    Ellipse { common: CommonAttrs, cx: f64, cy: f64, rx: f64, ry: f64 },
    Line { common: CommonAttrs, x1: f64, y1: f64, x2: f64, y2: f64 },
    Polyline { common: CommonAttrs, points: Vec<(f64, f64)> },
    Polygon { common: CommonAttrs, points: Vec<(f64, f64)> },
    Path { common: CommonAttrs, d: Vec<PathCommand> },
    Group { common: CommonAttrs, #[serde(default)] children: Vec<SvgElement> },
    Text {
        common: CommonAttrs,
        #[serde(default, skip_serializing_if = "Option::is_none")] x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")] y: Option<f64>,
        #[serde(default)] children: Vec<SvgElement>,
    },
    Tspan {
        common: CommonAttrs,
        #[serde(default, skip_serializing_if = "Option::is_none")] x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")] y: Option<f64>,
        #[serde(default)] children: Vec<SvgElement>,
    },
    Defs { common: CommonAttrs, #[serde(default)] children: Vec<SvgElement> },
    LinearGradient {
        common: CommonAttrs,
        #[serde(default, skip_serializing_if = "Option::is_none")] id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] x1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] y1: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] x2: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] y2: Option<String>,
        #[serde(default)] children: Vec<SvgElement>,
    },
    RadialGradient {
        common: CommonAttrs,
        #[serde(default, skip_serializing_if = "Option::is_none")] id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] cx: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] cy: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] r: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] fx: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] fy: Option<String>,
        #[serde(default)] children: Vec<SvgElement>,
    },
    Stop {
        common: CommonAttrs,
        offset: String,
        #[serde(default, skip_serializing_if = "Option::is_none")] stop_color: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")] stop_opacity: Option<String>,
    },
    Use {
        common: CommonAttrs,
        href: String,
        #[serde(default, skip_serializing_if = "Option::is_none")] x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")] y: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")] width: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")] height: Option<f64>,
    },
    /// 🚪 Escape hatch: any element name outside the typed set above, kept byte-for-byte.
    Unknown { name: String, #[serde(default)] attrs: Vec<XmlAttr>, #[serde(default)] children: Vec<SvgElement> },
    TextNode(String),
    CData(String),
    Comment(String),
    ProcessingInstruction { target: String, data: String },
}

fn convert_children(children: &[XmlNode]) -> Result<Vec<SvgElement>, String> {
    children.iter().map(svg_element_from_xml_node).collect()
}

/// 🌳 Converts one generic (lossless) `XmlNode` into the typed SVG model. Dispatches on the local
/// (namespace-prefix-stripped) tag name against the typed set; anything else becomes `Unknown`
/// (with the ORIGINAL, still-prefixed name preserved) rather than being dropped.
pub fn svg_element_from_xml_node(node: &XmlNode) -> Result<SvgElement, String> {
    match node {
        XmlNode::Text { text } => Ok(SvgElement::TextNode(text.clone())),
        XmlNode::CData { text } => Ok(SvgElement::CData(text.clone())),
        XmlNode::Comment { text } => Ok(SvgElement::Comment(text.clone())),
        XmlNode::ProcessingInstruction { target, data } => Ok(SvgElement::ProcessingInstruction { target: target.clone(), data: data.clone() }),
        XmlNode::Element { name, attrs, children } => match local_name(name) {
            "svg" => {
                let common = parse_common_attrs(attrs, &["viewBox", "width", "height", "xmlns"]);
                let view_box = match attr_val(attrs, "viewBox") {
                    Some(v) => Some(parse_view_box(v)?),
                    None => None,
                };
                Ok(SvgElement::Svg {
                    common,
                    view_box,
                    width: attr_string_opt(attrs, "width"),
                    height: attr_string_opt(attrs, "height"),
                    xmlns: attr_string_opt(attrs, "xmlns"),
                    children: convert_children(children)?,
                })
            }
            "rect" => {
                let common = parse_common_attrs(attrs, &["x", "y", "width", "height", "rx", "ry"]);
                Ok(SvgElement::Rect {
                    common,
                    x: attr_f64(attrs, "x", 0.0)?,
                    y: attr_f64(attrs, "y", 0.0)?,
                    width: attr_f64(attrs, "width", 0.0)?,
                    height: attr_f64(attrs, "height", 0.0)?,
                    rx: attr_f64_opt(attrs, "rx")?,
                    ry: attr_f64_opt(attrs, "ry")?,
                })
            }
            "circle" => {
                let common = parse_common_attrs(attrs, &["cx", "cy", "r"]);
                Ok(SvgElement::Circle { common, cx: attr_f64(attrs, "cx", 0.0)?, cy: attr_f64(attrs, "cy", 0.0)?, r: attr_f64(attrs, "r", 0.0)? })
            }
            "ellipse" => {
                let common = parse_common_attrs(attrs, &["cx", "cy", "rx", "ry"]);
                Ok(SvgElement::Ellipse {
                    common,
                    cx: attr_f64(attrs, "cx", 0.0)?,
                    cy: attr_f64(attrs, "cy", 0.0)?,
                    rx: attr_f64(attrs, "rx", 0.0)?,
                    ry: attr_f64(attrs, "ry", 0.0)?,
                })
            }
            "line" => {
                let common = parse_common_attrs(attrs, &["x1", "y1", "x2", "y2"]);
                Ok(SvgElement::Line {
                    common,
                    x1: attr_f64(attrs, "x1", 0.0)?,
                    y1: attr_f64(attrs, "y1", 0.0)?,
                    x2: attr_f64(attrs, "x2", 0.0)?,
                    y2: attr_f64(attrs, "y2", 0.0)?,
                })
            }
            "polyline" => {
                let common = parse_common_attrs(attrs, &["points"]);
                let points = match attr_val(attrs, "points") { Some(v) => parse_points(v)?, None => Vec::new() };
                Ok(SvgElement::Polyline { common, points })
            }
            "polygon" => {
                let common = parse_common_attrs(attrs, &["points"]);
                let points = match attr_val(attrs, "points") { Some(v) => parse_points(v)?, None => Vec::new() };
                Ok(SvgElement::Polygon { common, points })
            }
            "path" => {
                let common = parse_common_attrs(attrs, &["d"]);
                let d = match attr_val(attrs, "d") { Some(v) => parse_path_data(v)?, None => Vec::new() };
                Ok(SvgElement::Path { common, d })
            }
            "g" => Ok(SvgElement::Group { common: parse_common_attrs(attrs, &[]), children: convert_children(children)? }),
            "text" => {
                let common = parse_common_attrs(attrs, &["x", "y"]);
                Ok(SvgElement::Text { common, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, children: convert_children(children)? })
            }
            "tspan" => {
                let common = parse_common_attrs(attrs, &["x", "y"]);
                Ok(SvgElement::Tspan { common, x: attr_f64_opt(attrs, "x")?, y: attr_f64_opt(attrs, "y")?, children: convert_children(children)? })
            }
            "defs" => Ok(SvgElement::Defs { common: parse_common_attrs(attrs, &[]), children: convert_children(children)? }),
            "linearGradient" => {
                let common = parse_common_attrs(attrs, &["id", "x1", "y1", "x2", "y2"]);
                Ok(SvgElement::LinearGradient {
                    common,
                    id: attr_string_opt(attrs, "id"),
                    x1: attr_string_opt(attrs, "x1"),
                    y1: attr_string_opt(attrs, "y1"),
                    x2: attr_string_opt(attrs, "x2"),
                    y2: attr_string_opt(attrs, "y2"),
                    children: convert_children(children)?,
                })
            }
            "radialGradient" => {
                let common = parse_common_attrs(attrs, &["id", "cx", "cy", "r", "fx", "fy"]);
                Ok(SvgElement::RadialGradient {
                    common,
                    id: attr_string_opt(attrs, "id"),
                    cx: attr_string_opt(attrs, "cx"),
                    cy: attr_string_opt(attrs, "cy"),
                    r: attr_string_opt(attrs, "r"),
                    fx: attr_string_opt(attrs, "fx"),
                    fy: attr_string_opt(attrs, "fy"),
                    children: convert_children(children)?,
                })
            }
            "stop" => {
                let common = parse_common_attrs(attrs, &["offset", "stop-color", "stop-opacity"]);
                Ok(SvgElement::Stop {
                    common,
                    offset: attr_string_opt(attrs, "offset").unwrap_or_default(),
                    stop_color: attr_string_opt(attrs, "stop-color"),
                    stop_opacity: attr_string_opt(attrs, "stop-opacity"),
                })
            }
            "use" => {
                let common = parse_common_attrs(attrs, &["href", "xlink:href", "x", "y", "width", "height"]);
                let href = attr_string_opt(attrs, "href").or_else(|| attr_string_opt(attrs, "xlink:href")).unwrap_or_default();
                Ok(SvgElement::Use {
                    common,
                    href,
                    x: attr_f64_opt(attrs, "x")?,
                    y: attr_f64_opt(attrs, "y")?,
                    width: attr_f64_opt(attrs, "width")?,
                    height: attr_f64_opt(attrs, "height")?,
                })
            }
            _ => Ok(SvgElement::Unknown { name: name.clone(), attrs: attrs.clone(), children: convert_children(children)? }),
        },
    }
}

/// 🌳 Lowers the typed model back into the generic (lossless) `XmlNode` tree that the xml codec's
/// text/binary writers already know how to serialize.
pub fn svg_element_to_xml_node(el: &SvgElement) -> XmlNode {
    match el {
        SvgElement::TextNode(t) => XmlNode::Text { text: t.clone() },
        SvgElement::CData(t) => XmlNode::CData { text: t.clone() },
        SvgElement::Comment(t) => XmlNode::Comment { text: t.clone() },
        SvgElement::ProcessingInstruction { target, data } => XmlNode::ProcessingInstruction { target: target.clone(), data: data.clone() },
        SvgElement::Svg { common, view_box, width, height, xmlns, children } => {
            let mut attrs = Vec::new();
            if let Some(vb) = view_box {
                attrs.push(XmlAttr { name: "viewBox".into(), value: view_box_to_string(vb) });
            }
            if let Some(w) = width {
                attrs.push(XmlAttr { name: "width".into(), value: w.clone() });
            }
            if let Some(h) = height {
                attrs.push(XmlAttr { name: "height".into(), value: h.clone() });
            }
            if let Some(x) = xmlns {
                attrs.push(XmlAttr { name: "xmlns".into(), value: x.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "svg".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Rect { common, x, y, width, height, rx, ry } => {
            let mut attrs = vec![
                XmlAttr { name: "x".into(), value: fmt_num(*x) },
                XmlAttr { name: "y".into(), value: fmt_num(*y) },
                XmlAttr { name: "width".into(), value: fmt_num(*width) },
                XmlAttr { name: "height".into(), value: fmt_num(*height) },
            ];
            if let Some(rx) = rx {
                attrs.push(XmlAttr { name: "rx".into(), value: fmt_num(*rx) });
            }
            if let Some(ry) = ry {
                attrs.push(XmlAttr { name: "ry".into(), value: fmt_num(*ry) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "rect".into(), attrs, children: vec![] }
        }
        SvgElement::Circle { common, cx, cy, r } => {
            let mut attrs = vec![
                XmlAttr { name: "cx".into(), value: fmt_num(*cx) },
                XmlAttr { name: "cy".into(), value: fmt_num(*cy) },
                XmlAttr { name: "r".into(), value: fmt_num(*r) },
            ];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "circle".into(), attrs, children: vec![] }
        }
        SvgElement::Ellipse { common, cx, cy, rx, ry } => {
            let mut attrs = vec![
                XmlAttr { name: "cx".into(), value: fmt_num(*cx) },
                XmlAttr { name: "cy".into(), value: fmt_num(*cy) },
                XmlAttr { name: "rx".into(), value: fmt_num(*rx) },
                XmlAttr { name: "ry".into(), value: fmt_num(*ry) },
            ];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "ellipse".into(), attrs, children: vec![] }
        }
        SvgElement::Line { common, x1, y1, x2, y2 } => {
            let mut attrs = vec![
                XmlAttr { name: "x1".into(), value: fmt_num(*x1) },
                XmlAttr { name: "y1".into(), value: fmt_num(*y1) },
                XmlAttr { name: "x2".into(), value: fmt_num(*x2) },
                XmlAttr { name: "y2".into(), value: fmt_num(*y2) },
            ];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "line".into(), attrs, children: vec![] }
        }
        SvgElement::Polyline { common, points } => {
            let mut attrs = vec![XmlAttr { name: "points".into(), value: points_to_string(points) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "polyline".into(), attrs, children: vec![] }
        }
        SvgElement::Polygon { common, points } => {
            let mut attrs = vec![XmlAttr { name: "points".into(), value: points_to_string(points) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "polygon".into(), attrs, children: vec![] }
        }
        SvgElement::Path { common, d } => {
            let mut attrs = vec![XmlAttr { name: "d".into(), value: path_data_to_string(d) }];
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "path".into(), attrs, children: vec![] }
        }
        SvgElement::Group { common, children } => {
            let mut attrs = Vec::new();
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "g".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Text { common, x, y, children } => {
            let mut attrs = Vec::new();
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "text".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Tspan { common, x, y, children } => {
            let mut attrs = Vec::new();
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "tspan".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Defs { common, children } => {
            let mut attrs = Vec::new();
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "defs".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::LinearGradient { common, id, x1, y1, x2, y2, children } => {
            let mut attrs = Vec::new();
            if let Some(id) = id {
                attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
            }
            if let Some(v) = x1 {
                attrs.push(XmlAttr { name: "x1".into(), value: v.clone() });
            }
            if let Some(v) = y1 {
                attrs.push(XmlAttr { name: "y1".into(), value: v.clone() });
            }
            if let Some(v) = x2 {
                attrs.push(XmlAttr { name: "x2".into(), value: v.clone() });
            }
            if let Some(v) = y2 {
                attrs.push(XmlAttr { name: "y2".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "linearGradient".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::RadialGradient { common, id, cx, cy, r, fx, fy, children } => {
            let mut attrs = Vec::new();
            if let Some(id) = id {
                attrs.push(XmlAttr { name: "id".into(), value: id.clone() });
            }
            if let Some(v) = cx {
                attrs.push(XmlAttr { name: "cx".into(), value: v.clone() });
            }
            if let Some(v) = cy {
                attrs.push(XmlAttr { name: "cy".into(), value: v.clone() });
            }
            if let Some(v) = r {
                attrs.push(XmlAttr { name: "r".into(), value: v.clone() });
            }
            if let Some(v) = fx {
                attrs.push(XmlAttr { name: "fx".into(), value: v.clone() });
            }
            if let Some(v) = fy {
                attrs.push(XmlAttr { name: "fy".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "radialGradient".into(), attrs, children: children.iter().map(svg_element_to_xml_node).collect() }
        }
        SvgElement::Stop { common, offset, stop_color, stop_opacity } => {
            let mut attrs = vec![XmlAttr { name: "offset".into(), value: offset.clone() }];
            if let Some(v) = stop_color {
                attrs.push(XmlAttr { name: "stop-color".into(), value: v.clone() });
            }
            if let Some(v) = stop_opacity {
                attrs.push(XmlAttr { name: "stop-opacity".into(), value: v.clone() });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "stop".into(), attrs, children: vec![] }
        }
        SvgElement::Use { common, href, x, y, width, height } => {
            let mut attrs = vec![XmlAttr { name: "href".into(), value: href.clone() }];
            if let Some(x) = x {
                attrs.push(XmlAttr { name: "x".into(), value: fmt_num(*x) });
            }
            if let Some(y) = y {
                attrs.push(XmlAttr { name: "y".into(), value: fmt_num(*y) });
            }
            if let Some(w) = width {
                attrs.push(XmlAttr { name: "width".into(), value: fmt_num(*w) });
            }
            if let Some(h) = height {
                attrs.push(XmlAttr { name: "height".into(), value: fmt_num(*h) });
            }
            push_common_attrs(&mut attrs, common);
            XmlNode::Element { name: "use".into(), attrs, children: vec![] }
        }
        SvgElement::Unknown { name, attrs, children } => {
            XmlNode::Element { name: name.clone(), attrs: attrs.clone(), children: children.iter().map(svg_element_to_xml_node).collect() }
        }
    }
}

pub fn svg_document_to_typed(doc: &XmlDocument) -> Result<SvgElement, String> {
    match &doc.root {
        Some(node) => svg_element_from_xml_node(node),
        None => Err("svg document has no root element".into()),
    }
}

pub fn typed_to_svg_document(root: &SvgElement, doctype: Option<String>) -> XmlDocument {
    XmlDocument { root: Some(svg_element_to_xml_node(root)), doctype, declaration: None, prolog: Vec::new() }
}
//#endregion 🔖️TypedElementModel

//#region 🔖️NodePath
/// 🧭 A child-index chain from the document root, used by the mutation vocabulary to address a
/// node inside `SvgSnapshot.doc` without needing the full typed model (mutations operate on the
/// persisted, always-lossless `XmlDocument`, not the typed view).
pub type NodePath = Vec<usize>;

pub fn node_at<'a>(doc: &'a XmlDocument, path: &[usize]) -> Result<&'a XmlNode, String> {
    let mut node = doc.root.as_ref().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            XmlNode::Element { children, .. } => {
                node = children.get(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

pub fn node_at_mut<'a>(doc: &'a mut XmlDocument, path: &[usize]) -> Result<&'a mut XmlNode, String> {
    let mut node = doc.root.as_mut().ok_or("document has no root element")?;
    for &idx in path {
        match node {
            XmlNode::Element { children, .. } => {
                node = children.get_mut(idx).ok_or_else(|| format!("child index {idx} out of range"))?;
            }
            _ => return Err("path descends into a non-element node".into()),
        }
    }
    Ok(node)
}

pub fn element_attr<'a>(node: &'a XmlNode, name: &str) -> Option<&'a str> {
    match node {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str()),
        _ => None,
    }
}

/// 🏷️ Sets `name` to `value` (updating the existing attribute IN PLACE if present, so its position
/// in the attribute list is preserved -- only a genuinely new attribute gets appended); `None`
/// removes it. Update-in-place (rather than remove-then-append) matters for `SetAttribute`'s
/// apply/inverse round trip to reproduce the exact original attribute order.
pub fn set_element_attr(node: &mut XmlNode, name: &str, value: Option<String>) {
    if let XmlNode::Element { attrs, .. } = node {
        match value {
            Some(v) => match attrs.iter_mut().find(|a| a.name == name) {
                Some(existing) => existing.value = v,
                None => attrs.push(XmlAttr { name: name.to_string(), value: v }),
            },
            None => attrs.retain(|a| a.name != name),
        }
    }
}
//#endregion 🔖️NodePath

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for SvgSnapshot {
    const EXTENSION: &'static str = "svg";
    fn envelope_id() -> &'static str { "stdio.svg" }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        match store::semio_format::split_text_preamble(text) {
            Ok((_, body)) => crate::artifacts::svg::schema::mutations::dec_svg_snapshot(body.trim())
                .map_err(|e| store::TextError::new(format!("svg state parse: {e}"), dsl::TextSpan::at(1, 1))),
            Err(_) => Self::import_utf8(text.as_bytes()).map_err(|e| store::TextError::new(format!("svg parse: {e}"), dsl::TextSpan::at(1, 1))),
        }
    }
    fn print_dsl(&self) -> String {
        let body = crate::artifacts::svg::schema::mutations::enc_svg_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🧪️ P2-FG3: `stdio.svg` is TEXT-NATIVE (per the W0 census row) — there is no "binary SVG"; the
/// pack container is the SEMIO envelope wrapping the artifact's own REAL wire text
/// (`write_svg_xml`/`parse_svg_xml`, themselves `xml_document_to_text`/`xml_document_from_text`)
/// verbatim, same treatment `📰xml`'s own `ArtifactPack` gives its restated XML text
/// (`📰xml/…/📸️snapshot/🦀️component.rs`'s own P2-FG1 fix). Replaces the previous
/// `serde_json::to_vec`/`from_slice` placeholder, which satisfied the trait but was a
/// literal-JSON-payload-disguised-as-binary violation of `POLICY_STDIO_JSON_TRANSFER_BAN` (flagged
/// by name in the P2-W0 recon report, `svg` row, "Yes — in scope").
impl store::ArtifactPack for SvgSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let mut raw = vec![1];
        crate::artifacts::svg::schema::mutations::enc_svg_snapshot_bin(self, &mut raw);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        let mut reader = store::ByteReader::new(&inner);
        let version = reader.read_u8().map_err(|e| store::PackError::Schema(e.to_string()))?;
        if version != 1 { return Err(store::PackError::Schema(format!("unsupported svg snapshot state version {version}"))); }
        crate::artifacts::svg::schema::mutations::dec_svg_snapshot_bin(&mut reader).map_err(store::PackError::Schema)
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region PathGrammar
    #[test]
    fn path_implicit_lineto_repetition_after_moveto() {
        let cmds = parse_path_data("M 0 0 10 10 20 20").unwrap();
        assert_eq!(
            cmds,
            vec![
                PathCommand::MoveTo { x: 0.0, y: 0.0, relative: false },
                PathCommand::LineTo { x: 10.0, y: 10.0, relative: false },
                PathCommand::LineTo { x: 20.0, y: 20.0, relative: false },
            ]
        );
    }

    #[test]
    fn path_arc_flag_squeeze_decomposes_correctly() {
        // 🚩 THE classic bug: "A5 5 0 108 8" must decompose as flags 1,0 then x=8,y=8 -- not 10,8,8.
        let cmds = parse_path_data("M40,20 A5 5 0 108 8").unwrap();
        match cmds.last().unwrap() {
            PathCommand::Arc { rx, ry, x_axis_rotation, large_arc, sweep, x, y, relative } => {
                assert_eq!((*rx, *ry, *x_axis_rotation), (5.0, 5.0, 0.0));
                assert_eq!((*large_arc, *sweep), (true, false));
                assert_eq!((*x, *y, *relative), (8.0, 8.0, false));
            }
            other => panic!("expected Arc, got {other:?}"),
        }
    }

    #[test]
    fn path_relative_and_close() {
        let cmds = parse_path_data("m0,0 10,0 0,10z").unwrap();
        assert_eq!(
            cmds,
            vec![
                PathCommand::MoveTo { x: 0.0, y: 0.0, relative: true },
                PathCommand::LineTo { x: 10.0, y: 0.0, relative: true },
                PathCommand::LineTo { x: 0.0, y: 10.0, relative: true },
                PathCommand::ClosePath,
            ]
        );
    }

    #[test]
    fn path_round_trips_through_string() {
        let cmds = parse_path_data("M0,0 C1,1 2,2 3,3 S4,4 5,5 A5 5 0 108 8 Z").unwrap();
        let text = path_data_to_string(&cmds);
        let reparsed = parse_path_data(&text).unwrap();
        assert_eq!(cmds, reparsed);
    }

    #[test]
    fn path_missing_leading_command_is_error() {
        assert!(parse_path_data("10 10 L20 20").is_err());
    }
    //#endregion PathGrammar

    //#region TransformGrammar
    #[test]
    fn transform_parses_all_functions() {
        let ops = parse_transform_list("translate(10,20) scale(2) rotate(90,5,5) skewX(30) skewY(-15) matrix(1,0,0,1,0,0)").unwrap();
        assert_eq!(
            ops,
            vec![
                TransformOp::Translate { x: 10.0, y: Some(20.0) },
                TransformOp::Scale { x: 2.0, y: None },
                TransformOp::Rotate { angle: 90.0, center: Some((5.0, 5.0)) },
                TransformOp::SkewX { angle: 30.0 },
                TransformOp::SkewY { angle: -15.0 },
                TransformOp::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 },
            ]
        );
    }

    #[test]
    fn transform_composition_order_matches_svg_semantics() {
        // translate(10,0) rotate(90) applied to (1,0) => (10,1): rotate happens in local space first.
        let m = transform_ops_to_matrix(&parse_transform_list("translate(10,0) rotate(90)").unwrap());
        let (px, py) = (m.a * 1.0 + m.c * 0.0 + m.e, m.b * 1.0 + m.d * 0.0 + m.f);
        assert!((px - 10.0).abs() < 1e-9 && (py - 1.0).abs() < 1e-9);
    }

    #[test]
    fn transform_rotate_about_center_fixes_that_point() {
        let m = transform_ops_to_matrix(&parse_transform_list("rotate(45,7,3)").unwrap());
        let (px, py) = (m.a * 7.0 + m.c * 3.0 + m.e, m.b * 7.0 + m.d * 3.0 + m.f);
        assert!((px - 7.0).abs() < 1e-9 && (py - 3.0).abs() < 1e-9);
    }

    #[test]
    fn transform_wrong_arity_is_error() {
        assert!(parse_transform_list("scale(1,2,3)").is_err());
        assert!(parse_transform_list("frobnicate(1)").is_err());
    }
    //#endregion TransformGrammar

    //#region StyleAndGeometry
    #[test]
    fn style_declarations_parse_and_unrecognized_ones_are_retained() {
        let mut p = PresentationAttrs::default();
        for (k, v) in parse_style_decls("fill: red; stroke:blue ; opacity:0.5; letter-spacing: 2px") {
            if !apply_presentation_attr(&mut p, &k, &v) {
                p.extra_style.push((k, v));
            }
        }
        assert_eq!(p.fill.as_deref(), Some("red"));
        assert_eq!(p.stroke.as_deref(), Some("blue"));
        assert_eq!(p.opacity.as_deref(), Some("0.5"));
        assert_eq!(p.extra_style, vec![("letter-spacing".to_string(), "2px".to_string())]);
    }

    #[test]
    fn view_box_and_points_parse() {
        assert_eq!(parse_view_box("0 0 100 50").unwrap(), ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 50.0 });
        assert_eq!(parse_points("0,0 10,0 5,10").unwrap(), vec![(0.0, 0.0), (10.0, 0.0), (5.0, 10.0)]);
        assert!(parse_view_box("0 0 100").is_err());
        assert!(parse_points("0,0 10").is_err());
    }
    //#endregion StyleAndGeometry

    //#region TypedParse
    #[test]
    fn typed_parse_of_multi_element_document_with_gradient_group_and_arc_path() {
        // 🚧️ `r##"..."##` (not `r#"..."#`) because the fixture's `stop-color="#ff0000"` contains
        // the literal 2-char sequence `"#`, which would otherwise close a single-hash raw string.
        let text = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">
  <defs>
    <linearGradient id="g1" x1="0" y1="0" x2="1" y2="0">
      <stop offset="0%" stop-color="#ff0000"/>
      <stop offset="100%" stop-color="#0000ff"/>
    </linearGradient>
  </defs>
  <g id="shapes" transform="translate(5,5)">
    <rect x="0" y="0" width="40" height="20" fill="url(#g1)"/>
    <circle cx="60" cy="20" r="15" style="fill: green; stroke: black; stroke-width: 2"/>
    <path d="M10,50 L40,50 A5 5 0 108 8 Z"/>
  </g>
</svg>"##;
        let doc = xml_document_from_text(text).expect("xml parses");
        let typed = svg_document_to_typed(&doc).expect("typed conversion");

        // 🧵 The fixture is pretty-printed, so raw XML children include whitespace-only text nodes
        // between elements (preserved losslessly, per the typed model's design) -- filter those
        // out before indexing into the REAL element children.
        fn elements_only(v: &[SvgElement]) -> Vec<&SvgElement> {
            v.iter().filter(|c| !matches!(c, SvgElement::TextNode(_))).collect()
        }

        let (view_box, children) = match &typed {
            SvgElement::Svg { view_box, children, .. } => (view_box.clone(), elements_only(children)),
            other => panic!("expected Svg root, got {other:?}"),
        };
        assert_eq!(view_box, Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 100.0, height: 100.0 }));
        assert_eq!(children.len(), 2, "expected <defs> and <g> as direct children");

        let defs_children = match children[0] {
            SvgElement::Defs { children, .. } => elements_only(children),
            other => panic!("expected Defs, got {other:?}"),
        };
        let (id, stops) = match defs_children[0] {
            SvgElement::LinearGradient { id, children, .. } => (id.clone(), elements_only(children)),
            other => panic!("expected LinearGradient, got {other:?}"),
        };
        assert_eq!(id.as_deref(), Some("g1"));
        assert_eq!(stops.len(), 2);
        match stops[0] {
            SvgElement::Stop { offset, stop_color, .. } => {
                assert_eq!(offset, "0%");
                assert_eq!(stop_color.as_deref(), Some("#ff0000"));
            }
            other => panic!("expected Stop, got {other:?}"),
        }

        let (group_common, group_children) = match children[1] {
            SvgElement::Group { common, children } => (common, elements_only(children)),
            other => panic!("expected Group, got {other:?}"),
        };
        assert_eq!(group_common.id.as_deref(), Some("shapes"));
        assert_eq!(group_common.transform, Some(vec![TransformOp::Translate { x: 5.0, y: Some(5.0) }]));
        assert_eq!(group_children.len(), 3);

        match group_children[0] {
            SvgElement::Rect { common, x, y, width, height, .. } => {
                assert_eq!((*x, *y, *width, *height), (0.0, 0.0, 40.0, 20.0));
                assert_eq!(common.presentation.fill.as_deref(), Some("url(#g1)"));
            }
            other => panic!("expected Rect, got {other:?}"),
        }
        match group_children[1] {
            SvgElement::Circle { common, cx, cy, r } => {
                assert_eq!((*cx, *cy, *r), (60.0, 20.0, 15.0));
                // 🎨 style="" must win and be REAL-parsed, not string-matched.
                assert_eq!(common.presentation.fill.as_deref(), Some("green"));
                assert_eq!(common.presentation.stroke.as_deref(), Some("black"));
                assert_eq!(common.presentation.stroke_width.as_deref(), Some("2"));
            }
            other => panic!("expected Circle, got {other:?}"),
        }
        match group_children[2] {
            SvgElement::Path { d, .. } => {
                assert_eq!(
                    d,
                    &vec![
                        PathCommand::MoveTo { x: 10.0, y: 50.0, relative: false },
                        PathCommand::LineTo { x: 40.0, y: 50.0, relative: false },
                        PathCommand::Arc { rx: 5.0, ry: 5.0, x_axis_rotation: 0.0, large_arc: true, sweep: false, x: 8.0, y: 8.0, relative: false },
                        PathCommand::ClosePath,
                    ]
                );
            }
            other => panic!("expected Path, got {other:?}"),
        }
    }

    #[test]
    fn unknown_element_and_attrs_survive_losslessly() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg"><customThing data-x="1"><rect x="0" y="0" width="1" height="1"/></customThing></svg>"#;
        let doc = xml_document_from_text(text).unwrap();
        let typed = svg_document_to_typed(&doc).unwrap();
        let children = match &typed {
            SvgElement::Svg { children, .. } => children,
            other => panic!("expected Svg, got {other:?}"),
        };
        match &children[0] {
            SvgElement::Unknown { name, attrs, children } => {
                assert_eq!(name, "customThing");
                assert_eq!(attrs.iter().find(|a| a.name == "data-x").map(|a| a.value.as_str()), Some("1"));
                assert_eq!(children.len(), 1);
                assert!(matches!(children[0], SvgElement::Rect { .. }));
            }
            other => panic!("expected Unknown, got {other:?}"),
        }
        // Round trip: the typed model lowers back to an XmlDocument that reparses identically.
        let doc2 = typed_to_svg_document(&typed, None);
        let typed2 = svg_document_to_typed(&doc2).unwrap();
        assert_eq!(typed, typed2);
    }
    //#endregion TypedParse
}
//#endregion 🧪️Tests
