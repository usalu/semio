//! 🧬️ XmlMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::artifacts::xml::schema::diff::{
    dec_declaration, dec_doctype, dec_doctype_bin, dec_prolog, dec_str, dec_xml_node, decode_option, enc_declaration, enc_doctype, enc_doctype_bin, enc_prolog, enc_str, enc_xml_node, encode_option, split_top_level, strip_brackets,
};
use crate::artifacts::xml::schema::diff::{diff_set_snapshot, XmlAttrAdded, XmlAttrModified, XmlAttributesDiff, XmlChildAdded, XmlChildrenDiff, XmlDiff, XmlElementDiff, XmlNodeDiff};
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlNode};
use crate::artifacts::xml::XmlSnapshot;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️NodePath
/// 🧭️ Path from the document root to a node: chain of child indices at each nesting level.
/// `XmlNodePath(vec![])` addresses the root itself. Mutation-level only (never appears inside
/// `XmlDiff` -- diffs nest via `XmlChildModified` chains instead, built by `diff_at_path`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct XmlNodePath(pub Vec<usize>);

impl XmlNodePath {
    /// 🌳 The empty path -- addresses the document root.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn root() -> Self {
        Self(Vec::new())
    }

    /// 🔎️ Walks `self` from `root`, returning the addressed node if it exists and every
    /// intermediate segment is itself an `Element` (any other shape or an out-of-range index is a
    /// graceful `None`, never a panic).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve<'a>(&self, root: Option<&'a XmlNode>) -> Option<&'a XmlNode> {
        let mut current = root?;
        for &index in &self.0 {
            let XmlNode::Element { children, .. } = current else { return None };
            current = children.get(index)?;
        }
        Some(current)
    }
}
//#endregion 🔖️NodePath

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.xml`. `InsertElement`/`RemoveElement`'s `path` addresses
/// the PARENT element (`index` is the position among the parent's children); every other
/// path-carrying variant's `path` addresses the target node itself.
/// 🧪️ F6 CONFIRMED (real `cargo check -p semio-s-plugin-stdio --lib`, not guessed): adding
/// `#[derive(dsl::DslOps)]` to this enum fails with `error[E0277]: the trait bound ...: DslField is
/// not satisfied` for FOUR distinct field types simultaneously -- `SetSnapshot{snapshot:
/// XmlSnapshot}` (recursively contains `XmlNode`), `SetDeclaration{declaration: Option<XmlDeclaration>}`,
/// every `path: XmlNodePath` field (a plain newtype wrapper, not even an enum -- `DslField` simply
/// has no impl for it either), and `InsertElement`'s `node: XmlNode` directly. Same structural
/// reason `SvgMutation` fails (`f6-recon-report.md` §3a). `OpText`/`OpBinary` hand-rolled below,
/// reusing `XmlDiff`'s `pub(crate)` grammar primitives (`hex_encode`/`enc_xml_node`/
/// `split_top_level`/...) rather than the previous `serde_json` placeholder (which satisfied the
/// trait's LAWS but was not a genuine handcrafted grammar, per the recon report's explicit warning
/// against copying `WriterDiff`'s shortcut).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum XmlMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: XmlSnapshot,
    },
    SetDeclaration {
        declaration: Option<XmlDeclaration>,
    },
    SetDoctype {
        doctype: Option<XmlDoctype>,
    },
    /// ➕️ Inserts `node` at `index` among the children of the element addressed by `path`.
    InsertElement {
        path: XmlNodePath,
        index: usize,
        node: XmlNode,
    },
    /// ➖️ Removes the child at `index` among the children of the element addressed by `path`.
    RemoveElement {
        path: XmlNodePath,
        index: usize,
    },
    /// 🏷️ Sets (or, if `value` is `None`, removes) the attribute `name` on the element addressed
    /// by `path`.
    SetAttribute {
        path: XmlNodePath,
        name: String,
        value: Option<String>,
    },
    /// 🔤️ Sets the literal text of the `Text` node addressed by `path`.
    SetText {
        path: XmlNodePath,
        text: String,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`
/// -- the diff is the single semantics source, never a separate imperative apply path.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_xml_mutation(snapshot: &mut XmlSnapshot, mutation: &XmlMutation) -> protocol::MutationOutcome<XmlDiff> {
    let outcome = Mutation::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::error(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}
//#endregion 🔖️Apply

//#region 🔖️MutationTrait
impl Mutation<XmlSnapshot> for XmlMutation {
    type Diff = XmlDiff;

    async fn diff(&self, base: &XmlSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            XmlMutation::NoMutation => XmlDiff::default(),
            XmlMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            XmlMutation::SetDeclaration { declaration } => XmlDiff { prolog: None, declaration: Some(declaration.clone()), doctype: None, root: None },
            XmlMutation::SetDoctype { doctype } => XmlDiff { prolog: None, declaration: None, doctype: Some(doctype.clone()), root: None },
            XmlMutation::InsertElement { path, index, node } => diff_at_path(
                &path.0,
                XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlChildAdded { index: *index, item: node.clone() }] }) }),
            ),
            XmlMutation::RemoveElement { path, index } => {
                diff_at_path(&path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: None, children: Some(XmlChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) }))
            }
            XmlMutation::SetAttribute { path, name, value } => {
                let target = path.resolve(base.doc.root.as_ref());
                let existing = target.and_then(|n| match n {
                    XmlNode::Element { attrs, .. } => attrs.iter().find(|a| &a.name == name),
                    _ => None,
                });
                let attrs_diff = match (existing, value) {
                    (Some(_), Some(v)) => XmlAttributesDiff { removed: Vec::new(), modified: vec![XmlAttrModified { name: name.clone(), value: v.clone() }], added: Vec::new() },
                    (Some(_), None) => XmlAttributesDiff { removed: vec![name.clone()], modified: Vec::new(), added: Vec::new() },
                    (None, Some(v)) => {
                        let next_index = match target {
                            Some(XmlNode::Element { attrs, .. }) => attrs.len(),
                            _ => 0,
                        };
                        XmlAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![XmlAttrAdded { index: next_index, name: name.clone(), value: v.clone() }] }
                    }
                    (None, None) => XmlAttributesDiff::default(),
                };
                diff_at_path(&path.0, XmlNodeDiff::Element(XmlElementDiff { name: None, attributes: Some(attrs_diff), children: None }))
            }
            XmlMutation::SetText { path, text } => diff_at_path(&path.0, XmlNodeDiff::Text { text: Some(text.clone()) }),
        }).await
    }

    async fn inverse(&self, base: &XmlSnapshot) -> Vec<Self> {
        match self {
            XmlMutation::NoMutation => vec![XmlMutation::NoMutation],
            XmlMutation::SetSnapshot { .. } => vec![XmlMutation::SetSnapshot { snapshot: base.clone() }],
            XmlMutation::SetDeclaration { .. } => vec![XmlMutation::SetDeclaration { declaration: base.doc.declaration.clone() }],
            XmlMutation::SetDoctype { .. } => vec![XmlMutation::SetDoctype { doctype: base.doc.doctype.clone() }],
            XmlMutation::InsertElement { path, index, .. } => {
                vec![XmlMutation::RemoveElement { path: path.clone(), index: *index }]
            }
            XmlMutation::RemoveElement { path, index } => {
                let parent = path.resolve(base.doc.root.as_ref());
                let node = parent
                    .and_then(|n| match n {
                        XmlNode::Element { children, .. } => children.get(*index).cloned(),
                        _ => None,
                    })
                    .unwrap_or(XmlNode::Text { text: String::new() });
                vec![XmlMutation::InsertElement { path: path.clone(), index: *index, node }]
            }
            XmlMutation::SetAttribute { path, name, .. } => {
                let target = path.resolve(base.doc.root.as_ref());
                let prior = target.and_then(|n| match n {
                    XmlNode::Element { attrs, .. } => attrs.iter().find(|a| &a.name == name).map(|a| a.value.clone()),
                    _ => None,
                });
                vec![XmlMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior }]
            }
            XmlMutation::SetText { path, .. } => {
                let prior = path
                    .resolve(base.doc.root.as_ref())
                    .and_then(|n| match n {
                        XmlNode::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .unwrap_or_default();
                vec![XmlMutation::SetText { path: path.clone(), text: prior }]
            }
        }
    }
}

/// 🧭️ `path`-addressing convenience over `crate::artifacts::xml::schema::diff::diff_at_path`
/// (which takes a bare `&[usize]` so the diff module never needs to depend on this one).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_at_path(path: &[usize], leaf: XmlNodeDiff) -> XmlDiff {
    crate::artifacts::xml::schema::diff::diff_at_path(path, leaf)
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6: **hand-rolled** `OpText`/`OpBinary` for `XmlMutation` (`#[derive(dsl::DslOps)]` confirmed
/// rejected above) — reuses `XmlDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_xml_node`/`split_top_level`/`encode_option`/...) rather than duplicating them
/// a second time in this file, same intra-artifact reuse pattern `SvgMutation` uses off `SvgDiff`.
/// Grammar: `keyword arg=value ...` (space-separated, same shape the derive's own
/// handcrafted-wrapper convention uses — see `f6-recon-report.md` §2), one match arm per variant
/// (no `DslVariants` scaffolding available since nothing here derives it). Replaces the previous
/// `serde_json`-based placeholder, which satisfied the trait's LAWS but was not a genuine
/// handcrafted grammar (the recon report explicitly warns against copying `WriterDiff`'s shortcut).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_path(p: &XmlNodePath) -> String {
    format!("[{}]", p.0.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_path(s: &str) -> Result<XmlNodePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse().map_err(|e: std::num::ParseIntError| e.to_string())).collect::<Result<Vec<usize>, String>>().map(XmlNodePath)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_snapshot(s: &XmlSnapshot) -> String {
    format!("[{},{},{},{},{}]", enc_str(&s.schema), encode_option(&s.doc.root, enc_xml_node), encode_option(&s.doc.doctype, enc_doctype), encode_option(&s.doc.declaration, enc_declaration), enc_prolog(&s.doc.prolog),)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_snapshot(s: &str) -> Result<XmlSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, root, doctype, declaration, prolog] = parts.as_slice() else { return Err(format!("xml snapshot: expected 5 fields, got {}", parts.len())) };
    Ok(XmlSnapshot {
        schema: dec_str(schema)?,
        doc: crate::artifacts::xml::schema::snapshot::XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)? },
    })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_xml_mutation(m: &XmlMutation) -> String {
    match m {
        XmlMutation::NoMutation => "no-mutation".to_string(),
        XmlMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_xml_snapshot(snapshot)),
        XmlMutation::SetDeclaration { declaration } => format!("set-declaration declaration={}", encode_option(declaration, enc_declaration)),
        XmlMutation::SetDoctype { doctype } => format!("set-doctype doctype={}", encode_option(doctype, enc_doctype)),
        XmlMutation::InsertElement { path, index, node } => format!("insert-element path={} index={index} node={}", enc_node_path(path), enc_xml_node(node)),
        XmlMutation::RemoveElement { path, index } => format!("remove-element path={} index={index}", enc_node_path(path)),
        XmlMutation::SetAttribute { path, name, value } => format!("set-attribute path={} name={} value={}", enc_node_path(path), enc_str(name), encode_option(value, |v| enc_str(v))),
        XmlMutation::SetText { path, text } => format!("set-text path={} text={}", enc_node_path(path), enc_str(text)),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_xml_mutation(line: &str) -> Result<XmlMutation, String> {
    if line == "no-mutation" {
        return Ok(XmlMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("xml mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("xml mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(XmlMutation::SetSnapshot { snapshot: dec_xml_snapshot(arg("snapshot")?)? }),
        "set-declaration" => Ok(XmlMutation::SetDeclaration { declaration: decode_option(arg("declaration")?, dec_declaration)? }),
        "set-doctype" => Ok(XmlMutation::SetDoctype { doctype: decode_option(arg("doctype")?, dec_doctype)? }),
        "insert-element" => Ok(XmlMutation::InsertElement { path: dec_node_path(arg("path")?)?, index: usize_arg("index")?, node: dec_xml_node(arg("node")?)? }),
        "remove-element" => Ok(XmlMutation::RemoveElement { path: dec_node_path(arg("path")?)?, index: usize_arg("index")? }),
        "set-attribute" => Ok(XmlMutation::SetAttribute { path: dec_node_path(arg("path")?)?, name: dec_str(arg("name")?)?, value: decode_option(arg("value")?, dec_str)? }),
        "set-text" => Ok(XmlMutation::SetText { path: dec_node_path(arg("path")?)?, text: dec_str(arg("text")?)? }),
        other => Err(format!("xml mutation: unknown keyword {other:?}")),
    }
}

impl OpText for XmlMutation {
    async fn print_op(&self) -> String {
        print_xml_mutation(self)
    }
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_xml_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG1: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors json's own `enc_json_path_bin`/`enc_json_snapshot_bin` shape
/// (`🔣️json/…/🧬️mutations/🦀️component.rs`), reusing `store::pack_rt::write_varint_u64` /
/// `store::ByteReader` plus `XmlDiff`'s own `write_str_lp`/`read_str_lp`/`enc_xml_node_bin`/
/// `dec_xml_node_bin`/`enc_declaration_bin`/`dec_declaration_bin` (`../🔺️diff/🦀️component.rs`,
/// `pub(crate)` to this artifact).
use crate::artifacts::xml::schema::diff::{dec_declaration_bin, dec_prolog_bin, dec_xml_node_bin, enc_declaration_bin, enc_prolog_bin, enc_xml_node_bin, read_str_lp, write_str_lp};

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_node_path_bin(p: &XmlNodePath, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.0.len() as u64);
    for index in &p.0 {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_node_path_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlNodePath, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        path.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    Ok(XmlNodePath(path))
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn enc_xml_snapshot_bin(s: &XmlSnapshot, out: &mut Vec<u8>) {
    write_str_lp(out, &s.schema);
    out.push(if s.doc.root.is_some() { 1 } else { 0 });
    if let Some(root) = &s.doc.root {
        enc_xml_node_bin(root, out);
    }
    out.push(if s.doc.doctype.is_some() { 1 } else { 0 });
    if let Some(doctype) = &s.doc.doctype {
        enc_doctype_bin(doctype, out);
    }
    out.push(if s.doc.declaration.is_some() { 1 } else { 0 });
    if let Some(declaration) = &s.doc.declaration {
        enc_declaration_bin(declaration, out);
    }
    enc_prolog_bin(&s.doc.prolog, out);
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn dec_xml_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<XmlSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    let prolog = dec_prolog_bin(reader)?;
    Ok(XmlSnapshot { schema, doc: crate::artifacts::xml::schema::snapshot::XmlDocument { root, doctype, declaration, prolog } })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ P2-FG1: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the `XmlMutation`
/// variant ordinal, in the same 0-7 order `print_xml_mutation`'s own keyword match uses.
impl protocol::OpBinary for XmlMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            XmlMutation::NoMutation => 0,
            XmlMutation::SetSnapshot { .. } => 1,
            XmlMutation::SetDeclaration { .. } => 2,
            XmlMutation::SetDoctype { .. } => 3,
            XmlMutation::InsertElement { .. } => 4,
            XmlMutation::RemoveElement { .. } => 5,
            XmlMutation::SetAttribute { .. } => 6,
            XmlMutation::SetText { .. } => 7,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            XmlMutation::NoMutation => {}
            XmlMutation::SetSnapshot { snapshot } => enc_xml_snapshot_bin(snapshot, &mut out),
            XmlMutation::SetDeclaration { declaration } => {
                out.push(if declaration.is_some() { 1 } else { 0 });
                if let Some(declaration) = declaration {
                    enc_declaration_bin(declaration, &mut out);
                }
            }
            XmlMutation::SetDoctype { doctype } => {
                out.push(if doctype.is_some() { 1 } else { 0 });
                if let Some(doctype) = doctype {
                    enc_doctype_bin(doctype, &mut out);
                }
            }
            XmlMutation::InsertElement { path, index, node } => {
                enc_node_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_xml_node_bin(node, &mut out);
            }
            XmlMutation::RemoveElement { path, index } => {
                enc_node_path_bin(path, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            XmlMutation::SetAttribute { path, name, value } => {
                enc_node_path_bin(path, &mut out);
                write_str_lp(&mut out, name);
                out.push(if value.is_some() { 1 } else { 0 });
                if let Some(value) = value {
                    write_str_lp(&mut out, value);
                }
            }
            XmlMutation::SetText { path, text } => {
                enc_node_path_bin(path, &mut out);
                write_str_lp(&mut out, text);
            }
        }
        Ok(out)
    }

    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes).await;
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().await.map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().await.map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(XmlMutation::NoMutation),
            1 => {
                let snapshot = dec_xml_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(XmlMutation::SetSnapshot { snapshot })
            }
            2 => {
                let has = reader.read_u8().await.map_err(|e| malformed("op declaration presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let declaration = if has != 0 { Some(dec_declaration_bin(&mut reader).map_err(|e| malformed("op declaration", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(XmlMutation::SetDeclaration { declaration })
            }
            3 => {
                let has = reader.read_u8().await.map_err(|e| malformed("op doctype presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let doctype = if has != 0 { Some(dec_doctype_bin(&mut reader).map_err(|e| malformed("op doctype", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(XmlMutation::SetDoctype { doctype })
            }
            4 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let index = reader.read_varint_u64().await.map_err(|e| malformed("op index", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))? as usize;
                let node = dec_xml_node_bin(&mut reader).map_err(|e| malformed("op node", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(XmlMutation::InsertElement { path, index, node })
            }
            5 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let index = reader.read_varint_u64().await.map_err(|e| malformed("op index", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))? as usize;
                Ok(XmlMutation::RemoveElement { path, index })
            }
            6 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let has = reader.read_u8().await.map_err(|e| malformed("op value presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let value = if has != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("op value", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(XmlMutation::SetAttribute { path, name, value })
            }
            7 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let text = read_str_lp(&mut reader).map_err(|e| malformed("op text", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(XmlMutation::SetText { path, text })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG1: representative `XmlMutation` values (every variant, incl. `InsertElement`'s bare
/// `XmlNode` payload and `SetSnapshot`'s full nested-document payload) -- the single source of
/// truth reused by `op_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests, so a new variant only needs
/// adding here once.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<XmlMutation> {
    use crate::artifacts::xml::schema::snapshot::XmlAttr;

    let base = <XmlSnapshot as store::ArtifactDsl>::parse_dsl(r#"<root a="1"><child x="0"/></root>"#).unwrap();
    vec![
        XmlMutation::NoMutation,
        XmlMutation::SetSnapshot { snapshot: base },
        XmlMutation::SetDeclaration { declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }) },
        XmlMutation::SetDeclaration { declaration: None },
        XmlMutation::SetDoctype { doctype: Some("<!DOCTYPE root>".into()) },
        XmlMutation::SetDoctype { doctype: None },
        XmlMutation::InsertElement { path: XmlNodePath(vec![]), index: 1, node: XmlNode::Element { name: "grandchild".into(), attrs: vec![XmlAttr { name: "r".into(), value: "1".into() }], children: vec![] } },
        XmlMutation::RemoveElement { path: XmlNodePath(vec![]), index: 0 },
        XmlMutation::SetAttribute { path: XmlNodePath(vec![0]), name: "width".into(), value: Some("99".into()) },
        XmlMutation::SetAttribute { path: XmlNodePath(vec![0]), name: "width".into(), value: None },
        XmlMutation::SetText { path: XmlNodePath(vec![0]), text: "hello world".into() },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod op_codec_tests {
    use super::*;
    use protocol::OpBinary;

    /// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `XmlMutation` grammar —
    /// exercises every variant incl. `InsertElement`'s bare `XmlNode` payload and `SetSnapshot`'s
    /// full nested-document payload (declaration + doctype + recursive node tree). Reuses
    /// `demo_mutation_cases()` (the single source of truth also consumed by
    /// `⚙️engine/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = XmlMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = XmlMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }
}
//#endregion 🧪️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/retags-the-catalog-revision-and-rewrites-an-item-label/🦀️component.rs"]
mod set_snapshot_retags_the_catalog_revision_and_rewrites_an_item_label;
//#endregion 🧪️FixtureCases
