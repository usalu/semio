//! 🔌️ Plugin host, studio document VCS store, backbone, and catalog.
// #region host

use crate::instance::{create_os_id, OsInstanceState};
use crate::registry::{os_app_registration, resolve_os_app_definition, PluginRegistry};
use crate::space;
use crate::workflow;
use protocol::Mutation;
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework::{AppDefinition, PluginManifest, ProgramContributionEntry, ViewModel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, LazyLock, Mutex};
use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::document_backbone_ref, os_store::document_materialize::materialize_document_snapshot, os_store::store_schema::ArtifactBackboneRef, os_store::store_schema::ArtifactCommand, os_store::store_schema::ArtifactEnvelope, ArtifactStore};
use ui_wgpu::wgpu::{ui_recovery_panel, UiNode};
use vcs::{ArtifactVcs, VcsError};

/// 🌉️ Synchronous bridge for the kernel/`semio_framework` calls this crate's already-published
/// sync API (`BackboneDocument` codecs, `OsBackbonePort`'s in-memory/localStorage bridge, media
/// negotiation, the `io_dialects_for`/`io_route`/`io_run`/`io_dispatch` mechanism, workflow node/
/// parameter/validation helpers, `AppIo`/`CommandGrammar` reconstruction) still depend on, now
/// that those crates made them `async fn` for their own pooled-actor substrate. Every caller of
/// this helper is over a documented-immediate operation — pure data transforms and registry
/// lookups, never the file/folder host ports (which stay behind their own genuinely-async
/// `crate::backbone::SpaceBackbonePort` path) — so a single poll always resolves `Ready`. Not a
/// second executor entry point: it holds no runtime, spawns nothing, and never parks — it is one
/// bare `Future::poll`, same shape and same justification as this crate's existing test-only
/// bridge just below (`ExtensionInstall`'s `block_on`), lifted out of `#[cfg(test)]` because this
/// crate's sync surface needs it in production too.
pub(crate) fn resolve_kernel_future<F: std::future::Future>(future: F) -> F::Output {
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut future = std::pin::pin!(future);
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("resolve_kernel_future: this call site is documented immediate"),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramHotSwapEvent {
    pub plugin_id: String,
    pub version: String,
    pub added_apps: Vec<String>,
    pub removed_apps: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoadedProgram {
    pub plugin_id: String,
    pub manifest: PluginManifest,
    pub artifact_uri: String,
}

pub struct PluginHost {
    registry: PluginRegistry,
    instances: HashMap<u32, OsInstanceState>,
    next_instance_id: u32,
    programs: HashMap<String, LoadedProgram>,
    //#region 🔖️Quarantine
    /// 🚧️ Plugin ids currently held out of rotation for `recovery_ui`. No producer in this crate
    /// sets this today (nothing here is wired to the actor kernel's `FailureStage::Quarantined` —
    /// see `📓️terra-C1-report.md`); `load_plugin`/`hot_swap_plugin` clear it on every successful
    /// or rolled-back load, matching a manual operator restart.
    quarantined: HashSet<String>,
    //#endregion 🔖️Quarantine
    //#region 🔖️ExtensionInstall
    /// 🧩️ Installed `.sxt` extension descriptors, keyed by `extension_id` — see
    /// `install_extension_package`. `.sxt` install is native-host tooling (mirrors
    /// `store::extension`'s own `not(all(target_arch = "wasm32", target_env = "p2"))` gate — a
    /// guest component never installs an extension into itself).
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    installed_extensions: HashMap<String, InstalledExtension>,
    //#endregion 🔖️ExtensionInstall
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            instances: HashMap::new(),
            next_instance_id: 1,
            programs: HashMap::new(),
            quarantined: HashSet::new(),
            #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
            installed_extensions: HashMap::new(),
        }
    }

    pub fn is_quarantined(&self, plugin_id: &str) -> bool {
        self.quarantined.contains(plugin_id)
    }

    pub fn registry(&self) -> &PluginRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut PluginRegistry {
        &mut self.registry
    }

    pub fn load_plugin(&mut self, program: LoadedProgram) -> ProgramHotSwapEvent {
        let plugin_id = program.plugin_id.clone();
        let version = program.manifest.version.clone();
        let previous_apps: Vec<String> = self.programs.get(&plugin_id).map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        let next_apps: Vec<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();
        for app in &program.manifest.apps {
            self.registry.register_app(app.clone());
            // 🪐️ B1 ripple resolved: seeds `registry::APP_REGISTRATIONS` (`workflow_palette()`'s
            // backing store) at native/test load time — the wasm-hosted browser shell has no direct
            // call path here, so it pushes the equivalent over the wire instead (see the space app's
            // `SpaceCommand::SetAppRegistrations`).
            crate::registry::register_app_io(&plugin_id, app);
        }
        crate::registry::register_artifact_descriptors(&program.manifest);
        self.programs.insert(plugin_id.clone(), program);
        self.quarantined.remove(&plugin_id);
        ProgramHotSwapEvent { plugin_id, version, added_apps: next_apps.iter().filter(|app| !previous_apps.contains(app)).cloned().collect(), removed_apps: previous_apps.iter().filter(|app| !next_apps.contains(app)).cloned().collect() }
    }

    pub fn hot_swap_plugin(&mut self, program: LoadedProgram) -> ProgramHotSwapEvent {
        let plugin_id = program.plugin_id.clone();
        let rollback = HotSwapRollback { previous_plugin: self.programs.get(&plugin_id).cloned(), instance_generations: self.instances.iter().map(|(id, state)| (*id, state.generation)).collect() };

        if let Err(error) = validate_plugin_manifest(&program) {
            self.quarantined.remove(&plugin_id);
            return rollback.emit_failure(plugin_id, error);
        }

        let previous_apps: Vec<String> = rollback.previous_plugin.as_ref().map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        let next_apps: Vec<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();

        if let Err(error) = self.validate_swap_apps(&program) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_instances(&plugin_id, &program) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_app_retention(&program, rollback.previous_plugin.as_ref()) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }
        if let Err(error) = self.validate_swap_window_kinds(&program) {
            return self.hot_swap_failed(plugin_id, error, rollback);
        }

        let controller_rebindings = self.plan_controller_rebindings(&plugin_id, &program);
        let version = program.manifest.version.clone();
        for app in &program.manifest.apps {
            self.registry.register_app(app.clone());
            // 🪐️ See the `load_plugin` sibling loop's comment above.
            crate::registry::register_app_io(&plugin_id, app);
        }
        crate::registry::register_artifact_descriptors(&program.manifest);
        self.programs.insert(plugin_id.clone(), program);
        for (instance_id, controller_id) in controller_rebindings {
            if let Some(instance) = self.instances.get_mut(&instance_id) {
                instance.controller_id = controller_id;
            }
        }
        for instance in self.instances.values_mut() {
            instance.generation += 1;
        }
        self.quarantined.remove(&plugin_id);
        ProgramHotSwapEvent { plugin_id, version, added_apps: next_apps.iter().filter(|app| !previous_apps.contains(app)).cloned().collect(), removed_apps: previous_apps.iter().filter(|app| !next_apps.contains(app)).cloned().collect() }
    }

    pub fn apps(&self) -> Vec<AppDefinition> {
        self.registry.apps()
    }

    pub fn contributions(&self) -> Vec<ProgramContributionEntry> {
        let mut entries = Vec::new();
        for loaded in self.programs.values() {
            for topic_contribution in &loaded.manifest.topic_contributions {
                entries.push(ProgramContributionEntry { plugin_id: loaded.plugin_id.clone(), topic_contribution: Some(topic_contribution.clone()) });
            }
        }
        entries
    }

    pub fn contributions_json(&self) -> String {
        semio_framework_pack_json::to_json_string(&self.contributions())
    }

    pub fn create_instance(&mut self, app_id: &str, document_json: String, view_state: ViewModel) -> Option<u32> {
        let app = self.registry.find_app(app_id)?;
        let id = self.next_instance_id;
        self.next_instance_id += 1;
        self.instances.insert(id, OsInstanceState { id, app_id: app.id.clone(), controller_id: app.controller_id.clone(), document_json, view_state, generation: 0 });
        Some(id)
    }

    pub fn instance(&self, instance_id: u32) -> Option<&OsInstanceState> {
        self.instances.get(&instance_id)
    }

    pub fn instance_mut(&mut self, instance_id: u32) -> Option<&mut OsInstanceState> {
        self.instances.get_mut(&instance_id)
    }

    //#region 🔖️ActionKernel

    /// 🩺️ Delegates to `ui_wgpu::wgpu::ui_recovery_panel`'s `🔖️StatusBuilders` builder — this host
    /// has no locale on hand at this call site (no `ViewModel` threaded into `recovery_ui`), so
    /// `is_de` is pinned to `false` (English) until a locale source is plumbed through.
    pub fn recovery_ui(&self, plugin_id: &str) -> UiNode {
        ui_recovery_panel(plugin_id, self.is_quarantined(plugin_id), false)
    }
    //#endregion 🔖️ActionKernel

    pub fn set_view_state(&mut self, instance_id: u32, view_state: ViewModel) {
        if let Some(instance) = self.instances.get_mut(&instance_id) {
            instance.view_state = view_state;
            instance.generation += 1;
        }
    }

    pub fn render_body(&self, instance_id: u32, body_key: &str, ui: UiNode) -> UiNode {
        let _ = (instance_id, body_key);
        ui
    }

    fn hot_swap_failed(&mut self, plugin_id: String, error: String, rollback: HotSwapRollback) -> ProgramHotSwapEvent {
        rollback.restore(self);
        self.quarantined.remove(&plugin_id);
        rollback.emit_failure(plugin_id, error)
    }

    fn validate_swap_apps(&self, program: &LoadedProgram) -> Result<(), String> {
        for app in &program.manifest.apps {
            if app.id.trim().is_empty() {
                return Err("app id must not be empty".into());
            }
            if app.controller_id.trim().is_empty() {
                return Err(format!("app {} controller_id must not be empty", app.id));
            }
        }
        Ok(())
    }

    fn validate_swap_instances(&self, plugin_id: &str, program: &LoadedProgram) -> Result<(), String> {
        let next_app_ids: HashSet<String> = program.manifest.apps.iter().map(|app| app.id.clone()).collect();
        let previous_app_ids: HashSet<String> = self.programs.get(plugin_id).map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        for instance in self.instances.values() {
            if !previous_app_ids.contains(&instance.app_id) {
                continue;
            }
            if !next_app_ids.contains(&instance.app_id) {
                return Err(format!("instance {} references removed app {}", instance.id, instance.app_id));
            }
        }
        Ok(())
    }

    /// 🔢️ Same-version hot-swaps must not silently drop apps — a version bump is required to
    /// shrink the app set, so a client relying on document/instance continuity can detect the change.
    fn validate_swap_app_retention(&self, program: &LoadedProgram, previous: Option<&LoadedProgram>) -> Result<(), String> {
        if let Some(previous) = previous {
            if previous.manifest.version == program.manifest.version && previous.manifest.apps.len() > program.manifest.apps.len() {
                return Err("cannot hot-swap to fewer apps within the same version".into());
            }
        }
        Ok(())
    }

    fn validate_swap_window_kinds(&self, program: &LoadedProgram) -> Result<(), String> {
        for app in &program.manifest.apps {
            if app.window_kinds.is_empty() {
                return Err(format!("app {} must declare at least one window kind", app.id));
            }
            for window_kind in &app.window_kinds {
                if window_kind.body_key.trim().is_empty() {
                    return Err(format!("app {} window kind {} body_key must not be empty", app.id, window_kind.id));
                }
            }
        }
        Ok(())
    }

    fn plan_controller_rebindings(&self, plugin_id: &str, program: &LoadedProgram) -> Vec<(u32, String)> {
        let apps_by_id: HashMap<&str, &AppDefinition> = program.manifest.apps.iter().map(|app| (app.id.as_str(), app)).collect();
        let previous_app_ids: HashSet<String> = self.programs.get(plugin_id).map(|existing| existing.manifest.apps.iter().map(|app| app.id.clone()).collect()).unwrap_or_default();
        self.instances.values().filter(|instance| previous_app_ids.contains(&instance.app_id)).filter_map(|instance| apps_by_id.get(instance.app_id.as_str()).map(|app| (instance.id, app.controller_id.clone()))).collect()
    }
}

struct HotSwapRollback {
    previous_plugin: Option<LoadedProgram>,
    instance_generations: HashMap<u32, u64>,
}

impl HotSwapRollback {
    fn emit_failure(self, plugin_id: String, _error: String) -> ProgramHotSwapEvent {
        let version = self.previous_plugin.as_ref().map(|previous| previous.manifest.version.clone()).unwrap_or_default();
        ProgramHotSwapEvent { plugin_id, version, added_apps: vec![], removed_apps: vec![] }
    }

    fn restore(&self, host: &mut PluginHost) {
        if let Some(previous) = &self.previous_plugin {
            for app in &previous.manifest.apps {
                host.registry.register_app(app.clone());
            }
            // 🚧️ B1 ripple: see `PluginHost::load_plugin`'s sibling loop comment.
            host.programs.insert(previous.plugin_id.clone(), previous.clone());
        }
        for (instance_id, generation) in &self.instance_generations {
            if let Some(instance) = host.instances.get_mut(instance_id) {
                instance.generation = *generation;
            }
        }
    }
}

fn validate_plugin_manifest(program: &LoadedProgram) -> Result<(), String> {
    if program.plugin_id.trim().is_empty() {
        return Err("plugin_id must not be empty".into());
    }
    if program.manifest.plugin_id.trim().is_empty() {
        return Err("manifest.plugin_id must not be empty".into());
    }
    if program.manifest.version.trim().is_empty() {
        return Err("manifest.version must not be empty".into());
    }
    if program.plugin_id != program.manifest.plugin_id {
        return Err("plugin_id must match manifest.plugin_id".into());
    }
    Ok(())
}

//#region 🔖️BackboneDocument
/// 🧬️ Generic backbone-document envelope — mirrors the dissolved `OsDocument`'s exact shape
/// (schema/id/name/vcs/transitions/outcomes/conflicts/backbone), parametrized over any `<P, Op>` pair
/// `store::os_store::document_materialize::create_document_envelope`/`materialize_document_snapshot`/`print_document_pack`/
/// `parse_document_pack` already support generically — nothing OS-specific left to hardcode. See
/// `## The inversion` in the plan: `OsSnapshot`/`OsMutation`/`OsDocument` dissolve into the three
/// type aliases below instead of one bespoke studio-only document type.
/// 🌱️ `Serialize`/`Deserialize` dropped outright (not dual-derived): every field already
/// carries `ToValue`/`FromValue` (`ArtifactVcs`, `protocol::MutationEnvelope`, `protocol::
/// EditMessages`/`Conflict`, `ArtifactBackboneRef`), and no caller anywhere in this crate
/// requires `BackboneDocument: Serialize` as a trait bound or calls `serde_json` on one directly
/// — confirmed by reading every `serde_json::to_*`/`from_*` call site in this file before
/// removing the derive. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
/// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
/// 📓️os-kernel-serde-final.md` — this was `🌿️vcs`'s own last blocker.
#[derive(Clone, Debug, PartialEq)]
pub struct BackboneDocument<P, Op> {
    pub schema: String,
    pub id: String,
    pub name: String,
    pub vcs: ArtifactVcs<protocol::os_vcs::io::binary::genesis::AdmittedArtifactGenesis<P>, Op>,
    /// 🔀️ The document's history transitions; with `vcs.edits` the complete event log
    /// every position (applied/redo/checkpoint/alternative) is folded from.
    pub transitions: Vec<protocol::MutationEnvelope>,
    pub edit_messages: Vec<protocol::EditMessages>,
    pub conflicts: Vec<protocol::Conflict>,
    pub backbone: Option<ArtifactBackboneRef>,
}

/// 🧬️ `ToValue` twin of the dropped `Serialize` derive — camelCase object shape mirrors the
/// old `#[serde(rename_all = "camelCase")]`; `backbone` is omitted when `None`, matching the
/// old `skip_serializing_if`.
impl<P: ToValue, Op: ToValue> ToValue for BackboneDocument<P, Op> {
    fn to_value(&self) -> DslValue {
        let mut fields = vec![
            ("schema".to_string(), self.schema.to_value()),
            ("id".to_string(), self.id.to_value()),
            ("name".to_string(), self.name.to_value()),
            ("vcs".to_string(), protocol::os_vcs::io::text::vcs::native_vcs_value(&self.vcs)),
            ("transitions".to_string(), self.transitions.to_value()),
            ("editMessages".to_string(), self.edit_messages.to_value()),
            ("conflicts".to_string(), self.conflicts.to_value()),
        ];
        if let Some(backbone) = &self.backbone {
            fields.push(("backbone".to_string(), backbone.to_value()));
        }
        DslValue::Object(fields)
    }
}

/// 🧬️ `FromValue` twin — missing `backbone` decodes to `None`, missing `editMessages`/
/// `conflicts` decode to empty (matching the old `Deserialize`'s implicit `Vec::default()`
/// for an absent, non-`Option` field with no `#[serde(default)]` — those two were never
/// actually optional on the wire in practice, but this preserves exact prior leniency).
impl<P: FromValue + protocol::os_store::ArtifactPack, Op: FromValue> FromValue for BackboneDocument<P, Op> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(fields) = value else {
            return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for BackboneDocument, found {value:?}")));
        };
        let mut schema = None;
        let mut id = None;
        let mut name = None;
        let mut vcs = None;
        let mut transitions = None;
        let mut edit_messages = None;
        let mut conflicts = None;
        let mut backbone = None;
        for (key, entry) in fields {
            match key.as_str() {
                "schema" => schema = Some(String::from_value(entry).map_err(|e| e.under("schema"))?),
                "id" => id = Some(String::from_value(entry).map_err(|e| e.under("id"))?),
                "name" => name = Some(String::from_value(entry).map_err(|e| e.under("name"))?),
                "vcs" => vcs = Some(protocol::os_vcs::io::text::vcs::native_vcs_from_value(entry,&protocol::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).map_err(|e| e.under("vcs"))?),
                "transitions" => transitions = Some(Vec::from_value(entry).map_err(|e| e.under("transitions"))?),
                "editMessages" => edit_messages = Some(Vec::from_value(entry).map_err(|e| e.under("editMessages"))?),
                "conflicts" => conflicts = Some(Vec::from_value(entry).map_err(|e| e.under("conflicts"))?),
                "backbone" => backbone = Some(ArtifactBackboneRef::from_value(entry).map_err(|e| e.under("backbone"))?),
                _ => {}
            }
        }
        Ok(BackboneDocument {
            schema: schema.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing schema"))?,
            id: id.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing id"))?,
            name: name.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing name"))?,
            vcs: vcs.ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "BackboneDocument missing vcs"))?,
            transitions: transitions.unwrap_or_default(),
            edit_messages: edit_messages.unwrap_or_default(),
            conflicts: conflicts.unwrap_or_default(),
            backbone,
        })
    }
}

impl<P, Op: Mutation<P>> BackboneDocument<P, Op> {
    /// 🧮️ The edits the event log leaves applied, in fold order — what a projection replays.
    pub fn applied_edit_ids(&self) -> Result<Vec<String>, VcsError> {
        let head = protocol::ViewerHead::canonical_trunk(&protocol::ArtifactId(self.id.clone()));
        store::os_store::ledger::fold_event_log::<P, Op>(&self.id, &self.vcs.edits.iter().collect::<Vec<_>>(), &self.transitions, &self.conflicts, &head).map(|fold| fold.applied)
    }
}

/// 🏠️ A space's manifest document — the space-catalog half of the dissolved `OsSnapshot`.
pub type OsSpaceDocument = BackboneDocument<space::SpaceSnapshot, space::SpaceMutation>;
/// 🗂️ One collection's folder/entry tree document.
pub type OsCollectionDocument = BackboneDocument<space::CollectionSnapshot, space::CollectionMutation>;
/// 🕸️ One `s.workflow` artifact document — the workflow-graph half of the dissolved `OsSnapshot`
/// (see the kernel `workflow` crate's `WorkflowSnapshot`).
pub type OsWorkflowArtifactDocument = BackboneDocument<workflow::WorkflowSnapshot, workflow::WorkflowMutation>;

/// 🌉️ Live `ArtifactStore` handle for a space-manifest session — no bespoke wrapper needed (unlike
/// `OsWorkflowStore` below, whose only extra logic is workflow-specific node/parameter id-minting);
/// every generic `ArtifactStore` method (`snapshot`/`dispatch`/`attach_backbone`/...) already
/// applies directly. `OsWorkflowStore::add_workflow_node` dispatches into one of these to install
/// the spawned app's plugin into the owning space's `programs` list.
pub type OsSpaceStore = ArtifactStore<space::SpaceSnapshot, space::SpaceMutation>;

/// 🌱️ Mints a fresh backbone document wrapping `initial_snapshot` with empty edit history.
pub fn create_backbone_document<P, Op>(schema: &str, id: &str, name: &str, initial_snapshot: P) -> BackboneDocument<P, Op>
where
    P: Clone + protocol::os_store::ArtifactPack,
    Op: Clone,
{
    BackboneDocument {
        schema: schema.into(),
        id: id.into(),
        name: name.into(),
        vcs: create_document_envelope::<P, Op>(schema, id, initial_snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).into_owners().vcs,
        transitions: Vec::new(),
        edit_messages: Vec::new(),
        conflicts: Vec::new(),
        backbone: None,
    }
}

/// 🌉️ Builds the authoritative `ArtifactEnvelope` a `BackboneDocument` wraps, dropping only
/// the app-level `name` and preserving its complete event log.
fn backbone_envelope_of<P, Op>(document: &BackboneDocument<P, Op>) -> ArtifactEnvelope<P, Op>
where
    P: Clone,
    Op: Clone,
{
    ArtifactEnvelope::from_owners(store::os_store::store_schema::ArtifactEnvelopeOwners {
        schema: document.schema.clone(),
        id: document.id.clone(),
        vcs: document.vcs.clone(),
        backbone: document.backbone.clone(),
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::causal::HistoryFoldIndex::new(),
        viewer_checkpoint_id: None,
        edit_messages: store::os_store::ledger::ArtifactEditMessageLedger::from_preflighted_entries(document.edit_messages.clone()),
        conflicts: document.conflicts.clone(),
        transitions: document.transitions.clone(),
        history_shape: protocol::HistoryShape::Document,
        open_transaction: None,
    })
}

/// 🧺️ Moves every backbone owner into the canonical envelope shell.
fn take_backbone_envelope<P, Op>(document: BackboneDocument<P, Op>) -> (String, ArtifactEnvelope<P, Op>) {
    let envelope = ArtifactEnvelope::from_owners(store::os_store::store_schema::ArtifactEnvelopeOwners {
        schema: document.schema,
        id: document.id,
        vcs: document.vcs,
        backbone: document.backbone,
        active_alternative_id: None,
        cursor: None,
        dialect: None,
        migrated_from: None,
        owner: None,
        lanes: protocol::causal::HistoryFoldIndex::new(),
        viewer_checkpoint_id: None,
        edit_messages: store::os_store::ledger::ArtifactEditMessageLedger::from_preflighted_entries(document.edit_messages),
        conflicts: document.conflicts,
        transitions: document.transitions,
        history_shape: protocol::HistoryShape::Document,
        open_transaction: None,
    });
    (document.name, envelope)
}

/// 🧺️ Retires a copied backbone envelope through its exact bounded owner catalogue.
fn close_backbone_envelope<P, Op>(envelope: ArtifactEnvelope<P, Op>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    use store::ErasedSnapshotRetirement as _;
    fn grant_of(demand: semio_framework_value::RetirementDemand) -> semio_framework_value::retained_clone::RetainedCloneGrant {
        semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) }
    }
    let fault = |error: ValueError| VcsError::ValidationFailed(error.to_string());
    let owners = store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<P, Op>().map_err(fault)?;
    let birth = grant_of(owners.uninstalled_envelope_retirement_demands(&envelope));
    let (mut retirement, _) = owners.retire_envelope_uninstalled(envelope, birth).map_err(|(error, _, _)| fault(error))?;
    loop {
        let demand = retirement.next_demand(4096).map_err(fault)?;
        match retirement.close_step(grant_of(demand)).map_err(fault)? {
            semio_framework_value::retained_clone::RetainedCloneStep::Complete(_) => {
                assert!(retirement.terminal_is_empty(), "backbone envelope retirement reports its exact terminal");
                return Ok(());
            }
            semio_framework_value::retained_clone::RetainedCloneStep::Progress(_) => std::thread::yield_now(),
        }
    }
}

/// 🧺️ Lends a copied authoritative envelope, then retires every exact nested owner.
fn with_backbone_envelope<P, Op, R>(document: &BackboneDocument<P, Op>, read: impl FnOnce(&ArtifactEnvelope<P, Op>) -> Result<R, VcsError>) -> Result<R, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    let envelope = backbone_envelope_of(document);
    let result = read(&envelope);
    close_backbone_envelope(envelope)?;
    result
}

pub fn materialize_backbone_snapshot<P, Op>(document: &BackboneDocument<P, Op>, applied_edit_ids: &[String]) -> Result<P, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    with_backbone_envelope(document, |envelope| resolve_kernel_future(materialize_document_snapshot(envelope, applied_edit_ids)))
}

/// 📤️ Exports an already-loaded backbone document as pack bytes + ops text.
pub fn export_backbone_pack<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::os_store::serialization::ArtifactPackFiles, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    with_backbone_envelope(document, |envelope| resolve_kernel_future(store::os_store::text_format::print_document_pack(envelope)))
}

/// 📤️ DSL-text counterpart of `export_backbone_pack`.
pub fn export_backbone_dsl<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::os_store::text_format::ArtifactTextFiles, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static + pack::snapshot::ArtifactDsl,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    with_backbone_envelope(document, |envelope| resolve_kernel_future(store::os_store::text_format::print_document_text(envelope)))
}

/// 📦️ Binary pack+spr payload for the whole `BackboneDocument` (name + genesis + event log)
/// — the persisted/synced form. `name` rides as a `store::os_store::serialization::encode_document_pack_bytes`-framed blob
/// wrapping a nested `pack`+`spr` pair; `spr` carries the events, whose fold restores the exact
/// undo/redo/checkpoint position.
pub fn encode_backbone_payload<P, Op>(document: &BackboneDocument<P, Op>) -> Result<Vec<u8>, VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    let files = with_backbone_envelope(document, |envelope| resolve_kernel_future(store::os_store::text_format::print_document_pack(envelope)))?;
    let inner = resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(&files.pack, &files.spr));
    Ok(resolve_kernel_future(store::os_store::serialization::encode_document_pack_bytes(document.name.as_bytes(), &inner)))
}

/// 📥️ Inverse of `encode_backbone_payload` — `expected_schema` guards against decoding one
/// document kind's bytes as another.
pub fn decode_backbone_payload<P, Op>(bytes: &[u8], expected_schema: &str) -> Result<BackboneDocument<P, Op>, VcsError>
where
    P: Clone + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
    Op: Clone + protocol::OpText + protocol::OpBinary + Mutation<P> + 'static,
{
    let (name_bytes, inner) = resolve_kernel_future(store::os_store::serialization::decode_document_pack_bytes(bytes))?;
    let name = String::from_utf8(name_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let (pack, spr) = resolve_kernel_future(store::os_store::serialization::decode_document_pack_bytes(&inner))?;
    let parsed: store::os_store::text_format::ParsedDocumentText<P, Op> = resolve_kernel_future(store::os_store::text_format::parse_document_pack(&pack, &spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let envelope = parsed.into_envelope();
    if envelope.schema != expected_schema {
        envelope.retire_unadopted();
        return Err(VcsError::Deserialize(format!("expected schema {expected_schema}")));
    }
    // 🧺️ Consume the shell: its `Drop` asserts the owners were detached, so reading fields off it
    // and letting it fall out of scope aborts the guest at runtime.
    let mut owners = envelope.into_owners();
    let mut edit_messages = Vec::with_capacity(owners.edit_messages.len());
    while let Some(entry) = owners.edit_messages.pop() { edit_messages.push(entry); }
    edit_messages.reverse();
    Ok(BackboneDocument { schema: owners.schema, id: owners.id, name, vcs: owners.vcs, transitions: owners.transitions, edit_messages, conflicts: owners.conflicts, backbone: owners.backbone })
}
//#endregion 🔖️BackboneDocument

//#region 🔖️GraphReconcile
/// 🧵️ Post-materialization workflow integrity pass, invoked explicitly by
/// `OsWorkflowStore::snapshot_with_conflicts` (NOT through `Mutation::reconcile` — the kernel
/// `workflow::WorkflowMutation` inherits that trait hook's no-op default, since the two rules that
/// used to run alongside these purely-structural ones need the os-core plugin/artifact registry the
/// kernel crate doesn't have; see `workflow::WorkflowMutation`'s own doc). Runs, in order: (1) drop
/// edges whose source/target node or port no longer exists (a concurrent delete tombstone wins over
/// the wiring), (2) drop edges whose port types no longer match (a concurrent re-typing wins over
/// the wiring), (3) dedupe edges with identical endpoints down to the lexicographically smallest id
/// (deterministic across peers replaying the same operation log), (4) break any cycle the previous
/// rules left behind, (5) drop parameter bindings whose target config field or parameter type no
/// longer validates. Each rule operates on the edge/binding set the previous one produced. Reports
/// through `protocol::MutationMessage` (the old free-form diagnostic struct's replacement,
/// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C10), always at
/// `Warning`: every rule here is corrective (the drop always happens, regardless of this
/// message's level — unlike a `Mutation::diff` outcome, `Fatal`/`Error`'s "no change applied" laws
/// don't bind a post-merge reconcile pass), so `Warning` is the level that means "the state
/// changed to survive a conflict, and the change is worth surfacing" rather than "rejected"
/// (`Error`/`Fatal`) or "a routine side effect of an otherwise-successful op" (`Info`).
fn reconcile_workflow_snapshot(mut document: workflow::WorkflowSnapshot) -> (workflow::WorkflowSnapshot, Vec<protocol::MutationMessage>) {
    let mut conflicts = Vec::new();
    let mut edges = std::mem::take(&mut document.graph.edges);
    let node_by_id: HashMap<&str, &workflow::WorkflowNode> = document.graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();

    //#region OrphanEdgeDrop
    edges.retain(|edge| {
        let source_ok = node_by_id.get(edge.source_node_id.as_str()).is_some_and(|node| node.outputs.iter().any(|port| port.id == edge.source_port_id));
        let target_ok = node_by_id.get(edge.target_node_id.as_str()).is_some_and(|node| node.inputs.iter().any(|port| port.id == edge.target_port_id));
        if source_ok && target_ok {
            true
        } else {
            conflicts.push(protocol::MutationMessage {
                level: semio_framework_diagnostic::Severity::Warning,
                code: semio_framework_diagnostic::FaultCode::new("workflow/edge-orphaned"),
                message: format!("edge {} references a node or port that no longer exists ({}:{} -> {}:{})", edge.id, edge.source_node_id, edge.source_port_id, edge.target_node_id, edge.target_port_id),
                target: vec![edge.id.clone()],
                op_index: None,
            });
            false
        }
    });
    //#endregion OrphanEdgeDrop

    //#region TypeMismatchDrop
    edges.retain(|edge| {
        let Some(source_port) = node_by_id.get(edge.source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id)) else {
            return false;
        };
        let Some(target_port) = node_by_id.get(edge.target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id)) else {
            return false;
        };
        match workflow::negotiate_media_contract(source_port, target_port) {
            Ok(contract) if contract == edge.contract => true,
            Ok(_) => {
                conflicts.push(protocol::MutationMessage {
                    level: semio_framework_diagnostic::Severity::Warning,
                    code: semio_framework_diagnostic::FaultCode::new("workflow/edge-type-mismatch"),
                    message: format!("edge {} contract stale: no longer matches negotiated port types", edge.id),
                    target: vec![edge.id.clone()],
                    op_index: None,
                });
                false
            }
            Err(reason) => {
                conflicts.push(protocol::MutationMessage {
                    level: semio_framework_diagnostic::Severity::Warning,
                    code: semio_framework_diagnostic::FaultCode::new("workflow/edge-type-mismatch"),
                    message: format!("edge {} connects ports whose types no longer match: {reason}", edge.id),
                    target: vec![edge.id.clone()],
                    op_index: None,
                });
                false
            }
        }
    });
    //#endregion TypeMismatchDrop

    //#region DuplicateWireDedupe
    let mut smallest_id_for_wire: HashMap<(String, String, String, String), String> = HashMap::new();
    for edge in &edges {
        let wire = (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
        smallest_id_for_wire
            .entry(wire)
            .and_modify(|smallest| {
                if edge.id < *smallest {
                    *smallest = edge.id.clone();
                }
            })
            .or_insert_with(|| edge.id.clone());
    }
    edges.retain(|edge| {
        let wire = (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
        smallest_id_for_wire.get(&wire).is_some_and(|smallest| smallest == &edge.id)
    });
    //#endregion DuplicateWireDedupe

    //#region CycleDrop
    edges = drop_workflow_cycle_edges(edges, &mut conflicts);
    //#endregion CycleDrop

    //#region ParameterBindingValidation
    // 🛡️ A binding's target `ConfigSpec` field (a concurrent app swap/downgrade removed or
    // re-shaped the field) or the bound parameter's own type (a concurrent `PatchParameter`
    // re-typed it) can go stale exactly like an edge's contract can — same defense-in-depth spirit
    // as `TypeMismatchDrop`, ported from os-core's dissolved `OsSnapshot`-based reconcile.
    let parameters = document.parameters.clone();
    let nodes = document.graph.nodes.clone();
    document.parameter_bindings.retain(|binding| {
        let Some(node) = nodes.iter().find(|node| node.id == binding.node_id) else { return true };
        let Some(registration) = os_app_registration(&node.plugin_id, &node.app_id) else { return true };
        let Some(parameter_type) = parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == binding.parameter_id).map(workflow_parameter_type_of) else { return true };
        match resolve_kernel_future(workflow::validate_workflow_parameter_config_binding(binding, &parameter_type, &registration.config)) {
            Ok(()) => true,
            Err(conflict) => {
                conflicts.push(conflict);
                false
            }
        }
    });
    document.graph = workflow::sync_workflow_parameter_ports(&document.graph, &document.parameter_bindings);
    //#endregion ParameterBindingValidation

    document.graph.edges = edges;
    (document, conflicts)
}

/// 🎛️ Maps a `workflow::WorkflowParameter` to its `WorkflowParameterType` tag — needed here
/// to type-check a binding's parameter against its target config field's `ArgSchema`.
fn workflow_parameter_type_of(parameter: &workflow::WorkflowParameter) -> workflow::WorkflowParameterType {
    match parameter {
        workflow::WorkflowParameter::Numeric { .. } => workflow::WorkflowParameterType::Numeric,
        workflow::WorkflowParameter::Categorical { .. } => workflow::WorkflowParameterType::Categorical,
        workflow::WorkflowParameter::Toggle { .. } => workflow::WorkflowParameterType::Toggle,
        workflow::WorkflowParameter::Text { .. } => workflow::WorkflowParameterType::Text,
    }
}

/// 🌀️ Repeatedly finds a cycle in `edges` (by node-id adjacency) and drops the participating
/// edge with the highest array index — a deterministic proxy for "newest edit" since
/// `reconcile_workflow_snapshot` only receives the materialized `WorkflowSnapshot` by value, not
/// per-edge `HybridLogicalTimestamp`s from the edit log. `apply_workflow_operation`'s `ConnectPorts`
/// handler appends new edges to the end of the vec, so a higher index approximates a later edit;
/// true HLT-based tie-breaking would need this pass to also see edit history, not just the document.
fn drop_workflow_cycle_edges(mut edges: Vec<workflow::WorkflowEdge>, conflicts: &mut Vec<protocol::MutationMessage>) -> Vec<workflow::WorkflowEdge> {
    while let Some(cycle_node_ids) = find_workflow_cycle_participants(&edges) {
        let newest_cycle_edge_index = edges.iter().enumerate().filter(|(_, edge)| cycle_node_ids.contains(&edge.source_node_id) && cycle_node_ids.contains(&edge.target_node_id)).map(|(index, _)| index).max();
        let Some(newest_cycle_edge_index) = newest_cycle_edge_index else { break };
        let dropped = edges.remove(newest_cycle_edge_index);
        conflicts.push(protocol::MutationMessage {
            level: semio_framework_diagnostic::Severity::Warning,
            code: semio_framework_diagnostic::FaultCode::new("workflow/edge-cycle"),
            message: format!("edge {} was dropped to break a cycle in the workflow", dropped.id),
            target: vec![dropped.id.clone()],
            op_index: None,
        });
    }
    edges
}

/// 🔍️ DFS cycle detection adapted from `workflow::validate_workflow`'s check, but returning
/// the participant node ids of the first cycle found (rather than just an error string) so the
/// caller can identify which edges are eligible for dropping.
fn find_workflow_cycle_participants(edges: &[workflow::WorkflowEdge]) -> Option<HashSet<String>> {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut node_ids: HashSet<String> = HashSet::new();
    for edge in edges {
        node_ids.insert(edge.source_node_id.clone());
        node_ids.insert(edge.target_node_id.clone());
        adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut visited = HashSet::new();
    for node_id in &node_ids {
        if visited.contains(node_id) {
            continue;
        }
        let mut stack: Vec<String> = Vec::new();
        let mut on_stack: HashSet<String> = HashSet::new();
        if let Some(cycle) = dfs_find_workflow_cycle(node_id, &adjacency, &mut stack, &mut on_stack, &mut visited) {
            return Some(cycle);
        }
    }
    None
}

fn dfs_find_workflow_cycle(node_id: &str, adjacency: &HashMap<String, Vec<String>>, stack: &mut Vec<String>, on_stack: &mut HashSet<String>, visited: &mut HashSet<String>) -> Option<HashSet<String>> {
    if on_stack.contains(node_id) {
        let start = stack.iter().position(|id| id == node_id).unwrap_or(0);
        return Some(stack[start..].iter().cloned().collect());
    }
    if visited.contains(node_id) {
        return None;
    }
    stack.push(node_id.to_string());
    on_stack.insert(node_id.to_string());
    for next in adjacency.get(node_id).into_iter().flatten() {
        if let Some(cycle) = dfs_find_workflow_cycle(next, adjacency, stack, on_stack, visited) {
            return Some(cycle);
        }
    }
    stack.pop();
    on_stack.remove(node_id);
    visited.insert(node_id.to_string());
    None
}
//#endregion 🔖️GraphReconcile

//#region 🔖️OsWorkflowStore
pub struct OsWorkflowStore {
    inner: ArtifactStore<workflow::WorkflowSnapshot, workflow::WorkflowMutation>,
    name: String,
}

impl OsWorkflowStore {
    pub fn new(document: OsWorkflowArtifactDocument, actor: protocol::ActorId) -> Result<Self, VcsError> {
        let (name, envelope) = take_backbone_envelope(document);
        let mut inner = resolve_kernel_future(ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor))?;
        inner.install_document_store_owners_exact(store::os_store::document_store_owners::funded_bounded_artifact_store_owners().map_err(|error| VcsError::ValidationFailed(error.to_string()))?).map_err(|(error, owners)| { std::mem::forget(owners); VcsError::ValidationFailed(error.to_string()) })?;
        Ok(Self { inner, name })
    }

    pub fn generation(&self) -> u64 {
        self.inner.generation()
    }

    pub fn snapshot(&self) -> Result<workflow::WorkflowSnapshot, VcsError> {
        self.inner.snapshot()
    }

    /// 🤝️ Fresh replay plus `reconcile_workflow_snapshot`'s whole 4(+1)-rule pipeline —
    /// invoked explicitly here rather than through `Mutation::reconcile` (a no-op default at the
    /// kernel-crate layer, since two of those rules need the os-core plugin/artifact registry).
    pub fn snapshot_with_conflicts(&self) -> Result<(workflow::WorkflowSnapshot, Vec<protocol::MutationMessage>), VcsError> {
        let document = self.inner.snapshot()?;
        Ok(reconcile_workflow_snapshot(document))
    }

    pub fn document(&self) -> OsWorkflowArtifactDocument {
        let envelope = self.inner.envelope();
        BackboneDocument {
            schema: envelope.schema.clone(),
            id: envelope.id.clone(),
            name: self.name.clone(),
            vcs: envelope.vcs.clone(),
            transitions: envelope.transitions.clone(),
            edit_messages: envelope.edit_messages.iter().cloned().collect(),
            conflicts: envelope.conflicts.clone(),
            backbone: envelope.backbone.clone(),
        }
    }

    pub fn dispatch_text(&mut self, command_text: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.dispatch_text(command_text, identity)).map(|_| ())
    }

    pub fn dispatch_binary(&mut self, command_bytes: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.dispatch_binary(command_bytes, identity)).map(|_| ())
    }

    pub fn dispatch_apply(&mut self, mutations: Vec<workflow::WorkflowMutation>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity)).map(|_| ())
    }

    pub fn set_workflow_name(&mut self, name: &str) {
        self.name = name.into();
        let _ = self.inner.generation();
    }

    /// 🆔️ Mints a fresh `WorkflowNode` (id, ports, document/config refs — everything) via
    /// `workflow::workflow_node_for_app`, at dispatch time, so replay never re-derives it. Also
    /// dispatches `space::SpaceMutation::InstallProgram` against `space_store` — the owning space's
    /// `programs` list moved off the dissolved `OsSnapshot` onto `space::SpaceSnapshot` (see
    /// `## The inversion` in the plan), so spawning a node into the workflow graph and installing its
    /// plugin into the space are now two operations against two separate documents.
    pub fn add_workflow_node(&mut self, plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64, space_store: &mut OsSpaceStore, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let app = resolve_os_app_definition(plugin_id, app_id).ok_or_else(|| VcsError::Deserialize(format!("unknown app {plugin_id}/{app_id}")))?;
        let node_id = create_os_id("node");
        let position = workflow::WorkflowPosition { x, y, width: 0.0, height: 0.0 };
        let mut node = resolve_kernel_future(workflow::workflow_node_for_app(&app, plugin_id, &node_id, &position));
        if let Some(label) = label {
            node.label = label.into();
        }
        self.dispatch_apply(vec![workflow::WorkflowMutation::AddNode(workflow::AddNode { node })], identity)?;
        resolve_kernel_future(space_store.dispatch(ArtifactCommand::Apply { mutations: vec![space::SpaceMutation::InstallProgram { plugin_id: plugin_id.into(), index: None }], transaction: None }, identity))?;
        Ok(node_id)
    }

    pub fn add_parameter(&mut self, parameter_type: &workflow::WorkflowParameterType, name: &str, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let parameter = resolve_kernel_future(workflow::create_default_workflow_parameter(parameter_type, name, None));
        let parameter_id_value = workflow::workflow_parameter_id(&parameter).to_string();
        self.dispatch_apply(vec![workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(parameter) })], identity)?;
        Ok(parameter_id_value)
    }

    pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &workflow::WorkflowParameterPatch, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let document = self.snapshot()?;
        let current = document.parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == target_parameter_id).cloned().ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
        let next = resolve_kernel_future(workflow::patch_workflow_parameter(&current, patch));
        self.dispatch_apply(vec![workflow::WorkflowMutation::ChangeParameter(workflow::ChangeParameter { parameter_id: target_parameter_id.into(), parameter: Box::new(next) })], identity)
    }

    /// 📡️ Pumps any queued inbound backbone messages into the edit timeline.
    pub fn tick(&mut self) -> Result<bool, VcsError> {
        resolve_kernel_future(self.inner.tick())
    }

    /// 🔗️ Resolves and attaches a backbone by uri. Only available inside the wasm sandbox
    /// (every scheme forwards to the host over the injected `BackboneChannelPort`, a pure queue) —
    /// see {@link attach_backbone} for the native counterpart, which takes an explicit
    /// `Box<dyn store::os_store::backbone::Backbone>` since native has no URI→IO auto-resolution anymore (`framework/sync`'s
    /// `ArtifactHost` module owns constructing the real endpoint via `ArtifactHost`).
    #[cfg(target_arch = "wasm32")]
    pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.attach_backbone_uri(uri))
    }

    /// 🔗️ Attaches an explicit native backbone channel (typically a `channel_backbone` handed
    /// out by `framework/sync`'s `ArtifactHost::open`, per `ArtifactHost`'s canonical sequence).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn attach_backbone(&mut self, backbone: store::os_store::backbone::Backbones) -> Result<(), VcsError> {
        resolve_kernel_future(self.inner.attach_backbone(backbone))
    }

    pub fn detach_backbone(&mut self) -> Result<Option<store::os_store::backbone::Backbones>, VcsError> {
        self.inner.detach_backbone()
    }

    pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.inner.backbone_ref()
    }
}
//#endregion 🔖️OsWorkflowStore

//#region 🔖️Backbone
/// 🔌️ Byte-oriented studio persistence port — `read`/`write` carry `encode_backbone_payload`'s
/// binary pack+spr blob, never JSON. Every implementor today (`MemoryBackbonePort`,
/// `LocalStorageBackbonePort`, and the `file://`/`folder://` host ports opened by
/// `open_file_space_backbone`/`open_folder_space_backbone`) is bridged from the underlying
/// string-typed `store::os_store::backbone::BackbonePort` via the blanket impl below — see its doc for why.
pub trait OsBackbonePort: Send + Sync {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError>;
    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError>;
}

/// 🌉️ `store::os_store::backbone::BackbonePort` is the shared string-typed transport for every document kind
/// across the whole kernel (localStorage, in-memory, host file/folder ports) — changing its own
/// signature to bytes is out of scope here. Base64 is the bridge: an empty payload maps to an
/// empty string both ways (preserving `delete_os_space`'s tombstone-write semantics), and every
/// non-empty payload round-trips byte-for-byte through the encoding.
impl<T: store::os_store::backbone::BackbonePort + Send + Sync> OsBackbonePort for T {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        let text = resolve_kernel_future(store::os_store::backbone::BackbonePort::read(self, uri))?;
        if text.is_empty() {
            return Ok(Vec::new());
        }
        base64_codec::base64_standard_decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
        if payload.is_empty() {
            return resolve_kernel_future(store::os_store::backbone::BackbonePort::write(self, uri, ""));
        }
        resolve_kernel_future(store::os_store::backbone::BackbonePort::write(self, uri, &base64_codec::base64_standard_encode(payload)))
    }
}

/// 🧬️ Enum dispatch over every `OsBackbonePort` implementor (O1 — dyn dispatch dropped in
/// favor of enum/match dispatch, mirroring `🏪️store`'s `BackbonePorts` shape exactly —
/// `📓️terra-store-dedyn-report.md`). `Store` covers every `store::os_store::backbone::BackbonePort`-shaped transport
/// (in-memory, localStorage) via the blanket bridge above; `Space` covers the host-only
/// file/folder-backed `backbone::SpaceBackbonePort`. These are the only two implementors in the
/// tree (verified: one blanket impl over `store::os_store::backbone::BackbonePort`, one direct impl on
/// `backbone::SpaceBackbonePort` — `📓️terra-os-backbone-report.md`).
pub enum OsBackbonePorts {
    Store(store::os_store::backbone::BackbonePorts),
    Space(crate::backbone::SpaceBackbonePort),
}

impl OsBackbonePort for OsBackbonePorts {
    fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
        match self {
            Self::Store(port) => OsBackbonePort::read(port, uri),
            Self::Space(port) => OsBackbonePort::read(port, uri),
        }
    }

    fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
        match self {
            Self::Store(port) => OsBackbonePort::write(port, uri, payload),
            Self::Space(port) => OsBackbonePort::write(port, uri, payload),
        }
    }
}

/// 🌉️ Writes any `BackboneDocument<P, Op>` to `uri`, stamping its own `backbone` ref first —
/// shared by every catalog write path below (space manifests, collections).
fn sync_backbone_document<P, Op>(document: &BackboneDocument<P, Op>, backbone_uri: &str, port: &Arc<OsBackbonePorts>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + pack::snapshot::ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Send + Sync + 'static,
    Op: Clone + ToValue + FromValue + Mutation<P> + protocol::OpText + protocol::OpBinary + semio_framework_value::retirement::RetireOwned + Send + store::ArtifactCanonicalJsonTree + 'static,
{
    let mut synced = document.clone();
    synced.backbone = Some(resolve_kernel_future(document_backbone_ref(backbone_uri)));
    let result = encode_backbone_payload(&synced).and_then(|payload| port.write(backbone_uri, &payload));
    close_backbone_envelope(take_backbone_envelope(synced).1)?;
    result
}
//#endregion 🔖️Backbone

// 🫀️ Presence used to be a `presence:` backbone-URI polling hack (`OS_PRESENCE_URI_PREFIX` /
// `write_os_presence` / `read_os_presence_peers`) — deleted. Presence now flows through the semio_hub's
// duplex `PresencePeer`/`HubServerFrame::Presence` frames (`framework/core/rs`'s 🔖️HubProtocol
// region) via `framework/sync`'s `ArtifactHost::subscribe` yielding `ArtifactEvent::Presence`. 🎯️
// ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C8.4: the old
// `presence_peers_json`/`ViewModel.presence_peers_json` JSON-array bridge is DELETED — plugins
// now receive the roster through the object-safe app trait's `adopt_presence` (§C7.6, the ONLY
// plugin ingress for peers) via `AppCommand::Presence`, not a `ViewModel` field a native host
// pre-translates.

//#region 🔖️SpaceCatalog
pub const OS_HOME_VFS_ROOT_ID: &str = "os-home-root";
pub const OS_SPACE_BACKBONE_URI_PREFIX: &str = "space://";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsSpaceCatalogEntry {
    pub id: String,
    pub name: String,
    pub backbone_uri: String,
    pub kind: space::SpaceKind,
    pub visibility: space::SpaceVisibility,
    pub collection_count: usize,
    pub updated_at: String,
}

static SPACE_CATALOG_URIS: LazyLock<Mutex<HashMap<usize, HashSet<String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

fn port_key(port: &Arc<OsBackbonePorts>) -> usize {
    Arc::as_ptr(port) as *const () as usize
}

fn track_os_space_backbone_uri(port: &Arc<OsBackbonePorts>, uri: &str) {
    SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(port_key(port)).or_default().insert(uri.into());
}

fn untrack_os_space_backbone_uri(port: &Arc<OsBackbonePorts>, uri: &str) {
    if let Some(uris) = SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get_mut(&port_key(port)) {
        uris.remove(uri);
    }
}

/// 🔎️ Extracts a bare space id from a `space://<id>` uri — deliberately excludes the
/// `space://<id>/collection/<cid>` form (`space::collection_backbone_uri`), which the catalog
/// tracks alongside a space's manifest uri but must not surface as a catalog row of its own.
fn os_space_id_from_backbone_uri(uri: &str) -> Option<String> {
    let id = uri.strip_prefix(OS_SPACE_BACKBONE_URI_PREFIX)?;
    if id.is_empty() || id.contains('/') {
        return None;
    }
    Some(id.to_string())
}

fn os_space_catalog_entry_from_document(backbone_uri: &str, document: &OsSpaceDocument) -> Result<OsSpaceCatalogEntry, VcsError> {
    let space_id = os_space_id_from_backbone_uri(backbone_uri).unwrap_or_else(|| document.id.clone());
    let snapshot = materialize_backbone_snapshot(document, &[])?;
    let updated_at = document.vcs.changes.last().map_or_else(|| "0".into(), |change| change.saved_at.clone());
    Ok(OsSpaceCatalogEntry { id: space_id, name: document.name.clone(), backbone_uri: backbone_uri.into(), kind: snapshot.kind, visibility: snapshot.visibility, collection_count: snapshot.collections.len(), updated_at })
}

/// 📚️ Lists persisted space manifests from the dev backbone namespace.
pub fn list_os_space_catalog_entries(port: &Arc<OsBackbonePorts>) -> Result<Vec<OsSpaceCatalogEntry>, VcsError> {
    let mut entries = Vec::new();
    let uris: Vec<String> = SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&port_key(port)).cloned().unwrap_or_default().into_iter().collect();
    for uri in uris {
        if os_space_id_from_backbone_uri(&uri).is_none() {
            continue;
        }
        let Ok(payload) = port.read(&uri) else {
            continue;
        };
        if payload.is_empty() {
            continue;
        }
        let document: OsSpaceDocument = decode_backbone_payload(&payload, space::S_SPACE_SCHEMA)?;
        entries.push(os_space_catalog_entry_from_document(&uri, &document)?);
    }
    entries.sort_by(|left, right| right.updated_at.cmp(&left.updated_at).then_with(|| left.name.cmp(&right.name)));
    Ok(entries)
}

/// 🆕️ Creates a space manifest PLUS one default "main" collection on the dev backbone — see
/// `## The inversion`/`Addressing` in the plan: a space no longer auto-creates a workflow artifact
/// (that's an explicit, later user action), only the collection every space needs to hold artifacts
/// in the first place.
pub fn create_os_space(name: &str, kind: space::SpaceKind, visibility: space::SpaceVisibility, owner: space::SpaceUser, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let space_id = create_os_id("space");
    let collection_id = create_os_id("collection");
    let mut space_snapshot = space::empty_space_snapshot(name.trim(), kind, visibility);
    space_snapshot.users.push(owner);
    space_snapshot.collections.push(space::CollectionRef { id: collection_id.clone(), name: "main".into(), document_id: collection_id.clone() });
    let space_document: OsSpaceDocument = create_backbone_document(space::S_SPACE_SCHEMA, &space_id, name.trim(), space_snapshot);
    let collection_document: OsCollectionDocument = create_backbone_document(space::S_COLLECTION_SCHEMA, &collection_id, "main", space::empty_collection_snapshot("main"));

    let space_uri = space::space_backbone_uri(&space_id);
    let collection_uri = space::collection_backbone_uri(&space_id, &collection_id);
    sync_backbone_document(&space_document, &space_uri, port)?;
    sync_backbone_document(&collection_document, &collection_uri, port)?;
    track_os_space_backbone_uri(port, &space_uri);
    track_os_space_backbone_uri(port, &collection_uri);
    os_space_catalog_entry_from_document(&space_uri, &space_document)
}

/// 🗑️ Deletes a space manifest and every collection it references from the dev backbone.
pub fn delete_os_space(space_id: &str, port: &Arc<OsBackbonePorts>) -> Result<(), VcsError> {
    let uri = space::space_backbone_uri(space_id);
    if let Ok(document) = load_os_space_document(space_id, port) {
        if let Ok(snapshot) = materialize_backbone_snapshot(&document, &[]) {
            for collection in &snapshot.collections {
                let collection_uri = space::collection_backbone_uri(space_id, &collection.id);
                untrack_os_space_backbone_uri(port, &collection_uri);
                port.write(&collection_uri, &[])?;
            }
        }
    }
    untrack_os_space_backbone_uri(port, &uri);
    port.write(&uri, &[])
}

/// 🌉️ Admits one space document into the catalog of `port` under its own id and name — the shared tail of
/// `import_os_space_from_dsl`/`import_os_space_from_pack` and of a studio newly bound to a file or folder backbone:
/// mints a fresh id when the source carried none, syncs, and tracks the catalog uri.
pub fn admit_os_space_document(mut document: OsSpaceDocument, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let space_id = if document.id.is_empty() { create_os_id("space") } else { document.id.clone() };
    let backbone_uri = space::space_backbone_uri(&space_id);
    document.id = space_id;
    sync_backbone_document(&document, &backbone_uri, port)?;
    track_os_space_backbone_uri(port, &backbone_uri);
    os_space_catalog_entry_from_document(&backbone_uri, &document)
}

/// 📥️ Imports a space manifest dsl text (`export_os_space_dsl`'s counterpart) onto the dev
/// backbone under the manifest's own name (the catalog lists a studio by its document name). Does not create a collection — a manifest imported this way is expected to already
/// reference its own collections (a fresh, collection-less space only comes from `create_os_space`).
pub fn import_os_space_from_dsl(dsl: &str, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let snapshot = <space::SpaceSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(dsl).map_err(|error| VcsError::Deserialize(error.message))?;
    let name = snapshot.name.trim().to_owned();
    admit_os_space_document(create_backbone_document(space::S_SPACE_SCHEMA, "", &name, snapshot), port)
}

/// 📦️ Pack counterpart of `import_os_space_from_dsl`: admits the pair under its own document id and the head
/// manifest's name (the catalog lists a studio by its document name).
pub fn import_os_space_from_pack(pack: &[u8], spr: &[u8], port: &Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
    let parsed: store::os_store::text_format::ParsedDocumentText<space::SpaceSnapshot, space::SpaceMutation> = resolve_kernel_future(store::os_store::text_format::parse_document_pack(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let name = parsed.snapshot.name.trim().to_owned();
    // 🧺️ See `decode_backbone_payload` above — the shell must be consumed, never dropped.
    let mut owners = parsed.into_envelope().into_owners();
    let mut edit_messages = Vec::with_capacity(owners.edit_messages.len());
    while let Some(entry) = owners.edit_messages.pop() { edit_messages.push(entry); }
    edit_messages.reverse();
    let document = BackboneDocument {
        schema: owners.schema,
        id: owners.id,
        name,
        vcs: owners.vcs,
        transitions: owners.transitions,
        edit_messages,
        conflicts: owners.conflicts,
        backbone: owners.backbone,
    };
    admit_os_space_document(document, port)
}

/// 📤️ Exports an already-loaded space manifest as pack bytes + ops text.
pub fn export_os_space_pack(document: &OsSpaceDocument) -> Result<store::os_store::serialization::ArtifactPackFiles, VcsError> {
    export_backbone_pack(document)
}

/// 📤️ DSL-text counterpart of `export_os_space_pack`.
pub fn export_os_space_dsl(document: &OsSpaceDocument) -> Result<store::os_store::text_format::ArtifactTextFiles, VcsError> {
    export_backbone_dsl(document)
}

/// 📂️ Loads a space manifest from the dev backbone.
pub fn load_os_space_document(space_id: &str, port: &Arc<OsBackbonePorts>) -> Result<OsSpaceDocument, VcsError> {
    let backbone_uri = space::space_backbone_uri(space_id);
    let payload = port.read(&backbone_uri)?;
    if payload.is_empty() {
        return Err(VcsError::Backbone(format!("unknown os space: {space_id}")));
    }
    decode_backbone_payload(&payload, space::S_SPACE_SCHEMA)
}

/// 🌱️ Seeds the demo space when the catalog is empty.
pub fn seed_os_space_catalog_if_empty(seed_document: OsSpaceDocument, port: &Arc<OsBackbonePorts>) -> Result<Option<OsSpaceCatalogEntry>, VcsError> {
    if !list_os_space_catalog_entries(port)?.is_empty() {
        return Ok(None);
    }
    let space_id = if seed_document.id.is_empty() { "default".into() } else { seed_document.id.clone() };
    let backbone_uri = space::space_backbone_uri(&space_id);
    let mut seeded = seed_document;
    seeded.id = space_id;
    sync_backbone_document(&seeded, &backbone_uri, port)?;
    track_os_space_backbone_uri(port, &backbone_uri);
    Ok(Some(os_space_catalog_entry_from_document(&backbone_uri, &seeded)?))
}
//#endregion 🔖️SpaceCatalog

//#region 🔖️ExtensionInstall
/// 🧩️ One verified, installed `.sxt` extension package — `manifest` is the byte-for-byte
/// unpacked `store::extension::ExtensionPackageManifest` (contract freeze §4 already re-checked
/// by `install_extension_package` below, not merely trusted from `verify`); `content_hash` is
/// the same blake3 digest `store::extension::content_hash` computes over the full `.sxt` bytes,
/// kept alongside so a re-install of byte-identical bytes is detectable without re-hashing.
///
/// Deliberately the `.sxt`-shaped twin of `kernel::ExtensionDescriptor`
/// (`🧰️framework/🔨️modules/🎠️kernel`, this ticket's other owned path) rather than a shared type:
/// this crate has no mount of that file, and `semio-framework-os-kernel` must never depend on
/// `semio-framework` (the same dependency-edge-law reason `PackagePluginDependency`'s own
/// docstring in `🧩️extension/🦀️.rs` gives for ITS wire-shape duplication) — so the two
/// shapes are duplicated on purpose, exactly like that established precedent.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Clone, Debug, PartialEq)]
pub struct InstalledExtension {
    pub manifest: store::extension::ExtensionPackageManifest,
    pub content_hash: String,
}

/// ⚠️ Failures installing an `.sxt` extension package. `Package` bubbles `store::extension`'s own
/// verify/unpack errors; `ExtendsMismatch` re-checks contract freeze §4
/// (`extends == dependencies[0].plugin_id`) at INSTALL time rather than only trusting the guest
/// SDK-side `assert!` (`ExtensionBundle::assert_extends_matches_primary_dependency`, which only
/// fires when the extension is BUILT) — a hand-crafted or corrupted `.sxt` must still be rejected
/// here, at the one place every runtime-installed extension actually passes through.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
#[derive(Debug)]
pub enum ExtensionInstallError {
    Package(store::extension::ExtensionPackageError),
    ExtendsMismatch { extension_id: String, extends: String, actual: String },
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::fmt::Display for ExtensionInstallError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Package(error) => write!(formatter, "extension package: {error}"),
            Self::ExtendsMismatch { extension_id, extends, actual } => {
                write!(formatter, "extension {extension_id:?} declares extends={extends:?} but its first dependency is {actual:?} — contract freeze §4 requires extends == dependencies[0].plugin_id")
            }
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl std::error::Error for ExtensionInstallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Package(error) => Some(error),
            Self::ExtendsMismatch { .. } => None,
        }
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl From<store::extension::ExtensionPackageError> for ExtensionInstallError {
    fn from(error: store::extension::ExtensionPackageError) -> Self {
        Self::Package(error)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl PluginHost {
    /// 📥️ Verifies + unpacks an `.sxt` byte stream and registers its descriptor, keyed by
    /// `extension_id` — idempotent for a byte-identical reinstall. This is the "install" half of
    /// `important.md`'s extension-activation ruling: "verify/unpack an `.sxt` → register its
    /// descriptor". `async` (unlike this impl block's sibling query methods below): `verify` and
    /// `ExtensionPackageManifest::extends_matches_primary_dependency` are still genuinely `async
    /// fn`; `content_hash` is a pure blake3 digest and is called synchronously below.
    pub async fn install_extension_package(&mut self, bytes: &[u8]) -> Result<InstalledExtension, ExtensionInstallError> {
        let manifest = store::extension::verify(bytes).await?;
        if !manifest.extends_matches_primary_dependency().await {
            let actual = manifest.dependencies.first().map(|dependency| dependency.plugin_id.clone()).unwrap_or_default();
            return Err(ExtensionInstallError::ExtendsMismatch { extension_id: manifest.extension_id.clone(), extends: manifest.extends.clone(), actual });
        }
        let content_hash = store::extension::content_hash(bytes);
        let installed = InstalledExtension { manifest, content_hash };
        self.installed_extensions.insert(installed.manifest.extension_id.clone(), installed.clone());
        Ok(installed)
    }

    /// 🗑️ Removes an installed extension's descriptor. Once uninstalled it is no longer handed out
    /// by `extensions_extending_plugin`, whether or not an actor for it is still live — see that
    /// method's own docstring for the deactivation-cascade gap this leaves open (`Kernel` itself
    /// exposes no `deactivate(...)` yet; see the report).
    pub fn uninstall_extension_package(&mut self, extension_id: &str) -> Option<InstalledExtension> {
        self.installed_extensions.remove(extension_id)
    }

    pub fn installed_extension(&self, extension_id: &str) -> Option<&InstalledExtension> {
        self.installed_extensions.get(extension_id)
    }

    /// 🔍️ Every installed extension descriptor whose `extends` names `plugin_id` — the exact
    /// query `important.md`'s extension-activation ruling specifies: "on plugin activation, the
    /// kernel queries installed descriptors for `extends == plugin_id` and activates each as
    /// `ActorKind::Extension`". Data-driven over `self.installed_extensions`: correct for 0, 1, or
    /// the scale fixture's 2,500 synthetic extensions with zero special-casing by count (proven at
    /// that scale by `🧰️framework/🔨️modules/🎠️kernel`'s own `extensions_extending` test and this
    /// packet's standalone verification script — see the report). The caller feeds each returned
    /// descriptor's `extension_id`/`extends` into `ActorKind::Extension { plugin, extension_id }`
    /// and `🎠️activation/🦀️.rs`'s `NativeKernelRuntime::activate` — see this method's own docstring
    /// in the report for why that final wiring step is not done HERE.
    pub fn extensions_extending_plugin(&self, plugin_id: &str) -> Vec<&InstalledExtension> {
        self.installed_extensions.values().filter(|installed| installed.manifest.extends == plugin_id).collect()
    }

    /// 🔒️ "capabilities scoped to the parent" — intersects an extension's own capability asks
    /// (`manifest.capabilities`, dotted capability-id strings) with its parent plugin's own
    /// already-effective set, so an extension actor can never end up holding a capability its
    /// host plugin does not itself hold (`📓️design-abi.md` §5's admission formula).
    pub fn extension_capabilities_scoped_to_parent<'a>(&self, extension: &'a InstalledExtension, parent_effective_capabilities: &[String]) -> Vec<&'a str> {
        extension.manifest.capabilities.iter().map(String::as_str).filter(|capability| parent_effective_capabilities.iter().any(|granted| granted == capability)).collect()
    }
}
//#endregion 🔖️ExtensionInstall

#[cfg(test)]
include!("../🧪️tests/🔬️host-unit/🦀️.rs");
// #endregion host
