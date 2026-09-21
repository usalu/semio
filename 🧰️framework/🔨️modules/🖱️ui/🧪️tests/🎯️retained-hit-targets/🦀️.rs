//! 🎯️ LAW: a retained window body registers a pointer target for every interactive node it paints,
//! at the rect it painted it at, and the flat registry a host scans answers that node instead of the
//! window beneath it.
//!
//! The defect: on `?plugin=generation3d&mode=generate` the host's registry held 31 targets — the
//! shell chrome — and not one row of any retained window body, so
//! `InputState::hit_at(160.696, 138)`, a point derived from the dock plan plus the published
//! `mounted_layout` rect `[0, 72, 315.392, 24]` of the `Add Generation` row, answered the WINDOW's
//! `HitKind::ScrollRegion`. No row action dispatched, `NodeFlags::HOVERED` was never set, and the
//! wheel gate (`ShellState::wheel_propagates_to_scene_surface`) ended every wheel before it reached
//! a scene (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
//! `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6).
//!
//! This law drives the REAL pipeline — `Ui::apply_tree` → the real `MountedLayoutJob` →
//! `Ui::frame_into_step`, whose own `RetainedPaintPhase::Hits` mints the registry from the same
//! walk the paint phase painted from — and then replays each fixture point against a real
//! `InputState` loaded exactly the way the shell loads it: the window's own `ScrollRegion` first,
//! the body's entries after.
//!
//! Oracle: `🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json`; its TypeScript twin is
//! `📺️renderer/🧑‍🎨engine/🧪️tests/🎯️retained-hit-targets/🟦️.ts`.

use super::{HitKind, HitTarget, InputState};
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{SurfaceKind, UiComponentSceneNode, UiNode, UiPresence, UiStackNode, UiTextNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};
use crate::wgpu::draw::{DrawList, IconAtlas};
use crate::wgpu::engine::{Ui, UiFrameStep, UiLayoutStep};
use crate::wgpu::geometry::Rect;
use crate::wgpu::scene_slots::{SceneHost, ScenePaintCursor, ScenePaintStep, SceneSlot};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::Label;
use serde_json::Value;

fn law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎯️retained-hit-targets/🔣️.json")).expect("retained hit target fixture")
}

/// 🎬️ A scene host that paints one rect per slot — enough for the `Scenes` phase of a body whose
/// only child is an engine-surface canvas to complete instead of faulting `scenes-no-host`.
struct LawSceneHost;

impl SceneHost for LawSceneHost {
    fn paint_slot_step(&mut self, slot: &SceneSlot<'_>, cursor: &mut ScenePaintCursor, draw: &mut DrawList, _atlas: &mut FontAtlas, _icons: Option<&IconAtlas>) -> ScenePaintStep {
        match cursor.bind(slot.node) {
            Ok(true) => {}
            Ok(false) => return ScenePaintStep::Pending,
            Err(_) => return ScenePaintStep::Fault,
        }
        draw.push_rounded([slot.rect.x, slot.rect.y, slot.rect.w, slot.rect.h], crate::wgpu::theme::Theme::default().accent, 0.0);
        cursor.finish()
    }
}

fn number(value: &Value, key: &str) -> f32 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture number {key}")) as f32
}

fn body_rect(case: &Value) -> Rect {
    let body = &case["body"];
    Rect::new(number(body, "x"), number(body, "y"), number(body, "w"), number(body, "h"))
}

fn action_of(value: &Value) -> Option<ActionDescriptor> {
    let binding = value.as_object()?;
    Some(ActionDescriptor { controller_id: binding["controller"].as_str()?.to_string(), action: binding["action"].as_str()?.to_string(), args: None })
}

fn tree_item(value: &Value) -> UiTreeItemNode {
    UiTreeItemNode {
        window: None,
        granularity: None,
        id: value["id"].as_str().expect("item id").to_string(),
        label: Label::data(value["label"].as_str().unwrap_or_default()),
        description: None,
        icon_id: None,
        presence: UiPresence::default(),
        default_open: None,
        action: action_of(&value["action"]),
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

/// 🌳️ Builds the authored document tree the fixture declares. The shapes are the ones the retained
/// document reconcile mounts for a published `Component::Tree` / `Component::Surface` /
/// `Component::Container` record, so the arena this law lays out is the arena 6118 lays out.
fn ui_node(value: &Value) -> UiNode {
    match value["kind"].as_str().expect("node kind") {
        "tree" => UiNode::Tree(UiTreeNode {
            sections: value["sections"]
                .as_array()
                .expect("sections")
                .iter()
                .map(|section| UiTreeSectionNode {
                    window: None,
                    id: section["id"].as_str().expect("section id").to_string(),
                    label: section["label"].as_str().map(Label::data),
                    default_open: section["defaultOpen"].as_bool(),
                    presence: UiPresence::default(),
                    items: section["items"].as_array().expect("items").iter().map(tree_item).collect(),
                })
                .collect(),
            presence: UiPresence::default(),
            drop_action: None,
            menu: None,
            interaction_domain: None,
        }),
        "surface" => UiNode::ComponentScene(UiComponentSceneNode {
            host_id: value["surfaceId"].as_str().expect("surface id").to_string(),
            surface_id: value["surfaceId"].as_str().expect("surface id").to_string(),
            controller_id: value["controllerId"].as_str().unwrap_or_default().to_string(),
            component_kind: match value["surfaceKind"].as_str().expect("surface kind") {
                "world-3d" => SurfaceKind::World3d,
                "node-graph" => SurfaceKind::NodeGraph,
                "tiled-map" => SurfaceKind::TiledMap,
                "board-2d" => SurfaceKind::Board2d,
                "table" => SurfaceKind::Table,
                other => panic!("fixture surface kind {other}"),
            },
            pane_id: None,
            binding_id: None,
            presence: UiPresence::default(),
            canvas_2d: None,
            world_3d: None,
            node_graph: None,
            text_editor: None,
            table: None,
            paint_2d: None,
            virtual_file_system: None,
            tiled_map: None,
            board2d: None,
            icon_render: None,
            ink_canvas: None,
            graph_timeline: None,
            block_list: None,
            diff_view: None,
            event_feed: None,
            menu: None,
        }),
        "stack" => UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: value["id"].as_str().map(str::to_string),
            presence: UiPresence::default(),
            activate: action_of(&value["activate"]),
            drop_action: None,
            drop_overlay: None,
            children: value["children"].as_array().map(|children| children.iter().map(ui_node).collect()).unwrap_or_default(),
            menu: None,
        }),
        "text" => UiNode::Text(UiTextNode { value: Label::data(value["value"].as_str().unwrap_or_default()), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }),
        other => panic!("fixture node kind {other}"),
    }
}

fn kind_name(kind: HitKind) -> &'static str {
    match kind {
        HitKind::Button => "button",
        HitKind::Toggle => "toggle",
        HitKind::Input => "input",
        HitKind::Select => "select",
        HitKind::Slider => "slider",
        HitKind::NumberStepper => "numberStepper",
        HitKind::Ring => "ring",
        HitKind::IconSelect => "iconSelect",
        HitKind::TreeItem => "treeItem",
        HitKind::TreeDragHandle => "treeDragHandle",
        HitKind::TreeDropTarget => "treeDropTarget",
        HitKind::PanelTab => "panelTab",
        HitKind::NavbarItem => "navbarItem",
        HitKind::Window => "window",
        HitKind::World3d => "world3d",
        HitKind::ComponentScene => "componentScene",
        HitKind::PanelResize => "panelResize",
        HitKind::DockSplit => "dockSplit",
        HitKind::DockJoinCorner => "dockJoinCorner",
        HitKind::ScrollRegion => "scrollRegion",
        HitKind::ContextMenu => "contextMenu",
        HitKind::DropdownItem => "dropdownItem",
        HitKind::Generic => "generic",
    }
}

/// 🖱️ `ShellState::wheel_propagates_to_scene_surface`'s own predicate, restated here so this crate's
/// law can answer the fixture's `wheelPropagatesToScene` column without depending on the shell.
fn wheel_propagates(target: &HitTarget<ActionDescriptor>) -> bool {
    match target.kind {
        HitKind::World3d | HitKind::Window => true,
        HitKind::ScrollRegion => target.control_id.as_deref().is_some_and(|id| id.ends_with(".pane") || id.ends_with(".map")),
        _ => false,
    }
}

fn layout_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1))
}

/// 🧵️ Drives the real layout queue and then the real retained paint of one window into a caller-owned
/// draw list — the production entry, whose `Hits` phase is what this law is about.
fn drive(ui: &mut Ui, window_id: &str, body: Rect, atlas: &mut FontAtlas) {
    ui.set_viewport(window_id, body.w, body.h);
    let operation = semio_framework_job::allocate_operation_id();
    let cancel = semio_framework_job::CancelToken::root_now();
    let pool = layout_pool();
    let mut preview_sequence = 0;
    // 📐️ The production ladder, not the queue's verdict: `step_layouts` answers `Idle` for the QUEUE,
    // which may be empty while THIS window is still dirty — the exact trap
    // `UiDocumentFramePhase::Layout` re-arms with `request_layout`.
    'settle: for _ in 0..64 {
        for _ in 0..16_384 {
            let mut cx = semio_framework_job::StepContext::new(operation, semio_framework_job::Generation(0), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), || Some(0), &mut preview_sequence);
            if matches!(ui.step_layouts(&pool, atlas, &mut cx), UiLayoutStep::Idle) {
                break;
            }
        }
        if !ui.layout_is_dirty(window_id) {
            break 'settle;
        }
        ui.request_layout(window_id);
    }
    assert!(!ui.layout_is_dirty(window_id), "{window_id}: layout never settled");
    let mut draw = DrawList::default();
    let mut host = LawSceneHost;
    for _ in 0..131_072 {
        match ui.frame_into_step(window_id, body, atlas, None, Some(&mut host), &mut draw) {
            UiFrameStep::Pending => {}
            UiFrameStep::Ready => return,
            step => panic!("{window_id}: retained paint answered {step:?} (phase {:?})", ui.paint_frame_phase(window_id)),
        }
    }
    panic!("{window_id}: retained paint never completed, parked in {:?} with {} targets", ui.paint_frame_phase(window_id), ui.window_hit_targets(window_id).len());
}

/// 🎯️ The registry a host actually scans: the window's own `ScrollRegion` first (the chrome
/// registers it before the body paints), then every entry the body published.
fn load_registry(ui: &Ui, window_id: &str, body: Rect) -> InputState<ActionDescriptor> {
    let mut input = InputState::<ActionDescriptor>::default();
    input.register_hit(HitTarget { rect: body, event: None, control_id: Some(window_id.to_string()), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None });
    for registration in ui.window_hit_targets(window_id) {
        input.register_hit(registration.to_hit_target());
    }
    input.publish_hits();
    input
}

fn mount(case: &Value) -> (Ui, InputState<ActionDescriptor>, String, Rect) {
    let window_id = case["windowId"].as_str().expect("window id").to_string();
    let body = body_rect(case);
    let mut ui = Ui::new();
    let mut atlas = FontAtlas::builtin();
    ui.apply_tree(&window_id, &ui_node(&case["tree"]));
    drive(&mut ui, &window_id, body, &mut atlas);
    let input = load_registry(&ui, &window_id, body);
    (ui, input, window_id, body)
}

/// 🎯️ One replay over the whole oracle: every case's authored document is mounted, laid out and
/// painted by the REAL pipeline, its registry is compared entry by entry against the fixture, every
/// fixture point is resolved on that registry, and the ordering rule is checked on the same load.
///
/// ⚖️ All of it in ONE test on purpose: `Ui`'s layout surface slots are a FIXED process-wide
/// aggregate, so a law that mounts a fresh `Ui` per `#[test]` starves itself once enough surfaces
/// have been admitted — measured here as `retained paint never completed, parked in None`.
#[test]
fn every_fixture_case_registers_resolves_and_orders_on_the_live_registry() {
    let law = law();
    for case in law["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("case name");
        let (ui, input, window_id, _body) = mount(case);
        for registration in ui.window_hit_targets(&window_id) {
            let authored = &ui.tree(&window_id).unwrap().node(registration.node).unwrap().spec.0;
            let expected_scene = match authored {
                UiNode::ComponentScene(scene) => Some(super::RetainedSceneHit { surface_id: scene.surface_id.clone(), kind: scene.component_kind }),
                _ => None,
            };
            assert_eq!(registration.scene, expected_scene, "{name}: scene routing provenance comes from the mounted component");
        }
        let published: Vec<(String, &'static str, Option<String>, [f32; 4])> = ui
            .window_hit_targets(&window_id)
            .iter()
            .map(|registration| (registration.control_id.clone(), kind_name(registration.kind), registration.action.as_ref().map(|action| action.action.clone()), [registration.rect.x, registration.rect.y, registration.rect.w, registration.rect.h]))
            .collect();
        let expected = case["expected"].as_array().expect("expected");
        assert_eq!(published.len(), expected.len(), "{name}: registry length, got {published:?}");
        for (index, want) in expected.iter().enumerate() {
            let (control_id, kind, action, rect) = &published[index];
            assert_eq!(control_id.as_str(), want["controlId"].as_str().expect("controlId"), "{name}[{index}]: control id");
            assert_eq!(*kind, want["kind"].as_str().expect("kind"), "{name}[{index}]: hit kind");
            assert_eq!(action.as_deref(), want["action"].as_str(), "{name}[{index}]: dispatched action");
            let want_rect = want["rect"].as_array().expect("rect");
            for (axis, label) in [(0usize, "x"), (1, "y"), (2, "w"), (3, "h")] {
                let want_value = want_rect[axis].as_f64().expect("rect scalar") as f32;
                assert!((rect[axis] - want_value).abs() < 0.01, "{name}[{index}] {control_id}: rect.{label} got {} want {want_value}", rect[axis]);
            }
        }

        for probe in case["probes"].as_array().expect("probes") {
            let x = number(probe, "x");
            let y = number(probe, "y");
            let hit = input.hit_at(x, y).unwrap_or_else(|| panic!("{name}: ({x}, {y}) resolved nothing"));
            assert_eq!(hit.control_id.as_deref(), probe["controlId"].as_str(), "{name}: ({x}, {y}) control id");
            assert_eq!(kind_name(hit.kind), probe["kind"].as_str().expect("kind"), "{name}: ({x}, {y}) hit kind");
            assert_eq!(hit.event.as_ref().map(|action| action.action.as_str()), probe["action"].as_str(), "{name}: ({x}, {y}) dispatched action");
            if let Some(want) = probe["wheelPropagatesToScene"].as_bool() {
                assert_eq!(wheel_propagates(hit), want, "{name}: ({x}, {y}) wheel propagation");
            }
            println!("[DEBUG] retained-hit-targets probe {name} ({x}, {y}) -> {:?}", hit.control_id);
        }

        // 🔢️ The ordering rule: the chrome registers the window's own region before the body paints,
        // `hit_at` scans in reverse, so a pointer at ANY body entry's own centre resolves that entry
        // and never the window under it.
        assert_eq!(input.hits().first().and_then(|target| target.control_id.as_deref()), Some(window_id.as_str()), "{name}: the window's own region must be registered first");
        for (control_id, kind, _, rect) in &published {
            let hit = input.hit_at(rect[0] + rect[2] * 0.5, rect[1] + rect[3] * 0.5).unwrap_or_else(|| panic!("{name}: {control_id} centre resolved nothing"));
            assert_eq!(hit.control_id.as_deref(), Some(control_id.as_str()), "{name}: {control_id} was outranked at its own centre");
            assert_eq!(kind_name(hit.kind), *kind, "{name}: {control_id} resolved another entry's kind at its own centre");
        }
        println!("[DEBUG] retained-hit-targets case {name}: {} entries pinned, {} points replayed", published.len(), case["probes"].as_array().map_or(0, Vec::len));
    }
}

/// 🩸️ The defect itself, refused: the exact point `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md`
/// §6 measured answering the window's own `ScrollRegion` must resolve the row and carry the row's
/// own action.
#[test]
fn the_defect_point_no_longer_answers_the_window() {
    let law = law();
    let case = law["cases"].as_array().expect("cases").iter().find(|entry| entry["name"] == "generate-mode-generations-window").expect("defect case");
    let (_ui, input, window_id, _body) = mount(case);
    let hit = input.hit_at(160.696, 138.0).expect("the derived row point must resolve");
    assert_ne!(hit.control_id.as_deref(), Some(window_id.as_str()), "the window's own ScrollRegion answered the row point again");
    assert_eq!(hit.kind, HitKind::TreeItem);
    assert_eq!(hit.event.as_ref().map(|action| action.action.as_str()), Some("addGeneration"));
    println!("[DEBUG] retained-hit-targets: (160.696, 138) -> {:?} / addGeneration", hit.control_id);
}

/// 🎯️ The oracle's own registry for one case, loaded straight from its declared entries instead of
/// from a live `Ui`. The buffering law is about WHEN an entry is resolvable, not about how it is
/// minted — the replay above owns that, against the real pipeline — and `Ui`'s layout surface slots
/// are a fixed process-wide aggregate, so a third live mount in this binary would starve itself.
fn fixture_registry(case: &Value) -> Vec<HitTarget<ActionDescriptor>> {
    let body = body_rect(case);
    let window_id = case["windowId"].as_str().expect("window id").to_string();
    let mut targets = vec![HitTarget { rect: body, event: None, control_id: Some(window_id), kind: HitKind::ScrollRegion, drag_axis: None, drag_data: None }];
    for entry in case["expected"].as_array().expect("expected") {
        let rect = entry["rect"].as_array().expect("rect");
        let scalar = |index: usize| rect[index].as_f64().expect("rect scalar") as f32;
        targets.push(HitTarget {
            rect: Rect::new(scalar(0), scalar(1), scalar(2), scalar(3)),
            event: entry["action"].as_str().map(|action| ActionDescriptor { controller_id: String::new(), action: action.to_string(), args: None }),
            control_id: entry["controlId"].as_str().map(str::to_string),
            kind: HitKind::TreeItem,
            drag_axis: None,
            drag_data: None,
        });
    }
    targets
}

/// 🩸️ LAW: a frame build never empties what the pointer reads.
///
/// The whole production cycle is replayed on the REAL `InputState`: `FrameBuildPhase::InputFrame`
/// retires the previous generation one entry per boundary step, the chrome walk re-mints the next
/// one entry at a time, and the walk's end publishes. A press is replayed at EVERY instant of that
/// cycle — 100 consecutive presses, each at a different point inside a build — and every one must
/// resolve the same row the preceding move resolved, carrying the same action.
///
/// ⚖️ The counter-model in the same test is today's production shape: ONE vector, drained by the
/// build and scanned by `hit_at`. It misses, which is what makes this law discriminating rather than
/// decorative (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-end-to-end-verification-2026-09-14.md` §D).
#[test]
fn a_frame_build_never_empties_what_the_pointer_reads() {
    let law = law();
    let cycle = &law["frameCycle"];
    let case_name = cycle["case"].as_str().expect("case");
    let case = law["cases"].as_array().expect("cases").iter().find(|entry| entry["name"] == case_name).expect("frame cycle case");
    let presses = cycle["presses"].as_u64().expect("presses") as usize;
    let retire_steps_per_frame = cycle["retireStepsPerFrame"].as_u64().expect("retire steps") as usize;
    let (x, y) = (number(&cycle["point"], "x"), number(&cycle["point"], "y"));
    let want_control = cycle["controlId"].as_str().expect("control id");
    let want_action = cycle["action"].as_str().expect("action");
    let targets = fixture_registry(case);

    let mut input = InputState::<ActionDescriptor>::default();
    for target in &targets {
        input.register_hit(target.clone());
    }
    assert!(input.hit_at(x, y).is_none(), "a registry that was never published must resolve nothing");
    assert_eq!(input.publish_hits(), 1);
    let move_hit = input.hit_at(x, y).expect("the move must resolve the row").control_id.clone();
    assert_eq!(move_hit.as_deref(), Some(want_control), "the preceding move resolved another row");
    // 🌀️ One more complete build, so the measured cycle starts in production steady state: a full
    // registry resolvable AND the outgoing one staged for the next build's retirement.
    for target in &targets {
        input.register_hit(target.clone());
    }
    let move_generation = input.publish_hits();
    assert_eq!(input.staged_hits().len(), targets.len(), "steady state stages the outgoing registry");

    let mut resolved = 0usize;
    let mut missed = 0usize;
    let press = |input: &InputState<ActionDescriptor>, resolved: &mut usize, missed: &mut usize, at: &str| match input.hit_at(x, y) {
        Some(hit) if hit.control_id.as_deref() == Some(want_control) && hit.event.as_ref().map(|action| action.action.as_str()) == Some(want_action) => *resolved += 1,
        other => {
            *missed += 1;
            panic!("press at {at} resolved {:?} instead of {want_control}/{want_action}", other.map(|hit| hit.control_id.clone()));
        }
    };

    for frame in 0..presses {
        press(&input, &mut resolved, &mut missed, "the instant before the build's first retirement");
        let mut retired = 0usize;
        while input.retire_hit_step() {
            retired += retire_steps_per_frame;
            assert_eq!(input.staged_hits().len(), targets.len() - retired, "retirement must drop exactly {retire_steps_per_frame} entry per boundary step");
            press(&input, &mut resolved, &mut missed, "mid-retirement");
        }
        assert_eq!(retired, targets.len(), "the build must retire the whole previous generation");
        assert!(input.staged_hits().is_empty(), "the staged buffer is empty when the chrome walk starts");
        for target in &targets {
            input.register_hit(target.clone());
            press(&input, &mut resolved, &mut missed, "mid chrome walk");
        }
        assert_eq!(input.hits().len(), targets.len(), "the resolvable registry must never change size during a build");
        let generation = input.publish_hits();
        assert_eq!(generation, move_generation + frame as u64 + 1, "every completed build publishes exactly one generation");
        press(&input, &mut resolved, &mut missed, "the instant after the publish");
    }

    let expected = &cycle["expected"];
    assert_eq!(missed, expected["doubleBufferedMissed"].as_u64().expect("missed") as usize);
    assert_eq!(resolved, presses * (2 + targets.len() * 2), "every press of every frame must resolve");
    assert!(resolved >= expected["doubleBufferedResolved"].as_u64().expect("resolved") as usize);

    // ⚖️ The counter-model: ONE vector, drained by the build and scanned by the pointer — the shape
    // the renderer had before this law. The same cycle, the same points, and it MISSES.
    let mut single = targets.clone();
    let mut single_missed = 0usize;
    let mut single_resolved = 0usize;
    let resolve = |registry: &[HitTarget<ActionDescriptor>]| registry.iter().rev().find(|target| target.rect.contains(x, y)).and_then(|target| target.control_id.clone());
    for _ in 0..presses {
        while single.pop().is_some() {
            if resolve(&single).as_deref() == Some(want_control) {
                single_resolved += 1;
            } else {
                single_missed += 1;
            }
        }
        single = targets.clone();
    }
    assert!(single_missed >= expected["singleBufferedMissedAtLeast"].as_u64().expect("single missed") as usize, "the single-buffer counter-model must miss, or this law proves nothing");
    println!("[DEBUG] retained-hit-targets frame cycle: {presses} presses over {} entries — double-buffered resolved {resolved} missed {missed}; single-buffered resolved {single_resolved} missed {single_missed}", targets.len());
}
