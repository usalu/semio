//! 🖌️ Drawing session: the retained gesture session with its previews, grab preparation, point queries and the pointer entry points of the canvas tool.

use super::*;

//#region 🔖️DrawingSession
/// 🕹️ Owned snapshot of the framework's `"strokes"` and `"points"` selections, read once per dispatch and threaded through
/// `DrawingSession` to every command handler — decouples handlers from `semio_framework_plugin::app::InteractionView`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawingInteractionSnapshot {
    pub ids: Vec<String>,
    pub points: Vec<String>,
}

/// 🧪️ `app_commands!` dispatch context — the canvas tool, the preview tick counter, the current selections, the Canvas
/// window's state, the committed base of this dispatch and the retained hit-test and trace jobs.
pub struct DrawingSession {
    pub identity_cancel:Option<semio_framework_job::CancelToken>,
    pub identity_checkpoints:u64,
    /// 🛠️ The canvas tool driving pointer gestures (ephemeral tool state, never history).
    pub tool: DrawingTool,
    /// 👻️ Monotone preview counter.
    preview_seq: u64,
    /// 🕹️ Current `"strokes"` / `"points"` selection — set before every dispatch.
    pub interaction: DrawingInteractionSnapshot,
    pub active_utility_id: String,
    pub window_config: DrawingCanvasWindowConfig,
    pub window_transient: DrawingCanvasWindowTransient,
    /// 🧱️ The committed document and admission of the current dispatch, shared without a copy when the retained owner has it.
    pub base: Option<DrawingToolBase>,
    pub trace_pointer: Option<TracePointerJob>,
    pub source_identity:Option<semio_s_artifact_draw_drawing_scene::identity::SceneIdentity>,
    pub point_query: Option<DrawingPointQuery>,
    move_sample_cursor: usize,
    pub node_marquee: Option<NodeMarquee>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DrawingGesturePreviewPhase {
    Move,
    Marquee,
    Shape,
    Draft,
    #[default]
    Idle,
}

/// 👁️ What the Canvas window paints of an in-flight gesture — derived from the tool's context only, never history.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawingGesturePreview {
    pub sequence: u64,
    pub phase: DrawingGesturePreviewPhase,
    pub context: DrawingToolContext,
    pub transformation: Option<(Vec<String>,[f64;6])>,
    pub node_translation: Option<DrawingNodeTranslation>,
}

/// 📍️ The points a node drag moves and its world offset so far.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawingNodeTranslation {
    pub targets:Vec<DrawingPathPointTarget>,
    pub delta:[f64;2],
}

pub struct NodeMarquee {
    layer_ids:Vec<String>,
    pub(super) current:Vec<String>,
    pub(super) mode:points::PointPickMode,
}

/// ✊️ Resolves what a select press grabs, in bounded steps: every selected layer found in document order, a layer inside
/// an already grabbed group dropped (it moves with the group), a locked, hidden or singular ancestry refused.
pub(super) struct LayerGrabPreparation {
    pub(super) ids: Vec<String>,
    pub(super) next: Option<TracePath>,
    pub(super) found: usize,
    pub(super) targets: Vec<(TracePath, PagedUtf8<{usize::MAX}>)>,
    pub(super) handle: Option<(usize, [f64; 4])>,
}

fn next_layer_path(document: &DrawingSnapshot, mut path: TracePath) -> Option<TracePath> {
    if matches!(drawing_layer_at_path(&document.layers,&path),Some(DrawingLayerNode::Group(group)) if !group.children.is_empty()) { return path.child(0); }
    loop {
        let depth=usize::from(path.len).checked_sub(1)?;
        let count=if depth==0 { document.layers.len() } else {
            let mut parent=path; parent.len-=1;
            match drawing_layer_at_path(&document.layers,&parent)? { DrawingLayerNode::Group(group)=>group.children.len(),_=>0 }
        };
        if usize::from(path.indices[depth])+1<count {
            path.indices[depth]=path.indices[depth].checked_add(1)?;
            return Some(path);
        }
        path.len-=1;
    }
}

impl LayerGrabPreparation {
    pub(super) fn new(document: &DrawingSnapshot, ids: Vec<String>, handle: Option<(usize, [f64; 4])>) -> Self {
        Self { ids, next: (!document.layers.is_empty()).then(|| TracePath::root(0)).flatten(), found: 0, targets: Vec::new(), handle }
    }

    pub(super) fn advance(&mut self, document: &DrawingSnapshot) -> Result<bool,Fault> {
        if self.found==self.ids.len() { return Ok(true); }
        let Some(path)=self.next else {
            if self.found!=self.ids.len() { return Err(Fault::from("A selected layer no longer exists")); }
            return Ok(true);
        };
        let layer=drawing_layer_at_path(&document.layers,&path).ok_or_else(||Fault::from("A selected layer no longer exists"))?;
        if matches!(layer,DrawingLayerNode::Group(group) if !group.children.is_empty()) && usize::from(path.len)>=TRACE_POINTER_MAX_DEPTH { return Err(Fault::from("Selection exceeds the gesture traversal depth")); }
        self.next=next_layer_path(document,path);
        let base=trace_layer_base(layer);
        if !self.ids.iter().any(|id|base.id.eq_str(id)) { return Ok(false); }
        self.found+=1;
        if self.targets.iter().any(|(target,_)|semio_s_artifact_draw_drawing_geometry::translation::path_contains(&target.indices[..usize::from(target.len)],&path.indices[..usize::from(path.len)])) { return Ok(false); }
        let mut prefix=path;
        while prefix.len>0 {
            let ancestor=trace_layer_base(drawing_layer_at_path(&document.layers,&prefix).ok_or_else(||Fault::from("Selected ancestry changed"))?);
            if ancestor.locked || !ancestor.visible { return Err(Fault::from("Unlock and show the selected layers before moving")); }
            prefix.len-=1;
        }
        let mut parent_path=path; parent_path.len-=1;
        let parent=trace_path_matrix(&document.layers,&parent_path).ok_or_else(||Fault::from("Selected parent transform is unavailable"))?;
        if semio_s_artifact_draw_drawing_geometry::inverse(parent).is_none() { return Err(Fault::from("Cannot move through a singular transform")); }
        self.targets.push((path,base.id.clone()));
        Ok(false)
    }

    pub(super) fn grab(self) -> DrawingGrab {
        DrawingGrab::Layers { targets: self.targets.into_iter().map(|(_, id)| id).collect(), handle: self.handle }
    }
}

pub struct DrawingPointQuery {
    pub command_id: &'static str,
    pub cursor: TracePointerJob,
    pub hover: bool,
    pub merge: String,
    pub marquee: bool,
    pub traversal_complete: bool,
    pub drag_start: Option<[f64;2]>,
    pub constrained: bool,
    pub centered: bool,
    grab_preparation: Option<LayerGrabPreparation>,
    grab_prepared: bool,
    pub preserve_selection: bool,
    pub node_selection: Option<Vec<String>>,
    area_current:Vec<String>,
    pub point_pick_mode: points::PointPickMode,
    target_cursor: usize,
    pub(super) targets: String,
}

pub enum DrawingQueryPublication {
    Pending,
    Complete(String),
    Fault,
}

impl DrawingPointQuery {
    pub fn new(command_id: &'static str, cursor: TracePointerJob, hover: bool, merge: String, marquee: bool) -> Self {
        Self { command_id, cursor, hover, merge, marquee, traversal_complete: false, drag_start: None, constrained:false, centered:false, grab_preparation: None, grab_prepared: false, preserve_selection: false, node_selection:None, area_current:Vec::new(), point_pick_mode:points::PointPickMode::Replace, target_cursor: 0, targets: String::with_capacity(DRAWING_QUERY_TARGET_BYTES) }
    }

    pub fn publication_step(&mut self) -> DrawingQueryPublication {
        if self.targets.is_empty() {
            self.targets.push('[');
        }
        let id = if let Some(selection)=&self.node_selection {
            selection.get(self.target_cursor)
        } else if self.marquee {
            self.cursor.hits.get(self.target_cursor)
        } else if self.target_cursor == 0 {
            self.cursor.best.as_ref().map(|candidate| &candidate.layer_id)
        } else {
            None
        };
        if let Some(id) = id {
            let id = semio_framework_pack_json::to_json_string(id);
            let prefix = if self.target_cursor == 0 { "" } else { "," };
            let granularity=if self.node_selection.is_some() {DRAWING_POINT_GRANULARITY}else {DRAWING_INTERACTION_GRANULARITY};
            let item = format!("{prefix}{{\"granularity\":\"{granularity}\",\"id\":{id}}}");
            if self.targets.len().checked_add(item.len()).is_none_or(|bytes| bytes >= DRAWING_QUERY_TARGET_BYTES) {
                return DrawingQueryPublication::Fault;
            }
            self.targets.push_str(&item);
            self.target_cursor += 1;
            return DrawingQueryPublication::Pending;
        }
        if self.targets.len().checked_add(1).is_none_or(|bytes| bytes > DRAWING_QUERY_TARGET_BYTES) {
            return DrawingQueryPublication::Fault;
        }
        self.targets.push(']');
        DrawingQueryPublication::Complete(std::mem::take(&mut self.targets))
    }
}

/// 📍️ The point ids a committed node drag leaves selected: every dragged point re-bound to its path's moved geometry.
pub fn drawing_rebound_points(base: &DrawingSnapshot, leaf: &DrawingMutation) -> Result<Vec<String>, Fault> {
    let DrawingMutation::DragPathPoints(drag) = leaf else { return Ok(Vec::new()) };
    let mut moved = base.clone();
    semio_s_artifact_draw_drawing_mutations::text::apply_drawing_mutation(&mut moved, leaf).map_err(|error| Fault::from(error.to_string()))?;
    let mut rebound = Vec::with_capacity(drag.targets.len());
    for target in &drag.targets {
        let Some(DrawingLayerNode::Path(path)) = semio_s_artifact_draw_drawing_model::find_drawing_layer(&moved, &target.layer_id) else { continue };
        let geometry = points::geometry_id(&path.segments).ok_or_else(|| Fault::from("Invalid moved path geometry"))?;
        rebound.push(points::point_id(&target.layer_id, &geometry, target.index, target.point).ok_or_else(|| Fault::from("Invalid selected point"))?);
    }
    Ok(rebound)
}

impl Default for DrawingSession {
    fn default() -> Self {
        Self::new(semio_s_artifact_draw_drawing_editor_interaction::DRAWING_DEFAULT_UTILITY, "")
    }
}

impl DrawingSession {
    pub fn with_identity_control<T>(&mut self,operation:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,Fault>)->Result<T,Fault>{
        let cancel=self.identity_cancel.as_ref().ok_or_else(||Fault::from("Drawing identity requires its caller cancellation scope"))?;
        let checkpoints=&mut self.identity_checkpoints;
        let mut observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|{*checkpoints=checkpoints.saturating_add(1);};
        semio_s_artifact_draw_drawing_io::text::identity::control::with_identity_control(cancel,1024*1024,&mut observer,operation)
    }

    /// 🚀️ A session at rest for `active_utility_id`, its canvas tool minting from `authoring_seed`.
    pub fn new(active_utility_id: &str, authoring_seed: &str) -> Self {
        Self {
            tool: DrawingTool::start(active_utility_id, authoring_seed),
            identity_cancel:None,identity_checkpoints:0,
            preview_seq: 0,
            interaction: DrawingInteractionSnapshot::default(),
            active_utility_id: active_utility_id.into(),
            window_config: DrawingCanvasWindowConfig::default(),
            window_transient: DrawingCanvasWindowTransient::default(),
            base: None,
            trace_pointer: None,
            source_identity:None,
            point_query: None,
            move_sample_cursor: 0,
            node_marquee: None,
        }
    }

    /// 🧱️ The committed base a tool commit yields against: the retained owner's shared document, else a copy of `doc`.
    pub fn tool_base(&self, doc: &ArtifactView<'_, DrawingSnapshot>) -> DrawingToolBase {
        self.base.clone().unwrap_or_else(|| DrawingToolBase { document: Arc::new(doc.snapshot.clone()), operation: doc.operation_optional().cloned() })
    }

    pub(super) fn threshold(&self) -> f64 {
        DRAWING_MARQUEE_THRESHOLD_PX / self.window_config.viewport.zoom.max(1e-6)
    }

    pub(super) fn tolerance(&self) -> f64 {
        DRAWING_PICK_TOLERANCE_PX / self.window_config.viewport.zoom.max(1e-6)
    }

    pub(super) fn step(&mut self, event: canvas_tool::Event) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        let step = self.tool.send(event);
        self.preview_seq = self.preview_seq.wrapping_add(1);
        drawing_tool_emit(step)
    }

    /// 🖱️ One press on the canvas.
    pub fn press(&mut self, pointer: DrawingPointer) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        self.step(canvas_tool::Event::PointerDown(pointer))
    }

    /// ↔️ One pointer sample with the live transform modifiers (shift constrains, alt centres).
    pub fn sample(&mut self, world: [f64; 2], constrained: bool, centered: bool) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        if !world.iter().all(|value| value.is_finite()) {
            return Ok(Emit::default());
        }
        let threshold = self.threshold();
        self.step(canvas_tool::Event::PointerMove(DrawingPointerMove { world, threshold, constrained, centered }))
    }

    /// 🚪️ Escape: a drag aborts with zero trace, a marquee, shape or draft is dropped, a node marquee forgotten.
    pub fn escape(&mut self) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        self.node_marquee = None;
        self.step(canvas_tool::Event::Escape)
    }

    /// 🚫️ A cancelled release (pointer left the canvas, capture lost): the live gesture aborts with zero trace and nothing
    /// is selected, picked or committed; a click-sequenced draft is left untouched.
    pub fn cancel(&mut self) -> Emit<DrawingMutation, NoConfigMutation> {
        if !self.tool.matches("drafting") {
            self.node_marquee = None;
            let _ = self.tool.abort(ToolAbortReason::CaptureLost);
            self.preview_seq = self.preview_seq.wrapping_add(1);
        }
        Emit::default()
    }

    /// ✅️ Commits the open pen or polygon draft as one transaction.
    pub fn finish_draft(&mut self, base: DrawingToolBase) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        let context=self.tool.context().clone();
        let identity=if context.points.len()<2{None}else{let operation=base.operation.as_ref().ok_or_else(||Fault::from("Drawing draft lacks original operation admission"))?;Some(self.with_identity_control(|control|crate::prepare_identity(&base.document,if context.utility=="pen"{"path"}else{"shape:polygon"},operation,control))?)};
        self.step(canvas_tool::Event::CommitDraft(DrawingDraftCommit {base,identity}))
    }

    /// 🖼️ Commits a finished trace as one transaction.
    pub(super) fn traced(&mut self, base: DrawingToolBase, source_key: Option<String>) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
        let identity=if source_key.is_none(){None}else{let operation=base.operation.as_ref().ok_or_else(||Fault::from("Drawing trace lacks original operation admission"))?;Some(self.with_identity_control(|control|crate::prepare_identity(&base.document,"trace",operation,control))?)};
        self.step(canvas_tool::Event::Traced(DrawingTraced { base, source_key,identity }))
    }

    /// ⬆️ One release. A marquee's or a resting select utility's release is interaction: it arms the bounded selection query
    /// (`None` until it publishes). Every other release goes to the tool, which commits a drag or a shape as ONE transaction;
    /// a committed node drag re-binds the point selection to the moved geometry.
    pub fn release(&mut self, command_id: &'static str, pointer: DrawingPointer, base: DrawingToolBase) -> Result<Option<Emit<DrawingMutation, NoConfigMutation>>, Fault> {
        let (threshold, tolerance) = (self.threshold(), self.tolerance());
        let document = base.document.clone();
        if self.tool.matches("marqueeing") {
            let context = self.tool.context().clone();
            self.step(canvas_tool::Event::PointerUp(DrawingRelease { pointer: pointer.clone(), threshold, base,identity:None }))?;
            let end = pointer.world;
            if let Some(selection) = self.node_marquee.take() {
                if !context.active {
                    let ids = points::merge_point_selection(&selection.current, &[], selection.mode);
                    let mut emit = Emit::default();
                    emit.effects.push(point_selection_effect(&ids));
                    return Ok(Some(emit));
                }
                let mut cursor = TracePointerJob::new_marquee(&document, context.start, end, false);
                cursor.node_editing = true;
                cursor.node_area = true;
                cursor.selected_ids = selection.layer_ids;
                let mut query = DrawingPointQuery::new(command_id, cursor, false, "replace".into(), true);
                query.preserve_selection = true;
                query.area_current = selection.current;
                query.point_pick_mode = selection.mode;
                self.point_query = Some(query);
                return Ok(None);
            }
            self.point_query = Some(if context.active {
                let query = if context.method == "lasso" { TracePointerJob::new_lasso(&document, LassoPolygon::from_points(&context.points, end)) } else { TracePointerJob::new_marquee(&document, context.start, end, end[0] < context.start[0]) };
                DrawingPointQuery::new(command_id, query, false, context.merge.clone(), true)
            } else {
                DrawingPointQuery::new(command_id, TracePointerJob::new_query(&document, end, tolerance, false), false, selection_merge_mode(pointer.shift, pointer.ctrl, pointer.meta).into(), false)
            });
            return Ok(None);
        }
        if self.tool.at_rest() && matches!(pointer.utility.as_str(), "selectDirect" | "editNodes") {
            self.point_query = Some(DrawingPointQuery::new(command_id, TracePointerJob::new_query(&document, pointer.world, tolerance, true), false, selection_merge_mode(pointer.shift, pointer.ctrl, pointer.meta).into(), false));
            return Ok(None);
        }
        let context=self.tool.context();let utility=context.utility.clone();let start=context.start;
        let identity=if self.tool.matches("shaping") && ((pointer.world[0]-start[0]).abs()>=1.0||(pointer.world[1]-start[1]).abs()>=1.0){let operation=base.operation.as_ref().ok_or_else(||Fault::from("Drawing shape lacks original operation admission"))?;let x=start[0].min(pointer.world[0]);let y=start[1].min(pointer.world[1]);let width=(pointer.world[0]-start[0]).abs();let height=(pointer.world[1]-start[1]).abs();let geometry=match utility.as_str(){"shapeLine"=>[start[0],start[1],pointer.world[0],pointer.world[1]],"shapeEllipse"=>[x+width/2.0,y+height/2.0,width/2.0,height/2.0],_=>[x,y,width,height]};Some(self.with_identity_control(|control|semio_s_artifact_draw_drawing_io::text::identity::creation::shape_identity(&utility,geometry,base.document.layers.len(),operation,control).map_err(|error|Fault::from(error.to_string())))?)}else{None};
        let mut emit = self.step(canvas_tool::Event::PointerUp(DrawingRelease { pointer, threshold, base,identity }))?;
        if let Some(leaf @ DrawingMutation::DragPathPoints(_)) = emit.artifact_mutations.first() {
            let rebound = drawing_rebound_points(&document, leaf)?;
            emit.effects.push(point_selection_effect(&rebound));
        }
        Ok(Some(emit))
    }

    /// ✊️ Resolves what the pending select press grabbed, in bounded steps, and hands it to the tool: path points under
    /// `editNodes`, else the hit layer or the whole selection (a hit transform handle keeps the selection). An empty node
    /// press starts a node marquee instead. `Ok(false)` while more document remains to walk.
    pub fn prepare_grab(&mut self, document: &DrawingSnapshot, ids: &[String], point_ids:&[String]) -> Result<bool,Fault> {
        let Some(query)=self.point_query.as_mut() else { return Ok(true); };
        if query.cursor.node_area && !query.grab_prepared {
            let hits=query.cursor.hits.iter().cloned().collect::<Vec<_>>();
            let selected=points::merge_point_selection(&query.area_current,&hits,query.point_pick_mode);
            if selected.len()>DRAWING_QUERY_HIT_CAPACITY || selected.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Point selection exceeds gesture capacity"));}
            query.node_selection=Some(selected);query.grab_prepared=true;return Ok(true);
        }
        if query.grab_prepared || query.drag_start.is_none() { return Ok(true); }
        if query.cursor.node_editing {
            query.grab_prepared=true;
            query.preserve_selection=true;
            if point_ids.len()>DRAWING_QUERY_HIT_CAPACITY || point_ids.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Point selection exceeds gesture capacity"));}
            let current=point_ids.iter().filter(|id|points::parse_point_id(id).is_some_and(|point|ids.iter().any(|id|id==point.layer_id))).cloned().collect::<Vec<_>>();
            let hit=if let Some((path,index,point,_))=query.cursor.node_hit {
                let Some(DrawingLayerNode::Path(layer))=drawing_layer_at_path(&document.layers,&path) else {return Err(Fault::from("Missing selected path"));};
                let geometry=points::geometry_id(&layer.segments).ok_or_else(||Fault::from("Invalid path geometry"))?;
                Some(points::point_id(&layer.base.id,&geometry,index,point).ok_or_else(||Fault::from("Invalid selected point"))?)
            } else {None};
            let selected=points::pick_point_selection(&current,hit.as_deref(),query.point_pick_mode);
            if selected.len()>DRAWING_QUERY_HIT_CAPACITY || selected.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Point selection exceeds gesture capacity"));}
            let start=query.drag_start.expect("a grab preparation starts at its press");
            if hit.is_none() {
                self.node_marquee=Some(NodeMarquee {layer_ids:ids.to_vec(),current,mode:query.point_pick_mode});
                self.step(canvas_tool::Event::NodeMarquee(DrawingPointer { utility:"selectMarquee".into(),world:start,shift:false,alt:false,ctrl:false,meta:false }))?;
                return Ok(true);
            }
            query.node_selection=Some(selected.clone());
            if hit.as_ref().is_some_and(|hit|selected.contains(hit)) {
                let targets=crate::drawing_point_targets(document,ids,&selected)?;
                self.step(canvas_tool::Event::Grab(DrawingGrab::Points { targets }))?;
            }
            return Ok(true);
        }
        if query.grab_preparation.is_none() {
            if ids.len()>DRAWING_QUERY_HIT_CAPACITY || ids.iter().map(String::len).sum::<usize>()>DRAWING_QUERY_TARGET_BYTES {return Err(Fault::from("Selection exceeds gesture capacity"));}
            let start=query.drag_start.expect("a grab preparation starts at its press");
            let handle=query.cursor.selection_bounds.and_then(|bounds|semio_s_artifact_draw_drawing_geometry::handles::hit_handle(bounds,start,self.window_config.viewport.zoom).map(|handle|(handle,bounds)));
            let mut selected=if handle.is_some() {
                query.preserve_selection=true;
                ids.to_vec()
            } else {
                if query.constrained {query.grab_prepared=true;query.preserve_selection=true;return Ok(true);}
                let Some(candidate)=query.cursor.best.as_ref() else {query.grab_prepared=true;return Ok(true);};
                let mut prefix=candidate.path;
                while prefix.len>0 {
                    if drawing_layer_at_path(&document.layers,&prefix).is_some_and(|layer|ids.iter().any(|id|trace_layer_base(layer).id.eq_str(id))) {query.preserve_selection=true;break;}
                    prefix.len-=1;
                }
                if query.preserve_selection {ids.to_vec()} else {vec![candidate.layer_id.clone()]}
            };
            selected.sort();selected.dedup();
            query.grab_preparation=Some(LayerGrabPreparation::new(document,selected,handle));
            return Ok(false);
        }
        let preparation=query.grab_preparation.as_mut().expect("a started grab preparation remains retained");
        if !preparation.advance(document)? { return Ok(false); }
        let grab=query.grab_preparation.take().expect("a finished grab preparation remains retained").grab();
        query.grab_prepared=true;
        self.step(canvas_tool::Event::Grab(grab))?;
        Ok(true)
    }

    pub fn advance_lasso_move(&mut self, payload: &crate::canvas_pointer_move::CanvasPointerMove) -> Result<Option<Emit<DrawingMutation,NoConfigMutation>>,Fault> {
        let [x,y]=payload.samples.get(self.move_sample_cursor).copied().unwrap_or([payload.x,payload.y]);
        let (x,y)=canvas_point_to_world(&self.window_config.viewport,x,y,payload.width,payload.height);
        let emit=self.sample([x,y],payload.shift,payload.alt)?;
        self.move_sample_cursor+=1;
        if self.move_sample_cursor<payload.samples.len() { return Ok(None); }
        self.move_sample_cursor=0;
        Ok(Some(emit))
    }

    pub(super) fn retain_trace_pointer(&mut self, job: TracePointerJob) -> Result<(), TracePointerJob> {
        if self.trace_pointer.is_some() {
            return Err(job);
        }
        self.trace_pointer = Some(job);
        Ok(())
    }

    pub(super) fn take_trace_pointer(&mut self, app_instance_id: u32, document_id: &(impl Utf8Text+?Sized), operation_id: u64, generation: u64, base_revision: &str) -> Option<TracePointerJob> {
        let job = self.trace_pointer.as_ref()?;
        if job.app_instance_id != app_instance_id || !job.document_id.eq_text(document_id) || job.operation_id != operation_id || job.generation != generation || job.base_revision != base_revision {
            return None;
        }
        self.trace_pointer.take()
    }

    pub fn cancel_trace_pointer(&mut self, app_instance_id: u32, document_id: &(impl Utf8Text+?Sized), generation: u64) -> bool {
        let matches = self.trace_pointer.as_ref().is_some_and(|job| job.app_instance_id == app_instance_id && job.document_id.eq_text(document_id) && generation != 0 && job.generation == generation);
        if matches {
            self.trace_pointer = None;
        }
        matches
    }

    /// 👁️ The in-flight gesture as the Canvas window paints it, derived from the tool's context only.
    pub fn preview(&self) -> DrawingGesturePreview {
        let phase = if self.tool.matches("pressing") || self.tool.matches("dragging") {
            DrawingGesturePreviewPhase::Move
        } else if self.tool.matches("marqueeing") {
            DrawingGesturePreviewPhase::Marquee
        } else if self.tool.matches("shaping") {
            DrawingGesturePreviewPhase::Shape
        } else if self.tool.matches("drafting") {
            DrawingGesturePreviewPhase::Draft
        } else {
            DrawingGesturePreviewPhase::Idle
        };
        let context = self.tool.context();
        let dragging = self.tool.matches("dragging") && context.active;
        let (transformation, node_translation) = match context.grab.as_ref().filter(|_| dragging) {
            Some(DrawingGrab::Layers { targets, handle }) => {
                let matrix = match handle {
                    None => Some([1.0, 0.0, 0.0, 1.0, context.cursor[0] - context.start[0], context.cursor[1] - context.start[1]]),
                    Some((handle, bounds)) => handle_motion(*handle, *bounds, context.start, context.cursor, context.constrained, context.centered).map(HandleMotion::matrix),
                };
                (matrix.map(|matrix| (targets.iter().map(PagedUtf8::to_string_owner).collect(), matrix)), None)
            }
            Some(grab @ DrawingGrab::Points { targets }) => match drawing_grab_leaf(grab, context.start, context.cursor, context.constrained, context.centered) {
                Some(DrawingMutation::DragPathPoints(drag)) => (None, Some(DrawingNodeTranslation { targets: targets.clone(), delta: [drag.dx, drag.dy] })),
                _ => (None, None),
            },
            None => (None, None),
        };
        DrawingGesturePreview { sequence: self.preview_seq, phase, context: context.clone(), transformation, node_translation }
    }
}
//#endregion 🔖️DrawingSession
