//! 🧬️ Curation diff schema — sparse field delta over the artifact.

use crate::{CuratedItem, ObjectKindExtra, CurationSnapshot};
use crate::schema::CurationArtifact;
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse parent delta for catalog identity, sourcing entries and selection.
/// Kit content changes belong to the child's own mutation history.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::schema::CurationArtifact>>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub stock_extra: Option<CurationStockExtraDelta>,
    #[state(artifact)]
    pub curated: Option<CurationCuratedDelta>,
}
impl dsl::FromValue for CurationDiff {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut result = Self::default();
        let mut seen = 0u8;
        for (key, value) in dsl::DslValue::into_object(value)? {
            let bit = match key.as_str() { "artifact" => 1, "catalog" => 2, "stockExtra" => 4, "curated" => 8, _ => return Err(dsl::ValueError::new(format!("unknown Curation diff field {key}"))) };
            if seen & bit != 0 { return Err(dsl::ValueError::new(format!("duplicate Curation diff field {key}"))); }
            seen |= bit;
            match key.as_str() {
                "artifact" => result.artifact = dsl::FromValue::from_value(value)?,
                "catalog" => result.catalog = dsl::FromValue::from_value(value)?,
                "stockExtra" => result.stock_extra = dsl::FromValue::from_value(value)?,
                "curated" => result.curated = dsl::FromValue::from_value(value)?,
                _ => unreachable!(),
            }
        }
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}
impl CurationDiff {
    /// 🛡 Checks typed parent replacements before applying document changes.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if let Some(artifact) = &self.artifact { validate_semio_child_identity(&artifact.catalog.child_id, &artifact.catalog.target, "kit")?; }
        if let Some(catalog) = &self.catalog { validate_semio_child_identity(&catalog.child_id, &catalog.target, "kit")?; }
        Ok(())
    }
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🩹 One patched stock-extra entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationObjectKindExtraPatchEntry {
    pub id: String,
    pub extra: ObjectKindExtra,
}

/// 🧩 Identified-collection delta for `stock_extra`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CurationStockExtraDelta {
    pub added: Vec<ObjectKindExtra>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationObjectKindExtraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationCuratedPatchEntry {
    pub object_id: String,
    pub count: Option<u32>,
}

/// 🧺 Identified-collection delta for `curated`.
#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CurationCuratedDelta {
    pub added: Vec<CuratedItem>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationCuratedPatchEntry>,
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
pub fn apply_stock_extra_delta(stock_extra: &[ObjectKindExtra], delta: &CurationStockExtraDelta) -> protocol::MutationApplyResult<Vec<ObjectKindExtra>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "stock entry is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if !stock_extra.iter().any(|extra| &extra.id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed stock entry does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<_> = stock_extra.iter().map(|extra| extra.id.clone()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, extra) in delta.added.iter().enumerate() {
        if !identities.insert(extra.id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added stock entry identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "stock entry is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.id.as_str()) || !identities.contains(&entry.id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched stock entry does not exist").at(["patched".to_string(), index.to_string()]));
        }
        if entry.extra.id != entry.id {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "stock entry patch cannot change its identity").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<_> = stock_extra.iter().filter(|extra| !removed.contains(extra.id.as_str())).cloned().collect();
    next.extend(delta.added.iter().cloned());
    for entry in &delta.patched {
        let target = next
            .iter_mut()
            .find(|extra| extra.id == entry.id)
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched stock entry does not exist after structural edits").at(["patched".to_string(), entry.id.clone()]))?;
        *target = entry.extra.clone();
    }
    reorder_named(next, delta.reordered.as_deref(), |extra| extra.id.as_str())
}

pub fn apply_curated_delta(curated: &[CuratedItem], delta: &CurationCuratedDelta) -> protocol::MutationApplyResult<Vec<CuratedItem>> {
    let mut removed = std::collections::BTreeSet::new();
    for (index, id) in delta.removed.iter().enumerate() {
        if !removed.insert(id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "curated item is removed more than once").at(["removed".to_string(), index.to_string()]));
        }
        if !curated.iter().any(|item| &item.object_id == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed curated item does not exist").at(["removed".to_string(), index.to_string()]));
        }
    }
    let mut identities: std::collections::BTreeSet<_> = curated.iter().map(|item| item.object_id.clone()).collect();
    for id in &delta.removed {
        identities.remove(id);
    }
    for (index, item) in delta.added.iter().enumerate() {
        if !identities.insert(item.object_id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added curated item identity already exists").at(["added".to_string(), index.to_string()]));
        }
    }
    let mut patched = std::collections::BTreeSet::new();
    for (index, entry) in delta.patched.iter().enumerate() {
        if !patched.insert(entry.object_id.as_str()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "curated item is patched more than once").at(["patched".to_string(), index.to_string()]));
        }
        if removed.contains(entry.object_id.as_str()) || !identities.contains(&entry.object_id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched curated item does not exist").at(["patched".to_string(), index.to_string()]));
        }
    }
    let mut next: Vec<_> = curated.iter().filter(|item| !removed.contains(item.object_id.as_str())).cloned().collect();
    next.extend(delta.added.iter().cloned());
    for entry in &delta.patched {
        let target = next
            .iter_mut()
            .find(|item| item.object_id == entry.object_id)
            .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched curated item does not exist after structural edits").at(["patched".to_string(), entry.object_id.clone()]))?;
        if let Some(count) = entry.count {
            target.count = count;
        }
    }
    reorder_named(next, delta.reordered.as_deref(), |item| item.object_id.as_str())
}

fn reorder_named<T>(items: Vec<T>, order: Option<&[String]>, id: impl for<'a> Fn(&'a T) -> &'a str) -> protocol::MutationApplyResult<Vec<T>> {
    let Some(order) = order else {
        return Ok(items);
    };
    if order.len() != items.len() || order.iter().enumerate().any(|(index, target)| order[..index].contains(target) || !items.iter().any(|item| id(item) == target)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
    }
    let mut remaining = items;
    let mut ordered = Vec::with_capacity(order.len());
    for target in order {
        let index = remaining.iter().position(|item| id(item) == target).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered item does not exist").at(["reordered".to_string(), target.clone()]))?;
        ordered.push(remaining.remove(index));
    }
    Ok(ordered)
}

impl CurationDiff {
    /// 🧬️ Applies sparse document fields onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &CurationArtifact) -> protocol::MutationApplyResult<CurationArtifact> {
        self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.child-identity", message))?;
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(handle) = &self.catalog {
                next.catalog = handle.clone();
            }
            if let Some(delta) = &self.stock_extra {
                next.stock_extra = apply_stock_extra_delta(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
            }
            if let Some(delta) = &self.curated {
                next.curated = apply_curated_delta(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
            }
            next
        })
    }
}

/// 🖼️ Whole-artifact replacement from a snapshot (UI fields defaulted).
pub fn diff_set_snapshot(snapshot: &CurationSnapshot) -> CurationDiff {
    CurationDiff { artifact: Some(Box::new(CurationArtifact::from_snapshot(snapshot.clone()))), ..Default::default() }
}

impl MutationDiff<CurationSnapshot> for CurationDiff {
    fn apply(&self, snapshot: &CurationSnapshot) -> protocol::MutationApplyResult<CurationSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.child-identity", message))?;
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(handle) = &self.catalog {
                next.catalog = handle.clone();
            }
            if let Some(delta) = &self.stock_extra {
                next.stock_extra = apply_stock_extra_delta(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
            }
            if let Some(delta) = &self.curated {
                next.curated = apply_curated_delta(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(catalog);
        match (&mut self.stock_extra, other.stock_extra) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);








                let removed_ids: std::collections::BTreeSet<&str> = dst.removed.iter().map(String::as_str).collect();
                dst.patched.retain(|entry| !removed_ids.contains(entry.id.as_str()));
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.stock_extra = Some(src),
            _ => {}
        }
        match (&mut self.curated, other.curated) {
            (Some(dst), Some(src)) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);

                dst.patched.retain(|entry| !dst.removed.iter().any(|id| id == &entry.object_id));
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            (None, Some(src)) => self.curated = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
