//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `html5ever`/`markup5ever_rcdom` reference implementation so the subject's own
//! mutation has an independent result to be compared against instead of being checked against its
//! own reading. `html5ever` is the WHATWG-conformant tokenizer/tree-builder; `markup5ever_rcdom`
//! supplies the reference-counted DOM it builds into and the `SerializableHandle` it re-serializes
//! from — together the only registered crate pair that both PARSES and WRITES real HTML5.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it. HTML has no shared family helper (unlike
//! `document`/`raster`/...) — no other subset genuinely shares this implementation.
//!
//! ## Two real, documented normalizations this oracle's DOM cannot avoid
//! 1. **Boolean attributes collapse to empty-string value.** The WHATWG tokenizer spec gives every
//!    attribute a value, defaulting to `""` when no `=` appears in the source — `rcdom`'s
//!    `Attribute.value` is a plain `StrTendril`, with no slot for "valueless". This subset's own
//!    `HtmlAttr{value: Option<String>}` DOES distinguish `<p disabled>` (`None`) from
//!    `<p disabled="">` (`Some("")`), a real distinction this oracle's tree cannot reproduce. Every
//!    `set-attribute` example this case exercises therefore uses a concrete non-empty value, never
//!    the valueless tri-state branch, so the differential never asks the oracle to draw a distinction
//!    its own DOM has already erased — that branch is exercised by this subset's own Rust-level unit
//!    tests instead (`schema/🧬️mutations/🦀️.rs::set_attribute_tristate_apply_and_inverse_round_trip`).
//! 2. **`write_doctype` drops the public/system id.** `html5ever::serialize::HtmlSerializer` only
//!    ever emits `<!DOCTYPE name>` (see `html5ever::serialize::mod::write_doctype`), so a doctype
//!    with a public/system id (legacy HTML 4 quirks-mode triggers, absent from every real HTML5 page
//!    this case touches) cannot round-trip through this oracle. Recorded, not worked around — the
//!    real fixture only ever carries the bare `<!doctype html>` this loses nothing on.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself.

use semio_repo_test_host::Json;

//#region 🔖️Kinds
/// 📇️ Mirrors `../../../../../🧬️schema/🧬️mutations/🦀️.rs::KINDS` -- duplicated, not
/// imported, because the oracle role must not link the subject crate at all (fleet brief §5.3). Used
/// here only by this module's own real-fixture sweep test, which is what keeps this copy honest
/// against a drift (the case adapter's own `KINDS` is the one the runner actually dispatches on).
#[cfg(test)]
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-doctype", "insert-node", "remove-node", "set-element-name", "set-attribute", "set-text", "set-comment", "set-raw-text"];
//#endregion 🔖️Kinds

#[cfg(feature = "oracles")]
//#region 🔖️Oracles
mod oracles {
    use html5ever::serialize::{serialize, SerializeOpts};
    use html5ever::tendril::TendrilSink;
    use html5ever::{parse_document, ParseOpts};
    use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};
    use semio_repo_test_host::Json;

    //#region 🔖️Tree
    /// 🌳 Owned HTML node, independent of
    /// `crate::standards::v5::subsets::any::schema::snapshot::HtmlNode` (this crate
    /// never depends on `semio-s-plugin-stdio`, the production crate that type lives in — see this
    /// file's own header) but shaped identically variant for variant, so a spec written for the
    /// oracle reads the same as one written for the subject.
    #[derive(Clone, Debug, PartialEq)]
    enum HNode {
        Element { name: String, attrs: Vec<(String, Option<String>)>, children: Vec<HNode> },
        Text(String),
        Comment(String),
        RawText { script: bool, text: String },
    }

    #[derive(Clone, Debug, Default, PartialEq)]
    struct HDoc {
        doctype: Option<String>,
        root: Option<HNode>,
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

    /// 🏳️ Tri-state attribute value read from a mutation spec's `value` key: the key ABSENT means
    /// "remove the attribute" (`None`), present and `Json::Null` means "valueless" (`Some(None)`),
    /// present and a string means "set to that value" (`Some(Some(v))`) — mirrors
    /// `HtmlMutation::SetAttribute`'s own `Option<Option<String>>` field exactly.
    fn tristate_value(params: &Json) -> Option<Option<String>> {
        match params.get("value") {
            None => None,
            Some(Json::Null) => Some(None),
            Some(Json::String(text)) => Some(Some(text.clone())),
            Some(_) => Some(None),
        }
    }

    /// 🔎️ Owned node-spec JSON grammar mutation params speak: `{"kind":"element","name":...,
    /// "attributes":[{"name":...,"value":string|null}],"children":[...]}` |
    /// `{"kind":"text"|"comment","text":...}` | `{"kind":"rawText","parentKind":"script"|"style",
    /// "text":...}`.
    fn json_to_hnode(value: &Json) -> Result<HNode, String> {
        match value.str("kind").as_str() {
            "element" => Ok(HNode::Element {
                name: value.str("name"),
                attrs: value
                    .array("attributes")
                    .iter()
                    .map(|attr| {
                        (
                            attr.str("name"),
                            match attr.get("value") {
                                Some(Json::String(text)) => Some(text.clone()),
                                _ => None,
                            },
                        )
                    })
                    .collect(),
                children: value.array("children").iter().map(json_to_hnode).collect::<Result<Vec<_>, _>>()?,
            }),
            "text" => Ok(HNode::Text(value.str("text"))),
            "comment" => Ok(HNode::Comment(value.str("text"))),
            "rawText" => Ok(HNode::RawText { script: value.str("parentKind") != "style", text: value.str("text") }),
            other => Err(format!("unknown node kind {other:?}")),
        }
    }

    /// 🔁️ The reverse of [`json_to_hnode`] — used to capture a removed node's exact value so an
    /// inverse spec can hand it back to [`json_to_hnode`] as the undo's own `insert-node` params, and
    /// to project a node for comparison.
    fn hnode_to_json(node: &HNode) -> Json {
        match node {
            HNode::Element { name, attrs, children } => Json::Object(vec![
                ("kind".to_string(), Json::String("element".to_string())),
                ("name".to_string(), Json::String(name.clone())),
                (
                    "attributes".to_string(),
                    Json::Array(
                        attrs
                            .iter()
                            .map(|(key, value)| {
                                Json::Object(vec![
                                    ("name".to_string(), Json::String(key.clone())),
                                    (
                                        "value".to_string(),
                                        match value {
                                            Some(v) => Json::String(v.clone()),
                                            None => Json::Null,
                                        },
                                    ),
                                ])
                            })
                            .collect(),
                    ),
                ),
                ("children".to_string(), Json::Array(children.iter().map(hnode_to_json).collect())),
            ]),
            HNode::Text(text) => Json::Object(vec![("kind".to_string(), Json::String("text".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            HNode::Comment(text) => Json::Object(vec![("kind".to_string(), Json::String("comment".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            HNode::RawText { script, text } => Json::Object(vec![
                ("kind".to_string(), Json::String("rawText".to_string())),
                ("parentKind".to_string(), Json::String(if *script { "script".to_string() } else { "style".to_string() })),
                ("text".to_string(), Json::String(text.clone())),
            ]),
        }
    }
    //#endregion 🔖️JsonValue

    //#region 🔖️PathAddressing
    /// 🔎️ Immutable walk of `path` (a chain of child indices) from `root`, mirroring
    /// `crate::standards::v5::subsets::any::schema::snapshot::node_at` — `path ==
    /// []` addresses `root` itself.
    fn resolve<'a>(root: Option<&'a HNode>, path: &[usize]) -> Option<&'a HNode> {
        let mut current = root?;
        for &index in path {
            let HNode::Element { children, .. } = current else { return None };
            current = children.get(index)?;
        }
        Some(current)
    }

    fn resolve_mut<'a>(root: Option<&'a mut HNode>, path: &[usize]) -> Option<&'a mut HNode> {
        let mut current = root?;
        for &index in path {
            let HNode::Element { children, .. } = current else { return None };
            current = children.get_mut(index)?;
        }
        Some(current)
    }
    //#endregion 🔖️PathAddressing

    //#region 🔖️Parse
    /// 🌳 One `rcdom::Handle` into an owned [`HNode`] — `<script>`/`<style>` elements collapse their
    /// (possibly-multiple, `rcdom` never merges adjacent `Text` nodes across a `create_comment`/
    /// `create_pi` boundary but always DOES merge adjacent literal text via `append_to_existing_text`)
    /// single `Text` child into one [`HNode::RawText`], the same content-model distinction this
    /// subset's own parser draws (`markup5ever_rcdom` itself has no `RawText` node kind — RAWTEXT
    /// content is still plain `NodeData::Text` there, see this file's own header, point 1's sibling
    /// finding).
    fn node_from_handle(handle: &Handle) -> Option<HNode> {
        match &handle.data {
            NodeData::Element { name, attrs, .. } => {
                let tag = name.local.to_string();
                let raw_kind = if tag.eq_ignore_ascii_case("script") {
                    Some(true)
                } else if tag.eq_ignore_ascii_case("style") {
                    Some(false)
                } else {
                    None
                };
                let attrs = attrs.borrow().iter().map(|attr| (attr.name.local.to_string(), Some(attr.value.to_string()))).collect();
                let children_handles = handle.children.borrow();
                if let Some(script) = raw_kind {
                    let text: String = children_handles
                        .iter()
                        .filter_map(node_from_handle)
                        .map(|node| match node {
                            HNode::Text(text) => text,
                            _ => String::new(),
                        })
                        .collect();
                    return Some(HNode::Element { name: tag, attrs, children: if text.is_empty() { Vec::new() } else { vec![HNode::RawText { script, text }] } });
                }
                let children = children_handles.iter().filter_map(node_from_handle).collect();
                Some(HNode::Element { name: tag, attrs, children })
            }
            NodeData::Text { contents } => Some(HNode::Text(contents.borrow().to_string())),
            NodeData::Comment { contents } => Some(HNode::Comment(contents.to_string())),
            // 🚫️ Document/Doctype/ProcessingInstruction never appear as an ELEMENT's child in a
            // conformant HTML5 tree — a PI can only occur at the (skipped) document top level, since
            // HTML5 (unlike XML) has no processing-instruction content model inside elements.
            NodeData::Document | NodeData::Doctype { .. } | NodeData::ProcessingInstruction { .. } => None,
        }
    }

    /// 🔓️ Parses real HTML5 bytes through the WHATWG-conformant `html5ever` tree builder. `root` is
    /// the first `Element` child of the `Document` node (per spec there is always exactly one after
    /// tree construction — `html5ever` inserts an implied `<html>` if the source omits it); `doctype`
    /// is `Some("DOCTYPE {name}")` when a `Doctype` document child exists, matching this subset's own
    /// raw-content convention.
    fn parse(bytes: &[u8]) -> Result<HDoc, String> {
        let dom: RcDom = parse_document(RcDom::default(), ParseOpts::default()).from_utf8().read_from(&mut &bytes[..]).map_err(|error| error.to_string())?;
        let mut doctype = None;
        let mut root = None;
        for child in dom.document.children.borrow().iter() {
            match &child.data {
                NodeData::Doctype { name, .. } => doctype = Some(format!("DOCTYPE {name}")),
                NodeData::Element { .. } if root.is_none() => root = node_from_handle(child),
                _ => {}
            }
        }
        Ok(HDoc { doctype, root })
    }
    //#endregion 🔖️Parse

    //#region 🔖️Serialize
    /// 🖊️ Rebuilds a fresh `rcdom` tree from an owned [`HNode`] and serializes it with
    /// `html5ever::serialize` (the only channel back to bytes — this oracle never hand-formats HTML
    /// text itself). `<script>`/`<style>` RAWTEXT content re-enters as a plain `Text` child, the same
    /// direction [`node_from_handle`] came from; `HtmlSerializer::write_text` already special-cases
    /// those two element names to skip escaping, so the RAWTEXT content model survives the trip.
    fn handle_from_node(node: &HNode) -> Handle {
        use html5ever::{ns, LocalName, QualName};
        use markup5ever_rcdom::Node;
        match node {
            HNode::Text(text) => Node::new(NodeData::Text { contents: std::cell::RefCell::new(text.as_str().into()) }),
            HNode::Comment(text) => Node::new(NodeData::Comment { contents: text.as_str().into() }),
            HNode::RawText { text, .. } => Node::new(NodeData::Text { contents: std::cell::RefCell::new(text.as_str().into()) }),
            HNode::Element { name, attrs, children } => {
                let qual = QualName::new(None, ns!(html), LocalName::from(name.as_str()));
                let built_attrs = attrs.iter().map(|(key, value)| html5ever::Attribute { name: QualName::new(None, ns!(), LocalName::from(key.as_str())), value: value.clone().unwrap_or_default().as_str().into() }).collect();
                let element = Node::new(NodeData::Element { name: qual, attrs: std::cell::RefCell::new(built_attrs), template_contents: std::cell::RefCell::new(None), mathml_annotation_xml_integration_point: false });
                for child in children {
                    let child_handle = handle_from_node(child);
                    child_handle.parent.set(Some(std::rc::Rc::downgrade(&element)));
                    element.children.borrow_mut().push(child_handle);
                }
                element
            }
        }
    }

    fn serialize_doc(doc: &HDoc) -> Result<Vec<u8>, String> {
        use markup5ever_rcdom::Node;
        let document = Node::new(NodeData::Document);
        if let Some(doctype) = &doc.doctype {
            let name = doctype.strip_prefix("DOCTYPE ").unwrap_or(doctype.as_str());
            let doctype_handle = Node::new(NodeData::Doctype { name: name.into(), public_id: "".into(), system_id: "".into() });
            doctype_handle.parent.set(Some(std::rc::Rc::downgrade(&document)));
            document.children.borrow_mut().push(doctype_handle);
        }
        if let Some(root) = &doc.root {
            let root_handle = handle_from_node(root);
            root_handle.parent.set(Some(std::rc::Rc::downgrade(&document)));
            document.children.borrow_mut().push(root_handle);
        }
        let mut buf = Vec::new();
        serialize(&mut buf, &SerializableHandle::from(document), SerializeOpts::default()).map_err(|error| error.to_string())?;
        Ok(buf)
    }
    //#endregion 🔖️Serialize

    //#region 🔖️Forward
    /// ▶️ Applies one `{kind, params}` mutation to `doc` in place. Out-of-range indices / unresolved
    /// paths are errors here (never a silent no-op), matching this dispatch's own contract, though
    /// every example this subset's own feature exercises resolves against the real document.
    fn apply_kind(doc: &mut HDoc, kind: &str, params: &Json) -> Result<(), String> {
        match kind {
            "no-mutation" => {}
            "set-snapshot" => {
                let doctype = match params.get("doctype") {
                    Some(Json::String(text)) => Some(text.clone()),
                    _ => None,
                };
                let root = Some(json_to_hnode(&params.get("root").cloned().unwrap_or(Json::Null))?);
                *doc = HDoc { doctype, root };
            }
            "set-doctype" => {
                doc.doctype = match params.get("doctype") {
                    Some(Json::String(text)) => Some(text.clone()),
                    _ => None,
                }
            }
            "insert-node" => {
                let parent = usize_path(params.array("parent"));
                let index = usize_field(params, "index");
                let node = json_to_hnode(&params.get("node").cloned().unwrap_or(Json::Null))?;
                let HNode::Element { children, .. } = resolve_mut(doc.root.as_mut(), &parent).ok_or("insert-node: parent does not resolve to an element")? else {
                    return Err("insert-node: parent does not address an element".to_string());
                };
                children.insert(index.min(children.len()), node);
            }
            "remove-node" => {
                let parent = usize_path(params.array("parent"));
                let index = usize_field(params, "index");
                let HNode::Element { children, .. } = resolve_mut(doc.root.as_mut(), &parent).ok_or("remove-node: parent does not resolve to an element")? else {
                    return Err("remove-node: parent does not address an element".to_string());
                };
                if index < children.len() {
                    children.remove(index);
                }
            }
            "set-element-name" => {
                let path = usize_path(params.array("path"));
                let name = params.str("name");
                let HNode::Element { name: current, .. } = resolve_mut(doc.root.as_mut(), &path).ok_or("set-element-name: path does not resolve to an element")? else {
                    return Err("set-element-name: path does not address an element".to_string());
                };
                *current = name;
            }
            "set-attribute" => {
                let path = usize_path(params.array("path"));
                let name = params.str("name");
                let value = tristate_value(params);
                let HNode::Element { attrs, .. } = resolve_mut(doc.root.as_mut(), &path).ok_or("set-attribute: path does not resolve to an element")? else {
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
                let HNode::Text(current) = resolve_mut(doc.root.as_mut(), &path).ok_or("set-text: path does not resolve to a text node")? else {
                    return Err("set-text: path does not address a text node".to_string());
                };
                *current = text;
            }
            "set-comment" => {
                let path = usize_path(params.array("path"));
                let text = params.str("text");
                let HNode::Comment(current) = resolve_mut(doc.root.as_mut(), &path).ok_or("set-comment: path does not resolve to a comment node")? else {
                    return Err("set-comment: path does not address a comment node".to_string());
                };
                *current = text;
            }
            "set-raw-text" => {
                let path = usize_path(params.array("path"));
                let text = params.str("text");
                let HNode::RawText { text: current, .. } = resolve_mut(doc.root.as_mut(), &path).ok_or("set-raw-text: path does not resolve to a raw-text node")? else {
                    return Err("set-raw-text: path does not address a raw-text node".to_string());
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
    /// params}` — same law `HtmlMutation::inverse` proves at the Rust-model level
    /// (`schema/🧬️mutations/🦀️.rs`), computed here independently against this oracle's own
    /// tree instead.
    fn inverse_spec(base: &HDoc, kind: &str, params: &Json) -> Json {
        let spec = |inverse_kind: &str, inverse_params: Json| Json::Object(vec![("kind".to_string(), Json::String(inverse_kind.to_string())), ("params".to_string(), inverse_params)]);
        let obj = |entries: Vec<(&str, Json)>| Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect());
        match kind {
            "no-mutation" => spec("no-mutation", obj(vec![])),
            "set-snapshot" => spec(
                "set-snapshot",
                obj(vec![
                    (
                        "doctype",
                        match &base.doctype {
                            Some(text) => Json::String(text.clone()),
                            None => Json::Null,
                        },
                    ),
                    (
                        "root",
                        match &base.root {
                            Some(root) => hnode_to_json(root),
                            None => Json::Null,
                        },
                    ),
                ]),
            ),
            "set-doctype" => spec(
                "set-doctype",
                obj(vec![(
                    "doctype",
                    match &base.doctype {
                        Some(text) => Json::String(text.clone()),
                        None => Json::Null,
                    },
                )]),
            ),
            "insert-node" => {
                let parent = params.array("parent");
                let index = usize_field(params, "index");
                spec("remove-node", obj(vec![("parent", Json::Array(parent)), ("index", Json::Number(index as f64))]))
            }
            "remove-node" => {
                let parent_json = params.array("parent");
                let parent = usize_path(parent_json.clone());
                let index = usize_field(params, "index");
                let node = match resolve(base.root.as_ref(), &parent) {
                    Some(HNode::Element { children, .. }) => children.get(index).cloned(),
                    _ => None,
                };
                match node {
                    Some(existing) => spec("insert-node", obj(vec![("parent", Json::Array(parent_json)), ("index", Json::Number(index as f64)), ("node", hnode_to_json(&existing))])),
                    None => spec("no-mutation", obj(vec![])),
                }
            }
            "set-element-name" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(HNode::Element { name, .. }) => name.clone(),
                    _ => return spec("no-mutation", obj(vec![])),
                };
                spec("set-element-name", obj(vec![("path", Json::Array(path_json)), ("name", Json::String(prior))]))
            }
            "set-attribute" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let name = params.str("name");
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(HNode::Element { attrs, .. }) => attrs.iter().find(|(key, _)| key == &name).map(|(_, value)| value.clone()),
                    _ => None,
                };
                let value_json = match prior {
                    Some(Some(value)) => Json::String(value),
                    Some(None) => Json::Null,
                    None => Json::Null,
                };
                spec("set-attribute", obj(vec![("path", Json::Array(path_json)), ("name", Json::String(name)), ("value", value_json)]))
            }
            "set-text" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(HNode::Text(text)) => text.clone(),
                    _ => String::new(),
                };
                spec("set-text", obj(vec![("path", Json::Array(path_json)), ("text", Json::String(prior))]))
            }
            "set-comment" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(HNode::Comment(text)) => text.clone(),
                    _ => String::new(),
                };
                spec("set-comment", obj(vec![("path", Json::Array(path_json)), ("text", Json::String(prior))]))
            }
            "set-raw-text" => {
                let path_json = params.array("path");
                let path = usize_path(path_json.clone());
                let prior = match resolve(base.root.as_ref(), &path) {
                    Some(HNode::RawText { text, .. }) => text.clone(),
                    _ => String::new(),
                };
                spec("set-raw-text", obj(vec![("path", Json::Array(path_json)), ("text", Json::String(prior))]))
            }
            other => spec(other, params.clone()),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Routing
    pub fn apply_mutation(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let mut doc = parse(input)?;
        apply_kind(&mut doc, kind, params)?;
        serialize_doc(&doc)
    }

    /// ↩️ Applies `{kind, params}` and then its computed inverse, in sequence, and returns the
    /// re-serialized result — the caller compares its projection against the ORIGINAL input's own.
    /// ↩️ Applies `{kind, params}` and its own computed inverse on the SAME in-memory tree, never
    /// round-tripping through bytes in between (unlike a `mutate-<kind>` scenario, which genuinely
    /// serializes once). A real, HTML5-inherent reason: two `Text` nodes serialize with no boundary
    /// marker between them, so if `apply_kind` ever leaves two `Text` siblings adjacent (e.g.
    /// `remove-node` deleting an element that sat between two whitespace-only text runs — this
    /// fixture's own indentation makes that the common case, not an edge case), a REPARSE of the
    /// intermediate bytes would coalesce them into one node via `html5ever`'s own real
    /// `append_to_existing_text` tree-construction behaviour, silently shortening the sibling list
    /// before the inverse ever runs. Working on the tree directly is not a workaround for a defect —
    /// it is what `HtmlMutation::inverse`'s OWN law already assumes at the model level (`apply(base,
    /// m)` then `apply(_, inverse(m, base)) == base`, no serialization step named anywhere in it) —
    /// and it is genuinely independent of the subject: this oracle's tree, dispatch and serializer
    /// share no code with `crate::standards::v5::subsets::any`.
    pub fn apply_mutation_inverse(input: &[u8], kind: &str, params: &Json) -> Result<Vec<u8>, String> {
        let base = parse(input)?;
        let inverse = inverse_spec(&base, kind, params);
        let mut doc = base;
        apply_kind(&mut doc, kind, params)?;
        apply_kind(&mut doc, &inverse.str("kind"), inverse.get("params").unwrap_or(&Json::Null))?;
        serialize_doc(&doc)
    }

    //#region 🔖️Projection
    /// 👁️ This subset's own semantic projection — doctype plus the full element tree in document
    /// order, independently re-derived by re-parsing `bytes` through `html5ever` rather than trusting
    /// whatever produced them. Attributes project as a name/value MAP sorted by key (real writer
    /// freedom: HTML gives attribute order no semantic meaning), never the ordered list the tree
    /// stores them as — sorted explicitly here rather than leaned on the comparison mechanism's own
    /// canonicalization, so this projection is genuinely order-independent by construction.
    pub fn project(bytes: &[u8]) -> Result<Json, String> {
        let doc = parse(bytes)?;
        Ok(Json::Object(vec![
            (
                "doctype".to_string(),
                match &doc.doctype {
                    Some(text) => Json::String(text.clone()),
                    None => Json::Null,
                },
            ),
            (
                "root".to_string(),
                match &doc.root {
                    Some(root) => node_projection(root),
                    None => Json::Null,
                },
            ),
        ]))
    }

    fn node_projection(node: &HNode) -> Json {
        match node {
            HNode::Element { name, attrs, children } => {
                let mut sorted: Vec<(String, Json)> = attrs
                    .iter()
                    .map(|(key, value)| {
                        (
                            key.clone(),
                            match value {
                                Some(v) => Json::String(v.clone()),
                                None => Json::Null,
                            },
                        )
                    })
                    .collect();
                sorted.sort_by(|a, b| a.0.cmp(&b.0));
                Json::Object(vec![
                    ("kind".to_string(), Json::String("element".to_string())),
                    ("name".to_string(), Json::String(name.clone())),
                    ("attributes".to_string(), Json::Object(sorted)),
                    ("children".to_string(), Json::Array(children.iter().map(node_projection).collect())),
                ])
            }
            HNode::Text(text) => Json::Object(vec![("kind".to_string(), Json::String("text".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            HNode::Comment(text) => Json::Object(vec![("kind".to_string(), Json::String("comment".to_string())), ("text".to_string(), Json::String(text.clone()))]),
            HNode::RawText { script, text } => Json::Object(vec![
                ("kind".to_string(), Json::String("rawText".to_string())),
                ("parentKind".to_string(), Json::String(if *script { "script".to_string() } else { "style".to_string() })),
                ("text".to_string(), Json::String(text.clone())),
            ]),
        }
    }
    //#endregion 🔖️Projection
    //#endregion 🔖️Routing

    //#region 🧪️Tests
    #[cfg(test)]
    include!("🧪️tests/🔬️oracles-unit/🦀️.rs");
    //#endregion 🧪️Tests
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
/// same `apply(inverse(m, base), apply(m, base)) == base` law `HtmlMutation::inverse` proves at the
/// Rust-model level, here against the registered reference implementation instead.
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
pub fn project_html_5(bytes: &[u8]) -> Result<Json, String> {
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
pub fn project_html_5(_bytes: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch
