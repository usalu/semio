/// 🧾️ History, kit and close laws: the history edit reference chips, the kit in-port, the backbone convergence and the terminal close witness.

//#region 🔖️HistoryEditReferences
/// 🪧️ N3 (design §16.4): a history-edit reference chip names a puzzle 3d entity as the outliner does — an unlabelled
/// object by its kind's catalog label, a vortex as `<object> · <vortex>`, an attraction as `<object> → <object>` — and
/// leaves target volumes, undeclared kinds and unknown ids to the framework's generic `<Kind> <short id>`.
#[test]
fn history_edit_reference_chips_name_entities_as_the_outliner_does() {
    use semio_s_artifact_puzzle_3d_model::io::text::snapshot::parse_dsl;
    let mut snapshot = parse_dsl(&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../📚️examples/🌲️concrete-forest/🖼️assets/🌲️forest/🗣️.dsl.semio")).expect("committed example reads")).expect("the example parses");
    let object = snapshot.objects[0].id.clone();
    let vortex = puzzle3d_vortex_full_id(&object, &snapshot.objects[0].vortices[0].id);
    let vortex_label = snapshot.objects[0].vortices[0].label.clone().expect("the example labels its vortices");
    let kind = snapshot.objects[0].object_kind.clone().expect("the seed names its kind");
    snapshot.objects[0].label = None;
    snapshot.meta.kind_catalogs.as_mut().expect("the example ships its kind catalogs").objects.iter_mut().find(|entry| entry.id == kind).expect("the seed's kind is catalogued").label = "Forest Left".into();
    snapshot.attractions.push(semio_s_artifact_puzzle_3d_model::Puzzle3dAttraction { id: "attraction-chip".into(), attracting: vortex.clone(), attracted: vortex.clone(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    let chip = |kinds: &[&str], id: &str| puzzle3d_entity_label(&snapshot, &kinds.iter().map(|kind| kind.to_string()).collect::<Vec<_>>(), id).map(|label| label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).to_string());
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_OBJECT, PUZZLE3D_GRANULARITY_TARGET_VOLUME], &object).as_deref(), Some("Forest Left"), "an unlabelled object reads its kind's catalog label");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_VORTEX], &vortex), Some(format!("Forest Left \u{b7} {vortex_label}")), "a vortex reads its object and its own label");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_ATTRACTION], "attraction-chip").as_deref(), Some("Forest Left \u{2192} Forest Left"), "an attraction reads its two objects");
    assert_eq!(chip(&[PUZZLE3D_GRANULARITY_TARGET_VOLUME], &object), None, "an id outside the reference's declared kinds keeps the generic label");
    assert_eq!(chip(&[], "ghost"), None, "an unknown id keeps the generic label");
}

/// 🔗️ N3 (design §16.4): while a history edit of a gesture is open, the world paints exactly the objects its draft
/// references highlighted (one changed instance record, so the delta lane carries it) — and none once the edit exits.
#[semio_framework_async_macros::async_test]
async fn a_history_edit_draft_highlights_exactly_the_objects_it_references() {
    let (mut app, object_id) = gumball_app().await;
    let (result, settled) = dispatch_reporting(&mut app, "translateSelection", Some(&json!({ "ids": [object_id.as_str()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), None).await;
    result.expect("the drag commits");
    let mutation_id = edit_rows(&settled).first().and_then(|row| row.mutations.first()).map(|mutation| mutation.mutation_id.clone()).expect("the drag is one editable mutation");
    let highlighted = |node: &Value| instances_of(node).into_iter().filter(|instance| instance.get("highlighted").and_then(Value::as_bool) == Some(true)).filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string)).collect::<Vec<_>>();
    assert!(highlighted(&render_composite(&mut app).await).is_empty(), "nothing is highlighted before a draft opens");
    let admitted = dispatch_reserved_unsettled(&mut app, semio_framework_time_travel_actions::HISTORY_EDIT_BEGIN_ACTION_ID, Some(&json!({ "mutationId": mutation_id })), None).await.expect("historyEditBegin admits");
    settle_reserved(&mut app, admitted).await.expect("historyEditBegin settles");
    assert_eq!(highlighted(&render_composite(&mut app).await), vec![object_id.clone()], "the draft's target is painted highlighted");
    let admitted = dispatch_reserved_unsettled(&mut app, semio_framework_time_travel_actions::HISTORY_EDIT_EXIT_ACTION_ID, None, None).await.expect("historyEditExit admits");
    settle_reserved(&mut app, admitted).await.expect("historyEditExit settles");
    assert!(highlighted(&render_composite(&mut app).await).is_empty(), "closing the edit clears the highlight");
}
//#endregion 🔖️HistoryEditReferences

//#region 🔖️KitInPort
/// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like block3d's
/// `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
/// `Puzzle3dPlayApp::import_media` must normalize `objectKinds` → `objects` / `vortexKinds` →
/// `vortices` and, after applying the returned operations, land that object kind inside
/// `meta.kind_catalogs.objects` (and the vortex kind inside `.vortices`).
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_upserts_object_and_vortex_kinds_into_meta_kind_catalogs() {
    let projection = Puzzle3dPlayApp::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);

    let fragment = json!({
        "schema": "manifest",
        "objectKinds": [{
            "id": "capsule",
            "name": "capsule",
            "label": "Capsule",
            "meshUrl": "/mesh/capsule.glb",
            "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
        }],
        "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
    assert!(!emit.artifact_mutations.is_empty(), "importing a non-empty fragment must emit real operations");

    let mut next_projection = projection.typed().clone();
    for operation in &emit.artifact_mutations {
        next_projection = protocol::apply_diff_cold(protocol::Mutation::<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot>::diff(operation, &next_projection).diff(), &next_projection).expect("valid mutation diff");
    }

    let next_projection = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&next_projection));
    let objects = next_projection.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert!(objects.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")), "the imported object kind must appear in meta.kind_catalogs.objects");
    let capsule = objects.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).unwrap();
    assert_eq!(capsule.pointer("/representations/0/url").and_then(Value::as_str), Some("/mesh/capsule.glb"));
    assert_eq!(capsule.pointer("/vortices/0/vortexKind").and_then(Value::as_str), Some("door"), "the per-object vortex template keeps its vortexKind after normalization");

    let vortices = next_projection.pointer("/meta/kindCatalogs/vortices").and_then(Value::as_array).expect("vortices catalog present");
    assert!(vortices.iter().any(|entry| entry.get("id").and_then(Value::as_str) == Some("door")), "the imported vortex kind must appear in meta.kind_catalogs.vortices");

    let compatibility = next_projection.pointer("/meta/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
    assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
}

/// 🔁️ Re-importing the SAME fragment (a second producer edge, or a redelivered message on a
/// `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
#[semio_framework_async_macros::async_test]
async fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
    let projection = Puzzle3dPlayApp::reference_birth().expect("explicit reference snapshot producer")().expect("the reference snapshot births");
    let history = semio_framework_plugin::HistoryView::empty();
    let mut current = projection.typed().clone();

    let fragment = json!({
        "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
        "vortexKinds": [],
        "cableKinds": [],
        "attractionKinds": [],
        "kindCompatibility": [],
    });
    let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type }, payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() } };

    for _ in 0..2 {
        let doc_projection = Puzzle3dPlaySnapshot::new(current.clone());
        let doc = ArtifactView::new(&doc_projection, &history);
        let emit = Puzzle3dPlayApp::import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        for operation in &emit.artifact_mutations {
            current = protocol::apply_diff_cold(protocol::Mutation::<semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot>::diff(operation, &current).diff(), &current).expect("valid mutation diff");
        }
    }

    let current = semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(&current));
    let objects = current.pointer("/meta/kindCatalogs/objects").and_then(Value::as_array).expect("objects catalog present");
    assert_eq!(objects.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
}

#[semio_framework_async_macros::async_test]
async fn kit_in_port_is_declared_on_the_app_io() {
    let io = Puzzle3dPlayApp::io().expect("puzzle3d declares an AppIo");
    let port = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
    assert_eq!(port.kind_id.as_deref(), Some("kit.catalog"));
    assert_eq!(port.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
    assert!(matches!(port.multiplicity, PortMultiplicity::Many));
}
//#endregion 🔖️KitInPort

//#region 🔖️Convergence
/// 🧪️ Definitional convergence proof: two instances on one backbone make DISJOINT object edits and,
/// after exchanging operations, both converge to contain BOTH objects — impossible under
/// whole-document `setSnapshot` snapshots, which would clobber one side.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_object_edits_via_backbone() {
    use store::os_store::backbone::MemoryBackbone;
    let mut instance_a = app().await;
    let mut instance_b = app().await;
    let seeded = object_count(&instance_a);
    let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://puzzle3d-convergence", "mem://puzzle3d-convergence").await;
    instance_a.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::os_store::backbone::Backbones::Memory(backbone_b)).await.expect("attach b");

    dispatch(&mut instance_a, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 0.0, 0.0] })), None).await.expect("a adds object");
    dispatch(&mut instance_b, "addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.0, 0.0, 0.0] })), None).await.expect("b adds object");

    dispatch(&mut instance_a, "commitCheckpoint", None, None).await.expect("pump a");
    dispatch(&mut instance_b, "commitCheckpoint", None, None).await.expect("pump b");

    assert_eq!(object_count(&instance_a), seeded + 2, "instance A must contain both objects");
    assert_eq!(object_count(&instance_b), seeded + 2, "instance B must contain both objects");
    let _ = Puzzle3dCamera::default();
}

//#endregion 🔖️Convergence

//#region 🔖️Close
/// 🧹️ ticket 26/09/02/PUZZLE-3D-END-TO-END: the app-close contract, stated exactly once and asserted
/// by a test rather than by a destructor. `VcsArtifactApp::close_step` walks seven owned lanes in a
/// fixed order (`document-store`, `config-store`, `draft-store`, `presence-store`, `transient-store`,
/// window-transient, `interaction-store`) and every lane is mandatory: `drive_artifact_owned_disposer`
/// faults `interactive-job.close-owned-disposer-missing` the moment `A::build_<lane>_disposer()`
/// returned `None`. Until that holds for every lane, no puzzle3d instance can ever reach its
/// terminal-empty witness — in a test process OR in a host, where the same `close_step` runs on
/// shutdown and the framework-installed `ArtifactStoreCursorDisposer` members then panic in `Drop`.
#[semio_framework_async_macros::async_test]
async fn fixture_app_reaches_its_terminal_empty_close_witness() {
    match close_witness(&mut app().await) {
        Ok(true) => {}
        Ok(false) => panic!("close_step reported Complete without reaching terminal-empty ownership"),
        Err(fault) => panic!("every owned close lane must supply its bounded disposer, got {fault:?}"),
    }
}
//#endregion 🔖️Close
