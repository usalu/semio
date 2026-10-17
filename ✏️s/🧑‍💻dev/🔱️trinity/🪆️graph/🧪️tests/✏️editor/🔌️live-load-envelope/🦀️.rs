//! 🔌️ Jack envelope wires, admission and the live load pump of the editor laws. See [`super`].

use super::*;

fn jack_envelope_wire() -> Vec<u8> {
    jack_envelope_wire_of(jack::standards::v1::subsets::any::schema::snapshot::empty_trinity_graph_snapshot::<SemioJackComposition>())
}

/// 📦️ The exact envelope wire a host hands the guest for `snapshot` (initial snapshot, no edits).
pub(super) fn jack_envelope_wire_of(snapshot: jack::standards::v1::subsets::any::schema::snapshot::JackSnapshot) -> Vec<u8> {
    use pack::snapshot::ArtifactPack;

    let snapshot_pack = snapshot.encode_pack();
    let snapshot_hex = snapshot_pack.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = semio_framework_pack_json::to_string(&semio_framework_pack_json::json!({
        "schema": TRINITY_GRAPH_SCHEMA,
        "id": "jack-live-load",
        "vcs": {
            "initialPack": snapshot_hex,
            "edits": [],
            "changes": [],
            "checkpoints": [],
            "alternatives": []
        },
        "editMessages": [],
        "conflicts": []
    }))
    .into_bytes();
    let envelope = store::os_store::document_materialize::create_document_envelope(TRINITY_GRAPH_SCHEMA, "jack-live-load", snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    jack::standards::v1::subsets::any::schema::content::drive_retirement_to_terminal(jack::standards::v1::subsets::any::schema::host::jack_envelope_decode_owner_bundle().retire_envelope(envelope)).expect("Jack fixture envelope retirement");
    wire
}

pub(super) fn admit_jack_envelope(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, wire: &[u8],cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle {
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len().max(1)).expect("Jack live envelope ingress credits");
    for chunk in wire.chunks(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES) {
        let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, chunk.len()).expect("bounded Jack live envelope page");
        app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("Jack live envelope page admission failed: {fault:?}"));
    }
    assert!(app.seal_artifact_envelope_ingress(handle,cx).expect("Jack live envelope seal/submit"));
    handle
}

pub(super) fn drive_jack_live_load(app: &mut VcsArtifactApp<EditorApp<TrinityJackPlayApp<SemioJackComposition>>, semio_s_artifact_stdio_semio::SemioMembers>, handle: semio_framework_plugin::ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll {
    // 📏️ The runtime host pumps a document load with 32 KiB turns (`DOCUMENT_ARCHIVE_POLL_STEP_BYTES`);
    // a 4 KiB envelope-page grant can never release the decode worker's 16 KiB job-payload pages
    // (`JOB_PAYLOAD_PAGE_BYTES`), so the load spun on its own close step forever.
    for _ in 0..100_000 {
        app.maintenance_step(cx,jack::standards::v1::subsets::any::schema::content::trinity_mounted_owner_policy().maintenance).expect("one Jack live maintenance turn");
        let poll = app.advance_artifact_envelope_load(handle,cx).expect("Jack live load advancement");
        if matches!(poll, semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled | semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Fault) {
            return poll;
        }
        std::thread::yield_now();
    }
    panic!("Jack live envelope load did not reach terminal")
}

#[semio_framework_async_macros::async_test]
async fn jack_live_envelope_submit_pump_swap_displaced_store_and_exact_ack_succeed() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let base_generation = app.artifact_generation_now();
    let handle = admit_jack_envelope(&mut app, &jack_envelope_wire(),cx);
    assert_eq!(handle.generation, base_generation);
    assert_eq!(drive_jack_live_load(&mut app, handle,cx), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Ready);
    assert_eq!(app.artifact_generation_now().0, base_generation.0 + 1);
    assert!(app.acknowledge_artifact_store_replacement(handle).expect("first exact Jack load acknowledgement"));
    assert!(!app.acknowledge_artifact_store_replacement(handle).expect("duplicate Jack load acknowledgement is a no-op"));

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}

#[semio_framework_async_macros::async_test]
async fn jack_live_envelope_cancel_closes_retained_pages_without_publication() {
    let mut original_caller=artifact_app_laws::OriginalFixtureCaller::admit().expect("closed original Jack law caller");
    let mut original_context=original_caller.context();
    let cx=&mut original_context;

    let mut app = new_app().await;
    let base_generation = app.artifact_generation_now();
    let wire = jack_envelope_wire();
    let pages = wire.len().div_ceil(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).max(1);
    let handle = app.begin_artifact_envelope_ingress(pages, wire.len()).expect("cancelled Jack ingress credits");
    let first = &wire[..wire.len().min(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)];
    let mut bytes = [0; store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES];
    bytes[..first.len()].copy_from_slice(first);
    let page = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage::try_from_array(bytes, first.len()).expect("cancelled Jack first page");
    app.admit_artifact_envelope_ingress_page(handle, page).unwrap_or_else(|(fault, _page)| panic!("cancelled Jack page admission failed: {fault:?}"));
    app.cancel_artifact_envelope_load(handle).expect("cancel exact Jack ingress");
    assert_eq!(drive_jack_live_load(&mut app, handle,cx), semio_framework_plugin::ArtifactEnvelopeDecodeOperationPoll::Cancelled);
    assert_eq!(app.artifact_generation_now(), base_generation);

    app.close(cx);

    let remaining_fuel=cx.fuel_remaining();
    drop(original_context);
    original_caller.retire(remaining_fuel).expect("paid original Jack law caller retirement");
}
