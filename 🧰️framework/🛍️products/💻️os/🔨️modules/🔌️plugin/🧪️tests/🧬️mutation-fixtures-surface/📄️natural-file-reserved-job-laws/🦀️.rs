/// 📄️ Natural file reserved job laws of the surface fixture: the reserved job builder with its policies, step, outcome and close helpers and the accounting, cancellation, monolithic decoder and controlled decoder laws.

fn surface_natural_file_reserved_job(
    snapshot_read: store::os_store::snapshot_read::SnapshotRead<SurfaceSnapshot>,
    data: Vec<u8>,
    maximum_work_units_per_step: u64,
) -> (
    ArtifactReservedToolJob,
    ArtifactToolCompletion<EditorApp<SurfaceEditorFixture>>,
    std::sync::Arc<SurfaceSnapshot>,
    std::sync::Arc<HistoryView>,
) {
    let snapshot = std::sync::Arc::new(SurfaceSnapshot::default());
    let history = std::sync::Arc::new(HistoryView::empty());
    let completion = ArtifactToolCompletion::<EditorApp<SurfaceEditorFixture>>::new();
    let request = ArtifactReservedToolJobRequest {
        retained: SURFACE_ONE_UNIT_POLICY,
        controller_id: SURFACE_CONTROLLER_ID.into(),
        tool_id: "framework.reserved.import-media".into(),
        payload_schema_id: "semio.testkit-surface-natural/v1".into(),
        contract: ToolExecutionContract::resumable(8_192, 8_192, maximum_work_units_per_step, 8_192, 7_500, 1, 1),
        app_instance_id: 7,
        ui_axes: None,
        parent_document_id: "surface-natural-file-fixture".into(),
        canonical_base_revision: [0; 32],
        authoring_seed: "surface-natural-file-fixture-admission".into(),
        snapshot: snapshot.clone(),
        snapshot_read,
        config: std::sync::Arc::new(NoConfig::default()),
        history: history.clone(),
        children: ChildContentView::EMPTY,
        raw_wire: Vec::new(),
        input: ArtifactReservedToolInput::Media {
            port: NATURAL_FILE_PORT.into(),
            media: Media {
                media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                payload: MediaPayload::Intrinsic {
                    schema: "semio.testkit-surface-natural/v1".into(),
                    value: semio_framework_value::DslValue::Bytes(data),
                },
            },
        },
        completion: completion.clone(),
    };
    let job = <SurfaceEditorFixture as ArtifactEditor>::build_reserved_tool_job(request).expect("natural-file reserved builder").expect("mounted natural-file codec owns its reserved route");
    (job, completion, snapshot, history)
}

const SURFACE_JOB_POLICY:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64};
const SURFACE_ONE_UNIT_POLICY:semio_framework_job::RetainedCloneGrant=semio_framework_job::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:262144,maximum_release_bytes:1048576,maximum_depth:4096};
fn surface_natural_file_step(job:&mut ArtifactReservedToolJob,cx:&mut semio_framework_job::StepContext<'_>)->crate::app_testing::artifact_app_laws::FixtureStepOutcome{
    let mut outcome=None;cx.with_retained_loan(SURFACE_JOB_POLICY,|same|{outcome=Some(crate::app_testing::artifact_app_laws::fixture_step(job,same));Ok(())}).expect("original surface job event");outcome.expect("original surface event returned")
}
fn surface_natural_file_outcome(job:&mut ArtifactReservedToolJob,cx:&mut semio_framework_job::StepContext<'_>)->crate::app_testing::artifact_app_laws::FixtureStepOutcome{
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){let outcome=surface_natural_file_step(job,cx);if !matches!(outcome,crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle|crate::app_testing::artifact_app_laws::FixtureStepOutcome::Yield){return outcome}}
    panic!("original surface job retained its outcome frontier")
}
fn close_surface_natural_file_job(job:&mut ArtifactReservedToolJob,cx:&mut semio_framework_job::StepContext<'_>){
    use semio_framework_job::{InteractiveJob as Job,InteractiveJobCloseStep as Step,RetainedCloneProgress};
    Job::begin_close(job);let zero=semio_framework_job::RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};assert_eq!(cx.with_retained_loan(zero,|same|Job::close_step(job,same)).unwrap().progress(),RetainedCloneProgress::default());
    for _ in 0..crate::app_testing::artifact_app_laws::OriginalFixtureCaller::maximum_turns(){
        if Job::terminal_is_empty(job){break}
        let demand=Job::next_close_release_byte_demand(job).expect("original Surface release demand");assert_eq!(Job::next_close_release_byte_demand(job).unwrap(),demand);
        if demand>64{let denied=semio_framework_job::RetainedCloneGrant{maximum_release_bytes:64,..SURFACE_JOB_POLICY};let step=cx.with_retained_loan(denied,|same|Job::close_step(job,same)).expect("original short-release close");assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(Job::next_close_release_byte_demand(job).unwrap(),demand);assert!(!Job::terminal_is_empty(job));}
        let step=cx.with_retained_loan(SURFACE_JOB_POLICY,|same|Job::close_step(job,same)).expect("original surface full close");assert!(step.progress().fits(SURFACE_JOB_POLICY));match step{Step::Pending{progress}=>assert!(progress!=RetainedCloneProgress::default(),"original Surface fixed policy made no progress"),Step::Complete{..}=>{},other=>panic!("original Surface fixed policy close retained its frontier: {other:?}")}
    }
    assert!(Job::terminal_is_empty(job),"original Surface exact terminal witness")
}

/// 🧵️ A mounted natural-file importer accounts owned input bytes across turns, observes
/// cancellation before decode, and refuses a monolithic decoder whose input exceeds its exact
/// per-turn grant. Both exits retire every retained owner with bounded positive progress.
#[semio_framework_async_macros::async_test]
async fn natural_file_reserved_job_accounts_cancels_refuses_and_retires_boundedly() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["registeredEditor"];
    let data = row["octets"].as_array().expect("registered editor octets").iter().map(|octet| octet.as_u64().expect("octet") as u8).collect::<Vec<_>>();
    let turn = row["accountBytesPerTurn"].as_u64().expect("account turn bytes");
    let cancel_after = row["cancelAfterAccountedBytes"].as_u64().expect("cancelled accounted bytes");
    let (mut cancelled, cancelled_completion, cancelled_snapshot, cancelled_history) = surface_natural_file_reserved_job(app.stores.store.snapshot_read().expect("surface store read"), data,turn);
    let mut first = crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle;
    for _ in 0..64 {
        first = surface_natural_file_step(&mut cancelled,cx);
        if !matches!(first, crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle) { break; }
    }
    match first {
        crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { applied_progress, .. } => {
            assert_eq!(applied_progress, cancel_after);
        }
        other => panic!("first bounded natural-file turn must checkpoint accounted bytes, got {other:?}"),
    }
    cx.original_cancel_token().cancel_now();
    assert!(matches!(
        surface_natural_file_step(&mut cancelled,cx),
        crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled
    ));
    drop((cancelled_snapshot, cancelled_history));
    close_surface_natural_file_job(&mut cancelled,cx);
    crate::app_testing::mutation_fixture::job_close::retire_completion(cancelled_completion,cx,SURFACE_JOB_POLICY);

    close_registered_fixture_app(&mut app,crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).unwrap();
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).unwrap();result
}

#[semio_framework_async_macros::async_test]
async fn natural_file_monolithic_decoder_refuses_and_retires_on_original_caller(){
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().unwrap();let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app=new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),crate::app_testing::artifact_app_laws::fixture_mounted_policy(),&mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).unwrap();let row=&fixture["registeredEditor"];
    let oversized = usize::try_from(row["oversizedOctetCount"].as_u64().expect("oversized octet count")).expect("usize octet count");
    let maximum = u64::try_from(oversized - 1).expect("bounded maximum work");
    let (mut refused, refused_completion, refused_snapshot, refused_history) = surface_natural_file_reserved_job(app.stores.store.snapshot_read().expect("surface store read"), vec![0; oversized], maximum);
    for expected in [maximum, u64::try_from(oversized).expect("oversized progress")] {
        match surface_natural_file_outcome(&mut refused,cx) {
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { applied_progress, .. } => {
                assert_eq!(applied_progress, expected);
            }
            other => panic!("natural-file accounting must checkpoint monotonically, got {other:?}"),
        }
    }
    match surface_natural_file_outcome(&mut refused,cx) {
        crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(bytes) => {
            let detail = String::from_utf8(bytes).expect("UTF-8 refusal");
            assert_eq!(detail, "natural-file codec requires a controlled decoder for this input size");
        }
        other => panic!("an oversized monolithic natural-file decoder must refuse deterministically, got {other:?}"),
    }
    close_surface_natural_file_job(&mut refused,cx);
    assert!(refused_completion.take_emit().expect("refused completion cell").is_none());
    drop((refused_snapshot, refused_history));
    crate::app_testing::mutation_fixture::job_close::retire_completion(refused_completion,cx,SURFACE_JOB_POLICY);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}

/// 🧭️ A format-owned decoder consumes a source larger than one scheduler grant over
/// monotonic checkpoints, publishes the independently computed value once, then retires its sole
/// source owner under the ordinary reserved-job close contract.
#[semio_framework_async_macros::async_test]
async fn natural_file_controlled_decoder_crosses_turns_and_retires_its_owner() {
    let mut original_caller=crate::app_testing::artifact_app_laws::OriginalFixtureCaller::admit().expect("original caller policy admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>(protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), crate::app_testing::artifact_app_laws::fixture_mounted_policy(), &mut crate::app_testing::artifact_app_laws::fixture_identity()).await;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json")).expect("natural-file lifecycle fixture");
    let row = &fixture["controlledEditor"];
    let word_count = usize::try_from(row["wordCount"].as_u64().expect("controlled word count")).expect("usize word count");
    let last_word = i32::try_from(row["lastWord"].as_i64().expect("controlled last word")).expect("i32 last word");
    let work = row["maximumWorkUnitsPerStep"].as_u64().expect("controlled work grant");
    let expected_minimum_turns = usize::try_from(row["expectedMinimumTurns"].as_u64().expect("minimum turns")).expect("usize turns");
    let expected_decoded_items = usize::try_from(row["expectedDecodedItems"].as_u64().expect("decoded items")).expect("usize decoded items");
    assert_eq!(word_count, expected_decoded_items);
    let mut data = Vec::with_capacity(word_count * 4);
    for word in std::iter::repeat_n(0_i32, word_count - 1).chain(std::iter::once(last_word)) {
        data.extend_from_slice(&word.to_be_bytes());
    }
    let oracle = data.chunks_exact(4).map(|word| i32::from_be_bytes(word.try_into().expect("four-byte word"))).try_fold(0_i32, i32::checked_add).expect("independent checked sum");
    assert_eq!(oracle, last_word);

    let (mut job, completion, snapshot, history) = surface_natural_file_reserved_job(app.stores.store.snapshot_read().expect("surface store read"), data, work);
    let mut checkpoints = 0;
    let mut last_progress = 0;
    loop {
        match surface_natural_file_step(&mut job,cx) {
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Checkpoint { applied_progress, .. } => {
                assert!(applied_progress >= last_progress, "controlled decoder checkpoints are monotonic");
                last_progress = applied_progress;
                checkpoints += 1;
            }
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Complete { .. } => {
                break;
            }
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Yield | crate::app_testing::artifact_app_laws::FixtureStepOutcome::Idle => {}
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Cancelled => panic!("controlled natural-file decode cancelled without a request"),
            crate::app_testing::artifact_app_laws::FixtureStepOutcome::Fault(bytes) => {
                let detail = String::from_utf8_lossy(&bytes).into_owned();
                panic!("controlled natural-file decode faulted: {detail}");
            }
            other => panic!("unexpected controlled natural-file outcome: {other:?}"),
        }
    }
    assert!(checkpoints >= expected_minimum_turns, "fixture must cross many scheduler turns");
    let (emit, _) = completion.take_emit().expect("controlled completion cell").expect("controlled completion value");
    let emit = emit.expect("controlled import emit");
    assert!(emit.artifact_mutations.is_empty(), "natural-file Open is a document load: it emits no mutation");
    let loads = emit.effects.iter().filter_map(|effect| match effect { semio_framework::kernel::Effect::LoadDocument { pack, .. } => Some(pack), _ => None }).collect::<Vec<_>>();
    assert_eq!(loads.len(), 1, "the controlled decoder publishes exactly one document load");
    assert_eq!(<SurfaceSnapshot as pack::snapshot::ArtifactPack>::decode_pack(loads[0]).expect("loaded surface pack"), SurfaceSnapshot { count: oracle });
    drop((snapshot, history));
    close_surface_natural_file_job(&mut job,cx);
    crate::app_testing::mutation_fixture::job_close::retire_completion(completion,cx,SURFACE_JOB_POLICY);
    close_registered_fixture_app(&mut app, crate::app_testing::artifact_app_laws::fixture_mounted_policy(),cx).expect("original fixture terminal close");

    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original caller paid terminal retirement");result
}
