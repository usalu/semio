//! 🧊️ Collision index of the precompute geometry layer: the collision bodies, bounds and the spatial broad-phase index with its incremental mutation and query cursors.

use super::*;

//#region 🔖️Collision
#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct CollisionMeshPart {
    pub shape: CollisionShape,
    pub local_pose: Pose3d,
}

#[derive(Clone, semio_framework_value::RetireOwned)]
pub struct CollisionBody {
    pub parts: Vec<CollisionMeshPart>,
    pub local_bounds_min: Point3d,
    pub local_bounds_max: Point3d,
}

pub fn collision_body_from_buffers(positions: &[f32], indices: &[u32]) -> Option<CollisionBody> {
    if positions.len() < 9 || indices.len() < 3 {
        return None;
    }
    precompute_work(positions.len() / 3 + indices.len() / 3);
    let mut verts: Vec<Point3d> = Vec::with_capacity(positions.len() / 3);
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for chunk in positions.as_chunks::<3>().0 {
        let rp = Point3d::new(chunk[0], chunk[1], chunk[2]);
        verts.push(rp);
        min = min.inf(&rp);
        max = max.sup(&rp);
    }
    let extent = (max - min).amax();
    if !extent.is_finite() || extent < BRUSH_COLLISION_MESH_MIN_EXTENT as f32 {
        return None;
    }
    let mut tris: Vec<[u32; 3]> = Vec::with_capacity(indices.len() / 3);
    for chunk in indices.as_chunks::<3>().0 {
        tris.push(*chunk);
    }
    let shape = CollisionShape::from_triangle_mesh(&verts, tris);
    Some(CollisionBody { parts: vec![CollisionMeshPart { shape, local_pose: Pose3d::identity() }], local_bounds_min: min, local_bounds_max: max })
}

pub fn world_bounds(body: &CollisionBody, world: &Pose3d) -> (Point3d, Point3d) {
    let corners = [
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_min.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_min.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_min.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
        Point3d::new(body.local_bounds_max.x(), body.local_bounds_max.y(), body.local_bounds_max.z()),
    ];
    let mut min = Point3d::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max = Point3d::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
    for corner in corners {
        let w = world.transform_point(&corner);
        min = min.inf(&w);
        max = max.sup(&w);
    }
    (min, max)
}

pub fn volume_scale_vec(scale: &Option<semio_framework_value::DslValue>) -> [f32; 3] {
    match scale {
        Some(semio_framework_value::DslValue::Number(n)) => {
            let s = n.as_f64() as f32;
            [s, s, s]
        }
        Some(semio_framework_value::DslValue::Array(values)) if values.len() == 3 => {
            let read = |index: usize| values.get(index).and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
            [read(0), read(1), read(2)]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

pub fn world_volumes_contain_aabb(volumes: &[WorldVolumeProps], min: Point3d, max: Point3d) -> bool {
    if volumes.is_empty() {
        return true;
    }
    let corners = [
        min,
        Point3d::new(max.x(), min.y(), min.z()),
        Point3d::new(min.x(), max.y(), min.z()),
        Point3d::new(max.x(), max.y(), min.z()),
        Point3d::new(min.x(), min.y(), max.z()),
        Point3d::new(max.x(), min.y(), max.z()),
        Point3d::new(min.x(), max.y(), max.z()),
        max,
    ];
    for volume in volumes {
        let scale = volume_scale_vec(&volume.scale);
        let world = pose_isometry(volume.origin, volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &None);
        let inv = world.inverse();
        let hx = 0.5 + 1e-3;
        let hy = 0.5 + 1e-3;
        let hz = 0.5 + 1e-3;
        let mut inside = true;
        for corner in corners {
            let relative = inv.transform_point(&corner);
            let local = Point3d::new(relative.x() / scale[0], relative.y() / scale[1], relative.z() / scale[2]);
            if local.x().abs() > hx || local.y().abs() > hy || local.z().abs() > hz {
                inside = false;
                break;
            }
        }
        if inside {
            return true;
        }
    }
    false
}

#[cfg(test)]
pub fn point_inside_body(body: &CollisionBody, world: &Pose3d, point: Point3d) -> bool {
    let local = world.inverse().transform_point(&point);
    for part in &body.parts {
        let part_local = part.local_pose.inverse().transform_point(&local);
        if part.shape.contains_point(&part.local_pose, &part_local) {
            return true;
        }
    }
    false
}

#[cfg(test)]
pub fn bodies_intersect(a: &CollisionBody, world_a: &Pose3d, b: &CollisionBody, world_b: &Pose3d) -> bool {
    let (amin, amax) = world_bounds(a, world_a);
    let (bmin, bmax) = world_bounds(b, world_b);
    if amax.x() < bmin.x() || bmax.x() < amin.x() || amax.y() < bmin.y() || bmax.y() < amin.y() || amax.z() < bmin.z() || bmax.z() < amin.z() {
        return false;
    }
    for part_a in &a.parts {
        let pose_a = world_a.semio_compose_rs(&part_a.local_pose);
        for part_b in &b.parts {
            let pose_b = world_b.semio_compose_rs(&part_b.local_pose);
            if shapes_intersect(&pose_a, &part_a.shape, &pose_b, &part_b.shape) {
                return true;
            }
        }
    }
    let center = Point3d::from_coords((amin.coords() + amax.coords() + bmin.coords() + bmax.coords()) * 0.25);
    point_inside_body(a, world_a, center) && point_inside_body(b, world_b, center)
}

//#region 🗺️BroadPhase
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
pub struct CollisionAabb {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

impl CollisionAabb {
    pub fn from_body(body: &CollisionBody, world: &Pose3d) -> Self {
        let (min, max) = world_bounds(body, world);
        Self { min: [min.x(), min.y(), min.z()], max: [max.x(), max.y(), max.z()] }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.max[0] >= other.min[0] && other.max[0] >= self.min[0] && self.max[1] >= other.min[1] && other.max[1] >= self.min[1] && self.max[2] >= other.min[2] && other.max[2] >= self.min[2]
    }
}

/// 🏙️ One spatial cell's member ids: document-wide ([`DOCUMENT_CELL_MEMBER_SLOTS`]), claimed a bookkeeping page at a time.
pub type CollisionCellMembers = FixedOwnerSet<String, DOCUMENT_CELL_MEMBER_SLOTS, FIXED_OWNER_SLOTS>;

#[derive(Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CollisionSpatialIndex {
    cell_size: f32,
    pub(super) entries: FixedOwnerMap<String, CollisionAabb, DOCUMENT_OBJECT_SLOTS>,
    pub(super) cells: FixedOwnerMap<(i32, i32, i32), CollisionCellMembers, DOCUMENT_CELL_SLOTS>,
    pub(super) oversized: FixedOwnerSet<String, DOCUMENT_KIND_SLOTS>,
    retiring_key: Option<String>,
    retiring_bucket: Option<CollisionCellMembers>,
}

#[derive(Clone, Debug)]
#[derive(semio_framework_value::RetireOwned)]
pub enum CollisionIndexRejectedOwner {
    Capacity(String),
}

impl CollisionIndexRejectedOwner {
    pub fn retire_one(&mut self) -> bool {
        match self {
            Self::Capacity(id) if id.capacity() != 0 => {
                drop(std::mem::take(id));
                false
            }
            Self::Capacity(_) => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
pub struct CollisionIndexOwner {
    pub operation: u64,
    pub generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
struct CollisionCellSpan {
    pub(super) min: [i32; 3],
    pub(super) max: [i32; 3],
    pub(super) count: u64,
}

impl CollisionCellSpan {
    pub(super) fn new(cell_size: f32, bounds: CollisionAabb) -> Option<Self> {
        let cell = |value: f32| (value / cell_size).floor().clamp(i32::MIN as f32, i32::MAX as f32) as i32;
        let min = [cell(bounds.min[0]), cell(bounds.min[1]), cell(bounds.min[2])];
        let max = [cell(bounds.max[0]), cell(bounds.max[1]), cell(bounds.max[2])];
        let spans = [0, 1, 2].map(|axis| u64::try_from(i64::from(max[axis]) - i64::from(min[axis]) + 1).ok());
        let count = spans.into_iter().try_fold(1_u64, |count, span| count.checked_mul(span?))?;
        (count <= CollisionSpatialIndex::MAX_CELLS_PER_ENTRY).then_some(Self { min, max, count })
    }

    pub(super) fn cell(self, cursor: u64) -> Option<(i32, i32, i32)> {
        if cursor >= self.count {
            return None;
        }
        let z_span = u64::try_from(i64::from(self.max[2]) - i64::from(self.min[2]) + 1).ok()?;
        let y_span = u64::try_from(i64::from(self.max[1]) - i64::from(self.min[1]) + 1).ok()?;
        let yz = y_span.checked_mul(z_span)?;
        let x = cursor / yz;
        let rest = cursor % yz;
        let y = rest / z_span;
        let z = rest % z_span;
        Some((i32::try_from(i64::from(self.min[0]) + i64::try_from(x).ok()?).ok()?, i32::try_from(i64::from(self.min[1]) + i64::try_from(y).ok()?).ok()?, i32::try_from(i64::from(self.min[2]) + i64::try_from(z).ok()?).ok()?))
    }

    pub(super) fn contains(self, cell: (i32, i32, i32)) -> bool {
        cell.0 >= self.min[0] && cell.0 <= self.max[0] && cell.1 >= self.min[1] && cell.1 <= self.max[1] && cell.2 >= self.min[2] && cell.2 <= self.max[2]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
enum CollisionMutationStage {
    PreflightNew,
    PreflightOld,
    Remove,
    Insert,
    Commit,
    Complete,
    Rejected,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct CollisionIndexMutation {
    pub(super) owner: CollisionIndexOwner,
    pub(super) id: String,
    pub(super) bounds: CollisionAabb,
    old_bounds: Option<CollisionAabb>,
    old_span: Option<CollisionCellSpan>,
    new_span: Option<CollisionCellSpan>,
    pub(super) stage: CollisionMutationStage,
    pub(super) cursor: u64,
    missing_cells: usize,
    reclaimed_cells: usize,
}

#[derive(Debug)]
pub enum CollisionMutationStep {
    Pending,
    Complete,
    Rejected(CollisionIndexRejectedOwner),
    Stale,
}

/// 🗑️ Resumable withdrawal of one indexed owner — the production counterpart of
/// [`CollisionIndexMutation`], driven one cell per step by the interactive brush lane's incremental
/// scene sync so a deleted object leaves the broad phase without a whole-index rebuild.
#[derive(semio_framework_value::RetireOwned)]
pub struct CollisionIndexRemoval {
    pub(super) owner: CollisionIndexOwner,
    pub(super) id: String,
    span: Option<CollisionCellSpan>,
    pub(super) cursor: u64,
    complete: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
enum CollisionQueryStage {
    Cells,
    Oversized,
    Entries,
    Complete,
}

#[derive(semio_framework_value::RetireOwned)]
pub struct CollisionQueryCursor {
    pub(super) owner: CollisionIndexOwner,
    pub(super) bounds: CollisionAabb,
    span: Option<CollisionCellSpan>,
    pub(super) stage: CollisionQueryStage,
    cell_cursor: u64,
    member_cursor: usize,
    candidates: FixedOwnerSet<String, DOCUMENT_OBJECT_SLOTS>,
    pub(super) truncated: bool,
    pub(super) examined_cells: usize,
    pub(super) examined_members: usize,
    retiring_key: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionQueryStep {
    Pending,
    Complete,
    Stale,
}

impl CollisionQueryCursor {
    pub fn candidate(&self, index: usize) -> Option<&String> {
        self.candidates.iter().nth(index)
    }

    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    pub fn truncated(&self) -> bool {
        self.truncated
    }

    /// 📏️ Broad-phase work actually spent by this cursor — `(cells, members)`. The interactive brush
    /// lane publishes it so a document-scale scene can prove the query touched only the cells its own
    /// bounds span instead of every placed object.
    pub fn examined(&self) -> (usize, usize) {
        (self.examined_cells, self.examined_members)
    }

    pub fn retire_one_owner(&mut self) -> bool {
        if let Some(key) = self.retiring_key.as_mut() {
            if key.capacity() != 0 {
                drop(std::mem::take(key));
                return false;
            }
            self.retiring_key.take();
            return false;
        }
        if let Some(key) = self.candidates.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if self.candidates.retire_backing() {
            return false;
        }
        true
    }

    pub fn terminal_owners_empty(&self) -> bool {
        self.candidates.terminal_owners_empty() && self.retiring_key.is_none()
    }
}

impl CollisionIndexMutation {
    pub fn retire_one_owner(&mut self) -> bool {
        if self.id.capacity() != 0 {
            drop(std::mem::take(&mut self.id));
            return false;
        }
        true
    }
}

impl CollisionIndexRemoval {
    /// ♻️ One owner per close grant, exactly as the replacement cursor: the withdrawn owner id is the
    /// removal's only allocation.
    pub fn retire_one_owner(&mut self) -> bool {
        if self.id.capacity() != 0 {
            drop(std::mem::take(&mut self.id));
            return false;
        }
        true
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollisionIndexOwnerCensusStep {
    Pending { items: usize, bytes: usize },
    Complete,
    Rejected,
}

#[cfg(test)]
#[derive(Default)]
pub struct CollisionIndexOwnerCensusCursor {
    section: u8,
    pub(super) index: usize,
    inner: usize,
}

#[cfg(test)]
fn collision_index_string_credit(value: &String) -> Option<(usize, usize)> {
    (value.capacity() <= 16 * 1024).then_some((usize::from(value.capacity() != 0), value.capacity()))
}

#[cfg(test)]
/// 📏️ Credit for a fixed page that may already have been handed back: a container whose backing was
/// retired owns nothing and costs nothing. Only a semantic owner over its declared byte cap is a
/// census refusal, so an index walked WHILE it retires (the close census does exactly that) must not
/// read a released page as "over capacity".
fn collision_index_backing_credit(credit: Option<(usize, usize)>) -> (usize, usize) {
    credit.unwrap_or((0, 0))
}

impl CollisionSpatialIndex {
    const MAX_CELLS_PER_ENTRY: u64 = 4_096;

    pub fn original_empty(cell_size: f32) -> Self {
        Self { cell_size, entries: FixedOwnerMap::refused(), cells: FixedOwnerMap::refused(), oversized: FixedOwnerSet { values: FixedOwnerMap::refused() }, retiring_key: None, retiring_bucket: None }
    }

    pub fn new(cell_size: f32) -> Self {
        assert!(cell_size.is_finite() && cell_size > 0.0);
        Self { cell_size, entries: FixedOwnerMap::new(), cells: FixedOwnerMap::new(), oversized: FixedOwnerSet::new(), retiring_key: None, retiring_bucket: None }
    }

    /// 📐️ World bounds already indexed for one owner id, or `None` when it was never admitted.
    pub fn entry_bounds(&self, id: &str) -> Option<&CollisionAabb> {
        self.entries.get(id)
    }

    /// 🔑️ Every indexed owner id, in the entries page's own sorted order — the incremental scene sync
    /// walks it one id per step to withdraw owners the new scene no longer carries.
    pub fn entry_ids(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub fn entry_len(&self) -> usize {
        self.entries.len()
    }

    pub fn begin_replacement(&self, owner: CollisionIndexOwner, id: String, bounds: CollisionAabb) -> CollisionIndexMutation {
        let old_bounds = self.entries.get(id.as_str()).copied();
        CollisionIndexMutation {
            owner,
            id,
            bounds,
            old_bounds,
            old_span: old_bounds.and_then(|value| CollisionCellSpan::new(self.cell_size, value)),
            new_span: CollisionCellSpan::new(self.cell_size, bounds),
            stage: CollisionMutationStage::PreflightNew,
            cursor: 0,
            missing_cells: 0,
            reclaimed_cells: 0,
        }
    }

    pub fn step_replacement(&mut self, mutation: &mut CollisionIndexMutation, current: CollisionIndexOwner) -> CollisionMutationStep {
        if mutation.owner != current {
            return CollisionMutationStep::Stale;
        }
        precompute_work(1);
        match mutation.stage {
            CollisionMutationStage::PreflightNew => {
                if self.entries.get(mutation.id.as_str()).is_none() && self.entries.len() == self.entries.capacity() {
                    return Self::reject_mutation(mutation);
                }
                if let Some(span) = mutation.new_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        match self.cells.get(&cell) {
                            Some(bucket) if !bucket.contains(mutation.id.as_str()) && bucket.len() == bucket.capacity() => return Self::reject_mutation(mutation),
                            Some(_) => {}
                            None => mutation.missing_cells += 1,
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else if !self.oversized.contains(mutation.id.as_str()) && self.oversized.len() == self.oversized.capacity() {
                    return Self::reject_mutation(mutation);
                }
                mutation.stage = CollisionMutationStage::PreflightOld;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::PreflightOld => {
                if let Some(span) = mutation.old_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        if mutation.new_span.is_none_or(|next| !next.contains(cell)) && self.cells.get(&cell).is_some_and(|bucket| bucket.len() == 1 && bucket.contains(mutation.id.as_str())) {
                            mutation.reclaimed_cells += 1;
                        }
                        return CollisionMutationStep::Pending;
                    }
                }
                if self.cells.len().checked_add(mutation.missing_cells).and_then(|total| total.checked_sub(mutation.reclaimed_cells)).is_none_or(|total| total > self.cells.capacity()) {
                    return Self::reject_mutation(mutation);
                }
                mutation.stage = CollisionMutationStage::Remove;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Remove => {
                if let Some(span) = mutation.old_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        let empty = self.cells.get_mut(&cell).is_some_and(|bucket| {
                            drop(bucket.remove_entry(mutation.id.as_str()));
                            bucket.is_empty()
                        });
                        if empty {
                            drop(self.cells.remove_entry(&cell));
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else if mutation.old_bounds.is_some() {
                    drop(self.oversized.remove_entry(mutation.id.as_str()));
                }
                mutation.stage = CollisionMutationStage::Insert;
                mutation.cursor = 0;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Insert => {
                if let Some(span) = mutation.new_span {
                    if let Some(cell) = span.cell(mutation.cursor) {
                        mutation.cursor += 1;
                        if self.cells.get(&cell).is_none() {
                            let bucket = FixedOwnerSet::new();
                            if bucket.capacity() == 0 {
                                return Self::reject_mutation(mutation);
                            }
                            match self.cells.try_insert(cell, bucket) {
                                Ok(FixedOwnerMapInsert::Inserted) => {}
                                Ok(FixedOwnerMapInsert::Occupied { .. }) => {}
                                Err((_, bucket)) => {
                                    drop(bucket);
                                    return Self::reject_mutation(mutation);
                                }
                            }
                        }
                        let Some(bucket) = self.cells.get_mut(&cell) else { return Self::reject_mutation(mutation) };
                        match bucket.try_insert(mutation.id.clone()) {
                            Ok(FixedOwnerSetInsert::Inserted) => {}
                            Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                            Err(input) => {
                                drop(input);
                                return Self::reject_mutation(mutation);
                            }
                        }
                        return CollisionMutationStep::Pending;
                    }
                } else {
                    match self.oversized.try_insert(mutation.id.clone()) {
                        Ok(FixedOwnerSetInsert::Inserted) => {}
                        Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                        Err(input) => {
                            drop(input);
                            return Self::reject_mutation(mutation);
                        }
                    }
                }
                mutation.stage = CollisionMutationStage::Commit;
                CollisionMutationStep::Pending
            }
            CollisionMutationStage::Commit => {
                drop(self.entries.remove_entry(mutation.id.as_str()));
                let id = std::mem::take(&mut mutation.id);
                match self.entries.try_insert(id, mutation.bounds) {
                    Ok(FixedOwnerMapInsert::Inserted) => {}
                    Ok(FixedOwnerMapInsert::Occupied { input_key, input_value: _ }) | Err((input_key, _)) => {
                        mutation.id = input_key;
                        return Self::reject_mutation(mutation);
                    }
                }
                mutation.stage = CollisionMutationStage::Complete;
                CollisionMutationStep::Complete
            }
            CollisionMutationStage::Complete => CollisionMutationStep::Complete,
            CollisionMutationStage::Rejected => CollisionMutationStep::Rejected(CollisionIndexRejectedOwner::Capacity(String::new())),
        }
    }

    fn reject_mutation(mutation: &mut CollisionIndexMutation) -> CollisionMutationStep {
        mutation.stage = CollisionMutationStage::Rejected;
        CollisionMutationStep::Rejected(CollisionIndexRejectedOwner::Capacity(std::mem::take(&mut mutation.id)))
    }

    pub fn begin_removal(&self, owner: CollisionIndexOwner, id: String) -> Option<CollisionIndexRemoval> {
        let bounds = *self.entries.get(id.as_str())?;
        Some(CollisionIndexRemoval { owner, id, span: CollisionCellSpan::new(self.cell_size, bounds), cursor: 0, complete: false })
    }

    pub fn step_removal(&mut self, removal: &mut CollisionIndexRemoval, current: CollisionIndexOwner) -> CollisionMutationStep {
        if removal.owner != current {
            return CollisionMutationStep::Stale;
        }
        precompute_work(1);
        if removal.complete {
            return CollisionMutationStep::Complete;
        }
        if let Some(span) = removal.span {
            if let Some(cell) = span.cell(removal.cursor) {
                removal.cursor += 1;
                let empty = self.cells.get_mut(&cell).is_some_and(|bucket| {
                    drop(bucket.remove_entry(removal.id.as_str()));
                    bucket.is_empty()
                });
                if empty {
                    drop(self.cells.remove_entry(&cell));
                }
                return CollisionMutationStep::Pending;
            }
        } else {
            drop(self.oversized.remove_entry(removal.id.as_str()));
        }
        drop(self.entries.remove_entry(removal.id.as_str()));
        removal.complete = true;
        CollisionMutationStep::Complete
    }

    pub fn begin_query(&self, owner: CollisionIndexOwner, bounds: CollisionAabb) -> CollisionQueryCursor {
        let span = CollisionCellSpan::new(self.cell_size, bounds);
        CollisionQueryCursor {
            owner,
            bounds,
            span,
            stage: if span.is_some() { CollisionQueryStage::Cells } else { CollisionQueryStage::Entries },
            cell_cursor: 0,
            member_cursor: 0,
            candidates: FixedOwnerSet::new(),
            truncated: false,
            examined_cells: 0,
            examined_members: 0,
            retiring_key: None,
        }
    }

    pub fn step_query(&self, query: &mut CollisionQueryCursor, current: CollisionIndexOwner) -> CollisionQueryStep {
        if query.owner != current {
            return CollisionQueryStep::Stale;
        }
        precompute_work(query.member_cursor + 1);
        let candidate = match query.stage {
            CollisionQueryStage::Cells => {
                let span = query.span.expect("cell query span");
                let Some(cell) = span.cell(query.cell_cursor) else {
                    query.stage = CollisionQueryStage::Oversized;
                    query.member_cursor = 0;
                    return CollisionQueryStep::Pending;
                };
                let Some(bucket) = self.cells.get(&cell) else {
                    query.cell_cursor += 1;
                    query.examined_cells += 1;
                    return CollisionQueryStep::Pending;
                };
                match bucket.iter().nth(query.member_cursor) {
                    Some(id) => {
                        query.member_cursor += 1;
                        Some(id)
                    }
                    None => {
                        query.cell_cursor += 1;
                        query.member_cursor = 0;
                        query.examined_cells += 1;
                        return CollisionQueryStep::Pending;
                    }
                }
            }
            CollisionQueryStage::Oversized => match self.oversized.iter().nth(query.member_cursor) {
                Some(id) => {
                    query.member_cursor += 1;
                    Some(id)
                }
                None => {
                    query.stage = CollisionQueryStage::Complete;
                    return CollisionQueryStep::Complete;
                }
            },
            CollisionQueryStage::Entries => match self.entries.keys().nth(query.member_cursor) {
                Some(id) => {
                    query.member_cursor += 1;
                    Some(id)
                }
                None => {
                    query.stage = CollisionQueryStage::Complete;
                    return CollisionQueryStep::Complete;
                }
            },
            CollisionQueryStage::Complete => return CollisionQueryStep::Complete,
        };
        if let Some(id) = candidate {
            query.examined_members += 1;
            if self.entries.get(id.as_str()).is_some_and(|entry| entry.intersects(&query.bounds)) && !query.candidates.contains(id.as_str()) {
                match query.candidates.try_insert(id.clone()) {
                    Ok(FixedOwnerSetInsert::Inserted) => {}
                    Ok(FixedOwnerSetInsert::Present { input }) => drop(input),
                    Err(input) => {
                        drop(input);
                        query.truncated = true;
                    }
                }
            }
        }
        CollisionQueryStep::Pending
    }

    pub fn retire_one_owner(&mut self) -> bool {
        if let Some(key) = self.retiring_key.as_mut() {
            if key.capacity() != 0 {
                drop(std::mem::take(key));
                return false;
            }
            self.retiring_key.take();
            return false;
        }
        if let Some(bucket) = self.retiring_bucket.as_mut() {
            if let Some(id) = bucket.pop_first() {
                self.retiring_key = Some(id);
                return false;
            }
            if bucket.retire_backing() {
                return false;
            }
            self.retiring_bucket.take();
            return false;
        }
        if let Some((key, _)) = self.entries.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if let Some((_, bucket)) = self.cells.pop_first() {
            self.retiring_bucket = Some(bucket);
            return false;
        }
        if let Some(key) = self.oversized.pop_first() {
            self.retiring_key = Some(key);
            return false;
        }
        if self.entries.retire_backing() {
            return false;
        }
        if self.cells.retire_backing() {
            return false;
        }
        if self.oversized.retire_backing() {
            return false;
        }
        true
    }

    pub fn terminal_owners_empty(&self) -> bool {
        self.entries.terminal_owners_empty() && self.cells.terminal_owners_empty() && self.oversized.terminal_owners_empty() && self.retiring_key.is_none() && self.retiring_bucket.is_none()
    }

    #[cfg(test)]
    pub fn fixed_backing_witness_for_test(&self) -> [(usize, usize, usize); 3] {
        [
            (self.entries.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, CollisionAabb, DOCUMENT_OBJECT_SLOTS>::page_bytes(), self.entries.len()),
            (self.cells.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<(i32, i32, i32), CollisionCellMembers, DOCUMENT_CELL_SLOTS>::page_bytes(), self.cells.len()),
            (self.oversized.backing_ptr().map_or(0, |pointer| pointer.cast::<()>() as usize), FixedOwnerMap::<String, (), DOCUMENT_KIND_SLOTS>::page_bytes(), self.oversized.len()),
        ]
    }

    #[cfg(test)]
    pub fn census_one_owner(&self, cursor: &mut CollisionIndexOwnerCensusCursor) -> CollisionIndexOwnerCensusStep {
        let credit = match cursor.section {
            0 => Some(collision_index_backing_credit(self.entries.backing_credit())),
            1 => match self.entries.keys().nth(cursor.index) {
                Some(id) => collision_index_string_credit(id).and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes))),
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            2 => Some(collision_index_backing_credit(self.cells.backing_credit())),
            3 => match self.cells.values().nth(cursor.index) {
                Some(bucket) if cursor.inner == 0 => {
                    cursor.inner = 1;
                    { let (items, bytes) = collision_index_backing_credit(bucket.backing_credit()); items.checked_add(1).map(|items| (items, bytes)) }
                }
                Some(bucket) => match bucket.iter().nth(cursor.inner - 1) {
                    Some(id) => collision_index_string_credit(id),
                    None => {
                        cursor.index += 1;
                        cursor.inner = 0;
                        return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                    }
                },
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    cursor.inner = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            4 => Some(collision_index_backing_credit(self.oversized.backing_credit())),
            5 => match self.oversized.iter().nth(cursor.index) {
                Some(id) => collision_index_string_credit(id).and_then(|(items, bytes)| items.checked_add(1).map(|items| (items, bytes))),
                None => {
                    cursor.section += 1;
                    cursor.index = 0;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                }
            },
            6 => self.retiring_key.as_ref().map_or(Some((0, 0)), collision_index_string_credit),
            7 => {
                let Some(bucket) = &self.retiring_bucket else {
                    cursor.section += 1;
                    return CollisionIndexOwnerCensusStep::Pending { items: 0, bytes: 0 };
                };
                if cursor.inner == 0 {
                    cursor.inner = 1;
                    Some(collision_index_backing_credit(bucket.backing_credit()))
                } else {
                    match bucket.iter().nth(cursor.inner - 1) {
                        Some(id) => collision_index_string_credit(id),
                        None => {
                            cursor.section += 1;
                            Some((0, 0))
                        }
                    }
                }
            }
            _ => return CollisionIndexOwnerCensusStep::Complete,
        };
        let Some((items, bytes)) = credit else { return CollisionIndexOwnerCensusStep::Rejected };
        if matches!(cursor.section, 1 | 5) {
            cursor.index += 1;
        } else if matches!(cursor.section, 3 | 7) && cursor.inner != 0 {
            cursor.inner += 1;
        } else {
            cursor.section += 1;
        }
        CollisionIndexOwnerCensusStep::Pending { items, bytes }
    }

    #[cfg(test)]
    pub(super) fn install_for_test(&mut self, id: &str, bounds: CollisionAabb) -> bool {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let mut mutation = self.begin_replacement(owner, id.to_string(), bounds);
        for _ in 0..20_000 {
            match self.step_replacement(&mut mutation, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => return true,
                CollisionMutationStep::Rejected(_) | CollisionMutationStep::Stale => return false,
            }
        }
        false
    }

    #[cfg(test)]
    pub(super) fn remove_for_test(&mut self, id: &str) -> bool {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let Some(mut removal) = self.begin_removal(owner, id.to_string()) else {
            return false;
        };
        for _ in 0..20_000 {
            match self.step_removal(&mut removal, owner) {
                CollisionMutationStep::Pending => {}
                CollisionMutationStep::Complete => return true,
                CollisionMutationStep::Rejected(_) | CollisionMutationStep::Stale => return false,
            }
        }
        false
    }

    #[cfg(test)]
    pub(super) fn candidates_for_test(&self, bounds: CollisionAabb) -> Vec<String> {
        let owner = CollisionIndexOwner { operation: 1, generation: 1 };
        let mut query = self.begin_query(owner, bounds);
        for _ in 0..20_000 {
            match self.step_query(&mut query, owner) {
                CollisionQueryStep::Pending => {}
                CollisionQueryStep::Complete => return query.candidates.iter().cloned().collect(),
                CollisionQueryStep::Stale => return Vec::new(),
            }
        }
        Vec::new()
    }
}
//#endregion 🗺️BroadPhase
//#endregion 🔖️Collision
