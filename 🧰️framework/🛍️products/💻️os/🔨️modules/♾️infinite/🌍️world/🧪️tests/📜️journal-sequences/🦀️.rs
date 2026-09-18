//! 📜️ The ORDERED action journal a world pane publishes for one probe gesture.
//!
//! 🧫️ `🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json` carries, per journey step, the sequence
//! React was MEASURED publishing (the 2026-09-18 side-by-side run's `🗑️generated/parity-run-2/steps.json`)
//! and the sequence this lane answers, with the reason for every place they differ. The laws below
//! drive the REAL retained authority — `enqueue_world3d_event` → `step_world3d_interaction` → the
//! bounded action queue — with the probe's own gesture, so what they compare is the journal a live
//! page would produce, not a re-derivation of it.
//!
//! ⚖️ The three things this pins that the target got wrong before ticket
//! 26/09/17/WGPU-RENDERER-REACT-PARITY packet W10a: a navigation drag published one `setCamera` per
//! pointer MOVE where React publishes one trailing-debounced report per gesture, a gesture that moved
//! no camera published none at all where three's `OrbitControls` reports on EVERY pointer-up, and
//! `noteWorldNavigation` did not exist in the target at all.

use super::tests::{mesh_oracle_from_buffers, publish_oracle_mesh, take_actions, with_world_step_context};
use super::*;

const JOURNAL_SEQUENCES: &str = include_str!("../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/📜️journal-sequences.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(JOURNAL_SEQUENCES).expect("journal sequence fixture parses")
}

fn step_case(id: &str) -> serde_json::Value {
    fixture()["steps"].as_array().expect("step list").iter().find(|case| case["id"] == id).unwrap_or_else(|| panic!("fixture step {id}")).clone()
}

fn point(name: &str) -> [f32; 2] {
    let value = fixture()["surface"][name].as_array().expect("point").iter().map(|entry| entry.as_f64().expect("number") as f32).collect::<Vec<f32>>();
    [value[0], value[1]]
}

fn expected(id: &str, renderer: &str) -> Vec<String> {
    step_case(id)[renderer].as_array().expect("journal").iter().map(|entry| entry.as_str().expect("action id").to_string()).collect()
}

/// 🧫️ A pane whose pick rect covers the probe's aim point, with one small instance sitting on the
/// orbit target — the smallest scene that answers a hover, a pick and a camera gesture, and small
/// enough that a pan carries the target off it, which is what makes the hover CLEAR observable.
fn journey_state() -> World3dState {
    let mut state = World3dState::new("puzzle3d-main-perspective".into(), "controller-1".into());
    state.bounds = Rect { x: 0.0, y: 0.0, w: 560.0, h: 360.0 };
    state.pick_bounds = state.bounds;
    state.active_utility = "select".into();
    state.interaction_objects.revision = state.interaction_revision;
    let data = mesh_oracle_from_buffers(
        vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.5, 0.5, 0.0, -0.5, 0.5, 0.0],
        vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0],
        vec![0, 1, 2, 0, 2, 3],
    );
    store_mesh(&mut state, "mesh".into(), publish_oracle_mesh(data));
    let mesh_version = *state.mesh_versions.get("mesh").expect("mesh version");
    state.draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version, instances: vec![Instance3d { id: "object".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }] });
    state
}

/// 🏁️ Drives every queued intent — and the camera settle the drained queue owes — to idle, answering
/// with the ORDERED action ids published along the way.
fn journal(state: &mut World3dState) -> Vec<String> {
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let mut published = Vec::new();
    let mut turns = 0;
    while let Some(generation) = world3d_interaction_front_generation(state) {
        let step = with_world_step_context(1, |context| step_world3d_interaction(state, generation, &mut input, context));
        assert_ne!(step, WorldInteractionAuthorityStep::Fault, "a probe gesture never faults the authority");
        published.extend(take_actions(&mut input).into_iter().map(|action| action.action));
        turns += 1;
        assert!(turns < 4096, "a bounded journey terminates");
        if step == WorldInteractionAuthorityStep::Idle {
            break;
        }
    }
    published
}

fn enqueue(state: &mut World3dState, intent: WorldInteractionIntent) {
    enqueue_world3d_event(state, intent).expect("the world interaction queue admits a probe intent");
}

/// 🖱️ The probe's own drag: one move onto the aim point, a press, eight interpolated moves, a release.
fn drag(state: &mut World3dState, button: i16, to: [f32; 2]) {
    let modifiers = PointerModifiers::default();
    let from = point("aim");
    enqueue(state, WorldInteractionIntent::pointer_move(from[0], from[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(from[0], from[1], true, button, &modifiers));
    let mut last = from;
    for sample in 1..=8 {
        let next = [from[0] + (to[0] - from[0]) * sample as f32 / 8.0, from[1] + (to[1] - from[1]) * sample as f32 / 8.0];
        enqueue(state, WorldInteractionIntent::pointer_move(next[0], next[1], next[0] - last[0], next[1] - last[1], true, button, &modifiers));
        last = next;
    }
    enqueue(state, WorldInteractionIntent::pointer_button(last[0], last[1], false, button, &modifiers));
}

/// 🖱️ The probe's own click: one move onto the aim point, a press and a release at that same point.
fn click(state: &mut World3dState, button: i16) {
    let modifiers = PointerModifiers::default();
    let at = point("aim");
    enqueue(state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], true, button, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], false, button, &modifiers));
}

#[test]
fn an_orbit_drag_journals_the_reference_sequence_and_moves_no_camera() {
    let mut state = journey_state();
    let before = state.orbit.clone();
    drag(&mut state, 0, point("dragTo"));
    assert_eq!(journal(&mut state), expected("orbit-drag", "wgpu"));
    assert_eq!(state.orbit.target, before.target, "the LEFT button drives no camera on either renderer");
    assert_eq!(state.orbit.distance, before.distance);
}

#[test]
fn a_pan_drag_journals_one_navigation_note_and_one_debounced_camera_report() {
    let mut state = journey_state();
    let before = state.orbit.target;
    drag(&mut state, 1, point("panTo"));
    let published = journal(&mut state);
    assert_eq!(published, expected("pan-drag", "wgpu"));
    assert_eq!(published.iter().filter(|action| *action == "setCamera").count(), 1, "eight pointer moves owe ONE report, not eight");
    assert_ne!(state.orbit.target, before);
}

#[test]
fn a_wheel_zoom_journals_exactly_the_reference_sequence() {
    let mut state = journey_state();
    let at = point("aim");
    let delta = fixture()["surface"]["wheelDelta"].as_f64().expect("wheel delta") as f32;
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::wheel(at[0], at[1], delta, &modifiers));
    let published = journal(&mut state);
    assert_eq!(published, expected("zoom-wheel", "wgpu"));
    assert_eq!(published, expected("zoom-wheel", "react")[..published.len()], "the wheel step is React's own sequence up to the trailing hover only a multi-instance scene can change");
}

#[test]
fn a_pick_journals_one_selection_and_the_unconditional_camera_report() {
    let mut state = journey_state();
    let before = state.orbit.clone();
    click(&mut state, 0);
    let published = journal(&mut state);
    assert_eq!(published, expected("pick-instance", "wgpu"));
    assert_eq!(state.orbit.distance, before.distance, "a pick moves no camera — and still owes the report every pointer-up owes");
}

#[test]
fn a_right_click_journals_no_menu_verb_at_all() {
    let mut state = journey_state();
    click(&mut state, 2);
    let published = journal(&mut state);
    assert_eq!(published, expected("context-menu", "wgpu"));
    assert!(!published.iter().any(|action| action.contains("ontextMenu")), "the menu is the shell's; React deleted its own `contextMenuAt` and this lane deleted `worldContextMenuAt` with it");
}

/// 🖱️🎯️ **The non-primary press selects too.** R3F raises the canvas's `onPointerMissed` for ANY
/// button — on the `pointerdown` that resolved no event-handling object and again on the click — and
/// `🌐️World3dHost/🟦️.tsx`'s `handleEmptyClick` publishes `interactionSelect` from each. A primary
/// press arms the marquee, whose `wasMarqueeDragRef` swallows the first of the two; a middle or right
/// press arms nothing, which is exactly why React's `pan-drag` and `context-menu` journal TWO
/// selections and its `orbit-drag` one.
///
/// 🩸️ This lane published none for either: the middle press retired as a gesture-less intent and the
/// right press only opened the shell's menu, so pressing a pane with anything but the left button
/// never touched the selection (`🗑️generated/w12c-parity-run-19/steps.json` steps 20 and 23,
/// ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W13c). One selection, not two: a second
/// selection of the same target is a duplicate, not parity, and the probe's verdict is set-based.
#[test]
fn a_non_primary_press_publishes_the_selection_react_publishes_for_any_button() {
    for button in [1_i16, 2] {
        let mut state = journey_state();
        let at = point("aim");
        let modifiers = PointerModifiers::default();
        enqueue(&mut state, WorldInteractionIntent::pointer_button(at[0], at[1], true, button, &modifiers));
        let published = journal(&mut state);
        assert_eq!(published, ["interactionSelect"], "🎯️ button {button}: the press alone publishes one selection and nothing else");
    }

    let mut state = journey_state();
    drag(&mut state, 1, point("panTo"));
    assert_eq!(journal(&mut state), expected("pan-drag", "wgpu"), "🎯️ and the whole middle drag is React's own sequence");

    let mut state = journey_state();
    click(&mut state, 2);
    assert_eq!(journal(&mut state), expected("context-menu", "wgpu"));
}

/// 🖌️ The press verb belongs to the SELECT lane only. Component mode, the brush utility and vertex
/// granularity each own their own press route (`WorldComponentPickCursor`, `WorldObjectPickCursor`),
/// and a second selection published beside them would be this lane inventing a verb React never
/// sends — `handleEmptyClick` returns early under `paintMode` for the same reason.
#[test]
fn a_non_primary_press_publishes_nothing_outside_the_select_lane() {
    for (utility, granularity) in [("brush", "mesh"), ("select", "vertex"), ("surfaceBrush", "mesh")] {
        let mut state = journey_state();
        state.active_utility = utility.into();
        state.granularity = granularity.into();
        let at = point("aim");
        let modifiers = PointerModifiers::default();
        enqueue(&mut state, WorldInteractionIntent::pointer_button(at[0], at[1], true, 1, &modifiers));
        assert!(!journal(&mut state).iter().any(|action| action == "interactionSelect"), "🖌️ {utility}/{granularity} owns its own press verbs");
    }
}

/// 🫧 **The ingress fold.** The frame worker drains its input queue far faster than the retained
/// authority answers it, so a burst of moves arrives with nothing stepped in between — measured live
/// as `queued=10` in front of ONE eight-move drag, ten full pick/plan turns for one gesture
/// (`🗑️generated/w12c-parity-run-19/wgpu/console.txt`, ticket 26/09/17 packet W13c §3). React never
/// sees a move per DOM event either: its transport keeps one sample per pointer identity per
/// animation frame, and its hover dispatcher keeps at most one round trip outstanding and coalesces
/// the rest onto the latest target.
///
/// ⚖️ What the fold may NOT do is move the gesture, and this is the law that says so: the same drag
/// is driven twice — once folded, once on a LASSO pane, where `world3d_pointer_move_folds` refuses —
/// and the two journals, their order and the settled camera must be identical while the queue depth
/// is not. `dx`/`dy` accumulate and both camera operations integrate their delta linearly
/// (`OrbitController::pan` along an orientation-only basis, `orbit` straight onto yaw/pitch), and
/// `x`/`y` take the newest sample, which is the point React would have raycast.
#[test]
fn a_burst_of_pointer_moves_folds_onto_the_latest_without_moving_the_gesture() {
    let mut folded = journey_state();
    drag(&mut folded, 1, point("panTo"));
    let folded_depth = folded.interaction_authority.as_ref().expect("authority").queue.len;
    let folded_journal = journal(&mut folded);

    let mut whole = journey_state();
    whole.selection_method = "lasso".into();
    assert!(!world3d_pointer_move_folds(&whole), "🫧 a lasso marquee IS its own path, so its samples are never folded");
    drag(&mut whole, 1, point("panTo"));
    let whole_depth = whole.interaction_authority.as_ref().expect("authority").queue.len;
    let whole_journal = journal(&mut whole);

    assert_eq!(folded_journal, expected("coalesced-drag", "wgpu"));
    assert_eq!(folded_journal, whole_journal, "🫧 the fold publishes the same journal in the same order");
    assert_eq!(u32::from(whole_depth) - u32::from(folded_depth), 7, "🫧 eight moves are delivered as one — the fold is the whole point");
    assert!((folded.orbit.target.x - whole.orbit.target.x).abs() < 1e-3 && (folded.orbit.target.y - whole.orbit.target.y).abs() < 1e-3 && (folded.orbit.target.z - whole.orbit.target.z).abs() < 1e-3, "🫧 and the settled camera is the same camera");

    let mut painting = journey_state();
    painting.interaction_mode = "paint".into();
    painting.paint_stroke_active = true;
    assert!(!world3d_pointer_move_folds(&painting), "🫧 a live paint stroke raycasts once per move, so a folded move would leave a gap in the run");
}

/// 🫧 A fold never reaches the FRONT — a live cursor may already be half-stepping it — and never
/// crosses a change of button, of `down` or of the modifier state, each of which is a different
/// gesture.
#[test]
fn a_fold_never_touches_the_front_or_crosses_a_gesture_boundary() {
    let modifiers = PointerModifiers::default();
    let mut state = journey_state();
    let at = point("aim");
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 4.0, at[1], 4.0, 0.0, false, 0, &modifiers));
    assert_eq!(state.interaction_authority.as_ref().expect("authority").queue.len, 2, "🫧 the only queued move IS the front, so the second one queues behind it");

    let mut state = journey_state();
    enqueue(&mut state, WorldInteractionIntent::pointer_button(at[0], at[1], true, 1, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 4.0, at[1], 4.0, 0.0, true, 1, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 8.0, at[1], 4.0, 0.0, true, 1, &modifiers));
    assert_eq!(state.interaction_authority.as_ref().expect("authority").queue.len, 2, "🫧 two drag moves behind the press fold into one");
    let shifted = PointerModifiers { shift: true, ..PointerModifiers::default() };
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 12.0, at[1], 4.0, 0.0, true, 1, &shifted));
    assert_eq!(state.interaction_authority.as_ref().expect("authority").queue.len, 3, "🫧 a modifier change starts a new intent");
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0] + 16.0, at[1], 4.0, 0.0, false, 1, &shifted));
    assert_eq!(state.interaction_authority.as_ref().expect("authority").queue.len, 4, "🫧 and so does lifting the button");
}

/// 🧭️ The classifier is React's `classifyWorldNavigationGestures`, thresholds and push order
/// included — a set, never one verdict, so a drag that pans while it orbits reports both.
#[test]
fn the_navigation_classifier_answers_reacts_own_thresholds_and_order() {
    let still = WorldNavigationSnapshot { position: [0.0, 0.0, 10.0], target: [0.0, 0.0, 0.0], zoom: 1.0, parallel: false };
    assert_eq!(classify_world_navigation_gestures(still, still), [false, false, false]);
    let panned = WorldNavigationSnapshot { target: [4.0, 0.0, 0.0], position: [4.0, 0.0, 10.0], ..still };
    assert_eq!(classify_world_navigation_gestures(still, panned), [true, false, false]);
    let dollied = WorldNavigationSnapshot { position: [0.0, 0.0, 5.0], ..still };
    assert_eq!(classify_world_navigation_gestures(still, dollied), [false, true, false]);
    let orbited = WorldNavigationSnapshot { position: [10.0, 0.0, 0.0], ..still };
    assert_eq!(classify_world_navigation_gestures(still, orbited), [false, false, true]);
    let below_every_threshold = WorldNavigationSnapshot { position: [0.0, 0.0, 10.1], target: [0.0, 0.0, 0.01], ..still };
    assert_eq!(classify_world_navigation_gestures(still, below_every_threshold), [false, false, false], "a gesture under React's thresholds is no gesture");
}

/// 📐️ **The orthographic Top pane answers the same journal.** Pan, wheel and pick are
/// projection-agnostic all the way down: `OrbitController::pan` scales by `1 / zoom` under the
/// parallel frustum and `zoom` scales the frustum instead of dollying, `world3d_camera_zoom` reports
/// that live scale (a perspective pane reports the identity), and the classifier takes React's own
/// orthographic branch — `after.zoom / before.zoom` rather than a dolly ratio
/// (`captureNavigationSnapshot`). So a Top pane journals pan/zoom/pick exactly as the perspective one
/// does, with a `zoom` in the report that actually moves.
#[test]
fn an_orthographic_pane_journals_the_same_sequences_with_a_live_zoom() {
    let mut state = journey_state();
    state.orbit.projection = CameraProjection3d::Orthographic;
    state.orbit.zoom = 40.0;
    let before = world_navigation_snapshot(&state);
    assert!(before.parallel && (before.zoom - 40.0).abs() < 1e-6, "a parallel pane reports its live frustum scale, never the perspective identity");
    drag(&mut state, 1, point("panTo"));
    assert_eq!(journal(&mut state), expected("pan-drag", "wgpu"));

    let mut state = journey_state();
    state.orbit.projection = CameraProjection3d::Orthographic;
    state.orbit.zoom = 40.0;
    let at = point("aim");
    let delta = fixture()["surface"]["wheelDelta"].as_f64().expect("wheel delta") as f32;
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(&mut state, WorldInteractionIntent::wheel(at[0], at[1], delta, &modifiers));
    assert_eq!(journal(&mut state), expected("zoom-wheel", "wgpu"));
    assert_ne!(state.orbit.zoom, 40.0, "a parallel wheel moves the frustum scale, which is the number its report carries");

    let mut state = journey_state();
    state.orbit.projection = CameraProjection3d::Orthographic;
    state.orbit.zoom = 40.0;
    click(&mut state, 0);
    assert_eq!(journal(&mut state), expected("pick-instance", "wgpu"));
}

/// 🚪️ **The chrome-press law.** A pointer the shell's chrome owns hands this lane exactly one thing:
/// the clear of the hover it has published. No move, no press, no release and no wheel — React's
/// canvas receives none of those while a DOM layer above it is under the pointer, and the only event
/// it does receive is r3f's `onPointerOut`.
///
/// 🩸️ The eight journey steps of `📓️w11a-prepared-world-mesh-missing.md` §6 family A each journalled
/// an `interactionHover` AND an `interactionSelect` the reference never emits, and the stray select
/// carried empty `targets` — pressing a navbar panel tab CLEARED the world selection. The routing
/// that stops it is `🐚️Shell`'s `PointerCapture`/`pointer_owner_at`; what this law pins is the other
/// half: that the leave those two hand down is a hover clear and nothing more.
#[test]
fn a_chrome_owned_pointer_hands_the_world_lane_one_hover_clear_and_nothing_else() {
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    assert_eq!(journal(&mut state), ["interactionHover"], "🚪️ the pane publishes a hover while the pointer is still its own");
    assert!(world3d_hover_is_published(&state) && state.local_hover_id.is_some());

    let before = state.orbit.clone();
    enqueue(&mut state, WorldInteractionIntent::pointer_leave(at[0], at[1]));
    let published = journal(&mut state);
    assert_eq!(published, expected("chrome-press", "wgpu"), "🚪️ a leave answers the published hover and nothing else");
    assert_eq!(published, expected("chrome-press", "react"), "🚪️ which is exactly what React's `onPointerOut` publishes");
    assert!(!published.iter().any(|action| action == "interactionSelect" || action == "setCamera"), "🚪️ the press and the release never reached this lane, so neither the selection nor the camera moved");
    assert!(state.local_hover_id.is_none() && !world3d_hover_is_published(&state), "🚪️ and the hover is CLEAR, which is what the guest reads");
    assert_eq!(state.orbit.target, before.target);
    assert_eq!(state.orbit.distance, before.distance);

    enqueue(&mut state, WorldInteractionIntent::pointer_leave(at[0], at[1]));
    assert!(journal(&mut state).is_empty(), "🚪️ a leave over an already-clear surface journals nothing at all — React does not re-clear a hover it never had");
}

/// 🚪️ A leave aimed OUTSIDE the surface's own pick rect — the pointer travelled to the navbar, to a
/// sibling pane, off the canvas — is answered, never refused. Every pick cursor is built through
/// `pointer_in_pick_rect`, so a leave that had to run one would fault the frame, which is what
/// `📓️w9c-behaviour-parity-run-2.md` §7b measured a stale aim doing to the whole page.
#[test]
fn a_leave_from_outside_the_pick_rect_clears_the_hover_without_faulting() {
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    assert_eq!(journal(&mut state), ["interactionHover"]);
    let outside = state.bounds.x + state.bounds.w + 64.0;
    enqueue(&mut state, WorldInteractionIntent::pointer_leave(outside, 4.0));
    assert_eq!(journal(&mut state), expected("chrome-press", "wgpu"));
    assert!(state.local_hover_id.is_none());
}

/// 🫥️ LAW: a draw naming a mesh the guest has NOT published yet is SKIPPED by both interaction
/// cursors — the registry build and the ray pick — so the pane keeps hovering, picking and reporting
/// its camera exactly as if that draw were not there.
///
/// 🩸️ Both cursors answered an empty probe slot with `WorldInteractionStep::Fault`, which the frame
/// driver turns into `world3d retained interaction authority faulted` and the browser shell into a
/// DEAD PAGE. The guest publishes its draw list and its mesh leases in separate deliveries, so every
/// glb-bearing document has frames where a draw names a lease still landing: measured live at t≈9.4 s
/// on the first pointer move after the introduction tour was dismissed, which killed the page before
/// a single journey step could be measured (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w12c`,
/// `🗑️generated/w12c-bisect-world/wgpu/console.txt`).
///
/// ⚖️ React has no such failure: a `GlbInstanceMesh` whose loader has not resolved renders nothing and
/// raycasts to nothing. This is the interaction twin of W11a's "a miss is a skipped draw, not a
/// quarantine".
#[test]
fn a_draw_whose_mesh_has_not_landed_is_skipped_by_every_interaction_cursor() {
    let mut state = journey_state();
    let resident = journal_of_a_click(&mut journey_state());
    state.draws.push(SceneDraw3d {
        mesh_key: "mesh:🧊️still-loading".into(),
        mesh_version: 7,
        instances: vec![Instance3d { id: "unlanded".into(), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false }],
    });
    state.interaction_revision = state.interaction_revision.wrapping_add(1);

    assert_eq!(journal_of_a_click(&mut state), resident, "🫥️ the unpublished draw changes nothing the pane journals");
    assert!(!state.interaction_meshes.faulted && !state.interaction_objects.faulted, "🫥️ and nothing is quarantined");
}

/// 🖱️ The probe's own click — one move onto the aim point, a press and a release at the same point —
/// answered to idle.
fn journal_of_a_click(state: &mut World3dState) -> Vec<String> {
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], true, 0, &modifiers));
    enqueue(state, WorldInteractionIntent::pointer_button(at[0], at[1], false, 0, &modifiers));
    journal(state)
}

/// 🚪️ **The per-FRAME clear is idempotent.** A caller driven by frames, not by pointer events, must
/// ask `world3d_hover_clear_is_owed` and not `world3d_hover_is_published`: the published hover only
/// goes clear when the leave is ANSWERED, so while one waits in the queue the published predicate
/// stays true and the caller enqueues another, every frame.
///
/// 🩸️ Measured live at the journey's `context-menu` step: `queued=64 blocked=true` on
/// `puzzle3d-main-perspective`, the bounded intent queue full of identical leaves, then
/// `bounded frame input action authority faulted` and a dead page — with every step from 23 on
/// answering `renderer exposes no dumpChrome` (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `🗑️generated/w13c-parity-run-20`).
#[test]
fn a_frame_driven_hover_clear_is_owed_once_and_never_saturates_the_queue() {
    let mut state = journey_state();
    let at = point("aim");
    let modifiers = PointerModifiers::default();
    enqueue(&mut state, WorldInteractionIntent::pointer_move(at[0], at[1], 0.0, 0.0, false, 0, &modifiers));
    assert_eq!(journal(&mut state), ["interactionHover"]);
    assert!(world3d_hover_clear_is_owed(&state), "🚪️ a published hover with no leave in flight owes one");

    let outside = [state.bounds.x - 1.0, state.bounds.y - 1.0];
    for frame in 0..256 {
        if world3d_hover_clear_is_owed(&state) {
            enqueue(&mut state, WorldInteractionIntent::pointer_leave(outside[0], outside[1]));
        }
        let depth = state.interaction_authority.as_ref().expect("authority").queue.len;
        assert!(depth <= 1, "🚪️ frame {frame}: at most ONE leave is ever in flight, never a queue full of them (depth {depth})");
    }
    assert!(!world3d_hover_clear_is_owed(&state) && world3d_hover_is_published(&state), "🚪️ nothing more is owed while the leave waits, although the hover is still PUBLISHED — which is exactly why the published predicate cannot gate a frame-driven caller");

    assert_eq!(journal(&mut state), expected("modal-open", "wgpu"), "🚪️ and answering it publishes React's own single hover clear");
    assert!(!world3d_hover_clear_is_owed(&state), "🚪️ after which nothing is owed and no later frame enqueues anything");
    assert_eq!(state.interaction_authority.as_ref().expect("authority").queue.len, 0);
}
