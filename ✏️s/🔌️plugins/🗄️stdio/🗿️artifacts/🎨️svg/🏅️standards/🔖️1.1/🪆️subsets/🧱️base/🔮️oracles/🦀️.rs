//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `quick-xml` reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. No shared `xml`/`markup` family module
//! exists yet (unlike `document`/`raster`/`archive`/`tabular`/`audio`/`mesh`) and this module does
//! not create one: the sibling 📰️xml 1.0 oracle is still an unfilled stub as of this wave, so there
//! is no second subset to genuinely share an implementation with yet. Every helper below (the
//! quick-xml-backed tree model, the hand-written `viewBox`/`transform` grammars, the reader of each
//! leaf's wire payload) is owned by THIS module alone and must not be copied into or imported from
//! the xml subset.
//!
//! A spec's `params` is the leaf wire payload (the leaf payload schema is the contract), read here
//! from the schema's own shapes rather than from the subject's decoder. Three entry points:
//! [`oracle_apply_mutation`] performs the FORWARD mutation (the `mutate-<kind>` scenarios),
//! [`oracle_apply_mutation_inverse`] performs the forward mutation and then restores what it
//! discarded (the `inverse-<kind>` scenarios), [`oracle_round_trip`] decodes and re-encodes (the
//! identity scenario). [`project_svg_1_1`] is the independent
//! reader both the oracle's and the subject's re-serialized bytes are read back through before
//! comparison.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`SvgMutation::KINDS`).

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use quick_xml::events::{BytesCData, BytesDecl, BytesPI, BytesStart, BytesText, Event};
    use quick_xml::{Reader, Writer};
    use semio_repo_test_host::Json;

    //#region 🔖️Tree
    /// 🌳 This module's OWN quick-xml-backed element tree — independent of, and structurally
    /// parallel to, the production `XmlNode`/`XmlDocument` this subset's own codec persists
    /// (`../../🧬️schema/📸️snapshot/🦀️.rs`), never importing it. Attributes are an ordered
    /// `Vec` (source order preserved on read) since the WRITE side never needs it ordered — the
    /// projection sorts by name, per this module's own `comparisonProfiles` entry.
    #[derive(Clone, Debug, PartialEq)]
    enum QNode {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<QNode> },
        Text(String),
        CData(String),
        Comment(String),
        Pi { target: String, data: String },
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct QDecl {
        version: String,
        encoding: Option<String>,
        standalone: Option<bool>,
    }

    #[derive(Clone, Debug, Default)]
    struct QDoc {
        declaration: Option<QDecl>,
        doctype: Option<String>,
        doctype_prolog_position: usize,
        prolog: Vec<QNode>,
        root: Option<QNode>,
        epilog: Vec<QNode>,
    }
    //#endregion 🔖️Tree

    //#region 🔖️NodePath
    /// 🧭 Child-index chain from the root element — the same addressing scheme this subset's own
    /// `NodePath` uses (`node_at`/`node_at_mut`, `../../🧬️schema/📸️snapshot/🦀️.rs`),
    /// reimplemented here against `QDoc` rather than imported.
    fn q_node_at<'a>(doc: &'a QDoc, path: &[usize]) -> Result<&'a QNode, String> {
        let mut node = doc.root.as_ref().ok_or("document has no root element")?;
        for &index in path {
            match node {
                QNode::Element { children, .. } => node = children.get(index).ok_or_else(|| format!("child index {index} out of range"))?,
                _ => return Err("path descends into a non-element node".into()),
            }
        }
        Ok(node)
    }

    fn q_node_at_mut<'a>(doc: &'a mut QDoc, path: &[usize]) -> Result<&'a mut QNode, String> {
        let mut node = doc.root.as_mut().ok_or("document has no root element")?;
        for &index in path {
            match node {
                QNode::Element { children, .. } => node = children.get_mut(index).ok_or_else(|| format!("child index {index} out of range"))?,
                _ => return Err("path descends into a non-element node".into()),
            }
        }
        Ok(node)
    }

    fn q_element_attr<'a>(node: &'a QNode, name: &str) -> Option<&'a str> {
        match node {
            QNode::Element { attrs, .. } => attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str()),
            _ => None,
        }
    }

    /// 🏷️ Update-in-place when present (so an untouched attribute keeps its source position),
    /// append when new, remove on `None` — same shape as this subset's own `set_element_attr`.
    fn q_set_attr(node: &mut QNode, name: &str, value: Option<String>) {
        if let QNode::Element { attrs, .. } = node {
            match value {
                Some(v) => match attrs.iter_mut().find(|(key, _)| key == name) {
                    Some(entry) => entry.1 = v,
                    None => attrs.push((name.to_string(), v)),
                },
                None => attrs.retain(|(key, _)| key != name),
            }
        }
    }
    //#endregion 🔖️NodePath

    //#region 🔖️GeometryGrammar
    /// 🖼️ `viewBox="min-x min-y width height"`, comma-or-whitespace separated per the SVG 1.1
    /// `<number>`/`<comma-wsp>` production — hand-written independently of this subset's own
    /// `parse_view_box`/`view_box_to_string` (`../../🧬️schema/📸️snapshot/🦀️.rs`).
    fn parse_view_box(s: &str) -> Result<[f64; 4], String> {
        let parts: Vec<&str> = s.split(|c: char| c.is_whitespace() || c == ',').filter(|p| !p.is_empty()).collect();
        if parts.len() != 4 {
            return Err(format!("viewBox: expected 4 numbers, got {}", parts.len()));
        }
        let f = |p: &str| p.parse::<f64>().map_err(|e| e.to_string());
        Ok([f(parts[0])?, f(parts[1])?, f(parts[2])?, f(parts[3])?])
    }

    fn format_view_box(v: &[f64; 4]) -> String {
        format!("{} {} {} {}", v[0], v[1], v[2], v[3])
    }

    /// 🔄 `TransformOp` — this module's own copy of the SVG 1.1 `transform` grammar
    /// (`matrix|translate|scale|rotate|skewX|skewY`), independent of this subset's own
    /// `TransformOp`/`parse_transform_list`/`transform_list_to_string`.
    #[derive(Clone, Debug, PartialEq)]
    enum QTransformOp {
        Matrix { a: f64, b: f64, c: f64, d: f64, e: f64, f: f64 },
        Translate { x: f64, y: Option<f64> },
        Scale { x: f64, y: Option<f64> },
        Rotate { angle: f64, center: Option<(f64, f64)> },
        SkewX { angle: f64 },
        SkewY { angle: f64 },
    }

    fn parse_transform_list(s: &str) -> Result<Vec<QTransformOp>, String> {
        let mut ops = Vec::new();
        let mut rest = s.trim();
        while !rest.is_empty() {
            let open = rest.find('(').ok_or("transform: expected '('")?;
            let name = rest[..open].trim();
            let close = open + rest[open..].find(')').ok_or("transform: expected ')'")?;
            let nums: Vec<f64> = rest[open + 1..close].split(|c: char| c.is_whitespace() || c == ',').filter(|p| !p.is_empty()).map(|p| p.parse::<f64>().map_err(|e| e.to_string())).collect::<Result<_, String>>()?;
            let op = match name {
                "matrix" if nums.len() == 6 => QTransformOp::Matrix { a: nums[0], b: nums[1], c: nums[2], d: nums[3], e: nums[4], f: nums[5] },
                "translate" if nums.len() == 1 => QTransformOp::Translate { x: nums[0], y: None },
                "translate" if nums.len() == 2 => QTransformOp::Translate { x: nums[0], y: Some(nums[1]) },
                "scale" if nums.len() == 1 => QTransformOp::Scale { x: nums[0], y: None },
                "scale" if nums.len() == 2 => QTransformOp::Scale { x: nums[0], y: Some(nums[1]) },
                "rotate" if nums.len() == 1 => QTransformOp::Rotate { angle: nums[0], center: None },
                "rotate" if nums.len() == 3 => QTransformOp::Rotate { angle: nums[0], center: Some((nums[1], nums[2])) },
                "skewX" if nums.len() == 1 => QTransformOp::SkewX { angle: nums[0] },
                "skewY" if nums.len() == 1 => QTransformOp::SkewY { angle: nums[0] },
                other => return Err(format!("transform: unrecognised function {other:?} with {} args", nums.len())),
            };
            ops.push(op);
            rest = rest[close + 1..].trim_start_matches(|c: char| c.is_whitespace() || c == ',').trim_start();
        }
        Ok(ops)
    }

    fn format_transform_list(ops: &[QTransformOp]) -> String {
        ops.iter()
            .map(|op| match op {
                QTransformOp::Matrix { a, b, c, d, e, f } => format!("matrix({a},{b},{c},{d},{e},{f})"),
                QTransformOp::Translate { x, y: None } => format!("translate({x})"),
                QTransformOp::Translate { x, y: Some(y) } => format!("translate({x},{y})"),
                QTransformOp::Scale { x, y: None } => format!("scale({x})"),
                QTransformOp::Scale { x, y: Some(y) } => format!("scale({x},{y})"),
                QTransformOp::Rotate { angle, center: None } => format!("rotate({angle})"),
                QTransformOp::Rotate { angle, center: Some((cx, cy)) } => format!("rotate({angle},{cx},{cy})"),
                QTransformOp::SkewX { angle } => format!("skewX({angle})"),
                QTransformOp::SkewY { angle } => format!("skewY({angle})"),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    //#endregion 🔖️GeometryGrammar

    //#region 🔖️JsonValue
    fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    fn json_number(value: &Json) -> Option<f64> {
        match value {
            Json::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn member(value: &Json, key: &str) -> Json {
        value.get(key).cloned().unwrap_or(Json::Null)
    }

    fn json_to_path(value: &Json) -> Vec<usize> {
        match value {
            Json::Array(items) => items.iter().map(|item| json_number(item).unwrap_or(0.0).max(0.0) as usize).collect(),
            _ => Vec::new(),
        }
    }

    fn json_index(value: &Json, key: &str) -> usize {
        json_number(&member(value, key)).unwrap_or(0.0).max(0.0) as usize
    }

    /// 🔎️ One `XmlNode` wire value (`{"kind":"element"|"text"|"cData"|"comment"|"processingInstruction", ...}`, the
    /// leaf payload schema's own node union) read into this module's tree — written from the schema, not from the
    /// subject's decoder.
    fn qnode_from_wire(value: &Json) -> Result<QNode, String> {
        match value.str("kind").as_str() {
            "element" => Ok(QNode::Element {
                name: value.str("name"),
                attrs: value.array("attrs").iter().map(|a| (a.str("name"), a.str("value"))).collect(),
                children: value.array("children").iter().map(qnode_from_wire).collect::<Result<_, _>>()?,
            }),
            "text" => Ok(QNode::Text(value.str("text"))),
            "cData" => Ok(QNode::CData(value.str("text"))),
            "comment" => Ok(QNode::Comment(value.str("text"))),
            "processingInstruction" => Ok(QNode::Pi { target: value.str("target"), data: value.str("data") }),
            other => Err(format!("node wire: unrecognised kind {other:?}")),
        }
    }

    /// 🏳️ An `XmlDeclaration` wire value (`null` = no declaration). The delimiter facet `quote` is writer freedom this
    /// module's quick-xml writer does not model, and the projection never compares it.
    fn decl_from_wire(value: &Json) -> Option<QDecl> {
        match value {
            Json::Object(_) => Some(QDecl {
                version: value.str("version"),
                encoding: match value.get("encoding") {
                    Some(Json::String(text)) => Some(text.clone()),
                    _ => None,
                },
                standalone: match value.get("standalone") {
                    Some(Json::Bool(flag)) => Some(*flag),
                    _ => None,
                },
            }),
            _ => None,
        }
    }

    /// 📜️ An `XmlDoctype` wire value (`null` = no doctype) composed into the raw declaration body quick-xml reads and
    /// writes, following XML 1.0 §2.8 (`doctypedecl`) and §4.2 (`EntityDecl`), plus its prolog position.
    fn doctype_from_wire(value: &Json) -> Result<Option<(String, usize)>, String> {
        if !matches!(value, Json::Object(_)) {
            return Ok(None);
        }
        let mut raw = value.str("name");
        match value.get("externalId") {
            Some(id @ Json::Object(_)) => match id.str("kind").as_str() {
                "system" => raw.push_str(&format!(" SYSTEM \"{}\"", id.str("systemId"))),
                "public" => raw.push_str(&format!(" PUBLIC \"{}\" \"{}\"", id.str("publicId"), id.str("systemId"))),
                other => return Err(format!("doctype wire: unrecognised external identifier kind {other:?}")),
            },
            _ => {}
        }
        let declarations = value.array("declarations");
        if !declarations.is_empty() {
            raw.push_str(" [");
            for declaration in &declarations {
                if declaration.str("kind") != "entity" {
                    return Err(format!("doctype wire: unrecognised declaration kind {:?}", declaration.str("kind")));
                }
                let parameter = if matches!(declaration.get("parameter"), Some(Json::Bool(true))) { "% " } else { "" };
                raw.push_str(&format!("<!ENTITY {parameter}{} \"{}\">", declaration.str("name"), declaration.str("value")));
            }
            raw.push(']');
        }
        Ok(Some((raw, json_index(value, "prologPosition"))))
    }

    /// 🖼️ A `ViewBox` wire value (`{"minX","minY","width","height"}`, `null` = remove the attribute).
    fn view_box_from_wire(value: &Json) -> Option<[f64; 4]> {
        match value {
            Json::Object(_) => Some(["minX", "minY", "width", "height"].map(|key| json_number(&member(value, key)).unwrap_or(0.0))),
            _ => None,
        }
    }

    /// 🔄 One `TransformOp` wire value (tag `op`; `rotate`'s optional `center` is the `[cx, cy]` pair).
    fn transform_op_from_wire(value: &Json) -> Result<QTransformOp, String> {
        let num = |key: &str| json_number(&member(value, key)).unwrap_or(0.0);
        let opt_num = |key: &str| value.get(key).and_then(json_number);
        match value.str("op").as_str() {
            "matrix" => Ok(QTransformOp::Matrix { a: num("a"), b: num("b"), c: num("c"), d: num("d"), e: num("e"), f: num("f") }),
            "translate" => Ok(QTransformOp::Translate { x: num("x"), y: opt_num("y") }),
            "scale" => Ok(QTransformOp::Scale { x: num("x"), y: opt_num("y") }),
            "rotate" => Ok(QTransformOp::Rotate {
                angle: num("angle"),
                center: match value.get("center") {
                    Some(Json::Array(pair)) if pair.len() == 2 => Some((json_number(&pair[0]).unwrap_or(0.0), json_number(&pair[1]).unwrap_or(0.0))),
                    _ => None,
                },
            }),
            "skewX" => Ok(QTransformOp::SkewX { angle: num("angle") }),
            "skewY" => Ok(QTransformOp::SkewY { angle: num("angle") }),
            other => Err(format!("transform wire: unrecognised op {other:?}")),
        }
    }

    fn transform_op_to_json(op: &QTransformOp) -> Json {
        let n = |v: f64| Json::Number(v);
        match op {
            QTransformOp::Matrix { a, b, c, d, e, f } => obj(vec![("kind", Json::String("matrix".into())), ("a", n(*a)), ("b", n(*b)), ("c", n(*c)), ("d", n(*d)), ("e", n(*e)), ("f", n(*f))]),
            QTransformOp::Translate { x, y } => obj(vec![("kind", Json::String("translate".into())), ("x", n(*x)), ("y", y.map(n).unwrap_or(Json::Null))]),
            QTransformOp::Scale { x, y } => obj(vec![("kind", Json::String("scale".into())), ("x", n(*x)), ("y", y.map(n).unwrap_or(Json::Null))]),
            QTransformOp::Rotate { angle, center } => obj(vec![("kind", Json::String("rotate".into())), ("angle", n(*angle)), ("cx", center.map(|(cx, _)| n(cx)).unwrap_or(Json::Null)), ("cy", center.map(|(_, cy)| n(cy)).unwrap_or(Json::Null))]),
            QTransformOp::SkewX { angle } => obj(vec![("kind", Json::String("skewX".into())), ("angle", n(*angle))]),
            QTransformOp::SkewY { angle } => obj(vec![("kind", Json::String("skewY".into())), ("angle", n(*angle))]),
        }
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️Parse
    /// 📥️ Builds a [`QDoc`] from real SVG/XML bytes via `quick_xml::Reader`'s zero-copy event
    /// stream. `Event::Text` bodies arrive with entity/character references already split out into
    /// separate `Event::GeneralRef` events (a quick-xml 0.42 reader behaviour, not opt-in); runs of
    /// `Text`+`GeneralRef` between two structural events are coalesced back into ONE `QNode::Text`,
    /// matching this subset's own single-Text-node-per-run model.
    fn parse_svg(bytes: &[u8]) -> Result<QDoc, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("svg source is not UTF-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut doc = QDoc::default();
        let mut stack: Vec<(String, Vec<(String, String)>, Vec<QNode>)> = Vec::new();
        let mut text_buf = String::new();

        fn flush(text_buf: &mut String, stack: &mut [(String, Vec<(String, String)>, Vec<QNode>)]) {
            if text_buf.is_empty() {
                return;
            }
            if let Some((_, _, children)) = stack.last_mut() {
                children.push(QNode::Text(std::mem::take(text_buf)));
            } else {
                text_buf.clear();
            }
        }

        fn attach(stack: &mut Vec<(String, Vec<(String, String)>, Vec<QNode>)>, doc: &mut QDoc, node: QNode, is_misc: bool) {
            if let Some((_, _, children)) = stack.last_mut() {
                children.push(node);
            } else if doc.root.is_none() {
                if is_misc {
                    doc.prolog.push(node);
                } else {
                    doc.root = Some(node);
                }
            } else if is_misc {
                doc.epilog.push(node);
            }
        }

        fn read_attrs(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
            let mut attrs = Vec::new();
            for attr in start.attributes() {
                let attr = attr.map_err(|error| error.to_string())?;
                let value = attr.unescape_value().map_err(|error| error.to_string())?.into_owned();
                attrs.push((attr.key.as_ref().to_string(), value));
            }
            Ok(attrs)
        }

        loop {
            match reader.read_event().map_err(|error| error.to_string())? {
                Event::Decl(decl) => {
                    let version = decl.version().map_err(|error| error.to_string())?.into_owned();
                    let encoding = decl.encoding().transpose().map_err(|error| error.to_string())?.map(|c| c.into_owned());
                    let standalone = decl.standalone().transpose().map_err(|error| error.to_string())?.map(|c| c.as_ref() == "yes");
                    doc.declaration = Some(QDecl { version, encoding, standalone });
                }
                Event::DocType(_) if doc.root.is_some() => return Err("doctype cannot appear after the root element".to_string()),
                Event::DocType(_) if doc.doctype.is_some() => return Err("duplicate XML doctype".to_string()),
                Event::DocType(raw) => {
                    doc.doctype_prolog_position = doc.prolog.len();
                    doc.doctype = Some(raw.as_ref().to_string());
                }
                Event::PI(pi) => {
                    flush(&mut text_buf, &mut stack);
                    attach(&mut stack, &mut doc, QNode::Pi { target: pi.target().to_string(), data: pi.content().to_string() }, true);
                }
                Event::Comment(text) => {
                    flush(&mut text_buf, &mut stack);
                    attach(&mut stack, &mut doc, QNode::Comment(text.as_ref().to_string()), true);
                }
                Event::CData(text) => {
                    flush(&mut text_buf, &mut stack);
                    if let Some((_, _, children)) = stack.last_mut() {
                        children.push(QNode::CData(text.as_ref().to_string()));
                    }
                }
                Event::Text(text) => text_buf.push_str(text.as_ref()),
                Event::GeneralRef(reference) => match reference.resolve_char_ref().map_err(|error| error.to_string())? {
                    Some(ch) => text_buf.push(ch),
                    None => match quick_xml::escape::resolve_predefined_entity(reference.as_ref()) {
                        Some(resolved) => text_buf.push_str(resolved),
                        None => return Err(format!("svg source references unknown entity &{};", reference.as_ref())),
                    },
                },
                Event::Start(start) => {
                    flush(&mut text_buf, &mut stack);
                    let attrs = read_attrs(&start)?;
                    stack.push((start.name().as_ref().to_string(), attrs, Vec::new()));
                }
                Event::Empty(start) => {
                    flush(&mut text_buf, &mut stack);
                    let attrs = read_attrs(&start)?;
                    attach(&mut stack, &mut doc, QNode::Element { name: start.name().as_ref().to_string(), attrs, children: Vec::new() }, false);
                }
                Event::End(_) => {
                    flush(&mut text_buf, &mut stack);
                    let (name, attrs, children) = stack.pop().ok_or("svg source: unmatched closing tag")?;
                    attach(&mut stack, &mut doc, QNode::Element { name, attrs, children }, false);
                }
                Event::Eof => {
                    flush(&mut text_buf, &mut stack);
                    break;
                }
            }
        }
        if doc.root.is_none() {
            return Err("svg document requires root element".into());
        }
        Ok(doc)
    }
    //#endregion 🔖️Parse

    //#region 🔖️Write
    /// 📤️ Re-serializes a [`QDoc`] via `quick_xml::Writer` — self-closing vs explicit empty
    /// open/close is never distinguished on read (both parse to zero children), so an empty
    /// element is always written `Event::Empty`; this is real writer freedom, narrowed out of the
    /// projection rather than chased byte-for-byte.
    fn write_node(writer: &mut Writer<Vec<u8>>, node: &QNode) -> Result<(), String> {
        match node {
            QNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
            QNode::CData(text) => writer.write_event(Event::CData(BytesCData::new(text.as_str()))).map_err(|error| error.to_string()),
            QNode::Comment(text) => writer.write_event(Event::Comment(BytesText::from_escaped(text.as_str()))).map_err(|error| error.to_string()),
            QNode::Pi { target, data } => {
                let content = if data.is_empty() { target.clone() } else { format!("{target} {data}") };
                writer.write_event(Event::PI(BytesPI::new(content))).map_err(|error| error.to_string())
            }
            QNode::Element { name, attrs, children } => {
                let mut start = BytesStart::new(name.as_str());
                for (key, value) in attrs {
                    start.push_attribute((key.as_str(), value.as_str()));
                }
                if children.is_empty() {
                    return writer.write_event(Event::Empty(start)).map_err(|error| error.to_string());
                }
                let end = start.to_end().into_owned();
                writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
                for child in children {
                    write_node(writer, child)?;
                }
                writer.write_event(Event::End(end)).map_err(|error| error.to_string())
            }
        }
    }

    fn write_svg(doc: &QDoc) -> Result<Vec<u8>, String> {
        let mut writer = Writer::new(Vec::new());
        if let Some(decl) = &doc.declaration {
            let standalone = decl.standalone.map(|value| if value { "yes" } else { "no" });
            writer.write_event(Event::Decl(BytesDecl::new(&decl.version, decl.encoding.as_deref(), standalone))).map_err(|error| error.to_string())?;
        }
        if doc.doctype.is_some() && doc.doctype_prolog_position > doc.prolog.len() {
            return Err("doctype prolog position exceeds prolog length".to_string());
        }
        for (index, node) in doc.prolog.iter().enumerate() {
            if doc.doctype.is_some() && doc.doctype_prolog_position == index {
                writer.write_event(Event::DocType(BytesText::from_escaped(doc.doctype.as_ref().unwrap().as_str()))).map_err(|error| error.to_string())?;
            }
            write_node(&mut writer, node)?;
        }
        if let Some(raw) = doc.doctype.as_ref().filter(|_| doc.doctype_prolog_position == doc.prolog.len()) {
            writer.write_event(Event::DocType(BytesText::from_escaped(raw.as_str()))).map_err(|error| error.to_string())?;
        }
        if let Some(root) = &doc.root {
            write_node(&mut writer, root)?;
        }
        for node in &doc.epilog {
            write_node(&mut writer, node)?;
        }
        Ok(writer.into_inner())
    }
    //#endregion 🔖️Write

    //#region 🔖️Apply
    /// 🦠️ Mutates `doc` in place for one declared kind, reading `params` as that leaf's wire payload. An unrecognised
    /// kind is an error, never a silent no-op.
    /// 🌲️ One node as the `XmlNode` wire [`qnode_from_wire`] reads.
    fn qnode_to_wire(node: &QNode) -> Json {
        let text = |kind: &str, text: &str| obj(vec![("kind", Json::String(kind.into())), ("text", Json::String(text.into()))]);
        match node {
            QNode::Element { name, attrs, children } => obj(vec![
                ("kind", Json::String("element".into())),
                ("name", Json::String(name.clone())),
                ("attrs", Json::Array(attrs.iter().map(|(name, value)| obj(vec![("name", Json::String(name.clone())), ("value", Json::String(value.clone()))])).collect())),
                ("children", Json::Array(children.iter().map(qnode_to_wire).collect())),
            ]),
            QNode::Text(value) => text("text", value),
            QNode::CData(value) => text("cData", value),
            QNode::Comment(value) => text("comment", value),
            QNode::Pi { target, data } => obj(vec![("kind", Json::String("processingInstruction".into())), ("target", Json::String(target.clone())), ("data", Json::String(data.clone()))]),
        }
    }

    /// 🩹️ `doc` with a `patch-snapshot` row's one pointer operation applied to this tree's own `{schema, doc}` reading
    /// (`semio_repo_test_host::law::patched_snapshot`); the doctype stays the raw body this tree keeps, so a pointer into
    /// `/doc/doctype` is refused rather than guessed.
    fn patched_doc(doc: &QDoc, patch: &Json) -> Result<QDoc, String> {
        if patch.str("path").starts_with("/doc/doctype") || patch.str("from").starts_with("/doc/doctype") {
            return Err("patch-snapshot: this reference tree keeps the doctype raw and reads no pointer into it".to_string());
        }
        let declaration = doc.declaration.as_ref().map_or(Json::Null, |declaration| {
            obj(vec![("version", Json::String(declaration.version.clone())), ("encoding", declaration.encoding.clone().map_or(Json::Null, Json::String)), ("standalone", declaration.standalone.map_or(Json::Null, Json::Bool))])
        });
        let nodes = |nodes: &[QNode]| Json::Array(nodes.iter().map(qnode_to_wire).collect());
        let reading = obj(vec![
            ("schema", Json::String("stdio.svg".into())),
            ("doc", obj(vec![("declaration", declaration), ("doctype", Json::Null), ("prolog", nodes(&doc.prolog)), ("root", doc.root.as_ref().map_or(Json::Null, qnode_to_wire)), ("epilog", nodes(&doc.epilog))])),
        ]);
        let patched = member(&semio_repo_test_host::law::patched_snapshot(&reading, patch)?, "doc");
        let read = |key: &str| patched.array(key).iter().map(qnode_from_wire).collect::<Result<Vec<_>, _>>();
        Ok(QDoc {
            declaration: decl_from_wire(&member(&patched, "declaration")),
            doctype: doc.doctype.clone(),
            doctype_prolog_position: doc.doctype_prolog_position,
            prolog: read("prolog")?,
            root: match member(&patched, "root") {
                Json::Null => None,
                root => Some(qnode_from_wire(&root)?),
            },
            epilog: read("epilog")?,
        })
    }

    fn apply_kind(doc: &mut QDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "patch-snapshot" => {
                *doc = patched_doc(doc, &member(params, "patch"))?;
                Ok(())
            }
            "set-declaration" => {
                doc.declaration = decl_from_wire(&member(params, "declaration"));
                Ok(())
            }
            "set-doctype" => {
                let (raw, position) = doctype_from_wire(&member(params, "doctype"))?.map_or((None, 0), |(raw, position)| (Some(raw), position));
                doc.doctype = raw;
                doc.doctype_prolog_position = position;
                Ok(())
            }
            "insert-element" => {
                let parent = json_to_path(&member(params, "parent"));
                let index = json_index(params, "index");
                let node = qnode_from_wire(&member(params, "node"))?;
                match q_node_at_mut(doc, &parent)? {
                    QNode::Element { children, .. } => {
                        let clamped = index.min(children.len());
                        children.insert(clamped, node);
                        Ok(())
                    }
                    _ => Err("insert-element: parent is not an element".into()),
                }
            }
            "remove-element" => {
                let parent = json_to_path(&member(params, "parent"));
                let index = json_index(params, "index");
                match q_node_at_mut(doc, &parent)? {
                    QNode::Element { children, .. } if index < children.len() => {
                        children.remove(index);
                        Ok(())
                    }
                    QNode::Element { .. } => Err("remove-element: index out of range".into()),
                    _ => Err("remove-element: parent is not an element".into()),
                }
            }
            "set-element-name" => match q_node_at_mut(doc, &json_to_path(&member(params, "path")))? {
                QNode::Element { name, .. } => {
                    *name = params.str("name");
                    Ok(())
                }
                _ => Err("set-element-name: target is not an element".into()),
            },
            "set-attribute" => {
                let value = match params.get("value") {
                    Some(Json::String(v)) => Some(v.clone()),
                    _ => None,
                };
                q_set_attr(q_node_at_mut(doc, &json_to_path(&member(params, "path")))?, &params.str("name"), value);
                Ok(())
            }
            "set-text" => match q_node_at_mut(doc, &json_to_path(&member(params, "path")))? {
                QNode::Text(text) => {
                    *text = params.str("text");
                    Ok(())
                }
                _ => Err("set-text: target is not a text node".into()),
            },
            "set-view-box" => {
                let view_box = view_box_from_wire(&member(params, "viewBox")).map(|value| format_view_box(&value));
                q_set_attr(q_node_at_mut(doc, &json_to_path(&member(params, "path")))?, "viewBox", view_box);
                Ok(())
            }
            "set-transform" => {
                let transform = match member(params, "transform") {
                    Json::Array(items) => Some(format_transform_list(&items.iter().map(transform_op_from_wire).collect::<Result<Vec<_>, String>>()?)),
                    _ => None,
                };
                q_set_attr(q_node_at_mut(doc, &json_to_path(&member(params, "path")))?, "transform", transform);
                Ok(())
            }
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Apply

    //#region 🔖️Inverse
    /// ↩️ Undoes `{kind, params}` on `doc` (the tree the forward step left behind) by restoring exactly what the forward
    /// step discarded from `base` (the tree before it) — this module's own algebra, one arm per kind, never the
    /// subject's `Mutation::inverse`.
    fn invert(base: &QDoc, doc: &mut QDoc, kind: &str, params: &Json) -> Result<(), String> {
        let prior_attr = |path: &[usize], name: &str| q_node_at(base, path).ok().and_then(|node| q_element_attr(node, name)).map(str::to_string);
        match kind {
            "patch-snapshot" => *doc = base.clone(),
            "set-declaration" => doc.declaration = base.declaration.clone(),
            "set-doctype" => {
                doc.doctype = base.doctype.clone();
                doc.doctype_prolog_position = base.doctype_prolog_position;
            }
            "insert-element" => match q_node_at_mut(doc, &json_to_path(&member(params, "parent")))? {
                QNode::Element { children, .. } => {
                    let index = json_index(params, "index").min(children.len().saturating_sub(1));
                    children.remove(index);
                }
                _ => return Err("inverse insert-element: parent is not an element".into()),
            },
            "remove-element" => {
                let parent = json_to_path(&member(params, "parent"));
                let index = json_index(params, "index");
                let captured = match q_node_at(base, &parent)? {
                    QNode::Element { children, .. } => children.get(index).cloned().ok_or_else(|| format!("inverse remove-element: the original document has no child {index} under {parent:?}"))?,
                    _ => return Err("inverse remove-element: parent is not an element".into()),
                };
                match q_node_at_mut(doc, &parent)? {
                    QNode::Element { children, .. } => children.insert(index.min(children.len()), captured),
                    _ => return Err("inverse remove-element: parent is not an element".into()),
                }
            }
            "set-element-name" => {
                let path = json_to_path(&member(params, "path"));
                let prior = match q_node_at(base, &path)? {
                    QNode::Element { name, .. } => name.clone(),
                    _ => return Err("inverse set-element-name: the original target is not an element".into()),
                };
                if let QNode::Element { name, .. } = q_node_at_mut(doc, &path)? {
                    *name = prior;
                }
            }
            "set-attribute" => {
                let path = json_to_path(&member(params, "path"));
                let name = params.str("name");
                q_set_attr(q_node_at_mut(doc, &path)?, &name, prior_attr(&path, &name));
            }
            "set-text" => {
                let path = json_to_path(&member(params, "path"));
                let prior = match q_node_at(base, &path)? {
                    QNode::Text(text) => text.clone(),
                    _ => return Err("inverse set-text: the original target is not a text node".into()),
                };
                if let QNode::Text(text) = q_node_at_mut(doc, &path)? {
                    *text = prior;
                }
            }
            "set-view-box" | "set-transform" => {
                let path = json_to_path(&member(params, "path"));
                let name = if kind == "set-view-box" { "viewBox" } else { "transform" };
                q_set_attr(q_node_at_mut(doc, &path)?, name, prior_attr(&path, name));
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let mut doc = parse_svg(input)?;
        apply_kind(&mut doc, kind, params)?;
        write_svg(&doc)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse to ONE parsed tree — the caller
    /// compares its projection against the ORIGINAL input's own, proving `apply(inverse(m,base),
    /// apply(m, base)) == base` against this independent reference implementation.
    ///
    /// 🐛️ The two steps deliberately share a tree rather than passing serialized bytes between
    /// them. Routing the undo through `apply_mutation(&mutated, ...)` re-parsed the intermediate
    /// document, and XML parsing COALESCES adjacent character data: in this drawing every
    /// `<g …>\n<rect …/>\n</g>` group holds `[text "\n", rect, text "\n"]`, so removing index 1
    /// left two adjacent text nodes that re-read as the single node `"\n\n"`, and the undo then
    /// inserted the rect at index 1 of a ONE-child list. That made `inverse-remove-element` fail
    /// against the real fixture — a defect in this routing, not in the vocabulary and not in the
    /// law, since the subject applies both steps to one snapshot with no serialization between.
    /// The coalescing itself is real and is not being papered over: it is why an inverse addressed
    /// by index is only meaningful against the tree the forward step left behind.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let base = parse_svg(input)?;
        let mut doc = base.clone();
        apply_kind(&mut doc, kind, params)?;
        invert(&base, &mut doc, kind, params)?;
        write_svg(&doc)
    }

    /// 🔁️ Decode then re-encode through this module's own tree alone — the oracle half of the identity law.
    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        write_svg(&parse_svg(input)?)
    }

    /// 👁️ This subset's own semantic projection — declaration/doctype presence and fields, the
    /// prolog, and the root tree with attributes name-sorted and `viewBox`/`transform` decomposed
    /// into typed numeric geometry (excluded from the raw-attrs list so writer-freedom in their
    /// STRING formatting never fails an otherwise-identical mutation).
    pub fn project_svg_1_1(bytes: &[u8]) -> Result<Json, String> {
        let doc = parse_svg(bytes)?;
        Ok(obj(vec![
            (
                "declaration",
                match &doc.declaration {
                    Some(decl) => obj(vec![
                        ("present", Json::Bool(true)),
                        ("version", Json::String(decl.version.clone())),
                        ("encoding", decl.encoding.clone().map(Json::String).unwrap_or(Json::Null)),
                        ("standalone", decl.standalone.map(Json::Bool).unwrap_or(Json::Null)),
                    ]),
                    None => obj(vec![("present", Json::Bool(false))]),
                },
            ),
            (
                "doctype",
                match &doc.doctype {
                    Some(raw) => obj(vec![("present", Json::Bool(true)), ("raw", Json::String(raw.clone())), ("prologPosition", Json::Number(doc.doctype_prolog_position as f64))]),
                    None => obj(vec![("present", Json::Bool(false))]),
                },
            ),
            ("prolog", Json::Array(doc.prolog.iter().map(project_node).collect())),
            ("root", doc.root.as_ref().map(project_node).unwrap_or(Json::Null)),
            ("epilog", Json::Array(doc.epilog.iter().map(project_node).collect())),
        ]))
    }

    fn project_node(node: &QNode) -> Json {
        match node {
            QNode::Text(text) => obj(vec![("kind", Json::String("text".into())), ("text", Json::String(text.clone()))]),
            QNode::CData(text) => obj(vec![("kind", Json::String("cdata".into())), ("text", Json::String(text.clone()))]),
            QNode::Comment(text) => obj(vec![("kind", Json::String("comment".into())), ("text", Json::String(text.clone()))]),
            QNode::Pi { target, data } => obj(vec![("kind", Json::String("pi".into())), ("target", Json::String(target.clone())), ("data", Json::String(data.clone()))]),
            QNode::Element { name, attrs, children } => {
                let mut plain: Vec<&(String, String)> = attrs.iter().filter(|(key, _)| key != "viewBox" && key != "transform").collect();
                plain.sort_by(|a, b| a.0.cmp(&b.0));
                let view_box = attrs.iter().find(|(key, _)| key == "viewBox").and_then(|(_, value)| parse_view_box(value).ok());
                let transform = attrs.iter().find(|(key, _)| key == "transform").and_then(|(_, value)| parse_transform_list(value).ok());
                obj(vec![
                    ("kind", Json::String("element".into())),
                    ("name", Json::String(name.clone())),
                    ("attrs", Json::Array(plain.into_iter().map(|(key, value)| Json::Array(vec![Json::String(key.clone()), Json::String(value.clone())])).collect())),
                    ("viewBox", view_box.map(|v| Json::Array(v.iter().map(|x| Json::Number(*x)).collect())).unwrap_or(Json::Null)),
                    ("transform", transform.map(|ops| Json::Array(ops.iter().map(transform_op_to_json).collect())).unwrap_or(Json::Null)),
                    ("children", Json::Array(children.iter().map(project_node).collect())),
                ])
            }
        }
    }
    //#endregion 🔖️Routing
}
//#endregion 🔖️Oracles

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation(input, &kind, &params)
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation_inverse(input, &kind, &params)
}

/// 🔁️ Decodes and re-encodes one real artifact through the reference implementation alone.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    oracles::round_trip(input)
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project_svg_1_1`].
#[cfg(feature = "oracles")]
pub fn project_svg_1_1(bytes: &[u8]) -> Result<Json, String> {
    oracles::project_svg_1_1(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_svg_1_1(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
