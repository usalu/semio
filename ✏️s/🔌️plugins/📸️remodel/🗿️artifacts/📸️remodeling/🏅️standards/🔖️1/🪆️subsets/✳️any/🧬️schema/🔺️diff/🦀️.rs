//! 🧬️ Remodeling diff schema — sparse typed delta over the artifact: keyed rows per id-keyed collection (streams, assets,
//! cameras, rig, ground control points), ordered content rows for the durable chunk store, per-facet params and per-slot
//! results. Every row names exactly the entity and fields it changes; the central applier is the only caller of
//! [`MutationDiff::apply`].

use crate::{
    ByteBuffer, CameraCalibration, CameraTrajectory, DenseCloud, DenseParams, FeatureParams, FrameRef, GcpObservation, GeoParams, GeoProducts, GroundControlPoint, IngestParams, MatchParams, MediaStream, MeshParams, MotionParams, MotionTrackSummary,
    QcReportSnapshot, RemodelingAssetChild, RemodelingDurableArtifact, RemodelingMesh, RemodelingSnapshot, RigExtrinsic, SfmParams, SparseCloud, VideoSource,
};
use framework_schema::ArtifactSchema;
use protocol::{ApplyCapability, DiffAlgebra, MutationApplyError, MutationApplyResult, MutationDiff};
use semio_framework_value_derive::{FromValue, ToValue};
use std::cmp::Ordering;
use std::collections::BTreeMap;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the remodeling artifact; persistent entries apply via MutationDiff.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.remodel.remodeling")]
pub struct RemodelingDiff {
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub streams: Option<RemodelingStreamsDelta>,
    #[state(artifact)]
    pub assets: Option<RemodelingAssetsDelta>,
    #[state(artifact)]
    pub durable_artifacts: Option<RemodelingContentDelta>,
    #[state(artifact)]
    pub calibration: Option<RemodelingCalibrationDelta>,
    #[state(artifact)]
    pub params: Option<RemodelingParamsDiff>,
    #[state(artifact)]
    pub gcps: Option<RemodelingGcpsDelta>,
    #[state(artifact)]
    pub results: Option<RemodelingResultsDiff>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 🎯️ An explicitly assigned optional value: wraps the value so assigning `None` stays distinct from leaving the slot untouched on the wire (a bare nested `Option` collapses both to `null`).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingAssigned<T> {
    pub value: T,
}

impl<T> RemodelingAssigned<T> {
    /// 🏗️ Wraps the assigned value.
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// 🩹 The patch of a collection whose rows never patch a member (a member is inserted, replaced as a whole record or removed).
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
pub struct RemodelingNoPatch {}

/// 🗝️ A member of an id-keyed, key-ordered collection: it names its key and knows how to apply, merge and restore its own patch.
pub trait RemodelingEntity<P>: Clone + PartialEq {
    /// 🗝️ The member's key; the collection is kept ordered by it.
    fn key(&self) -> String;
    /// ✍️ Writes the patch's slots into the member and rejects a patch the member cannot take.
    fn write_patch(&mut self, patch: &P) -> Result<(), MutationApplyError>;
    /// 🔁️ The patch that restores this member's value for exactly the slots `patch` names.
    fn restore_patch(&self, patch: &P) -> P;
    /// ➕️ Composes `later` over `patch`: the later value of a slot wins.
    fn merge_patch(patch: &mut P, later: P);
}

/// 🧩 Keyed rows over a key-ordered collection: at most one row per key when they compose (a pair that cannot compose stays as two rows so the sequence keeps being rejected), kept in key order.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingRows<E, P> {
    pub rows: Vec<RemodelingRow<E, P>>,
}

impl<E, P> Default for RemodelingRows<E, P> {
    fn default() -> Self {
        Self { rows: Vec::new() }
    }
}

/// 🧱️ One keyed row: `insert` needs an absent key and lands at the key's ordered position, `replace` swaps a present member whole, `remove` drops a present member, `patch` writes a present member's slots.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "row", rename_all = "camelCase")]
pub enum RemodelingRow<E, P> {
    #[value(rename = "insert", rename_all = "camelCase")]
    Insert { entity: E },
    #[value(rename = "replace", rename_all = "camelCase")]
    Replace { entity: E },
    #[value(rename = "remove", rename_all = "camelCase")]
    Remove { key: String },
    #[value(rename = "patch", rename_all = "camelCase")]
    Patch { key: String, patch: P },
}

impl<E: RemodelingEntity<P>, P> RemodelingRow<E, P> {
    fn key(&self) -> String {
        match self {
            Self::Insert { entity } | Self::Replace { entity } => entity.key(),
            Self::Remove { key } | Self::Patch { key, .. } => key.clone(),
        }
    }
}

/// 🧩 Media-stream collection delta.
pub type RemodelingStreamsDelta = RemodelingRows<MediaStream, MediaStreamPatch>;
/// 🧩 Ground-control-point collection delta.
pub type RemodelingGcpsDelta = RemodelingRows<GroundControlPoint, GroundControlPointPatch>;
/// 🧩 Camera-calibration collection delta.
pub type RemodelingCamerasDelta = RemodelingRows<CameraCalibration, RemodelingNoPatch>;
/// 🧩 Rig-extrinsic collection delta.
pub type RemodelingRigDelta = RemodelingRows<RigExtrinsic, RemodelingNoPatch>;
/// 🧩 Asset-handle collection delta, keyed by asset key.
pub type RemodelingAssetsDelta = RemodelingRows<RemodelingAssetEntry, RemodelingNoPatch>;

/// 🖼️ One asset-handle row member: the asset key beside its composed child handle.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingAssetEntry {
    pub key: String,
    pub child: RemodelingAssetChild,
}

/// 📐️ Calibration delta: cameras and rig extrinsics are separate keyed collections.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingCalibrationDelta {
    pub cameras: RemodelingCamerasDelta,
    pub rig: RemodelingRigDelta,
}

/// 🎞️ A multiset of members of an ordered list: `removed` members leave (the first equal one), `added` members enter at their ordered position.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingMembers<T> {
    pub removed: Vec<T>,
    pub added: Vec<T>,
}

impl<T> Default for RemodelingMembers<T> {
    fn default() -> Self {
        Self { removed: Vec::new(), added: Vec::new() }
    }
}

/// 🔢️ A list member with a total order the owning list is kept in.
pub trait RemodelingOrdered: Clone + PartialEq {
    /// 🔢️ The member order.
    fn rank(&self, other: &Self) -> Ordering;
}

impl RemodelingOrdered for FrameRef {
    fn rank(&self, other: &Self) -> Ordering {
        (self.index, &self.asset_id).cmp(&(other.index, &other.asset_id)).then(self.timestamp_ms.total_cmp(&other.timestamp_ms))
    }
}

impl RemodelingOrdered for GcpObservation {
    fn rank(&self, other: &Self) -> Ordering {
        (&self.stream_id, self.frame_index).cmp(&(&other.stream_id, other.frame_index)).then(self.pixel[0].total_cmp(&other.pixel[0])).then(self.pixel[1].total_cmp(&other.pixel[1]))
    }
}

impl<T: RemodelingOrdered> RemodelingMembers<T> {
    fn write_into(&self, list: &mut Vec<T>) -> Result<(), MutationApplyError> {
        for member in &self.removed {
            let position = list.iter().position(|item| item == member).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "removed member does not exist").at(["removed"]))?;
            list.remove(position);
        }
        for member in &self.added {
            let position = list.partition_point(|item| item.rank(member) == Ordering::Less);
            list.insert(position, member.clone());
        }
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        for member in later.removed {
            match self.added.iter().position(|added| added == &member) {
                Some(position) => {
                    self.added.remove(position);
                }
                None => self.removed.push(member),
            }
        }
        for member in later.added {
            match self.removed.iter().position(|removed| removed == &member) {
                Some(position) => {
                    self.removed.remove(position);
                }
                None => self.added.push(member),
            }
        }
        self.removed.sort_by(|a, b| a.rank(b));
        self.added.sort_by(|a, b| a.rank(b));
    }

    fn restoring(&self) -> Self {
        Self { removed: self.added.clone(), added: self.removed.clone() }
    }

    fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty()
    }
}

/// 🩹 Sparse media-stream patch: the slots a stream edit owns after creation.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct MediaStreamPatch {
    pub sync_offset_ms: Option<f64>,
    pub source: Option<RemodelingAssigned<Option<VideoSource>>>,
    pub frames: Option<RemodelingMembers<FrameRef>>,
}

/// 🩹 Sparse ground-control-point patch: the slots a point edit owns after creation.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct GroundControlPointPatch {
    pub observations: Option<RemodelingMembers<GcpObservation>>,
}

fn merge_members<T: RemodelingOrdered>(slot: &mut Option<RemodelingMembers<T>>, later: Option<RemodelingMembers<T>>) {
    match (slot.as_mut(), later) {
        (Some(prior), Some(later)) => prior.absorb(later),
        (None, Some(later)) => *slot = Some(later),
        _ => {}
    }
}

impl RemodelingEntity<MediaStreamPatch> for MediaStream {
    fn key(&self) -> String {
        self.id.clone()
    }

    fn write_patch(&mut self, patch: &MediaStreamPatch) -> Result<(), MutationApplyError> {
        if let Some(sync_offset_ms) = patch.sync_offset_ms {
            self.sync_offset_ms = sync_offset_ms;
        }
        if let Some(source) = &patch.source {
            self.source = source.value.clone();
        }
        if let Some(frames) = &patch.frames {
            frames.write_into(&mut self.frames).map_err(|error| error.under(["frames"]))?;
        }
        Ok(())
    }

    fn restore_patch(&self, patch: &MediaStreamPatch) -> MediaStreamPatch {
        MediaStreamPatch {
            sync_offset_ms: patch.sync_offset_ms.map(|_| self.sync_offset_ms),
            source: patch.source.as_ref().map(|_| RemodelingAssigned::new(self.source.clone())),
            frames: patch.frames.as_ref().map(RemodelingMembers::restoring),
        }
    }

    fn merge_patch(patch: &mut MediaStreamPatch, later: MediaStreamPatch) {
        if later.sync_offset_ms.is_some() {
            patch.sync_offset_ms = later.sync_offset_ms;
        }
        if later.source.is_some() {
            patch.source = later.source;
        }
        merge_members(&mut patch.frames, later.frames);
    }
}

impl RemodelingEntity<GroundControlPointPatch> for GroundControlPoint {
    fn key(&self) -> String {
        self.id.clone()
    }

    fn write_patch(&mut self, patch: &GroundControlPointPatch) -> Result<(), MutationApplyError> {
        if let Some(observations) = &patch.observations {
            observations.write_into(&mut self.observations).map_err(|error| error.under(["observations"]))?;
        }
        Ok(())
    }

    fn restore_patch(&self, patch: &GroundControlPointPatch) -> GroundControlPointPatch {
        GroundControlPointPatch { observations: patch.observations.as_ref().map(RemodelingMembers::restoring) }
    }

    fn merge_patch(patch: &mut GroundControlPointPatch, later: GroundControlPointPatch) {
        merge_members(&mut patch.observations, later.observations);
    }
}

macro_rules! whole_record_entity {
    ($entity:ty, $key:ident) => {
        impl RemodelingEntity<RemodelingNoPatch> for $entity {
            fn key(&self) -> String {
                self.$key.clone()
            }

            fn write_patch(&mut self, _patch: &RemodelingNoPatch) -> Result<(), MutationApplyError> {
                Ok(())
            }

            fn restore_patch(&self, _patch: &RemodelingNoPatch) -> RemodelingNoPatch {
                RemodelingNoPatch {}
            }

            fn merge_patch(_patch: &mut RemodelingNoPatch, _later: RemodelingNoPatch) {}
        }
    };
}

whole_record_entity!(CameraCalibration, id);
whole_record_entity!(RigExtrinsic, camera_id);
whole_record_entity!(RemodelingAssetEntry, key);
//#endregion 🔖️DeltaHelpers

//#region 🔖️Rows
impl<E: RemodelingEntity<P>, P: Clone> RemodelingRows<E, P> {
    /// 🧬️ Runs the rows over `items`; private so that only [`MutationDiff::apply`] (the central applier's entry) reaches it.
    fn write_into(&self, items: &[E]) -> MutationApplyResult<Vec<E>> {
        let mut next = items.to_vec();
        for (row, entry) in self.rows.iter().enumerate() {
            let at = |field: &str| ["rows".to_string(), row.to_string(), field.to_string()];
            match entry {
                RemodelingRow::Insert { entity } => {
                    let key = entity.key();
                    if next.iter().any(|item| item.key() == key) {
                        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "inserted member key already exists").at(at("entity")));
                    }
                    let position = next.partition_point(|item| item.key() < key);
                    next.insert(position, entity.clone());
                }
                RemodelingRow::Replace { entity } => {
                    let key = entity.key();
                    let position = next.iter().position(|item| item.key() == key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "replaced member does not exist").at(at("entity")))?;
                    next[position] = entity.clone();
                }
                RemodelingRow::Remove { key } => {
                    let position = next.iter().position(|item| &item.key() == key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "removed member does not exist").at(at("key")))?;
                    next.remove(position);
                }
                RemodelingRow::Patch { key, patch } => {
                    let member = next.iter_mut().find(|item| &item.key() == key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "patched member does not exist").at(at("key")))?;
                    member.write_patch(patch).map_err(|error| error.under(["rows".to_string(), row.to_string(), "patch".to_string()]))?;
                }
            }
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self` per key: insert∘replace and insert∘patch stay an insert, insert∘remove cancels, replace∘replace and replace∘patch stay a replace, replace∘remove and patch∘remove are a remove, remove∘insert is a replace, patch∘patch merges, patch∘replace is the replace; an impossible pair stays as two rows.
    pub fn absorb(&mut self, later: Self) {
        for row in later.rows {
            let key = row.key();
            let Some(position) = self.rows.iter().position(|prior| prior.key() == key) else {
                self.rows.push(row);
                continue;
            };
            match coalesce_rows(&self.rows[position], row) {
                Ok(Some(merged)) => self.rows[position] = merged,
                Ok(None) => {
                    self.rows.remove(position);
                }
                Err(rejected) => self.rows.push(rejected),
            }
        }
        self.rows.sort_by_key(|row| row.key());
    }

    /// 🔁️ The negative delta against `base`: each row undone against the member it displaced, rows kept in key order.
    pub fn negative(&self, base: &[E]) -> Self {
        let mut current = base.to_vec();
        let mut undo = Vec::new();
        for entry in &self.rows {
            match entry {
                RemodelingRow::Insert { entity } => {
                    undo.push(RemodelingRow::Remove { key: entity.key() });
                    let key = entity.key();
                    let position = current.partition_point(|item| item.key() < key);
                    current.insert(position, entity.clone());
                }
                RemodelingRow::Replace { entity } => {
                    let key = entity.key();
                    if let Some(position) = current.iter().position(|item| item.key() == key) {
                        undo.push(RemodelingRow::Replace { entity: std::mem::replace(&mut current[position], entity.clone()) });
                    }
                }
                RemodelingRow::Remove { key } => {
                    if let Some(position) = current.iter().position(|item| &item.key() == key) {
                        undo.push(RemodelingRow::Insert { entity: current.remove(position) });
                    }
                }
                RemodelingRow::Patch { key, patch } => {
                    if let Some(member) = current.iter_mut().find(|item| &item.key() == key) {
                        undo.push(RemodelingRow::Patch { key: key.clone(), patch: member.restore_patch(patch) });
                        let _ = member.write_patch(patch);
                    }
                }
            }
        }
        undo.reverse();
        undo.sort_by_key(|row| row.key());
        Self { rows: undo }
    }

    /// 🧭️ The delta from `base` to `other` for sync and import: absent members are inserted, gone members removed, changed members replaced whole.
    pub fn between(base: &[E], other: &[E]) -> Self {
        let mut rows = Vec::new();
        for item in other {
            match base.iter().find(|candidate| candidate.key() == item.key()) {
                None => rows.push(RemodelingRow::Insert { entity: item.clone() }),
                Some(prior) if prior != item => rows.push(RemodelingRow::Replace { entity: item.clone() }),
                Some(_) => {}
            }
        }
        rows.extend(base.iter().filter(|item| !other.iter().any(|candidate| candidate.key() == item.key())).map(|item| RemodelingRow::Remove { key: item.key() }));
        rows.sort_by_key(|row| row.key());
        Self { rows }
    }

    /// 🕳️ Whether the delta names no row.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

fn coalesce_rows<E: RemodelingEntity<P>, P: Clone>(earlier: &RemodelingRow<E, P>, later: RemodelingRow<E, P>) -> Result<Option<RemodelingRow<E, P>>, RemodelingRow<E, P>> {
    use RemodelingRow::{Insert, Patch, Remove, Replace};
    match (earlier, later) {
        (Insert { .. }, Replace { entity }) => Ok(Some(Insert { entity })),
        (Insert { entity }, Patch { patch, .. }) => {
            let mut patched = entity.clone();
            match patched.write_patch(&patch) {
                Ok(()) => Ok(Some(Insert { entity: patched })),
                Err(_) => Err(Patch { key: entity.key(), patch }),
            }
        }
        (Insert { .. }, Remove { .. }) => Ok(None),
        (Replace { .. }, Replace { entity }) => Ok(Some(Replace { entity })),
        (Replace { entity }, Patch { patch, .. }) => {
            let mut patched = entity.clone();
            match patched.write_patch(&patch) {
                Ok(()) => Ok(Some(Replace { entity: patched })),
                Err(_) => Err(Patch { key: entity.key(), patch }),
            }
        }
        (Replace { entity }, Remove { .. }) => Ok(Some(Remove { key: entity.key() })),
        (Remove { .. }, Insert { entity }) => Ok(Some(Replace { entity })),
        (Patch { key, patch: first }, Patch { patch: second, .. }) => {
            let mut merged = first.clone();
            E::merge_patch(&mut merged, second);
            Ok(Some(Patch { key: key.clone(), patch: merged }))
        }
        (Patch { .. }, Replace { entity }) => Ok(Some(Replace { entity })),
        (Patch { key, .. }, Remove { .. }) => Ok(Some(Remove { key: key.clone() })),
        (_, rejected) => Err(rejected),
    }
}
//#endregion 🔖️Rows

//#region 🔖️Content
/// 📦️ The presentation a durable content artifact is created with.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct RemodelingContentHeader {
    pub kind: String,
    pub mime: Option<String>,
    pub width: u32,
    pub height: u32,
}

/// 📦️ Ordered row delta over the durable content store: a row appends leaves to one content artifact or truncates it.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingContentDelta {
    pub rows: Vec<RemodelingContentRow>,
}

/// 📦️ One content row. `append` with a `header` creates an absent artifact, without one it extends a present artifact; `truncate` keeps the first `from` leaves and removes the artifact when `from` is `0`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "row", rename_all = "camelCase")]
pub enum RemodelingContentRow {
    #[value(rename = "append", rename_all = "camelCase")]
    Append { id: String, header: Option<RemodelingContentHeader>, chunks: Vec<ByteBuffer> },
    #[value(rename = "truncate", rename_all = "camelCase")]
    Truncate { id: String, from: u64 },
}

impl RemodelingContentDelta {
    /// 🏗️ The rows that bring a whole artifact into existence.
    pub fn create(id: &str, artifact: &RemodelingDurableArtifact) -> RemodelingContentRow {
        RemodelingContentRow::Append { id: id.to_string(), header: Some(RemodelingContentHeader { kind: artifact.kind.clone(), mime: artifact.mime.clone(), width: artifact.width, height: artifact.height }), chunks: artifact.chunks.clone() }
    }

    fn write_into(&self, store: &BTreeMap<String, RemodelingDurableArtifact>) -> MutationApplyResult<BTreeMap<String, RemodelingDurableArtifact>> {
        let mut next = store.clone();
        for (row, entry) in self.rows.iter().enumerate() {
            let at = |field: &str| ["rows".to_string(), row.to_string(), field.to_string()];
            match entry {
                RemodelingContentRow::Append { id, header: Some(header), chunks } => {
                    if next.contains_key(id) {
                        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "created content already exists").at(at("id")));
                    }
                    next.insert(id.clone(), RemodelingDurableArtifact { kind: header.kind.clone(), mime: header.mime.clone(), width: header.width, height: header.height, chunks: chunks.clone() });
                }
                RemodelingContentRow::Append { id, header: None, chunks } => {
                    let artifact = next.get_mut(id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "extended content does not exist").at(at("id")))?;
                    artifact.chunks.extend(chunks.iter().cloned());
                }
                RemodelingContentRow::Truncate { id, from } => {
                    let artifact = next.get_mut(id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "truncated content does not exist").at(at("id")))?;
                    let keep = usize::try_from(*from).ok().filter(|keep| *keep < artifact.chunks.len()).ok_or_else(|| MutationApplyError::new("mutation.apply.invalid-index", "truncation point is not inside the stored leaves").at(at("from")))?;
                    if keep == 0 {
                        next.remove(id);
                    } else {
                        artifact.chunks.truncate(keep);
                    }
                }
            }
        }
        Ok(next)
    }

    /// ➕️ Sequentially composes `later` after `self`: contiguous appends to one artifact merge and a deeper truncation replaces a shallower one directly before it.
    pub fn absorb(&mut self, later: Self) {
        for row in later.rows {
            match (self.rows.last_mut(), &row) {
                (Some(RemodelingContentRow::Append { id, chunks, .. }), RemodelingContentRow::Append { id: later_id, header: None, chunks: more }) if id == later_id => chunks.extend(more.iter().cloned()),
                (Some(RemodelingContentRow::Truncate { id, from }), RemodelingContentRow::Truncate { id: later_id, from: deeper }) if id == later_id && deeper < from => *from = *deeper,
                _ => self.rows.push(row),
            }
        }
    }

    /// 🔁️ The negative delta against `base`: each row undone against the leaves it displaced, in reverse order.
    pub fn negative(&self, base: &BTreeMap<String, RemodelingDurableArtifact>) -> Self {
        let mut current = base.clone();
        let mut undo = Vec::new();
        for entry in &self.rows {
            match entry {
                RemodelingContentRow::Append { id, header: Some(header), chunks } => {
                    undo.push(RemodelingContentRow::Truncate { id: id.clone(), from: 0 });
                    current.insert(id.clone(), RemodelingDurableArtifact { kind: header.kind.clone(), mime: header.mime.clone(), width: header.width, height: header.height, chunks: chunks.clone() });
                }
                RemodelingContentRow::Append { id, header: None, chunks } => {
                    if let Some(artifact) = current.get_mut(id) {
                        undo.push(RemodelingContentRow::Truncate { id: id.clone(), from: artifact.chunks.len() as u64 });
                        artifact.chunks.extend(chunks.iter().cloned());
                    }
                }
                RemodelingContentRow::Truncate { id, from } => {
                    let Some(artifact) = current.get(id).cloned() else { continue };
                    let keep = usize::try_from(*from).unwrap_or(usize::MAX);
                    if keep == 0 {
                        undo.push(Self::create(id, &artifact));
                        current.remove(id);
                    } else if keep < artifact.chunks.len() {
                        undo.push(RemodelingContentRow::Append { id: id.clone(), header: None, chunks: artifact.chunks[keep..].to_vec() });
                        if let Some(kept) = current.get_mut(id) {
                            kept.chunks.truncate(keep);
                        }
                    }
                }
            }
        }
        undo.reverse();
        Self { rows: undo }
    }

    /// 🧭️ The delta from `base` to `other` for sync and import: gone artifacts are truncated away, changed ones are rebuilt, new ones are created.
    pub fn between(base: &BTreeMap<String, RemodelingDurableArtifact>, other: &BTreeMap<String, RemodelingDurableArtifact>) -> Self {
        let mut rows = Vec::new();
        for (id, prior) in base {
            if other.get(id) != Some(prior) {
                rows.push(RemodelingContentRow::Truncate { id: id.clone(), from: 0 });
            }
        }
        for (id, artifact) in other {
            if base.get(id) != Some(artifact) {
                rows.push(Self::create(id, artifact));
            }
        }
        Self { rows }
    }

    /// 🕳️ Whether the delta names no row.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
//#endregion 🔖️Content

//#region 🔖️Params
/// ⚙️ Sparse reconstruction-parameter delta: each slot replaces one whole pipeline-stage facet.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingParamsDiff {
    pub ingest: Option<IngestParams>,
    pub feature: Option<FeatureParams>,
    pub matching: Option<MatchParams>,
    pub sfm: Option<SfmParams>,
    pub dense: Option<DenseParams>,
    pub mesh: Option<MeshParams>,
    pub motion: Option<MotionParams>,
    pub geo: Option<GeoParams>,
}

macro_rules! facet_algebra {
    ($($slot:ident),+) => {
        impl RemodelingParamsDiff {
            fn write_into(&self, params: &mut crate::ReconstructionParams) {
                $(if let Some(facet) = &self.$slot {
                    params.$slot = facet.clone();
                })+
            }

            fn absorb(&mut self, later: Self) {
                $(if later.$slot.is_some() {
                    self.$slot = later.$slot;
                })+
            }

            /// 🔁️ The delta that restores `base` for exactly the facets this delta names.
            pub fn restoring(&self, base: &crate::ReconstructionParams) -> Self {
                Self { $($slot: self.$slot.as_ref().map(|_| base.$slot.clone()),)+ }
            }

            /// 🧭️ The delta from `base` to `other`.
            pub fn between(base: &crate::ReconstructionParams, other: &crate::ReconstructionParams) -> Self {
                Self { $($slot: (base.$slot != other.$slot).then(|| other.$slot.clone()),)+ }
            }

            /// 🕳️ Whether the delta names no facet.
            pub fn is_empty(&self) -> bool {
                $(self.$slot.is_none())&&+
            }
        }
    };
}

facet_algebra!(ingest, feature, matching, sfm, dense, mesh, motion, geo);
//#endregion 🔖️Params

//#region 🔖️Results
/// 📦️ Sparse reconstruction-results delta: each slot replaces one result sub-payload as a whole.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct RemodelingResultsDiff {
    pub sparse: Option<RemodelingAssigned<Option<SparseCloud>>>,
    pub dense: Option<RemodelingAssigned<Option<DenseCloud>>>,
    pub mesh: Option<RemodelingMesh>,
    pub trajectory: Option<RemodelingAssigned<Option<CameraTrajectory>>>,
    pub tracks: Option<Vec<MotionTrackSummary>>,
    pub geo: Option<RemodelingAssigned<Option<GeoProducts>>>,
    pub qc: Option<RemodelingAssigned<Option<QcReportSnapshot>>>,
}

impl RemodelingResultsDiff {
    fn write_into(&self, results: &mut crate::ReconstructionResults) {
        if let Some(sparse) = &self.sparse {
            results.sparse = sparse.value.clone();
        }
        if let Some(dense) = &self.dense {
            results.dense = dense.value.clone();
        }
        if let Some(mesh) = &self.mesh {
            results.mesh = mesh.clone();
        }
        if let Some(trajectory) = &self.trajectory {
            results.trajectory = trajectory.value.clone();
        }
        if let Some(tracks) = &self.tracks {
            results.tracks = tracks.clone();
        }
        if let Some(geo) = &self.geo {
            results.geo = geo.value.clone();
        }
        if let Some(qc) = &self.qc {
            results.qc = qc.value.clone();
        }
    }

    fn absorb(&mut self, later: Self) {
        macro_rules! take {
            ($($slot:ident),+) => {
                $(if later.$slot.is_some() {
                    self.$slot = later.$slot;
                })+
            };
        }
        take!(sparse, dense, mesh, trajectory, tracks, geo, qc);
    }

    /// 🔁️ The delta that restores `base` for exactly the slots this delta names.
    pub fn restoring(&self, base: &crate::ReconstructionResults) -> Self {
        Self {
            sparse: self.sparse.as_ref().map(|_| RemodelingAssigned::new(base.sparse.clone())),
            dense: self.dense.as_ref().map(|_| RemodelingAssigned::new(base.dense.clone())),
            mesh: self.mesh.as_ref().map(|_| base.mesh.clone()),
            trajectory: self.trajectory.as_ref().map(|_| RemodelingAssigned::new(base.trajectory.clone())),
            tracks: self.tracks.as_ref().map(|_| base.tracks.clone()),
            geo: self.geo.as_ref().map(|_| RemodelingAssigned::new(base.geo.clone())),
            qc: self.qc.as_ref().map(|_| RemodelingAssigned::new(base.qc.clone())),
        }
    }

    /// 🧭️ The delta from `base` to `other`.
    pub fn between(base: &crate::ReconstructionResults, other: &crate::ReconstructionResults) -> Self {
        Self {
            sparse: (base.sparse != other.sparse).then(|| RemodelingAssigned::new(other.sparse.clone())),
            dense: (base.dense != other.dense).then(|| RemodelingAssigned::new(other.dense.clone())),
            mesh: (base.mesh != other.mesh).then(|| other.mesh.clone()),
            trajectory: (base.trajectory != other.trajectory).then(|| RemodelingAssigned::new(other.trajectory.clone())),
            tracks: (base.tracks != other.tracks).then(|| other.tracks.clone()),
            geo: (base.geo != other.geo).then(|| RemodelingAssigned::new(other.geo.clone())),
            qc: (base.qc != other.qc).then(|| RemodelingAssigned::new(other.qc.clone())),
        }
    }

    /// 🕳️ Whether the delta names no slot.
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔖️Results

//#region 🔖️Builders
impl RemodelingDiff {
    /// 🏗️ A diff carrying the given stream rows.
    pub fn stream_rows(rows: Vec<RemodelingRow<MediaStream, MediaStreamPatch>>) -> Self {
        Self { streams: Some(RemodelingRows { rows }), ..Default::default() }
    }

    /// 🏗️ A diff carrying the given ground-control-point rows.
    pub fn gcp_rows(rows: Vec<RemodelingRow<GroundControlPoint, GroundControlPointPatch>>) -> Self {
        Self { gcps: Some(RemodelingRows { rows }), ..Default::default() }
    }

    /// 🏗️ A diff carrying the given camera-calibration rows.
    pub fn camera_rows(rows: Vec<RemodelingRow<CameraCalibration, RemodelingNoPatch>>) -> Self {
        Self { calibration: Some(RemodelingCalibrationDelta { cameras: RemodelingRows { rows }, rig: RemodelingRows::default() }), ..Default::default() }
    }

    /// 🏗️ A diff carrying the given rig-extrinsic rows.
    pub fn rig_rows(rows: Vec<RemodelingRow<RigExtrinsic, RemodelingNoPatch>>) -> Self {
        Self { calibration: Some(RemodelingCalibrationDelta { cameras: RemodelingRows::default(), rig: RemodelingRows { rows } }), ..Default::default() }
    }

    /// 🏗️ A diff carrying the given asset-handle rows.
    pub fn asset_rows(rows: Vec<RemodelingRow<RemodelingAssetEntry, RemodelingNoPatch>>) -> Self {
        Self { assets: Some(RemodelingRows { rows }), ..Default::default() }
    }

    /// 🏗️ A diff carrying the given content rows.
    pub fn content_rows(rows: Vec<RemodelingContentRow>) -> Self {
        Self { durable_artifacts: Some(RemodelingContentDelta { rows }), ..Default::default() }
    }
}
//#endregion 🔖️Builders

//#region 🔖️Apply
fn asset_entries(assets: &BTreeMap<String, RemodelingAssetChild>) -> Vec<RemodelingAssetEntry> {
    assets.iter().map(|(key, child)| RemodelingAssetEntry { key: key.clone(), child: child.clone() }).collect()
}

fn asset_map(entries: Vec<RemodelingAssetEntry>) -> BTreeMap<String, RemodelingAssetChild> {
    entries.into_iter().map(|entry| (entry.key, entry.child)).collect()
}

fn absorb_rows<E: RemodelingEntity<P>, P: Clone>(slot: &mut Option<RemodelingRows<E, P>>, later: Option<RemodelingRows<E, P>>) {
    match (slot.as_mut(), later) {
        (Some(prior), Some(later)) => prior.absorb(later),
        (None, Some(later)) => *slot = Some(later),
        _ => {}
    }
}

fn nonempty<T>(delta: T, is_empty: impl Fn(&T) -> bool) -> Option<T> {
    (!is_empty(&delta)).then_some(delta)
}

impl MutationDiff<RemodelingSnapshot> for RemodelingDiff {
    fn apply(&self, base: &RemodelingSnapshot, _capability: ApplyCapability) -> MutationApplyResult<RemodelingSnapshot> {
        let mut next = base.clone();
        if let Some(schema) = &self.schema {
            next.schema = schema.clone();
        }
        if let Some(id) = &self.id {
            next.id = id.clone();
        }
        if let Some(delta) = &self.streams {
            next.streams = delta.write_into(&base.streams).map_err(|error| error.under(["streams"]))?;
        }
        if let Some(delta) = &self.assets {
            next.assets = asset_map(delta.write_into(&asset_entries(&base.assets)).map_err(|error| error.under(["assets"]))?);
        }
        if let Some(delta) = &self.durable_artifacts {
            next.durable_artifacts = delta.write_into(&base.durable_artifacts).map_err(|error| error.under(["durableArtifacts"]))?;
        }
        if let Some(delta) = &self.calibration {
            next.calibration.cameras = delta.cameras.write_into(&base.calibration.cameras).map_err(|error| error.under(["calibration", "cameras"]))?;
            next.calibration.rig = delta.rig.write_into(&base.calibration.rig).map_err(|error| error.under(["calibration", "rig"]))?;
        }
        if let Some(params) = &self.params {
            params.write_into(&mut next.params);
        }
        if let Some(delta) = &self.gcps {
            next.gcps = delta.write_into(&base.gcps).map_err(|error| error.under(["gcps"]))?;
        }
        if let Some(results) = &self.results {
            results.write_into(&mut next.results);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.schema.is_some() {
            self.schema = other.schema;
        }
        if other.id.is_some() {
            self.id = other.id;
        }
        absorb_rows(&mut self.streams, other.streams);
        absorb_rows(&mut self.assets, other.assets);
        absorb_rows(&mut self.gcps, other.gcps);
        match (self.durable_artifacts.as_mut(), other.durable_artifacts) {
            (Some(prior), Some(later)) => prior.absorb(later),
            (None, Some(later)) => self.durable_artifacts = Some(later),
            _ => {}
        }
        match (self.calibration.as_mut(), other.calibration) {
            (Some(prior), Some(later)) => {
                prior.cameras.absorb(later.cameras);
                prior.rig.absorb(later.rig);
            }
            (None, Some(later)) => self.calibration = Some(later),
            _ => {}
        }
        match (self.params.as_mut(), other.params) {
            (Some(prior), Some(later)) => prior.absorb(later),
            (None, Some(later)) => self.params = Some(later),
            _ => {}
        }
        match (self.results.as_mut(), other.results) {
            (Some(prior), Some(later)) => prior.absorb(later),
            (None, Some(later)) => self.results = Some(later),
            _ => {}
        }
    }
}

impl DiffAlgebra<RemodelingSnapshot> for RemodelingDiff {
    fn inverse(&self, base: &RemodelingSnapshot) -> Self {
        Self {
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            id: self.id.as_ref().map(|_| base.id.clone()),
            streams: self.streams.as_ref().map(|delta| delta.negative(&base.streams)),
            assets: self.assets.as_ref().map(|delta| delta.negative(&asset_entries(&base.assets))),
            durable_artifacts: self.durable_artifacts.as_ref().map(|delta| delta.negative(&base.durable_artifacts)),
            calibration: self.calibration.as_ref().map(|delta| RemodelingCalibrationDelta { cameras: delta.cameras.negative(&base.calibration.cameras), rig: delta.rig.negative(&base.calibration.rig) }),
            params: self.params.as_ref().map(|params| params.restoring(&base.params)),
            gcps: self.gcps.as_ref().map(|delta| delta.negative(&base.gcps)),
            results: self.results.as_ref().map(|results| results.restoring(&base.results)),
        }
    }

    fn between(base: &RemodelingSnapshot, other: &RemodelingSnapshot) -> Self {
        let calibration = RemodelingCalibrationDelta { cameras: RemodelingRows::between(&base.calibration.cameras, &other.calibration.cameras), rig: RemodelingRows::between(&base.calibration.rig, &other.calibration.rig) };
        Self {
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            id: (base.id != other.id).then(|| other.id.clone()),
            streams: nonempty(RemodelingRows::between(&base.streams, &other.streams), RemodelingRows::is_empty),
            assets: nonempty(RemodelingRows::between(&asset_entries(&base.assets), &asset_entries(&other.assets)), RemodelingRows::is_empty),
            durable_artifacts: nonempty(RemodelingContentDelta::between(&base.durable_artifacts, &other.durable_artifacts), RemodelingContentDelta::is_empty),
            calibration: (!calibration.cameras.is_empty() || !calibration.rig.is_empty()).then_some(calibration),
            params: nonempty(RemodelingParamsDiff::between(&base.params, &other.params), RemodelingParamsDiff::is_empty),
            gcps: nonempty(RemodelingRows::between(&base.gcps, &other.gcps), RemodelingRows::is_empty),
            results: nonempty(RemodelingResultsDiff::between(&base.results, &other.results), RemodelingResultsDiff::is_empty),
        }
    }

    fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
//#endregion 🔖️Apply

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::schema::RemodelingArtifact;
//#endregion 🔁️Re-exports

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
