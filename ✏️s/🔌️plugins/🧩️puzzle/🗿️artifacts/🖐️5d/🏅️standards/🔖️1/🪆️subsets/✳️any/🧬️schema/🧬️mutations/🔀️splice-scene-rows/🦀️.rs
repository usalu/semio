//! 🔀️ Reversible edits of bounded, ordered scene rows.

use crate::standards::v1::subsets::any::schema::diff::*;
use crate::standards::v1::subsets::any::schema::document_records::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dPart};
use crate::Puzzle5dSnapshot;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dParts;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::collections::HashSet;
use std::ops::Range;

pub const SCENE_ROW_CHUNK: usize = 64;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
#[dsl(keyword = "splice-scene-rows")]
pub struct SpliceSceneRows {
    pub table: String,
    pub start: usize,
    pub delete_count: usize,
    #[dsl(table)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub parts: Vec<Puzzle5dPart>,
    #[dsl(table)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub fasteners: Vec<Puzzle5dFastener>,
    #[dsl(table)]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(any(test, feature = "testkit"), serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
}

enum SceneRows<'a> {
    Parts(&'a Puzzle5dParts, &'a [Puzzle5dPart]),
    Fasteners(&'a [Puzzle5dFastener], &'a [Puzzle5dFastener]),
    Compatibility(&'a [Puzzle5dKindCompatibility], &'a [Puzzle5dKindCompatibility]),
}

impl SpliceSceneRows {
    fn rows<'a>(&'a self, base: &'a Puzzle5dSnapshot) -> Result<SceneRows<'a>, &'static str> {
        match self.table.as_str() {
            "parts" if self.fasteners.is_empty() && self.kind_compatibility.is_empty() => Ok(SceneRows::Parts(&base.parts, &self.parts)),
            "fasteners" if self.parts.is_empty() && self.kind_compatibility.is_empty() => Ok(SceneRows::Fasteners(&base.fasteners, &self.fasteners)),
            "kindCompatibility" if self.parts.is_empty() && self.fasteners.is_empty() => Ok(SceneRows::Compatibility(&base.kind_compatibility, &self.kind_compatibility)),
            _ => Err("scene row payload does not match its table"),
        }
    }

    fn range(&self, before: usize, inserted: usize) -> Result<Range<usize>, &'static str> {
        if self.delete_count > SCENE_ROW_CHUNK || inserted > SCENE_ROW_CHUNK {
            return Err("scene row splice exceeds its bounded extent");
        }
        let end = self.start.checked_add(self.delete_count).ok_or("scene row splice overflows its table")?;
        if end > before {
            return Err("scene row splice lies outside its table");
        }
        Ok(self.start..end)
    }

    fn checked_rows<'a>(&'a self, base: &'a Puzzle5dSnapshot) -> Result<(SceneRows<'a>, Range<usize>), &'static str> {
        let rows = self.rows(base)?;
        let range = match rows {
            SceneRows::Parts(before, inserted) => {
                let range = self.range(before.len(), inserted.len())?;
                validate_keys(before.iter(), inserted, &range, |row| row.id.as_str())?;
                if !base.fasteners.is_empty() && !range.is_empty() {
                    let retained = inserted.iter().flat_map(|row| row.grips.iter().map(|grip| format!("{}:{}", row.id, grip.id))).collect::<HashSet<_>>();
                    let removed = range.clone().map(|index| &before[index]).flat_map(|row| row.grips.iter().map(|grip| format!("{}:{}", row.id, grip.id))).filter(|id| !retained.contains(id)).collect::<HashSet<_>>();
                    if base.fasteners.iter().any(|row| removed.contains(&row.source) || removed.contains(&row.target)) {
                        return Err("disconnect fasteners before removing their grips");
                    }
                }
                range
            }
            SceneRows::Fasteners(before, inserted) => {
                let range = self.range(before.len(), inserted.len())?;
                validate_keys(before.iter(), inserted, &range, |row| row.id.as_str())?;
                range
            }
            SceneRows::Compatibility(before, inserted) => {
                let range = self.range(before.len(), inserted.len())?;
                validate_keys(before.iter(), inserted, &range, |row| (row.source.as_str(), row.target.as_str()))?;
                range
            }
        };
        Ok((rows, range))
    }

    fn inverse_rows(&self, rows: SceneRows<'_>, range: Range<usize>) -> Self {
        let mut inverse = Self { table: self.table.clone(), start: self.start, ..Default::default() };
        inverse.delete_count = match rows {
            SceneRows::Parts(before, inserted) => { inverse.parts = range.map(|index| before[index].clone()).collect(); inserted.len() }
            SceneRows::Fasteners(before, inserted) => { inverse.fasteners = before[range].to_vec(); inserted.len() }
            SceneRows::Compatibility(before, inserted) => { inverse.kind_compatibility = before[range].to_vec(); inserted.len() }
        };
        inverse
    }

    /// 🔀️ Applies one validated splice to an unpublished scene already owned by the caller.
    pub(super) fn apply_to_batch(&self, base: &mut Puzzle5dSnapshot, inverse: Option<&mut Vec<Self>>) -> Result<(), &'static str> {
        let (rows, range) = self.checked_rows(base)?;
        if let Some(inverse) = inverse { inverse.push(self.inverse_rows(rows, range.clone())); }
        match self.table.as_str() {
            "parts" => {
                for index in range.clone().rev() { base.parts.remove(index); }
                for (offset, row) in self.parts.iter().cloned().enumerate() { base.parts.insert(range.start + offset, row); }
            }
            "fasteners" => { base.fasteners.splice(range, self.fasteners.iter().cloned()); }
            "kindCompatibility" => { base.kind_compatibility.splice(range, self.kind_compatibility.iter().cloned()); }
            _ => return Err("scene row payload does not match its table"),
        }
        Ok(())
    }
}

fn validate_keys<'a, Row: 'a, Key: Eq + std::hash::Hash>(
    before: impl Iterator<Item = &'a Row>,
    inserted: &'a [Row],
    range: &Range<usize>,
    key: impl Fn(&'a Row) -> Key,
) -> Result<(), &'static str> {
    let mut keys = before.enumerate().filter(|(index, _)| !range.contains(index)).map(|(_, row)| key(row)).collect::<HashSet<_>>();
    if inserted.iter().map(key).all(|key| keys.insert(key)) {
        Ok(())
    } else {
        Err("scene row splice introduces a duplicate key")
    }
}

impl<Op> protocol::MutationKind<Puzzle5dSnapshot, Op> for SpliceSceneRows
where
    Op: protocol::Mutation<Puzzle5dSnapshot, Diff = Puzzle5dDiff> + From<Self>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "scene-rows", kind: "splice-scene-rows", record: "SplicedSceneRows" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Splice scene rows", "Szenenzeilen bearbeiten")
    }

    fn target(&self) -> Vec<String> {
        vec![self.table.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<Puzzle5dSnapshot, Op> for SpliceSceneRows
where
    Op: protocol::ColdMutation<Puzzle5dSnapshot, ReferenceDiff = Puzzle5dDiff> + From<Self>,
{
    fn diff(&self, base: &Puzzle5dSnapshot) -> protocol::MutationOutcome<Puzzle5dDiff> {
        let (rows, range) = match self.checked_rows(base) {
            Ok(value) => value,
            Err(reason) => return protocol::MutationOutcome::fatal("mutation.invariant", reason, vec![self.table.clone()]),
        };
        let mut diff = Puzzle5dDiff::default();
        match rows {
            SceneRows::Parts(before, inserted) => {
                let mut delta = Puzzle5dPartsDelta::removals_by_id(range.clone().map(|index| (before[index].id.clone(), index)).collect());
                delta.inserted = inserted.iter().cloned().enumerate().map(|(offset, row)| Puzzle5dPartInsertion { index: range.start + offset, row }).collect();
                diff.parts = Some(delta);
            }
            SceneRows::Fasteners(before, inserted) => {
                let mut delta = Puzzle5dFastenersDelta::removals_by_id(range.clone().map(|index| (before[index].id.clone(), index)).collect());
                delta.inserted = inserted.iter().cloned().enumerate().map(|(offset, row)| Puzzle5dFastenerInsertion { index: range.start + offset, row }).collect();
                diff.fasteners = Some(delta);
            }
            SceneRows::Compatibility(before, inserted) => {
                let mut delta = Puzzle5dKindCompatibilityDelta::removals_by_id(range.clone().map(|index| {
                    let row = &before[index];
                    (Puzzle5dKindCompatibilityKey { source: row.source.clone(), target: row.target.clone() }, index)
                }).collect());
                delta.inserted = inserted.iter().cloned().enumerate().map(|(offset, row)| Puzzle5dKindCompatibilityInsertion { index: range.start + offset, row }).collect();
                diff.kind_compatibility = Some(delta);
            }
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, ValueError> {
        let (rows, range) = self.checked_rows(base).map_err(|reason| ValueError::literal(ValueRefusalKind::InvalidValue, reason))?;
        Ok(vec![self.inverse_rows(rows, range).into()])
    }
}
