//! 🧵️ Drawing trace pointer job: the bounded trace pick job, its candidates and path helpers and the trace continuation queue.

use super::*;

//#region 🧵️TracePointerJob
pub(super) const TRACE_POINTER_WORK_PER_STEP: usize = 32;

pub(super) const TRACE_POINTER_MAX_DEPTH: usize = 32;

const TRACE_POINTER_WORK_CAPACITY: usize = TRACE_POINTER_MAX_DEPTH * 3 + 4;

pub(super) const DRAWING_QUERY_HIT_CAPACITY: usize = 256;

pub(super) const DRAWING_QUERY_TARGET_BYTES: usize = 8_192;

pub(super) const MAX_GESTURE_POINTS: usize = 48;

static NEXT_TRACE_POINTER_REQUEST: AtomicU64 = AtomicU64::new(20_000);

#[derive(Clone, Copy, Debug, Default, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue)]
pub(super) struct TracePath {
    pub(super) indices: [u16; TRACE_POINTER_MAX_DEPTH],
    pub(super) len: u8,
}

impl TracePath {
    pub(super) fn root(index: usize) -> Option<Self> {
        let mut path = Self::default();
        path.indices[0] = u16::try_from(index).ok()?;
        path.len = 1;
        Some(path)
    }

    pub(super) fn child(mut self, index: usize) -> Option<Self> {
        let next = usize::from(self.len);
        if next >= TRACE_POINTER_MAX_DEPTH {
            return None;
        }
        self.indices[next] = u16::try_from(index).ok()?;
        self.len += 1;
        Some(self)
    }

    pub(super) fn indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.indices[..usize::from(self.len)].iter().map(|index| usize::from(*index))
    }
}

#[derive(Clone, Debug)]
enum TracePointerWork {
    SelectionPaths {next:usize},
    SelectionBounds {next:usize},
    ControlNodes {next:usize},
    ControlSegments {index:usize,path:TracePath,next:usize,matrix:[f64;6]},
    NodeArea {path:TracePath,next:usize,matrix:[f64;6]},
    PublishNodeArea {path:TracePath,next:usize,geometry:String},
    Pick,
    PublishPick {next:usize},
}

#[derive(Clone, Debug, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
pub struct TracePickCandidate {
    generality: i32,
    pub layer_id: String,
    image_key: Option<String>,
    pub(super) path: TracePath,
}

// No `ToValue`/`FromValue`: `work`/`hits` are framework `UiFixedList` fields with no `ToValue`
// impl — never serialized in this plugin (grep-confirmed).
pub struct TracePointerJob {
    pub(super) app_instance_id: u32,
    pub(super) document_id: PagedUtf8<{usize::MAX}>,
    pub(super) operation_id: u64,
    pub(super) generation: u64,
    pub(super) base_revision: String,
    pub(super) world: [f64; 2],
    pub(super) tolerance: f64,
    include_control_points: bool,
    pub(super) marquee: Option<([f64; 2], [f64; 2], bool)>,
    lasso: Option<LassoPolygon>,
    pub(super) work: UiFixedList<TracePointerWork, TRACE_POINTER_WORK_CAPACITY>,
    pub best: Option<TracePickCandidate>,
    pub hits: UiFixedList<String, DRAWING_QUERY_HIT_CAPACITY>,
    pub node_editing: bool,
    pub(super) node_area:bool,
    node_hash:Option<semio_framework_hash::Hasher>,
    node_indices:Vec<usize>,
    node_bytes:usize,
    pub(super) node_hit: Option<(TracePath,usize,PathPoint,f64)>,
    pub(super) selected_ids: Vec<String>,
    selected_paths: Vec<TracePath>,
    pub(super) selection_bounds: Option<[f64;4]>,
    pub(super) completed_work: usize,
    cache_identity:Option<PreparedSceneIdentity>,
    pick:Option<PreparedScenePickJob>,
    pub failure:Option<String>,
    pub overflowed: bool,
}

impl TracePointerJob {
    pub(super) fn new(generation: u64, document: &DrawingSnapshot, world: [f64; 2]) -> Self {
        Self::new_bound(generation, document, world, format!("unbound:{}", document.id))
    }

    fn new_bound(generation: u64, document: &DrawingSnapshot, world: [f64; 2], base_revision: String) -> Self {
        let mut work = UiFixedList::default();
        let _ = work.try_push(TracePointerWork::SelectionPaths {next:0});
        Self {
            app_instance_id: 0,
            document_id: document.id.clone(),
            operation_id: 0,
            generation,
            base_revision,
            world,
            tolerance: 0.0,
            include_control_points: false,
            marquee: None,
            lasso: None,
            work,
            best: None,
            hits: UiFixedList::default(),
            node_editing: false,
            node_area:false,node_hash:None,node_indices:Vec::new(),node_bytes:0,
            node_hit: None,
            selected_ids: Vec::new(),
            selected_paths: Vec::new(),
            selection_bounds: None,
            completed_work: 0,
            cache_identity:None,pick:None,failure:None,
            overflowed: false,
        }
    }

    pub(super) fn new_operation(operation: &AppOperationContext, document: &DrawingSnapshot, world: [f64; 2]) -> Self {
        let mut job = Self::new_bound(operation.generation, document, world, operation.canonical_base_revision_hex());
        job.app_instance_id = operation.app_instance_id;
        job.document_id = operation.parent_document_id.as_str().into();
        job.operation_id = operation.operation_id;
        job
    }

    pub fn new_query(document: &DrawingSnapshot, world: [f64; 2], tolerance: f64, include_control_points: bool) -> Self {
        let mut job = Self::new(0, document, world);
        job.tolerance = tolerance.max(0.0);
        job.include_control_points = include_control_points;
        job
    }

    pub fn retain_selection_bounds(&mut self,ids: &[String]) -> Result<(),Fault> {
        if ids.len()>DRAWING_QUERY_HIT_CAPACITY || ids.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Selection exceeds gesture capacity"));}
        self.selected_ids=ids.to_vec();
        Ok(())
    }

    pub fn new_marquee(document: &DrawingSnapshot, start: [f64; 2], end: [f64; 2], crossing: bool) -> Self {
        let mut job = Self::new(0, document, end);
        job.marquee = Some((start, end, crossing));
        job
    }

    pub fn new_lasso(document: &DrawingSnapshot, polygon: LassoPolygon) -> Self {
        let mut job=Self::new(0,document,[0.0;2]);
        job.lasso=Some(polygon);
        job
    }

    fn push_work(&mut self, work: TracePointerWork) {
        if self.work.try_push(work).is_err() {
            self.overflowed = true;
        }
    }

    /// 🪪️ Every granted step and eventual publication refers to the exact complete cache.
    pub fn validate_cache(&self,borrowed:&MountedSceneQuery<'_>)->Result<(),Fault>{
        let live=PreparedSceneIdentity{source:borrowed.source,build:borrowed.build,flatness:borrowed.scene.flatness};
        if self.cache_identity!=Some(live) {return Err(Fault::from("Drawing geometry changed during the pointer query"));}
        if let Some(error)=&self.failure {return Err(Fault::from(error.clone()));}
        Ok(())
    }

    pub fn advance(&mut self,document:&DrawingSnapshot,borrowed:&MountedSceneQuery<'_>)->bool {
        if self.overflowed{return true;}
        let live=PreparedSceneIdentity{source:borrowed.source,build:borrowed.build,flatness:borrowed.scene.flatness};
        if self.cache_identity.is_none(){self.cache_identity=Some(live);}
        if let Err(error)=self.validate_cache(borrowed){self.failure=Some(error.message);self.overflowed=true;return true;}
        let scene=borrowed.scene;
        for _ in 0..TRACE_POINTER_WORK_PER_STEP {
            let Some(work)=self.work.pop()else{return true};self.completed_work+=1;
            let step=(||->Result<(),Fault>{
                match work {
                    TracePointerWork::SelectionPaths{next}=>{
                        if let Some(node)=scene.plan.nodes.get(next) {
                            if self.selected_ids.contains(&node.id) {self.selected_paths.push(TracePath::from_source(&node.source_path).ok_or_else(||Fault::from("Invalid prepared layer address"))?);}
                            self.push_work(TracePointerWork::SelectionPaths{next:next+1});
                        }else{self.push_work(TracePointerWork::SelectionBounds{next:0});}
                    }
                    TracePointerWork::SelectionBounds{next}=>{
                        if let Some(node)=scene.plan.nodes.get(next) {
                            if node.visible&&node.opacity>0.0&&node.groups.iter().all(|group|group.opacity>0.0)&&semio_s_artifact_draw_drawing_scene::preparation::selection::scene_selection_relation(&node.source_path,node.locked_ancestors,&self.selected_paths).map_err(|error|Fault::from(error.to_string()))?.bounds_selection.is_some() {
                                if let Some([x,y,r,b])=scene.geometry[next].bounds.or(scene.geometry[next].geometry_bounds) {let bounds=[x,y,r-x,b-y];self.selection_bounds=Some(self.selection_bounds.map_or(bounds,|old|{let(x,y)=(old[0].min(bounds[0]),old[1].min(bounds[1]));[x,y,(old[0]+old[2]).max(bounds[0]+bounds[2])-x,(old[1]+old[3]).max(bounds[1]+bounds[3])-y]}));}
                            }
                            self.push_work(TracePointerWork::SelectionBounds{next:next+1});
                        }else if self.node_area||self.node_editing||self.include_control_points{self.push_work(TracePointerWork::ControlNodes{next:scene.plan.nodes.len()});}
                        else{self.push_work(TracePointerWork::Pick);}
                    }
                    TracePointerWork::ControlNodes{next}=>{
                        if next==0 {if !self.node_area{self.push_work(TracePointerWork::Pick);}return Ok(());}
                        let index=next-1;self.push_work(TracePointerWork::ControlNodes{next:index});let node=&scene.plan.nodes[index];
                        if !node.visible||node.opacity<=0.0||node.groups.iter().any(|group|group.opacity<=0.0)||node.locked_ancestors!=0{return Ok(());}
                        let path=TracePath::from_source(&node.source_path).ok_or_else(||Fault::from("Invalid prepared layer address"))?;
                        if !matches!(drawing_layer_at_path(&document.layers,&path),Some(DrawingLayerNode::Path(_))){return Ok(());}
                        if self.node_area {if self.selected_ids.contains(&node.id){self.node_hash=Some(points::geometry_hasher());self.node_indices.clear();self.push_work(TracePointerWork::NodeArea{path,next:0,matrix:node.transform});}}
                        else if self.node_editing&&self.selected_ids.contains(&node.id)||self.include_control_points{self.push_work(TracePointerWork::ControlSegments{index,path,next:0,matrix:node.transform});}
                    }
                    TracePointerWork::ControlSegments{index,path,next,matrix}=>{
                        let Some(DrawingLayerNode::Path(layer))=drawing_layer_at_path(&document.layers,&path)else{return Err(Fault::from("Authored control path changed"));};
                        if let Some(segment)=layer.segments.get(next){
                            if let Some((point,distance))=path_point_hit(segment,matrix,self.world,self.tolerance){
                                if self.node_editing&&self.selected_ids.iter().any(|id|layer.base.id.eq_str(id))&&self.node_hit.as_ref().is_none_or(|(previous,_,_,old)|previous.indices==path.indices&&previous.len==path.len&&distance<*old){self.node_hit=Some((path,next,point,distance));}
                                if self.include_control_points&&self.best.as_ref().is_none_or(|best|best.generality<4){self.best=Some(TracePickCandidate{generality:4,layer_id:scene.plan.nodes[index].id.clone(),image_key:None,path});}
                            }
                            self.push_work(TracePointerWork::ControlSegments{index,path,next:next+1,matrix});
                        }
                    }
                    TracePointerWork::NodeArea{path,next,matrix}=>{
                        let Some(DrawingLayerNode::Path(layer))=drawing_layer_at_path(&document.layers,&path)else{return Err(Fault::from("Authored anchor path changed"));};
                        if let Some(segment)=layer.segments.get(next){
                            points::hash_segment(self.node_hash.as_mut().expect("active node hash"),segment).ok_or_else(||Fault::from("Invalid authored anchor"))?;
                            if self.marquee.is_some_and(|(start,end,_)|points::anchor_in_marquee(segment,matrix,start,end)){if self.node_indices.len()+self.hits.len()>=DRAWING_QUERY_HIT_CAPACITY{return Err(Fault::from("Anchor selection exceeds capacity"));}self.node_indices.push(next);}
                            self.push_work(TracePointerWork::NodeArea{path,next:next+1,matrix});
                        }else{let geometry=self.node_hash.take().expect("finished node hash").finalize().to_hex();self.push_work(TracePointerWork::PublishNodeArea{path,next:0,geometry});}
                    }
                    TracePointerWork::PublishNodeArea{path,next,geometry}=>{
                        if let Some(index)=self.node_indices.get(next){
                            let layer=drawing_layer_at_path(&document.layers,&path).ok_or_else(||Fault::from("Authored anchor path changed"))?;
                            let id=points::point_id(&trace_layer_base(layer).id,&geometry,*index,PathPoint::Anchor).ok_or_else(||Fault::from("Invalid anchor selection"))?;
                            self.node_bytes+=id.len();if self.node_bytes>DRAWING_QUERY_TARGET_BYTES||self.hits.try_push(id).is_err(){return Err(Fault::from("Anchor selection exceeds capacity"));}
                            self.push_work(TracePointerWork::PublishNodeArea{path,next:next+1,geometry});
                        }
                    }
                    TracePointerWork::Pick=>{
                        if self.pick.is_none(){
                            let query=if let Some(polygon)=&self.lasso{PreparedScenePick::Lasso{points:polygon.as_slice().to_vec()}}else if let Some((start,end,crossing))=self.marquee{PreparedScenePick::Rectangle{start,end,crossing}}else{PreparedScenePick::Point{point:self.world,tolerance:self.tolerance,required_flatness:if self.tolerance==0.0{scene.flatness}else{(self.tolerance/80.0).clamp(1e-6,16.0)}}};
                            self.pick=Some(PreparedScenePickJob::new(live,query,DRAWING_QUERY_HIT_CAPACITY).map_err(Fault::from)?);
                            self.push_work(TracePointerWork::Pick);
                        }else if self.pick.as_mut().unwrap().advance(scene,live,1).map_err(Fault::from)?.done{self.push_work(TracePointerWork::PublishPick{next:0});}
                        else{self.push_work(TracePointerWork::Pick);}
                    }
                    TracePointerWork::PublishPick{next}=>{
                        if let Some(index)=self.pick.as_ref().unwrap().result(live).map_err(Fault::from)?.get(next).copied(){
                            let node=&scene.plan.nodes[index];let path=TracePath::from_source(&node.source_path).ok_or_else(||Fault::from("Invalid prepared layer address"))?;
                            if self.marquee.is_some()||self.lasso.is_some(){self.node_bytes+=node.id.len();if self.node_bytes>DRAWING_QUERY_TARGET_BYTES||self.hits.try_push(node.id.clone()).is_err(){return Err(Fault::from("Layer selection exceeds capacity"));}}
                            else if self.best.as_ref().is_none_or(|best|best.generality<4){self.best=Some(TracePickCandidate{generality:2,layer_id:node.id.clone(),image_key:match &node.content{semio_s_artifact_draw_drawing_scene::preparation::DocumentSceneContent::Image{asset,..}=>Some(asset.clone()),_=>None},path});}
                            self.push_work(TracePointerWork::PublishPick{next:next+1});
                        }
                    }
                }Ok(())
            })();
            if let Err(error)=step{self.failure=Some(error.message);self.overflowed=true;return true;}
        }
        self.work.is_empty()
    }
}

impl AsRef<[u16]> for TracePath{fn as_ref(&self)->&[u16]{&self.indices[..usize::from(self.len)]}}

impl TracePath{
 fn from_source(source:&[u16])->Option<Self>{if source.is_empty()||source.len()>TRACE_POINTER_MAX_DEPTH{return None;}let mut path=Self{indices:[0;TRACE_POINTER_MAX_DEPTH],len:source.len()as u8};path.indices[..source.len()].copy_from_slice(source);Some(path)}
}

pub(super) fn drawing_layer_at_path<'a>(roots: &'a PagedList<DrawingLayerNode,{usize::MAX}>, path: &TracePath) -> Option<&'a DrawingLayerNode> {
    let mut indices = path.indices();
    let mut layer = roots.get(indices.next()?)?;
    for index in indices {
        let DrawingLayerNode::Group(group) = layer else { return None };
        layer = group.children.get(index)?;
    }
    Some(layer)
}

pub(super) fn trace_path_matrix(roots: &PagedList<DrawingLayerNode,{usize::MAX}>, path: &TracePath) -> Option<[f64;6]> {
    let mut layers=roots;
    let mut matrix=[1.0,0.0,0.0,1.0,0.0,0.0];
    let mut indices=path.indices().peekable();
    while let Some(index)=indices.next() {
        let layer=layers.get(index)?;
        matrix=semio_s_artifact_draw_drawing_geometry::multiply(matrix,semio_s_artifact_draw_drawing_model::affine::drawing_transform_to_matrix(&trace_layer_base(layer).transform));
        if indices.peek().is_some() {
            let DrawingLayerNode::Group(group)=layer else {return None;};
            layers=&group.children;
        }
    }
    Some(matrix)
}

pub(super) fn trace_layer_base(layer: &DrawingLayerNode) -> &semio_s_artifact_draw_drawing_model::DrawingLayerBase {
    match layer {
        DrawingLayerNode::Shape(value) => &value.base,
        DrawingLayerNode::Path(value) => &value.base,
        DrawingLayerNode::Text(value) => &value.base,
        DrawingLayerNode::Image(value) => &value.base,
        DrawingLayerNode::Group(value) => &value.base,
        DrawingLayerNode::Boolean(value) => &value.base,
        DrawingLayerNode::Trace(value) => &value.base,
    }
}
//#endregion 🧵️TracePointerJob

//#region 🧵️TracePointerContinuation
fn retain_trace_progress(session: &mut DrawingSession, job: &TracePointerJob) {
    session.window_transient.trace_pointer_generation = job.generation;
    session.window_transient.trace_pointer_completed_work = job.completed_work as u64;
    session.window_transient.trace_pointer_pending_work = job.work.len() as u64;
}

fn queue_trace_pointer(payload: &CanvasPointerDown, job: &TracePointerJob) -> Effect {
    let continuation = CanvasPointerDown {
        app_instance_id: Some(job.app_instance_id),
        parent_document_id: Some(job.document_id.to_string_owner()),
        operation_id: Some(job.operation_id),
        generation: Some(job.generation),
        base_revision: Some(job.base_revision.clone()),
        world_x: Some(job.world[0]),
        world_y: Some(job.world[1]),
        checkpoint_completed_work: Some(job.completed_work as u64),
        checkpoint_pending_work: Some(job.work.len() as u64),
        ..payload.clone()
    };
    let args = Some(semio_framework_value::ToValue::to_value(&continuation));
    Effect::DispatchAction { req: RequestId(NEXT_TRACE_POINTER_REQUEST.fetch_add(1, Ordering::Relaxed)), action: "canvasPointerDown".into(), args, delay_ms: 0 }
}

/// 🖼️ One bounded step of the trace pointer job: requeued while the document walk continues, then the canvas tool commits
/// the trace layer of the image under the pointer as one transaction.
pub(super) fn advance_trace_pointer(session: &mut DrawingSession, mut job: TracePointerJob, payload: &CanvasPointerDown, base: DrawingToolBase) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    let captured=session.source_identity.ok_or_else(||Fault::from("Drawing pointer has no source authority"))?;
    semio_s_artifact_draw_drawing_editor_interaction::geometry_session::prepare_query(captured,&base.document);
    let complete=semio_s_artifact_draw_drawing_editor_interaction::geometry_session::with_query(captured,|status,borrowed|match status{
        semio_s_artifact_draw_drawing_scene::identity::admission::SceneAdmissionStatus::Pending=>Ok(false),
        semio_s_artifact_draw_drawing_scene::identity::admission::SceneAdmissionStatus::Ready=>{let complete=job.advance(&base.document,&borrowed.unwrap());if job.overflowed{Err(Fault::from(job.failure.clone().unwrap_or_else(||"Drawing pointer exceeds capacity".into())))}else{Ok(complete)}},
        _=>Err(Fault::from("Drawing pointer geometry is unavailable or changed")),
    })?;
    if !complete {
        let effect = queue_trace_pointer(payload, &job);
        retain_trace_progress(session, &job);
        let _ = session.retain_trace_pointer(job);
        return Ok(Emit { effects: vec![effect], ..Default::default() });
    }
    let source_key = job.best.and_then(|candidate| candidate.image_key).or_else(|| base.document.assets.keys().next().map(PagedUtf8::to_string_owner));
    session.window_transient.trace_pointer_generation = 0;
    session.window_transient.trace_pointer_completed_work = 0;
    session.window_transient.trace_pointer_pending_work = 0;
    session.traced(base, source_key)
}
//#endregion 🧵️TracePointerContinuation
