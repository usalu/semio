//! 🪟 The resulting population borrows immutable original graph and final template mutation coordinates.
use super::retained::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{
    retained_clone::{RetainedCloneClose, RetainedCloneGrant, RetainedCloneStep},
    RetirementDemand, ValueError,
};
pub(super) use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcSnapshot, IfcValue};
pub(super) use semio_s_artifact_stdio_ifc_4_mutations::standards::v4::subsets::any::schema::mutations::IfcMutation;
use std::mem::size_of;

#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
enum Origin {
    Original(usize),
    Inserted(usize),
}
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
struct Row {
    id: u64,
    origin: Origin,
}
#[derive(Clone, Copy, Debug, semio_framework_value::RetireOwned)]
struct Patch {
    id: u64,
    argument: usize,
    mutation: usize,
}
#[derive(Default, semio_framework_value::RetireOwned)]
struct Storage {
    rows: Vec<Row>,
    scratch: Vec<Row>,
    patches: Vec<Patch>,
}
#[derive(Clone, Copy, Default)]
enum Phase {
    #[default]
    Allocate,
    Originals,
    Insertions,
    Merge,
    Duplicates,
    Patches,
    PatchOrder,
    Ready,
}

#[derive(Default)]
pub struct PopulationOverlay {
    storage: Option<Storage>,
    retirement: RetainedCloneClose,
    source: usize,
    source_entities: usize,
    source_length: usize,
    mutations: usize,
    mutation_length: usize,
    phase: Phase,
    position: usize,
    inserted: usize,
    width: usize,
    base: usize,
    left: usize,
    right: usize,
    middle: usize,
    end: usize,
    output: usize,
    closing: bool,
}
impl PopulationOverlay {
    /// 🪪 Refuse a different source or mutation owner even when its graph values compare equal.
    pub fn check_authority(&self, source: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<(), ValueError> {
        if self.source != source as *const IfcSnapshot as usize
            || self.source_entities != source.entities.as_ptr() as usize
            || self.source_length != source.entities.len()
            || self.mutations != mutations.as_ptr() as usize
            || self.mutation_length != mutations.len()
        {
            return Err(refusal("express.overlay-authority-changed"));
        }
        Ok(())
    }
    /// 📐 Quote the next actual contiguous allocation or one metadata transition.
    pub fn demands(&self, source: &IfcSnapshot, mutations: &[IfcMutation]) -> Result<RetirementDemand, ValueError> {
        let mut demand = fixed(4 * size_of::<Row>() + 12 * size_of::<usize>());
        if matches!(self.phase, Phase::Allocate) {
            let maximum = source.entities.len().checked_add(mutations.len()).ok_or_else(|| refusal("express.overlay-count-overflow"))?;
            demand.capacity_bytes = capacity(maximum, 2 * size_of::<Row>())?.checked_add(capacity(mutations.len(), size_of::<Patch>())?).ok_or_else(|| refusal("express.overlay-capacity-overflow"))?;
        }
        Ok(demand)
    }
    fn storage(&self) -> &Storage {
        self.storage.as_ref().expect("live retained overlay storage")
    }
    fn storage_mut(&mut self) -> &mut Storage {
        self.storage.as_mut().expect("live retained overlay storage")
    }
    fn merge_range(&mut self) {
        self.middle = self.base.saturating_add(self.width).min(self.len());
        self.end = self.middle.saturating_add(self.width).min(self.len());
        self.left = self.base;
        self.right = self.middle;
        self.output = self.base;
    }
    /// 🧭 Advance one original row, merge cell, duplicate check or mutation coordinate.
    pub fn advance(&mut self, source: &IfcSnapshot, mutations: &[IfcMutation], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        if self.closing {
            return Err(refusal("express.overlay-closing").into());
        }
        let demand = self.demands(source, mutations)?;
        if !admitted(cx, demand)? {
            return Ok(false);
        }
        if !matches!(self.phase, Phase::Allocate) {
            self.check_authority(source, mutations)?;
        }
        match self.phase {
            Phase::Allocate => {
                let maximum = source.entities.len() + mutations.len();
                self.storage = Some(Storage { rows: Vec::with_capacity(maximum), scratch: Vec::with_capacity(maximum), patches: Vec::with_capacity(mutations.len()) });
                self.source = source as *const IfcSnapshot as usize;
                self.source_entities = source.entities.as_ptr() as usize;
                self.source_length = source.entities.len();
                self.mutations = mutations.as_ptr() as usize;
                self.mutation_length = mutations.len();
                self.phase = Phase::Originals;
            }
            Phase::Originals => {
                if let Some(entity) = source.entities.get(self.position) {
                    if entity.id == 0 {
                        return Err(refusal("express.overlay-invalid-id").into());
                    }
                    let row = Row { id: entity.id, origin: Origin::Original(self.position) };
                    self.storage_mut().rows.push(row);
                    self.storage_mut().scratch.push(row);
                    self.position += 1;
                } else {
                    self.position = 0;
                    self.phase = Phase::Insertions;
                }
            }
            Phase::Insertions => {
                if let Some(mutation) = mutations.get(self.position) {
                    match mutation {
                        IfcMutation::InsertEntity(change) => {
                            if change.index != source.entities.len() + self.inserted || change.entity.id == 0 {
                                return Err(refusal("express.overlay-insert-not-final").into());
                            }
                            let row = Row { id: change.entity.id, origin: Origin::Inserted(self.position) };
                            self.storage_mut().rows.push(row);
                            self.storage_mut().scratch.push(row);
                            self.inserted += 1;
                        }
                        IfcMutation::SetEntityArg(_) => {}
                        _ => return Err(refusal("express.overlay-mutation-shape").into()),
                    }
                    self.position += 1;
                } else {
                    self.width = 1;
                    self.base = 0;
                    self.merge_range();
                    self.phase = Phase::Merge;
                }
            }
            Phase::Merge => {
                if self.width >= self.len() {
                    self.position = 0;
                    self.phase = Phase::Duplicates;
                } else if self.output < self.end {
                    let take_left = self.left < self.middle && (self.right >= self.end || self.storage().rows[self.left].id <= self.storage().rows[self.right].id);
                    let selected = if take_left {
                        let index = self.left;
                        self.left += 1;
                        index
                    } else {
                        let index = self.right;
                        self.right += 1;
                        index
                    };
                    let row = self.storage().rows[selected];
                    let output = self.output;
                    self.storage_mut().scratch[output] = row;
                    self.output += 1;
                } else {
                    self.base = self.end;
                    if self.base >= self.len() {
                        let storage = self.storage_mut();
                        std::mem::swap(&mut storage.rows, &mut storage.scratch);
                        self.width = self.width.checked_mul(2).ok_or_else(|| refusal("express.overlay-merge-overflow"))?;
                        self.base = 0;
                    }
                    self.merge_range();
                }
            }
            Phase::Duplicates => {
                if self.position + 1 < self.len() {
                    if self.storage().rows[self.position].id == self.storage().rows[self.position + 1].id {
                        return Err(refusal("express.overlay-duplicate-id").into());
                    }
                    self.position += 1;
                } else {
                    self.position = 0;
                    self.phase = Phase::Patches;
                }
            }
            Phase::Patches => {
                if let Some(mutation) = mutations.get(self.position) {
                    if let IfcMutation::SetEntityArg(change) = mutation {
                        let index = self.find(change.id).ok_or_else(|| refusal("express.overlay-missing-target"))?;
                        if !matches!(self.storage().rows[index].origin, Origin::Original(_)) {
                            return Err(refusal("express.overlay-insert-then-edit").into());
                        }
                        if change.index >= self.entity(source, mutations, index)?.args.len() {
                            return Err(refusal("express.overlay-missing-argument").into());
                        }
                        let patch = Patch { id: change.id, argument: change.index, mutation: self.position };
                        self.storage_mut().patches.push(patch);
                    }
                    self.position += 1;
                } else {
                    self.position = 1;
                    self.left = 1;
                    self.phase = Phase::PatchOrder;
                }
            }
            Phase::PatchOrder => {
                if self.position >= self.storage().patches.len() {
                    self.phase = Phase::Ready;
                } else if self.left == 0 {
                    self.position += 1;
                    self.left = self.position;
                } else {
                    let a = self.storage().patches[self.left - 1];
                    let b = self.storage().patches[self.left];
                    if (a.id, a.argument) == (b.id, b.argument) {
                        return Err(refusal("express.overlay-duplicate-slot").into());
                    }
                    if (a.id, a.argument) > (b.id, b.argument) {
                        let left = self.left;
                        self.storage_mut().patches.swap(left - 1, left);
                        self.left -= 1;
                    } else {
                        self.position += 1;
                        self.left = self.position;
                    }
                }
            }
            Phase::Ready => {}
        }
        debit(cx, demand)?;
        Ok(matches!(self.phase, Phase::Ready))
    }
    pub fn len(&self) -> usize {
        self.storage.as_ref().map_or(0, |storage| storage.rows.len())
    }
    pub fn is_ready(&self) -> bool {
        matches!(self.phase, Phase::Ready) && !self.closing
    }
    pub fn find(&self, id: u64) -> Option<usize> {
        self.storage.as_ref()?.rows.binary_search_by_key(&id, |row| row.id).ok()
    }
    /// 👁 Borrow the actual original or inserted entity; no native graph DTO is constructed.
    pub fn entity<'a>(&self, source: &'a IfcSnapshot, mutations: &'a [IfcMutation], index: usize) -> Result<&'a IfcEntity, ValueError> {
        self.check_authority(source, mutations)?;
        match self.storage.as_ref().and_then(|storage| storage.rows.get(index)).ok_or_else(|| refusal("express.overlay-row"))?.origin {
            Origin::Original(index) => source.entities.get(index).ok_or_else(|| refusal("express.overlay-original")),
            Origin::Inserted(index) => {
                if let Some(IfcMutation::InsertEntity(change)) = mutations.get(index) {
                    Ok(&change.entity)
                } else {
                    Err(refusal("express.overlay-insert"))
                }
            }
        }
    }
    /// 👁 Borrow the final argument from either retained owner using a sorted patch lookup.
    pub fn argument<'a>(&self, source: &'a IfcSnapshot, mutations: &'a [IfcMutation], row: usize, argument: usize) -> Result<&'a IfcValue, ValueError> {
        let entity = self.entity(source, mutations, row)?;
        if let Ok(patch) = self.storage().patches.binary_search_by_key(&(entity.id, argument), |patch| (patch.id, patch.argument)) {
            if let Some(IfcMutation::SetEntityArg(change)) = mutations.get(self.storage().patches[patch].mutation) {
                return Ok(&change.value);
            }
            return Err(refusal("express.overlay-patch"));
        }
        entity.args.get(argument).ok_or_else(|| refusal("express.overlay-argument"))
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
    }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        self.retirement.next_owner_demands::<Storage>(self.storage.is_some(), &None, body)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing {
            return Err(refusal("express.overlay-not-closing"));
        }
        if self.storage.is_some() {
            self.retirement.begin_granted(&mut self.storage, grant)?.ok_or_else(|| refusal("express.overlay-close-owner"))
        } else if !self.retirement.is_empty() {
            self.retirement.step_granted(grant)
        } else {
            Ok(RetainedCloneStep::Complete(Default::default()))
        }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.storage.is_none() && self.retirement.is_empty()
    }
}
