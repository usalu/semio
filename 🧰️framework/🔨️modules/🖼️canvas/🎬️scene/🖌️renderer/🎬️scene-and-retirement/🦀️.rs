//! 🎬️ Scene and retirement: the recorded scene, the opaque scene retirement registry and its entry points, the canvas scene retirement custody and the parsed SVG document. See [`super`].

use super::*;

#[derive(Clone, Default)]
pub struct Scene(pub(crate) Vec<SceneCommand>);

pub(super) const OPAQUE_SCENE_RETIREMENT_CAPACITY: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpaqueSceneRetirementToken {
    slot: u16,
    generation: u64,
}

/// 🎬️ One exact scene-retirement turn with current credit and physical release split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpaqueSceneRetirementStep {
    Blocked,
    Pending { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Complete { released_items: usize, credited_bytes: usize, released_bytes: usize },
    Fault,
}

pub(super) struct OpaqueSceneRetirementSlot {
    generation: u64,
    occupied: bool,
    pub(super) credited_bytes: usize,
    pub(super) scene: Option<ManuallyDrop<Scene>>,
    pub(super) command: Option<ManuallyDrop<SceneCommand>>,
    command_backing_bytes: usize,
    command_credited_bytes: usize,
}

pub(super) struct OpaqueSceneRetirementRegistry {
    slots: Box<[OpaqueSceneRetirementSlot; OPAQUE_SCENE_RETIREMENT_CAPACITY]>,
    pub(super) faulted: bool,
}

impl Default for OpaqueSceneRetirementRegistry {
    fn default() -> Self {
        Self { slots: semio_framework_async_slots::boxed_fixed_slots(|| OpaqueSceneRetirementSlot { generation: 0, occupied: false, credited_bytes: 0, scene: None, command: None, command_backing_bytes: 0, command_credited_bytes: 0 }), faulted: false }
    }
}

impl OpaqueSceneRetirementRegistry {
    pub(super) fn reserve(&mut self) -> Option<OpaqueSceneRetirementToken> {
        let Some(index) = self.slots.iter().position(|slot| !slot.occupied) else {
            self.faulted = true;
            return None;
        };
        let slot = &mut self.slots[index];
        assert!(slot.scene.is_none(), "released opaque scene slot owns no scene");
        assert!(slot.command.is_none(), "released opaque scene slot owns no command");
        assert_eq!(slot.credited_bytes, 0, "released opaque scene slot owns no byte credit");
        assert_eq!(slot.command_backing_bytes, 0, "released opaque scene slot owns no command backing");
        assert_eq!(slot.command_credited_bytes, 0, "released opaque scene slot owns no command byte credit");
        slot.generation = slot.generation.wrapping_add(1).max(1);
        slot.occupied = true;
        Some(OpaqueSceneRetirementToken { slot: index as u16, generation: slot.generation })
    }

    pub(super) fn token_is_current(&self, token: OpaqueSceneRetirementToken) -> bool {
        self.slots.get(usize::from(token.slot)).is_some_and(|slot| slot.occupied && slot.generation == token.generation)
    }

    pub(super) fn publish(&mut self, token: OpaqueSceneRetirementToken, scene: Scene) {
        assert!(self.token_is_current(token), "opaque scene retirement token remains current before ownership transfer");
        let slot = &mut self.slots[usize::from(token.slot)];
        assert!(slot.scene.is_none(), "opaque scene retirement token remains unpublished");
        slot.scene = Some(ManuallyDrop::new(scene));
    }

    pub(super) fn advance(&mut self, token: OpaqueSceneRetirementToken, maximum_items: usize, maximum_bytes: usize) -> OpaqueSceneRetirementStep {
        if maximum_items == 0 || maximum_bytes == 0 {
            return OpaqueSceneRetirementStep::Blocked;
        }
        if !self.token_is_current(token) {
            self.faulted = true;
            return OpaqueSceneRetirementStep::Fault;
        }
        let slot = &mut self.slots[usize::from(token.slot)];
        let Some(scene) = slot.scene.as_mut() else {
            return OpaqueSceneRetirementStep::Blocked;
        };
        if let Some(command) = slot.command.as_mut() {
            let remaining_bytes = slot.command_backing_bytes.saturating_sub(slot.command_credited_bytes);
            let credited_bytes = maximum_bytes.min(remaining_bytes);
            slot.command_credited_bytes = slot.command_credited_bytes.saturating_add(credited_bytes);
            if slot.command_credited_bytes != slot.command_backing_bytes {
                return OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
            }
            if !command.retirement_step() {
                return OpaqueSceneRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes: 0 };
            }
            let command = slot.command.take().expect("terminal opaque scene command remains retained");
            drop(ManuallyDrop::into_inner(command));
            let released_bytes = slot.command_backing_bytes;
            slot.command_backing_bytes = 0;
            slot.command_credited_bytes = 0;
            return OpaqueSceneRetirementStep::Pending { released_items: 1, credited_bytes, released_bytes };
        }
        if let Some(command) = scene.take_retirement_command() {
            slot.command_backing_bytes = command.retirement_backing_bytes();
            slot.command = Some(ManuallyDrop::new(command));
            return OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes: 0, released_bytes: 0 };
        }
        let released_bytes = scene.retirement_backing_bytes();
        let remaining_bytes = released_bytes.saturating_sub(slot.credited_bytes);
        let credited_bytes = maximum_bytes.min(remaining_bytes);
        slot.credited_bytes = slot.credited_bytes.saturating_add(credited_bytes);
        if slot.credited_bytes != released_bytes {
            return OpaqueSceneRetirementStep::Pending { released_items: 0, credited_bytes, released_bytes: 0 };
        }
        let scene = slot.scene.take().expect("terminal opaque scene owner remains retained");
        drop(ManuallyDrop::into_inner(scene));
        slot.occupied = false;
        slot.credited_bytes = 0;
        debug_assert!(slot.command.is_none());
        debug_assert_eq!(slot.command_backing_bytes, 0);
        debug_assert_eq!(slot.command_credited_bytes, 0);
        OpaqueSceneRetirementStep::Complete { released_items: 1, credited_bytes, released_bytes }
    }

    pub(super) fn active(&self) -> usize {
        self.slots.iter().filter(|slot| slot.occupied).count()
    }
}

fn opaque_scene_retirements() -> &'static Mutex<OpaqueSceneRetirementRegistry> {
    static REGISTRY: OnceLock<Mutex<OpaqueSceneRetirementRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(OpaqueSceneRetirementRegistry::default()))
}

pub fn reserve_opaque_scene_retirement() -> Option<OpaqueSceneRetirementToken> {
    let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    registry.reserve()
}

pub fn publish_opaque_scene_retirement(token: OpaqueSceneRetirementToken, scene: Scene) {
    let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    registry.publish(token, scene);
}

pub fn advance_opaque_scene_retirement(token: OpaqueSceneRetirementToken, maximum_items: usize, maximum_bytes: usize) -> OpaqueSceneRetirementStep {
    let mut registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    registry.advance(token, maximum_items, maximum_bytes)
}

pub fn opaque_scene_retirement_status() -> (usize, bool) {
    let registry = opaque_scene_retirements().lock().expect("worker opaque scene retirement registry");
    (registry.active(), registry.faulted)
}

/// ♻️ Retires exact first-party scene children and original command storage under supplied grants.
#[derive(Default)]
pub struct CanvasSceneRetirement{pub(super) active:Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>,active_bytes:usize,pub(super) image:Option<semio_framework_value::retirement::controlled::ControlledRetirement<RasterImage>>}

impl CanvasSceneRetirement{
    pub fn terminal_is_empty(&self,scene:&Scene)->bool{scene.0.is_empty()&&scene.0.capacity()==0&&self.active.is_none()&&self.image.is_none()}
    pub fn next_demands(&self,scene:&Scene,copy:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
        if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(RetirementDemand{release_bytes:self.active_bytes,depth:1,..Default::default()})}else{active.next_demand(copy)}}
        if let Some(image)=self.image.as_ref(){return image.selected_demand(copy)}
        let shape=match scene.0.last(){Some(SceneCommand::Fill{shape,..})=>Some(shape),Some(SceneCommand::Stroke{stroke,shape,..})=>{if stroke.dash_capacity()!=0{return Ok(crate::CanvasStrokeRetirement::birth_demand())}Some(shape)},Some(SceneCommand::PushLayer{clip,..}|SceneCommand::PushClipLayer{clip,..})=>Some(clip),Some(SceneCommand::DrawImage{..})=>return Ok(RetirementDemand{depth:1,..Default::default()}),
            #[cfg(not(all(target_arch="wasm32",target_env="p2")))]
            Some(SceneCommand::VelloFragment{..})=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"opaque Vello scene has no priced original retirement owner")),_=>None};
        if shape.is_some_and(|shape|matches!(shape,RecordedShape::BezPath(path)if path.retirement_backing_bytes()!=0)){return Ok(crate::CanvasPathRetirement::birth_demand())}
        Ok(RetirementDemand{copy_bytes:if scene.0.is_empty(){0}else{size_of::<SceneCommand>()},release_bytes:if scene.0.is_empty(){scene.retirement_backing_bytes()}else{0},depth:usize::from(!self.terminal_is_empty(scene)),..Default::default()})
    }
    pub fn close_step(&mut self,scene:&mut Scene,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneStep,semio_framework_value::ValueError>{
        use semio_framework_value::{ErasedSnapshotRetirement,RetainedCloneStep as Step,RetainedCloneProgress as Progress};
        let empty=Progress::default();if self.terminal_is_empty(scene){return Ok(Step::Complete(empty))}
        let demand=self.next_demands(scene,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(Step::Progress(empty))}
        if let Some(active)=self.active.as_mut(){if active.terminal_is_empty(){drop(self.active.take());let bytes=std::mem::take(&mut self.active_bytes);return Ok(Step::Progress(Progress{copied_items:1,released_bytes:bytes,..empty}))}return active.close_step(grant)}
        if let Some(image)=self.image.as_mut(){let step=image.step(grant)?;if image.terminal_is_empty(){self.image=None}return Ok(Step::Progress(step.progress()))}
        if let Some(SceneCommand::Stroke{stroke,..})=scene.0.last_mut(){if stroke.dash_capacity()!=0{let original=std::mem::replace(stroke,Stroke::new(0.0));let (owner,progress)=crate::CanvasStrokeRetirement::admit(original,grant).map_err(|(error,original)|{*stroke=original;error})?;self.active_bytes=size_of::<crate::CanvasStrokeRetirement>();self.active=Some(owner);return Ok(Step::Progress(progress))}}
        let shape=match scene.0.last_mut(){Some(SceneCommand::Fill{shape,..}|SceneCommand::Stroke{shape,..})=>Some(shape),Some(SceneCommand::PushLayer{clip,..}|SceneCommand::PushClipLayer{clip,..})=>Some(clip),_=>None};
        if let Some(RecordedShape::BezPath(path))=shape{if path.retirement_backing_bytes()!=0{let original=std::mem::take(path);let(owner,progress)=crate::CanvasPathRetirement::admit(original,grant).map_err(|(error,original)|{*path=original;error})?;self.active_bytes=size_of::<crate::CanvasPathRetirement>();self.active=Some(owner);return Ok(Step::Progress(progress))}}
        if matches!(scene.0.last(),Some(SceneCommand::DrawImage{..})){let Some(SceneCommand::DrawImage{image,..})=scene.0.pop()else{unreachable!()};self.image=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(image).unwrap_or_else(|_|panic!("first-party raster image retirement admitted")));return Ok(Step::Progress(Progress{copied_items:1,..empty}))}
        if scene.0.is_empty(){let bytes=scene.retirement_backing_bytes();drop(std::mem::take(&mut scene.0));return Ok(Step::Complete(Progress{copied_items:1,released_bytes:bytes,..empty}))}
        scene.0.pop();Ok(Step::Progress(Progress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))
    }
}

impl Scene {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /// 🎬️ Transfers one exact command into the retained retirement cursor.
    fn take_retirement_command(&mut self) -> Option<SceneCommand> {
        self.0.pop()
    }

    pub fn retirement_is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn retirement_backing_bytes(&self) -> usize {
        self.0.capacity().saturating_mul(size_of::<SceneCommand>())
    }
    /// 📦️ Bytes this scene keeps resident: its command list plus every command's own backing (path elements,
    /// stroke dashes, image pixels) — what a cache holding the scene accounts it at.
    pub fn retained_bytes(&self) -> usize {
        self.0.iter().fold(self.retirement_backing_bytes(), |total, command| total.saturating_add(command.retirement_backing_bytes()))
    }
    pub fn fill<'a>(&mut self, rule: FillRule, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::Fill { rule, transform, paint: paint.into(), brush_transform, shape: shape.into().into() });
    }
    pub fn stroke<'a>(&mut self, stroke: &Stroke, transform: Affine, paint: impl Into<Paint>, brush_transform: Option<Affine>, shape: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::Stroke { stroke: stroke.clone(), transform, paint: paint.into(), brush_transform, shape: shape.into().into() });
    }
    pub fn draw_image(&mut self, image: &RasterImage, transform: Affine) {
        self.0.push(SceneCommand::DrawImage { image: image.clone_data(), transform });
    }
    /// 🔗️ Flattens `other`'s commands directly into `self` (composing `transform` into each,
    /// see `SceneCommand::transformed`) rather than nesting a sub-scene, so every appended
    /// command stays independently transferable into the retained retirement cursor.
    pub fn append(&mut self, other: &Scene, transform: Option<Affine>) {
        self.append_range(other, 0, other.0.len(), transform);
    }

    pub fn command_len(&self) -> usize {
        self.0.len()
    }

    /// 🎬️ Copies `other[start..end]` into `self`, composing `transform` the same way [`Scene::append`] does.
    pub fn append_range(&mut self, other: &Scene, start: usize, end: usize, transform: Option<Affine>) {
        let commands = &other.0[start..end];
        match transform {
            Some(outer) => self.0.extend(commands.iter().cloned().map(|command| command.transformed(outer))),
            None => self.0.extend(commands.iter().cloned()),
        }
    }
    pub fn push_layer<'a>(&mut self, rule: FillRule, blend: BlendMode, alpha: f32, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::PushLayer { rule, blend, alpha, transform, clip: clip.into().into() });
    }
    pub fn pop_layer(&mut self) {
        self.0.push(SceneCommand::PopLayer);
    }
    pub fn push_clip_layer<'a>(&mut self, rule: FillRule, transform: Affine, clip: impl Into<ShapeRef<'a>>) {
        self.0.push(SceneCommand::PushClipLayer { rule, transform, clip: clip.into().into() });
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// 🔢️ A diagnostic hint (see `encoded_scene_hint`'s own docstring), not an exact-value
    /// contract — counts commands that draw or clip a path, same intent as the old
    /// `vello_encoding::Encoding::path_tags.len()` count but at first-party command
    /// granularity rather than per-verb granularity.
    pub fn path_count(&self) -> usize {
        self.0.iter().filter(|command| matches!(command, SceneCommand::Fill { .. } | SceneCommand::Stroke { .. } | SceneCommand::PushLayer { .. } | SceneCommand::PushClipLayer { .. })).count()
    }
    /// 🖥️ Host/browser-only: replays every recorded command into a freshly built
    /// `vello::Scene`, owned by the caller — the sole place this crate constructs a real
    /// `vello::Scene` from `Scene`'s first-party command list. `wasm32-wasip2` never reaches
    /// this method (nothing in `🎲️board` calls it — see module docstring).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub(crate) fn vello_scene(&self) -> backend::Scene {
        let mut built = backend::Scene::new();
        for command in &self.0 {
            command.replay_into(&mut built);
        }
        built
    }
}

/// 🏷️ Parsed SVG document for icon and label rasterization. Host/browser only — see
/// `vello_backend`'s `usvg` re-export docstring above; its only real callers are
/// `IconPaintCache::get_or_build`'s native arm and `#[cfg(test)]` code.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct SvgDocument(pub(crate) backend::usvg::Tree);

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl SvgDocument {
    pub(crate) fn from_tree(tree: backend::usvg::Tree) -> Self {
        Self(tree)
    }

    /// 🏷️ Appends the SVG tree into a scene. Builds a standalone real `vello::Scene`
    /// from the tree, then records it as one `SceneCommand::VelloFragment` — `Scene` itself
    /// stays a first-party command list even on this host/browser-only path.
    pub fn append_to_scene(&self, scene: &mut Scene) {
        let mut fragment = backend::Scene::new();
        vello_svg::append_tree(&mut fragment, &self.0);
        scene.0.push(SceneCommand::VelloFragment { scene: SharedArc::new(fragment), transform: Affine::IDENTITY });
    }
}

/// 🏷️ Appends a parsed SVG document into a scene.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub fn append_svg_document(scene: &mut Scene, doc: &SvgDocument) {
    doc.append_to_scene(scene);
}
