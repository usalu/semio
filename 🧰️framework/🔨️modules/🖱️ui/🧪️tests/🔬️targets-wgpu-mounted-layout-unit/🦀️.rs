
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

#[test]
fn debug_sizes() {
    println!("[DEBUG] LayoutInputNode={} LayoutNodeKind={} MountedLayoutJob={} Theme={}", std::mem::size_of::<LayoutInputNode>(), std::mem::size_of::<LayoutNodeKind>(), std::mem::size_of::<MountedLayoutJob>(), std::mem::size_of::<Theme>());
}
