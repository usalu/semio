//! 🧬️ SvgMutation — document mutation dispatch. Every variant's `diff()` is handcrafted (never
//! apply-and-capture) and every variant's `inverse()` is handcrafted, key/index-aware.

use crate::artifacts::svg::schema::diff::{dec_declaration, dec_doctype, dec_prolog, dec_str, dec_xml_node, decode_option, enc_declaration, enc_doctype, enc_prolog, enc_str, enc_xml_node, encode_option, split_top_level, strip_brackets};
use crate::artifacts::svg::schema::diff::{dec_declaration_bin, dec_doctype_bin, dec_prolog_bin, dec_xml_node_bin, enc_declaration_bin, enc_doctype_bin, enc_prolog_bin, enc_xml_node_bin, read_str_lp, write_str_lp};
use crate::artifacts::svg::schema::diff::{diff_at_path, diff_set_snapshot, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgChildAdded, SvgChildrenDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::artifacts::svg::schema::snapshot::{element_attr, node_at, parse_transform_list, parse_view_box, transform_list_to_string, view_box_to_string, NodePath, TransformOp, ViewBox};
use crate::artifacts::svg::SvgSnapshot;
use crate::artifacts::xml::schema::snapshot::{XmlDeclaration, XmlDoctype, XmlNode};
use protocol::OpBinary;
use protocol::{Mutation, OpText};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// 📐️ Typed content mutation for `stdio.svg`. Beyond the baseline `{NoMutation, SetSnapshot}`,
/// this is a flagship mutation vocabulary (plan D2) addressing nodes in the persisted
/// `SvgSnapshot.doc` tree by `NodePath` (child-index chain from the root `<svg>` element).
/// `InsertElement`/`RemoveElement`'s `parent` addresses the PARENT element (`index` is the
/// position among the parent's children); every other path-carrying variant's `path` addresses
/// the target node itself.
/// 🧪️ F6-PILOT CONFIRMED: `#[derive(dsl::DslOps)]` on this enum ALSO fails (independent
/// confirmation beyond `SvgDiff`'s `DiffCodec` blocker) — `SetSnapshot{snapshot: SvgSnapshot}`
/// recursively contains `XmlNode` (no `DslField`, same reason as `SvgDiff`), and `InsertElement`'s
/// `node: XmlNode` / `SetTransform`'s `transform: Option<Vec<TransformOp>>` carry a
/// data-enum-shaped payload DIRECTLY as a variant field, not just via a nested snapshot. This is
/// the mutation-side twin of the same root cause: the derive requires DslField on every reachable
/// type, and requires it whether the enum-shaped value arrives via a Diff struct field OR a
/// Mutation variant field. `OpText`/`OpBinary` hand-rolled below, reusing `SvgDiff`'s
/// `pub(crate)` grammar primitives (`hex_encode`/`enc_xml_node`/`split_top_level`/...).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum SvgMutation {
    #[default]
    NoMutation,
    SetSnapshot {
        snapshot: SvgSnapshot,
    },
    /// 🏳️ Sets (or, if `None`, clears) the document's XML declaration.
    SetDeclaration {
        declaration: Option<XmlDeclaration>,
    },
    /// 📜️ Sets (or, if `None`, clears) the document's typed doctype declaration.
    SetDoctype {
        doctype: Option<XmlDoctype>,
    },
    /// ➕️ Inserts `node` as child `index` of the element at `parent`.
    InsertElement {
        parent: NodePath,
        index: usize,
        node: XmlNode,
    },
    /// ➖️ Removes child `index` of the element at `parent`.
    RemoveElement {
        parent: NodePath,
        index: usize,
    },
    /// 🏷️ Renames the element at `path`.
    SetElementName {
        path: NodePath,
        name: String,
    },
    /// 🏷️ Sets (or, with `value: None`, removes) attribute `name` on the element at `path`.
    SetAttribute {
        path: NodePath,
        name: String,
        value: Option<String>,
    },
    /// ✍️ Replaces the literal text of the `Text` node at `path`.
    SetText {
        path: NodePath,
        text: String,
    },
    /// 🖼️ Sets (or clears) the typed `viewBox` of the element at `path`.
    SetViewBox {
        path: NodePath,
        view_box: Option<ViewBox>,
    },
    /// 🔄 Sets (or clears) the typed `transform` list of the element at `path`.
    SetTransform {
        path: NodePath,
        transform: Option<Vec<TransformOp>>,
    },
}
//#endregion 🔖️Mutations

//#region 🔖️Apply
/// ▶️ Applies `mutation` to `snapshot`: `let d = mutation.diff(&*snapshot); *snapshot =
/// d.apply(snapshot); d` -- the diff is the single semantics source, never a separate imperative
/// apply path (apply-and-capture is banned).
pub async fn apply_svg_mutation(snapshot: &mut SvgSnapshot, mutation: &SvgMutation) -> protocol::MutationOutcome<SvgDiff> {
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

//#region 🔖️AttributeHelper
/// 🏷️ Shared diff-construction for the three attribute-shaped mutations (`SetAttribute`,
/// `SetViewBox`, `SetTransform` -- the latter two are typed sugar over a single named attribute).
/// Resolves the PRIOR value of `name` on the element addressed by `path` against `base`, then
/// builds the exact `SvgAttributesDiff` entry (`removed`/`modified`/`added`) the transition
/// requires, lowered through `diff_at_path`.
async fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<String>) -> SvgDiff {
    let target = node_at(&base.doc, path).ok();
    let existing = target.and_then(|n| match n {
        XmlNode::Element { attrs, .. } => attrs.iter().find(|a| a.name == name),
        _ => None,
    });
    let attrs_diff = match (existing, value) {
        (Some(_), Some(v)) => SvgAttributesDiff { removed: Vec::new(), modified: vec![SvgAttrModified { name: name.to_string(), value: v }], added: Vec::new() },
        (Some(_), None) => SvgAttributesDiff { removed: vec![name.to_string()], modified: Vec::new(), added: Vec::new() },
        (None, Some(v)) => {
            let next_index = match target {
                Some(XmlNode::Element { attrs, .. }) => attrs.len(),
                _ => 0,
            };
            SvgAttributesDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgAttrAdded { index: next_index, name: name.to_string(), value: v }] }
        }
        (None, None) => SvgAttributesDiff::default(),
    };
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(attrs_diff), children: None }))
}

/// 🔎 Reads the PRIOR value of attribute `name` on the element addressed by `path` in `base`.
fn prior_attribute(base: &SvgSnapshot, path: &[usize], name: &str) -> Option<String> {
    node_at(&base.doc, path).ok().and_then(|n| element_attr(n, name)).map(|s| s.to_string())
}
//#endregion 🔖️AttributeHelper

//#region 🔖️MutationTrait
impl Mutation<SvgSnapshot> for SvgMutation {
    type Diff = SvgDiff;

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        protocol::MutationOutcome::new(match self {
            SvgMutation::NoMutation => SvgDiff::default(),
            SvgMutation::SetSnapshot { snapshot } => diff_set_snapshot(base, snapshot),
            SvgMutation::SetDeclaration { declaration } => SvgDiff { prolog: None, declaration: Some(declaration.clone()), doctype: None, root: None },
            SvgMutation::SetDoctype { doctype } => SvgDiff { prolog: None, declaration: None, doctype: Some(doctype.clone()), root: None },
            SvgMutation::InsertElement { parent, index, node } => diff_at_path(
                parent,
                SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: Vec::new(), added: vec![SvgChildAdded { index: *index, item: node.clone() }] }) }),
            ),
            SvgMutation::RemoveElement { parent, index } => {
                diff_at_path(parent, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: vec![*index], modified: Vec::new(), added: Vec::new() }) }))
            }
            SvgMutation::SetElementName { path, name } => diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: Some(name.clone()), attributes: None, children: None })),
            SvgMutation::SetAttribute { path, name, value } => attribute_diff_at_path(base, path, name, value.clone()),
            SvgMutation::SetText { path, text } => diff_at_path(path, SvgNodeDiff::Text { text: Some(text.clone()) }),
            SvgMutation::SetViewBox { path, view_box } => attribute_diff_at_path(base, path, "viewBox", view_box.as_ref().map(view_box_to_string)),
            SvgMutation::SetTransform { path, transform } => attribute_diff_at_path(base, path, "transform", transform.as_ref().map(|ops| transform_list_to_string(ops))),
        })
    }

    fn inverse(&self, base: &SvgSnapshot) -> Vec<Self> {
        match self {
            SvgMutation::NoMutation => vec![SvgMutation::NoMutation],
            SvgMutation::SetSnapshot { .. } => vec![SvgMutation::SetSnapshot { snapshot: base.clone() }],
            SvgMutation::SetDeclaration { .. } => vec![SvgMutation::SetDeclaration { declaration: base.doc.declaration.clone() }],
            SvgMutation::SetDoctype { .. } => vec![SvgMutation::SetDoctype { doctype: base.doc.doctype.clone() }],
            SvgMutation::InsertElement { parent, index, .. } => vec![SvgMutation::RemoveElement { parent: parent.clone(), index: *index }],
            SvgMutation::RemoveElement { parent, index } => match node_at(&base.doc, parent) {
                Ok(XmlNode::Element { children, .. }) => match children.get(*index) {
                    Some(node) => vec![SvgMutation::InsertElement { parent: parent.clone(), index: *index, node: node.clone() }],
                    None => vec![SvgMutation::NoMutation],
                },
                _ => vec![SvgMutation::NoMutation],
            },
            SvgMutation::SetElementName { path, .. } => {
                let prior = match node_at(&base.doc, path) {
                    Ok(XmlNode::Element { name, .. }) => name.clone(),
                    _ => return vec![SvgMutation::NoMutation],
                };
                vec![SvgMutation::SetElementName { path: path.clone(), name: prior }]
            }
            SvgMutation::SetAttribute { path, name, .. } => {
                vec![SvgMutation::SetAttribute { path: path.clone(), name: name.clone(), value: prior_attribute(base, path, name) }]
            }
            SvgMutation::SetText { path, .. } => {
                let old = match node_at(&base.doc, path) {
                    Ok(XmlNode::Text { text }) => text.clone(),
                    _ => String::new(),
                };
                vec![SvgMutation::SetText { path: path.clone(), text: old }]
            }
            SvgMutation::SetViewBox { path, .. } => {
                let old = prior_attribute(base, path, "viewBox").and_then(|v| parse_view_box(&v).ok());
                vec![SvgMutation::SetViewBox { path: path.clone(), view_box: old }]
            }
            SvgMutation::SetTransform { path, .. } => {
                let old = prior_attribute(base, path, "transform").and_then(|v| parse_transform_list(&v).ok());
                vec![SvgMutation::SetTransform { path: path.clone(), transform: old }]
            }
        }
    }
}
//#endregion 🔖️MutationTrait

//#region OpCodecs
/// 🧪️ F6-PILOT: **hand-rolled** `OpText`/`OpBinary` for `SvgMutation` (`#[derive(dsl::DslOps)]`
/// confirmed rejected above) — reuses `SvgDiff`'s `pub(crate)` grammar primitives
/// (`hex_encode`/`enc_xml_node`/`split_top_level`/`encode_option`/...) rather than duplicating
/// them a second time in this file. Grammar: `keyword arg=value ...` (space-separated, same shape
/// the derive's own handcrafted-wrapper convention uses), one match arm per variant (no
/// `DslVariants` scaffolding available since nothing here derives it).
async fn enc_node_path(p: &NodePath) -> String {
    format!("[{}]", p.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","))
}
async fn dec_node_path(s: &str) -> Result<NodePath, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|s| s.parse().map_err(|e: std::num::ParseIntError| e.to_string())).collect()
}
async fn enc_view_box(v: &ViewBox) -> String {
    format!("[{},{},{},{}]", v.min_x, v.min_y, v.width, v.height)
}
async fn dec_view_box(s: &str) -> Result<ViewBox, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [min_x, min_y, width, height] = parts.as_slice() else { return Err(format!("view box: expected 4 fields, got {}", parts.len())) };
    let f = |s: &str| s.parse::<f64>().map_err(|e| e.to_string());
    Ok(ViewBox { min_x: f(min_x)?, min_y: f(min_y)?, width: f(width)?, height: f(height)? })
}
/// 🔄 `TransformOp` is itself a data-carrying enum (same reason `SvgNodeDiff` needs a hand-rolled
/// codec) — tag-prefixed like `enc_xml_node`: `M[a,b,c,d,e,f]` / `T[x,[y?]]` / `S[x,[y?]]` /
/// `R[angle,[cx,cy]?]` / `X[angle]` (skew-x) / `Y[angle]` (skew-y).
async fn enc_transform_op(t: &TransformOp) -> String {
    let f64_opt = |o: &Option<f64>| encode_option(o, |v| v.to_string());
    match t {
        TransformOp::Matrix { a, b, c, d, e, f } => format!("M[{a},{b},{c},{d},{e},{f}]"),
        TransformOp::Translate { x, y } => format!("T[{x},{}]", f64_opt(y)),
        TransformOp::Scale { x, y } => format!("S[{x},{}]", f64_opt(y)),
        TransformOp::Rotate { angle, center } => format!("R[{angle},{}]", encode_option(center, |(cx, cy)| format!("[{cx},{cy}]"))),
        TransformOp::SkewX { angle } => format!("X[{angle}]"),
        TransformOp::SkewY { angle } => format!("Y[{angle}]"),
    }
}
async fn dec_transform_op(s: &str) -> Result<TransformOp, String> {
    let (tag, rest) = s.split_at(1);
    let f = |s: &str| s.parse::<f64>().map_err(|e: std::num::ParseFloatError| e.to_string());
    let inner = strip_brackets(rest)?;
    match tag {
        "M" => {
            let parts = split_top_level(inner, ',');
            let [a, b, c, d, e, g] = parts.as_slice() else { return Err(format!("matrix: expected 6 fields, got {}", parts.len())) };
            Ok(TransformOp::Matrix { a: f(a)?, b: f(b)?, c: f(c)?, d: f(d)?, e: f(e)?, f: f(g)? })
        }
        "T" => {
            let parts = split_top_level(inner, ',');
            let [x, y] = parts.as_slice() else { return Err(format!("translate: expected 2 fields, got {}", parts.len())) };
            Ok(TransformOp::Translate { x: f(x)?, y: decode_option(y, f)? })
        }
        "S" => {
            let parts = split_top_level(inner, ',');
            let [x, y] = parts.as_slice() else { return Err(format!("scale: expected 2 fields, got {}", parts.len())) };
            Ok(TransformOp::Scale { x: f(x)?, y: decode_option(y, f)? })
        }
        "R" => {
            let parts = split_top_level(inner, ',');
            let [angle, center] = parts.as_slice() else { return Err(format!("rotate: expected 2 fields, got {}", parts.len())) };
            let center = decode_option(center, |s| {
                let cp = split_top_level(strip_brackets(s)?, ',');
                let [cx, cy] = cp.as_slice() else { return Err(format!("rotate center: expected 2 fields, got {}", cp.len())) };
                Ok((f(cx)?, f(cy)?))
            })?;
            Ok(TransformOp::Rotate { angle: f(angle)?, center })
        }
        "X" => Ok(TransformOp::SkewX { angle: f(inner)? }),
        "Y" => Ok(TransformOp::SkewY { angle: f(inner)? }),
        other => Err(format!("transform op: unknown tag {other:?}")),
    }
}
fn enc_transform_list(list: &[TransformOp]) -> String {
    format!("[{}]", list.iter().map(enc_transform_op).collect::<Vec<_>>().join(","))
}
async fn dec_transform_list(s: &str) -> Result<Vec<TransformOp>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(dec_transform_op).collect()
}
pub(crate) async fn enc_svg_snapshot(s: &SvgSnapshot) -> String {
    format!("[{},{},{},{},{}]", enc_str(&s.schema), encode_option(&s.doc.root, enc_xml_node), encode_option(&s.doc.doctype, enc_doctype), encode_option(&s.doc.declaration, enc_declaration), enc_prolog(&s.doc.prolog),)
}
pub(crate) fn dec_svg_snapshot(s: &str) -> Result<SvgSnapshot, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [schema, root, doctype, declaration, prolog] = parts.as_slice() else { return Err(format!("svg snapshot: expected 5 fields, got {}", parts.len())) };
    Ok(SvgSnapshot {
        schema: dec_str(schema)?,
        doc: crate::artifacts::xml::schema::snapshot::XmlDocument { root: decode_option(root, dec_xml_node)?, doctype: decode_option(doctype, dec_doctype)?, declaration: decode_option(declaration, dec_declaration)?, prolog: dec_prolog(prolog)? },
    })
}

fn print_svg_mutation(m: &SvgMutation) -> String {
    match m {
        SvgMutation::NoMutation => "no-mutation".to_string(),
        SvgMutation::SetSnapshot { snapshot } => format!("set-snapshot snapshot={}", enc_svg_snapshot(snapshot)),
        SvgMutation::SetDeclaration { declaration } => format!("set-declaration declaration={}", encode_option(declaration, enc_declaration)),
        SvgMutation::SetDoctype { doctype } => format!("set-doctype doctype={}", encode_option(doctype, enc_doctype)),
        SvgMutation::InsertElement { parent, index, node } => format!("insert-element parent={} index={index} node={}", enc_node_path(parent), enc_xml_node(node)),
        SvgMutation::RemoveElement { parent, index } => format!("remove-element parent={} index={index}", enc_node_path(parent)),
        SvgMutation::SetElementName { path, name } => format!("set-element-name path={} name={}", enc_node_path(path), enc_str(name)),
        SvgMutation::SetAttribute { path, name, value } => format!("set-attribute path={} name={} value={}", enc_node_path(path), enc_str(name), encode_option(value, |v| enc_str(v))),
        SvgMutation::SetText { path, text } => format!("set-text path={} text={}", enc_node_path(path), enc_str(text)),
        SvgMutation::SetViewBox { path, view_box } => format!("set-view-box path={} view-box={}", enc_node_path(path), encode_option(view_box, enc_view_box)),
        SvgMutation::SetTransform { path, transform } => format!("set-transform path={} transform={}", enc_node_path(path), encode_option(transform, |v| enc_transform_list(v))),
    }
}
async fn parse_svg_mutation(line: &str) -> Result<SvgMutation, String> {
    if line == "no-mutation" {
        return Ok(SvgMutation::NoMutation);
    }
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|s| !s.is_empty()).map(|tok| tok.split_once('=').ok_or_else(|| format!("svg mutation: bad arg token {tok:?}"))).collect::<Result<Vec<_>, String>>()?.into_iter().collect();
    let arg = |k: &str| args.get(k).copied().ok_or_else(|| format!("svg mutation: missing arg '{k}' for '{keyword}'"));
    let usize_arg = |k: &str| -> Result<usize, String> { arg(k)?.parse().map_err(|e: std::num::ParseIntError| e.to_string()) };
    match keyword {
        "set-snapshot" => Ok(SvgMutation::SetSnapshot { snapshot: dec_svg_snapshot(arg("snapshot")?)? }),
        "set-declaration" => Ok(SvgMutation::SetDeclaration { declaration: decode_option(arg("declaration")?, dec_declaration)? }),
        "set-doctype" => Ok(SvgMutation::SetDoctype { doctype: decode_option(arg("doctype")?, dec_doctype)? }),
        "insert-element" => Ok(SvgMutation::InsertElement { parent: dec_node_path(arg("parent")?).await?, index: usize_arg("index")?, node: dec_xml_node(arg("node")?)? }),
        "remove-element" => Ok(SvgMutation::RemoveElement { parent: dec_node_path(arg("parent")?).await?, index: usize_arg("index")? }),
        "set-element-name" => Ok(SvgMutation::SetElementName { path: dec_node_path(arg("path")?).await?, name: dec_str(arg("name")?)? }),
        "set-attribute" => Ok(SvgMutation::SetAttribute { path: dec_node_path(arg("path")?).await?, name: dec_str(arg("name")?)?, value: decode_option(arg("value")?, dec_str)? }),
        "set-text" => Ok(SvgMutation::SetText { path: dec_node_path(arg("path")?).await?, text: dec_str(arg("text")?)? }),
        "set-view-box" => Ok(SvgMutation::SetViewBox { path: dec_node_path(arg("path")?).await?, view_box: decode_option(arg("view-box")?, dec_view_box)? }),
        "set-transform" => Ok(SvgMutation::SetTransform { path: dec_node_path(arg("path")?).await?, transform: decode_option(arg("transform")?, dec_transform_list)? }),
        other => Err(format!("svg mutation: unknown keyword {other:?}")),
    }
}

impl OpText for SvgMutation {
    fn print_op(&self) -> String {
        print_svg_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_svg_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
}

//#region 🔖️OpBinaryCodec
/// 🧪️ P2-FG3: real recursive binary primitives backing the upgraded `OpBinary` impl below --
/// mirrors `📰xml`'s own `enc_node_path_bin`/`enc_xml_snapshot_bin` shape
/// (`📰xml/…/🧬️mutations/🦀️component.rs`), reusing `store::pack_rt::write_varint_u64`/
/// `store::ByteReader` plus `SvgDiff`'s own `write_str_lp`/`read_str_lp`/`enc_xml_node_bin`/
/// `dec_xml_node_bin`/`enc_declaration_bin`/`dec_declaration_bin` (`../🔺️diff/🦀️component.rs`,
/// `pub(crate)` to this artifact).
async fn enc_node_path_bin(p: &NodePath, out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, p.len() as u64);
    for index in p {
        store::pack_rt::write_varint_u64(out, *index as u64);
    }
}
fn dec_node_path_bin(reader: &mut store::ByteReader<'_>) -> Result<NodePath, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut path = Vec::with_capacity(count as usize);
    for _ in 0..count {
        path.push(reader.read_varint_u64().map_err(|e| e.to_string())? as usize);
    }
    Ok(path)
}
/// 📐️ `ViewBox` -- four fixed-width LE `f64` fields, in `min_x`/`min_y`/`width`/`height` order.
async fn enc_view_box_bin(v: &ViewBox, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.min_x.to_le_bytes());
    out.extend_from_slice(&v.min_y.to_le_bytes());
    out.extend_from_slice(&v.width.to_le_bytes());
    out.extend_from_slice(&v.height.to_le_bytes());
}
fn dec_view_box_bin(reader: &mut store::ByteReader<'_>) -> Result<ViewBox, String> {
    Ok(ViewBox { min_x: reader.read_f64_le().map_err(|e| e.to_string())?, min_y: reader.read_f64_le().map_err(|e| e.to_string())?, width: reader.read_f64_le().map_err(|e| e.to_string())?, height: reader.read_f64_le().map_err(|e| e.to_string())? })
}
/// 🔄️ `TransformOp` -- 1-byte kind tag (`0`=Matrix/`1`=Translate/`2`=Scale/`3`=Rotate/`4`=SkewX/
/// `5`=SkewY, distinct numbering from the text codec's letter tags) followed by its fixed-width LE
/// `f64` fields; an `Option<f64>`/`Option<(f64,f64)>` slot gets its own presence byte first.
async fn enc_transform_op_bin(t: &TransformOp, out: &mut Vec<u8>) {
    let opt_f64 = |out: &mut Vec<u8>, v: &Option<f64>| {
        out.push(if v.is_some() { 1 } else { 0 });
        if let Some(v) = v {
            out.extend_from_slice(&v.to_le_bytes());
        }
    };
    match t {
        TransformOp::Matrix { a, b, c, d, e, f } => {
            out.push(0);
            for v in [a, b, c, d, e, f] {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        TransformOp::Translate { x, y } => {
            out.push(1);
            out.extend_from_slice(&x.to_le_bytes());
            opt_f64(out, y);
        }
        TransformOp::Scale { x, y } => {
            out.push(2);
            out.extend_from_slice(&x.to_le_bytes());
            opt_f64(out, y);
        }
        TransformOp::Rotate { angle, center } => {
            out.push(3);
            out.extend_from_slice(&angle.to_le_bytes());
            out.push(if center.is_some() { 1 } else { 0 });
            if let Some((cx, cy)) = center {
                out.extend_from_slice(&cx.to_le_bytes());
                out.extend_from_slice(&cy.to_le_bytes());
            }
        }
        TransformOp::SkewX { angle } => {
            out.push(4);
            out.extend_from_slice(&angle.to_le_bytes());
        }
        TransformOp::SkewY { angle } => {
            out.push(5);
            out.extend_from_slice(&angle.to_le_bytes());
        }
    }
}
async fn dec_transform_op_bin(reader: &mut store::ByteReader<'_>) -> Result<TransformOp, String> {
    let opt_f64 = |reader: &mut store::ByteReader<'_>| -> Result<Option<f64>, String> { Ok(if semio_framework_plugin::resolve_ready(reader.read_u8()).map_err(|e| e.to_string())? != 0 { Some(semio_framework_plugin::resolve_ready(reader.read_f64_le()).map_err(|e| e.to_string())?) } else { None }) };
    let tag = reader.read_u8().map_err(|e| e.to_string())?;
    match tag {
        0 => {
            let mut vals = [0.0f64; 6];
            for v in vals.iter_mut() {
                *v = reader.read_f64_le().map_err(|e| e.to_string())?;
            }
            Ok(TransformOp::Matrix { a: vals[0], b: vals[1], c: vals[2], d: vals[3], e: vals[4], f: vals[5] })
        }
        1 => {
            let x = reader.read_f64_le().map_err(|e| e.to_string())?;
            let y = opt_f64(reader)?;
            Ok(TransformOp::Translate { x, y })
        }
        2 => {
            let x = reader.read_f64_le().map_err(|e| e.to_string())?;
            let y = opt_f64(reader)?;
            Ok(TransformOp::Scale { x, y })
        }
        3 => {
            let angle = reader.read_f64_le().map_err(|e| e.to_string())?;
            let center = if reader.read_u8().map_err(|e| e.to_string())? != 0 {
                let cx = reader.read_f64_le().map_err(|e| e.to_string())?;
                let cy = reader.read_f64_le().map_err(|e| e.to_string())?;
                Some((cx, cy))
            } else {
                None
            };
            Ok(TransformOp::Rotate { angle, center })
        }
        4 => Ok(TransformOp::SkewX { angle: reader.read_f64_le().map_err(|e| e.to_string())? }),
        5 => Ok(TransformOp::SkewY { angle: reader.read_f64_le().map_err(|e| e.to_string())? }),
        other => Err(format!("transform op binary: unknown tag {other}")),
    }
}
async fn enc_transform_list_bin(list: &[TransformOp], out: &mut Vec<u8>) {
    store::pack_rt::write_varint_u64(out, list.len() as u64);
    for op in list {
        enc_transform_op_bin(op, out);
    }
}
async fn dec_transform_list_bin(reader: &mut store::ByteReader<'_>) -> Result<Vec<TransformOp>, String> {
    let count = reader.read_varint_u64().map_err(|e| e.to_string())?;
    let mut list = Vec::with_capacity(count as usize);
    for _ in 0..count {
        list.push(dec_transform_op_bin(reader).await?);
    }
    Ok(list)
}
pub(crate) async fn enc_svg_snapshot_bin(s: &SvgSnapshot, out: &mut Vec<u8>) {
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
pub(crate) fn dec_svg_snapshot_bin(reader: &mut store::ByteReader<'_>) -> Result<SvgSnapshot, String> {
    let schema = read_str_lp(reader)?;
    let root = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_xml_node_bin(reader)?) } else { None };
    let doctype = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_doctype_bin(reader)?) } else { None };
    let declaration = if reader.read_u8().map_err(|e| e.to_string())? != 0 { Some(dec_declaration_bin(reader)?) } else { None };
    let prolog = dec_prolog_bin(reader)?;
    Ok(SvgSnapshot { schema, doc: crate::artifacts::xml::schema::snapshot::XmlDocument { root, doctype, declaration, prolog } })
}
//#endregion 🔖️OpBinaryCodec

/// 🧪️ P2-FG3: REAL binary op frame (`format u8 | tag u8 | variant payload`), matching
/// `../💾️binary/📡️component.protocol.semio`'s `header fixed 2` + `chain payload bytes` shape --
/// upgraded from F6's `print_op().into_bytes()` text-as-binary shortcut. `tag` is the
/// `SvgMutation` variant ordinal, in the same 0-10 order `print_svg_mutation`'s own keyword match
/// uses.
impl OpBinary for SvgMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            SvgMutation::NoMutation => 0,
            SvgMutation::SetSnapshot { .. } => 1,
            SvgMutation::SetDeclaration { .. } => 2,
            SvgMutation::SetDoctype { .. } => 3,
            SvgMutation::InsertElement { .. } => 4,
            SvgMutation::RemoveElement { .. } => 5,
            SvgMutation::SetElementName { .. } => 6,
            SvgMutation::SetAttribute { .. } => 7,
            SvgMutation::SetText { .. } => 8,
            SvgMutation::SetViewBox { .. } => 9,
            SvgMutation::SetTransform { .. } => 10,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            SvgMutation::NoMutation => {}
            SvgMutation::SetSnapshot { snapshot } => enc_svg_snapshot_bin(snapshot, &mut out),
            SvgMutation::SetDeclaration { declaration } => {
                out.push(if declaration.is_some() { 1 } else { 0 });
                if let Some(declaration) = declaration {
                    enc_declaration_bin(declaration, &mut out);
                }
            }
            SvgMutation::SetDoctype { doctype } => {
                out.push(if doctype.is_some() { 1 } else { 0 });
                if let Some(doctype) = doctype {
                    enc_doctype_bin(doctype, &mut out);
                }
            }
            SvgMutation::InsertElement { parent, index, node } => {
                enc_node_path_bin(parent, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
                enc_xml_node_bin(node, &mut out);
            }
            SvgMutation::RemoveElement { parent, index } => {
                enc_node_path_bin(parent, &mut out);
                store::pack_rt::write_varint_u64(&mut out, *index as u64);
            }
            SvgMutation::SetElementName { path, name } => {
                enc_node_path_bin(path, &mut out);
                write_str_lp(&mut out, name);
            }
            SvgMutation::SetAttribute { path, name, value } => {
                enc_node_path_bin(path, &mut out);
                write_str_lp(&mut out, name);
                out.push(if value.is_some() { 1 } else { 0 });
                if let Some(value) = value {
                    write_str_lp(&mut out, value);
                }
            }
            SvgMutation::SetText { path, text } => {
                enc_node_path_bin(path, &mut out);
                write_str_lp(&mut out, text);
            }
            SvgMutation::SetViewBox { path, view_box } => {
                enc_node_path_bin(path, &mut out);
                out.push(if view_box.is_some() { 1 } else { 0 });
                if let Some(view_box) = view_box {
                    enc_view_box_bin(view_box, &mut out);
                }
            }
            SvgMutation::SetTransform { path, transform } => {
                enc_node_path_bin(path, &mut out);
                out.push(if transform.is_some() { 1 } else { 0 });
                if let Some(transform) = transform {
                    enc_transform_list_bin(transform, &mut out);
                }
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            0 => Ok(SvgMutation::NoMutation),
            1 => {
                let snapshot = dec_svg_snapshot_bin(&mut reader).map_err(|e| malformed("op snapshot", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(SvgMutation::SetSnapshot { snapshot })
            }
            2 => {
                let has = reader.read_u8().map_err(|e| malformed("op declaration presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let declaration = if has != 0 { Some(dec_declaration_bin(&mut reader).map_err(|e| malformed("op declaration", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(SvgMutation::SetDeclaration { declaration })
            }
            3 => {
                let has = reader.read_u8().map_err(|e| malformed("op doctype presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let doctype = if has != 0 { Some(dec_doctype_bin(&mut reader).map_err(|e| malformed("op doctype", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(SvgMutation::SetDoctype { doctype })
            }
            4 => {
                let parent = dec_node_path_bin(&mut reader).map_err(|e| malformed("op parent", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))? as usize;
                let node = dec_xml_node_bin(&mut reader).map_err(|e| malformed("op node", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(SvgMutation::InsertElement { parent, index, node })
            }
            5 => {
                let parent = dec_node_path_bin(&mut reader).map_err(|e| malformed("op parent", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let index = reader.read_varint_u64().map_err(|e| malformed("op index", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))? as usize;
                Ok(SvgMutation::RemoveElement { parent, index })
            }
            6 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(SvgMutation::SetElementName { path, name })
            }
            7 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let name = read_str_lp(&mut reader).map_err(|e| malformed("op name", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let has = reader.read_u8().map_err(|e| malformed("op value presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let value = if has != 0 { Some(read_str_lp(&mut reader).map_err(|e| malformed("op value", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(SvgMutation::SetAttribute { path, name, value })
            }
            8 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let text = read_str_lp(&mut reader).map_err(|e| malformed("op text", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                Ok(SvgMutation::SetText { path, text })
            }
            9 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let has = reader.read_u8().map_err(|e| malformed("op view_box presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let view_box = if has != 0 { Some(dec_view_box_bin(&mut reader).map_err(|e| malformed("op view_box", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(SvgMutation::SetViewBox { path, view_box })
            }
            10 => {
                let path = dec_node_path_bin(&mut reader).map_err(|e| malformed("op path", semio_framework_plugin::resolve_ready(reader.position()), e))?;
                let has = reader.read_u8().map_err(|e| malformed("op transform presence", semio_framework_plugin::resolve_ready(reader.position()), e.to_string()))?;
                let transform = if has != 0 { Some(dec_transform_list_bin(&mut reader).map_err(|e| malformed("op transform", semio_framework_plugin::resolve_ready(reader.position()), e))?) } else { None };
                Ok(SvgMutation::SetTransform { path, transform })
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion OpCodecs

//#region 🔖️DemoCases
/// 🧪️ P2-FG3: representative `SvgMutation` values (every variant, incl. `InsertElement`'s bare
/// `XmlNode` payload, `SetSnapshot`'s full nested-document payload, and `SetViewBox`/
/// `SetTransform`'s typed geometry payloads) — the single source of truth reused by
/// `op_text_binary_roundtrip_law` below AND by `⚙️engine/🦀️component.rs`'s
/// `ops_grammar_conformance_law`/`protocol_walk_law` conformance tests.
#[cfg(test)]
pub(crate) async fn demo_mutation_cases() -> Vec<SvgMutation> {
    use crate::artifacts::xml::schema::snapshot::XmlAttr;

    let base = SvgSnapshot::import_utf8(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect x="0" y="0" width="5" height="5"/></svg>"#.as_bytes()).unwrap();
    vec![
        SvgMutation::NoMutation,
        SvgMutation::SetSnapshot { snapshot: base.clone() },
        SvgMutation::SetDeclaration { declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }) },
        SvgMutation::SetDeclaration { declaration: None },
        SvgMutation::SetDoctype { doctype: Some("<!DOCTYPE svg>".into()) },
        SvgMutation::SetDoctype { doctype: None },
        SvgMutation::InsertElement { parent: vec![], index: 1, node: XmlNode::Element { name: "circle".into(), attrs: vec![XmlAttr { name: "r".into(), value: "1".into() }], children: vec![] } },
        SvgMutation::RemoveElement { parent: vec![0], index: 2 },
        SvgMutation::SetElementName { path: vec![0], name: "g".into() },
        SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some("99".into()) },
        SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: None },
        SvgMutation::SetText { path: vec![0, 1], text: "hello world".into() },
        SvgMutation::SetViewBox { path: vec![], view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 10.5, height: 20.25 }) },
        SvgMutation::SetViewBox { path: vec![], view_box: None },
        SvgMutation::SetTransform {
            path: vec![],
            transform: Some(vec![
                TransformOp::Matrix { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 5.0, f: -5.0 },
                TransformOp::Translate { x: 1.0, y: Some(2.0) },
                TransformOp::Translate { x: 1.0, y: None },
                TransformOp::Scale { x: 2.0, y: None },
                TransformOp::Rotate { angle: 90.0, center: Some((1.0, 2.0)) },
                TransformOp::Rotate { angle: 90.0, center: None },
                TransformOp::SkewX { angle: 15.0 },
                TransformOp::SkewY { angle: 15.0 },
            ]),
        },
        SvgMutation::SetTransform { path: vec![], transform: None },
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::svg::schema::diff::{SvgChildAdded as SvgChildAddedT, SvgNodeDiff as SvgNodeDiffT};
    use crate::artifacts::svg::schema::snapshot::write_svg_xml;
    use crate::artifacts::xml::schema::snapshot::{XmlAttr, XmlDocument};
    use protocol::command::DiffAlgebra;
    use protocol::{DiffCodec, MutationDiff};

    async fn fixture() -> SvgSnapshot {
        SvgSnapshot::import_utf8(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect x="0" y="0" width="5" height="5"/></svg>"#.as_bytes()).unwrap()
    }

    async fn exact_fixture_bytes() -> Vec<u8> {
        std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../temp/artifacts.svg")).expect("read temp/artifacts.svg")
    }

    async fn exact_fixture() -> SvgSnapshot {
        SvgSnapshot::import_utf8(&exact_fixture_bytes()).expect("import temp/artifacts.svg")
    }

    #[semio_framework_async_macros::async_test]
    async fn insert_then_remove_element_apply_and_inverse() {
        let base = fixture();
        let insert = SvgMutation::InsertElement { parent: vec![], index: 1, node: XmlNode::Element { name: "circle".into(), attrs: vec![XmlAttr { name: "r".into(), value: "1".into() }], children: vec![] } };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &insert);
        match &after.doc.root {
            Some(XmlNode::Element { children, .. }) => assert_eq!(children.len(), 2),
            other => panic!("unexpected root {other:?}"),
        }
        let inverses = Mutation::inverse(&insert, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_svg_mutation(&mut restored, inv);
        }
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn set_attribute_apply_and_inverse_round_trip() {
        let base = fixture();
        let mutation = SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some("99".into()) };
        let diff = Mutation::diff(&mutation, &base);
        let after = <SvgDiff as MutationDiff<SvgSnapshot>>::apply(diff.diff(), &base).expect("diff must apply to base");
        assert_eq!(element_attr(node_at(&after.doc, &[0]).unwrap(), "width"), Some("99"));

        let inverses = Mutation::inverse(&mutation, &base);
        let mut restored = after.clone();
        for inv in &inverses {
            apply_svg_mutation(&mut restored, inv);
        }
        assert_eq!(write_svg_xml(&restored.doc), write_svg_xml(&base.doc));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_view_box_and_set_transform_apply_and_inverse() {
        let base = fixture();
        let vb = SvgMutation::SetViewBox { path: vec![], view_box: Some(ViewBox { min_x: 1.0, min_y: 2.0, width: 3.0, height: 4.0 }) };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &vb);
        assert_eq!(element_attr(node_at(&after.doc, &[]).unwrap(), "viewBox"), Some("1 2 3 4"));
        for inv in Mutation::inverse(&vb, &base) {
            apply_svg_mutation(&mut after, &inv);
        }
        assert_eq!(write_svg_xml(&after.doc), write_svg_xml(&base.doc));

        let tf = SvgMutation::SetTransform { path: vec![0], transform: Some(vec![TransformOp::Translate { x: 2.0, y: None }]) };
        let mut after2 = base.clone();
        apply_svg_mutation(&mut after2, &tf);
        assert_eq!(element_attr(node_at(&after2.doc, &[0]).unwrap(), "transform"), Some("translate(2)"));
        for inv in Mutation::inverse(&tf, &base) {
            apply_svg_mutation(&mut after2, &inv);
        }
        assert_eq!(write_svg_xml(&after2.doc), write_svg_xml(&base.doc));
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_element_inverse_restores_removed_node() {
        let base = fixture();
        let remove = SvgMutation::RemoveElement { parent: vec![], index: 0 };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &remove);
        match &after.doc.root {
            Some(XmlNode::Element { children, .. }) => assert!(children.is_empty()),
            other => panic!("unexpected root {other:?}"),
        }
        for inv in Mutation::inverse(&remove, &base) {
            apply_svg_mutation(&mut after, &inv);
        }
        assert_eq!(write_svg_xml(&after.doc), write_svg_xml(&base.doc));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_element_name_apply_and_inverse() {
        let base = fixture();
        let mutation = SvgMutation::SetElementName { path: vec![0], name: "circle".into() };
        let mut after = base.clone();
        apply_svg_mutation(&mut after, &mutation);
        match node_at(&after.doc, &[0]).unwrap() {
            XmlNode::Element { name, .. } => assert_eq!(name, "circle"),
            other => panic!("unexpected node {other:?}"),
        }
        for inv in Mutation::inverse(&mutation, &base) {
            apply_svg_mutation(&mut after, &inv);
        }
        assert_eq!(write_svg_xml(&after.doc), write_svg_xml(&base.doc));
    }

    //#region 🔖️Fixtures
    /// 🌱 `sweep_a`/`sweep_b`: differ in EVERY mutable field. `declaration`/`doctype` both go
    /// `Some(x) -> None` (tri-state `Some(None)`). `root`'s attrs (name-keyed, so a single triple
    /// can show all three flavors at once) exercise removed+modified+added simultaneously. The
    /// naive positional `between_children` (recipe-specified: pairwise `0..min`, base-tail
    /// removed, other-tail added) can only ever show ONE of {removed-tail, added-tail} per
    /// instance -- so `removed` is exercised at the top-level children triple and `added` at the
    /// nested triple inside the modified child, while that same modified child's OWN diff
    /// (name+attributes+children all `Some`) is the "modified-in-every-field" collection entry.
    async fn sweep_a() -> SvgSnapshot {
        SvgSnapshot {
            schema: crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true) }),
                doctype: Some("<!DOCTYPE svg>".into()),
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "svg".into(),
                    attrs: vec![XmlAttr { name: "keep".into(), value: "k".into() }, XmlAttr { name: "toRemove".into(), value: "r".into() }, XmlAttr { name: "toModify".into(), value: "old".into() }],
                    children: vec![
                        XmlNode::Element { name: "g".into(), attrs: vec![XmlAttr { name: "x".into(), value: "1".into() }], children: vec![XmlNode::Element { name: "rect".into(), attrs: Vec::new(), children: Vec::new() }] },
                        XmlNode::Text { text: "stay".into() },
                        XmlNode::Element { name: "toDrop".into(), attrs: Vec::new(), children: Vec::new() },
                    ],
                }),
            },
        }
    }

    async fn sweep_b() -> SvgSnapshot {
        let snapshot = SvgSnapshot {
            schema: crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: None,
                doctype: None,
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "svgRenamed".into(),
                    attrs: vec![XmlAttr { name: "keep".into(), value: "k".into() }, XmlAttr { name: "toModify".into(), value: "new".into() }, XmlAttr { name: "added".into(), value: "a".into() }],
                    children: vec![
                        XmlNode::Element {
                            name: "gModified".into(),
                            attrs: vec![XmlAttr { name: "x".into(), value: "2".into() }, XmlAttr { name: "y".into(), value: "3".into() }],
                            children: vec![XmlNode::Element { name: "rect".into(), attrs: Vec::new(), children: Vec::new() }, XmlNode::Element { name: "circle".into(), attrs: Vec::new(), children: Vec::new() }],
                        },
                        XmlNode::Text { text: "stay".into() },
                    ],
                }),
            },
        };
        snapshot
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️MutationDiffLaw
    /// 🎯️ `SetAttribute{value: None}`'s round trip below removes/restores `"height"` (the LAST
    /// attribute on `fixture()`'s `rect`), never a middle one -- by design, `SetAttribute` (like
    /// its `📰xml` sibling) is position-agnostic: a re-added attribute is appended, since
    /// attribute order carries no XML/SVG-spec meaning (only round-trip fidelity, per `📰xml`'s
    /// own absorb docs). Removing a MIDDLE attribute and reinstating it via a bare `SetAttribute`
    /// mutation therefore does not restore its original Vec position -- exact positional
    /// restoration in that case is only guaranteed at the DIFF level (`DiffAlgebra::inverse`,
    /// which tracks the true original index via `inverse_attrs_diff`), never at the
    /// mutation-replay level, which is exercised here on the LAST attribute specifically so the
    /// append-on-restore behavior coincides with the original position.
    async fn sample_mutations() -> Vec<SvgMutation> {
        vec![
            SvgMutation::NoMutation,
            SvgMutation::SetSnapshot { snapshot: sweep_b() },
            SvgMutation::SetDeclaration { declaration: Some(XmlDeclaration { version: "1.1".into(), encoding: Some("UTF-8".into()), standalone: Some(false) }) },
            SvgMutation::SetDeclaration { declaration: None },
            SvgMutation::SetDoctype { doctype: Some("<!DOCTYPE foo>".into()) },
            SvgMutation::InsertElement { parent: vec![], index: 1, node: XmlNode::Element { name: "circle".into(), attrs: Vec::new(), children: Vec::new() } },
            SvgMutation::RemoveElement { parent: vec![], index: 0 },
            SvgMutation::SetElementName { path: vec![0], name: "circle".into() },
            SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: Some("99".into()) },
            SvgMutation::SetAttribute { path: vec![0], name: "height".into(), value: None },
            SvgMutation::SetText { path: vec![], text: "hi".into() },
            SvgMutation::SetViewBox { path: vec![], view_box: Some(ViewBox { min_x: 0.0, min_y: 0.0, width: 20.0, height: 20.0 }) },
            SvgMutation::SetTransform { path: vec![0], transform: Some(vec![TransformOp::Scale { x: 2.0, y: None }]) },
        ]
    }

    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law() {
        for mutation in sample_mutations() {
            let base = fixture();
            let diff_direct = Mutation::diff(&mutation, &base);
            let applied_via_diff = MutationDiff::apply(diff_direct.diff(), &base).unwrap();

            let mut via_apply = base.clone();
            let diff_from_apply = apply_svg_mutation(&mut via_apply, &mutation);

            assert_eq!(applied_via_diff, via_apply, "mutation_diff_law: apply mismatch for {mutation:?}");
            assert_eq!(diff_direct, diff_from_apply, "mutation_diff_law: diff mismatch for {mutation:?}");
        }
    }
    //#endregion 🔖️MutationDiffLaw

    //#region 🔖️InverseLaw
    #[semio_framework_async_macros::async_test]
    async fn inverse_law() {
        for mutation in sample_mutations() {
            let base = fixture();

            let mut round_tripped = base.clone();
            apply_svg_mutation(&mut round_tripped, &mutation);
            for inverse_mutation in <SvgMutation as Mutation<SvgSnapshot>>::inverse(&mutation, &base) {
                apply_svg_mutation(&mut round_tripped, &inverse_mutation);
            }
            assert_eq!(round_tripped, base, "inverse_law (mutation-level) failed for {mutation:?}");

            let diff = Mutation::diff(&mutation, &base);
            let next = MutationDiff::apply(diff.diff(), &base).unwrap();
            let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
            let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
            assert_eq!(restored, base, "inverse_law (diff-level) failed for {mutation:?}");
        }
    }

    /// 🎯️ Proves the diff-level half of the position claim in `sample_mutations`'s doc comment:
    /// removing a MIDDLE attribute (`"width"`, not the last) and inverting at the DIFF level (not
    /// via a bare replayed `SetAttribute` mutation) DOES restore its exact original Vec position,
    /// because `inverse_attrs_diff` tracks the true original index directly off `base`.
    #[semio_framework_async_macros::async_test]
    async fn inverse_diff_level_restores_middle_attribute_position() {
        let base = fixture();
        let mutation = SvgMutation::SetAttribute { path: vec![0], name: "width".into(), value: None };
        let diff = Mutation::diff(&mutation, &base);
        let next = MutationDiff::apply(diff.diff(), &base).unwrap();
        assert_eq!(element_attr(node_at(&next.doc, &[0]).unwrap(), "width"), None);

        let inverse_diff = DiffAlgebra::inverse(diff.diff(), &base);
        let restored = MutationDiff::apply(&inverse_diff, &next).unwrap();
        assert_eq!(restored, base, "diff-level inverse must restore the exact original attribute order");
    }
    //#endregion 🔖️InverseLaw

    //#region 🔖️AbsorbLaw
    async fn two_child_root(a_name: &str, b_name: &str) -> SvgSnapshot {
        SvgSnapshot {
            schema: crate::artifacts::svg::STDIO_SVG_DOCUMENT_SCHEMA.into(),
            doc: XmlDocument {
                declaration: None,
                doctype: None,
                prolog: Vec::new(),
                root: Some(XmlNode::Element {
                    name: "svg".into(),
                    attrs: Vec::new(),
                    children: vec![XmlNode::Element { name: a_name.into(), attrs: Vec::new(), children: Vec::new() }, XmlNode::Element { name: b_name.into(), attrs: Vec::new(), children: Vec::new() }],
                }),
            },
        }
    }

    async fn assert_absorb_matches_sequential(base: &SvgSnapshot, d1: &SvgDiff, d2: &SvgDiff) -> SvgDiff {
        let sequential = MutationDiff::apply(d2, &MutationDiff::apply(d1, base).unwrap()).unwrap();
        let mut absorbed = d1.clone();
        MutationDiff::absorb(&mut absorbed, d2.clone());
        assert_eq!(MutationDiff::apply(&absorbed, base).unwrap(), sequential, "absorb_law: apply(absorb(d1,d2), base) != sequential");
        absorbed
    }

    async fn root_children_diff(diff: &SvgDiff) -> &SvgChildrenDiff {
        match diff.root.as_ref().expect("root diff present") {
            SvgNodeDiffT::Element(e) => e.children.as_ref().expect("children diff present"),
            other => panic!("expected element diff, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn absorb_law() {
        // Canonical: Insert(2)+Remove(0) -> {removed:[0], added:[(1,f)]}.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&SvgMutation::RemoveElement { parent: vec![], index: 0 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.removed, vec![0]);
            assert_eq!(triple.added.len(), 1);
            assert_eq!(triple.added[0].index, 1);
            let XmlNode::Element { name, .. } = &triple.added[0].item else { panic!("expected element") };
            assert_eq!(name, "f");
        }

        // Canonical: Insert(2,f)+Insert(2,g) -> both survive.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert_eq!(triple.added.len(), 2, "both inserts must survive absorb, not LWW-clobber");
            let names: Vec<&str> = triple
                .added
                .iter()
                .map(|a| match &a.item {
                    XmlNode::Element { name, .. } => name.as_str(),
                    _ => "",
                })
                .collect();
            assert!(names.contains(&"f"));
            assert!(names.contains(&"g"));
        }

        // Canonical: Insert(1,f)+SetField(1,v) -> patch into the added payload.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 1, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&SvgMutation::SetAttribute { path: vec![1], name: "k".into(), value: Some("v".into()) }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "patch-into-added must not surface as a separate modified entry");
            assert_eq!(triple.added.len(), 1);
            let XmlNode::Element { attrs, .. } = &triple.added[0].item else { panic!("expected element") };
            assert!(attrs.iter().any(|a| a.name == "k" && a.value == "v"));
        }

        // Canonical: Modify+Remove -> the modify is annihilated by the later remove.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&SvgMutation::SetAttribute { path: vec![1], name: "k".into(), value: Some("v".into()) }, &base);
            let mid = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&SvgMutation::RemoveElement { parent: vec![], index: 1 }, &mid);
            let absorbed = assert_absorb_matches_sequential(&base, d1.diff(), d2.diff());
            let triple = root_children_diff(&absorbed);
            assert!(triple.modified.is_empty(), "modify of a since-removed item must not survive absorb");
            assert_eq!(triple.removed, vec![1]);
        }

        // Associativity over a triple.
        {
            let base = two_child_root("a", "b");
            let d1 = Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "f".into(), attrs: Vec::new(), children: Vec::new() } }, &base);
            let mid1 = MutationDiff::apply(d1.diff(), &base).unwrap();
            let d2 = Mutation::diff(&SvgMutation::InsertElement { parent: vec![], index: 2, node: XmlNode::Element { name: "g".into(), attrs: Vec::new(), children: Vec::new() } }, &mid1);
            let mid2 = MutationDiff::apply(d2.diff(), &mid1).unwrap();
            let d3 = Mutation::diff(&SvgMutation::RemoveElement { parent: vec![], index: 0 }, &mid2);
            let sequential = MutationDiff::apply(d3.diff(), &mid2).unwrap();

            let mut left = d1.diff().clone();
            MutationDiff::absorb(&mut left, d2.diff().clone());
            MutationDiff::absorb(&mut left, d3.diff().clone());

            let mut d2_then_d3 = d2.diff().clone();
            MutationDiff::absorb(&mut d2_then_d3, d3.diff().clone());
            let mut right = d1.diff().clone();
            MutationDiff::absorb(&mut right, d2_then_d3);

            assert_eq!(MutationDiff::apply(&left, &base).unwrap(), sequential, "absorb associativity (left) failed");
            assert_eq!(MutationDiff::apply(&right, &base).unwrap(), sequential, "absorb associativity (right) failed");
        }
    }
    //#endregion 🔖️AbsorbLaw

    //#region 🔖️BetweenRoundtripLaw
    #[semio_framework_async_macros::async_test]
    async fn between_roundtrip_law() {
        let a = sweep_a();
        let b = sweep_b();
        assert_eq!(MutationDiff::apply(&<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&a, &b), &a).unwrap(), b);
        assert_eq!(MutationDiff::apply(&<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&b, &a), &b).unwrap(), a);

        let sample = fixture();
        assert_eq!(MutationDiff::apply(&<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&sample, &sample), &sample).unwrap(), sample);

        // "Real" fixture leg: a realistic multi-element SVG doc diffed against a mutated variant.
        let real = SvgSnapshot::import_utf8(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><g id="layer1"><rect x="0" y="0" width="10" height="10"/><circle cx="50" cy="50" r="5"/></g></svg>"#.as_bytes()).unwrap();
        let mut mutated = real.clone();
        apply_svg_mutation(&mut mutated, &SvgMutation::SetAttribute { path: vec![], name: "id".into(), value: Some("root".into()) });
        assert_ne!(real, mutated);
        assert_eq!(MutationDiff::apply(&<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&real, &mutated), &real).unwrap(), mutated);
        assert_eq!(MutationDiff::apply(&<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&mutated, &real), &mutated).unwrap(), real);
    }
    //#endregion 🔖️BetweenRoundtripLaw

    //#region 🔖️CodecRetentionLaw
    #[semio_framework_async_macros::async_test]
    async fn codec_retention_law() {
        let text = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><g id="layer1"><rect x="0" y="0" width="10" height="10"/><circle cx="50" cy="50" r="5"/></g></svg>"#;
        let doc = crate::artifacts::svg::schema::snapshot::parse_svg_xml(text).expect("fixture parses");
        // Documented normal form: attribute/element structure round-trips byte-for-byte since the
        // fixture has no whitespace-sensitive text content, self-closing shorthand, or entities.
        let re_encoded = write_svg_xml(&doc);
        assert_eq!(re_encoded, text);

        let snap = SvgSnapshot::import_utf8(text.as_bytes()).expect("import fixture");
        assert_eq!(snap.doc, doc);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <SvgSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }
    //#endregion 🔖️CodecRetentionLaw

    //#region 🔖️FieldSweep
    /// 🎯️ THE acceptance criterion: `sweep_a`/`sweep_b` differ in every mutable field (see the
    /// fixtures' doc comment for exactly how each collection flavor -- removed/modified/added --
    /// is exercised given the recipe's naive positional `between_children`).
    #[semio_framework_async_macros::async_test]
    async fn field_sweep() {
        let a = sweep_a();
        let b = sweep_b();

        let diff_ab = <SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&a, &b);
        assert_eq!(MutationDiff::apply(&diff_ab, &a).unwrap(), b);
        let diff_ba = <SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&b, &a);
        assert_eq!(MutationDiff::apply(&diff_ba, &b).unwrap(), a);
        assert!(<SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&a, &a).is_empty());

        // Hand-written per-field assertion: every top-level SvgDiff field is populated, and both
        // tri-state scalars exercise `Some(None)`.
        assert_eq!(diff_ab.declaration, Some(None));
        assert_eq!(diff_ab.doctype, Some(None));
        assert!(diff_ab.prolog.is_some());
        assert!(diff_ab.root.is_some());

        let SvgNodeDiffT::Element(root_diff) = diff_ab.root.as_ref().unwrap() else { panic!("expected element diff") };
        assert!(root_diff.name.is_some());
        let attrs_diff = root_diff.attributes.as_ref().expect("attrs diff present");
        assert!(!attrs_diff.removed.is_empty(), "attrs: removed not exercised");
        assert!(!attrs_diff.modified.is_empty(), "attrs: modified not exercised");
        assert!(!attrs_diff.added.is_empty(), "attrs: added not exercised");

        let children_diff = root_diff.children.as_ref().expect("children diff present");
        assert!(!children_diff.removed.is_empty(), "children: removed not exercised");
        assert_eq!(children_diff.modified.len(), 1);
        let modified_entry = &children_diff.modified[0];
        let SvgNodeDiffT::Element(modified_element) = &modified_entry.diff else { panic!("expected element diff") };
        assert!(modified_element.name.is_some(), "modified child: name not exercised");
        assert!(modified_element.attributes.is_some(), "modified child: attributes not exercised");
        let nested_children = modified_element.children.as_ref().expect("nested children diff present");
        let nested_added: &Vec<SvgChildAddedT> = &nested_children.added;
        assert!(!nested_added.is_empty(), "children: added (nested) not exercised");
    }
    //#endregion 🔖️FieldSweep

    /// 🧪️ `OpText`/`OpBinary` round-trip laws for the hand-rolled `SvgMutation` grammar —
    /// exercises every variant incl. `InsertElement`'s bare `XmlNode` payload, `SetSnapshot`'s
    /// full nested-document payload, and `SetViewBox`/`SetTransform`'s typed geometry payloads.
    /// Reuses `demo_mutation_cases()` (the single source of truth also consumed by
    /// `⚙️engine/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`).
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        for mutation in demo_mutation_cases() {
            let printed = mutation.print_op();
            assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SvgMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

            let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
            let decoded = SvgMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
        }
    }

    //#region 🔖️LosslessLogicalState
    #[semio_framework_async_macros::async_test]
    async fn exact_native_direct_pack_and_dsl_roundtrips() {
        let original = exact_fixture_bytes();
        let imported = SvgSnapshot::import_utf8(&original).expect("direct import");
        assert_eq!(imported.export_utf8().expect("direct export"), original);

        let packed = store::ArtifactPack::encode_pack(&imported);
        let unpacked = <SvgSnapshot as store::ArtifactPack>::decode_pack(&packed).expect("pack decode");
        assert_eq!(unpacked, imported);
        assert_eq!(unpacked.export_utf8().expect("pack export"), original);

        let printed = store::ArtifactDsl::print_dsl(&imported);
        let parsed = <SvgSnapshot as store::ArtifactDsl>::parse_dsl(&printed).expect("dsl parse");
        assert_eq!(parsed, imported);
        assert_eq!(parsed.export_utf8().expect("dsl export"), original);

        let xml = crate::artifacts::svg::standards::v1_1::subsets::any::io::export::serializers::artifacts::xml::v1_0::any::serialize(&imported).expect("svg to xml");
        assert_eq!(xml.export_utf8().expect("xml export"), original);
        let restored = crate::artifacts::svg::standards::v1_1::subsets::any::io::import::deserializers::artifacts::xml::v1_0::any::deserialize(&xml).expect("xml to svg");
        assert_eq!(restored.export_utf8().expect("xml bridge export"), original);
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_native_between_noop_inverse_and_absorb_roundtrips() {
        let original = exact_fixture_bytes();
        let imported = exact_fixture();

        let self_diff = <SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&imported, &imported);
        assert!(self_diff.is_empty());
        let after_self = MutationDiff::apply(&self_diff, &imported).unwrap();
        assert_eq!(after_self.export_utf8().expect("self export"), original);

        let mut after_noop = imported.clone();
        apply_svg_mutation(&mut after_noop, &SvgMutation::NoMutation);
        assert_eq!(after_noop.export_utf8().expect("noop export"), original);

        let mutation = SvgMutation::SetAttribute { path: vec![], name: "data-semio-roundtrip".into(), value: Some("changed".into()) };
        let d1 = Mutation::diff(&mutation, &imported);
        let changed = MutationDiff::apply(d1.diff(), &imported).unwrap();
        let changed_bytes = changed.export_utf8().expect("changed export");
        assert_ne!(changed_bytes, original);
        crate::artifacts::svg::schema::snapshot::parse_svg_xml(std::str::from_utf8(&changed_bytes).expect("changed UTF-8")).expect("changed SVG parses");

        let inverse_mutation = Mutation::inverse(&mutation, &imported).into_iter().next().expect("inverse mutation");
        let d2 = Mutation::diff(&inverse_mutation, &changed);
        let restored = MutationDiff::apply(d2.diff(), &changed).unwrap();
        assert_eq!(restored, imported);
        assert_eq!(restored.export_utf8().expect("inverse export"), original);

        let mut absorbed = d1.diff().clone();
        MutationDiff::absorb(&mut absorbed, d2.diff().clone());
        let absorbed_result = MutationDiff::apply(&absorbed, &imported).unwrap();
        assert_eq!(absorbed_result, imported);
        assert_eq!(absorbed_result.export_utf8().expect("absorbed export"), original);
    }

    #[semio_framework_async_macros::async_test]
    async fn exact_native_logical_state_survives_diff_and_set_snapshot_codecs() {
        let original = exact_fixture_bytes();
        let imported = exact_fixture();
        assert!(imported.doc.prolog.iter().any(|node| matches!(node, XmlNode::Comment { .. })));
        let projection = imported.semantic_projection();
        assert_eq!(projection, imported.await);

        let diff = <SvgDiff as DiffAlgebra<SvgSnapshot>>::between(&projection, &imported);
        let text_diff = SvgDiff::parse_diff(&diff.print_diff()).await.expect("diff text decode");
        let binary_diff = SvgDiff::decode_diff(&diff.encode_diff().expect("diff binary encode")).await.expect("diff binary decode");
        assert_eq!(text_diff, diff);
        assert_eq!(binary_diff, diff);
        let restored_projection = MutationDiff::apply(&binary_diff, &projection).unwrap();
        assert_eq!(restored_projection, imported.await);
        assert_eq!(restored_projection.export_utf8().expect("diff export"), original);

        let mutation = SvgMutation::SetSnapshot { snapshot: imported.clone() };
        let text_op = SvgMutation::parse_op(&mutation.print_op()).await.expect("op text decode");
        let binary_op = SvgMutation::decode_op(&mutation.encode_op().await.expect("op binary encode")).await.expect("op binary decode");
        assert_eq!(text_op, mutation);
        assert_eq!(binary_op, mutation);
        let set_snapshot_outcome = Mutation::diff(&binary_op, &projection);
        let applied = MutationDiff::apply(&set_snapshot_outcome.await.diff().await, &projection).unwrap();
        assert_eq!(applied, imported);
        assert_eq!(applied.export_utf8().expect("set snapshot export"), original);
    }
    //#endregion 🔖️LosslessLogicalState
}
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
// 🧪️ Handcrafted mutation fixtures (contract D1, ticket 26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION),
// one case per mutation leaf. Wired HERE and not in `📦️glue.rs`: that file is shared with the
// agents migrating the other stdio artifacts, so the production mounts there stay untouched while
// this artifact owns its own test mount. `#[path = "."]` re-bases the children on this file's own
// directory, which is what makes the leaf-relative path below resolve.
#[cfg(test)]
#[path = "."]
mod fixture_tests {
    #[path = "📄set-snapshot/🧪️tests/recolours-the-circle-fill-to-crimson/🦀️component.rs"]
    mod tests_set_snapshot_recolours_the_circle_fill_to_crimson;
}
//#endregion 🧪️FixtureTests
