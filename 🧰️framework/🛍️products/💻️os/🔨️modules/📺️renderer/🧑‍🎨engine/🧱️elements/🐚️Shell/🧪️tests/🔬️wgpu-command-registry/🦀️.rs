
use super::*;
use semio_framework::{
    ActionArgControl, ActionKind, AppDefinition, AppRole, ArtifactDialect, CommandDefinition, CommandOwnerAddress, ModeDefinition, Modes, PanelGroup, PanelTabDefinition, PanelTabKind, PluginManifest, WindowKindDefinition, WindowKinds,
};

fn test_app(commands: Vec<CommandDefinition>, mode_commands: Vec<CommandDefinition>) -> AppDefinition {
    AppDefinition {
        id: "test-app".into(),
        role: AppRole::Editor,
        dialect: ArtifactDialect { artifact_kind: "s.test.app".into(), standard: "1".into(), subset: "*".into() },
        label: LocalizedLabel::data("Test App"),
        breadcrumb: vec!["semio".into(), "test".into()],
        icon_id: None,
        controller_id: "test".into(),
        modes: Modes::one(ModeDefinition { id: "default".into(), label: LocalizedLabel::data("Default"), icon_id: "pencil".into(), tools: vec![], layout_id: None, commands: mode_commands }),
        default_mode_id: "default".into(),
        window_kinds: WindowKinds::try_from(vec![WindowKindDefinition {
            id: "main".into(),
            label: LocalizedLabel::data("Main"),
            body_key: "main.body".into(),
            surface_kind: ui_wgpu::wgpu::SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: Default::default(),
            actions: vec![],
            utilities: vec![],
            interactions: vec![],
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: vec![],
        }])
        .expect("non-empty"),
        panel_tabs: vec![PanelTabDefinition { kind: PanelTabKind::App("tab".into()), label: LocalizedLabel::data("Tab"), group: PanelGroup::Workbench, body_key: Some("tab.body".into()), children: vec![] }],
        keybindings: vec![],
        utilities: vec![],
        tools: vec![],
        commands,
        interactions: vec![],
        named_layouts: vec![],
        default_layout: None,
        terminologies: vec!["de".into()],
        terminology_breadcrumbs: HashMap::new(),
        introduction: None,
        tutorials: Vec::new(),
        dialogs: Vec::new(),
        media_inputs: Vec::new(),
        media_outputs: Vec::new(),
        artifact_kinds: Vec::new(),
        config: semio_framework_async::block_on(semio_framework::ConfigSpec::empty()),
        command_grammar: semio_framework_async::block_on(semio_framework::CommandGrammar::empty()),
        io: semio_framework::AppIo::default(),
    }
}

fn test_shell_state() -> ShellState {
    ShellState::new(Vec::new(), String::new())
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Default)]
struct DirectoryBootstrapFakeTransport {
    responses: std::sync::Arc<std::sync::Mutex<std::collections::VecDeque<semio_framework_os_kernel::os_directory::client::HttpResponse>>>,
    urls: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct DirectoryBootstrapFakeWs {
    close_code: Option<u16>,
}

#[cfg(not(target_arch = "wasm32"))]
impl DirectoryWsConnection for DirectoryBootstrapFakeWs {
    fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
        Ok(())
    }
    fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
        Ok(())
    }
    fn try_recv_text(&mut self) -> Result<semio_framework_os_kernel::os_directory::client::DirectoryWsPoll, TransportError> {
        Ok(self.close_code.take().map_or(semio_framework_os_kernel::os_directory::client::DirectoryWsPoll::Pending, |code| semio_framework_os_kernel::os_directory::client::DirectoryWsPoll::Closed(Some(code))))
    }
    fn close(&mut self) {}
}

#[cfg(not(target_arch = "wasm32"))]
impl DirectoryTransport for DirectoryBootstrapFakeTransport {
    type Ws = DirectoryBootstrapFakeWs;

    async fn http(
        &self,
        _ctx: &OperationContext,
        _method: semio_framework_os_kernel::os_directory::client::HttpMethod,
        url: &str,
        _bearer: Option<&str>,
        _body: Option<Vec<u8>>,
    ) -> Result<semio_framework_os_kernel::os_directory::client::HttpResponse, TransportError> {
        self.urls.lock().expect("fake directory urls").push(url.to_string());
        self.responses.lock().expect("fake directory responses").pop_front().ok_or_else(|| TransportError::Io("missing fake response".into()))
    }

    fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<semio_framework_os_kernel::os_directory::client::HttpResponse, TransportError> {
        Err(TransportError::Io("socket grant is outside the page fixture".into()))
    }

    fn open_ws(&self, _ctx: &OperationContext, _url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        Err(TransportError::Io("socket open is outside the page fixture".into()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn directory_page_json(after: u64, through: u64, has_more: bool, _receipt_seed: char) -> String {
    let mut page = semio_framework_os_kernel::os_directory::DirectoryEventPageV1 {
        schema: "semio.directory.event-page.v1".into(),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 9,
        after_seq_exclusive: after,
        through_seq_inclusive: through,
        has_more,
        events: Vec::new(),
        receipt_sha256: String::new(),
    };
    page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    store::os_pack::json::to_json_string(&page)
}

//#region 💡️InferencePortLaws
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_kernel::os_directory::schema::{
    GisMapInferenceApprovalReceiptV1, GisMapInferenceEventPageV1, GisMapInferenceJobReceiptV1, GisMapInferenceJobStateV1, GisMapInferencePreviewV1, GisMapInferenceProgressV1, GisMapInferenceProposalStateV1,
};

#[cfg(not(target_arch = "wasm32"))]
fn inference_test_scope() -> DocumentScope {
    DocumentScope { space_id: "sp-1".into(), document_id: "doc-1".into() }
}

#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_TEST_JOB: &str = "11111111111111111111111111111111";
#[cfg(not(target_arch = "wasm32"))]
const INFERENCE_TEST_HASH: &str = "9071779b724c67e0a45d5e23fddc8dbeb3d9b537936a4a14c293bc373960b130";

#[cfg(not(target_arch = "wasm32"))]
fn inference_test_receipt() -> GisMapInferenceJobReceiptV1 {
    GisMapInferenceJobReceiptV1 {
        schema: "semio.hub.inference-receipt/v1".into(),
        job_id: INFERENCE_TEST_JOB.into(),
        state: GisMapInferenceJobStateV1::Accepted,
        proposal_state: GisMapInferenceProposalStateV1::None,
        proposal_hash: None,
        cursor: 0,
        expires_at_ms: 1_700_000_060_000,
    }
}

#[cfg(not(target_arch = "wasm32"))]
/// 🗺️ The exact bounded rectangular preview the hub publishes beside an offered proposal.
#[cfg(not(target_arch = "wasm32"))]
fn inference_test_preview(job_id: &str, proposal_hash: &str) -> GisMapInferencePreviewV1 {
    GisMapInferencePreviewV1 {
        schema: "semio.hub.gis-map-inference-preview/v1".into(),
        job_id: job_id.to_string(),
        proposal_hash: proposal_hash.to_string(),
        region_id: format!("inference-{job_id}"),
        ring: [[1.0, 2.0], [3.0, 2.0], [3.0, 4.0], [1.0, 4.0], [1.0, 2.0]],
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn inference_test_page(state: GisMapInferenceJobStateV1, proposal_state: GisMapInferenceProposalStateV1, proposal_hash: Option<&str>, cancel_requested: bool, stale: bool) -> GisMapInferenceEventPageV1 {
    GisMapInferenceEventPageV1 {
        schema: "semio.hub.inference-events/v1".into(),
        job_id: INFERENCE_TEST_JOB.into(),
        state,
        proposal_state,
        cancel_requested,
        stale,
        proposal_hash: proposal_hash.map(str::to_string),
        preview: proposal_hash.map(|hash| inference_test_preview(INFERENCE_TEST_JOB, hash)),
        events: Vec::new(),
        progress: vec![GisMapInferenceProgressV1 { cursor: 1, run_epoch: 1, completed: 1, total: 4, at_ms: 1_700_000_001_000 }],
        next_cursor: 1,
    }
}

/// 🪪️ The lease precondition is the very first thing the driver checks: with no verified
/// execution target it never asks for a single call and reports the localized terminal.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_refuses_to_start_without_a_verified_execution_target_lease() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), false);
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Terminal);
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Failed);
    assert_eq!(driver.status().code, Some(GisMapInferencePortCodeV1::LeaseUnverified));
    assert!(driver.status().job_id.is_none());
    assert_eq!(driver.turn(1_000), GisMapInferenceTurnV1::Terminal, "a terminal port is hard");
}

/// 🔄️ One bounded action at a time: a submit occupies the driver until its exact answer lands,
/// the next turn only polls after the timer deadline, and a Cancel click never fabricates a
/// `cancelled` phase — only the server's own page may.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_runs_one_bounded_action_at_a_time_and_never_optimistically_cancels() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Idle, "no intent, no work");
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Submitting);
    assert!(matches!(driver.turn(0), GisMapInferenceTurnV1::WaitUntil(_)), "a second action never starts while one is in flight");
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Running);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Poll { job_id: INFERENCE_TEST_JOB.into(), after: 0 });
    driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Running, GisMapInferenceProposalStateV1::None, None, false, false)));
    assert_eq!(driver.status().completed, 1);
    assert_eq!(driver.status().total, 4);
    assert!(matches!(driver.turn(0), GisMapInferenceTurnV1::WaitUntil(_)), "polling is timer-armed, never a busy loop");

    driver.intend(GisMapInferenceIntentV1::Cancel);
    assert!(driver.status().cancel_requested, "a Cancel click is recorded as requested");
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Running, "and never as an optimistic terminal");
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Cancel { job_id: INFERENCE_TEST_JOB.into() });
    driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Cancelled, GisMapInferenceProposalStateV1::Cancelled, None, true, false)));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Cancelled);
    assert_eq!(driver.turn(10_000), GisMapInferenceTurnV1::Terminal);
}

/// ✅️ Approval is reachable only from an offered proposal with the server's own hash, is asked
/// for exactly once, and `applied` requires a receipt that actually committed.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_approves_only_an_offered_proposal_and_is_terminal_once() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Idle, "approval without an offer asks for nothing");
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    let mut previewless = inference_test_page(GisMapInferenceJobStateV1::Succeeded, GisMapInferenceProposalStateV1::Offered, Some(INFERENCE_TEST_HASH), false, false);
    previewless.preview = None;
    driver.complete(&GisMapInferencePortEventV1::Page(previewless));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Offered);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(5_000), GisMapInferenceTurnV1::Poll { job_id: INFERENCE_TEST_JOB.into(), after: 1 }, "an offer with no matching preview is never approved");
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Offered);
    driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Succeeded, GisMapInferenceProposalStateV1::Offered, Some(INFERENCE_TEST_HASH), false, false)));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Offered);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(10_000), GisMapInferenceTurnV1::Approve { job_id: INFERENCE_TEST_JOB.into(), proposal_hash: INFERENCE_TEST_HASH.into() });
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Approving);
    driver.complete(&GisMapInferencePortEventV1::Approval(GisMapInferenceApprovalReceiptV1 {
        schema: "semio.hub.inference-approval-receipt/v1".into(),
        job_id: INFERENCE_TEST_JOB.into(),
        mutation_id: INFERENCE_TEST_HASH.into(),
        command_hash: INFERENCE_TEST_HASH.into(),
        proposal_hash: INFERENCE_TEST_HASH.into(),
        applied: true,
        undo: semio_framework_os_kernel::os_directory::GisMapApprovalUndoHandleV1 {
            target_id: "22".repeat(16),
            expected_current: semio_framework_os_kernel::os_directory::CheckpointPublicationFrontierV1 {
                document_id: "document-map".into(),
                head_edit_ordinal: 2,
                head_edit_id: "approval-edit".into(),
                last_commit_seq: 2,
                chain_sha256: INFERENCE_TEST_HASH.into(),
            },
        },
    }));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Applied);
    assert_eq!(driver.turn(20_000), GisMapInferenceTurnV1::Terminal);
    driver.intend(GisMapInferenceIntentV1::Approve);
    assert_eq!(driver.turn(30_000), GisMapInferenceTurnV1::Terminal, "a committed port never re-approves");
}

/// ⏳️ The poll budget is finite: an unanswered job retires as indeterminate instead of spinning.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_retires_after_its_bounded_poll_budget() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    let mut now_ms = 0u64;
    for _ in 0..GisMapInferenceDriverV1::MAX_POLL_TURNS {
        assert!(matches!(driver.turn(now_ms), GisMapInferenceTurnV1::Poll { .. }));
        driver.complete(&GisMapInferencePortEventV1::Page(inference_test_page(GisMapInferenceJobStateV1::Running, GisMapInferenceProposalStateV1::None, None, false, false)));
        now_ms = now_ms.saturating_add(GisMapInferenceDriverV1::POLL_INTERVAL_MS);
    }
    assert_eq!(driver.turn(now_ms), GisMapInferenceTurnV1::Terminal);
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Failed);
    assert_eq!(driver.status().code, Some(GisMapInferencePortCodeV1::Transport));
}

/// 🧊️ A page for a different job id can never move this port.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn inference_driver_ignores_an_answer_for_another_job() {
    let mut driver = GisMapInferenceDriverV1::new(inference_test_scope(), true);
    driver.intend(GisMapInferenceIntentV1::Propose);
    assert_eq!(driver.turn(0), GisMapInferenceTurnV1::Submit);
    driver.complete(&GisMapInferencePortEventV1::Receipt(inference_test_receipt()));
    let mut foreign = inference_test_page(GisMapInferenceJobStateV1::Succeeded, GisMapInferenceProposalStateV1::Offered, Some(INFERENCE_TEST_HASH), false, false);
    foreign.job_id = "22222222222222222222222222222222".into();
    driver.complete(&GisMapInferencePortEventV1::Page(foreign));
    assert_eq!(driver.status().phase, GisMapInferencePortPhaseV1::Running);
    assert!(driver.status().proposal_hash.is_none());
}
//#endregion 💡️InferencePortLaws

#[cfg(not(target_arch = "wasm32"))]
fn directory_test_context() -> OperationContext {
    OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root_now(), capability: None }
}

#[cfg(not(target_arch = "wasm32"))]
fn directory_test_runner(since: u64) -> std::sync::Arc<ShellDirectoryRunner> {
    let pool = crate::renderer_worker_pool();
    let runtime = std::sync::Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("directory-bootstrap-law"), None);
    let compute = std::sync::Arc::new(ComputePool::with_pool(1, pool.clone()));
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime, scope, compute, 1_000_000, 1, DirectoryPackageId("directory-bootstrap-law".into()), DirectoryActorId(0));
    let stream = std::sync::Arc::new(DirectoryClient::new(transport, "http://hub.test")).stream_acknowledged(since).expect("acknowledged native stream");
    std::sync::Arc::new(ShellDirectoryRunner {
        pool,
        stream: std::sync::Mutex::new(stream),
        context: directory_test_context(),
        events: std::sync::Mutex::new(std::collections::VecDeque::new()),
        scheduled: std::sync::atomic::AtomicBool::new(false),
        notified: std::sync::atomic::AtomicBool::new(false),
        cancelled: std::sync::atomic::AtomicBool::new(false),
        terminal: std::sync::atomic::AtomicBool::new(false),
        timer_deadline_ms: std::sync::atomic::AtomicU64::new(0),
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn directory_terminal_result(acknowledgement: &DirectoryEventPageAckV1, extra_field: bool) -> semio_framework::kernel::InvocationResult {
    let mut fields = vec![
        ("schema".into(), DslValue::String("semio.space.home.directory-projection-receipt.v1".into())),
        ("sessionBindingSha256".into(), DslValue::String(acknowledgement.session_binding_sha256.clone())),
        ("authorizationGeneration".into(), DslValue::uint(acknowledgement.authorization_generation)),
        ("throughSeqInclusive".into(), DslValue::uint(acknowledgement.through_seq_inclusive)),
        ("receiptSha256".into(), DslValue::String(acknowledgement.receipt_sha256.clone())),
    ];
    if extra_field {
        fields.push(("authority".into(), DslValue::String("client".into())));
    }
    semio_framework::kernel::InvocationResult {
        output: DslValue::Null,
        mutations: Vec::new(),
        inverse_group: semio_framework::kernel::UndoGroup { invocation_id: semio_framework::kernel::InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
        diagnostics: Vec::new(),
        requested_effects: vec![semio_framework::kernel::Effect::PublishEvent { topic: "semio.space.home.directory-projection-receipt.v1".into(), payload: store::pack_rt::encode_wire_value(&DslValue::Object(fields)) }],
        events: Vec::new(),
        ui_scope: semio_framework::kernel::UiDirtyScope::default(),
        history_patch: None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct DirectoryBootstrapFakeHome {
    retained_instance_id: u32,
    visible_instance_id: u32,
    published_pages: Vec<String>,
    published_receipts: Vec<DirectoryEventPageAckV1>,
}

#[cfg(not(target_arch = "wasm32"))]
impl DirectoryBootstrapFakeHome {
    fn publish(&mut self, target_instance_id: u32, page: &CanonicalDirectoryEventPageV1, bootstrap_epoch: u64) -> semio_framework::kernel::InvocationResult {
        assert_eq!(target_instance_id, self.retained_instance_id);
        assert_ne!(target_instance_id, self.visible_instance_id);
        let acknowledgement = page.acknowledgement(bootstrap_epoch);
        self.published_pages.push(page.canonical_json().to_string());
        self.published_receipts.push(acknowledgement.clone());
        directory_terminal_result(&acknowledgement, false)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn space_administration_test_page(members: &[(&str, DirectorySpaceRole, bool)], invites: &[(&str, i64, bool)]) -> String {
    let mut page = DirectorySpaceAdministrationPageV1::Author {
        schema: semio_framework_os_kernel::os_directory::DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
        session_binding_sha256: "a".repeat(64),
        authorization_generation: 5,
        space_id: "space-admin-01".into(),
        space: semio_framework_os_kernel::os_directory::MemberSpaceViewV1 {
            id: "space-admin-01".into(),
            name: "Administered".into(),
            kind: DirectorySpaceKind::Studio,
            visibility: DirectorySpaceVisibility::Private,
            owner_user_id: "user-a".into(),
            role: DirectorySpaceRole::Author,
            member_count: members.len() as u32,
            document_count: 0,
            active_connections: 0,
            created_at_ms: 1,
            updated_at_ms: 2,
        },
        members: semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationMemberWindowV1 {
            rows: members.iter().map(|(user_id, role, owner)| DirectorySpaceAdministrationMemberRowV1 { user_id: (*user_id).into(), email: format!("{user_id}@example.invalid"), display_name: (*user_id).into(), role: *role, owner: *owner }).collect(),
            next_cursor: None,
        },
        documents: semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationDocumentWindowV1 { rows: Vec::new(), next_cursor: None },
        invites: semio_framework_os_kernel::os_directory::DirectorySpaceAdministrationInviteWindowV1 {
            rows: invites
                .iter()
                .map(|(invite_id, created_at_ms, revoked)| DirectorySpaceAdministrationInviteRowV1 {
                    invite_id: (*invite_id).into(),
                    role: DirectorySpaceRole::Spectator,
                    created_at_ms: *created_at_ms,
                    expires_at_ms: 900_000,
                    revoked: *revoked,
                    accepted: false,
                })
                .collect(),
            next_cursor: None,
        },
        capabilities: DirectorySpaceAdministrationCapabilitiesV1 { rename_space: true, set_visibility: true, delete_space: true, upsert_member: true, remove_member: true, create_invite: true, revoke_invite: true },
        receipt_sha256: String::new(),
    };
    let receipt = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
    if let DirectorySpaceAdministrationPageV1::Author { receipt_sha256, .. } = &mut page {
        *receipt_sha256 = receipt;
    }
    semio_framework_os_kernel::os_pack::json::to_json_string(&page)
}

/// 🏛️ The finite administration operation drives a REAL `DirectoryClient` through
/// `Loading → Ready → Submitting → Receipt → Refreshing → Ready`, never advances on anything but
/// an exact server receipt, and derives every control from the server's own capability flags.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn space_administration_operation_drives_a_real_directory_client_through_its_finite_turns() {
    semio_framework_async::block_on(async {
        let canonical = space_administration_test_page(&[("user-a", DirectorySpaceRole::Author, true), ("user-b", DirectorySpaceRole::Spectator, false)], &[("invite-live", 400, false), ("invite-dead", 200, true)]);
        let transport = DirectoryBootstrapFakeTransport::default();
        transport.responses.lock().expect("fake responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: canonical.clone().into_bytes() });
        let client = DirectoryClient::new(transport.clone(), "http://hub.test");
        let mut operation = ShellSpaceAdministrationOperationV1::open(1, "space-admin-01");
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Loading);

        let ShellSpaceAdministrationTurnV1::FetchPage { cursor } = operation.turn() else { panic!("first turn fetches the page") };
        assert_eq!(cursor, None);
        let fetched = client.space_administration_page(&directory_test_context(), "space-admin-01", None).await.expect("canonical administration page");
        assert_eq!(fetched.canonical_json(), canonical);
        assert!(operation.apply_page(fetched.canonical_json().to_string(), fetched.page().clone()));
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Ready);
        assert_eq!(transport.urls.lock().expect("fake urls").as_slice(), ["http://hub.test/directory/spaces/space-admin-01"]);

        let capabilities = operation.capabilities().expect("author capabilities");
        let DirectorySpaceAdministrationPageV1::Author { members, invites, .. } = operation.page().expect("ready page") else { panic!("author page") };
        assert!(!shell_space_administration_member_removable(&members.rows[0], Some(capabilities)), "the owner row can never be removed");
        assert!(shell_space_administration_member_removable(&members.rows[1], Some(capabilities)));
        assert!(shell_space_administration_invite_revocable(&invites.rows[0], Some(capabilities)));
        assert!(!shell_space_administration_invite_revocable(&invites.rows[1], Some(capabilities)), "a revoked invitation is terminal");
        assert!(shell_space_administration_member_removable(&members.rows[1], None).eq(&false), "a page without capabilities authorizes nothing");

        let english = shell_space_administration_controls(&operation, false);
        let german = shell_space_administration_controls(&operation, true);
        assert_eq!(english.len(), german.len());
        assert!(english.iter().zip(german.iter()).all(|(left, right)| left.control_id == right.control_id && left.enabled == right.enabled && left.label != right.label), "every control has an explicit EN and DE label");
        assert!(english.iter().any(|control| control.control_id == "os.space-administration.remove.user-b" && control.enabled));
        assert!(english.iter().any(|control| control.control_id == "os.space-administration.remove.user-a" && !control.enabled));
        assert!(english.iter().all(|control| control.control_id != "os.space-administration.invite.copy"), "no capability is offered before a receipt");

        let request = DirectoryCommandRequestV1::new(mint_directory_command_request_id(), DirectoryCommand::CreateInvite { space_id: "space-admin-01".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 });
        assert!(operation.request_command(request.clone()));
        let ShellSpaceAdministrationTurnV1::Submit { request: submitted } = operation.turn() else { panic!("second turn submits the command") };
        assert_eq!(submitted.request_id, request.request_id);
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Submitting);
        assert!(operation.receipt_sha256().is_none(), "no receipt exists before the server answers");
        assert!(!operation.phase().is_dispatchable(), "a second mutation cannot be dispatched while one is in flight");

        let mut receipt = DirectoryCommandReceiptV1 {
            schema: "semio.directory.command-receipt.v1".into(),
            request_id: request.request_id.clone(),
            command_sha256: directory_command_sha256(&request.command),
            outcome: DirectoryCommandOutcomeV1::Accepted,
            events: Vec::new(),
            result: DirectoryCommandResultV1::Invite { invite_token: "invite.v1.secret".into() },
            receipt_sha256: String::new(),
        };
        receipt.receipt_sha256 = semio_framework_hash::sha256_hex(receipt.canonical_unsigned_json().as_bytes());
        assert!(operation.apply_receipt(&receipt));
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Receipt);
        assert!(operation.invite_capability_pending());
        assert!(shell_space_administration_controls(&operation, false).iter().any(|control| control.control_id == "os.space-administration.invite.copy"));

        let ShellSpaceAdministrationTurnV1::FetchPage { cursor } = operation.turn() else { panic!("a receipt is always followed by a mandatory refresh") };
        assert_eq!(cursor, None);
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Refreshing);
        transport.responses.lock().expect("fake responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: canonical.clone().into_bytes() });
        let refreshed = client.space_administration_page(&directory_test_context(), "space-admin-01", None).await.expect("refreshed page");
        assert!(operation.apply_page(refreshed.canonical_json().to_string(), refreshed.page().clone()));
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Ready);
        assert_eq!(operation.acknowledge_capability().as_deref(), Some("invite.v1.secret"));
        assert_eq!(operation.acknowledge_capability(), None, "the one-shot capability is erased by its single handover");
        assert!(matches!(operation.turn(), ShellSpaceAdministrationTurnV1::Idle));
    });
}

/// 🧯️ Every denial class is terminal and erases the page, the receipt, and the capability, and a
/// mutation is never retried: an indeterminate transport is `Failed`, not a silent reissue.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn space_administration_operation_terminates_on_denial_stale_and_indeterminate_transport() {
    semio_framework_async::block_on(async {
        let canonical = space_administration_test_page(&[("user-a", DirectorySpaceRole::Author, true)], &[]);
        for (status, expected) in [(401u16, ShellSpaceAdministrationPhaseV1::Denied), (404, ShellSpaceAdministrationPhaseV1::Denied), (409, ShellSpaceAdministrationPhaseV1::Stale), (503, ShellSpaceAdministrationPhaseV1::Failed)] {
            let transport = DirectoryBootstrapFakeTransport::default();
            transport.responses.lock().expect("fake responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status, body: Vec::new() });
            let client = DirectoryClient::new(transport, "http://hub.test");
            let mut operation = ShellSpaceAdministrationOperationV1::open(2, "space-admin-01");
            let _ = operation.turn();
            let error = client.space_administration_page(&directory_test_context(), "space-admin-01", None).await.expect_err("denied page");
            operation.fail_page(&error);
            assert_eq!(operation.phase(), expected, "status {status} settles the wrong terminal phase");
            assert!(operation.page().is_none() && operation.canonical_json().is_none() && operation.receipt_sha256().is_none() && !operation.invite_capability_pending());
            assert!(matches!(operation.turn(), ShellSpaceAdministrationTurnV1::Closed));
            assert!(!operation.request_page(None), "a terminal operation never fetches again");
            assert!(shell_space_administration_controls(&operation, false).len() == 1, "a terminal pane renders only its status line");
        }

        let transport = DirectoryBootstrapFakeTransport::default();
        transport.responses.lock().expect("fake responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: canonical.into_bytes() });
        let client = DirectoryClient::new(transport, "http://hub.test");
        let mut operation = ShellSpaceAdministrationOperationV1::open(3, "space-admin-01");
        let _ = operation.turn();
        let fetched = client.space_administration_page(&directory_test_context(), "space-admin-01", None).await.expect("page");
        assert!(operation.apply_page(fetched.canonical_json().to_string(), fetched.page().clone()));
        let request = DirectoryCommandRequestV1::new(mint_directory_command_request_id(), DirectoryCommand::RemoveMember { space_id: "space-admin-01".into(), user_id: "user-b".into() });
        assert!(operation.request_command(request));
        let _ = operation.turn();
        operation.fail_command(DirectoryCommandErrorCodeV1::Transport);
        assert_eq!(operation.phase(), ShellSpaceAdministrationPhaseV1::Failed, "an indeterminate transport is unknown-outcome, never an auto-retry");
        assert_eq!(operation.code(), Some(DirectoryCommandErrorCodeV1::Transport));
        assert!(operation.page().is_none());
        assert_eq!(shell_space_administration_status(operation.phase(), false), "Unknown outcome — refresh required before retrying.");
        assert_eq!(shell_space_administration_status(operation.phase(), true), "Unbekanntes Ergebnis — vor einem erneuten Versuch aktualisieren.");
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn directory_home_bootstrap_waits_for_terminal_config_ack_and_retains_home_across_visibility() {
    semio_framework_async::block_on(async {
        let transport = DirectoryBootstrapFakeTransport::default();
        for body in [directory_page_json(3, 5, true, 'b'), directory_page_json(5, 8, false, 'c')] {
            transport.responses.lock().expect("fake responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: body.into_bytes() });
        }
        let client = DirectoryClient::new(transport.clone(), "http://hub.test");
        let mut home = DirectoryHomeProjection::new("s".into(), 41, test_app(Vec::new(), Vec::new()), ViewModel::default()).expect("retained Home");
        home.bootstrap = DirectoryEventPageBootstrapV1::new(7, 3).expect("fixture epoch");
        let first = client.event_page(&directory_test_context(), 3).await.expect("first canonical page");
        let first_for_home = first.clone();
        let expected = home.bootstrap.present(first).expect("first page admission");
        assert_eq!(home.bootstrap.after(), 3, "fetching and Home publication cannot advance the durable cursor");
        assert_eq!(transport.urls.lock().expect("fake urls").as_slice(), ["http://hub.test/directory/event-page/v1?after=3"], "page two cannot be requested before terminal Home publication");
        for field in ["bootstrapEpoch", "sessionBindingSha256", "authorizationGeneration", "throughSeqInclusive", "receiptSha256"] {
            let mut forged = expected.clone();
            match field {
                "bootstrapEpoch" => forged.bootstrap_epoch += 1,
                "sessionBindingSha256" => forged.session_binding_sha256 = "d".repeat(64),
                "authorizationGeneration" => forged.authorization_generation += 1,
                "throughSeqInclusive" => forged.through_seq_inclusive += 1,
                "receiptSha256" => forged.receipt_sha256 = "e".repeat(64),
                _ => unreachable!(),
            }
            assert!(home.bootstrap.acknowledge(&forged).is_err(), "{field} substitution must retain the committed cursor");
            assert_eq!(home.bootstrap.after(), 3);
        }
        let mut bridge = DirectoryBootstrapFakeHome { retained_instance_id: home.instance_id, visible_instance_id: 73, published_pages: Vec::new(), published_receipts: Vec::new() };
        let terminal = terminal_directory_home_ack(&bridge.publish(home.instance_id, &first_for_home, 7), 7).expect("terminal Config receipt");
        assert_eq!(terminal, expected);
        assert!(matches!(home.bootstrap.acknowledge(&terminal), Ok(DirectoryBootstrapTransition::Fetch { after: 5 })));
        let second = client.event_page(&directory_test_context(), 5).await.expect("second canonical page after ACK");
        assert_eq!(transport.urls.lock().expect("fake urls").as_slice(), ["http://hub.test/directory/event-page/v1?after=3", "http://hub.test/directory/event-page/v1?after=5"]);
        let second_for_home = second.clone();
        let expected_second = home.bootstrap.present(second).expect("second page waits behind first terminal receipt");
        let terminal_second = terminal_directory_home_ack(&bridge.publish(home.instance_id, &second_for_home, 7), 7).expect("second terminal Config receipt");
        assert_eq!(bridge.published_pages, [first_for_home.canonical_json(), second_for_home.canonical_json()]);
        assert_eq!(bridge.published_receipts, [expected, expected_second.clone()]);
        assert!(matches!(home.bootstrap.acknowledge(&terminal_second), Ok(DirectoryBootstrapTransition::Live { since: 8 })));
        let visible_elsewhere = bridge.visible_instance_id;
        assert_ne!(visible_elsewhere, home.instance_id);
        assert_eq!(home.active_session().instance_id, 41, "publication address remains the retained Home instance while another app is visible");
        assert_eq!(home.take_destroy_authority(), Some(("s".into(), 41)));
        assert_eq!(home.take_destroy_authority(), None, "close and hot reload cannot destroy retained Home twice");
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss() {
    semio_framework_async::block_on(async {
        let transport = DirectoryBootstrapFakeTransport::default();
        transport.responses.lock().expect("fake responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: directory_page_json(3, 8, false, 'b').into_bytes() });
        let client = DirectoryClient::new(transport, "http://hub.test");
        let mut home = DirectoryHomeProjection::new("s".into(), 41, test_app(Vec::new(), Vec::new()), ViewModel::default()).expect("retained Home");
        home.bootstrap = DirectoryEventPageBootstrapV1::new(7, 3).expect("fixture epoch");
        let failed_transport = DirectoryBootstrapFakeTransport::default();
        failed_transport.responses.lock().expect("fake failure").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 503, body: Vec::new() });
        let failed_client = DirectoryClient::new(failed_transport.clone(), "http://hub.test");
        assert!(matches!(failed_client.event_page(&directory_test_context(), 3).await, Err(DirectoryClientError::Http { status: 503, .. })));
        assert_eq!(home.bootstrap.after(), 3, "transport failure cannot advance the committed cursor");
        assert_eq!(failed_transport.urls.lock().expect("failed urls").as_slice(), ["http://hub.test/directory/event-page/v1?after=3"]);
        let page = client.event_page(&directory_test_context(), 3).await.expect("canonical page");
        let expected = home.bootstrap.present(page).expect("page admission");
        assert!(matches!(home.retry(1_000, &expected.receipt_sha256), Ok(3)), "matching Home rejection retries the exact committed cursor");
        assert_eq!(home.retry_at_ms, 1_050);
        let page = DirectoryClient::new(
            DirectoryBootstrapFakeTransport {
                responses: std::sync::Arc::new(std::sync::Mutex::new(std::collections::VecDeque::from([semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: directory_page_json(3, 8, false, 'c').into_bytes() }]))),
                urls: Default::default(),
            },
            "http://hub.test",
        )
        .event_page(&directory_test_context(), 3)
        .await
        .expect("retried page");
        let expected = home.bootstrap.present(page).expect("retried admission");
        assert!(matches!(home.bootstrap.acknowledge(&expected), Ok(DirectoryBootstrapTransition::Live { since: 8 })));
        let runner = directory_test_runner(8);
        assert_eq!(runner.stream.lock().expect("directory stream").since(), 8);
        home.stream = Some(runner.clone());
        assert!(matches!(home.wake(false), Ok(Some(8))), "live event wakes a refetch at the acknowledged cursor");
        assert!(runner.cancelled.load(std::sync::atomic::Ordering::Acquire), "dirty wake closes the acknowledged stream");
        assert!(matches!(home.wake(false), Ok(None)), "duplicate live wake coalesces while the page refetch is already pending");
        home.bootstrap = DirectoryEventPageBootstrapV1::new(7, 8).expect("restore live fixture");
        let live_page_transport = DirectoryBootstrapFakeTransport::default();
        live_page_transport.responses.lock().expect("responses").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: directory_page_json(8, 8, false, 'd').into_bytes() });
        let page = DirectoryClient::new(live_page_transport, "http://hub.test").event_page(&directory_test_context(), 8).await.expect("empty live page");
        let ack = home.bootstrap.present(page).expect("live page");
        assert!(matches!(home.bootstrap.acknowledge(&ack), Ok(DirectoryBootstrapTransition::Live { since: 8 })));
        assert!(matches!(home.wake(true), Ok(Some(0))), "rebootstrap resets the frontier");
        assert_eq!(home.bootstrap.bootstrap_epoch(), 8);
        assert!(home.bootstrap.acknowledge(&ack).is_err(), "a stale epoch receipt cannot revive the retired stream");
        let rebootstrap_transport = DirectoryBootstrapFakeTransport::default();
        rebootstrap_transport.responses.lock().expect("rebootstrap response").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: directory_page_json(0, 0, false, 'e').into_bytes() });
        let rebootstrap_page = DirectoryClient::new(rebootstrap_transport, "http://hub.test").event_page(&directory_test_context(), 0).await.expect("rebootstrap page");
        let rebootstrap_ack = home.bootstrap.present(rebootstrap_page).expect("rebootstrap page admission");
        assert!(matches!(home.bootstrap.acknowledge(&rebootstrap_ack), Ok(DirectoryBootstrapTransition::Live { since: 0 })));
        let terminal_client = std::sync::Arc::new(DirectoryClient::new(DirectoryBootstrapFakeTransport::default(), "http://hub.test"));
        let mut terminal_stream = terminal_client.stream_acknowledged(0).expect("terminal stream");
        let terminal_context = directory_test_context();
        assert!(matches!(terminal_stream.turn(&terminal_context, 0), DirectoryStreamTurn::Dial { since: 0, .. }));
        assert!(matches!(terminal_stream.complete_dial(0, Ok(DirectoryBootstrapFakeWs { close_code: Some(4401) })), DirectoryStreamTurn::Idle));
        assert!(matches!(terminal_stream.turn(&terminal_context, 1), DirectoryStreamTurn::Closed), "authenticated terminal close cannot reconnect");
        assert!(matches!(terminal_stream.turn(&terminal_context, u64::MAX), DirectoryStreamTurn::Closed), "terminal stream remains closed after every reconnect deadline");
        assert_eq!(home.begin_epoch(0).expect("terminal identity epoch"), 9);
        assert_eq!(home.bootstrap.after(), 0, "terminal identity close restarts from raw cursor zero");
        assert!(home.bootstrap.acknowledge(&rebootstrap_ack).is_err(), "pre-terminal epoch receipt cannot revive the retired stream");
        let pending_transport = DirectoryBootstrapFakeTransport::default();
        pending_transport.responses.lock().expect("pending response").push_back(semio_framework_os_kernel::os_directory::client::HttpResponse { status: 200, body: directory_page_json(0, 13, false, 'e').into_bytes() });
        let pending_page = DirectoryClient::new(pending_transport, "http://hub.test").event_page(&directory_test_context(), 0).await.expect("pending page");
        let pending_ack = home.bootstrap.present(pending_page).expect("pending page admission");
        let pending_epoch = home.bootstrap.bootstrap_epoch();
        let page_cancel = CancelToken::root_now();
        let (late_tx, late_rx) = std::sync::mpsc::channel::<ShellDirectoryPageResult>();
        let page_task = ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, std::future::pending::<()>());
        let (late_home_tx, late_home_rx) = std::sync::mpsc::channel::<ShellDirectoryHomePublicationResult>();
        let home_task = ShellPoolFuture::spawn(crate::renderer_worker_pool(), Lane::Io, std::future::pending::<()>());
        home.page_cancel = Some(page_cancel.clone());
        home.page_rx = Some(late_rx);
        home.page_task = Some(page_task.clone());
        home.home_rx = Some(late_home_rx);
        home.home_task = Some(home_task.clone());
        home.close();
        assert!(page_cancel.is_cancelled_now() && page_task.cancelled.load(std::sync::atomic::Ordering::Acquire) && home_task.cancelled.load(std::sync::atomic::Ordering::Acquire));
        assert!(home.closed && home.page_task.is_none() && home.page_rx.is_none() && home.home_task.is_none() && home.stream.is_none());
        assert!(home.bootstrap.acknowledge(&pending_ack).is_err(), "close clears pending canonical page authority");
        assert!(late_tx.send((pending_epoch, Err(DirectoryClientError::Cancelled))).is_err(), "late page result has no surviving receiver or ACK path");
        assert!(late_home_tx.send((pending_epoch, 41, pending_ack.clone(), ShellDirectoryHomePublicationOutcome::Published(pending_ack))).is_err(), "late Home publication has no surviving receiver or ACK path");
    });
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn directory_home_terminal_receipt_rejects_unknown_fields_and_nonreceipt_effects() {
    let expected = DirectoryEventPageAckV1 { bootstrap_epoch: 7, session_binding_sha256: "a".repeat(64), authorization_generation: 9, through_seq_inclusive: 8, receipt_sha256: "b".repeat(64) };
    assert!(terminal_directory_home_ack(&directory_terminal_result(&expected, true), 7).is_err());
    let mut missing = directory_terminal_result(&expected, false);
    missing.requested_effects.clear();
    assert!(terminal_directory_home_ack(&missing, 7).is_err());
    let mut duplicate = directory_terminal_result(&expected, false);
    let mut duplicate_source = directory_terminal_result(&expected, false);
    duplicate.requested_effects.push(duplicate_source.requested_effects.pop().expect("receipt effect"));
    assert!(terminal_directory_home_ack(&duplicate, 7).is_err());
    let mut nonreceipt = directory_terminal_result(&expected, false);
    nonreceipt.requested_effects.push(semio_framework::kernel::Effect::Navigate { uri: "/".into() });
    assert!(terminal_directory_home_ack(&nonreceipt, 7).is_err());
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn native_document_admission_is_bound_to_verified_package_app_window_and_renderer() {
    let mut app = test_app(Vec::new(), Vec::new());
    app.io.document_schema = "s.test.document".into();
    let manifest = PluginManifest {
        plugin_id: "s.test".into(),
        label: "Test".into(),
        version: "1.2.3".into(),
        apps: vec![app.clone()],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![],
        artifact_kinds: vec![],
        dependencies: vec![],
        contributions: vec![],
    };
    let authority = document_socket_surface_from_descriptor("s.test", Some("s.test.package"), &manifest, &app, "main").expect("verified descriptor selection");
    assert_eq!(authority, semio_framework::manifest::surface_app_id(&app.dialect, app.role));
    assert!(document_socket_surface_from_descriptor("s.test", None, &manifest, &app, "main").is_err());
    assert!(document_socket_surface_from_descriptor("hostile.plugin", Some("s.test.package"), &manifest, &app, "main").is_err());
    assert!(document_socket_surface_from_descriptor("s.test", Some("s.test.package"), &manifest, &app, "unrelated-window").is_err());
    let unrelated_app = test_app(Vec::new(), Vec::new());
    let empty_manifest = PluginManifest { apps: Vec::new(), ..manifest };
    assert!(document_socket_surface_from_descriptor("s.test", Some("s.test.package"), &empty_manifest, &unrelated_app, "main").is_err());
}

#[test]
fn build_os_commands_covers_every_wired_setting() {
    let shell = test_shell_state();
    let ids: Vec<String> = shell.build_os_commands().into_iter().map(|command| command.id).collect();
    assert_eq!(ids, vec!["os.toggleFullscreen", "os.setAppearance", "os.setDriver", "os.setLocale", "os.setTerminology", "os.setThemeId", "os.resetDock",]);
}

#[test]
fn fullscreen_command_requests_the_host_transition() {
    let mut shell = test_shell_state();
    semio_framework_async::block_on(shell.apply_os_command("os.toggleFullscreen", None)).expect("fullscreen command");
    assert!(shell.fullscreen_toggle_requested);
}

#[test]
fn build_os_commands_terminology_options_include_app_terminologies() {
    let mut shell = test_shell_state();
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: ViewModel::default() });
    let terminology_command = shell.build_os_commands().into_iter().find(|command| command.id == "os.setTerminology").expect("terminology command present");
    let ActionArgControl::Select { options } = &terminology_command.args[0].control() else {
        panic!("expected a select control");
    };
    let values: Vec<&str> = options.iter().map(|option| option.value.as_str()).collect();
    assert_eq!(values, vec!["native", "de"]);
}

#[test]
fn resolve_commands_tags_every_source() {
    let os_commands = vec![CommandDefinition::bounded_catalog("os.setLocale", LocalizedLabel::data("Set Locale"), "language", ActionKind::Shell)];
    let app_command = CommandDefinition::bounded_catalog("export", LocalizedLabel::data("Export"), "app", ActionKind::Mutation);
    let mode_command = CommandDefinition::bounded_catalog("focus", LocalizedLabel::data("Focus Mode"), "mode", ActionKind::View);
    let app = test_app(vec![app_command.clone()], vec![mode_command.clone()]);
    let plugin_manifest = PluginManifest {
        plugin_id: "plugin".into(),
        label: "Plugin".into(),
        version: "0.0.0".into(),
        apps: vec![],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![CommandDefinition::bounded_catalog("doThing", LocalizedLabel::data("Do Thing"), "plugin", ActionKind::Shell)],
        artifact_kinds: vec![],
        dependencies: vec![],
        contributions: vec![],
    };
    let resolved = resolve_commands(os_commands, Some(&plugin_manifest), "plugin", &app, "default");
    let sources: Vec<(&str, CommandOwnerAddress)> = resolved.iter().map(|entry| (entry.definition.id.as_str(), entry.address.owner.clone())).collect();
    assert_eq!(
        sources,
        vec![
            ("os.setLocale", CommandOwnerAddress::Os),
            ("doThing", CommandOwnerAddress::Plugin { plugin_id: "plugin".into() }),
            ("export", CommandOwnerAddress::App { plugin_id: "plugin".into(), app_id: "test-app".into() }),
            ("focus", CommandOwnerAddress::Mode { plugin_id: "plugin".into(), app_id: "test-app".into(), mode_id: "default".into() }),
        ]
    );
}

#[test]
fn identical_local_command_ids_have_collision_free_owner_keys() {
    let local_id = "refresh";
    let app =
        test_app(vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh App"), "app", ActionKind::View)], vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh Mode"), "mode", ActionKind::View)]);
    let plugin_manifest = PluginManifest {
        plugin_id: "plugin".into(),
        label: "Plugin".into(),
        version: "0.0.0".into(),
        apps: vec![],
        examples: vec![],
        capabilities: vec![],
        topic_contributions: vec![],
        commands: vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh Plugin"), "plugin", ActionKind::View)],
        artifact_kinds: vec![],
        dependencies: vec![],
        contributions: vec![],
    };
    let resolved = resolve_commands(vec![CommandDefinition::bounded_catalog(local_id, LocalizedLabel::data("Refresh Shell"), "general", ActionKind::Shell)], Some(&plugin_manifest), "plugin", &app, "default");
    let keys: Vec<String> = resolved.iter().map(|entry| command_address_stable_key(&entry.address)).collect();
    assert_eq!(keys, vec!["os:refresh", "plugin:plugin:refresh", "app:plugin:test-app:refresh", "mode:plugin:test-app:default:refresh"]);
}

#[test]
fn command_categories_orders_by_first_appearance_and_dedupes() {
    let resolved = vec![
        ResolvedCommand::new(CommandDefinition::bounded_catalog("a", LocalizedLabel::data("A"), "appearance", ActionKind::Shell), CommandOwnerAddress::Os),
        ResolvedCommand::new(CommandDefinition::bounded_catalog("b", LocalizedLabel::data("B"), "layout", ActionKind::Shell), CommandOwnerAddress::Os),
        ResolvedCommand::new(CommandDefinition::bounded_catalog("c", LocalizedLabel::data("C"), "appearance", ActionKind::Shell), CommandOwnerAddress::Os),
    ];
    assert_eq!(command_categories(&resolved), vec![("appearance".to_string(), "Appearance".to_string()), ("layout".to_string(), "Layout".to_string())]);
}

#[test]
fn command_category_label_titleizes_hyphenated_ids() {
    assert_eq!(command_category_label("named-layout"), "Named Layout");
    assert_eq!(command_category_label("general"), "General");
}

#[test]
fn command_search_items_expands_select_options_and_tags_os_category() {
    let mut shell = test_shell_state();
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: ViewModel::default() });
    let items = shell.command_search_items();
    let appearance_dark = items.iter().find(|item| item.id == "command.os:os.setAppearance.dark").expect("expanded dark option present");
    assert_eq!(appearance_dark.label, "Set Appearance: Dark");
    assert_eq!(appearance_dark.action.as_deref(), Some("os-command:os.setAppearance:dark"));
    assert_eq!(appearance_dark.category, Some(CommandOwnerAddress::Os));
    let reset_dock = items.iter().find(|item| item.id == "command.os:os.resetDock").expect("zero-arg reset dock present");
    assert_eq!(reset_dock.action.as_deref(), Some("os-command:os.resetDock"));
    assert!(reset_dock.dispatch_action.is_none());
}

// 🔌️ Plain `#[test]` plus the sanctioned test-entrypoint driver keeps these tests
// deterministic without introducing a runtime.
#[test]
fn apply_os_command_reset_dock_clears_layout_override_locally() {
    let mut shell = test_shell_state();
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: ViewModel::default() });
    shell.layout_override = Some(shell.dock.to_window_layout());
    semio_framework_async::block_on(shell.apply_os_command("os.resetDock", None)).expect("reset dock never errors");
    assert!(shell.layout_override.is_none());
}

#[test]
fn apply_os_command_set_locale_dispatches_through_framework_controller() {
    let mut shell = test_shell_state();
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: ViewModel::default() });
    semio_framework_async::block_on(shell.apply_os_command("os.setLocale", Some("de"))).expect("set locale never errors");
    assert_eq!(shell.locale_id, "de");
}

#[test]
fn apply_os_command_set_driver_updates_driver_id() {
    let mut shell = test_shell_state();
    assert_eq!(shell.driver_id, "default");
    semio_framework_async::block_on(shell.apply_os_command("os.setDriver", Some("compact"))).expect("set driver never errors");
    assert_eq!(shell.driver_id, "compact");
    semio_framework_async::block_on(shell.apply_os_command("os.setDriver", Some("default"))).expect("set driver never errors");
    assert_eq!(shell.driver_id, "default");
}

/// 🎨️ `os.setThemeId` (added alongside `build_settings_theme_ui`'s reachable Theme tab — see that
/// function's doc comment) round-trips through the same `"framework"` `dispatch_action` arm as every
/// other os select-command, landing in `active_theme_id()` (the `CHROME_PREFS` thread-local
/// `frame()`'s `resolve_theme_for_ids` call already reads every frame), not a `ShellState` field.
#[test]
fn apply_os_command_set_theme_id_updates_active_theme() {
    let mut shell = test_shell_state();
    semio_framework_async::block_on(shell.apply_os_command("os.setThemeId", Some("mono"))).expect("set theme never errors");
    assert_eq!(active_theme_id(), "mono");
    assert_eq!(shell.chrome_build.preferences.theme_id, "mono");
    semio_framework_async::block_on(shell.apply_os_command("os.setThemeId", Some("semio"))).expect("set theme never errors");
    assert_eq!(active_theme_id(), "semio");
    assert_eq!(shell.chrome_build.preferences.theme_id, "semio");
}

#[test]
fn build_command_panel_ui_groups_rows_under_category_headers() {
    let mut shell = test_shell_state();
    shell.session = Some(ActiveSession { plugin_id: "test".into(), instance_id: 0, app: test_app(vec![], vec![]), view_state: ViewModel::default() });
    let UiNode::Stack(panel) = shell.build_command_panel_ui() else {
        panic!("expected a stack root");
    };
    // 🗂️ One section per distinct `CommandDefinition.category`: fullscreen is in window,
    // appearance contains setAppearance/setThemeId, layout contains setDriver/resetDock, and
    // language contains setLocale/setTerminology.
    assert_eq!(panel.children.len(), 4);
}

#[test]
fn fuzzy_match_score_finds_scattered_subsequence_and_rejects_non_matches() {
    assert!(fuzzy_match_score("stlc", "Set Locale").is_some());
    assert!(fuzzy_match_score("xyz", "Set Locale").is_none());
    assert!(fuzzy_match_score("", "Set Locale").is_some());
}

#[test]
fn fuzzy_match_score_ranks_contiguous_prefix_above_scattered_match() {
    let contiguous = fuzzy_match_score("set", "Set Locale").expect("contiguous match");
    let scattered = fuzzy_match_score("sca", "Set Locale").expect("scattered match");
    assert!(contiguous > scattered, "contiguous {contiguous} should outrank scattered {scattered}");
}

#[test]
fn fuzzy_match_score_is_case_insensitive() {
    assert_eq!(fuzzy_match_score("SET", "set locale"), fuzzy_match_score("set", "SET LOCALE"));
}

//#region ShellCommandHistoryTests
#[test]
fn note_shell_command_action_carries_command_id_label_and_detail() {
    let action = ShellState::note_shell_command_action("controller-1", "os.setLocale", "Set Locale", Some(serde_json::json!({ "value": "de" })));
    assert_eq!(action.controller_id, "controller-1");
    assert_eq!(action.action, "noteShellCommand");
    let args = action.args.expect("noteShellCommand always carries args");
    assert_eq!(args.get("commandId").and_then(DslValue::as_str), Some("os.setLocale"));
    assert_eq!(args.get("label").and_then(DslValue::as_str), Some("Set Locale"));
    assert_eq!(args.get("detail").and_then(|value| value.get("value")).and_then(DslValue::as_str), Some("de"));
}

#[test]
fn note_shell_command_action_omits_detail_entirely_when_none() {
    let action = ShellState::note_shell_command_action("controller-1", "shell.windowResize", "Resize Window", None);
    let args = action.args.expect("args always present");
    assert!(args.get("detail").is_none(), "no detail key at all, not a serialized null");
}

#[test]
fn shell_command_label_for_setting_matches_build_os_commands_and_theme_chrome_labels() {
    let shell = test_shell_state();
    assert_eq!(shell.shell_command_label_for_setting("os.setAppearance"), "Set Appearance");
    assert_eq!(shell.shell_command_label_for_setting("os.setDriver"), "Set Driver");
    assert_eq!(shell.shell_command_label_for_setting("os.setLocale"), "Set Locale");
    assert_eq!(shell.shell_command_label_for_setting("os.setTerminology"), "Set Terminology");
    assert_eq!(shell.shell_command_label_for_setting("os.setThemeId"), "Set Theme");
    assert_eq!(shell.shell_command_label_for_setting("os.resetThemeId"), shell_chrome_string("settings.theme.reset", false));
    assert_eq!(shell.shell_command_label_for_setting("os.deleteThemeId"), shell_chrome_string("settings.theme.delete", false));
}

/// 🕒️ The `handle_shell_hit` control-id-to-shell-command mapping table: every discrete window/panel
/// chrome command this ticket wires, plus a representative non-matching id proving the mapping
/// doesn't fire on everything.
#[test]
fn shell_command_for_control_maps_dock_and_panel_control_ids() {
    assert_eq!(ShellState::shell_command_for_control("dock.tab.0.a.close", false), Some(("shell.windowClose", "Close".to_string())));
    assert_eq!(ShellState::shell_command_for_control("dock.tab.0.a.focus", false), Some(("shell.windowMaximize", "Focus".to_string())));
    assert_eq!(ShellState::shell_command_for_control("shell.layout.compact", false), Some(("shell.applyNamedLayout", "Apply Layout".to_string())));
    assert_eq!(ShellState::shell_command_for_control("ui.panelToggle.details", false), Some(("shell.panelToggle", shell_chrome_string("panelToggle.details", false).to_string())));
    assert_eq!(ShellState::shell_command_for_control("ui.panelToggle.settings", true), Some(("shell.panelToggle", shell_chrome_string("panelToggle.settings", true).to_string())));
    assert_eq!(ShellState::shell_command_for_control("ui.panelToggle.display", false), None, "left-panel toggles are out of scope");
    assert_eq!(ShellState::shell_command_for_control("ui.nav.back", false), None);
}
//#endregion ShellCommandHistoryTests
