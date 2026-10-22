//! 🧊️ The aggregate `PlyDiff` with its validation, apply, inverse and the mutation diff builders.

use super::*;

/// 💬 Index-keyed comments triple: removed base indices, replaced comment texts, inserted comments at their final index.
pub type PlyCommentsDiff = IndexedDiff<String, String>;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.ply`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.ply.diff")]
pub struct PlyDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<PlyFormat>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<PlyCommentsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub elements: Option<PlyElementsDiff>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_rows_diff(properties: &[PlyProperty], rows: &[PlyRow], diff: &PlyRowsDiff, prefix: &[String]) -> MutationApplyResult<()> {
    let mut property_positions = BTreeMap::new();
    for (index, property) in properties.iter().enumerate() {
        if property_positions.insert(property.name(), index).is_some() {
            let target = [prefix, &["properties".to_string(), property.name().to_string()][..]].concat();
            return Err(target_error("mutation.apply.duplicate-base-target", "property names must be unique", target));
        }
    }
    let mut removed = BTreeSet::new();
    for &index in &diff.removed {
        let target = [prefix, &["rows".to_string(), index.to_string()][..]].concat();
        if index >= rows.len() || !removed.insert(index) {
            return Err(target_error("mutation.apply.invalid-remove-index", "row removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let row_target = [prefix, &["rows".to_string(), entry.index.to_string()][..]].concat();
        if entry.index >= rows.len() || removed.contains(&entry.index) || !modified.insert(entry.index) {
            return Err(target_error("mutation.apply.invalid-modify-index", "row modification target must exist exactly once and remain present", row_target));
        }
        let mut fields = BTreeSet::new();
        for field in &entry.diff.fields {
            let target = [prefix, &["rows".to_string(), entry.index.to_string(), "fields".to_string(), field.name.clone()][..]].concat();
            let position = property_positions.get(field.name.as_str()).copied();
            if !fields.insert(field.name.as_str()) || position.is_none() || position.is_some_and(|value| value >= rows[entry.index].values.len()) {
                return Err(target_error("invalid-field-target", "row field target must be unique and resolve to an existing cell", target));
            }
        }
    }
    let mut additions: Vec<usize> = diff.added.iter().map(|entry| entry.index).collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (rows.len() - removed.len()..).zip(additions) {
        let target = [prefix, &["rows".to_string(), index.to_string()][..]].concat();
        if index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-index", "row addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_elements_diff(base: &[PlyElement], diff: &PlyElementsDiff) -> MutationApplyResult<()> {
    let mut base_by_name = BTreeMap::new();
    for element in base {
        if base_by_name.insert(element.name.as_str(), element).is_some() {
            return Err(target_error("mutation.apply.duplicate-base-target", "base element names must be unique", vec!["elements".to_string(), element.name.clone()]));
        }
    }
    let mut removed = BTreeSet::new();
    for name in &diff.removed {
        if !base_by_name.contains_key(name.as_str()) || !removed.insert(name.as_str()) {
            return Err(target_error("mutation.apply.invalid-remove-target", "element removal target must exist exactly once", vec!["elements".to_string(), name.clone()]));
        }
    }
    let mut modified = BTreeSet::new();
    for entry in &diff.modified {
        let base_element = base_by_name.get(entry.name.as_str()).copied();
        if base_element.is_none() || removed.contains(entry.name.as_str()) || !modified.insert(entry.name.as_str()) {
            return Err(target_error("mutation.apply.invalid-modify-target", "element modification target must exist exactly once and remain present", vec!["elements".to_string(), entry.name.clone()]));
        }
        if let (Some(rows), Some(element)) = (&entry.diff.rows, base_element) {
            let properties = entry.diff.properties.as_deref().unwrap_or(&element.properties);
            validate_rows_diff(properties, &element.rows, rows, &["elements".to_string(), entry.name.clone()])?;
        }
    }
    let mut additions: Vec<&PlyElementAdded> = diff.added.iter().collect();
    additions.sort_by_key(|entry| entry.index);
    let mut added_names = BTreeSet::new();
    let mut previous = None;
    for (length, entry) in (base.len() - removed.len()..).zip(additions) {
        if base_by_name.contains_key(entry.element.name.as_str()) || !added_names.insert(entry.element.name.as_str()) || entry.index > length || previous == Some(entry.index) {
            return Err(target_error("mutation.apply.invalid-add-target", "element name and position must be unique and valid", vec!["elements".to_string(), entry.element.name.clone()]));
        }
        previous = Some(entry.index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_ply_diff_unchecked(diff: &PlyDiff, base: &PlySnapshot) -> PlySnapshot {
    let mut next = base.clone();
    if let Some(format) = diff.format {
        next.format = format;
    }
    if let Some(comments) = &diff.comments {
        next.comments = apply_indexed(&base.comments, comments, |_, replacement: &String| replacement.clone());
    }
    if let Some(elements) = &diff.elements {
        for modified in &elements.modified {
            for element in &mut next.elements {
                if element.name == modified.name {
                    apply_element_diff(element, &modified.diff);
                }
            }
        }
        let removed: HashSet<&str> = elements.removed.iter().map(String::as_str).collect();
        next.elements.retain(|element| !removed.contains(element.name.as_str()));
        let mut additions: Vec<&PlyElementAdded> = elements.added.iter().collect();
        additions.sort_by_key(|entry| entry.index);
        for entry in additions {
            next.elements.insert(entry.index, entry.element.clone());
        }
    }
    next
}

impl MutationDiff<PlySnapshot> for PlyDiff {}

impl protocol::ColdMutationDiff<PlySnapshot> for PlyDiff {
    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Scalars: LWW.
    /// `elements`: name-keyed transport (no renames — `AddElement`/`RemoveElement` only), one
    /// nested `rows` absorb per surviving modified element.
    fn absorb(&mut self, other: Self) {
        if other.format.is_some() {
            self.format = other.format;
        }
        match (&mut self.comments, other.comments) {
            (Some(existing), Some(other_comments)) => absorb_indexed(existing, other_comments, |a: &mut String, b: String| *a = b, |t: &mut String, d: &String| *t = d.clone()),
            (slot @ None, Some(other_comments)) => *slot = Some(other_comments),
            _ => {}
        }
        self.elements = absorb_elements(self.elements.take(), other.elements);
    }

    fn apply(&self, unpublished: &mut PlySnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        let base = &*unpublished;
        if let Some(diff) = &self.comments {
            validate_indexed(&base.comments, diff, |_, _| Ok(()))?;
        }
        if let Some(diff) = &self.elements {
            validate_elements_diff(&base.elements, diff)?;
        }
        *unpublished = apply_ply_diff_unchecked(self, base);
        Ok(())
    }
}

/// ↩️ Negative rows for one row triple against its BASE rows: added rows become removals at their final index, removed rows return
/// at their base index, and each modified row restores its base cells at the index the row has after the diff.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_rows(properties: &[PlyProperty], diff: &PlyRowsDiff, base: &[PlyRow]) -> PlyRowsDiff {
    let removed_sorted = semio_s_artifact_stdio_contract::ordered_unique(&diff.removed);
    let mut added_final: Vec<usize> = diff.added.iter().map(|added| added.index).collect();
    added_final.sort_unstable();
    let after_index = |index: usize| {
        let survivor = index - removed_sorted.iter().filter(|dropped| **dropped < index).count();
        added_final.iter().fold(survivor, |position, inserted| if *inserted <= position { position + 1 } else { position })
    };
    let restore = |row_diff: &PlyRowDiff, row: &PlyRow| PlyRowDiff {
        fields: row_diff.fields.iter().filter_map(|change| properties.iter().position(|property| property.name() == change.name).and_then(|cell| row.values.get(cell)).map(|value| PlyRowFieldChange { name: change.name.clone(), value: value.clone() })).collect(),
    };
    let mut modified: Vec<PlyRowModified> = diff.modified.iter().filter_map(|row| base.get(row.index).map(|base_row| PlyRowModified { index: after_index(row.index), diff: restore(&row.diff, base_row) })).collect();
    modified.sort_by_key(|row| row.index);
    let added = removed_sorted.iter().filter_map(|index| base.get(*index).map(|row| PlyRowAdded { index: *index, row: row.clone() })).collect();
    PlyRowsDiff { removed: added_final, modified, added }
}

/// ↩️ The element patch restoring exactly what `diff` replaces in the BASE element.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_element_diff(diff: &PlyElementDiff, base: &PlyElement) -> PlyElementDiff {
    PlyElementDiff {
        count: diff.count.map(|_| base.count),
        properties: diff.properties.as_ref().map(|_| base.properties.clone()),
        rows: diff.rows.as_ref().map(|rows| inverse_rows(&base.properties, rows, &base.rows)).filter(|rows| !rows.is_empty()),
    }
}

/// ↩️ Negative rows for the elements triple against its BASE elements; removed elements return at their base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_elements(diff: &PlyElementsDiff, base: &[PlyElement]) -> PlyElementsDiff {
    let mut added_final: Vec<&PlyElementAdded> = diff.added.iter().collect();
    added_final.sort_by_key(|added| added.index);
    let removed = added_final.iter().map(|added| added.element.name.clone()).collect();
    let modified = diff.modified.iter().filter_map(|row| base.iter().find(|element| element.name == row.name).map(|element| PlyElementModified { name: row.name.clone(), diff: inverse_element_diff(&row.diff, element) })).collect();
    let mut restored: Vec<PlyElementAdded> = diff.removed.iter().filter_map(|name| base.iter().position(|element| &element.name == name).map(|index| PlyElementAdded { index, element: base[index].clone() })).collect();
    restored.sort_by_key(|added| added.index);
    PlyElementsDiff { removed, modified, added: restored }
}

impl DiffAlgebra<PlySnapshot> for PlyDiff {
    fn is_empty(&self) -> bool {
        self.format.is_none() && self.comments.is_none() && self.elements.as_ref().is_none_or(PlyElementsDiff::is_empty)
    }
}

impl protocol::ColdDiffAlgebra<PlySnapshot> for PlyDiff {
    /// ↩️ Concrete diff-level undo: the format and comments return to their base values, and the elements triple turns into its
    /// negative rows (added elements become removals, removed elements return at their base index, modified elements restore
    /// their base declaration and rows).
    fn inverse(&self, base: &PlySnapshot) -> Self {
        PlyDiff {
            format: self.format.map(|_| base.format),
            comments: self.comments.as_ref().map(|comments| inverse_indexed(comments, &base.comments, |_, text: &String| text.clone())).filter(|comments| !comments.is_empty()),
            elements: self.elements.as_ref().map(|elements| inverse_elements(elements, &base.elements)).filter(|elements| !elements.is_empty()),
        }
    }
}
//#endregion 🔖️Diff

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_format(format: PlyFormat) -> PlyDiff {
    PlyDiff { format: Some(format), comments: None, elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_comment(index: usize, comment: String) -> PlyDiff {
    PlyDiff { format: None, comments: Some(PlyCommentsDiff { removed: vec![], modified: vec![], added: vec![IndexedAdded { index, item: comment }] }), elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_comment(index: usize) -> PlyDiff {
    PlyDiff { format: None, comments: Some(PlyCommentsDiff { removed: vec![index], modified: vec![], added: vec![] }), elements: None }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_add_element(index: usize, element: PlyElement) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![], modified: vec![], added: vec![PlyElementAdded { index, element }] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_element(name: &str) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![name.to_string()], modified: vec![], added: vec![] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_element_field(name: &str, diff: PlyElementDiff) -> PlyDiff {
    PlyDiff { format: None, comments: None, elements: Some(PlyElementsDiff { removed: vec![], modified: vec![PlyElementModified { name: name.to_string(), diff }], added: vec![] }) }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_insert_row(element_name: &str, index: usize, row: PlyRow) -> PlyDiff {
    diff_element_field(element_name, PlyElementDiff { count: None, properties: None, rows: Some(PlyRowsDiff { removed: vec![], modified: vec![], added: vec![PlyRowAdded { index, row }] }) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_remove_row(element_name: &str, index: usize) -> PlyDiff {
    diff_element_field(element_name, PlyElementDiff { count: None, properties: None, rows: Some(PlyRowsDiff { removed: vec![index], modified: vec![], added: vec![] }) })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_row_property(element_name: &str, row_index: usize, property_name: &str, value: PlyValue) -> PlyDiff {
    diff_element_field(
        element_name,
        PlyElementDiff {
            count: None,
            properties: None,
            rows: Some(PlyRowsDiff { removed: vec![], modified: vec![PlyRowModified { index: row_index, diff: PlyRowDiff { fields: vec![PlyRowFieldChange { name: property_name.to_string(), value }] } }], added: vec![] }),
        },
    )
}
//#endregion 🔖️MutationDiffBuilders
