//! ♻️ Original board engine fields retain physical ownership until exact admitted closure.
use super::*;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,deferred,deferred_birth_bytes_for,sequence,sequence_birth_bytes};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use std::mem::ManuallyDrop;

#[derive(Clone,Copy,Debug)]
pub(super) struct RetainedPoint(pub Point);
impl std::ops::Deref for RetainedPoint{type Target=Point;fn deref(&self)->&Point{&self.0}}
semio_framework_value::artifact_retire_leaf!(Camera,InteractionMode,ProximityConnection,RetainedPoint);
semio_framework_value::artifact_retire_struct!(EngineSelectionOptions {method,mode,select_nodes,select_handles,select_edges});

struct PointBuffer(Vec<Point>);
struct PointRetirement{source:ManuallyDrop<Vec<Point>>,tail_bytes:usize}
impl RetireOwned for PointBuffer{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(PointRetirement{source:ManuallyDrop::new(self.0),tail_bytes:0})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<PointRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for PointRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
  if grant.maximum_items==0{return RetirementStep::BudgetExhausted}
  if !self.source.is_empty(){let remaining=std::mem::size_of::<Point>()-self.tail_bytes;let bytes=remaining.min(grant.maximum_copy_bytes);if bytes==0{return RetirementStep::BudgetExhausted}self.tail_bytes+=bytes;if self.tail_bytes==std::mem::size_of::<Point>(){self.source.pop();self.tail_bytes=0;}return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})}
  let bytes=self.source.capacity()*std::mem::size_of::<Point>();if bytes>grant.maximum_release_bytes{return RetirementStep::BudgetExhausted}if bytes>0{drop(std::mem::take(&mut *self.source));return RetirementStep::Bytes(bytes)}RetirementStep::Complete
 }
 fn terminal_is_empty(&self)->bool{self.source.is_empty()&&self.source.capacity()==0&&self.tail_bytes==0}
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(usize::from(!self.source.is_empty()))}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.source.is_empty(){self.source.capacity()*std::mem::size_of::<Point>()}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl Drop for PointRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original point backing abandoned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

impl RetireOwned for BoardEvent{
 fn retirement(self)->Box<dyn RetirementCursor>{match self{Self::HoverChanged{id}=>id.retirement(),Self::NodeMoved{id,x,y}=>(id,x,y).retirement(),Self::EdgeConnected{id,source,target}=>(id,source,target).retirement(),Self::EdgeRemoved{id}=>id.retirement(),Self::SelectionChanged{edge_ids,handle_ids,node_ids}=>(edge_ids,handle_ids,node_ids).retirement(),Self::PreselectChanged{edge_ids,handle_ids,node_ids,removed_edge_ids,removed_handle_ids,removed_node_ids}=>sequence(vec![deferred(edge_ids),deferred(handle_ids),deferred(node_ids),deferred(removed_edge_ids),deferred(removed_handle_ids),deferred(removed_node_ids)])}}
 fn retirement_birth_bytes(&self)->Option<usize>{match self{Self::HoverChanged{id}=>id.retirement_birth_bytes(),Self::NodeMoved{id,x,y}=>(*id,*x,*y).retirement_birth_bytes(),Self::EdgeConnected{id,source,target}=>(*id,*source,*target).retirement_birth_bytes(),Self::EdgeRemoved{id}=>id.retirement_birth_bytes(),Self::SelectionChanged{edge_ids,handle_ids,node_ids}=>sequence_birth_bytes(&[deferred_birth_bytes_for(edge_ids),deferred_birth_bytes_for(handle_ids),deferred_birth_bytes_for(node_ids)]),Self::PreselectChanged{edge_ids,handle_ids,node_ids,removed_edge_ids,removed_handle_ids,removed_node_ids}=>sequence_birth_bytes(&[deferred_birth_bytes_for(edge_ids),deferred_birth_bytes_for(handle_ids),deferred_birth_bytes_for(node_ids),deferred_birth_bytes_for(removed_edge_ids),deferred_birth_bytes_for(removed_handle_ids),deferred_birth_bytes_for(removed_node_ids)])}}
 fn controlled_retirement_supported()->bool{true}
}

impl<P:GraphPortModel+Send+'static,D:Directedness+Send+'static> RetireOwned for GraphEngine<P,D> where P::Endpoint:RetireOwned {
 fn retirement(self)->Box<dyn RetirementCursor>{let Self{camera,edges,edge_semantics,enforce_acyclic,events,handles,hover,interaction,nodes,selection,preselect,preselect_removed,selection_options,handle_pointer_picking,proximity_distance_world,proximity_distance_override,selection_preview_points,selection_preview_crossing,area_initial,area_points,area_screen_points,drag_start_positions,proximity_connection,next_edge_id,retirement_backing_credited_bytes,_directedness:_,_port:_}=self;sequence(vec![deferred(camera),deferred(edges),deferred(edge_semantics),deferred(enforce_acyclic),deferred(events),deferred(handles),deferred(hover),deferred(interaction),deferred(nodes),deferred(selection),deferred(preselect),deferred(preselect_removed),deferred(selection_options),deferred(handle_pointer_picking),deferred(proximity_distance_world),deferred(proximity_distance_override),deferred(PointBuffer(selection_preview_points)),deferred(selection_preview_crossing),deferred(area_initial),deferred(PointBuffer(area_points)),deferred(PointBuffer(area_screen_points)),deferred(drag_start_positions),deferred(proximity_connection),deferred(next_edge_id),deferred(retirement_backing_credited_bytes)])}
 fn retirement_birth_bytes(&self)->Option<usize>{sequence_birth_bytes(&[deferred_birth_bytes_for(&self.camera),deferred_birth_bytes_for(&self.edges),deferred_birth_bytes_for(&self.edge_semantics),deferred_birth_bytes_for(&self.enforce_acyclic),deferred_birth_bytes_for(&self.events),deferred_birth_bytes_for(&self.handles),deferred_birth_bytes_for(&self.hover),deferred_birth_bytes_for(&self.interaction),deferred_birth_bytes_for(&self.nodes),deferred_birth_bytes_for(&self.selection),deferred_birth_bytes_for(&self.preselect),deferred_birth_bytes_for(&self.preselect_removed),deferred_birth_bytes_for(&self.selection_options),deferred_birth_bytes_for(&self.handle_pointer_picking),deferred_birth_bytes_for(&self.proximity_distance_world),deferred_birth_bytes_for(&self.proximity_distance_override),semio_framework_value::retirement::deferred_birth_bytes::<PointBuffer>(),deferred_birth_bytes_for(&self.selection_preview_crossing),deferred_birth_bytes_for(&self.area_initial),semio_framework_value::retirement::deferred_birth_bytes::<PointBuffer>(),semio_framework_value::retirement::deferred_birth_bytes::<PointBuffer>(),deferred_birth_bytes_for(&self.drag_start_positions),deferred_birth_bytes_for(&self.proximity_connection),deferred_birth_bytes_for(&self.next_edge_id),deferred_birth_bytes_for(&self.retirement_backing_credited_bytes)])}
 fn controlled_retirement_supported()->bool{P::Endpoint::controlled_retirement_supported()}
}
