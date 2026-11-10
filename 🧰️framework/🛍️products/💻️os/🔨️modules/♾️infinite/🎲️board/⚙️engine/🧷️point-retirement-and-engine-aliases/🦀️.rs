//! 🧷️ Point retirement and engine aliases of the board engine: the retained point, the point buffer and its retirement cursor and the undirected and directed engine type aliases. See [`super`].

use super::*;

#[derive(Clone,Copy,Debug)]
#[doc(hidden)]
pub struct RetainedPoint(pub Point);

impl std::ops::Deref for RetainedPoint{type Target=Point;fn deref(&self)->&Point{&self.0}}

semio_framework_value::artifact_retire_leaf!(Camera,InteractionMode,ProximityConnection,RetainedPoint);

semio_framework_value::artifact_retire_struct!(EngineSelectionOptions {method,mode,select_nodes,select_handles,select_edges});

#[doc(hidden)]
pub struct PointBuffer(Vec<Point>);

#[doc(hidden)]
pub struct PointRetirement{pub source:ManuallyDrop<Vec<Point>>}

impl RetireOwned for PointBuffer{
 type RetirementCursor=PointRetirement;
 fn retirement_seed(self)->Self::RetirementCursor{PointRetirement{source:ManuallyDrop::new(self.0)}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PointRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

impl RetirementCursor for PointRetirement{
 semio_framework_value::native_retirement_reply!();
 fn next_computational_work(&self,_:usize)->Result<u64,semio_framework_value::ValueError>{Ok(u64::from(!self.terminal_is_empty()))}
 fn next_demand(&self,_:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand{copy_bytes:if self.source.is_empty(){0}else{size_of::<Point>()},release_bytes:if self.source.is_empty(){self.source.capacity()*size_of::<Point>()}else{0},depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
 fn terminal_is_empty(&self)->bool{self.source.is_empty()&&self.source.capacity()==0}
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,_:usize)->Option<usize>{Some(0)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(if self.source.is_empty(){self.source.capacity()*size_of::<Point>()}else{0})}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

impl PointRetirement{
 fn close_step(&mut self,body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete(0)}
  let demand=match self.next_demand(body){Ok(demand)=>demand,Err(error)=>return RetirementStep::Failure(error.into(),0)};
  if effect.maximum_work_units()==0||!semio_framework_value::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return RetirementStep::BudgetExhausted}
  let work=match semio_framework_value::retirement::record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return RetirementStep::Failure(error.into(),work)};
  if self.source.is_empty(){drop(std::mem::take(&mut *self.source))}else{self.source.pop();}
  RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()},work)
 }
}

impl Drop for PointRetirement{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original point backing abandoned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.source)}}}}

type EngineFields<E>=((Camera,HistoryFoldIndex<EdgeId,GraphEdge<E>>,HistoryFoldIndex<EdgeId,ElementSemantics>,bool,Vec<BoardEvent>,HistoryFoldIndex<HandleId,Handle>,Option<u64>,InteractionMode,HistoryFoldIndex<NodeId,Node>,Selection,Selection,Selection),(EngineSelectionOptions,bool,f64,Option<f64>,PointBuffer,bool,Selection,PointBuffer,PointBuffer,HistoryFoldIndex<NodeId,RetainedPoint>,Option<ProximityConnection>,u64),usize);

impl<P:GraphPortModel+Send+'static,D:Directedness+Send+'static> RetireOwned for GraphEngine<P,D> where P::Endpoint:RetireOwned {
 type RetirementCursor=<EngineFields<P::Endpoint> as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.camera.retirement_ready()&&self.edges.retirement_ready()&&self.edge_semantics.retirement_ready()&&self.events.retirement_ready()&&self.handles.retirement_ready()&&self.nodes.retirement_ready()&&self.selection.retirement_ready()&&self.preselect.retirement_ready()&&self.preselect_removed.retirement_ready()&&self.area_initial.retirement_ready()&&self.drag_start_positions.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{let Self{camera,edges,edge_semantics,enforce_acyclic,events,handles,hover,interaction,nodes,selection,preselect,preselect_removed,selection_options,handle_pointer_picking,proximity_distance_world,proximity_distance_override,selection_preview_points,selection_preview_crossing,area_initial,area_points,area_screen_points,drag_start_positions,proximity_connection,next_edge_id,retirement_backing_credited_bytes,_directedness:_,_port:_}=self;<EngineFields<P::Endpoint> as RetireOwned>::retirement_seed(((camera,edges,edge_semantics,enforce_acyclic,events,handles,hover,interaction,nodes,selection,preselect,preselect_removed),(selection_options,handle_pointer_picking,proximity_distance_world,proximity_distance_override,PointBuffer(selection_preview_points),selection_preview_crossing,area_initial,PointBuffer(area_points),PointBuffer(area_screen_points),drag_start_positions,proximity_connection,next_edge_id),retirement_backing_credited_bytes))}
 fn controlled_retirement_supported()->bool{semio_framework_value::retirement::support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&semio_framework_value::retirement::RetirementSupport<'_>)->bool{path.supports::<EngineFields<P::Endpoint>>()}
}

pub type UndirectedGraphEngine=GraphEngine<Normal,Undirected>;

/// ➡️ Node graph engine without ports; relationships are directed node pairs.
pub type DirectedGraphEngine = GraphEngine<Normal, Directed>;

/// ↔ Port graph engine; handle endpoints are unordered pairs.
pub type UndirectedPortGraphEngine = GraphEngine<Ported, Undirected>;

/// 🪢️ Port edge with handle endpoints (unordered).
pub type UndirectedEdge = GraphEdge<HandleId>;

semio_framework_value::artifact_retire_leaf!(CanvasPalette);
