use super::*;
use crate::wgpu::component::layout::ActionDescriptor;
use crate::wgpu::component::ui::{UiButtonNode, UiFieldNode, UiPresence, UiSectionNode, UiStackNode, UiTextNode};
use crate::wgpu::Label;

fn observe_layout_step(job: &mut MountedLayoutJob, cx: &mut semio_framework_job::StepContext<'_>) -> Option<semio_framework_job::JobOutcomeKind> {
    let outcome = semio_framework_job::InteractiveJob::step(job, cx).expect("original layout step").map(semio_framework_job::JobOutcomeBorrow::into_descriptor);
    outcome.map(|mut descriptor| {
        let kind = descriptor.kind();
        semio_framework_job::InteractiveJob::borrow_outcome(job, &descriptor).expect("same original layout descriptor");
        assert!(matches!(descriptor.acknowledge(ui_contract::UI_WORKER_RETIREMENT_POLICY), semio_framework_value::RetainedCloneStep::Complete(_)));
        kind
    })
}

fn clock_zero() -> Option<u64> {
    Some(0)
}

fn text_tree(value: String) -> (UiTree, NodeId) {
    let mut tree = UiTree::new();
    tree.apply_tree(&UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }));
    let root = tree.root.unwrap_or_else(|| panic!("text tree root"));
    (tree, root)
}

fn wide_tree(children: usize) -> (UiTree, NodeId) {
    let children = (0..children).map(|_| UiNode::Text(UiTextNode { value: Label::data(""), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })).collect();
    let mut tree = UiTree::new();
    tree.apply_tree(&UiNode::Stack(UiStackNode {
        direction: "vertical".into(),
        gap: None,
        padding: None,
        id: Some("hostile-wide".into()),
        presence: UiPresence::default(),
        activate: None,
        drop_action: None,
        drop_overlay: None,
        children,
        menu: None,
    }));
    let root = tree.root.unwrap_or_else(|| panic!("wide tree root"));
    (tree, root)
}

fn deep_tree(depth: usize) -> (UiTree, NodeId) {
    let mut node = UiNode::Text(UiTextNode { value: Label::data("deep"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None });
    for index in 0..depth {
        node = UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(format!("deep-{index}")),
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            children: vec![node],
            menu: None,
        });
    }
    let mut tree = UiTree::new();
    tree.apply_tree(&node);
    let root = tree.root.unwrap_or_else(|| panic!("deep tree root"));
    (tree, root)
}

fn text_job(tree: &UiTree, root: NodeId) -> MountedLayoutJob {
    MountedLayoutJob::try_new(tree, root, MountedLayoutIdentity { surface: UiSurfaceToken::new(3, 7), generation: 11, revision: 13, theme_revision: 17, viewport_revision: 19 }, Theme::default(), 640.0, 480.0, false, ui_contract::FlowInline::Ltr)
        .unwrap_or_else(|fault| panic!("mounted text job: {fault:?}"))
}

fn admit(job: &mut MountedLayoutJob, tree: &UiTree, cancel: &semio_framework_job::CancelToken) -> LayoutJobStep {
    let mut preview = 0;
    loop {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(23), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), clock_zero, &mut preview,&mut actual_retained_progress);
        let step = job.admit_one(tree, &mut cx);
        if job.is_admitted() || matches!(step, LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled) {
            return step;
        }
    }
}

#[test]
fn mounted_layout_node_max_plus_one_returns_the_exact_owner() {
    let mut nodes = ui_contract::UiFixedList::<u64, LAYOUT_NODE_CREDITS>::default();
    for owner in 0..LAYOUT_NODE_CREDITS as u64 {
        assert_eq!(nodes.try_push(owner), Ok(()));
    }
    assert_eq!(nodes.try_push(u64::MAX), Err(u64::MAX));
}

#[test]
fn mounted_layout_glyph_max_plus_one_returns_the_exact_owner() {
    let mut glyphs = ui_contract::UiFixedList::<u64, LAYOUT_GLYPH_CREDITS>::default();
    for owner in 0..LAYOUT_GLYPH_CREDITS as u64 {
        assert_eq!(glyphs.try_push(owner), Ok(()));
    }
    assert_eq!(glyphs.try_push(u64::MAX), Err(u64::MAX));
}

#[test]
fn mounted_layout_actual_glyph_max_plus_one_retains_the_exact_rejected_scalar() {
    let (tree, root) = text_tree("x".repeat(LAYOUT_GLYPH_CREDITS + 1));
    let mut job = text_job(&tree, root);
    let cancel = semio_framework_job::CancelToken::root_now();
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Fault("layout.glyph-credits")));
    assert_eq!(job.rejected_glyph().map(|owner| owner.scalar), Some('x'));
    job.begin_close();
    let before = job.glyphs.as_deref().expect("original layout owner").len();
    assert!(!job.close_one());
    assert_eq!(job.glyphs.as_deref().expect("original layout owner").len(), before);
    while !job.close_one() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn mounted_layout_actual_node_max_plus_one_retains_the_exact_rejected_tree_owner() {
    let (tree, root) = wide_tree(LAYOUT_NODE_CREDITS);
    let mut job = text_job(&tree, root);
    let cancel = semio_framework_job::CancelToken::root_now();
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Fault("layout.node-credits")));
    let rejected = job.rejected_node().unwrap_or_else(|| panic!("rejected node owner"));
    assert!(tree.contains(rejected.id));
    assert!(!job.nodes.as_deref().expect("original layout owner").iter().any(|retained| retained.id == rejected.id));
    job.begin_close();
    while !job.close_one() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn mounted_layout_deep_tree_depth_refusal_retains_walk_authority_for_close() {
    let (tree, root) = deep_tree(LAYOUT_DEPTH_CREDITS + 1);
    let mut job = text_job(&tree, root);
    let cancel = semio_framework_job::CancelToken::root_now();
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Fault("layout.depth-credits")));
    assert!(job.rejected_walk.is_some());
    job.begin_close();
    let retained = job.nodes.as_deref().expect("original layout owner").len() + job.walk.as_deref().expect("original layout owner").len() + usize::from(job.rejected_walk.is_some());
    assert!(!job.close_one());
    let after = job.nodes.as_deref().expect("original layout owner").len() + job.walk.as_deref().expect("original layout owner").len() + usize::from(job.rejected_walk.is_some());
    assert_eq!(retained - after, 1);
    while !job.close_one() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn() {
    let (tree, root) = text_tree("🙂".repeat(LAYOUT_GLYPH_CREDITS));
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut job = text_job(&tree, root);
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
    let mut turns = 0;
    let mut preview_sequence = 0;
    while job.stage() == LayoutJobStage::ShapeText {
        let before = job.glyph_cursor;
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(27), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), clock_zero, &mut preview_sequence,&mut actual_retained_progress);
        let _ = observe_layout_step(&mut job, &mut cx);
        assert!(job.glyph_cursor - before <= 1);
        turns += 1;
    }
    assert_eq!(job.glyph_cursor, LAYOUT_GLYPH_CREDITS);
    assert_eq!(job.atlas_candidate.page_cursor, LAYOUT_ATLAS_PAGE_CREDITS - 1);
    assert!(turns > LAYOUT_GLYPH_CREDITS);
    assert!(job.glyph_previews.as_deref().expect("original layout owner").iter().all(|preview| preview.generation == 11 && preview.revision == 13));
    job.begin_close();
    while !job.close_one() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn mounted_layout_worker_runs_on_shared_user_visible_lane_and_pool_thread() {
    let (tree, root) = text_tree("worker".to_string());
    let mut job = text_job(&tree, root);
    let cancel = semio_framework_job::CancelToken::root_now();
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
    let params = semio_framework_job::BatchJobParams {
        operation: semio_framework_job::OperationId(29),
        generation: semio_framework_job::Generation(11),
        cancel: cancel.clone(),
        config: semio_framework_job::BatchDriveConfig { retained:ui_contract::UI_WORKER_RETIREMENT_POLICY, site: "ui.layout-text.worker.law", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: clock_zero,
    };
    let mut original_job = Some(job);
    let mut original_params = Some(params);
    let mut original_receipt = semio_framework_job::RetainedCloneProgress::default();
    let mut original_preview = 0;
    let mut original_context = semio_framework_job::StepContext::new(semio_framework_job::OperationId(29), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, 1000, ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), clock_zero, &mut original_preview, &mut original_receipt);
    let (admitted, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_job::MountedWorkerJobSession::try_admit_owned(&mut original_job, &mut original_params, &mut original_context));
    let (mut session, birth) = admitted.expect("original mounted worker authority").expect("mounted worker session credit");
    assert_eq!((birth.retained_capacity_bytes, birth.released_bytes), (heap.requested_bytes, heap.released_bytes));
    assert_eq!(original_context.retained_progress(), birth);
    assert!(birth.fits(ui_contract::UI_WORKER_RETIREMENT_POLICY));
    assert!(original_job.is_none() && original_params.is_none());
    drop(original_context);
    eprintln!("[DEBUG] actual Mounted admission original operation29/generation11 cancellation witness and clock; receipt before exposure={birth:?} physical={heap:?}");
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let lane = semio_framework_async::Lane::UserVisible;
    for _ in 0..10_000 {
        let _ = session.pump_one(&pool, lane, ui_contract::UI_WORKER_RETIREMENT_POLICY);
        if session.poll() == semio_framework_job::WorkerJobPoll::CheckedOut {
            break;
        }
        std::thread::yield_now();
    }
    assert_eq!(lane, semio_framework_async::Lane::UserVisible);
    assert!(session.checked_out_job().is_some_and(|owner| owner.worker_thread_observed()));
    session.begin_close();
    for _ in 0..LAYOUT_GLYPH_CREDITS + LAYOUT_NODE_CREDITS * 4 {
        let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
        let demand = session.retirement_demands(grant.maximum_copy_bytes).expect("original layout worker close demands");
        if !ui_contract::ui_worker_retirement_permits(demand) {
            eprintln!("[DEBUG] mounted layout retained original close phase={:?} demand={demand:?} fixedPolicy={grant:?}", session.close_phase());
        }
        assert!(ui_contract::ui_worker_retirement_permits(demand));
        let phase=session.close_phase();
        if session.has_original_cancel_alias_witness(&cancel).expect("original mounted cancellation witness") {
            let (returned, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(||session.return_original_cancel_alias_step(&cancel, grant).expect("original mounted alias return").expect("pending original mounted alias"));
            assert!(returned.progress().fits(grant));
            assert_eq!((returned.progress().retained_capacity_bytes, returned.progress().released_bytes), (heap.requested_bytes, heap.released_bytes));
            continue;
        }
        let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||session.close_step(grant));
        let progress=step.progress();
        if (progress.retained_capacity_bytes,progress.released_bytes)!=(heap.requested_bytes,heap.released_bytes){eprintln!("[DEBUG] actual mounted worker physical receipt phase={phase:?} demand={demand:?} progress={progress:?} originalHeap={heap:?}");}
        assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(heap.requested_bytes,heap.released_bytes));
        assert!(progress.fits(grant));
        assert!(!matches!(step, semio_framework_job::WorkerJobCloseStep::Refused { .. }));
        if session.terminal_is_empty() {
            break;
        }
    }
    assert!(session.terminal_is_empty());
    let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(session));
    eprintln!("[DEBUG] actual mounted worker terminal Drop physical={} birth={}",heap.released_bytes,heap.requested_bytes);
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

#[test]
fn mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained() {
    let (tree, root) = text_tree("cancel".to_string());
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut before = text_job(&tree, root);
    assert!(matches!(admit(&mut before, &tree, &cancel), LayoutJobStep::Yield { .. }));
    cancel.cancel_now();
    let mut preview = 0;
    let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut before_cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(31), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel, clock_zero, &mut preview,&mut actual_retained_progress);
    assert!(matches!(observe_layout_step(&mut before, &mut before_cx), Some(semio_framework_job::JobOutcomeKind::Cancelled)));
    assert_eq!(before.glyph_cursor, 0);

    let after_cancel = semio_framework_job::CancelToken::root_now();
    let mut after = text_job(&tree, root);
    assert!(matches!(admit(&mut after, &tree, &after_cancel), LayoutJobStep::Yield { .. }));
    after.cancel_after_shape(after_cancel.clone());
    let mut after_preview = 0;
    let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut after_cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(37), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), after_cancel, clock_zero, &mut after_preview,&mut actual_retained_progress);
    assert!(matches!(observe_layout_step(&mut after, &mut after_cx), Some(semio_framework_job::JobOutcomeKind::Cancelled)));
    assert_eq!(after.glyph_cursor, 1);
    assert_eq!(after.latest_glyph_preview().map(|preview| preview.generation), Some(11));
    after.begin_close();
    while !after.close_one() {}
    assert!(after.terminal_is_empty());
}

#[test]
fn mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner() {
    let (tree, root) = text_tree("deadline".to_string());
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut job = text_job(&tree, root);
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
    let mut preview = 0;
    let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
    let mut expired = semio_framework_job::StepContext::new(semio_framework_job::OperationId(41), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, 0,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel, clock_zero, &mut preview,&mut actual_retained_progress);
    assert!(matches!(observe_layout_step(&mut job, &mut expired), Some(semio_framework_job::JobOutcomeKind::Yield)));
    assert_eq!(job.glyph_cursor, 0);
    let retained = job.glyphs.as_deref().expect("original layout owner").len() + job.nodes.as_deref().expect("original layout owner").len() + job.runs.as_deref().expect("original layout owner").len() + LAYOUT_ATLAS_PAGE_CREDITS;
    job.begin_close();
    assert!(!job.close_one());
    let after_one = job.glyphs.as_deref().expect("original layout owner").len() + job.nodes.as_deref().expect("original layout owner").len() + job.runs.as_deref().expect("original layout owner").len() + job.atlas_candidate.pages.iter().filter(|page| page.is_some()).count();
    assert_eq!(retained - after_one, 1);
    while !job.close_one() {}
    assert!(job.terminal_is_empty());
}

#[test]
fn mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once() {
    let (mut tree, root) = text_tree("publish".to_string());
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut job = text_job(&tree, root);
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
    let mut preview_sequence = 0;
    while job.stage() != LayoutJobStage::PublishResults {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(43), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), clock_zero, &mut preview_sequence,&mut actual_retained_progress);
        let _ = observe_layout_step(&mut job, &mut cx);
    }
    let stale = (UiSurfaceToken::new(3, 8), 11, 13, 17, 19, 640.0, 480.0);
    assert!(matches!(job.publish_one(&mut tree, stale), LayoutJobStep::Fault("layout.stale")));
    job.fault = None;
    let identity = job.identity();
    let before = tree.accepted_layout_generation();
    loop {
        let step = job.publish_one(&mut tree, identity);
        if matches!(step, LayoutJobStep::Complete) {
            break;
        }
        assert_eq!(tree.accepted_layout_generation(), before);
    }
    assert_eq!(tree.accepted_layout_generation(), 11);
    assert!(matches!(job.publish_one(&mut tree, identity), LayoutJobStep::Complete));
    assert_eq!(tree.accepted_layout_generation(), 11);
    job.begin_close();
    while !job.close_one() {}
    assert!(job.terminal_is_empty());
}

//#region 📐️AuthoredLayoutRects
use crate::wgpu::tree::{Node, NodeFlags, NodeKey, WidgetSpec};
use ui_contract::{Align, Axis, EdgeSpace, GridLayout, GridTrack, Justify, LayoutSpec, LeafLayout, Sizing, SpaceToken, StackLayout};

/// 🧪️ The arena is what layout reads, so a fixture mounts nodes straight into it with the AUTHORED
/// `LayoutSpec` a `UiNodeRecord` would have carried — the same channel `reconcile`'s document mount
/// stamps on every node it mounts.
fn mount(tree: &mut UiTree, parent: Option<NodeId>, ordinal: u32, node: UiNode, spec: LayoutSpec) -> NodeId {
    let mut mounted = Node::new(NodeKey::Positional(7, ordinal), WidgetSpec(node));
    mounted.layout_spec = Some(spec);
    tree.insert_child(parent, mounted)
}

fn leaf() -> UiNode {
    UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None })
}

fn fixed_leaf_spec() -> LayoutSpec {
    LayoutSpec::Leaf(LeafLayout { width: Sizing::Fixed(SpaceToken::Xxl), height: Sizing::Fixed(SpaceToken::Lg) })
}

fn stack_spec(axis: Axis, align: Align, justify: Justify, wrap: bool) -> LayoutSpec {
    LayoutSpec::Stack(StackLayout { axis, gap: SpaceToken::None, padding: EdgeSpace::default(), align, justify, grow: false, wrap })
}

fn solved(tree: &UiTree, id: NodeId) -> (f32, f32, f32, f32) {
    tree.mounted_layout(id).unwrap_or_else(|| panic!("published layout"))
}

/// 📐️ Layout resolves in fractional logical pixels, the way a DOM rect does — the shared ramp's
/// `Xxl` is 38.4px, not 38 — so a fixture compares within a hairline instead of demanding equality.
fn close(left: f32, right: f32) -> bool {
    (left - right).abs() < 0.01
}

/// 📐️ `justify: SpaceBetween` distributes leftover main-axis space BETWEEN children. The pre-parity
/// arrange step pushed 100% of it INTO every child unconditionally, so this fixture is the direct
/// regression guard for "elements placed totally different".
#[test]
fn mounted_layout_publishes_space_between_rects_without_growing_children() {
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Horizontal, Align::Start, Justify::SpaceBetween, false));
    let children: Vec<NodeId> = (0..3).map(|ordinal| mount(&mut tree, Some(root), ordinal + 1, leaf(), fixed_leaf_spec())).collect();
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);

    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 300.0, 100.0));

    assert_eq!(solved(&tree, root), (0.0, 0.0, 300.0, 100.0));
    assert!(close(solved(&tree, children[0]).0, 0.0));
    assert!(close(solved(&tree, children[1]).0, 130.8), "got {}", solved(&tree, children[1]).0);
    assert!(close(solved(&tree, children[2]).0, 261.6), "got {}", solved(&tree, children[2]).0);
    for child in &children {
        assert!(close(solved(&tree, *child).2, 38.4), "a space-between child keeps its own Fixed(Xxl) width");
    }
}

#[test]
fn mounted_layout_publishes_a_two_column_grid_as_two_columns() {
    let mut grid = GridLayout::default();
    assert_eq!(grid.try_push_column(GridTrack::Fraction(1)), Ok(()));
    assert_eq!(grid.try_push_column(GridTrack::Fraction(1)), Ok(()));
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), LayoutSpec::Grid(grid));
    let first = mount(&mut tree, Some(root), 1, leaf(), fixed_leaf_spec());
    let second = mount(&mut tree, Some(root), 2, leaf(), fixed_leaf_spec());
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);

    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 200.0, 100.0));

    assert!(close(solved(&tree, first).0, 0.0));
    assert!(close(solved(&tree, second).0, 100.0), "the second cell sits in the second COLUMN, never on a second row");
    assert!(close(solved(&tree, first).1, solved(&tree, second).1));
}

#[test]
fn mounted_layout_publishes_the_shared_overlay_flow_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️overlay-flow/🔣️.json")).expect("overlay flow fixture");
    let layout = |node: &str| serde_json::from_value(fixture[node]["layout"].clone()).unwrap_or_else(|error| panic!("{node} layout: {error}"));
    let expected = |node: &str| {
        let rect = &fixture[node]["expectedRect"];
        let number = |field: &str| rect[field].as_f64().unwrap_or_else(|| panic!("{node}.{field}")) as f32;
        (number("x"), number("y"), number("width"), number("height"))
    };
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), layout("root"));
    let overlay = mount(&mut tree, Some(root), 1, leaf(), layout("overlay"));
    let content = mount(&mut tree, Some(overlay), 2, leaf(), layout("content"));
    let absolute = mount(&mut tree, Some(root), 3, leaf(), layout("absolute"));
    let following = mount(&mut tree, Some(root), 4, leaf(), layout("following"));
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);

    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), fixture["viewport"]["width"].as_f64().expect("viewport width") as f32, fixture["viewport"]["height"].as_f64().expect("viewport height") as f32));

    for (node, id) in [("root", root), ("overlay", overlay), ("content", content), ("absolute", absolute), ("following", following)] {
        let actual = solved(&tree, id);
        let expected = expected(node);
        assert!(close(actual.0, expected.0) && close(actual.1, expected.1) && close(actual.2, expected.2) && close(actual.3, expected.3), "{node}: expected {expected:?}, got {actual:?}");
    }
}

#[test]
fn mounted_layout_wraps_an_overflowing_row_onto_a_second_line() {
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Horizontal, Align::Start, Justify::Start, true));
    let children: Vec<NodeId> = (0..4).map(|ordinal| mount(&mut tree, Some(root), ordinal + 1, leaf(), fixed_leaf_spec())).collect();
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);

    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 100.0, 100.0));

    assert!(close(solved(&tree, children[0]).1, 0.0));
    assert!(close(solved(&tree, children[1]).1, 0.0));
    assert!(solved(&tree, children[2]).1 > 0.0, "the third 38.4px child overflows a 100px line and wraps");
}

#[test]
fn mounted_layout_aligns_the_cross_axis_instead_of_always_stretching() {
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Horizontal, Align::Center, Justify::Start, false));
    let child = mount(&mut tree, Some(root), 1, leaf(), fixed_leaf_spec());
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);

    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 200.0, 100.0));

    let (_, y, _, height) = solved(&tree, child);
    assert!(close(height, 19.2), "align:center keeps the child's own height, never stretches it, got {height}");
    assert!(close(y, 40.4), "got {y}");
}

/// ✍️ A text run inside a narrow flex item wraps the way CSS does — the measured height grows by a
/// whole line box per wrapped line, from the shaped advances the job already collected.
#[test]
fn mounted_layout_wraps_text_inside_a_narrow_flex_item() {
    let build = |width: f32| {
        let mut tree = UiTree::new();
        let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Vertical, Align::Stretch, Justify::Start, false));
        let text = mount(&mut tree, Some(root), 1, UiNode::Text(UiTextNode { value: Label::data("alpha beta gamma delta"), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), LayoutSpec::default());
        tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
        assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), width, 400.0));
        solved(&tree, text)
    };
    let (_, _, wide_width, wide_height) = build(600.0);
    let (_, _, narrow_width, narrow_height) = build(60.0);
    assert!(wide_height > 0.0);
    assert!(narrow_height > wide_height, "a narrower item wraps onto more lines, got {narrow_height} vs {wide_height}");
    assert!(narrow_width <= wide_width);
}

/// 🤝️ LAW (ticket 26/09/23 session 14d, WG11 T7b): the retained layout's text worker prices a run with the atlas's own pair
/// kerning — a hugging text node is exactly as wide as the atlas measures the run, never its unkerned advance sum.
#[test]
fn mounted_layout_prices_a_text_run_with_the_atlas_pair_kerning() {
    let value = "The quick brown fox jumps over the lazy dog";
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Horizontal, Align::Start, Justify::Start, false));
    let text = mount(&mut tree, Some(root), 1, UiNode::Text(UiTextNode { value: Label::data(value), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None }), LayoutSpec::default());
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 1_000.0, 400.0));
    let (_, _, width, _) = solved(&tree, text);
    let mut atlas = crate::wgpu::text::FontAtlas::shaped_default();
    let kerned = atlas.measure_text(value, DEFAULT_TEXT_SIZE_PX).0;
    let unkerned: f32 = value.chars().map(|ch| atlas.ensure_glyph(ch, DEFAULT_TEXT_SIZE_PX).advance).sum();
    assert!(unkerned - kerned > 0.5, "the pangram kerns: {unkerned} vs {kerned}");
    assert!(close(width, kerned), "the laid-out run is the atlas's kerned width: {width} vs {kerned}");
}

fn composite_intrinsic_and_layout(tree: &mut UiTree, root: NodeId, width: f32) -> f32 {
    let identity = MountedLayoutIdentity { surface: UiSurfaceToken::new(5, 1), generation: 1, revision: 0, theme_revision: 0, viewport_revision: 0 };
    let mut job = MountedLayoutJob::try_new(tree, root, identity, Theme::default(), width, 400.0, false, ui_contract::FlowInline::Ltr).expect("composite layout job");
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut preview = 0;
    while !job.is_admitted() {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(61), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), clock_zero, &mut preview,&mut actual_retained_progress);
        assert!(!matches!(job.admit_one(tree, &mut cx), LayoutJobStep::Fault(_) | LayoutJobStep::Cancelled));
    }
    while job.stage() != LayoutJobStage::PublishResults {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(61), semio_framework_job::Generation(1), semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY), cancel.clone(), clock_zero, &mut preview,&mut actual_retained_progress);
        assert!(!matches!(observe_layout_step(&mut job, &mut cx), Some(semio_framework_job::JobOutcomeKind::Fault | semio_framework_job::JobOutcomeKind::Cancelled)));
    }
    let intrinsic = job.root_intrinsic_height().expect("root intrinsic height");
    let identity = job.identity();
    while !matches!(job.publish_one(tree, identity), LayoutJobStep::Complete) {}
    job.begin_close();
    while !job.close_one() {}
    intrinsic
}

fn geometry_button() -> UiNode {
    UiNode::Button(UiButtonNode {
        id: Some("geometry.control".into()),
        icon_id: crate::wgpu::IconName::CircleDot,
        label: Label::data("Control"),
        action: ActionDescriptor { controller_id: "geometry".into(), action: "change".into(), args: None },
        style: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

#[test]
fn section_and_field_measure_the_shared_wrapped_chrome_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📐️section-field-presentation/🔣️.json")).expect("section field fixture");
    let number = |path: &[&str]| path.iter().fold(&fixture, |value, key| &value[*key]).as_f64().expect("fixture number") as f32;
    let text = |path: &[&str]| path.iter().fold(&fixture, |value, key| &value[*key]).as_str().expect("fixture text");

    let field = UiNode::Field(UiFieldNode {
        id: "geometry".into(),
        label: Label::data(text(&["field", "label"])),
        description: Some(text(&["field", "description"]).into()),
        required: Some(true),
        error: Some(text(&["field", "error"]).into()),
        child: Box::new(geometry_button()),
        presence: UiPresence::default(),
        menu: None,
    });
    let mut field_tree = UiTree::new();
    field_tree.apply_tree(&field);
    let field_root = field_tree.root.expect("field root");
    let field_control = field_tree.node(field_root).and_then(|node| node.first_child).expect("field control");
    let field_height = composite_intrinsic_and_layout(&mut field_tree, field_root, number(&["field", "width"]));
    let control = solved(&field_tree, field_control);
    assert!(close(field_height, number(&["field", "totalHeight"])), "field hug height {field_height}");
    assert!(close(control.1, number(&["field", "controlTop"])), "control follows label, description and both gaps: {control:?}");
    assert!(close(control.3, number(&["field", "controlHeight"])), "control keeps its own line: {control:?}");
    let metrics =
        field_chrome_metrics(Some(number(&["density", "fieldDetailLineHeight"]) * number(&["field", "detailLines"])), control.3, Some(number(&["density", "fieldDetailLineHeight"]) * number(&["field", "detailLines"])), number(&["density", "gap"]));
    assert!(close(metrics.error.expect("error band").y, number(&["field", "errorTop"])));

    let section = |width: f32| {
        let mut tree = UiTree::new();
        tree.apply_tree(&UiNode::Section(UiSectionNode { id: "settings".into(), label: Some(Label::data(text(&["section", "title"]))), default_open: Some(true), presence: UiPresence::default(), menu: None, children: vec![geometry_button()] }));
        let root = tree.root.expect("section root");
        let child = tree.node(root).and_then(|node| node.first_child).expect("section child");
        let intrinsic = composite_intrinsic_and_layout(&mut tree, root, width);
        (intrinsic, solved(&tree, child))
    };
    let (narrow_height, narrow_child) = section(number(&["section", "width"]));
    let (wide_height, _) = section(500.0);
    assert!(close(narrow_child.1, number(&["section", "contentTop"])), "wrapped title owns two complete line boxes: {narrow_child:?}");
    assert!(close(narrow_height - wide_height, number(&["density", "sectionTitleLineHeight"])), "one extra title line increases hug height exactly once");
    assert!(close(narrow_height, number(&["section", "contentTop"]) + narrow_child.3 + number(&["section", "trailingMargin"])));
}

//#endregion 📐️AuthoredLayoutRects

//#region 🧩️HostContentLeaf
/// 🧩️ The panel content-projection ESCAPE HATCH: an `ExternalSlot` reserves the band its host declared
/// and fills the parent's width, instead of collapsing the way a childless `Leaf` does. This is what
/// lets a host-painted surface (React hosts an arbitrary subtree through `Tree`'s `emptyState`) live
/// inside an otherwise declarative document — the shell audit's recommendation 7.
#[test]
fn a_host_content_slot_reserves_the_band_its_host_declared() {
    let declared = 240.0;
    let mut tree = UiTree::new();
    let root = mount(
        &mut tree,
        None,
        0,
        UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, children: Vec::new(), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None }),
        LayoutSpec::default(),
    );
    let slot = mount(
        &mut tree,
        Some(root),
        1,
        UiNode::ExternalSlot(crate::wgpu::component::ui::UiExternalSlotNode {
            plugin_id: "framework".into(),
            app_id: "shell".into(),
            body_key: "framework.chat.transcript".into(),
            params_json: format!("{{\"hostContentHeight\": {declared}}}"),
            host_status: None,
            presence: UiPresence::default(),
            menu: None,
        }),
        LayoutSpec::default(),
    );
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);
    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 320.0, 800.0));
    let (_, _, width, height) = solved(&tree, slot);
    assert!(close(height, declared), "🧩️ the host's band is honoured exactly, got {height}");
    assert!(width > 0.0, "🧩️ and the slot fills its parent's width rather than measuring nothing");
}

/// 🧩️ A slot whose host declares no height still gets a visible box, and an absurd number is clamped
/// rather than asking for a band no viewport could hold.
#[test]
fn a_host_content_slot_without_a_declared_height_falls_back_and_clamps() {
    let theme = Theme::default();
    assert!(close(host_content_height("", &theme), theme.control_height * 6.0), "🧩️ no JSON at all still reserves a visible band");
    assert!(close(host_content_height("{}", &theme), theme.control_height * 6.0), "🧩️ nor does JSON without the key collapse it");
    assert!(close(host_content_height("{\"hostContentHeight\": 999999}", &theme), 4096.0), "🧩️ and an absurd number is clamped");
    assert!(close(host_content_height("{\"hostContentHeight\": -5}", &theme), 0.0), "🧩️ a negative band is nothing, never an inversion");
}
//#endregion 🧩️HostContentLeaf

#[test]
fn mounted_layout_original_fault_descriptor_preserves_custody_and_paid_acknowledgement() {
    use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, JobOutcomeKind, JobOutcomeView, RetainedCloneGrant, RetainedCloneProgress};
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🫴️receiving.json")).unwrap();
    let grant = ui_contract::UI_WORKER_RETIREMENT_POLICY;
    assert_eq!(grant.maximum_items, law["grant"]["maximumItems"].as_u64().unwrap() as usize);
    assert_eq!(grant.maximum_copy_bytes, law["grant"]["maximumCopyBytes"].as_u64().unwrap() as usize);
    assert_eq!(grant.maximum_capacity_bytes, law["grant"]["maximumCapacityBytes"].as_u64().unwrap() as usize);
    assert_eq!(grant.maximum_release_bytes, law["grant"]["maximumReleaseBytes"].as_u64().unwrap() as usize);
    assert_eq!(grant.maximum_depth, law["grant"]["maximumDepth"].as_u64().unwrap() as usize);
    let (tree, root) = text_tree("original".to_owned());
    let mut job = text_job(&tree, root);
    let cancel = semio_framework_job::CancelToken::root_now();
    assert!(matches!(admit(&mut job, &tree, &cancel), LayoutJobStep::Yield { .. }));
    job.fault = Some(MountedLayoutFault::Solver);
    let original_results = job.results.as_deref().unwrap() as *const _;
    let mut preview = 0;
    for denied in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }] {
        let mut progress = RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(law["operation"].as_u64().unwrap()), semio_framework_job::Generation(law["generation"].as_u64().unwrap()), semio_framework_job::StepBudget::new(1, u64::MAX, denied), cancel.clone(), clock_zero, &mut preview, &mut progress);
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| InteractiveJob::step(&mut job, &mut cx).map(|result| result.map(JobOutcomeBorrow::into_descriptor)));
        assert!(result.unwrap().is_none());
        assert_eq!(cx.retained_progress(), RetainedCloneProgress::default());
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(job.results.as_deref().unwrap() as *const _, original_results);
    }
    let mut accepted = false;
    for _ in 0..law["maximumFaultTurns"].as_u64().unwrap() {
        let mut progress = RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(law["operation"].as_u64().unwrap()), semio_framework_job::Generation(law["generation"].as_u64().unwrap()), semio_framework_job::StepBudget::new(1, u64::MAX, grant), cancel.clone(), clock_zero, &mut preview, &mut progress);
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| InteractiveJob::step(&mut job, &mut cx).map(|result| result.map(JobOutcomeBorrow::into_descriptor)));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let Some(mut descriptor) = result.unwrap() else { continue };
        assert_eq!(descriptor.kind(), JobOutcomeKind::Fault);
        assert_eq!(descriptor.admission().progress(), cx.retained_progress());
        let (pointer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| match job.borrow_outcome(&descriptor).unwrap() { JobOutcomeView::Fault { detail, .. } => detail as *const _, _ => panic!("original layout fault variant changed") });
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(pointer, job.fault_payload.published().unwrap() as *const _);
        assert_eq!(descriptor.acknowledge(RetainedCloneGrant { maximum_items: 0, ..grant }).progress(), RetainedCloneProgress::default());
        assert!(!descriptor.is_acknowledged());
        assert_eq!(descriptor.acknowledge(grant).progress(), RetainedCloneProgress { copied_items: 1, ..Default::default() });
        accepted = true;
        break;
    }
    assert!(accepted);
    assert_eq!(serde_json::to_value(job.fault.unwrap().label()).unwrap(), serde_json::json!("layout.solver"));
    job.begin_close();
    for _ in 0..law["maximumCloseTurns"].as_u64().unwrap() {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| job.close_step(grant));
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (heap.requested_bytes, heap.released_bytes));
        if job.terminal_is_empty() { break; }
    }
    assert!(job.terminal_is_empty());
    eprintln!("[DEBUG] Mounted original fault custody samePointer=true deniedAdmission0heap=true nativeBorrow0heap=true separatePaidAck=true originalPolicy=true physicalClose=true");
}
