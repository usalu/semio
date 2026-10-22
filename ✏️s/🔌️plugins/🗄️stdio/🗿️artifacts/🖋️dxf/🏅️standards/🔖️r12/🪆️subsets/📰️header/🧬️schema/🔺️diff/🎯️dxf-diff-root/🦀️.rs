//! 🎯️ Root diff of the DXF artifact: the sparse diff record, the target validation, the unchecked apply and the mutation and diff-algebra implementations. See [`super`].

use super::*;

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.dxf`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf.diff")]
pub struct DxfDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub header_vars: Option<DxfHeaderVarsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tables: Option<DxfTablesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blocks: Option<DxfBlocksDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<DxfEntitiesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub other_tables: Option<Vec<DxfOtherTable>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, prefix: &[String]) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        let target = [prefix, &[index.to_string()][..]].concat();
        if index >= base_len || !removed.insert(index) {
            return Err(target_error("mutation.apply.invalid-remove-index", "removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        let target = [prefix, &[index.to_string()][..]].concat();
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(target_error("mutation.apply.invalid-modify-index", "modification target must exist exactly once and remain present", target));
        }
    }
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        let target = [prefix, &[index.to_string()][..]].concat();
        if index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-index", "addition target must be unique and within the evolving sequence", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_named_targets<'a>(
    base_keys: impl IntoIterator<Item = &'a str>,
    removed_keys: impl IntoIterator<Item = &'a str>,
    modified_keys: impl IntoIterator<Item = &'a str>,
    added: impl IntoIterator<Item = (usize, &'a str)>,
    prefix: &[String],
) -> MutationApplyResult<()> {
    let mut occurrences: BTreeMap<&str, usize> = BTreeMap::new();
    for key in base_keys {
        *occurrences.entry(key).or_insert(0) += 1;
    }
    let base_len: usize = occurrences.values().sum();
    let unique = |key: &str| occurrences.get(key) == Some(&1);
    let present = |key: &str| occurrences.contains_key(key);
    let mut removed = BTreeSet::new();
    for key in removed_keys {
        let target = [prefix, &[key.to_string()][..]].concat();
        if !unique(key) || !removed.insert(key) {
            return Err(target_error("mutation.apply.invalid-remove-target", "removal target must exist exactly once", target));
        }
    }
    let mut modified = BTreeSet::new();
    for key in modified_keys {
        let target = [prefix, &[key.to_string()][..]].concat();
        if !unique(key) || removed.contains(key) || !modified.insert(key) {
            return Err(target_error("mutation.apply.invalid-modify-target", "modification target must exist exactly once and remain present", target));
        }
    }
    let mut additions: Vec<(usize, &str)> = added.into_iter().collect();
    additions.sort_by_key(|(index, _)| *index);
    let mut added_keys = BTreeSet::new();
    let mut previous = None;
    for (length, (index, key)) in (base_len - removed.len()..).zip(additions) {
        let target = [prefix, &[key.to_string()][..]].concat();
        if present(key) || !added_keys.insert(key) || index > length || previous == Some(index) {
            return Err(target_error("mutation.apply.invalid-add-target", "addition name and position must be unique and valid", target));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_diff_matches(entity: &DxfEntity, diff: &DxfEntityDiff) -> bool {
    matches!(
        (entity, diff),
        (_, DxfEntityDiff::Replace { .. })
            | (DxfEntity::Line(DxfLine { .. }), DxfEntityDiff::Line(_))
            | (DxfEntity::Circle(DxfCircle { .. }), DxfEntityDiff::Circle(_))
            | (DxfEntity::Arc(DxfArc { .. }), DxfEntityDiff::Arc(_))
            | (DxfEntity::Polyline(DxfPolyline { .. }), DxfEntityDiff::Polyline(_))
            | (DxfEntity::Text(DxfText { .. }), DxfEntityDiff::Text(_))
            | (DxfEntity::Solid(DxfSolid { .. }), DxfEntityDiff::Solid(_))
            | (DxfEntity::Insert(DxfInsert { .. }), DxfEntityDiff::Insert(_))
            | (DxfEntity::Other(DxfOther { .. }), DxfEntityDiff::Other(_))
    )
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_entities_diff(base: &[DxfEntity], diff: &DxfEntitiesDiff, prefix: &[String]) -> MutationApplyResult<()> {
    validate_indexed_targets(base.len(), &diff.removed, diff.modified.iter().map(|entry| entry.index), diff.added.iter().map(|entry| entry.index), prefix)?;
    for entry in &diff.modified {
        if !entity_diff_matches(&base[entry.index], &entry.diff) {
            let target = [prefix, &[entry.index.to_string()][..]].concat();
            return Err(target_error("entity-kind-mismatch", "kind-specific entity diff must match its target entity", target));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_dxf_diff(diff: &DxfDiff, base: &DxfSnapshot) -> MutationApplyResult<()> {
    if let Some(value) = &diff.header_vars {
        validate_named_targets(
            base.header_vars.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.header_var.name.as_str())),
            &["headerVars".to_string()],
        )?;
    }
    if let Some(tables) = &diff.tables {
        if let Some(value) = &tables.layers {
            validate_named_targets(
                base.tables.layers.iter().map(|entry| entry.name.as_str()),
                value.removed.iter().map(String::as_str),
                value.modified.iter().map(|entry| entry.name.as_str()),
                value.added.iter().map(|entry| (entry.index, entry.layer.name.as_str())),
                &["tables".to_string(), "layers".to_string()],
            )?;
        }
        if let Some(value) = &tables.styles {
            validate_named_targets(
                base.tables.styles.iter().map(|entry| entry.name.as_str()),
                value.removed.iter().map(String::as_str),
                value.modified.iter().map(|entry| entry.name.as_str()),
                value.added.iter().map(|entry| (entry.index, entry.style.name.as_str())),
                &["tables".to_string(), "styles".to_string()],
            )?;
        }
        if let Some(value) = &tables.linetypes {
            validate_named_targets(
                base.tables.linetypes.iter().map(|entry| entry.name.as_str()),
                value.removed.iter().map(String::as_str),
                value.modified.iter().map(|entry| entry.name.as_str()),
                value.added.iter().map(|entry| (entry.index, entry.linetype.name.as_str())),
                &["tables".to_string(), "linetypes".to_string()],
            )?;
        }
    }
    if let Some(value) = &diff.blocks {
        validate_indexed_targets(base.blocks.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), &["blocks".to_string()])?;
        for entry in &value.modified {
            if let Some(entities) = &entry.diff.entities {
                validate_entities_diff(&base.blocks[entry.index].entities, entities, &["blocks".to_string(), entry.index.to_string(), "entities".to_string()])?;
            }
        }
    }
    if let Some(value) = &diff.entities {
        validate_entities_diff(&base.entities, value, &["entities".to_string()])?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_dxf_diff_unchecked(diff: &DxfDiff, base: &DxfSnapshot) -> DxfSnapshot {
    DxfSnapshot {
        schema: base.schema.clone(),
        header_vars: diff.header_vars.as_ref().map_or_else(|| base.header_vars.clone(), |value| value.apply(&base.header_vars)),
        tables: diff.tables.as_ref().map_or_else(|| base.tables.clone(), |value| value.apply(&base.tables)),
        other_tables: diff.other_tables.clone().unwrap_or_else(|| base.other_tables.clone()),
        blocks: diff.blocks.as_ref().map_or_else(|| base.blocks.clone(), |value| value.apply(&base.blocks)),
        entities: diff.entities.as_ref().map_or_else(|| base.entities.clone(), |value| value.apply(&base.entities)),
    }
}

impl MutationDiff<DxfSnapshot> for DxfDiff {}

impl protocol::ColdMutationDiff<DxfSnapshot> for DxfDiff {
    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract): every
    /// collection uses its own generic absorb-pair transport; `tables` recurses into its own
    /// three sub-collections.
    fn absorb(&mut self, other: Self) {
        self.header_vars = match (self.header_vars.take(), other.header_vars) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfHeaderVarsDiff::absorb(a, b),
        };
        self.tables = match (self.tables.take(), other.tables) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfTablesDiff::absorb(a, b),
        };
        self.blocks = match (self.blocks.take(), other.blocks) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfBlocksDiff::absorb(a, b),
        };
        self.entities = match (self.entities.take(), other.entities) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfEntitiesDiff::absorb(a, b),
        };
        if other.other_tables.is_some() {
            self.other_tables = other.other_tables;
        }
    }

    fn apply(&self, unpublished: &mut DxfSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        let base = &*unpublished;
        validate_dxf_diff(self, base)?;
        *unpublished = apply_dxf_diff_unchecked(self, base);
        Ok(())
    }
}

impl DiffAlgebra<DxfSnapshot> for DxfDiff {
    fn is_empty(&self) -> bool {
        self.header_vars.as_ref().is_none_or(DxfHeaderVarsDiff::is_empty)
            && self.tables.as_ref().is_none_or(DxfTablesDiff::is_empty)
            && self.blocks.as_ref().is_none_or(DxfBlocksDiff::is_empty)
            && self.entities.as_ref().is_none_or(DxfEntitiesDiff::is_empty)
            && self.other_tables.is_none()
    }
}

impl protocol::ColdDiffAlgebra<DxfSnapshot> for DxfDiff {
    /// ↩️ Concrete diff-level undo: every keyed collection turns into its negative rows against the base (removed rows return at
    /// their base index) and the nested table diffs do the same one level down.
    fn inverse(&self, base: &DxfSnapshot) -> Self {
        DxfDiff {
            header_vars: self.header_vars.as_ref().and_then(|header_vars| header_vars.inverse(&base.header_vars)),
            tables: self.tables.as_ref().and_then(|tables| tables.inverse(&base.tables)),
            blocks: self.blocks.as_ref().and_then(|blocks| blocks.inverse(&base.blocks)),
            entities: self.entities.as_ref().and_then(|entities| entities.inverse(&base.entities)),
            other_tables: self.other_tables.as_ref().filter(|tables| **tables != base.other_tables).map(|_| base.other_tables.clone()),
        }
    }
}
//#endregion 🔖️Diff
