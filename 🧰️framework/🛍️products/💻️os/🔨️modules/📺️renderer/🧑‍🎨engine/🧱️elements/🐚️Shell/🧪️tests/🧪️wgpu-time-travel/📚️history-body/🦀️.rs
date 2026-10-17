/// 📚️ History body: the guest's real producer, paged transaction rows, refused edits, list and chip editors, replay progress and the single scroll region.

//#region 📚️HistoryBody
/// 🔢️ Operations of the law's one drag transaction, and how many of them its history row projects.
const HISTORY_BODY_OPERATIONS: u32 = 120;
const HISTORY_BODY_PROJECTED: u32 = 3;

/// 🧱️ Publishes the guest producer through the same bounded, parent/key reconciler as its runtime.
fn publish_history_body(current: &mut Option<semio_framework_ui_runtime::SurfaceReconciler>, built: ui_contract::BuiltNode, generation: u64) -> UiDocumentLease {
    use semio_framework_ui_runtime::{ComponentTree, SurfaceReconcileJob, SurfaceReconcileJobStep, SurfaceReconciler};
    let surface = SurfaceId::try_from(FRAMEWORK_PANEL_TAB_HISTORY_ID).expect("History surface");
    let mut job = SurfaceReconcileJob::try_new(current.take().unwrap_or_else(|| SurfaceReconciler::new(surface)), ComponentTree::new(built), generation).expect("history producer admission");
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut context_retained = semio_framework_job::RetainedCloneProgress::default();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, u64::MAX, UNBOUNDED_STEP_GRANT), semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us, &mut sequence,
            &mut context_retained,
        );
        match job.drive_one(&mut context) {
            SurfaceReconcileJobStep::Ready => break,
            SurfaceReconcileJobStep::Fault => panic!("history producer refused: {:?}", job.fault()),
            SurfaceReconcileJobStep::MoreWork => {}
        }
    }
    let mut patch = None;
    assert_eq!(job.take_ready_into(current, &mut patch, SurfaceReconcileJob::required_ready_transfer_bytes()), Ok(true), "history producer completes within its bound");
    if let Some(mut patch) = patch { while !patch.close_step() {} }
    let mut terminal = job.into_terminal();
    while !terminal.close_step() {}
    let mut document = None;
    assert_eq!(current.as_ref().expect("published reconciler").capture_document(&mut document, usize::MAX), Ok(true));
    document.expect("canonical history document")
}

/// 🧹️ Retires the fixture publisher after its renderer and document readers release their roots.
fn close_history_producer(current: &mut Option<semio_framework_ui_runtime::SurfaceReconciler>) {
    if let Some(current) = current.take() {
        let mut terminal = semio_framework_ui_runtime::SurfaceReconcileTerminal::try_from_reconciler(current, 1000).expect("publisher close admission");
        while !terminal.close_step() {}
    }
}

/// ✏️ Operation `index` of the drag transaction, editable.
fn drag_mutation(index: u32) -> MutationView {
    MutationView { mutation_id: format!("m-{index}"), position: 0, op_index: index, label: LocalizedLabel::native("Drag selection", "Auswahl ziehen"), worst: None, messages: Vec::new(), superseded: false, withdrawn: false, editable: true, withdrawable: true, store: None }
}

/// ⏳️ A running replay: `Begin` is refused (`Illegal`), so every Edit is disabled with its reason.
fn replaying_panel() -> TimeTravelPanel {
    TimeTravelPanel {
        status: session("a running replay"),
        store: None,
        stage: semio_framework_time_travel::TimeTravelStage::Replaying,
        pending_after: None,
        finalize_refusal: None,
        review: None,
        rerun_refusal: None,
        begin_refusal: Some(semio_framework_time_travel::TimeTravelRefusal::Illegal),
        next_problem: None,
        editor: None,
        outcomes: Default::default(),
        edited: Default::default(),
        accepted: Default::default(),
    }
}

/// 📋️ A session editing `m-1` whose draft holds a full list (`/points`: two items, `minItems` = `maxItems` = 2) and a
/// two-chip reference list (`/targets`, `minItems` 1, `n1` labelled "Corner"/"Ecke") — the N2 list rows.
fn list_editor_panel() -> TimeTravelPanel {
    let row = |pointer: &str, input: ActionArgDef, value: Value, list: Option<TimeTravelList>, item: Option<TimeTravelListItem>| TimeTravelInputRow { pointer: pointer.into(), label: input.label.clone(), input, value: DslValue::from(value), list, item };
    let point = |index: usize, value: f64| row(&format!("/points/{index}"), ActionArgDef::number("point", LocalizedLabel::native("Point", "Punkt")), serde_json::json!(value), None, Some(TimeTravelListItem { index, removable: false, min: Some(2) }));
    let targets = ActionArgDef {
        schema: ArgSchema::Reference { kinds: vec!["node".into()], domain: Some("vortex".into()), granularity: Some("node".into()), many: true, min_items: Some(1), max_items: None, id_type: semio_framework::ReferenceIdType::String },
        ..ActionArgDef::text("targets", LocalizedLabel::native("Targets", "Ziele"))
    };
    TimeTravelPanel {
        status: session("editing a mutation"),
        stage: semio_framework_time_travel::TimeTravelStage::Editing,
        begin_refusal: None,
        editor: Some(TimeTravelEditorPanel {
            target: "m-1".into(),
            label: LocalizedLabel::native("Drag selection", "Auswahl ziehen"),
            rows: vec![
                row("/points", ActionArgDef::text("points", LocalizedLabel::native("Points", "Punkte")), Value::Null, Some(TimeTravelList { len: 2, addable: false, max: Some(2) }), None),
                point(0, 1.0),
                point(1, 2.0),
                row("/targets", targets, serde_json::json!(["n1", "n2"]), None, None),
            ],
            editable: true,
            inputs_refused: None,
            withdrawn: false,
            outcome: Vec::new(),
            refused: None,
            changed: false,
            reference_labels: std::collections::BTreeMap::from([("n1".to_string(), LocalizedLabel::native("Corner", "Ecke"))]),
        }),
        ..replaying_panel()
    }
}

/// 📚️ The history body the guest publishes, built by its REAL producer `ui_history_panel`: one transaction row of
/// [`HISTORY_BODY_OPERATIONS`] operations ([`HISTORY_BODY_PROJECTED`] projected), the host's window `(offset, rows)` over that
/// row with the store's page past the projection, the session `panel` when a history edit is open, and the history step
/// still replaying (`reprojection`).
fn history_body(current: &mut Option<semio_framework_ui_runtime::SurfaceReconciler>, window: Option<(u32, u32)>, panel: Option<TimeTravelPanel>, reprojection: Option<HistoryReprojection>, locale: Locale, generation: u64) -> UiDocumentLease {
    let history = HistoryView {
        commands: vec![CommandView {
            seq: 1,
            action_id: "drag".into(),
            label: LocalizedLabel::native("Drag selection", "Auswahl ziehen"),
            kind: semio_framework::ActionKind::Mutation,
            timestamp: "2026-10-02T10:00:00Z".into(),
            edit_id: Some("e-1".into()),
            child_edit_ids: Vec::new(),
            transition_id: None,
            author: None,
            op_lines: Vec::new(),
            op_count: HISTORY_BODY_OPERATIONS as usize,
            applied: true,
            revertible: false,
            count: 1,
            inverse: None,
            transaction: None,
            mutations: (0..HISTORY_BODY_PROJECTED).map(drag_mutation).collect(),
        }],
        ..HistoryView::empty()
    };
    let mut pages = HistoryMutationPages::new();
    let mut view = ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native);
    if let Some((offset, rows)) = window {
        let start = offset.min(HISTORY_BODY_OPERATIONS.saturating_sub(rows));
        let paged = start.max(HISTORY_BODY_PROJECTED);
        pages.insert(1, HistoryMutationPage { from: (paged - HISTORY_BODY_PROJECTED) as usize, rows: (paged..(start + rows).min(HISTORY_BODY_OPERATIONS)).map(drag_mutation).collect() });
        view.tree_windows = vec![semio_framework::TreeWindowRequest { body_key: semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY.into(), node_key: history_row_window_path(1), open: Some(true), offset, rows }];
    }
    let built = semio_framework_async::block_on(ui_history_panel(&history, panel.as_ref(), reprojection.as_ref(), &pages, "s.test.history", locale, false, false, &view)).expect("the guest's history body assembles");
    let viewport: Value = serde_json::from_str(include_str!("../../../../../../../🔌️plugin/🧪️tests/🧫️fixtures/history-panel-command-window/🔣️.json")).expect("the neutral history viewport fixture");
    assert_eq!(serde_json::to_value(&built.layout).expect("history viewport layout"), viewport["rootLayout"]);
    publish_history_body(current, built, generation)
}

/// 🔑️ Reads the canonical identity a retained document assigns to an authored key.
fn history_document_node_id(document: &UiDocumentLease, key: &str) -> ui_contract::UiNodeId {
    let read = document.try_read().expect("history document read");
    (0..read.len()).find_map(|ordinal| read.node_at(ordinal).filter(|record| record.key.as_str() == key).map(|record| record.id)).expect("history document contains key")
}

/// ♿️ The History surface's mirror node whose key names `key`.
fn history_mirror_node(key: &str) -> Option<ui_contract::AccessibilityProjectionNode> {
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_nodes_for_test(FRAMEWORK_PANEL_TAB_HISTORY_ID).into_iter().find(|node| projected_key_is(&node.key, key))
}

/// 📜️ The mutation rows the History surface's mirror shows, in reading order.
fn history_mirror_mutations() -> Vec<String> {
    semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_nodes_for_test(FRAMEWORK_PANEL_TAB_HISTORY_ID)
        .into_iter()
        .filter_map(|node| node.key.rsplit('/').next().and_then(|key| key.strip_prefix("framework.history.mutation.")).filter(|id| !id.contains("::")).map(str::to_owned))
        .collect()
}

/// 🛞️ Paints `document` and wheels the History body down over its rows, a few rows a notch, until the mutation row `wanted`
/// is projected — the person scrolling to a window the tree-window observer asked for.
fn wheel_history_window_into_view(shell: &mut ShellState, surface: &str, document: &UiDocumentLease, body: Rect, wanted: &str) {
    let mut input = paint_retained_body(shell, surface, document, body);
    for _ in 0..256 {
        if history_mirror_mutations().iter().any(|id| id == wanted) {
            return;
        }
        let (x, y, row_h) = input.hits().iter().find(|hit| hit.kind == HitKind::TreeItem && body.contains(hit.rect.x + 1.0, hit.rect.y + 1.0)).map_or((body.x + body.w * 0.5, body.y + body.h * 0.5, 24.0), |hit| (hit.rect.x + hit.rect.w * 0.5, hit.rect.y + hit.rect.h * 0.5, hit.rect.h));
        assert!(shell.input.handle_pointer_wheel(x, y, 0.0, row_h * 3.0, &mut input), "the History body owns the wheel");
        input = paint_retained_body(shell, surface, document, body);
    }
    panic!("`{wanted}` never scrolled into the History body: windows={:?} shown={:?}", semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::tree_window_measures(surface), history_mirror_mutations());
}

/// 🖱️ Activates one History mirror node the way the ARIA mirror's click does; answers what it dispatched.
fn activate_history_mirror_node(shell: &mut ShellState, key: &str) -> Vec<ActionDescriptor> {
    let node = history_mirror_node(key).unwrap_or_else(|| panic!("`{key}` is in the mirror"));
    let target = semio_framework_os_renderer_wgpu_ui_engine_dump::chrome_ledger::published_accessibility_target_for_test(FRAMEWORK_PANEL_TAB_HISTORY_ID, &node.key).expect("a mirror node is an accessibility target");
    let mut input = InputState::<ActionDescriptor>::default();
    semio_framework_async::block_on(PointerCtx::of(&mut shell).handle_accessibility_event(&target, &ui_render::AccessibilityEvent::Activate, &mut input)).expect("the activation is handled");
    semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions(&mut input)
}

/// ⚖️ LAW (gaps N1 + N15 on wgpu, through the guest's REAL producer and the ARIA mirror): a closed transaction row of 120
/// operations announces that it folds open (its window's total) and opens by the mirror's activation; the wgpu tree-window
/// observer then asks the guest for that row by the SAME window path the guest reads
/// (`framework.history.commands␟framework.history.entry.1`), and the slice it answers renders past the projected
/// mutations; a window at 100, wheeled into view (the mirror projects what the body shows), shows exactly operations 100–107.
/// While a replay runs, every Edit is an
/// aria-disabled button that tells the reason ("Not possible right now" / "Derzeit nicht möglich", in its name or its
/// description) and activating it or its row begins nothing; with Begin allowed the same button begins an edit of its mutation.
#[test]
fn the_guest_history_body_opens_pages_and_refuses_edit_with_its_reason() {
    let mut producer = None;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    let row = "framework.history.entry.1";
    let mut shell = session_shell();
    let mut documents = vec![history_body(&mut producer, None, None, None, Locale::En, 1)];
    let _ = paint_retained_body(&mut shell, surface, &documents[0], body);
    let closed = history_mirror_node(row).expect("the transaction row is in the mirror");
    assert_eq!(closed.expanded, Some(false), "a closed row of {HISTORY_BODY_OPERATIONS} operations announces it folds open, never a leaf: {closed:?}");
    assert!(history_mirror_mutations().is_empty(), "a closed row materialises no mutation");

    let _ = activate_history_mirror_node(&mut shell, row);
    let _ = paint_retained_body(&mut shell, surface, &documents[0], body);
    assert_eq!(history_mirror_node(row).and_then(|node| node.expanded), Some(true), "the mirror's activation opens the row");
    shell.dock.observe_tree_windows(surface, &shell.docs);
    let (requests, _) = shell.dock.tree_windows.view_state_fields();
    let path = history_row_window_path(1);
    let asked = requests.iter().find(|request| request.body_key == semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY && request.node_key == path).cloned().unwrap_or_else(|| panic!("the opened row is asked for by the guest's window path {path:?}: {requests:?}"));
    assert!(asked.open == Some(true) && asked.rows > HISTORY_BODY_PROJECTED, "the request opens the row over more than its projection: {asked:?}");

    documents.push(history_body(&mut producer, Some((asked.offset, asked.rows)), None, None, Locale::En, 2));
    let _ = paint_retained_body(&mut shell, surface, &documents[1], body);
    let shown = history_mirror_mutations();
    assert!(shown.len() > HISTORY_BODY_PROJECTED as usize, "the answered window renders past the projected mutations, paged from the store: {shown:?}");
    assert_eq!(shown, (0..shown.len() as u32).map(|index| format!("m-{index}")).collect::<Vec<_>>(), "in op order from the first");

    for (generation, locale, edit_word, reason) in [(3, Locale::En, "Edit", "Not possible right now"), (4, Locale::De, "Bearbeiten", "Derzeit nicht möglich")] {
        documents.push(history_body(&mut producer, Some((100, 8)), Some(replaying_panel()), None, locale, generation));
        assert_eq!(history_document_node_id(documents.last().expect("published"), row), history_document_node_id(&documents[1], row), "an inserted editor retains the opened transaction's canonical identity");
        wheel_history_window_into_view(&mut shell, surface, documents.last().expect("published"), body, "m-100");
        assert_eq!(history_mirror_mutations(), (100..108).map(|index| format!("m-{index}")).collect::<Vec<_>>(), "{locale:?}: the scrolled window shows exactly the operations it asked for");
        for index in 100..108 {
            let edit = history_mirror_node(&format!("framework.history.mutation.m-{index}::row-action::0")).unwrap_or_else(|| panic!("{locale:?}: m-{index}'s Edit is in the mirror"));
            assert!(edit.role == "button" && edit.disabled && !edit.actionable, "{locale:?}: a refused Edit is an aria-disabled button: {edit:?}");
            let told = [edit.label.as_deref(), edit.description.as_deref()].into_iter().flatten().collect::<Vec<_>>().join(" · ");
            assert!(told.contains(edit_word) && told.contains(reason), "{locale:?}: it is named Edit and tells the reason: {told:?}");
        }
        let begun = [activate_history_mirror_node(&mut shell, "framework.history.mutation.m-100::row-action::0"), activate_history_mirror_node(&mut shell, "framework.history.mutation.m-100")].concat();
        assert!(!begun.iter().any(|action| action.action == semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID), "{locale:?}: neither the refused Edit nor its row begins an edit: {begun:?}");
    }

    documents.push(history_body(&mut producer, Some((100, 8)), None, None, Locale::En, 5));
    wheel_history_window_into_view(&mut shell, surface, documents.last().expect("published"), body, "m-100");
    let edit = history_mirror_node("framework.history.mutation.m-100::row-action::0").expect("m-100's Edit is in the mirror");
    assert!(!edit.disabled && edit.actionable && edit.label.as_deref().is_some_and(|label| label.starts_with("Edit:")), "Begin allowed: Edit is an enabled button: {edit:?}");
    let begun = activate_history_mirror_node(&mut shell, "framework.history.mutation.m-100::row-action::0");
    assert_eq!(begun.iter().map(|action| (action.action.as_str(), args_of(action)["mutationId"].clone())).collect::<Vec<_>>(), [(semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID, serde_json::json!("m-100"))], "and begins an edit of its own mutation");
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}

/// ⚖️ LAW (gap N2 on wgpu, through the guest's REAL producer and the ARIA mirror): a full list's "Add item" is a disabled
/// button and its row names the count and the ceiling; an item at the floor keeps a disabled "Remove item" whose row names
/// the floor; a reference list above its floor shows one removable chip per id, named by the entity's label (else its
/// id), and a chip's activation drafts `historyEditInput{path: "/targets/<i>", edit: "remove"}` — in English and German;
/// the disabled controls dispatch nothing.
#[test]
fn the_guest_editor_offers_list_and_chip_edits_within_their_bounds() {
    let mut producer = None;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 900.0);
    let mut shell = session_shell();
    let mut documents = Vec::new();
    for (generation, locale, add, count, remove, floor, chips) in [
        (1, Locale::En, "Add item", "Items: 2 · Maximum 2 items", "Remove item", "Minimum 2 items", ["Remove Corner", "Remove n2"]),
        (2, Locale::De, "Element hinzufügen", "Elemente: 2 · Höchstens 2 Einträge", "Element entfernen", "Mindestens 2 Einträge", ["Entfernen Ecke", "Entfernen n2"]),
    ] {
        documents.push(history_body(&mut producer, None, Some(list_editor_panel()), None, locale, generation));
        let _ = paint_retained_body(&mut shell, surface, documents.last().expect("published"), body);
        let node = |key: &str| history_mirror_node(key).unwrap_or_else(|| panic!("{locale:?}: `{key}` is in the mirror"));
        let add_button = node("framework.history.editor.input.points.add");
        assert!(add_button.role == "button" && add_button.disabled && add_button.label.as_deref() == Some(add), "{locale:?}: a full list's Add item is disabled: {add_button:?}");
        assert!(node("framework.history.editor.input.points.row").description.as_deref().is_some_and(|description| description.contains(count)), "{locale:?}: the list row names its count and ceiling: {:?}", node("framework.history.editor.input.points.row"));
        for index in 0..2 {
            let button = node(&format!("framework.history.editor.input.points.{index}.remove"));
            assert!(button.disabled && button.label.as_deref() == Some(remove), "{locale:?}: an item at the floor keeps a disabled Remove item: {button:?}");
            assert_eq!(node(&format!("framework.history.editor.input.points.{index}.remove.row")).description.as_deref(), Some(floor), "{locale:?}: its row names the floor");
        }
        for (index, chip) in chips.iter().enumerate() {
            let button = node(&format!("framework.history.editor.input.targets.chip.{index}"));
            assert!(button.role == "button" && !button.disabled && button.actionable && button.label.as_deref() == Some(*chip), "{locale:?}: a removable chip named by its entity: {button:?}");
        }
        assert!(activate_history_mirror_node(&mut shell, "framework.history.editor.input.points.add").is_empty(), "{locale:?}: the disabled Add item dispatches nothing");
        let removed = activate_history_mirror_node(&mut shell, "framework.history.editor.input.targets.chip.0");
        assert_eq!(
            removed.iter().map(|action| (action.action.as_str(), args_of(action)["path"].clone(), args_of(action)["edit"].clone())).collect::<Vec<_>>(),
            [(semio_framework::history_edit::HISTORY_EDIT_INPUT_ACTION_ID, serde_json::json!("/targets/0"), serde_json::json!(semio_framework::history_edit::HISTORY_EDIT_INPUT_REMOVE))],
            "{locale:?}: a chip drafts the removal of its own item"
        );
    }
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}

/// ⚖️ LAW (gap N17 + the stepped document load on wgpu, the REAL producer): a history change replaying is the body's
/// `framework.history.reprojection` section by its `kind` — this replica's own history step ("History step", "Replaying
/// history: 12 of 400 steps") and a whole-document load ("Document load", "Loading document: 12 of 400") — with an
/// enabled Cancel replay that sends `historyEditCancelReplay` without a session generation (the runtime drops the step or
/// the load with zero trace), and a refused step reads "History step refused: <reason>" with no control, in English and
/// German.
#[test]
fn a_replaying_history_step_or_document_load_shows_its_progress_and_cancels_in_the_mirror() {
    let mut producer = None;
    use semio_framework::kernel::HistoryReprojectionKind as Kind;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    let mut shell = session_shell();
    let mut documents = Vec::new();
    let replaying = |kind: Kind| HistoryReprojection { done: 12, total: 400, processed: None, kind, paused: false, fault: None };
    let cases = [
        (Locale::En, Kind::Step, "History step", "Replaying history: 12 of 400 steps", "Cancel replay"),
        (Locale::De, Kind::Step, "Verlaufsschritt", "Verlauf wird neu angewendet: 12 von 400 Schritten", "Neuanwendung abbrechen"),
        (Locale::En, Kind::Load, "Document load", "Loading document: 12 of 400", "Cancel replay"),
        (Locale::De, Kind::Load, "Dokument laden", "Dokument wird geladen: 12 von 400", "Neuanwendung abbrechen"),
    ];
    for (generation, (locale, kind, title, progress, cancel)) in (1..).zip(cases) {
        documents.push(history_body(&mut producer, None, None, Some(replaying(kind)), locale, generation));
        let _ = paint_retained_body(&mut shell, surface, documents.last().expect("published"), body);
        let node = |key: &str| history_mirror_node(key).unwrap_or_else(|| panic!("{locale:?} {kind:?}: `{key}` is in the mirror"));
        assert_eq!(node("framework.history.reprojection").label.as_deref(), Some(title), "{locale:?} {kind:?}: the section names what replays");
        assert_eq!(node("framework.history.reprojection.status").label.as_deref(), Some(progress), "{locale:?} {kind:?}: its progress in words");
        let button = node("framework.history.reprojection.cancelReplay");
        assert!(button.role == "button" && button.actionable && !button.disabled && button.label.as_deref() == Some(cancel), "{locale:?} {kind:?}: Cancel replay is an enabled button: {button:?}");
        let sent = activate_history_mirror_node(&mut shell, "framework.history.reprojection.cancelReplay");
        assert_eq!(sent.iter().map(|action| (action.action.as_str(), args_of(action).get("generation").cloned())).collect::<Vec<_>>(), [(semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, None)], "{locale:?} {kind:?}: it cancels, addressing no session");
    }
    for (generation, locale, refusal) in [(10, Locale::En, "History step refused: "), (11, Locale::De, "Verlaufsschritt abgelehnt: ")] {
        documents.push(history_body(&mut producer, None, None, Some(HistoryReprojection { done: 0, total: 0, processed: None, kind: Kind::Step, paused: false, fault: Some("timeTravel.blocked".into()) }), locale, generation));
        let _ = paint_retained_body(&mut shell, surface, documents.last().expect("published"), body);
        let status = history_mirror_node("framework.history.reprojection.status").and_then(|node| node.label).unwrap_or_default();
        assert!(status.starts_with(refusal) && status.len() > refusal.len(), "{locale:?}: a refused step names its reason: {status:?}");
        assert!(history_mirror_node("framework.history.reprojection.cancelReplay").is_none(), "{locale:?}: a refused step offers no control");
    }
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}

/// ⚖️ LAW (probe readiness, `verify time-travel --renderer wgpu` `scrollHistory`): a panel's retained body registers its whole
/// content as the scroll region `framework.panel.history.scroll` beneath its row hits, owned by the History surface, so a wheel
/// over the gap below the last row scrolls the body (React's overflow container) and `dumpChrome` names the region by kind and
/// owner.
#[test]
fn a_panel_body_is_one_scroll_region_under_its_rows() {
    let mut producer = None;
    let surface = FRAMEWORK_PANEL_TAB_HISTORY_ID;
    let body = Rect::new(0.0, 0.0, 420.0, 640.0);
    let mut shell = session_shell();
    let documents = vec![history_body(&mut producer, None, None, None, Locale::En, 1)];
    let mut input = paint_retained_body(&mut shell, surface, &documents[0], body);
    let region = format!("{surface}.scroll");
    let hits = input.hits().to_vec();
    let index = hits.iter().position(|hit| hit.control_id.as_deref() == Some(region.as_str())).expect("the panel's scroll region is registered");
    assert!(hits[index].kind == HitKind::ScrollRegion && hits[index].rect == body, "the whole content rect: {:?}", hits[index]);
    assert!(hits[index + 1..].iter().any(|hit| hit.kind == HitKind::TreeItem), "its rows register above it");
    assert_eq!(shell.input.retained_hit_windows.get(&region).map(|(owner, _)| owner.as_str()), Some(surface), "owned by the History surface");
    let gap = (body.x + body.w * 0.5, body.y + body.h - 4.0);
    assert_eq!(input.hit_at(gap.0, gap.1).and_then(|hit| hit.control_id.clone()).as_deref(), Some(region.as_str()), "the gap below the rows is the region");
    assert!(shell.input.handle_pointer_wheel(gap.0, gap.1, 0.0, 40.0, &mut input), "a wheel over the gap scrolls the body");
    close_retained_body(surface, documents);
    close_history_producer(&mut producer);
}
//#endregion 📚️HistoryBody
