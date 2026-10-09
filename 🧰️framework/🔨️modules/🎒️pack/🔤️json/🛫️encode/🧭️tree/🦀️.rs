//! 🧭️ Native ordinal canonical traversal retains real root-backed projections and paged frontier ownership.
use crate::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText, ArtifactCanonicalJsonScalarBytes as ScalarBytes, canonical_escaped_byte};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, list::PagedList, paged::{PagedMap, PagedUtf8}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedOwnedProjection}};
use std::mem::size_of;

fn refusal(reason: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, reason) }

/// 🌲️ Direct native child ordinals preserve original record fields and collection order.
pub trait ArtifactCanonicalJsonTree: Sync + 'static {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError>;
    fn canonical_tree_child(&self, _ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { Err(refusal("canonical tree scalar has no child")) }
    fn canonical_tree_key(&self, _ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { Err(refusal("canonical tree node has no key")) }
}

impl<const N: usize> ArtifactCanonicalJsonTree for PagedUtf8<N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Text(ArtifactCanonicalJsonText::Native(self))) }
}
impl<T: ArtifactCanonicalJsonTree, const N: usize> ArtifactCanonicalJsonTree for PagedList<T, N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Array(self.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.get(ordinal).map(|value| value as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| refusal("canonical native array ordinal is absent")) }
}
impl<T: ArtifactCanonicalJsonTree, const N: usize> ArtifactCanonicalJsonTree for PagedMap<T, N> {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Object(self.len())) }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn ArtifactCanonicalJsonTree, ValueError> { self.entry_at(ordinal).map(|(_, value)| value as &dyn ArtifactCanonicalJsonTree).ok_or_else(|| refusal("canonical native object ordinal is absent")) }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> { self.entry_at(ordinal).map(|(key, _)| ArtifactCanonicalJsonText::Native(key)).ok_or_else(|| refusal("canonical native object key is absent")) }
}
impl ArtifactCanonicalJsonTree for bool { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::Bool(*self)) } }
impl ArtifactCanonicalJsonTree for u64 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::U64(*self)) } }
impl ArtifactCanonicalJsonTree for i64 { fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> { Ok(Node::I64(*self)) } }

impl ArtifactCanonicalJsonTree for String { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }
impl ArtifactCanonicalJsonTree for semio_framework_value::SharedUtf8 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Text(ArtifactCanonicalJsonText::Native(self)))}}
impl ArtifactCanonicalJsonTree for &'static str { fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(self))} }
impl<T:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Vec<T> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Array(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.get(ordinal).map(|value|value as &dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical original vector ordinal is absent"))}
}
impl<T:ArtifactCanonicalJsonTree,const N:usize> ArtifactCanonicalJsonTree for [T;N] {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Array(N))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.get(ordinal).map(|value|value as &dyn ArtifactCanonicalJsonTree).ok_or_else(||refusal("canonical original fixed array ordinal is absent"))}
}
impl<T:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Option<T> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{self.as_ref().map_or(Ok(Node::Null),|value|value.canonical_tree_node())}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.as_ref().ok_or_else(||refusal("canonical absent optional field has no child"))?.canonical_tree_child(ordinal)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{self.as_ref().ok_or_else(||refusal("canonical absent optional field has no key"))?.canonical_tree_key(ordinal)}
}
impl<T:ArtifactCanonicalJsonTree> ArtifactCanonicalJsonTree for Box<T> {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{self.as_ref().canonical_tree_node()}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{self.as_ref().canonical_tree_child(ordinal)}
 fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{self.as_ref().canonical_tree_key(ordinal)}
}
impl ArtifactCanonicalJsonTree for f64 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::F64(*self))}}
impl ArtifactCanonicalJsonTree for i32 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::I64(i64::from(*self)))}}
impl ArtifactCanonicalJsonTree for usize {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U64(u64::try_from(*self).map_err(|_|refusal("canonical native ordinal exceeds unsigned64"))?))}}
impl ArtifactCanonicalJsonTree for u8 {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::U64(u64::from(*self)))}}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Inspect, Scalar, Text, ArrayStart, ArrayNext, ArrayChild, ObjectStart, ObjectNext, ObjectKey, ObjectColon, ObjectChild, Complete }

#[derive(Default)]
struct TextState { chunk: usize, offset: usize, phase: u8, original: u8, escaped: usize }
impl TextState {
    fn step(&mut self, text: ArtifactCanonicalJsonText<'_>) -> Result<Option<u8>, ValueError> {
        match self.phase {
            0 => { self.phase = 1; Ok(Some(b'"')) }
            1 => match text.next_byte(&mut self.chunk, &mut self.offset).map_err(refusal)? {
                Some(byte) => { self.original = byte; self.escaped = 0; self.phase = 2; Ok(None) }
                None => { self.phase = 3; Ok(Some(b'"')) }
            },
            2 => { let byte = canonical_escaped_byte(self.original, self.escaped).ok_or_else(|| refusal("canonical native escape offset is absent"))?; self.escaped += 1; if canonical_escaped_byte(self.original, self.escaped).is_none() { self.phase = 1; } Ok(Some(byte)) }
            _ => Err(refusal("canonical native text advanced after terminal quote")),
        }
    }
}

struct NativeFrame {
    owner: RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>,
    phase: Phase,
    ordinal: usize,
    length: usize,
    scalar: ScalarBytes,
    offset: usize,
    text: TextState,
}
impl NativeFrame {
    fn new(owner: RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>) -> Self { Self { owner, phase: Phase::Inspect, ordinal: 0, length: 0, scalar: ScalarBytes { bytes: [0; 64], length: 0 }, offset: 0, text: TextState::default() } }
    fn closure_demand(&self) -> Result<RetirementDemand, ValueError> {
        let copy_bytes = self.owner.next_close_copy_byte_demand()?;
        Ok(RetirementDemand { copy_bytes, capacity_bytes: self.owner.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: self.owner.next_close_release_byte_demand()?, depth: self.owner.next_close_depth_demand()? })
    }
}

/// 📊️ One admitted native action reports only its initialized byte prefix and common ownership receipt.
pub struct ArtifactCanonicalJsonTreeStep { pub ownership: RetainedCloneStep, pub written_bytes: usize }

/// 🧭️ Owns actual immutable-root aliases in a genuinely paged traversal frontier without path rescans.
pub struct ArtifactCanonicalJsonTreeCursor {
    frames: PagedList<NativeFrame, {usize::MAX}>,
    pending: Option<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>,
    closing: bool,
}

impl ArtifactCanonicalJsonTreeCursor {
    pub fn constructor_demand() -> RetirementDemand { RetirementDemand { copy_bytes: size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(), depth: 1, ..Default::default() } }
    pub fn admit(owner: RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>, grant: RetainedCloneGrant) -> Result<(Self, RetainedCloneProgress), (ValueError, RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>)> {
        let demand = Self::constructor_demand();
        if !Self::admitted(demand, grant) { return Err((ValueError::literal(ValueRefusalKind::WorkLimit, "canonical tree constructor requires original alias transfer admission"), owner)); }
        Ok((Self { frames: PagedList::empty(), pending: Some(owner), closing: false }, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }))
    }

    fn admitted(demand: RetirementDemand, grant: RetainedCloneGrant) -> bool { grant.maximum_items != 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth }
    fn pending_closure_demand(&self) -> Result<RetirementDemand, ValueError> {
        let owner = self.pending.as_ref().ok_or_else(|| refusal("canonical pending alias is absent"))?;
        if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(), depth: 1, ..Default::default() }); }
        let copy_bytes = owner.next_close_copy_byte_demand()?;
        Ok(RetirementDemand { copy_bytes, capacity_bytes: owner.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: owner.next_close_release_byte_demand()?, depth: owner.next_close_depth_demand()? })
    }

    pub fn next_demand(&self) -> Result<RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.pending.is_some() {
            if self.closing { return self.pending_closure_demand(); }
            if !self.frames.has_reserved_slot() { return Ok(RetirementDemand { capacity_bytes: self.frames.next_allocation_bytes().map_err(ValueError::from)?, depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: size_of::<NativeFrame>(), depth: 1, ..Default::default() });
        }
        if let Some(frame) = self.frames.last() {
            if self.closing || frame.phase == Phase::Complete {
                if frame.owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<NativeFrame>(), depth: 1, ..Default::default() }); }
                return frame.closure_demand();
            }
            let copy_bytes = match frame.phase {
                Phase::Inspect => size_of::<ScalarBytes>(),
                Phase::ArrayChild | Phase::ObjectChild => size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(),
                Phase::ArrayNext if frame.ordinal < frame.length && frame.ordinal == 0 => size_of::<RetainedOwnedProjection<dyn ArtifactCanonicalJsonTree>>(),
                _ => 1,
            };
            return Ok(RetirementDemand { copy_bytes, depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { release_bytes: self.frames.next_release_allocation_bytes().map_err(ValueError::from)?, depth: 1, ..Default::default() })
    }

    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.frames.is_empty() && self.frames.allocated_bytes() == 0 }
    pub fn is_complete(&self) -> bool { !self.closing && self.terminal_is_empty() }

    pub fn advance(&mut self, output: &mut [u8], grant: RetainedCloneGrant) -> Result<ArtifactCanonicalJsonTreeStep, ValueError> {
        if self.terminal_is_empty() { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Complete(Default::default()), written_bytes: 0 }); }
        if grant.maximum_items == 0 { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
        let demand = self.next_demand()?;
        if !Self::admitted(demand, grant) { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
        let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        let mut written_bytes = 0;
        if self.pending.is_some() {
            if self.closing {
                if self.pending.as_ref().unwrap().terminal_is_empty() { self.pending = None; progress.copied_bytes = demand.copy_bytes; }
                else { progress = self.pending.as_mut().unwrap().close_step(grant)?.progress(); }
            } else if !self.frames.has_reserved_slot() {
                let step = self.frames.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
                progress.retained_capacity_bytes = step.allocated_bytes;
            } else {
                let frame = NativeFrame::new(self.pending.take().unwrap());
                self.frames.push_reserved(frame).unwrap_or_else(|_| unreachable!("canonical frontier reserved its exact native slot"));
                progress.copied_bytes = demand.copy_bytes;
            }
        } else if let Some(frame) = self.frames.last_mut() {
            if self.closing || frame.phase == Phase::Complete {
                if frame.owner.terminal_is_empty() { drop(self.frames.pop().unwrap()); progress.copied_bytes = demand.copy_bytes; }
                else { progress = frame.owner.close_step(grant)?.progress(); }
            } else {
                let byte = match frame.phase {
                    Phase::Inspect => {
                        let reference = frame.owner.borrow()?;
                        let node = reference.get().canonical_tree_node()?;
                        match node {
                            Node::Array(length) => { frame.length = length; frame.phase = Phase::ArrayStart; }
                            Node::Object(length) => { frame.length = length; frame.phase = Phase::ObjectStart; }
                            Node::String(_) | Node::Text(_) => frame.phase = Phase::Text,
                            scalar => { frame.scalar = ScalarBytes::from_node(scalar).map_err(|_| refusal("canonical native scalar could not encode"))?; frame.phase = Phase::Scalar; }
                        }
                        progress.copied_bytes = demand.copy_bytes;
                        None
                    }
                    Phase::ArrayChild | Phase::ObjectChild | Phase::ArrayNext if frame.phase != Phase::ArrayNext || frame.ordinal == 0 && frame.ordinal < frame.length => {
                        let ordinal = frame.ordinal;
                        frame.owner.borrow()?.get().canonical_tree_child(ordinal)?;
                        self.pending = Some(frame.owner.project(ordinal, |node| node.canonical_tree_child(ordinal).expect("checked immutable native canonical child"))?);
                        frame.ordinal += 1;
                        frame.phase = if frame.phase == Phase::ObjectChild { Phase::ObjectNext } else { Phase::ArrayNext };
                        progress.copied_bytes = demand.copy_bytes;
                        None
                    }
                    _ if output.is_empty() => { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
                    Phase::Scalar => {
                        let byte = frame.scalar.bytes[frame.offset]; frame.offset += 1;
                        if frame.offset == frame.scalar.length { frame.phase = Phase::Complete; }
                        Some(byte)
                    }
                    Phase::Text => {
                        let reference = frame.owner.borrow()?;
                        let text = match reference.get().canonical_tree_node()? { Node::String(text) => text.into(), Node::Text(text) => text, _ => return Err(refusal("canonical native text role changed")) };
                        let byte = frame.text.step(text)?;
                        if frame.text.phase == 3 { frame.phase = Phase::Complete; }
                        byte
                    }
                    Phase::ArrayStart => { frame.phase = Phase::ArrayNext; Some(b'[') }
                    Phase::ArrayNext => { if frame.ordinal == frame.length { frame.phase = Phase::Complete; Some(b']') } else { frame.phase = Phase::ArrayChild; Some(b',') } }
                    Phase::ObjectStart => { frame.phase = Phase::ObjectNext; Some(b'{') }
                    Phase::ObjectNext => {
                        if frame.ordinal == frame.length { frame.phase = Phase::Complete; Some(b'}') }
                        else { frame.phase = Phase::ObjectKey; frame.text = TextState::default(); if frame.ordinal != 0 { Some(b',') } else { None } }
                    }
                    Phase::ObjectKey => {
                        let reference = frame.owner.borrow()?;
                        let text = reference.get().canonical_tree_key(frame.ordinal)?;
                        let byte = frame.text.step(text)?;
                        if frame.text.phase == 3 { frame.phase = Phase::ObjectColon; }
                        byte
                    }
                    Phase::ObjectColon => { frame.phase = Phase::ObjectChild; Some(b':') }
                    _ => return Err(refusal("canonical native traversal phase is invalid")),
                };
                if progress.copied_bytes == 0 { progress.copied_bytes = 1; }
                if let Some(byte) = byte { output[0] = byte; written_bytes = 1; }
            }
        } else {
            let step = self.frames.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            progress.released_bytes = step.released_allocation_bytes;
        }
        Ok(ArtifactCanonicalJsonTreeStep { ownership: if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }, written_bytes })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.begin_close(); Ok(self.advance(&mut [], grant)?.ownership) }
}
impl Drop for ArtifactCanonicalJsonTreeCursor { fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "canonical native traversal requires exact frontier closure"); } }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
