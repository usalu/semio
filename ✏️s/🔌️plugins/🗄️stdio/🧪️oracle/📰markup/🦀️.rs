//! 📰 Shared markup reference helpers — the `quick-xml`-backed element tree, the SVG 1.1
//! `viewBox`/`transform` grammars, the JSON spec codec and the semantic projection that the
//! `🎨️svg` 1.1 `✳️tiny` and `✳️basic` subset oracles both drive.
//!
//! This is a family module, the same shape as `📄️document`/`🖼️raster`/`🎒️archive`/`🔊️audio`/
//! `📊️tabular`/`🧊️mesh`: two subsets that need the same reference machinery share it HERE rather
//! than by copying it into both. `✳️tiny` and `✳️basic` are restrictions of one schema, so their
//! oracles differ only in the profile gate they apply and the vocabulary they dispatch — every
//! parse, write, address and projection below is genuinely common to both.
//!
//! Nothing here knows what a profile is. `MarkupDoc` is this module's OWN tree, structurally
//! parallel to (and never imported from) the production `XmlNode`/`XmlDocument` the `📰️xml` subset
//! persists, so a subset's own codec is never the thing measuring itself.
//!
//! @see ../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️tiny/🦀️oracle.rs
//! @see ../../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️basic/🦀️oracle.rs

//#region 🔖️Live
#[cfg(feature = "oracles")]
pub mod live {
    use quick_xml::events::{BytesCData, BytesDecl, BytesPI, BytesStart, BytesText, Event};
    use quick_xml::{Reader, Writer};
    use semio_repo_test_host::Json;

    //#region 🔖️Tree
    /// 🌳 One markup node as the reference reader sees it. Attributes stay an ordered `Vec` (source
    /// order preserved on read); the projection sorts them, since attribute order is writer freedom.
    #[derive(Clone, Debug, PartialEq)]
    pub enum MarkupNode {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<MarkupNode> },
        Text(String),
        CData(String),
        Comment(String),
        Pi { target: String, data: String },
    }

    /// 🏳️ The XML declaration, decomposed.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct MarkupDecl {
        pub version: String,
        pub encoding: Option<String>,
        pub standalone: Option<bool>,
    }

    /// 📄️ A whole markup document: declaration, doctype, the misc prolog and the root element.
    #[derive(Clone, Debug, Default)]
    pub struct MarkupDoc {
        pub declaration: Option<MarkupDecl>,
        pub doctype: Option<String>,
        pub prolog: Vec<MarkupNode>,
        pub root: Option<MarkupNode>,
    }
    //#endregion 🔖️Tree

    //#region 🔖️Address
    /// ✂️ Strips an XML namespace prefix (`xlink:href` → `href`) for vocabulary matching only.
    pub fn local_name(name: &str) -> &str {
        name.rsplit(':').next().unwrap_or(name)
    }

    /// 🧭 Child-index chain from the root element — the addressing scheme the SVG subsets' own
    /// `NodePath` uses, reimplemented here against `MarkupDoc` rather than imported.
    pub fn node_at<'a>(doc: &'a MarkupDoc, path: &[usize]) -> Result<&'a MarkupNode, String> {
        let mut node = doc.root.as_ref().ok_or("document has no root element")?;
        for &index in path {
            match node {
                MarkupNode::Element { children, .. } => node = children.get(index).ok_or_else(|| format!("child index {index} out of range"))?,
                _ => return Err("path descends into a non-element node".into()),
            }
        }
        Ok(node)
    }

    pub fn node_at_mut<'a>(doc: &'a mut MarkupDoc, path: &[usize]) -> Result<&'a mut MarkupNode, String> {
        let mut node = doc.root.as_mut().ok_or("document has no root element")?;
        for &index in path {
            match node {
                MarkupNode::Element { children, .. } => node = children.get_mut(index).ok_or_else(|| format!("child index {index} out of range"))?,
                _ => return Err("path descends into a non-element node".into()),
            }
        }
        Ok(node)
    }

    pub fn element_attr<'a>(node: &'a MarkupNode, name: &str) -> Option<&'a str> {
        match node {
            MarkupNode::Element { attrs, .. } => attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str()),
            _ => None,
        }
    }

    /// 🏷️ Update-in-place when present (so an untouched attribute keeps its source position), append
    /// when new, remove on `None`.
    pub fn set_attr(node: &mut MarkupNode, name: &str, value: Option<String>) {
        if let MarkupNode::Element { attrs, .. } = node {
            match value {
                Some(v) => match attrs.iter_mut().find(|(key, _)| key == name) {
                    Some(entry) => entry.1 = v,
                    None => attrs.push((name.to_string(), v)),
                },
                None => attrs.retain(|(key, _)| key != name),
            }
        }
    }

    /// ➕️ Inserts `node` as child `index` of the element at `parent`, clamping to the child count.
    pub fn insert_child(doc: &mut MarkupDoc, parent: &[usize], index: usize, node: MarkupNode) -> Result<(), String> {
        match node_at_mut(doc, parent)? {
            MarkupNode::Element { children, .. } => {
                let clamped = index.min(children.len());
                children.insert(clamped, node);
                Ok(())
            }
            _ => Err("insert: parent is not an element".into()),
        }
    }

    /// ➖️ Removes and returns child `index` of the element at `parent`.
    pub fn remove_child(doc: &mut MarkupDoc, parent: &[usize], index: usize) -> Result<MarkupNode, String> {
        match node_at_mut(doc, parent)? {
            MarkupNode::Element { children, .. } if index < children.len() => Ok(children.remove(index)),
            MarkupNode::Element { .. } => Err("remove: index out of range".into()),
            _ => Err("remove: parent is not an element".into()),
        }
    }

    /// 🔎 The child at `parent`/`index`, or `None` when either coordinate does not resolve.
    pub fn child_at<'a>(doc: &'a MarkupDoc, parent: &[usize], index: usize) -> Option<&'a MarkupNode> {
        match node_at(doc, parent).ok()? {
            MarkupNode::Element { children, .. } => children.get(index),
            _ => None,
        }
    }

    /// 🗺️ The path of the first element carrying `id`, depth-first from the root.
    pub fn path_of_id(doc: &MarkupDoc, id: &str) -> Option<Vec<usize>> {
        fn walk(node: &MarkupNode, id: &str, prefix: &mut Vec<usize>) -> Option<Vec<usize>> {
            if let MarkupNode::Element { attrs, children, .. } = node {
                if attrs.iter().any(|(key, value)| key == "id" && value == id) {
                    return Some(prefix.clone());
                }
                for (index, child) in children.iter().enumerate() {
                    prefix.push(index);
                    if let Some(found) = walk(child, id, prefix) {
                        return Some(found);
                    }
                    prefix.pop();
                }
            }
            None
        }
        walk(doc.root.as_ref()?, id, &mut Vec::new())
    }

    /// 🌳 Every element in the subtree rooted at `node`, itself included, depth-first.
    pub fn elements_of(node: &MarkupNode, out: &mut Vec<(String, Vec<(String, String)>)>) {
        if let MarkupNode::Element { name, attrs, children } = node {
            out.push((name.clone(), attrs.clone()));
            for child in children {
                elements_of(child, out);
            }
        }
    }

    /// 🌳 Rewrites every element in the subtree in place, depth-first, parents before children.
    pub fn rewrite_elements(node: &mut MarkupNode, rewrite: &mut impl FnMut(&mut String, &mut Vec<(String, String)>)) {
        if let MarkupNode::Element { name, attrs, children } = node {
            rewrite(name, attrs);
            for child in children.iter_mut() {
                rewrite_elements(child, rewrite);
            }
        }
    }

    /// ✂️ Drops every child (at any depth) the predicate rejects, then recurses into what is left.
    pub fn retain_elements(node: &mut MarkupNode, keep: &mut impl FnMut(&MarkupNode) -> bool) {
        if let MarkupNode::Element { children, .. } = node {
            children.retain(|child| keep(child));
            for child in children.iter_mut() {
                retain_elements(child, keep);
            }
        }
    }
    //#endregion 🔖️Address

    //#region 🔖️GeometryGrammar
    /// 🖼️ `viewBox="min-x min-y width height"`, comma-or-whitespace separated per SVG 1.1's own
    /// `<number>`/`<comma-wsp>` production — written independently of the subsets' `parse_view_box`.
    pub fn parse_view_box(source: &str) -> Result<[f64; 4], String> {
        let parts: Vec<&str> = source.split(|c: char| c.is_whitespace() || c == ',').filter(|p| !p.is_empty()).collect();
        if parts.len() != 4 {
            return Err(format!("viewBox: expected 4 numbers, got {}", parts.len()));
        }
        let f = |p: &str| p.parse::<f64>().map_err(|e| e.to_string());
        Ok([f(parts[0])?, f(parts[1])?, f(parts[2])?, f(parts[3])?])
    }

    pub fn format_view_box(value: &[f64; 4]) -> String {
        format!("{} {} {} {}", value[0], value[1], value[2], value[3])
    }

    /// 🔄 The SVG 1.1 `transform` grammar (`matrix|translate|scale|rotate|skewX|skewY`), written
    /// independently of the subsets' own `TransformOp`/`parse_transform_list`.
    #[derive(Clone, Debug, PartialEq)]
    pub enum MarkupTransformOp {
        Matrix { a: f64, b: f64, c: f64, d: f64, e: f64, f: f64 },
        Translate { x: f64, y: Option<f64> },
        Scale { x: f64, y: Option<f64> },
        Rotate { angle: f64, center: Option<(f64, f64)> },
        SkewX { angle: f64 },
        SkewY { angle: f64 },
    }

    pub fn parse_transform_list(source: &str) -> Result<Vec<MarkupTransformOp>, String> {
        let mut ops = Vec::new();
        let mut rest = source.trim();
        while !rest.is_empty() {
            let open = rest.find('(').ok_or("transform: expected '('")?;
            let name = rest[..open].trim();
            let close = open + rest[open..].find(')').ok_or("transform: expected ')'")?;
            let nums: Vec<f64> = rest[open + 1..close].split(|c: char| c.is_whitespace() || c == ',').filter(|p| !p.is_empty()).map(|p| p.parse::<f64>().map_err(|e| e.to_string())).collect::<Result<_, String>>()?;
            let op = match name {
                "matrix" if nums.len() == 6 => MarkupTransformOp::Matrix { a: nums[0], b: nums[1], c: nums[2], d: nums[3], e: nums[4], f: nums[5] },
                "translate" if nums.len() == 1 => MarkupTransformOp::Translate { x: nums[0], y: None },
                "translate" if nums.len() == 2 => MarkupTransformOp::Translate { x: nums[0], y: Some(nums[1]) },
                "scale" if nums.len() == 1 => MarkupTransformOp::Scale { x: nums[0], y: None },
                "scale" if nums.len() == 2 => MarkupTransformOp::Scale { x: nums[0], y: Some(nums[1]) },
                "rotate" if nums.len() == 1 => MarkupTransformOp::Rotate { angle: nums[0], center: None },
                "rotate" if nums.len() == 3 => MarkupTransformOp::Rotate { angle: nums[0], center: Some((nums[1], nums[2])) },
                "skewX" if nums.len() == 1 => MarkupTransformOp::SkewX { angle: nums[0] },
                "skewY" if nums.len() == 1 => MarkupTransformOp::SkewY { angle: nums[0] },
                other => return Err(format!("transform: unrecognised function {other:?} with {} args", nums.len())),
            };
            ops.push(op);
            rest = rest[close + 1..].trim_start_matches(|c: char| c.is_whitespace() || c == ',').trim_start();
        }
        Ok(ops)
    }

    pub fn format_transform_list(ops: &[MarkupTransformOp]) -> String {
        ops.iter()
            .map(|op| match op {
                MarkupTransformOp::Matrix { a, b, c, d, e, f } => format!("matrix({a},{b},{c},{d},{e},{f})"),
                MarkupTransformOp::Translate { x, y: None } => format!("translate({x})"),
                MarkupTransformOp::Translate { x, y: Some(y) } => format!("translate({x},{y})"),
                MarkupTransformOp::Scale { x, y: None } => format!("scale({x})"),
                MarkupTransformOp::Scale { x, y: Some(y) } => format!("scale({x},{y})"),
                MarkupTransformOp::Rotate { angle, center: None } => format!("rotate({angle})"),
                MarkupTransformOp::Rotate { angle, center: Some((cx, cy)) } => format!("rotate({angle},{cx},{cy})"),
                MarkupTransformOp::SkewX { angle } => format!("skewX({angle})"),
                MarkupTransformOp::SkewY { angle } => format!("skewY({angle})"),
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    //#endregion 🔖️GeometryGrammar

    //#region 🔖️JsonCodec
    pub fn obj(pairs: Vec<(&str, Json)>) -> Json {
        Json::Object(pairs.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
    }

    pub fn json_number(value: &Json) -> Option<f64> {
        match value {
            Json::Number(number) => Some(*number),
            _ => None,
        }
    }

    pub fn member(value: &Json, key: &str) -> Json {
        value.get(key).cloned().unwrap_or(Json::Null)
    }

    pub fn usize_member(value: &Json, key: &str) -> usize {
        json_number(&member(value, key)).unwrap_or(0.0).max(0.0) as usize
    }

    pub fn non_empty_str(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    pub fn json_to_path(value: &Json) -> Vec<usize> {
        match value {
            Json::Array(items) => items.iter().map(|item| json_number(item).unwrap_or(0.0).max(0.0) as usize).collect(),
            _ => Vec::new(),
        }
    }

    pub fn path_to_json(path: &[usize]) -> Json {
        Json::Array(path.iter().map(|&index| Json::Number(index as f64)).collect())
    }

    /// 🔎️ `{"kind":"element"|"text"|"cdata"|"comment"|"pi", ...}` — the node grammar every
    /// insert-shaped mutation spec speaks, and [`node_to_json`] its exact reverse.
    pub fn json_to_node(value: &Json) -> MarkupNode {
        match value.str("kind").as_str() {
            "text" => MarkupNode::Text(value.str("text")),
            "cdata" => MarkupNode::CData(value.str("text")),
            "comment" => MarkupNode::Comment(value.str("text")),
            "pi" => MarkupNode::Pi { target: value.str("target"), data: value.str("data") },
            _ => MarkupNode::Element { name: value.str("name"), attrs: value.array("attrs").iter().map(|a| (a.str("name"), a.str("value"))).collect(), children: value.array("children").iter().map(json_to_node).collect() },
        }
    }

    pub fn node_to_json(node: &MarkupNode) -> Json {
        match node {
            MarkupNode::Text(text) => obj(vec![("kind", Json::String("text".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::CData(text) => obj(vec![("kind", Json::String("cdata".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::Comment(text) => obj(vec![("kind", Json::String("comment".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::Pi { target, data } => obj(vec![("kind", Json::String("pi".into())), ("target", Json::String(target.clone())), ("data", Json::String(data.clone()))]),
            MarkupNode::Element { name, attrs, children } => obj(vec![
                ("kind", Json::String("element".into())),
                ("name", Json::String(name.clone())),
                ("attrs", Json::Array(attrs.iter().map(|(key, value)| obj(vec![("name", Json::String(key.clone())), ("value", Json::String(value.clone()))])).collect())),
                ("children", Json::Array(children.iter().map(node_to_json).collect())),
            ]),
        }
    }

    pub fn json_to_transform_op(value: &Json) -> Result<MarkupTransformOp, String> {
        let num = |key: &str| json_number(&member(value, key)).unwrap_or(0.0);
        let opt_num = |key: &str| value.get(key).and_then(json_number);
        match value.str("kind").as_str() {
            "matrix" => Ok(MarkupTransformOp::Matrix { a: num("a"), b: num("b"), c: num("c"), d: num("d"), e: num("e"), f: num("f") }),
            "translate" => Ok(MarkupTransformOp::Translate { x: num("x"), y: opt_num("y") }),
            "scale" => Ok(MarkupTransformOp::Scale { x: num("x"), y: opt_num("y") }),
            "rotate" => Ok(MarkupTransformOp::Rotate {
                angle: num("angle"),
                center: match (opt_num("cx"), opt_num("cy")) {
                    (Some(cx), Some(cy)) => Some((cx, cy)),
                    _ => None,
                },
            }),
            "skewX" => Ok(MarkupTransformOp::SkewX { angle: num("angle") }),
            "skewY" => Ok(MarkupTransformOp::SkewY { angle: num("angle") }),
            other => Err(format!("transform op: unrecognised kind {other:?}")),
        }
    }

    pub fn transform_op_to_json(op: &MarkupTransformOp) -> Json {
        let n = Json::Number;
        match op {
            MarkupTransformOp::Matrix { a, b, c, d, e, f } => obj(vec![("kind", Json::String("matrix".into())), ("a", n(*a)), ("b", n(*b)), ("c", n(*c)), ("d", n(*d)), ("e", n(*e)), ("f", n(*f))]),
            MarkupTransformOp::Translate { x, y } => obj(vec![("kind", Json::String("translate".into())), ("x", n(*x)), ("y", y.map(n).unwrap_or(Json::Null))]),
            MarkupTransformOp::Scale { x, y } => obj(vec![("kind", Json::String("scale".into())), ("x", n(*x)), ("y", y.map(n).unwrap_or(Json::Null))]),
            MarkupTransformOp::Rotate { angle, center } => obj(vec![("kind", Json::String("rotate".into())), ("angle", n(*angle)), ("cx", center.map(|(cx, _)| n(cx)).unwrap_or(Json::Null)), ("cy", center.map(|(_, cy)| n(cy)).unwrap_or(Json::Null))]),
            MarkupTransformOp::SkewX { angle } => obj(vec![("kind", Json::String("skewX".into())), ("angle", n(*angle))]),
            MarkupTransformOp::SkewY { angle } => obj(vec![("kind", Json::String("skewY".into())), ("angle", n(*angle))]),
        }
    }
    //#endregion 🔖️JsonCodec

    //#region 🔖️Parse
    /// 📥️ Builds a [`MarkupDoc`] from real bytes via `quick_xml::Reader`'s event stream.
    /// `Event::Text` bodies arrive with entity/character references split out into separate
    /// `Event::GeneralRef` events (a quick-xml 0.42 reader behaviour, not opt-in); runs of
    /// `Text`+`GeneralRef` between two structural events are coalesced back into ONE
    /// `MarkupNode::Text`, matching the single-text-node-per-run model the SVG subsets persist.
    pub fn parse_markup(bytes: &[u8]) -> Result<MarkupDoc, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| format!("markup source is not UTF-8: {error}"))?;
        let mut reader = Reader::from_str(text);
        let mut doc = MarkupDoc::default();
        let mut stack: Vec<(String, Vec<(String, String)>, Vec<MarkupNode>)> = Vec::new();
        let mut text_buf = String::new();

        fn flush(text_buf: &mut String, stack: &mut [(String, Vec<(String, String)>, Vec<MarkupNode>)]) {
            if text_buf.is_empty() {
                return;
            }
            if let Some((_, _, children)) = stack.last_mut() {
                children.push(MarkupNode::Text(std::mem::take(text_buf)));
            } else {
                text_buf.clear();
            }
        }

        fn attach(stack: &mut Vec<(String, Vec<(String, String)>, Vec<MarkupNode>)>, doc: &mut MarkupDoc, node: MarkupNode, is_misc: bool) {
            if let Some((_, _, children)) = stack.last_mut() {
                children.push(node);
            } else if doc.root.is_none() {
                if is_misc {
                    doc.prolog.push(node);
                } else {
                    doc.root = Some(node);
                }
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
                    doc.declaration = Some(MarkupDecl { version, encoding, standalone });
                }
                Event::DocType(raw) => doc.doctype = Some(raw.as_ref().to_string()),
                Event::PI(pi) => {
                    flush(&mut text_buf, &mut stack);
                    attach(&mut stack, &mut doc, MarkupNode::Pi { target: pi.target().to_string(), data: pi.content().to_string() }, true);
                }
                Event::Comment(text) => {
                    flush(&mut text_buf, &mut stack);
                    attach(&mut stack, &mut doc, MarkupNode::Comment(text.as_ref().to_string()), true);
                }
                Event::CData(text) => {
                    flush(&mut text_buf, &mut stack);
                    if let Some((_, _, children)) = stack.last_mut() {
                        children.push(MarkupNode::CData(text.as_ref().to_string()));
                    }
                }
                Event::Text(text) => text_buf.push_str(text.as_ref()),
                Event::GeneralRef(reference) => match reference.resolve_char_ref().map_err(|error| error.to_string())? {
                    Some(ch) => text_buf.push(ch),
                    None => match quick_xml::escape::resolve_predefined_entity(reference.as_ref()) {
                        Some(resolved) => text_buf.push_str(resolved),
                        None => return Err(format!("markup source references unknown entity &{};", reference.as_ref())),
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
                    attach(&mut stack, &mut doc, MarkupNode::Element { name: start.name().as_ref().to_string(), attrs, children: Vec::new() }, false);
                }
                Event::End(_) => {
                    flush(&mut text_buf, &mut stack);
                    let (name, attrs, children) = stack.pop().ok_or("markup source: unmatched closing tag")?;
                    attach(&mut stack, &mut doc, MarkupNode::Element { name, attrs, children }, false);
                }
                Event::Eof => {
                    flush(&mut text_buf, &mut stack);
                    break;
                }
            }
        }
        if doc.root.is_none() {
            return Err("markup document requires a root element".into());
        }
        Ok(doc)
    }
    //#endregion 🔖️Parse

    //#region 🔖️Write
    /// 📤️ Re-serializes a [`MarkupDoc`] via `quick_xml::Writer`. Self-closing versus explicit empty
    /// open/close is never distinguished on read (both parse to zero children), so an empty element
    /// is always written `Event::Empty` — real writer freedom, narrowed out of the projection rather
    /// than chased byte-for-byte.
    fn write_node(writer: &mut Writer<Vec<u8>>, node: &MarkupNode) -> Result<(), String> {
        match node {
            MarkupNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
            MarkupNode::CData(text) => writer.write_event(Event::CData(BytesCData::new(text.as_str()))).map_err(|error| error.to_string()),
            MarkupNode::Comment(text) => writer.write_event(Event::Comment(BytesText::from_escaped(text.as_str()))).map_err(|error| error.to_string()),
            MarkupNode::Pi { target, data } => {
                let content = if data.is_empty() { target.clone() } else { format!("{target} {data}") };
                writer.write_event(Event::PI(BytesPI::new(content))).map_err(|error| error.to_string())
            }
            MarkupNode::Element { name, attrs, children } => {
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

    pub fn write_markup(doc: &MarkupDoc) -> Result<Vec<u8>, String> {
        let mut writer = Writer::new(Vec::new());
        if let Some(decl) = &doc.declaration {
            let standalone = decl.standalone.map(|value| if value { "yes" } else { "no" });
            writer.write_event(Event::Decl(BytesDecl::new(&decl.version, decl.encoding.as_deref(), standalone))).map_err(|error| error.to_string())?;
        }
        for node in &doc.prolog {
            write_node(&mut writer, node)?;
        }
        if let Some(raw) = &doc.doctype {
            writer.write_event(Event::DocType(BytesText::from_escaped(raw.as_str()))).map_err(|error| error.to_string())?;
        }
        if let Some(root) = &doc.root {
            write_node(&mut writer, root)?;
        }
        Ok(writer.into_inner())
    }
    //#endregion 🔖️Write

    //#region 🔖️Projection
    /// 👁️ The shared semantic projection: declaration/doctype presence and fields, the prolog, and
    /// the root tree with attributes name-sorted and `viewBox`/`transform` decomposed into typed
    /// numeric geometry (excluded from the raw attribute list, so writer freedom in their STRING
    /// formatting never fails an otherwise-identical mutation).
    pub fn project_markup(doc: &MarkupDoc) -> Json {
        obj(vec![
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
                    Some(raw) => obj(vec![("present", Json::Bool(true)), ("raw", Json::String(raw.clone()))]),
                    None => obj(vec![("present", Json::Bool(false))]),
                },
            ),
            ("prolog", Json::Array(doc.prolog.iter().map(project_node).collect())),
            ("root", doc.root.as_ref().map(project_node).unwrap_or(Json::Null)),
        ])
    }

    pub fn project_node(node: &MarkupNode) -> Json {
        match node {
            MarkupNode::Text(text) => obj(vec![("kind", Json::String("text".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::CData(text) => obj(vec![("kind", Json::String("cdata".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::Comment(text) => obj(vec![("kind", Json::String("comment".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::Pi { target, data } => obj(vec![("kind", Json::String("pi".into())), ("target", Json::String(target.clone())), ("data", Json::String(data.clone()))]),
            MarkupNode::Element { name, attrs, children } => {
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
    //#endregion 🔖️Projection
}
//#endregion 🔖️Live
