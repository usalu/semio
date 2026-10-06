//! 📝️ Text representation codec surface for `stdio.svg` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type SvgSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v1_1::subsets::base::schema::snapshot::*;
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text_checked, XmlAttr, XmlDocument, XmlNode};

impl store::ArtifactDsl for SvgSnapshot {
    const EXTENSION: &'static str = "svg";
    fn envelope_id() -> &'static str {
        "stdio.svg"
    }

    /// 📥️ Two real inputs, told apart by the envelope: text that CARRIES a semio preamble is this
    /// artifact's own snapshot DSL (`encode_snapshot`'s structured body), and text that does not is
    /// the document's own `.svg` markup, which [`SvgSnapshot::import_utf8`] parses losslessly. Same
    /// two-branch shape the sibling `📰️xml` artifact's `parse_dsl` uses, and for the same reason:
    /// `from_text` (this subset's `DerivedConstruction`, and `📚️examples`' own raw markup) hands raw
    /// SVG straight in, and refusing it made every such caller fail on the preamble check alone.
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        match store::semio_format::split_text_preamble(text) {
            Ok((_, body)) => crate::schema::mutation_support::decode_snapshot(body.trim()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("svg state parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
            Err(_) => Self::import_utf8(text.as_bytes()).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("svg parse: {e}"), semio_framework_diagnostic::TextSpan::at(1, 1))),
        }
    }
    fn print_dsl(&self) -> String {
        let body = crate::schema::mutation_support::encode_snapshot(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::standards::v1_1::subsets::base::schema::snapshot::*;
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text_checked, XmlAttr, XmlDocument, XmlNode};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn write_svg_xml(doc: &XmlDocument) -> Result<String, String> {
    xml_document_to_text_checked(doc)
}

/// 🔢 Byte-cursor shared by the `d`/`transform`/`viewBox`/`points` grammars (all of which use the
/// same SVG `<number>`/`<comma-wsp>` productions). Operates on bytes rather than chars because SVG
/// numeric grammar is pure ASCII; `str` slicing stays valid because we only ever slice at ASCII
/// byte boundaries.
pub(crate) struct NumCursor<'a> {
    s: &'a [u8],
    pos: usize,
}

impl<'a> NumCursor<'a> {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn new(s: &'a str) -> Self {
        Self { s: s.as_bytes(), pos: 0 }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn peek(&self) -> Option<u8> {
        self.s.get(self.pos).copied()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn is_eof(&self) -> bool {
        self.pos >= self.s.len()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_wsp_comma(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') | Some(b',')) {
            self.pos += 1;
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn skip_wsp(&mut self) {
        while matches!(self.peek(), Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r')) {
            self.pos += 1;
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_number(&mut self) -> Result<f64, String> {
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
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(crate) fn parse_flag(&mut self) -> Result<bool, String> {
        self.skip_wsp_comma();
        match self.peek() {
            Some(b'0') => {
                self.pos += 1;
                Ok(false)
            }
            Some(b'1') => {
                self.pos += 1;
                Ok(true)
            }
            other => Err(format!("expected arc flag (0/1), got {other:?} at byte {}", self.pos)),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_number_list(s: &str) -> Result<Vec<f64>, String> {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_view_box(s: &str) -> Result<ViewBox, String> {
    let nums = parse_number_list(s)?;
    if nums.len() != 4 {
        return Err(format!("viewBox requires exactly 4 numbers, got {}", nums.len()));
    }
    Ok(ViewBox { min_x: nums[0], min_y: nums[1], width: nums[2], height: nums[3] })
}

/// 🔗️ `points="x1,y1 x2,y2 ..."` (polyline/polygon).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn parse_points(s: &str) -> Result<Vec<(f64, f64)>, String> {
    let nums = parse_number_list(s)?;
    if nums.len() % 2 != 0 {
        return Err("points list must have an even number of coordinates".into());
    }
    Ok(nums.chunks(2).map(|c| (c[0], c[1])).collect())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🖊️ Parses a `d` attribute per the SVG path mini-language grammar. Verified against 18 checks
/// (incl. the arc-flag squeeze edge case) in a standalone scratch crate before porting here, per
/// the technique in the ticket's STATUS.md.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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

/// 🧩 Real `key: value; key2: value2` parsing (declaration-list split on `;`, each split on the
/// FIRST `:`) -- not a substring/`contains()` hack.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
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




}
pub use snapshot_wire_codec::*;

#[allow(unused_imports)]
mod snapshot_wire2_codec {
use super::*;
use crate::standards::v1_1::subsets::base::schema::snapshot::*;
use crate::STDIO_SVG_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_xml::schema::snapshot::{xml_document_from_text, xml_document_to_text_checked, XmlAttr, XmlDocument, XmlNode};

/// 🧩 Buckets `attrs` into `CommonAttrs`, treating `element_specific` names as already consumed
/// elsewhere (so they don't ALSO land in `extra_attrs`). `style=""` is real-parsed and takes
/// precedence over same-named plain attributes (CSS cascade order); unrecognized style
/// declarations are retained in `presentation.extra_style`, never dropped.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_common_attrs(attrs: &[XmlAttr], element_specific: &[&str]) -> CommonAttrs {
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

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn attr_val<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}
}
pub use snapshot_wire2_codec::*;

#[allow(unused_imports)]
mod snapshot_wire3_codec {
use crate::standards::v1_1::subsets::base::schema::mutation_support::*;
use crate::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::node_at;
use crate::SvgSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument, XmlNode};

pub(crate) fn encode_snapshot(snapshot: &SvgSnapshot) -> String {
    use crate::schema::diff::{enc_xml_node};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_doctype};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_declaration};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_prolog};
    use crate::standards::v1_1::subsets::base::io::text::diff::{encode_option};
    use crate::standards::v1_1::subsets::base::io::text::diff::{enc_str};
    format!(
        "[{},{},{},{},{},{}]",
        enc_str(&snapshot.schema),
        encode_option(&snapshot.doc.root, enc_xml_node),
        encode_option(&snapshot.doc.doctype, enc_doctype),
        encode_option(&snapshot.doc.declaration, enc_declaration),
        enc_prolog(&snapshot.doc.prolog),
        enc_prolog(&snapshot.doc.epilog)
    )
}

pub(crate) fn decode_snapshot(value: &str) -> Result<SvgSnapshot, String> {
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_xml_node};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_doctype};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_declaration};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_prolog};
    use crate::standards::v1_1::subsets::base::io::text::diff::{decode_option};
    use crate::standards::v1_1::subsets::base::io::text::diff::{strip_brackets};
    use crate::standards::v1_1::subsets::base::io::text::diff::{split_top_level};
    use crate::standards::v1_1::subsets::base::io::text::diff::{dec_str};
    let parts = split_top_level(strip_brackets(value)?, ',');
    let [schema, root, doctype, declaration, prolog, epilog] = parts.as_slice() else {
        return Err(format!("svg snapshot: expected 6 fields, got {}", parts.len()));
    };
    let snapshot = SvgSnapshot {
        schema: dec_str(schema)?,
        doc: XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)?, epilog: dec_prolog(epilog)? },
    };
    Ok(snapshot)
}
}
pub use snapshot_wire3_codec::*;
