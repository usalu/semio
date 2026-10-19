//! 🪟️ WASM session and input application of the DAG host: the wasm-only DAG session and its methods and the input application with its retirement. See [`super`].

use super::*;

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
struct DagSessionInner {
    pub host: DagHost,
    gpu: canvas::gpu_session::CanvasGpuSession,
    pub width: u32,
    pub height: u32,
    pub dpr: f64,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
pub struct DagSession {
    pub state: Rc<RefCell<DagSessionInner>>,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[wasm_bindgen]
impl DagSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(DagSessionInner { host: DagHost::default_demo(), gpu: canvas::gpu_session::CanvasGpuSession::default(), width: 1, height: 1, dpr: 1.0 })) }
    }

    #[wasm_bindgen(js_name = loadHostSnapshotJson)]
    pub fn load_host_snapshot_json(&self, json: &str) -> Result<(), JsValue> {
        let host = DagHost::load_host_snapshot_json(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().host = host;
        Ok(())
    }

    #[wasm_bindgen(js_name = hostSnapshotJson)]
    pub fn host_snapshot_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.graph.host_snapshot_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = nodeOverlaysJson)]
    pub fn node_overlays_json(&self) -> Result<String, JsValue> {
        match &self.state.borrow().host { __host => dag_node_overlays_json(&__host.viewport, &__host.graph) }.map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = labelOverlayPaintStateJson)]
    pub fn label_overlay_paint_state_json(&self) -> Result<String, JsValue> {
        self.state.borrow().host.label_overlay_paint_state_json().map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = attachCanvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> js_sys::Promise {
        let inner = self.state.clone();
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        future_to_promise(async move {
            let admission = canvas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph).await.map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            g.width = lw;
            g.height = lh;
            g.dpr = dpr;
            g.host.viewport.set_viewport(lw, lh, dpr);
            g.gpu.finish_attach(admission);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let mut inner = self.state.borrow_mut();
        inner.width = width.max(1);
        inner.height = height.max(1);
        inner.dpr = dpr.max(1.0);
        let (w, h, d) = (inner.width, inner.height, inner.dpr);
        inner.host.viewport.set_viewport(w, h, d);
        let pw = ((w as f64 * d).round() as u32).max(1);
        let ph = ((h as f64 * d).round() as u32).max(1);
        inner.gpu.resize_surface(pw, ph);
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera(&self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.graph.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active(&self, active: bool) {
        match &mut self.state.borrow_mut().host { __host => dag_set_wheel_zoom_active(&mut __host.viewport, &mut __host.graph, active) };
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json(&self) -> String {
        dag_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod(&self, enabled: bool) {
        self.state.borrow_mut().host.viewport.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label(&self, label: &str) {
        self.state.borrow_mut().host.viewport.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = drawLodLabel)]
    pub fn draw_lod_label(&self) -> String {
        match &self.state.borrow().host { __host => dag_draw_lod_label(&__host.viewport, &__host.graph) }.to_string()
    }

    #[wasm_bindgen(js_name = pointerDown)]
    pub fn pointer_down(&self, x: f64, y: f64, extend: bool) {
        self.state.borrow_mut().host.pointer_down(x, y, extend);
    }

    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.pointer_move(x, y);
    }

    #[wasm_bindgen(js_name = pointerUp)]
    pub fn pointer_up(&self, x: f64, y: f64) {
        self.state.borrow_mut().host.pointer_up(x, y);
    }

    #[wasm_bindgen(js_name = takePendingOpenInstanceId)]
    pub fn take_pending_open_instance_id(&self) -> Option<String> {
        dag_take_pending_open_instance_id(&mut self.state.borrow_mut().host.pointer)
    }

    #[wasm_bindgen(js_name = screenToWorld)]
    pub fn screen_to_world(&self, x: f64, y: f64) -> js_sys::Array {
        let (wx, wy) = dag_screen_to_world(&self.state.borrow().host.viewport, &self.state.borrow().host.graph, x, y);
        let out = js_sys::Array::new();
        out.push(&JsValue::from_f64(wx));
        out.push(&JsValue::from_f64(wy));
        out
    }

    #[wasm_bindgen(js_name = reorganize)]
    pub fn reorganize(&self,options_json:&str,maximum_bytes:u32,maximum_work:u32,progress:js_sys::Function)->Result<(),JsValue>{
        let report=|phase:&str,completed:u64|progress.call2(&JsValue::NULL,&JsValue::from_str(phase),&JsValue::from_f64(completed as f64)).map(|v|v.as_bool().unwrap_or(false)).unwrap_or(false);
        let mut decoding=|p:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|report("decode",p.completed as u64);let mut decode=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(maximum_bytes as usize,&mut decoding);
        let opts=decode_dag_options_json(options_json,&mut decode).map_err(|e|JsValue::from_str(&e.to_string()))?;
        let mut working=|p:LayoutProgress|report("layout",p.completed);let mut work=LayoutControl::new(maximum_work as u64,&mut working);
        self.state.borrow_mut().host.graph.reorganize(&opts,&mut work).map_err(|e|JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = setCanvasThemeJson)]
    pub fn set_canvas_theme_json(&mut self, json: &str) {
        let mut accepted=|_|true;
        let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(64*1024,&mut accepted);
        if let Ok(overlay)=decode_board_palette_overlay_json(json,&mut control){match &mut self.state.borrow_mut().host { __host => dag_set_canvas_palette(&mut __host.viewport, &mut __host.paint, &overlay) };}
    }

    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&self) -> Result<(), JsValue> {
        let mut inner = self.state.borrow_mut();
        let mut scene = canvas::Scene::new();
        let clear = inner.host.viewport.canvas_theme.raster_clear;
        inner.host.paint_scene(&mut scene, inner.width, inner.height, inner.dpr);
        let scene = canvas::render::scale_scene_for_device_pixel_ratio(scene, inner.dpr);
        inner.gpu.render_frame(&scene, clear)
    }
}

// #endregion 🔖️WasmSession

pub struct DagInputApplication{pub facts:Option<DagInputFacts>,pub selection:Selection,pub active:Option<NodeId>,pub stale:NumericSet<NodeId>,pub(super) statuses:NumericIndex<NodeId,DagNodeEvalStatusKind>,pub ports:NumericSet<(NodeId,usize)>,pub field:usize,pub row:usize,pub port:usize,pub identity:IdentityCursor,pub node:Option<NodeId>,pub phase:u8,pub channels:bool,id_insert:Option<NumericInsertCursor<u64,()>>,status_insert:Option<NumericInsertCursor<u64,DagNodeEvalStatusKind>>,pub port_insert:Option<NumericInsertCursor<(u64,usize),()>>,pub complete:bool,pub progress:semio_framework_value::retained_clone::RetainedCloneProgress}

impl DagInputApplication{
 pub fn new(facts:DagInputFacts)->Self{Self{facts:Some(facts),selection:Selection::default(),active:None,stale:NumericSet::new(),statuses:NumericIndex::new(),ports:NumericSet::new(),field:0,row:0,port:0,identity:IdentityCursor::default(),node:None,phase:0,channels:false,id_insert:None,status_insert:None,port_insert:None,complete:false,progress:Default::default()}}
 pub fn normal_step_progress(&self)->semio_framework_value::retained_clone::RetainedCloneProgress{self.progress}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{match self.phase{
  3=>{let output=match self.facts.as_ref().unwrap(){DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&self.selection.node_ids,1=>&self.selection.edge_ids,_=>&self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&self.selection.handle_ids}else{&self.selection.node_ids}};output.insert_capacity_byte_demand(self.id_insert.as_ref().unwrap())},
  4=>self.status_insert.as_ref().unwrap().next_capacity_byte_demand(&self.statuses),6=>self.ports.insert_capacity_byte_demand(self.port_insert.as_ref().unwrap()),_=>Ok(0)}}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{match self.phase{
  3=>{let output=match self.facts.as_ref().unwrap(){DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&self.selection.node_ids,1=>&self.selection.edge_ids,_=>&self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&self.selection.handle_ids}else{&self.selection.node_ids}};output.insert_depth_demand(self.id_insert.as_ref().unwrap())},
  4=>self.status_insert.as_ref().unwrap().next_depth_demand(&self.statuses),6=>self.ports.insert_depth_demand(self.port_insert.as_ref().unwrap()),_=>Ok(1)}}
 pub fn step(&mut self,viewport: &DagViewportState, graph: &DagGraphState,units:usize,control:&mut NativeDecodeControl<'_>,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<bool,ValueError>{
  self.progress=Default::default();if units==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}control.checkpoint()?;if self.complete{return Ok(true)}if self.next_capacity_byte_demand()?>grant.maximum_capacity_bytes||self.next_depth_demand()?>grant.maximum_depth{return Ok(false)}control.admit_turn_capacity(grant.maximum_capacity_bytes)?;let before=control.owned_bytes();let result=self.advance(viewport, graph,control);self.progress.copied_items=1;self.progress.retained_capacity_bytes=control.owned_bytes()-before;result?;control.step()?;Ok(self.complete)
 }
 fn next_row(&mut self){self.row+=1;self.phase=0;self.identity=IdentityCursor::default();self.node=None;self.port=0;}
 pub fn advance(&mut self,viewport: &DagViewportState, graph: &DagGraphState,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
  let facts=self.facts.as_ref().unwrap();
  if self.phase==0{
   let present=match facts{
    DagInputFacts::Progress(facts)=>if self.field==0{if facts.active.is_none(){self.field=1;return Ok(())}true}else{self.row<facts.stale.len()},
    DagInputFacts::Statuses(facts)=>self.row<facts.len(),
    DagInputFacts::Selection(facts)=>{let rows=match self.field{0=>&facts.nodes,1=>&facts.edges,_=>&facts.handles};if self.row==rows.len(){self.field+=1;self.row=0;if self.field==3{self.complete=true;}return Ok(())}true},
    DagInputFacts::Channels(facts)=>{self.channels=draw_lod_for_frame(viewport, graph).uses_channel_row_pick();self.row<facts.len()},
   };if !present{self.complete=true;return Ok(())}self.phase=1;return Ok(())
  }
  if self.phase==1{
   let(target,domain)=match facts{
    DagInputFacts::Progress(facts)=>(Identity{first:if self.field==0{facts.active.as_deref().unwrap()}else{facts.stale[self.row].as_str()},second:None},0),
    DagInputFacts::Statuses(facts)=>(Identity{first:facts.entry_at_rank(self.row).unwrap().0,second:None},0),
    DagInputFacts::Selection(facts)=>(Identity{first:match self.field{0=>&facts.nodes[self.row],1=>&facts.edges[self.row],_=>&facts.handles[self.row]},second:None},self.field),
    DagInputFacts::Channels(facts)=>(Identity{first:&facts[self.row].widget_id,second:self.channels.then_some(facts[self.row].port.as_str())},if self.channels{2}else{0}),
   };
   match self.identity.step(graph,target,domain,None){IdentityStep::Progress=>{},IdentityStep::Missing=>{if matches!(facts,DagInputFacts::Progress(_))&&self.field==0{self.field=1;self.identity=IdentityCursor::default();self.phase=0;}else{self.next_row()}},IdentityStep::Resolved(id)=>{
    self.node=Some(id);self.identity=IdentityCursor::default();self.phase=2;
   }}return Ok(())
  }
  if self.phase==2{
   let id=self.node.unwrap();match facts{
    DagInputFacts::Progress(_)=>{if self.field==0{self.active=Some(id);self.field=1;self.phase=0;self.identity=IdentityCursor::default();return Ok(())}if self.active==Some(id){self.next_row();return Ok(())}self.id_insert=Some(NumericInsertCursor::new(id,()));self.phase=3;},
    DagInputFacts::Statuses(facts)=>{let status=facts.entry_at_rank(self.row).unwrap().1;let kind=match status{DagNodeEvaluationStatus::Ok{}=>DagNodeEvalStatusKind::Ok,DagNodeEvaluationStatus::Queued{}=>DagNodeEvalStatusKind::Queued,DagNodeEvaluationStatus::Computing{}=>{self.active=Some(id);DagNodeEvalStatusKind::Computing},DagNodeEvaluationStatus::Error{..}=>DagNodeEvalStatusKind::Error,DagNodeEvaluationStatus::Blocked{..}=>DagNodeEvalStatusKind::Blocked};self.status_insert=Some(NumericInsertCursor::new(id,kind));self.phase=4;},
    DagInputFacts::Selection(_)|DagInputFacts::Channels(_)=>{self.id_insert=Some(NumericInsertCursor::new(id,()));self.phase=3;},
   }return Ok(())
  }
  if self.phase==3{
   let output=match facts{DagInputFacts::Progress(_)|DagInputFacts::Statuses(_)=>&mut self.stale,DagInputFacts::Selection(_)=>match self.field{0=>&mut self.selection.node_ids,1=>&mut self.selection.edge_ids,_=>&mut self.selection.handle_ids},DagInputFacts::Channels(_)=>if self.channels{&mut self.selection.handle_ids}else{&mut self.selection.node_ids}};
   if output.insert_step(self.id_insert.as_mut().unwrap(),1,control)?{self.id_insert.take();self.next_row();}return Ok(())
  }
  if self.phase==4{
   if self.status_insert.as_mut().unwrap().step(&mut self.statuses,1,control)?{self.status_insert.take();let DagInputFacts::Statuses(facts)=facts else{unreachable!()};match facts.entry_at_rank(self.row).unwrap().1{DagNodeEvaluationStatus::Queued{}=>{self.id_insert=Some(NumericInsertCursor::new(self.node.unwrap(),()));self.phase=3},DagNodeEvaluationStatus::Blocked{..}=>self.phase=5,_=>self.next_row()}}return Ok(())
  }
  if self.phase==5{
   let DagInputFacts::Statuses(facts)=facts else{unreachable!()};let DagNodeEvaluationStatus::Blocked{ports}=facts.entry_at_rank(self.row).unwrap().1 else{unreachable!()};if self.port==ports.len(){self.next_row();return Ok(())}
   match self.identity.step(graph,Identity{first:&ports[self.port],second:None},3,self.node){IdentityStep::Progress=>{},IdentityStep::Missing=>{self.port+=1;self.identity=IdentityCursor::default()},IdentityStep::Resolved(port)=>{self.port_insert=Some(NumericInsertCursor::new((self.node.unwrap(),port as usize),()));self.phase=6;}}return Ok(())
  }
  if self.ports.insert_step(self.port_insert.as_mut().unwrap(),1,control)?{self.port_insert.take();self.identity.candidate+=1;self.identity.byte=0;self.phase=5;}Ok(())
 }
 pub fn commit(&mut self,graph: &mut DagGraphState, paint: &mut DagPaintState)->DagInputDisplaced{
  assert!(self.complete);let mut displaced=DagInputDisplaced{selection:None,chrome:None,progress:None};
  match self.facts.as_ref().unwrap(){
   DagInputFacts::Selection(_)|DagInputFacts::Channels(_)=>displaced.selection=Some((std::mem::replace(&mut graph.engine.selection,std::mem::take(&mut self.selection)),std::mem::take(&mut graph.engine.preselect),std::mem::take(&mut graph.engine.preselect_removed))),
   DagInputFacts::Progress(_)=>displaced.progress=Some((std::mem::replace(&mut paint.computing_active,self.active),std::mem::replace(&mut paint.computing_stale,std::mem::take(&mut self.stale)))),
   DagInputFacts::Statuses(_)=>displaced.chrome=Some((std::mem::replace(&mut paint.computing_active,self.active),std::mem::replace(&mut paint.computing_stale,std::mem::take(&mut self.stale)),std::mem::replace(&mut paint.node_eval_status,std::mem::take(&mut self.statuses)),std::mem::replace(&mut paint.unresolved_input_ports,std::mem::take(&mut self.ports)))),
  }displaced
 }
}

semio_framework_value::artifact_retire_struct!(DagInputApplication{facts,selection,active,stale,statuses,ports,field,row,port,identity,node,phase,channels,id_insert,status_insert,port_insert,complete,progress});
