//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `quick-xml` reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. XML has no shared family helper (unlike
//! `document`/`raster`/...) since no other subset genuinely shares this implementation — SVG 1.1
//! also builds on `quick-xml` but owns its own subset and its own oracle module untouched here.
//!
//! Two entry points: [`oracle_apply_mutation`] performs the FORWARD mutation (the `mutate-<kind>`
//! scenarios), [`oracle_apply_mutation_inverse`] performs the forward mutation and then its computed
//! inverse in sequence (the `inverse-<kind>` scenarios) — the same "apply, then apply the inverse,
//! land back on the start" law `XmlMutation::inverse` proves at the Rust-model level, proven here
//! independently against the registered reference library. [`project_xml_1_0`] is the shared
//! independent-reader projection both this module's own handlers AND the case's subject handlers
//! read their results back through before comparison.
//!
//! `quick-xml` 0.42 splits every `&entity;`/`&#NNN;` reference out of `Text` into its own
//! `Event::GeneralRef`, so a text run is accumulated across `Text`/`GeneralRef` events rather than
//! read as one event — see [`resolve_general_ref`]. DOCTYPE internal-subset support is intentionally
//! narrowed to SYSTEM/PUBLIC external ids plus typed `<!ENTITY>` declarations, the exact same scope
//! `crate::artifacts::xml::schema::snapshot::XmlDoctype` itself models (this subset's own writer
//! freedom, documented rather than silently dropped).
//!
//! @see ./🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`XmlMutation::KINDS`).

use semio_repo_test_host::Json;

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use quick_xml::escape::resolve_xml_entity;
    use quick_xml::events::{BytesCData, BytesDecl, BytesEnd, BytesPI, BytesRef, BytesStart, BytesText, Event};
    use quick_xml::reader::Reader;
    use quick_xml::writer::Writer;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::Json;
    use std::io::Cursor;

    //#region 🔖️Tree
    /// 🌳 Owned XML node, independent of `crate::artifacts::xml::schema::snapshot::XmlNode` (this
    /// crate never depends on `semio-s-plugin-stdio`, the production crate that type lives in — see
    /// this file's own header) but shaped identically variant for variant, so a spec written for the
    /// oracle reads the same as one written for the subject.
    #[derive(Clone, Debug, PartialEq)]
    enum XNode {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<XNode> },
        Text(String),
        CData(String),
        Comment(String),
        Pi { target: String, data: String },
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct XDecl {
        version: String,
        encoding: Option<String>,
        standalone: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq)]
    enum XExternalId {
        System { system_id: String },
        Public { public_id: String, system_id: String },
    }

    #[derive(Clone, Debug, PartialEq)]
    struct XEntity {
        parameter: bool,
        name: String,
        value: String,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct XDoctype {
        name: String,
        external_id: Option<XExternalId>,
        entities: Vec<XEntity>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct XDoc {
        declaration: Option<XDecl>,
        doctype: Option<XDoctype>,
        prolog: Vec<XNode>,
        root: Option<XNode>,
    }
    //#endregion 🔖️Tree

    //#region 🔖️JsonValue
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }

    fn usize_path(items: Vec<Json>) -> Vec<usize> {
        items
            .iter()
            .map(|item| match item {
                Json::Number(n) => n.max(0.0) as usize,
                _ => 0,
            })
            .collect()
    }

    fn non_empty(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    /// 🔎️ Owned node-spec JSON grammar mutation params speak: `{"kind":"element","name":...,
    /// "attrs":[{"name":...,"value":...}],"children":[...]}` | `{"kind":"text"|"cdata"|"comment",
    /// "text":...}` | `{"kind":"pi","target":...,"data":...}`.
    fn json_to_xnode(value: &Json) -> Result<XNode, String> {
        match value.str("kind").as_str() {
            "element" => Ok(XNode::Element {
                name: value.str("name"),
                attrs: value.array("attrs").iter().map(|attr| (attr.str("name"), attr.str("value"))).collect(),
                children: value.array("children").iter().map(json_to_xnode).collect::<Result<Vec<_>, _>>()?,
            }),
            "text" => Ok(XNode::Text(value.str("text"))),
            "cdata" => Ok(XNode::CData(value.str("text"))),
            "comment" => Ok(XNode::Comment(value.str("text"))),
            "pi" => Ok(XNode::Pi { target: value.str("target"), data: value.str("data") }),
            other => Err(format!("unknown node kind {other:?}")),
        }
    }

    /// 🔁️ The reverse of [`json_to_xnode`] — used to capture a removed node's exact value so
    /// `inverse_spec` can hand it back to [`json_to_xnode`] as the undo's own `insert-element` params.
    fn xnode_to_json(node: &XNode) -> Json {
        match node {
            XNode::Element { name, attrs, children } => Json::Object(vec![
                ("kind".to_string(), Json::String("element".to_string())),
                ("name".to_string(), Json::String(name.clone())),
                ("attrs".to_string(), Json::Array(attrs.iter().map(|(key, value)| Json::Object(vec![("name".to_string(), Json::String(key.clone())), ("value".to_string(), Json::String(value.clone()))])).collect())),
                ("children".to_string(), Json::Array(children.iter().map(xnode_to_json).collect())),
            ]),
            XNode::Text(text) => Json::Object(vec![("kind".to_string(), Json::String("text".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            XNode::CData(text) => Json::Object(vec![("kind".to_string(), Json::String("cdata".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            XNode::Comment(text) => Json::Object(vec![("kind".to_string(), Json::String("comment".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            XNode::Pi { target, data } => Json::Object(vec![("kind".to_string(), Json::String("pi".to_string())), ("target".to_string(), Json::String(target.clone())), ("data".to_string(), Json::String(data.clone()))]),
        }
    }

    /// 📄️ `{"version":...,"encoding":...,"standalone":...}` when present, absent (no `version` key)
    /// meaning "no declaration" — the same convention `set-doctype`'s `name` key uses below.
    fn json_to_declaration(params: &Json) -> Option<XDecl> {
        non_empty(params, "version").map(|version| XDecl {
            version,
            encoding: non_empty(params, "encoding"),
            standalone: match params.get("standalone") {
                Some(Json::Bool(value)) => Some(*value),
                _ => None,
            },
        })
    }

    fn declaration_to_json(declaration: &Option<XDecl>) -> Json {
        match declaration {
            None => Json::Object(vec![]),
            Some(decl) => Json::Object(vec![
                ("version".to_string(), Json::String(decl.version.clone())),
                (
                    "encoding".to_string(),
                    match &decl.encoding {
                        Some(value) => Json::String(value.clone()),
                        None => Json::Null,
                    },
                ),
                (
                    "standalone".to_string(),
                    match decl.standalone {
                        Some(value) => Json::Bool(value),
                        None => Json::Null,
                    },
                ),
            ]),
        }
    }

    fn json_to_doctype(params: &Json) -> Option<XDoctype> {
        let name = non_empty(params, "name")?;
        let external_id = match params.get("externalId") {
            Some(value) if !matches!(value, Json::Null) => match value.str("kind").as_str() {
                "system" => Some(XExternalId::System { system_id: value.str("systemId") }),
                "public" => Some(XExternalId::Public { public_id: value.str("publicId"), system_id: value.str("systemId") }),
                _ => None,
            },
            _ => None,
        };
        let entities = params.array("entities").iter().map(|entry| XEntity { parameter: matches!(entry.get("parameter"), Some(Json::Bool(true))), name: entry.str("name"), value: entry.str("value") }).collect();
        Some(XDoctype { name, external_id, entities })
    }

    fn doctype_to_json(doctype: &Option<XDoctype>) -> Json {
        match doctype {
            None => Json::Object(vec![]),
            Some(dt) => Json::Object(vec![
                ("name".to_string(), Json::String(dt.name.clone())),
                (
                    "externalId".to_string(),
                    match &dt.external_id {
                        None => Json::Null,
                        Some(XExternalId::System { system_id }) => Json::Object(vec![("kind".to_string(), Json::String("system".to_string())), ("systemId".to_string(), Json::String(system_id.clone()))]),
                        Some(XExternalId::Public { public_id, system_id }) => {
                            Json::Object(vec![("kind".to_string(), Json::String("public".to_string())), ("publicId".to_string(), Json::String(public_id.clone())), ("systemId".to_string(), Json::String(system_id.clone()))])
                        }
                    },
                ),
                (
                    "entities".to_string(),
                    Json::Array(
                        dt.entities
                            .iter()
                            .map(|entity| Json::Object(vec![("parameter".to_string(), Json::Bool(entity.parameter)), ("name".to_string(), Json::String(entity.name.clone())), ("value".to_string(), Json::String(entity.value.clone()))]))
                            .collect(),
                    ),
                ),
            ]),
        }
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️PathAddressing
    /// 🔎️ Immutable walk of `path` (a chain of child indices) from `root`, mirroring
    /// `crate::artifacts::xml::schema::mutations::XmlNodePath::resolve` — `path == []` addresses
    /// `root` itself.
    fn resolve<'a>(root: Option<&'a XNode>, path: &[usize]) -> Option<&'a XNode> {
        let mut current = root?;
        for &index in path {
            let XNode::Element { children, .. } = current else { return None };
            current = children.get(index)?;
        }
        Some(current)
    }

    fn resolve_mut<'a>(root: Option<&'a mut XNode>, path: &[usize]) -> Option<&'a mut XNode> {
        let mut current = root?;
        for &index in path {
            let XNode::Element { children, .. } = current else { return None };
            current = children.get_mut(index)?;
        }
        Some(current)
    }
    //#endregion 🔖️PathAddressing

    //#region 🔖️Parse
    /// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text — numeric
    /// character references via `resolve_char_ref`, the five predefined XML entities via
    /// `resolve_xml_entity`, anything else a hard parse error. Exactly the same 5-entity-plus-numeric
    /// scope `crate::artifacts::xml::schema::snapshot::xml_unescape_text` narrows production to.
    fn resolve_general_ref(reference: &BytesRef) -> Result<String, String> {
        if let Some(ch) = reference.resolve_char_ref().map_err(|error| error.to_string())? {
            return Ok(ch.to_string());
        }
        match resolve_xml_entity(reference.as_ref()) {
            Some(resolved) => Ok(resolved.to_string()),
            None => Err(format!("unknown entity &{};", reference.as_ref())),
        }
    }

    fn read_attrs(start: &BytesStart) -> Result<Vec<(String, String)>, String> {
        start
            .attributes()
            .map(|attr| {
                let attr = attr.map_err(|error| error.to_string())?;
                let value = attr.normalized_value(XmlVersion::Explicit1_0).map_err(|error| error.to_string())?;
                Ok((attr.key.as_ref().to_string(), value.to_string()))
            })
            .collect()
    }

    fn flush_text(text_run: &mut String, children: &mut Vec<XNode>) {
        if !text_run.is_empty() {
            children.push(XNode::Text(std::mem::take(text_run)));
        }
    }

    /// 🌳 Recursive-descent element parse: reads events until this element's own `End`, recursing
    /// into `Start` children — avoids hand-rolling an explicit open-element stack.
    fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<XNode, String> {
        let name = start.name().as_ref().to_string();
        let attrs = read_attrs(&start)?;
        let mut children = Vec::new();
        let mut text_run = String::new();
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::End(_) => {
                    flush_text(&mut text_run, &mut children);
                    return Ok(XNode::Element { name, attrs, children });
                }
                Event::Start(child_start) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(parse_element(reader, child_start)?);
                }
                Event::Empty(child_start) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(XNode::Element { name: child_start.name().as_ref().to_string(), attrs: read_attrs(&child_start)?, children: Vec::new() });
                }
                Event::Text(text) => text_run.push_str(text.as_ref()),
                Event::GeneralRef(reference) => text_run.push_str(&resolve_general_ref(&reference)?),
                Event::CData(cdata) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(XNode::CData(cdata.into_inner().into_owned()));
                }
                Event::Comment(comment) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(XNode::Comment(comment.as_ref().to_string()));
                }
                Event::PI(pi) => {
                    flush_text(&mut text_run, &mut children);
                    children.push(XNode::Pi { target: pi.target().to_string(), data: pi.content().to_string() });
                }
                Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
                Event::Decl(_) | Event::DocType(_) => return Err(format!("declaration/doctype cannot appear inside element <{name}>")),
            }
        }
    }

    fn decl_from_event(decl: &BytesDecl) -> Result<XDecl, String> {
        let version = decl.version().map_err(|error| error.to_string())?.to_string();
        let encoding = match decl.encoding() {
            Some(result) => Some(result.map_err(|error| error.to_string())?.to_string()),
            None => None,
        };
        let standalone = match decl.standalone() {
            Some(result) => Some(result.map_err(|error| error.to_string())?.as_ref() == "yes"),
            None => None,
        };
        Ok(XDecl { version, encoding, standalone })
    }

    /// 📜️ Parses the DOCTYPE content `quick-xml`'s `Event::DocType` hands back (everything between
    /// `<!DOCTYPE` and the matching `>`) into `name (SYSTEM "sysid" | PUBLIC "pubid" "sysid")? ([
    /// <!ENTITY (%)? name "value"> ... ])?`, the same narrowed scope
    /// `crate::artifacts::xml::schema::snapshot::parse_doctype` models. Independent hand-rolled
    /// parser (this crate never depends on that production module).
    fn parse_doctype(raw: &str) -> Result<XDoctype, String> {
        let mut pos = 0usize;
        let bytes = raw.as_bytes();
        skip_ws(bytes, &mut pos);
        let name = parse_name(raw, &mut pos)?;
        skip_ws(bytes, &mut pos);
        let external_id = if raw[pos..].starts_with("SYSTEM") {
            pos += "SYSTEM".len();
            Some(XExternalId::System { system_id: parse_quoted(raw, &mut pos)? })
        } else if raw[pos..].starts_with("PUBLIC") {
            pos += "PUBLIC".len();
            let public_id = parse_quoted(raw, &mut pos)?;
            let system_id = parse_quoted(raw, &mut pos)?;
            Some(XExternalId::Public { public_id, system_id })
        } else {
            None
        };
        skip_ws(bytes, &mut pos);
        let mut entities = Vec::new();
        if raw[pos..].starts_with('[') {
            pos += 1;
            loop {
                skip_ws(bytes, &mut pos);
                if raw[pos..].starts_with(']') {
                    break;
                }
                if !raw[pos..].starts_with("<!ENTITY") {
                    return Err("unsupported XML DTD declaration; only typed ENTITY declarations are modeled".to_string());
                }
                pos += "<!ENTITY".len();
                skip_ws(bytes, &mut pos);
                let parameter = raw[pos..].starts_with('%');
                if parameter {
                    pos += 1;
                    skip_ws(bytes, &mut pos);
                }
                let entity_name = parse_name(raw, &mut pos)?;
                let value = parse_quoted(raw, &mut pos)?;
                skip_ws(bytes, &mut pos);
                if !raw[pos..].starts_with('>') {
                    return Err("expected > after XML entity declaration".to_string());
                }
                pos += 1;
                entities.push(XEntity { parameter, name: entity_name, value });
            }
        }
        Ok(XDoctype { name, external_id, entities })
    }

    fn skip_ws(bytes: &[u8], pos: &mut usize) {
        while *pos < bytes.len() && bytes[*pos].is_ascii_whitespace() {
            *pos += 1;
        }
    }

    fn parse_name(s: &str, pos: &mut usize) -> Result<String, String> {
        let start = *pos;
        while *pos < s.len() {
            let ch = s[*pos..].chars().next().unwrap();
            if ch.is_whitespace() || ch == '[' || ch == '>' {
                break;
            }
            *pos += ch.len_utf8();
        }
        if *pos == start {
            return Err("expected XML doctype name".to_string());
        }
        Ok(s[start..*pos].to_string())
    }

    fn parse_quoted(s: &str, pos: &mut usize) -> Result<String, String> {
        skip_ws(s.as_bytes(), pos);
        let quote = s[*pos..].chars().next().ok_or("expected quoted doctype literal")?;
        if quote != '"' && quote != '\'' {
            return Err("doctype literal must be quoted".to_string());
        }
        *pos += 1;
        let start = *pos;
        while *pos < s.len() {
            let ch = s[*pos..].chars().next().unwrap();
            if ch == quote {
                let value = s[start..*pos].to_string();
                *pos += 1;
                return Ok(value);
            }
            *pos += ch.len_utf8();
        }
        Err("unclosed doctype literal".to_string())
    }

    fn parse(bytes: &[u8]) -> Result<XDoc, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        let mut reader = Reader::from_str(text);
        let mut doc = XDoc::default();
        let mut root_seen = false;
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::Eof => break,
                Event::Decl(decl) => doc.declaration = Some(decl_from_event(&decl)?),
                Event::DocType(doctype) => doc.doctype = Some(parse_doctype(doctype.as_ref().trim())?),
                Event::PI(pi) if !root_seen => doc.prolog.push(XNode::Pi { target: pi.target().to_string(), data: pi.content().to_string() }),
                Event::PI(_) => {}
                Event::Comment(comment) if !root_seen => doc.prolog.push(XNode::Comment(comment.as_ref().to_string())),
                Event::Comment(_) => {}
                Event::Text(text) => {
                    if !text.as_ref().trim().is_empty() {
                        return Err(if root_seen { "trailing content after root element".to_string() } else { "unexpected text before the root element".to_string() });
                    }
                }
                Event::GeneralRef(reference) => return Err(format!("unexpected entity reference &{}; outside the root element", reference.as_ref())),
                Event::CData(_) => return Err("unexpected CDATA section outside the root element".to_string()),
                Event::Start(start) => {
                    if root_seen {
                        return Err("multiple root elements".to_string());
                    }
                    doc.root = Some(parse_element(&mut reader, start)?);
                    root_seen = true;
                }
                Event::Empty(start) => {
                    if root_seen {
                        return Err("multiple root elements".to_string());
                    }
                    doc.root = Some(XNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() });
                    root_seen = true;
                }
                Event::End(_) => return Err("unexpected closing tag before the root element".to_string()),
            }
        }
        if doc.root.is_none() {
            return Err("document has no root element".to_string());
        }
        Ok(doc)
    }
    //#endregion 🔖️Parse

    //#region 🔖️Serialize
    fn doctype_content(doctype: &XDoctype) -> String {
        let mut out = doctype.name.clone();
        match &doctype.external_id {
            Some(XExternalId::System { system_id }) => out.push_str(&format!(" SYSTEM \"{}\"", escape_dtd_literal(system_id))),
            Some(XExternalId::Public { public_id, system_id }) => out.push_str(&format!(" PUBLIC \"{}\" \"{}\"", escape_dtd_literal(public_id), escape_dtd_literal(system_id))),
            None => {}
        }
        if !doctype.entities.is_empty() {
            out.push_str(" [");
            for entity in &doctype.entities {
                out.push_str("<!ENTITY ");
                if entity.parameter {
                    out.push_str("% ");
                }
                out.push_str(&entity.name);
                out.push_str(&format!(" \"{}\">", escape_dtd_literal(&entity.value)));
            }
            out.push(']');
        }
        out
    }

    /// ✂️️ Minimal escaping for a quoted DTD literal (system/public id, entity value) — mirrors
    /// `crate::artifacts::xml::schema::snapshot::xml_escape_attr`'s own narrow scope (`&`, `"`).
    fn escape_dtd_literal(raw: &str) -> String {
        raw.replace('&', "&amp;").replace('"', "&quot;")
    }

    fn write_node<W: std::io::Write>(writer: &mut Writer<W>, node: &XNode) -> Result<(), String> {
        match node {
            XNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
            XNode::CData(text) => writer.write_event(Event::CData(BytesCData::new(text.as_str()))).map_err(|error| error.to_string()),
            XNode::Comment(text) => writer.write_event(Event::Comment(BytesText::from_escaped(text.as_str()))).map_err(|error| error.to_string()),
            XNode::Pi { target, data } => {
                let content = if data.is_empty() { target.clone() } else { format!("{target} {data}") };
                writer.write_event(Event::PI(BytesPI::new(content))).map_err(|error| error.to_string())
            }
            XNode::Element { name, attrs, children } => {
                let mut start = BytesStart::new(name.as_str());
                for (key, value) in attrs {
                    start.push_attribute((key.as_str(), value.as_str()));
                }
                if children.is_empty() {
                    return writer.write_event(Event::Empty(start)).map_err(|error| error.to_string());
                }
                writer.write_event(Event::Start(start)).map_err(|error| error.to_string())?;
                for child in children {
                    write_node(writer, child)?;
                }
                writer.write_event(Event::End(BytesEnd::new(name.as_str()))).map_err(|error| error.to_string())
            }
        }
    }

    fn serialize(doc: &XDoc) -> Result<Vec<u8>, String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        if let Some(decl) = &doc.declaration {
            let standalone = decl.standalone.map(|value| if value { "yes" } else { "no" });
            writer.write_event(Event::Decl(BytesDecl::new(&decl.version, decl.encoding.as_deref(), standalone))).map_err(|error| error.to_string())?;
        }
        for node in &doc.prolog {
            write_node(&mut writer, node)?;
        }
        if let Some(doctype) = &doc.doctype {
            writer.write_event(Event::DocType(BytesText::from_escaped(doctype_content(doctype)))).map_err(|error| error.to_string())?;
        }
        if let Some(root) = &doc.root {
            write_node(&mut writer, root)?;
        }
        Ok(writer.into_inner().into_inner())
    }
    //#endregion 🔖️Serialize

    //#region 🔖️Forward
    /// ▶️ Applies one `{kind, params}` mutation to `doc` in place. Out-of-range indices / unresolved
    /// paths are errors here (never a silent no-op), matching this dispatch's own contract, though
    /// every example this subset's own feature exercises resolves against the real document.
    fn apply_kind(doc: &mut XDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => {
                *doc = parse(params.str("xml").as_bytes())?;
            }
            "set-declaration" => doc.declaration = json_to_declaration(params),
            "set-doctype" => doc.doctype = json_to_doctype(params),
            "insert-element" => {
                let path = usize_path(params.array("path"));
                let index = usize_field(params, "index");
                let node = json_to_xnode(&params.get("node").cloned().unwrap_or(Json::Null))?;
                let XNode::Element { children, .. } = resolve_mut(doc.root.as_mut(), &path).ok_or("insert-element: path does not resolve to an element")? else {
                    return Err("insert-element: path does not address an element".to_string());
                };
                children.insert(index.min(children.len()), node);
            }
            "remove-element" => {
                let path = usize_path(params.array("path"));
                let index = usize_field(params, "index");
                let XNode::Element { children, .. } = resolve_mut(doc.root.as_mut(), &path).ok_or("remove-element: path does not resolve to an element")? else {
                    return Err("remove-element: path does not address an element".to_string());
                };
                if index < children.len() {
                    children.remove(index);
                }
            }
            "set-attribute" => {
                let path = usize_path(params.array("path"));
                let name = params.str("name");
                let value = match params.get("value") {
                    Some(Json::String(text)) => Some(text.clone()),
                    _ => None,
                };
                let XNode::Element { attrs, .. } = resolve_mut(doc.root.as_mut(), &path).ok_or("set-attribute: path does not resolve to an element")? else {
                    return Err("set-attribute: path does not address an element".to_string());
                };
                match value {
                    Some(next) => match attrs.iter_mut().find(|(key, _)| key == &name) {
                        Some(entry) => entry.1 = next,
                        None => attrs.push((name, next)),
                    },
                    None => attrs.retain(|(key, _)| key != &name),
                }
            }
            "set-text" => {
                let path = usize_path(params.array("path"));
                let text = params.str("text");
                let XNode::Text(current) = resolve_mut(doc.root.as_mut(), &path).ok_or("set-text: path does not resolve to a text node")? else {
                    return Err("set-text: path does not address a text node".to_string());
                };
                *current = text;
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Reads `base` (the CURRENT, pre-mutation document) to build the spec that undoes `{kind,
    /// params}` — same law `XmlMutation::inverse` proves at the Rust-model level, computed here
    /// against the reference library instead.
    fn inverse_spec(base: &XDoc, kind: &str, params: &Json) -> Json {
        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);
        let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec("set-snapshot", obj(vec![("xml", Json::String(String::from_utf8(serialize(base).unwrap_or_default()).unwrap_or_default()))])),
            "set-declaration" => spec("set-declaration", declaration_to_json(&base.declaration)),
            "set-doctype" => spec("set-doctype", doctype_to_json(&base.doctype)),
            "insert-element" => {
                let path_json = params.array("path");
                let index = usize_field(params, "index");
                spec("remove-element", obj(vec![("path", Json::Array(path_json)), ("index", Json::Number(index as f64))]))
            }
            "remove-element" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let index = usize_field(params, "index");
                let node = match resolve(base.root.as_ref(), &path) {
                    Some(XNode::Element { children, .. }) => children.get(index).cloned(),
                    _ => None,
                };
                match node {
                    Some(existing) => spec("insert-element", obj(vec![("path", Json::Array(path_json)), ("index", Json::Number(index as f64)), ("node", xnode_to_json(&existing))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-attribute" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let name = params.str("name");
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(XNode::Element { attrs, .. }) => attrs.iter().find(|(key, _)| key == &name).map(|(_, value)| value.clone()),
                    _ => None,
                };
                spec(
                    "set-attribute",
                    obj(vec![
                        ("path", Json::Array(path_json)),
                        ("name", Json::String(name)),
                        (
                            "value",
                            match prior {
                                Some(value) => Json::String(value),
                                None => Json::Null,
                            },
                        ),
                    ]),
                )
            }
            "set-text" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(XNode::Text(text)) => text.clone(),
                    _ => String::new(),
                };
                spec("set-text", obj(vec![("path", Json::Array(path_json)), ("text", Json::String(prior))]))
            }
            other => spec(other, params.clone()),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut doc = parse(input)?;
        apply_kind(&mut doc, kind, params)?;
        serialize(&doc)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
    /// re-serialized result — the caller compares its projection against the ORIGINAL input's own.
    /// 🐛️ The forward step and the undo share ONE parsed tree rather than passing serialized bytes
    /// between them. Routing the undo through `apply_mutation(&mutated, ...)` re-parsed the
    /// intermediate document, and XML parsing COALESCES adjacent character data, so removing an
    /// element that sat between two whitespace text nodes left an index space the undo could no
    /// longer address — the same defect the sibling SVG oracle's `inverse-remove-element` failed on
    /// against a pretty-printed real drawing. This fixture is minified and never showed it; the
    /// routing was wrong either way, and the subject applies both steps to one snapshot with no
    /// serialization between, so this is what the law actually claims.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let base = parse(input)?;
        let inverse = inverse_spec(&base, kind, params);
        let mut doc = base;
        apply_kind(&mut doc, kind, params)?;
        apply_kind(&mut doc, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))?;
        serialize(&doc)
    }

    //#region 🔖️Projection
    fn declaration_projection(declaration: &Option<XDecl>) -> Json {
        match declaration {
            None => Json::Null,
            Some(decl) => Json::Object(vec![
                ("version".to_string(), Json::String(decl.version.clone())),
                (
                    "encoding".to_string(),
                    match &decl.encoding {
                        Some(value) => Json::String(value.clone()),
                        None => Json::Null,
                    },
                ),
                (
                    "standalone".to_string(),
                    match decl.standalone {
                        Some(value) => Json::Bool(value),
                        None => Json::Null,
                    },
                ),
            ]),
        }
    }

    fn doctype_projection(doctype: &Option<XDoctype>) -> Json {
        match doctype {
            None => Json::Null,
            Some(_) => doctype_to_json(doctype),
        }
    }

    /// 👁️ `attrs` project as an unordered name/value MAP (`Json::Object`), not the ordered list the
    /// tree stores them as — the comparison mechanism's own `canonicalize()` sorts object keys before
    /// comparing, so attribute order becomes invisible here exactly where the fleet brief's "writer
    /// freedom" carve-out belongs, without needing a profile-level `ignoreKeys` entry for it.
    fn node_projection(node: &XNode) -> Json {
        match node {
            XNode::Element { name, attrs, children } => Json::Object(vec![
                ("kind".to_string(), Json::String("element".to_string())),
                ("name".to_string(), Json::String(name.clone())),
                ("attrs".to_string(), Json::Object(attrs.iter().map(|(key, value)| (key.clone(), Json::String(value.clone()))).collect())),
                ("children".to_string(), Json::Array(children.iter().map(node_projection).collect())),
            ]),
            XNode::Text(text) => Json::Object(vec![("kind".to_string(), Json::String("text".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            XNode::CData(text) => Json::Object(vec![("kind".to_string(), Json::String("cdata".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            XNode::Comment(text) => Json::Object(vec![("kind".to_string(), Json::String("comment".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            XNode::Pi { target, data } => Json::Object(vec![("kind".to_string(), Json::String("pi".to_string())), ("target".to_string(), Json::String(target.clone())), ("data".to_string(), Json::String(data.clone()))]),
        }
    }

    /// 👁️ This subset's own semantic projection — declaration, doctype, prolog and the full element
    /// tree in document order, independently re-derived by re-parsing `bytes` through `quick-xml`
    /// rather than trusting whatever produced them.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let doc = parse(bytes)?;
        Ok(Json::Object(vec![
            ("declaration".to_string(), declaration_projection(&doc.declaration)),
            ("doctype".to_string(), doctype_projection(&doc.doctype)),
            ("prolog".to_string(), Json::Array(doc.prolog.iter().map(node_projection).collect())),
            (
                "root".to_string(),
                match &doc.root {
                    Some(root) => node_projection(root),
                    None => Json::Null,
                },
            ),
        ]))
    }
    //#endregion 🔖️Projection
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
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation(input, &kind, &params)
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence, proving the
/// same `apply(inverse(m, base), apply(m, base)) == base` law `XmlMutation::inverse` proves at the
/// Rust-model level, here against the registered reference library instead.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    let params = spec.get("params").cloned().unwrap_or(Json::Null);
    oracles::apply_mutation_inverse(input, &kind, &params)
}

/// 👁️ This subset's own semantic projection. @see [`oracles::project`].
#[cfg(feature = "oracles")]
pub fn project_xml_1_0(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
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
pub fn project_xml_1_0(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
