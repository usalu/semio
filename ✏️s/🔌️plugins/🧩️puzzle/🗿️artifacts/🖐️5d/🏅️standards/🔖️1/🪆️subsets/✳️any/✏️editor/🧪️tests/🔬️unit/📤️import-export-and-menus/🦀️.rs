//! 📤️ Example, export, import, dialog, context menu, paste and part kind tests of the puzzle 5d editor. See [`super`].
use crate::editor::puzzle5d::entities::puzzle5d_grip_full_id;

use super::*;

//#region 📤️📥️DocumentIO
/// 📚️ LAW (diagnosis-first): the three SHIPPED examples really carry puzzle 5d content. Every other
/// law in this region, and most of the clipboard, dialog and context-menu laws, read a shipped
/// document — so when the examples are empty they all fail at once and none of them names WHY.
///
/// 🧊️ `Puzzle5dDocument` takes `#[serde(default)]` on `parts`/`fasteners` and ignores unknown members,
/// so a document of the WRONG artifact shape (puzzle 3d's `objects`/`attractions`) deserializes
/// silently into an empty 5d document instead of failing. This law is the one place that refuses it.
#[test]
fn every_shipped_example_document_really_carries_its_content() {
    for (id, document, parts, fasteners, catalogs) in [
        (PUZZLE5D_EXAMPLE_CONCRETE_FOREST, concrete_forest_example_document(), 1, 0, false),
        (PUZZLE5D_EXAMPLE_NAKAGIN, nakagin_example_document(), 180, 179, true),
        (PUZZLE5D_EXAMPLE_CAPSULE_DREAM, capsule_dream_example_document(), 2880, 2864, true),
    ] {
        assert_eq!(document.schema, PUZZLE5D_SCHEMA, "{id} declares the puzzle 5d schema");
        assert!(document.label.as_deref().is_some_and(|label| !label.is_empty()), "{id} carries a label (the export filename is its slug)");
        assert_eq!(document.parts.len(), parts, "{id} carries its own part census — a wrong count means the example asset is not this artifact's document");
        assert_eq!(document.fasteners.len(), fasteners, "{id} carries its own fastener census");
        assert_eq!(document.kind_catalogs.is_some(), catalogs, "{id} authors kindCatalogs exactly as its shipped asset does");
    }
}

/// 📤️📥️ LAW: exporting a document and importing those exact bytes back yields a BYTE-IDENTICAL
/// document, for every shipped example — including the one above the transport budget, because the
/// projection and the import decode are the same law the transport only carries. The decode reads the
/// whole file the framework hands the import (`puzzle5d_decode_document`).
#[test]
fn export_import_round_trips_every_shipped_example_byte_for_byte() {
    for document in [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()] {
        // 🚦️ Non-vacuity: an EMPTY document round-trips trivially, so this law would pass while proving
        // nothing the moment the shipped examples regress (see the diagnosis law above).
        assert!(!document.parts.is_empty(), "a round trip over an empty document proves nothing");
        let exported = export_snapshot::puzzle5d_export_json(&document);
        let root = import_snapshot::puzzle5d_decode_document(&exported).expect("the whole export decodes as a puzzle 5d document");
        let reimported: Puzzle5dDocument = crate::editor::puzzle5d::puzzle5d_record_from_projection(root).expect("the decoded root is a puzzle 5d document");
        assert_eq!(export_snapshot::puzzle5d_export_json(&reimported), exported, "{} did not round-trip byte-for-byte", document.label.clone().unwrap_or_default());
    }
}

/// 📤️ LAW: the export lane is chosen by payload size, not by guesswork — inline under one guest wire
/// page, segmented above it, and a NOTICE above what one segmented download may carry. The three
/// shipped examples are deliberately one case each.
#[test]
fn export_publication_picks_its_lane_by_payload_size() {
    let inline_budget = export_snapshot::puzzle5d_export_inline_budget_bytes();
    let segmented_budget = export_snapshot::puzzle5d_export_segmented_budget_bytes().expect("segmented budget");
    assert!(inline_budget < segmented_budget, "the inline page must be the narrower of the two budgets");
    for document in [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()] {
        let bytes = export_snapshot::puzzle5d_export_json(&document).len();
        let publication = export_snapshot::puzzle5d_export_publication(&document).expect("publication resolves");
        let arm = match publication {
            export_snapshot::Puzzle5dExportPublication::Inline(_) => "inline",
            export_snapshot::Puzzle5dExportPublication::Segmented(_) => "segmented",
            export_snapshot::Puzzle5dExportPublication::Refused(_) => "refused",
        };
        let expected = if bytes <= inline_budget {
            "inline"
        } else if bytes <= segmented_budget {
            "segmented"
        } else {
            "refused"
        };
        assert_eq!(arm, expected, "{} is {bytes} B", document.label.clone().unwrap_or_default());
    }
    // 🗂️ Concrete Forest is the inline case and Capsule Dream the refusal case; if either stopped being
    // so the budgets moved, and this law would otherwise pass while proving nothing.
    assert!(export_snapshot::puzzle5d_export_json(&concrete_forest_example_document()).len() <= inline_budget, "Concrete Forest is the inline case");
    assert!(export_snapshot::puzzle5d_export_json(&capsule_dream_example_document()).len() > segmented_budget, "Capsule Dream is the refusal case");
}

/// 📤️ LAW: the download filename is the document's own label as a slug, so exporting two examples
/// never lands as two files with one name; a document with no label keeps the app-generic name.
#[test]
fn export_filename_follows_the_document_label() {
    assert_eq!(export_snapshot::puzzle5d_export_filename(&concrete_forest_example_document()), "concrete-forest.json");
    assert_eq!(export_snapshot::puzzle5d_export_filename(&nakagin_example_document()), "nakagin-capsule-tower.json");
    assert_eq!(export_snapshot::puzzle5d_export_filename(&capsule_dream_example_document()), "capsule-dream.json");
    assert_eq!(export_snapshot::puzzle5d_export_filename(&empty_document()), "puzzle-5d.json");
}

/// 📥️ LAW: a chunked pick goes through the SDK's own action dispatch: every chunk but the last is staged by
/// the framework (`semio_framework::kernel::ImportStaging`) and edits nothing, and the closing chunk hands
/// `importSnapshot` the whole file, which lands as ONE whole-document load (no mutation, no history row).
#[semio_framework_async_macros::async_test]
async fn a_chunked_pick_lands_as_one_whole_document_load_through_the_framework_staging() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": "" })), None).expect("empty document");
    assert_eq!(part_count(&app), 0);
    let (target, chunks) = [concrete_forest_example_document(), nakagin_example_document()]
        .into_iter()
        .find_map(|document| {
            let chunks = semio_framework::kernel::import_payload_chunks(&export_snapshot::puzzle5d_export_json(&document));
            (chunks.len() > 1).then_some((document, chunks))
        })
        .expect("a shipped document whose export spans more than one host chunk");
    let mut loaded = None;
    for chunk in &chunks {
        let result = dispatch_through_action(&mut app, "importSnapshot", &semio_framework::kernel::import_chunk_arguments("chunked.json", chunk, None)).expect("import chunk");
        assert!(result.mutations.is_empty(), "chunk {} of {} publishes no mutation", chunk.chunk, chunk.chunk_count);
        if chunk.chunk + 1 < chunk.chunk_count {
            assert_eq!(part_count(&app), 0, "chunk {} of {} must leave the document untouched", chunk.chunk, chunk.chunk_count);
        } else {
            loaded = result.requested_effects.iter().find_map(|effect| match effect {
                Effect::LoadDocument { pack, .. } => Some(pack.clone()),
                _ => None,
            });
        }
    }
    let pack = loaded.expect("the closing chunk publishes the whole-document load");
    let document = <crate::Puzzle5dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&pack).expect("the load carries a puzzle 5d document");
    assert_eq!(document.parts.len(), target.parts.len(), "the closing chunk loaded the whole document");
    close_app(&mut app);
}

/// 📥️ LAW: every refusal answers. A file over the import budget and a payload that is not a puzzle 5d
/// document each publish a named notice and leave the document exactly as it was — never a silent no-op,
/// and never a fault.
#[semio_framework_async_macros::async_test]
async fn every_refused_import_publishes_a_notice_and_changes_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let oversized = "x".repeat(crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES + 1);
    let cases = [
        // 📦️ One byte above what one export may stream.
        ("capacity", semio_framework_pack_json::json!({ "payload": oversized.as_str(), "name": "huge.json" })),
        // 🔤️ A whole file that is not a puzzle 5d document.
        ("payload", semio_framework_pack_json::json!({ "payload": "{\"schema\":\"note.v1\",\"body\":\"\"}", "name": "note.json" })),
    ];
    for (case, args) in cases {
        let result = dispatch(&mut app, "importSnapshot", Some(&args), None).expect("a refused import answers, it never faults");
        assert!(result.mutations.is_empty(), "the {case} refusal emits no mutation");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::Notify { .. })), "the {case} refusal is visible: {:?}", result.requested_effects);
        assert_eq!(projection_of(&app), before, "the {case} refusal leaves the document alone");
    }
    close_app(&mut app);
}

/// 🕳️ LAW: a chunk that does not continue its pick is refused BY THE FRAMEWORK with the typed
/// `file-import.gap` fault — the staging never resumes into bytes nobody can account for — and the
/// document stays exactly as it was.
#[semio_framework_async_macros::async_test]
async fn a_gap_in_a_chunked_pick_is_a_typed_framework_refusal() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let chunk = semio_framework::kernel::ImportChunk { payload: "\"parts\":[],".into(), chunk: 1, chunk_count: 3 };
    let fault = dispatch_through_action(&mut app, "importSnapshot", &semio_framework::kernel::import_chunk_arguments("gap.json", &chunk, None)).expect_err("a chunk past the cursor of no open run is refused");
    assert_eq!(fault.code.0, semio_framework::kernel::ImportStagingRefusal::Gap.code(), "the refusal is the framework's typed gap code: {fault:?}");
    assert_eq!(projection_of(&app), before, "a refused chunk leaves the document alone");
    close_app(&mut app);
}

/// 🗂️ LAW: `openImportSnapshot` asks the HOST for a file and edits nothing — the picker re-dispatches
/// `importSnapshot`, which is the verb that owns the edit.
#[semio_framework_async_macros::async_test]
async fn open_import_snapshot_requests_a_file_and_edits_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, "openImportSnapshot", None, None).expect("openImportSnapshot");
    assert!(result.mutations.is_empty(), "a file picker is not a document edit");
    assert_eq!(projection_of(&app), before);
    let requested = result
        .requested_effects
        .iter()
        .find_map(|effect| match effect {
            Effect::RequestFileOpen { import_action, accept, multiple, .. } => Some((import_action.clone(), accept.clone(), *multiple)),
            _ => None,
        })
        .expect("openImportSnapshot requests a file open");
    assert_eq!(requested.0, "importSnapshot", "the picker re-dispatches the import verb");
    assert!(requested.1.contains("json"));
    assert!(!requested.2, "one document at a time");
    close_app(&mut app);
}
//#endregion 📤️📥️DocumentIO

//#region 🗨️AddPartDialog
/// 🗨️ LAW: `openAddPartDialog` opens the DECLARED dialog and edits nothing; the dialog submits the
/// existing `addPartKind` operation, so the dialog is a form over a verb, never a second verb.
#[semio_framework_async_macros::async_test]
async fn open_add_part_dialog_opens_the_declared_dialog_and_edits_nothing() {
    let mut app = app_with_registry();
    let before = projection_of(&app);
    let result = dispatch(&mut app, "openAddPartDialog", None, None).expect("openAddPartDialog");
    assert!(result.mutations.is_empty(), "opening a dialog is not a document edit");
    assert_eq!(projection_of(&app), before);
    let opened = result.requested_effects.iter().find_map(|effect| match effect {
        Effect::OpenDialog { dialog_id, .. } => Some(dialog_id.clone()),
        _ => None,
    });
    assert_eq!(opened.as_deref(), Some(open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG));
    let definition = create_puzzle5d_app();
    let dialog = definition.dialogs.iter().find(|dialog| dialog.id == open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG).expect("the opened dialog is declared");
    assert_eq!(dialog.submit_action.as_str(), "addPartKind", "the dialog submits the existing add-part verb");
    close_app(&mut app);
}

/// 📇️ Every `ActionDefinition` the built manifest carries — bare app actions are cloned onto every
/// window kind by `build_definition`, so one window's set is the whole declared vocabulary.
/// 🕹️ Every action DISPATCHABLE in this app, once per id.
///
/// 🪪️ App-level actions are no longer CLONED onto every window kind (that made a package descriptor
/// grow as `apps × window kinds × actions`): `semio_framework::window_kind_actions` resolves a
/// window's own roster PLUS every app-level row no window claims, at read time. Reading
/// `WindowKindDefinition::actions` directly therefore sees only this app's framework-injected
/// interaction verbs, and none of the authored vocabulary — which is why `addPartKind`,
/// `openAddPartDialog`, `setActiveExample` and the rest "vanished" from these laws. Same fix
/// `🧊️3d`'s `dispatchable_actions` carries, for the same framework change.
pub(super) fn declared_actions(definition: &semio_framework_plugin::AppDefinition) -> Vec<&semio_framework_plugin::ActionDefinition> {
    let mut seen = std::collections::BTreeSet::new();
    definition
        .window_kinds
        .iter()
        .flat_map(|window| semio_framework::window_kind_actions(definition, window))
        .filter(|action| seen.insert(action.id.clone()))
        .collect()
}

/// 🗨️ LAW: the dialog enumerates LIVE part kinds from the shipped documents' own `kindCatalogs` —
/// every option is a kind some document really declares, the literal placeholder is gone, and the
/// dialog and the standalone arg form offer the SAME select so they cannot drift.
#[semio_framework_async_macros::async_test]
async fn add_part_dialog_enumerates_live_part_kinds() {
    let definition = create_puzzle5d_app();
    let dialog = definition.dialogs.iter().find(|dialog| dialog.id == open_add_part_dialog::PUZZLE5D_ADD_PART_DIALOG).expect("addPart dialog");
    let arg = dialog.args.iter().find(|arg| arg.id == "partKind").expect("the dialog declares the partKind select");
    assert!(arg.required, "a kind must be picked before the dialog submits");
    let options = |arg: &semio_framework_plugin::ActionArgDef| {
        let semio_framework_plugin::ArgSchema::String { options, .. } = &arg.schema else { panic!("the partKind arg must stay a string select") };
        options.iter().map(|option| option.value.clone()).collect::<Vec<_>>()
    };
    let offered = options(arg);
    let offered: Vec<&str> = offered.iter().map(String::as_str).collect();
    assert!(!offered.is_empty() && offered.len() <= PUZZLE5D_PART_KIND_OPTIONS_MAX);
    assert!(!offered.contains(&"Part"), "the static placeholder kind is gone: {offered:?}");
    let declared: std::collections::BTreeSet<String> = [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()]
        .iter()
        .flat_map(|document| {
            document
                .kind_catalogs
                .as_ref()
                .and_then(|catalogs| catalogs.get("parts"))
                .and_then(semio_framework_pack_json::Value::as_array)
                .map(|entries| entries.iter().filter_map(|entry| entry.get("id").and_then(semio_framework_pack_json::Value::as_str).map(str::to_string)).collect::<Vec<_>>())
                .unwrap_or_default()
        })
        .chain(
            [concrete_forest_example_document(), nakagin_example_document(), capsule_dream_example_document()]
                .iter()
                .flat_map(|document| document.parts.iter().map(|part| part.part_kind.clone()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
        .collect();
    assert!(offered.iter().all(|option| declared.contains(*option)), "every offered kind is declared by a shipped catalog or inferred from a shipped part: {offered:?}");
    assert!(declared.iter().any(|kind| offered.contains(&kind.as_str())), "at least one real catalog kind is reachable");
    let actions = declared_actions(&definition);
    let form = actions.iter().find(|action| action.id == "addPartKind").expect("addPartKind is declared");
    assert!(!form.in_palette, "the parametrized verb is not a second palette row beside the dialog");
    assert_eq!(options(form.args.iter().find(|arg| arg.id == "partKind").expect("the arg form declares the same select")), offered.iter().map(|id| (*id).to_string()).collect::<Vec<_>>(), "the dialog and the arg form offer the SAME kinds");
    let prompt = actions.iter().find(|action| action.id == "openAddPartDialog").expect("openAddPartDialog is declared");
    assert!(prompt.in_palette, "the dialog row is the one the palette offers");
}
//#endregion 🗨️AddPartDialog

//#region 🖱️ContextMenuRows
/// 🖱️ Builds one context-menu request for a surface selection of `(domain, ids)` groups.
fn context_menu_of(app: &mut Puzzle5dApp, groups: Vec<(&str, Vec<String>)>) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    context_menu_at(app, groups, None)
}

/// 🖱️ `context_menu_of` with the world host's pointer hit — `(domain, id)` exactly as `world3dContextMenuSurfaceV1`
/// sends it (`vortex` for a grip marker, `object` for a part instance).
fn context_menu_at(app: &mut Puzzle5dApp, groups: Vec<(&str, Vec<String>)>, hit: Option<(&str, &str)>) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    let request = ContextMenuRequest {
        menu: UiMenuRef { id: "world3d".into(), args: None },
        surface: Some(ContextMenuSurfaceTarget {
            surface_id: world3d::WINDOW_KIND_ID.into(),
            kind: "world3d".into(),
            hits: hit.into_iter().map(|(domain, id)| semio_framework_plugin::ContextMenuHit { domain: domain.into(), id: id.into(), label: None }).collect(),
            selection: groups.into_iter().map(|(domain, ids)| ContextMenuSelectionGroup { domain: domain.into(), ids }).collect(),
            text: None,
        }),
        window_instance_id: None,
        point: None,
    };
    ::semio_framework_async::poll::resolve_ready(app.context_menu(&request, &semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)))
}

/// 🖱️ Every action id a menu row carries, at any depth.
fn context_menu_actions(rows: &[semio_framework_plugin::ContextMenuItemSpec]) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack: Vec<&semio_framework_plugin::ContextMenuItemSpec> = rows.iter().collect();
    while let Some(row) = stack.pop() {
        if let Some(action) = row.action.as_deref() {
            out.push(action.to_string());
        }
        if let Some(children) = row.children.as_ref() {
            stack.extend(children.iter());
        }
    }
    out
}

/// 🖱️ LAW: EVERY context-menu row resolves to something the app can really dispatch — a declared
/// action classified `Migrated` (an unmigrated verb is hard-dead at the dispatch gate, so a row
/// pointing at one is a visibly dead menu entry), or one of the four framework-reserved verbs this
/// menu is allowed to name. This is the law the 3d `zoomToSelection` defect would have caught.
#[semio_framework_async_macros::async_test]
async fn every_context_menu_row_resolves_to_a_live_verb() {
    let definition = create_puzzle5d_app();
    let actions = declared_actions(&definition);
    let migrated: std::collections::BTreeSet<&str> = actions
        .iter()
        .filter(|action| action.semantics.execution.interactive_job == InteractiveJobClassification::Migrated)
        .map(|action| action.id.as_str())
        .collect();
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let part_id = first_part_id(&app);
    let projection = projection_of(&app);
    let grip_id = projection
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find_map(|part| Some(puzzle5d_grip_full_id(part.get("id")?.as_str()?, part.get("grips")?.as_array()?.first()?.get("id")?.as_str()?))))
        .expect("a shipped part carries a grip");
    let fastener_id = projection.get("fasteners").and_then(Value::as_array).and_then(|rows| rows.first()).and_then(|row| row.get("id")).and_then(Value::as_str).expect("a shipped fastener").to_string();
    let surfaces = [
        vec![],
        vec![(PUZZLE5D_GRANULARITY_PART, vec![part_id.clone()])],
        vec![(PUZZLE5D_GRANULARITY_GRIP, vec![grip_id.clone()])],
        vec![(PUZZLE5D_GRANULARITY_FASTENER, vec![fastener_id.clone()])],
    ];
    for groups in surfaces {
        let rows = context_menu_of(&mut app, groups.clone());
        assert!(!rows.is_empty(), "every selection granularity offers rows, {groups:?} offered none");
        for action in context_menu_actions(&rows) {
            assert!(
                migrated.contains(action.as_str()) || PUZZLE5D_CONTEXT_MENU_RESERVED_ACTIONS.contains(&action.as_str()),
                "context-menu row {action:?} for {groups:?} is neither a Migrated app action nor a declared reserved verb"
            );
        }
    }
    close_app(&mut app);
}

/// 🖱️ LAW: the rows offered depend on WHAT is selected — an empty surface offers the no-subject
/// verbs, a part the selection vocabulary, a grip the suggestions entry point, a fastener its own
/// delete. Each branch names the verbs that branch exists for.
#[semio_framework_async_macros::async_test]
async fn context_menu_rows_follow_the_selected_granularity() {
    let mut app = app_with_registry();
    dispatch(&mut app, "setActiveExample", Some(&semio_framework_pack_json::json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None).expect("load nakagin");
    let part_id = first_part_id(&app);
    let projection = projection_of(&app);
    let grip_id = projection
        .get("parts")
        .and_then(Value::as_array)
        .and_then(|parts| parts.iter().find_map(|part| Some(puzzle5d_grip_full_id(part.get("id")?.as_str()?, part.get("grips")?.as_array()?.first()?.get("id")?.as_str()?))))
        .expect("a shipped part carries a grip");
    let fastener_id = projection.get("fasteners").and_then(Value::as_array).and_then(|rows| rows.first()).and_then(|row| row.get("id")).and_then(Value::as_str).expect("a shipped fastener").to_string();

    let empty = context_menu_actions(&context_menu_of(&mut app, vec![]));
    for expected in ["selectAll", "paste", "openAddPartDialog"] {
        assert!(empty.contains(&expected.to_string()), "an empty surface offers {expected}, got {empty:?}");
    }
    assert!(!empty.contains(&"deleteSelection".to_string()), "nothing selected means nothing to delete: {empty:?}");

    let part = context_menu_actions(&context_menu_of(&mut app, vec![(PUZZLE5D_GRANULARITY_PART, vec![part_id])]));
    for expected in ["duplicateSelection", "copy", "cut", "selectSameKindSelection", "focusSelection", "setSelectionFlag", "deleteSelection"] {
        assert!(part.contains(&expected.to_string()), "a part selection offers {expected}, got {part:?}");
    }

    let grip_rows = context_menu_of(&mut app, vec![(PUZZLE5D_GRANULARITY_GRIP, vec![grip_id.clone()])]);
    let grip = context_menu_actions(&grip_rows);
    assert!(grip.contains(&"openVortexSuggestions".to_string()), "one selected grip offers the suggestions submenu the world host fills live, got {grip:?}");
    assert!(!grip.contains(&"targetBrushSuggestions".to_string()), "the brush hover verb is no menu row — it only acts while the brush is armed: {grip:?}");
    let suggest = grip_rows.iter().find(|row| row.id == "suggest").expect("the suggest row");
    assert_eq!(suggest.args.as_ref().and_then(|args| args.get("fullId")).and_then(semio_framework_value::DslValue::as_str), Some(grip_id.as_str()), "the suggest row names the grip it was opened on");

    // 🎯️ A right-click on the grip of a SELECTED part is that grip's menu — grip markers only draw on marked
    // parts, so the part being selected is the normal case, not an edge case.
    let grip_part = grip_id.split(':').next().expect("full grip id").to_string();
    let on_grip = context_menu_actions(&context_menu_at(&mut app, vec![("object", vec![grip_part.clone()])], Some(("vortex", grip_id.as_str()))));
    assert!(on_grip.contains(&"openVortexSuggestions".to_string()) && !on_grip.contains(&"duplicateSelection".to_string()), "the grip under the pointer is the subject: {on_grip:?}");
    let inside = context_menu_actions(&context_menu_at(&mut app, vec![("object", vec![grip_part.clone()])], Some(("object", grip_part.as_str()))));
    assert!(inside.contains(&"duplicateSelection".to_string()), "a hit inside the selection keeps the whole selection as the subject: {inside:?}");

    let fastener_rows = context_menu_of(&mut app, vec![(PUZZLE5D_GRANULARITY_FASTENER, vec![fastener_id.clone()])]);
    let fastener = context_menu_actions(&fastener_rows);
    assert_eq!(fastener, vec!["deleteFastener".to_string()], "a fastener offers exactly its own delete, got {fastener:?}");
    let delete = fastener_rows.iter().find(|row| row.id == "delete").expect("the delete row");
    assert_eq!(delete.args.as_ref().and_then(|args| args.get("id")).and_then(semio_framework_value::DslValue::as_str), Some(fastener_id.as_str()), "the delete row names the fastener it was opened on");
    assert_eq!(delete.destructive, Some(true));
    close_app(&mut app);
}
//#endregion 🖱️ContextMenuRows

//#region 📋️ClipboardLaws
/// 🔒️ LAW: a CUT copies a locked part but never removes it — a lock exists to refuse a destructive
/// gesture, and the fasteners of a surviving part survive with it. The unlocked siblings still go.
#[test]
fn a_cut_copies_a_locked_part_but_never_removes_it() {
    let mut document = nakagin_example_document();
    assert!(document.parts.len() >= 2, "the locked-cut law needs at least two parts");
    let locked_id = document.parts[0].id.clone();
    let free_id = document.parts[1].id.clone();
    document.parts[0].part_2d.locked = Some(true);
    let snapshot = Puzzle5dPlaySnapshot::new(crate::editor::puzzle5d::entities::puzzle5d_snapshot_from_document(&document).expect("typed host document admits"));
    let ids = vec![locked_id.clone(), free_id.clone()];
    let fragment = puzzle5d_copy_fragment(&snapshot, &ids, &[]).expect("copy both parts");
    assert!(fragment.dsl_text.contains(locked_id.as_str()), "the locked part is still COPIED");
    let operations = puzzle5d_cut_operations(&snapshot, &ids, &[]);
    let deleted: Vec<&str> = operations
        .iter()
        .filter_map(|operation| match operation {
            Puzzle5dMutation::DeletePart(delete) => Some(delete.id.as_str()),
            _ => None,
        })
        .collect();
    assert!(deleted.contains(&free_id.as_str()), "the unlocked part is cut: {deleted:?}");
    assert!(!deleted.contains(&locked_id.as_str()), "the locked part is NOT cut: {deleted:?}");
}

/// 📋️ LAW: a paste preserves BOTH poses and offsets both by the same delta, keeps the fasteners
/// between copied parts with their endpoints remapped onto the fresh ids, and lands as ONE edit.
#[test]
fn a_paste_preserves_both_poses_and_the_fasteners_between_copied_parts() {
    let document = nakagin_example_document();
    let with_fastener = document.fasteners.first().cloned().expect("the shipped example connects parts");
    let endpoints = vec![owning_part_id_local(&with_fastener.source).to_string(), owning_part_id_local(&with_fastener.target).to_string()];
    let (parts, fasteners) = copy_selection_local(&document, &endpoints, &[]);
    assert!(fasteners.iter().any(|fastener| fastener.id == with_fastener.id), "the fastener BETWEEN two copied parts comes along");
    let delta = (7.0, -3.0);
    let (fresh_parts, fresh_fasteners) = paste_selection_local(&document, &parts, &fasteners, delta);
    assert_eq!(fresh_parts.len(), parts.len());
    for (source, pasted) in parts.iter().zip(&fresh_parts) {
        assert_ne!(source.id, pasted.id, "a pasted part carries a FRESH id");
        assert!(document.parts.iter().all(|existing| existing.id != pasted.id), "a fresh id collides with nothing live");
        assert_eq!((pasted.part_2d.x, pasted.part_2d.y), (source.part_2d.x + delta.0, source.part_2d.y + delta.1), "the flat pose moved by the delta");
        assert_eq!([pasted.part_3d.origin[0], pasted.part_3d.origin[1]], [source.part_3d.origin[0] + delta.0, source.part_3d.origin[1] + delta.1], "the volume pose moved by the SAME delta");
        assert_eq!(pasted.part_3d.origin[2], source.part_3d.origin[2], "the delta is planar");
        assert_eq!(pasted.grips.len(), source.grips.len(), "a pasted part keeps its grips");
    }
    let fresh_ids: Vec<&str> = fresh_parts.iter().map(|part| part.id.as_str()).collect();
    for fastener in &fresh_fasteners {
        assert!(fresh_ids.contains(&owning_part_id_local(&fastener.source)), "an endpoint was remapped onto a fresh part");
        assert!(fresh_ids.contains(&owning_part_id_local(&fastener.target)), "an endpoint was remapped onto a fresh part");
    }
}

/// 📋️ LAW: a default paste (no explicit position) still MOVES the fragment in both poses, so a paste
/// never lands exactly under the original where it is invisible.
#[test]
fn the_default_paste_placement_offsets_the_fragment_in_both_poses() {
    let document = nakagin_example_document();
    let ids: Vec<String> = document.parts.iter().take(2).map(|part| part.id.clone()).collect();
    let (parts, _) = copy_selection_local(&document, &ids, &[]);
    let placement = PastePlacement { anchor: PasteAnchor::Centroid, position: None };
    let delta = paste_delta_2d(&parts, &document.parts, &placement);
    assert!(delta != (0.0, 0.0), "the default placement is a real offset, got {delta:?}");
    let (fresh, _) = paste_selection_local(&document, &parts, &[], delta);
    for (source, pasted) in parts.iter().zip(&fresh) {
        assert_ne!((pasted.part_2d.x, pasted.part_2d.y), (source.part_2d.x, source.part_2d.y), "the flat pose moved");
        assert_ne!([pasted.part_3d.origin[0], pasted.part_3d.origin[1]], [source.part_3d.origin[0], source.part_3d.origin[1]], "the volume pose moved with it");
    }
}
//#endregion 📋️ClipboardLaws

//#region 🔖️Pz2ShippedKindCatalog
/// 🗨️ One document's `(id, label)` part-kind rows: its declared `kindCatalogs.parts`, or — when it
/// declares none — the distinct `partKind` values its own parts carry, the same fallback
/// `📌️panels/🛍️catalogue` renders so the dialog and the catalogue can never offer different kinds.
fn puzzle5d_part_kind_rows(document: &Puzzle5dDocument) -> Vec<(String, String)> {
    let declared: Vec<(String, String)> = document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(semio_framework_pack_json::Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let id = entry.get("id").and_then(semio_framework_pack_json::Value::as_str)?;
                    let label = ["label", "name"].iter().find_map(|key| entry.get(*key).and_then(semio_framework_pack_json::Value::as_str)).filter(|label| !label.is_empty()).unwrap_or(id);
                    Some((id.to_string(), label.to_string()))
                })
                .collect()
        })
        .unwrap_or_default();
    if !declared.is_empty() {
        return declared;
    }
    let mut inferred: Vec<String> = document.parts.iter().map(|part| part.part_kind.clone()).filter(|kind| !kind.is_empty()).collect();
    inferred.sort();
    inferred.dedup();
    inferred.into_iter().map(|kind| (kind.clone(), kind)).collect()
}

/// 🗂️ The authored [`PUZZLE5D_SHIPPED_PART_KINDS`] IS the two named examples' own catalog rows, in
/// their own order — the law that lets the `partKind` select be a `const` instead of a parse of
/// 171 591 B of DSL on the `describe()` path (slice PZ2, 2026-09-22). It pays that parse HERE, once,
/// where a slow test costs nothing, and fails the moment an example's kinds change without the
/// authored list following.
#[semio_framework_async_macros::async_test]
async fn shipped_part_kinds_are_the_two_named_examples_own_catalog_rows() {
    let mut derived: Vec<(String, String)> = Vec::new();
    for document in [&*CONCRETE_FOREST_EXAMPLE_DOCUMENT, &*NAKAGIN_EXAMPLE_DOCUMENT] {
        for (id, label) in puzzle5d_part_kind_rows(document) {
            if derived.len() >= PUZZLE5D_PART_KIND_OPTIONS_MAX {
                break;
            }
            if derived.iter().any(|(existing, _)| existing == &id) {
                continue;
            }
            derived.push((id, label));
        }
    }
    let authored: Vec<(String, String)> = PUZZLE5D_SHIPPED_PART_KINDS.iter().map(|(id, label)| ((*id).to_string(), (*label).to_string())).collect();
    assert_eq!(authored, derived, "PUZZLE5D_SHIPPED_PART_KINDS drifted from the shipped documents — re-author it from `concrete-forest` then `nakagin-capsule-tower`");
}
//#endregion 🔖️Pz2ShippedKindCatalog

//#region 🔖️HistoryEditReferences
/// 🪧️ N3 (design §16.4): a history-edit reference chip names a puzzle 5d entity as the outliner does — a part by its
/// volume label, else its flat text, else its kind; a grip as `<part> · <grip kind>`; a fastener as `<part> → <part>` —
/// and leaves target volumes, undeclared kinds and unknown ids to the framework's generic `<Kind> <short id>`.
#[test]
fn history_edit_reference_chips_name_entities_as_the_outliner_does() {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::examples::puzzle5d::concrete_forest::DSL_TEXT).expect("the example parses");
    let part = snapshot.parts[0].id.clone();
    let grip = puzzle5d_grip_full_id(&part, &snapshot.parts[0].grips[0].id);
    let grip_kind = snapshot.parts[0].grips[0].grip_kind.clone().expect("the example names its grip kinds");
    snapshot.parts[0].part_3d.label = None;
    snapshot.parts[0].part_2d.text = Some("Forest Left".into());
    snapshot.fasteners.push(crate::standards::v1::subsets::any::schema::document_records::Puzzle5dFastener { id: "fastener-chip".into(), source: grip.clone(), target: grip.clone(), fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    let chip = |kinds: &[&str], id: &str| puzzle5d_entity_label(&snapshot, &kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id).map(|label| label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string());
    assert_eq!(chip(&[PUZZLE5D_GRANULARITY_PART, PUZZLE5D_GRANULARITY_TARGET_VOLUME], &part).as_deref(), Some("Forest Left"), "a part without a volume label reads its flat text");
    assert_eq!(chip(&[PUZZLE5D_GRANULARITY_GRIP], &grip), Some(format!("Forest Left \u{b7} {grip_kind}")), "a grip reads its part and its kind");
    assert_eq!(chip(&[PUZZLE5D_GRANULARITY_FASTENER], "fastener-chip").as_deref(), Some("Forest Left \u{2192} Forest Left"), "a fastener reads its two parts");
    assert_eq!(chip(&[PUZZLE5D_GRANULARITY_TARGET_VOLUME], &part), None, "an id outside the reference's declared kinds keeps the generic label");
    assert_eq!(chip(&[], "ghost"), None, "an unknown id keeps the generic label");
}
//#endregion 🔖️HistoryEditReferences

//#region 🛡️ReservedWirePreflight
/// 📏️ The exact route cap admits the wire untouched; one byte more refuses it and hands the original owner back before any fixed-page copy.
#[test]
fn reserved_wire_exact_max_and_plus_one_preflight_return_the_original_owner() {
    let at_cap = vec![7u8; 64];
    let admitted = puzzle5d_preflight_reserved_wire(at_cap.clone(), 64).expect("the exact cap is admitted");
    assert_eq!(admitted, at_cap);
    let over = vec![9u8; 65];
    let Err((fault, owner)) = puzzle5d_preflight_reserved_wire(over.clone(), 64) else { panic!("one byte over the cap must be refused") };
    assert_eq!(owner, over, "the refusal hands the original wire back");
    assert!(fault.message.contains("exceeds its exact route cap"));
}
//#endregion 🛡️ReservedWirePreflight
