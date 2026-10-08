//! 🧬️ Curation diff schema — sparse field delta over the artifact.

use crate::{CuratedItem, ObjectKindExtra, CurationSnapshot};
use protocol::MutationDiff;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse parent delta for catalog identity, sourcing entries and selection.
/// Kit content changes belong to the child's own mutation history.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationDiff {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: Option<store::ArtifactChild<SemioKitSnapshot>>,
    #[state(artifact)]
    pub stock_extra: Option<CurationStockExtraDelta>,
    #[state(artifact)]
    pub curated: Option<CurationCuratedDelta>,
}
impl semio_framework_value::FromValue for CurationDiff {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let mut result = Self::default();
        let mut seen = 0u8;
        for (key, value) in semio_framework_value::DslValue::into_object(value)? {
            let bit = match key.as_str() { "catalog" => 1, "stockExtra" => 2, "curated" => 4, _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Curation diff field {key}"))) };
            if seen & bit != 0 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("duplicate Curation diff field {key}"))); }
            seen |= bit;
            match key.as_str() {
                "catalog" => result.catalog = semio_framework_value::FromValue::from_value(value)?,
                "stockExtra" => result.stock_extra = semio_framework_value::FromValue::from_value(value)?,
                "curated" => result.curated = semio_framework_value::FromValue::from_value(value)?,
                _ => unreachable!(),
            }
        }
        result.validate().map_err(|message| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message))?;
        Ok(result)
    }
}
impl CurationDiff {
    /// 🛡 Checks typed parent replacements before applying document changes.
    pub fn validate(&self) -> Result<(), String> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity;
        if let Some(catalog) = &self.catalog { validate_semio_child_identity(&catalog.child_id, &catalog.target, "kit")?; }
        Ok(())
    }
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🩹 One patched stock-extra entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationObjectKindExtraPatchEntry {
    pub id: String,
    pub extra: ObjectKindExtra,
}

/// 🧩 Identified-collection delta for `stock_extra`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CurationStockExtraDelta {
    pub added: Vec<ObjectKindExtra>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationObjectKindExtraPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched curated entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CurationCuratedPatchEntry {
    pub object_id: String,
    pub count: Option<u32>,
}

/// 🧺 Identified-collection delta for `curated`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct CurationCuratedDelta {
    pub added: Vec<CuratedItem>,
    pub removed: Vec<String>,
    pub patched: Vec<CurationCuratedPatchEntry>,
    pub reordered: Option<Vec<String>>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️Apply
//#region 🧺️KeyedDelta
/// 🧺️ Id-keyed ordered-collection delta (`added`/`removed`/`patched`/`reordered`) and its algebra: apply, composition (create∘delete
/// cancels, delete∘create replaces, patch∘patch composes, patch∘create folds), negative delta and state delta.
pub trait KeyedDelta: Sized {
    type Row: Clone;
    type Patch: Clone;
    fn added(&self) -> &[Self::Row];
    fn removed(&self) -> &[String];
    fn patched(&self) -> &[Self::Patch];
    fn reordered(&self) -> Option<&[String]>;
    fn assemble(added: Vec<Self::Row>, removed: Vec<String>, patched: Vec<Self::Patch>, reordered: Option<Vec<String>>) -> Self;
    fn row_key(row: &Self::Row) -> &str;
    fn patch_key(patch: &Self::Patch) -> &str;
    fn patch_fold(patch: &Self::Patch, row: &mut Self::Row) -> Result<(), protocol::MutationApplyError>;
    fn patch_compose(first: &Self::Patch, later: &Self::Patch) -> Self::Patch;
    fn patch_inverse(patch: &Self::Patch, base: &Self::Row) -> Self::Patch;
    fn patch_between(base: &Self::Row, other: &Self::Row) -> Option<Self::Patch>;
    fn patch_is_empty(patch: &Self::Patch) -> bool;
}

fn keyed_error(code: &str, message: &str, at: [&str; 2]) -> protocol::MutationApplyError {
    protocol::MutationApplyError::new(code, message).at(at)
}

pub fn keyed_apply<D: KeyedDelta>(rows: &[D::Row], delta: &D) -> Result<Vec<D::Row>, protocol::MutationApplyError> {
    let key = D::row_key;
    for (index, id) in delta.removed().iter().enumerate() {
        if delta.removed()[..index].contains(id) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is removed more than once", ["removed", &index.to_string()]));
        }
        if !rows.iter().any(|row| key(row) == id) {
            return Err(keyed_error("mutation.apply.missing-target", "removed row does not exist", ["removed", &index.to_string()]));
        }
    }
    let mut next: Vec<D::Row> = rows.iter().filter(|row| !delta.removed().iter().any(|id| id == key(row))).cloned().collect();
    for (index, row) in delta.added().iter().enumerate() {
        if next.iter().any(|existing| key(existing) == key(row)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "added row identity already exists", ["added", &index.to_string()]));
        }
        next.push(row.clone());
    }
    for (index, patch) in delta.patched().iter().enumerate() {
        if delta.patched()[..index].iter().any(|earlier| D::patch_key(earlier) == D::patch_key(patch)) {
            return Err(keyed_error("mutation.apply.duplicate-target", "row is patched more than once", ["patched", &index.to_string()]));
        }
        let row = next.iter_mut().find(|row| key(row) == D::patch_key(patch)).ok_or_else(|| keyed_error("mutation.apply.missing-target", "patched row does not exist", ["patched", &index.to_string()]))?;
        D::patch_fold(patch, row).map_err(|error| error.under(["patched".to_string(), index.to_string()]))?;
    }
    let Some(order) = delta.reordered() else { return Ok(next) };
    if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| key(row) == id)) {
        return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered".to_string()]));
    }
    Ok(order.iter().filter_map(|id| next.iter().find(|row| key(row) == id).cloned()).collect())
}

fn canonical<D: KeyedDelta>(mut added: Vec<D::Row>, mut removed: Vec<String>, mut patched: Vec<D::Patch>, reordered: Option<Vec<String>>) -> D {
    if let Some(order) = &reordered {
        added.sort_by_key(|row| order.iter().position(|id| id == D::row_key(row)).unwrap_or(usize::MAX));
    }
    let reordered = reordered.filter(|order| {
        let tail = added.len();
        !(order.len() <= tail + 1 && order.len() >= tail && order[order.len() - tail..].iter().map(String::as_str).eq(added.iter().map(D::row_key)))
    });
    removed.sort();
    removed.dedup();
    patched.retain(|patch| !D::patch_is_empty(patch));
    patched.sort_by(|left, right| D::patch_key(left).cmp(D::patch_key(right)));
    D::assemble(added, removed, patched, reordered)
}

/// ➕️ Normal form of `first` then `later`: create∘delete cancels, delete∘create replaces, patch∘patch composes, patch∘create folds.
pub fn keyed_absorb<D: KeyedDelta>(first: &D, later: &D) -> D {
    let mut added: Vec<D::Row> = first.added().to_vec();
    let mut removed: Vec<String> = first.removed().to_vec();
    let mut patched: Vec<D::Patch> = Vec::new();
    for patch in first.patched() {
        match added.iter_mut().find(|row| D::row_key(row) == D::patch_key(patch)) {
            Some(row) => {
                let _ = D::patch_fold(patch, row);
            }
            None => patched.push(patch.clone()),
        }
    }
    for id in later.removed() {
        if let Some(position) = added.iter().position(|row| D::row_key(row) == id) {
            added.remove(position);
        } else {
            patched.retain(|patch| D::patch_key(patch) != id);
            if !removed.contains(id) {
                removed.push(id.clone());
            }
        }
    }
    added.extend(later.added().iter().cloned());
    for patch in later.patched() {
        let key = D::patch_key(patch);
        if let Some(row) = added.iter_mut().find(|row| D::row_key(row) == key) {
            let _ = D::patch_fold(patch, row);
        } else if let Some(existing) = patched.iter_mut().find(|existing| D::patch_key(existing) == key) {
            *existing = D::patch_compose(existing, patch);
        } else {
            patched.push(patch.clone());
        }
    }
    let reordered = match (later.reordered(), first.reordered()) {
        (Some(order), _) => Some(order.to_vec()),
        (None, Some(order)) => Some(order.iter().filter(|id| !later.removed().contains(id)).cloned().chain(later.added().iter().map(|row| D::row_key(row).to_string())).collect()),
        (None, None) => None,
    };
    canonical::<D>(added, removed, patched, reordered)
}

fn ids_after<D: KeyedDelta>(base: &[String], delta: &D) -> Vec<String> {
    match delta.reordered() {
        Some(order) => order.to_vec(),
        None => base.iter().filter(|id| !delta.removed().contains(id)).cloned().chain(delta.added().iter().map(|row| D::row_key(row).to_string())).collect(),
    }
}

/// 🔁️ The negative delta: removes what `delta` added, restores what it removed, undoes its patches, restores the base order.
pub fn keyed_inverse<D: KeyedDelta>(delta: &D, base: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = delta.added().iter().map(|row| D::row_key(row).to_string()).collect();
    let added: Vec<D::Row> = delta.removed().iter().filter_map(|id| base.iter().find(|row| D::row_key(row) == id).cloned()).collect();
    let patched: Vec<D::Patch> = delta
        .patched()
        .iter()
        .filter(|patch| !removed.iter().any(|id| id == D::patch_key(patch)))
        .filter_map(|patch| base.iter().find(|row| D::row_key(row) == D::patch_key(patch)).map(|row| D::patch_inverse(patch, row)))
        .collect();
    let after = ids_after(&base_ids, delta);
    let natural: Vec<String> = after.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != base_ids).then_some(base_ids);
    canonical::<D>(added, removed, patched, reordered)
}

/// 🧭️ The delta turning `base` into `other` (sync/import only).
pub fn keyed_between<D: KeyedDelta>(base: &[D::Row], other: &[D::Row]) -> D {
    let base_ids: Vec<String> = base.iter().map(|row| D::row_key(row).to_string()).collect();
    let other_ids: Vec<String> = other.iter().map(|row| D::row_key(row).to_string()).collect();
    let removed: Vec<String> = base_ids.iter().filter(|id| !other_ids.contains(id)).cloned().collect();
    let added: Vec<D::Row> = other.iter().filter(|row| !base_ids.iter().any(|id| id == D::row_key(row))).cloned().collect();
    let patched: Vec<D::Patch> = other.iter().filter_map(|row| base.iter().find(|candidate| D::row_key(candidate) == D::row_key(row)).and_then(|candidate| D::patch_between(candidate, row))).collect();
    let natural: Vec<String> = base_ids.iter().filter(|id| !removed.contains(id)).cloned().chain(added.iter().map(|row| D::row_key(row).to_string())).collect();
    let reordered = (natural != other_ids).then_some(other_ids);
    canonical::<D>(added, removed, patched, reordered)
}

pub fn keyed_is_empty<D: KeyedDelta>(delta: &D) -> bool {
    delta.added().is_empty() && delta.removed().is_empty() && delta.patched().iter().all(D::patch_is_empty) && delta.reordered().is_none()
}
//#endregion 🧺️KeyedDelta

impl KeyedDelta for CurationStockExtraDelta {
    type Row = ObjectKindExtra;
    type Patch = CurationObjectKindExtraPatchEntry;
    fn added(&self) -> &[ObjectKindExtra] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[CurationObjectKindExtraPatchEntry] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<ObjectKindExtra>, removed: Vec<String>, patched: Vec<CurationObjectKindExtraPatchEntry>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &ObjectKindExtra) -> &str {
        &row.id
    }
    fn patch_key(patch: &CurationObjectKindExtraPatchEntry) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &CurationObjectKindExtraPatchEntry, row: &mut ObjectKindExtra) -> Result<(), protocol::MutationApplyError> {
        if patch.extra.id != patch.id {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-target", "stock entry patch cannot change its identity"));
        }
        *row = patch.extra.clone();
        Ok(())
    }
    fn patch_compose(_first: &CurationObjectKindExtraPatchEntry, later: &CurationObjectKindExtraPatchEntry) -> CurationObjectKindExtraPatchEntry {
        later.clone()
    }
    fn patch_inverse(patch: &CurationObjectKindExtraPatchEntry, base: &ObjectKindExtra) -> CurationObjectKindExtraPatchEntry {
        CurationObjectKindExtraPatchEntry { id: patch.id.clone(), extra: base.clone() }
    }
    fn patch_between(base: &ObjectKindExtra, other: &ObjectKindExtra) -> Option<CurationObjectKindExtraPatchEntry> {
        (base != other).then(|| CurationObjectKindExtraPatchEntry { id: other.id.clone(), extra: other.clone() })
    }
    fn patch_is_empty(_patch: &CurationObjectKindExtraPatchEntry) -> bool {
        false
    }
}

impl KeyedDelta for CurationCuratedDelta {
    type Row = CuratedItem;
    type Patch = CurationCuratedPatchEntry;
    fn added(&self) -> &[CuratedItem] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[CurationCuratedPatchEntry] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<CuratedItem>, removed: Vec<String>, patched: Vec<CurationCuratedPatchEntry>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &CuratedItem) -> &str {
        &row.object_id
    }
    fn patch_key(patch: &CurationCuratedPatchEntry) -> &str {
        &patch.object_id
    }
    fn patch_fold(patch: &CurationCuratedPatchEntry, row: &mut CuratedItem) -> Result<(), protocol::MutationApplyError> {
        if let Some(count) = patch.count {
            row.count = count;
        }
        Ok(())
    }
    fn patch_compose(first: &CurationCuratedPatchEntry, later: &CurationCuratedPatchEntry) -> CurationCuratedPatchEntry {
        CurationCuratedPatchEntry { object_id: first.object_id.clone(), count: later.count.or(first.count) }
    }
    fn patch_inverse(patch: &CurationCuratedPatchEntry, base: &CuratedItem) -> CurationCuratedPatchEntry {
        CurationCuratedPatchEntry { object_id: patch.object_id.clone(), count: patch.count.map(|_| base.count) }
    }
    fn patch_between(base: &CuratedItem, other: &CuratedItem) -> Option<CurationCuratedPatchEntry> {
        (base.count != other.count).then(|| CurationCuratedPatchEntry { object_id: other.object_id.clone(), count: Some(other.count) })
    }
    fn patch_is_empty(patch: &CurationCuratedPatchEntry) -> bool {
        patch.count.is_none()
    }
}

impl MutationDiff<CurationSnapshot> for CurationDiff {
    fn apply(&self, snapshot: &CurationSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CurationSnapshot> {
        self.validate().map_err(|message| protocol::MutationApplyError::new("mutation.apply.child-identity", message))?;
        let mut next = snapshot.clone();
        if let Some(handle) = &self.catalog {
            next.catalog = handle.clone();
        }
        if let Some(delta) = &self.stock_extra {
            next.stock_extra = keyed_apply(&next.stock_extra, delta).map_err(|error| error.under(["stockExtra"]))?;
        }
        if let Some(delta) = &self.curated {
            next.curated = keyed_apply(&next.curated, delta).map_err(|error| error.under(["curated"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.catalog.is_some() {
            self.catalog = other.catalog;
        }
        self.stock_extra = match (self.stock_extra.take(), other.stock_extra) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
        self.curated = match (self.curated.take(), other.curated) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<CurationSnapshot> for CurationDiff {
    fn inverse(&self, base: &CurationSnapshot) -> Self {
        Self {
            catalog: self.catalog.as_ref().map(|_| base.catalog.clone()),
            stock_extra: self.stock_extra.as_ref().map(|delta| keyed_inverse(delta, &base.stock_extra)),
            curated: self.curated.as_ref().map(|delta| keyed_inverse(delta, &base.curated)),
        }
    }
    fn between(base: &CurationSnapshot, other: &CurationSnapshot) -> Self {
        let stock_extra = keyed_between::<CurationStockExtraDelta>(&base.stock_extra, &other.stock_extra);
        let curated = keyed_between::<CurationCuratedDelta>(&base.curated, &other.curated);
        Self { catalog: (base.catalog != other.catalog).then(|| other.catalog.clone()), stock_extra: (!keyed_is_empty(&stock_extra)).then_some(stock_extra), curated: (!keyed_is_empty(&curated)).then_some(curated) }
    }
    fn is_empty(&self) -> bool {
        self.catalog.is_none() && self.stock_extra.as_ref().is_none_or(keyed_is_empty) && self.curated.as_ref().is_none_or(keyed_is_empty)
    }
}
//#endregion 🔖️Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
