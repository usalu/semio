//! 🎮️ Borrowed connection preparation retains exact scalar priority and proximity evidence.

use super::ConnectHandles;
use crate::{Puzzle2dSnapshot, standards::v1::subsets::any::schema::snapshot::lookup::{Puzzle2dLookupCursor, Puzzle2dLookupLocation, Puzzle2dLookupScope, Puzzle2dLookupStep}};
use semio_framework_value::{ValueError, ValueRefusalKind, retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dConnectHandlesDisposition { Changed, DuplicateId, Nonfinite(u8), NegativeTolerance }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dConnectHandlesWarning { None, MissingHandle, TooFar }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Puzzle2dConnectHandlesPlan {
    pub disposition: Puzzle2dConnectHandlesDisposition,
    pub warning: Puzzle2dConnectHandlesWarning,
    pub position: Option<usize>,
    pub source_handle: Option<Puzzle2dLookupLocation>,
    pub target_handle: Option<Puzzle2dLookupLocation>,
    pub distance: Option<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Puzzle2dConnectHandlesPreparationStep { Pending(RetainedCloneProgress), Complete { plan: Puzzle2dConnectHandlesPlan, progress: RetainedCloneProgress } }

pub struct Puzzle2dConnectHandlesPreparationCursor {
    source: Option<RetainedCloneBinding>,
    mutation: Option<RetainedCloneBinding>,
    lookup: Puzzle2dLookupCursor,
    phase: u8,
    disposition: Puzzle2dConnectHandlesDisposition,
    warning: Puzzle2dConnectHandlesWarning,
    source_handle: Option<Puzzle2dLookupLocation>,
    target_handle: Option<Puzzle2dLookupLocation>,
    source_point: Option<(f64, f64)>,
    target_point: Option<(f64, f64)>,
    distance: Option<f64>,
    output: Option<Puzzle2dConnectHandlesPlan>,
    spent: bool,
    closing: bool,
}

impl Default for Puzzle2dConnectHandlesPreparationCursor {
    fn default() -> Self { Self { source: None, mutation: None, lookup: Puzzle2dLookupCursor::new(Puzzle2dLookupScope::Edge), phase: 0, disposition: Puzzle2dConnectHandlesDisposition::Changed, warning: Puzzle2dConnectHandlesWarning::None, source_handle: None, target_handle: None, source_point: None, target_point: None, distance: None, output: None, spent: false, closing: false } }
}

impl Puzzle2dConnectHandlesPreparationCursor {
    pub fn advance(&mut self, source: RetainedCloneRef<'_, Puzzle2dSnapshot>, mutation: RetainedCloneRef<'_, ConnectHandles>, grant: RetainedCloneGrant) -> Result<Puzzle2dConnectHandlesPreparationStep, ValueError> {
        if self.closing || self.spent { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "connect preparation is closing or spent")); }
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(Default::default())); }
        if let Some(progress)=source.bind(&mut self.source,grant)?{return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(progress))}
        if let Some(progress)=mutation.bind(&mut self.mutation,grant)?{return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(progress))}
        if let Some(plan) = self.output { return Ok(Puzzle2dConnectHandlesPreparationStep::Complete { plan, progress: Default::default() }); }
        if self.phase < 10 {
            if grant.maximum_copy_bytes < 8 { return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(Default::default())); }
            let payload = mutation.get();
            if self.phase < 9 {
                let value = match self.phase { 0 => payload.gap, 1 => payload.shift, 2 => payload.rise, 3 => payload.rotation, 4 => payload.turn, 5 => payload.tilt, 6 => payload.x, 7 => payload.y, _ => payload.tolerance.unwrap_or(0.0) };
                if !value.is_finite() { self.disposition = Puzzle2dConnectHandlesDisposition::Nonfinite(self.phase); self.phase = 20; } else { self.phase += 1; }
            } else if payload.tolerance.is_some_and(|value| value < 0.0) { self.disposition = Puzzle2dConnectHandlesDisposition::NegativeTolerance; self.phase = 20; } else { self.phase = 10; }
            return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: 8, ..Default::default() }));
        }
        if matches!(self.phase, 10 | 12 | 14) {
            let phase = self.phase;
            let target = mutation.project(phase as usize + 1, |payload| match phase { 10 => &payload.id, 12 => &payload.source, _ => &payload.target });
            return match self.lookup.advance(source, target, grant)? {
                Puzzle2dLookupStep::Pending(progress) => Ok(Puzzle2dConnectHandlesPreparationStep::Pending(progress)),
                Puzzle2dLookupStep::Complete { location, progress } => {
                    self.lookup.take();
                    if phase == 10 && location.is_some() { self.disposition = Puzzle2dConnectHandlesDisposition::DuplicateId; }
                    else if phase == 12 { self.source_handle = location; }
                    else if phase == 14 { self.target_handle = location; }
                    self.lookup.begin_close();
                    self.phase += 1;
                    Ok(Puzzle2dConnectHandlesPreparationStep::Pending(progress))
                }
            };
        }
        if matches!(self.phase, 11 | 13 | 15) {
            let step=self.lookup.close_step(grant)?;
            if self.lookup.terminal_is_empty() {
                    self.lookup = Puzzle2dLookupCursor::new(Puzzle2dLookupScope::Handle);
                    self.phase = if self.phase == 11 { if self.disposition != Puzzle2dConnectHandlesDisposition::Changed || mutation.get().tolerance.is_none() { 20 } else { 12 } } else { self.phase + 1 };
            }
            return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(step.progress()));
        }
        if matches!(self.phase, 16 | 17) {
            let location = if self.phase == 16 { self.source_handle } else { self.target_handle };
            let bytes = if location.is_some() { 96 } else { 0 };
            if grant.maximum_copy_bytes < bytes { return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(Default::default())); }
            let point = location.map(|location| {
                let node = source.get().nodes.get(location.outer).expect("immutable connected handle node");
                let handle = node.handles.get(location.inner.expect("native handle ordinal")).expect("immutable connected handle");
                let center = semio_framework_geometry::Point::new(node.x, node.y);
                let point = if node.shape.as_ref().is_some_and(|shape| shape.eq_str("rectangle")) {
                    semio_framework_graph::drawing::routing::handle_position_on_rectangle(center, node.width.unwrap_or(48.0), node.height.unwrap_or(48.0), handle.angle)
                } else {
                    semio_framework_graph::drawing::routing::handle_position_on_circle(center, node.radius.unwrap_or(24.0), handle.angle)
                };
                (point.x, point.y)
            });
            if self.phase == 16 { self.source_point = point; } else { self.target_point = point; }
            self.phase += 1;
            return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
        }
        if self.phase == 18 {
            if grant.maximum_copy_bytes < 40 { return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(Default::default())); }
            match (self.source_point, self.target_point) {
                (Some((sx, sy)), Some((tx, ty))) => { let distance = (tx - sx).hypot(ty - sy); self.distance = Some(distance); if !(distance <= mutation.get().tolerance.expect("recorded connection tolerance")) { self.warning = Puzzle2dConnectHandlesWarning::TooFar; } }
                _ => self.warning = Puzzle2dConnectHandlesWarning::MissingHandle,
            }
            self.phase = 20;
            return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(RetainedCloneProgress { copied_items: 1, copied_bytes: 40, ..Default::default() }));
        }
        let bytes = std::mem::size_of::<Puzzle2dConnectHandlesPlan>();
        if grant.maximum_copy_bytes < bytes { return Ok(Puzzle2dConnectHandlesPreparationStep::Pending(Default::default())); }
        let position = (self.disposition == Puzzle2dConnectHandlesDisposition::Changed).then(|| mutation.get().index.unwrap_or(source.get().edges.len()).min(source.get().edges.len()));
        let plan = Puzzle2dConnectHandlesPlan { disposition: self.disposition, warning: self.warning, position, source_handle: self.source_handle, target_handle: self.target_handle, distance: self.distance };
        self.output = Some(plan);
        Ok(Puzzle2dConnectHandlesPreparationStep::Complete { plan, progress: RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() } })
    }

    pub fn take(&mut self) -> Option<Puzzle2dConnectHandlesPlan> { if self.closing { return None; } let output = self.output.take(); if output.is_some() { self.spent = true; } output }
    pub fn begin_close(&mut self) { self.closing = true; self.lookup.begin_close(); }
    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if self.output.is_some(){return Ok(std::mem::size_of_val(&self.output));}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_copy_byte_demand();}RetainedCloneBinding::copy_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{if self.output.is_some(){return Ok(0);}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_capacity_byte_demand(body);}RetainedCloneBinding::capacity_demand(if self.source.is_some(){&self.source}else{&self.mutation},body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if self.output.is_some(){return Ok(0);}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_release_byte_demand();}RetainedCloneBinding::release_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{if self.output.is_some(){return Ok(1);}if !self.lookup.terminal_is_empty(){return self.lookup.next_close_depth_demand();}RetainedCloneBinding::depth_demand(if self.source.is_some(){&self.source}else{&self.mutation})}
    pub fn close_step(&mut self, grant:RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.closing { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "connect preparation closure was not started")); }
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

#[path = "🪢️edge/🦀️.rs"]
pub mod edge;

#[path = "📸️candidate/🦀️.rs"]
pub mod candidate;
