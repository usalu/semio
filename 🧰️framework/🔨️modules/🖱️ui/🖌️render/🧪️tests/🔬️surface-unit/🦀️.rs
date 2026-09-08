
use super::*;
use crate::scene::{DrawBatch, FinishParams, PipelineKind, Scene, ScissorRect};
use crate::schedule::InvalidationReason;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn test_placement(id: &str, bounds: Bounds, clip: ClipId, z_index: i32) -> SurfacePlacement {
    SurfacePlacement { id: SurfaceId(ui_contract::UiText::try_from_str(id).expect("bounded fixture surface")), bounds, clip, transform: Transform2D::IDENTITY, z_index }
}

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn test_caps() -> DeviceCapabilities {
    DeviceCapabilities {
        max_texture_dimension: 4096,
        max_bind_groups: 4,
        supports_msaa: false,
        supports_timestamp_queries: false,
        supports_storage_buffers: false,
        preferred_surface_format: crate::backend::SurfaceFormat::Rgba8UnormSrgb,
        memory_class: crate::backend::MemoryClass::Standard,
        gpu_tier: crate::backend::GpuTier::Integrated,
    }
}

//#region Placement

#[test]
fn clip_id_root_is_a_distinct_stable_value() {
    assert_eq!(ClipId::ROOT, ClipId(0));
}

#[test]
fn transform2d_identity_inverts_to_itself() {
    assert_eq!(Transform2D::IDENTITY.invert(), Some(Transform2D::IDENTITY));
}

#[test]
fn transform2d_apply_point_translates() {
    assert_eq!(Transform2D::translation(5.0, -3.0).apply_point([1.0, 1.0]), [6.0, -2.0]);
}

#[test]
fn route_pointer_event_returns_none_outside_bounds_and_local_coordinates_inside() {
    let placement = test_placement("s", Bounds::new(100.0, 50.0, 200.0, 100.0), ClipId::ROOT, 0);
    assert_eq!(route_pointer_event(&placement, [10.0, 10.0]), None, "a point outside the surface's bounds must not route");
    assert_eq!(route_pointer_event(&placement, [150.0, 80.0]), Some([50.0, 30.0]), "a point inside must convert to surface-local coordinates");
}

#[test]
fn route_pointer_event_applies_the_inverse_transform_after_the_bounds_offset() {
    let placement = test_placement("s", Bounds::new(0.0, 0.0, 100.0, 100.0), ClipId::ROOT, 0);
    let placement = SurfacePlacement { transform: Transform2D::translation(10.0, 20.0), ..placement };
    let local = route_pointer_event(&placement, [50.0, 50.0]).expect("inside bounds");
    assert_eq!(local, [40.0, 30.0]);
}

//#endregion Placement

//#region SurfaceLifecycle

#[derive(Default)]
struct RevisionSnapshot {
    revision: u64,
}

#[derive(Default)]
struct RevisionSurface {
    snapshot_revision: u64,
    rendered_revision: Option<u64>,
}

impl Surface for RevisionSurface {
    type Snapshot = RevisionSnapshot;
    type Intent = ();

    fn update_snapshot(&mut self, snapshot: Rc<Self::Snapshot>) {
        self.snapshot_revision = snapshot.revision;
    }

    fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
        let dirty = self.rendered_revision != Some(self.snapshot_revision);
        SurfacePrepare { dirty, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
    }

    fn render(&mut self, _cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
        self.rendered_revision = Some(self.snapshot_revision);
        Ok(())
    }

    fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
        Vec::new()
    }

    fn next_deadline(&self) -> Option<Deadline> {
        None
    }
}

#[test]
fn a_snapshot_revision_that_has_not_changed_does_not_mark_the_surface_dirty() {
    let placement = test_placement("s", Bounds::default(), ClipId::ROOT, 0);
    let caps = test_caps();
    let mut surface = RevisionSurface::default();

    surface.update_snapshot(Rc::new(RevisionSnapshot { revision: 1 }));
    assert!(surface.prepare(&placement, &caps).dirty, "a never-rendered surface must report dirty");

    let mut scene = SceneBuilder::default();
    let mut resources = ResourceRegistry::default();
    render_placed_surface(&mut surface, &placement, &mut scene, &mut resources, 0.0).expect("render");
    assert!(!surface.prepare(&placement, &caps).dirty, "an unchanged revision right after a render must not be dirty");

    surface.update_snapshot(Rc::new(RevisionSnapshot { revision: 1 }));
    assert!(!surface.prepare(&placement, &caps).dirty, "re-applying the same revision must not mark dirty");

    surface.update_snapshot(Rc::new(RevisionSnapshot { revision: 2 }));
    assert!(surface.prepare(&placement, &caps).dirty, "a changed revision must mark dirty again");
}

#[derive(Default)]
struct DeadlineSurface {
    deadline: Option<Deadline>,
}

impl Surface for DeadlineSurface {
    type Snapshot = ();
    type Intent = ();

    fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

    fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
        SurfacePrepare { dirty: false, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
    }

    fn render(&mut self, _cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
        Ok(())
    }

    fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
        Vec::new()
    }

    fn next_deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

#[test]
fn an_animating_surfaces_next_deadline_reaches_the_scheduler_while_a_still_one_yields_none() {
    let animating = DeadlineSurface { deadline: Some(Deadline { due: 1.5, reason: InvalidationReason::ANIMATION }) };
    let still = DeadlineSurface::default();
    assert_eq!(animating.next_deadline(), Some(Deadline { due: 1.5, reason: InvalidationReason::ANIMATION }));
    assert_eq!(still.next_deadline(), None);
}

//#endregion SurfaceLifecycle

//#region Input

#[derive(Default)]
struct RecordingInputSurface {
    received: Vec<[f32; 2]>,
}

impl Surface for RecordingInputSurface {
    type Snapshot = ();
    type Intent = [f32; 2];

    fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

    fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
        SurfacePrepare { dirty: false, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
    }

    fn render(&mut self, _cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
        Ok(())
    }

    fn handle_input(&mut self, event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
        match *event {
            SurfaceInput::PointerMoved { local } => {
                self.received.push(local);
                vec![local]
            }
            _ => Vec::new(),
        }
    }

    fn next_deadline(&self) -> Option<Deadline> {
        None
    }
}

#[test]
fn dispatch_pointer_moved_reaches_the_surface_only_when_inside_bounds_and_in_local_coordinates() {
    let placement = test_placement("s", Bounds::new(0.0, 0.0, 100.0, 100.0), ClipId::ROOT, 0);
    let mut surface = RecordingInputSurface::default();

    let outside = dispatch_pointer_moved(&mut surface, &placement, [500.0, 500.0]);
    assert!(outside.is_empty(), "an out-of-bounds pointer event must not reach the surface");
    assert!(surface.received.is_empty());

    let inside = dispatch_pointer_moved(&mut surface, &placement, [10.0, 20.0]);
    assert_eq!(inside, vec![[10.0, 20.0]]);
    assert_eq!(surface.received, vec![[10.0, 20.0]]);
}

//#endregion Input

//#region PlacementAndZOrder

struct PaintingSurface {
    color: [f32; 4],
    seen_clip: Option<ClipId>,
}

impl PaintingSurface {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn with_color(color: [f32; 4]) -> Self {
        Self { color, seen_clip: None }
    }
}

impl Surface for PaintingSurface {
    type Snapshot = ();
    type Intent = ();

    fn update_snapshot(&mut self, _snapshot: Rc<Self::Snapshot>) {}

    fn prepare(&mut self, _placement: &SurfacePlacement, _caps: &DeviceCapabilities) -> SurfacePrepare {
        SurfacePrepare { dirty: true, target: SurfaceRenderTarget::Inline, needs: SurfaceResourceNeeds::default() }
    }

    fn render(&mut self, cx: &mut SurfaceRenderCx<'_>) -> Result<(), SurfaceError> {
        self.seen_clip = Some(cx.placement.clip);
        let Bounds { x, y, w, h } = cx.placement.bounds;
        cx.scene.push_solid([x, y, w, h], self.color);
        Ok(())
    }

    fn handle_input(&mut self, _event: &SurfaceInput, _placement: &SurfacePlacement) -> Vec<Self::Intent> {
        Vec::new()
    }

    fn next_deadline(&self) -> Option<Deadline> {
        None
    }
}

#[test]
fn render_placed_surface_scissors_by_bounds_preserves_call_order_for_z_and_passes_clip_through() {
    let mut scene = SceneBuilder::default();
    let mut resources = ResourceRegistry::default();
    let back = test_placement("back", Bounds::new(0.0, 0.0, 50.0, 50.0), ClipId::ROOT, 0);
    let front = test_placement("front", Bounds::new(10.0, 10.0, 20.0, 20.0), ClipId(7), 1);
    let mut back_surface = PaintingSurface::with_color([1.0, 0.0, 0.0, 1.0]);
    let mut front_surface = PaintingSurface::with_color([0.0, 1.0, 0.0, 1.0]);

    render_placed_surface(&mut back_surface, &back, &mut scene, &mut resources, 0.0).expect("back renders");
    render_placed_surface(&mut front_surface, &front, &mut scene, &mut resources, 0.0).expect("front renders");

    assert_eq!(back_surface.seen_clip, Some(ClipId::ROOT), "the placement's clip must reach the render call unchanged");
    assert_eq!(front_surface.seen_clip, Some(ClipId(7)), "each placement's own clip must reach its own render call, not the previous one's");

    let packet = Scene::finish(scene, FinishParams { viewport: [100.0, 100.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
    let quad_batches: Vec<&DrawBatch> = packet.batches.iter().filter(|batch| batch.pipeline == PipelineKind::UiQuad).collect();
    assert_eq!(quad_batches.len(), 2, "two differently-scissored surfaces must not merge into one batch");
    assert_eq!(quad_batches[0].layer_state.scissor, Some(ScissorRect::from_rect(back.bounds)), "the first-rendered (lower z) placement's bounds must be the first batch's scissor");
    assert_eq!(quad_batches[1].layer_state.scissor, Some(ScissorRect::from_rect(front.bounds)), "the second-rendered (higher z) placement's bounds must be the second batch's scissor, proving call order encodes z-order");
}

#[test]
fn surface_resource_needs_is_empty_reports_correctly() {
    assert!(SurfaceResourceNeeds::default().is_empty());
    assert!(!SurfaceResourceNeeds { textures: 1, meshes: 0 }.is_empty());
}

//#endregion PlacementAndZOrder

//#region Registry

#[test]
fn registering_a_kind_makes_create_resolve_it_without_an_error() {
    let mut registry = SurfaceRegistry::new();
    registry.register::<DeadlineSurface>(SurfaceKind::World3d);
    assert!(registry.is_registered(SurfaceKind::World3d));

    let (surface, error) = registry.create(SurfaceKind::World3d);
    assert!(error.is_none());
    assert_eq!(surface.kind(), SurfaceKind::World3d);
}

#[test]
fn an_unregistered_kind_produces_a_visible_placeholder_plus_an_error_rather_than_silence() {
    let registry = SurfaceRegistry::new();
    let (mut surface, creation_error) = registry.create(SurfaceKind::World3d);
    assert!(matches!(creation_error, Some(SurfaceError::Unregistered(SurfaceKind::World3d))), "an unresolved kind must report a clear error, not silently succeed");

    let placement = test_placement("s", Bounds::new(0.0, 0.0, 10.0, 10.0), ClipId::ROOT, 0);
    let mut scene = SceneBuilder::default();
    let mut resources = ResourceRegistry::default();
    let render_result = surface.render_placed(&placement, &mut scene, &mut resources, 0.0);
    assert!(matches!(render_result, Err(SurfaceError::Unregistered(SurfaceKind::World3d))), "rendering the placeholder must keep reporting the same error");

    let packet = Scene::finish(scene, FinishParams { viewport: [50.0, 50.0], dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }).expect("finish");
    assert!(!packet.batches.is_empty(), "an unregistered surface must still paint a visible placeholder, never a silent blank");
}

#[test]
fn handle_input_through_any_surface_erases_and_boxes_the_concrete_intent_type() {
    let mut registry = SurfaceRegistry::new();
    registry.register::<RecordingInputSurface>(SurfaceKind::NodeGraph);
    let (mut surface, error) = registry.create(SurfaceKind::NodeGraph);
    assert!(error.is_none());

    let placement = test_placement("s", Bounds::new(0.0, 0.0, 100.0, 100.0), ClipId::ROOT, 0);
    let intents = surface.handle_input(&SurfaceInput::PointerMoved { local: [4.0, 5.0] }, &placement);
    assert_eq!(intents.len(), 1);
    let intent = intents.into_iter().next().expect("one intent").downcast::<[f32; 2]>().expect("RecordingInputSurface::Intent is [f32; 2]");
    assert_eq!(*intent, [4.0, 5.0]);
}

//#endregion Registry
