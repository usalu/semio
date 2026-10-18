/// 🧰️ Supersede replay harness fixtures: the identity byte ceiling, the hydration grant, the demo owners, the author, planning and retirement policies, the finalize helper and the apply, set, add, draft and outcome helpers.

const FIXTURE_IDENTITY_BYTE_CEILING: usize = 201 * semio_framework_job::JOB_PAYLOAD_PAGE_BYTES;

fn fixture_hydration_grant()->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:65536,maximum_depth:128}}

fn fixture_demo_owners()->DocumentStoreOwners<DemoSnapshot,DemoMutation> {
    let grant=fixture_hydration_grant();
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new().member_store_owners(grant));
    let(owners,receipt)=match result{Ok(admitted)=>admitted,Err(refusal)=>panic!("declared original fixture catalog refused: {:?}",refusal.error)};
    assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));complete_test_catalog(Ok::<_,DocumentStoreOwnersAdmissionError<DemoSnapshot,DemoMutation>>((owners,receipt)))
}

pub(super) async fn fixture_author<P, M>(store: &mut ArtifactStore<P, M>, command: ArtifactCommand<M>) -> Result<CommandReceipt, VcsError>
where
    P: Clone + ToValue + FromValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static,
    M: Clone + ToValue + FromValue + Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let mut observed = 0;
    let mut observer = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
        assert!(progress.owned_bytes <= FIXTURE_IDENTITY_BYTE_CEILING);
        observed = progress.owned_bytes;
        true
    };
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_> = semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(FIXTURE_IDENTITY_BYTE_CEILING, &mut observer).expect("declared fixture identity authority");
    let result = store.dispatch(command, &mut identity).await;
    drop(identity.pause().expect("original cumulative fixture identity receipt"));
    drop(observer);
    result
}

fn fixture_planning_policy()->RetainedCloneGrant { RetainedCloneGrant { maximum_copy_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES,..physical_test_close_grant() } }

/// 🎟️ Finalizes the original replay through the fixture caller's existing identity authority.
async fn fixture_finalize<P,M>(store:&mut ArtifactStore<P,M>,finished:EditReplayResult<P,M>,finalization:HistoryFinalization)->Result<CommandReceipt,VcsError>
where P:Clone+ToValue+FromValue+ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot+Send+Sync+'static,M:Clone+ToValue+FromValue+Mutation<P>+OpBinary+OpText+Send+'static {
    let mut observed=0;
    let mut observer=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{assert!(progress.owned_bytes<=FIXTURE_IDENTITY_BYTE_CEILING);observed=progress.owned_bytes;true};
    let mut identity:semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>=semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::<semio_framework_os_vcs::io::binary::entity_identity::control::Observer<'_>>::new(FIXTURE_IDENTITY_BYTE_CEILING,&mut observer).expect("original fixture finalization identity authority");
    let result=store.commit_finished_replay(finished,finalization,&mut identity).await;
    drop(identity.pause().expect("original fixture finalization cumulative identity receipt"));drop(observer);
    result
}

fn fixture_retirement_policy(maximum_items:usize,maximum_release_bytes:usize)->RetainedCloneGrant { RetainedCloneGrant { maximum_items:maximum_items.min(1),maximum_release_bytes,..physical_test_close_grant() } }

//#region 🧰️Harness
async fn demo_store(id: &str, n: Option<i32>) -> ArtifactStore<DemoSnapshot, DemoMutation> {
    ArtifactStore::new(create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", id, DemoSnapshot { n }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer), &semio_framework_os_store_testing_doubles::DemoMemberStoreProducer::<_>::new()).await
}

async fn apply(store: &mut ArtifactStore<DemoSnapshot, DemoMutation>, mutations: Vec<DemoMutation>) {
    fixture_author(store, ArtifactCommand::Apply { mutations, transaction: None }).await.expect("a clean edit applies");
}

fn set(n: i32) -> DemoMutation {
    DemoMutation::SetN(SetN { n })
}

fn add(delta: i32) -> DemoMutation {
    DemoMutation::AddN(AddN { delta })
}

fn operation_ids(store: &ArtifactStore<DemoSnapshot, DemoMutation>) -> Vec<MutationId> {
    store.mutation_ops().expect("applied operations").into_iter().map(|operation| operation.mutation_id).collect()
}

fn input(target: &MutationId, replacement: Option<DemoMutation>) -> SupersedeInput<DemoMutation> {
    SupersedeInput { target: target.clone(), replacement }
}

fn draft(replacement: Option<DemoMutation>) -> protocol::InputReplacement {
    match replacement {
        Some(operation) => protocol::InputReplacement::Input { schema: "demo/v1".into(), payload: operation.encode_op().expect("demo operations encode") },
        None => protocol::InputReplacement::Withdrawn,
    }
}

/// 🔡️ The bytes a lowercase hex string spells.
fn hex(text: &str) -> Vec<u8> {
    text.as_bytes().chunks(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("ascii hex"), 16).expect("hex digit pair")).collect()
}

fn drafts(entries: &[(&MutationId, Option<DemoMutation>)]) -> protocol::HistoryInputDrafts {
    entries.iter().map(|(target, replacement)| ((*target).clone(), draft(replacement.clone()))).collect()
}

fn finish(store: &ArtifactStore<DemoSnapshot, DemoMutation>, mut replay: EditReplay<DemoSnapshot, DemoMutation>) -> EditReplayResult<DemoSnapshot, DemoMutation> {
    drive_test_report_replay(&mut replay, store.replay_edits());
    replay.finish().expect("a finished replay yields its result")
}

/// 📋️ Per-mutation outcomes keyed by identity: what replicas with different edit boundaries must agree on.
fn outcomes_by_mutation(outcomes: &[protocol::MutationReplayOutcome]) -> BTreeMap<MutationId, (Option<semio_framework_diagnostic::Severity>, Vec<String>, bool, bool)> {
    outcomes.iter().map(|outcome| (outcome.mutation_id.clone(), (outcome.worst, outcome.messages.iter().map(|message| message.code.0.clone()).collect(), outcome.superseded, outcome.withdrawn))).collect()
}

/// 📋️ Per-mutation outcomes by applied position and operation index: what two independently authored stores agree on.
fn outcomes_by_position(store: &ArtifactStore<DemoSnapshot, DemoMutation>, outcomes: &[protocol::MutationReplayOutcome]) -> Vec<(usize, u32, Option<semio_framework_diagnostic::Severity>, Vec<String>, bool, bool)> {
    outcomes
        .iter()
        .map(|outcome| {
            (
                store.applied_edit_ids().iter().position(|id| *id == outcome.edit_id).expect("an outcome names an applied edit"),
                outcome.op_index,
                outcome.worst,
                outcome.messages.iter().map(|message| message.code.0.clone()).collect(),
                outcome.superseded,
                outcome.withdrawn,
            )
        })
        .collect()
}
//#endregion 🧰️Harness
