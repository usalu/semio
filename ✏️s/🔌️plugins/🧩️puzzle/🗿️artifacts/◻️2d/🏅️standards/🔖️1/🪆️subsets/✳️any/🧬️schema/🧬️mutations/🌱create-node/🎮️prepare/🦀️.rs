//! 🎮️ Borrowed create semantics validate one native field before bounded duplicate lookup.

use super::CreateNode;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor, Puzzle2dLookupScope, Puzzle2dLookupStep}};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dCreateNodeDisposition { Changed, DuplicateId, NonfiniteX, NonfiniteY, InvalidShape, InvalidRadius, InvalidWidth, InvalidHeight, InvalidScale, InvalidHandleAngle, InvalidHandleRadius, InvalidHandleScale }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Puzzle2dCreateNodePlan {
    pub disposition: Puzzle2dCreateNodeDisposition,
    pub position: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dCreateNodePreparationStep { Pending(RetainedCloneProgress), Complete { plan: Puzzle2dCreateNodePlan, progress: RetainedCloneProgress } }

pub struct Puzzle2dCreateNodePreparationCursor {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    lookup: Puzzle2dLookupCursor,
    phase: u8,
    handle: usize,
    handle_field: u8,
    disposition: Puzzle2dCreateNodeDisposition,
    output: Option<Puzzle2dCreateNodePlan>,
    spent: bool,
    closing: bool,
}

impl Default for Puzzle2dCreateNodePreparationCursor {
    fn default() -> Self { Self { source: None, mutation: None, lookup: Puzzle2dLookupCursor::new(Puzzle2dLookupScope::Node), phase: 0, handle: 0, handle_field: 0, disposition: Puzzle2dCreateNodeDisposition::Changed, output: None, spent: false, closing: false } }
}

impl Puzzle2dCreateNodePreparationCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, CreateNode>, grant: RetainedCloneGrant) -> Result<Puzzle2dCreateNodePreparationStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "create preparation is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dCreateNodePreparationStep::Pending(Default::default())); }
        source.bind(&mut self.source)?;
        mutation.bind(&mut self.mutation)?;
        if let Some(plan) = self.output { return Ok(Puzzle2dCreateNodePreparationStep::Complete { plan, progress: Default::default() }); }
        let node = &mutation.get().node;
        if self.phase < 7 || self.phase == 7 && self.handle < node.handles.len() {
            let bytes = match self.phase { 0 | 1 => std::mem::size_of::<f64>(), 2 => node.shape.as_ref().map_or(0, |shape| if shape.len() == 6 || shape.len() == 9 { shape.len() } else { 0 }), 7 if self.handle_field == 0 => std::mem::size_of::<f64>(), _ => std::mem::size_of::<Option<f64>>() };
            if grant.maximum_copy_bytes < bytes { return Ok(Puzzle2dCreateNodePreparationStep::Pending(Default::default())); }
            let invalid = match self.phase {
                0 => (!node.x.is_finite()).then_some(Puzzle2dCreateNodeDisposition::NonfiniteX),
                1 => (!node.y.is_finite()).then_some(Puzzle2dCreateNodeDisposition::NonfiniteY),
                2 => node.shape.as_ref().filter(|shape| !(shape.eq_str("circle") || shape.eq_str("rectangle"))).map(|_| Puzzle2dCreateNodeDisposition::InvalidShape),
                3 => node.radius.filter(|value| !value.is_finite() || *value <= 0.0).map(|_| Puzzle2dCreateNodeDisposition::InvalidRadius),
                4 => node.width.filter(|value| !value.is_finite() || *value <= 0.0).map(|_| Puzzle2dCreateNodeDisposition::InvalidWidth),
                5 => node.height.filter(|value| !value.is_finite() || *value <= 0.0).map(|_| Puzzle2dCreateNodeDisposition::InvalidHeight),
                6 => node.scale.filter(|value| !value.is_finite() || *value <= 0.0).map(|_| Puzzle2dCreateNodeDisposition::InvalidScale),
                _ => { let handle = node.handles.get(self.handle).expect("immutable create preparation handle"); match self.handle_field { 0 => (!handle.angle.is_finite()).then_some(Puzzle2dCreateNodeDisposition::InvalidHandleAngle), 1 => handle.radius.filter(|value| !value.is_finite() || *value <= 0.0).map(|_| Puzzle2dCreateNodeDisposition::InvalidHandleRadius), _ => handle.scale.filter(|value| !value.is_finite() || *value <= 0.0).map(|_| Puzzle2dCreateNodeDisposition::InvalidHandleScale) } },
            };
            if let Some(invalid) = invalid { self.disposition = invalid; self.phase = 9; }
            else if self.phase < 7 { self.phase += 1; }
            else if self.handle_field < 2 { self.handle_field += 1; }
            else { self.handle += 1; self.handle_field = 0; }
            return Ok(Puzzle2dCreateNodePreparationStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        if self.phase == 7 { self.phase = 8; return Ok(Puzzle2dCreateNodePreparationStep::Pending(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
        if self.phase == 8 {
            return match self.lookup.advance(source, mutation.project(1, |payload| &payload.node).project(1, |node| &node.id), grant)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dCreateNodePreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => { if location.is_some() { self.disposition = Puzzle2dCreateNodeDisposition::DuplicateId; } self.lookup.take(); self.phase = 9; Ok(Puzzle2dCreateNodePreparationStep::Pending(progress)) },
            };
        }
        let bytes = std::mem::size_of::<Option<usize>>() + std::mem::size_of::<usize>();
        if grant.maximum_copy_bytes < bytes { return Ok(Puzzle2dCreateNodePreparationStep::Pending(Default::default())); }
        let position = (self.disposition == Puzzle2dCreateNodeDisposition::Changed).then(|| mutation.get().index.unwrap_or(source.get().nodes.len()).min(source.get().nodes.len()));
        let plan = Puzzle2dCreateNodePlan { disposition: self.disposition, position };
        self.output = Some(plan);
        Ok(Puzzle2dCreateNodePreparationStep::Complete { plan, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } })
    }

    pub fn take(&mut self) -> Option<Puzzle2dCreateNodePlan> { if self.closing { return None; } let output = self.output.take(); if output.is_some() { self.spent = true; } output }

    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { if self.output.is_some(){return Ok(std::mem::size_of_val(&self.output));}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_copy_byte_demand();}RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation}) }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if self.output.is_some(){return Ok(0);}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_capacity_byte_demand(body);}RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { if self.output.is_some(){return Ok(0);}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_release_byte_demand();}RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation}) }
    pub fn next_close_depth_demand(&self) -> Result<usize, ValueError> { if self.output.is_some(){return Ok(1);}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_depth_demand();}RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation}) }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "create preparation closure was not started")); }
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}if grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.output.is_some(){let bytes=std::mem::size_of_val(&self.output);if grant.maximum_copy_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}self.output=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}));}
        if !self.lookup.terminal_is_empty(){return self.lookup.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        let step=RetainedCloneBinding::close_one(if self.source.is_some(){&mut self.source}else{&mut self.mutation},grant)?;Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.output.is_none() && self.lookup.terminal_is_empty() && self.source.is_none() && self.mutation.is_none() }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path = "↩️inverse/🦀️.rs"]
pub mod inverse;

#[path = "📸️candidate/🦀️.rs"]
pub mod candidate;
