//! 🪞️ Progressive preimages retain only borrowed-owner indices under the original turn authority.
use super::*;

#[derive(Clone, Copy)]
pub(super) enum Origin {
    Snapshot(usize),
    Insert(usize),
}
#[derive(Clone, Copy)]
pub(super) enum Argument {
    Origin(Origin, usize),
    Mutation(usize),
}
#[derive(Clone, Copy, Default)]
pub(super) struct EntityState {
    pub origin: Option<Origin>,
    pub position: usize,
    pub arguments: usize,
    pub name: Option<usize>,
    pub complex: Option<usize>,
}
#[derive(Clone, Copy, Default)]
enum Change {
    #[default]
    None,
    Insert(usize),
    Remove(usize),
}
#[derive(Clone, Copy, Default)]
struct Record {
    id: Option<u64>,
    state: EntityState,
    change: Change,
    argument: Option<usize>,
}
#[derive(Default)]
enum Stage {
    #[default]
    Idle,
    Clear,
    Find,
    Source,
    Replay,
    Resolve,
    ReserveArguments,
    Argument,
    Ready,
}

#[derive(Default)]
pub(super) struct Overlay {
    records: Vec<Record>,
    pub arguments: Vec<Argument>,
    pub before: EntityState,
    pub header: Option<usize>,
    pub argument: Option<Argument>,
    stage: Stage,
    row: usize,
    scan: usize,
    replay: usize,
    source_position: usize,
    slot: usize,
    target: usize,
    entity_count: usize,
    initialized: bool,
}

fn capacity_bytes<T>(count: usize) -> Result<usize, ValueError> {
    count.checked_mul(std::mem::size_of::<T>()).ok_or_else(|| refusal("IFC preimage metadata size overflow"))
}
fn same_header(left: &IfcMutation, right: &IfcMutation) -> bool {
    matches!((left, right), (IfcMutation::SetFileDescription(_), IfcMutation::SetFileDescription(_)) | (IfcMutation::SetFileName(_), IfcMutation::SetFileName(_)) | (IfcMutation::SetFileSchema(_), IfcMutation::SetFileSchema(_)))
}

impl Overlay {
    pub fn begin(&mut self, row: usize) {
        self.row = row;
        self.scan = row;
        self.replay = 0;
        self.source_position = 0;
        self.before = EntityState::default();
        self.header = None;
        self.argument = None;
        self.stage = Stage::Clear;
    }
    pub fn demands(&self, rows: usize) -> Result<RetirementDemand, ValueError> {
        let capacity = if !self.initialized {
            capacity_bytes::<Record>(rows)?
        } else if matches!(self.stage, Stage::ReserveArguments) && self.before.arguments > self.arguments.capacity() {
            capacity_bytes::<Argument>(self.before.arguments)?
        } else {
            0
        };
        let release = if capacity != 0 && self.initialized { capacity_bytes::<Argument>(self.arguments.capacity())? } else { 0 };
        Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Record>() * 2, capacity_bytes: capacity, release_bytes: release, depth: 1, ..Default::default() })
    }
    pub fn advance(&mut self, snapshot: &IfcSnapshot, mutations: &[IfcMutation], cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if !self.initialized {
            self.records.try_reserve_exact(mutations.len()).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC preimage metadata allocation refused"))?;
            self.entity_count = snapshot.entities.len();
            self.initialized = true;
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity_bytes::<Record>(self.records.capacity())?, ..Default::default() })?;
            return Ok(false);
        }
        let mutation = &mutations[self.row];
        let id = entity_id(mutation);
        let mut progress = RetainedCloneProgress { copied_items: 1, copied_bytes: std::mem::size_of::<Record>() * 2, ..Default::default() };
        match self.stage {
            Stage::Idle => return Err(refusal("IFC preimage cursor was not begun").into()),
            Stage::Clear => {
                if self.arguments.pop().is_none() {
                    self.stage = Stage::Find;
                }
            }
            Stage::Find => {
                if self.scan == 0 {
                    self.stage = if id.is_some() { Stage::Source } else { Stage::Resolve };
                } else {
                    self.scan -= 1;
                    if let Some(id) = id {
                        let record = self.records[self.scan];
                        if record.id == Some(id) {
                            self.before = record.state;
                            self.replay = self.scan + 1;
                            self.stage = Stage::Replay;
                        }
                    } else if same_header(&mutations[self.scan], mutation) {
                        self.header = Some(self.scan);
                        self.stage = Stage::Resolve;
                    }
                }
            }
            Stage::Source => {
                if let Some(entity) = snapshot.entities.get(self.source_position) {
                    if Some(entity.id) == id {
                        self.before = EntityState { origin: Some(Origin::Snapshot(self.source_position)), position: self.source_position, arguments: entity.args.len(), ..Default::default() };
                        self.stage = Stage::Replay;
                    } else {
                        self.source_position += 1;
                    }
                } else {
                    self.stage = Stage::Replay;
                }
            }
            Stage::Replay => {
                if self.replay == self.row {
                    self.stage = Stage::Resolve;
                } else {
                    if self.before.origin.is_some() {
                        match self.records[self.replay].change {
                            Change::Insert(index) if index <= self.before.position => self.before.position = self.before.position.checked_add(1).ok_or_else(|| refusal("IFC preimage entity position overflow"))?,
                            Change::Remove(index) if index < self.before.position => self.before.position -= 1,
                            _ => {}
                        }
                    }
                    self.replay += 1;
                }
            }
            Stage::Resolve => {
                let mut record = Record { id, state: self.before, ..Default::default() };
                match mutation {
                    IfcMutation::InsertEntity(value) => {
                        if self.before.origin.is_some() {
                            return Err(refusal("IFC insertion reuses an existing entity identity").into());
                        }
                        let position = value.index.min(self.entity_count);
                        record.state = EntityState { origin: Some(Origin::Insert(self.row)), position, arguments: value.entity.args.len(), ..Default::default() };
                        record.change = Change::Insert(position);
                        self.entity_count = self.entity_count.checked_add(1).ok_or_else(|| refusal("IFC preimage entity count overflow"))?;
                    }
                    IfcMutation::RemoveEntity(_) if self.before.origin.is_some() => {
                        record.state.origin = None;
                        record.change = Change::Remove(self.before.position);
                        self.entity_count -= 1;
                    }
                    IfcMutation::SetEntityName(_) if self.before.origin.is_some() => record.state.name = Some(self.row),
                    IfcMutation::SetEntityComplex(_) if self.before.origin.is_some() => record.state.complex = Some(self.row),
                    IfcMutation::InsertEntityArg(value) if self.before.origin.is_some() => {
                        record.argument = Some(value.index.min(self.before.arguments));
                        record.state.arguments = record.state.arguments.checked_add(1).ok_or_else(|| refusal("IFC preimage argument count overflow"))?;
                    }
                    IfcMutation::RemoveEntityArg(value) if self.before.origin.is_some() && value.index < self.before.arguments => {
                        record.argument = Some(value.index);
                        record.state.arguments -= 1;
                    }
                    _ => {}
                }
                self.records.push(record);
                match mutation {
                    IfcMutation::RemoveEntity(_) if self.before.origin.is_some() => self.stage = Stage::ReserveArguments,
                    IfcMutation::SetEntityArg(value) if self.before.origin.is_some() && value.index < self.before.arguments => {
                        self.target = value.index;
                        self.slot = value.index;
                        self.scan = self.row;
                        self.stage = Stage::Argument;
                    }
                    IfcMutation::RemoveEntityArg(value) if self.before.origin.is_some() && value.index < self.before.arguments => {
                        self.target = value.index;
                        self.slot = value.index;
                        self.scan = self.row;
                        self.stage = Stage::Argument;
                    }
                    _ => self.stage = Stage::Ready,
                }
            }
            Stage::ReserveArguments => {
                if self.before.arguments > self.arguments.capacity() {
                    let released = capacity_bytes::<Argument>(self.arguments.capacity())?;
                    self.arguments.try_reserve_exact(self.before.arguments).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "IFC preimage argument metadata allocation refused"))?;
                    progress.retained_capacity_bytes = capacity_bytes::<Argument>(self.arguments.capacity())?;
                    progress.released_bytes = released;
                }
                if self.before.arguments == 0 {
                    self.stage = Stage::Ready;
                } else {
                    self.target = 0;
                    self.slot = 0;
                    self.scan = self.row;
                    self.stage = Stage::Argument;
                }
            }
            Stage::Argument => {
                let origin = self.before.origin.ok_or_else(|| refusal("IFC immediate entity origin missing"))?;
                let finished = match origin {
                    Origin::Insert(row) => self.scan == row + 1,
                    Origin::Snapshot(_) => self.scan == 0,
                };
                let mut owner = if finished { Some(Argument::Origin(origin, self.slot)) } else { None };
                if !finished {
                    self.scan -= 1;
                    let prior = &mutations[self.scan];
                    if entity_id(prior) == id {
                        match prior {
                            IfcMutation::SetEntityArg(value) if value.index == self.slot => owner = Some(Argument::Mutation(self.scan)),
                            IfcMutation::InsertEntityArg(_) => {
                                if let Some(index) = self.records[self.scan].argument {
                                    if self.slot == index {
                                        owner = Some(Argument::Mutation(self.scan));
                                    } else if self.slot > index {
                                        self.slot -= 1;
                                    }
                                }
                            }
                            IfcMutation::RemoveEntityArg(_) => {
                                if let Some(index) = self.records[self.scan].argument {
                                    if self.slot >= index {
                                        self.slot = self.slot.checked_add(1).ok_or_else(|| refusal("IFC preimage argument position overflow"))?;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if let Some(owner) = owner {
                    if matches!(mutation, IfcMutation::RemoveEntity(_)) {
                        self.arguments.push(owner);
                        self.target += 1;
                        if self.target == self.before.arguments {
                            self.stage = Stage::Ready;
                        } else {
                            self.slot = self.target;
                            self.scan = self.row;
                        }
                    } else {
                        self.argument = Some(owner);
                        self.stage = Stage::Ready;
                    }
                }
            }
            Stage::Ready => return Ok(true),
        }
        cx.consume_retained(progress)?;
        Ok(false)
    }
    pub fn has_metadata(&self) -> bool {
        self.records.capacity() != 0 || self.arguments.capacity() != 0
    }
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        let release = if self.arguments.is_empty() && self.arguments.capacity() != 0 {
            capacity_bytes::<Argument>(self.arguments.capacity())?
        } else if self.arguments.capacity() == 0 && self.records.is_empty() {
            capacity_bytes::<Record>(self.records.capacity())?
        } else {
            0
        };
        Ok(RetirementDemand { release_bytes: release, depth: 1, ..Default::default() })
    }
    pub fn close(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if grant.maximum_items == 0 || self.close_demands()?.release_bytes > grant.maximum_release_bytes || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.arguments.pop().is_some() {
        } else if self.arguments.capacity() != 0 {
            progress.released_bytes = capacity_bytes::<Argument>(self.arguments.capacity())?;
            self.arguments = Vec::new();
        } else if self.records.pop().is_some() {
        } else if self.records.capacity() != 0 {
            progress.released_bytes = capacity_bytes::<Record>(self.records.capacity())?;
            self.records = Vec::new();
        } else {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        Ok(if self.has_metadata() { RetainedCloneStep::Progress(progress) } else { RetainedCloneStep::Complete(progress) })
    }
    pub fn entity<'a>(&self, origin: Origin, snapshot: &'a IfcSnapshot, mutations: &'a [IfcMutation]) -> Result<&'a IfcEntity, ValueError> {
        match origin {
            Origin::Snapshot(index) => snapshot.entities.get(index),
            Origin::Insert(row) => match mutations.get(row) {
                Some(IfcMutation::InsertEntity(value)) => Some(&value.entity),
                _ => None,
            },
        }
        .ok_or_else(|| refusal("IFC immediate entity owner missing"))
    }
    pub fn value<'a>(&self, owner: Argument, snapshot: &'a IfcSnapshot, mutations: &'a [IfcMutation]) -> Result<&'a IfcValue, ValueError> {
        match owner {
            Argument::Origin(origin, index) => self.entity(origin, snapshot, mutations)?.args.get(index),
            Argument::Mutation(row) => match mutations.get(row) {
                Some(IfcMutation::SetEntityArg(value)) => Some(&value.value),
                Some(IfcMutation::InsertEntityArg(value)) => Some(&value.value),
                _ => None,
            },
        }
        .ok_or_else(|| refusal("IFC immediate argument owner missing"))
    }
}
