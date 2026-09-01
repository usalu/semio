//! 🎨️ SVG carrier — writer AND reader, both `quick-xml` 0.42, matching
//! `quick-xml-note-drawing-reader` in `../../🧪️oracle/🔣️.json`. Reproduces the structural shape
//! `note_document_to_drawing_snapshot` + `draw_node_from_note_block` + `write_svg_xml` build (see
//! `../../🚪️io/🦀️component.rs`): every VISIBLE block, flattened, wrapped in
//! `<g transform="matrix(a,b,c,d,e,f)">` (the block's own x/y/rotation), then kind-specific content —
//! Ink -> `<path d="…" stroke-width="…"/>`, Text -> `<text x="0" y="{font_size}">…</text>` (font-size
//! wired to the Y COORDINATE, never a size attribute — the real subject bug this carrier surfaces,
//! reproduced here rather than "corrected"), Image -> `<image width height href="data:…"/>` when the
//! referenced asset resolves else a fallback `<rect>`, Table/Math/Group -> always `<rect width height/>`.

use crate::{flatten_all, obj, s, Block, Json, NoteDoc};
use quick_xml::events::{BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use std::io::Cursor;

/// 🔢️ `+0.0` never `-0.0` — `(-0.0_f64).to_string()` prints `"-0"`, which is a legal but needlessly
/// confusing `matrix(...)` component for a 0° rotation's `-sin(0)`. Normalized here rather than at
/// every call site.
fn fmt(v: f64) -> String {
    format!("{}", if v == 0.0 { 0.0 } else { v })
}

/// ✖️ `[a,b,c,d,e,f]`, the SVG `matrix(a,b,c,d,e,f)` layout — a plain Z-axis rotation composed with
/// translation, matching `note_block_transform`'s quaternion-around-Z construction (see
/// `../../🚪️io/🦀️component.rs`; a 0° rotation reduces to the pure-translate `matrix(1,0,0,1,x,y)`
/// shape the ticket's scratch crate already proved `quick-xml` reads back verbatim).
fn matrix_for(x: f64, y: f64, rotation_degrees: f64) -> [f64; 6] {
    let theta = rotation_degrees.to_radians();
    let (sin, cos) = theta.sin_cos();
    [cos, sin, -sin, cos, x, y]
}

fn bounds(doc: &NoteDoc) -> (f64, f64) {
    let mut max_x = 1024.0_f64;
    let mut max_y = 1024.0_f64;
    for block in flatten_all(doc) {
        let (x, y, _rotation, width, height, visible) = block.common();
        if !visible {
            continue;
        }
        max_x = max_x.max(x + width);
        max_y = max_y.max(y + height);
    }
    (max_x.max(1.0).round(), max_y.max(1.0).round())
}

fn path_d(points: &[[f64; 2]]) -> String {
    let mut d = format!("M{},{}", fmt(points[0][0]), fmt(points[0][1]));
    for point in &points[1..] {
        d.push_str(&format!(" L{},{}", fmt(point[0]), fmt(point[1])));
    }
    d
}

fn write_block_content<W: std::io::Write>(writer: &mut Writer<W>, doc: &NoteDoc, block: &Block) -> Result<(), std::io::Error> {
    match block {
        Block::Text { font_size, text, .. } => {
            writer.create_element("text").with_attribute(("x", "0")).with_attribute(("y", fmt(*font_size).as_str())).write_text_content(BytesText::new(text))?;
        }
        Block::Ink { points, stroke_width, .. } => {
            let d = path_d(points);
            writer.create_element("path").with_attribute(("d", d.as_str())).with_attribute(("stroke-width", fmt(*stroke_width).as_str())).write_empty()?;
        }
        Block::Image { width, height, image_key, .. } => match doc.assets.get(image_key) {
            Some(asset) => {
                let href = format!("data:{};base64,{}", asset.mime, crate::base64_encode(&asset.bytes));
                writer.create_element("image").with_attribute(("width", fmt(*width).as_str())).with_attribute(("height", fmt(*height).as_str())).with_attribute(("href", href.as_str())).write_empty()?;
            }
            None => {
                writer.create_element("rect").with_attribute(("width", fmt(*width).as_str())).with_attribute(("height", fmt(*height).as_str())).write_empty()?;
            }
        },
        Block::Table { width, height, .. } | Block::Math { width, height, .. } | Block::Group { width, height, .. } => {
            writer.create_element("rect").with_attribute(("width", fmt(*width).as_str())).with_attribute(("height", fmt(*height).as_str())).write_empty()?;
        }
    }
    Ok(())
}

/// ✍️ Writes real SVG XML with `quick_xml::writer::Writer` — nothing hand-formatted.
pub fn write_svg(doc: &NoteDoc) -> Result<Vec<u8>, String> {
    let (width, height) = bounds(doc);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer
        .create_element("svg")
        .with_attribute(("xmlns", "http://www.w3.org/2000/svg"))
        .with_attribute(("width", fmt(width).as_str()))
        .with_attribute(("height", fmt(height).as_str()))
        .write_inner_content(|writer| -> std::io::Result<()> {
            writer.create_element("g").with_attribute(("id", "layer-0")).write_inner_content(|writer| -> std::io::Result<()> {
                for block in flatten_all(doc) {
                    let (x, y, rotation, _w, _h, visible) = block.common();
                    if !visible {
                        continue;
                    }
                    let m = matrix_for(x, y, rotation);
                    let transform = format!("matrix({},{},{},{},{},{})", fmt(m[0]), fmt(m[1]), fmt(m[2]), fmt(m[3]), fmt(m[4]), fmt(m[5]));
                    writer.create_element("g").with_attribute(("transform", transform.as_str())).write_inner_content(|writer| write_block_content(writer, doc, block))?;
                }
                Ok(())
            })?;
            Ok(())
        })
        .map_err(|e| format!("quick-xml write: {e}"))?;
    Ok(writer.into_inner().into_inner())
}

/// 📐️ One block's reading, exactly what a third-party SVG reader can recover — never our schema.
#[derive(Debug, Default, Clone)]
pub struct BlockReading {
    pub transform: [f64; 6],
    pub kind: String,
    pub text: Option<String>,
    pub d: Option<String>,
    pub stroke_width: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
    pub href: Option<String>,
}

fn parse_matrix(text: &str) -> Result<[f64; 6], String> {
    let inner = text.trim().strip_prefix("matrix(").and_then(|rest| rest.strip_suffix(')')).ok_or_else(|| format!("not a matrix(...) transform: {text:?}"))?;
    let parts: Vec<f64> = inner.split(',').map(|p| p.trim().parse::<f64>().map_err(|e| format!("bad matrix component {p:?}: {e}"))).collect::<Result<_, _>>()?;
    if parts.len() != 6 {
        return Err(format!("expected 6 matrix components, got {}", parts.len()));
    }
    Ok([parts[0], parts[1], parts[2], parts[3], parts[4], parts[5]])
}

/// 📖 Reads SVG bytes with `quick_xml::reader::Reader` and reduces each block's `<g transform>` +
/// content element to a `BlockReading`, walking the fixed `svg > g#layer-0 > g[transform] > content`
/// shape by DEPTH — independent of this file's own writer, the same crate reading back what it (or
/// note's real serializer) wrote.
pub fn project_svg(bytes: &[u8]) -> Result<Vec<BlockReading>, String> {
    let mut reader = Reader::from_reader(bytes);
    let mut buf = Vec::new();
    let mut depth: i32 = 0;
    let mut current: Option<BlockReading> = None;
    let mut out = Vec::new();
    loop {
        let event = reader.read_event_into(&mut buf).map_err(|e| format!("quick-xml read: {e}"))?;
        match &event {
            Event::Eof => break,
            Event::Start(e) | Event::Empty(e) => {
                let is_empty = matches!(event, Event::Empty(_));
                let name: String = e.name().as_ref().to_string();
                let attr = |key: &str| -> Option<String> { e.attributes().flatten().find(|a| a.key.as_ref() == key).map(|a| a.value.into_owned()) };
                if depth == 2 && name == "g" {
                    let transform = attr("transform").ok_or("block <g> missing transform attribute")?;
                    current = Some(BlockReading { transform: parse_matrix(&transform)?, ..Default::default() });
                } else if depth == 3 {
                    if let Some(reading) = current.as_mut() {
                        reading.kind = name;
                        reading.d = attr("d");
                        reading.stroke_width = attr("stroke-width").and_then(|v| v.parse().ok());
                        reading.width = attr("width").and_then(|v| v.parse().ok());
                        reading.height = attr("height").and_then(|v| v.parse().ok());
                        reading.href = attr("href");
                    }
                }
                if !is_empty {
                    depth += 1;
                }
            }
            Event::Text(t) => {
                if depth == 4 {
                    if let Some(reading) = current.as_mut() {
                        reading.text = Some(t.as_ref().to_string());
                    }
                }
            }
            Event::End(_) => {
                if depth == 3 {
                    if let Some(reading) = current.take() {
                        out.push(reading);
                    }
                }
                depth -= 1;
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(out)
}

pub fn project_svg_json(bytes: &[u8]) -> Result<Json, String> {
    let readings = project_svg(bytes)?;
    Ok(obj(vec![
        ("blockCount", Json::Int(readings.len() as i64)),
        (
            "blocks",
            Json::Arr(
                readings
                    .into_iter()
                    .map(|r| {
                        obj(vec![
                            ("transform", crate::nums(&r.transform)),
                            ("kind", s(&r.kind)),
                            ("text", r.text.as_deref().map(s).unwrap_or(Json::Str(String::new()))),
                            ("d", r.d.as_deref().map(s).unwrap_or(Json::Str(String::new()))),
                            ("strokeWidth", r.stroke_width.map(Json::Num).unwrap_or(Json::Bool(false))),
                            ("width", r.width.map(Json::Num).unwrap_or(Json::Bool(false))),
                            ("height", r.height.map(Json::Num).unwrap_or(Json::Bool(false))),
                            ("href", r.href.as_deref().map(s).unwrap_or(Json::Str(String::new()))),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]))
}

/// ⚖️ Positional comparison — block ORDER is `flatten_blocks`' deterministic document order, not
/// writer freedom, per `semantic-note-svg-drawing-v1` (no `"arrays": "set"` declared, unlike the DXF
/// profile).
pub fn compare_svg(expected: &[u8], actual: &[u8]) -> Result<(bool, Vec<String>), String> {
    let e = project_svg(expected)?;
    let a = project_svg(actual)?;
    let mut problems = Vec::new();
    if e.len() != a.len() {
        problems.push(format!("block count differs: expected {} actual {}", e.len(), a.len()));
    }
    const TOL: f64 = 1e-9;
    for (i, (eb, ab)) in e.iter().zip(a.iter()).enumerate() {
        if eb.kind != ab.kind {
            problems.push(format!("block[{i}] kind differs: {:?} vs {:?}", eb.kind, ab.kind));
            continue;
        }
        for j in 0..6 {
            if (eb.transform[j] - ab.transform[j]).abs() > TOL {
                problems.push(format!("block[{i}].transform[{j}] differs: {} vs {}", eb.transform[j], ab.transform[j]));
            }
        }
        if eb.text != ab.text {
            problems.push(format!("block[{i}].text differs: {:?} vs {:?}", eb.text, ab.text));
        }
        if eb.d != ab.d {
            problems.push(format!("block[{i}].d differs: {:?} vs {:?}", eb.d, ab.d));
        }
        if eb.stroke_width != ab.stroke_width {
            problems.push(format!("block[{i}].strokeWidth differs: {:?} vs {:?}", eb.stroke_width, ab.stroke_width));
        }
        if eb.width != ab.width {
            problems.push(format!("block[{i}].width differs: {:?} vs {:?}", eb.width, ab.width));
        }
        if eb.height != ab.height {
            problems.push(format!("block[{i}].height differs: {:?} vs {:?}", eb.height, ab.height));
        }
        if eb.href != ab.href {
            problems.push(format!("block[{i}].href differs (image payload)"));
        }
    }
    Ok((problems.is_empty(), problems))
}
