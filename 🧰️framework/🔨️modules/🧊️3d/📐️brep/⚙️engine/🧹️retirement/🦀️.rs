//! 🧹️ Typed native ownership frontiers with explicit item and payload-byte grants.
use super::{Brep, Entity, MeshTransfer};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneStep};
use crate::brep::representation::{arena::{ArenaId, Store}, curve::{Curve2, Curve3}, surface::Surface, topology::Body};
use std::{collections::LinkedList, mem::{size_of, ManuallyDrop}};

/// 🎟️ Work charged by one native resource retirement turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeRetirementStep { Blocked, Pending { released_items: usize, released_bytes: usize }, Complete }
/// 🧵️ One typed structural frontier, transferring at most one collection entry per turn.
pub trait RetirementFrontier: Send {
    fn advance(&mut self,payloads:&mut PayloadRetirement,grant:RetainedCloneGrant)->bool;
    fn next_close_byte_demand(&self)->usize {0}
}
trait Allocation: Send {}
impl<T: Send> Allocation for Vec<T> {}
enum Owner { Allocation { values: Box<dyn Allocation>, remaining: usize }, Frontier(Box<dyn RetirementFrontier>), Owned(Box<dyn semio_framework_value::ErasedSnapshotRetirement>) }
/// 🧹️ Owns native payloads until explicit terminal-empty retirement.
#[must_use = "native resources require explicit terminal-empty retirement"]
pub struct PayloadRetirement { owners: ManuallyDrop<LinkedList<Owner>> }
impl Default for PayloadRetirement { fn default() -> Self { Self { owners: ManuallyDrop::new(LinkedList::new()) } } }
impl PayloadRetirement {
    pub fn terminal_is_empty(&self) -> bool { self.owners.is_empty() }
    pub fn frontier(&mut self, frontier: impl RetirementFrontier + 'static) { self.owners.push_back(Owner::Frontier(Box::new(frontier))); }
    pub fn owned<T:semio_framework_value::retirement::RetireOwned>(&mut self,value:T,grant:RetainedCloneGrant)->Result<semio_framework_value::retained_clone::RetainedCloneProgress,(semio_framework_value::ValueError,T)> {
        let (owner,progress)=semio_framework_value::retirement::admit_owned_retirement(value,grant)?;
        self.owners.push_back(Owner::Owned(owner));Ok(progress)
    }
    pub fn next_close_byte_demand(&self)->usize {
        match self.owners.front() {
            Some(Owner::Allocation {remaining,..})=>*remaining,
            Some(Owner::Frontier(frontier))=>frontier.next_close_byte_demand(),
            Some(Owner::Owned(owned))=>{let copy=owned.next_copy_byte_demand().expect("native copy demand");let release=owned.next_release_byte_demand().expect("native release demand");owned.next_capacity_byte_demand(copy.max(release)).expect("native capacity demand").max(release).max(copy)},
            None=>0,
        }
    }
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
        let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:maximum_bytes,maximum_release_bytes:maximum_bytes,maximum_depth:256};
        let mut released_bytes = 0;
        match self.owners.pop_front().expect("native retirement owner") {
            Owner::Allocation { values, remaining } => {
                if remaining>maximum_bytes {self.owners.push_front(Owner::Allocation {values,remaining});return NativeRetirementStep::Blocked;}
                released_bytes=remaining;
            }
            Owner::Frontier(mut frontier) => if !frontier.advance(self,grant) { self.owners.push_front(Owner::Frontier(frontier)); },
            Owner::Owned(mut owned) => {
                let copy=owned.next_copy_byte_demand();let release=owned.next_release_byte_demand();let depth=owned.next_depth_demand();
                let admissible=copy.and_then(|copy|release.and_then(|release|owned.next_capacity_byte_demand(copy.max(release)).map(|capacity|copy<=maximum_bytes&&release<=maximum_bytes&&capacity<=maximum_bytes))).and_then(|bytes|depth.map(|depth|bytes&&depth<=grant.maximum_depth));
                if !matches!(admissible,Ok(true)) {self.owners.push_front(Owner::Owned(owned));return NativeRetirementStep::Blocked;}
                let step=match owned.close_step(grant) {Ok(step)=>step,Err(_)=>{self.owners.push_front(Owner::Owned(owned));return NativeRetirementStep::Blocked;}};
                match step {
                    RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>{
                        assert!(progress.copied_items<=1 && progress.released_bytes<=maximum_bytes,"native owned retirement exceeded its grant");
                        released_bytes=progress.released_bytes;
                        if !owned.terminal_is_empty() {self.owners.push_front(Owner::Owned(owned));}
                    }
                }
            }
        }
        NativeRetirementStep::Pending { released_items: 1, released_bytes }
    }
}
impl Drop for PayloadRetirement {
    fn drop(&mut self) { if !std::thread::panicking() { assert!(self.terminal_is_empty(), "native resources require terminal-empty retirement"); } }
}
struct Rows<T>(Vec<Vec<T>>);
impl<T: Copy + Send + 'static> RetirementFrontier for Rows<T> {
    fn advance(&mut self, payloads: &mut PayloadRetirement,_grant:RetainedCloneGrant) -> bool {
        if let Some(row) = self.0.pop() { payloads.pod(row); false } else { payloads.empty_allocation(std::mem::take(&mut self.0)); true }
    }
}
struct Arena<T, Id> { values: Store<T, Id>, transfer: fn(T, &mut PayloadRetirement) }
impl<T: Send + 'static, Id: ArenaId + 'static> RetirementFrontier for Arena<T, Id> {
    fn advance(&mut self, payloads: &mut PayloadRetirement,_grant:RetainedCloneGrant) -> bool {
        if let Some(slot) = self.values.retirement_pop() { if let Some(value) = slot { (self.transfer)(value,payloads); } false }
        else { self.values.retirement_backing(payloads); true }
    }
}
fn arena<T: Send + 'static, Id: ArenaId + 'static>(values: Store<T,Id>, transfer: fn(T,&mut PayloadRetirement), payloads: &mut PayloadRetirement) { payloads.frontier(Arena { values, transfer }); }
fn curve3(curve: Curve3, payloads: &mut PayloadRetirement) { if let Curve3::Nurbs { knots, controls, weights } = curve { payloads.pod(knots.knots); payloads.pod(controls); payloads.pod(weights); } }
fn curve2(curve: Curve2, payloads: &mut PayloadRetirement) { if let Curve2::Nurbs { knots, controls, weights } = curve { payloads.pod(knots.knots); payloads.pod(controls); payloads.pod(weights); } }
fn surface(surface: Surface, payloads: &mut PayloadRetirement) { if let Surface::Nurbs { u_knots, v_knots, controls, weights } = surface { payloads.pod(u_knots.knots); payloads.pod(v_knots.knots); payloads.rows(controls); payloads.rows(weights); } }
struct Live(semio_framework_mesh_engine::HistoryFoldIndex<String,Entity>);
impl RetirementFrontier for Live {
    fn advance(&mut self, payloads: &mut PayloadRetirement,_grant:RetainedCloneGrant) -> bool {
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
    fn advance(&mut self, payloads: &mut PayloadRetirement,_grant:RetainedCloneGrant) -> bool {
        if let Some(value) = self.values.pop() { payloads.text((self.text)(value)); false }
        else { payloads.empty_allocation(std::mem::take(&mut self.values)); true }
    }
}
impl PayloadRetirement {
    pub fn mesh_transfer(&mut self, mesh: MeshTransfer) {
        self.pod(mesh.position); self.pod(mesh.normal); self.pod(mesh.index); self.pod(mesh.edges); self.pod(mesh.points);
        self.frontier(Metadata { values:mesh.vertex_groups, text:|v| v.entity_id });
        self.frontier(Metadata { values:mesh.face_groups, text:|v| v.entity_id }); self.frontier(Metadata { values:mesh.edge_groups, text:|v| v.entity_id });
        self.frontier(Metadata { values:mesh.face_infos, text:|v| v.entity_id }); self.frontier(Metadata { values:mesh.edge_infos, text:|v| v.entity_id });
    }
}
