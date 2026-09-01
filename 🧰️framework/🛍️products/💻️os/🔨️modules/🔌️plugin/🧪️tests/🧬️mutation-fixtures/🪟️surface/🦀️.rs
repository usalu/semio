#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetSurfaceCount, SurfaceMutation};

// 🧪️ Proves the viewer helpers against a minimal editor/viewer pair sharing one dialect.
use crate::app::{built_text_to_component_tree, ArtifactEditor, ArtifactViewer, ArtifactView, ConfigView, DraftView, EditorApp, Emit, Media, MediaClass, MediaForm, MediaPayload, MediaType, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, PluginApp, REVERT_TO_COMMAND_ACTION_ID, UiAssemblyResult, ViewEmit, ViewerApp};
use crate::app::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates, meta, new_app, new_viewer};
use protocol::{Mutation, MutationDiff};
use semio_framework::{Dialect, Fault, FaultOrigin, StandardId, SubsetId};
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};
use store::EngineHandles;

const SURFACE_TESTKIT_DIALECT: Dialect = Dialect { artifact_kind: "testkit.surface", standard: StandardId("1"), subset: SubsetId::ANY };

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[dsl(extension = "testkit-surface")]
pub(crate) struct SurfaceSnapshot {
    count: i32,
}

impl store::ArtifactDsl for SurfaceSnapshot {
    const EXTENSION: &'static str = "testkit-surface";
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

impl store::ArtifactPack for SurfaceSnapshot {
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
pub(crate) struct SurfaceDiff {
    count: Option<i32>,
}

impl MutationDiff<SurfaceSnapshot> for SurfaceDiff {
    fn apply(&self, snapshot: &SurfaceSnapshot) -> protocol::MutationApplyResult<SurfaceSnapshot> {
        Ok(SurfaceSnapshot { count: self.count.unwrap_or(snapshot.count) })
    }
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
enum SurfaceEditorCommand {
    #[dsl(key = "increment")]
    Increment,
}

impl ::protocol::OpText for SurfaceEditorCommand {
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

impl ::protocol::OpBinary for SurfaceEditorCommand {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum SurfaceViewerCommand {
    #[default]
    Noop,
}

impl ::protocol::OpBinary for SurfaceViewerCommand {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Ok(SurfaceViewerCommand::Noop)
    }
}

#[derive(Default)]
struct SurfaceEditorFixture;

impl ArtifactEditor for SurfaceEditorFixture {
    const DIALECT: Dialect = SURFACE_TESTKIT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-surface/v1";
    type Snapshot = SurfaceSnapshot;
    type Mutation = SurfaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app::NoTransient;
    type TransientMutation = crate::app::NoTransientMutation;
    type Command = SurfaceEditorCommand;

    fn initial_snapshot() -> SurfaceSnapshot {
        SurfaceSnapshot::default()
    }

    fn handle(
        command: &SurfaceEditorCommand,
        doc: &ArtifactView<'_, SurfaceSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app::InteractionView<'_>,
        _draft: &DraftView<'_, NoDraft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<SurfaceMutation>, Fault> {
        match command {
            SurfaceEditorCommand::Increment => Ok(Emit { artifact_mutations: vec![SetSurfaceCount { value: doc.snapshot.count + 1 }.into()], description: Some("increment".into()), ..Default::default() }),
        }
    }

    fn render(_body_key: &str, doc: &ArtifactView<'_, SurfaceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

#[derive(Default)]
struct SurfaceViewerFixture;

impl ArtifactViewer for SurfaceViewerFixture {
    const DIALECT: Dialect = SURFACE_TESTKIT_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-surface/v1";
    type Snapshot = SurfaceSnapshot;
    type Mutation = SurfaceMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Presence = NoPresence;
    type PresenceMutation = NoPresenceMutation;
    type Transient = crate::app::NoTransient;
    type TransientMutation = crate::app::NoTransientMutation;
    type Command = SurfaceViewerCommand;

    fn initial_snapshot() -> SurfaceSnapshot {
        SurfaceSnapshot::default()
    }

    fn handle(
        _command: &SurfaceViewerCommand,
        _doc: &ArtifactView<'_, SurfaceSnapshot>,
        _cfg: &ConfigView<'_, NoConfig>,
        _interaction: &crate::app::InteractionView<'_>,
        _engines: &EngineHandles,
    ) -> Result<ViewEmit<NoConfigMutation>, Fault> {
        Ok(ViewEmit::default())
    }

    fn render(_body_key: &str, doc: &ArtifactView<'_, SurfaceSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)))
    }
}

#[semio_framework_async_macros::async_test]
async fn viewer_never_mutates_the_document_or_draft_store() {
    assert_viewer_never_mutates::<SurfaceViewerFixture>().await;
}

#[semio_framework_async_macros::async_test]
async fn editor_and_viewer_share_one_dialect() {
    assert_editor_and_viewer_share_dialect::<SurfaceEditorFixture, SurfaceViewerFixture>().await;
}

#[semio_framework_async_macros::async_test]
async fn new_viewer_constructs_a_registry_less_wrapper() {
    let app = new_viewer::<SurfaceViewerFixture>().await;
    assert_eq!(app.snapshot().unwrap().count, 0);
}

#[semio_framework_async_macros::async_test]
async fn editor_fixture_still_mutates_normally() {
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>().await;
    app.dispatch_typed(SurfaceEditorCommand::Increment, &meta("local")).await.expect("increment");
    assert_eq!(app.snapshot().unwrap().count, 1);
}

/// 🐛️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-G — WITH
/// TEETH: `ShellHost::encodeWindowActionInvocation` addresses every real click with the
/// surface's canonical id (`surface_app_id`, e.g. `s.space.home@1/*#editor`), never
/// `EditorApp::APP_ID`'s runtime-const placeholder (`"surface"`). Before the fix,
/// `handle_action_invocation`'s ownership check compared `address.app_id` against that
/// literal placeholder, so even the textbook-correct canonical id was rejected — every
/// button click across every `EditorApp`/`ViewerApp` surface in the product. This fixture's
/// registry is empty (`new_app`, contract-enforcement-less), so once ownership passes the
/// very next check (`registry.has_mode`) must fail instead — proving the rejection was
/// specifically the ownership line, not a coincidence of an otherwise-valid dispatch.
#[semio_framework_async_macros::async_test]
async fn handle_action_invocation_accepts_the_real_canonical_surface_app_id() {
    use semio_framework::manifest::{ActionAddress, ActionInvocation};
    let mut app = new_app::<EditorApp<SurfaceEditorFixture>>().await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor);
    let invocation = ActionInvocation {
        address: ActionAddress { plugin_id: "test".into(), app_id: real_id.clone(), mode_id: "edit".into(), window_kind_id: "main".into(), window_instance_id: "main-instance".into(), action_id: "increment".into() },
        arguments: Default::default(),
    };
    let error = app.handle_action_invocation(&invocation, Some("edit"), &meta("local")).await.expect_err("registry-less fixture declares no modes");
    assert!(!error.message.contains("does not match"), "the real canonical app id must satisfy the ownership check, got: {}", error.message);
    assert!(error.message.contains("unknown action mode owner"), "expected ownership to pass and the mode lookup to fail instead, got: {}", error.message);
    assert_eq!(app.app_id().await, real_id, "PluginApp::app_id must report the real canonical id, not the APP_ID placeholder");
}

/// 🪪️ Verifies `EditorApp` initializes its document, config, draft, and interaction
/// envelopes with the real canonical surface app id, not the `APP_ID` placeholder.
#[semio_framework_async_macros::async_test]
async fn editor_app_envelopes_carry_the_real_canonical_surface_app_id() {
    let app = new_app::<EditorApp<SurfaceEditorFixture>>().await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Editor);
    assert_eq!(app.store.envelope().id, real_id);
    assert_eq!(app.config_store.envelope().id, format!("{real_id}-config"));
    assert_eq!(app.draft_store.envelope().id, format!("{real_id}-draft"));
    assert_eq!(app.interaction_store.envelope().id, format!("{real_id}-interaction"));
}

/// 🪪️ Verifies `ViewerApp` initializes its document, config, draft, and interaction
/// envelopes with the real canonical surface app id, not the `APP_ID` placeholder.
#[semio_framework_async_macros::async_test]
async fn viewer_app_envelopes_carry_the_real_canonical_surface_app_id() {
    let app = new_viewer::<SurfaceViewerFixture>().await;
    let real_id = semio_framework::surface_app_id(&SURFACE_TESTKIT_DIALECT.into(), semio_framework::AppRole::Viewer);
    assert_eq!(app.store.envelope().id, real_id);
    assert_eq!(app.config_store.envelope().id, format!("{real_id}-config"));
    assert_eq!(app.draft_store.envelope().id, format!("{real_id}-draft"));
    assert_eq!(app.interaction_store.envelope().id, format!("{real_id}-interaction"));
}

/// 👁️🔒 Contract §2.3 clause 1/2 — WITH TEETH: dispatches the eight frozen mutating verbs
/// through the full `VcsArtifactApp<ViewerApp<V>>` runtime path (`handle_action` for the
/// seven string actions, `import_media` for the eighth) and asserts every one comes back
/// `Fault { origin: FaultOrigin::Framework, code: FaultCode::new("viewer.read-only"), .. }`.
#[semio_framework_async_macros::async_test]
async fn viewer_rejects_every_contract_mutating_verb() {
    let mut app = new_viewer::<SurfaceViewerFixture>().await;
    for verb in ["undo", "redo", "commitCheckpoint", "createAlternative", REVERT_TO_COMMAND_ACTION_ID, "cut", "paste"] {
        let error = app.handle_action(verb, None, &meta("local")).await.err().unwrap_or_else(|| panic!("'{verb}' must be rejected on a viewer instance"));
        assert_eq!(error.origin, FaultOrigin::Framework, "'{verb}' rejection must carry FaultOrigin::Framework");
        assert_eq!(error.code.0, "viewer.read-only", "'{verb}' rejection must carry the frozen viewer.read-only code");
    }
    let media = Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "semio.testkit-surface/v1".into(), json: "{}".into() } };
    let error = app.import_media("any-port", media, &meta("local")).await.err().expect("'import' must be rejected on a viewer instance");
    assert_eq!(error.origin, FaultOrigin::Framework, "'import' rejection must carry FaultOrigin::Framework");
    assert_eq!(error.code.0, "viewer.read-only", "'import' rejection must carry the frozen viewer.read-only code");
}
