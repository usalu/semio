//! 🔮️ Third-party codec for `s.note.note@1/✳️any`'s three registered carriers.
//!
//! Every BEFORE/AFTER byte pair this binary's `generate` command writes is built DIRECTLY by a
//! third-party library — never by executing note's own (currently non-building) production
//! serializers, and never by "applying" a mutation in code: both states of every recipe are
//! independently authored in `recipes()` below, exactly the shape `…💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🏭️generator/📜️script.ts`
//! and `…✳️cad/🔬️probes/🦀️oracle-probe/src/main.rs` (the sibling this file's structure mirrors)
//! already use.
//!
//! `dxf` 0.6 both WRITES (`Drawing::save`) and READS (`Drawing::load`) DXF R12: only `Ink` blocks'
//! raw `points.windows(2)` become `LINE` entities on layer `"0"` — no `x`/`y`/`rotation` transform,
//! no visibility filter, no width — reproducing `NoteIntoDxf::serialize`'s body exactly (see
//! `../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dxf/🔖️r12/✳️any/🦀️component.rs`).
//!
//! `quick-xml` 0.42 both WRITES (`quick_xml::Writer`) and READS (`quick_xml::Reader`) the SVG XML:
//! every VISIBLE block (flatten + `block_visible` filter — visibility IS honoured here) wrapped in
//! `<g transform="matrix(a,b,c,d,e,f)">` where the matrix is the block's own `x`/`y`/`rotation`, and
//! kind-specific content inside (see `../../🚪️io/🦀️component.rs`'s `draw_node_from_note_block`).
//!
//! `lopdf` 0.44 both WRITES (`Document::save_to`) and READS (`Document::load_mem` +
//! `content::Content::decode`) the PDF: `title` + every `Text` block's paragraphs, space-joined, onto
//! ONE page — no visibility filter (the same cross-carrier bug DXF has), no position, no other kind
//! (see `../../🚪️io/📤️export/🧵️serializers/🗿️artifacts/📄️pdf/🔖️1.4/✳️any/🦀️component.rs`).
//!
//! @see ../../🧪️oracle/🔣️.json — the three oracle registrations this binary reuses (pinned to the
//!      exact same versions) and the `fixtureManifests`/`probes` entries this binary's output feeds.
//! @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️note-layout-carrier-oracle-findings.md
//!      — the 16-witnessable/17-un-oracled split this corpus covers (16, no more, no less).

use std::collections::BTreeMap;

//#region 🧾️Json
/// 🧾️ The smallest JSON writer that can express a probe report — same shape as
/// `…✳️cad/🔬️probes/🦀️oracle-probe/src/main.rs`'s own, copied rather than shared because these two
/// crates must never depend on each other (each is its own `[workspace]`).
#[derive(Clone, Debug, PartialEq)]
enum Json {
    Bool(bool),
    Num(f64),
    Int(i64),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

impl Json {
    fn render(&self, out: &mut String) {
        match self {
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Int(i) => out.push_str(&i.to_string()),
            Json::Num(n) => out.push_str(&(if n.is_finite() { format!("{n:?}") } else { "null".to_string() })),
            Json::Str(s) => out.push_str(&format!("\"{}\"", escape(s))),
            Json::Arr(items) => {
                out.push('[');
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    item.render(out);
                }
                out.push(']');
            }
            Json::Obj(fields) => {
                out.push('{');
                for (i, (k, v)) in fields.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    out.push_str(&format!("\"{}\":", escape(k)));
                    v.render(out);
                }
                out.push('}');
            }
        }
    }
    fn to_text(&self) -> String {
        let mut out = String::new();
        self.render(&mut out);
        out
    }
}
fn obj(fields: Vec<(&str, Json)>) -> Json {
    Json::Obj(fields.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}
fn s(v: &str) -> Json {
    Json::Str(v.to_string())
}
fn nums(v: &[f64]) -> Json {
    Json::Arr(v.iter().map(|x| Json::Num(*x)).collect())
}
//#endregion 🧾️Json

//#region 🔬️Report
struct Report {
    probe: &'static str,
    engine: (&'static str, &'static str, &'static str),
    status: &'static str,
    measurements: Vec<(String, Json)>,
    diagnostics: Vec<(&'static str, String)>,
}
impl Report {
    fn new(probe: &'static str, engine: (&'static str, &'static str, &'static str)) -> Self {
        Report { probe, engine, status: "ok", measurements: Vec::new(), diagnostics: Vec::new() }
    }
    fn put(&mut self, key: &str, value: Json) {
        self.measurements.push((key.to_string(), value));
    }
    fn fail(&mut self, message: String) {
        self.status = "failed";
        self.diagnostics.push(("error", message));
    }
    fn emit(self, duration_ms: u128) -> String {
        let mut fields = vec![
            ("schema".to_string(), s("semio.repository-test.probe-report/v2")),
            ("probe".to_string(), s(self.probe)),
            ("probeVersion".to_string(), s("dxf@0.6 + quick-xml@0.42 + lopdf@0.44")),
            ("engine".to_string(), obj(vec![("family", s(self.engine.0)), ("implementation", s(self.engine.1)), ("version", s(self.engine.2))])),
            ("status".to_string(), s(self.status)),
            ("durationMs".to_string(), Json::Int(duration_ms as i64)),
            ("measurements".to_string(), Json::Obj(self.measurements)),
        ];
        if !self.diagnostics.is_empty() {
            fields.push(("diagnostics".to_string(), Json::Arr(self.diagnostics.into_iter().map(|(sev, msg)| obj(vec![("severity", s(sev)), ("message", s(&msg))])).collect())));
        }
        Json::Obj(fields).to_text()
    }
}
//#endregion 🔬️Report

//#region 🧬️Domain
/// 🧬️ One note block, reduced to exactly the fields the three carriers ever read — never note's
/// full production schema, which this crate deliberately does not link.
#[derive(Clone)]
enum Block {
    Text { id: &'static str, x: f64, y: f64, rotation: f64, width: f64, height: f64, visible: bool, font_size: f64, text: String },
    Ink { id: &'static str, x: f64, y: f64, rotation: f64, width: f64, height: f64, visible: bool, points: Vec<[f64; 2]>, stroke_width: f64 },
    Image { id: &'static str, x: f64, y: f64, rotation: f64, width: f64, height: f64, visible: bool, image_key: &'static str },
    Table { id: &'static str, x: f64, y: f64, rotation: f64, width: f64, height: f64, visible: bool },
    Math { id: &'static str, x: f64, y: f64, rotation: f64, width: f64, height: f64, visible: bool },
    Group { id: &'static str, x: f64, y: f64, rotation: f64, width: f64, height: f64, visible: bool, children: Vec<Block> },
}
impl Block {
    fn common(&self) -> (f64, f64, f64, f64, f64, bool) {
        match self {
            Block::Text { x, y, rotation, width, height, visible, .. }
            | Block::Ink { x, y, rotation, width, height, visible, .. }
            | Block::Image { x, y, rotation, width, height, visible, .. }
            | Block::Table { x, y, rotation, width, height, visible, .. }
            | Block::Math { x, y, rotation, width, height, visible, .. }
            | Block::Group { x, y, rotation, width, height, visible, .. } => (*x, *y, *rotation, *width, *height, *visible),
        }
    }
}

#[derive(Clone)]
struct Asset {
    mime: &'static str,
    bytes: Vec<u8>,
}

#[derive(Clone)]
struct NoteDoc {
    title: Option<String>,
    blocks: Vec<Block>,
    assets: BTreeMap<&'static str, Asset>,
}

/// 🧬️ `flatten_blocks` — parent BEFORE children, depth-first, exactly the order note's own
/// `flatten_blocks` walks (a `Group` node contributes itself AND every descendant as its own entry).
fn flatten<'a>(blocks: &'a [Block], out: &mut Vec<&'a Block>) {
    for block in blocks {
        out.push(block);
        if let Block::Group { children, .. } = block {
            flatten(children, out);
        }
    }
}
fn flatten_all(doc: &NoteDoc) -> Vec<&Block> {
    let mut out = Vec::new();
    flatten(&doc.blocks, &mut out);
    out
}
//#endregion 🧬️Domain

//#region 🖼️Assets
const TINY_PNG: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x08, 0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd, 0x8d, 0xb0, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82];
const TINY_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="2" height="2"><rect width="2" height="2" fill="#336699"/></svg>"##;

fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity((bytes.len() + 2) / 3 * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let n = (b0 as u32) << 16 | (b1 as u32) << 8 | b2 as u32;
        out.push(ALPHABET[(n >> 18 & 0x3f) as usize] as char);
        out.push(ALPHABET[(n >> 12 & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 { ALPHABET[(n >> 6 & 0x3f) as usize] as char } else { '=' });
        out.push(if chunk.len() > 2 { ALPHABET[(n & 0x3f) as usize] as char } else { '=' });
    }
    out
}
//#endregion 🖼️Assets

#[path = "🧫️recipes/🦀️.rs"]
mod recipes;
#[path = "🖊️dxf-codec/🦀️.rs"]
mod dxf_codec;
#[path = "🔣️svg-codec/🦀️.rs"]
mod svg_codec;
#[path = "📕️pdf-codec/🦀️.rs"]
mod pdf_codec;
#[path = "⌨️cli/🦀️.rs"]
mod cli;

fn main() {
    cli::run();
}
