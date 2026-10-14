//! 👁️ The read views an app receives, its operation contexts and the empty state lanes.
use semio_framework::ActionKind;
use semio_framework::ActionDefinition;
use semio_framework::FaultCode;
use semio_framework::FaultOrigin;
use super::window_config::{WindowConfigOwner, WindowConfigSnapshot};
use semio_framework::Fault;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use std::collections::BTreeMap;
use pack::snapshot::ArtifactPack;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_value::{ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use crate::sdk_fault::PluginAssemblyError;
use crate::child_view::ChildContentView;
use crate::history_view::HistoryView;
use crate::interaction_topology::empty_domain_hover;
use crate::interaction_topology::empty_domain_selection;
use crate::interaction_topology::leftover_selected_ids_of;

use crate::sdk_fault::plugin_sdk_fault;
use crate::tool_contract::ArtifactInstanceOperationOwnerHandle;

use crate::tool_run_contract::ToolRunView;
use crate::window_surface::UiAssemblyResult;

/// 🧾️ Read-only view of an app's document handed to `ArtifactApp::handle_action`/`render`:
/// the materialized snapshot plus the history metadata (checkpoints/alternatives/undo state)
/// derived from the owning {@link VcsArtifactApp}'s persistent {@link ArtifactStore}.
pub struct ArtifactView<'a, P> {
    pub snapshot: &'a P,
    pub history: &'a HistoryView,
    /// 🧸️ Read access to this document's OWNED CHILDREN, each of which is its own envelope with
    /// its own `ArtifactVcs` history (never an inline subtree). See {@link ChildContentView}.
    pub children: ChildContentView,
    operation: Option<AppOperationContext>,
    render_operation: Option<AppRenderOperationContext>,
    parent_document_id: Option<String>,
    snapshot_read: std::sync::Mutex<Option<store::os_store::snapshot_read::SnapshotRead<P>>>,
    tool_run: Option<ToolRunView>,
    operations: &'a [ArtifactOperationProgress],
    provisional_generation: u64,
    instance: Option<ArtifactInstanceOperationOwnerHandle>,
}

/// 🪪️ Durable authority captured once at public command admission and copied into every
/// plugin continuation record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppOperationContext {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: u64,
    pub generation: u64,
    pub canonical_base_revision: [u8; 32],
    pub retained:RetainedCloneGrant,
    /// 🪪️ Who admitted this command, and when: content-addressed over the admitting actor and the document
    /// store's hybrid logical clock ticked for the admission (`VcsArtifactApp::authoring_seed`). Every id a
    /// command mints is scoped by it, so two writers — or two sessions of one — never mint the same id at one base.
    pub authoring_seed: String,
}

/// 🎨️ Fixed authority for one live renderer-observed document revision. Unlike a command
/// operation, this carries no user invocation id: it binds an app-owned retained background job to
/// the exact instance, base revision and store generation observed by the renderer/refresh pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppRenderOperationContext {
    pub app_instance_id: u32,
    pub base_revision: semio_framework_job::RevisionId,
    pub generation: semio_framework_job::Generation,
    pub canonical_base_revision: [u8; 32],
    pub mounted_policy: crate::MountedOwnerPolicyV1,
}

/// 🧾️ Projects the exact render-pinned store revision onto the decimal-u64 completion lane.
pub fn render_operation_publication_revision(operation: AppRenderOperationContext) -> UiPublicationRevision {
    UiPublicationRevision(u64::from_be_bytes(operation.canonical_base_revision[..8].try_into().expect("revision lane width")))
}

/// 🧾️ Reads the render-pinned store authority required by editable retained controls.
pub fn artifact_render_publication_revision<P>(doc: &ArtifactView<'_, P>) -> UiAssemblyResult<UiPublicationRevision> {
    doc.render_operation().map(render_operation_publication_revision).ok_or_else(|| PluginAssemblyError::new("framework.artifact-render.publication-revision", "editable rendering requires a render-pinned document revision"))
}

impl AppOperationContext {
    /// 🧬️ Fixed-width lowercase encoding for typed persisted continuation fields.
    pub fn canonical_base_revision_hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut result = String::with_capacity(64);
        for byte in self.canonical_base_revision {
            result.push(HEX[usize::from(byte >> 4)] as char);
            result.push(HEX[usize::from(byte & 0x0f)] as char);
        }
        result
    }
}

impl<'a, P> ArtifactView<'a, P> {
    /// 🏗️ A view over a document with no composed children — the overwhelming majority, and the
    /// shape every leaf app and test uses. Prefer this over a struct literal: lane views grow
    /// over time, and a constructor absorbs that growth without touching every call site.
    pub fn new(snapshot: &'a P, history: &'a HistoryView) -> Self {
        Self { snapshot, history, children: ChildContentView::EMPTY, operation: None, render_operation: None, parent_document_id: None, snapshot_read: std::sync::Mutex::new(None), tool_run: None, operations: &[], provisional_generation: 0, instance: None }
    }

    /// 🪪️ Synchronous retained-worker view bound to one exact admitted operation.
    pub fn with_operation(snapshot: &'a P, history: &'a HistoryView, operation: AppOperationContext) -> Self {
        Self {
            snapshot,
            history,
            children: ChildContentView::EMPTY,
            operation: Some(operation),
            render_operation: None,
            parent_document_id: None,
            snapshot_read: std::sync::Mutex::new(None),
            tool_run: None,
            operations: &[],
            provisional_generation: 0,
            instance: None,
        }
    }

    /// 🧩️ App-side composing view for retained command work, which observes its children through
    /// [`ArtifactOwnedToolJobContext::children`] rather than through a host render pass and so
    /// carries no render identity. Synchronous on purpose: it only moves owned fields.
    pub fn with_children(snapshot: &'a P, history: &'a HistoryView, children: ChildContentView) -> Self {
        Self { snapshot, history, children, operation: None, render_operation: None, parent_document_id: None, snapshot_read: std::sync::Mutex::new(None), tool_run: None, operations: &[], provisional_generation: 0, instance: None }
    }

    /// 🪪️ Binds a composing retained view to the exact admitted operation it runs under, so its handlers mint ids
    /// and tool transactions from the admission's authoring seed exactly like a leaf view bound by `with_operation`.
    pub fn bound_to_operation(mut self, operation: AppOperationContext) -> Self {
        self.operation = Some(operation);
        self
    }

    /// 🏗️ A view over a composing document, wired to its live child stores AND to the identity the
    /// live revision is observed under. Every host entry point that hands an app a composing view —
    /// `render`, `pending_effects`, `window_engagements`, `window_measures`, `tool_measures`,
    /// `context_menu`, `interaction_topology`, `export_media`, `media_fingerprint` — goes through
    /// here, so [`ArtifactView::render_operation`] is total on the host and an app that leases a
    /// per-instance session off it (puzzle 3d's `puzzle3d_view_session_key`) never runs cold.
    pub(crate) async fn with_render_context(snapshot: &'a P, history: &'a HistoryView, children: ChildContentView, render_operation: AppRenderOperationContext, parent_document_id: String, snapshot_read: Option<store::os_store::snapshot_read::SnapshotRead<P>>) -> Self {
        Self {
            snapshot,
            history,
            children,
            operation: None,
            render_operation: Some(render_operation),
            parent_document_id: Some(parent_document_id),
            snapshot_read: std::sync::Mutex::new(snapshot_read),
            tool_run: None,
            operations: &[],
            provisional_generation: 0,
            instance: None,
        }
    }

    /// 🔮️ Binds the mounted instance this view belongs to, whose handle owns the instance's derived-state sessions.
    pub fn with_instance(mut self, instance: ArtifactInstanceOperationOwnerHandle) -> Self {
        self.instance = Some(instance);
        self
    }

    /// 🔮️ The mounted instance this view belongs to, `None` for a view no instance hosts (a fixture, a one-shot read).
    pub fn instance(&self) -> Option<&ArtifactInstanceOperationOwnerHandle> {
        self.instance.as_ref()
    }

    /// ⏯️ Binds the tool run this document instance shows; `snapshot` is then committed ⊕ provisional.
    pub(crate) fn with_tool_run(mut self, tool_run: Option<ToolRunView>) -> Self {
        self.tool_run = tool_run;
        self
    }

    /// ⏯️ The run whose provisional entities render with the `provisional` style token and stay
    /// out of interaction topology (contract §4.1 layer 1).
    pub fn tool_run(&self) -> Option<&ToolRunView> {
        self.tool_run.as_ref()
    }

    /// 🔢️ Binds the instance's continuous-tool overlay generation, which moves on every tick, release and zero-trace
    /// abort of an open press or typing run.
    pub(crate) fn with_provisional_generation(mut self, generation: u64) -> Self {
        self.provisional_generation = generation;
        self
    }

    /// 🔢️ The continuous-tool overlay generation as of this view: a derived view (a preview) whose input follows the
    /// open transactions (`ArtifactOwnedToolJobContext::provisional`) owes a fresh evaluation whenever it moves, even
    /// when no edit landed (a zero-trace abort).
    pub fn provisional_generation(&self) -> u64 {
        self.provisional_generation
    }

    /// ⏳️ Current local command work; observation never enters artifact history.
    pub fn operations(&self) -> &[ArtifactOperationProgress] {
        self.operations
    }

    pub(crate) fn with_operations(mut self, operations: &'a [ArtifactOperationProgress]) -> Self {
        self.operations = operations;
        self
    }

    /// 🪪️ Returns the actual public job authority for command handlers.
    pub fn operation(&self) -> Result<&AppOperationContext, Fault> {
        self.operation.as_ref().ok_or_else(|| plugin_sdk_fault("the artifact view is not bound to a public command operation"))
    }

    /// 🎟️ Returns the original explicitly admitted command or render maintenance authority.
    pub fn retained_grant(&self)->Result<RetainedCloneGrant,Fault>{
        if let Some(operation)=&self.operation{return Ok(operation.retained);}
        if let Some(operation)=self.render_operation{return Ok(operation.mounted_policy.maintenance);}
        Err(plugin_sdk_fault("artifact view has no original admitted retained authority"))
    }

    /// 🪪️ Optional form for render/test-only views which do not enter command authority.
    pub fn operation_optional(&self) -> Option<&AppOperationContext> {
        self.operation.as_ref()
    }

    /// 🎨️ Returns the immutable identity observed by a live render/refresh pass.
    pub fn render_operation(&self) -> Option<AppRenderOperationContext> {
        self.render_operation
    }

    /// 🧾️ Returns the exact live store envelope id bound to this host render pass.
    pub fn parent_document_id(&self) -> Option<&str> {
        self.parent_document_id.as_deref()
    }

    /// 🧵️ Transfers the one registered immutable snapshot-read lease to a retained worker
    /// owner. Contention and a second take fail closed without cloning the underlying snapshot.
    pub fn take_snapshot_read(&self) -> Result<store::os_store::snapshot_read::SnapshotRead<P>, Fault> {
        let mut owner = self.snapshot_read.try_lock().map_err(|_| plugin_sdk_fault("the live snapshot-read lease is busy"))?;
        owner.take().ok_or_else(|| plugin_sdk_fault("the artifact view has no transferable live snapshot-read lease"))
    }
}

/// 🧮️ Read-only view of an app's config snapshot — same role as {@link ArtifactView} for the
/// config {@link ConfigStore} owned by {@link VcsArtifactApp}.
pub struct ConfigView<'a, C> {
    pub snapshot: &'a C,
    pub window: Option<&'a WindowConfigSnapshot>,
}

impl<'a, C> ConfigView<'a, C> {
    pub fn window<O: WindowConfigOwner>(&self) -> Option<&O::State> {
        self.window.and_then(WindowConfigSnapshot::get::<O>)
    }
}

/// 📝️ Read-only view of an app's volatile draft snapshot — same role as {@link ConfigView}
/// for the draft {@link store::DraftStore} (ephemeral; never checkpoints).
pub struct DraftView<'a, D> {
    pub snapshot: &'a D,
}

/// 👥️ Read-only view of the PRESENCE lane: this actor's own live shared state plus every
/// peer's, as last broadcast. Ephemeral and shared — never persisted, never undoable.
pub struct PresencePeersView<'a, P> {
    pub(crate) root: &'a store::os_store::presence::PresencePeersRoot<P>,
}

impl<'a, P> PresencePeersView<'a, P> {
    /// 👥️ Iterates the already-sorted immutable peer root without projecting a roster.
    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &'a P)> {
        self.root.peers()
    }

    pub fn len(&self) -> usize {
        self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }
}

pub struct PresenceView<'a, P> {
    /// 👤️ This actor's own presence — the value that gets broadcast.
    pub local: &'a P,
    /// 👥️ Every other peer's presence, sorted by actor id for a stable render order.
    pub peers: PresencePeersView<'a, P>,
}

/// 🕹️ Read-only view of the framework-owned INTERACTION mechanism (hover + selection + active
/// mode/granularity per domain) — threaded into `ArtifactApp::handle`/`copy_fragment`/
/// `cut_operations` so an app reads its own current selection/hover instead of ever storing it
/// again itself. `state` is the persisted-local half (selection/mode/granularity, one
/// `VcsArtifactApp::interaction_store` `ApplyInLane{lane: HistoryLane::Interaction}` dispatch away
/// from every change); `hover` is the ephemeral-local half (`VcsArtifactApp::interaction_hover`,
/// never persisted — see that field's own doc comment). Both are combined once per dispatch by
/// `VcsArtifactApp::interaction_view`, never mutated through this read-only handle.
pub struct InteractionView<'a> {
    pub(crate) state: &'a protocol::InteractionState,
    pub(crate) hover: &'a InteractionHoverState,
    /// 👥️ Every OTHER peer's last-adopted presence, keyed by actor — see
    /// `VcsArtifactApp::peer_presence`'s own doc comment. Sourced from `BTreeMap` iteration
    /// order, which is already sorted by actor.
    pub(crate) peers: &'a PeerPresenceRoot,
    /// ⏪️ What the open time-travel draft references, per selection domain
    /// ([`crate::time_travel::TimeTravelLedger::draft_references`]) — empty while no draft editor is open.
    pub(crate) draft_references: &'a BTreeMap<String, Vec<String>>,
}

impl<'a> InteractionView<'a> {
    /// 🔗️ `domain`'s ids the open time-travel draft references — every id its `Reference` inputs of that domain
    /// hold, so a render highlights what the edited mutation acts on; empty while no draft editor is open.
    pub fn draft_references(&self, domain: &str) -> &[String] {
        self.draft_references.get(domain).map_or(&[], Vec::as_slice)
    }

    /// 🖱️ `domain`'s current selection — an empty `DomainSelection` (not `None`) for an undeclared
    /// or never-selected domain, so call sites never have to unwrap.
    pub fn selection(&self, domain: &str) -> &protocol::DomainSelection {
        self.state.selection.get(domain).unwrap_or_else(|| empty_domain_selection())
    }

    /// 🕹️ Leftover.ids for Inspection: every domain's selected ids as a SET (see
    /// {@link leftover_selected_ids_of}). A pick can land on leftover `selectedIds` while
    /// `selection(vortex)` is still the empty persist snapshot.
    pub fn leftover_selected_ids(&self) -> Vec<String> {
        leftover_selected_ids_of(self.state)
    }

    /// 🐁️ `domain`'s current hover on `channel` — empty when nothing is hovered on that channel
    /// right now. `InteractionState.hover` holds exactly one live channel per domain at a time (the
    /// most recently hovered — see `next_hover`'s doc), so a `channel` mismatch also reads empty
    /// rather than returning a stale different-channel hover.
    pub fn hover(&self, domain: &str, channel: &str) -> &protocol::DomainHover {
        match self.hover.get(domain) {
            Some(hover) if hover.channel == channel => hover,
            _ => empty_domain_hover(),
        }
    }

    /// 🪜️ `domain`'s active granularity id, `None` for an undeclared domain (never dispatched
    /// through `validate_state` yet — e.g. a registry-less test construction).
    pub fn active_granularity(&self, domain: &str) -> Option<&str> {
        self.state.active_granularity.get(domain).map(String::as_str)
    }

    /// 🔀️ `domain`'s active `SelectionMode`, `None` for an undeclared domain.
    pub fn active_mode(&self, domain: &str) -> Option<protocol::SelectionMode> {
        self.state.active_mode.get(domain).copied()
    }

    /// 👥️ Every OTHER peer currently selecting `id` within `domain`, sorted by actor
    /// (contract-freeze §C7.6) — `[]` for a peer with no adopted interaction, an interaction with
    /// no matching domain, or a domain where `id` is not in `selected`.
    /// 🚫️async: E1 pure in-memory filter, no suspension point — see R9.
    ///
    /// ⚠️ Gap (SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY packet `sdk-helpers` — see
    /// `📓️terra-sdk-helpers-report.md`): the sync `marks_for` closure this used to feed
    /// (`stamp_and_cache_interaction_ui`'s old presence-stamping half) is gone — that destination no
    /// longer exists on a built node. This method itself is unaffected and still tested directly; a
    /// future packet publishing real `PresenceUpdate`s is the next intended caller.
    pub fn peers_selecting(&self, domain: &str, id: &str) -> Vec<PeerMark<'a>> {
        self.peers
            .iter()
            .filter_map(|(actor, presence)| {
                let interaction = presence.interaction.as_ref()?;
                let matched = interaction.domains.iter().find(|candidate| candidate.domain == domain)?;
                matched.selected.iter().any(|selected_id| selected_id == id).then_some(PeerMark { actor, color: presence.color })
            })
            .collect()
    }

    /// 👥️ `peers_selecting`'s hover twin — every OTHER peer currently hovering `id` within
    /// `domain`, sorted by actor.
    /// 🚫️async: E1 pure in-memory filter — see `peers_selecting`'s doc for why this must be sync.
    pub fn peers_hovering(&self, domain: &str, id: &str) -> Vec<PeerMark<'a>> {
        self.peers
            .iter()
            .filter_map(|(actor, presence)| {
                let interaction = presence.interaction.as_ref()?;
                let matched = interaction.domains.iter().find(|candidate| candidate.domain == domain)?;
                matched.hovered.iter().any(|hovered_id| hovered_id == id).then_some(PeerMark { actor, color: presence.color })
            })
            .collect()
    }
}

/// 🕳️ The diff of an uninhabited lane (`NoConfig`, `NoPresence`, `NoTransient`): there is no mutation to produce one, so it
/// carries nothing and leaves every state as it is.
#[derive(Clone, Debug, PartialEq, Default, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoStateDiff {}

/// 🧮️ Default `ArtifactApp::Config` for apps with no config artifact yet.
#[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetireOwned, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoConfig {}

impl semio_framework_schema_composition::ArtifactCompositionFields for NoConfig {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl pack::snapshot::ArtifactDsl for NoConfig {
    const EXTENSION: &'static str = "nocfg";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no config", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl semio_framework_dsl_record::DslField for NoConfig {
    fn shape() -> semio_framework_dsl_record::Shape {
        fn spec() -> semio_framework_dsl_record::RecordSpec {
            semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, Vec::new())
        }
        semio_framework_dsl_record::Shape::Record(semio_framework_dsl_record::RecordSpecProducer {
            ordinary: spec,
            decoding: |control| {
                control.checkpoint()?;
                Ok(semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, control.allocate_vec(0)?))
            },
            encoding: |control| {
                control.checkpoint()?;
                Ok(semio_framework_dsl_record::RecordSpec::new(None, semio_framework_dsl_record::RecordLayout::Lines, control.allocate_vec(0)?))
            },
        })
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Record(semio_framework_dsl_record::RecordValue::default())
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Record(record) if record.fields.is_empty() => Ok(Self::default()),
            _ => Err("no config state requires an empty record".into()),
        }
    }
}

impl ArtifactPack for NoConfig {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            Ok(Self::default())
        } else {
            Err(store::PackError::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty state must not contain bytes")))
        }
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NoConfig {

}

impl store::ConfigRecord for NoConfig {}

impl ::protocol::DiffAlgebra<NoConfig> for NoStateDiff {
    fn inverse(&self, _base: &NoConfig) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl ::protocol::MutationDiff<NoConfig> for NoStateDiff {
    fn apply(&self, base: &NoConfig, _capability: ::protocol::ApplyCapability) -> protocol::MutationApplyResult<NoConfig> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_dsl_record_derive::DslEnum, semio_framework_dsl_record_derive::DslReceiving, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum NoConfigMutation {}
semio_framework_os_kernel::original_ordinal_operation_receiving!(NoConfigMutation);


impl ::protocol::Mutation<NoConfig> for NoConfigMutation {
    type Diff = NoStateDiff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        match *self {}
    }

    fn diff(&self, _base: &NoConfig) -> ::protocol::MutationOutcome<NoStateDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &NoConfig) -> Result<Vec<Self>, ValueError> {
        Ok((|| match *self {})())
    }
}

/// 🕳️ The uninhabited lane has no kinds, so no operation of it is ever labelled.
impl protocol::SemanticMutation<NoConfig> for NoConfigMutation {
    fn kinds() -> &'static [protocol::SemanticDescriptor] {
        &[]
    }
    fn semantics(&self) -> &'static protocol::SemanticDescriptor {
        match *self {}
    }
    fn label(&self) -> LocalizedLabel {
        match *self {}
    }
    fn target(&self) -> Vec<String> {
        match *self {}
    }
}

impl ::protocol::OpText for NoConfigMutation {
    fn parse_op(_line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        Err(::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no config mutations exist", ::semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl ::protocol::OpBinary for NoConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Err(::protocol::ProtocolError::Malformed { what: "no-config-mutation", offset: 0, detail: "no config mutations exist".into() })
    }
}

/// 📝️ Default `ArtifactApp::Draft` for apps with no draft lane yet.
pub type NoDraft = NoConfig;

/// 📝️ Default `ArtifactApp::DraftMutation` twin of {@link NoDraft}.
pub type NoDraftMutation = NoConfigMutation;

/// 👥️ Default `ArtifactApp::Presence` for apps with no shareable live state yet.
#[derive(Clone, Debug, PartialEq, Default, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoPresence {}

impl store::ArtifactPresenceSnapshot for NoPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut crate::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Self, ValueError> {
        let grant = owner.remaining_grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "empty presence needs original work admission")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "empty presence needs original depth admission")); }
        owner.native().checkpoint()?;
        if !bytes.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "empty presence has no encoded fields")); }
        owner.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(Self {})
    }

    fn encode_presence_native(&self, owner: &mut crate::io::control::NativeSnapshotEncodeOwner<'_, '_>) -> Result<Vec<u8>, ValueError> {
        let grant = owner.remaining_grant();
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "empty presence needs original work admission")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "empty presence needs original depth admission")); }
        owner.native().checkpoint()?;
        owner.record_progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
        Ok(Vec::new())
    }
}

impl pack::snapshot::ArtifactDsl for NoPresence {
    const EXTENSION: &'static str = "nopres";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no presence", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for NoPresence {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            Ok(Self::default())
        } else {
            Err(store::PackError::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty state must not contain bytes")))
        }
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NoPresence {

}

impl ::protocol::DiffAlgebra<NoPresence> for NoStateDiff {
    fn inverse(&self, _base: &NoPresence) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl ::protocol::MutationDiff<NoPresence> for NoStateDiff {
    fn apply(&self, base: &NoPresence, _capability: ::protocol::ApplyCapability) -> protocol::MutationApplyResult<NoPresence> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[derive(semio_framework_value::RetireOwned)]
pub enum NoPresenceMutation {}

impl ::protocol::Mutation<NoPresence> for NoPresenceMutation {
    type Diff = NoStateDiff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        match *self {}
    }

    fn diff(&self, _base: &NoPresence) -> ::protocol::MutationOutcome<NoStateDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &NoPresence) -> Result<Vec<Self>, ValueError> {
        Ok((|| match *self {})())
    }
}

impl ::protocol::OpText for NoPresenceMutation {
    fn parse_op(_line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        Err(::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no presence mutations exist", ::semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl ::protocol::OpBinary for NoPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Err(::protocol::ProtocolError::Malformed { what: "no-presence-mutation", offset: 0, detail: "no presence mutations exist".into() })
    }
}

/// 🫧️ Default `ArtifactApp::Transient` for apps with no ephemeral local UI state yet.
///
/// 🎯️ The FOURTH and last state mechanism. The four are exhaustive and mutually exclusive:
/// **artifact** = persisted + shared, **config** = persisted + local-only, **presence** =
/// ephemeral + shared, **transient** = ephemeral + local-only. Anything that used to live in a
/// plugin `thread_local!`, a process-local ephemeral box, or an untyped shell field belongs
/// here — typed, dispatched through `Emit`, and readable through `Lanes`, exactly like the other
/// three, rather than reachable from anywhere at any time.
///
/// ⚠️ Transient is NOT the draft lane. A DRAFT is an ephemeral *artifact* — real document
/// content that simply has not been committed. TRANSIENT is UI state that is never document
/// content at all (which pane is focused, what is hovered, an in-flight gesture). They differ in
/// what the state IS, not in how long it lives.
#[derive(Clone, Debug, PartialEq, Default, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoTransient {}

impl pack::snapshot::ArtifactDsl for NoTransient {
    const EXTENSION: &'static str = "notrans";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no transient", semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        String::new()
    }
}

impl ArtifactPack for NoTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(Vec::new())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            Ok(Self::default())
        } else {
            Err(store::PackError::from(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "empty state must not contain bytes")))
        }
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for NoTransient {

}

impl ::protocol::DiffAlgebra<NoTransient> for NoStateDiff {
    fn inverse(&self, _base: &NoTransient) -> Self {
        Self {}
    }
    fn is_empty(&self) -> bool {
        true
    }
}

impl ::protocol::MutationDiff<NoTransient> for NoStateDiff {
    fn apply(&self, base: &NoTransient, _capability: ::protocol::ApplyCapability) -> protocol::MutationApplyResult<NoTransient> {
        Ok(base.clone())
    }
    fn absorb(&mut self, _other: Self) {}
}

#[derive(Clone, Debug, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[derive(semio_framework_value::RetireOwned)]
pub enum NoTransientMutation {}

impl ::protocol::Mutation<NoTransient> for NoTransientMutation {
    type Diff = NoStateDiff;
    const DESCRIPTORS: &'static [::protocol::MutationLeafDescriptor] = &[];

    fn descriptor(&self) -> &'static ::protocol::MutationLeafDescriptor {
        match *self {}
    }

    fn diff(&self, _base: &NoTransient) -> ::protocol::MutationOutcome<NoStateDiff> {
        match *self {}
    }

    fn inverse(&self, _base: &NoTransient) -> Result<Vec<Self>, ValueError> {
        Ok((|| match *self {})())
    }
}

impl ::protocol::OpText for NoTransientMutation {
    fn parse_op(_line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        Err(::semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "no transient mutations exist", ::semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        match *self {}
    }
}

impl ::protocol::OpBinary for NoTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        match *self {}
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Err(::protocol::ProtocolError::Malformed { what: "no-transient-mutation", offset: 0, detail: "no transient mutations exist".into() })
    }
}

/// 🐁️ The ephemeral, never-persisted half of the interaction mechanism — `VcsArtifactApp`'s hover
/// map, one `DomainHover` per domain (mirrors `protocol::InteractionState.hover`'s own shape; see
/// that field's doc for why one slot per domain, not per channel). Deliberately a plain type alias,
/// not a generic `TransientStore<P, M>` instance: hover here is applied directly by the interaction
/// dispatch interception, never through an app's own `Emit`/ephemeral lane, so the `Mutation`-trait
/// machinery `TransientStore` requires would be unused ceremony.
pub type InteractionHoverState = BTreeMap<String, protocol::DomainHover>;

#[derive(Clone, Debug)]
pub struct ArtifactOperationProgress {
    pub operation_id: u64,
    pub generation: u64,
    pub label: LocalizedLabel,
    pub completed_units: u64,
    pub cancelling: bool,
}

/// 👥️ One OTHER peer's last-adopted artifact/app-scope presence slice — the plugin-side
/// twin of `PresencePeer`'s `color`/`surface`/`interaction` fields, keyed by actor in
/// `VcsArtifactApp::peer_presence` (contract-freeze §C7.6). `presence_pack`'s decoded app-typed
/// value is NOT carried here — it lands in `presence_store` instead (the existing peer-roster
/// mechanism, unchanged), so this struct stays generic over every `ArtifactApp`.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct PeerPresence {
    pub color: Option<u8>,
    pub surface: Option<String>,
    pub interaction: Option<protocol::PresenceInteraction>,
}

pub(crate) const PEER_PRESENCE_SLOTS: usize = 64;

const PEER_PRESENCE_ID_BYTES: usize = 256;

#[derive(semio_framework_value::RetireOwned)]
pub(crate) struct PeerPresenceEntry {
    actor: String,
    presence: PeerPresence,
}

pub(crate) struct PeerPresenceRoot {
    pub(crate) entries: [Option<std::sync::Arc<PeerPresenceEntry>>; PEER_PRESENCE_SLOTS],
    pub(crate) len: usize,
}

impl PeerPresenceRoot {
    pub(crate) fn empty() -> Self {
        Self { entries: std::array::from_fn(|_| None), len: 0 }
    }

    pub(crate) fn empty_shared() -> std::sync::Arc<Self> {
        static EMPTY: std::sync::OnceLock<std::sync::Arc<PeerPresenceRoot>> = std::sync::OnceLock::new();
        EMPTY.get_or_init(|| std::sync::Arc::new(Self::empty())).clone()
    }

    pub(crate) fn insert(&mut self, actor: String, presence: PeerPresence) -> Result<(), Fault> {
        if actor.is_empty() || actor.len() > PEER_PRESENCE_ID_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-presence-actor"), "peer actor identity is empty or exceeds its fixed byte authority"));
        }
        let mut index = 0usize;
        while index < self.len && self.entries[index].as_ref().is_some_and(|entry| entry.actor.as_str() < actor.as_str()) {
            index += 1;
        }
        if self.entries.get(index).and_then(Option::as_ref).is_some_and(|entry| entry.actor == actor) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-presence-duplicate"), "one presence roster contains a duplicate actor identity"));
        }
        if self.len == PEER_PRESENCE_SLOTS {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-presence-capacity"), "presence roster exceeds its fixed peer authority"));
        }
        for cursor in (index..self.len).rev() {
            self.entries[cursor + 1] = self.entries[cursor].take();
        }
        self.entries[index] = Some(std::sync::Arc::new(PeerPresenceEntry { actor, presence }));
        self.len += 1;
        Ok(())
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &PeerPresence)> {
        self.entries[..self.len].iter().filter_map(|entry| entry.as_deref().map(|entry| (entry.actor.as_str(), &entry.presence)))
    }

    fn get(&self, actor: &str) -> Option<&PeerPresence> {
        self.iter().find_map(|(candidate, presence)| (candidate == actor).then_some(presence))
    }

    pub(crate) fn contains_key(&self, actor: &str) -> bool {
        self.get(actor).is_some()
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// 👥️ One peer's mark on a single interaction target — `InteractionView::peers_selecting`/
/// `peers_hovering`'s element type (contract-freeze §C7.6), sorted by actor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PeerMark<'a> {
    pub actor: &'a str,
    pub color: Option<u8>,
}

pub const CANCEL_TYPED_OPERATION_ACTION_ID: &str = "cancelTypedOperation";

pub fn cancellation_action_definition() -> ActionDefinition {
    ActionDefinition::resumable_framework(CANCEL_TYPED_OPERATION_ACTION_ID, LocalizedLabel::native("Cancel Operation", "Vorgang abbrechen"), ActionKind::View, "square").with_in_palette(false)
}
