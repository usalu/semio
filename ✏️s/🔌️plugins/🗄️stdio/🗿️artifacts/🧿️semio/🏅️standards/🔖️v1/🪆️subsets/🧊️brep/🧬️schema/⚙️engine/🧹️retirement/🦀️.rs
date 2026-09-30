//! 🧹️ Typed native ownership frontiers with explicit item and payload-byte grants.
use super::{Brep, Entity, MeshTransfer};
use crate::standards::v1::subsets::brep::schema::snapshot::{arena::{ArenaId, Store}, curve::{Curve2, Curve3}, surface::Surface, topology::Body};
use std::{collections::{BTreeMap, LinkedList}, mem::{size_of, ManuallyDrop}};

/// 🎟️ Work charged by one native resource retirement turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeRetirementStep { Blocked, Pending { released_items: usize, released_bytes: usize }, Complete }
/// 🧵️ One typed structural frontier, transferring at most one collection entry per turn.
pub trait RetirementFrontier: Send { fn advance(&mut self, payloads: &mut PayloadRetirement) -> bool; }
trait Allocation: Send {}
impl<T: Send> Allocation for Vec<T> {}
enum Owner { Allocation { values: Box<dyn Allocation>, remaining: usize }, Frontier(Box<dyn RetirementFrontier>) }
/// 🧹️ Owns native payloads until explicit terminal-empty retirement.
#[must_use = "native resources require explicit terminal-empty retirement"]
pub struct PayloadRetirement { owners: ManuallyDrop<LinkedList<Owner>> }
impl Default for PayloadRetirement { fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } } }
impl PayloadRetirement {
    pub fn terminal_is_empty(&self) -> bool { self.owners.is_empty() }
    pub fn frontier(&mut self, frontier: impl RetirementFrontier + 'static) { self.owners.push_back(Owner::Frontier(Box::new(frontier))); }
    pub fn pod<T: Copy + Send + 'static>(&mut self, values: Vec<T>) { self.allocation(values); }
    pub fn empty_allocation<T: Send + 'static>(&mut self, values: Vec<T>) { assert!(values.is_empty()); self.allocation(values); }
    fn allocation<T: Send + 'static>(&mut self, values: Vec<T>) {
        let remaining = values.capacity().saturating_mul(size_of::<T>());
        if remaining != 0 { self.owners.push_back(Owner::Allocation { values: Box::new(values), remaining }); }
    }
    pub fn text(&mut self, text: String) { self.pod(text.into_bytes()); }
    pub fn rows<T: Copy + Send + 'static>(&mut self, rows: Vec<Vec<T>>) { self.frontier(Rows(rows)); }
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> NativeRetirementStep {
        if self.owners.is_empty() { return NativeRetirementStep::Complete; }
        if maximum_items == 0 || maximum_bytes == 0 { return NativeRetirementStep::Blocked; }
        let mut released_bytes = 0;
        match self.owners.pop_front().expect("native retirement owner") {
            Owner::Allocation { values, remaining } => {
                released_bytes = maximum_bytes.min(remaining);
                let remaining = remaining - released_bytes;
                if remaining != 0 { self.owners.push_front(Owner::Allocation { values, remaining }); }
            }
            Owner::Frontier(mut frontier) => if !frontier.advance(self) { self.owners.push_front(Owner::Frontier(frontier)); },
        }
        NativeRetirementStep::Pending { released_items: 1, released_bytes }
    }
}
impl Drop for PayloadRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "native resources require terminal-empty retirement"); } }
}
struct Rows<T>(Vec<Vec<T>>);
impl<T: Copy + Send + 'static> RetirementFrontier for Rows<T> {
    fn advance(&mut self, payloads: &mut PayloadRetirement) -> bool {
        if let Some(row) = self.0.pop() { payloads.pod(row); false } else { payloads.empty_allocation(std::mem::take(&mut self.0)); true }
    }
}
struct Arena<T, Id> { values: Store<T, Id>, transfer: fn(T, &mut PayloadRetirement) }
impl<T: Send + 'static, Id: ArenaId + 'static> RetirementFrontier for Arena<T, Id> {
    fn advance(&mut self, payloads: &mut PayloadRetirement) -> bool {
        if let Some(slot) = self.values.retirement_pop() { if let Some(value) = slot { (self.transfer)(value,payloads); } false }
        else { self.values.retirement_backing(payloads); true }
    }
}
fn arena<T: Send + 'static, Id: ArenaId + 'static>(values: Store<T,Id>, transfer: fn(T,&mut PayloadRetirement), payloads: &mut PayloadRetirement) { payloads.frontier(Arena { values, transfer }); }
fn curve3(curve: Curve3, payloads: &mut PayloadRetirement) { if let Curve3::Nurbs { knots, controls, weights } = curve { payloads.pod(knots.knots); payloads.pod(controls); payloads.pod(weights); } }
fn curve2(curve: Curve2, payloads: &mut PayloadRetirement) { if let Curve2::Nurbs { knots, controls, weights } = curve { payloads.pod(knots.knots); payloads.pod(controls); payloads.pod(weights); } }
fn surface(surface: Surface, payloads: &mut PayloadRetirement) { if let Surface::Nurbs { u_knots, v_knots, controls, weights } = surface { payloads.pod(u_knots.knots); payloads.pod(v_knots.knots); payloads.rows(controls); payloads.rows(weights); } }
struct Live(BTreeMap<String,Entity>);
impl RetirementFrontier for Live {
    fn advance(&mut self, payloads: &mut PayloadRetirement) -> bool {
        let Some((handle,entity)) = self.0.pop_first() else { return true };
        payloads.text(handle);
        match entity {
            Entity::Wire(wire,_) => payloads.pod(wire.members), Entity::Compound(solids,_) => payloads.pod(solids),
            Entity::Curve(curve,_) => curve3(curve,payloads), Entity::Surface(value,_) => surface(value,payloads), _ => {}
        }
        self.0.is_empty()
    }
}
impl Brep {
    /// 🚪️ Detaches the terminal native family without reachability scans or arena compaction.
    pub fn detach_retirement(&mut self, payloads: &mut PayloadRetirement) {
        let Body { vertices, edges, coedges, loops, faces, shells, solids, curves3, curves2, surfaces, labels: _ } = std::mem::take(&mut self.body);
        payloads.frontier(Live(std::mem::take(&mut self.live)));
        arena(vertices, |_,_| {},payloads); arena(edges, |_,_| {},payloads); arena(coedges, |_,_| {},payloads); arena(loops, |_,_| {},payloads);
        arena(faces, |v,p| p.pod(v.inners),payloads); arena(shells, |v,p| p.pod(v.faces),payloads); arena(solids, |v,p| p.pod(v.inners),payloads);
        arena(curves3,curve3,payloads); arena(curves2,curve2,payloads); arena(surfaces,surface,payloads);
    }
}
struct Metadata<T> { values: Vec<T>, text: fn(T) -> String }
impl<T: Send + 'static> RetirementFrontier for Metadata<T> {
    fn advance(&mut self, payloads: &mut PayloadRetirement) -> bool {
        if let Some(value) = self.values.pop() { payloads.text((self.text)(value)); false }
        else { payloads.empty_allocation(std::mem::take(&mut self.values)); true }
    }
}
impl PayloadRetirement {
    pub fn mesh_transfer(&mut self, mesh: MeshTransfer) {
        self.pod(mesh.position); self.pod(mesh.normal); self.pod(mesh.index); self.pod(mesh.edges); self.pod(mesh.points);
        self.frontier(Metadata { values:mesh.face_groups, text:|v| v.entity_id }); self.frontier(Metadata { values:mesh.edge_groups, text:|v| v.entity_id });
        self.frontier(Metadata { values:mesh.face_infos, text:|v| v.entity_id }); self.frontier(Metadata { values:mesh.edge_infos, text:|v| v.entity_id });
    }
}
