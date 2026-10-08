//! 🧰 Shared attribute diff construction for direct SVG mutations.
use crate::schema::diff::{diff_at_path, SvgAttrAdded, SvgAttrModified, SvgAttributesDiff, SvgDiff, SvgElementDiff, SvgNodeDiff};
use crate::schema::snapshot::{element_attr, node_at, SvgAttr, SvgAttributeValue, SvgNode};
use crate::SvgSnapshot;

/// 🏷️ One attribute transition: the name, the value to hold (`None` removes it) and the position it takes when it is added.
pub type AttributeChange<'a> = (&'a str, Option<SvgAttributeValue>, Option<usize>);

/// 🏷️ The sparse diff that carries the element at `path` through `changes`: unchanged values contribute nothing, a new
/// attribute is added at its position (last by default), and the empty diff results when nothing changes.
pub fn attributes_diff_at_path(base: &SvgSnapshot, path: &[usize], changes: &[AttributeChange<'_>]) -> SvgDiff {
    let existing: &[SvgAttr] = match node_at(&base.doc, path) {
        Ok(SvgNode::Element { attrs, .. }) => attrs.as_slice(),
        _ => &[],
    };
    let present = |name: &str| existing.iter().find(|attribute| attribute.name == name);
    let removed_count = changes.iter().filter(|(name, value, _)| value.is_none() && present(name).is_some()).count();
    let added_count = changes.iter().filter(|(name, value, _)| value.is_some() && present(name).is_none()).count();
    let final_length = existing.len() - removed_count + added_count;
    let mut append_at = existing.len() - removed_count;
    let mut diff = SvgAttributesDiff::default();
    for (name, value, index) in changes {
        match (present(name), value) {
            (Some(current), Some(value)) if current.value != *value => diff.modified.push(SvgAttrModified { name: (*name).to_string(), value: value.clone() }),
            (Some(_), None) => diff.removed.push((*name).to_string()),
            (None, Some(value)) => {
                let at = index.map_or(append_at, |index| index.min(final_length - 1));
                append_at += 1;
                diff.added.push(SvgAttrAdded { index: at, name: (*name).to_string(), value: value.clone() });
            }
            _ => {}
        }
    }
    if diff.removed.is_empty() && diff.modified.is_empty() && diff.added.is_empty() {
        return SvgDiff::default();
    }
    diff_at_path(path, SvgNodeDiff::Element(SvgElementDiff { name: None, attributes: Some(diff), children: None }))
}

/// 🏷️ The sparse diff that sets, adds (at `index`, default last) or removes attribute `name` on the element at `path`.
pub fn attribute_diff_at_path(base: &SvgSnapshot, path: &[usize], name: &str, value: Option<SvgAttributeValue>, index: Option<usize>) -> SvgDiff {
    attributes_diff_at_path(base, path, &[(name, value, index)])
}

/// 🔎️ The current value and position of attribute `name` on the element at `path`, as the `(value, index)` pair a restoring
/// mutation sets back.
pub fn prior_attribute(base: &SvgSnapshot, path: &[usize], name: &str) -> (Option<SvgAttributeValue>, Option<usize>) {
    match node_at(&base.doc, path) {
        Ok(SvgNode::Element { attrs, .. }) => match attrs.iter().position(|attribute| attribute.name == name) {
            Some(position) => (Some(attrs[position].value.clone()), Some(position)),
            None => (None, None),
        },
        _ => (None, None),
    }
}
