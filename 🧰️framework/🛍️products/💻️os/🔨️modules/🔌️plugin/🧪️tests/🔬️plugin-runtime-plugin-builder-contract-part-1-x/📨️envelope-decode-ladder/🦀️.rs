// 📨️ Envelope decode ladder: the slow fresh envelope field decoder, the decode ladder drivers, the live decode pending report and the reactor turn pump.

//#region 📨️EnvelopeDecodeLadder
/// 🐢️ Wraps the real fresh field decoder and yields `budget` times on the first offered token so the
/// decode ladder is measurable while the wire still decodes through the domain catalog.
struct SlowFreshEnvelopeFieldDecoder {
    pub budget: usize,
    pub steps: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    pub inner: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<ComposedParentSnapshot, RecursiveFixtureMutation>>,
}

impl SlowFreshEnvelopeFieldDecoder {
    pub fn new(budget: usize, steps: &std::sync::Arc<std::sync::atomic::AtomicUsize>, inner: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<ComposedParentSnapshot, RecursiveFixtureMutation>>) -> Self {
        Self { budget, steps: std::sync::Arc::clone(steps), inner }
    }
}

impl store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<ComposedParentSnapshot, RecursiveFixtureMutation> for SlowFreshEnvelopeFieldDecoder {
    fn accept_field_token(
        &mut self,
        field_id: u16,
        token: store::os_store::owned_schema_decode::OwnedSchemaToken,
        terminal: bool,
        source: &store::os_store::owned_schema_decode::OwnedSchemaRecordCursor,
        cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.steps.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        cx.consume_fuel(1);
        if self.budget > 0 {
            self.budget -= 1;
            return Ok(store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep::Pending);
        }
        self.inner.accept_field_token(field_id, token, terminal, source, cx)
    }

    fn finish_record(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecodeStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.inner.finish_record(cx)
    }

    fn maximum_close_byte_demand(&self) -> usize {
        self.inner.maximum_close_byte_demand()
    }

    fn maximum_retained_close_bytes(&self) -> usize {
        self.inner.maximum_retained_close_bytes()
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.inner.next_close_copy_byte_demand()
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.inner.next_close_capacity_byte_demand(maximum_copy_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.inner.next_close_release_byte_demand()
    }

    fn next_close_depth_demand(&self) -> Result<usize, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.inner.next_close_depth_demand()
    }

    fn close_step(&mut self, grant: store::RetainedCloneGrant) -> Result<store::RetainedCloneStep, store::os_store::owned_schema_decode::OwnedSchemaDecodeDiagnostic> {
        self.inner.close_step(grant)
    }

    fn terminal_is_empty(&self) -> bool {
        self.inner.terminal_is_empty()
    }
}

fn envelope_law_pages() -> store::os_store::owned_schema_decode::OwnedSchemaDecodePages {
    let snapshot = ComposedParentSnapshot { slot: None, revision: 0 };
    let snapshot_hex = snapshot.encode_pack().iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let wire = serde_json::to_vec(&serde_json::json!({
        "schema": ComposedParentApp::<false>::DOCUMENT_SCHEMA,
        "id": "envelope-decode-law",
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
    .expect("schema-first composed envelope law fixture");
    let chunks = wire.chunks(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES).collect::<Vec<_>>();
    let mut pages =
        store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages: chunks.len(), maximum_bytes: wire.len() }).expect("envelope law page credits");
    for chunk in chunks {
        pages.admit_page(store::os_store::owned_schema_decode::OwnedSchemaDecodePage::try_from_slice(chunk).expect("bounded envelope law page")).unwrap_or_else(|_| panic!("pre-admitted envelope law page"));
    }
    pages.seal().expect("sealed envelope law page set");
    pages
}

fn install_slow_envelope_decode(
    app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>,
    operation: u64,
    budget: usize,
    steps: &std::sync::Arc<std::sync::atomic::AtomicUsize>, cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle {
    let operation = semio_framework_job::OperationId(operation);
    let generation = semio_framework_job::Generation(app.stores.store.generation_now());
    let bundle = ComposedParentApp::<true>::build_envelope_decode_owner_bundle().expect("composed parent envelope decode bundle");
    let completion = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion::new();
    let inner = bundle.begin_fresh_decoder(operation, generation, std::sync::Arc::clone(&app.ingest.envelope_completed_records), std::sync::Arc::clone(&completion));
    app.ingest.admit_artifact_envelope_decode_owner(operation, generation, envelope_law_pages(), Box::new(SlowFreshEnvelopeFieldDecoder::new(budget, steps, inner)), completion, cx)
        .unwrap_or_else(|_| panic!("exact envelope decode owner admission"))
}

/// 🧹️ Pumps the post-`Ready` worker close ladder (same work production loads run via [`VcsArtifactApp::advance_ready_envelope_decode_worker_owners`]).
fn pump_envelope_decode_worker_close(app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>, handle: semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle, cx:&mut semio_framework_job::StepContext<'_>) {
    for _ in 0..32 {
        if app.artifact_envelope_decode_worker_owners_are_terminal(handle) {
            return;
        }
        app.ingest.advance_ready_envelope_decode_worker_owners(handle, app.mounted_policy.maintenance, cx).expect("envelope decode worker-owner drain");
        if let Err(source)=semio_framework_job_worker::pump_worker_job_retirements(8,cx){close_composition_refusal(source.into(),cx)}
    }
    assert!(app.artifact_envelope_decode_worker_owners_are_terminal(handle), "envelope decode worker owners still active after bounded drain");
}

/// 🚿️ Drives one live envelope decode to its terminal poll through the reactor-turn pump, so the
/// app can close with every field lease returned.
async fn drain_envelope_decode(app: &mut VcsArtifactApp<ComposedParentApp, TestMembers>, handle: semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle, cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll {
    let mut poll = semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Pending;
    for _ in 0..100_000 {
        PluginApp::advance_typed_operation_publication(app, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("one reactor turn drives the envelope decode worker");
        poll = app.poll_artifact_envelope_decode(handle);
        if matches!(poll, semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Fault | semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Cancelled) {
            return poll;
        }
        if poll == semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        let demand = app.maintenance_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("original maintenance demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        assert!(grant.maximum_copy_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES && grant.maximum_release_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
        let step = app.maintenance_step(cx,grant).expect("envelope decode maintenance");
        if let Some(progress) = step.progress() { assert!(progress.fits(grant)); }
        if let Err(source)=semio_framework_job_worker::pump_worker_job_retirements(8,cx){close_composition_refusal(source.into(),cx)}
        poll = app.poll_artifact_envelope_decode(handle);
        if matches!(poll, semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Fault | semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Cancelled) {
            return poll;
        }
        if poll == semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready {
            break;
        }
        semio_framework_async::yield_once().await;
    }
    if poll == semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready {
        pump_envelope_decode_worker_close(app, handle, cx);
    }
    poll
}

/// ⚖️ LAW: a decode that has not finished is reported as `Pending`, never as a terminal `Fault`.
/// The store replacement it will hand to does not exist until the decode is `Ready`, and reading
/// that absent job here made every in-progress document load fail closed at the first turn
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn advance_artifact_envelope_load_reports_a_live_decode_as_pending_not_fault() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = live_composed_replacement_app().await;
    let steps = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let handle = install_slow_envelope_decode(&mut app, 811, 64, &steps, &mut cx);
    assert_eq!(app.poll_artifact_envelope_decode(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Pending);
    assert_eq!(
        app.advance_artifact_envelope_load(handle, &mut cx).expect("live decode advancement"),
        semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Pending,
        "an unfinished decode is pending, not a terminal fault"
    );
    assert_eq!(
        app.poll_artifact_store_replacement(handle),
        semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Fault,
        "no replacement job exists yet, which is exactly the reading that must not leak out of the load API"
    );
    drain_envelope_decode(&mut app, handle, &mut cx).await;
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// ⚖️ LAW: the reactor turn pumps the envelope decode worker to its terminal poll.
/// `ActiveArtifactEnvelopeDecode::drive` only SUBMITS a step to the process pool, and the pool runs
/// a submitted step on wasm solely when it is pumped; before this the ONLY driver was the
/// cooperative-maintenance rotation, so a document load never advanced inside an interactive turn.
#[semio_framework_async_macros::async_test]
async fn one_reactor_turn_pumps_the_envelope_decode_worker_to_its_terminal_poll() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = live_composed_replacement_app().await;
    let steps = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let handle = install_slow_envelope_decode(&mut app, 812, 256, &steps, &mut cx);
    assert!(PluginApp::has_runnable_typed_operations(&app), "a live envelope decode is runnable reactor-turn work");
    drain_envelope_decode(&mut app, handle, &mut cx).await;
    let driven = steps.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(app.poll_artifact_envelope_decode(handle), semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll::Ready, "the reactor-turn pump left the decode ready after {driven} steps");
    assert!(driven >= 256, "the reactor-turn pump drove only {driven} decode steps");
    assert!(!PluginApp::has_runnable_typed_operations(&app), "a retired decode is no longer runnable reactor-turn work");
    app.cancel_artifact_envelope_load(handle).expect("cancel envelope law decode before fixture close");
    for _ in 0..10_000 {
        let demand = app.maintenance_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("original maintenance demand");
        let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth };
        assert!(grant.maximum_copy_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES && grant.maximum_release_bytes <= store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
        let step = app.maintenance_step(&mut cx,grant).expect("envelope law decode cancellation maintenance");
        if let Some(progress) = step.progress() { assert!(progress.fits(grant)); }
        if let Err(source)=semio_framework_job_worker::pump_worker_job_retirements(8,&mut cx){close_composition_refusal(source.into(),&mut cx)}
    }
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}
//#endregion 📨️EnvelopeDecodeLadder

#[test]
fn declared_child_projection_override_preserves_authored_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧩️composition/🔎️projection/🧪️tests/🧫️fixtures/🔣️.json"))).expect("closed projection fixture");
    let refusal = &fixture["overrideRefusal"];
    let snapshot = ComposedParentSnapshot { slot: None, revision: refusal["revision"].as_i64().expect("revision") as i32 };
    assert!(semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot).expect("structural leaf").is_empty());
    let actual = <ComposedParentApp<false> as ArtifactApp>::child_restore_projection(&snapshot).expect_err("authored nested override must be selected");
    assert_eq!(actual.code.0, refusal["code"].as_str().expect("original code"));
    assert_eq!(actual.message, refusal["message"].as_str().expect("original message"));
}
