#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetTransactionCount, SetTransactionCountAndNotify, SetTransactionCountWithoutPreflight, TxnMutation};

#[cfg(test)]
#[path = "🧪️tests/🧪️command-close/🦀️.rs"]
mod command_close_tests;

// 🧪️ Proves the `🧪️testkit` transaction helpers and the underlying transaction machinery
// against a minimal `ArtifactApp` fixture whose notify mutation carries a real foreign step.
use crate::app::{built_text_to_component_tree, ArtifactApp, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolCompletion, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, ConfigView, DraftView, Emit, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, PluginApp, UiAssemblyResult, VcsArtifactApp};
use crate::app::testkit::{assert_proposes_transaction, assert_transaction_commits_as_one_edit, assert_transaction_rollback_leaves_state_untouched, meta, new_registered_app};
use protocol::{Mutation, MutationDiff};
use semio_framework::{ActionKind, Fault, IconName, ToolFactoryKey, ToolJobFactory, ToolOperationSpec, ToolExecutionContract};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use store::{Backbone, BackboneMessage, EngineHandles, MemoryBackbone};
use ui_wgpu::wgpu::LocalizedLabel;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(extension = "testkit-txn")]
pub(crate) struct TxnSnapshot {
    count: i32,
}

impl store::ArtifactDsl for TxnSnapshot {
    const EXTENSION: &'static str = "testkit-txn";
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl store::ArtifactPack for TxnSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct TxnDiff {
    count: Option<i32>,
}

impl MutationDiff<TxnSnapshot> for TxnDiff {
    fn apply(&self, snapshot: &TxnSnapshot) -> protocol::MutationApplyResult<TxnSnapshot> {
        Ok(TxnSnapshot { count: self.count.unwrap_or(snapshot.count) })
    }
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
enum TxnCommand {
    #[dsl(key = "increment")]
    Increment,
    #[dsl(key = "coalesced-increment")]
    CoalescedIncrement,
    #[dsl(key = "increment-and-notify")]
    IncrementAndNotify,
}

impl ::protocol::OpText for TxnCommand {
    fn parse_op(line: &str) -> Result<Self, ::store::TextError> {
        let variants = <Self as ::dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = ::dsl::parse(body, &spec_fn(), &::dsl::ParseOptions { limits: ::dsl::Limits::default(), mode: ::dsl::SourceMode::Inline })?;
                return <Self as ::dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(::dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as ::dsl::DslVariants>::to_named_record(self);
        let variants = <Self as ::dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl ::protocol::OpBinary for TxnCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &["increment", "coalesced-increment", "increment-and-notify"];

    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

//#region 🧪️TransactionRegisteredFactory
const TXN_PAYLOAD_SCHEMA: &str = "semio.testkit-txn.command.v1";
const TXN_TOOL_IDS: [&str; 3] = ["increment", "coalesced-increment", "increment-and-notify"];

struct TxnFixtureJob {
    command: Option<Box<TxnCommand>>,
    completion: Option<ArtifactToolCompletion<TxnApp>>,
    count: i32,
    closing: bool,
}

impl semio_framework_job::InteractiveJob for TxnFixtureJob {
    fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> semio_framework_job::StepOutcome {
        if cx.is_cancelled() { return semio_framework_job::StepOutcome::Cancelled; }
        if cx.should_yield() { return semio_framework_job::StepOutcome::Yield; }
        let Some(command) = self.command.as_deref() else { return semio_framework_job::StepOutcome::Cancelled; };
        let value = self.count + 1;
        let emit = match command {
            TxnCommand::Increment => Emit { artifact_mutations: vec![SetTransactionCountWithoutPreflight { value }.into()], description: Some("increment".into()), ..Default::default() },
            TxnCommand::CoalescedIncrement => Emit { artifact_mutations: vec![SetTransactionCountWithoutPreflight { value }.into()], description: Some("coalesced-increment".into()), coalesce_key: Some("counter".into()), ..Default::default() },
            TxnCommand::IncrementAndNotify => Emit { artifact_mutations: vec![SetTransactionCountAndNotify { value }.into()], description: Some("increment-and-notify".into()), ..Default::default() },
        };
        self.completion.as_ref().expect("transaction fixture completion").complete(Ok(emit), crate::app::EphemeralEmit::default()).expect("one exact transaction completion");
        semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing || maximum_items == 0 { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
        if let Some(command) = self.command.as_deref() {
            let released_bytes = std::mem::size_of_val(command);
            if maximum_bytes < released_bytes { return semio_framework_job::InteractiveJobCloseStep::Blocked; }
            drop(self.command.take());
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if self.completion.take().is_some() { return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }; }
        semio_framework_job::InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool { self.closing && self.command.is_none() && self.completion.is_none() }
}

struct TxnFixtureFactory { keys: Vec<ToolFactoryKey> }

impl ToolJobFactory for TxnFixtureFactory {
    type Payload = TxnFixtureJob;
    type Job = TxnFixtureJob;
    fn keys(&self) -> &[ToolFactoryKey] { &self.keys }
    fn payload_schema_id(&self) -> &str { TXN_PAYLOAD_SCHEMA }
    fn classification(&self) -> semio_framework::InteractiveJobClassification { semio_framework::InteractiveJobClassification::Migrated }
    fn execution_contract(&self) -> ToolExecutionContract { ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1) }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> { Ok(payload) }
}

impl ArtifactOwnedToolJobFactory for TxnFixtureFactory {
    type Owner = TxnApp;
    const TOOL_IDS: &'static [&'static str] = &TXN_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = TxnApp::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[
        ArtifactToolPublicationContract { tool_id: "increment", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "coalesced-increment", lanes: &[ArtifactToolPublicationLane::Artifact] },
        ArtifactToolPublicationContract { tool_id: "increment-and-notify", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ];
    fn latest_wins_target(_command: &TxnCommand) -> Option<&str> { None }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<TxnCommand>>> { None }
}

async fn transaction_manifest() -> crate::app::App {
    let mut builder = crate::app::App::builder(TxnApp::APP_ID, LocalizedLabel::data("Transaction Fixture"))
        .await
        .document(["state"])
        .mode("edit", LocalizedLabel::data("Edit"), "pencil")
        .await
        .window_kind("main", LocalizedLabel::data("Main"), "transaction.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, IconName::AppWindow)
        .await;
    for tool_id in TXN_TOOL_IDS {
        builder = builder.app_command(tool_id, LocalizedLabel::data(tool_id), "fixture", ActionKind::Mutation).await;
    }
    crate::app::App::from_builder(builder.interactive_jobs(semio_framework::InteractiveJobClassification::Migrated).await).await
}
//#endregion 🧪️TransactionRegisteredFactory

#[derive(Default)]
struct TxnApp;

impl ArtifactApp for TxnApp {
    const APP_ID: &'static str = "testkit-txn";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-txn/v1";
    type Snapshot = TxnSnapshot;
    type Mutation = TxnMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app::NoTransient;
    type TransientMutation = crate::app::NoTransientMutation;
    type Command = TxnCommand;

    crate::bounded_first_step_tool_proofs! {
        owner: TxnApp, owner_file: "plugin/🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs", controller: "testkit-txn", document_schema: "semio.testkit-txn/v1",
        factory: "TxnFixtureFactory", factory_type: TxnFixtureFactory,
        contract: ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1), tools: ["increment", "coalesced-increment", "increment-and-notify"]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, Self>) -> Result<(), Fault> {
        registry.register(TxnFixtureFactory { keys: TXN_TOOL_IDS.into_iter().map(|tool_id| ToolFactoryKey::new(registry.controller_id(), tool_id)).collect() })
    }

    async fn build_tool_job(request: ArtifactOwnedToolJobRequest<Self>) -> Result<Option<ToolOperationSpec>, Fault> {
        let job = TxnFixtureJob { command: Some(request.command), completion: Some(request.completion), count: request.snapshot.count, closing: false };
        Ok(Some(ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, job, request.operation)))
    }

    async fn initial_snapshot() -> TxnSnapshot {
        TxnSnapshot::default()
    }

    async fn handle(
        command: &TxnCommand,
        doc: &ArtifactView<'_, TxnSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app::InteractionView<'_>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<TxnMutation>, Fault> {
        match command {
            TxnCommand::Increment => Ok(Emit { artifact_mutations: vec![SetTransactionCountWithoutPreflight { value: doc.snapshot.count + 1 }.into()], description: Some("increment".into()), ..Default::default() }),
            TxnCommand::CoalescedIncrement => {
                Ok(Emit { artifact_mutations: vec![SetTransactionCountWithoutPreflight { value: doc.snapshot.count + 1 }.into()], description: Some("coalesced-increment".into()), coalesce_key: Some("counter".into()), ..Default::default() })
            }
            TxnCommand::IncrementAndNotify => Ok(Emit { artifact_mutations: vec![SetTransactionCountAndNotify { value: doc.snapshot.count + 1 }.into()], description: Some("increment-and-notify".into()), ..Default::default() }),
        }
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TxnSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)))
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::app::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(crate::app::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> { Some(crate::app::bounded_document_store_owners::<Self::Draft, Self::DraftMutation>()) }
    fn build_document_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> { Some(crate::app::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>()) }
    fn build_config_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> { Some(crate::app::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>()) }
    fn build_draft_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> { Some(crate::app::bounded_document_store_disposer::<Self::Draft, Self::DraftMutation>()) }
    fn build_presence_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> { Some(crate::app::mutation_fixture::no_state::presence_store_disposer()) }
    fn build_transient_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> { Some(crate::app::mutation_fixture::no_state::transient_store_disposer()) }
    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_peer_retirement_factory()) }
    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> { Some(crate::app::mutation_fixture::no_state::presence_local_root_retirement_factory()) }
    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> { Some(crate::app::mutation_fixture::no_state::transient_local_root_retirement_factory()) }
}

fn close_transaction_store_roots(app: &mut VcsArtifactApp<TxnApp>) {
    for _ in 0..64 {
        if app.close_terminal_is_empty() { return; }
        match app.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("fixture app close") {
            crate::app::PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES),
            crate::app::PluginCloseStep::Blocked { reason } => panic!("fixture close blocked: {reason}"),
            crate::app::PluginCloseStep::Complete => break,
        }
    }
    assert!(app.close_terminal_is_empty(), "transaction fixture must retire every exact store owner");
}

#[semio_framework_async_macros::async_test]
async fn dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    let draft = assert_proposes_transaction(&mut app, TxnCommand::IncrementAndNotify).await;
    assert_eq!(draft.local_ops.len(), 1, "the local op must still be encoded for the proposal");
    assert_eq!(draft.foreign.len(), 1, "the foreign step must be reported");
    assert_eq!(draft.foreign[0].target.artifact_id, "peer-doc");
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn plain_command_still_applies_normally() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    app.dispatch_typed(TxnCommand::Increment, &meta("local")).await.expect("increment");
    assert_eq!(app.snapshot().unwrap().count, 1);
    assert!(app.take_pending_transaction_proposal().await.is_none(), "a plain command must not stash a proposal");
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn command_cache_inputs_share_immutable_arcs() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    app.refresh_cache().await.expect("refresh cache");
    let (_, cached_snapshot, cached_config, cached_history) = app.cache.as_ref().expect("cache");
    let (snapshot, config, history) = app.command_cache_inputs();
    assert!(std::sync::Arc::ptr_eq(cached_snapshot, &snapshot));
    assert!(std::sync::Arc::ptr_eq(cached_config, &config));
    assert!(std::sync::Arc::ptr_eq(cached_history, &history));
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn amended_edit_extends_cached_history_in_place() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    app.dispatch_typed(TxnCommand::CoalescedIncrement, &meta("local")).await.expect("first increment");
    let history_ptr = std::sync::Arc::as_ptr(&app.cache.as_ref().expect("first history cache").3);
    app.dispatch_typed(TxnCommand::CoalescedIncrement, &meta("local")).await.expect("second increment");
    app.refresh_cache().await.expect("extend history cache");
    let history = &app.cache.as_ref().expect("extended history cache").3;
    assert_eq!(std::sync::Arc::as_ptr(history), history_ptr, "an amend must update the uniquely-owned history allocation in place");
    assert_eq!(app.store.envelope().vcs.edits.len(), 1, "coalesced increments stay one undo edit");
    assert_eq!(history.commands.len(), 1, "coalesced increments stay one command row");
    assert_eq!(history.commands[0].op_lines.len(), 2, "only the new operation tail is appended to cached history");
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn commit_produces_exactly_one_edit_with_group_id_and_origin() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    let origin = protocol::MutationOrigin::Transaction { initiator: protocol::ForeignTarget { artifact_id: "initiator-doc".into(), artifact_kind: "s.testkit.txn".into(), dialect: None } };
    let edit_id = assert_transaction_commits_as_one_edit(&mut app, "txn-1", vec![SetTransactionCount { value: 7 }.into()], "peer-write", origin).await;
    assert_eq!(app.snapshot().unwrap().count, 7);
    assert!(!edit_id.is_empty());
    assert!(app.transaction_commit("txn-1", &meta("local")).await.is_err(), "committing an already-committed txn_id must fail, not double-apply");
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn rollback_leaves_state_untouched() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    app.dispatch_typed(TxnCommand::Increment, &meta("local")).await.expect("increment");
    assert_transaction_rollback_leaves_state_untouched(&mut app, "txn-2", vec![SetTransactionCount { value: 99 }.into()], "peer-write").await;
    assert_eq!(app.snapshot().unwrap().count, 1, "rollback must leave the earlier state exactly as it was");
    close_transaction_store_roots(&mut app);
}

/// 🔀️ Contract §5.8 and §5.10 are COMPLEMENTARY, and this test is where that shows: §5.10
/// already blocks every local mutating command while a transaction is pending, so a local
/// edit can never be what moves the generation out from under a prepared member. The only
/// remaining way for the base generation to go stale is an edit that does not come from a
/// command at all — a remote envelope ingested from the backbone mid-transaction — which is
/// precisely the race §5.8's check exists to catch. Driving this through `dispatch_typed`
/// instead only proves §5.10 a second time (it rejects with `transaction.instance-busy`).
#[semio_framework_async_macros::async_test]
async fn generation_mismatch_is_rejected_with_the_frozen_code() {
    let mut sender = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    let (near, mut far) = MemoryBackbone::pair("mem://txn", "mem://txn").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
    sender.dispatch_typed(TxnCommand::Increment, &meta("remote")).await.expect("the peer edits its own copy");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the peer's edit must reach the channel");
    let operations = protocol::encode_envelopes(&envelopes);

    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    let outcome = app.transaction_prepare("txn-3", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 5 })).expect("encode")], "peer-write", Some(protocol::MutationOrigin::Owner)).await;
    assert!(outcome.rejection.is_none());
    app.ingest_operations(&operations).await.expect("a remote edit lands while the transaction is pending");
    let error = app.transaction_commit("txn-3", &meta("local")).await.expect_err("commit must reject a stale generation");
    assert_eq!(error.code.0, "transaction.generation-mismatch");
    close_transaction_store_roots(&mut sender);
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn second_prepare_while_pending_is_rejected_instance_busy() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    let first = app.transaction_prepare("txn-4a", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 1 })).expect("encode")], "first", Some(protocol::MutationOrigin::Owner)).await;
    assert!(first.rejection.is_none());
    let second = app.transaction_prepare("txn-4b", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 2 })).expect("encode")], "second", Some(protocol::MutationOrigin::Owner)).await;
    let rejection = second.rejection.expect("second prepare while pending must be rejected");
    assert_eq!(rejection.code.0, "transaction.instance-busy");
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn a_mutating_command_while_pending_is_rejected_but_reads_still_work() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    let prepared = app.transaction_prepare("txn-5", "", &[], &[::protocol::OpBinary::encode_op(&TxnMutation::from(SetTransactionCount { value: 1 })).expect("encode")], "peer-write", Some(protocol::MutationOrigin::Owner)).await;
    assert!(prepared.rejection.is_none());
    let blocked = app.dispatch_typed(TxnCommand::Increment, &meta("local")).await;
    assert!(blocked.is_err(), "a command emitting artifact mutations must be rejected while a transaction is pending");
    assert_eq!(blocked.unwrap_err().code.0, "transaction.instance-busy");
    // 🔖️ Read-only surfaces stay unaffected — `render`/`snapshot` never go through
    // `dispatch_emit` at all, matching contract §5.10's carve-out for
    // RefreshUi/ReadDocument/ContextMenu/ephemeral lanes.
    assert_eq!(app.snapshot().unwrap().count, 0, "the pending transaction must not have applied anything yet");
    close_transaction_store_roots(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn undo_and_redo_by_group() {
    let mut app = new_registered_app::<TxnApp, _>(transaction_manifest()).await;
    assert_transaction_commits_as_one_edit(&mut app, "txn-6", vec![SetTransactionCount { value: 42 }.into()], "peer-write", protocol::MutationOrigin::Owner).await;
    assert_eq!(app.snapshot().unwrap().count, 42);
    app.transaction_undo("txn-6").await.expect("undo the group");
    assert_eq!(app.snapshot().unwrap().count, 0, "undo must revert the transaction's edit");
    app.transaction_redo("txn-6").await.expect("redo the group");
    assert_eq!(app.snapshot().unwrap().count, 42, "redo must reapply the transaction's edit");
    close_transaction_store_roots(&mut app);
}
