//! 🖥️ Plugin-based OS kernel: hot-swappable WASM plugins, workflow, document VCS.

#[cfg(feature = "os-host-full")]
pub mod host {
    // #region host
    //! 🔌️ Plugin host, studio document VCS store, backbone, and catalog.

    use crate::instance::{create_os_id, OsInstanceState};
    use crate::registry::{os_app_registration, resolve_os_app_definition, PluginRegistry};
    use crate::space;
    use crate::workflow;
    use protocol::Mutation;
    use semio_framework::{AppDefinition, PluginManifest, TopicContribution, ViewModel};
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, LazyLock, Mutex};
    use store::{create_document_envelope, document_backbone_ref, materialize_document_snapshot, ArtifactBackboneRef, ArtifactCommand, ArtifactEnvelope, ArtifactStore};
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ProgramContributionEntry {
        pub plugin_id: String,
        pub topic_contribution: TopicContribution,
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
                    entries.push(ProgramContributionEntry { plugin_id: loaded.plugin_id.clone(), topic_contribution: topic_contribution.clone() });
                }
            }
            entries
        }

        pub fn contributions_json(&self) -> String {
            serde_json::to_string(&self.contributions()).unwrap_or_else(|_| "[]".into())
        }

        pub fn create_instance(&mut self, app_id: &str, document_json: String) -> Option<u32> {
            let app = self.registry.find_app(app_id)?;
            let id = self.next_instance_id;
            self.next_instance_id += 1;
            self.instances.insert(id, OsInstanceState { id, app_id: app.id.clone(), controller_id: app.controller_id.clone(), document_json, view_state: ViewModel::default(), generation: 0 });
            Some(id)
        }

        pub fn instance(&self, instance_id: u32) -> Option<&OsInstanceState> {
            self.instances.get(&instance_id)
        }

        pub fn instance_mut(&mut self, instance_id: u32) -> Option<&mut OsInstanceState> {
            self.instances.get_mut(&instance_id)
        }

        //#region 🔖️ActionKernel

        /// @emoji 🩺️ Delegates to `ui_wgpu::wgpu::ui_recovery_panel`'s `🔖️StatusBuilders` builder — this host
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

        /// @emoji 🔢️ Same-version hot-swaps must not silently drop apps — a version bump is required to
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
    /// (schema/id/name/vcs/cursor/outcomes/conflicts/backbone), parametrized over any `<P, Op>` pair
    /// `store::create_document_envelope`/`materialize_document_snapshot`/`print_document_pack`/
    /// `parse_document_pack` already support generically — nothing OS-specific left to hardcode. See
    /// `## The inversion` in the plan: `OsSnapshot`/`OsMutation`/`OsDocument` dissolve into the three
    /// type aliases below instead of one bespoke studio-only document type.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BackboneDocument<P, Op> {
        pub schema: String,
        pub id: String,
        pub name: String,
        pub vcs: ArtifactVcs<P, Op>,
        pub cursor: store::ArtifactCursor,
        pub edit_messages: Vec<protocol::EditMessages>,
        pub conflicts: Vec<protocol::Conflict>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backbone: Option<ArtifactBackboneRef>,
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

    /// @emoji 🌱️ Mints a fresh backbone document wrapping `initial_snapshot` with empty edit history.
    pub fn create_backbone_document<P, Op>(schema: &str, id: &str, name: &str, initial_snapshot: P) -> BackboneDocument<P, Op>
    where
        P: Clone,
        Op: Clone,
    {
        BackboneDocument {
            schema: schema.into(),
            id: id.into(),
            name: name.into(),
            vcs: create_document_envelope::<P, Op>(schema, id, initial_snapshot, None).vcs.clone(),
            cursor: store::ArtifactCursor::default(),
            edit_messages: Vec::new(),
            conflicts: Vec::new(),
            backbone: None,
        }
    }

    /// @emoji 🌉️ Builds the authoritative `ArtifactEnvelope` a `BackboneDocument` wraps, dropping only
    /// the app-level `name` and preserving its complete persisted cursor.
    fn backbone_envelope_of<P, Op>(document: &BackboneDocument<P, Op>) -> ArtifactEnvelope<P, Op>
    where
        P: Clone,
        Op: Clone,
    {
        ArtifactEnvelope::from_owners(store::ArtifactEnvelopeOwners {
            schema: document.schema.clone(),
            id: document.id.clone(),
            vcs: document.vcs.clone(),
            backbone: document.backbone.clone(),
            active_alternative_id: None,
            cursor: Some(document.cursor.clone()),
            dialect: None,
            migrated_from: None,
            owner: None,
            lanes: std::collections::BTreeMap::new(),
            edit_messages: store::ArtifactEditMessageLedger::from_preflighted_entries(document.edit_messages.clone()),
            conflicts: document.conflicts.clone(),
        })
    }

    pub fn materialize_backbone_snapshot<P, Op>(document: &BackboneDocument<P, Op>, applied_edit_ids: &[String]) -> Result<P, VcsError>
    where
        P: Clone,
        Op: Clone + Mutation<P>,
    {
        let envelope = backbone_envelope_of(document);
        resolve_kernel_future(materialize_document_snapshot(&envelope, applied_edit_ids))
    }

    /// @emoji 📤️ Exports an already-loaded backbone document as pack bytes + ops text.
    pub fn export_backbone_pack<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::ArtifactPackFiles, VcsError>
    where
        P: Clone + store::ArtifactPack,
        Op: Clone + protocol::OpText + protocol::OpBinary,
    {
        resolve_kernel_future(store::print_document_pack(&backbone_envelope_of(document)))
    }

    /// @emoji 📤️ DSL-text counterpart of `export_backbone_pack`.
    pub fn export_backbone_dsl<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::ArtifactTextFiles, VcsError>
    where
        P: Clone + store::ArtifactDsl,
        Op: Clone + protocol::OpText,
    {
        resolve_kernel_future(store::print_document_text(&backbone_envelope_of(document)))
    }

    /// @emoji 📦️ Binary pack+spr payload for the whole `BackboneDocument` (name + applied-edit cursor +
    /// vcs) — the persisted/synced form. `name` rides as a `store::encode_document_pack_bytes`-framed
    /// blob wrapping a nested `pack`+`spr` pair, and the complete cursor rides through the envelope
    /// so `spr` restores the exact undo/redo/checkpoint position.
    pub fn encode_backbone_payload<P, Op>(document: &BackboneDocument<P, Op>) -> Result<Vec<u8>, VcsError>
    where
        P: Clone + store::ArtifactPack,
        Op: Clone + protocol::OpText + protocol::OpBinary,
    {
        let files = resolve_kernel_future(store::print_document_pack(&backbone_envelope_of(document)))?;
        let inner = resolve_kernel_future(store::encode_document_pack_bytes(&files.pack, &files.spr));
        Ok(resolve_kernel_future(store::encode_document_pack_bytes(document.name.as_bytes(), &inner)))
    }

    /// @emoji 📥️ Inverse of `encode_backbone_payload` — `expected_schema` guards against decoding one
    /// document kind's bytes as another.
    pub fn decode_backbone_payload<P, Op>(bytes: &[u8], expected_schema: &str) -> Result<BackboneDocument<P, Op>, VcsError>
    where
        P: Clone + store::ArtifactPack,
        Op: Clone + protocol::OpText + protocol::OpBinary + Mutation<P>,
    {
        let (name_bytes, inner) = resolve_kernel_future(store::decode_document_pack_bytes(bytes))?;
        let name = String::from_utf8(name_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let (pack, spr) = resolve_kernel_future(store::decode_document_pack_bytes(&inner))?;
        let parsed: store::ParsedDocumentText<P, Op> = resolve_kernel_future(store::parse_document_pack(&pack, &spr)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        if parsed.envelope.schema != expected_schema {
            return Err(VcsError::Deserialize(format!("expected schema {expected_schema}")));
        }
        let cursor = parsed.envelope.cursor.clone().ok_or_else(|| VcsError::Deserialize("backbone payload has no cursor".to_string()))?;
        Ok(BackboneDocument { schema: parsed.envelope.schema.clone(), id: parsed.envelope.id.clone(), name, vcs: parsed.envelope.vcs.clone(), cursor, edit_messages: parsed.envelope.edit_messages.iter().cloned().collect(), conflicts: parsed.envelope.conflicts.clone(), backbone: parsed.envelope.backbone.clone() })
    }
    //#endregion 🔖️BackboneDocument

    //#region 🔖️GraphReconcile
    /// @emoji 🧵️ Post-materialization workflow integrity pass, invoked explicitly by
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
                    level: dsl::Severity::Warning,
                    code: dsl::FaultCode::new("workflow/edge-orphaned"),
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
                        level: dsl::Severity::Warning,
                        code: dsl::FaultCode::new("workflow/edge-type-mismatch"),
                        message: format!("edge {} contract stale: no longer matches negotiated port types", edge.id),
                        target: vec![edge.id.clone()],
                        op_index: None,
                    });
                    false
                }
                Err(reason) => {
                    conflicts.push(protocol::MutationMessage {
                        level: dsl::Severity::Warning,
                        code: dsl::FaultCode::new("workflow/edge-type-mismatch"),
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
                        *smallest = edge.id.clone()
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

    /// @emoji 🎛️ Maps a `workflow::WorkflowParameter` to its `WorkflowParameterType` tag — needed here
    /// to type-check a binding's parameter against its target `ConfigFieldShape`.
    fn workflow_parameter_type_of(parameter: &workflow::WorkflowParameter) -> workflow::WorkflowParameterType {
        match parameter {
            workflow::WorkflowParameter::Numeric { .. } => workflow::WorkflowParameterType::Numeric,
            workflow::WorkflowParameter::Categorical { .. } => workflow::WorkflowParameterType::Categorical,
            workflow::WorkflowParameter::Toggle { .. } => workflow::WorkflowParameterType::Toggle,
            workflow::WorkflowParameter::Text { .. } => workflow::WorkflowParameterType::Text,
        }
    }

    /// @emoji 🌀️ Repeatedly finds a cycle in `edges` (by node-id adjacency) and drops the participating
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
                level: dsl::Severity::Warning,
                code: dsl::FaultCode::new("workflow/edge-cycle"),
                message: format!("edge {} was dropped to break a cycle in the workflow", dropped.id),
                target: vec![dropped.id.clone()],
                op_index: None,
            });
        }
        edges
    }

    /// @emoji 🔍️ DFS cycle detection adapted from `workflow::validate_workflow`'s check, but returning
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
        pub fn new(document: OsWorkflowArtifactDocument) -> Result<Self, VcsError> {
            let cursor = document.cursor.clone();
            let envelope = ArtifactEnvelope::from_owners(store::ArtifactEnvelopeOwners {
                schema: document.schema,
                id: document.id,
                vcs: document.vcs,
                backbone: document.backbone,
                active_alternative_id: None,
                cursor: Some(cursor),
                dialect: None,
                migrated_from: None,
                owner: None,
                lanes: std::collections::BTreeMap::new(),
                edit_messages: store::ArtifactEditMessageLedger::from_preflighted_entries(document.edit_messages),
                conflicts: document.conflicts,
            });
            let inner = resolve_kernel_future(ArtifactStore::new(envelope))?;
            Ok(Self { inner, name: document.name })
        }

        pub fn generation(&self) -> u64 {
            self.inner.generation()
        }

        pub fn snapshot(&self) -> Result<workflow::WorkflowSnapshot, VcsError> {
            self.inner.snapshot()
        }

        /// @emoji 🤝️ Fresh replay plus `reconcile_workflow_snapshot`'s whole 4(+1)-rule pipeline —
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
                cursor: envelope.cursor.clone().expect("artifact stores persist an explicit cursor"),
                edit_messages: envelope.edit_messages.iter().cloned().collect(),
                conflicts: envelope.conflicts.clone(),
                backbone: envelope.backbone.clone(),
            }
        }

        pub fn dispatch_text(&mut self, command_text: &str) -> Result<(), VcsError> {
            resolve_kernel_future(self.inner.dispatch_text(command_text)).map(|_| ())
        }

        pub fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<(), VcsError> {
            resolve_kernel_future(self.inner.dispatch_binary(command_bytes)).map(|_| ())
        }

        pub fn dispatch_apply(&mut self, mutations: Vec<workflow::WorkflowMutation>) -> Result<(), VcsError> {
            resolve_kernel_future(self.inner.dispatch(ArtifactCommand::Apply { mutations, description: None })).map(|_| ())
        }

        pub fn set_workflow_name(&mut self, name: &str) {
            self.name = name.into();
            let _ = self.inner.generation();
        }

        /// @emoji 🆔️ Mints a fresh `WorkflowNode` (id, ports, document/config refs — everything) via
        /// `workflow::workflow_node_for_app`, at dispatch time, so replay never re-derives it. Also
        /// dispatches `space::SpaceMutation::InstallProgram` against `space_store` — the owning space's
        /// `programs` list moved off the dissolved `OsSnapshot` onto `space::SpaceSnapshot` (see
        /// `## The inversion` in the plan), so spawning a node into the workflow graph and installing its
        /// plugin into the space are now two operations against two separate documents.
        pub fn add_workflow_node(&mut self, plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64, space_store: &mut OsSpaceStore) -> Result<String, VcsError> {
            let app = resolve_os_app_definition(plugin_id, app_id).ok_or_else(|| VcsError::Deserialize(format!("unknown app {plugin_id}/{app_id}")))?;
            let node_id = create_os_id("node");
            let position = workflow::WorkflowPosition { x, y, width: 0.0, height: 0.0 };
            let mut node = resolve_kernel_future(workflow::workflow_node_for_app(&app, plugin_id, &node_id, &position));
            if let Some(label) = label {
                node.label = label.into();
            }
            self.dispatch_apply(vec![workflow::WorkflowMutation::AddNode(workflow::AddNode { node })])?;
            resolve_kernel_future(space_store.dispatch(ArtifactCommand::Apply { mutations: vec![space::SpaceMutation::InstallProgram { plugin_id: plugin_id.into() }], description: None }))?;
            Ok(node_id)
        }

        pub fn add_parameter(&mut self, parameter_type: &workflow::WorkflowParameterType, name: &str) -> Result<String, VcsError> {
            let parameter = resolve_kernel_future(workflow::create_default_workflow_parameter(parameter_type, name, None));
            let parameter_id_value = workflow::workflow_parameter_id(&parameter).to_string();
            self.dispatch_apply(vec![workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(parameter) })])?;
            Ok(parameter_id_value)
        }

        pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &workflow::WorkflowParameterPatch) -> Result<(), VcsError> {
            let document = self.snapshot()?;
            let current = document.parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == target_parameter_id).cloned().ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
            let next = resolve_kernel_future(workflow::patch_workflow_parameter(&current, patch));
            self.dispatch_apply(vec![workflow::WorkflowMutation::ChangeParameter(workflow::ChangeParameter { parameter_id: target_parameter_id.into(), parameter: Box::new(next) })])
        }

        /// @emoji 📡️ Pumps any queued inbound backbone messages into the edit timeline.
        pub fn tick(&mut self) -> Result<bool, VcsError> {
            resolve_kernel_future(self.inner.tick())
        }

        /// @emoji 🔗️ Resolves and attaches a backbone by uri. Only available inside the wasm sandbox
        /// (every scheme forwards to the host over the injected `BackboneChannelPort`, a pure queue) —
        /// see {@link attach_backbone} for the native counterpart, which takes an explicit
        /// `Box<dyn store::Backbone>` since native has no URI→IO auto-resolution anymore (`framework/sync`'s
        /// `host_runtime` module owns constructing the real endpoint via `ArtifactHost`).
        #[cfg(target_arch = "wasm32")]
        pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
            resolve_kernel_future(self.inner.attach_backbone_uri(uri))
        }

        /// @emoji 🔗️ Attaches an explicit native backbone channel (typically a `channel_backbone` handed
        /// out by `framework/sync`'s `ArtifactHost::open`, per `host_runtime`'s canonical sequence).
        #[cfg(not(target_arch = "wasm32"))]
        pub fn attach_backbone(&mut self, backbone: Box<dyn store::Backbone>) -> Result<(), VcsError> {
            self.inner.attach_backbone(backbone)
        }

        pub fn detach_backbone(&mut self) {
            self.inner.detach_backbone();
        }

        pub fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
            self.inner.backbone_ref()
        }
    }
    //#endregion 🔖️OsWorkflowStore

    //#region 🔖️Backbone
    /// @emoji 🔌️ Byte-oriented studio persistence port — `read`/`write` carry `encode_backbone_payload`'s
    /// binary pack+spr blob, never JSON. Every implementor today (`MemoryBackbonePort`,
    /// `LocalStorageBackbonePort`, and the `file://`/`folder://` host ports opened by
    /// `open_file_space_backbone`/`open_folder_space_backbone`) is bridged from the underlying
    /// string-typed `store::BackbonePort` via the blanket impl below — see its doc for why.
    pub trait OsBackbonePort: Send + Sync {
        fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError>;
        fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError>;
    }

    /// @emoji 🌉️ `store::BackbonePort` is the shared string-typed transport for every document kind
    /// across the whole kernel (localStorage, in-memory, host file/folder ports) — changing its own
    /// signature to bytes is out of scope here. Base64 is the bridge: an empty payload maps to an
    /// empty string both ways (preserving `delete_os_space`'s tombstone-write semantics), and every
    /// non-empty payload round-trips byte-for-byte through the encoding.
    impl<T: store::BackbonePort + Send + Sync> OsBackbonePort for T {
        fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
            use base64::Engine;
            let text = resolve_kernel_future(store::BackbonePort::read(self, uri))?;
            if text.is_empty() {
                return Ok(Vec::new());
            }
            base64::engine::general_purpose::STANDARD.decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
        }

        fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
            use base64::Engine;
            if payload.is_empty() {
                return resolve_kernel_future(store::BackbonePort::write(self, uri, ""));
            }
            resolve_kernel_future(store::BackbonePort::write(self, uri, &base64::engine::general_purpose::STANDARD.encode(payload)))
        }
    }

    /// @emoji 🧬️ Enum dispatch over every `OsBackbonePort` implementor (O1 — dyn dispatch dropped in
    /// favor of enum/match dispatch, mirroring `🏪️store`'s `BackbonePorts` shape exactly —
    /// `📓️terra-store-dedyn-report.md`). `Store` covers every `store::BackbonePort`-shaped transport
    /// (in-memory, localStorage) via the blanket bridge above; `Space` covers the host-only
    /// file/folder-backed `backbone::SpaceBackbonePort`. These are the only two implementors in the
    /// tree (verified: one blanket impl over `store::BackbonePort`, one direct impl on
    /// `backbone::SpaceBackbonePort` — `📓️terra-os-backbone-report.md`).
    pub enum OsBackbonePorts {
        Store(store::BackbonePorts),
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

    /// @emoji 🌉️ Writes any `BackboneDocument<P, Op>` to `uri`, stamping its own `backbone` ref first —
    /// shared by every catalog write path below (space manifests, collections).
    fn sync_backbone_document<P, Op>(document: &BackboneDocument<P, Op>, backbone_uri: &str, port: &Arc<OsBackbonePorts>) -> Result<(), VcsError>
    where
        P: Clone + store::ArtifactPack,
        Op: Clone + protocol::OpText + protocol::OpBinary,
    {
        let mut synced = document.clone();
        synced.backbone = Some(resolve_kernel_future(document_backbone_ref(backbone_uri)));
        port.write(backbone_uri, &encode_backbone_payload(&synced)?)
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

    /// @emoji 🔎️ Extracts a bare space id from a `space://<id>` uri — deliberately excludes the
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
        let updated_at = document.vcs.changes.last().map(|change| change.saved_at.clone()).unwrap_or_else(|| "0".into());
        Ok(OsSpaceCatalogEntry { id: space_id, name: document.name.clone(), backbone_uri: backbone_uri.into(), kind: snapshot.kind, visibility: snapshot.visibility, collection_count: snapshot.collections.len(), updated_at })
    }

    /// @emoji 📚️ Lists persisted space manifests from the dev backbone namespace.
    pub fn list_os_space_catalog_entries(port: Arc<OsBackbonePorts>) -> Result<Vec<OsSpaceCatalogEntry>, VcsError> {
        let mut entries = Vec::new();
        let uris: Vec<String> = SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&port_key(&port)).cloned().unwrap_or_default().into_iter().collect();
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

    /// @emoji 🆕️ Creates a space manifest PLUS one default "main" collection on the dev backbone — see
    /// `## The inversion`/`Addressing` in the plan: a space no longer auto-creates a workflow artifact
    /// (that's an explicit, later user action), only the collection every space needs to hold artifacts
    /// in the first place.
    pub fn create_os_space(name: &str, kind: space::SpaceKind, visibility: space::SpaceVisibility, owner: space::SpaceUser, port: Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let space_id = create_os_id("space");
        let collection_id = create_os_id("collection");
        let mut space_snapshot = space::empty_space_snapshot(name.trim(), kind, visibility);
        space_snapshot.users.push(owner);
        space_snapshot.collections.push(space::CollectionRef { id: collection_id.clone(), name: "main".into(), document_id: collection_id.clone() });
        let space_document: OsSpaceDocument = create_backbone_document(space::S_SPACE_SCHEMA, &space_id, name.trim(), space_snapshot);
        let collection_document: OsCollectionDocument = create_backbone_document(space::S_COLLECTION_SCHEMA, &collection_id, "main", space::empty_collection_snapshot("main"));

        let space_uri = space::space_backbone_uri(&space_id);
        let collection_uri = space::collection_backbone_uri(&space_id, &collection_id);
        sync_backbone_document(&space_document, &space_uri, &port)?;
        sync_backbone_document(&collection_document, &collection_uri, &port)?;
        track_os_space_backbone_uri(&port, &space_uri);
        track_os_space_backbone_uri(&port, &collection_uri);
        os_space_catalog_entry_from_document(&space_uri, &space_document)
    }

    /// @emoji 🗑️ Deletes a space manifest and every collection it references from the dev backbone.
    pub fn delete_os_space(space_id: &str, port: Arc<OsBackbonePorts>) -> Result<(), VcsError> {
        let uri = space::space_backbone_uri(space_id);
        if let Ok(document) = load_os_space_document(space_id, port.clone()) {
            if let Ok(snapshot) = materialize_backbone_snapshot(&document, &[]) {
                for collection in &snapshot.collections {
                    let collection_uri = space::collection_backbone_uri(space_id, &collection.id);
                    untrack_os_space_backbone_uri(&port, &collection_uri);
                    port.write(&collection_uri, &[])?;
                }
            }
        }
        untrack_os_space_backbone_uri(&port, &uri);
        port.write(&uri, &[])
    }

    /// @emoji 🌉️ Shared admission tail for `import_os_space_from_dsl`/`import_os_space_from_pack`:
    /// mints a fresh id when the source carried none, syncs, and tracks the catalog uri.
    fn admit_os_space_document(mut document: OsSpaceDocument, port: Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let space_id = if document.id.is_empty() { create_os_id("space") } else { document.id.clone() };
        let backbone_uri = space::space_backbone_uri(&space_id);
        document.id = space_id;
        sync_backbone_document(&document, &backbone_uri, &port)?;
        track_os_space_backbone_uri(&port, &backbone_uri);
        os_space_catalog_entry_from_document(&backbone_uri, &document)
    }

    /// @emoji 📥️ Imports a space manifest dsl text (`export_os_space_dsl`'s counterpart) onto the dev
    /// backbone. Does not create a collection — a manifest imported this way is expected to already
    /// reference its own collections (a fresh, collection-less space only comes from `create_os_space`).
    pub fn import_os_space_from_dsl(dsl: &str, port: Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let snapshot = <space::SpaceSnapshot as store::ArtifactDsl>::parse_dsl(dsl).map_err(|error| VcsError::Deserialize(error.message))?;
        let vcs = create_document_envelope::<space::SpaceSnapshot, space::SpaceMutation>(space::S_SPACE_SCHEMA, "", snapshot, None).vcs.clone();
        admit_os_space_document(
            BackboneDocument {
                schema: space::S_SPACE_SCHEMA.into(),
                id: String::new(),
                name: String::new(),
                vcs,
                cursor: store::ArtifactCursor::default(),
                edit_messages: Vec::new(),
                conflicts: Vec::new(),
                backbone: None,
            },
            port,
        )
    }

    /// @emoji 📦️ Pack counterpart of `import_os_space_from_dsl`.
    pub fn import_os_space_from_pack(pack: &[u8], spr: &[u8], port: Arc<OsBackbonePorts>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let parsed: store::ParsedDocumentText<space::SpaceSnapshot, space::SpaceMutation> = resolve_kernel_future(store::parse_document_pack(pack, spr)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let cursor = parsed.envelope.cursor.clone().ok_or_else(|| VcsError::Deserialize("space pack has no cursor".to_string()))?;
        let document = BackboneDocument {
            schema: parsed.envelope.schema.clone(),
            id: parsed.envelope.id.clone(),
            name: String::new(),
            vcs: parsed.envelope.vcs.clone(),
            cursor,
            edit_messages: parsed.envelope.edit_messages.iter().cloned().collect(),
            conflicts: parsed.envelope.conflicts.clone(),
            backbone: parsed.envelope.backbone.clone(),
        };
        admit_os_space_document(document, port)
    }

    /// @emoji 📤️ Exports an already-loaded space manifest as pack bytes + ops text.
    pub fn export_os_space_pack(document: &OsSpaceDocument) -> Result<store::ArtifactPackFiles, VcsError> {
        export_backbone_pack(document)
    }

    /// @emoji 📤️ DSL-text counterpart of `export_os_space_pack`.
    pub fn export_os_space_dsl(document: &OsSpaceDocument) -> Result<store::ArtifactTextFiles, VcsError> {
        export_backbone_dsl(document)
    }

    /// @emoji 📂️ Loads a space manifest from the dev backbone.
    pub fn load_os_space_document(space_id: &str, port: Arc<OsBackbonePorts>) -> Result<OsSpaceDocument, VcsError> {
        let backbone_uri = space::space_backbone_uri(space_id);
        let payload = port.read(&backbone_uri)?;
        if payload.is_empty() {
            return Err(VcsError::Backbone(format!("unknown os space: {space_id}")));
        }
        decode_backbone_payload(&payload, space::S_SPACE_SCHEMA)
    }

    /// @emoji 🌱️ Seeds the demo space when the catalog is empty.
    pub fn seed_os_space_catalog_if_empty(seed_document: OsSpaceDocument, port: Arc<OsBackbonePorts>) -> Result<Option<OsSpaceCatalogEntry>, VcsError> {
        if !list_os_space_catalog_entries(port.clone())?.is_empty() {
            return Ok(None);
        }
        let space_id = if seed_document.id.is_empty() { "default".into() } else { seed_document.id.clone() };
        let backbone_uri = space::space_backbone_uri(&space_id);
        let mut seeded = seed_document;
        seeded.id = space_id;
        sync_backbone_document(&seeded, &backbone_uri, &port)?;
        track_os_space_backbone_uri(&port, &backbone_uri);
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
    /// docstring in `🧩️extension/🦀️component.rs` gives for ITS wire-shape duplication) — so the two
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
        /// and `🎠️activation.rs`'s `NativeKernelRuntime::activate` — see this method's own docstring
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
    mod tests {
        use super::*;
        use crate::workflow::{empty_workflow, placeholder_media_contract, validate_workflow, MediaContract, WorkflowEdge, WorkflowPosition};
        use semio_framework::{AppRole, ArtifactDialect, MediaClass, MediaForm, MediaType, MediaWireFormat, ModeDefinition, PluginManifest, WindowKindDefinition};
        use std::sync::Arc;
        use store::{MemoryBackbone, MemoryBackbonePort};
        use ui_wgpu::wgpu::{LocalizedLabel, SurfaceKind};

        #[test]
        fn loads_plugin_apps_into_registry() {
            let mut host = PluginHost::new();
            let manifest = PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.1.0".into(),
                apps: vec![AppDefinition {
                    id: "draw-play".into(),
                    role: AppRole::Editor,
                    dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
                    label: LocalizedLabel::data("Draw"),
                    breadcrumb: vec!["semio".into(), "draw".into()],
                    icon_id: None,
                    controller_id: "draw-play".into(),
                    modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                    default_mode_id: "edit".into(),
                    window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                        id: "composite".into(),
                        label: LocalizedLabel::data("Canvas"),
                        body_key: "composite".into(),
                        surface_kind: SurfaceKind::Canvas2d,
                        icon_id: "pen-tool".into(),
                        options: ui_wgpu::wgpu::WindowOptions::default(),
                        actions: Vec::new(),
                        utilities: Vec::new(),
                        interactions: Vec::new(),
                        params_schema: None,
                        artifact_snapshot_schema: None,
                        input_event_schema: None,
                        output_schema: None,
                        capabilities: vec![],
                    }),
                    panel_tabs: vec![],
                    keybindings: vec![],
                    utilities: Vec::new(),
                    tools: Vec::new(),
                    commands: Vec::new(),
                    interactions: Vec::new(),
                    named_layouts: Vec::new(),
                    default_layout: None,
                    terminologies: Vec::new(),
                    terminology_breadcrumbs: std::collections::HashMap::new(),
                    introduction: None,
                    dialogs: Vec::new(),
                    media_inputs: Vec::new(),
                    media_outputs: Vec::new(),
                    artifact_kinds: Vec::new(),
                    config: semio_framework::ConfigSpec::empty(),
                    command_grammar: semio_framework::CommandGrammar::empty(),
                    io: semio_framework::AppIo::default(),
                    tutorials: Vec::new(),
                }],
                capabilities: vec![],
                topic_contributions: vec![],
                examples: vec![],
                commands: vec![],
                artifact_kinds: vec![],
                dependencies: vec![],
                contributions: vec![],
            };
            host.load_plugin(LoadedProgram { plugin_id: "draw".into(), manifest, artifact_uri: "program://draw".into() });
            assert_eq!(host.apps().len(), 1);
        }

        #[test]
        fn hot_swap_bumps_instance_generation_and_tracks_app_changes() {
            let mut host = PluginHost::new();
            let draw_app = AppDefinition {
                id: "draw-play".into(),
                role: AppRole::Editor,
                dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
                label: LocalizedLabel::data("Draw"),
                breadcrumb: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: LocalizedLabel::data("Canvas"),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    interactions: Vec::new(),
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                interactions: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_breadcrumbs: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework::ConfigSpec::empty(),
                command_grammar: semio_framework::CommandGrammar::empty(),
                io: semio_framework::AppIo::default(),
                tutorials: Vec::new(),
            };
            let note_app = AppDefinition {
                id: "note-play".into(),
                role: AppRole::Editor,
                dialect: ArtifactDialect { artifact_kind: "s.test.note".into(), standard: "1".into(), subset: "*".into() },
                label: LocalizedLabel::data("Note"),
                breadcrumb: vec!["semio".into(), "note".into()],
                icon_id: None,
                controller_id: "note-play".into(),
                modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: LocalizedLabel::data("Canvas"),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    interactions: Vec::new(),
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                interactions: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_breadcrumbs: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework::ConfigSpec::empty(),
                command_grammar: semio_framework::CommandGrammar::empty(),
                io: semio_framework::AppIo::default(),
                tutorials: Vec::new(),
            };
            host.load_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest {
                    plugin_id: "draw".into(),
                    label: "Draw".into(),
                    version: "0.1.0".into(),
                    apps: vec![draw_app.clone()],
                    capabilities: vec![],
                    topic_contributions: vec![],
                    examples: vec![],
                    commands: vec![],
                    artifact_kinds: vec![],
                    dependencies: vec![],
                    contributions: vec![],
                },
                artifact_uri: "program://draw".into(),
            });
            let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
            let generation_before = host.instance(instance_id).expect("instance").generation;
            let event = host.hot_swap_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest {
                    plugin_id: "draw".into(),
                    label: "Draw".into(),
                    version: "0.2.0".into(),
                    apps: vec![draw_app, note_app],
                    capabilities: vec![],
                    topic_contributions: vec![],
                    examples: vec![],
                    commands: vec![],
                    artifact_kinds: vec![],
                    dependencies: vec![],
                    contributions: vec![],
                },
                artifact_uri: "program://draw".into(),
            });
            assert_eq!(event.added_apps, vec!["note-play".to_string()]);
            assert!(event.removed_apps.is_empty());
            assert_eq!(event.plugin_id, "draw");
            assert_eq!(event.version, "0.2.0");
            assert!(host.instance(instance_id).expect("instance").generation > generation_before, "hot swap must bump instance generation");
            assert_eq!(host.apps().len(), 2);
        }

        #[test]
        fn hot_swap_rollback_on_invalid_manifest_keeps_old_plugin() {
            let mut host = PluginHost::new();
            let draw_app = AppDefinition {
                id: "draw-play".into(),
                role: AppRole::Editor,
                dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
                label: LocalizedLabel::data("Draw"),
                breadcrumb: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: LocalizedLabel::data("Canvas"),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    interactions: Vec::new(),
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                interactions: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_breadcrumbs: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework::ConfigSpec::empty(),
                command_grammar: semio_framework::CommandGrammar::empty(),
                io: semio_framework::AppIo::default(),
                tutorials: Vec::new(),
            };
            host.load_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest {
                    plugin_id: "draw".into(),
                    label: "Draw".into(),
                    version: "0.1.0".into(),
                    apps: vec![draw_app],
                    capabilities: vec![],
                    topic_contributions: vec![],
                    examples: vec![],
                    commands: vec![],
                    artifact_kinds: vec![],
                    dependencies: vec![],
                    contributions: vec![],
                },
                artifact_uri: "program://draw".into(),
            });
            let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
            let generation_before = host.instance(instance_id).expect("instance").generation;
            let event = host.hot_swap_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest {
                    plugin_id: "draw".into(),
                    label: "Draw".into(),
                    version: "".into(),
                    apps: vec![],
                    capabilities: vec![],
                    topic_contributions: vec![],
                    examples: vec![],
                    commands: vec![],
                    artifact_kinds: vec![],
                    dependencies: vec![],
                    contributions: vec![],
                },
                artifact_uri: "program://draw".into(),
            });
            assert_eq!(event.plugin_id, "draw");
            assert_eq!(event.version, "0.1.0");
            assert!(event.added_apps.is_empty());
            assert_eq!(host.apps().len(), 1);
            assert_eq!(host.instance(instance_id).expect("instance").generation, generation_before);
            assert_eq!(host.programs.get("draw").expect("plugin").manifest.version, "0.1.0");
        }

        #[test]
        fn contributions_track_plugin_load_and_hot_swap() {
            let mut host = PluginHost::new();
            let topic_contribution = TopicContribution::new(
                "playbook.blockKind",
                semio_framework::DslValue::object([
                    ("appId".to_string(), semio_framework::DslValue::String("playbook-module-procedural".to_string())),
                    ("blockKind".to_string(), semio_framework::DslValue::String("buildingComponent".to_string())),
                    ("label".to_string(), semio_framework::DslValue::String("Building Component".to_string())),
                    ("iconId".to_string(), semio_framework::DslValue::String("building".to_string())),
                    ("defaultValueJson".to_string(), semio_framework::DslValue::String("{}".to_string())),
                    ("paramsBodyKey".to_string(), semio_framework::DslValue::String("params".to_string())),
                    ("previewBodyKey".to_string(), semio_framework::DslValue::String("preview".to_string())),
                ]),
            );
            host.load_plugin(LoadedProgram {
                plugin_id: "playbook-module-procedural".into(),
                manifest: PluginManifest {
                    plugin_id: "playbook-module-procedural".into(),
                    label: "Playbook Module Procedural".into(),
                    version: "0.1.0".into(),
                    apps: vec![],
                    capabilities: vec![],
                    topic_contributions: vec![topic_contribution.clone()],
                    examples: vec![],
                    commands: vec![],
                    artifact_kinds: vec![],
                    dependencies: vec![],
                    contributions: vec![],
                },
                artifact_uri: "program://playbook-module-procedural".into(),
            });
            assert_eq!(host.contributions().len(), 1);
            assert_eq!(host.contributions()[0].plugin_id, "playbook-module-procedural");
            host.hot_swap_plugin(LoadedProgram {
                plugin_id: "playbook-module-procedural".into(),
                manifest: PluginManifest {
                    plugin_id: "playbook-module-procedural".into(),
                    label: "Playbook Module Procedural".into(),
                    version: "0.2.0".into(),
                    apps: vec![],
                    capabilities: vec![],
                    topic_contributions: vec![],
                    examples: vec![],
                    commands: vec![],
                    artifact_kinds: vec![],
                    dependencies: vec![],
                    contributions: vec![],
                },
                artifact_uri: "program://playbook-module-procedural".into(),
            });
            assert!(host.contributions().is_empty());
        }

        #[test]
        fn recovery_ui_renders_actions_for_quarantined_plugin() {
            let mut host = PluginHost::new();
            host.quarantined.insert("draw".into());
            let ui = host.recovery_ui("draw");
            match ui {
                UiNode::Stack(stack) => assert_eq!(stack.children.len(), 5, "title + message + restart/disable/showDiagnostics buttons"),
                other => panic!("expected recovery stack, got {other:?}"),
            }
        }

        /// 🧷️ Minimal `AppDefinition` for registry tests — every field but `io`/`document` is filler;
        /// `register_app_io` only reads `.id`/`.label`/`.io` (see `workflow::workflow_node_for_app`).
        fn test_app_definition(id: &str, label: &str, document_schema: &str, ports: Vec<semio_framework::MediaPortSpec>) -> AppDefinition {
            AppDefinition {
                id: id.into(),
                role: AppRole::Editor,
                dialect: ArtifactDialect { artifact_kind: format!("s.test.{id}"), standard: "1".into(), subset: "*".into() },
                label: LocalizedLabel::data(label),
                breadcrumb: vec!["semio".into(), id.into()],
                icon_id: None,
                controller_id: format!("{id}-play"),
                modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                    id: id.into(),
                    label: LocalizedLabel::data(label),
                    body_key: id.into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "app-window".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    interactions: Vec::new(),
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: Vec::new(),
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                interactions: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_breadcrumbs: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework::ConfigSpec::empty(),
                command_grammar: semio_framework::CommandGrammar::empty(),
                io: semio_framework::AppIo::from_document(
                    document_schema,
                    MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    semio_framework::ArtifactPresentation { id: id.into(), name: label.into(), dimension: "2d".into(), component_kind: id.into() },
                )
                .with_ports(ports),
                tutorials: Vec::new(),
            }
        }

        fn seed_draw_plugin() {
            crate::registry::register_app_io("draw", &test_app_definition("draw", "Draw", "draw.document", Vec::new()));
        }

        /// 🧲️ `draw` declares zero extra output ports, so tests that need to wire an edge *into* a
        /// spawned node register this minimal sink alongside it, wired via the implicit `document:*`
        /// ports every app carries (see `AppIo::all_ports`).
        fn seed_sink_plugin() {
            crate::registry::register_app_io("sink", &test_app_definition("sink", "Sink", "sink.document", Vec::new()));
        }

        fn test_space_store() -> OsSpaceStore {
            let envelope = create_document_envelope(space::S_SPACE_SCHEMA, "space", space::empty_space_snapshot("Space", space::SpaceKind::Studio, space::SpaceVisibility::Private), None);
            ArtifactStore::new(envelope).expect("valid artifact store fixture")
        }

        fn test_workflow_store() -> OsWorkflowStore {
            OsWorkflowStore::new(create_backbone_document(workflow::S_WORKFLOW_SCHEMA, "workflow", "Workflow", workflow::empty_workflow_snapshot())).expect("valid workflow store fixture")
        }

        #[test]
        fn backbone_and_workflow_store_round_trips_preserve_outcomes_and_conflicts() {
            let mut store = test_workflow_store();
            store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Durable").expect("create one edit");
            let mut document = store.document();
            let edit_id = document.cursor.applied_edit_ids.last().expect("one applied edit").clone();
            let edit = document.vcs.edits.iter().find(|edit| edit.id == edit_id).expect("applied edit is persisted");
            let messages = vec![protocol::MutationMessage::warn("mutation.clamped", "durable host outcome").at(["parameters", "0"]).at_op(0)];
            document.edit_messages = vec![protocol::EditMessages { edit_id: edit_id.clone(), messages: messages.clone() }];
            let kind = protocol::ConflictKind::Degraded { edit_ids: vec![edit_id] };
            let timestamp = edit.mutation_meta.first().expect("operation metadata").timestamp;
            let mutation_ids = edit.mutation_meta.iter().map(|meta| meta.mutation_id.clone().expect("stable mutation identity")).collect::<Vec<_>>();
            let actors = vec![protocol::ActorId(edit.actor.clone().expect("stable edit actor"))];
            document.conflicts = vec![protocol::Conflict { id: protocol::ConflictId::new(&kind, &protocol::ArtifactId(document.id.clone()), &mutation_ids, &timestamp), kind, status: protocol::ConflictStatus::Open, messages, actors, timestamp }];

            let payload = encode_backbone_payload(&document).expect("backbone payload encodes");
            let decoded: OsWorkflowArtifactDocument = decode_backbone_payload(&payload, workflow::S_WORKFLOW_SCHEMA).expect("backbone payload decodes");
            assert_eq!(decoded.cursor, document.cursor);
            assert_eq!(decoded.edit_messages, document.edit_messages);
            assert_eq!(decoded.conflicts, document.conflicts);

            let rebuilt = OsWorkflowStore::new(decoded).expect("workflow store rebuilds");
            let rebuilt_document = rebuilt.document();
            assert_eq!(rebuilt_document.edit_messages, document.edit_messages);
            assert_eq!(rebuilt_document.conflicts, document.conflicts);

            let text = export_backbone_dsl(&document).expect("backbone text encodes");
            let parsed = store::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &text.ops).expect("backbone text decodes");
            assert_eq!(parsed.envelope.edit_messages, document.edit_messages);
            assert_eq!(parsed.envelope.conflicts, document.conflicts);
            assert_eq!(parsed.envelope.cursor.expect("text carries explicit cursor"), document.cursor);

            let mut invalid = document.clone();
            invalid.conflicts[0].id = protocol::ConflictId("conflict-invalid".into());
            assert!(encode_backbone_payload(&invalid).is_err(), "host binary persistence must reject a non-content-addressed conflict id");
            let malformed_text = text.ops.replacen(&document.conflicts[0].id.0, "conflict-invalid", 1);
            assert!(store::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &malformed_text).is_err(), "host text persistence must reject a non-content-addressed conflict id");
        }

        #[test]
        fn backbone_binary_text_and_workflow_store_preserve_the_complete_cursor() {
            let mut store = test_workflow_store();
            store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Committed").expect("first edit");
            store.inner.dispatch(store::ArtifactCommand::CommitCheckpoint { message: Some("cursor checkpoint".into()), authors: Vec::new() }).expect("checkpoint");
            store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Undone").expect("second edit");
            store.dispatch_text("undo").expect("undo second edit");
            let document = store.document();
            assert!(!document.cursor.redo_edit_ids.is_empty(), "precondition: redo lane is populated");
            assert!(document.cursor.checkpoint_id.is_some(), "precondition: checkpoint is populated");

            let payload = encode_backbone_payload(&document).expect("binary encode");
            let decoded: OsWorkflowArtifactDocument = decode_backbone_payload(&payload, workflow::S_WORKFLOW_SCHEMA).expect("binary decode");
            assert_eq!(decoded.cursor, document.cursor);
            assert_eq!(OsWorkflowStore::new(decoded).expect("workflow rebuild").document().cursor, document.cursor);

            let text = export_backbone_dsl(&document).expect("text encode");
            let parsed = store::parse_document_text::<workflow::WorkflowSnapshot, workflow::WorkflowMutation>(&text.dsl, &text.ops).expect("text decode");
            assert_eq!(parsed.envelope.cursor.expect("text cursor"), document.cursor);
        }

        #[test]
        fn spawns_and_removes_app_instances() {
            seed_draw_plugin();
            let mut space_store = test_space_store();
            let mut store = test_workflow_store();
            store.add_workflow_node("draw", "draw", None, 40.0, 40.0, &mut space_store).expect("spawn");
            assert_eq!(store.snapshot().expect("projection").graph.nodes.len(), 1);
            assert!(space_store.snapshot().expect("projection").programs.contains(&"draw".to_string()), "spawning a node must install its plugin into the owning space");
            store.dispatch_text("undo").expect("undo");
            assert_eq!(store.snapshot().expect("projection").graph.nodes.len(), 0);
        }

        #[test]
        fn adds_and_patches_studio_parameters() {
            let mut store = test_workflow_store();
            let parameter_id = store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Zoom").expect("add");
            store.patch_parameter(&parameter_id, &serde_json::json!({ "value": 12.0, "max": 10.0 })).expect("patch");
            match &store.snapshot().expect("projection").parameters[0] {
                workflow::WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
                _ => panic!("expected numeric"),
            }
        }

        #[test]
        fn creates_and_lists_space_catalog_entries() {
            let port = Arc::new(OsBackbonePorts::Store(store::BackbonePorts::Memory(MemoryBackbonePort::new())));
            let owner = space::SpaceUser { id: "user-1".into(), name: "Ada".into(), avatar: None, role: space::SpaceRole::Author };
            let entry = create_os_space("Catalog Space", space::SpaceKind::Studio, space::SpaceVisibility::Private, owner, port.clone()).expect("create");
            assert_eq!(entry.collection_count, 1, "create_os_space must seed exactly one default collection");
            assert_eq!(entry.kind, space::SpaceKind::Studio);
            assert_eq!(entry.visibility, space::SpaceVisibility::Private);
            let listed = list_os_space_catalog_entries(port.clone()).expect("list");
            assert!(listed.iter().any(|row| row.id == entry.id));
            delete_os_space(&entry.id, port.clone()).expect("delete");
            assert!(!list_os_space_catalog_entries(port).expect("list").iter().any(|row| row.id == entry.id));
        }

        #[test]
        fn validates_workflow_cycles() {
            assert!(validate_workflow(&empty_workflow()).ok);
        }

        #[test]
        fn concurrent_delete_and_wire_reconciles_without_a_dangling_edge() {
            seed_draw_plugin();
            seed_sink_plugin();
            let mut space_store_a = test_space_store();
            let mut store_a = test_workflow_store();
            let node_a_id = store_a.add_workflow_node("draw", "draw", None, 0.0, 0.0, &mut space_store_a).expect("spawn a");
            let node_b_id = store_a.add_workflow_node("sink", "sink", None, 200.0, 0.0, &mut space_store_a).expect("spawn b");
            let mut store_b = OsWorkflowStore::new(store_a.document()).expect("valid replicated workflow store fixture");

            let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://reconcile-race", "mem://reconcile-race");
            store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

            let document = store_a.snapshot().expect("projection");
            let node_a = document.graph.nodes.iter().find(|node| node.id == node_a_id).expect("node a");
            let node_b = document.graph.nodes.iter().find(|node| node.id == node_b_id).expect("node b");
            let source_node_id = node_a.id.clone();
            let source_port_id = node_a.outputs.first().expect("node a output port").id.clone();
            let target_node_id = node_b.id.clone();
            let target_port_id = node_b.inputs.first().expect("node b input port").id.clone();

            // 🏃️ Actor A deletes node B; actor B (unaware of the delete) concurrently wires a new edge
            // to a port on node B — the classic delete/wire race `reconcile` must clean up post-merge.
            store_a.dispatch_apply(vec![workflow::WorkflowMutation::RemoveNode(workflow::RemoveNode { node_id: node_b_id.clone() })]).expect("remove node b");
            store_b
                .dispatch_apply(vec![workflow::WorkflowMutation::ConnectPorts(workflow::ConnectPorts {
                    edge: WorkflowEdge { id: "edge-race".into(), source_node_id: source_node_id.clone(), source_port_id, target_node_id: target_node_id.clone(), target_port_id, contract: placeholder_media_contract("draw") },
                })])
                .expect("wire edge to node b");
            store_a.tick().expect("pump a");
            store_b.tick().expect("pump b");

            let (converged_a, conflicts_a) = store_a.snapshot_with_conflicts().expect("snapshot with conflicts a");
            let (converged_b, conflicts_b) = store_b.snapshot_with_conflicts().expect("snapshot with conflicts b");
            assert_eq!(converged_a, converged_b, "both peers must converge on the same reconciled document");
            assert!(converged_a.graph.nodes.iter().all(|node| node.id != node_b_id), "node b must stay removed");
            assert!(converged_a.graph.edges.iter().all(|edge| edge.target_node_id != target_node_id), "the edge wired to the deleted node must be dropped, not dangling");
            assert!(
                conflicts_a.iter().any(|conflict| conflict.code == dsl::FaultCode::new("workflow/edge-orphaned") && conflict.level == dsl::Severity::Warning && conflict.target == vec!["edge-race".to_string()]),
                "dropping the dangling edge must surface a Warning-level conflict targeting the dropped edge"
            );
            assert_eq!(conflicts_a, conflicts_b, "both peers must report the same reconciliation conflicts");
        }

        // 🫀️ The old `presence_upserts_prunes_and_excludes_self` test exercised the deleted `presence:`
        // backbone-URI hack (`write_os_presence`/`read_os_presence_peers`). Presence now flows through
        // the semio_hub's `PresencePeer`/`HubServerFrame::Presence` frames and `framework/sync`'s
        // `ArtifactEvent::Presence` — see `framework/product/os/semio_hub/rs/bin.rs` and
        // `framework/sync/rs/lib.rs` for that layer's own coverage.

        // #region 🔖️DslAndOpText
        /// 🧵️ A representative `WorkflowSnapshot` exercising every collection: two workflow nodes wired
        /// by one edge, one of each `WorkflowParameter` variant, and one parameter binding — so the DSL
        /// round trip actually covers the workflow encoding, not just an empty-document fixpoint.
        fn sample_workflow_snapshot() -> workflow::WorkflowSnapshot {
            let node_a = workflow::WorkflowNode {
                id: "app-1".into(),
                plugin_id: "puzzle".into(),
                app_id: "puzzle2d".into(),
                label: "Puzzle Board \"3D\"".into(),
                yields: "puzzle.2d.fixture".into(),
                artifact_ref: "artifacts/app-1".into(),
                config_ref: "config/app-1".into(),
                x: 0.0,
                y: 0.0,
                width: 220.0,
                height: 92.0,
                inputs: Vec::new(),
                outputs: vec![workflow::WorkflowMediaPort {
                    id: "app-1:puzzle.out:out".into(),
                    spec: semio_framework::MediaPortSpec {
                        id: "puzzle.out".into(),
                        label: "Out".into(),
                        direction: semio_framework::MediaPortDirection::Out,
                        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                        kind_id: Some("puzzle.2d.fixture".into()),
                        required: false,
                        multiplicity: semio_framework::PortMultiplicity::One,
                    },
                }],
            };
            let node_b = workflow::WorkflowNode {
                id: "app-2".into(),
                plugin_id: "draw".into(),
                app_id: "draw".into(),
                label: "Draw Sink".into(),
                yields: "draw.document".into(),
                artifact_ref: "artifacts/app-2".into(),
                config_ref: "config/app-2".into(),
                x: 240.0,
                y: 0.0,
                width: 220.0,
                height: 92.0,
                inputs: vec![workflow::WorkflowMediaPort {
                    id: "app-2:draw.in:in".into(),
                    spec: semio_framework::MediaPortSpec {
                        id: "draw.in".into(),
                        label: "In".into(),
                        direction: semio_framework::MediaPortDirection::In,
                        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                        kind_id: Some("puzzle.2d.fixture".into()),
                        required: false,
                        multiplicity: semio_framework::PortMultiplicity::One,
                    },
                }],
                outputs: Vec::new(),
            };
            let edge = workflow::WorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "app-1".into(),
                source_port_id: "app-1:puzzle.out:out".into(),
                target_node_id: "app-2".into(),
                target_port_id: "app-2:draw.in:in".into(),
                contract: MediaContract { kind_id: "puzzle.2d.fixture".into(), media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, wire: MediaWireFormat::Document { schema: "puzzle.2d.fixture".into() }, conversion: None },
            };
            workflow::WorkflowSnapshot {
                schema: workflow::S_WORKFLOW_SCHEMA.into(),
                graph: workflow::Workflow { schema: workflow::WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b], edges: vec![edge] },
                parameters: vec![
                    workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
                    workflow::WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B, with comma".into()] },
                    workflow::WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: true },
                    workflow::WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hello \"world\"\nnewline".into() },
                ],
                parameter_bindings: vec![workflow::WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "app-1".into(), field_path: "/zoom".into() }],
                inputs: Vec::new(),
                input_bindings: Vec::new(),
                output_bindings: Vec::new(),
            }
        }

        #[test]
        fn dsl_round_trips_default_workflow_snapshot() {
            store::test_support::assert_dsl_round_trip(&workflow::empty_workflow_snapshot());
            store::test_support::assert_dsl_pack_equivalence(&workflow::empty_workflow_snapshot());
        }

        #[test]
        fn dsl_round_trips_workflow_snapshot_with_graph_and_parameters() {
            store::test_support::assert_dsl_round_trip(&sample_workflow_snapshot());
            store::test_support::assert_dsl_pack_equivalence(&sample_workflow_snapshot());
        }

        #[test]
        fn op_text_round_trips_add_workflow_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddNode(workflow::AddNode {
                node: workflow::WorkflowNode {
                    id: "node-1".into(),
                    plugin_id: "puzzle".into(),
                    app_id: "puzzle2d".into(),
                    label: "Puzzle Board".into(),
                    yields: "puzzle.2d.fixture".into(),
                    artifact_ref: "artifacts/node-1".into(),
                    config_ref: "config/node-1".into(),
                    x: 10.0,
                    y: -20.5,
                    width: 220.0,
                    height: 92.0,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
            }));
        }

        #[test]
        fn op_text_round_trips_remove_workflow_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::RemoveNode(workflow::RemoveNode { node_id: "app-1".into() }));
        }

        #[test]
        fn op_text_round_trips_connect_media_ports() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::ConnectPorts(workflow::ConnectPorts {
                edge: workflow::WorkflowEdge {
                    id: "edge-1".into(),
                    source_node_id: "node-1".into(),
                    source_port_id: "app-1:out:out".into(),
                    target_node_id: "node-2".into(),
                    target_port_id: "app-2:in:in".into(),
                    contract: MediaContract {
                        kind_id: "puzzle.2d.fixture".into(),
                        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                        wire: MediaWireFormat::Binary { format_kind: "stl".into() },
                        conversion: Some((MediaForm::Brep, MediaForm::Mesh)),
                    },
                },
            }));
        }

        #[test]
        fn op_text_round_trips_disconnect_media_edge() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::DisconnectEdge(workflow::DisconnectEdge { edge_id: "edge-1".into() }));
        }

        #[test]
        fn op_text_round_trips_move_media_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::MoveNode(workflow::MoveNode { node_id: "node-1".into(), x: 5.5, y: -6.25 }));
        }

        #[test]
        fn op_text_round_trips_patch_workflow_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::RenameNode(workflow::RenameNode { node_id: "app-1".into(), label: "Renamed \"Board\"".into() }));
        }

        #[test]
        fn op_text_round_trips_add_parameter() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) }) }));
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(workflow::WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] }) }));
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(workflow::WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: false }) }));
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::AddParameter(workflow::AddParameter { parameter: Box::new(workflow::WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hi there".into() }) }));
        }

        #[test]
        fn op_text_round_trips_remove_parameter() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::RemoveParameter(workflow::RemoveParameter { parameter_id: "p1".into() }));
        }

        #[test]
        fn op_text_round_trips_patch_parameter() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::ChangeParameter(workflow::ChangeParameter {
                parameter_id: "p1".into(),
                parameter: Box::new(workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 20.0, min: None, max: None, step: None }),
            }));
        }

        #[test]
        fn op_text_round_trips_bind_parameter_field() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::BindParameterField(workflow::BindParameterField { binding: workflow::WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "app-1".into(), field_path: "/zoom".into() } }));
        }

        #[test]
        fn op_text_round_trips_unbind_parameter_field() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::UnbindParameterField(workflow::UnbindParameterField { node_id: "app-1".into(), field_path: "/zoom".into() }));
        }

        #[test]
        fn op_text_round_trips_sync_node_ports() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowMutation::UpdateNodePorts(workflow::UpdateNodePorts {}));
        }

        #[test]
        fn document_text_round_trips_store_with_applied_operation() {
            let envelope = create_document_envelope(workflow::S_WORKFLOW_SCHEMA, "workflow-text-test", workflow::empty_workflow_snapshot(), None);
            let mut store = ArtifactStore::new(envelope).expect("valid artifact store fixture");
            store.dispatch(ArtifactCommand::Apply { mutations: vec![workflow::WorkflowMutation::UpdateNodePorts(workflow::UpdateNodePorts {})], description: None }).expect("apply");
            store::test_support::assert_document_text_round_trip(&store);
            store::test_support::assert_document_pack_round_trip(&store);
        }
        // #endregion 🔖️DslAndOpText

        //#region 🔖️ExtensionInstall
        /// 🚫️async: E5 executor bridge — test-only (`📌️important.md` R4 clause 5: a `#[test] fn`
        /// body is a sanctioned executor entry point). Separate from `🎠️activation.rs`'s one
        /// PRODUCTION bridge in this same crate — R2's "at most one per crate" governs production
        /// code, and R4 clause 5 explicitly does not count a test bridge against that census.
        /// `store::extension::pack`/`verify`/`content_hash` and this module's own
        /// `install_extension_package` are I/O-free (pure zip/hash work over in-memory bytes), so
        /// they resolve on the first poll by construction.
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        fn block_on<F: std::future::Future>(future: F) -> F::Output {
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
                Poll::Pending => panic!("block_on: extension install/pack futures are documented I/O-free"),
            }
        }

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        fn sample_extension_sxt(extension_id: &str, extends: &str, capabilities: Vec<String>) -> Vec<u8> {
            let manifest = store::extension::ExtensionPackageManifest {
                extension_id: extension_id.into(),
                label: extension_id.into(),
                version: "0.1.0".into(),
                extends: extends.into(),
                capabilities,
                topic_contributions: serde_json::json!([]),
                dependencies: vec![store::extension::PackagePluginDependency { plugin_id: extends.into(), version: "^1.0.0".into() }],
                contributions: serde_json::json!([]),
                package_format: store::extension::EXTENSION_PACKAGE_FORMAT,
            };
            let component = b"\0asm\x01\x00\x00\x00fake-component".to_vec();
            block_on(store::extension::pack(&manifest, &component, &[])).expect("pack a valid sample .sxt")
        }

        /// 🧫️ Installs 30 real `.sxt` packages (real zip bytes, real blake3 hash — not a mock)
        /// across 3 distinct parent plugins and proves `extensions_extending_plugin` returns
        /// exactly the matching subset for each, with none for a plugin that installed nothing —
        /// the actual install→query pipeline, not just the pure filter (that claim, at the scale
        /// fixture's full 2,500-descriptor shape, is proven separately by
        /// `🧰️framework/🔨️modules/🎠️kernel`'s own `extensions_extending` test and this packet's
        /// standalone verification script — see the report).
        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        #[test]
        fn install_extension_package_registers_descriptors_queryable_by_extends() {
            let mut host = PluginHost::new();
            for i in 0..30 {
                let extends = format!("plugin-{}", i % 3);
                let bytes = sample_extension_sxt(&format!("ext-{i}"), &extends, vec!["storage.read".into()]);
                let installed = block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");
                assert_eq!(installed.manifest.extension_id, format!("ext-{i}"));
                assert!(!installed.content_hash.is_empty());
            }

            for plugin_index in 0..3 {
                let plugin_id = format!("plugin-{plugin_index}");
                let matched = host.extensions_extending_plugin(&plugin_id);
                assert_eq!(matched.len(), 10, "10 of 30 synthetic extensions extend {plugin_id}");
                assert!(matched.iter().all(|installed| installed.manifest.extends == plugin_id));
            }
            assert!(host.extensions_extending_plugin("plugin-nonexistent").is_empty());
        }

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        #[test]
        fn install_extension_package_rejects_extends_mismatch() {
            // 🩹️ `sample_extension_sxt` always writes a matching dependency; to exercise the
            // rejection path this test packs a manifest whose `extends` disagrees with its own
            // first dependency directly, rather than corrupting bytes (which `verify`'s own
            // envelope/zip checks would reject for an unrelated reason).
            let mismatched_manifest = store::extension::ExtensionPackageManifest {
                extension_id: "ext-mismatch".into(),
                label: "Ext Mismatch".into(),
                version: "0.1.0".into(),
                extends: "flow".into(),
                capabilities: vec![],
                topic_contributions: serde_json::json!([]),
                dependencies: vec![store::extension::PackagePluginDependency { plugin_id: "cad".into(), version: "^1.0.0".into() }],
                contributions: serde_json::json!([]),
                package_format: store::extension::EXTENSION_PACKAGE_FORMAT,
            };
            let manifest_bytes_source = block_on(store::extension::pack(&mismatched_manifest, b"\0asm\x01\x00\x00\x00x", &[])).expect("pack a structurally-valid but contract-violating .sxt");

            let mut host = PluginHost::new();
            let error = block_on(host.install_extension_package(&manifest_bytes_source)).expect_err("extends != dependencies[0].plugin_id must be rejected at install time");
            assert!(matches!(error, ExtensionInstallError::ExtendsMismatch { extension_id, extends, actual } if extension_id == "ext-mismatch" && extends == "flow" && actual == "cad"));
            assert!(host.extensions_extending_plugin("flow").is_empty(), "a rejected install must not register a descriptor");
        }

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        #[test]
        fn extension_capabilities_scoped_to_parent_drops_what_the_parent_lacks() {
            let mut host = PluginHost::new();
            let bytes = sample_extension_sxt("ext-caps", "cad", vec!["storage.read".into(), "storage.write".into()]);
            let installed = block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");

            let parent_effective = vec!["storage.read".to_string(), "http:example.com".to_string()];
            let scoped = host.extension_capabilities_scoped_to_parent(&installed, &parent_effective);
            assert_eq!(scoped, vec!["storage.read"], "storage.write is not in the parent's effective set");
        }

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        #[test]
        fn uninstall_extension_package_removes_it_from_the_query() {
            let mut host = PluginHost::new();
            let bytes = sample_extension_sxt("ext-removable", "cad", vec![]);
            block_on(host.install_extension_package(&bytes)).expect("valid sample .sxt installs");
            assert_eq!(host.extensions_extending_plugin("cad").len(), 1);

            let removed = host.uninstall_extension_package("ext-removable").expect("previously installed");
            assert_eq!(removed.manifest.extension_id, "ext-removable");
            assert!(host.extensions_extending_plugin("cad").is_empty());
            assert!(host.uninstall_extension_package("ext-removable").is_none(), "uninstall is not idempotent-returning on a second call");
        }
        //#endregion 🔖️ExtensionInstall
    }
    // #endregion host
}

#[cfg(feature = "os-host-full")]
pub mod backbone {
    // #region backbone
    //! 🗄️ Trusted host-side backbone ports for local studio storage — reads/writes the raw persisted
    //! json directly, bypassing the duplex `Backbone` channel since there is no other process here.

    use crate::host::{OsBackbonePort, OsBackbonePorts};
    use crate::space;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::store_sync::{FolderEventLogStorage, FolderTextStorage};
    #[cfg(not(target_arch = "wasm32"))]
    use std::sync::Arc;
    use store::MemoryBackbonePort;
    use vcs::VcsError;

    /// @emoji 🗂️ Conventional single-document id used inside a folder-backed studio backbone — a studio
    /// folder holds exactly one os document at its root (app documents get their own document ids once
    /// {@link OsArtifactRef} routes them through `framework/sync`'s multi-document `ArtifactHost`).
    #[cfg(not(target_arch = "wasm32"))]
    const SPACE_FOLDER_DOCUMENT_ID: &str = "studio";

    enum SpacePortKind {
        /// @emoji 🗃️ A single document's pack blob addressed by an arbitrary `file://` path —
        /// `<folder>/<document_id>.<extension>.pack` (authoritative) + `.ops` + a DSL mirror, via
        /// `FolderTextStorage::write_pack`/`read_pack` and the typed `store::parse_document_pack`/
        /// `print_document_pack::<OsSnapshot, OsMutation>` (this crate is fully typed, no
        /// `store::ArtifactCodec` indirection needed).
        #[cfg(not(target_arch = "wasm32"))]
        File { uri: String, storage: FolderTextStorage, document_id: String, extension: String },
        #[cfg(not(target_arch = "wasm32"))]
        Folder(String, FolderEventLogStorage),
    }

    pub struct SpaceBackbonePort {
        kind: Option<SpacePortKind>,
        memory: MemoryBackbonePort,
    }

    impl SpaceBackbonePort {
        #[cfg(not(target_arch = "wasm32"))]
        pub fn file(file_path: &str) -> Result<Self, VcsError> {
            let uri = format!("file://{file_path}");
            let path = std::path::Path::new(file_path);
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("txt").to_string();
            let document_id = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("document").to_string();
            let folder = path.parent().map(|parent| parent.to_path_buf()).unwrap_or_else(|| std::path::PathBuf::from("."));
            Ok(Self { kind: Some(SpacePortKind::File { uri, storage: FolderTextStorage::new(folder), document_id, extension }), memory: MemoryBackbonePort::new() })
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub fn folder(folder_path: &str) -> Result<Self, VcsError> {
            let uri = format!("folder://{folder_path}");
            Ok(Self { kind: Some(SpacePortKind::Folder(uri, FolderEventLogStorage::new(std::path::PathBuf::from(folder_path)))), memory: MemoryBackbonePort::new() })
        }
    }

    /// @emoji 🌉️ `read`/`write`'s `payload` is the same `encode_os_space_payload` blob every
    /// `OsBackbonePort` implementor carries: a `name` byte blob wrapping a nested `pack`+`spr` pair
    /// (`store::encode_document_pack_bytes`, twice). The file/folder storage backends below have no
    /// slot for `name` (they only ever persisted `pack`+`spr`, or before this crate's pack/dsl rollout
    /// bare JSON) — that half of the payload is dropped on write and reconstituted empty on read,
    /// same loss the previous JSON-through-`OsEnvelope` bridge already had (`OsEnvelope` itself has no
    /// `name` field), not a regression.
    impl OsBackbonePort for SpaceBackbonePort {
        fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
            if let Some(kind) = &self.kind {
                match kind {
                    #[cfg(not(target_arch = "wasm32"))]
                    SpacePortKind::File { uri: file_uri, storage, document_id, extension } if uri == file_uri => {
                        let (pack, spr) = if let Some(pack_files) = storage.read_pack(document_id, extension)? {
                            (pack_files.pack, pack_files.spr)
                        } else {
                            match storage.read(document_id, extension)? {
                                Some(text_files) => {
                                    let snapshot = <space::SpaceSnapshot as store::ArtifactDsl>::parse_dsl(&text_files.dsl).map_err(|error| VcsError::Deserialize(error.message))?;
                                    let envelope = store::create_document_envelope::<space::SpaceSnapshot, space::SpaceMutation>(space::S_SPACE_SCHEMA, document_id, snapshot, None);
                                    let pack_files = crate::host::resolve_kernel_future(store::print_document_pack(&envelope))?;
                                    (pack_files.pack, pack_files.spr)
                                }
                                None => return Err(VcsError::Backbone(format!("missing backbone file {uri}"))),
                            }
                        };
                        let inner = crate::host::resolve_kernel_future(store::encode_document_pack_bytes(&pack, &spr));
                        return Ok(crate::host::resolve_kernel_future(store::encode_document_pack_bytes(&[], &inner)));
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    SpacePortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                        let (pack, spr) = storage.read(SPACE_FOLDER_DOCUMENT_ID)?.ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))?;
                        let inner = crate::host::resolve_kernel_future(store::encode_document_pack_bytes(&pack, &spr));
                        return Ok(crate::host::resolve_kernel_future(store::encode_document_pack_bytes(&[], &inner)));
                    }
                    _ => {}
                }
            }
            self.read_via_memory(uri)
        }

        fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
            if let Some(kind) = &self.kind {
                match kind {
                    #[cfg(not(target_arch = "wasm32"))]
                    SpacePortKind::File { uri: file_uri, storage, document_id, extension } if uri == file_uri => {
                        let (pack, spr) = decode_os_space_pack_payload(payload)?;
                        let parsed: store::ParsedDocumentText<space::SpaceSnapshot, space::SpaceMutation> = crate::host::resolve_kernel_future(store::parse_document_pack(&pack, &spr)).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                        let dsl_mirror = store::ArtifactDsl::print_dsl(&parsed.envelope.vcs.initial_snapshot);
                        let pack_files = store::ArtifactPackFiles { pack, spr, ops: String::new() };
                        return storage.write_pack(document_id, extension, &pack_files, &dsl_mirror);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    SpacePortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                        let (pack, spr) = decode_os_space_pack_payload(payload)?;
                        return storage.write(SPACE_FOLDER_DOCUMENT_ID, space::S_SPACE_SCHEMA, &pack, &spr);
                    }
                    _ => {}
                }
            }
            self.write_via_memory(uri, payload)
        }
    }

    /// @emoji 🔓️ Strips `encode_os_space_payload`'s `name` wrapper, returning the inner `pack`+`spr`
    /// pair — the half of the payload `SpaceBackbonePort`'s file/folder storage actually persists.
    #[cfg(not(target_arch = "wasm32"))]
    fn decode_os_space_pack_payload(payload: &[u8]) -> Result<(Vec<u8>, Vec<u8>), VcsError> {
        let (_name, inner) = crate::host::resolve_kernel_future(store::decode_document_pack_bytes(payload))?;
        crate::host::resolve_kernel_future(store::decode_document_pack_bytes(&inner))
    }

    impl SpaceBackbonePort {
        /// @emoji 🌉️ `self.memory` is a plain `store::BackbonePort` (string-typed) fallback for any uri
        /// that isn't this port's own configured file/folder uri (e.g. the space catalog uri) — bridge
        /// bytes↔string via base64, same as the blanket `impl<T: store::BackbonePort> OsBackbonePort`.
        fn read_via_memory(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
            use base64::Engine;
            let text = crate::host::resolve_kernel_future(store::BackbonePort::read(&self.memory, uri))?;
            if text.is_empty() {
                return Ok(Vec::new());
            }
            base64::engine::general_purpose::STANDARD.decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
        }

        fn write_via_memory(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
            use base64::Engine;
            if payload.is_empty() {
                return crate::host::resolve_kernel_future(store::BackbonePort::write(&self.memory, uri, ""));
            }
            crate::host::resolve_kernel_future(store::BackbonePort::write(&self.memory, uri, &base64::engine::general_purpose::STANDARD.encode(payload)))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_folder_space_backbone(folder_path: &str) -> Result<Arc<OsBackbonePorts>, VcsError> {
        Ok(Arc::new(OsBackbonePorts::Space(SpaceBackbonePort::folder(folder_path)?)))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_file_space_backbone(file_path: &str) -> Result<Arc<OsBackbonePorts>, VcsError> {
        Ok(Arc::new(OsBackbonePorts::Space(SpaceBackbonePort::file(file_path)?)))
    }
    // #endregion backbone
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "os-host-full")]
pub mod host_runtime {
    // #region host_runtime
    //! 🧵️ Canonical native document-open sequencing shared by every native host that links this crate
    //! (currently the wgpu shell). Native-only: it depends on `framework/sync`'s `ArtifactHost`, whose
    //! actor is a native-thread (or wasm `spawn_local`) concern — WASI-P2 plugins never see it, and the
    //! browser React shell talks to its own TS twin (`framework/product/os/core/js/🟦️backbone-worker.ts`)
    //! through a different FFI boundary (the WIT program sandbox), not through this Rust module. Keeping
    //! this doc-comment as the single canonical description of the sequence — referenced from both
    //! `os-shell.tsx`'s `openDocument` and `framework/renderer/wgpu/rs/lib.rs` — is how the two stay in
    //! lockstep without a literal shared code path across the Rust/TS boundary.
    //!
    //! ## Canonical open/spawn/effect sequence (mirrored in TS by `os-shell.tsx`'s `openDocument`):
    //! 1. Build a `ArtifactActorConfig{document_id, schema, bindings, watch_external, actor}` for the
    //!    document being opened — either the os/studio document itself, or one app instance's
    //!    {@link crate::instance::OsArtifactRef}.
    //! 2. `ArtifactHost::open(config)` → `ArtifactChannels{cmd_tx, channel_backbone}`.
    //! 3. Attach `channel_backbone` to the document's own store: `store.attach_backbone(Box::new(...))`.
    //!    For a native WASM plugin instance this ALSO means calling `framework/plugin/host`'s
    //!    `WasmPluginRuntime::register_host_backbone(uri, Box::new(channel_backbone))` so the sandboxed
    //!    plugin's `backbone-send`/`backbone-poll` host imports reach the same channel — this crate does
    //!    not link `framework/plugin/host` directly (no existing dependency edge), so the wgpu shell,
    //!    which links both, is the one that actually performs that registration call using the
    //!    {@link OpenedDocument} this module hands back.
    //! 4. `ArtifactHost::subscribe(&document_id)` → `broadcast::Receiver<ArtifactEvent>`; on each event:
    //!    - `RemoteMutations`/`SnapshotReplaced` are already pushed into the store's inbound queue by the actor
    //!      — the caller just needs to call `store.tick()` (step 5) to materialize them.
    //!    - `Presence{peers}` is pushed to the plugin instance as `AppCommand::Presence` (contract-
    //!      freeze §C7.6) — the ONLY plugin ingress for peers; the old `presence:` backbone-URI hack
    //!      and the later `ViewModel.presence_peers_json` JSON-array bridge are both gone entirely.
    //!    - `Status`/`Conflict` surface on the shell's sync-status badge / conflict card.
    //! 5. Every tick/frame: `store.tick()` drains the attached backbone's inbound queue into the store.
    //! 6. On `Effect::SpawnPluginInstance`/`OpenPluginInstance` from an action result: mint (if
    //!    needed) a fresh `OsArtifactRef` (see {@link crate::instance::create_os_artifact_id}), then repeat
    //!    steps 1-5 for that app's own document.
    //! 7. On close: send `ArtifactActorMsg::Detach` (flushes pending operations) via `host.send(id, Detach)`, then
    //!    `ArtifactHost::close(&id)`, then `store.detach_backbone()` /
    //!    `WasmPluginRuntime::deregister_host_backbone(uri)`.

    use crate::instance::OsArtifactRef;
    use crate::store_sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactChannels, ArtifactEvent, ArtifactHost, PersistenceBinding};

    /// @emoji 📌️ The local persistence binding for a folder-backed document (one row per `document_id`
    /// in the folder's `.semio` append-only event log — see `FolderEventLogStorage`).
    pub fn folder_binding(folder_path: std::path::PathBuf) -> PersistenceBinding {
        PersistenceBinding::Folder { path: folder_path }
    }

    /// @emoji ☁️ The semio_hub persistence binding for a document. `surface` is the out-of-band
    /// presence scope (ticket 26/08/16/HUB-SPACES-…, contract §C0) — `None` for non-presence
    /// documents (e.g. the OS config/home documents, which stay folder-only per contract §C3).
    pub fn hub_binding(base_url: impl Into<String>, space_id: impl Into<String>, token: Option<String>, surface: Option<String>) -> PersistenceBinding {
        PersistenceBinding::Hub { base_url: base_url.into(), space_id: space_id.into(), token, surface }
    }

    /// @emoji 🔗️ Builds the `ArtifactActorConfig` to open an app instance's own document, from its
    /// `OsArtifactRef` — step 1 of the canonical sequence.
    pub fn app_artifact_config(document: &OsArtifactRef, bindings: Vec<PersistenceBinding>, actor: &str) -> ArtifactActorConfig {
        ArtifactActorConfig { document_id: document.document_id.clone(), schema: document.schema.clone(), bindings, watch_external: true, actor: actor.to_string() }
    }

    /// @emoji 🧵️ Channels + a fresh event receiver for one opened document — steps 2 and 4 of the
    /// canonical sequence.
    pub struct OpenedDocument {
        pub channels: ArtifactChannels,
        pub events: tokio::sync::broadcast::Receiver<ArtifactEvent>,
    }

    /// @emoji 🚀️ Opens a document on `host` and subscribes to its events in one call (steps 1-2 & 4).
    pub fn open_document(host: &ArtifactHost, document_id: &str, schema: &str, bindings: Vec<PersistenceBinding>, actor: &str) -> OpenedDocument {
        let channels = host.open(ArtifactActorConfig { document_id: document_id.to_string(), schema: schema.to_string(), bindings, watch_external: true, actor: actor.to_string() });
        let events = host.subscribe(document_id);
        OpenedDocument { channels, events }
    }

    /// @emoji ✂️ Detaches and closes a document's actor (step 7's `ArtifactHost` half).
    pub fn close_document(host: &ArtifactHost, document_id: &str) {
        host.send(document_id, ArtifactActorMsg::Detach);
        host.close(document_id);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn test_pool() -> Arc<semio_framework_async::WorkerPool> {
            static POOL: std::sync::OnceLock<Arc<semio_framework_async::WorkerPool>> = std::sync::OnceLock::new();
            POOL.get_or_init(|| Arc::new(semio_framework_async::WorkerPool::new(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 3)))).clone()
        }

        #[test]
        fn opens_a_document_and_subscribes_to_its_events() {
            let host = ArtifactHost::new(test_pool());
            let opened = open_document(&host, "doc-1", "test.schema", vec![], "actor-1");
            drop(opened.events);
            close_document(&host, "doc-1");
        }

        #[test]
        fn app_artifact_config_carries_the_artifact_ref_through() {
            let document = OsArtifactRef { document_id: "doc-2".into(), schema: "draw.document".into() };
            let config = app_artifact_config(&document, vec![], "actor-1");
            assert_eq!(config.document_id, "doc-2");
            assert_eq!(config.schema, "draw.document");
        }
    }
    // #endregion host_runtime
}

#[cfg(feature = "os-host-full")]
pub mod instance {
    // #region instance
    //! 📦️ App instance schemas, parameters, and studio bindings.

    use crate::workflow;
    use semio_framework::ConfigSpec;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::atomic::{AtomicU64, Ordering};

    pub const OS_PARAMETER_PORT_PREFIX: &str = "param.";

    //#region 🔖️Schemas
    /// @emoji 🔗️ Handle to an app's own `framework/sync`-hosted vcs document — the os document never
    /// embeds app content, only this reference (mirrors `framework/sync`'s `ArtifactActorConfig`).
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsArtifactRef {
        pub document_id: String,
        pub schema: String,
    }

    /// @emoji 🆔️ Mints a fresh app document id — uuid-v7 (time-ordered), matching the id shape semio_hub already
    /// uses for its own entities (`framework/product/os/semio_hub/rs/bin.rs`'s `Uuid::now_v7()`).
    pub fn create_os_artifact_id() -> String {
        semio_framework_os_kernel::os_identity::time_ordered_id()
    }

    // 🧷️ `OsAppInstance` is deleted — `workflow::WorkflowNode` (kernel crate) absorbs it entirely;
    // `WorkflowNode.id` IS the app-instance identity now (see the kernel crate's `🔖️InstanceIdentity`
    // region doc). `OsArtifactRef` stays (still used generically by `host_runtime`'s document-open
    // sequence), just no longer nested inside a per-instance record here.

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsInstanceState {
        pub id: u32,
        pub app_id: String,
        pub controller_id: String,
        pub document_json: String,
        pub view_state: semio_framework::ViewModel,
        pub generation: u64,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum OsParameterType {
        Numeric,
        Categorical,
        Toggle,
        Text,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsParameterFieldSpec {
        pub field_path: String,
        pub label: String,
        #[serde(rename = "type")]
        pub parameter_type: OsParameterType,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsParameterFieldBinding {
        pub parameter_id: String,
        pub node_id: String,
        /// 🎯️ Names a `ConfigFieldSpec.key` in the target `node`'s app's declared `ConfigSpec`
        /// (resolved via `registry::os_app_registration(node.plugin_id, node.app_id).config`) — see
        /// `build_configure_config` (overlays the bound parameter's value onto that config field for
        /// an `AppCommand::Configure` payload; the analogous type-check against `ConfigFieldShape`
        /// this field never had its own live caller for lives on the real workflow-graph binding type,
        /// `workflow::validate_workflow_parameter_config_binding`). Historically a JSON pointer into
        /// the node's live document (`apply_parameter_values_to_snapshot`'s still-live overlay,
        /// used only by the media-export path today) — that document-snapshot sense is now
        /// superseded by the config-field sense for anything driving a running app instance.
        pub field_path: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
    #[serde(tag = "type", rename_all = "lowercase")]
    pub enum OsParameter {
        Numeric {
            id: String,
            name: String,
            value: f64,
            #[serde(skip_serializing_if = "Option::is_none")]
            min: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            max: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            step: Option<f64>,
        },
        Categorical {
            id: String,
            name: String,
            value: String,
            options: Vec<String>,
        },
        Toggle {
            id: String,
            name: String,
            value: bool,
        },
        Text {
            id: String,
            name: String,
            value: String,
        },
    }
    //#endregion 🔖️Schemas

    //#region 🔖️Parameters
    static OS_ID: AtomicU64 = AtomicU64::new(0);

    /// @emoji 🆔️ Allocates stable ids for OS studio entities.
    pub fn create_os_id(prefix: &str) -> String {
        let n = OS_ID.fetch_add(1, Ordering::Relaxed) + 1;
        format!("{prefix}-{n}")
    }

    /// @emoji 🎛️ Reads the runtime value from a space parameter definition.
    pub fn os_parameter_value(parameter: &OsParameter) -> Value {
        match parameter {
            OsParameter::Numeric { value, .. } => Value::from(*value),
            OsParameter::Categorical { value, .. } => Value::from(value.clone()),
            OsParameter::Toggle { value, .. } => Value::from(*value),
            OsParameter::Text { value, .. } => Value::from(value.clone()),
        }
    }

    /// @emoji 🎛️ Returns whether a parameter type can drive a bindable field type.
    pub fn os_parameter_types_compatible(left: &OsParameterType, right: &OsParameterType) -> bool {
        left == right
    }

    /// @emoji 🎛️ Creates a default space parameter of the given type.
    pub fn create_default_os_parameter(parameter_type: &OsParameterType, name: &str, id: Option<&str>) -> OsParameter {
        let parameter_id = id.map(str::to_string).unwrap_or_else(|| create_os_id("param"));
        match parameter_type {
            OsParameterType::Numeric => OsParameter::Numeric { id: parameter_id, name: name.into(), value: 0.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
            OsParameterType::Categorical => OsParameter::Categorical { id: parameter_id, name: name.into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] },
            OsParameterType::Toggle => OsParameter::Toggle { id: parameter_id, name: name.into(), value: false },
            OsParameterType::Text => OsParameter::Text { id: parameter_id, name: name.into(), value: String::new() },
        }
    }

    fn clamp_numeric_value(value: f64, min: Option<f64>, max: Option<f64>, step: Option<f64>) -> f64 {
        let mut next = value;
        if let Some(min) = min.filter(|v| v.is_finite()) {
            next = next.max(min);
        }
        if let Some(max) = max.filter(|v| v.is_finite()) {
            next = next.min(max);
        }
        if let Some(step) = step.filter(|v| v.is_finite() && *v > 0.0) {
            let anchor = min.filter(|v| v.is_finite()).unwrap_or(0.0);
            next = anchor + ((next - anchor) / step).round() * step;
            if let Some(min) = min.filter(|v| v.is_finite()) {
                next = next.max(min);
            }
            if let Some(max) = max.filter(|v| v.is_finite()) {
                next = next.min(max);
            }
        }
        next
    }

    /// @emoji 🎛️ Applies a partial patch to a space parameter, enforcing type constraints.
    pub fn patch_os_parameter(parameter: &OsParameter, patch: &Value) -> OsParameter {
        let name = patch.get("name").and_then(|v| v.as_str()).map(str::to_string).unwrap_or_else(|| parameter_name(parameter));
        let patch_type = patch.get("type").and_then(|v| v.as_str());
        let use_numeric = patch_type == Some("numeric") || (patch_type.is_none() && matches!(parameter, OsParameter::Numeric { .. }));
        if use_numeric {
            let current = match parameter {
                OsParameter::Numeric { .. } => parameter.clone(),
                _ => create_default_os_parameter(&OsParameterType::Numeric, &name, Some(parameter_id(parameter))),
            };
            if let OsParameter::Numeric { id, min: current_min, max: current_max, step: current_step, value: current_value, .. } = current {
                let min = patch.get("min").and_then(|v| v.as_f64()).or(current_min);
                let max = patch.get("max").and_then(|v| v.as_f64()).or(current_max);
                let step = patch.get("step").and_then(|v| v.as_f64()).or(current_step);
                let raw_value = patch.get("value").and_then(|v| v.as_f64()).unwrap_or(current_value);
                return OsParameter::Numeric { id, name, min, max, step, value: clamp_numeric_value(raw_value, min, max, step) };
            }
        }
        let use_categorical = patch_type == Some("categorical") || (patch_type.is_none() && matches!(parameter, OsParameter::Categorical { .. }));
        if use_categorical {
            let current = match parameter {
                OsParameter::Categorical { .. } => parameter.clone(),
                _ => create_default_os_parameter(&OsParameterType::Categorical, &name, Some(parameter_id(parameter))),
            };
            if let OsParameter::Categorical { id, value: current_value, options: current_options, .. } = current {
                let options = patch.get("options").and_then(|v| v.as_array()).map(|entries| entries.iter().filter_map(|entry| entry.as_str().map(str::to_string)).collect::<Vec<_>>()).unwrap_or(current_options);
                let unique_options = if options.is_empty() { vec!["Option A".into()] } else { options };
                let value = patch
                    .get("value")
                    .and_then(|v| v.as_str())
                    .filter(|v| unique_options.iter().any(|option| option == *v))
                    .map(str::to_string)
                    .or_else(|| unique_options.iter().find(|option| **option == current_value).cloned())
                    .unwrap_or_else(|| unique_options[0].clone());
                return OsParameter::Categorical { id, name, options: unique_options, value };
            }
        }
        if patch_type == Some("toggle") || (patch_type.is_none() && matches!(parameter, OsParameter::Toggle { .. })) {
            let current = match parameter {
                OsParameter::Toggle { .. } => parameter.clone(),
                _ => create_default_os_parameter(&OsParameterType::Toggle, &name, Some(parameter_id(parameter))),
            };
            if let OsParameter::Toggle { id, value: current_value, .. } = current {
                let value = patch.get("value").and_then(|v| v.as_bool()).unwrap_or(current_value);
                return OsParameter::Toggle { id, name, value };
            }
        }
        let current = match parameter {
            OsParameter::Text { .. } => parameter.clone(),
            _ => create_default_os_parameter(&OsParameterType::Text, &name, Some(parameter_id(parameter))),
        };
        if let OsParameter::Text { id, value: current_value, .. } = current {
            let value = patch.get("value").and_then(|v| v.as_str()).map(str::to_string).unwrap_or(current_value);
            return OsParameter::Text { id, name, value };
        }
        parameter.clone()
    }

    fn parameter_id(parameter: &OsParameter) -> &str {
        match parameter {
            OsParameter::Numeric { id, .. } | OsParameter::Categorical { id, .. } | OsParameter::Toggle { id, .. } | OsParameter::Text { id, .. } => id,
        }
    }

    fn parameter_name(parameter: &OsParameter) -> String {
        match parameter {
            OsParameter::Numeric { name, .. } | OsParameter::Categorical { name, .. } | OsParameter::Toggle { name, .. } | OsParameter::Text { name, .. } => name.clone(),
        }
    }

    fn json_pointer_segments(pointer: &str) -> Vec<String> {
        if let Some(rest) = pointer.strip_prefix('/') {
            rest.split('/').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
        } else {
            pointer.split('.').filter(|segment| !segment.is_empty()).map(str::to_string).collect()
        }
    }

    /// @emoji 🎛️ Deep-sets a JSON-pointer path on a plain object snapshot.
    pub fn set_json_pointer_value(root: &mut Value, pointer: &str, value: Value) {
        let segments = json_pointer_segments(pointer);
        if segments.is_empty() {
            return;
        }
        let mut current = root;
        for segment in &segments[..segments.len() - 1] {
            if !current.is_object() {
                *current = Value::Object(Default::default());
            }
            // infallible: the branch above just forced `current` to `Value::Object(_)` when it wasn't already one.
            let object = current.as_object_mut().expect("current is always an object here");
            let entry = object.entry(segment.clone()).or_insert_with(|| Value::Object(Default::default()));
            if !entry.is_object() {
                *entry = Value::Object(Default::default());
            }
            current = entry;
        }
        if let Some(object) = current.as_object_mut() {
            object.insert(segments.last().cloned().unwrap_or_default(), value);
        }
    }

    /// @emoji 🎛️ Applies bound space parameter values onto an app snapshot via JSON pointers. 🩹️
    /// Pre-`ConfigSpec` document-snapshot overlay, kept for its one remaining live caller
    /// (`app_instance_document_patches_for_binding`, the media-export path's synthetic-document seed)
    /// — `field_path` here is still read as a JSON pointer into that bare document, distinct from the
    /// `ConfigFieldSpec.key` sense `build_configure_config` gives it for driving a running app
    /// instance's config (see `OsParameterFieldBinding::field_path`'s doc).
    pub fn apply_parameter_values_to_snapshot(snapshot: Value, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], node_id: &str) -> Value {
        let node_bindings: Vec<_> = bindings.iter().filter(|binding| binding.node_id == node_id).collect();
        if node_bindings.is_empty() {
            return snapshot;
        }
        let mut clone = snapshot;
        for binding in node_bindings {
            let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
                continue;
            };
            set_json_pointer_value(&mut clone, &binding.field_path, os_parameter_value(parameter));
        }
        clone
    }

    // 🪦️ `validate_parameter_config_binding` (this `instance` module's own copy, over
    // `OsParameterFieldBinding`/`OsParameterType`) DELETED
    // (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` M1): its only callers
    // repo-wide were its own 3 unit tests below (also deleted) — dead code, superseded by the
    // structurally identical, actually-live `workflow::validate_workflow_parameter_config_binding`
    // (called from `reconcile_workflow_snapshot`'s `ParameterBindingValidation` region against the
    // real `WorkflowParameterBinding`/`WorkflowParameterType`/`workflow`-graph types, not this
    // module's parallel `Os*` vocabulary).

    trait OsParameterId {
        fn id(&self) -> &str;
    }

    impl OsParameterId for OsParameter {
        fn id(&self) -> &str {
            parameter_id(self)
        }
    }

    /// @emoji 🎛️ Resolves bound parameter values for a workflow node as a field-path map.
    pub fn resolve_parameter_values_for_instance(bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], node_id: &str) -> HashMap<String, Value> {
        let mut values = HashMap::new();
        for binding in bindings.iter().filter(|entry| entry.node_id == node_id) {
            let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
                continue;
            };
            values.insert(binding.field_path.clone(), os_parameter_value(parameter));
        }
        values
    }

    /// @emoji 🎛️ Builds the workflow input port id for a bound space parameter.
    pub fn parameter_port_id(node_id: &str, parameter_id: &str) -> String {
        media_port_id_for_spec(node_id, &format!("{OS_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
    }

    /// @emoji 🎛️ Returns whether a media port id denotes a space parameter input channel.
    pub fn is_parameter_port_id(port_id: &str) -> bool {
        media_port_spec_id(port_id).map(|spec_id| spec_id.starts_with(OS_PARAMETER_PORT_PREFIX)).unwrap_or(false)
    }

    /// @emoji 🎛️ Extracts the space parameter id from a parameter input port id.
    pub fn parameter_id_from_port_id(port_id: &str) -> Option<String> {
        let spec_id = media_port_spec_id(port_id)?;
        spec_id.strip_prefix(OS_PARAMETER_PORT_PREFIX).map(str::to_string)
    }

    pub fn media_port_id_for_spec(instance_id: &str, spec_id: &str, direction: &str) -> String {
        format!("{instance_id}:{spec_id}:{direction}")
    }

    pub fn media_port_spec_id(port_id: &str) -> Option<String> {
        let parts: Vec<_> = port_id.split(':').collect();
        if parts.len() < 3 {
            return None;
        }
        Some(parts[1..parts.len() - 1].join(":"))
    }
    //#endregion 🔖️Parameters

    //#region 🔖️Materialize
    use std::sync::{Mutex, OnceLock};

    static OS_FIXTURE_JSON: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();

    fn os_fixture_json_registry() -> &'static Mutex<HashMap<String, String>> {
        OS_FIXTURE_JSON.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// @emoji 📎️ Registers bundled fixture JSON for `payloadRef` materialization.
    pub fn register_os_fixture_json(slug: &str, json: &str) {
        os_fixture_json_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(slug.into(), json.into());
    }

    /// @emoji 📎️ Looks up bundled fixture JSON by slug — the seed content for a freshly spawned app
    /// document. Replaces the old `OsSourceDocument.payloadRef = "fixture:…"` resolution: since app
    /// content no longer embeds in the os document, seeding now happens once, host-side, at
    /// {@link OsArtifactRef} creation time (see `host_runtime`), not on every materialize/read.
    pub fn os_fixture_json(slug: &str) -> Option<String> {
        os_fixture_json_registry().lock().ok().and_then(|registry| registry.get(slug).cloned())
    }

    /// @emoji 🎚️ Default config value seeded from `config_spec.fields[].default` — what a freshly
    /// spawned instance's config resolves to before any explicit `instance.config`/binding overlay.
    fn config_spec_default_value(config_spec: &ConfigSpec) -> Value {
        let mut defaults = serde_json::Map::new();
        for field in &config_spec.fields {
            if let Some(default) = &field.default {
                let json_default = Value::from(default.clone());
                defaults.insert(field.key.clone(), json_default);
            }
        }
        Value::Object(defaults)
    }

    /// @emoji 🧩️ Builds the dynamic config value for an `AppCommand::Configure` payload: starts from the
    /// app's own `ConfigSpec` defaults, then overlays every parameter bound to one of `config_spec`'s
    /// fields with that parameter's current value — the config-driving counterpart to
    /// `apply_parameter_values_to_snapshot`'s document-JSON-pointer overlay (see
    /// `OsParameterFieldBinding::field_path`'s doc for how the two diverge). Callers (the
    /// renderer/headless-runner drivers dispatching `AppCommand::Configure`, both out of this crate's
    /// scope) `store::pack_rt::encode_wire_value` the result themselves — this function only builds the
    /// value, it never sends anything over a channel.
    ///
    /// 🚧️ WP-1.B ripple: this used to start from `OsAppInstance::config` (an inline `DslValue` on the
    /// deleted per-instance record); config now lives on the node's own config artifact
    /// (`WorkflowNode::config_ref`), which this work package does not wire up (out of scope — see the
    /// master plan's "Config on the node" wave). Always starts from `config_spec`'s own defaults until
    /// that lands.
    pub fn build_configure_config(node_id: &str, parameters: &[OsParameter], bindings: &[OsParameterFieldBinding], config_spec: &ConfigSpec) -> dsl::DslValue {
        let mut config = dsl::DslValue::from(&config_spec_default_value(config_spec));
        let entries = match &mut config {
            dsl::DslValue::Object(entries) => entries,
            _ => {
                config = dsl::DslValue::Object(vec![]);
                match &mut config {
                    dsl::DslValue::Object(entries) => entries,
                    _ => unreachable!("config object branch"),
                }
            }
        };
        for binding in bindings.iter().filter(|binding| binding.node_id == node_id) {
            let Some(field) = config_spec.fields.iter().find(|field| field.key == binding.field_path) else {
                continue;
            };
            let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
                continue;
            };
            let value = dsl::DslValue::from(&os_parameter_value(parameter));
            if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == &field.key) {
                *slot = value;
            } else {
                entries.push((field.key.clone(), value));
            }
        }
        config
    }

    /// @emoji 🧩️ Overlays bound parameter values onto an app instance's current document snapshot.
    /// Content itself lives in the app's own `framework/sync`-hosted document (referenced by
    /// {@link OsArtifactRef}, read host-side and passed in as `current_document_json`) — this function
    /// no longer resolves embedded/upstream source documents; that concept was deleted with
    /// `OsSourceDocument`. Cross-instance ("upstream") dataflow through workflow edges is deferred
    /// (see `host_runtime` doc-comment) to a follow-up that reads the upstream app's live document.
    pub fn materialize_os_app_instance_document_json(current_document_json: &str, node_id: &str, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter]) -> String {
        let snapshot: Value = serde_json::from_str(current_document_json).unwrap_or_else(|_| json!({}));
        let with_params = apply_parameter_values_to_snapshot(snapshot, bindings, parameters, node_id);
        serde_json::to_string(&with_params).unwrap_or_else(|_| "{}".into())
    }

    /// @emoji 🔀️ Host-side hook for the common case: when a bound parameter's value changes, computes the
    /// patched document JSON for every app instance with a field bound to it, keyed by document id — the
    /// host dispatches each as a snapshot replace into that app's own document store (e.g. via the program
    /// WIT boundary's `load-app-document`, or `framework/sync`'s document actor once the app is wired onto
    /// `ArtifactHost`). This covers the "common/simple case" per the JSON-pointer overlay convention
    /// {@link apply_parameter_values_to_snapshot} already established — a true typed operation into the bound
    /// app's own `Mutation` vocabulary requires that app's real (non-opaque) Mutation type and is left to each app's
    /// own `ArtifactApp` migration (WS-F); until then this snapshot-replace path is the host's only lever.
    pub fn app_instance_document_patches_for_binding(
        parameter_id: &str,
        nodes: &[workflow::WorkflowNode],
        bindings: &[OsParameterFieldBinding],
        parameters: &[OsParameter],
        current_document_json: impl Fn(&str) -> Option<String>,
    ) -> Vec<(String, String)> {
        let bound_node_ids: HashSet<String> = bindings.iter().filter(|binding| binding.parameter_id == parameter_id).map(|binding| binding.node_id.clone()).collect();
        nodes
            .iter()
            .filter(|node| bound_node_ids.contains(&node.id))
            .filter_map(|node| {
                let current_json = current_document_json(&node.artifact_ref)?;
                let patched = materialize_os_app_instance_document_json(&current_json, &node.id, bindings, parameters);
                Some((node.artifact_ref.clone(), patched))
            })
            .collect()
    }
    //#endregion 🔖️Materialize

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn patches_numeric_parameter_with_constraints() {
            let parameter = create_default_os_parameter(&OsParameterType::Numeric, "Zoom", None);
            let patched = patch_os_parameter(&parameter, &serde_json::json!({ "value": 12.0, "max": 10.0 }));
            match patched {
                OsParameter::Numeric { value, .. } => assert_eq!(value, 10.0),
                _ => panic!("expected numeric"),
            }
        }

        #[test]
        fn applies_json_pointer_parameter_overrides() {
            let snapshot = serde_json::json!({ "brushSize": 8 });
            let overridden = apply_parameter_values_to_snapshot(
                snapshot,
                &[OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "/brushSize".into() }],
                &[OsParameter::Numeric { id: "p1".into(), name: "Brush".into(), value: 42.0, min: None, max: None, step: None }],
                "i1",
            );
            assert_eq!(overridden["brushSize"], 42.0);
        }

        #[test]
        fn resolves_fixture_json_by_slug() {
            register_os_fixture_json("🖍️semio.draw.json", r#"{"schema":"draw.document","id":"semio"}"#);
            let json = os_fixture_json("🖍️semio.draw.json").expect("registered fixture");
            let parsed: Value = serde_json::from_str(&json).expect("json");
            assert_eq!(parsed["schema"], "draw.document");
            assert_eq!(parsed["id"], "semio");
        }

        #[test]
        fn materializes_instance_documents_with_parameter_overrides() {
            let json = materialize_os_app_instance_document_json(r#"{"schema":"draw.document","id":"semio"}"#, "app-draw-1", &[], &[]);
            let parsed: Value = serde_json::from_str(&json).expect("json");
            assert_eq!(parsed["schema"], "draw.document");
            assert_eq!(parsed["id"], "semio");
        }

        fn sample_config_spec() -> ConfigSpec {
            ConfigSpec {
                fields: vec![
                    semio_framework::ConfigFieldSpec {
                        key: "zoom".into(),
                        label: "Zoom".into(),
                        shape: semio_framework::ConfigFieldShape::Number { min: None, max: None, step: None },
                        default: Some(dsl::DslValue::from(&serde_json::json!(1.0))),
                    },
                    semio_framework::ConfigFieldSpec {
                        key: "mode".into(),
                        label: "Mode".into(),
                        shape: semio_framework::ConfigFieldShape::Select { options: vec!["A".into(), "B".into()] },
                        default: Some(dsl::DslValue::from(&serde_json::json!("A"))),
                    },
                    semio_framework::ConfigFieldSpec { key: "flag".into(), label: "Flag".into(), shape: semio_framework::ConfigFieldShape::Toggle, default: None },
                    semio_framework::ConfigFieldSpec { key: "label".into(), label: "Label".into(), shape: semio_framework::ConfigFieldShape::Text, default: None },
                ],
            }
        }

        // 🪦️ `validates_matching_parameter_config_bindings`/`rejects_mismatched_parameter_config_bindings`/
        // `rejects_parameter_config_binding_to_unknown_field` DELETED alongside the dead
        // `validate_parameter_config_binding` they exclusively exercised (see that deletion's note
        // above) — the type-check logic they asserted is duplicated verbatim (per its own doc
        // comment, "ported from os-core's `validate_parameter_config_binding`") in the live
        // `workflow::validate_workflow_parameter_config_binding`, which these tests never called.

        #[test]
        fn build_configure_config_starts_from_config_spec_defaults() {
            let config_spec = sample_config_spec();
            let config = build_configure_config("i1", &[], &[], &config_spec);
            let config: Value = Value::from(config);
            assert_eq!(config["zoom"], 1.0);
            assert_eq!(config["mode"], "A");
        }

        #[test]
        fn build_configure_config_overlays_bound_parameter_values() {
            let config_spec = sample_config_spec();
            let parameters = vec![OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 42.0, min: None, max: None, step: None }];
            let bindings = vec![OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "zoom".into() }];
            let config = build_configure_config("i1", &parameters, &bindings, &config_spec);
            let config: Value = Value::from(config);
            assert_eq!(config["zoom"], 42.0);
            assert_eq!(config["mode"], "A");
        }
    }
    //#endregion 🧪️Tests
    // #endregion instance
}

pub mod media_export_raster {
    // #region media_export_raster
    //! 🖼️ SVG/DWG media helpers: SVG builders and DWG-to-SVG stay target-neutral; rasterization and
    //! SVG-path flattening use the native renderer tier and report unavailable on wasm32-wasip2.

    #[cfg(not(feature = "os-host-full"))]
    use std::sync::{LazyLock, Mutex};

    //#region 🔖️MediaRegistryRegistryStubs
    // 🧬️ Default-feature builds have no `workflow` module, so keep a local registry. With
    // `os-host-full`, re-export the workflow registry so `register_*` and `export_os_app_instance_media`
    // share one OnceLock (stubs previously shadowed the real handlers at crate root). Keyed on string
    // format kind ids, not the legacy format enum (retired — ticket 26/08/11/
    // SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
    #[cfg(feature = "os-host-full")]
    pub use crate::workflow::{export_os_app_instance_media_kind, import_os_app_instance_media_kind, register_os_media_export_handler_kind, register_os_media_import_handler_kind, OsMediaExportResult};

    #[cfg(not(feature = "os-host-full"))]
    /// 🖼️ Host-local media export result (workflow module gated behind os-host-full).
    #[derive(Clone, Debug, PartialEq)]
    pub struct OsMediaExportResult {
        pub data: String,
        pub mime_type: String,
        pub file_name: String,
        pub encoding: Option<String>,
    }

    #[cfg(not(feature = "os-host-full"))]
    impl OsMediaExportResult {
        /// 📤️ Build an export result from raw bytes + stdio format kind id.
        pub fn from_format_kind_bytes(bytes: Vec<u8>, format_artifact_kind: &str, file_stem: &str) -> Result<Self, String> {
            let entry = semio_framework::format_descriptor(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
            let mime_type = entry.mimes.first().cloned().ok_or_else(|| format!("stdio format kind `{format_artifact_kind}` has no MIME claim"))?;
            let extension = entry.extensions.first().ok_or_else(|| format!("stdio format kind `{format_artifact_kind}` has no extension claim"))?;
            let data = if entry.is_binary { base64::engine::general_purpose::STANDARD.encode(&bytes) } else { String::from_utf8(bytes).map_err(|error| error.to_string())? };
            Ok(Self { data, mime_type, file_name: format!("{file_stem}{extension}"), encoding: if entry.is_binary { Some("base64".into()) } else { None } })
        }
    }

    /// 🗂️ Build a file-picker `accept` filter from stdio format kind ids (`dwg` / `stdio.dwg`).
    pub fn media_accept_filter_kinds(format_artifact_kinds: &[&str]) -> Result<String, semio_framework::FormatRegistryError> {
        semio_framework::format_accept_filter(format_artifact_kinds)
    }

    #[cfg(not(feature = "os-host-full"))]
    type OsMediaExportHandler = Box<dyn Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync>;
    #[cfg(not(feature = "os-host-full"))]
    type OsMediaImportHandler = Box<dyn Fn(&[u8]) -> Result<Value, String> + Send + Sync>;

    #[cfg(not(feature = "os-host-full"))]
    static OS_MEDIA_EXPORT_HANDLERS: LazyLock<Mutex<std::collections::HashMap<(String, String), OsMediaExportHandler>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));
    #[cfg(not(feature = "os-host-full"))]
    static OS_MEDIA_IMPORT_HANDLERS: LazyLock<Mutex<std::collections::HashMap<(String, String), OsMediaImportHandler>>> = LazyLock::new(|| Mutex::new(std::collections::HashMap::new()));

    #[cfg(not(feature = "os-host-full"))]
    pub fn register_os_media_export_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static) {
        OS_MEDIA_EXPORT_HANDLERS.lock().expect("media export registry").insert((artifact_kind.to_string(), format_artifact_kind.to_string()), Box::new(handler));
    }

    #[cfg(not(feature = "os-host-full"))]
    pub fn register_os_media_import_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&[u8]) -> Result<Value, String> + Send + Sync + 'static) {
        OS_MEDIA_IMPORT_HANDLERS.lock().expect("media import registry").insert((artifact_kind.to_string(), format_artifact_kind.to_string()), Box::new(handler));
    }
    //#endregion 🔖️MediaRegistryRegistryStubs
    use base64::Engine;
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use png::{BitDepth, ColorType, Encoder};
    /// 🌉️ ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS G2b: the DWG
    /// structural codec relocated out of `semio_framework` (G2) into stdio's `🖊️dwg` ac1024 subset;
    /// `semio-framework-os` may depend on `semio-s-plugin-stdio` (verified: not in stdio's own
    /// dependency closure), the direction this ticket's other framework-product crates already use.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    use semio_s_plugin_stdio::artifacts::dwg::{DwgColor, DwgEntity};
    use semio_s_plugin_stdio::artifacts::dwg::{DwgDrawing, DwgGeometry};
    use serde_json::Value;

    /// @emoji 🖼️ Rasterizes SVG markup to a base64-encoded PNG payload.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn rasterize_svg_to_png_base64(svg: &str, width: u32, height: u32) -> Result<String, String> {
        let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
        let size = tree.size();
        let render_w = if width > 0 { width } else { size.width().ceil().max(1.0) as u32 };
        let render_h = if height > 0 { height } else { size.height().ceil().max(1.0) as u32 };
        let mut pixmap = tiny_skia::Pixmap::new(render_w, render_h).ok_or_else(|| "invalid raster dimensions".to_string())?;
        let scale_x = render_w as f32 / size.width().max(1.0);
        let scale_y = render_h as f32 / size.height().max(1.0);
        resvg::render(&tree, tiny_skia::Transform::from_scale(scale_x, scale_y), &mut pixmap.as_mut());
        let png_bytes = encode_rgba_png(pixmap.data(), pixmap.width(), pixmap.height())?;
        Ok(base64::engine::general_purpose::STANDARD.encode(png_bytes))
    }

    /// @emoji 🖼️ Preserves the raster-export API where the shipped guest has no native renderer tier.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn rasterize_svg_to_png_base64(_: &str, _: u32, _: u32) -> Result<String, String> {
        Err("SVG rasterization requires the native semio-framework-os host".into())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn encode_rgba_png(pixels: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut bytes = Vec::new();
        {
            let mut encoder = Encoder::new(&mut bytes, width, height);
            encoder.set_color(ColorType::Rgba);
            encoder.set_depth(BitDepth::Eight);
            let mut writer = encoder.write_header().map_err(|error| error.to_string())?;
            writer.write_image_data(pixels).map_err(|error| error.to_string())?;
        }
        Ok(bytes)
    }

    /// @emoji 📐️ Flattens SVG markup into a DWG drawing by walking usvg path geometry into layered polylines.
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    pub fn svg_to_dwg_bytes(svg: &str) -> Result<Vec<u8>, String> {
        let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        let height = tree.size().height() as f64;
        collect_svg_children(tree.root().children(), &mut drawing, layer, height);
        semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing)
    }

    /// @emoji 📐️ Preserves the SVG-to-DWG API where the shipped guest has no native parser tier.
    #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
    pub fn svg_to_dwg_bytes(_: &str) -> Result<Vec<u8>, String> {
        Err("SVG-to-DWG conversion requires the native semio-framework-os host".into())
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn collect_svg_children(nodes: &[usvg::Node], drawing: &mut DwgDrawing, layer: usize, height: f64) {
        for node in nodes {
            match node {
                usvg::Node::Group(group) => {
                    let id = node.id();
                    let group_layer = if id.is_empty() { layer } else { drawing.ensure_layer(id) };
                    collect_svg_children(group.children(), drawing, group_layer, height);
                }
                usvg::Node::Path(path) => collect_svg_path(path, drawing, layer, height),
                _ => {}
            }
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn collect_svg_path(path: &usvg::Path, drawing: &mut DwgDrawing, layer: usize, height: f64) {
        let transform = path.abs_transform();
        let mut vertices: Vec<[f64; 2]> = Vec::new();
        let mut closed = false;
        for segment in path.data().segments() {
            match segment {
                usvg::tiny_skia_path::PathSegment::MoveTo(p) => {
                    flush_svg_polyline(drawing, layer, &mut vertices, &mut closed);
                    vertices.push(transformed_svg_point(transform, p, height));
                }
                usvg::tiny_skia_path::PathSegment::LineTo(p) => {
                    vertices.push(transformed_svg_point(transform, p, height));
                }
                usvg::tiny_skia_path::PathSegment::QuadTo(c, p) => {
                    flatten_quad_into(&mut vertices, transform, c, p, height);
                }
                usvg::tiny_skia_path::PathSegment::CubicTo(c1, c2, p) => {
                    flatten_cubic_into(&mut vertices, transform, c1, c2, p, height);
                }
                usvg::tiny_skia_path::PathSegment::Close => {
                    closed = true;
                }
            }
        }
        flush_svg_polyline(drawing, layer, &mut vertices, &mut closed);
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn transformed_svg_point(transform: usvg::Transform, point: usvg::tiny_skia_path::Point, height: f64) -> [f64; 2] {
        let mut p = point;
        transform.map_point(&mut p);
        [p.x as f64, height - p.y as f64]
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn flatten_quad_into(vertices: &mut Vec<[f64; 2]>, transform: usvg::Transform, ctrl: usvg::tiny_skia_path::Point, to: usvg::tiny_skia_path::Point, height: f64) {
        let from = vertices.last().copied().unwrap_or([0.0, 0.0]);
        let ctrl_p = transformed_svg_point(transform, ctrl, height);
        let to_p = transformed_svg_point(transform, to, height);
        const STEPS: usize = 12;
        for step in 1..=STEPS {
            let t = step as f64 / STEPS as f64;
            let mt = 1.0 - t;
            vertices.push([mt * mt * from[0] + 2.0 * mt * t * ctrl_p[0] + t * t * to_p[0], mt * mt * from[1] + 2.0 * mt * t * ctrl_p[1] + t * t * to_p[1]]);
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn flatten_cubic_into(vertices: &mut Vec<[f64; 2]>, transform: usvg::Transform, c1: usvg::tiny_skia_path::Point, c2: usvg::tiny_skia_path::Point, to: usvg::tiny_skia_path::Point, height: f64) {
        let from = vertices.last().copied().unwrap_or([0.0, 0.0]);
        let c1p = transformed_svg_point(transform, c1, height);
        let c2p = transformed_svg_point(transform, c2, height);
        let to_p = transformed_svg_point(transform, to, height);
        const STEPS: usize = 16;
        for step in 1..=STEPS {
            let t = step as f64 / STEPS as f64;
            let mt = 1.0 - t;
            vertices.push([mt * mt * mt * from[0] + 3.0 * mt * mt * t * c1p[0] + 3.0 * mt * t * t * c2p[0] + t * t * t * to_p[0], mt * mt * mt * from[1] + 3.0 * mt * mt * t * c1p[1] + 3.0 * mt * t * t * c2p[1] + t * t * t * to_p[1]]);
        }
    }

    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    fn flush_svg_polyline(drawing: &mut DwgDrawing, layer: usize, vertices: &mut Vec<[f64; 2]>, closed: &mut bool) {
        if vertices.len() > 1 {
            let count = vertices.len();
            drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed: *closed, elevation: 0.0, vertices: std::mem::take(vertices), bulges: vec![0.0; count] } });
        } else {
            vertices.clear();
        }
        *closed = false;
    }

    /// @emoji 📐️ Renders a DWG drawing back to flat SVG markup (lines and closed polygons), for the raster import path.
    pub fn dwg_drawing_to_svg(drawing: &DwgDrawing) -> Result<(String, u32, u32), String> {
        let width = (drawing.extmax[0] - drawing.extmin[0]).max(1.0).ceil() as u32;
        let height = (drawing.extmax[1] - drawing.extmin[1]).max(1.0).ceil() as u32;
        let mut paths = String::new();
        for entity in &drawing.entities {
            if let DwgGeometry::LwPolyline { vertices, closed, .. } = &entity.geometry {
                if vertices.is_empty() {
                    continue;
                }
                let mut d = format!("M {} {}", vertices[0][0] - drawing.extmin[0], drawing.extmax[1] - vertices[0][1]);
                for v in &vertices[1..] {
                    d.push_str(&format!(" L {} {}", v[0] - drawing.extmin[0], drawing.extmax[1] - v[1]));
                }
                if *closed {
                    d.push_str(" Z");
                }
                paths.push_str(&format!("<path d=\"{d}\" fill=\"none\" stroke=\"black\" stroke-width=\"1\"/>"));
            }
        }
        let svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">{paths}</svg>");
        Ok((svg, width, height))
    }

    /// @emoji 🧷️ Signature every 2D-resource-kind SVG document renderer must match to register via {@link register_2d_export_handlers}.
    pub type Svg2dDocumentRenderer = fn(&Value) -> Result<(String, u32, u32), String>;

    /// @emoji 💾️ Registers SVG, PNG, and DWG export handlers for one 2D resource kind.
    pub fn register_2d_export_handlers(artifact_kind: &'static str, file_stem: &'static str, document_to_svg: Svg2dDocumentRenderer) {
        register_os_media_export_handler_kind(artifact_kind, "svg", move |doc| {
            let (svg, _width, _height) = document_to_svg(doc)?;
            Ok(OsMediaExportResult { data: svg, mime_type: "image/svg+xml".into(), file_name: format!("{file_stem}.svg"), encoding: None })
        });
        register_os_media_export_handler_kind(artifact_kind, "png", move |doc| {
            let (svg, width, height) = document_to_svg(doc)?;
            let data = rasterize_svg_to_png_base64(&svg, width, height)?;
            Ok(OsMediaExportResult { data, mime_type: "image/png".into(), file_name: format!("{file_stem}.png"), encoding: Some("base64".into()) })
        });
        register_os_media_export_handler_kind(artifact_kind, "dwg", move |doc| {
            let (svg, _width, _height) = document_to_svg(doc)?;
            let bytes = svg_to_dwg_bytes(&svg)?;
            Ok(OsMediaExportResult { data: base64::engine::general_purpose::STANDARD.encode(bytes), mime_type: "image/vnd.dwg".into(), file_name: format!("{file_stem}.dwg"), encoding: Some("base64".into()) })
        });
    }

    // 🚪️ `register_dwg_import_handler` DELETED (ticket
    // 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1): zero remaining
    // callers repo-wide (census re-run after migrating cad/gismap/puzzle2d/space off it — each now
    // resolves DWG import through its own artifact `io_registry::entries()` `ComposerEntry` via
    // `registry_import_media`'s `io_dispatch` path once its `native_dialect_kind` bridging bug and
    // double-`"stdio."`-prefix bug were fixed, both in this same file). `register_mesh_dwg_import_handler`
    // below is a DIFFERENT function (mesh-DWG, not plain DWG) and is NOT one of this wave's five
    // targeted functions -- it stays.

    /// @emoji 🧵️ Registers one `MeshExporter` format (Obj/Glb/Stl/…) for a mesh resource kind; call once per format — `mesh_from_document` bridges the OS workflow's per-document export pipeline down to the format-agnostic `MeshData` the exporter instance actually encodes. DWG stays on `register_mesh_dwg_import_handler`'s sibling below; it is not part of the `MeshExporter` mechanism.
    pub fn register_mesh_exporter(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>, exporter: Box<dyn semio_framework_plugin::MeshExporter>) {
        let format_kind = exporter.format_kind();
        register_os_media_export_handler_kind(artifact_kind, format_kind, move |doc| {
            let descriptor = semio_framework::format_descriptor(format_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown mesh export format kind `{format_kind}`"))?;
            let extension = descriptor.extensions.first().ok_or_else(|| format!("mesh export format kind `{format_kind}` has no extension claim"))?;
            let mime_type = descriptor.mimes.first().cloned().ok_or_else(|| format!("mesh export format kind `{format_kind}` has no MIME claim"))?;
            let mesh = mesh_from_document(doc)?;
            let bytes = exporter.export(&mesh)?;
            let data = if descriptor.is_binary { base64::engine::general_purpose::STANDARD.encode(&bytes) } else { String::from_utf8(bytes).map_err(|error| error.to_string())? };
            Ok(OsMediaExportResult { data, mime_type, file_name: format!("{file_stem}{extension}"), encoding: descriptor.is_binary.then(|| "base64".into()) })
        });
    }

    /// @emoji 🧵️ Registers one `MeshImporter` format (Obj/Glb/Stl/…) for a mesh resource kind; `document_from_mesh` bridges the decoded `MeshData` back into the app's own document shape.
    pub fn register_mesh_importer(artifact_kind: &'static str, document_from_mesh: fn(&semio_framework_plugin::MeshData) -> Result<Value, String>, importer: Box<dyn semio_framework_plugin::MeshImporter>) {
        let format_kind = importer.format_kind();
        register_os_media_import_handler_kind(artifact_kind, format_kind, move |bytes| {
            let mesh = importer.import(bytes)?;
            document_from_mesh(&mesh)
        });
    }

    /// @emoji 📥️ Registers a DWG import handler for one mesh resource kind.
    pub fn register_mesh_dwg_import_handler(artifact_kind: &'static str, document_from_mesh: fn(&semio_framework_plugin::MeshData) -> Result<Value, String>) {
        register_os_media_import_handler_kind(artifact_kind, "dwg", move |bytes| {
            let drawing = semio_s_plugin_stdio::artifacts::dwg::dwg_from_bytes(bytes)?;
            let mesh = semio_s_plugin_stdio::artifacts::dwg::dwg_drawing_to_mesh(&drawing);
            document_from_mesh(&mesh)
        });
    }

    /// @emoji 💾️ Registers a DWG export handler for one mesh resource kind; DWG is not part of the `MeshExporter` mechanism (it flattens a mesh into a DWG drawing, not a mesh codec), so it stays a dedicated registrar alongside `register_mesh_exporter`.
    pub fn register_mesh_dwg_export_handler(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>) {
        register_os_media_export_handler_kind(artifact_kind, "dwg", move |doc| {
            let mesh = mesh_from_document(doc)?;
            let drawing = semio_s_plugin_stdio::artifacts::dwg::mesh_to_dwg_drawing(&mesh);
            let bytes = semio_s_plugin_stdio::artifacts::dwg::dwg_to_bytes(&drawing)?;
            Ok(OsMediaExportResult { data: base64::engine::general_purpose::STANDARD.encode(bytes), mime_type: "image/vnd.dwg".into(), file_name: format!("{file_stem}.dwg"), encoding: Some("base64".into()) })
        });
    }

    #[cfg(all(test, target_arch = "wasm32", target_env = "p2"))]
    mod wasip2_tests {
        use super::{rasterize_svg_to_png_base64, svg_to_dwg_bytes};

        #[test]
        fn native_svg_engines_report_unavailable() {
            assert_eq!(
                rasterize_svg_to_png_base64("<svg/>", 1, 1),
                Err("SVG rasterization requires the native semio-framework-os host".into())
            );
            assert_eq!(
                svg_to_dwg_bytes("<svg/>"),
                Err("SVG-to-DWG conversion requires the native semio-framework-os host".into())
            );
        }
    }

    // 🚪️ `//#region SolidMediaExport` DELETED WHOLESALE (ticket
    // 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1): this used to
    // hold `SolidExporterRegistry`/`SolidImporterRegistry`/`solid_exporters`/`solid_importers`/
    // `solid_registry_key`/`register_solid_exporter`/`register_solid_importer`/`solid_exporter_for`/
    // `export_registered_solid`/`import_registered_solid`, all keyed on
    // `semio_framework_3d::brep::kernel::{SolidExporter, SolidImporter, Brep, GeometryHandle}`.
    // Census (repo-wide, `grep -rn "register_solid_exporter(\|register_solid_importer("`, excluding
    // `🎯️target` and this ticket's own scratch files) found exactly one registrant (cad's
    // `register_host_io`, three formats each) and ZERO production callers of the read side --
    // `export_registered_solid`/`import_registered_solid` were called from nowhere but this file's
    // own self-test below (now also deleted) and cad's now-deleted `solid_exporter_for` assertions.
    // cad's REAL per-solid export/import path (`export_solids_as`/`import_step_object`/
    // `import_obj_object`/`import_stl_object`, `🗿️artifacts/📐️cad/…/🚪️io/🦀️component.rs`) already
    // called the genuine stdio `ArtifactSerializer`/`ArtifactDeserializer` leaves directly
    // (`SemioMeshToObj`, `SemioMeshToStl`, `SemioBrepToStep`, `SemioBrepFromStep`) -- this whole
    // region was dead weight shadowing that real path, not a gap needing a new artifact-io leaf.
    // Deleting it also removes the host's last three references to
    // `semio_framework_3d::brep::kernel::{SolidExporter, SolidImporter, Brep, GeometryHandle}` --
    // see the wave's report for the repo-wide census of who ELSE still depends on that module
    // (several framework/plugin crates do; wave IO1 does not touch it further).
    // #endregion media_export_raster
}

pub mod media_export_simple {
    // #region media_export_simple
    //! 🖼️ Lightweight SVG builders for simple document exports.

    use serde_json::Value;

    fn escape_svg_text(value: &str) -> String {
        value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
    }

    /// @emoji 🖼️ Wraps SVG body markup with explicit dimensions.
    pub fn wrap_svg(width: u32, height: u32, body: &str) -> (String, u32, u32) {
        let svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}">{body}</svg>"#);
        (svg, width, height)
    }

    /// @emoji 🏷️ Builds a title-card SVG from a document JSON value.
    pub fn title_card_svg(value: &Value, label: &str, width: u32, height: u32) -> Result<(String, u32, u32), String> {
        let title = value.get("title").and_then(|entry| entry.as_str()).or_else(|| value.get("id").and_then(|entry| entry.as_str())).unwrap_or(label);
        let body = format!("<rect width=\"100%\" height=\"100%\" fill=\"white\"/><text x=\"32\" y=\"64\" font-size=\"32\" fill=\"#111827\">{}</text>", escape_svg_text(title));
        Ok(wrap_svg(width, height, &body))
    }

    /// @emoji 📄️ Serializes page-like rectangles from a `pages` array.
    pub fn pages_rects_svg(value: &Value, fallback_label: &str) -> Result<(String, u32, u32), String> {
        let pages = value.get("pages").and_then(|entry| entry.as_array()).cloned().unwrap_or_default();
        if pages.is_empty() {
            return title_card_svg(value, fallback_label, 1024, 768);
        }
        let mut max_x = 0.0f64;
        let mut max_y = 0.0f64;
        let mut body = String::new();
        for (index, page) in pages.iter().enumerate() {
            let width = page.get("width").and_then(|entry| entry.as_f64()).unwrap_or(800.0);
            let height = page.get("height").and_then(|entry| entry.as_f64()).unwrap_or(600.0);
            let x = page.get("x").and_then(|entry| entry.as_f64()).unwrap_or((index as f64) * (width + 24.0));
            let y = page.get("y").and_then(|entry| entry.as_f64()).unwrap_or(0.0);
            max_x = max_x.max(x + width);
            max_y = max_y.max(y + height);
            body.push_str(&format!("<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" fill=\"white\" stroke=\"#94a3b8\" stroke-width=\"2\"/>"));
        }
        Ok(wrap_svg(max_x.max(1.0).round() as u32, max_y.max(1.0).round() as u32, &body))
    }

    /// @emoji 🗺️ Serializes point features from common GIS fixture fields.
    pub fn map_points_svg(value: &Value, fallback_label: &str) -> Result<(String, u32, u32), String> {
        let positions = value.get("positions").or_else(|| value.get("points")).and_then(|entry| entry.as_array()).cloned().unwrap_or_default();
        if positions.is_empty() {
            return title_card_svg(value, fallback_label, 1024, 768);
        }
        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;
        for position in &positions {
            let Some(coords) = position.as_array() else { continue };
            let x = coords.first().and_then(|entry| entry.as_f64()).unwrap_or(0.0);
            let y = coords.get(1).and_then(|entry| entry.as_f64()).unwrap_or(0.0);
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        let pad = 32.0;
        let width = ((max_x - min_x) + pad * 2.0).max(256.0).round() as u32;
        let height = ((max_y - min_y) + pad * 2.0).max(256.0).round() as u32;
        let shifted = positions
            .iter()
            .filter_map(|position| position.as_array())
            .map(|coords| {
                let x = coords.first().and_then(|entry| entry.as_f64()).unwrap_or(0.0) - min_x + pad;
                let y = coords.get(1).and_then(|entry| entry.as_f64()).unwrap_or(0.0) - min_y + pad;
                format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"#2563eb\"/>")
            })
            .collect::<Vec<_>>()
            .join("");
        if shifted.is_empty() {
            return title_card_svg(value, fallback_label, 1024, 768);
        }
        Ok(wrap_svg(width, height, &shifted))
    }
    // #endregion media_export_simple
}

#[cfg(feature = "os-host-full")]
pub mod workflow {
    // #region workflow
    //! 🎬️ Workflow, VFS snapshot types, and media export registry.

    // 🧬️ Kernel re-exports — the persisted graph model itself (`Workflow`/`WorkflowNode`/`WorkflowEdge`/
    // `WorkflowMediaPort`/`WorkflowPosition`/`MediaContract`/`WorkflowDelivery`/`WorkflowFixture`/
    // `plan_workflow`/`workflow_node_for_app`/`placeholder_media_contract`/`empty_workflow`) lives in
    // the `semio-framework-os-kernel-workflow` crate (dependency name `workflow`) — see its
    // `🔖️InstanceIdentity` doc. Re-exported here so every existing `crate::workflow::X` call site in
    // this file keeps working unchanged. `workflow::validate_workflow` (dangling-edge + cycle checks
    // only) is re-exported under a different name because this module's own `validate_workflow` (below)
    // wraps it with the contract-renegotiation check that still needs the artifact registry, which only
    // exists at this layer.
    // 🧬️ `WorkflowSnapshot`/`WorkflowMutation`/`WorkflowParameter*`/`WorkflowInput*`/`WorkflowOutputBinding`
    // absorb os-core's dissolved `OsSnapshot`/`OsMutation`/`instance::OsParameter*` (see `## The
    // inversion` in the plan) — re-exported here too so every `crate::workflow::X` call site (and every
    // downstream crate importing via `semio_framework_os::workflow::X`/`semio_framework_os::X`) keeps a
    // single source of truth for the workflow document vocabulary.
    #[cfg(feature = "os-host-full")]
    pub use crate::workflow_kernel::{
        apply_workflow_operation, create_default_workflow_parameter, empty_workflow, empty_workflow_snapshot, media_port_spec_id, patch_workflow_parameter, placeholder_media_contract, plan_workflow, sync_workflow_parameter_ports,
        validate_workflow as kernel_validate_workflow, validate_workflow_parameter_config_binding, validate_workflow_snapshot, workflow_node_for_app, workflow_parameter_id, workflow_parameter_id_from_port_id, workflow_parameter_name,
        workflow_parameter_types_compatible, workflow_parameter_value, MediaContract, Workflow, WorkflowDelivery, WorkflowEdge, WorkflowFixture, WorkflowInput, WorkflowInputBinding, WorkflowMediaPort, WorkflowMutation, WorkflowNode,
        WorkflowOutputBinding, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterPatch, WorkflowParameterType, WorkflowPosition, WorkflowSnapshot, WorkflowValidation, AddInput, AddNode, AddParameter, BindInput,
        BindOutput, BindParameterField, ChangeParameter, ConnectPorts, DisconnectEdge, MoveNode, RemoveInput, RemoveNode, RemoveParameter, RenameNode, UnbindInput, UnbindOutput, UnbindParameterField, UpdateNodePorts,
        S_WORKFLOW_SCHEMA, WORKFLOW_SCHEMA,
    };

    #[cfg(feature = "os-host-full")]
    use crate::instance::create_os_id;
    #[cfg(not(feature = "os-host-full"))]
    fn create_os_id(prefix: &str) -> String {
        format!("{prefix}-stub")
    }
    //#region 🔖️RegistryStubs
    #[cfg(feature = "os-host-full")]
    use crate::registry::{os_app_registration, os_artifact_descriptor, OsArtifactDescriptor};
    #[cfg(not(feature = "os-host-full"))]
    #[derive(Clone, Debug, Default)]
    pub struct OsArtifactDescriptor {
        pub kind: String,
        pub name: String,
        pub source_format: String,
        pub component_kind: String,
        pub dimension: String,
        pub schema: String,
    }
    #[cfg(not(feature = "os-host-full"))]
    fn os_app_registration(_id: &str) -> Option<()> {
        None
    }
    #[cfg(not(feature = "os-host-full"))]
    fn os_artifact_descriptor(_kind: &str) -> Option<OsArtifactDescriptor> {
        None
    }
    //#endregion 🔖️RegistryStubs
    use base64::Engine;
    use semio_framework::{media_types_compatible, ArtifactDialect, MediaCompat, MediaWireFormat};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::{Mutex, OnceLock};

    pub const OS_SPACE_SCHEMA: &str = "os.space";
    pub const OS_WORKFLOW_VFS_ROOT_ID: &str = "os-workflow-root";
    pub const OS_MEDIA_FLOW_MODULE_ID: &str = "os-media";

    //#region 🔖️Workflow
    /// @emoji 🤝️ Negotiates the wire contract for connecting `source_port` (a producer/output) to
    /// `target_port` (a consumer/input). Compatibility itself is decided from the ports' own
    /// `MediaPortSpec.media_type` (always accurate, even for the implicit `document:*` ports whose
    /// `kind_id` is `None`); the artifact-kind registry is only consulted for wire-encoding details
    /// (schema / export / import formats), keyed by `kind_id` when present. `Err` means the connect
    /// must be rejected outright (see the `s::plugin` connect handlers).
    pub fn negotiate_media_contract(source_port: &WorkflowMediaPort, target_port: &WorkflowMediaPort) -> Result<MediaContract, String> {
        let source_descriptor = os_artifact_descriptor(source_port.spec.kind_id.as_deref().unwrap_or_default());
        let target_descriptor = os_artifact_descriptor(target_port.spec.kind_id.as_deref().unwrap_or_default());
        let conversion = match crate::host::resolve_kernel_future(media_types_compatible(&source_port.spec.media_type, &target_port.spec.media_type)) {
            MediaCompat::Direct => None,
            MediaCompat::Convert { from, to } => Some((from, to)),
            MediaCompat::Reject => {
                return Err(format!(
                    "cannot connect `{}` ({:?}/{:?}) to `{}` ({:?}/{:?}): incompatible media types",
                    source_port.spec.id, source_port.spec.media_type.class, source_port.spec.media_type.form, target_port.spec.id, target_port.spec.media_type.class, target_port.spec.media_type.form
                ));
            }
        };
        let wire = negotiate_wire_format(&source_descriptor, &target_descriptor)?.ok_or_else(|| format!("cannot connect `{}` to `{}`: no shared wire format", source_port.spec.id, target_port.spec.id))?;
        let kind_id = target_port.spec.kind_id.clone().unwrap_or_else(|| target_descriptor.kind.clone());
        Ok(MediaContract { kind_id, media_type: target_port.spec.media_type, wire, conversion })
    }

    /// 🔀️ Prefers a shared `Document{schema}` wire (structured payloads round-trip losslessly) over a shared
    /// `Binary{format_kind}` wire (the first common format kind id between the two descriptors' export/import
    /// lists) — see `MediaWireFormat`. The legacy format enum was retired — ticket 26/08/11/
    /// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6.
    fn negotiate_wire_format(source: &OsArtifactDescriptor, target: &OsArtifactDescriptor) -> Result<Option<MediaWireFormat>, String> {
        if !source.schema.is_empty() && source.schema == target.schema {
            return Ok(Some(MediaWireFormat::Document { schema: source.schema.clone() }));
        }
        if !source.export_stdio_kinds.is_empty() && !target.import_stdio_kinds.is_empty() {
            for kind in &source.export_stdio_kinds {
                let format_kind = semio_framework::format_descriptor(kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown source stdio format kind `{kind}`"))?.kind_id;
                for other in &target.import_stdio_kinds {
                    let target_format_kind = semio_framework::format_descriptor(other).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown target stdio format kind `{other}`"))?.kind_id;
                    if target_format_kind == format_kind {
                        return Ok(Some(MediaWireFormat::Binary { format_kind }));
                    }
                }
            }
        }
        if let Some(format_kind) = registry_shared_stdio_dialect(&native_dialect_kind(&source.kind), &native_dialect_kind(&target.kind))? {
            return Ok(Some(MediaWireFormat::Binary { format_kind }));
        }
        Ok(source.export_formats.iter().find(|format| target.import_formats.contains(format)).map(|format| MediaWireFormat::Binary { format_kind: format.clone() }))
    }

    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: see the os-side twin of this function for
    /// the full rationale — consults the live typed IO registry as a supplement to the
    /// `export_stdio_kinds`/`import_stdio_kinds` static lists, catching drift between the two.
    fn registry_shared_stdio_dialect(source_kind: &str, target_kind: &str) -> Result<Option<String>, String> {
        use semio_framework::IoDirection;
        let target_reads: HashSet<&str> = crate::host::resolve_kernel_future(semio_framework::io_dialects_for(target_kind, IoDirection::Import)).map_err(|error| format!("{} registry unavailable", error.registry))?.iter().map(|d| d.artifact_kind).collect();
        if target_reads.contains(source_kind) {
            let descriptor = semio_framework::format_descriptor(source_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown source dialect format kind `{source_kind}`"))?;
            return Ok(Some(descriptor.kind_id));
        }
        let source_reads: HashSet<&str> = crate::host::resolve_kernel_future(semio_framework::io_dialects_for(source_kind, IoDirection::Import)).map_err(|error| format!("{} registry unavailable", error.registry))?.iter().map(|d| d.artifact_kind).collect();
        for candidate in target_reads.intersection(&source_reads) {
            let descriptor = semio_framework::format_descriptor(candidate).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown shared dialect format kind `{candidate}`"))?;
            return Ok(Some(descriptor.kind_id));
        }
        Ok(None)
    }

    /// @emoji ✅️ Validates workflow connectivity, cycle freedom (via `workflow::validate_workflow`,
    /// re-exported as `kernel_validate_workflow`), and edge-contract consistency (this layer's own
    /// pass, since it needs the artifact registry the kernel crate doesn't have).
    pub fn validate_workflow(graph: &Workflow) -> WorkflowValidation {
        let mut validation = crate::host::resolve_kernel_future(kernel_validate_workflow(graph));

        //#region ContractConsistency
        // 🛡️ Defense in depth for merged/imported studio documents: re-negotiate each edge's endpoints
        // against the *current* artifact registry and flag any edge whose stored `contract` no longer
        // matches — a concurrent re-typing or a stale import can leave a wire's contract behind.
        let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        for edge in &graph.edges {
            let Some(source_port) = node_by_id.get(edge.source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id)) else { continue };
            let Some(target_port) = node_by_id.get(edge.target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id)) else { continue };
            match negotiate_media_contract(source_port, target_port) {
                Ok(contract) if contract == edge.contract => {}
                Ok(_) => validation.errors.push(format!("edge {} contract stale: no longer matches negotiated port types", edge.id)),
                Err(reason) => validation.errors.push(format!("edge {} contract invalid: {reason}", edge.id)),
            }
        }
        //#endregion ContractConsistency

        validation.ok = validation.errors.is_empty();
        validation
    }

    pub fn os_media_neuron_kind_for_node(node_id: &str) -> String {
        format!("os.media.node.{node_id}")
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowCamera {
        pub x: f64,
        pub y: f64,
        pub zoom: f64,
    }

    impl Default for OsWorkflowCamera {
        fn default() -> Self {
            Self { x: 0.0, y: 0.0, zoom: 1.0 }
        }
    }

    pub fn os_workflow_to_flow_fixture(graph: &Workflow, camera: &OsWorkflowCamera) -> Value {
        let widgets: Vec<_> = graph
            .nodes
            .iter()
            .map(|node| {
                json!({
                    "kind": "neuron",
                    "id": node.id,
                    "neuronKind": os_media_neuron_kind_for_node(&node.id),
                    "inputPorts": node.inputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                    "outputPorts": node.outputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                    "params": {
                        "nodeId": node.id,
                        "pluginId": node.plugin_id,
                        "appId": node.app_id,
                    },
                    "preview": true,
                })
            })
            .collect();
        let layout: HashMap<_, _> = graph.nodes.iter().map(|node| (node.id.clone(), json!({ "x": node.x + node.width / 2.0, "y": node.y + node.height / 2.0 }))).collect();
        let synapses: Vec<_> = graph
            .edges
            .iter()
            .map(|edge| {
                json!({
                    "id": edge.id,
                    "from": edge.source_node_id,
                    "to": edge.target_node_id,
                    "fromPort": edge.source_port_id,
                    "toPort": edge.target_port_id,
                })
            })
            .collect();
        json!({
            "schema": "flow.fixture",
            "camera": { "x": camera.x, "y": camera.y, "zoom": camera.zoom },
            "widgets": widgets,
            "synapses": synapses,
            "layout": layout,
        })
    }

    /** @emoji 🔁️ Diffs a flow fixture back into workflow operations — inverse of [`os_workflow_to_flow_fixture`]. */
    pub fn apply_flow_fixture_to_os_workflow(graph: &Workflow, fixture_json: &str) -> Vec<WorkflowMutation> {
        let Ok(fixture) = serde_json::from_str::<Value>(fixture_json) else {
            return Vec::new();
        };
        let mut operations = Vec::new();
        if let Some(layout) = fixture.get("layout").and_then(Value::as_object) {
            for node in &graph.nodes {
                let Some(position) = layout.get(&node.id) else { continue };
                let (Some(center_x), Some(center_y)) = (position.get("x").and_then(Value::as_f64), position.get("y").and_then(Value::as_f64)) else {
                    continue;
                };
                let x = center_x - node.width / 2.0;
                let y = center_y - node.height / 2.0;
                if (x - node.x).abs() > 1e-6 || (y - node.y).abs() > 1e-6 {
                    operations.push(WorkflowMutation::MoveNode(MoveNode { node_id: node.id.clone(), x, y }));
                }
            }
        }
        let mut removed_node_ids = HashSet::new();
        if let Some(widgets) = fixture.get("widgets").and_then(Value::as_array) {
            let widget_ids: HashSet<&str> = widgets.iter().filter_map(|widget| widget.get("id").and_then(Value::as_str)).collect();
            for node in &graph.nodes {
                if !widget_ids.contains(node.id.as_str()) {
                    removed_node_ids.insert(node.id.clone());
                    operations.push(WorkflowMutation::RemoveNode(RemoveNode { node_id: node.id.clone() }));
                }
            }
        }
        let synapse_endpoints = |synapse: &Value| -> Option<(String, String, String, String)> {
            Some((synapse.get("from").and_then(Value::as_str)?.into(), synapse.get("fromPort").and_then(Value::as_str)?.into(), synapse.get("to").and_then(Value::as_str)?.into(), synapse.get("toPort").and_then(Value::as_str)?.into()))
        };
        let edge_endpoints = |edge: &WorkflowEdge| (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
        let synapses = fixture.get("synapses").and_then(Value::as_array).cloned().unwrap_or_default();
        let fixture_endpoints: HashSet<_> = synapses.iter().filter_map(synapse_endpoints).collect();
        let graph_endpoints: HashSet<_> = graph.edges.iter().map(edge_endpoints).collect();
        let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        for synapse in &synapses {
            let Some(endpoints) = synapse_endpoints(synapse) else { continue };
            if graph_endpoints.contains(&endpoints) {
                continue;
            }
            let (source_node_id, source_port_id, target_node_id, target_port_id) = endpoints;
            // 🤝️ Only wire the edge if the endpoints still negotiate a valid contract — a stale/hand-edited
            // fixture referencing an incompatible or now-removed port silently drops the synapse instead of
            // producing an untyped edge (see `negotiate_media_contract`).
            let Some(source_port) = node_by_id.get(source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == source_port_id)) else { continue };
            let Some(target_port) = node_by_id.get(target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == target_port_id)) else { continue };
            let Ok(contract) = negotiate_media_contract(source_port, target_port) else { continue };
            let id = synapse.get("id").and_then(Value::as_str).filter(|value| !value.is_empty()).map(str::to_string).unwrap_or_else(|| create_os_id("edge"));
            operations.push(WorkflowMutation::ConnectPorts(ConnectPorts { edge: WorkflowEdge { id, source_node_id, source_port_id, target_node_id, target_port_id, contract } }));
        }
        if fixture.get("synapses").and_then(Value::as_array).is_some() {
            for edge in &graph.edges {
                if fixture_endpoints.contains(&edge_endpoints(edge)) {
                    continue;
                }
                if removed_node_ids.contains(&edge.source_node_id) || removed_node_ids.contains(&edge.target_node_id) {
                    continue;
                }
                operations.push(WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: edge.id.clone() }));
            }
        }
        operations
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowNodeGraphPayload {
        pub nodes_json: String,
        pub edges_json: String,
        pub viewport_json: String,
        pub find_items_json: String,
    }

    /// @emoji 🕸️ Serializes an OS workflow into generic node-graph scene payloads.
    ///
    /// 🚧️ TEMP(Wave 3): still emits JSON-string payloads (typed `NodeGraphScene` records land with
    /// WP-0.3/WP-3.2) — kept load-bearing for the space plugin ui crate (WP-1.5) until then. Port keys
    /// now read `artifactKind` off `WorkflowMediaPort.spec.kind_id` (was the deleted `OsMediaPort`'s
    /// `artifact_kind` string field under the stale `resourceKind` key).
    pub fn os_workflow_to_node_graph_payload(graph: &Workflow) -> OsWorkflowNodeGraphPayload {
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .map(|node| {
                json!({
                    "id": node.id,
                    "instanceId": node.id,
                    "label": format!("{} / {}", node.plugin_id, node.app_id),
                    "x": node.x,
                    "y": node.y,
                    "width": node.width,
                    "height": node.height,
                    "inputs": node.inputs.iter().map(|port| json!({
                        "id": port.id,
                        "artifactKind": port.spec.kind_id.clone().unwrap_or_default(),
                        "direction": "in",
                        "label": port.spec.label,
                    })).collect::<Vec<_>>(),
                    "outputs": node.outputs.iter().map(|port| json!({
                        "id": port.id,
                        "artifactKind": port.spec.kind_id.clone().unwrap_or_default(),
                        "direction": "out",
                        "label": port.spec.label,
                    })).collect::<Vec<_>>(),
                })
            })
            .collect();
        let edges: Vec<_> = graph
            .edges
            .iter()
            .map(|edge| {
                json!({
                    "id": edge.id,
                    "sourceNodeId": edge.source_node_id,
                    "sourcePortId": edge.source_port_id,
                    "targetNodeId": edge.target_node_id,
                    "targetPortId": edge.target_port_id,
                    // 🏷️ Data plumbing only (no renderer changes here) — lets a later ticket badge/dash
                    // conversion edges without re-deriving the contract client-side.
                    "contract": edge.contract,
                    "isConversion": edge.contract.conversion.is_some(),
                })
            })
            .collect();
        let find_items: Vec<_> = graph.nodes.iter().map(|node| json!({ "id": node.id, "label": format!("{} / {}", node.plugin_id, node.app_id), "category": "Workflow" })).collect();
        OsWorkflowNodeGraphPayload {
            nodes_json: serde_json::to_string(&nodes).unwrap_or_else(|_| "[]".into()),
            edges_json: serde_json::to_string(&edges).unwrap_or_else(|_| "[]".into()),
            viewport_json: r#"{"x":0,"y":0,"zoom":1}"#.into(),
            find_items_json: serde_json::to_string(&find_items).unwrap_or_else(|_| "[]".into()),
        }
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct OsWorkflowChannelSpec {
        pub name: String,
        pub code: String,
        pub abbreviation: String,
        pub full_name: String,
        pub operators: Vec<String>,
    }

    /// 🌱️ `ToValue` only (additive, alongside `serde` — this crate is framework, exempt from the
    /// serde ban) — `space`'s `json_array_to_node_graph_operators` shim only ENCODES this type
    /// (never decodes it back), see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
    /// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, value_derive::ToValue)]
    #[serde(rename_all = "camelCase")]
    #[value(rename_all = "camelCase")]
    pub struct OsWorkflowOperatorInfo {
        pub id: String,
        pub module: String,
        pub name: String,
        pub abbreviation: String,
        pub icon: String,
        pub summary: String,
        pub inputs: Vec<OsWorkflowChannelSpec>,
        pub outputs: Vec<OsWorkflowChannelSpec>,
    }

    /// @emoji 🎬️ Derives one `OsWorkflowChannelSpec` directly from a `WorkflowMediaPort`'s
    /// `MediaPortSpec` — `operators` (accepted schema ids) is the port's own `kind_id` (falling back to
    /// its id when unset, e.g. the implicit `document:*` ports).
    fn os_workflow_channel_spec(port: &WorkflowMediaPort, label: &str) -> OsWorkflowChannelSpec {
        let code = port.spec.id.chars().next().map(|ch| ch.to_uppercase().collect::<String>()).unwrap_or_else(|| "P".into());
        let abbreviation = if label.chars().count() <= 3 { label.into() } else { label.chars().take(3).collect() };
        let operator = port.spec.kind_id.clone().unwrap_or_else(|| port.spec.id.clone());
        OsWorkflowChannelSpec { name: port.spec.id.clone(), code, abbreviation, full_name: label.into(), operators: vec![operator] }
    }

    /// @emoji 🧩️ Registers per-node neuron metadata for the OS workflow flow extension — derived
    /// directly from each node's `WorkflowMediaPort.spec: MediaPortSpec` (no more stringly synthesis
    /// from a separate `OsAppInstance` join). `parameters`/port-id helpers are the kernel `workflow`
    /// crate's own (`workflow::WorkflowParameter`/`workflow::media_port_spec_id`/
    /// `workflow::workflow_parameter_id_from_port_id`) — `WorkflowSnapshot.parameters` absorbed the
    /// dissolved `OsSnapshot.parameters` in W3, see `## The inversion`.
    pub fn build_os_workflow_operator_infos(graph: &Workflow, parameters: &[WorkflowParameter]) -> Vec<OsWorkflowOperatorInfo> {
        let parameter_by_id: HashMap<_, _> = parameters.iter().map(|row| (workflow_parameter_id(row).to_string(), row)).collect();
        graph
            .nodes
            .iter()
            .map(|node| {
                let registration = os_app_registration(&node.plugin_id, &node.app_id);
                let neuron_kind = os_media_neuron_kind_for_node(&node.id);
                OsWorkflowOperatorInfo {
                    id: neuron_kind,
                    module: OS_MEDIA_FLOW_MODULE_ID.into(),
                    name: node.label.clone(),
                    abbreviation: if node.app_id.chars().count() <= 3 { node.app_id.clone() } else { node.app_id.chars().take(3).collect() },
                    icon: format!("emoji:{}", registration.map(|row| row.component_kind.clone()).unwrap_or_else(|| "s".into())),
                    summary: format!("{}/{}", node.plugin_id, node.app_id),
                    inputs: node
                        .inputs
                        .iter()
                        .map(|port| {
                            let parameter_id = workflow_parameter_id_from_port_id(&port.id);
                            let label = parameter_id.as_ref().and_then(|id| parameter_by_id.get(id)).map(|parameter| workflow_parameter_name(parameter)).or_else(|| media_port_spec_id(&port.id)).unwrap_or_else(|| port.id.clone());
                            os_workflow_channel_spec(port, &label)
                        })
                        .collect(),
                    outputs: node
                        .outputs
                        .iter()
                        .map(|port| {
                            let label = media_port_spec_id(&port.id).unwrap_or_else(|| port.id.clone());
                            os_workflow_channel_spec(port, &label)
                        })
                        .collect(),
                }
            })
            .collect()
    }
    //#endregion 🔖️Workflow

    // 🧷️ `WorkflowDelivery`/`WorkflowFixture`/`plan_workflow` now live in the kernel `workflow` crate
    // (re-exported above, `producer_node_id`/`consumer_node_id` field names). `WorkflowInstanceRegistry`
    // was confirmed dead (zero callers) and deleted outright.

    //#region 🔖️MediaExport
    //#region 🔖️IoDialectBridge
    /// 🌉️ Resolves a legacy OS-workflow kind id (`ArtifactKindSpec.id`/`OsArtifactDescriptor.kind`,
    /// e.g. `"3d.cad"`, `"3d.puzzle"`, `"2d.map"` — dimension-prefixed, what `WorkflowNode.yields`
    /// and every `register_mesh_exporter`/`register_solid_exporter`/`register_dwg_import_handler`
    /// call site historically keyed on) to the REAL `Dialect.artifact_kind` its `ComposerEntry` is
    /// registered under (`"s." + component_kind`, e.g. `"s.cad"`, `"s.puzzle3d"`, `"s.gismap"`).
    /// Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1 finding:
    /// these are two different id namespaces that happen to look similar, so `registry_export_media`/
    /// `registry_import_media`/`registry_shared_stdio_dialect` were silently building a `native_kind`
    /// (`format!("s.{artifact_kind}")` off the RAW workflow id) that could never match any real
    /// composer dialect -- the "try the typed registry first" path was dead for every production
    /// caller, always falling through to the legacy `register_os_media_{export,import}_handler_kind`
    /// map even for artifacts (cad, puzzle3d/5d, puzzle2d, gismap) that already carry a full
    /// `ComposerEntry` roster for the exact same formats. `OsArtifactDescriptor.component_kind` is
    /// the one field already carrying the un-prefixed dialect slug, so no new registry is needed --
    /// only reading the right field. An unregistered/synthetic kind (e.g. a unit test's throwaway
    /// `"3d.__mesh_exporter_test"`) resolves through `os_artifact_descriptor`'s own placeholder
    /// fallback to `"s.panel"`, which will not match any registered dialect and therefore still
    /// falls through to the legacy handler map exactly as before -- this bridge only changes
    /// resolution for kinds that were ever registered via `register_artifact_descriptor(s)`.
    /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: no longer
    /// re-derives the `"s.{component_kind}"` string on every call -- reads `crate::registry::
    /// os_artifact_dialect(workflow_kind).artifact_kind`, the ONE stored `ArtifactDialect` computed
    /// once at registration time (`crate::registry::dialect_from_component_kind`). Return type stays
    /// `String` (just the `artifact_kind` segment, no `@standard/subset`) because this function's own
    /// two remaining callers (`registry_shared_stdio_dialect` below, and the OLD `io_dispatch`
    /// fallback paths in `registry_export_media`/`registry_import_media`) both talk to the OLD
    /// `semio_framework::io_dialects_for`/`IoKey` registry (debt D2), whose `Dialect.artifact_kind`
    /// is a bare `&str` with no standard/subset fields of its own -- the NEW io-mechanism path below
    /// builds a full `ArtifactDialect` directly from `os_artifact_dialect`, not from this function.
    fn native_dialect_kind(workflow_kind: &str) -> String {
        crate::registry::os_artifact_dialect(workflow_kind).artifact_kind
    }
    //#endregion 🔖️IoDialectBridge
    //#region 🔖️MediaCapability
    #[cfg(feature = "os-host-full")]
    pub use crate::registry::os_resource_media_capability;
    #[cfg(not(feature = "os-host-full"))]
    fn os_resource_media_capability(_kind: &str) -> semio_framework::OsMediaCapability {
        semio_framework::OsMediaCapability::MeshOnly
    }
    /// 🗂️ Defined in `semio_framework` alongside `ArtifactKindSpec`; re-exported here
    /// verbatim. `os_resource_media_capability` is a registry lookup (see `crate::registry`) driven by each
    /// app's declared `ArtifactKindSpec.media_capability` instead of a hardcoded per-app match.
    pub use semio_framework::OsMediaCapability;
    //#endregion 🔖️MediaCapability

    #[derive(Clone, Debug, PartialEq)]
    pub struct OsMediaExportResult {
        pub data: String,
        pub mime_type: String,
        pub file_name: String,
        pub encoding: Option<String>,
    }

    impl OsMediaExportResult {
        /// 📤️ Build an export result from raw bytes + stdio format kind id (the legacy format enum was retired —
        /// ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
        pub fn from_format_kind_bytes(bytes: Vec<u8>, format_artifact_kind: &str, file_stem: &str) -> Result<Self, String> {
            let entry = semio_framework::format_descriptor(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
            let mime_type = entry.mimes.first().cloned().ok_or_else(|| format!("stdio format kind `{format_artifact_kind}` has no MIME claim"))?;
            let extension = entry.extensions.first().ok_or_else(|| format!("stdio format kind `{format_artifact_kind}` has no extension claim"))?;
            let data = if entry.is_binary { base64::engine::general_purpose::STANDARD.encode(&bytes) } else { String::from_utf8(bytes).map_err(|error| error.to_string())? };
            Ok(Self { data, mime_type, file_name: format!("{file_stem}{extension}"), encoding: if entry.is_binary { Some("base64".into()) } else { None } })
        }
    }

    type OsMediaExportHandler = Box<dyn Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync>;

    fn export_handlers() -> &'static Mutex<HashMap<String, OsMediaExportHandler>> {
        static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaExportHandler>>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// 🗂️ Registry key `(artifact_kind, format_artifact_kind)` — stdio kind ids.
    fn os_media_handler_key(artifact_kind: &str, format_artifact_kind: &str) -> String {
        format!("{artifact_kind}:{format_artifact_kind}")
    }

    /// 🗄️ Registers an export handler keyed by `(artifact_kind, format_artifact_kind)` stdio/kind ids.
    pub fn register_os_media_export_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static) {
        export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(os_media_handler_key(artifact_kind, format_artifact_kind), Box::new(handler));
    }

    /// 📤️ Export via `(artifact_kind, format_artifact_kind)` stdio kind ids.
    pub fn export_os_app_instance_media_kind(node: &WorkflowNode, source_document: &Value, format_artifact_kind: &str) -> Result<OsMediaExportResult, String> {
        let format_kind = semio_framework::normalize_format_kind(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
        if let Some(result) = registry_export_media(&node.yields, &format_kind, source_document) {
            return result;
        }
        let handlers = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers
            .get(&os_media_handler_key(&node.yields, &format_kind))
            .or_else(|| handlers.get(&os_media_handler_key(&node.yields, format_artifact_kind)))
            .ok_or_else(|| format!("no export handler for {}:{}", node.yields, format_artifact_kind))?;
        handler(source_document)
    }

    /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1: the io-mechanism
    /// export path -- **the fix for the bug this whole ticket exists to remove**. The OLD
    /// `registry_export_media_legacy` below built a FAKE `s.stdio.json@rfc8259/*` "bridge" dialect for
    /// `source_document` and handed it to the OLD registry's `io_dispatch`, whose `composed.payload`
    /// (per `composer_entry_of`) is the artifact's raw internal `ArtifactPack::encode_pack(snapshot)`
    /// bytes for any kind that never registered a REAL composer for the requested format -- so "export
    /// as .png" wrote an unopenable `.semio` pack container to a `.png` file. This function instead:
    /// (1) resolves `artifact_kind`'s REAL `ArtifactDialect` from the catalog (task 2's
    /// `crate::registry::os_artifact_dialect`, never a hardcoded `standard: "1"` string literal built
    /// fresh here), (2) picks the carrier dialect (`CARRIER_BINARY`/`CARRIER_TEXT`, design.md §3) by
    /// whether the target `format_kind` is binary or text, (3) `io_route`s from the artifact's own
    /// dialect all the way to that carrier (up to 3 hops -- a route through the requested format's own
    /// dialect and then that format's native codec down to raw bytes is exactly the kind of path this
    /// graph search finds), (4) `io_run`s it. The resulting carrier payload IS the raw file content --
    /// no JSON bridge, no `IoKey`, no `io_dispatch` -- written verbatim by `from_format_kind_bytes`.
    /// Returns `None` (not an error) whenever no route exists yet (every real plugin today -- no
    /// subset has migrated onto `declare_artifact`/`io_register` yet, W1-D openQuestion #4), so
    /// `registry_export_media` below falls through to the OLD path for every production caller until
    /// W2+ cuts real subsets over -- this is debt D2's "coexist, do not bridge" shape, not a silent gap.
    fn registry_export_media_via_io_mechanism(artifact_dialect: &ArtifactDialect, format_kind: &str, source_document: &Value, file_stem: &str) -> Option<Result<OsMediaExportResult, String>> {
        use semio_framework::io::io_mechanism::{io_route, io_run};
        use semio_framework::io_schema::{IoPayload as NewIoPayload, CARRIER_BINARY, CARRIER_TEXT};

        let is_binary = semio_framework::format_descriptor(format_kind).ok().flatten()?.is_binary;
        let carrier: ArtifactDialect = (if is_binary { CARRIER_BINARY } else { CARRIER_TEXT }).into();
        let route = crate::host::resolve_kernel_future(io_route(artifact_dialect, &carrier, 3)).ok()?.value;
        // 🌉️ `source_document` is this artifact's own JSON-shaped snapshot as the OS document store
        // already carries it -- a legitimate `IoPayload::Text` reading of `artifact_dialect`'s own
        // native encoding whenever that dialect's `NativeCodecs.snapshot.text` is a plain-serde-json
        // `ArtifactDsl` impl (the common case today, D7), NOT a re-introduction of the deleted
        // `s.stdio.json` bridge dialect -- the dialect claimed here is the artifact's REAL dialect, not
        // a fake stand-in. If a migrated subset's real DSL grammar differs, the first hop's
        // `Deserializer::deserialize` fails to parse and `io_run` returns `Err`, `.ok()?` below yields
        // `None`, and `registry_export_media` safely falls through to the OLD path -- never a silent
        // wrong-content export.
        let json_text = serde_json::to_string(source_document).ok()?;
        let outcome = crate::host::resolve_kernel_future(io_run(&route, NewIoPayload::Text(json_text))).ok()?;
        let bytes = match outcome.value {
            NewIoPayload::Binary(b) => b,
            NewIoPayload::Text(t) => t.into_bytes(),
        };
        Some(OsMediaExportResult::from_format_kind_bytes(bytes, format_kind, file_stem))
    }

    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: see the os-side twin of this function for
    /// the full rationale -- dispatches export via `io_dispatch` (real subset validation + one-hop
    /// fallback guard, ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT
    /// W6) before falling back to the old stringly handler map.
    /// 🐛️ Ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1 fix:
    /// `format_kind` here is ALWAYS the caller's already-`normalize_format_kind`d value (see both
    /// call sites below), which returns the canonical `FormatDescriptor.kind_id` -- already in
    /// `"stdio.<format>"` form, e.g. `"stdio.obj"`. Prefixing another literal `"stdio."` on top
    /// built `"s.stdio.stdio.obj"`, which can never match a registered `"s.stdio.obj"` dialect --
    /// so this lookup silently missed for EVERY artifact/format pair and always fell through to
    /// the legacy handler map, regardless of `native_dialect_kind`. Only `"s."` belongs here.
    /// 🎯️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1 (debt D2): kept
    /// working, UNCHANGED logic, as the fallback for artifacts that have not migrated onto the new
    /// io-mechanism yet -- W6 deletes this function outright. Never merged with the new path above;
    /// `registry_export_media` picks ONE or the other per call, never blends their results.
    fn registry_export_media_legacy(artifact_kind: &str, format_kind: &str, source_document: &Value) -> Option<Result<OsMediaExportResult, String>> {
        use semio_framework::{Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
        let native_kind = native_dialect_kind(artifact_kind);
        let target_kind = format!("s.{format_kind}");
        let target = match crate::host::resolve_kernel_future(semio_framework::io_dialects_for(&native_kind, IoDirection::Export)) {
            Ok(dialects) => dialects.into_iter().find(|dialect| dialect.artifact_kind == target_kind)?,
            Err(error) => return Some(Err(format!("{} registry unavailable", error.registry))),
        };
        let key = IoKey {
            artifact_kind: native_kind,
            standard: "1".to_string(),
            subset: "*".to_string(),
            direction: IoDirection::Export,
            format_kind: target.artifact_kind.to_string(),
            format_standard: target.standard.0.to_string(),
            format_subset: target.subset.0.to_string(),
        };
        let json_bridge = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
        let json_text = serde_json::to_string(source_document).ok()?;
        let sources = [ErasedComposeSource { dialect: json_bridge, payload: IoPayload::Text(json_text) }];
        let composed = crate::host::resolve_kernel_future(semio_framework::io_dispatch(&key, &sources)).ok()?;
        let bytes = match composed.payload {
            IoPayload::Binary(b) => b,
            IoPayload::Text(t) => t.into_bytes(),
        };
        Some(OsMediaExportResult::from_format_kind_bytes(bytes, format_kind, artifact_kind))
    }

    /// 🚪️ Entry point `export_os_app_instance_media_kind` calls: tries the NEW io-mechanism path
    /// first (task 1's real fix), falls through to the OLD `io_dispatch` path (debt D2) when no route
    /// exists yet, falls through again to the stringly handler map at the call site. One path wins per
    /// call -- never merged (design.md's rejected-approaches list; `📌️important.md`).
    fn registry_export_media(artifact_kind: &str, format_kind: &str, source_document: &Value) -> Option<Result<OsMediaExportResult, String>> {
        let dialect = crate::registry::os_artifact_dialect(artifact_kind);
        if let Some(result) = registry_export_media_via_io_mechanism(&dialect, format_kind, source_document, artifact_kind) {
            return Some(result);
        }
        registry_export_media_legacy(artifact_kind, format_kind, source_document)
    }

    pub fn os_media_export_extension_for_format_kind(format_artifact_kind: &str) -> Result<Option<String>, semio_framework::FormatRegistryError> {
        Ok(semio_framework::format_descriptor(format_artifact_kind)?.and_then(|row| row.extensions.first().cloned()))
    }

    type OsMediaImportHandler = Box<dyn Fn(&[u8]) -> Result<Value, String> + Send + Sync>;

    fn import_handlers() -> &'static Mutex<HashMap<String, OsMediaImportHandler>> {
        static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaImportHandler>>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// 🗄️ Registers an import handler keyed by `(artifact_kind, format_artifact_kind)` stdio/kind ids.
    pub fn register_os_media_import_handler_kind(artifact_kind: &str, format_artifact_kind: &str, handler: impl Fn(&[u8]) -> Result<Value, String> + Send + Sync + 'static) {
        import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(os_media_handler_key(artifact_kind, format_artifact_kind), Box::new(handler));
    }

    /// 📥️ Import via `(artifact_kind, format_artifact_kind)` stdio kind ids.
    pub fn import_os_app_instance_media_kind(node: &WorkflowNode, data: &[u8], format_artifact_kind: &str) -> Result<Value, String> {
        let format_kind = semio_framework::normalize_format_kind(format_artifact_kind).map_err(|error| error.to_string())?.ok_or_else(|| format!("unknown stdio format kind `{format_artifact_kind}`"))?;
        if let Some(result) = registry_import_media(&node.yields, &format_kind, data) {
            return result;
        }
        let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers
            .get(&os_media_handler_key(&node.yields, &format_kind))
            .or_else(|| handlers.get(&os_media_handler_key(&node.yields, format_artifact_kind)))
            .ok_or_else(|| format!("no import handler for {}:{}", node.yields, format_artifact_kind))?;
        handler(data)
    }

    /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1: the io-mechanism
    /// import path, mirroring `registry_export_media_via_io_mechanism`'s reasoning in reverse. The
    /// caller (`registry_import_media`, via `import_os_app_instance_media_kind`) already knows both
    /// `artifact_dialect` (the target the bytes should become) and `format_kind` (what the bytes
    /// already are), so per design.md §3 ("When the caller already knows the dialect, skip identify")
    /// this is a SINGLE `io_route(carrier -> artifact_dialect)` + `io_run`, never `io_identify` --
    /// `io_identify` is for the genuinely-unknown-dialect "open this file" case, not this one.
    fn registry_import_media_via_io_mechanism(artifact_dialect: &ArtifactDialect, format_kind: &str, data: &[u8]) -> Option<Result<Value, String>> {
        use semio_framework::io::io_mechanism::{io_route, io_run};
        use semio_framework::io_schema::{IoPayload as NewIoPayload, CARRIER_BINARY, CARRIER_TEXT};

        let is_binary = semio_framework::format_descriptor(format_kind).ok().flatten()?.is_binary;
        let carrier: ArtifactDialect = (if is_binary { CARRIER_BINARY } else { CARRIER_TEXT }).into();
        let carrier_payload = if is_binary { NewIoPayload::Binary(data.to_vec()) } else { NewIoPayload::Text(String::from_utf8(data.to_vec()).ok()?) };
        let route = crate::host::resolve_kernel_future(io_route(&carrier, artifact_dialect, 3)).ok()?.value;
        let outcome = crate::host::resolve_kernel_future(io_run(&route, carrier_payload)).ok()?;
        // 🌉️ Mirrors the export side: the JSON text this yields is read back as this artifact's own
        // OS-document-store shape, not re-wrapped through the deleted `s.stdio.json` bridge dialect.
        let json_text = match outcome.value {
            NewIoPayload::Text(t) => t,
            NewIoPayload::Binary(b) => String::from_utf8(b).ok()?,
        };
        let value: Value = serde_json::from_str(&json_text).ok()?;
        Some(Ok(value))
    }

    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: see the os-side twin of this function for
    /// the full rationale -- two-hop dispatch (target bytes -> native pack bytes -> json text)
    /// through `io_dispatch` before falling back to the old stringly handler map.
    /// 🐛️ Same double-`"stdio."` fix as `registry_export_media` -- see that function's doc comment.
    /// 🎯️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1 (debt D2): kept
    /// working, UNCHANGED logic, as the fallback for artifacts that have not migrated onto the new
    /// io-mechanism yet -- W6 deletes this function outright.
    fn registry_import_media_legacy(artifact_kind: &str, format_kind: &str, data: &[u8]) -> Option<Result<Value, String>> {
        use semio_framework::{ErasedComposeSource, IoDirection, IoKey, IoPayload};
        let native_kind = native_dialect_kind(artifact_kind);
        let target_kind = format!("s.{format_kind}");

        let source_dialect = match crate::host::resolve_kernel_future(semio_framework::io_dialects_for(&native_kind, IoDirection::Import)) {
            Ok(dialects) => dialects.into_iter().find(|dialect| dialect.artifact_kind == target_kind)?,
            Err(error) => return Some(Err(format!("{} registry unavailable", error.registry))),
        };
        let import_key = IoKey {
            artifact_kind: native_kind.clone(),
            standard: "1".to_string(),
            subset: "*".to_string(),
            direction: IoDirection::Import,
            format_kind: source_dialect.artifact_kind.to_string(),
            format_standard: source_dialect.standard.0.to_string(),
            format_subset: source_dialect.subset.0.to_string(),
        };
        let sources = [ErasedComposeSource { dialect: source_dialect, payload: IoPayload::Binary(data.to_vec()) }];
        let native = crate::host::resolve_kernel_future(semio_framework::io_dispatch(&import_key, &sources)).ok()?;

        let export_dialect = match crate::host::resolve_kernel_future(semio_framework::io_dialects_for(&native_kind, IoDirection::Export)) {
            Ok(dialects) => dialects.into_iter().find(|dialect| dialect.artifact_kind == "s.stdio.json")?,
            Err(error) => return Some(Err(format!("{} registry unavailable", error.registry))),
        };
        let export_key = IoKey {
            artifact_kind: native_kind,
            standard: "1".to_string(),
            subset: "*".to_string(),
            direction: IoDirection::Export,
            format_kind: export_dialect.artifact_kind.to_string(),
            format_standard: export_dialect.standard.0.to_string(),
            format_subset: export_dialect.subset.0.to_string(),
        };
        let native_sources = [ErasedComposeSource { dialect: native.dialect, payload: native.payload }];
        let json_out = crate::host::resolve_kernel_future(semio_framework::io_dispatch(&export_key, &native_sources)).ok()?;
        let bytes = match json_out.payload {
            IoPayload::Binary(b) => b,
            IoPayload::Text(t) => t.into_bytes(),
        };
        Some(serde_json::from_slice(&bytes).map_err(|e| e.to_string()))
    }

    /// 🚪️ Entry point `import_os_app_instance_media_kind` calls: tries the NEW io-mechanism path
    /// first, falls through to the OLD `io_dispatch` path (debt D2) when no route exists yet, falls
    /// through again to the stringly handler map at the call site. One path wins per call.
    fn registry_import_media(artifact_kind: &str, format_kind: &str, data: &[u8]) -> Option<Result<Value, String>> {
        let dialect = crate::registry::os_artifact_dialect(artifact_kind);
        if let Some(result) = registry_import_media_via_io_mechanism(&dialect, format_kind, data) {
            return Some(result);
        }
        registry_import_media_legacy(artifact_kind, format_kind, data)
    }

    //#endregion 🔖️MediaExport

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 1: the real
        /// end-to-end proof for the bug this whole ticket exists to remove -- "export as .xyz" must
        /// write raw file content, never a `SemioEnvelope`-wrapped pack container (`store::BINARY_MAGIC`
        /// = `[0x89, 'S','E','M', 0x0D,0x0A,0x1A,0x0A]`, `🧬️semio/🦀️component.rs`). Registers a
        /// throwaway `IoEntry` straight from a synthetic artifact dialect to `CARRIER_BINARY` (its
        /// `run` returns whatever raw bytes its input carries -- no pack framing of any kind, exactly
        /// the shape a real format serializer produces), registers a matching `OsArtifactDescriptor`
        /// so `crate::registry::os_artifact_dialect` derives the SAME dialect the entry was registered
        /// under, then calls the real `registry_export_media` (task 1's actual production entry point,
        /// not a private helper) end to end: `os_artifact_dialect` -> `io_route` -> `io_run` ->
        /// `OsMediaExportResult`. Asserts the decoded bytes are byte-identical to the raw content and
        /// do NOT start with the pack magic header.
        #[test]
        fn export_via_io_mechanism_writes_raw_bytes_not_a_pack_container() {
            use base64::Engine;
            use semio_framework::io::io_mechanism::{io_register, IoEntry};
            use semio_framework::io_schema::{IoFidelity, IoOutcome, IoPayload as NewIoPayload, CARRIER_BINARY};

            const TEST_KIND: &str = "3d.__w1b_export_bug_proof";
            const TEST_DIALECT: semio_framework::Dialect = semio_framework::Dialect { artifact_kind: "s.__w1b_export_bug_proof", standard: semio_framework::StandardId("1"), subset: semio_framework::SubsetId("*") };
            /// 🧷️ The literal bytes of `store::BINARY_MAGIC` / `os_semio::BINARY_MAGIC`
            /// (`🧬️semio/🦀️component.rs`), inlined so this assertion never depends on that constant's
            /// own export path -- a genuinely independent check of the OLD pack format's header.
            const PACK_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];

            fn run(payload: &NewIoPayload) -> semio_framework::io_schema::IoResult<NewIoPayload> {
                let NewIoPayload::Text(json) = payload else {
                    return Err(semio_framework::io_schema::IoError { message: "expected a text native payload".to_string(), diagnostics: Vec::new() });
                };
                let value: serde_json::Value = serde_json::from_str(json).map_err(|error| semio_framework::io_schema::IoError { message: error.to_string(), diagnostics: Vec::new() })?;
                let raw = value["value"].as_str().unwrap_or_default().to_string();
                Ok(IoOutcome::clean(NewIoPayload::Binary(raw.into_bytes())))
            }

            // 🧷️ Built directly as a one-element array literal (never a separately-named `static`
            // copied into an array) -- `IoEntry` derives no `Copy`/`Clone`, so moving a value OUT of
            // a separate `static` to build `[ENTRY]` would not compile; a single constant-expression
            // array literal has no such move.
            static ENTRIES: [IoEntry; 1] = [IoEntry { from: TEST_DIALECT, into: CARRIER_BINARY, fidelity: IoFidelity::Exact, sniff: None, run }];
            // 📌️ Idempotent re-registration (nextest runs this file's tests in one process) -- a
            // second run of this same test binary registering the identical static entry must not error.
            io_register(&ENTRIES).ok();

            crate::registry::register_artifact_descriptor(&semio_framework::ArtifactKindSpec {
                id: TEST_KIND.to_string(),
                name: "W1b Export Bug Proof".to_string(),
                source_format: TEST_KIND.to_string(),
                component_kind: "__w1b_export_bug_proof".to_string(),
                dimension: "data".to_string(),
                media_capability: semio_framework::OsMediaCapability::MeshOnly,
                media_type: semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
                schema: TEST_KIND.to_string(),
                export_formats: Vec::new(),
                import_formats: Vec::new(),
                export_stdio_kinds: Vec::new(),
                import_stdio_kinds: Vec::new(),
            });
            assert_eq!(crate::registry::os_artifact_dialect(TEST_KIND).to_coordinate(), "s.__w1b_export_bug_proof@1/*", "catalog-derived dialect must exactly match the dialect the test IoEntry was registered under");

            semio_framework::register_format_descriptors([semio_framework::FormatDescriptor {
                kind_id: "stdio.__w1b_export_bug_proof_fmt".to_string(),
                short_id: "w1bproof".to_string(),
                aliases: Vec::new(),
                mimes: vec!["application/octet-stream".to_string()],
                extensions: vec![".w1bproof".to_string()],
                name: "W1b Proof Format".to_string(),
                full_name: "W1b Export Bug Proof Format".to_string(),
                neutral: false,
                dir_name: "w1bproof".to_string(),
                is_binary: true,
            }])
            .ok();

            let source_document = serde_json::json!({ "value": "RAW-FILE-CONTENT-not-a-pack" });
            let outcome =
                registry_export_media(TEST_KIND, "stdio.__w1b_export_bug_proof_fmt", &source_document).expect("io-mechanism export path must find the registered route, not fall through to the legacy/handler-map paths").expect("export must succeed");

            let bytes = base64::engine::general_purpose::STANDARD.decode(&outcome.data).expect("OsMediaExportResult base64-encodes binary payloads");
            assert_eq!(bytes, b"RAW-FILE-CONTENT-not-a-pack".to_vec(), "exported bytes must be exactly the raw content the io-mechanism route produced, byte for byte");
            assert!(!bytes.starts_with(&PACK_MAGIC), "exported bytes must NOT carry the SemioEnvelope pack magic header -- this is the exact `registry_export_media` bug (design.md, `📌️important.md`) this ticket exists to remove");
        }

        #[test]
        fn validates_empty_workflow() {
            assert!(validate_workflow(&empty_workflow()).ok);
        }

        #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
        #[test]
        fn svg_to_dwg_round_trip_produces_a_polyline() {
            let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect x="1" y="1" width="4" height="4"/></svg>"#;
            let bytes = crate::media_export_raster::svg_to_dwg_bytes(svg).expect("svg to dwg");
            let drawing = semio_s_plugin_stdio::artifacts::dwg::dwg_from_bytes(&bytes).expect("dwg from bytes");
            assert!(!drawing.entities.is_empty());
        }

        #[test]
        fn mesh_dwg_registrar_round_trips_a_box() {
            use base64::Engine;
            crate::media_export_raster::register_mesh_dwg_export_handler("3d.__dwg_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
            let result = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&os_media_handler_key("3d.__dwg_test", "dwg")).expect("dwg handler registered")(&serde_json::json!({})).expect("export dwg");
            let bytes = base64::engine::general_purpose::STANDARD.decode(result.data).expect("decode base64");
            let drawing = semio_s_plugin_stdio::artifacts::dwg::dwg_from_bytes(&bytes).expect("dwg from bytes");
            assert!(!drawing.entities.is_empty());
        }

        #[test]
        fn mesh_exporter_registrar_round_trips_a_box_through_glb() {
            use base64::Engine;
            crate::media_export_raster::register_mesh_exporter("3d.__mesh_exporter_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
            let result = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&os_media_handler_key("3d.__mesh_exporter_test", "glb")).expect("glb handler registered")(&serde_json::json!({})).expect("export glb");
            let bytes = base64::engine::general_purpose::STANDARD.decode(result.data).expect("decode base64");
            let mesh = semio_framework::mesh_from_glb(&bytes).expect("glb decodes back to a mesh");
            assert!(mesh.vertex_count() > 0);
        }

        #[test]
        fn mesh_importer_registrar_round_trips_a_box_through_obj() {
            crate::media_export_raster::register_mesh_importer("3d.__mesh_importer_test", |mesh| Ok(serde_json::json!({ "vertexCount": mesh.vertex_count() })), Box::new(semio_framework_plugin::ObjImporter));
            let obj_bytes = semio_framework::mesh_to_obj(&semio_framework_plugin::mesh_from_kind("box"), "box").into_bytes();
            let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let handler = handlers.get(&os_media_handler_key("3d.__mesh_importer_test", "obj")).expect("obj handler registered");
            let document = handler(&obj_bytes).expect("import obj");
            assert!(document["vertexCount"].as_u64().expect("vertex count") > 0);
        }

        // 🚪️ `solid_exporter_and_importer_registrars_round_trip_a_box_through_step` DELETED (ticket
        // 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave IO1): it exercised
        // exactly the `register_solid_exporter`/`register_solid_importer`/`solid_exporter_for`/
        // `export_registered_solid`/`import_registered_solid` mechanism deleted above with it -- see
        // `//#region SolidMediaExport`'s removal note for why that mechanism was dead weight, not a
        // migration gap. The equivalent real coverage (a box round-tripped through STEP via the
        // genuine stdio `semio/brep` bridge) lives in cad's own
        // `export_solids_as_step_round_trips_through_real_semio_brep_bridge`
        // (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`),
        // which this test's own STEP path always deferred to in production anyway.

        /// 🧷️ Hand-built node for tests that don't need a real app registration — `os_workflow_to_flow_fixture`/
        /// `build_os_workflow_operator_infos`/VFS listing all read straight off the node now (no more
        /// separate `OsAppInstance` join), so a plain struct literal is enough.
        fn media_node(id: &str, x: f64, y: f64) -> WorkflowNode {
            let port = |direction: semio_framework::MediaPortDirection| WorkflowMediaPort {
                id: format!("{id}:{}", if direction == semio_framework::MediaPortDirection::In { "in" } else { "out" }),
                spec: semio_framework::MediaPortSpec {
                    id: if direction == semio_framework::MediaPortDirection::In { "in".into() } else { "out".into() },
                    label: "Port".into(),
                    direction,
                    media_type: semio_framework::MediaType { class: semio_framework::MediaClass::TwoD, form: semio_framework::MediaForm::Vector },
                    kind_id: Some("2d.drawing".into()),
                    required: false,
                    multiplicity: semio_framework::PortMultiplicity::One,
                },
            };
            WorkflowNode {
                id: id.into(),
                plugin_id: "draw".into(),
                app_id: "draw".into(),
                label: id.into(),
                yields: "2d.drawing".into(),
                artifact_ref: format!("artifacts/{id}"),
                config_ref: format!("config/{id}"),
                x,
                y,
                width: 160.0,
                height: 72.0,
                inputs: vec![port(semio_framework::MediaPortDirection::In)],
                outputs: vec![port(semio_framework::MediaPortDirection::Out)],
            }
        }

        #[test]
        fn flow_fixture_projects_neuron_preview() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 0.0, 0.0));
            let fixture = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
            assert_eq!(fixture["schema"], "flow.fixture");
            assert_eq!(fixture["widgets"][0]["preview"], true);
            assert_eq!(fixture["widgets"][0]["params"]["nodeId"], "node-1");
            assert_eq!(fixture["widgets"][0]["params"]["pluginId"], "draw");
            assert_eq!(fixture["widgets"][0]["params"]["appId"], "draw");
            let operators = build_os_workflow_operator_infos(&graph, &[]);
            assert_eq!(operators.len(), 1);
            assert_eq!(operators[0].id, "os.media.node.node-1");
            assert_eq!(operators[0].module, OS_MEDIA_FLOW_MODULE_ID);
            assert_eq!(operators[0].name, "node-1");
        }

        // 🚧️ `vfs_inputs_folder_lists_a_dwg_import_row_for_2d_kinds` exercised the deleted
        // `🔖️WorkflowVfs` region (`list_os_workflow_vfs_children`/`os_workflow_vfs_inputs_folder_id`)
        // — a full collection-browser UI replaces it in a later wave, see the os-core dissolve ticket.

        #[test]
        fn flow_fixture_round_trips_camera_and_diffs_back_to_operations() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 40.0, 80.0));
            graph.nodes.push(media_node("node-2", 300.0, 80.0));
            graph.edges.push(WorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "node-1".into(),
                source_port_id: "node-1:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "node-2:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            let camera = OsWorkflowCamera { x: 12.0, y: -8.0, zoom: 1.5 };
            let fixture = os_workflow_to_flow_fixture(&graph, &camera);
            assert_eq!(fixture["camera"]["x"], 12.0);
            assert_eq!(fixture["camera"]["zoom"], 1.5);
            let unchanged = apply_flow_fixture_to_os_workflow(&graph, &fixture.to_string());
            assert!(unchanged.is_empty());
            let mut moved = fixture.clone();
            moved["layout"]["node-1"] = json!({ "x": 220.0, "y": 156.0 });
            let operations = apply_flow_fixture_to_os_workflow(&graph, &moved.to_string());
            assert_eq!(operations, vec![WorkflowMutation::MoveNode(MoveNode { node_id: "node-1".into(), x: 140.0, y: 120.0 })]);
        }

        #[test]
        fn flow_fixture_diff_connects_disconnects_and_removes() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", 200.0, 0.0));
            graph.edges.push(WorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "node-1".into(),
                source_port_id: "node-1:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "node-2:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            let mut fixture = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
            fixture["synapses"] = json!([
                { "id": "", "from": "node-2", "fromPort": "node-2:out", "to": "node-1", "toPort": "node-1:in" }
            ]);
            let operations = apply_flow_fixture_to_os_workflow(&graph, &fixture.to_string());
            assert!(matches!(
                &operations[0],
                WorkflowMutation::ConnectPorts(ConnectPorts { edge }) if edge.source_node_id == "node-2" && edge.target_port_id == "node-1:in" && !edge.id.is_empty()
            ));
            assert!(operations.contains(&WorkflowMutation::DisconnectEdge(DisconnectEdge { edge_id: "edge-1".into() })));
            let mut removal = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
            removal["widgets"] = json!([{ "id": "node-1" }]);
            removal["synapses"] = json!([]);
            let removal_operations = apply_flow_fixture_to_os_workflow(&graph, &removal.to_string());
            assert!(removal_operations.contains(&WorkflowMutation::RemoveNode(RemoveNode { node_id: "node-2".into() })));
            assert!(!removal_operations.iter().any(|operation| matches!(operation, WorkflowMutation::DisconnectEdge(DisconnectEdge { .. }))));
        }

        //#region 🔖️WorkflowPlanner
        fn dirty_set(node_ids: &[&str]) -> HashSet<String> {
            node_ids.iter().map(|id| id.to_string()).collect()
        }

        #[test]
        fn plans_a_single_delivery_across_one_dirty_edge() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", 200.0, 0.0));
            graph.edges.push(WorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "node-1".into(),
                source_port_id: "node-1:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "node-2:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            let deliveries = plan_workflow(&graph, &dirty_set(&["node-1"]));
            assert_eq!(deliveries, vec![WorkflowDelivery { edge_id: "edge-1".into(), producer_node_id: "node-1".into(), producer_port_id: "node-1:out".into(), consumer_node_id: "node-2".into(), consumer_port_id: "node-2:in".into() }]);
        }

        #[test]
        fn plans_a_chain_in_topological_order_when_only_the_root_is_dirty() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", 200.0, 0.0));
            graph.nodes.push(media_node("node-3", 400.0, 0.0));
            graph.edges.push(WorkflowEdge {
                id: "edge-ab".into(),
                source_node_id: "node-1".into(),
                source_port_id: "node-1:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "node-2:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            graph.edges.push(WorkflowEdge {
                id: "edge-bc".into(),
                source_node_id: "node-2".into(),
                source_port_id: "node-2:out".into(),
                target_node_id: "node-3".into(),
                target_port_id: "node-3:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            let deliveries = plan_workflow(&graph, &dirty_set(&["node-1"]));
            assert_eq!(deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect::<Vec<_>>(), vec!["edge-ab", "edge-bc"], "A→B must be planned before B→C");
        }

        #[test]
        fn plans_a_diamond_with_one_delivery_per_incoming_edge() {
            // 🔀️ One delivery per edge, not per node: D has two producers (B and C), so D is the
            // target of two separate deliveries rather than a single merged one.
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-a", 0.0, 0.0));
            graph.nodes.push(media_node("node-b", 200.0, -80.0));
            graph.nodes.push(media_node("node-c", 200.0, 80.0));
            graph.nodes.push(media_node("node-d", 400.0, 0.0));
            graph.edges.push(WorkflowEdge {
                id: "edge-ab".into(),
                source_node_id: "node-a".into(),
                source_port_id: "node-a:out".into(),
                target_node_id: "node-b".into(),
                target_port_id: "node-b:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            graph.edges.push(WorkflowEdge {
                id: "edge-ac".into(),
                source_node_id: "node-a".into(),
                source_port_id: "node-a:out".into(),
                target_node_id: "node-c".into(),
                target_port_id: "node-c:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            graph.edges.push(WorkflowEdge {
                id: "edge-bd".into(),
                source_node_id: "node-b".into(),
                source_port_id: "node-b:out".into(),
                target_node_id: "node-d".into(),
                target_port_id: "node-d:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            graph.edges.push(WorkflowEdge {
                id: "edge-cd".into(),
                source_node_id: "node-c".into(),
                source_port_id: "node-c:out".into(),
                target_node_id: "node-d".into(),
                target_port_id: "node-d:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            let deliveries = plan_workflow(&graph, &dirty_set(&["node-a"]));
            let edge_ids: Vec<&str> = deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect();
            assert_eq!(edge_ids.len(), 4);
            let index_of = |id: &str| edge_ids.iter().position(|candidate| *candidate == id).unwrap();
            assert!(index_of("edge-bd") > index_of("edge-ab"), "B→D must be planned after A→B");
            assert!(index_of("edge-cd") > index_of("edge-ac"), "C→D must be planned after A→C");
        }

        #[test]
        fn plans_nothing_when_no_instance_is_dirty() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", 200.0, 0.0));
            graph.edges.push(WorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "node-1".into(),
                source_port_id: "node-1:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "node-2:in".into(),
                contract: placeholder_media_contract("2d.drawing"),
            });
            assert!(plan_workflow(&graph, &dirty_set(&[])).is_empty());
        }

        #[test]
        fn plans_nothing_for_a_dirty_node_with_no_outgoing_edges() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", 0.0, 0.0));
            assert!(plan_workflow(&graph, &dirty_set(&["node-1"])).is_empty());
        }

        /// 🔬️ Shared fixtures replay (`framework/product/os/core/fixtures/*.dsl`) — the same files
        /// drive `planWorkflow`'s vitest harness in `js/index.ts` (decoded there via the sibling
        /// `.spk` through a wasm export), keeping the two implementations in lockstep. See
        /// `framework/product/os/core/fixtures/README.md`.
        fn workflow_fixture_dsl_paths() -> Vec<std::path::PathBuf> {
            let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures");
            let entries = std::fs::read_dir(&fixtures_dir).unwrap_or_else(|error| panic!("read fixtures dir {fixtures_dir:?}: {error}"));
            let mut paths: Vec<std::path::PathBuf> = entries.map(|entry| entry.expect("dir entry").path()).filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("dsl")).collect();
            paths.sort();
            paths
        }

        #[test]
        fn workflow_fixtures_match_expected_deliveries() {
            let paths = workflow_fixture_dsl_paths();
            for path in &paths {
                let contents = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read fixture {path:?}: {error}"));
                let fixture = <WorkflowFixture as store::ArtifactDsl>::parse_dsl(&contents).unwrap_or_else(|error| panic!("parse fixture {path:?}: {error}"));
                let dirty: HashSet<String> = fixture.dirty_node_ids.iter().cloned().collect();
                let deliveries = plan_workflow(&fixture.graph, &dirty);
                assert_eq!(deliveries, fixture.expected_deliveries, "fixture {} mismatch", fixture.name);
            }
            assert!(paths.len() >= 5, "expected workflow fixtures in fixtures dir, found {}", paths.len());
        }

        /// 🧬️ Every fixture ships as a `.dsl`/`.spk` pair: both must decode to the identical
        /// `WorkflowFixture`, the `.dsl` text must already be its own canonical `print_dsl`
        /// fixpoint, and the `.spk` bytes must match a fresh canonical `encode_pack()` of the
        /// parsed document byte-for-byte (canonical pack encoding is deterministic, independent of
        /// field-map iteration order — see `store`'s pack facade docs).
        #[test]
        fn workflow_fixture_dsl_and_spk_pairs_are_canonical_and_equivalent() {
            let paths = workflow_fixture_dsl_paths();
            for dsl_path in &paths {
                let file_name = dsl_path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
                let spk_name = if file_name.starts_with("🗣️") { file_name.replacen("🗣️", "📦️", 1).replace(".dsl", ".spk") } else { file_name.replace(".dsl", ".spk") };
                let spk_path = dsl_path.with_file_name(spk_name);
                let dsl_text = std::fs::read_to_string(dsl_path).unwrap_or_else(|error| panic!("read {dsl_path:?}: {error}"));
                let spk_bytes = std::fs::read(&spk_path).unwrap_or_else(|error| panic!("read {spk_path:?}: {error}"));
                let via_dsl = <WorkflowFixture as store::ArtifactDsl>::parse_dsl(&dsl_text).unwrap_or_else(|error| panic!("parse {dsl_path:?}: {error}"));
                let via_pack = <WorkflowFixture as store::ArtifactPack>::decode_pack(&spk_bytes).unwrap_or_else(|error| panic!("decode {spk_path:?}: {error}"));
                assert_eq!(via_dsl, via_pack, "{dsl_path:?} and {spk_path:?} decode to different documents");
                assert_eq!(store::ArtifactDsl::print_dsl(&via_dsl), dsl_text, "{dsl_path:?} is not its own canonical print_dsl fixpoint");
                assert_eq!(store::ArtifactPack::encode_pack(&via_dsl), spk_bytes, "{spk_path:?} does not match a fresh canonical encode_pack()");
                store::test_support::assert_dsl_pack_equivalence(&via_dsl);
            }
        }
        //#endregion 🔖️WorkflowPlanner
    }
    //#endregion 🧪️Tests
    // #endregion workflow
}

pub mod codec_abi {
    //#region 🧬️Schema
    use semio_framework::{
        AbiBytes, AbiControl, AbiCursorStep, AbiError, AbiErrorCode, AbiEvent, AbiEventCode, AbiHandle, AbiHandleTable, AbiMessageBytes, AbiOperation, AbiPage, AbiPageReader, AbiRejectedPage, AbiReply, AbiRequest, AbiRequestId, AbiStatus,
        AbiStatusCode, AbiWorkBudget, ABI_MAX_BODY_BYTES, ABI_MAX_MESSAGE_BYTES, ABI_MAX_PAGES_PER_TRANSFER, ABI_MAX_PAGE_BYTES,
    };

    pub const OS_HOST_CODEC_SCHEMA_JSON: &str = include_str!("🧬️schema/🔣️codec-abi.json");
    pub const OS_HOST_CODEC_LEDGER_FIXTURE: &str = include_str!("🧪️fixtures/📒️codec-abi.tsv");
    pub const OS_HOST_CODEC_MAX_INPUT_BYTES: usize = ABI_MAX_BODY_BYTES;
    pub const OS_HOST_CODEC_MAX_OUTPUT_BYTES: usize = ABI_MAX_BODY_BYTES;
    pub const OS_HOST_CODEC_MAX_KIND_COUNT: usize = 256;
    pub const OS_HOST_CODEC_MAX_KIND_BYTES: usize = ABI_MAX_MESSAGE_BYTES;
    pub const OS_HOST_CODEC_PROGRESS_EVENT: u16 = 1;
    const WORKFLOW_PACK_MAGIC: [u8; 4] = *b"WFP1";
    const WORKFLOW_PACK_HEADER_BYTES: usize = 9;

    #[repr(u16)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OsHostCodecOperation {
        DecodeWorkflowFixturePack = 1537,
        ParseWorkflowFixtureDsl = 1538,
        MediaAcceptFilterKinds = 1539,
        NormalizeStdioFormatKind = 1540,
    }

    impl OsHostCodecOperation {
        pub fn from_abi(operation: AbiOperation) -> Result<Self, AbiErrorCode> {
            match operation.get() {
                1537 => Ok(Self::DecodeWorkflowFixturePack),
                1538 => Ok(Self::ParseWorkflowFixtureDsl),
                1539 => Ok(Self::MediaAcceptFilterKinds),
                1540 => Ok(Self::NormalizeStdioFormatKind),
                _ => Err(AbiErrorCode::UnknownOperation),
            }
        }

        pub fn abi(self) -> AbiOperation {
            AbiOperation::try_new(self as u16).expect("schema operation codes are bounded and non-zero")
        }

        const fn reply_kind(self) -> u8 {
            match self {
                Self::DecodeWorkflowFixturePack | Self::ParseWorkflowFixtureDsl => 1,
                Self::MediaAcceptFilterKinds => 2,
                Self::NormalizeStdioFormatKind => 3,
            }
        }
    }

    #[repr(u16)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OsHostCodecErrorCode {
        MalformedRequest = 1,
        MalformedPack = 2,
        MalformedDsl = 3,
        MissingKindArray = 4,
        UnknownKind = 5,
        InvalidUtf8 = 6,
        InputLimit = 7,
        OutputLimit = 8,
        InvalidState = 9,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct OsHostCodecFailure {
        pub code: OsHostCodecErrorCode,
        pub message: Vec<u8>,
    }

    impl OsHostCodecFailure {
        fn fixed(code: OsHostCodecErrorCode, message: &'static str) -> Self {
            Self { code, message: message.as_bytes().to_vec() }
        }

        fn abi_code(&self) -> AbiErrorCode {
            match self.code {
                OsHostCodecErrorCode::MalformedRequest | OsHostCodecErrorCode::MalformedPack | OsHostCodecErrorCode::MalformedDsl | OsHostCodecErrorCode::InvalidState => AbiErrorCode::MalformedTag,
                OsHostCodecErrorCode::MissingKindArray => AbiErrorCode::MissingField,
                OsHostCodecErrorCode::UnknownKind => AbiErrorCode::UnknownOperation,
                OsHostCodecErrorCode::InvalidUtf8 => AbiErrorCode::InvalidUtf8,
                OsHostCodecErrorCode::InputLimit | OsHostCodecErrorCode::OutputLimit => AbiErrorCode::LimitExceeded,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct OsHostCodecFormat {
        pub short_id: String,
        pub extensions: Vec<String>,
    }

    trait OsHostFormatResolver {
        fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure>;
    }

    #[cfg(feature = "os-host-full")]
    #[derive(Default)]
    struct RegisteredOsHostFormatResolver;

    #[cfg(feature = "os-host-full")]
    impl OsHostFormatResolver for RegisteredOsHostFormatResolver {
        fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
            semio_framework::format_descriptor(kind)
                .map(|row| row.map(|row| OsHostCodecFormat { short_id: row.short_id, extensions: row.extensions }))
                .map_err(|_| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "stdio format registry unavailable"))
        }
    }
    //#endregion 🧬️Schema

    //#region ⏳️RetainedOperation
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    #[repr(u8)]
    pub enum OsHostCodecPhase {
        Input = 1,
        Decode = 2,
        Output = 3,
        AwaitingAcknowledgement = 4,
        Reply = 5,
        Closing = 6,
        Closed = 7,
        Cancelled = 8,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OsHostCodecStepState {
        Progress,
        InputAcknowledged,
        OutputPage,
        AwaitingAcknowledgement,
        Reply,
        Idle,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct OsHostCodecStep {
        pub state: OsHostCodecStepState,
        pub event: AbiEvent,
        pub input_acknowledgement: Option<AbiControl>,
        pub page: Option<AbiPage>,
        pub reply: Option<AbiReply>,
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct OsHostCodecCancelOutcome {
        pub page: Option<AbiPage>,
        pub admitted_byte_credits: usize,
        pub copied_bytes: usize,
    }

    struct PendingInputPage {
        page: AbiPage,
        cursor: usize,
        retire_cursor: usize,
    }

    #[derive(Clone, Copy)]
    enum WorkflowStructuralKind {
        Pack,
        Dsl,
    }

    struct Utf8Cursor {
        remaining: u8,
        next_minimum: u8,
        next_maximum: u8,
        invalid: bool,
    }

    impl Utf8Cursor {
        fn new() -> Self {
            Self { remaining: 0, next_minimum: 0x80, next_maximum: 0xbf, invalid: false }
        }

        fn feed(&mut self, byte: u8) {
            if self.invalid {
                return;
            }
            if self.remaining != 0 {
                if byte < self.next_minimum || byte > self.next_maximum {
                    self.invalid = true;
                    return;
                }
                self.remaining -= 1;
                self.next_minimum = 0x80;
                self.next_maximum = 0xbf;
                return;
            }
            match byte {
                0x00..=0x7f => {}
                0xc2..=0xdf => self.remaining = 1,
                0xe0 => {
                    self.remaining = 2;
                    self.next_minimum = 0xa0;
                }
                0xe1..=0xec | 0xee..=0xef => self.remaining = 2,
                0xed => {
                    self.remaining = 2;
                    self.next_maximum = 0x9f;
                }
                0xf0 => {
                    self.remaining = 3;
                    self.next_minimum = 0x90;
                }
                0xf1..=0xf3 => self.remaining = 3,
                0xf4 => {
                    self.remaining = 3;
                    self.next_maximum = 0x8f;
                }
                _ => self.invalid = true,
            }
        }

        fn complete(&self) -> bool {
            !self.invalid && self.remaining == 0
        }
    }

    struct PatternCursor {
        index: usize,
        found: bool,
    }

    impl PatternCursor {
        fn new() -> Self {
            Self { index: 0, found: false }
        }

        fn feed(&mut self, byte: u8, pattern: &[u8]) {
            if self.found {
                return;
            }
            if byte == pattern[self.index] {
                self.index += 1;
                self.found = self.index == pattern.len();
            } else {
                self.index = usize::from(byte == pattern[0]);
            }
        }
    }

    struct WorkflowStructuralCursor {
        kind: WorkflowStructuralKind,
        declared_input_bytes: usize,
        pack_header: [u8; WORKFLOW_PACK_HEADER_BYTES],
        pack_header_cursor: usize,
        declared_canonical_bytes: Option<usize>,
        canonical_bytes: usize,
        payload: Option<Vec<u8>>,
        utf8: Utf8Cursor,
        prefix_cursor: usize,
        name_has_value: bool,
        name_line_finished: bool,
        braces: usize,
        brackets: usize,
        quoted: bool,
        escaped: bool,
        last_byte: Option<u8>,
        graph: PatternCursor,
        dirty: PatternCursor,
        deliveries: PatternCursor,
        failure: Option<OsHostCodecFailure>,
    }

    impl WorkflowStructuralCursor {
        fn new(operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
            let kind = if operation == OsHostCodecOperation::DecodeWorkflowFixturePack { WorkflowStructuralKind::Pack } else { WorkflowStructuralKind::Dsl };
            let mut cursor = Self {
                kind,
                declared_input_bytes,
                pack_header: [0; WORKFLOW_PACK_HEADER_BYTES],
                pack_header_cursor: 0,
                declared_canonical_bytes: None,
                canonical_bytes: 0,
                payload: None,
                utf8: Utf8Cursor::new(),
                prefix_cursor: 0,
                name_has_value: false,
                name_line_finished: false,
                braces: 0,
                brackets: 0,
                quoted: false,
                escaped: false,
                last_byte: None,
                graph: PatternCursor::new(),
                dirty: PatternCursor::new(),
                deliveries: PatternCursor::new(),
                failure: None,
            };
            if matches!(kind, WorkflowStructuralKind::Dsl) {
                cursor.declared_canonical_bytes = Some(declared_input_bytes);
                cursor.install_payload_header(declared_input_bytes);
            }
            cursor
        }

        fn malformed_code(&self) -> OsHostCodecErrorCode {
            match self.kind {
                WorkflowStructuralKind::Pack => OsHostCodecErrorCode::MalformedPack,
                WorkflowStructuralKind::Dsl => OsHostCodecErrorCode::MalformedDsl,
            }
        }

        fn fail(&mut self, code: OsHostCodecErrorCode, message: &'static str) {
            if self.failure.is_none() {
                self.failure = Some(OsHostCodecFailure::fixed(code, message));
            }
        }

        fn install_payload_header(&mut self, canonical_bytes: usize) {
            let Some(payload_bytes) = canonical_bytes.checked_add(6) else {
                self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                return;
            };
            if payload_bytes > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
                self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                return;
            }
            let mut payload = Vec::with_capacity(payload_bytes);
            payload.push(1);
            payload.push(OsHostCodecOperation::ParseWorkflowFixtureDsl.reply_kind());
            payload.extend_from_slice(&(canonical_bytes as u32).to_le_bytes());
            self.payload = Some(payload);
        }

        fn feed(&mut self, byte: u8) {
            if matches!(self.kind, WorkflowStructuralKind::Pack) && self.pack_header_cursor < WORKFLOW_PACK_HEADER_BYTES {
                self.pack_header[self.pack_header_cursor] = byte;
                self.pack_header_cursor += 1;
                if self.pack_header_cursor == WORKFLOW_PACK_HEADER_BYTES {
                    self.finish_pack_header();
                }
                return;
            }
            self.feed_dsl(byte);
        }

        fn finish_pack_header(&mut self) {
            if self.pack_header[..4] != WORKFLOW_PACK_MAGIC || self.pack_header[4] != 1 {
                self.fail(OsHostCodecErrorCode::MalformedPack, "malformed workflow fixture structural pack");
                return;
            }
            let canonical_bytes = u32::from_le_bytes(self.pack_header[5..9].try_into().expect("fixed structural pack length")) as usize;
            self.declared_canonical_bytes = Some(canonical_bytes);
            if canonical_bytes.checked_add(WORKFLOW_PACK_HEADER_BYTES) != Some(self.declared_input_bytes) {
                self.fail(OsHostCodecErrorCode::MalformedPack, "malformed workflow fixture structural pack length");
                return;
            }
            self.install_payload_header(canonical_bytes);
        }

        fn feed_dsl(&mut self, byte: u8) {
            self.canonical_bytes = self.canonical_bytes.saturating_add(1);
            self.utf8.feed(byte);
            if let Some(payload) = self.payload.as_mut() {
                payload.push(byte);
            }
            const PREFIX: &[u8] = b"name=";
            if self.prefix_cursor < PREFIX.len() {
                if byte == PREFIX[self.prefix_cursor] {
                    self.prefix_cursor += 1;
                } else {
                    self.fail(self.malformed_code(), "workflow fixture DSL is not canonical");
                }
            } else if !self.name_line_finished {
                if byte == b'\n' {
                    self.name_line_finished = true;
                } else if !byte.is_ascii_whitespace() {
                    self.name_has_value = true;
                }
            }
            self.graph.feed(byte, b"\ngraph {");
            self.dirty.feed(byte, b"\ndirty-node-ids=[");
            self.deliveries.feed(byte, b"\nexpected-deliveries ");
            if byte.is_ascii() {
                self.feed_ascii_structure(byte);
            }
            self.last_byte = Some(byte);
        }

        fn feed_ascii_structure(&mut self, byte: u8) {
            if self.quoted {
                if self.escaped {
                    self.escaped = false;
                } else if byte == b'\\' {
                    self.escaped = true;
                } else if byte == b'"' {
                    self.quoted = false;
                }
                return;
            }
            match byte {
                b'"' => self.quoted = true,
                b'{' => self.braces += 1,
                b'}' if self.braces != 0 => self.braces -= 1,
                b'}' => self.fail(self.malformed_code(), "workflow fixture DSL has an unmatched closing brace"),
                b'[' => self.brackets += 1,
                b']' if self.brackets != 0 => self.brackets -= 1,
                b']' => self.fail(self.malformed_code(), "workflow fixture DSL has an unmatched closing bracket"),
                b'\n' | b' '..=b'~' => {}
                _ => self.fail(self.malformed_code(), "workflow fixture DSL contains a non-canonical control byte"),
            }
        }

        fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
            if matches!(self.kind, WorkflowStructuralKind::Pack) && self.pack_header_cursor != WORKFLOW_PACK_HEADER_BYTES {
                self.fail(OsHostCodecErrorCode::MalformedPack, "truncated workflow fixture structural pack header");
            }
            if !self.utf8.complete() {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "workflow fixture DSL is not UTF-8");
            }
            if self.declared_canonical_bytes != Some(self.canonical_bytes) {
                self.fail(self.malformed_code(), "truncated workflow fixture structural payload");
            }
            if self.prefix_cursor != 5 || !self.name_has_value || !self.graph.found || !self.dirty.found || !self.deliveries.found || self.braces != 0 || self.brackets != 0 || self.quoted || self.escaped || self.last_byte != Some(b'\n') {
                self.fail(self.malformed_code(), "workflow fixture DSL is not canonical");
            }
            if let Some(failure) = self.failure.clone() {
                return Err(failure);
            }
            self.payload.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "workflow fixture structural output is unavailable"))
        }

        fn close_one(&mut self) -> bool {
            if self.payload.as_mut().is_some_and(|payload| payload.pop().is_some()) {
                return false;
            }
            if self.pack_header_cursor != 0 {
                self.pack_header_cursor -= 1;
                self.pack_header[self.pack_header_cursor] = 0;
                return false;
            }
            true
        }

        fn terminal_is_empty(&self) -> bool {
            self.payload.as_ref().is_none_or(Vec::is_empty) && self.pack_header_cursor == 0
        }
    }

    struct FilterKindsStructuralCursor {
        header: [u8; 3],
        header_cursor: usize,
        count: usize,
        index: usize,
        length: [u8; 2],
        length_cursor: usize,
        expected_kind_bytes: usize,
        kind: [u8; OS_HOST_CODEC_MAX_KIND_BYTES],
        kind_cursor: usize,
        utf8: Utf8Cursor,
        output: Option<Vec<u8>>,
        complete: bool,
        failure: Option<OsHostCodecFailure>,
    }

    impl FilterKindsStructuralCursor {
        fn new() -> Self {
            Self {
                header: [0; 3],
                header_cursor: 0,
                count: 0,
                index: 0,
                length: [0; 2],
                length_cursor: 0,
                expected_kind_bytes: 0,
                kind: [0; OS_HOST_CODEC_MAX_KIND_BYTES],
                kind_cursor: 0,
                utf8: Utf8Cursor::new(),
                output: Some(Vec::new()),
                complete: false,
                failure: None,
            }
        }

        fn fail(&mut self, code: OsHostCodecErrorCode, message: &'static str) {
            if self.failure.is_none() {
                self.failure = Some(OsHostCodecFailure::fixed(code, message));
            }
        }

        fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
            if self.failure.is_some() {
                return;
            }
            if self.complete {
                self.fail(OsHostCodecErrorCode::MalformedRequest, "trailing stdio format kind array bytes");
                return;
            }
            if self.header_cursor < self.header.len() {
                self.header[self.header_cursor] = byte;
                self.header_cursor += 1;
                if self.header_cursor == self.header.len() {
                    self.finish_header();
                }
                return;
            }
            if self.length_cursor < self.length.len() {
                self.length[self.length_cursor] = byte;
                self.length_cursor += 1;
                if self.length_cursor == self.length.len() {
                    self.expected_kind_bytes = u16::from_le_bytes(self.length) as usize;
                    if self.expected_kind_bytes > OS_HOST_CODEC_MAX_KIND_BYTES {
                        self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind exceeds input limit");
                    } else if self.expected_kind_bytes == 0 {
                        self.finish_kind(resolver);
                    }
                }
                return;
            }
            if self.kind_cursor >= self.expected_kind_bytes || self.kind_cursor >= self.kind.len() {
                self.fail(OsHostCodecErrorCode::MalformedRequest, "malformed stdio format kind array");
                return;
            }
            self.kind[self.kind_cursor] = byte;
            self.kind_cursor += 1;
            self.utf8.feed(byte);
            if self.utf8.invalid {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            } else if self.kind_cursor == self.expected_kind_bytes {
                if self.utf8.complete() {
                    self.finish_kind(resolver);
                } else {
                    self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                }
            }
        }

        fn finish_header(&mut self) {
            if self.header[0] != 1 {
                self.fail(OsHostCodecErrorCode::MalformedRequest, "invalid stdio format kind array version");
                return;
            }
            self.count = u16::from_le_bytes([self.header[1], self.header[2]]) as usize;
            if self.count > OS_HOST_CODEC_MAX_KIND_COUNT {
                self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind array exceeds item limit");
            } else if self.count == 0 {
                self.complete = true;
            }
        }

        fn finish_kind<R: OsHostFormatResolver>(&mut self, resolver: &mut R) {
            let kind = match std::str::from_utf8(&self.kind[..self.kind_cursor]) {
                Ok(kind) => kind,
                Err(_) => {
                    self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                    return;
                }
            };
            let row = match resolver.resolve_format(kind) {
                Ok(Some(row)) => row,
                Ok(None) => {
                    self.fail(OsHostCodecErrorCode::UnknownKind, "unknown stdio format kind");
                    return;
                }
                Err(failure) => {
                    self.failure = Some(failure);
                    return;
                }
            };
            for extension in row.extensions {
                let output = self.output.as_mut().expect("filter output is retained until seal");
                let separator = usize::from(!output.is_empty());
                let Some(output_bytes) = output.len().checked_add(separator).and_then(|bytes| bytes.checked_add(extension.len())).and_then(|bytes| bytes.checked_add(6)) else {
                    self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                    return;
                };
                if output_bytes > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
                    self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit");
                    return;
                }
                if separator != 0 {
                    output.push(b',');
                }
                output.extend_from_slice(extension.as_bytes());
            }
            self.index += 1;
            self.length = [0; 2];
            self.length_cursor = 0;
            self.expected_kind_bytes = 0;
            self.kind[..self.kind_cursor].fill(0);
            self.kind_cursor = 0;
            self.utf8 = Utf8Cursor::new();
            self.complete = self.index == self.count;
        }

        fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
            if self.header_cursor == 0 {
                self.fail(OsHostCodecErrorCode::MissingKindArray, "missing stdio format kind array");
            } else if self.header_cursor != self.header.len() || !self.complete || self.length_cursor != 0 || self.kind_cursor != 0 {
                self.fail(OsHostCodecErrorCode::MalformedRequest, "truncated stdio format kind array");
            }
            if let Some(failure) = self.failure.take() {
                return Err(failure);
            }
            self.output.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "stdio format filter output is unavailable"))
        }

        fn close_one(&mut self) -> bool {
            if self.output.as_mut().is_some_and(|output| output.pop().is_some()) {
                return false;
            }
            if self.kind_cursor != 0 {
                self.kind_cursor -= 1;
                self.kind[self.kind_cursor] = 0;
                return false;
            }
            if self.length_cursor != 0 {
                self.length_cursor -= 1;
                self.length[self.length_cursor] = 0;
                return false;
            }
            if self.header_cursor != 0 {
                self.header_cursor -= 1;
                self.header[self.header_cursor] = 0;
                return false;
            }
            true
        }

        fn terminal_is_empty(&self) -> bool {
            self.output.as_ref().is_none_or(Vec::is_empty) && self.kind_cursor == 0 && self.length_cursor == 0 && self.header_cursor == 0
        }
    }

    struct NormalizeKindStructuralCursor {
        declared_input_bytes: usize,
        kind: [u8; OS_HOST_CODEC_MAX_KIND_BYTES],
        kind_cursor: usize,
        utf8: Utf8Cursor,
        output: Option<Vec<u8>>,
        failure: Option<OsHostCodecFailure>,
    }

    impl NormalizeKindStructuralCursor {
        fn new(declared_input_bytes: usize) -> Self {
            let failure = (declared_input_bytes == 0).then(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::MalformedRequest, "missing stdio format kind"));
            Self { declared_input_bytes, kind: [0; OS_HOST_CODEC_MAX_KIND_BYTES], kind_cursor: 0, utf8: Utf8Cursor::new(), output: None, failure }
        }

        fn fail(&mut self, code: OsHostCodecErrorCode, message: &'static str) {
            if self.failure.is_none() {
                self.failure = Some(OsHostCodecFailure::fixed(code, message));
            }
        }

        fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
            if self.failure.is_some() {
                return;
            }
            if self.kind_cursor >= self.declared_input_bytes || self.kind_cursor >= self.kind.len() {
                self.fail(OsHostCodecErrorCode::InputLimit, "stdio format kind exceeds input limit");
                return;
            }
            self.kind[self.kind_cursor] = byte;
            self.kind_cursor += 1;
            self.utf8.feed(byte);
            if self.utf8.invalid {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            } else if self.kind_cursor == self.declared_input_bytes {
                if self.utf8.complete() {
                    self.resolve(resolver);
                } else {
                    self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                }
            }
        }

        fn resolve<R: OsHostFormatResolver>(&mut self, resolver: &mut R) {
            let kind = match std::str::from_utf8(&self.kind[..self.kind_cursor]) {
                Ok(kind) => kind,
                Err(_) => {
                    self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
                    return;
                }
            };
            match resolver.resolve_format(kind) {
                Ok(Some(row)) if row.short_id.len().checked_add(6).is_some_and(|bytes| bytes <= OS_HOST_CODEC_MAX_OUTPUT_BYTES) => self.output = Some(row.short_id.into_bytes()),
                Ok(Some(_)) => self.fail(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"),
                Ok(None) => self.fail(OsHostCodecErrorCode::UnknownKind, "unknown stdio format kind"),
                Err(failure) => self.failure = Some(failure),
            }
        }

        fn finish(&mut self) -> Result<Vec<u8>, OsHostCodecFailure> {
            if self.kind_cursor != self.declared_input_bytes {
                self.fail(OsHostCodecErrorCode::MalformedRequest, "truncated stdio format kind");
            } else if !self.utf8.complete() {
                self.fail(OsHostCodecErrorCode::InvalidUtf8, "stdio format kind is not UTF-8");
            }
            if let Some(failure) = self.failure.take() {
                return Err(failure);
            }
            self.output.take().ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::InvalidState, "normalized stdio format kind output is unavailable"))
        }

        fn close_one(&mut self) -> bool {
            if self.output.as_mut().is_some_and(|output| output.pop().is_some()) {
                return false;
            }
            if self.kind_cursor != 0 {
                self.kind_cursor -= 1;
                self.kind[self.kind_cursor] = 0;
                return false;
            }
            true
        }

        fn terminal_is_empty(&self) -> bool {
            self.output.as_ref().is_none_or(Vec::is_empty) && self.kind_cursor == 0
        }
    }

    enum OsHostCodecInput {
        Workflow(WorkflowStructuralCursor),
        Filter(FilterKindsStructuralCursor),
        Normalize(NormalizeKindStructuralCursor),
    }

    impl OsHostCodecInput {
        fn new(operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
            match operation {
                OsHostCodecOperation::DecodeWorkflowFixturePack | OsHostCodecOperation::ParseWorkflowFixtureDsl => Self::Workflow(WorkflowStructuralCursor::new(operation, declared_input_bytes)),
                OsHostCodecOperation::MediaAcceptFilterKinds => Self::Filter(FilterKindsStructuralCursor::new()),
                OsHostCodecOperation::NormalizeStdioFormatKind => Self::Normalize(NormalizeKindStructuralCursor::new(declared_input_bytes)),
            }
        }

        fn feed<R: OsHostFormatResolver>(&mut self, byte: u8, resolver: &mut R) {
            match self {
                Self::Workflow(cursor) => cursor.feed(byte),
                Self::Filter(cursor) => cursor.feed(byte, resolver),
                Self::Normalize(cursor) => cursor.feed(byte, resolver),
            }
        }

        fn workflow(&mut self) -> Option<&mut WorkflowStructuralCursor> {
            match self {
                Self::Workflow(cursor) => Some(cursor),
                _ => None,
            }
        }

        fn filter(&mut self) -> Option<&mut FilterKindsStructuralCursor> {
            match self {
                Self::Filter(cursor) => Some(cursor),
                _ => None,
            }
        }

        fn normalize(&mut self) -> Option<&mut NormalizeKindStructuralCursor> {
            match self {
                Self::Normalize(cursor) => Some(cursor),
                _ => None,
            }
        }

        fn retire_one(&mut self) -> bool {
            match self {
                Self::Workflow(cursor) => !cursor.close_one(),
                Self::Filter(cursor) => !cursor.close_one(),
                Self::Normalize(cursor) => !cursor.close_one(),
            }
        }

        fn terminal_is_empty(&self) -> bool {
            match self {
                Self::Workflow(cursor) => cursor.terminal_is_empty(),
                Self::Filter(cursor) => cursor.terminal_is_empty(),
                Self::Normalize(cursor) => cursor.terminal_is_empty(),
            }
        }
    }

    struct OsHostCodecSession {
        request: AbiRequest,
        operation: OsHostCodecOperation,
        handle: Option<AbiHandle>,
        declared_input_bytes: usize,
        input_bytes_received: usize,
        input: OsHostCodecInput,
        pending_input: Option<PendingInputPage>,
        next_input_index: u32,
        sealed: bool,
        decoded: bool,
        output: Option<AbiPageReader>,
        output_bytes: usize,
        output_copied: usize,
        reply_emitted: bool,
        terminal_failure: Option<OsHostCodecFailure>,
        sequence: u32,
        cancelled: bool,
        closing: bool,
    }

    impl OsHostCodecSession {
        fn new(request: AbiRequest, operation: OsHostCodecOperation, declared_input_bytes: usize) -> Self {
            Self {
                request,
                operation,
                handle: None,
                declared_input_bytes,
                input_bytes_received: 0,
                input: OsHostCodecInput::new(operation, declared_input_bytes),
                pending_input: None,
                next_input_index: 0,
                sealed: false,
                decoded: false,
                output: None,
                output_bytes: 0,
                output_copied: 0,
                reply_emitted: false,
                terminal_failure: None,
                sequence: 0,
                cancelled: false,
                closing: false,
            }
        }

        fn handle(&self) -> AbiHandle {
            self.handle.expect("service assigns a handle before exposing a session")
        }

        fn event(&mut self, phase: OsHostCodecPhase, completed: usize, total: usize) -> AbiEvent {
            self.sequence = self.sequence.saturating_add(1);
            let mut body = Vec::with_capacity(18);
            body.push(1);
            body.push(phase as u8);
            body.extend_from_slice(&(completed as u64).to_le_bytes());
            body.extend_from_slice(&(total as u64).to_le_bytes());
            AbiEvent {
                request_id: self.request.request_id,
                generation: self.request.generation,
                sequence: self.sequence,
                event: AbiEventCode::try_new(OS_HOST_CODEC_PROGRESS_EVENT).expect("schema event code is bounded"),
                status: AbiStatus::OK,
                bytes: AbiBytes::try_new(body).expect("fixed progress body is bounded"),
            }
        }

        fn step_result(&mut self, state: OsHostCodecStepState, phase: OsHostCodecPhase, completed: usize, total: usize, input_acknowledgement: Option<AbiControl>, page: Option<AbiPage>, reply: Option<AbiReply>) -> OsHostCodecStep {
            OsHostCodecStep { state, event: self.event(phase, completed, total), input_acknowledgement, page, reply }
        }

        fn offer(&mut self, page: AbiPage) -> Result<(), AbiRejectedPage> {
            let reject = |code, page: AbiPage| AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() };
            if page.handle != self.handle() {
                let code = classify_handle(self.handle(), page.handle);
                return Err(reject(code, page));
            }
            if self.cancelled {
                return Err(reject(AbiErrorCode::Cancelled, page));
            }
            if self.closing {
                return Err(reject(AbiErrorCode::Closed, page));
            }
            if self.sealed {
                return Err(reject(AbiErrorCode::Sealed, page));
            }
            if self.pending_input.is_some() {
                return Err(reject(AbiErrorCode::Busy, page));
            }
            let total = self.input_bytes_received.checked_add(page.bytes.len());
            if page.index >= ABI_MAX_PAGES_PER_TRANSFER || page.bytes.len() > ABI_MAX_PAGE_BYTES || total.is_none_or(|total| total > self.declared_input_bytes || total > OS_HOST_CODEC_MAX_INPUT_BYTES) {
                return Err(reject(AbiErrorCode::LimitExceeded, page));
            }
            if page.index != self.next_input_index {
                return Err(reject(AbiErrorCode::OutOfOrderPage, page));
            }
            self.pending_input = Some(PendingInputPage { page, cursor: 0, retire_cursor: 0 });
            Ok(())
        }

        fn seal(&mut self) -> Result<(), AbiErrorCode> {
            if self.cancelled {
                return Err(AbiErrorCode::Cancelled);
            }
            if self.pending_input.is_some() {
                return Err(AbiErrorCode::Busy);
            }
            if self.input_bytes_received != self.declared_input_bytes {
                return Err(AbiErrorCode::MalformedLength);
            }
            self.sealed = true;
            Ok(())
        }

        fn cancel(&mut self) -> OsHostCodecCancelOutcome {
            self.cancelled = true;
            if let Some(output) = self.output.as_mut() {
                output.cancel();
            }
            let pending = self.pending_input.take();
            OsHostCodecCancelOutcome { admitted_byte_credits: pending.as_ref().map_or(0, |pending| pending.page.bytes.len()), copied_bytes: pending.as_ref().map_or(0, |pending| pending.cursor), page: pending.map(|pending| pending.page) }
        }

        fn failure_reply(&self, failure: &OsHostCodecFailure) -> AbiReply {
            let message = AbiMessageBytes::try_new(failure.message.clone()).unwrap_or_else(|_| AbiMessageBytes::from_text("bounded OS host codec failure").expect("fixed fallback is bounded"));
            AbiReply {
                request_id: self.request.request_id,
                generation: self.request.generation,
                status: AbiStatus { code: AbiStatusCode::Rejected, error: Some(AbiError { code: failure.abi_code(), message }) },
                bytes: AbiBytes::try_new((failure.code as u16).to_le_bytes().to_vec()).expect("error code body is bounded"),
            }
        }

        fn success_reply(&self) -> AbiReply {
            let mut summary = Vec::with_capacity(6);
            summary.push(1);
            summary.push(self.operation.reply_kind());
            summary.extend_from_slice(&(self.output_bytes as u32).to_le_bytes());
            AbiReply { request_id: self.request.request_id, generation: self.request.generation, status: AbiStatus::OK, bytes: AbiBytes::try_new(summary).expect("reply summary is bounded") }
        }

        fn install_output(&mut self, bytes: Vec<u8>) -> Result<(), OsHostCodecFailure> {
            let payload_len = bytes.len().checked_add(6).ok_or_else(|| OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"))?;
            if payload_len > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
                return Err(OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"));
            }
            let mut payload = Vec::with_capacity(payload_len);
            payload.push(1);
            payload.push(self.operation.reply_kind());
            payload.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
            payload.extend_from_slice(&bytes);
            self.install_payload(payload)
        }

        fn install_payload(&mut self, payload: Vec<u8>) -> Result<(), OsHostCodecFailure> {
            if payload.len() > OS_HOST_CODEC_MAX_OUTPUT_BYTES {
                return Err(OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds output limit"));
            }
            self.output_bytes = payload.len();
            self.output = Some(AbiPageReader::try_new(self.handle(), payload).map_err(|_| OsHostCodecFailure::fixed(OsHostCodecErrorCode::OutputLimit, "OS host codec reply exceeds transfer limit"))?);
            self.decoded = true;
            Ok(())
        }

        fn execute(&mut self) -> Result<(), OsHostCodecFailure> {
            match self.operation {
                OsHostCodecOperation::DecodeWorkflowFixturePack | OsHostCodecOperation::ParseWorkflowFixtureDsl => {
                    let payload = self.input.workflow().expect("workflow operation owns a structural cursor").finish()?;
                    self.install_payload(payload)
                }
                OsHostCodecOperation::NormalizeStdioFormatKind => {
                    let output = self.input.normalize().expect("normalize operation owns a structural cursor").finish()?;
                    self.install_output(output)
                }
                OsHostCodecOperation::MediaAcceptFilterKinds => {
                    let output = self.input.filter().expect("filter operation owns a structural cursor").finish()?;
                    self.install_output(output)
                }
            }
        }

        fn step<R: OsHostFormatResolver>(&mut self, resolver: &mut R, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
            if self.cancelled {
                return Err(AbiErrorCode::Cancelled);
            }
            if self.closing {
                return Err(AbiErrorCode::Closed);
            }
            validate_budget(budget)?;
            if self.pending_input.is_some() {
                let (byte, index, complete) = {
                    let pending = self.pending_input.as_mut().expect("checked pending input");
                    let byte = if pending.cursor < pending.page.bytes.len() {
                        let byte = pending.page.bytes.as_slice()[pending.cursor];
                        pending.cursor += 1;
                        Some(byte)
                    } else {
                        None
                    };
                    (byte, pending.page.index, pending.cursor == pending.page.bytes.len())
                };
                if let Some(byte) = byte {
                    self.input_bytes_received += 1;
                    self.input.feed(byte, resolver);
                }
                if complete {
                    self.pending_input = None;
                    self.next_input_index += 1;
                    let acknowledgement = AbiControl::Acknowledge { handle: self.handle(), index };
                    return Ok(self.step_result(OsHostCodecStepState::InputAcknowledged, OsHostCodecPhase::Input, self.input_bytes_received, self.declared_input_bytes, Some(acknowledgement), None, None));
                }
                return Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecPhase::Decode, self.input_bytes_received, self.declared_input_bytes, None, None, None));
            }
            if !self.sealed {
                return Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecPhase::Input, self.input_bytes_received, self.declared_input_bytes, None, None, None));
            }
            if let Some(failure) = self.terminal_failure.clone() {
                if self.reply_emitted {
                    return Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecPhase::Reply, 1, 1, None, None, None));
                }
                self.reply_emitted = true;
                let reply = self.failure_reply(&failure);
                return Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecPhase::Reply, 1, 1, None, None, Some(reply)));
            }
            if !self.decoded {
                match self.execute() {
                    Ok(()) => return Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecPhase::Decode, 1, 1, None, None, None)),
                    Err(failure) => {
                        self.terminal_failure = Some(failure.clone());
                        self.reply_emitted = true;
                        let reply = self.failure_reply(&failure);
                        return Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecPhase::Reply, 1, 1, None, None, Some(reply)));
                    }
                }
            }
            let output_step = self.output.as_mut().expect("decoded operation owns a paged output").read_step(AbiWorkBudget { byte_credit: 1, ..budget })?;
            match output_step {
                AbiCursorStep::Advanced(copied) => {
                    self.output_copied = self.output_copied.saturating_add(copied).min(self.output_bytes);
                    Ok(self.step_result(OsHostCodecStepState::Progress, OsHostCodecPhase::Output, self.output_copied, self.output_bytes, None, None, None))
                }
                AbiCursorStep::PageComplete(index) => {
                    let page = self.output.as_ref().and_then(AbiPageReader::page).cloned().expect("page-complete retains the exact page until ACK");
                    self.output_copied = (index as usize * ABI_MAX_PAGE_BYTES + page.bytes.len()).min(self.output_bytes);
                    Ok(self.step_result(OsHostCodecStepState::OutputPage, OsHostCodecPhase::Output, self.output_copied, self.output_bytes, None, Some(page), None))
                }
                AbiCursorStep::AwaitingAcknowledgement(_) => Ok(self.step_result(OsHostCodecStepState::AwaitingAcknowledgement, OsHostCodecPhase::AwaitingAcknowledgement, self.output_copied, self.output_bytes, None, None, None)),
                AbiCursorStep::Complete => {
                    if self.reply_emitted {
                        Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecPhase::Reply, self.output_bytes, self.output_bytes, None, None, None))
                    } else {
                        self.reply_emitted = true;
                        let reply = self.success_reply();
                        Ok(self.step_result(OsHostCodecStepState::Reply, OsHostCodecPhase::Reply, self.output_bytes, self.output_bytes, None, None, Some(reply)))
                    }
                }
                AbiCursorStep::Idle => Ok(self.step_result(OsHostCodecStepState::Idle, OsHostCodecPhase::Output, 0, self.output_bytes, None, None, None)),
            }
        }

        fn acknowledge(&mut self, control: AbiControl) -> Result<(), AbiErrorCode> {
            self.output.as_mut().ok_or(AbiErrorCode::UnknownHandle)?.acknowledge(control)
        }

        fn close_step(&mut self, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
            self.closing = true;
            let input_empty = self.input.terminal_is_empty();
            let failure_empty = self.terminal_failure.as_ref().is_none_or(|failure| failure.message.is_empty());
            if self.pending_input.is_none() && input_empty && failure_empty && self.output.as_ref().is_none_or(AbiPageReader::terminal_is_empty) {
                return Ok(true);
            }
            validate_budget(budget)?;
            if let Some(pending) = self.pending_input.as_mut() {
                if pending.page.bytes.is_empty() {
                    self.pending_input = None;
                    return Ok(false);
                }
                pending.retire_cursor += 1;
                if pending.retire_cursor == pending.page.bytes.len() {
                    self.pending_input = None;
                }
                return Ok(false);
            }
            if self.input.retire_one() {
                return Ok(false);
            }
            if let Some(failure) = self.terminal_failure.as_mut() {
                if failure.message.pop().is_some() {
                    return Ok(false);
                }
            }
            if let Some(output) = self.output.as_mut() {
                return output.close_step(AbiWorkBudget { byte_credit: 1, ..budget }).map(|step| step == AbiCursorStep::Complete);
            }
            Ok(true)
        }
    }

    fn validate_budget(budget: AbiWorkBudget) -> Result<(), AbiErrorCode> {
        if budget.cancelled {
            Err(AbiErrorCode::Cancelled)
        } else if budget.interrupted {
            Err(AbiErrorCode::Interrupted)
        } else if budget.deadline_ms.is_some_and(|deadline| budget.now_ms >= deadline) {
            Err(AbiErrorCode::DeadlineExceeded)
        } else if budget.byte_credit == 0 {
            Err(AbiErrorCode::NoCredit)
        } else {
            Ok(())
        }
    }

    fn classify_handle(expected: AbiHandle, actual: AbiHandle) -> AbiErrorCode {
        if expected.slot() != actual.slot() {
            AbiErrorCode::UnknownHandle
        } else if actual.generation() < expected.generation() {
            AbiErrorCode::AbaHandle
        } else if actual.generation() > expected.generation() {
            AbiErrorCode::StaleGeneration
        } else {
            AbiErrorCode::UnknownHandle
        }
    }

    fn request_slot(request_id: AbiRequestId) -> usize {
        (request_id.0 % 256) as usize
    }

    struct RetainedOsHostCodecService<R> {
        resolver: R,
        handles: AbiHandleTable<OsHostCodecSession>,
        requests: [Option<AbiHandle>; 256],
    }

    impl<R: OsHostFormatResolver> RetainedOsHostCodecService<R> {
        pub fn new(resolver: R) -> Self {
            Self { resolver, handles: AbiHandleTable::new(), requests: [None; 256] }
        }

        pub fn begin(&mut self, request: AbiRequest) -> Result<AbiHandle, (AbiErrorCode, AbiRequest)> {
            let operation = match OsHostCodecOperation::from_abi(request.operation) {
                Ok(operation) => operation,
                Err(code) => return Err((code, request)),
            };
            if request.generation == 0 {
                return Err((AbiErrorCode::StaleGeneration, request));
            }
            let declared_input_bytes = match decode_request_metadata(request.bytes.as_slice()) {
                Ok(value) => value,
                Err(code) => return Err((code, request)),
            };
            if operation == OsHostCodecOperation::NormalizeStdioFormatKind && declared_input_bytes > OS_HOST_CODEC_MAX_KIND_BYTES {
                return Err((AbiErrorCode::LimitExceeded, request));
            }
            let slot = request_slot(request.request_id);
            if self.requests[slot].is_some() {
                return Err((AbiErrorCode::Busy, request));
            }
            let session = OsHostCodecSession::new(request, operation, declared_input_bytes);
            let handle = match self.handles.open(session) {
                Ok(handle) => handle,
                Err((code, session)) => return Err((code, session.request)),
            };
            self.handles.get_mut(handle).expect("new handle resolves").handle = Some(handle);
            self.requests[slot] = Some(handle);
            Ok(handle)
        }

        pub fn offer(&mut self, handle: AbiHandle, page: AbiPage) -> Result<(), AbiRejectedPage> {
            match self.handles.get_mut(handle) {
                Ok(session) => session.offer(page),
                Err(code) => Err(AbiRejectedPage { code, handle: page.handle, index: page.index, bytes: page.bytes.into_vec() }),
            }
        }

        pub fn seal(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
            self.handles.get_mut(handle)?.seal()
        }

        pub fn step(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
            self.handles.get_mut(handle)?.step(&mut self.resolver, budget)
        }

        pub fn control(&mut self, control: AbiControl) -> Result<Option<OsHostCodecCancelOutcome>, AbiErrorCode> {
            match control {
                AbiControl::Cancel { request_id, generation } => {
                    let handle = self.requests[request_slot(request_id)].ok_or(AbiErrorCode::UnknownHandle)?;
                    let session = self.handles.get_mut(handle)?;
                    if session.request.request_id != request_id {
                        return Err(AbiErrorCode::UnknownHandle);
                    }
                    if generation < session.request.generation {
                        return Err(AbiErrorCode::AbaHandle);
                    }
                    if generation > session.request.generation {
                        return Err(AbiErrorCode::StaleGeneration);
                    }
                    Ok(Some(session.cancel()))
                }
                AbiControl::Acknowledge { handle, .. } => {
                    self.handles.get_mut(handle)?.acknowledge(control)?;
                    Ok(None)
                }
                AbiControl::Close { .. } => Err(AbiErrorCode::MalformedTag),
            }
        }

        pub fn close_step(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
            let AbiControl::Close { handle } = control else { return Err(AbiErrorCode::MalformedTag) };
            let complete = self.handles.get_mut(handle)?.close_step(budget)?;
            if complete {
                let session = self.handles.close(handle)?;
                let slot = request_slot(session.request.request_id);
                if self.requests[slot] == Some(handle) {
                    self.requests[slot] = None;
                }
            }
            Ok(complete)
        }

        pub fn lose(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
            let session = self.handles.lose(handle)?;
            let slot = request_slot(session.request.request_id);
            if self.requests[slot] == Some(handle) {
                self.requests[slot] = None;
            }
            Ok(())
        }
    }

    #[cfg(feature = "os-host-full")]
    /// 🌉️ Owned retained OS-host codec service; UI callers can submit only A1 requests and pages.
    pub struct OsHostCodecService {
        retained: RetainedOsHostCodecService<RegisteredOsHostFormatResolver>,
    }

    #[cfg(feature = "os-host-full")]
    impl Default for OsHostCodecService {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(feature = "os-host-full")]
    impl OsHostCodecService {
        pub fn new() -> Self {
            Self { retained: RetainedOsHostCodecService::new(RegisteredOsHostFormatResolver) }
        }

        pub fn begin(&mut self, request: AbiRequest) -> Result<AbiHandle, (AbiErrorCode, AbiRequest)> {
            self.retained.begin(request)
        }

        pub fn offer(&mut self, handle: AbiHandle, page: AbiPage) -> Result<(), AbiRejectedPage> {
            self.retained.offer(handle, page)
        }

        pub fn seal(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
            self.retained.seal(handle)
        }

        pub fn step(&mut self, handle: AbiHandle, budget: AbiWorkBudget) -> Result<OsHostCodecStep, AbiErrorCode> {
            self.retained.step(handle, budget)
        }

        pub fn control(&mut self, control: AbiControl) -> Result<Option<OsHostCodecCancelOutcome>, AbiErrorCode> {
            self.retained.control(control)
        }

        pub fn close_step(&mut self, control: AbiControl, budget: AbiWorkBudget) -> Result<bool, AbiErrorCode> {
            self.retained.close_step(control, budget)
        }

        pub fn lose(&mut self, handle: AbiHandle) -> Result<(), AbiErrorCode> {
            self.retained.lose(handle)
        }
    }

    fn decode_request_metadata(bytes: &[u8]) -> Result<usize, AbiErrorCode> {
        if bytes.len() != 5 || bytes[0] != 1 {
            return Err(AbiErrorCode::MalformedTag);
        }
        let declared = u32::from_le_bytes(bytes[1..5].try_into().expect("fixed metadata width")) as usize;
        if declared > OS_HOST_CODEC_MAX_INPUT_BYTES {
            Err(AbiErrorCode::LimitExceeded)
        } else {
            Ok(declared)
        }
    }
    //#endregion ⏳️RetainedOperation

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use semio_framework::{decode_abi_message, AbiMessage, AbiPageBytes};

        #[derive(Default)]
        struct FixtureFormatResolver;

        impl OsHostFormatResolver for FixtureFormatResolver {
            fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                Ok(match kind {
                    "stdio.dwg" | "dwg" => Some(OsHostCodecFormat { short_id: "dwg".into(), extensions: vec![".dwg".into()] }),
                    "stdio.step" | "step" => Some(OsHostCodecFormat { short_id: "step".into(), extensions: vec![".step".into(), ".stp".into()] }),
                    _ => None,
                })
            }
        }

        fn request(operation: OsHostCodecOperation, request_id: u64, generation: u32, input_len: usize) -> AbiRequest {
            let mut metadata = vec![1];
            metadata.extend_from_slice(&(input_len as u32).to_le_bytes());
            AbiRequest { operation: operation.abi(), request_id: AbiRequestId(request_id), generation, bytes: AbiBytes::try_new(metadata).unwrap() }
        }

        fn page(handle: AbiHandle, index: u32, bytes: &[u8]) -> AbiPage {
            AbiPage::try_new(handle, index, bytes.to_vec()).unwrap()
        }

        fn admit_input<R: OsHostFormatResolver>(service: &mut RetainedOsHostCodecService<R>, handle: AbiHandle, bytes: &[u8]) {
            service.offer(handle, page(handle, 0, bytes)).unwrap();
            loop {
                let step = service.step(handle, AbiWorkBudget::credits(99)).unwrap();
                if step.state == OsHostCodecStepState::InputAcknowledged {
                    assert_eq!(step.input_acknowledgement, Some(AbiControl::Acknowledge { handle, index: 0 }));
                    break;
                }
            }
            service.seal(handle).unwrap();
        }

        fn finish<R: OsHostFormatResolver>(service: &mut RetainedOsHostCodecService<R>, handle: AbiHandle) -> (Vec<u8>, AbiReply) {
            let mut output = Vec::new();
            let mut output_progress = 0_u64;
            loop {
                let step = service.step(handle, AbiWorkBudget::credits(99)).unwrap();
                if step.event.bytes.as_slice()[1] == OsHostCodecPhase::Output as u8 {
                    let completed = u64::from_le_bytes(step.event.bytes.as_slice()[2..10].try_into().unwrap());
                    let total = u64::from_le_bytes(step.event.bytes.as_slice()[10..18].try_into().unwrap());
                    assert_eq!(completed, output_progress + 1);
                    assert!(completed <= total);
                    output_progress = completed;
                }
                if let Some(page) = step.page {
                    output.extend_from_slice(page.bytes.as_slice());
                    service.control(AbiControl::Acknowledge { handle, index: page.index }).unwrap();
                }
                if let Some(reply) = step.reply {
                    return (output, reply);
                }
            }
        }

        fn payload(bytes: &[u8]) -> &[u8] {
            let len = u32::from_le_bytes(bytes[2..6].try_into().unwrap()) as usize;
            assert_eq!(bytes.len(), len + 6);
            &bytes[6..]
        }

        fn run<R: OsHostFormatResolver>(resolver: R, operation: OsHostCodecOperation, request_id: u64, bytes: &[u8]) -> (Vec<u8>, AbiReply) {
            let mut service = RetainedOsHostCodecService::new(resolver);
            let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
            if bytes.is_empty() {
                service.seal(handle).unwrap();
            } else {
                admit_input(&mut service, handle, bytes);
            }
            finish(&mut service, handle)
        }

        fn hex_bytes(hex: &str) -> Vec<u8> {
            hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
        }

        const CANONICAL_DSL: &[u8] = b"name=fixture\ngraph {\n}\ndirty-node-ids=[ ]\nexpected-deliveries [edge-id:TEXT] {\n}\n";

        fn structural_pack(dsl: &[u8]) -> Vec<u8> {
            let mut bytes = WORKFLOW_PACK_MAGIC.to_vec();
            bytes.push(1);
            bytes.extend_from_slice(&(dsl.len() as u32).to_le_bytes());
            bytes.extend_from_slice(dsl);
            bytes
        }

        fn admit_page<R: OsHostFormatResolver>(service: &mut RetainedOsHostCodecService<R>, handle: AbiHandle, index: u32, bytes: &[u8]) {
            service.offer(handle, page(handle, index, bytes)).unwrap();
            loop {
                let step = service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap();
                if step.state == OsHostCodecStepState::InputAcknowledged {
                    assert_eq!(step.input_acknowledgement, Some(AbiControl::Acknowledge { handle, index }));
                    return;
                }
            }
        }

        #[test]
        fn schema_and_language_neutral_fixture_cover_every_operation() {
            for name in ["decodeWorkflowFixturePack", "parseWorkflowFixtureDsl", "mediaAcceptFilterKinds", "normalizeStdioFormatKind"] {
                assert!(OS_HOST_CODEC_SCHEMA_JSON.contains(name));
            }
            for name in ["workflowPackOrder", "canonicalDslBytes:u32le", "workflowDslCanonical", "filterRetainedState", "normalizeRetainedState", "one-byte-or-completed-item-opportunity-per-grant", "transferPages"] {
                assert!(OS_HOST_CODEC_SCHEMA_JSON.contains(name));
            }
            for name in ["decode_pack_request", "parse_dsl_request", "filter_request", "normalize_request"] {
                assert!(OS_HOST_CODEC_LEDGER_FIXTURE.contains(name));
            }
            for line in OS_HOST_CODEC_LEDGER_FIXTURE.lines().skip(1).take(5) {
                let (_, hex) = line.split_once('\t').unwrap();
                assert!(matches!(decode_abi_message(&hex_bytes(hex)), Ok(AbiMessage::Request(_) | AbiMessage::Reply(_))));
            }
            let rows: Vec<_> = OS_HOST_CODEC_LEDGER_FIXTURE.lines().filter_map(|line| line.split_once('\t')).collect();
            let dsl = hex_bytes(rows.iter().find(|(name, _)| *name == "workflow_dsl_canonical").unwrap().1);
            let pack = hex_bytes(rows.iter().find(|(name, _)| *name == "workflow_pack_structural").unwrap().1);
            assert_eq!(&pack[..4], &WORKFLOW_PACK_MAGIC);
            assert_eq!(&pack[9..], dsl);
        }

        #[test]
        fn valid_pack_and_dsl_are_equivalent_deterministic_paged_replies() {
            let pack = structural_pack(CANONICAL_DSL);
            let mut replies = Vec::new();
            for (operation, bytes) in [(OsHostCodecOperation::DecodeWorkflowFixturePack, pack.as_slice()), (OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL)] {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, 7, 1, bytes.len())).unwrap();
                admit_input(&mut service, handle, bytes);
                let (output, reply) = finish(&mut service, handle);
                assert_eq!(payload(&output), CANONICAL_DSL);
                assert_eq!(reply.status, AbiStatus::OK);
                replies.push((output, reply.bytes.into_vec()));
            }
            assert_eq!(replies[0], replies[1]);
        }

        #[test]
        fn workflow_pack_and_dsl_accept_every_byte_and_field_split() {
            let pack = structural_pack(CANONICAL_DSL);
            for (operation, bytes) in [(OsHostCodecOperation::DecodeWorkflowFixturePack, pack.as_slice()), (OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL)] {
                for split in 0..=bytes.len() {
                    let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                    let handle = service.begin(request(operation, 70, 1, bytes.len())).unwrap();
                    admit_page(&mut service, handle, 0, &bytes[..split]);
                    admit_page(&mut service, handle, 1, &bytes[split..]);
                    service.seal(handle).unwrap();
                    let (output, reply) = finish(&mut service, handle);
                    assert_eq!(payload(&output), CANONICAL_DSL, "split={split}");
                    assert_eq!(reply.status, AbiStatus::OK, "split={split}");
                }
            }
        }

        #[test]
        fn filter_and_normalize_preserve_registered_format_behavior() {
            let filter = [1, 2, 0, 3, 0, b'd', b'w', b'g', 4, 0, b's', b't', b'e', b'p'];
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(OsHostCodecOperation::MediaAcceptFilterKinds, 8, 1, filter.len())).unwrap();
            admit_input(&mut service, handle, &filter);
            let (output, _) = finish(&mut service, handle);
            assert_eq!(payload(&output), b".dwg,.step,.stp");

            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 9, 1, 9)).unwrap();
            admit_input(&mut service, handle, b"stdio.dwg");
            let (output, _) = finish(&mut service, handle);
            assert_eq!(payload(&output), b"dwg");
        }

        #[test]
        fn filter_and_normalize_accept_every_byte_and_field_split() {
            let filter = [1, 2, 0, 3, 0, b'd', b'w', b'g', 4, 0, b's', b't', b'e', b'p'];
            for (request_id, operation, bytes, expected) in
                [(81, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice(), b".dwg,.step,.stp".as_slice()), (82, OsHostCodecOperation::NormalizeStdioFormatKind, b"stdio.dwg".as_slice(), b"dwg".as_slice())]
            {
                for split in 0..=bytes.len() {
                    let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                    let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
                    admit_page(&mut service, handle, 0, &bytes[..split]);
                    admit_page(&mut service, handle, 1, &bytes[split..]);
                    service.seal(handle).unwrap();
                    let (output, reply) = finish(&mut service, handle);
                    assert_eq!(payload(&output), expected, "split={split}");
                    assert_eq!(reply.status, AbiStatus::OK, "split={split}");
                }
            }

            struct UnicodeKindResolver;
            impl OsHostFormatResolver for UnicodeKindResolver {
                fn resolve_format(&mut self, kind: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                    Ok((kind == "dωg").then(|| OsHostCodecFormat { short_id: "unicode".into(), extensions: vec![".unicode".into()] }))
                }
            }
            let kind = "dωg".as_bytes();
            let mut filter = vec![1, 1, 0];
            filter.extend_from_slice(&(kind.len() as u16).to_le_bytes());
            filter.extend_from_slice(kind);
            for (request_id, operation, bytes, expected) in [(103, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice(), b".unicode".as_slice()), (104, OsHostCodecOperation::NormalizeStdioFormatKind, kind, b"unicode".as_slice())] {
                for split in 0..=bytes.len() {
                    let mut service = RetainedOsHostCodecService::new(UnicodeKindResolver);
                    let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
                    admit_page(&mut service, handle, 0, &bytes[..split]);
                    admit_page(&mut service, handle, 1, &bytes[split..]);
                    service.seal(handle).unwrap();
                    let (output, reply) = finish(&mut service, handle);
                    assert_eq!(payload(&output), expected, "unicode split={split}");
                    assert_eq!(reply.status, AbiStatus::OK, "unicode split={split}");
                }
            }
        }

        #[test]
        fn filter_and_normalize_zero_max_and_plus_one_bounds_precede_item_copy() {
            struct AnyKindResolver;
            impl OsHostFormatResolver for AnyKindResolver {
                fn resolve_format(&mut self, _: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                    Ok(Some(OsHostCodecFormat { short_id: "x".into(), extensions: Vec::new() }))
                }
            }

            let (output, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 83, &[1, 0, 0]);
            assert!(payload(&output).is_empty());
            assert_eq!(reply.status, AbiStatus::OK);

            let mut maximum_count = vec![1, 0, 1];
            for _ in 0..OS_HOST_CODEC_MAX_KIND_COUNT {
                maximum_count.extend_from_slice(&0_u16.to_le_bytes());
            }
            let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 84, &maximum_count);
            assert_eq!(reply.status, AbiStatus::OK);

            let plus_one_count = [1, 1, 1];
            let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 85, &plus_one_count);
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::InputLimit as u16);

            let mut maximum_kind = vec![1, 1, 0];
            maximum_kind.extend_from_slice(&(OS_HOST_CODEC_MAX_KIND_BYTES as u16).to_le_bytes());
            maximum_kind.extend(std::iter::repeat_n(b'x', OS_HOST_CODEC_MAX_KIND_BYTES));
            let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 86, &maximum_kind);
            assert_eq!(reply.status, AbiStatus::OK);
            let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::NormalizeStdioFormatKind, 87, &maximum_kind[5..]);
            assert_eq!(reply.status, AbiStatus::OK);

            let plus_one_kind = (OS_HOST_CODEC_MAX_KIND_BYTES + 1) as u16;
            let filter_plus_one = [1, 1, 0, plus_one_kind as u8, (plus_one_kind >> 8) as u8];
            let (_, reply) = run(AnyKindResolver, OsHostCodecOperation::MediaAcceptFilterKinds, 88, &filter_plus_one);
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::InputLimit as u16);

            let mut service = RetainedOsHostCodecService::new(AnyKindResolver);
            let rejected = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 89, 1, OS_HOST_CODEC_MAX_KIND_BYTES + 1)).unwrap_err();
            assert_eq!(rejected.0, AbiErrorCode::LimitExceeded);
            assert_eq!(rejected.1.request_id, AbiRequestId(89));
        }

        #[test]
        fn filter_and_normalize_reject_malformed_truncated_and_invalid_utf8_incrementally() {
            let cases: &[(OsHostCodecOperation, &[u8], OsHostCodecErrorCode)] = &[
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[2, 0, 0], OsHostCodecErrorCode::MalformedRequest),
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[1], OsHostCodecErrorCode::MalformedRequest),
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 3], OsHostCodecErrorCode::MalformedRequest),
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 3, 0, b'd'], OsHostCodecErrorCode::MalformedRequest),
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 1, 0, 0xc3], OsHostCodecErrorCode::InvalidUtf8),
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 0, 0, 0], OsHostCodecErrorCode::MalformedRequest),
                (OsHostCodecOperation::MediaAcceptFilterKinds, &[1, 1, 0, 3, 0, b'w', b'a', b't'], OsHostCodecErrorCode::UnknownKind),
                (OsHostCodecOperation::NormalizeStdioFormatKind, &[0xc3], OsHostCodecErrorCode::InvalidUtf8),
                (OsHostCodecOperation::NormalizeStdioFormatKind, &[0xff], OsHostCodecErrorCode::InvalidUtf8),
                (OsHostCodecOperation::NormalizeStdioFormatKind, &[], OsHostCodecErrorCode::MalformedRequest),
            ];
            for (index, (operation, bytes, code)) in cases.iter().enumerate() {
                let (_, reply) = run(FixtureFormatResolver, *operation, 90 + index as u64, bytes);
                assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), *code as u16, "case={index}");
                assert_eq!(reply.status.code, AbiStatusCode::Rejected, "case={index}");
            }
        }

        #[test]
        fn malformed_pack_dsl_missing_array_and_unknown_kind_are_owned_failures() {
            for (operation, bytes, code) in [
                (OsHostCodecOperation::DecodeWorkflowFixturePack, b"bad".as_slice(), OsHostCodecErrorCode::MalformedPack),
                (OsHostCodecOperation::ParseWorkflowFixtureDsl, b"bad".as_slice(), OsHostCodecErrorCode::MalformedDsl),
                (OsHostCodecOperation::MediaAcceptFilterKinds, b"".as_slice(), OsHostCodecErrorCode::MissingKindArray),
                (OsHostCodecOperation::NormalizeStdioFormatKind, b"wat".as_slice(), OsHostCodecErrorCode::UnknownKind),
            ] {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, 11, 1, bytes.len())).unwrap();
                if bytes.is_empty() {
                    service.seal(handle).unwrap();
                } else {
                    admit_input(&mut service, handle, bytes);
                }
                let (_, reply) = finish(&mut service, handle);
                assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), code as u16);
                assert_eq!(reply.status.code, AbiStatusCode::Rejected);
            }
        }

        #[test]
        fn truncated_pack_dsl_and_invalid_utf8_fail_after_retained_decode() {
            let mut truncated_pack = structural_pack(CANONICAL_DSL);
            truncated_pack.pop();
            let mut truncated_dsl = CANONICAL_DSL.to_vec();
            truncated_dsl.pop();
            let mut invalid_utf8 = CANONICAL_DSL.to_vec();
            invalid_utf8.insert(invalid_utf8.len() - 1, 0xff);
            for (request_id, operation, bytes, code) in [
                (12, OsHostCodecOperation::DecodeWorkflowFixturePack, truncated_pack, OsHostCodecErrorCode::MalformedPack),
                (13, OsHostCodecOperation::ParseWorkflowFixtureDsl, truncated_dsl, OsHostCodecErrorCode::MalformedDsl),
                (14, OsHostCodecOperation::ParseWorkflowFixtureDsl, invalid_utf8, OsHostCodecErrorCode::InvalidUtf8),
            ] {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
                admit_input(&mut service, handle, &bytes);
                let (_, reply) = finish(&mut service, handle);
                assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), code as u16);
                assert_eq!(reply.status.code, AbiStatusCode::Rejected);
            }
        }

        #[test]
        fn exact_input_and_output_limits_reject_plus_one_without_consuming_request() {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let maximum = request(OsHostCodecOperation::DecodeWorkflowFixturePack, 20, 1, OS_HOST_CODEC_MAX_INPUT_BYTES);
            assert!(service.begin(maximum).is_ok());
            let mut plus_one = vec![1];
            plus_one.extend_from_slice(&((OS_HOST_CODEC_MAX_INPUT_BYTES + 1) as u32).to_le_bytes());
            let rejected = AbiRequest { operation: OsHostCodecOperation::DecodeWorkflowFixturePack.abi(), request_id: AbiRequestId(21), generation: 1, bytes: AbiBytes::try_new(plus_one).unwrap() };
            let rejected = service.begin(rejected).unwrap_err();
            assert_eq!(rejected.0, AbiErrorCode::LimitExceeded);
            assert_eq!(rejected.1.request_id, AbiRequestId(21));

            struct LargeFormatResolver(usize);
            impl OsHostFormatResolver for LargeFormatResolver {
                fn resolve_format(&mut self, _: &str) -> Result<Option<OsHostCodecFormat>, OsHostCodecFailure> {
                    Ok(Some(OsHostCodecFormat { short_id: "x".repeat(self.0), extensions: Vec::new() }))
                }
            }
            let mut service = RetainedOsHostCodecService::new(LargeFormatResolver(OS_HOST_CODEC_MAX_OUTPUT_BYTES - 6));
            let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 22, 1, 1)).unwrap();
            service.offer(handle, page(handle, 0, b"x")).unwrap();
            service.step(handle, AbiWorkBudget::credits(1)).unwrap();
            service.seal(handle).unwrap();
            assert_eq!(service.step(handle, AbiWorkBudget::credits(1)).unwrap().state, OsHostCodecStepState::Progress);
            assert_eq!(service.handles.get_mut(handle).unwrap().output_bytes, OS_HOST_CODEC_MAX_OUTPUT_BYTES);
            let mut service = RetainedOsHostCodecService::new(LargeFormatResolver(OS_HOST_CODEC_MAX_OUTPUT_BYTES - 5));
            let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 23, 1, 1)).unwrap();
            service.offer(handle, page(handle, 0, b"x")).unwrap();
            service.step(handle, AbiWorkBudget::credits(1)).unwrap();
            service.seal(handle).unwrap();
            let reply = service.step(handle, AbiWorkBudget::credits(1)).unwrap().reply.unwrap();
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::OutputLimit as u16);
        }

        #[test]
        fn page_count_limit_precedes_sequence_classification_and_returns_the_page() {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(OsHostCodecOperation::DecodeWorkflowFixturePack, 24, 1, 0)).unwrap();
            let page = AbiPage { handle, index: ABI_MAX_PAGES_PER_TRANSFER, bytes: AbiPageBytes::default() };
            let rejected = service.offer(handle, page).unwrap_err();
            assert_eq!(rejected.code, AbiErrorCode::LimitExceeded);
            assert_eq!(rejected.index, ABI_MAX_PAGES_PER_TRANSFER);
            assert!(rejected.bytes.is_empty());
        }

        #[test]
        fn cancel_mid_every_structural_cursor_returns_exact_page_and_blocks_progress() {
            let pack = structural_pack(CANONICAL_DSL);
            let filter = [1, 2, 0, 3, 0, b'd', b'w', b'g', 4, 0, b's', b't', b'e', b'p'];
            for (request_id, operation, bytes) in [
                (30, OsHostCodecOperation::DecodeWorkflowFixturePack, pack.as_slice()),
                (31, OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL),
                (33, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice()),
                (34, OsHostCodecOperation::NormalizeStdioFormatKind, b"stdio.dwg".as_slice()),
            ] {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, request_id, 4, bytes.len())).unwrap();
                service.offer(handle, page(handle, 0, bytes)).unwrap();
                let copied = bytes.len() / 2;
                for _ in 0..copied {
                    service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap();
                }
                let outcome = service.control(AbiControl::Cancel { request_id: AbiRequestId(request_id), generation: 4 }).unwrap().unwrap();
                assert_eq!(outcome.page.unwrap().bytes.as_slice(), bytes);
                assert_eq!((outcome.admitted_byte_credits, outcome.copied_bytes), (bytes.len(), copied));
                assert_eq!(service.step(handle, AbiWorkBudget::credits(1)), Err(AbiErrorCode::Cancelled));
            }
        }

        #[test]
        fn deadline_interruption_and_zero_credit_do_not_advance_any_structural_cursor() {
            let filter = [1, 1, 0, 3, 0, b'd', b'w', b'g'];
            for (request_id, operation, bytes) in
                [(32, OsHostCodecOperation::ParseWorkflowFixtureDsl, CANONICAL_DSL), (35, OsHostCodecOperation::MediaAcceptFilterKinds, filter.as_slice()), (36, OsHostCodecOperation::NormalizeStdioFormatKind, b"stdio.dwg".as_slice())]
            {
                let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
                let handle = service.begin(request(operation, request_id, 1, bytes.len())).unwrap();
                service.offer(handle, page(handle, 0, bytes)).unwrap();
                assert_eq!(service.step(handle, AbiWorkBudget::credits(0)), Err(AbiErrorCode::NoCredit));
                assert_eq!(service.step(handle, AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::Interrupted));
                assert_eq!(service.step(handle, AbiWorkBudget { now_ms: 5, deadline_ms: Some(5), ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::DeadlineExceeded));
                let step = service.step(handle, AbiWorkBudget::credits(1)).unwrap();
                assert_eq!(step.event.bytes.as_slice()[2..10], 1_u64.to_le_bytes());
                assert_eq!(service.handles.get_mut(handle).unwrap().input_bytes_received, 1);
            }
        }

        #[test]
        fn handle_loss_stale_generation_duplicate_ack_and_interrupted_close_are_exact() {
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let first = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 40, 1, 3)).unwrap();
            service.lose(first).unwrap();
            assert_eq!(service.step(first, AbiWorkBudget::credits(1)), Err(AbiErrorCode::UnknownHandle));
            let second = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 40, 2, 3)).unwrap();
            assert_eq!(service.step(first, AbiWorkBudget::credits(1)), Err(AbiErrorCode::AbaHandle));
            admit_input(&mut service, second, b"dwg");
            let mut output_page = None;
            while output_page.is_none() {
                output_page = service.step(second, AbiWorkBudget::credits(1)).unwrap().page;
            }
            let output_page = output_page.unwrap();
            let ack = AbiControl::Acknowledge { handle: second, index: output_page.index };
            service.control(ack).unwrap();
            assert_eq!(service.control(ack), Err(AbiErrorCode::DuplicateAcknowledgement));
            assert_eq!(service.close_step(AbiControl::Close { handle: second }, AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::Interrupted));
            while !service.close_step(AbiControl::Close { handle: second }, AbiWorkBudget::credits(1)).unwrap() {}
        }

        #[test]
        fn filter_output_acknowledgement_and_interrupted_close_are_exact() {
            let filter = [1, 1, 0, 4, 0, b's', b't', b'e', b'p'];
            let mut service = RetainedOsHostCodecService::new(FixtureFormatResolver);
            let handle = service.begin(request(OsHostCodecOperation::MediaAcceptFilterKinds, 41, 1, filter.len())).unwrap();
            admit_input(&mut service, handle, &filter);
            let output_page = loop {
                if let Some(page) = service.step(handle, AbiWorkBudget::credits(1)).unwrap().page {
                    break page;
                }
            };
            assert_eq!(payload(output_page.bytes.as_slice()), b".step,.stp");
            let acknowledgement = AbiControl::Acknowledge { handle, index: output_page.index };
            service.control(acknowledgement).unwrap();
            assert_eq!(service.control(acknowledgement), Err(AbiErrorCode::DuplicateAcknowledgement));
            assert_eq!(service.close_step(AbiControl::Close { handle }, AbiWorkBudget { interrupted: true, ..AbiWorkBudget::credits(1) }), Err(AbiErrorCode::Interrupted));
            while !service.close_step(AbiControl::Close { handle }, AbiWorkBudget::credits(1)).unwrap() {}
        }

        #[test]
        fn public_interactive_route_has_no_batch_or_whole_input_capability() {
            let source = include_str!("🦀️component.rs");
            let start = source.find("pub mod codec_abi {").unwrap();
            let tests = source[start..].find("//#region 🧪️Tests").unwrap() + start;
            let production = &source[start..tests];
            for forbidden in ["UiForbidden", "ArtifactPack", "ArtifactDsl", "decode_pack(", "parse_dsl(", "decode_workflow_fixture_pack", "parse_workflow_fixture_dsl"] {
                assert!(!production.contains(forbidden), "public route contains forbidden batch edge {forbidden}");
            }
            for forbidden in [concat!("Bytes(Vec<", "u8>)"), concat!("Self::", "Bytes("), concat!("input.", "bytes()"), concat!("execute_", "filter(")] {
                assert!(!production.contains(forbidden), "public route contains forbidden whole-input edge {forbidden}");
            }
            assert!(production.contains("RegisteredOsHostFormatResolver"));
            assert!(production.contains("WorkflowStructuralCursor"));
            assert!(production.contains("FilterKindsStructuralCursor"));
            assert!(production.contains("NormalizeKindStructuralCursor"));
        }

        #[cfg(feature = "os-host-full")]
        #[test]
        fn public_service_runs_the_retained_workflow_cursor_without_a_format_backend() {
            let mut service = OsHostCodecService::new();
            let handle = service.begin(request(OsHostCodecOperation::ParseWorkflowFixtureDsl, 80, 1, CANONICAL_DSL.len())).unwrap();
            service.offer(handle, page(handle, 0, CANONICAL_DSL)).unwrap();
            loop {
                if service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap().state == OsHostCodecStepState::InputAcknowledged {
                    break;
                }
            }
            service.seal(handle).unwrap();
            let mut output = Vec::new();
            loop {
                let step = service.step(handle, AbiWorkBudget::credits(usize::MAX)).unwrap();
                if let Some(page) = step.page {
                    output.extend_from_slice(page.bytes.as_slice());
                    service.control(AbiControl::Acknowledge { handle, index: page.index }).unwrap();
                }
                if let Some(reply) = step.reply {
                    assert_eq!(reply.status, AbiStatus::OK);
                    break;
                }
            }
            assert_eq!(payload(&output), CANONICAL_DSL);
        }

        #[cfg(feature = "os-host-full")]
        #[test]
        fn public_service_runs_filter_and_normalize_structural_cursors() {
            let mut service = OsHostCodecService::new();
            let filter = [1, 0, 0];
            let handle = service.begin(request(OsHostCodecOperation::MediaAcceptFilterKinds, 101, 1, filter.len())).unwrap();
            service.offer(handle, page(handle, 0, &filter)).unwrap();
            loop {
                if service.step(handle, AbiWorkBudget::credits(1)).unwrap().state == OsHostCodecStepState::InputAcknowledged {
                    break;
                }
            }
            service.seal(handle).unwrap();
            let mut output = Vec::new();
            loop {
                let step = service.step(handle, AbiWorkBudget::credits(1)).unwrap();
                if let Some(page) = step.page {
                    output.extend_from_slice(page.bytes.as_slice());
                    service.control(AbiControl::Acknowledge { handle, index: page.index }).unwrap();
                }
                if let Some(reply) = step.reply {
                    assert_eq!(reply.status, AbiStatus::OK);
                    break;
                }
            }
            assert!(payload(&output).is_empty());

            let mut service = OsHostCodecService::new();
            let handle = service.begin(request(OsHostCodecOperation::NormalizeStdioFormatKind, 102, 1, 3)).unwrap();
            service.offer(handle, page(handle, 0, b"wat")).unwrap();
            loop {
                if service.step(handle, AbiWorkBudget::credits(1)).unwrap().state == OsHostCodecStepState::InputAcknowledged {
                    break;
                }
            }
            service.seal(handle).unwrap();
            let reply = service.step(handle, AbiWorkBudget::credits(1)).unwrap().reply.unwrap();
            assert_eq!(u16::from_le_bytes(reply.bytes.as_slice().try_into().unwrap()), OsHostCodecErrorCode::UnknownKind as u16);
        }
    }
    //#endregion 🧪️Tests
}

#[cfg(feature = "os-host-full")]
pub mod registry {
    // #region registry
    //! 🗂️ Plugin manifest registry and OS plugin/artifact catalog.

    use crate::instance::OsParameterFieldSpec;
    use crate::space;
    use crate::workflow;
    use semio_framework::{AppDefinition, AppRole, ArtifactDialect, ArtifactKindSpec, ConfigSpec, MediaClass, MediaForm, MediaType, ModeDefinition, OsMediaCapability, PluginManifest, WindowKindDefinition};
    use semio_framework::{Locale, Terminology};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    use ui_wgpu::wgpu::{LocalizedLabel, SurfaceKind};

    pub type OsArtifactKindId = String;

    //#region 🔖️ResourceDescriptors
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsArtifactDescriptor {
        pub kind: OsArtifactKindId,
        pub name: String,
        pub source_format: String,
        pub component_kind: String,
        pub dimension: String,
        /// 🧬️ The `MediaType` this resource kind negotiates on the workflow — see
        /// `semio_framework::media_types_compatible`.
        pub media_type: MediaType,
        /// 🔌️ Structured-payload schema id, mirrored from `ArtifactKindSpec::schema` — see
        /// `crate::workflow::negotiate_media_contract`, which prefers a matching schema over a shared
        /// binary format kind id.
        pub schema: String,
        pub export_formats: Vec<String>,
        pub import_formats: Vec<String>,
        /// 🗄️ Stdio export target kind ids (`stdio.json` / short `json`).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub export_stdio_kinds: Vec<String>,
        /// 🗄️ Stdio import source kind ids.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub import_stdio_kinds: Vec<String>,
        /// 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: the real
        /// `ArtifactDialect` this resource kind's `component_kind` was derived into, computed ONCE at
        /// `artifact_kind_entry_from_spec` (never re-formatted ad hoc per call site the way
        /// `native_dialect_kind` used to). Shells (task 3) read this to call the new `io-routes`/
        /// `io-entries` host imports and populate a real "Export as…" list instead of a hard-coded
        /// format table. `standard`/`subset` are placeholder `"1"`/`"*"` until the owning plugin
        /// migrates onto `declare_artifact` and supplies a real dialect (design.md §2) — see
        /// `artifact_kind_entry_from_spec`'s doc comment.
        pub dialect: ArtifactDialect,
    }

    /// 🗂️ One registered resource kind's full catalog entry — the descriptor plus the media capability
    /// its exporters/importers target (kept alongside rather than in `OsArtifactDescriptor` itself since
    /// the descriptor is also the wire-facing presentation shape).
    struct ArtifactKindEntry {
        descriptor: OsArtifactDescriptor,
        media_capability: OsMediaCapability,
    }

    /// 🎯️ Builds the one real `ArtifactDialect` a legacy `component_kind` slug derives into —
    /// `"s." + component_kind` at standard `"1"`, subset `"*"` (the same coordinate
    /// `native_dialect_kind` used to reformat from scratch on every call). Ticket
    /// 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: this is now the ONE place that
    /// derivation happens — `artifact_kind_entry_from_spec`/`seed_builtin_artifact_kinds` call it once
    /// at registration time and store the result, so `native_dialect_kind`, `os_artifact_dialect`, and
    /// the io-mechanism export/import path (`registry_export_media`/`registry_import_media`) all read
    /// the SAME stored value instead of three independent `format!("s.{...}")` call sites. A plugin
    /// that has migrated onto `declare_artifact` (design.md §2) would supply its subsets' own real
    /// `ArtifactDialect`s directly instead of this placeholder-derivation path — no such plugin exists
    /// yet (W1-D openQuestion #4), so every dialect this catalog holds today is still derived, not
    /// declared.
    fn dialect_from_component_kind(component_kind: &str) -> ArtifactDialect {
        ArtifactDialect { artifact_kind: format!("s.{component_kind}"), standard: "1".to_string(), subset: "*".to_string() }
    }

    fn artifact_kind_entry_from_spec(spec: &ArtifactKindSpec) -> ArtifactKindEntry {
        let dialect = dialect_from_component_kind(&spec.component_kind);
        ArtifactKindEntry {
            descriptor: OsArtifactDescriptor {
                kind: spec.id.clone(),
                name: spec.name.clone(),
                source_format: spec.source_format.clone(),
                component_kind: spec.component_kind.clone(),
                dimension: spec.dimension.clone(),
                media_type: spec.media_type,
                schema: spec.schema.clone(),
                export_formats: spec.export_formats.clone(),
                import_formats: spec.import_formats.clone(),
                export_stdio_kinds: spec.export_stdio_kinds.iter().map(|row| (*row).to_string()).collect(),
                import_stdio_kinds: spec.import_stdio_kinds.iter().map(|row| (*row).to_string()).collect(),
                dialect,
            },
            media_capability: spec.media_capability,
        }
    }

    /// 🌱️ `parameter.value` is not one app's document format — every app's parameter fields share it as
    /// their port resource kind (see `crate::instance::OsParameterFieldSpec`) — so it is seeded as a
    /// framework-level builtin instead of declared via any single app's `AppBuilder::artifact_kind(...)`.
    fn seed_builtin_artifact_kinds() -> HashMap<OsArtifactKindId, ArtifactKindEntry> {
        let mut registry = HashMap::new();
        registry.insert(
            "parameter.value".to_string(),
            ArtifactKindEntry {
                descriptor: OsArtifactDescriptor {
                    kind: "parameter.value".into(),
                    name: "Parameter".into(),
                    source_format: "parameter.value".into(),
                    component_kind: "parameter".into(),
                    dimension: "data".into(),
                    media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
                    schema: "parameter.value".into(),
                    export_formats: Vec::new(),
                    import_formats: Vec::new(),
                    export_stdio_kinds: Vec::new(),
                    import_stdio_kinds: Vec::new(),
                    dialect: dialect_from_component_kind("parameter"),
                },
                media_capability: OsMediaCapability::MeshOnly,
            },
        );
        // 🌱️ `s.workflow`/`s.space`/`s.collection` are the three backbone-document kinds the os-core
        // dissolve (`## The inversion`) now persists directly — seeded as builtins for the same reason
        // `parameter.value` is: they are not any single app's document format.
        registry.insert(
            workflow::S_WORKFLOW_SCHEMA.to_string(),
            ArtifactKindEntry {
                descriptor: OsArtifactDescriptor {
                    kind: workflow::S_WORKFLOW_SCHEMA.into(),
                    name: "Workflow".into(),
                    source_format: workflow::S_WORKFLOW_SCHEMA.into(),
                    component_kind: "workflow".into(),
                    dimension: "data".into(),
                    media_type: MediaType { class: MediaClass::Graph, form: MediaForm::Document },
                    schema: workflow::S_WORKFLOW_SCHEMA.into(),
                    export_formats: Vec::new(),
                    import_formats: Vec::new(),
                    export_stdio_kinds: Vec::new(),
                    import_stdio_kinds: Vec::new(),
                    dialect: dialect_from_component_kind("workflow"),
                },
                media_capability: OsMediaCapability::MeshOnly,
            },
        );
        registry.insert(
            space::S_SPACE_SCHEMA.to_string(),
            ArtifactKindEntry {
                descriptor: OsArtifactDescriptor {
                    kind: space::S_SPACE_SCHEMA.into(),
                    name: "Space".into(),
                    source_format: space::S_SPACE_SCHEMA.into(),
                    component_kind: "space".into(),
                    dimension: "data".into(),
                    media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Document },
                    schema: space::S_SPACE_SCHEMA.into(),
                    export_formats: Vec::new(),
                    import_formats: Vec::new(),
                    export_stdio_kinds: Vec::new(),
                    import_stdio_kinds: Vec::new(),
                    dialect: dialect_from_component_kind("space"),
                },
                media_capability: OsMediaCapability::MeshOnly,
            },
        );
        registry.insert(
            space::S_COLLECTION_SCHEMA.to_string(),
            ArtifactKindEntry {
                descriptor: OsArtifactDescriptor {
                    kind: space::S_COLLECTION_SCHEMA.into(),
                    name: "Collection".into(),
                    source_format: space::S_COLLECTION_SCHEMA.into(),
                    component_kind: "collection".into(),
                    dimension: "data".into(),
                    media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Document },
                    schema: space::S_COLLECTION_SCHEMA.into(),
                    export_formats: Vec::new(),
                    import_formats: Vec::new(),
                    export_stdio_kinds: Vec::new(),
                    import_stdio_kinds: Vec::new(),
                    dialect: dialect_from_component_kind("collection"),
                },
                media_capability: OsMediaCapability::MeshOnly,
            },
        );
        registry
    }

    /// 🗂️ Manifest-driven OS artifact catalog, populated at plugin registration time instead of hardcoding
    /// the app roster — mirrors the `crate::workflow::export_handlers()` runtime-registry pattern.
    static RESOURCE_KIND_REGISTRY: LazyLock<Mutex<HashMap<OsArtifactKindId, ArtifactKindEntry>>> = LazyLock::new(|| Mutex::new(seed_builtin_artifact_kinds()));

    /// @emoji 📚️ Registers every `ArtifactKindSpec` declared by `manifest`'s apps into the OS resource
    /// catalog — call at plugin registration time (`PluginHost::load_plugin`/`hot_swap_plugin`).
    pub fn register_artifact_descriptors(manifest: &PluginManifest) {
        let mut registry = RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for spec in &manifest.artifact_kinds {
            registry.insert(spec.id.clone(), artifact_kind_entry_from_spec(spec));
        }
        for app in &manifest.apps {
            for spec in &app.artifact_kinds {
                registry.insert(spec.id.clone(), artifact_kind_entry_from_spec(spec));
            }
        }
    }

    /// @emoji 🧪️ Registers one resource kind directly, for tests/fixtures that don't build a full
    /// `PluginManifest`.
    pub fn register_artifact_descriptor(spec: &ArtifactKindSpec) {
        RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(spec.id.clone(), artifact_kind_entry_from_spec(spec));
    }

    /// @emoji 📚️ Lists all registered OS resource descriptors, sorted by kind id for a stable snapshot.
    pub fn list_os_artifact_descriptors() -> Vec<OsArtifactDescriptor> {
        let registry = RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut descriptors: Vec<OsArtifactDescriptor> = registry.values().map(|entry| entry.descriptor.clone()).collect();
        descriptors.sort_by(|left, right| left.kind.cmp(&right.kind));
        descriptors
    }

    /// @emoji 📚️ Resolves presentation metadata for one resource kind. An unregistered kind falls back to a
    /// bare placeholder built from the kind id itself — dimension is declared by the app, never inferred
    /// from an id-prefix convention.
    pub fn os_artifact_descriptor(kind: &str) -> OsArtifactDescriptor {
        RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(kind).map(|entry| entry.descriptor.clone()).unwrap_or_else(|| OsArtifactDescriptor {
            kind: kind.into(),
            name: kind.into(),
            source_format: kind.into(),
            component_kind: "panel".into(),
            dimension: "unknown".into(),
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            schema: kind.into(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            export_stdio_kinds: Vec::new(),
            import_stdio_kinds: Vec::new(),
            dialect: dialect_from_component_kind("panel"),
        })
    }

    /// @emoji 🎯️🆕️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1b task 2: the single
    /// accessor for a resource kind's real `ArtifactDialect` — reads the SAME stored value
    /// `os_artifact_descriptor` returns embedded (`.dialect`), so `native_dialect_kind`, the
    /// io-mechanism export/import path, and any shell calling `io-routes`/`io-entries` all resolve
    /// one workflow kind id to one identical dialect. Same placeholder-fallback shape as
    /// `os_artifact_descriptor` (`"s.panel"`) for an unregistered/synthetic kind.
    pub fn os_artifact_dialect(kind: &str) -> ArtifactDialect {
        os_artifact_descriptor(kind).dialect
    }

    /// @emoji 🧬️ Registry lookup for a resource kind's media capability; unregistered kinds default to
    /// `MeshOnly` (the lighter, dependency-free representation).
    pub fn os_resource_media_capability(kind: &str) -> OsMediaCapability {
        RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(kind).map(|entry| entry.media_capability).unwrap_or(OsMediaCapability::MeshOnly)
    }

    /// @emoji 🚫️ Fail-closed sibling of `os_artifact_descriptor` for workflow connect-time validation —
    /// `None` for an unknown kind instead of a fabricated placeholder (that fallback is still right for
    /// best-effort callers like rendering, which keep using `os_artifact_descriptor`).
    pub fn try_os_artifact_descriptor(kind: &str) -> Option<OsArtifactDescriptor> {
        RESOURCE_KIND_REGISTRY.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(kind).map(|entry| entry.descriptor.clone())
    }

    // 🧷️ `resources_compatible` (`left == right` string equality) deleted — callers now compare real
    // `MediaType`s via `semio_framework::media_types_compatible`, or go through
    // `crate::workflow::negotiate_media_contract` for a full connect-time decision.
    //#endregion 🔖️ResourceDescriptors

    //#region 🔖️PluginRegistry
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsAppRegistration {
        pub id: String,
        pub role: AppRole,
        pub dialect: ArtifactDialect,
        pub label: LocalizedLabel,
        pub breadcrumb: Vec<String>,
        pub controller_id: String,
        pub inputs: Vec<semio_framework::MediaPortSpec>,
        pub outputs: Vec<semio_framework::MediaPortSpec>,
        pub source_format: String,
        pub component_kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_mode_id: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub parameter_fields: Vec<OsParameterFieldSpec>,
        pub modes: Vec<ModeDefinition>,
        /// 🧮️ The app's declared `AppDefinition.config` — how `host::reconcile_os_workflow` resolves
        /// a `plugin_id`/`app_id` app instance's `ConfigSpec` to type-check/materialize its parameter
        /// bindings (`workflow::validate_workflow_parameter_config_binding`/`instance::build_configure_config`).
        #[serde(default)]
        pub config: ConfigSpec,
    }

    pub fn os_app_primary_output_kind(registration: &OsAppRegistration) -> OsArtifactKindId {
        registration.outputs.first().and_then(|port| port.kind_id.clone()).unwrap_or_else(|| "graph.dag".into())
    }

    /// 🗂️ Production app registration store, keyed by `(plugin_id, app_id)` — populated solely by
    /// `register_app_io`, called from `PluginHost::load_plugin`/`hot_swap_plugin` beside
    /// `register_artifact_descriptors`. Replaces the deleted `BUILTIN_WORKFLOWS`/`EXTENSION_WORKFLOWS`
    /// (`OsWorkflowDefinition`-grouped) registry, which only ever existed to route around apps not
    /// declaring real `AppIo` ports in production.
    static APP_REGISTRATIONS: LazyLock<Mutex<HashMap<(String, String), OsAppRegistration>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

    /// @emoji 📚️ Derives an `OsAppRegistration` from `app.io.all_ports()` (the implicit `document:in`/
    /// `document:out` ports plus every declared `AppIo.ports` entry) and registers it — call at plugin
    /// registration time (`PluginHost::load_plugin`/`hot_swap_plugin`), beside `register_artifact_descriptors`.
    pub fn register_app_io(plugin_id: &str, app: &AppDefinition) {
        let ports = crate::host::resolve_kernel_future(app.io.all_ports());
        let (inputs, outputs): (Vec<_>, Vec<_>) = ports.into_iter().partition(|port| port.direction == semio_framework::MediaPortDirection::In);
        let registration = OsAppRegistration {
            id: app.id.clone(),
            role: app.role,
            dialect: app.dialect.clone(),
            label: app.label.clone(),
            breadcrumb: app.breadcrumb.clone(),
            controller_id: app.controller_id.clone(),
            inputs,
            outputs,
            source_format: app.io.document_schema.clone(),
            component_kind: app.io.artifact.component_kind.clone(),
            default_mode_id: Some(app.default_mode_id.clone()),
            parameter_fields: Vec::new(),
            modes: app.modes.iter().cloned().collect(),
            config: app.config.clone(),
        };
        APP_REGISTRATIONS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert((plugin_id.to_string(), app.id.clone()), registration);
    }

    pub fn os_app_registration(plugin_id: &str, app_id: &str) -> Option<OsAppRegistration> {
        APP_REGISTRATIONS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&(plugin_id.to_string(), app_id.to_string())).cloned()
    }

    /// 🧩️ Reconstructs an `AppIo` from an `OsAppRegistration`'s already-resolved `inputs`/`outputs` —
    /// the document ports' `media_type` is read back off whichever of them is present (so
    /// `AppIo::all_ports()` re-derives byte-for-byte the same document ports `register_app_io` stored),
    /// every other port becomes a declared `AppIo.ports` entry.
    fn app_io_for_registration(registration: &OsAppRegistration) -> semio_framework::AppIo {
        let document_media_type = registration
            .inputs
            .iter()
            .find(|port| port.id == "document:in")
            .or_else(|| registration.outputs.iter().find(|port| port.id == "document:out"))
            .map(|port| port.media_type)
            .unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let declared_ports: Vec<_> = registration.inputs.iter().chain(registration.outputs.iter()).filter(|port| port.id != "document:in" && port.id != "document:out").cloned().collect();
        let io = crate::host::resolve_kernel_future(semio_framework::AppIo::from_document(
            registration.source_format.clone(),
            document_media_type,
            semio_framework::ArtifactPresentation {
                id: registration.id.clone(),
                // 🚧️ No locale context reaches this reconstruction path — resolves native/English
                // pending a documented follow-up (same gap as Menu::action_with_args in the plugin SDK).
                name: registration.label.resolve(Terminology::Native, Locale::En).to_string(),
                dimension: String::new(),
                component_kind: registration.component_kind.clone(),
            },
        ));
        crate::host::resolve_kernel_future(io.with_ports(declared_ports))
    }

    /// @emoji 🧩️ Resolves the AppDefinition backing an embedded os app instance. Returns `None` if the
    /// registration declares zero modes — every app must declare at least one, so an ad hoc "inject a
    /// fake edit mode" fallback would just hide a mis-registered app instead of surfacing it. An embedded
    /// os app instance renders through exactly one component surface, so this synthesizes the single
    /// window kind that represents it rather than leaving `window_kinds` empty (now impossible). `io` is
    /// reconstructed via `app_io_for_registration` so `workflow::workflow_node_for_app` derives the same
    /// ports `register_app_io` stored — the mechanism `OsStore::add_workflow_node` mints nodes through.
    pub fn resolve_os_app_definition(plugin_id: &str, app_id: &str) -> Option<AppDefinition> {
        let registration = os_app_registration(plugin_id, app_id)?;
        let modes = semio_framework::Modes::try_from(registration.modes.clone()).ok()?;
        let default_mode_id = registration.default_mode_id.clone().unwrap_or_else(|| modes.first().id.clone());
        let window_kinds = semio_framework::WindowKinds::one(WindowKindDefinition {
            id: registration.component_kind.clone(),
            label: registration.label.clone(),
            body_key: registration.component_kind.clone(),
            surface_kind: SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: ui_wgpu::wgpu::WindowOptions::default(),
            actions: Vec::new(),
            utilities: Vec::new(),
            interactions: Vec::new(),
            params_schema: None,
            artifact_snapshot_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        });
        let io = app_io_for_registration(&registration);
        Some(AppDefinition {
            id: registration.id,
            role: registration.role,
            dialect: registration.dialect,
            label: registration.label,
            breadcrumb: registration.breadcrumb,
            icon_id: None,
            controller_id: registration.controller_id,
            modes,
            default_mode_id,
            window_kinds,
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
            utilities: Vec::new(),
            tools: Vec::new(),
            commands: Vec::new(),
            interactions: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_breadcrumbs: HashMap::new(),
            introduction: None,
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: registration.config,
            command_grammar: crate::host::resolve_kernel_future(semio_framework::CommandGrammar::empty()),
            io,
            tutorials: Vec::new(),
        })
    }

    /// @emoji 🎨️ One palette entry the browser shell can spawn a workflow node from — a thin,
    /// wire-friendly snapshot of `OsAppRegistration` (drops `ConfigSpec`/`ModeDefinition`s the
    /// palette doesn't need). `ports` is `app.io.all_ports()` so the palette UI can preview a node's
    /// wiring before it's spawned.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AppPaletteEntry {
        pub plugin_id: String,
        pub app_id: String,
        pub label: LocalizedLabel,
        pub icon_id: String,
        pub ports: Vec<semio_framework::MediaPortSpec>,
    }

    /// @emoji 🎨️ Lists every registered app as a palette entry, sorted by `(plugin_id, app_id)` for a
    /// stable snapshot. Replaces `buildSpacePrograms`'s manifest-`workflows`-driven palette (deleted with
    /// `WorkflowDefinition`/`PluginManifest.workflows` in WP-0.1) — the browser shell wiring to consume
    /// this via a wasm export lands in Wave 3 (WP-3.1); this crate only builds the plain Rust function for
    /// now (no `#[cfg(target_arch = "wasm32")]` export region exists in this file to pair it with yet).
    pub fn workflow_palette() -> Vec<AppPaletteEntry> {
        let registry = APP_REGISTRATIONS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut entries: Vec<_> = registry
            .iter()
            .map(|((plugin_id, app_id), registration)| AppPaletteEntry {
                plugin_id: plugin_id.clone(),
                app_id: app_id.clone(),
                label: registration.label.clone(),
                icon_id: registration.component_kind.clone(),
                ports: registration.inputs.iter().chain(registration.outputs.iter()).cloned().collect(),
            })
            .collect();
        entries.sort_by(|left, right| (left.plugin_id.as_str(), left.app_id.as_str()).cmp(&(right.plugin_id.as_str(), right.app_id.as_str())));
        entries
    }
    //#endregion 🔖️PluginRegistry

    //#region 🔖️PluginRegistry
    // 🚧️ B1 ripple: `WorkflowDefinition` was deleted from framework-core (WP-0.1, concurrent) — its
    // `PluginRegistry.workflows` field/`register_workflow`/`find_workflow`/`workflows()` methods (per
    // the master plan's explicit deletion list) go with it. Real workflow-palette derivation moves to
    // `register_app_io`/`workflow_palette()` (`AppIo`-driven) in Wave 1; no call sites referenced these
    // beyond the now-deleted loops in `load_plugin`/`hot_swap_plugin`/`restore`, so nothing else broke.
    pub struct PluginRegistry {
        apps: HashMap<String, AppDefinition>,
    }

    impl Default for PluginRegistry {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PluginRegistry {
        pub fn new() -> Self {
            Self { apps: HashMap::new() }
        }

        pub fn register_app(&mut self, app: AppDefinition) {
            self.apps.insert(app.id.clone(), app);
        }

        pub fn find_app(&self, app_id: &str) -> Option<&AppDefinition> {
            self.apps.get(app_id)
        }

        pub fn apps(&self) -> Vec<AppDefinition> {
            self.apps.values().cloned().collect()
        }
    }
    //#endregion 🔖️PluginRegistry

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn registers_app_io_and_resolves_registration() {
            let app = AppDefinition {
                id: "draw".into(),
                role: AppRole::Editor,
                dialect: ArtifactDialect { artifact_kind: "s.test.draw".into(), standard: "1".into(), subset: "*".into() },
                label: LocalizedLabel::data("Draw"),
                breadcrumb: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework::Modes::one(ModeDefinition { id: "edit".into(), label: LocalizedLabel::data("Edit"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework::WindowKinds::one(WindowKindDefinition {
                    id: "draw".into(),
                    label: LocalizedLabel::data("Draw"),
                    body_key: "draw".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    interactions: Vec::new(),
                    params_schema: None,
                    artifact_snapshot_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: Vec::new(),
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                interactions: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_breadcrumbs: HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: ConfigSpec::empty(),
                command_grammar: semio_framework::CommandGrammar::empty(),
                io: semio_framework::AppIo::from_document(
                    "draw.document",
                    MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    semio_framework::ArtifactPresentation { id: "draw".into(), name: "Draw".into(), dimension: "2d".into(), component_kind: "draw".into() },
                ),
                tutorials: Vec::new(),
            };
            register_app_io("draw", &app);
            let registration = os_app_registration("draw", "draw").expect("registration");
            assert_eq!(registration.source_format, "draw.document");
            let palette = workflow_palette();
            assert!(palette.iter().any(|entry| entry.plugin_id == "draw" && entry.app_id == "draw"));
        }
    }
    //#endregion 🧪️Tests
    // #endregion registry
}

#[cfg(feature = "os-host-full")]
#[cfg(feature = "os-host-full")]
pub use crate::space::*;
#[cfg(feature = "os-host-full")]
pub use crate::workflow::{
    apply_flow_fixture_to_os_workflow, apply_workflow_operation, build_os_workflow_operator_infos, create_default_workflow_parameter, empty_workflow, empty_workflow_snapshot, export_os_app_instance_media_kind, import_os_app_instance_media_kind,
    negotiate_media_contract, os_media_export_extension_for_format_kind, os_media_neuron_kind_for_node, os_resource_media_capability, os_workflow_to_flow_fixture, os_workflow_to_node_graph_payload, patch_workflow_parameter,
    placeholder_media_contract, plan_workflow, sync_workflow_parameter_ports, validate_workflow, validate_workflow_parameter_config_binding, validate_workflow_snapshot, workflow_node_for_app, workflow_parameter_id,
    workflow_parameter_id_from_port_id, workflow_parameter_name, workflow_parameter_types_compatible, workflow_parameter_value, MediaContract, OsMediaCapability, OsWorkflowCamera, OsWorkflowNodeGraphPayload, OsWorkflowOperatorInfo, Workflow,
    WorkflowDelivery, WorkflowEdge, WorkflowFixture, WorkflowInput, WorkflowInputBinding, WorkflowMediaPort, WorkflowMutation, WorkflowNode, WorkflowOutputBinding, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterPatch,
    WorkflowParameterType, WorkflowPosition, WorkflowSnapshot, WorkflowValidation, OS_MEDIA_FLOW_MODULE_ID, OS_SPACE_SCHEMA, OS_WORKFLOW_VFS_ROOT_ID, S_WORKFLOW_SCHEMA, WORKFLOW_SCHEMA,
};
#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "os-host-full")]
pub use backbone::{open_file_space_backbone, open_folder_space_backbone};
#[cfg(feature = "os-host-full")]
pub use host::{
    create_backbone_document, create_os_space, decode_backbone_payload, delete_os_space, encode_backbone_payload, export_backbone_dsl, export_backbone_pack, export_os_space_dsl, export_os_space_pack, import_os_space_from_dsl,
    import_os_space_from_pack, list_os_space_catalog_entries, load_os_space_document, materialize_backbone_snapshot, seed_os_space_catalog_if_empty, BackboneDocument, LoadedProgram, OsBackbonePort, OsBackbonePorts, OsCollectionDocument,
    OsSpaceCatalogEntry, OsSpaceDocument, OsSpaceStore, OsWorkflowArtifactDocument, OsWorkflowStore, PluginHost, ProgramHotSwapEvent, OS_HOME_VFS_ROOT_ID, OS_SPACE_BACKBONE_URI_PREFIX,
};
#[cfg(feature = "os-host-full")]
#[cfg(feature = "os-host-full")]
pub use instance::{
    apply_parameter_values_to_snapshot, create_default_os_parameter, create_os_artifact_id, create_os_id, is_parameter_port_id, materialize_os_app_instance_document_json, media_port_id_for_spec, media_port_spec_id, os_fixture_json,
    os_parameter_types_compatible, os_parameter_value, parameter_id_from_port_id, parameter_port_id, patch_os_parameter, register_os_fixture_json, resolve_parameter_values_for_instance, set_json_pointer_value, OsArtifactRef, OsInstanceState,
    OsParameter, OsParameterFieldBinding, OsParameterFieldSpec, OsParameterType, OS_PARAMETER_PORT_PREFIX,
};
pub use media_export_raster::{
    dwg_drawing_to_svg, media_accept_filter_kinds, rasterize_svg_to_png_base64, register_2d_export_handlers, register_mesh_dwg_export_handler, register_mesh_dwg_import_handler, register_mesh_exporter, register_mesh_importer,
    register_os_media_export_handler_kind, register_os_media_import_handler_kind, svg_to_dwg_bytes, OsMediaExportResult,
};
pub use media_export_simple::{map_points_svg, pages_rects_svg, title_card_svg, wrap_svg};
#[cfg(feature = "os-host-full")]
#[cfg(feature = "os-host-full")]
pub use registry::{
    list_os_artifact_descriptors, os_app_primary_output_kind, os_app_registration, os_artifact_descriptor, os_artifact_dialect, register_app_io, register_artifact_descriptor, register_artifact_descriptors, resolve_os_app_definition,
    try_os_artifact_descriptor, workflow_palette, AppPaletteEntry, OsAppRegistration, OsArtifactDescriptor, OsArtifactKindId, PluginRegistry,
};
pub use semio_framework::*;
pub use store::{document_backbone_ref, set_host_backbone_port, ArtifactBackboneRef, ArtifactCommand, LocalStorageBackbonePort, MemoryBackbonePort};
pub use ui_wgpu::wgpu::*;
pub use vcs::{Author, Checkpoint, VcsError};
