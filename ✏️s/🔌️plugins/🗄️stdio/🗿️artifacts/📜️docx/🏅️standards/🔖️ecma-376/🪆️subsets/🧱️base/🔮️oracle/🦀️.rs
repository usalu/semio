//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed
//! independently of this repository's own codec so the subject has something real to be compared
//! against instead of being checked against its own reading.
//!
//! A DOCX is a ZIP of XML parts (ECMA-376 Part 2, Open Packaging Conventions), and this module
//! composes two genuinely independent third-party crates to read and write one for real: `zip`
//! unzips/rezips the OPC container, `quick-xml` parses/edits/re-serializes every OOXML part
//! (`word/document.xml`, `word/styles.xml`, `[Content_Types].xml`, every `*.rels`). This file never
//! imports `semio-s-plugin-stdio` — the OPC layer and the WordprocessingML block-tree layer are
//! reimplemented here, independently, against the two reference crates directly, mirroring the
//! shape `📰️xml/…/🔖️1.0/…/🦀️oracle.rs` (the quick-xml half) and
//! `🎒️zip/…/🔖️2.0/…/🦀️oracle.rs` (the zip half) each already establish on their own.
//!
//! The vocabulary is per SUBSET, not per artifact: `📕️xlsx`/`🎞️pptx` are also ECMA-376 ZIP+XML
//! packages but declare their own mutation vocabularies and own their own oracle modules —
//! untouched here.
//!
//! Two entry points: [`oracle_apply_mutation`] performs the FORWARD mutation (the `mutate-<kind>`
//! scenarios), [`oracle_apply_mutation_inverse`] performs the forward mutation and then its computed
//! inverse in sequence (the `inverse-<kind>` scenarios) — the same "apply, then apply the inverse,
//! land back on the start" law `DocxMutation::inverse` proves at the Rust-model level, proven here
//! independently against the two registered reference libraries. [`project_docx_ecma_376`] is the
//! shared independent-reader projection both this module's own handlers AND the case's subject
//! handlers read their results back through before comparison.
//!
//! `quick-xml` 0.42 splits every `&entity;`/`&#NNN;` reference out of `Text` into its own
//! `Event::GeneralRef`, so a text run is accumulated across `Text`/`GeneralRef` events rather than
//! read as one event — see [`oracles::resolve_general_ref`], same technique the XML 1.0 oracle uses.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`DocxMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Vocabulary
/// 🧾️ Kebab-case spelling of every variant this subset's `DocxMutation` declares, in declaration
/// order. The `docx-ecma-376-any` catalog is measured against this exact list, and the
/// production-side `kinds_const_matches_enum_variants_in_declaration_order` proves enum, constant
/// and manifest never drift apart. Declared here rather than in the case adapter so the adapter,
/// this module's own law tests and the manifest all read ONE list.
pub const KINDS: &[&str] = &["set-snapshot", "insert-block", "remove-block", "set-block-content", "set-run-text", "set-run-formatting", "insert-style", "remove-style", "set-style-name", "set-style-based-on", "set-part", "remove-part"];
//#endregion 🔖️Vocabulary

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use quick_xml::escape::resolve_xml_entity;
    use quick_xml::events::{BytesDecl, BytesEnd, BytesRef, BytesStart, BytesText, Event};
    use quick_xml::reader::Reader;
    use quick_xml::writer::Writer;
    use quick_xml::XmlVersion;
    use semio_repo_test_host::{digest, Json};
    use std::collections::HashMap;
    use std::io::{Cursor, Read, Write};

    //#region 🔖️GenericXmlTree
    /// 🌳 Owned XML node, used ONLY for the small typed OPC/WordprocessingML documents this oracle
    /// reads (`[Content_Types].xml`, `*.rels`, `word/document.xml`, `word/styles.xml`) — no CDATA,
    /// comment or processing-instruction support, since a real OOXML part never emits any of those
    /// (unlike `📰️xml`'s general-purpose oracle, which has to).
    #[derive(Clone, Debug, PartialEq)]
    enum XNode {
        Element { name: String, attrs: Vec<(String, String)>, children: Vec<XNode> },
        Text(String),
    }

    fn elem(name: &str, attrs: Vec<(&str, String)>, children: Vec<XNode>) -> XNode {
        XNode::Element { name: name.to_string(), attrs: attrs.into_iter().map(|(k, v)| (k.to_string(), v)).collect(), children }
    }

    fn find_attr<'a>(attrs: &'a [(String, String)], name: &str) -> Option<&'a str> {
        attrs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
    }

    /// 🔓️ Resolves one `Event::GeneralRef` (`&name;` or `&#NNN;`) to its literal text — numeric
    /// character references via `resolve_char_ref`, the five predefined XML entities via
    /// `resolve_xml_entity`, anything else a hard parse error. Same technique
    /// `📰️xml/…/🦀️oracle.rs`'s own `resolve_general_ref` uses, independently rewritten
    /// here (this file imports nothing from that module).
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
    /// into `Start` children. Comments/CDATA/PI are rejected — real OOXML parts never carry them.
    fn parse_element(reader: &mut Reader<&[u8]>, start: BytesStart) -> Result<XNode, String> {
        let name = start.name().as_ref().to_string();
        let attrs = read_attrs(&start)?;
        let mut children = Vec::new();
        let mut text_run = String::new();
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {} in <{name}>: {error}", reader.error_position()))?;
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
                Event::Eof => return Err(format!("unclosed element <{name}>: unexpected end of input")),
                other => return Err(format!("unexpected {other:?} inside <{name}> (OOXML parts carry no comments/CDATA/PI)")),
            }
        }
    }

    /// 📄 Parses a real OOXML/OPC part's bytes into its root `XNode`, skipping the leading
    /// declaration (this oracle re-emits its own on write; the declaration itself carries no
    /// semantics this subset's vocabulary ever mutates).
    fn parse_xml(bytes: &[u8]) -> Result<XNode, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        let mut reader = Reader::from_str(text);
        loop {
            let event = reader.read_event().map_err(|error| format!("quick-xml parse error at byte {}: {error}", reader.error_position()))?;
            match event {
                Event::Decl(_) => continue,
                // 🩹 Real OOXML parts carry `\r\n` (or plain whitespace) between the declaration and
                // the root element — only NON-whitespace text before the root is actually malformed.
                Event::Text(text) if text.as_ref().trim().is_empty() => continue,
                Event::Start(start) => return parse_element(&mut reader, start),
                Event::Empty(start) => return Ok(XNode::Element { name: start.name().as_ref().to_string(), attrs: read_attrs(&start)?, children: Vec::new() }),
                Event::Eof => return Err("document has no root element".to_string()),
                other => return Err(format!("unexpected {other:?} before the root element")),
            }
        }
    }

    fn write_node<W: Write>(writer: &mut Writer<W>, node: &XNode) -> Result<(), String> {
        match node {
            XNode::Text(text) => writer.write_event(Event::Text(BytesText::new(text))).map_err(|error| error.to_string()),
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

    /// 📝 Serializes `root` as a real, standalone-declared OOXML part — the same declaration form
    /// every real Word-produced part carries.
    fn write_xml(root: &XNode) -> Result<Vec<u8>, String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).map_err(|error| error.to_string())?;
        write_node(&mut writer, root)?;
        Ok(writer.into_inner().into_inner())
    }
    //#endregion 🔖️GenericXmlTree

    //#region 🔖️OpcTypes
    const CONTENT_TYPES_PART: &str = "[Content_Types].xml";
    const CONTENT_TYPES_NS: &str = "http://schemas.openxmlformats.org/package/2006/content-types";
    const RELATIONSHIPS_NS: &str = "http://schemas.openxmlformats.org/package/2006/relationships";
    const RELS_CONTENT_TYPE: &str = "application/vnd.openxmlformats-package.relationships+xml";
    const REL_TYPE_OFFICE_DOCUMENT: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
    const REL_TYPE_STYLES: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
    const W_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

    #[derive(Clone, Debug, Default)]
    struct OContentTypes {
        defaults: Vec<(String, String)>,
        overrides: Vec<(String, String)>,
    }

    impl OContentTypes {
        fn resolve(&self, part_path: &str) -> Option<&str> {
            let part_name = format!("/{}", part_path.trim_start_matches('/'));
            if let Some((_, ct)) = self.overrides.iter().find(|(p, _)| *p == part_name) {
                return Some(ct);
            }
            let ext = part_path.rsplit('.').next()?.to_ascii_lowercase();
            self.defaults.iter().find(|(e, _)| *e == ext).map(|(_, ct)| ct.as_str())
        }

        fn set_override(&mut self, part_path: &str, content_type: &str) {
            let part_name = format!("/{}", part_path.trim_start_matches('/'));
            match self.overrides.iter_mut().find(|(p, _)| *p == part_name) {
                Some(existing) => existing.1 = content_type.to_string(),
                None => self.overrides.push((part_name, content_type.to_string())),
            }
        }

        fn from_xnode(node: &XNode) -> Result<Self, String> {
            let XNode::Element { name, children, .. } = node else { return Err("[Content_Types].xml: root is not an element".to_string()) };
            if name != "Types" {
                return Err(format!("[Content_Types].xml: expected <Types>, got <{name}>"));
            }
            let mut out = OContentTypes::default();
            for child in children {
                let XNode::Element { name, attrs, .. } = child else { continue };
                match name.as_str() {
                    "Default" => {
                        let ext = find_attr(attrs, "Extension").ok_or("<Default> missing Extension")?;
                        let ct = find_attr(attrs, "ContentType").ok_or("<Default> missing ContentType")?;
                        out.defaults.push((ext.to_ascii_lowercase(), ct.to_string()));
                    }
                    "Override" => {
                        let part = find_attr(attrs, "PartName").ok_or("<Override> missing PartName")?;
                        let ct = find_attr(attrs, "ContentType").ok_or("<Override> missing ContentType")?;
                        out.overrides.push((part.to_string(), ct.to_string()));
                    }
                    _ => {}
                }
            }
            Ok(out)
        }

        fn to_xnode(&self) -> XNode {
            let mut children = Vec::with_capacity(self.defaults.len() + self.overrides.len());
            for (ext, ct) in &self.defaults {
                children.push(elem("Default", vec![("Extension", ext.clone()), ("ContentType", ct.clone())], vec![]));
            }
            for (part, ct) in &self.overrides {
                children.push(elem("Override", vec![("PartName", part.clone()), ("ContentType", ct.clone())], vec![]));
            }
            elem("Types", vec![("xmlns", CONTENT_TYPES_NS.to_string())], children)
        }
    }

    #[derive(Clone, Debug)]
    struct ORel {
        id: String,
        rel_type: String,
        target: String,
        external: bool,
    }

    fn relationships_from_xnode(node: &XNode) -> Result<Vec<ORel>, String> {
        let XNode::Element { children, .. } = node else { return Err("relationships part: root is not an element".to_string()) };
        let mut out = Vec::new();
        for child in children {
            let XNode::Element { name, attrs, .. } = child else { continue };
            if name != "Relationship" {
                continue;
            }
            out.push(ORel {
                id: find_attr(attrs, "Id").ok_or("<Relationship> missing Id")?.to_string(),
                rel_type: find_attr(attrs, "Type").ok_or("<Relationship> missing Type")?.to_string(),
                target: find_attr(attrs, "Target").ok_or("<Relationship> missing Target")?.to_string(),
                external: find_attr(attrs, "TargetMode") == Some("External"),
            });
        }
        Ok(out)
    }

    fn relationships_to_xnode(rels: &[ORel]) -> XNode {
        let children = rels
            .iter()
            .map(|r| {
                let mut attrs = vec![("Id", r.id.clone()), ("Type", r.rel_type.clone()), ("Target", r.target.clone())];
                if r.external {
                    attrs.push(("TargetMode", "External".to_string()));
                }
                elem("Relationship", attrs, vec![])
            })
            .collect();
        elem("Relationships", vec![("xmlns", RELATIONSHIPS_NS.to_string())], children)
    }

    /// 📍 The `*.rels` part path that carries `owner`'s relationships (`""` = package root ->
    /// `_rels/.rels`; `"word/document.xml"` -> `"word/_rels/document.xml.rels"`). Independent
    /// reimplementation of the same OPC §9 convention `crate::artifacts::zip::opc` encodes.
    fn rels_part_path_for(owner: &str) -> String {
        if owner.is_empty() {
            "_rels/.rels".to_string()
        } else if let Some(slash) = owner.rfind('/') {
            format!("{}/_rels/{}.rels", &owner[..slash], &owner[slash + 1..])
        } else {
            format!("_rels/{owner}.rels")
        }
    }

    fn owner_for_rels_path(path: &str) -> Option<String> {
        let file = path.rsplit('/').next()?;
        let name = file.strip_suffix(".rels")?;
        let dir = &path[..path.len() - file.len()];
        let dir = dir.strip_suffix("_rels/")?;
        let name = if name == "." { "" } else { name };
        Some(format!("{dir}{name}"))
    }

    /// 🧭️ Resolves a relationship `Target` against the directory of its owner part (OPC §9.3).
    fn resolve_relationship_target(owner: &str, target: &str) -> String {
        if let Some(stripped) = target.strip_prefix('/') {
            return stripped.to_string();
        }
        let base_dir = match owner.rfind('/') {
            Some(slash) => &owner[..=slash],
            None => "",
        };
        normalize_path(&format!("{base_dir}{target}"))
    }

    fn normalize_path(path: &str) -> String {
        let mut out: Vec<&str> = Vec::new();
        for seg in path.split('/') {
            match seg {
                "" | "." => {}
                ".." => {
                    out.pop();
                }
                other => out.push(other),
            }
        }
        out.join("/")
    }

    #[derive(Clone, Debug)]
    struct OPart {
        path: String,
        content_type: String,
        bytes: Vec<u8>,
    }

    #[derive(Clone, Debug, Default)]
    struct OPackage {
        parts: Vec<OPart>,
        content_types: OContentTypes,
        /// 🗺️ Owner part path (`""` = package root) -> that owner's relationships.
        relationships: HashMap<String, Vec<ORel>>,
    }

    impl OPackage {
        fn part(&self, path: &str) -> Option<&OPart> {
            let p = path.trim_start_matches('/');
            self.parts.iter().find(|part| part.path == p)
        }

        fn set_part(&mut self, path: &str, content_type: &str, bytes: Vec<u8>) {
            let p = path.trim_start_matches('/').to_string();
            self.content_types.set_override(&p, content_type);
            match self.parts.iter_mut().find(|part| part.path == p) {
                Some(existing) => {
                    existing.bytes = bytes;
                    existing.content_type = content_type.to_string();
                }
                None => self.parts.push(OPart { path: p, content_type: content_type.to_string(), bytes }),
            }
        }

        fn resolve_relationship(&self, owner: &str, rel_type: &str) -> Option<String> {
            let rel = self.relationships.get(owner)?.iter().find(|r| r.rel_type == rel_type)?;
            Some(resolve_relationship_target(owner, &rel.target))
        }
    }

    /// 📦 Unzips `bytes` with the registered `zip` reference reader and interprets it as an OPC
    /// package: `[Content_Types].xml` and every `*.rels` become the typed metadata tables, every
    /// other entry becomes a verbatim `OPart`.
    fn read_opc(bytes: &[u8]) -> Result<OPackage, String> {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes.to_vec())).map_err(|error| format!("independent zip reader could not parse the OPC container: {error}"))?;
        let mut entries: Vec<(String, Vec<u8>)> = Vec::with_capacity(archive.len());
        for index in 0..archive.len() {
            let mut member = archive.by_index(index).map_err(|error| format!("independent zip reader could not read entry {index}: {error}"))?;
            if member.is_dir() {
                continue;
            }
            let name = member.name().to_string();
            let mut data = Vec::new();
            member.read_to_end(&mut data).map_err(|error| format!("independent zip reader could not decompress {name}: {error}"))?;
            entries.push((name, data));
        }

        let (_, ct_bytes) = entries.iter().find(|(name, _)| name == CONTENT_TYPES_PART).ok_or("OPC package has no [Content_Types].xml")?;
        let content_types = OContentTypes::from_xnode(&parse_xml(ct_bytes)?)?;

        let mut parts = Vec::new();
        let mut relationships: HashMap<String, Vec<ORel>> = HashMap::new();
        for (name, data) in &entries {
            if name == CONTENT_TYPES_PART {
                continue;
            }
            if name.ends_with(".rels") {
                let rels = relationships_from_xnode(&parse_xml(data)?)?;
                let owner = owner_for_rels_path(name).ok_or_else(|| format!("relationship part at unexpected path: {name}"))?;
                relationships.insert(owner, rels);
                continue;
            }
            let content_type = content_types.resolve(name).ok_or_else(|| format!("part {name} has no resolvable content type"))?.to_string();
            parts.push(OPart { path: name.clone(), content_type, bytes: data.clone() });
        }
        Ok(OPackage { parts, content_types, relationships })
    }

    /// 📦 Re-encodes `pkg` as a real OPC container with the registered `zip` reference writer:
    /// `[Content_Types].xml` and every owner's `*.rels` are regenerated from the typed tables (never
    /// carried as stray verbatim parts), every content part is re-emitted deflated.
    fn write_opc(pkg: &OPackage) -> Result<Vec<u8>, String> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::<u8>::new()));
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        writer.start_file(CONTENT_TYPES_PART, options).map_err(|error| format!("zip start_file [Content_Types].xml: {error}"))?;
        writer.write_all(&write_xml(&pkg.content_types.to_xnode())?).map_err(|error| error.to_string())?;

        let mut owners: Vec<&String> = pkg.relationships.keys().collect();
        owners.sort();
        for owner in owners {
            let rels = &pkg.relationships[owner];
            if rels.is_empty() {
                continue;
            }
            let rels_path = rels_part_path_for(owner);
            writer.start_file(&rels_path, options).map_err(|error| format!("zip start_file {rels_path}: {error}"))?;
            writer.write_all(&write_xml(&relationships_to_xnode(rels))?).map_err(|error| error.to_string())?;
        }

        for part in &pkg.parts {
            writer.start_file(&part.path, options).map_err(|error| format!("zip start_file {}: {error}", part.path))?;
            writer.write_all(&part.bytes).map_err(|error| error.to_string())?;
        }

        let cursor = writer.finish().map_err(|error| format!("zip finish: {error}"))?;
        Ok(cursor.into_inner())
    }
    //#endregion 🔖️OpcTypes

    //#region 🔖️WordprocessingModel
    #[derive(Clone, Debug, Default, PartialEq)]
    struct WRun {
        text: String,
        bold: bool,
        italic: bool,
        underline: bool,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct WParagraph {
        runs: Vec<WRun>,
        style: Option<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct WCell {
        blocks: Vec<WBlock>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct WRow {
        cells: Vec<WCell>,
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct WTable {
        rows: Vec<WRow>,
    }

    #[derive(Clone, Debug, PartialEq)]
    enum WBlock {
        Paragraph(WParagraph),
        Table(WTable),
    }

    #[derive(Clone, Debug, PartialEq)]
    struct WStyle {
        id: String,
        name: String,
        based_on: Option<String>,
    }

    //#region 🔖️Parse
    fn run_from_xnode(node: &XNode) -> WRun {
        let mut run = WRun::default();
        let XNode::Element { children, .. } = node else { return run };
        for child in children {
            let XNode::Element { name, children: inner, .. } = child else { continue };
            match name.as_str() {
                "w:rPr" => {
                    for prop in inner {
                        let XNode::Element { name, .. } = prop else { continue };
                        match name.as_str() {
                            "w:b" => run.bold = true,
                            "w:i" => run.italic = true,
                            "w:u" => run.underline = true,
                            _ => {}
                        }
                    }
                }
                "w:t" => {
                    for t in inner {
                        if let XNode::Text(text) = t {
                            run.text.push_str(text);
                        }
                    }
                }
                _ => {}
            }
        }
        run
    }

    fn paragraph_from_xnode(node: &XNode) -> WParagraph {
        let mut paragraph = WParagraph::default();
        let XNode::Element { children, .. } = node else { return paragraph };
        for child in children {
            let XNode::Element { name, children: inner, .. } = child else { continue };
            match name.as_str() {
                "w:pPr" => {
                    for prop in inner {
                        let XNode::Element { name, attrs, .. } = prop else { continue };
                        if name == "w:pStyle" {
                            paragraph.style = find_attr(attrs, "w:val").map(str::to_string);
                        }
                    }
                }
                "w:r" => paragraph.runs.push(run_from_xnode(child)),
                _ => {}
            }
        }
        paragraph
    }

    fn cell_from_xnode(node: &XNode) -> WCell {
        let mut cell = WCell::default();
        let XNode::Element { children, .. } = node else { return cell };
        for child in children {
            let XNode::Element { name, .. } = child else { continue };
            match name.as_str() {
                "w:p" => cell.blocks.push(WBlock::Paragraph(paragraph_from_xnode(child))),
                "w:tbl" => cell.blocks.push(WBlock::Table(table_from_xnode(child))),
                _ => {}
            }
        }
        cell
    }

    fn row_from_xnode(node: &XNode) -> WRow {
        let mut row = WRow::default();
        let XNode::Element { children, .. } = node else { return row };
        for child in children {
            let XNode::Element { name, .. } = child else { continue };
            if name == "w:tc" {
                row.cells.push(cell_from_xnode(child));
            }
        }
        row
    }

    fn table_from_xnode(node: &XNode) -> WTable {
        let mut table = WTable::default();
        let XNode::Element { children, .. } = node else { return table };
        for child in children {
            let XNode::Element { name, .. } = child else { continue };
            if name == "w:tr" {
                table.rows.push(row_from_xnode(child));
            }
        }
        table
    }

    /// 📄 `word/document.xml`'s `<w:document><w:body>...</w:body></w:document>` into a block tree,
    /// independently re-derived by re-parsing the part's own bytes through `quick-xml` rather than
    /// trusting whatever produced them. Mirrors the exact WordprocessingML shape
    /// `crate::artifacts::docx::…::io::import::deserializers::document_from_xml` reads, reimplemented
    /// here against this file's own `XNode` (no shared code, no shared dependency).
    fn document_body_from_xnode(root: &XNode) -> Result<Vec<WBlock>, String> {
        let XNode::Element { name, children, .. } = root else { return Err("word/document.xml: root is not an element".to_string()) };
        if name != "w:document" {
            return Err(format!("word/document.xml: expected <w:document>, got <{name}>"));
        }
        let body = children
            .iter()
            .find_map(|c| match c {
                XNode::Element { name, children, .. } if name == "w:body" => Some(children),
                _ => None,
            })
            .ok_or("word/document.xml: missing <w:body>")?;
        let mut blocks = Vec::new();
        for node in body {
            let XNode::Element { name, .. } = node else { continue };
            match name.as_str() {
                "w:p" => blocks.push(WBlock::Paragraph(paragraph_from_xnode(node))),
                "w:tbl" => blocks.push(WBlock::Table(table_from_xnode(node))),
                _ => {}
            }
        }
        Ok(blocks)
    }

    fn styles_from_xnode(root: &XNode) -> Result<Vec<WStyle>, String> {
        let XNode::Element { name, children, .. } = root else { return Err("word/styles.xml: root is not an element".to_string()) };
        if name != "w:styles" {
            return Err(format!("word/styles.xml: expected <w:styles>, got <{name}>"));
        }
        let mut styles = Vec::new();
        for child in children {
            let XNode::Element { name, attrs, children: inner } = child else { continue };
            if name != "w:style" {
                continue;
            }
            let id = find_attr(attrs, "w:styleId").unwrap_or_default().to_string();
            let mut style_name = id.clone();
            let mut based_on = None;
            for prop in inner {
                let XNode::Element { name, attrs: pattrs, .. } = prop else { continue };
                match name.as_str() {
                    "w:name" => style_name = find_attr(pattrs, "w:val").unwrap_or(&style_name).to_string(),
                    "w:basedOn" => based_on = find_attr(pattrs, "w:val").map(str::to_string),
                    _ => {}
                }
            }
            styles.push(WStyle { id, name: style_name, based_on });
        }
        Ok(styles)
    }
    //#endregion 🔖️Parse

    //#region 🔖️Serialize
    fn run_to_xnode(run: &WRun) -> XNode {
        let mut children = Vec::new();
        if run.bold || run.italic || run.underline {
            let mut props = Vec::new();
            if run.bold {
                props.push(elem("w:b", vec![], vec![]));
            }
            if run.italic {
                props.push(elem("w:i", vec![], vec![]));
            }
            if run.underline {
                props.push(elem("w:u", vec![], vec![]));
            }
            children.push(elem("w:rPr", vec![], props));
        }
        children.push(elem("w:t", vec![("xml:space", "preserve".to_string())], vec![XNode::Text(run.text.clone())]));
        elem("w:r", vec![], children)
    }

    fn paragraph_to_xnode(paragraph: &WParagraph) -> XNode {
        let mut children = Vec::new();
        if let Some(style) = &paragraph.style {
            children.push(elem("w:pPr", vec![], vec![elem("w:pStyle", vec![("w:val", style.clone())], vec![])]));
        }
        children.extend(paragraph.runs.iter().map(run_to_xnode));
        elem("w:p", vec![], children)
    }

    fn cell_to_xnode(cell: &WCell) -> XNode {
        elem("w:tc", vec![], cell.blocks.iter().map(block_to_xnode).collect())
    }

    fn row_to_xnode(row: &WRow) -> XNode {
        elem("w:tr", vec![], row.cells.iter().map(cell_to_xnode).collect())
    }

    fn table_to_xnode(table: &WTable) -> XNode {
        elem("w:tbl", vec![], table.rows.iter().map(row_to_xnode).collect())
    }

    fn block_to_xnode(block: &WBlock) -> XNode {
        match block {
            WBlock::Paragraph(p) => paragraph_to_xnode(p),
            WBlock::Table(t) => table_to_xnode(t),
        }
    }

    fn document_to_xnode(body: &[WBlock]) -> XNode {
        elem("w:document", vec![("xmlns:w", W_NS.to_string())], vec![elem("w:body", vec![], body.iter().map(block_to_xnode).collect())])
    }

    fn styles_to_xnode(styles: &[WStyle]) -> XNode {
        let children = styles
            .iter()
            .map(|style| {
                let mut inner = vec![elem("w:name", vec![("w:val", style.name.clone())], vec![])];
                if let Some(based_on) = &style.based_on {
                    inner.push(elem("w:basedOn", vec![("w:val", based_on.clone())], vec![]));
                }
                elem("w:style", vec![("w:styleId", style.id.clone())], inner)
            })
            .collect();
        elem("w:styles", vec![("xmlns:w", W_NS.to_string())], children)
    }
    //#endregion 🔖️Serialize
    //#endregion 🔖️WordprocessingModel

    //#region 🔖️Package
    /// 📦 The full real DOCX this oracle mutates: the lossless OPC container plus the typed
    /// `document.xml`/`styles.xml` view, and the two parts' own paths (resolved once, through the
    /// package's OWN relationships, exactly as a real Word consumer resolves them — never assumed).
    struct WPackage {
        opc: OPackage,
        main_path: String,
        styles_path: Option<String>,
        body: Vec<WBlock>,
        styles: Vec<WStyle>,
    }

    fn read_package(bytes: &[u8]) -> Result<WPackage, String> {
        let opc = read_opc(bytes)?;
        let main_path = opc.resolve_relationship("", REL_TYPE_OFFICE_DOCUMENT).ok_or("OPC package has no root officeDocument relationship")?;
        let main_bytes = opc.part(&main_path).ok_or_else(|| format!("main document part {main_path} referenced by relationship but not present"))?.bytes.clone();
        let body = document_body_from_xnode(&parse_xml(&main_bytes)?)?;

        let styles_path = opc.resolve_relationship(&main_path, REL_TYPE_STYLES);
        let styles = match &styles_path {
            Some(path) => match opc.part(path) {
                Some(part) => styles_from_xnode(&parse_xml(&part.bytes)?)?,
                None => Vec::new(),
            },
            None => Vec::new(),
        };
        Ok(WPackage { opc, main_path, styles_path, body, styles })
    }

    /// 📦 Re-serializes `word/document.xml` and (when present) `word/styles.xml` from the typed
    /// model alone and re-encodes the OPC container — every other real part (docProps/*, any part a
    /// mutation added) survives verbatim.
    fn write_package(pkg: &WPackage) -> Result<Vec<u8>, String> {
        let mut opc = pkg.opc.clone();
        let main_content_type = opc.part(&pkg.main_path).map(|p| p.content_type.clone()).unwrap_or_else(|| "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml".to_string());
        opc.set_part(&pkg.main_path, &main_content_type, write_xml(&document_to_xnode(&pkg.body))?);
        if let Some(styles_path) = &pkg.styles_path {
            let styles_content_type = opc.part(styles_path).map(|p| p.content_type.clone()).unwrap_or_else(|| "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml".to_string());
            opc.set_part(styles_path, &styles_content_type, write_xml(&styles_to_xnode(&pkg.styles))?);
        }
        write_opc(&opc)
    }
    //#endregion 🔖️Package

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

    fn bool_field(value: &Json, key: &str) -> bool {
        matches!(value.get(key), Some(Json::Bool(true)))
    }

    fn non_empty(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    /// 🧭️ `{"segments":[{"blockIndex":..,"row":..,"cell":..}, ...], "index": N}` — the same
    /// segments-plus-index shape `DocxBlockPath` itself carries (production field names, camelCased
    /// for JSON), so a spec written for the oracle reads the same as one written for the subject.
    #[derive(Clone, Debug)]
    struct DPathSegment {
        block_index: usize,
        row: usize,
        cell: usize,
    }

    #[derive(Clone, Debug, Default)]
    struct DPath {
        segments: Vec<DPathSegment>,
        index: usize,
    }

    fn json_to_path(value: &Json) -> DPath {
        let segments = value.array("segments").iter().map(|s| DPathSegment { block_index: usize_field(s, "blockIndex"), row: usize_field(s, "row"), cell: usize_field(s, "cell") }).collect();
        DPath { segments, index: usize_field(value, "index") }
    }

    fn path_to_json(path: &DPath) -> Json {
        Json::Object(vec![
            (
                "segments".to_string(),
                Json::Array(
                    path.segments.iter().map(|s| Json::Object(vec![("blockIndex".to_string(), Json::Number(s.block_index as f64)), ("row".to_string(), Json::Number(s.row as f64)), ("cell".to_string(), Json::Number(s.cell as f64))])).collect(),
                ),
            ),
            ("index".to_string(), Json::Number(path.index as f64)),
        ])
    }

    fn json_to_run(value: &Json) -> WRun {
        WRun { text: value.str("text"), bold: bool_field(value, "bold"), italic: bool_field(value, "italic"), underline: bool_field(value, "underline") }
    }

    fn run_to_json(run: &WRun) -> Json {
        Json::Object(vec![("text".to_string(), Json::String(run.text.clone())), ("bold".to_string(), Json::Bool(run.bold)), ("italic".to_string(), Json::Bool(run.italic)), ("underline".to_string(), Json::Bool(run.underline))])
    }

    /// 🔎️ Owned block-spec JSON grammar mutation params speak: `{"kind":"paragraph","style":...,
    /// "runs":[{"text":...,"bold":...,"italic":...,"underline":...}]}` |
    /// `{"kind":"table","rows":[{"cells":[{"blocks":[...]}]}]}`.
    fn json_to_block(value: &Json) -> Result<WBlock, String> {
        match value.str("kind").as_str() {
            "paragraph" => Ok(WBlock::Paragraph(WParagraph { runs: value.array("runs").iter().map(json_to_run).collect(), style: non_empty(value, "style") })),
            "table" => Ok(WBlock::Table(WTable { rows: value.array("rows").iter().map(json_to_row).collect::<Result<_, _>>()? })),
            other => Err(format!("unknown block kind {other:?}")),
        }
    }

    fn json_to_row(value: &Json) -> Result<WRow, String> {
        Ok(WRow { cells: value.array("cells").iter().map(json_to_cell).collect::<Result<_, _>>()? })
    }

    fn json_to_cell(value: &Json) -> Result<WCell, String> {
        Ok(WCell { blocks: value.array("blocks").iter().map(json_to_block).collect::<Result<_, _>>()? })
    }

    fn block_to_json(block: &WBlock) -> Json {
        match block {
            WBlock::Paragraph(p) => Json::Object(vec![
                ("kind".to_string(), Json::String("paragraph".to_string())),
                (
                    "style".to_string(),
                    match &p.style {
                        Some(s) => Json::String(s.clone()),
                        None => Json::Null,
                    },
                ),
                ("runs".to_string(), Json::Array(p.runs.iter().map(run_to_json).collect())),
            ]),
            WBlock::Table(t) => Json::Object(vec![
                ("kind".to_string(), Json::String("table".to_string())),
                (
                    "rows".to_string(),
                    Json::Array(
                        t.rows
                            .iter()
                            .map(|row| Json::Object(vec![("cells".to_string(), Json::Array(row.cells.iter().map(|cell| Json::Object(vec![("blocks".to_string(), Json::Array(cell.blocks.iter().map(block_to_json).collect()))])).collect()))]))
                            .collect(),
                    ),
                ),
            ]),
        }
    }

    fn json_to_style(value: &Json) -> WStyle {
        WStyle { id: value.str("id"), name: value.str("name"), based_on: non_empty(value, "basedOn") }
    }

    fn style_to_json(style: &WStyle) -> Json {
        Json::Object(vec![
            ("id".to_string(), Json::String(style.id.clone())),
            ("name".to_string(), Json::String(style.name.clone())),
            (
                "basedOn".to_string(),
                match &style.based_on {
                    Some(v) => Json::String(v.clone()),
                    None => Json::Null,
                },
            ),
        ])
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️PathAddressing
    /// 🔎️ Resolves the block list `path`'s segments navigate to (the parent list `path.index` slots
    /// into), immutable form. Mirrors `crate::artifacts::docx::…::diff::resolve_blocks`'s exact
    /// `Table -> rows -> cells -> blocks` traversal, reimplemented independently.
    fn resolve_blocks<'a>(body: &'a [WBlock], segments: &[DPathSegment]) -> Option<&'a [WBlock]> {
        match segments.split_first() {
            None => Some(body),
            Some((seg, rest)) => {
                let WBlock::Table(table) = body.get(seg.block_index)? else { return None };
                let row = table.rows.get(seg.row)?;
                let cell = row.cells.get(seg.cell)?;
                resolve_blocks(&cell.blocks, rest)
            }
        }
    }

    fn resolve_blocks_mut<'a>(body: &'a mut Vec<WBlock>, segments: &[DPathSegment]) -> Option<&'a mut Vec<WBlock>> {
        match segments.split_first() {
            None => Some(body),
            Some((seg, rest)) => {
                let WBlock::Table(table) = body.get_mut(seg.block_index)? else { return None };
                let row = table.rows.get_mut(seg.row)?;
                let cell = row.cells.get_mut(seg.cell)?;
                resolve_blocks_mut(&mut cell.blocks, rest)
            }
        }
    }
    //#endregion 🔖️PathAddressing

    //#region 🔖️Forward
    /// 🦠️ Applies one declared mutation kind, described by `spec` (`{"kind": ..., "params": {...}}`),
    /// to an already-decoded package. An unrecognised kind is an error, never a silent no-op.
    fn apply_kind(pkg: &mut WPackage, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => {
                pkg.body = params.array("body").iter().map(json_to_block).collect::<Result<_, _>>()?;
                pkg.styles = params.array("styles").iter().map(|s| Ok::<_, String>(json_to_style(s))).collect::<Result<_, _>>()?;
            }
            "insert-block" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let block = json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))?;
                let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("insert-block: path does not resolve to a block list")?;
                let index = path.index.min(list.len());
                list.insert(index, block);
            }
            "remove-block" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("remove-block: path does not resolve to a block list")?;
                if path.index >= list.len() {
                    return Err(format!("remove-block: index {} out of range (len {})", path.index, list.len()));
                }
                list.remove(path.index);
            }
            "set-block-content" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let block = json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))?;
                let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("set-block-content: path does not resolve to a block list")?;
                let slot = list.get_mut(path.index).ok_or_else(|| format!("set-block-content: index {} out of range", path.index))?;
                *slot = block;
            }
            "set-run-text" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let run_index = usize_field(params, "runIndex");
                let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("set-run-text: path does not resolve to a block list")?;
                let WBlock::Paragraph(paragraph) = list.get_mut(path.index).ok_or("set-run-text: index out of range")? else { return Err("set-run-text: addressed block is not a paragraph".to_string()) };
                let run = paragraph.runs.get_mut(run_index).ok_or("set-run-text: run index out of range")?;
                run.text = params.str("text");
            }
            "set-run-formatting" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let run_index = usize_field(params, "runIndex");
                let list = resolve_blocks_mut(&mut pkg.body, &path.segments).ok_or("set-run-formatting: path does not resolve to a block list")?;
                let WBlock::Paragraph(paragraph) = list.get_mut(path.index).ok_or("set-run-formatting: index out of range")? else { return Err("set-run-formatting: addressed block is not a paragraph".to_string()) };
                let run = paragraph.runs.get_mut(run_index).ok_or("set-run-formatting: run index out of range")?;
                run.bold = bool_field(params, "bold");
                run.italic = bool_field(params, "italic");
                run.underline = bool_field(params, "underline");
            }
            "insert-style" => pkg.styles.push(json_to_style(&params.get("style").cloned().unwrap_or(Json::Null))),
            "remove-style" => {
                let id = params.str("id");
                pkg.styles.retain(|style| style.id != id);
            }
            "set-style-name" => {
                let id = params.str("id");
                let style = pkg.styles.iter_mut().find(|style| style.id == id).ok_or_else(|| format!("set-style-name: no style {id:?}"))?;
                style.name = params.str("name");
            }
            "set-style-based-on" => {
                let id = params.str("id");
                let style = pkg.styles.iter_mut().find(|style| style.id == id).ok_or_else(|| format!("set-style-based-on: no style {id:?}"))?;
                style.based_on = non_empty(params, "basedOn");
            }
            "set-part" => {
                let path = params.str("path");
                let content_type = params.str("contentType");
                pkg.opc.set_part(&path, &content_type, params.str("content").into_bytes());
            }
            "remove-part" => {
                let path = params.str("path").trim_start_matches('/').to_string();
                pkg.opc.parts.retain(|part| part.path != path);
            }
            other => return Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
        Ok(())
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Reads `base` (the CURRENT, pre-mutation package) to build the spec that undoes `{kind,
    /// params}` — same law `DocxMutation::inverse` proves at the Rust-model level, computed here
    /// against the reference libraries instead.
    fn inverse_spec(base: &WPackage, kind: &str, params: &Json) -> Result<Json, String> {
        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);
        let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        Ok(match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec("set-snapshot", obj(vec![("body", Json::Array(base.body.iter().map(block_to_json).collect())), ("styles", Json::Array(base.styles.iter().map(style_to_json).collect()))])),
            "insert-block" => {
                let path_json = params.get("path").cloned().unwrap_or(Json::Null);
                spec("remove-block", obj(vec![("path", path_json)]))
            }
            "remove-block" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let removed = resolve_blocks(&base.body, &path.segments).and_then(|list| list.get(path.index)).ok_or("inverse remove-block: original package has no block at this path")?;
                spec("insert-block", obj(vec![("path", path_to_json(&path)), ("block", block_to_json(removed))]))
            }
            "set-block-content" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let original = resolve_blocks(&base.body, &path.segments).and_then(|list| list.get(path.index)).ok_or("inverse set-block-content: original package has no block at this path")?;
                spec("set-block-content", obj(vec![("path", path_to_json(&path)), ("block", block_to_json(original))]))
            }
            "set-run-text" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let run_index = usize_field(params, "runIndex");
                let list = resolve_blocks(&base.body, &path.segments).ok_or("inverse set-run-text: original package has no block list at this path")?;
                let Some(WBlock::Paragraph(paragraph)) = list.get(path.index) else { return Err("inverse set-run-text: addressed block is not a paragraph".to_string()) };
                let run = paragraph.runs.get(run_index).ok_or("inverse set-run-text: run index out of range")?;
                spec("set-run-text", obj(vec![("path", path_to_json(&path)), ("runIndex", Json::Number(run_index as f64)), ("text", Json::String(run.text.clone()))]))
            }
            "set-run-formatting" => {
                let path = json_to_path(&params.get("path").cloned().unwrap_or(Json::Null));
                let run_index = usize_field(params, "runIndex");
                let list = resolve_blocks(&base.body, &path.segments).ok_or("inverse set-run-formatting: original package has no block list at this path")?;
                let Some(WBlock::Paragraph(paragraph)) = list.get(path.index) else { return Err("inverse set-run-formatting: addressed block is not a paragraph".to_string()) };
                let run = paragraph.runs.get(run_index).ok_or("inverse set-run-formatting: run index out of range")?;
                spec("set-run-formatting", obj(vec![("path", path_to_json(&path)), ("runIndex", Json::Number(run_index as f64)), ("bold", Json::Bool(run.bold)), ("italic", Json::Bool(run.italic)), ("underline", Json::Bool(run.underline))]))
            }
            "insert-style" => {
                let style = params.get("style").cloned().unwrap_or(Json::Null);
                spec("remove-style", obj(vec![("id", Json::String(style.str("id")))]))
            }
            // ↩️ `remove-style` is invertible only for the LAST style, and that is a property of the
            // vocabulary rather than of this oracle: `DocxMutation::InsertStyle` carries only a
            // `style` and appends (`../🧬️schema/🧬️mutations/🦀️.rs:181` → `diff_insert_style`),
            // so no declared kind can put a style back at an INTERIOR position. Removing `Title`
            // from this fixture's `[Normal, Title, Heading1, Heading2, Heading3, Code, TableCell]`
            // and appending it again leaves `Heading1` where `Title` was — which is exactly what the
            // case's inverse law caught. Refusing outright is the honest answer; returning an undo
            // that does not undo is not. The production `inverse()` at that file's line 227 has the
            // same gap and answers `InsertStyle` regardless.
            "remove-style" => {
                let id = params.str("id");
                match base.styles.iter().position(|style| style.id == id) {
                    Some(index) if index + 1 == base.styles.len() => spec("insert-style", obj(vec![("style", style_to_json(&base.styles[index]))])),
                    Some(index) => {
                        return Err(format!(
                            "remove-style at index {index} of {} has no inverse in this vocabulary: insert-style carries only a style and appends, so no declared kind can put {id:?} back where it was",
                            base.styles.len()
                        ))
                    }
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-style-name" => {
                let id = params.str("id");
                let style = base.styles.iter().find(|style| style.id == id).ok_or_else(|| format!("inverse set-style-name: no style {id:?}"))?;
                spec("set-style-name", obj(vec![("id", Json::String(id)), ("name", Json::String(style.name.clone()))]))
            }
            "set-style-based-on" => {
                let id = params.str("id");
                let style = base.styles.iter().find(|style| style.id == id).ok_or_else(|| format!("inverse set-style-based-on: no style {id:?}"))?;
                spec(
                    "set-style-based-on",
                    obj(vec![
                        ("id", Json::String(id)),
                        (
                            "basedOn",
                            match &style.based_on {
                                Some(v) => Json::String(v.clone()),
                                None => Json::Null,
                            },
                        ),
                    ]),
                )
            }
            "set-part" => {
                let path = params.str("path");
                match base.opc.part(&path) {
                    Some(part) => spec("set-part", obj(vec![("path", Json::String(path)), ("contentType", Json::String(part.content_type.clone())), ("content", Json::String(String::from_utf8_lossy(&part.bytes).into_owned()))])),
                    None => spec("remove-part", obj(vec![("path", Json::String(path))])),
                }
            }
            "remove-part" => {
                let path = params.str("path");
                match base.opc.part(&path) {
                    Some(part) => spec("set-part", obj(vec![("path", Json::String(path)), ("contentType", Json::String(part.content_type.clone())), ("content", Json::String(String::from_utf8_lossy(&part.bytes).into_owned()))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            other => return Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        })
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut pkg = read_package(input)?;
        apply_kind(&mut pkg, kind, params)?;
        write_package(&pkg)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
    /// re-serialized result — the caller compares its projection against the ORIGINAL input's own.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let base = read_package(input)?;
        let inverse = inverse_spec(&base, kind, params)?;
        let mutated = apply_mutation(input, kind, params)?;
        apply_mutation(&mutated, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))
    }

    //#region 🔖️Projection
    /// 👁️ This subset's own semantic projection: `body`/`styles` — the ordered, document-order-
    /// sensitive WordprocessingML block tree and style list, independently re-derived by re-parsing
    /// `word/document.xml`/`word/styles.xml` through `quick-xml` — plus `parts`, every OTHER real OPC
    /// part (docProps/*, anything `set-part`/`remove-part` touched) projected as an UNORDERED
    /// path-keyed map of `{contentType, digest}` so writer-freedom part order never registers as a
    /// difference. `word/document.xml`/`word/styles.xml` themselves are excluded from `parts` since
    /// two independent writers legitimately differ byte-for-byte on non-semantic form for the exact
    /// same document — that's exactly why `body`/`styles` exist as the typed comparison instead.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let pkg = read_package(bytes)?;
        let mut part_entries: Vec<(String, Json)> = pkg
            .opc
            .parts
            .iter()
            .filter(|part| part.path != pkg.main_path && Some(&part.path) != pkg.styles_path.as_ref())
            .map(|part| (part.path.clone(), Json::Object(vec![("contentType".to_string(), Json::String(part.content_type.clone())), ("digest".to_string(), Json::String(digest(&part.bytes)))])))
            .collect();
        part_entries.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(Json::Object(vec![("body".to_string(), Json::Array(pkg.body.iter().map(block_to_json).collect())), ("styles".to_string(), Json::Array(pkg.styles.iter().map(style_to_json).collect())), ("parts".to_string(), Json::Object(part_entries))]))
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
/// same `apply(inverse(m, base), apply(m, base)) == base` law `DocxMutation::inverse` proves at the
/// Rust-model level, here against the registered reference libraries instead.
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
pub fn project_docx_ecma_376(bytes: &[u8]) -> Result<Json, String> {
    oracles::project(bytes)
}

/// 🚫️ Without the `oracles` feature the reference implementations are not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_docx_ecma_376(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🧪️Tests
#[cfg(all(test, feature = "oracles"))]
mod tests {
    use super::*;
    use crate::law::feature_rows;

    /// 🧫️ The real committed package `📜️mutate-docx-ecma-376` runs on — the WordprocessingML document
    /// derived once from this repository's own `README.md`: 414 top-level body blocks, a real 37-row
    /// `w:tbl`, seven declared styles and seven OPC parts.
    const FIXTURE: &[u8] = include_bytes!("../🧫️fixtures/📜️example-readme.docx");

    /// 🧾️ The case's own `Examples` rows, read rather than restated — see [`crate::law::feature_rows`].
    const FEATURE: &str = include_str!("../🧪️tests/📜️mutate-docx-ecma-376/🥒️.feature");

    fn spec(kind: &str, params: &Json) -> Json {
        Json::Object(vec![("kind".to_string(), Json::String(kind.to_string())), ("params".to_string(), params.clone())])
    }

    /// ⚖️ The two laws `📜️mutate-docx-ecma-376`'s adapter asserts in role, proven here against the
    /// real package without the runner: every declared kind moves the projection it is compared
    /// through, and every declared kind's own computed inverse lands back on the untouched
    /// package's projection. Nothing is exempt from either — a DOCX carries its whole typed view in
    /// `word/document.xml` and `word/styles.xml`, and the OPC parts the typed view does not model
    /// are projected by content-type and digest, so all thirteen kinds reach the surface.
    #[test]
    fn every_declared_kind_is_observable_and_its_inverse_restores_the_document() {
        let base = project_docx_ecma_376(FIXTURE).expect("the independent reader projects the real package");
        let rows = feature_rows(FEATURE);
        assert_eq!(rows.len(), KINDS.len(), "the feature must carry exactly one Examples row per declared kind");
        for (kind, params) in &rows {
            assert!(KINDS.contains(&kind.as_str()), "the feature exercises {kind:?}, which the docx-ecma-376-any catalog does not declare");
            let forward = spec(kind, params);
            let mutated = oracle_apply_mutation(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: {error}"));
            let moved = project_docx_ecma_376(&mutated).unwrap_or_else(|error| panic!("{kind}: projecting the result failed: {error}"));
            if kind != "no-mutation" {
                assert_ne!(moved, base, "{kind} left the compared projection untouched, so its scenario would pass whether or not the mutation ran");
            }
            let restored = oracle_apply_mutation_inverse(FIXTURE, &forward).unwrap_or_else(|error| panic!("{kind}: inverse: {error}"));
            assert_eq!(project_docx_ecma_376(&restored).unwrap(), base, "{kind}: applying the mutation and then its own inverse must restore the package's projection");
        }
    }

    /// 🚫️ The one inverse this vocabulary genuinely cannot express, refused rather than faked.
    /// `DocxMutation::InsertStyle` carries a style and APPENDS, so removing an INTERIOR style can
    /// never be undone — the oracle rejects the request outright instead of returning an undo that
    /// leaves `Heading1` where `Title` was. The Examples row removes the LAST style, which append
    /// genuinely restores.
    #[test]
    fn removing_an_interior_style_is_refused_because_append_cannot_put_it_back() {
        let interior = spec("remove-style", &Json::Object(vec![("id".to_string(), Json::String("Title".to_string()))]));
        assert!(oracle_apply_mutation_inverse(FIXTURE, &interior).is_err(), "Title is the second of seven declared styles; no declared kind can reinsert it there");
        let last = spec("remove-style", &Json::Object(vec![("id".to_string(), Json::String("TableCell".to_string()))]));
        assert!(oracle_apply_mutation_inverse(FIXTURE, &last).is_ok(), "TableCell is the last declared style, so append restores it exactly");
    }

    /// 🔒️ Both halves of the identity law, on the real package.
    #[test]
    fn the_round_trip_is_projection_stable_and_not_a_byte_passthrough() {
        let rebuilt = oracle_apply_mutation(FIXTURE, &spec("no-mutation", &Json::Object(Vec::new()))).expect("the reference re-serializes the package");
        assert_ne!(rebuilt.as_slice(), FIXTURE, "zip+quick-xml rebuild the archive and every part from their own trees; identical bytes would mean the input was smuggled");
        assert_eq!(project_docx_ecma_376(&rebuilt).unwrap(), project_docx_ecma_376(FIXTURE).unwrap());
    }

    #[test]
    fn unknown_kind_is_an_error_never_a_silent_no_op() {
        let unknown = spec("not-a-real-kind", &Json::Object(Vec::new()));
        assert!(oracle_apply_mutation(FIXTURE, &unknown).is_err());
        assert!(oracle_apply_mutation_inverse(FIXTURE, &unknown).is_err());
        assert!(oracle_apply_mutation(FIXTURE, &Json::Object(vec![("params".to_string(), Json::Object(Vec::new()))])).is_err(), "a spec with no kind at all is an error too");
    }

    /// 📇️ [`KINDS`] against the catalog that declares it.
    #[test]
    fn kinds_matches_the_catalog() {
        let manifest = include_str!("🔣️.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "the docx-ecma-376-any catalog is missing {kind:?}");
        }
        assert_eq!(KINDS.len(), 12, "DocxMutation declares twelve kinds");
    }
}
//#endregion 🧪️Tests
