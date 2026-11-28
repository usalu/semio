//! ✉️ Drawing editor unit laws over the live envelope: wire framing, admission, load driving, recursive clone swap, cancellation and edit identifier bounds.

use super::*;
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;

fn drawing_envelope_wire() -> Vec<u8> {
    drawing_envelope_with(true, true)
}

fn drawing_envelope_with(asset: bool, edit: bool) -> Vec<u8> {
    use semio_os_kernel::os_pack::snapshot::ArtifactPack;

    let mut snapshot = default_drawing_document("drawing-retained-load", None);
    let mut group = semio_s_artifact_draw_drawing_model::create_drawing_group_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Nested")).to_string().into()).expect("nonempty authored identity"), "Nested");
    if let DrawingLayerNode::Group(value) = &mut group {
        value.children.push(semio_s_artifact_draw_drawing_model::create_drawing_path_layer(semio_s_artifact_draw_drawing_model::identity::DrawingIdentity::admit((("Path")).to_string().into()).expect("nonempty authored identity"), "Path", vec![semio_s_artifact_draw_drawing_model::PathSegment::Move { to: [1.0, 2.0] }, semio_s_artifact_draw_drawing_model::PathSegment::Line { to: [3.0, 4.0] }].into()));
    }
    let retained_target = match &group {
        DrawingLayerNode::Group(value) => layer_id(&value.children[0]).to_string(),
        _ => unreachable!("retained Drawing fixture group remains exact"),
    };
    snapshot.layers.push(group);
    if asset {
        snapshot.assets.insert(semio_framework_value::paged::PagedUtf8::from("image-a"), semio_s_artifact_draw_drawing_model::DrawingImageAsset { width: 1, height: 1, samples: vec![[0,0,0,0];1].into() });
    }
    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mutation = DrawingMutation::RenameLayer(semio_s_artifact_draw_drawing_metadata::rename_layer::mutation::RenameLayer { layer_id: retained_target.clone().into(), new_name: "Retained Path".into() });
    let mutation_hex = semio_s_artifact_draw_drawing_mutations::binary::encode_op(&mutation).expect("Drawing fixture mutation pack").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let mut envelope = serde_json::json!({
        "schema": DRAWING_DOCUMENT_SCHEMA,
        "id": "drawing-retained-load",
        "vcs": {
            "initialPack": snapshot_hex,
            "edits": [{
                "id": "drawing-retained-edit-final",
                "actor": "drawing-retained-actor",
                "line": null,
                "forwards": [mutation_hex],
                "inverse": [],
                "sequenceNumber": 1,
                "startedAt": "2026-08-23T00:00:00.000Z"
            }],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    });
    if !edit { envelope["vcs"]["edits"] = serde_json::json!([]); }
    let wire = serde_json::to_vec(&envelope).expect("schema-first Drawing fixture envelope");
    semio_s_artifact_draw_drawing_scene::preparation::testkit::scene_wallet().close_owned(snapshot);
    wire
}

#[semio_framework_async_macros::async_test]
async fn drawing_envelope_accepts_assets_without_edits() {
    let mut app = drawing_app().await;
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_with(true, false));
    assert_eq!(drive_drawing_load(&mut app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).unwrap());
    assert_eq!(app.snapshot().unwrap().assets.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn drawing_envelope_accepts_edits_without_assets() {
    let mut app = drawing_app().await;
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_with(false, true));
    assert_eq!(drive_drawing_load(&mut app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert!(app.acknowledge_artifact_store_replacement(handle).unwrap());
    assert_eq!(app.history_snapshot().await.unwrap().edit_count, 1);
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_submit_recursive_clone_swap_displaced_store_and_exact_ack_succeed() {
    let mut app = drawing_app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
    assert_eq!(handle.generation, base_generation);
    let poll = drive_drawing_load(&mut app, handle).await;
    assert_eq!(poll, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "valid retained Drawing load refusal: {:?}", app.artifact_store_replacement_refusal(handle));
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    let projection = app.snapshot().expect("Drawing retained mutation publication");
    let renamed = semio_s_artifact_draw_drawing_model::find_drawing_layer(&projection, "Path").expect("retained Drawing target");
    assert_eq!(semio_s_artifact_draw_drawing_model::layer_base(renamed).name, "Retained Path");
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first Drawing acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Drawing acknowledgement"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut app = drawing_app().await;
    let base_generation = app.artifact_generation_now();
    let wire = drawing_envelope_wire();
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Drawing ingress credits");
    let first = &wire[..wire.len().min(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Drawing first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Drawing page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel Drawing ingress");
    assert_eq!(drive_drawing_load(&mut app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
    assert_eq!(app.artifact_generation_now(), base_generation);
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_initializer_candidate_container_commit_ack_cancel_stale_preserve_last_valid_and_exact_handle() {
    for turns in [0usize, 1, 2, 8] {
        let mut app = drawing_app().await;
        let base_generation = app.artifact_generation_now();
        let base_id = app.snapshot().expect("Drawing last-valid snapshot").id;
        let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
        for _ in 0..turns {
            app.0.maintenance_step(app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("bounded Drawing staged maintenance");
        }
        let stale = semio_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
        assert!(app.0.advance_artifact_envelope_load(stale, app.1.cx()).is_err(), "stale staged handle cannot consume the exact operation owner");
        app.cancel_artifact_envelope_load(handle).expect("exact Drawing staged cancellation");
        assert!(matches!(drive_drawing_load(&mut app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault | semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled));
        assert_eq!(app.artifact_generation_now(), base_generation);
        assert_eq!(app.snapshot().expect("Drawing last-valid survives staged cancel").id, base_id);
    }

    let mut app = drawing_app().await;
    let handle = admit_drawing_envelope(&mut app, &drawing_envelope_wire());
    let poll = drive_drawing_load(&mut app, handle).await;
    assert_eq!(poll, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready, "valid staged Drawing load refusal: {:?}", app.artifact_store_replacement_refusal(handle));
    let stale = semio_plugin::ArtifactEnvelopeDecodeOperationHandle { operation: handle.operation, generation: semio_framework_job::Generation(handle.generation.0 + 1) };
    assert!(!app.acknowledge_artifact_store_replacement(stale).expect("stale Drawing ACK is refused without retiring the exact owner"));
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("exact staged Drawing ACK"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate staged Drawing ACK is idempotent"));
}

#[semio_framework_async_macros::async_test]
async fn drawing_live_envelope_rejects_single_and_final_edit_id_plus_one_before_mutation_candidate() {
    for final_edit in [false, true] {
        let mut value: serde_json::Value = serde_json::from_slice(&drawing_envelope_wire()).expect("Drawing retained fixture JSON");
        let edits = value.pointer_mut("/vcs/edits").and_then(serde_json::Value::as_array_mut).expect("Drawing retained edits");
        if final_edit {
            let mut first = edits[0].clone();
            first["id"] = serde_json::Value::String("drawing-retained-edit-first".into());
            edits.insert(0, first);
        }
        edits.last_mut().expect("Drawing final edit")["id"] = serde_json::Value::String("x".repeat(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES + 1));
        let wire = serde_json::to_vec(&value).expect("hostile Drawing edit fixture");
        let mut app = drawing_app().await;
        let generation = app.artifact_generation_now();
        let handle = admit_drawing_envelope(&mut app, &wire);
        assert_eq!(drive_drawing_load(&mut app, handle).await, semio_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault);
        assert_eq!(app.artifact_generation_now(), generation);
    }
}
