//! 🔬️ Third-party carrier reader for `s.draw.draw@1/✳️any`.
//!
//! This binary READS. It parses the exported SVG 1.1 markup with `quick-xml` — an already-approved
//! `test-oracle` entry (`🔒️dependencies.json`, registered for `semio@v1/drawing`'s, `svg@1.1/any`'s,
//! `bcf`'s, `docx`'s, `xlsx`'s and `pptx`'s own oracles) — and reports what that library recovered
//! from the bytes. It knows nothing about this repository's mutation vocabulary and it is never
//! asked what a mutation should produce: an oracle that predicts the expected result is this
//! repository's own semantics wearing a third-party name, which is the exact shape the
//! `reimplementation-registered-as-third-party` gate blocks.
//!
//! WHY THIS READS THE SAME ATTRIBUTE VOCABULARY AS `semio@v1/drawing`'s OWN PROBE. `draw`'s
//! `DrawIntoSvg` serializer does not hand-roll SVG itself — it builds a `SemioDrawingSnapshot`
//! (`…🖍️draw/…/🚪️io/🦀️component.rs:144-154`) and dispatches through `io_dispatch` into
//! `semio@v1/drawing`'s OWN svg bridge, which renders through the SAME `write_svg_xml`
//! (`…🖍️draw/…/🚪️io/🦀️component.rs:159-179`). So the wire vocabulary this probe reads —
//! `id`/`transform`/`d`/`x`/`y`/`fill`/`stroke`/`stroke-width`/`opacity` — is not guessed; it is the
//! same vocabulary `semio@v1/drawing`'s own already-qualified `quick-xml-drawing-svg-reader` reads.
//!
//! WHAT THE BRIDGE DROPS, READ FROM `…🖍️draw/…/🚪️io/🦀️component.rs`. Every `SemioDrawNode` variant
//! (`Group`/`Path`/`Text`/`Image`) carries no `id` field at all (lines 121-139), so a `draw` layer's
//! `id`/`name` has no representation anywhere downstream — `rename-layer` is NOT witnessable here.
//! `blend_mode` and `fill_rule` are read off the scene node (lines 177-181 of the schema) but never
//! forwarded into `intern_semio_style` (lines 98-109) — `set-layer-blend-mode` is NOT witnessable.
//! `locked` has no field on `DrawSceneNode` at all — `set-layer-locked` is NOT witnessable. An
//! `Image` leaf never references its interned style (line 130), so fill/stroke/opacity mutations on
//! an IMAGE layer specifically are invisible here even though the mutation itself is general;
//! fixtures below target Path/Text layers, where the style IS referenced.
//!
//! Usage — one probe per invocation, one JSON body on stdout:
//!   semio-draw-oracle-probe svg-structure --input <a.svg>
//!   semio-draw-oracle-probe svg-compare   --input <expected.svg> --input <actual.svg>
//!   semio-draw-oracle-probe gate-inputs   --out <dir>
//!   semio-draw-oracle-probe fixtures      --out <dir>
//!
//! @see ../📜️script.ts — the wrapper that stamps the ProbeReport envelope around this output
//! @see ../../🧪️oracle/🔣️.json — the oracle, probe and pipeline registrations
//! @see ../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️drawing/🔬️probes/🦀️oracle-probe/🦀️component.rs
//!      — the sibling probe this file's SVG reader/comparator mirrors field-for-field

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
/// attributes `draw`'s exported carrier actually carries. Nothing is inferred — an attribute the file
/// does not carry stays `None`.
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
                let mut node = SvgNode { path: address, tag: element.name().as_ref().to_string(), id: None, transform: None, d: None, x: None, y: None, fill: None, stroke: None, stroke_width: None, opacity: None, text: None };
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
/// ⚖️ The measured difference between two SVG carriers, all of it read out of the two files. Geometry
/// is gated NEAR-EXACT — there is no legitimate re-tessellation of an SVG path here, only number
/// FORMATTING and attribute ORDER freedom, which is why the comparison is on parsed values and not on
/// bytes.
// 🚫️async: pure comparison over already-parsed values.
fn compare_svg(expected: &[SvgNode], actual: &[SvgNode]) -> J {
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
            differing.push(J::O(vec![("path".to_string(), J::S(before.path.clone())), ("tag".to_string(), J::S(before.tag.clone())), ("reasons".to_string(), J::A(reasons.into_iter().map(J::S).collect()))]));
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
    ])
}
//#endregion ⚖️SvgCompare

//#region 🏭️Fixtures
/// 🏭️ Writes an SVG document directly with `quick-xml`'s own `Writer` — never by calling this
/// repository's `write_svg_xml`. One `<g transform="matrix(...)">` per scene node (mirroring
/// `draw_document_to_svg`'s own one-group-per-node shape), wrapping one `<path>` or `<text>` leaf.
struct SceneNode<'a> {
    matrix: [f64; 6],
    kind: &'a str, // "path" | "text"
    d: Option<&'a str>,
    text: Option<&'a str>,
    fill: Option<&'a str>,
    stroke: Option<&'a str>,
    stroke_width: Option<&'a str>,
    opacity: Option<&'a str>,
}

fn write_scene(target: &str, canvas: (u32, u32), nodes: &[SceneNode]) -> Result<(), String> {
    use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
    let mut writer = quick_xml::Writer::new(Vec::new());
    let mut root = BytesStart::new("svg");
    root.push_attribute(("xmlns", "http://www.w3.org/2000/svg"));
    root.push_attribute(("viewBox", format!("0 0 {} {}", canvas.0, canvas.1).as_str()));
    writer.write_event(Event::Start(root)).map_err(|e| e.to_string())?;
    for node in nodes {
        let mut group = BytesStart::new("g");
        let m = node.matrix;
        group.push_attribute(("transform", format!("matrix({} {} {} {} {} {})", m[0], m[1], m[2], m[3], m[4], m[5]).as_str()));
        writer.write_event(Event::Start(group)).map_err(|e| e.to_string())?;
        if node.kind == "path" {
            let mut path = BytesStart::new("path");
            path.push_attribute(("d", node.d.unwrap_or("")));
            if let Some(v) = node.fill {
                path.push_attribute(("fill", v));
            }
            if let Some(v) = node.stroke {
                path.push_attribute(("stroke", v));
            }
            if let Some(v) = node.stroke_width {
                path.push_attribute(("stroke-width", v));
            }
            if let Some(v) = node.opacity {
                path.push_attribute(("opacity", v));
            }
            writer.write_event(Event::Empty(path)).map_err(|e| e.to_string())?;
        } else {
            let mut text_el = BytesStart::new("text");
            text_el.push_attribute(("x", "0"));
            text_el.push_attribute(("y", "0"));
            if let Some(v) = node.fill {
                text_el.push_attribute(("fill", v));
            }
            if let Some(v) = node.opacity {
                text_el.push_attribute(("opacity", v));
            }
            writer.write_event(Event::Start(text_el)).map_err(|e| e.to_string())?;
            writer.write_event(Event::Text(BytesText::new(node.text.unwrap_or("")))).map_err(|e| e.to_string())?;
            writer.write_event(Event::End(BytesEnd::new("text"))).map_err(|e| e.to_string())?;
        }
        writer.write_event(Event::End(BytesEnd::new("g"))).map_err(|e| e.to_string())?;
    }
    writer.write_event(Event::End(BytesEnd::new("svg"))).map_err(|e| e.to_string())?;
    std::fs::write(target, writer.into_inner()).map_err(|error| format!("{target}: {error}"))
}

const IDENTITY: [f64; 6] = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0];

fn gate_rect(d: &str) -> SceneNode<'_> {
    SceneNode { matrix: IDENTITY, kind: "path", d: Some(d), text: None, fill: Some("rgba(255,0,0,1)"), stroke: Some("rgba(0,0,0,1)"), stroke_width: Some("1"), opacity: None }
}

fn gate_inputs(out: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(out).map_err(|error| format!("{out}: {error}"))?;
    write_scene(&format!("{out}/good-a.svg"), (100, 50), &[gate_rect("M 0 0 L 10 0 L 10 10 L 0 10 Z")])?;
    write_scene(&format!("{out}/good-b.svg"), (100, 50), &[SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0.0 0.0 L 10.00 0.0 L 10.0 10.0 L 0.0 10.0 Z"), text: None, fill: Some("rgba(255,0,0,1.0)"), stroke: Some("rgba(0,0,0,1)"), stroke_width: Some("1.0"), opacity: None }])?;
    write_scene(&format!("{out}/bad-geometry.svg"), (100, 50), &[gate_rect("M 0 0 L 10.05 0 L 10 10 L 0 10 Z")])?;
    write_scene(&format!("{out}/bad-paint.svg"), (100, 50), &[SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(5,0,0,1)"), stroke: Some("rgba(0,0,0,1)"), stroke_width: Some("1"), opacity: None }])?;
    Ok(["good-a.svg", "good-b.svg", "bad-geometry.svg", "bad-paint.svg"].map(String::from).to_vec())
}

/// 🏭️ Writes the nine mutation-recipe BEFORE/AFTER pairs this pass built fixtures for — the eleven
/// witnessable kinds minus `set-layer-boolean-operation`/`update-layer-trace-params`, whose real
/// geometry comes out of this repository's own 2D boolean/trace kernel and was not hand-replicated
/// here (see this subset's 🧪️oracle/🔣️.json `notes` on those two entries).
fn fixtures(out: &str) -> Result<Vec<String>, String> {
    std::fs::create_dir_all(out).map_err(|error| format!("{out}: {error}"))?;
    let mut written = Vec::new();
    let base_rect = || SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(0,128,0,1)"), stroke: Some("rgba(0,0,0,1)"), stroke_width: Some("1"), opacity: None };
    let base_text = || SceneNode { matrix: IDENTITY, kind: "text", d: None, text: Some("Hello"), fill: Some("rgba(0,0,0,1)"), stroke: None, stroke_width: None, opacity: None };

    macro_rules! recipe {
        ($dir:expr, $canvas:expr, $before:expr, $after:expr) => {{
            let dir = format!("{out}/{}", $dir);
            std::fs::create_dir_all(&dir).map_err(|error| format!("{dir}: {error}"))?;
            write_scene(&format!("{dir}/before.svg"), $canvas, $before)?;
            write_scene(&format!("{dir}/after.svg"), $canvas, $after)?;
            written.push(format!("{}/before.svg", $dir));
            written.push(format!("{}/after.svg", $dir));
        }};
    }

    // create-layer: a second node appears.
    recipe!("create-layer-adds-a-node", (100, 50), &[base_rect()], &[base_rect(), SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 20 0 L 30 0 L 30 10 L 20 10 Z"), text: None, fill: Some("rgba(0,0,255,1)"), stroke: None, stroke_width: None, opacity: None }]);

    // delete-layer: the second node disappears.
    recipe!("delete-layer-removes-a-node", (100, 50), &[base_rect(), base_text()], &[base_rect()]);

    // duplicate-layer: a second, identical node is inserted right after the source.
    recipe!("duplicate-layer-inserts-a-copy", (100, 50), &[base_rect()], &[base_rect(), base_rect()]);

    // reorder-layer: two nodes swap position (structural address changes, tags stay the same set).
    recipe!("reorder-layer-swaps-two-nodes", (100, 50), &[base_rect(), base_text()], &[base_text(), base_rect()]);

    // set-layer-visible: turning a layer invisible removes its scene node entirely (flatten skips
    // `!base.visible` layers before they ever become a DrawSceneNode).
    recipe!("set-layer-visible-hides-a-node", (100, 50), &[base_rect(), base_text()], &[base_rect()]);

    // update-layer-transform: the wrapping group's matrix changes; geometry (`d`) is untouched.
    recipe!(
        "update-layer-transform-moves-a-node",
        (100, 50),
        &[base_rect()],
        &[SceneNode { matrix: [1.0, 0.0, 0.0, 1.0, 15.0, 5.0], kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(0,128,0,1)"), stroke: Some("rgba(0,0,0,1)"), stroke_width: Some("1"), opacity: None }]
    );

    // replace-layer-fill: only the fill channel changes.
    recipe!(
        "replace-layer-fill-recolors-a-node",
        (100, 50),
        &[base_rect()],
        &[SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(255,0,0,1)"), stroke: Some("rgba(0,0,0,1)"), stroke_width: Some("1"), opacity: None }]
    );

    // replace-layer-stroke: only the stroke color/width changes.
    recipe!(
        "replace-layer-stroke-changes-outline",
        (100, 50),
        &[base_rect()],
        &[SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(0,128,0,1)"), stroke: Some("rgba(0,0,255,1)"), stroke_width: Some("3"), opacity: None }]
    );

    // set-layer-opacity: only the opacity channel changes (Path leaf; Image leaves never see this).
    recipe!(
        "set-layer-opacity-fades-a-node",
        (100, 50),
        &[SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(0,128,0,1)"), stroke: None, stroke_width: None, opacity: Some("1") }],
        &[SceneNode { matrix: IDENTITY, kind: "path", d: Some("M 0 0 L 10 0 L 10 10 L 0 10 Z"), text: None, fill: Some("rgba(0,128,0,1)"), stroke: None, stroke_width: None, opacity: Some("0.4") }]
    );

    Ok(written)
}
//#endregion 🏭️Fixtures

//#region 🚀️Entry
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
        "svg-compare" => {
            if !need(2) {
                return;
            }
            for input in inputs.iter().take(2) {
                if extension(input) != "svg" {
                    return unsupported(&format!(".{} is not an SVG carrier", extension(input)));
                }
            }
            match (read_svg(&inputs[0]), read_svg(&inputs[1])) {
                (Err(error), _) | (_, Err(error)) => failed("svg parse failed", &error),
                (Ok(expected), Ok(actual)) => emit("ok", compare_svg(&expected, &actual), Vec::new()),
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
        "fixtures" => {
            if out.is_empty() {
                return failed("missing --out", "fixtures needs --out <dir>");
            }
            match fixtures(&out) {
                Err(error) => failed("fixture generation failed", &error),
                Ok(written) => emit("ok", J::O(vec![("written".to_string(), J::A(written.into_iter().map(J::S).collect())), ("out".to_string(), J::S(out))]), Vec::new()),
            }
        }
        other => failed("unknown probe", &format!("{other:?} — known: svg-structure, svg-compare, gate-inputs, fixtures")),
    }
}
//#endregion 🚀️Entry
