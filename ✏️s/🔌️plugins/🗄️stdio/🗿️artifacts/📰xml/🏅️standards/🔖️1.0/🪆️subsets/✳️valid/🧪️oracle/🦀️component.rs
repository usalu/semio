//! 🔮️ Mutation oracle for `stdio.xml` 1.0/✳️valid — every mutation kind THIS subset declares,
//! performed by the registered `quick-xml` reference implementation so the subject's own mutation
//! has an independent result to be compared against instead of being checked against its own
//! reading.
//!
//! The vocabulary is per SUBSET. `✳️valid` is not a longer `✳️any`: W3C XML 1.0 Fifth Edition §2.8
//! makes a document *valid* only if it carries a document type declaration whose Name is the
//! document element's name, so the operations that make sense inside the subset are the
//! subset-closed ones — a DOCTYPE declaration that takes no Name (it derives it), a root rename that
//! retags the DOCTYPE in the same step, the two halves of §2.9's standalone/external-subset pair,
//! and positional edits of the §4.2 internal subset. `✳️any`'s `set-doctype` has no counterpart
//! here, because a document that admits `SetDoctype { doctype: None }` is not a valid document.
//!
//! The reader/writer machinery (the quick-xml event tree, the address helpers, the serializer) is
//! shared with the SVG profile oracles through the `📰markup` family module rather than copied —
//! `MarkupDoc` keeps the DOCTYPE as the RAW internal-subset string, which is exactly the shape this
//! subset's own validity rules read. What is NOT shared is everything below: the DOCTYPE grammar
//! (`parse_doctype`/`render_doctype`), the four §2.8/§2.9 verdicts, and the projection. Those are
//! written here directly from the W3C text, independently of this repository's own
//! `check_valid_conformance` and of its `📸️snapshot` DOCTYPE parser, which is the point of an oracle.
//!
//! ⚠️ `parse_doctype` below accepts a strictly larger internal subset than the subject's schema
//! does: it keeps `<!ELEMENT>`/`<!ATTLIST>`/`<!NOTATION>` declarations as opaque raw markup instead
//! of failing, where `📸️snapshot/🦀️component.rs` rejects them outright ("only typed ENTITY
//! declarations are modeled"). That difference is deliberate and is reported in the projection as
//! `doctype.opaqueDeclarations`, so a document carrying real content-model markup is visibly out of
//! the subject's reach rather than silently equal to one without it.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the vocabulary itself (`XmlValidMutation::KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Live
#[cfg(feature = "oracles")]
mod live {
    use crate::markup::live::{json_to_path, member, node_at, node_at_mut, obj, parse_markup, usize_member, write_markup, MarkupDoc, MarkupNode};
    use semio_repo_test_host::Json;

    //#region 🔖️DoctypeGrammar
    /// 🔗️ An XML 1.0 §4.2.2 external identifier.
    #[derive(Clone, Debug, PartialEq)]
    pub enum ExternalId {
        System { system_id: String },
        Public { public_id: String, system_id: String },
    }

    /// 🏷️ One §4.2 internal-subset declaration. `Entity` is the typed shape both sides model;
    /// `Opaque` is every other declaration form, kept verbatim so it is never lost silently.
    #[derive(Clone, Debug, PartialEq)]
    pub enum Declaration {
        Entity { parameter: bool, name: String, value: String },
        Opaque { raw: String },
    }

    /// 📜️ The document type declaration, decomposed out of `MarkupDoc`'s raw string.
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct Doctype {
        pub name: String,
        pub external_id: Option<ExternalId>,
        pub declarations: Vec<Declaration>,
    }

    /// 📥️ Splits `Name (SYSTEM sysid | PUBLIC pubid sysid)? ('[' markupdecl* ']')?` — everything
    /// `quick-xml` hands back between `<!DOCTYPE` and its matching `>` — into the typed shape.
    /// Hand-rolled from the W3C grammar; this crate never links the subject's own parser.
    pub fn parse_doctype(raw: &str) -> Result<Doctype, String> {
        let mut rest = raw.trim();
        let split = rest.find(|c: char| c.is_whitespace() || c == '[').unwrap_or(rest.len());
        let name = rest[..split].to_string();
        if name.is_empty() {
            return Err("doctype: XML 1.0 §2.8 requires a Name after <!DOCTYPE".to_string());
        }
        rest = rest[split..].trim_start();
        let external_id = if let Some(tail) = rest.strip_prefix("SYSTEM") {
            let (system_id, tail) = take_literal(tail)?;
            rest = tail.trim_start();
            Some(ExternalId::System { system_id })
        } else if let Some(tail) = rest.strip_prefix("PUBLIC") {
            let (public_id, tail) = take_literal(tail)?;
            let (system_id, tail) = take_literal(tail)?;
            rest = tail.trim_start();
            Some(ExternalId::Public { public_id, system_id })
        } else {
            None
        };
        let declarations = match rest.strip_prefix('[') {
            None if rest.is_empty() => Vec::new(),
            None => return Err(format!("doctype: unexpected trailing text {rest:?} after the external identifier")),
            Some(inner) => parse_internal_subset(inner.rsplit_once(']').ok_or("doctype: unterminated internal subset")?.0)?,
        };
        Ok(Doctype { name, external_id, declarations })
    }

    fn take_literal(source: &str) -> Result<(String, &str), String> {
        let rest = source.trim_start();
        let quote = rest.chars().next().ok_or("doctype: expected a quoted literal")?;
        if quote != '"' && quote != '\'' {
            return Err(format!("doctype: literal must be quoted, found {rest:?}"));
        }
        let body = &rest[quote.len_utf8()..];
        let end = body.find(quote).ok_or("doctype: unterminated quoted literal")?;
        Ok((body[..end].to_string(), &body[end + quote.len_utf8()..]))
    }

    fn parse_internal_subset(source: &str) -> Result<Vec<Declaration>, String> {
        let mut out = Vec::new();
        let mut rest = source.trim();
        while !rest.is_empty() {
            if !rest.starts_with("<!") {
                return Err(format!("doctype: internal subset carries non-markup text {rest:?}"));
            }
            let end = rest.find('>').ok_or("doctype: unterminated internal-subset declaration")?;
            let (declaration, tail) = rest.split_at(end + 1);
            out.push(parse_declaration(declaration)?);
            rest = tail.trim();
        }
        Ok(out)
    }

    fn parse_declaration(raw: &str) -> Result<Declaration, String> {
        let Some(body) = raw.strip_prefix("<!ENTITY") else {
            return Ok(Declaration::Opaque { raw: raw.to_string() });
        };
        let mut rest = body.trim_start();
        let parameter = rest.starts_with('%');
        if parameter {
            rest = rest[1..].trim_start();
        }
        let split = rest.find(|c: char| c.is_whitespace()).ok_or("doctype: entity declaration has no value")?;
        let name = rest[..split].to_string();
        let (value, tail) = take_literal(&rest[split..])?;
        if tail.trim() != ">" {
            return Err(format!("doctype: unsupported entity declaration form, trailing {:?}", tail.trim()));
        }
        Ok(Declaration::Entity { parameter, name, value })
    }

    /// 📤️ Renders the typed shape back into the raw string `write_markup` puts between
    /// `<!DOCTYPE` and `>`.
    pub fn render_doctype(doctype: &Doctype) -> String {
        let mut out = doctype.name.clone();
        match &doctype.external_id {
            None => {}
            Some(ExternalId::System { system_id }) => out.push_str(&format!(" SYSTEM \"{system_id}\"")),
            Some(ExternalId::Public { public_id, system_id }) => out.push_str(&format!(" PUBLIC \"{public_id}\" \"{system_id}\"")),
        }
        if !doctype.declarations.is_empty() {
            out.push_str(" [");
            for declaration in &doctype.declarations {
                match declaration {
                    Declaration::Entity { parameter, name, value } => out.push_str(&format!("<!ENTITY {}{name} \"{value}\">", if *parameter { "% " } else { "" })),
                    Declaration::Opaque { raw } => out.push_str(raw),
                }
            }
            out.push(']');
        }
        out
    }
    //#endregion 🔖️DoctypeGrammar

    //#region 🔖️Conformance
    /// 🌳️ The document element's tag name, when the document has one.
    pub fn document_element_name(doc: &MarkupDoc) -> Option<&str> {
        match &doc.root {
            Some(MarkupNode::Element { name, .. }) => Some(name.as_str()),
            _ => None,
        }
    }

    /// 🛡️ The §2.8/§2.9 verdicts, read straight off the document. Transcribed from the W3C text
    /// independently of this repository's own `check_valid_conformance`; both encode the same
    /// normative rules, neither is derived from the other, and the projection carries all four so a
    /// mutation that moves one is visible rather than merely applied.
    pub fn verdicts(doc: &MarkupDoc) -> Result<Json, String> {
        let doctype = doc.doctype.as_deref().map(parse_doctype).transpose()?;
        let root = document_element_name(doc);
        Ok(obj(vec![
            ("doctypePresent", Json::Bool(doctype.is_some())),
            ("doctypeNameMatchesDocumentElement", Json::Bool(matches!((&doctype, root), (Some(declared), Some(actual)) if declared.name == actual))),
            (
                "standaloneBesideExternalSubset",
                Json::Bool(doc.declaration.as_ref().and_then(|declaration| declaration.standalone) == Some(true) && doctype.as_ref().is_some_and(|declared| declared.external_id.is_some())),
            ),
            ("internalSubsetFullyTyped", Json::Bool(doctype.as_ref().is_none_or(|declared| declared.declarations.iter().all(|entry| matches!(entry, Declaration::Entity { .. }))))),
        ]))
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Forward
    fn doctype_of(doc: &MarkupDoc, kind: &str) -> Result<Doctype, String> {
        doc.doctype.as_deref().map(parse_doctype).transpose()?.ok_or_else(|| format!("{kind}: the document has no DOCTYPE"))
    }

    fn json_to_external_id(value: &Json) -> Option<ExternalId> {
        match value.str("kind").as_str() {
            "system" => Some(ExternalId::System { system_id: value.str("systemId") }),
            "public" => Some(ExternalId::Public { public_id: value.str("publicId"), system_id: value.str("systemId") }),
            _ => None,
        }
    }

    fn json_to_declarations(value: &Json) -> Vec<Declaration> {
        value
            .array("declarations")
            .iter()
            .map(|entry| match entry.get("raw") {
                Some(Json::String(raw)) => Declaration::Opaque { raw: raw.clone() },
                _ => Declaration::Entity { parameter: matches!(entry.get("parameter"), Some(Json::Bool(true))), name: entry.str("name"), value: entry.str("value") },
            })
            .collect()
    }

    fn set_doctype(doc: &mut MarkupDoc, doctype: Doctype) {
        doc.doctype = Some(render_doctype(&doctype));
    }

    /// 🦠️ Applies one declared kind to `doc` in place. An unrecognised kind is an error, never a
    /// silent no-op: a quietly skipped mutation reports as a passing test.
    pub fn apply(doc: &mut MarkupDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => Ok(()),
            "set-snapshot" => {
                let replacement = parse_markup(params.str("xml").as_bytes())?;
                let verdict = verdicts(&replacement)?;
                if !matches!(verdict.get("doctypePresent"), Some(Json::Bool(true))) || !matches!(verdict.get("doctypeNameMatchesDocumentElement"), Some(Json::Bool(true))) {
                    return Err("set-snapshot: the replacement document is not XML 1.0 valid — §2.8 requires a DOCTYPE whose Name is the document element's name".to_string());
                }
                *doc = replacement;
                Ok(())
            }
            "declare-doctype" => {
                let name = document_element_name(doc).ok_or("declare-doctype: the document has no document element, so §2.8 gives the DOCTYPE no Name to carry")?.to_string();
                let declarations = doc.doctype.as_deref().map(parse_doctype).transpose()?.map(|declared| declared.declarations).unwrap_or_default();
                set_doctype(doc, Doctype { name, external_id: json_to_external_id(&member(params, "externalId")), declarations });
                Ok(())
            }
            "rename-document-element" => {
                let mut doctype = doctype_of(doc, "rename-document-element")?;
                let next = params.str("name");
                if next.is_empty() {
                    return Err("rename-document-element: the new name is empty".to_string());
                }
                match doc.root.as_mut() {
                    Some(MarkupNode::Element { name, .. }) => *name = next.clone(),
                    _ => return Err("rename-document-element: the document has no document element to rename".to_string()),
                }
                doctype.name = next;
                set_doctype(doc, doctype);
                Ok(())
            }
            "set-external-subset" => {
                let mut doctype = doctype_of(doc, "set-external-subset")?;
                doctype.external_id = json_to_external_id(&member(params, "externalId"));
                set_doctype(doc, doctype);
                Ok(())
            }
            "set-standalone" => {
                let standalone = match params.get("standalone") {
                    Some(Json::Bool(value)) => Some(*value),
                    _ => None,
                };
                doc.declaration = match (&doc.declaration, standalone) {
                    (None, None) => None,
                    (None, Some(value)) => Some(crate::markup::live::MarkupDecl { version: "1.0".to_string(), encoding: None, standalone: Some(value) }),
                    (Some(declaration), value) => Some(crate::markup::live::MarkupDecl { version: declaration.version.clone(), encoding: declaration.encoding.clone(), standalone: value }),
                };
                Ok(())
            }
            "declare-entity" => {
                let mut doctype = doctype_of(doc, "declare-entity")?;
                let name = params.str("name");
                if doctype.declarations.iter().any(|entry| matches!(entry, Declaration::Entity { name: declared, .. } if declared == &name)) {
                    return Err(format!("declare-entity: '{name}' is already declared — XML 1.0 §4.2 binds the FIRST declaration, so a second one is dead markup"));
                }
                let at = usize_member(params, "index").min(doctype.declarations.len());
                doctype.declarations.insert(at, Declaration::Entity { parameter: matches!(params.get("parameter"), Some(Json::Bool(true))), name, value: params.str("value") });
                set_doctype(doc, doctype);
                Ok(())
            }
            "set-internal-subset" => {
                let mut doctype = doctype_of(doc, "set-internal-subset")?;
                doctype.declarations = json_to_declarations(params);
                set_doctype(doc, doctype);
                Ok(())
            }
            "set-text" => match node_at_mut(doc, &json_to_path(&member(params, "path")))? {
                MarkupNode::Text(text) => {
                    *text = params.str("text");
                    Ok(())
                }
                _ => Err("set-text: target is not a text node".into()),
            },
            other => Err(format!("mutation kind {other:?} has no oracle implementation")),
        }
    }
    //#endregion 🔖️Forward

    //#region 🔖️Inverse
    /// ↩️ Computes and applies this kind's OWN inverse against the already-mutated document,
    /// sourcing whatever the forward mutation discarded from `base` — the document as it stood
    /// before the forward mutation ran. Mirrors the algebra `../🧬️schema/🧬️mutations/🦀️component.rs`
    /// defines for the subject, computed independently here so the property has two producers to
    /// disagree.
    ///
    /// Both internal-subset kinds invert to the prior declaration LIST rather than to a name-keyed
    /// undo: §4.2 binds the first declaration of a name, so position is semantic and restoring only
    /// the name would silently reorder the subset.
    pub fn invert(base: &MarkupDoc, mut mutated: MarkupDoc, kind: &str, params: &Json) -> Result<MarkupDoc, String> {
        match kind {
            "no-mutation" => Ok(mutated),
            "set-snapshot" => Ok(base.clone()),
            "declare-doctype" => match base.doctype.as_deref().map(parse_doctype).transpose()? {
                Some(prior) => {
                    let name = document_element_name(&mutated).ok_or("inverse declare-doctype: the document has no document element")?.to_string();
                    set_doctype(&mut mutated, Doctype { name, external_id: prior.external_id, declarations: prior.declarations });
                    Ok(mutated)
                }
                None => Ok(base.clone()),
            },
            "rename-document-element" => {
                let prior = document_element_name(base).ok_or("inverse rename-document-element: the original document has no document element")?.to_string();
                let mut doctype = doctype_of(&mutated, "inverse rename-document-element")?;
                match mutated.root.as_mut() {
                    Some(MarkupNode::Element { name, .. }) => *name = prior.clone(),
                    _ => return Err("inverse rename-document-element: the mutated document has no document element".into()),
                }
                doctype.name = prior;
                set_doctype(&mut mutated, doctype);
                Ok(mutated)
            }
            "set-external-subset" => {
                let prior = base.doctype.as_deref().map(parse_doctype).transpose()?.and_then(|declared| declared.external_id);
                let mut doctype = doctype_of(&mutated, "inverse set-external-subset")?;
                doctype.external_id = prior;
                set_doctype(&mut mutated, doctype);
                Ok(mutated)
            }
            "set-standalone" => {
                mutated.declaration = base.declaration.clone();
                Ok(mutated)
            }
            "declare-entity" | "set-internal-subset" => {
                let prior = base.doctype.as_deref().map(parse_doctype).transpose()?.map(|declared| declared.declarations).unwrap_or_default();
                let mut doctype = doctype_of(&mutated, "inverse set-internal-subset")?;
                doctype.declarations = prior;
                set_doctype(&mut mutated, doctype);
                Ok(mutated)
            }
            "set-text" => {
                let path = json_to_path(&member(params, "path"));
                let prior = match node_at(base, &path)? {
                    MarkupNode::Text(text) => text.clone(),
                    _ => return Err("inverse set-text: the original target is not a text node".into()),
                };
                match node_at_mut(&mut mutated, &path)? {
                    MarkupNode::Text(text) => *text = prior,
                    _ => return Err("inverse set-text: target is not a text node".into()),
                }
                Ok(mutated)
            }
            other => Err(format!("mutation kind {other:?} has no oracle inverse implementation")),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Projection
    /// 👁️ The `semantic-xml-valid-1-0-v1` shape: the two prolog declarations this subset's validity
    /// rules read, the four verdicts derived from them, and the element tree. Deliberately NOT
    /// `📰markup`'s own `project_markup`: that one keeps the DOCTYPE as one opaque string (so a
    /// reordered internal subset and a rewritten external id look the same shape of change) and
    /// decomposes `viewBox`/`transform` into typed geometry, which is the SVG profiles' concern and
    /// not XML validity's.
    pub fn project(doc: &MarkupDoc) -> Result<Json, String> {
        let doctype = doc.doctype.as_deref().map(parse_doctype).transpose()?;
        Ok(obj(vec![
            (
                "declaration",
                match &doc.declaration {
                    Some(declaration) => obj(vec![
                        ("present", Json::Bool(true)),
                        ("version", Json::String(declaration.version.clone())),
                        ("encoding", declaration.encoding.clone().map(Json::String).unwrap_or(Json::Null)),
                        ("standalone", declaration.standalone.map(Json::Bool).unwrap_or(Json::Null)),
                    ]),
                    None => obj(vec![("present", Json::Bool(false))]),
                },
            ),
            (
                "doctype",
                match &doctype {
                    Some(declared) => obj(vec![
                        ("present", Json::Bool(true)),
                        ("name", Json::String(declared.name.clone())),
                        (
                            "externalId",
                            match &declared.external_id {
                                None => Json::Null,
                                Some(ExternalId::System { system_id }) => obj(vec![("kind", Json::String("system".into())), ("systemId", Json::String(system_id.clone()))]),
                                Some(ExternalId::Public { public_id, system_id }) => {
                                    obj(vec![("kind", Json::String("public".into())), ("publicId", Json::String(public_id.clone())), ("systemId", Json::String(system_id.clone()))])
                                }
                            },
                        ),
                        (
                            "entities",
                            Json::Array(
                                declared
                                    .declarations
                                    .iter()
                                    .filter_map(|entry| match entry {
                                        Declaration::Entity { parameter, name, value } => Some(obj(vec![("parameter", Json::Bool(*parameter)), ("name", Json::String(name.clone())), ("value", Json::String(value.clone()))])),
                                        Declaration::Opaque { .. } => None,
                                    })
                                    .collect(),
                            ),
                        ),
                        (
                            "opaqueDeclarations",
                            Json::Array(
                                declared
                                    .declarations
                                    .iter()
                                    .filter_map(|entry| match entry {
                                        Declaration::Opaque { raw } => Some(Json::String(raw.clone())),
                                        Declaration::Entity { .. } => None,
                                    })
                                    .collect(),
                            ),
                        ),
                    ]),
                    None => obj(vec![("present", Json::Bool(false))]),
                },
            ),
            ("documentElement", document_element_name(doc).map(|name| Json::String(name.to_string())).unwrap_or(Json::Null)),
            ("validity", verdicts(doc)?),
            ("prolog", Json::Array(doc.prolog.iter().map(project_node).collect())),
            ("root", doc.root.as_ref().map(project_node).unwrap_or(Json::Null)),
        ]))
    }

    /// 🌳️ One node, with attributes name-sorted because XML attribute order is writer freedom while
    /// sibling and child order are normative and are never sorted.
    fn project_node(node: &MarkupNode) -> Json {
        match node {
            MarkupNode::Text(text) => obj(vec![("kind", Json::String("text".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::CData(text) => obj(vec![("kind", Json::String("cdata".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::Comment(text) => obj(vec![("kind", Json::String("comment".into())), ("text", Json::String(text.clone()))]),
            MarkupNode::Pi { target, data } => obj(vec![("kind", Json::String("pi".into())), ("target", Json::String(target.clone())), ("data", Json::String(data.clone()))]),
            MarkupNode::Element { name, attrs, children } => {
                let mut sorted: Vec<&(String, String)> = attrs.iter().collect();
                sorted.sort_by(|one, other| one.0.cmp(&other.0));
                obj(vec![
                    ("kind", Json::String("element".into())),
                    ("name", Json::String(name.clone())),
                    ("attrs", Json::Array(sorted.into_iter().map(|(key, value)| Json::Array(vec![Json::String(key.clone()), Json::String(value.clone())])).collect())),
                    ("children", Json::Array(children.iter().map(project_node).collect())),
                ])
            }
        }
    }
    //#endregion 🔖️Projection

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let mut doc = parse_markup(input)?;
        apply(&mut doc, kind, params)?;
        write_markup(&doc)
    }

    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        if kind.is_empty() {
            return Err("mutation spec carries no `kind`".to_string());
        }
        let base = parse_markup(input)?;
        let mut mutated = base.clone();
        apply(&mut mutated, kind, params)?;
        write_markup(&invert(&base, mutated, kind, params)?)
    }

    pub fn round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
        write_markup(&parse_markup(input)?)
    }

    pub fn project_bytes(bytes: &[u8]) -> Result<Json, String> {
        project(&parse_markup(bytes)?)
    }
    //#endregion 🔖️Routing

    #[cfg(test)]
    mod tests {
        use super::*;

        const PLIST: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>UTTypeIdentifier</key>
  <string>tech.semio.document</string>
</dict>
</plist>
"#;

        fn plist() -> MarkupDoc {
            parse_markup(PLIST).expect("the real fixture parses")
        }

        #[test]
        fn the_real_fixture_decomposes_into_apples_public_external_id() {
            let doctype = parse_doctype(plist().doctype.as_deref().expect("the fixture carries a DOCTYPE")).expect("parses");
            assert_eq!(doctype.name, "plist");
            assert_eq!(doctype.external_id, Some(ExternalId::Public { public_id: "-//Apple//DTD PLIST 1.0//EN".into(), system_id: "http://www.apple.com/DTDs/PropertyList-1.0.dtd".into() }));
            assert!(doctype.declarations.is_empty(), "the real fixture declares no internal subset");
        }

        #[test]
        fn the_doctype_grammar_round_trips_every_form_it_accepts() {
            for raw in [
                "plist",
                "plist SYSTEM \"plist.dtd\"",
                "plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\"",
                "requests [<!ENTITY semio \"Semio\"><!ENTITY % shared \"x\">]",
                "requests [<!ELEMENT requests (internal|external)*>]",
            ] {
                let parsed = parse_doctype(raw).unwrap_or_else(|error| panic!("{raw:?} must parse: {error}"));
                assert_eq!(render_doctype(&parsed), raw, "the grammar must be its own inverse for {raw:?}");
            }
        }

        #[test]
        fn an_opaque_content_model_declaration_is_reported_rather_than_dropped() {
            let doctype = parse_doctype("requests [<!ELEMENT requests (internal|external)*>]").expect("parses");
            assert_eq!(doctype.declarations, vec![Declaration::Opaque { raw: "<!ELEMENT requests (internal|external)*>".to_string() }]);
        }

        #[test]
        fn the_verdicts_read_the_two_hard_axes_and_the_standalone_pair() {
            let clean = verdicts(&plist()).expect("verdicts");
            assert_eq!(clean.get("doctypePresent"), Some(&Json::Bool(true)));
            assert_eq!(clean.get("doctypeNameMatchesDocumentElement"), Some(&Json::Bool(true)));
            assert_eq!(clean.get("standaloneBesideExternalSubset"), Some(&Json::Bool(false)));

            let mut standalone = plist();
            apply(&mut standalone, "set-standalone", &obj(vec![("standalone", Json::Bool(true))])).expect("applies");
            assert_eq!(verdicts(&standalone).expect("verdicts").get("standaloneBesideExternalSubset"), Some(&Json::Bool(true)), "§2.9 fires only once BOTH halves are set");

            let mismatched = parse_markup(b"<!DOCTYPE book>\n<plist/>").expect("parses");
            assert_eq!(verdicts(&mismatched).expect("verdicts").get("doctypeNameMatchesDocumentElement"), Some(&Json::Bool(false)));
        }

        #[test]
        fn rename_document_element_retags_the_doctype_in_the_same_step() {
            let mut doc = plist();
            apply(&mut doc, "rename-document-element", &obj(vec![("name", Json::String("propertyList".into()))])).expect("applies");
            assert_eq!(document_element_name(&doc), Some("propertyList"));
            assert_eq!(parse_doctype(doc.doctype.as_deref().expect("doctype")).expect("parses").name, "propertyList");
            assert_eq!(verdicts(&doc).expect("verdicts").get("doctypeNameMatchesDocumentElement"), Some(&Json::Bool(true)), "the rename must never pass through an invalid state");
        }

        #[test]
        fn set_snapshot_refuses_a_replacement_that_is_not_valid() {
            let mut doc = plist();
            assert!(apply(&mut doc, "set-snapshot", &obj(vec![("xml", Json::String("<plist/>".into()))])).is_err(), "a replacement with no DOCTYPE is not XML 1.0 valid");
            assert!(apply(&mut doc, "set-snapshot", &obj(vec![("xml", Json::String("<!DOCTYPE book>\n<plist/>".into()))])).is_err(), "§2.8 requires the DOCTYPE Name to be the document element's name");
        }

        #[test]
        fn declare_entity_places_at_the_index_and_refuses_a_duplicate() {
            let mut doc = plist();
            apply(&mut doc, "declare-entity", &obj(vec![("index", Json::Number(0.0)), ("name", Json::String("first".into())), ("value", Json::String("1".into()))])).expect("applies");
            apply(&mut doc, "declare-entity", &obj(vec![("index", Json::Number(0.0)), ("name", Json::String("zero".into())), ("value", Json::String("0".into()))])).expect("applies");
            let names: Vec<String> = parse_doctype(doc.doctype.as_deref().expect("doctype"))
                .expect("parses")
                .declarations
                .iter()
                .filter_map(|entry| match entry {
                    Declaration::Entity { name, .. } => Some(name.clone()),
                    Declaration::Opaque { .. } => None,
                })
                .collect();
            assert_eq!(names, vec!["zero".to_string(), "first".to_string()], "§4.2 makes position semantic");
            assert!(apply(&mut doc, "declare-entity", &obj(vec![("index", Json::Number(0.0)), ("name", Json::String("first".into())), ("value", Json::String("x".into()))])).is_err());
        }

        #[test]
        fn a_kind_this_subset_does_not_declare_is_an_error_not_a_no_op() {
            let mut doc = plist();
            assert!(apply(&mut doc, "set-doctype", &Json::Object(Vec::new())).is_err(), "`✳️any`'s own vocabulary must not be silently accepted here");
            assert!(apply(&mut doc, "undeclare-doctype", &Json::Object(Vec::new())).is_err());
        }
    }
}
//#endregion 🔖️Live

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::apply_mutation(input, &spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// ↩️ Applies one declared mutation kind and then its own computed inverse, in sequence.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    live::apply_mutation_inverse(input, &spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null))
}

/// 🔁️ Decodes with the independent reader and re-encodes from its own tree alone, no mutation
/// applied — the identity round trip.
#[cfg(feature = "oracles")]
pub fn oracle_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    live::round_trip(input)
}

/// 👁️ Projects XML bytes with the INDEPENDENT reader onto the `semantic-xml-valid-1-0-v1` shape.
#[cfg(feature = "oracles")]
pub fn project_xml_valid(bytes: &[u8]) -> Result<Json, String> {
    live::project_bytes(bytes)
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
pub fn project_xml_valid(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
