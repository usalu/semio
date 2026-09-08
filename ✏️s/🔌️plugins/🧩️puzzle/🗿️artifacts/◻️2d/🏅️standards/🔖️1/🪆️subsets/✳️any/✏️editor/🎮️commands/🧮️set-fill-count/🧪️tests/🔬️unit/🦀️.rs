
use super::*;

fn production_of(source: &str) -> &str {
    source.split("//#region 🧪️Tests").next().unwrap_or(source)
}

/// 🧵️ The session must stay inside the retained work: no process-global slot table, no worker
/// pool, no store lease, no self-dispatched continuation effect.
fn registry_free_session_source_contract(source: &str) -> bool {
    let production = production_of(source);
    let Some(start) = production.find("pub struct Puzzle2dFillSessionWork {") else { return false };
    let Some(end) = production[start..].find("impl Puzzle2dFillSessionWork {") else { return false };
    let owner = &production[start..start + end];
    owner.contains("search: Option<infinite_canvas::BoardFillJob>")
        && owner.contains("checkpoint: Option<infinite_canvas::BoardFillCheckpoint>")
        && owner.contains("apply: Option<FillPlacementApplyCursor>")
        && owner.contains("operation: semio_framework_job::Operation")
        && !owner.contains("app_instance_id")
        && !production.contains("AtomicPtr")
        && !production.contains("FILL_SESSION_SLOTS")
        && !production.contains("semio_framework_async::process_worker_pool")
        && !production.contains("MountedWorkerJobSession")
        && !production.contains("store::SnapshotRead")
        && !production.contains("Effect::DispatchAction")
        && production.contains("fn bind_operation(&mut self, operation: semio_framework_job::Operation)")
        && production.contains("semio_framework_job::StepContext::new(operation.operation, operation.generation, budget, semio_framework_job::root_cancel_token(), puzzle2d_fill_monotonic_zero, preview_sequence)")
        && production.contains("Puzzle2dConfigMutation::Fill { runtime }")
}

fn fixed_placement_owner_source_contract(source: &str) -> bool {
    let production = production_of(source);
    let Some(start) = production.find("struct FillPlacementApplyCursor {") else { return false };
    let Some(end) = production[start..].find("fn try_document_str") else { return false };
    let owners = &production[start..start + end];
    let Some(apply_start) = production.find("impl FillPlacementApplyCursor {") else { return false };
    let Some(apply_end) = production[apply_start..].find("impl Drop for FillPlacementApplyCursor") else { return false };
    let apply = &production[apply_start..apply_start + apply_end];
    let Some(view_start) = production.find("enum FillPlacementPublishHandles") else { return false };
    let Some(view_end) = production[view_start..].find("fn publish_fixed_placement") else { return false };
    let view = &production[view_start..view_start + view_end];
    owners.contains("edge_kind: infinite_canvas::BoardFillText")
        && owners.contains("text_byte: usize")
        && owners.contains("fn copy_fill_text_one")
        && owners.contains("destination.try_push_byte(value)")
        && !owners.contains("String")
        && !owners.contains("Vec<")
        && !owners.contains("BTreeMap")
        && !owners.contains("Puzzle2dNode")
        && !owners.contains("Puzzle2dHandle")
        && apply.matches("copy_fill_text_one(").count() == 9
        && apply.contains("fixed_handle_id(self.handle_cursor)")
        && apply.contains("fixed_handle_kind(self.handle_cursor)")
        && apply.contains("FillPlacementPublishView::from_cursor")
        && !apply.contains("let placement = infinite_canvas::BoardFillCommitPlacement")
        && !apply.contains("handles: std::array::from_fn(|index|")
        && !apply.contains("= placement.node_id;")
        && !apply.contains("= placement.edge_kind;")
        && !apply.contains(".to_string(")
        && view.contains("edge_kind: &'a infinite_canvas::BoardFillText")
        && view.contains("FillPlacementPublishHandles::Commit(&placement.handles)")
        && view.contains("FillPlacementPublishHandles::Cursor(handles)")
        && !view.contains("String")
        && !view.contains("Vec<")
        && !view.contains("BTreeMap")
}

fn full_terminal_candidate_source_contract(source: &str) -> bool {
    let production = production_of(source);
    let Some(start) = production.find("fn publish_commit_candidate") else { return false };
    let Some(end) = production[start..].find("impl FillPlacementApplyCursor") else { return false };
    let publish = &production[start..start + end];
    publish.contains("BoardFillCommitCandidate::from_commit_candidate(candidate)")
        && publish.contains("if let Some(placement) = candidate.placement.as_ref()")
        && publish.contains("publish_fixed_placement(&FillPlacementPublishView::from_commit(placement), mutations)")
        && publish.contains("Some(Ok(candidate.result))")
        && !publish.contains("BoardFillResult::from_commit_candidate")
        && !publish.contains("take_result")
}

fn granular_capture_source_contract(source: &str) -> bool {
    let production = production_of(source);
    let Some(start) = production.find("fn capture_node_one") else { return false };
    let Some(end) = production[start..].find("//#endregion 🔬️Capture") else { return false };
    let capture = &production[start..start + end];
    capture.matches(".get(").count() == 53
        && capture.matches("as_bytes().get(self.capture.byte)").count() == 7
        && capture.matches("self.ingress").count() == 39
        && capture.matches(".push_node_id_byte(").count() == 1
        && capture.matches(".push_handle_text_byte(").count() == 2
        && capture.matches(".push_kind_text_byte(").count() == 2
        && capture.matches(".push_kind_handle_text_byte(").count() == 1
        && capture.matches(".push_rule_text_byte(").count() == 1
        && !capture.contains("for ")
        && !capture.contains(".iter(")
        && !capture.contains(".clone(")
        && !capture.contains(".to_string(")
        && !capture.contains(".push_node(")
        && !capture.contains(".push_handle(")
        && !capture.contains(".push_rule(")
}

fn placement_publish_source_contract(source: &str) -> bool {
    let production = production_of(source);
    let Some(start) = production.find("fn publish_fixed_placement") else { return false };
    let Some(end) = production[start..].find("fn publish_commit_candidate") else { return false };
    let publish = &production[start..start + end];
    let Some(reserve) = publish.find("mutations.try_reserve_exact(2)") else { return false };
    let Some(handles) = publish.find("let mut handles = Vec::new()") else { return false };
    let Some(node) = publish.find("let node = crate::Puzzle2dNode") else { return false };
    reserve < handles && handles < node && publish.contains("Some(try_document_text(*placement.edge_kind)?)") && publish.matches("mutations.push(").count() == 2 && !production.contains("ReserveMutations")
}

/// 🧵️ Reintroducing the process-global slot registry, the worker pool, the store lease or the
/// self-dispatched continuation effect fails the live session-ownership law.
#[test]
fn registry_and_worker_pool_reintroductions_are_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(registry_free_session_source_contract(source));
    let registry = source.replacen("    closing: bool,\n}", "    closing: bool,\n    app_instance_id: u32,\n}", 1);
    assert!(!registry_free_session_source_contract(&registry));
    let pool = source.replacen("fn puzzle2d_fill_monotonic_zero", "fn pool() { semio_framework_async::process_worker_pool(); }\nfn puzzle2d_fill_monotonic_zero", 1);
    assert!(!registry_free_session_source_contract(&pool));
    let lease = source.replacen("    ingress: Option<infinite_canvas::BoardFillSnapshotIngress>,", "    lease: Option<store::SnapshotRead<Puzzle2dPlaySnapshot>>,\n    ingress: Option<infinite_canvas::BoardFillSnapshotIngress>,", 1);
    assert!(!registry_free_session_source_contract(&lease));
}

/// 🧷️ Injected dynamic retained placement text and re-coalesced fixed text both fail the placement ownership law.
#[test]
fn mounted_fill_fixed_placement_owner_mutations_are_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(fixed_placement_owner_source_contract(source));
    let dynamic = source.replacen("edge_kind: infinite_canvas::BoardFillText,", "edge_kind: String,", 1);
    assert!(!fixed_placement_owner_source_contract(&dynamic));
    let dynamic_view = source.replacen("edge_kind: &'a infinite_canvas::BoardFillText,", "edge_kind: String,", 1);
    assert!(!fixed_placement_owner_source_contract(&dynamic_view));
    let whole_text = source.replacen("if copy_fill_text_one(&placement.edge_kind, destination, &mut self.text_byte)? {", "if { *destination = placement.edge_kind; true } {", 1);
    assert!(!fixed_placement_owner_source_contract(&whole_text));
    let whole_candidate =
        source.replacen("FillPlacementPublishView::from_cursor(node, edge, &self.handles, self.handle_cursor)", "infinite_canvas::BoardFillCommitPlacement { handles: std::array::from_fn(|index| self.handles[index]), ..Default::default() }", 1);
    assert!(!fixed_placement_owner_source_contract(&whole_candidate));
}

/// 🔡️ A MAX fixed label advances by exactly one character byte per retained apply opportunity.
#[test]
fn mounted_fill_fixed_placement_text_cursor_is_one_byte_per_turn() {
    let source = "x".repeat(infinite_canvas::BOARD_FILL_TEXT_BYTES);
    let source = infinite_canvas::BoardFillText::try_from_str(&source).expect("MAX fixed placement text");
    let mut destination = infinite_canvas::BoardFillText::empty();
    let mut byte = 0usize;
    for index in 0..infinite_canvas::BOARD_FILL_TEXT_BYTES {
        let complete = copy_fill_text_one(&source, &mut destination, &mut byte).expect("fixed byte copy");
        assert_eq!(destination.as_str().len(), index + 1);
        assert_eq!(complete, index + 1 == infinite_canvas::BOARD_FILL_TEXT_BYTES);
    }
    assert_eq!(destination, source);
    let over = "x".repeat(infinite_canvas::BOARD_FILL_TEXT_BYTES + 1);
    assert!(infinite_canvas::BoardFillText::try_from_str(&over).is_err());
}

/// 📦️ Replacing the full exact terminal placement with a summary decoder fails the live terminal law.
#[test]
fn mounted_fill_summary_only_terminal_mutation_is_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(full_terminal_candidate_source_contract(source));
    let summary = source.replacen("BoardFillCommitCandidate::from_commit_candidate(candidate)", "BoardFillResult::from_commit_candidate(candidate)", 1);
    assert!(!full_terminal_candidate_source_contract(&summary));
    let discarded = source.replacen("if let Some(placement) = candidate.placement.as_ref() {", "if let Some(placement) = None {", 1);
    assert!(!full_terminal_candidate_source_contract(&discarded));
}

/// 🔬️ Re-coalescing source fields or whole text into one capture grant fails the live capture law.
#[test]
fn mounted_fill_capture_granularity_mutations_are_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(granular_capture_source_contract(source));
    let fields = source.replacen(
        "self.capture.node_x = Self::finite(node.get(\"x\").and_then(Value::as_f64), 0.0);",
        "self.capture.node_x = Self::finite(node.get(\"x\").and_then(Value::as_f64), 0.0); self.capture.node_y = Self::finite(node.get(\"y\").and_then(Value::as_f64), 0.0);",
        1,
    );
    assert!(!granular_capture_source_contract(&fields));
    let text = source.replacen("if let Some(byte) = value.as_bytes().get(self.capture.byte).copied() {", "for byte in value.as_bytes().iter().copied() {", 1);
    assert!(!granular_capture_source_contract(&text));
}

/// 🪪️ Placement publication pre-credits its live destination in the same bounded continuation.
#[test]
fn placement_publish_credit_mutation_is_rejected() {
    let source = include_str!("../../🦀️.rs");
    assert!(placement_publish_source_contract(source));
    let uncredited = source.replacen("mutations.try_reserve_exact(2)", "mutations.capacity().checked_add(2)", 1);
    assert!(!placement_publish_source_contract(&uncredited));
}

/// 🗂️ Fill's kind capture reads the document's own `meta.kindCatalogs.nodes` slice. It used to read
/// `nodeKinds` — the board *engine's* spelling, which the puzzle2d document schema forbids
/// (`additionalProperties: false` over `nodes`/`🐙️handles`/`edges`/`wires`) — so capture failed on
/// every run with `puzzle2d-fill-capture-node-kinds` and the job went straight to `Faulted`.
#[test]
fn fill_capture_reads_the_document_node_kind_slice() {
    let document = serde_json::json!({
        "meta": {
            "kindCatalogs": {
                "nodes": [{ "id": "seed", "name": "Seed", "label": "Seed", "handles": [] }],
                "handles": [],
                "edges": [],
                "wires": []
            }
        }
    });
    let kinds = Puzzle2dFillSessionWork::node_kinds(&document).expect("document node-kind slice");
    assert_eq!(kinds.len(), 1);
    assert_eq!(kinds[0].get("id").and_then(Value::as_str), Some("seed"));

    let engine_shaped = serde_json::json!({ "meta": { "kindCatalogs": { "nodeKinds": [{ "id": "seed" }] } } });
    assert_eq!(Puzzle2dFillSessionWork::node_kinds(&engine_shaped).err(), Some("puzzle2d-fill-capture-node-kinds"), "the engine's `nodeKinds` spelling must not satisfy fill's document read, else this guard proves nothing");
}

/// 🎛️ Every control verb is a pure runtime transition, and only the four search verbs ask for a
/// search — the property that lets `setActiveUtility` discard a session without reaching any
/// live owner.
#[test]
fn fill_control_verbs_are_pure_runtime_transitions() {
    let mut effects = Vec::new();
    let mut runtime = Puzzle2dFillRuntime::from_config(&Puzzle2dConfig::default());
    runtime.fill_count = 12;
    runtime.fill_job_accepted_count = 4;
    runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Running;
    assert_eq!(fill_session_control("brushFillSessionStep", None, &mut runtime, &mut effects), Ok(Some((8, 1))));
    assert!(effects.is_empty());

    runtime.fill_job_lifecycle = Puzzle2dFillLifecycle::Completed;
    assert_eq!(fill_session_control("brushFillSessionStep", None, &mut runtime, &mut effects), Ok(None));
    assert_eq!(runtime.fill_job_lifecycle, Puzzle2dFillLifecycle::Completed);

    assert_eq!(fill_session_control("brushFillSessionCancel", None, &mut runtime, &mut effects), Ok(None));
    assert_eq!(runtime.fill_job_lifecycle, Puzzle2dFillLifecycle::Cancelled);

    assert_eq!(fill_session_control("brushFillSessionClear", None, &mut runtime, &mut effects), Ok(None));
    assert_eq!(runtime.fill_job_lifecycle, Puzzle2dFillLifecycle::Discarded);
    assert_eq!(runtime.fill_count, 0);
    assert_eq!(runtime.fill_job_accepted_count, 0);

    assert_eq!(fill_session_control("setFillCount", Some(&serde_json::json!({ "count": 3 })), &mut runtime, &mut effects), Ok(Some((3, 1))));
    assert_eq!(effects.len(), 1);
    assert_eq!(fill_session_control("setFillCount", Some(&serde_json::json!({ "count": f64::from(fill::PUZZLE2D_FILL_COUNT_MAX) + 1.0 })), &mut runtime, &mut effects), Err("puzzle2d-fill-count-capacity"));
    assert_eq!(fill_session_control("setFillCount", None, &mut runtime, &mut effects), Err("puzzle2d-fill-count"));
    assert_eq!(fill_session_control("brushFillSessionBegin", Some(&serde_json::json!({ "maxCount": 2 })), &mut runtime, &mut effects), Err("puzzle2d-fill-start-seed"));
    assert_eq!(fill_session_control("brushFillSessionBegin", Some(&serde_json::json!({ "maxCount": 2, "seed": 9 })), &mut runtime, &mut effects), Ok(Some((2, 9))));
}

/// 📐️ A control verb declares four work items; a search verb declares its exact per-stage chunk
/// ceilings, and an over-count request is refused at preflight rather than mid-run.
#[test]
fn fill_session_extent_is_the_enforced_budget() {
    use crate::retained_command::PuzzleCommandWork;
    let snapshot = Puzzle2dPlaySnapshot(serde_json::json!({ "schema": crate::PUZZLE_2D_SCHEMA, "nodes": [], "edges": [] }));
    let interaction = protocol::InteractionState::default();
    let search_budget = PUZZLE2D_FILL_CAPTURE_CHUNKS + PUZZLE2D_FILL_SEARCH_CHUNKS + PUZZLE2D_FILL_APPLY_CHUNKS + PUZZLE2D_FILL_CONTROL_CHUNKS;
    assert!(search_budget <= crate::retained_command::PUZZLE_COMMAND_WORK_ITEMS);

    let cancel = Puzzle2dFillSessionWork::new("brushFillSessionCancel");
    let command = crate::editor::puzzle2d::Puzzle2dCommand::from_action("brushFillSessionCancel", None, None);
    assert_eq!(cancel.extent(&command, &snapshot, &interaction), Some(PUZZLE2D_FILL_CONTROL_CHUNKS));

    let fill = Puzzle2dFillSessionWork::new("setFillCount");
    let command = crate::editor::puzzle2d::Puzzle2dCommand::from_action("setFillCount", Some(serde_json::json!({ "count": 8 })), None);
    assert_eq!(fill.extent(&command, &snapshot, &interaction), Some(search_budget));

    let over = crate::editor::puzzle2d::Puzzle2dCommand::from_action("setFillCount", Some(serde_json::json!({ "count": u64::from(fill::PUZZLE2D_FILL_COUNT_MAX) + 1 })), None);
    assert_eq!(fill.extent(&over, &snapshot, &interaction), None);

    let foreign = crate::editor::puzzle2d::Puzzle2dCommand::from_action("addNode", None, None);
    assert_eq!(fill.extent(&foreign, &snapshot, &interaction), None);
}
