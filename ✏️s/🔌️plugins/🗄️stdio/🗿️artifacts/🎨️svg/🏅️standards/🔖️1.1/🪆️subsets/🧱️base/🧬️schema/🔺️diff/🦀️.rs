//! 🔺️ SvgDiff — handcrafted recursive tree diff over `SvgSnapshot.doc` (an `SvgDocument`).
//! `declaration`/`doctype` are tri-state top-level scalars (`Some(None)` = cleared); `root` nests
//! the recursive `SvgNodeDiff` tree, itself shaped like the `SvgNode` it targets
//! (`SvgNode::Element` <-> `SvgElementDiff`, `SvgNode::Text` <-> `Text{text}`, everything else --
//! CData/Comment/ProcessingInstruction, plus any node-KIND change -- via the `Replace` fallback).
//! Builds on the xml/svg node-diff pattern originated by `📰️xml`'s own `XmlDiff`
//! (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/
//! 🧬️schema-design.md`) but declares its OWN diff types (per the spec-mandated-reuse rule: svg
//! embeds xml's *node* model, never xml's *diff* model).

use crate::SvgSnapshot;
use framework_schema::ArtifactSchema;
use protocol::command::DiffAlgebra;
use protocol::{MutationApplyError, MutationApplyResult, MutationDiff};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDoctype, XmlDtdDeclaration, XmlExternalId, XmlQuote};

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.svg`. No `snapshot: Option<SvgSnapshot>` full-replace slot -- every mutation leaf
/// names exactly the prolog, epilog, declaration, doctype and node rows it changes.
/// 🧪️ F6-PILOT CONFIRMED: `#[derive(dsl::)]` on this struct fails to compile with TWO
/// independent, simultaneous reasons (both captured verbatim, see `f6-recon-report.md`): (1)
/// `root: Option<SvgNodeDiff>` — `SvgNodeDiff` is a genuine data-carrying enum (`Element`/`Text`/
/// `Replace`), and `DslField` has no impl for it (only `DslRecord`-derived structs and
/// `DslScalar`-derived UNIT-only enums implement `DslField`); (2) `declaration`/`doctype` are
/// tri-state `Option<Option<T>>` fields — same blocker as `GifDiff` (see that file). `DiffCodec`
/// is hand-rolled below.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.svg.diff")]
pub struct SvgDiff {
    /// 🧭 Tri-state logical document-prolog nodes.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub prolog: Option<Vec<SvgNode>>,
    /// 🧹 Tri-state logical document-epilog nodes.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub epilog: Option<Vec<SvgNode>>,
    /// 🏳️ Tri-state: `None` = unchanged, `Some(None)` = declaration removed, `Some(Some(d))` = set.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<Option<XmlDeclaration>>,
    /// 📜️ Tri-state: `None` = unchanged, `Some(None)` = doctype removed, `Some(Some(s))` = set.
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub doctype: Option<Option<XmlDoctype>>,
    /// 🌳 `None` = root subtree unchanged; `Some(diff)` = the root changed (recursive, possibly
    /// down to a deeply nested leaf via `diff_at_path`, or a wholesale `Replace` incl. root
    /// presence/absence itself).
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<SvgNodeDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️NodeDiff
/// 🌳 Recursive per-node diff, shaped like the `SvgNode` it targets.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum SvgNodeDiff {
    Element(SvgElementDiff),
    Text {
        #[value(default, skip_serializing_if = "Option::is_none")]
        text: Option<String>,
    },
    /// 🔁 Wholesale node replace -- node-KIND changes (e.g. `Text` -> `Element`, or either endpoint
    /// is `CData`/`Comment`/`ProcessingInstruction`) and, uniquely at the document ROOT, root
    /// presence/absence itself (`node: None` = root removed).
    Replace {
        node: Option<SvgNode>,
    },
}

/// 🏷️ Per-element diff.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgElementDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SvgAttributesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<SvgChildrenDiff>,
}

/// 🏷️ Name-keyed, ORDER-preserving attribute triple. Deliberately a Vec-based triple (not a
/// `HashMap`) -- attribute order carries no SVG/XML-spec meaning but IS significant for
/// byte-preserving round-trips.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgAttributesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<SvgAttrModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<SvgAttrAdded>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgAttrModified {
    pub name: String,
    pub value: crate::schema::snapshot::SvgAttributeValue,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgAttrAdded {
    pub index: usize,
    pub name: String,
    pub value: crate::schema::snapshot::SvgAttributeValue,
}

/// 🌳 Index-keyed, recursive children triple. `removed`/`modified` indices refer to BASE state
/// (descending removal order on apply); `added` indices refer to FINAL state (ascending insert).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgChildrenDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<SvgChildModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<SvgChildAdded>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgChildModified {
    pub index: usize,
    pub diff: SvgNodeDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SvgChildAdded {
    pub index: usize,
    pub item: SvgNode,
}
//#endregion 🔖️NodeDiff

//#region 🔖️DiffAtPath
/// 🧭️ Lowers a `leaf` diff targeting the node addressed by `path` (a chain of child indices from
/// the document root -- mirrors `crate::schema::mutations::NodePath`, kept as a
/// bare `&[usize]` here so this module never needs to depend on the mutations module) into a full
/// `SvgDiff` by nesting it through `SvgChildModified` entries from the root down to that depth.
/// `path == []` addresses the root itself, so `leaf` becomes `SvgDiff.root` directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_path(path: &[usize], leaf: SvgNodeDiff) -> SvgDiff {
    let mut node_diff = leaf;
    for &index in path.iter().rev() {
        node_diff = SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: None, children: Some(SvgChildrenDiff { removed: Vec::new(), modified: vec![SvgChildModified { index, diff: node_diff }], added: Vec::new() }) });
    }
    SvgDiff { prolog: None, epilog: None, declaration: None, doctype: None, root: Some(node_diff) }
}
//#endregion 🔖️DiffAtPath

//#region 🔖️Apply
impl MutationDiff<SvgSnapshot> for SvgDiff {
    fn apply(&self, base: &SvgSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<SvgSnapshot> {
        if let Some(root) = &self.root {
            validate_svg_node(base.doc.root.as_ref(), root)?;
        }
        let mut next = base.clone();
        if let Some(prolog) = &self.prolog {
            next.doc.prolog = prolog.clone();
        }
        if let Some(epilog) = &self.epilog {
            next.doc.epilog = epilog.clone();
        }
        if let Some(declaration) = &self.declaration {
            next.doc.declaration = declaration.clone();
        }
        if let Some(doctype) = &self.doctype {
            next.doc.doctype = doctype.clone();
        }
        if let Some(node_diff) = &self.root {
            next.doc.root = apply_root_diff(next.doc.root.as_ref(), node_diff);
        }
        next.doc.validate_attribute_owners().and_then(|_|next.doc.validate_boundaries())
            .map_err(|detail| MutationApplyError::new("mutation.apply.invalid-document-boundary", detail))?;
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.prolog.is_some() {
            self.prolog = other.prolog;
        }
        if other.epilog.is_some() {
            self.epilog = other.epilog;
        }
        if other.declaration.is_some() {
            self.declaration = other.declaration;
        }
        if other.doctype.is_some() {
            self.doctype = other.doctype;
        }
        self.root = match (self.root.take(), other.root) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_node_diff(a, b)),
        };
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_svg_node(current: Option<&SvgNode>, diff: &SvgNodeDiff) -> MutationApplyResult<()> {
    match diff {
        SvgNodeDiff::Replace { .. } => Ok(()),
        SvgNodeDiff::Text { .. } => match current {
            Some(SvgNode::Text { .. }) => Ok(()),
            Some(_) => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "text diff targets a non-text node")),
            None => Err(MutationApplyError::new("mutation.apply.missing-target", "text diff targets a missing root")),
        },
        SvgNodeDiff::Element(element) => match current {
            Some(SvgNode::Element { attrs, children, .. }) => {
                if let Some(attributes) = &element.attributes {
                    validate_svg_attrs(attrs, attributes)?;
                }
                if let Some(children_diff) = &element.children {
                    validate_svg_children(children, children_diff)?;
                }
                Ok(())
            }
            Some(_) => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "element diff targets a non-element node")),
            None => Err(MutationApplyError::new("mutation.apply.missing-target", "element diff targets a missing root")),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_svg_attrs(base: &[SvgAttr], diff: &SvgAttributesDiff) -> MutationApplyResult<()> {
    for (position, name) in diff.removed.iter().enumerate() {
        if !base.iter().any(|attr| attr.name == *name) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "attribute removal target does not exist"));
        }
        if diff.removed[..position].contains(name) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "attribute removal target is repeated"));
        }
    }
    for (position, modified) in diff.modified.iter().enumerate() {
        if !base.iter().any(|attr| attr.name == modified.name) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "attribute modification target does not exist"));
        }
        if diff.removed.contains(&modified.name) || diff.modified[..position].iter().any(|candidate| candidate.name == modified.name) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "attribute modification target conflicts or repeats"));
        }
    }
    let final_len = base.len() - diff.removed.len() + diff.added.len();
    for (position, added) in diff.added.iter().enumerate() {
        if added.index > final_len
            || diff.added[..position].iter().any(|candidate| candidate.index == added.index)
            || base.iter().any(|attr| attr.name == added.name)
            || diff.removed.contains(&added.name)
            || diff.modified.iter().any(|candidate| candidate.name == added.name)
        {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "attribute addition target is invalid or conflicting"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_svg_children(base: &[SvgNode], diff: &SvgChildrenDiff) -> MutationApplyResult<()> {
    let mut removed = std::collections::HashSet::new();
    for &index in &diff.removed {
        if index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "child removal target does not exist"));
        }
        if !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "child removal target is repeated"));
        }
    }
    let mut modified = std::collections::HashSet::new();
    for entry in &diff.modified {
        if entry.index >= base.len() {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "child modification target does not exist"));
        }
        if removed.contains(&entry.index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "child modification targets a removed node"));
        }
        if !modified.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "child modification target is repeated"));
        }
        validate_svg_node(Some(&base[entry.index]), &entry.diff).map_err(|error| error.under(vec!["modified".to_string(), entry.index.to_string()]))?;
    }
    let final_len = base.len() - removed.len() + diff.added.len();
    let mut added = std::collections::HashSet::new();
    for entry in &diff.added {
        if entry.index > final_len || !added.insert(entry.index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "child addition position is invalid or repeated"));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_root_diff(current: Option<&SvgNode>, diff: &SvgNodeDiff) -> Option<SvgNode> {
    match diff {
        SvgNodeDiff::Replace { node } => node.clone(),
        _ => current.map(|n| apply_node_diff(n, diff)),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_node_diff(node: &SvgNode, diff: &SvgNodeDiff) -> SvgNode {
    match diff {
        SvgNodeDiff::Replace { node: replacement } => replacement.clone().unwrap_or_else(|| node.clone()),
        SvgNodeDiff::Text { text } => match node {
            SvgNode::Text { text: current } => SvgNode::Text { text: text.clone().unwrap_or_else(|| current.clone()) },
            other => other.clone(),
        },
        SvgNodeDiff::Element(element_diff) => match node {
            SvgNode::Element { name, attrs, children } => SvgNode::Element {
                name: element_diff.name.clone().unwrap_or_else(|| name.clone()),
                attrs: match &element_diff.attributes {
                    Some(attrs_diff) => apply_attrs_diff(attrs, attrs_diff),
                    None => attrs.clone(),
                },
                children: match &element_diff.children {
                    Some(children_diff) => apply_children_diff(children, children_diff),
                    None => children.clone(),
                },
            },
            other => other.clone(),
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_attrs_diff(attrs: &[SvgAttr], diff: &SvgAttributesDiff) -> Vec<SvgAttr> {
    let mut out: Vec<SvgAttr> = attrs
        .iter()
        .filter(|a| !diff.removed.contains(&a.name))
        .map(|a| match diff.modified.iter().find(|m| m.name == a.name) {
            Some(m) => SvgAttr { name: a.name.clone(), value: m.value.clone() },
            None => a.clone(),
        })
        .collect();
    let mut additions: Vec<&SvgAttrAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, SvgAttr { name: add.name.clone(), value: add.value.clone() });
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_children_diff(children: &[SvgNode], diff: &SvgChildrenDiff) -> Vec<SvgNode> {
    let mut slots: Vec<Option<SvgNode>> = children.iter().cloned().map(Some).collect();
    for m in &diff.modified {
        if let Some(Some(node)) = slots.get(m.index) {
            let patched = apply_node_diff(node, &m.diff);
            slots[m.index] = Some(patched);
        }
    }
    let mut removed_sorted = diff.removed.clone();
    removed_sorted.sort_unstable_by(|a, b| b.cmp(a));
    removed_sorted.dedup();
    for idx in removed_sorted {
        if idx < slots.len() {
            slots.remove(idx);
        }
    }
    let mut out: Vec<SvgNode> = slots.into_iter().flatten().collect();
    let mut additions: Vec<&SvgChildAdded> = diff.added.iter().collect();
    additions.sort_by_key(|a| a.index);
    for add in additions {
        let at = add.index.min(out.len());
        out.insert(at, add.item.clone());
    }
    out
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SvgSnapshot> for SvgDiff {
    fn inverse(&self, base: &SvgSnapshot) -> Self {
        SvgDiff {
            prolog: self.prolog.as_ref().map(|_| base.doc.prolog.clone()),
            epilog: self.epilog.as_ref().map(|_| base.doc.epilog.clone()),
            declaration: self.declaration.as_ref().map(|_| base.doc.declaration.clone()),
            doctype: self.doctype.as_ref().map(|_| base.doc.doctype.clone()),
            root: self.root.as_ref().map(|d| inverse_node_diff(base.doc.root.as_ref(), d)),
        }
    }

    fn between(base: &SvgSnapshot, other: &SvgSnapshot) -> Self {
        SvgDiff {
            prolog: if base.doc.prolog != other.doc.prolog { Some(other.doc.prolog.clone()) } else { None },
            epilog: if base.doc.epilog != other.doc.epilog { Some(other.doc.epilog.clone()) } else { None },
            declaration: if base.doc.declaration != other.doc.declaration { Some(other.doc.declaration.clone()) } else { None },
            doctype: if base.doc.doctype != other.doc.doctype { Some(other.doc.doctype.clone()) } else { None },
            root: between_root(base.doc.root.as_ref(), other.doc.root.as_ref()),
        }
    }

    fn is_empty(&self) -> bool {
        self.prolog.is_none() && self.epilog.is_none() && self.declaration.is_none() && self.doctype.is_none() && self.root.is_none()
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_node_diff(current: Option<&SvgNode>, diff: &SvgNodeDiff) -> SvgNodeDiff {
    match diff {
        SvgNodeDiff::Replace { .. } => SvgNodeDiff::Replace { node: current.cloned() },
        SvgNodeDiff::Text { .. } => match current {
            Some(SvgNode::Text { text }) => SvgNodeDiff::Text { text: Some(text.clone()) },
            Some(other) => SvgNodeDiff::Replace { node: Some(other.clone()) },
            None => SvgNodeDiff::Replace { node: None },
        },
        SvgNodeDiff::Element(element_diff) => match current {
            Some(SvgNode::Element { name, attrs, children }) => SvgNodeDiff::Element(SvgElementDiff {
                name: element_diff.name.as_ref().map(|_| name.clone()),
                attributes: element_diff.attributes.as_ref().map(|ad| inverse_attrs_diff(attrs, ad)),
                children: element_diff.children.as_ref().map(|cd| inverse_children_diff(children, cd)),
            }),
            Some(other) => SvgNodeDiff::Replace { node: Some(other.clone()) },
            None => SvgNodeDiff::Replace { node: None },
        },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_attrs_diff(base_attrs: &[SvgAttr], diff: &SvgAttributesDiff) -> SvgAttributesDiff {
    let removed: Vec<String> = diff.added.iter().map(|a| a.name.clone()).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_attrs.iter().find(|a| a.name == m.name) {
            modified.push(SvgAttrModified { name: original.name.clone(), value: original.value.clone() });
        }
    }
    let mut added = Vec::new();
    for name in &diff.removed {
        if let Some(idx) = base_attrs.iter().position(|a| &a.name == name) {
            let original = &base_attrs[idx];
            added.push(SvgAttrAdded { index: idx, name: original.name.clone(), value: original.value.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    SvgAttributesDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_children_diff(base_children: &[SvgNode], diff: &SvgChildrenDiff) -> SvgChildrenDiff {
    let removed: Vec<usize> = diff.added.iter().map(|a| a.index).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_children.get(m.index) {
            let next_index = transform_index(m.index, &diff.removed, &diff.added);
            modified.push(SvgChildModified { index: next_index, diff: inverse_node_diff(Some(original), &m.diff) });
        }
    }
    let mut added = Vec::new();
    for &idx in &diff.removed {
        if let Some(original) = base_children.get(idx) {
            added.push(SvgChildAdded { index: idx, item: original.clone() });
        }
    }
    added.sort_by_key(|a| a.index);
    SvgChildrenDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_root(base: Option<&SvgNode>, other: Option<&SvgNode>) -> Option<SvgNodeDiff> {
    match (base, other) {
        (None, None) => None,
        (None, Some(n)) => Some(SvgNodeDiff::Replace { node: Some(n.clone()) }),
        (Some(_), None) => Some(SvgNodeDiff::Replace { node: None }),
        (Some(b), Some(o)) => between_node(b, o),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_node(base: &SvgNode, other: &SvgNode) -> Option<SvgNodeDiff> {
    if base == other {
        return None;
    }
    match (base, other) {
        (SvgNode::Text { .. }, SvgNode::Text { text: ot }) => Some(SvgNodeDiff::Text { text: Some(ot.clone()) }),
        (SvgNode::Element { name: bn, attrs: ba, children: bc }, SvgNode::Element { name: on, attrs: oa, children: oc }) => {
            let name = if bn != on { Some(on.clone()) } else { None };
            let attributes = between_attrs(ba, oa);
            let children = between_children(bc, oc);
            if name.is_none() && attributes.is_none() && children.is_none() {
                None
            } else {
                Some(SvgNodeDiff::Element(SvgElementDiff { name, attributes, children }))
            }
        }
        _ => Some(SvgNodeDiff::Replace { node: Some(other.clone()) }),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_attrs(base: &[SvgAttr], other: &[SvgAttr]) -> Option<SvgAttributesDiff> {
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        match other.iter().find(|o| o.name == b.name) {
            Some(o) if o.value != b.value => modified.push(SvgAttrModified { name: b.name.clone(), value: o.value.clone() }),
            Some(_) => {}
            None => removed.push(b.name.clone()),
        }
    }
    let mut added = Vec::new();
    for (i, o) in other.iter().enumerate() {
        if !base.iter().any(|b| b.name == o.name) {
            added.push(SvgAttrAdded { index: i, name: o.name.clone(), value: o.value.clone() });
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(SvgAttributesDiff { removed, modified, added })
    }
}

/// 🧮️ Naive positional child diff per the recipe's "between matching" rule for index-keyed
/// collections: pairwise-compare `0..min(base.len(), other.len())` as `modified`, the base tail
/// as `removed`, the other tail as `added`. Not an LCS-based diff (no move/reorder detection).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_children(base: &[SvgNode], other: &[SvgNode]) -> Option<SvgChildrenDiff> {
    let min_len = base.len().min(other.len());
    let mut modified = Vec::new();
    for i in 0..min_len {
        if base[i] != other[i] {
            if let Some(d) = between_node(&base[i], &other[i]) {
                modified.push(SvgChildModified { index: i, diff: d });
            }
        }
    }
    let removed: Vec<usize> = (other.len()..base.len()).collect();
    let added: Vec<SvgChildAdded> = (min_len..other.len()).map(|i| SvgChildAdded { index: i, item: other[i].clone() }).collect();
    if modified.is_empty() && removed.is_empty() && added.is_empty() {
        None
    } else {
        Some(SvgChildrenDiff { removed, modified, added })
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️Absorb
/// 🧮️ Sequential-coalesce absorb per the recipe's normative algorithm (base-free index-transport
/// over `d1`'s removed/added): `transform_index` maps a base-side index through `d1`'s own
/// removed/added to the position it ends up at once `d1` has been applied.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn transform_index(idx: usize, removed: &[usize], added: &[SvgChildAdded]) -> usize {
    let removed_before = removed.iter().filter(|&&r| r < idx).count();
    let pos = idx - removed_before;
    let mut order: Vec<usize> = added.iter().map(|a| a.index).collect();
    order.sort_unstable();
    let mut shift = 0usize;
    for target in order {
        if target <= pos + shift {
            shift += 1;
        } else {
            break;
        }
    }
    pos + shift
}

/// 🏷️ Which base position (survivor) or which `d1.added` slot a mid-array position originated
/// from -- built by `simulate_mid_origins` so `absorb_children_diff` can classify `d2`'s indices.
enum ChildOrigin {
    Base(usize),
    Added(usize),
}

/// 🧱️ Materializes a synthetic mid-array (base -> after `d1`) large enough to answer every index
/// `d1`/`d2` actually reference. Absorb is base-free (no real snapshot access), so `base_len` is
/// the SMALLEST synthetic length that avoids clamping any referenced position.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn simulate_mid_origins(base_len: usize, removed: &[usize], added: &[SvgChildAdded]) -> Vec<ChildOrigin> {
    let mut mid: Vec<ChildOrigin> = (0..base_len).filter(|i| !removed.contains(i)).map(ChildOrigin::Base).collect();
    let mut order: Vec<(usize, usize)> = added.iter().enumerate().map(|(k, a)| (a.index, k)).collect();
    order.sort_by_key(|(idx, _)| *idx);
    for (idx, k) in order {
        let at = idx.min(mid.len());
        mid.insert(at, ChildOrigin::Added(k));
    }
    mid
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_node_diff(a: SvgNodeDiff, b: SvgNodeDiff) -> SvgNodeDiff {
    match (a, b) {
        (_, SvgNodeDiff::Replace { node: Some(n) }) => SvgNodeDiff::Replace { node: Some(n) },
        (SvgNodeDiff::Replace { node: Some(n) }, b) => SvgNodeDiff::Replace { node: Some(apply_node_diff(&n, &b)) },
        (_, SvgNodeDiff::Replace { node: None }) => SvgNodeDiff::Replace { node: None },
        (SvgNodeDiff::Replace { node: None }, _) => SvgNodeDiff::Replace { node: None },
        (SvgNodeDiff::Text { text: ta }, SvgNodeDiff::Text { text: tb }) => SvgNodeDiff::Text { text: tb.or(ta) },
        (SvgNodeDiff::Element(ea), SvgNodeDiff::Element(eb)) => SvgNodeDiff::Element(absorb_element_diff(ea, eb)),
        (_, b) => b,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_element_diff(mut a: SvgElementDiff, b: SvgElementDiff) -> SvgElementDiff {
    if b.name.is_some() {
        a.name = b.name;
    }
    a.attributes = match (a.attributes.take(), b.attributes) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(absorb_attrs_diff(ad, &bd)),
    };
    a.children = match (a.children.take(), b.children) {
        (None, x) => x,
        (x, None) => x,
        (Some(ad), Some(bd)) => Some(absorb_children_diff(ad, &bd)),
    };
    a
}

/// 🏷️ Name-keyed absorb -- attribute NAME (not position) is the stable identity; only
/// `added.index` needs any position bookkeeping, approximated (not fully index-transported like
/// children) since attribute order carries no spec-mandated meaning, only round-trip fidelity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_attrs_diff(mut a: SvgAttributesDiff, b: &SvgAttributesDiff) -> SvgAttributesDiff {
    let a_added_names: std::collections::HashSet<String> = a.added.iter().map(|x| x.name.clone()).collect();
    let mut removed = a.removed.clone();
    let mut annihilated: Vec<String> = Vec::new();
    for name in &b.removed {
        if a_added_names.contains(name) {
            annihilated.push(name.clone());
        } else if !removed.contains(name) {
            removed.push(name.clone());
        }
    }
    a.added.retain(|x| !annihilated.contains(&x.name));
    let mut modified: Vec<SvgAttrModified> = a.modified.into_iter().filter(|m| !removed.contains(&m.name)).collect();
    for bm in &b.modified {
        if let Some(added) = a.added.iter_mut().find(|x| x.name == bm.name) {
            added.value = bm.value.clone();
            continue;
        }
        if removed.contains(&bm.name) {
            continue;
        }
        match modified.iter_mut().find(|m| m.name == bm.name) {
            Some(existing) => existing.value = bm.value.clone(),
            None => modified.push(bm.clone()),
        }
    }
    let mut added = a.added;
    for ba in &b.added {
        match added.iter_mut().find(|x| x.name == ba.name) {
            Some(existing) => {
                existing.value = ba.value.clone();
                existing.index = ba.index;
            }
            None => added.push(ba.clone()),
        }
    }
    added.sort_by_key(|x| x.index);
    SvgAttributesDiff { removed, modified, added }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_children_diff(d1: SvgChildrenDiff, d2: &SvgChildrenDiff) -> SvgChildrenDiff {
    let d1_ref_max = d1.removed.iter().copied().chain(d1.modified.iter().map(|m| m.index)).max();
    let mut base_len = d1_ref_max.map_or(0, |m| m + 1);
    let mid_len_needed_by_d1 = d1.added.iter().map(|a| a.index + 1).max().unwrap_or(0);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < mid_len_needed_by_d1 {
        base_len += 1;
    }
    let d2_ref_max = d2.removed.iter().copied().chain(d2.modified.iter().map(|m| m.index)).max();
    let required_mid_len = d2_ref_max.map_or(0, |m| m + 1);
    while base_len.saturating_sub(d1.removed.len()) + d1.added.len() < required_mid_len {
        base_len += 1;
    }

    let mid = simulate_mid_origins(base_len, &d1.removed, &d1.added);

    let mut removed = d1.removed.clone();
    let mut modified = d1.modified.clone();
    let mut working_added = d1.added;
    let mut annihilated: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &r2 in &d2.removed {
        match mid.get(r2) {
            Some(ChildOrigin::Base(bi)) => {
                if !removed.contains(bi) {
                    removed.push(*bi);
                }
                modified.retain(|m| &m.index != bi);
            }
            Some(ChildOrigin::Added(k)) => {
                annihilated.insert(*k);
            }
            None => {}
        }
    }
    for m2 in &d2.modified {
        match mid.get(m2.index) {
            Some(ChildOrigin::Base(bi)) => {
                if removed.contains(bi) {
                    continue;
                }
                match modified.iter_mut().find(|m| &m.index == bi) {
                    Some(existing) => existing.diff = absorb_node_diff(existing.diff.clone(), m2.diff.clone()),
                    None => modified.push(SvgChildModified { index: *bi, diff: m2.diff.clone() }),
                }
            }
            Some(ChildOrigin::Added(k)) => {
                if annihilated.contains(k) {
                    continue;
                }
                if let Some(add) = working_added.get_mut(*k) {
                    add.item = apply_node_diff(&add.item, &m2.diff);
                }
            }
            None => {}
        }
    }

    let mut added = Vec::new();
    for (k, add) in working_added.into_iter().enumerate() {
        if annihilated.contains(&k) {
            continue;
        }
        let final_index = transform_index(add.index, &d2.removed, &d2.added);
        added.push(SvgChildAdded { index: final_index, item: add.item });
    }
    for a2 in &d2.added {
        added.push(a2.clone());
    }
    added.sort_by_key(|a| a.index);

    SvgChildrenDiff { removed, modified, added }
}
//#endregion 🔖️Absorb


//#region 🔖️HandcraftedDiffCodec
/// 🧪️ F6-PILOT: **hand-rolled** `protocol::DiffCodec` for `SvgDiff` — the template every other
/// enum-shaped-diff artifact's F6 agent copies (svg is the plan's own named proof-of-concept for
/// this path, `SvgNodeDiff` being a real tagged enum in the tree; xml/json/dxf/pdf/md follow the
/// same shape). Same grammar style `GifDiff`'s hand-rolled codec uses (bracket-depth-aware split,
/// hex for strings/bytes, `[0]`/`[1,x]` for `Option<T>`) — see that file's doc comment for the
/// primitive rationale; this file re-derives its own copies of the small helper functions since
/// each hand-rolled codec is self-contained (no shared "hand-roll helpers" module exists yet —
/// flagged as a good future extraction once ≥3 artifacts hand-roll, not worth adding here for one).
//#region 🔖️Primitives
// 🚫️aaaaa️aaaa️a️agion 🔖️BinaryPrimitives
/// 🧪️aaa️a️aregion 🔖️BinaryPrimitives
//#endregion 🔖️Primitives

//#region 🔖️XmlValueCodecs

// 🚫️aa�️️aaaaaa gion 🔖️XmlValueBinaryCodecs

// 🚫️aaaaregion 🔖️XmlValueBinaryCodecs
//#endregion 🔖️XmlValueCodecs

//#region 🔖️DiffValueCodecs
// 🚫️aa� aaagion 🔖️DiffValueBinaryCodecs
/// 🧪️a️aa️aaregion 🔖️DiffValueBinaryCodecs
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
// 🚫️aaprregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️DemoCases
/// 🧪️ P2-FG3: representative `SvgDiff` values (both top-level tri-states, the recursive
/// `Element`/`Text`/`Replace` `SvgNodeDiff` tree, attribute add/remove/modify, nested child
/// add/remove/modify) — the single prolog of truth reused by `diff_codec_text_binary_roundtrip_law`
/// below AND by `⚙️engine/🦀️.rs`'s `diff_grammar_conformance_law`/`protocol_walk_law`
/// conformance tests.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SvgDiff> {
    use crate::schema::snapshot::SvgDocument;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn elem(name: &str, attrs: Vec<(&str, crate::schema::snapshot::SvgAttributeValue)>, children: Vec<SvgNode>) -> SvgNode {
        SvgNode::Element { name: name.to_string(), attrs: attrs.into_iter().map(|(n, v)| SvgAttr { name: n.to_string(), value: v }).collect(), children }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn snapshot(doc: SvgDocument) -> SvgSnapshot {
        SvgSnapshot { doc, ..Default::default() }
    }

    let a = snapshot(SvgDocument {
        root: Some(elem("svg", vec![("width", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:10.0,unit:"".into()}))], vec![elem("rect", vec![("x", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:0.0,unit:"".into()}))], vec![])])),
        doctype: Some(semio_s_artifact_stdio_xml::schema::snapshot::XmlDoctype{name:"svg".into(),..Default::default()}),
        declaration: Some(XmlDeclaration { version: "1.0".into(), encoding: Some("UTF-8".into()), standalone: Some(true), quote: XmlQuote::Single }),
        prolog: Vec::new(),
        epilog: Vec::new(),
    });
    let b = snapshot(SvgDocument { root: Some(elem("svg", vec![("width", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:20.0,unit:"".into()})), ("height", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:30.0,unit:"".into()}))], vec![elem("circle", vec![("r", crate::schema::snapshot::SvgAttributeValue::Length(crate::schema::snapshot::SvgLength{magnitude:5.0,unit:"".into()}))], vec![]), SvgNode::Text { text: "hi".into() }])), doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() });
    let c = snapshot(SvgDocument { root: None, doctype: None, declaration: None, prolog: Vec::new(), epilog: Vec::new() });

    vec![SvgDiff::default(), SvgDiff::between(&a, &b), SvgDiff::between(&b, &a), SvgDiff::between(&a, &c), SvgDiff::between(&c, &a)]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️handcrafted-diff-codec/🦀️.rs"]
mod handcrafted_diff_codec_tests;
//#endregion 🧪️Tests
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::snapshot::SvgAttr;
pub use semio_s_artifact_stdio_xml::schema::snapshot::XmlDeclaration;
pub use crate::schema::snapshot::SvgNode;
//#endregion 🔁️Re-exports
