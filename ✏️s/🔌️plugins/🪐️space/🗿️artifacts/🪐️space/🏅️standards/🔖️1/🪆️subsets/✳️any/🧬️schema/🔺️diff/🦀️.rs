//! 🧬️ S Space index diff schema — sparse field delta over the artifact. `artifacts` is an id-keyed row delta
//! (`added`/`removed`/`patched`/`reordered`): a mutation names exactly the rows it creates, deletes or patches.

use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactDialect, SpaceArtifactRow};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the S Space index artifact; persistent entries apply via
/// [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.space.space")]
pub struct SSpaceDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub artifacts: Option<SSpaceArtifactsDelta>,
}

/// 🧩 Id-keyed row delta for `artifacts`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SSpaceArtifactsDelta {
    pub added: Vec<SpaceArtifactRow>,
    pub removed: Vec<String>,
    pub patched: Vec<SSpaceArtifactPatch>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 Field patch of one artifact row; every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SSpaceArtifactPatch {
    pub id: String,
    pub name: Option<String>,
    pub kind_id: Option<String>,
    pub schema: Option<String>,
    pub dialect: Option<SpaceArtifactDialect>,
    pub created_at_ms: Option<u64>,
    pub created_by: Option<String>,
    pub updated_at_ms: Option<u64>,
    pub updated_by: Option<String>,
}
//#endregion 🔖️Diff

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

impl KeyedDelta for SSpaceArtifactsDelta {
    type Row = SpaceArtifactRow;
    type Patch = SSpaceArtifactPatch;
    fn added(&self) -> &[SpaceArtifactRow] {
        &self.added
    }
    fn removed(&self) -> &[String] {
        &self.removed
    }
    fn patched(&self) -> &[SSpaceArtifactPatch] {
        &self.patched
    }
    fn reordered(&self) -> Option<&[String]> {
        self.reordered.as_deref()
    }
    fn assemble(added: Vec<SpaceArtifactRow>, removed: Vec<String>, patched: Vec<SSpaceArtifactPatch>, reordered: Option<Vec<String>>) -> Self {
        Self { added, removed, patched, reordered }
    }
    fn row_key(row: &SpaceArtifactRow) -> &str {
        &row.id
    }
    fn patch_key(patch: &SSpaceArtifactPatch) -> &str {
        &patch.id
    }
    fn patch_fold(patch: &SSpaceArtifactPatch, row: &mut SpaceArtifactRow) -> Result<(), protocol::MutationApplyError> {
        if let Some(value) = &patch.name {
            row.name = value.clone();
        }
        if let Some(value) = &patch.kind_id {
            row.kind_id = value.clone();
        }
        if let Some(value) = &patch.schema {
            row.schema = value.clone();
        }
        if let Some(value) = &patch.dialect {
            row.dialect = value.clone();
        }
        if let Some(value) = patch.created_at_ms {
            row.created_at_ms = value;
        }
        if let Some(value) = &patch.created_by {
            row.created_by = value.clone();
        }
        if let Some(value) = patch.updated_at_ms {
            row.updated_at_ms = value;
        }
        if let Some(value) = &patch.updated_by {
            row.updated_by = value.clone();
        }
        Ok(())
    }
    fn patch_compose(first: &SSpaceArtifactPatch, later: &SSpaceArtifactPatch) -> SSpaceArtifactPatch {
        SSpaceArtifactPatch {
            id: first.id.clone(),
            name: later.name.clone().or_else(|| first.name.clone()),
            kind_id: later.kind_id.clone().or_else(|| first.kind_id.clone()),
            schema: later.schema.clone().or_else(|| first.schema.clone()),
            dialect: later.dialect.clone().or_else(|| first.dialect.clone()),
            created_at_ms: later.created_at_ms.or(first.created_at_ms),
            created_by: later.created_by.clone().or_else(|| first.created_by.clone()),
            updated_at_ms: later.updated_at_ms.or(first.updated_at_ms),
            updated_by: later.updated_by.clone().or_else(|| first.updated_by.clone()),
        }
    }
    fn patch_inverse(patch: &SSpaceArtifactPatch, base: &SpaceArtifactRow) -> SSpaceArtifactPatch {
        SSpaceArtifactPatch {
            id: patch.id.clone(),
            name: patch.name.as_ref().map(|_| base.name.clone()),
            kind_id: patch.kind_id.as_ref().map(|_| base.kind_id.clone()),
            schema: patch.schema.as_ref().map(|_| base.schema.clone()),
            dialect: patch.dialect.as_ref().map(|_| base.dialect.clone()),
            created_at_ms: patch.created_at_ms.map(|_| base.created_at_ms),
            created_by: patch.created_by.as_ref().map(|_| base.created_by.clone()),
            updated_at_ms: patch.updated_at_ms.map(|_| base.updated_at_ms),
            updated_by: patch.updated_by.as_ref().map(|_| base.updated_by.clone()),
        }
    }
    fn patch_between(base: &SpaceArtifactRow, other: &SpaceArtifactRow) -> Option<SSpaceArtifactPatch> {
        let patch = SSpaceArtifactPatch {
            id: other.id.clone(),
            name: (base.name != other.name).then(|| other.name.clone()),
            kind_id: (base.kind_id != other.kind_id).then(|| other.kind_id.clone()),
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            dialect: (base.dialect != other.dialect).then(|| other.dialect.clone()),
            created_at_ms: (base.created_at_ms != other.created_at_ms).then_some(other.created_at_ms),
            created_by: (base.created_by != other.created_by).then(|| other.created_by.clone()),
            updated_at_ms: (base.updated_at_ms != other.updated_at_ms).then_some(other.updated_at_ms),
            updated_by: (base.updated_by != other.updated_by).then(|| other.updated_by.clone()),
        };
        (!Self::patch_is_empty(&patch)).then_some(patch)
    }
    fn patch_is_empty(patch: &SSpaceArtifactPatch) -> bool {
        patch.name.is_none() && patch.kind_id.is_none() && patch.schema.is_none() && patch.dialect.is_none() && patch.created_at_ms.is_none() && patch.created_by.is_none() && patch.updated_at_ms.is_none() && patch.updated_by.is_none()
    }
}

//#region 🔖️Apply
impl protocol::MutationDiff<SSpaceSnapshot> for SSpaceDiff {
    fn apply(&self, snapshot: &SSpaceSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SSpaceSnapshot> {
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(delta) = &self.artifacts {
            next.artifacts = keyed_apply(&next.artifacts, delta).map_err(|error| error.under(["artifacts"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        self.artifacts = match (self.artifacts.take(), other.artifacts) {
            (Some(first), Some(later)) => Some(keyed_absorb(&first, &later)),
            (first, later) => later.or(first),
        };
    }
}

impl protocol::DiffAlgebra<SSpaceSnapshot> for SSpaceDiff {
    fn inverse(&self, base: &SSpaceSnapshot) -> Self {
        Self { schema: self.schema.as_ref().map(|_| base.schema.clone()), artifacts: self.artifacts.as_ref().map(|delta| keyed_inverse(delta, &base.artifacts)) }
    }
    fn between(base: &SSpaceSnapshot, other: &SSpaceSnapshot) -> Self {
        let artifacts = keyed_between::<SSpaceArtifactsDelta>(&base.artifacts, &other.artifacts);
        Self { schema: (base.schema != other.schema).then(|| other.schema.clone()), artifacts: (!keyed_is_empty(&artifacts)).then_some(artifacts) }
    }
    fn is_empty(&self) -> bool {
        self.schema.is_none() && self.artifacts.as_ref().is_none_or(keyed_is_empty)
    }
}
//#endregion 🔖️Apply
