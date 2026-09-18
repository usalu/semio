
use super::*;
use crate::wgpu::Label;
use crate::wgpu::component::ui::{UiPresence, UiStackNode, UiTextNode};

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
    MountedLayoutJob::try_new(tree, root, MountedLayoutIdentity { surface: UiSurfaceToken::new(3, 7), generation: 11, revision: 13, theme_revision: 17, viewport_revision: 19 }, Theme::default(), 640.0, 480.0).unwrap_or_else(|fault| panic!("mounted text job: {fault:?}"))
}

fn admit(job: &mut MountedLayoutJob, tree: &UiTree, cancel: &semio_framework_job::CancelToken) -> LayoutJobStep {
    let mut preview = 0;
    loop {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(23), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), clock_zero, &mut preview);
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
    let before = job.glyphs.len();
    assert!(!job.close_one());
    assert_eq!(job.glyphs.len(), before);
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
    assert!(!job.nodes.iter().any(|retained| retained.id == rejected.id));
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
    let retained = job.nodes.len() + job.walk.len() + usize::from(job.rejected_walk.is_some());
    assert!(!job.close_one());
    let after = job.nodes.len() + job.walk.len() + usize::from(job.rejected_walk.is_some());
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
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(27), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), clock_zero, &mut preview_sequence);
        let _ = semio_framework_job::InteractiveJob::step(&mut job, &mut cx);
        assert!(job.glyph_cursor - before <= 1);
        turns += 1;
    }
    assert_eq!(job.glyph_cursor, LAYOUT_GLYPH_CREDITS);
    assert_eq!(job.atlas_candidate.page_cursor, LAYOUT_ATLAS_PAGE_CREDITS - 1);
    assert!(turns > LAYOUT_GLYPH_CREDITS);
    assert!(job.glyph_previews.iter().all(|preview| preview.generation == 11 && preview.revision == 13));
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
        cancel,
        config: semio_framework_job::BatchDriveConfig { site: "ui.layout-text.worker.law", stage: semio_framework_job::InteractiveStage::UserVisibleSimStep, fuel_per_step: 1, step_budget_us: 1000 },
        now_us: semio_framework_job::default_now_us,
    };
    let mut session = semio_framework_job::MountedWorkerJobSession::try_new(job, params).unwrap_or_else(|_| panic!("mounted worker session credit"));
    let pool = semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::HeadlessBatch, 1));
    let lane = semio_framework_async::Lane::UserVisible;
    for _ in 0..10_000 {
        let _ = session.pump_one(&pool, lane);
        if session.poll() == semio_framework_job::WorkerJobPoll::CheckedOut {
            break;
        }
        std::thread::yield_now();
    }
    assert_eq!(lane, semio_framework_async::Lane::UserVisible);
    assert!(session.checked_out_job_mut().is_some_and(|owner| owner.worker_thread_observed()));
    session.begin_close();
    for _ in 0..LAYOUT_GLYPH_CREDITS + LAYOUT_NODE_CREDITS * 4 {
        let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        if session.terminal_is_empty() {
            break;
        }
    }
    assert!(session.terminal_is_empty());
}

#[test]
fn mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained() {
    let (tree, root) = text_tree("cancel".to_string());
    let cancel = semio_framework_job::CancelToken::root_now();
    let mut before = text_job(&tree, root);
    assert!(matches!(admit(&mut before, &tree, &cancel), LayoutJobStep::Yield { .. }));
    cancel.cancel_now();
    let mut preview = 0;
    let mut before_cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(31), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel, clock_zero, &mut preview);
    assert!(matches!(semio_framework_job::InteractiveJob::step(&mut before, &mut before_cx), semio_framework_job::StepOutcome::Cancelled));
    assert_eq!(before.glyph_cursor, 0);

    let after_cancel = semio_framework_job::CancelToken::root_now();
    let mut after = text_job(&tree, root);
    assert!(matches!(admit(&mut after, &tree, &after_cancel), LayoutJobStep::Yield { .. }));
    after.cancel_after_shape(after_cancel.clone());
    let mut after_preview = 0;
    let mut after_cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(37), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), after_cancel, clock_zero, &mut after_preview);
    assert!(matches!(semio_framework_job::InteractiveJob::step(&mut after, &mut after_cx), semio_framework_job::StepOutcome::Cancelled));
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
    let mut expired = semio_framework_job::StepContext::new(semio_framework_job::OperationId(41), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, 0), cancel, clock_zero, &mut preview);
    assert!(matches!(semio_framework_job::InteractiveJob::step(&mut job, &mut expired), semio_framework_job::StepOutcome::Yield));
    assert_eq!(job.glyph_cursor, 0);
    let retained = job.glyphs.len() + job.nodes.len() + job.runs.len() + LAYOUT_ATLAS_PAGE_CREDITS;
    job.begin_close();
    assert!(!job.close_one());
    let after_one = job.glyphs.len() + job.nodes.len() + job.runs.len() + job.atlas_candidate.pages.iter().filter(|page| page.is_some()).count();
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
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::OperationId(43), semio_framework_job::Generation(11), semio_framework_job::StepBudget::new(1, u64::MAX), cancel.clone(), clock_zero, &mut preview_sequence);
        let _ = semio_framework_job::InteractiveJob::step(&mut job, &mut cx);
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
use ui_contract::{Align, Axis, EdgeSpace, GridLayout, GridTrack, Justify, LayoutSpec, LeafLayout, OverlayLayout, Sizing, SpaceToken, StackLayout};

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
fn mounted_layout_keeps_an_overlay_out_of_flow() {
    let mut tree = UiTree::new();
    let root = mount(&mut tree, None, 0, leaf(), stack_spec(Axis::Vertical, Align::Stretch, Justify::Start, false));
    let overlay = mount(&mut tree, Some(root), 1, leaf(), LayoutSpec::Overlay(OverlayLayout { anchor: ui_contract::Anchor::Center, inset: EdgeSpace::All(SpaceToken::Md), dismissible: true }));
    let sibling = mount(&mut tree, Some(root), 2, leaf(), fixed_leaf_spec());
    tree.mark_dirty(root, NodeFlags::DIRTY_LAYOUT);

    assert!(crate::wgpu::mounted_layout::layout_tree_now(&mut tree, root, Theme::default(), 200.0, 100.0));

    assert!(close(solved(&tree, sibling).1, 0.0), "an overlay must not push its in-flow sibling down");
    let (x, y, width, height) = solved(&tree, overlay);
    assert!(close(x, 12.8) && close(y, 12.8) && close(width, 174.4) && close(height, 74.4), "got {:?}", solved(&tree, overlay));
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
    let root = mount(&mut tree, None, 0, UiNode::Stack(UiStackNode { direction: "column".into(), gap: None, padding: None, id: None, children: Vec::new(), presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, menu: None }), LayoutSpec::default());
    let slot = mount(
        &mut tree,
        Some(root),
        1,
        UiNode::ExternalSlot(crate::wgpu::component::ui::UiExternalSlotNode {
            plugin_id: "framework".into(),
            app_id: "shell".into(),
            body_key: "framework.chat.transcript".into(),
            params_json: format!("{{\"hostContentHeight\": {declared}}}"),
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
