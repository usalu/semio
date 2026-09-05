//! 🔬️ Third-party carrier readers for `s.stdio.semio@v1/🖊️drawing`.
//!
//! This binary READS. It parses an exported carrier with a third-party library — `quick-xml` for
//! SVG 1.1, the IxMilia `dxf` crate for DXF R12, `lopdf` for PDF 1.7 — and reports what that library
//! recovered from the bytes. It knows nothing about this repository's mutation vocabulary and it is
//! never asked what a mutation should produce: an oracle that predicts the expected result is this
//! repository's own semantics wearing a third-party name, which is the exact shape the
//! `reimplementation-registered-as-third-party` gate blocks.
//!
//! The carrier decides what is checkable, and that is why every probe here can answer
//! `"unsupported"`. Asking DXF about a stroke colour is not a comparison that passes — DXF has no
//! colour field for a path at all (`…🔄️dxf/🔖️r12/✳️any/🦀️.rs:140` binds `style` and never
//! reads it; `DxfLayer.color` is the literal `7`). An empty result reported as `ok` would let a
//! recolour mutation pass against evidence that was never in the file.
//!
//! Usage — one probe per invocation, one JSON body on stdout:
//!   semio-drawing-oracle-probe svg-structure  --input a.svg
//!   semio-drawing-oracle-probe dxf-entities   --input a.dxf
//!   semio-drawing-oracle-probe pdf-text       --input a.pdf
//!   semio-drawing-oracle-probe svg-compare    --input expected.svg --input actual.svg
//!   semio-drawing-oracle-probe dxf-compare    --input expected.dxf --input actual.dxf
//!   semio-drawing-oracle-probe pdf-compare    --input expected.pdf --input actual.pdf
//!   semio-drawing-oracle-probe style-compare  --input expected.svg --input actual.svg
//!   semio-drawing-oracle-probe gate-inputs    --out <dir>
//!
//! @see ../📜️script.ts — the wrapper that stamps the ProbeReport envelope around this output
//! @see ../../🔣️oracle.json — the oracle, probe and pipeline registrations

use std::collections::BTreeMap;
use std::fmt::Write as _;

//#region 🧾️Json
/// 🧾️ The smallest JSON value this probe needs. Hand-rolled on purpose: `serde_json` is a production
/// runtime dependency of this repository, so reaching for it inside an independence-critical probe
/// would put production code on the measurement path.
enum J {
    S(String),
    N(f64),
    B(bool),
    A(Vec<J>),
    O(Vec<(String, J)>),
}

// 🚫️async: pure formatting helper, no I/O.
fn esc(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

// 🚫️async: pure formatting helper, no I/O.
fn render(value: &J, out: &mut String) {
    match value {
        J::S(text) => {
            out.push('"');
            out.push_str(&esc(text));
            out.push('"');
        }
        J::N(number) => {
            if number.is_finite() {
                let _ = write!(out, "{number}");
            } else {
                out.push_str("null");
            }
        }
        J::B(flag) => out.push_str(if *flag { "true" } else { "false" }),
        J::A(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                render(item, out);
            }
            out.push(']');
        }
        J::O(fields) => {
            out.push('{');
            for (index, (key, item)) in fields.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                out.push('"');
                out.push_str(&esc(key));
                out.push_str("\":");
                render(item, out);
            }
            out.push('}');
        }
    }
}

// 🚫️async: pure formatting helper, no I/O.
fn emit(status: &str, measurements: J, diagnostics: Vec<(String, String, String)>) {
    let mut fields = vec![("status".to_string(), J::S(status.to_string())), ("measurements".to_string(), measurements)];
    if !diagnostics.is_empty() {
        fields.push((
            "diagnostics".to_string(),
            J::A(
                diagnostics
                    .into_iter()
                    .map(|(severity, message, detail)| J::O(vec![("severity".to_string(), J::S(severity)), ("message".to_string(), J::S(message)), ("detail".to_string(), J::S(detail))]))
                    .collect(),
            ),
        ));
    }
    let mut text = String::new();
    render(&J::O(fields), &mut text);
    println!("{text}");
}

// 🚫️async: pure formatting helper, no I/O.
fn unsupported(reason: &str) {
    emit("unsupported", J::O(vec![("reason".to_string(), J::S(reason.to_string()))]), Vec::new());
}

// 🚫️async: pure formatting helper, no I/O.
fn failed(message: &str, detail: &str) {
    emit("failed", J::O(Vec::new()), vec![("error".to_string(), message.to_string(), detail.to_string())]);
}
//#endregion 🧾️Json

//#region 🎨️Svg
/// 🎨️ One element as `quick-xml` recovered it: its structural address, its tag, and the presentation
/// attributes this subset's exporter actually writes. Nothing is inferred — an attribute the file does
/// not carry stays `None`, which is what lets a comparison distinguish "unchanged" from "absent".
#[derive(Clone, Debug, PartialEq)]
struct SvgNode {
    path: String,
    tag: String,
    id: Option<String>,
    transform: Option<String>,
    d: Option<String>,
    x: Option<String>,
    y: Option<String>,
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<String>,
    opacity: Option<String>,
    text: Option<String>,
}

// 🚫️async: pure parsing helper.
fn read_svg(path: &str) -> Result<Vec<SvgNode>, String> {
    let source = std::fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let mut reader = quick_xml::Reader::from_str(&source);
    let mut nodes: Vec<SvgNode> = Vec::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut siblings: Vec<usize> = vec![0];
    loop {
        match reader.read_event() {
            Err(error) => return Err(format!("{path}: xml parse error at {}: {error}", reader.buffer_position())),
            Ok(quick_xml::events::Event::Eof) => break,
            Ok(event @ (quick_xml::events::Event::Start(_) | quick_xml::events::Event::Empty(_))) => {
                let empty = matches!(event, quick_xml::events::Event::Empty(_));
                let element = match &event {
                    quick_xml::events::Event::Start(element) | quick_xml::events::Event::Empty(element) => element.clone(),
                    _ => unreachable!(),
                };
                let index = *siblings.last().unwrap_or(&0);
                let address = if stack.is_empty() { format!("/{index}") } else { format!("{}/{index}", nodes[stack.last().unwrap().0].path) };
                let mut node = SvgNode {
                    path: address,
                    tag: element.name().as_ref().to_string(),
                    id: None,
                    transform: None,
                    d: None,
                    x: None,
                    y: None,
                    fill: None,
                    stroke: None,
                    stroke_width: None,
                    opacity: None,
                    text: None,
                };
                for attribute in element.attributes().flatten() {
                    let key = attribute.key.as_ref().to_string();
                    let value = attribute.value.as_ref().to_string();
                    match key.as_str() {
                        "id" => node.id = Some(value),
                        "transform" => node.transform = Some(value),
                        "d" => node.d = Some(value),
                        "x" => node.x = Some(value),
                        "y" => node.y = Some(value),
                        "fill" => node.fill = Some(value),
                        "stroke" => node.stroke = Some(value),
                        "stroke-width" => node.stroke_width = Some(value),
                        "opacity" => node.opacity = Some(value),
                        _ => {}
                    }
                }
                nodes.push(node);
                let pushed = nodes.len() - 1;
                *siblings.last_mut().unwrap() += 1;
                // 🧭️An `Empty` element (`<path …/>`) never produces an `End`, so it must open and
                // close its own frame here. Treating it like a `Start` left the stack unbalanced and
                // every following sibling was reported as that element's child.
                if !empty {
                    stack.push((pushed, 0));
                    siblings.push(0);
                }
            }
            Ok(quick_xml::events::Event::End(_)) => {
                stack.pop();
                siblings.pop();
            }
            Ok(quick_xml::events::Event::Text(text)) => {
                let value = text.xml10_content().to_string();
                if !value.trim().is_empty() {
                    if let Some((owner, _)) = stack.last() {
                        let existing = nodes[*owner].text.take().unwrap_or_default();
                        nodes[*owner].text = Some(format!("{existing}{value}"));
                    }
                }
            }
            Ok(_) => {}
        }
    }
    Ok(nodes)
}

// 🚫️async: pure parsing helper.
fn numbers_of(text: &str) -> Vec<f64> {
    let mut found = Vec::new();
    let bytes = text.as_bytes();
    let mut at = 0usize;
    while at < bytes.len() {
        let c = bytes[at] as char;
        if c.is_ascii_digit() || ((c == '-' || c == '+' || c == '.') && at + 1 < bytes.len() && ((bytes[at + 1] as char).is_ascii_digit() || bytes[at + 1] == b'.')) {
            let start = at;
            at += 1;
            while at < bytes.len() {
                let n = bytes[at] as char;
                if n.is_ascii_digit() || n == '.' {
                    at += 1;
                } else if (n == 'e' || n == 'E') && at + 1 < bytes.len() {
                    at += 1;
                    if at < bytes.len() && (bytes[at] == b'-' || bytes[at] == b'+') {
                        at += 1;
                    }
                } else {
                    break;
                }
            }
            if let Ok(value) = text[start..at].parse::<f64>() {
                found.push(value);
            }
        } else {
            at += 1;
        }
    }
    found
}

// 🚫️async: pure parsing helper.
fn letters_of(text: &str) -> String {
    text.chars().filter(|c| c.is_ascii_alphabetic() && *c != 'e' && *c != 'E').collect()
}

/// 🎨️ Parses `rgba(r,g,b,a)`, `#rgb`, `#rrggbb` and `none` into 0..255 channels plus alpha, so a
/// recolour is measured as a channel distance rather than as a string inequality.
// 🚫️async: pure parsing helper.
fn color_channels(text: &str) -> Option<[f64; 4]> {
    let trimmed = text.trim();
    if trimmed.eq_ignore_ascii_case("none") {
        return None;
    }
    if let Some(hex) = trimmed.strip_prefix('#') {
        let expand = |slice: &str| u8::from_str_radix(slice, 16).ok().map(f64::from);
        if hex.len() == 6 {
            return Some([expand(&hex[0..2])?, expand(&hex[2..4])?, expand(&hex[4..6])?, 1.0]);
        }
        if hex.len() == 3 {
            let double = |c: char| u8::from_str_radix(&format!("{c}{c}"), 16).ok().map(f64::from);
            let mut chars = hex.chars();
            return Some([double(chars.next()?)?, double(chars.next()?)?, double(chars.next()?)?, 1.0]);
        }
        return None;
    }
    if trimmed.starts_with("rgb") {
        let parts = numbers_of(trimmed);
        if parts.len() >= 3 {
            return Some([parts[0], parts[1], parts[2], *parts.get(3).unwrap_or(&1.0)]);
        }
    }
    None
}
//#endregion 🎨️Svg

//#region ⚖️SvgCompare
/// ⚖️ The measured difference between two SVG carriers, all of it read out of the two files.
/// Geometry is gated NEAR-EXACT rather than in a tessellation tolerance, because there is no
/// legitimate re-tessellation of an SVG path: the `d` attribute IS the curve, and this subset's
/// exporter lowers a `PathSegment` to a command one-for-one
/// (`…🎨️svg/🔖️1.1/✳️any/🦀️.rs:27–38`). Two encodings may differ in number FORMATTING
/// (`10` against `10.0`) and in attribute ORDER, which is why the comparison is on parsed values and
/// not on bytes; they may not differ in a coordinate.
// 🚫️async: pure comparison over already-parsed values.
fn compare_svg(expected: &[SvgNode], actual: &[SvgNode]) -> (bool, J) {
    let mut differing: Vec<J> = Vec::new();
    let mut max_path_deviation = 0.0f64;
    let mut max_transform_deviation = 0.0f64;
    let mut max_origin_deviation = 0.0f64;
    let mut max_color_deviation = 0.0f64;
    let mut max_stroke_width_deviation = 0.0f64;
    let mut path_command_sequence_differs = false;
    let mut text_values_equal = true;

    let expected_shape: Vec<String> = expected.iter().map(|node| format!("{}:{}", node.path, node.tag)).collect();
    let actual_shape: Vec<String> = actual.iter().map(|node| format!("{}:{}", node.path, node.tag)).collect();
    let structure_equal = expected_shape == actual_shape;

    let paired = expected.len().min(actual.len());
    for index in 0..paired {
        let (before, after) = (&expected[index], &actual[index]);
        if before.path != after.path || before.tag != after.tag {
            continue;
        }
        let mut reasons: Vec<String> = Vec::new();
        let numeric = |left: &Option<String>, right: &Option<String>, label: &str, sink: &mut f64, reasons: &mut Vec<String>| match (left, right) {
            (Some(l), Some(r)) => {
                let (ln, rn) = (numbers_of(l), numbers_of(r));
                if letters_of(l) != letters_of(r) || ln.len() != rn.len() {
                    reasons.push(format!("{label} command sequence differs"));
                    return true;
                }
                for (a, b) in ln.iter().zip(rn.iter()) {
                    let delta = (a - b).abs();
                    if delta > *sink {
                        *sink = delta;
                    }
                    if delta > 0.0 {
                        reasons.push(format!("{label} differs by {delta}"));
                    }
                }
                false
            }
            (None, None) => false,
            _ => {
                reasons.push(format!("{label} present on one side only"));
                true
            }
        };
        if numeric(&before.d, &after.d, "d", &mut max_path_deviation, &mut reasons) {
            path_command_sequence_differs = true;
        }
        numeric(&before.transform, &after.transform, "transform", &mut max_transform_deviation, &mut reasons);
        numeric(&before.x, &after.x, "x", &mut max_origin_deviation, &mut reasons);
        numeric(&before.y, &after.y, "y", &mut max_origin_deviation, &mut reasons);
        numeric(&before.stroke_width, &after.stroke_width, "stroke-width", &mut max_stroke_width_deviation, &mut reasons);
        numeric(&before.opacity, &after.opacity, "opacity", &mut max_stroke_width_deviation, &mut reasons);
        for (left, right, label) in [(&before.fill, &after.fill, "fill"), (&before.stroke, &after.stroke, "stroke")] {
            match (left.as_deref().and_then(color_channels), right.as_deref().and_then(color_channels)) {
                (Some(l), Some(r)) => {
                    for channel in 0..4 {
                        let scale = if channel == 3 { 255.0 } else { 1.0 };
                        let delta = (l[channel] - r[channel]).abs() * scale;
                        if delta > max_color_deviation {
                            max_color_deviation = delta;
                        }
                        if delta > 0.0 {
                            reasons.push(format!("{label} channel {channel} differs by {delta}"));
                        }
                    }
                }
                (None, None) => {}
                _ => reasons.push(format!("{label} paint present on one side only")),
            }
        }
        if before.id != after.id {
            reasons.push("id differs".to_string());
        }
        if before.text != after.text {
            text_values_equal = false;
            reasons.push("text content differs".to_string());
        }
        if !reasons.is_empty() {
            differing.push(J::O(vec![
                ("path".to_string(), J::S(before.path.clone())),
                ("tag".to_string(), J::S(before.tag.clone())),
                ("reasons".to_string(), J::A(reasons.into_iter().map(J::S).collect())),
            ]));
        }
    }

    let equal = structure_equal
        && differing.is_empty()
        && !path_command_sequence_differs
        && text_values_equal
        && max_path_deviation == 0.0
        && max_transform_deviation == 0.0
        && max_origin_deviation == 0.0
        && max_color_deviation == 0.0
        && max_stroke_width_deviation == 0.0;

    (
        equal,
        J::O(vec![
            ("equal".to_string(), J::B(equal)),
            ("structureEqual".to_string(), J::B(structure_equal)),
            ("expectedElementCount".to_string(), J::N(expected.len() as f64)),
            ("actualElementCount".to_string(), J::N(actual.len() as f64)),
            ("pathCommandSequenceDiffers".to_string(), J::B(path_command_sequence_differs)),
            ("textValuesEqual".to_string(), J::B(text_values_equal)),
            ("maxPathPointDeviation".to_string(), J::N(max_path_deviation)),
            ("maxTransformComponentDeviation".to_string(), J::N(max_transform_deviation)),
            ("maxTextOriginDeviation".to_string(), J::N(max_origin_deviation)),
            ("maxColorChannelDeviation".to_string(), J::N(max_color_deviation)),
            ("maxStrokeWidthDeviation".to_string(), J::N(max_stroke_width_deviation)),
            ("differingElements".to_string(), J::A(differing)),
        ]),
    )
}
//#endregion ⚖️SvgCompare

//#region 🖊️Dxf
/// 🖊️ One entity as the IxMilia reader recovered it. `kind` is the DXF entity name, `points` its
/// coordinates in file order, `value` a TEXT entity's string.
#[derive(Clone, Debug, PartialEq)]
struct DxfItem {
    kind: String,
    layer: String,
    points: Vec<f64>,
    value: Option<String>,
}

// 🚫️async: pure parsing helper.
fn read_dxf(path: &str) -> Result<(Vec<(String, i16)>, Vec<DxfItem>), String> {
    let drawing = dxf::Drawing::load_file(path).map_err(|error| format!("{path}: {error}"))?;
    let layers = drawing.layers().map(|layer| (layer.name.clone(), layer.color.index().map_or(-1i16, i16::from))).collect();
    let mut items = Vec::new();
    for entity in drawing.entities() {
        let layer = entity.common.layer.clone();
        let item = match &entity.specific {
            dxf::entities::EntityType::Circle(circle) => DxfItem { kind: "CIRCLE".into(), layer, points: vec![circle.center.x, circle.center.y, circle.radius], value: None },
            dxf::entities::EntityType::Line(line) => DxfItem { kind: "LINE".into(), layer, points: vec![line.p1.x, line.p1.y, line.p2.x, line.p2.y], value: None },
            dxf::entities::EntityType::Text(text) => DxfItem { kind: "TEXT".into(), layer, points: vec![text.location.x, text.location.y], value: Some(text.value.clone()) },
            dxf::entities::EntityType::Polyline(polyline) => {
                let mut points = Vec::new();
                for vertex in polyline.vertices() {
                    points.push(vertex.location.x);
                    points.push(vertex.location.y);
                }
                DxfItem { kind: "POLYLINE".into(), layer, points, value: None }
            }
            dxf::entities::EntityType::LwPolyline(polyline) => {
                let mut points = Vec::new();
                for vertex in &polyline.vertices {
                    points.push(vertex.x);
                    points.push(vertex.y);
                }
                DxfItem { kind: "LWPOLYLINE".into(), layer, points, value: None }
            }
            other => DxfItem { kind: format!("{other:?}").split('(').next().unwrap_or("UNKNOWN").to_uppercase(), layer, points: Vec::new(), value: None },
        };
        items.push(item);
    }
    Ok((layers, items))
}

// 🚫️async: pure comparison over already-parsed values.
fn compare_dxf(expected: &(Vec<(String, i16)>, Vec<DxfItem>), actual: &(Vec<(String, i16)>, Vec<DxfItem>)) -> (bool, J) {
    let expected_layers: Vec<String> = expected.0.iter().map(|(name, _)| name.clone()).collect();
    let actual_layers: Vec<String> = actual.0.iter().map(|(name, _)| name.clone()).collect();
    let expected_kinds: Vec<String> = expected.1.iter().map(|item| format!("{}@{}", item.kind, item.layer)).collect();
    let actual_kinds: Vec<String> = actual.1.iter().map(|item| format!("{}@{}", item.kind, item.layer)).collect();
    let mut max_vertex_deviation = 0.0f64;
    let mut vertex_count_differs = false;
    let mut text_values_equal = true;
    let mut differing: Vec<J> = Vec::new();
    for (index, (before, after)) in expected.1.iter().zip(actual.1.iter()).enumerate() {
        let mut reasons: Vec<String> = Vec::new();
        if before.points.len() != after.points.len() {
            vertex_count_differs = true;
            reasons.push(format!("vertex count {} against {}", before.points.len(), after.points.len()));
        } else {
            for (a, b) in before.points.iter().zip(after.points.iter()) {
                let delta = (a - b).abs();
                if delta > max_vertex_deviation {
                    max_vertex_deviation = delta;
                }
                if delta > 0.0 {
                    reasons.push(format!("coordinate differs by {delta}"));
                }
            }
        }
        if before.value != after.value {
            text_values_equal = false;
            reasons.push("TEXT value differs".to_string());
        }
        if !reasons.is_empty() {
            differing.push(J::O(vec![("index".to_string(), J::N(index as f64)), ("kind".to_string(), J::S(before.kind.clone())), ("reasons".to_string(), J::A(reasons.into_iter().map(J::S).collect()))]));
        }
    }
    let equal = expected_layers == actual_layers && expected_kinds == actual_kinds && !vertex_count_differs && text_values_equal && max_vertex_deviation == 0.0;
    (
        equal,
        J::O(vec![
            ("equal".to_string(), J::B(equal)),
            ("layerNamesEqual".to_string(), J::B(expected_layers == actual_layers)),
            ("entitySequenceEqual".to_string(), J::B(expected_kinds == actual_kinds)),
            ("expectedLayerCount".to_string(), J::N(expected_layers.len() as f64)),
            ("actualLayerCount".to_string(), J::N(actual_layers.len() as f64)),
            ("expectedEntityCount".to_string(), J::N(expected.1.len() as f64)),
            ("actualEntityCount".to_string(), J::N(actual.1.len() as f64)),
            ("vertexCountDiffers".to_string(), J::B(vertex_count_differs)),
            ("textValuesEqual".to_string(), J::B(text_values_equal)),
            ("maxVertexDeviation".to_string(), J::N(max_vertex_deviation)),
            ("differingEntities".to_string(), J::A(differing)),
        ]),
    )
}
//#endregion 🖊️Dxf

//#region 📄️Pdf
// 🚫️async: pure parsing helper.
fn read_pdf(path: &str) -> Result<Vec<String>, String> {
    let document = lopdf::Document::load(path).map_err(|error| format!("{path}: {error}"))?;
    let pages: BTreeMap<u32, (u32, u16)> = document.get_pages();
    let mut texts = Vec::new();
    for number in pages.keys() {
        texts.push(document.extract_text(&[*number]).unwrap_or_default());
    }
    Ok(texts)
}
//#endregion 📄️Pdf

//#region 🚪️GateInputs
/// 🚪️ Writes the gate-validation inputs, each one built BY the third-party library's own writer.
///
/// These are NOT fixtures and they are not registered as any mutation's expected result. They exist to
/// answer one question the playbook insists on: does the comparison ACCEPT a known-good pair and
/// REJECT a known-bad one? `good-a`/`good-b` are the same drawing written twice with deliberately
/// different number formatting and attribute order — byte-different, semantically identical, so a
/// comparison that merely diffed bytes would fail them. `bad-geometry` and `bad-paint` each carry one
/// single, quantified error.
// 🚫️async: file writing is the whole point of this subcommand.
fn gate_inputs(out: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(out).map_err(|error| format!("{out}: {error}"))?;
    let mut written = Vec::new();

    let svg = |file: &str, d: &str, text_x: &str, text_y: &str, stroke: &str, width: &str, order_swapped: bool| -> Result<(), String> {
        let mut writer = quick_xml::Writer::new(Vec::new());
        use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
        let mut root = BytesStart::new("svg");
        root.push_attribute(("xmlns", "http://www.w3.org/2000/svg"));
        root.push_attribute(("viewBox", "0 0 100 50"));
        writer.write_event(Event::Start(root)).map_err(|e| e.to_string())?;
        let mut group = BytesStart::new("g");
        group.push_attribute(("id", "layer-l1"));
        writer.write_event(Event::Start(group)).map_err(|e| e.to_string())?;
        let mut path = BytesStart::new("path");
        if order_swapped {
            path.push_attribute(("stroke", stroke));
            path.push_attribute(("stroke-width", width));
            path.push_attribute(("d", d));
        } else {
            path.push_attribute(("d", d));
            path.push_attribute(("stroke", stroke));
            path.push_attribute(("stroke-width", width));
        }
        path.push_attribute(("fill", "rgba(255,0,0,1)"));
        writer.write_event(Event::Empty(path)).map_err(|e| e.to_string())?;
        let mut label = BytesStart::new("text");
        label.push_attribute(("x", text_x));
        label.push_attribute(("y", text_y));
        writer.write_event(Event::Start(label)).map_err(|e| e.to_string())?;
        writer.write_event(Event::Text(BytesText::new("Hello"))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("text"))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("g"))).map_err(|e| e.to_string())?;
        writer.write_event(Event::End(BytesEnd::new("svg"))).map_err(|e| e.to_string())?;
        let target = format!("{out}/{file}");
        std::fs::write(&target, writer.into_inner()).map_err(|error| format!("{target}: {error}"))?;
        Ok(())
    };

    svg("good-a.svg", "M 0 0 L 10 0 Z", "5", "5", "rgba(0,0,0,1)", "1", false)?;
    svg("good-b.svg", "M 0.0 0.0 L 10.00 0.0 Z", "5.0", "5.000", "rgba(0,0,0,1.0)", "1.0", true)?;
    svg("bad-geometry.svg", "M 0 0 L 10.05 0 Z", "5", "5", "rgba(0,0,0,1)", "1", false)?;
    svg("bad-paint.svg", "M 0 0 L 10 0 Z", "5", "5", "rgba(5,0,0,1)", "1", false)?;
    for name in ["good-a.svg", "good-b.svg", "bad-geometry.svg", "bad-paint.svg"] {
        written.push(name.to_string());
    }

    let dxf_file = |file: &str, second_x: f64, value: &str| -> Result<(), String> {
        let mut drawing = dxf::Drawing::new();
        drawing.add_layer(dxf::tables::Layer { name: "l1".to_string(), ..Default::default() });
        let mut polyline = dxf::entities::Polyline::default();
        for (x, y) in [(0.0f64, 0.0f64), (second_x, 0.0f64)] {
            polyline.add_vertex(&mut drawing, dxf::entities::Vertex { location: dxf::Point::new(x, y, 0.0), ..Default::default() });
        }
        drawing.add_entity(dxf::entities::Entity { common: dxf::entities::EntityCommon { layer: "l1".to_string(), ..Default::default() }, specific: dxf::entities::EntityType::Polyline(polyline) });
        drawing.add_entity(dxf::entities::Entity {
            common: dxf::entities::EntityCommon { layer: "l1".to_string(), ..Default::default() },
            specific: dxf::entities::EntityType::Text(dxf::entities::Text { location: dxf::Point::new(5.0, 5.0, 0.0), value: value.to_string(), text_height: 1.0, ..Default::default() }),
        });
        let target = format!("{out}/{file}");
        drawing.save_file(&target).map_err(|error| format!("{target}: {error}"))?;
        Ok(())
    };
    dxf_file("good-a.dxf", 10.0, "Hello")?;
    dxf_file("good-b.dxf", 10.0, "Hello")?;
    dxf_file("bad-geometry.dxf", 10.05, "Hello")?;
    dxf_file("bad-text.dxf", 10.0, "Hallo")?;
    for name in ["good-a.dxf", "good-b.dxf", "bad-geometry.dxf", "bad-text.dxf"] {
        written.push(name.to_string());
    }
    Ok(written)
}
//#endregion 🚪️GateInputs

//#region 🚀️Entry
// 🚫️async: process entry point.
fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let probe = argv.first().cloned().unwrap_or_default();
    let mut inputs: Vec<String> = Vec::new();
    let mut out = String::new();
    let mut at = 1usize;
    while at < argv.len() {
        if argv[at] == "--input" && at + 1 < argv.len() {
            inputs.push(argv[at + 1].clone());
            at += 2;
        } else if argv[at] == "--out" && at + 1 < argv.len() {
            out = argv[at + 1].clone();
            at += 2;
        } else {
            at += 1;
        }
    }
    let extension = |path: &str| path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    let need = |count: usize| -> bool {
        if inputs.len() < count {
            failed("missing inputs", &format!("{probe} needs {count} --input path(s), got {}", inputs.len()));
            false
        } else {
            true
        }
    };

    match probe.as_str() {
        "svg-structure" => {
            if !need(1) {
                return;
            }
            if extension(&inputs[0]) != "svg" {
                return unsupported(&format!(".{} is not an SVG carrier", extension(&inputs[0])));
            }
            match read_svg(&inputs[0]) {
                Err(error) => failed("svg parse failed", &error),
                Ok(nodes) => {
                    let listed: Vec<J> = nodes
                        .iter()
                        .map(|node| {
                            let mut fields = vec![("path".to_string(), J::S(node.path.clone())), ("tag".to_string(), J::S(node.tag.clone()))];
                            for (key, value) in [("id", &node.id), ("transform", &node.transform), ("d", &node.d), ("x", &node.x), ("y", &node.y), ("fill", &node.fill), ("stroke", &node.stroke), ("strokeWidth", &node.stroke_width), ("opacity", &node.opacity), ("text", &node.text)] {
                                if let Some(text) = value {
                                    fields.push((key.to_string(), J::S(text.clone())));
                                }
                            }
                            J::O(fields)
                        })
                        .collect();
                    emit("ok", J::O(vec![("parsed".to_string(), J::B(true)), ("elementCount".to_string(), J::N(nodes.len() as f64)), ("elements".to_string(), J::A(listed))]), Vec::new());
                }
            }
        }
        "dxf-entities" => {
            if !need(1) {
                return;
            }
            if extension(&inputs[0]) != "dxf" {
                return unsupported(&format!(".{} is not a DXF carrier", extension(&inputs[0])));
            }
            match read_dxf(&inputs[0]) {
                Err(error) => failed("dxf parse failed", &error),
                Ok((layers, items)) => {
                    let listed: Vec<J> = items
                        .iter()
                        .map(|item| {
                            let mut fields = vec![("kind".to_string(), J::S(item.kind.clone())), ("layer".to_string(), J::S(item.layer.clone())), ("points".to_string(), J::A(item.points.iter().map(|p| J::N(*p)).collect()))];
                            if let Some(value) = &item.value {
                                fields.push(("value".to_string(), J::S(value.clone())));
                            }
                            J::O(fields)
                        })
                        .collect();
                    emit(
                        "ok",
                        J::O(vec![
                            ("parsed".to_string(), J::B(true)),
                            ("layers".to_string(), J::A(layers.iter().map(|(name, color)| J::O(vec![("name".to_string(), J::S(name.clone())), ("color".to_string(), J::N(f64::from(*color)))])).collect())),
                            ("entityCount".to_string(), J::N(items.len() as f64)),
                            ("entities".to_string(), J::A(listed)),
                        ]),
                        Vec::new(),
                    );
                }
            }
        }
        "pdf-text" => {
            if !need(1) {
                return;
            }
            if extension(&inputs[0]) != "pdf" {
                return unsupported(&format!(".{} is not a PDF carrier", extension(&inputs[0])));
            }
            match read_pdf(&inputs[0]) {
                Err(error) => failed("pdf parse failed", &error),
                Ok(pages) => emit("ok", J::O(vec![("parsed".to_string(), J::B(true)), ("pageCount".to_string(), J::N(pages.len() as f64)), ("pageText".to_string(), J::A(pages.into_iter().map(J::S).collect()))]), Vec::new()),
            }
        }
        "svg-compare" | "style-compare" => {
            if !need(2) {
                return;
            }
            for input in inputs.iter().take(2) {
                if extension(input) != "svg" {
                    // ✘️SVG is the ONLY carrier of this subset that encodes paint and stroke width.
                    // DXF binds a node's style and never reads it; PDF carries text alone. Answering
                    // `ok` with an empty paint set would let a recolour pass against a file that
                    // never carried a colour.
                    return unsupported(&format!(".{} does not encode SVG structure, paint or stroke width", extension(input)));
                }
            }
            match (read_svg(&inputs[0]), read_svg(&inputs[1])) {
                (Err(error), _) | (_, Err(error)) => failed("svg parse failed", &error),
                (Ok(expected), Ok(actual)) => {
                    let (_, measurements) = compare_svg(&expected, &actual);
                    emit("ok", measurements, Vec::new());
                }
            }
        }
        "dxf-compare" => {
            if !need(2) {
                return;
            }
            for input in inputs.iter().take(2) {
                if extension(input) != "dxf" {
                    return unsupported(&format!(".{} is not a DXF carrier", extension(input)));
                }
            }
            match (read_dxf(&inputs[0]), read_dxf(&inputs[1])) {
                (Err(error), _) | (_, Err(error)) => failed("dxf parse failed", &error),
                (Ok(expected), Ok(actual)) => {
                    let (_, measurements) = compare_dxf(&expected, &actual);
                    emit("ok", measurements, Vec::new());
                }
            }
        }
        "pdf-compare" => {
            if !need(2) {
                return;
            }
            for input in inputs.iter().take(2) {
                if extension(input) != "pdf" {
                    return unsupported(&format!(".{} is not a PDF carrier", extension(input)));
                }
            }
            match (read_pdf(&inputs[0]), read_pdf(&inputs[1])) {
                (Err(error), _) | (_, Err(error)) => failed("pdf parse failed", &error),
                (Ok(expected), Ok(actual)) => {
                    let differing: Vec<J> = expected
                        .iter()
                        .zip(actual.iter())
                        .enumerate()
                        .filter(|(_, (before, after))| before != after)
                        .map(|(index, (before, after))| J::O(vec![("page".to_string(), J::N(index as f64)), ("expected".to_string(), J::S(before.clone())), ("actual".to_string(), J::S(after.clone()))]))
                        .collect();
                    let equal = expected.len() == actual.len() && differing.is_empty();
                    emit(
                        "ok",
                        J::O(vec![
                            ("equal".to_string(), J::B(equal)),
                            ("pageCountEqual".to_string(), J::B(expected.len() == actual.len())),
                            ("expectedPageCount".to_string(), J::N(expected.len() as f64)),
                            ("actualPageCount".to_string(), J::N(actual.len() as f64)),
                            ("differingPages".to_string(), J::A(differing)),
                        ]),
                        Vec::new(),
                    );
                }
            }
        }
        "gate-inputs" => {
            if out.is_empty() {
                return failed("missing --out", "gate-inputs needs --out <dir>");
            }
            match gate_inputs(&out) {
                Err(error) => failed("gate input generation failed", &error),
                Ok(written) => emit("ok", J::O(vec![("written".to_string(), J::A(written.into_iter().map(J::S).collect())), ("out".to_string(), J::S(out))]), Vec::new()),
            }
        }
        other => failed("unknown probe", &format!("{other:?} — known: svg-structure, dxf-entities, pdf-text, svg-compare, style-compare, dxf-compare, pdf-compare, gate-inputs")),
    }
}
//#endregion 🚀️Entry
