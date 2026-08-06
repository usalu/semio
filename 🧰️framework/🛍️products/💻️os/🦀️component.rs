//! 🖥️ Plugin-based OS kernel: hot-swappable WASM plugins, workflow, document VCS.
// 🪶️ `linkage` (nightly, already the pinned toolchain workspace-wide): lets
// `📦️plugin_bundle_installer_shim.rs`'s fallback stub declare itself weak — see that file's docstring.
#![feature(linkage)]

pub mod host {
    // #region host
    //! 🔌️ Plugin host, studio document VCS store, backbone, and catalog.

    use crate::instance::{create_os_id, OsInstanceState};
    use crate::registry::{os_app_registration, resolve_os_app_definition, PluginRegistry};
    use protocol::Operation;
    use semio_framework_core::{AppDefinition, Contribution, PluginManifest, ViewState};
    use serde::{Deserialize, Serialize};
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, LazyLock, Mutex};
    use store::{create_document_envelope, document_backbone_ref, materialize_document_projection, DocumentBackboneRef, DocumentCommand, DocumentEnvelope, DocumentStore, SpaceConflict};
    use ui_wgpu::wgpu::{ui_recovery_panel, UiNode};
    use vcs::{DocumentVcs, VcsError};

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
        pub contribution: Contribution,
    }

    //#region 🔖️ProgramSupervisorState
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ProgramSupervisorState {
        Loaded,
        Running,
        Crashed,
        TimedOut,
        Restarting,
        Quarantined,
        Unloaded,
    }
    //#endregion 🔖️ProgramSupervisorState

    pub struct PluginHost {
        registry: PluginRegistry,
        instances: HashMap<u32, OsInstanceState>,
        next_instance_id: u32,
        programs: HashMap<String, LoadedProgram>,
        supervisor: HashMap<String, ProgramSupervisorState>,
    }

    impl Default for PluginHost {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PluginHost {
        pub fn new() -> Self {
            Self { registry: PluginRegistry::new(), instances: HashMap::new(), next_instance_id: 1, programs: HashMap::new(), supervisor: HashMap::new() }
        }

        pub fn supervisor_state(&self, plugin_id: &str) -> Option<ProgramSupervisorState> {
            self.supervisor.get(plugin_id).copied()
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
            self.supervisor.insert(plugin_id.clone(), ProgramSupervisorState::Running);
            ProgramHotSwapEvent { plugin_id, version, added_apps: next_apps.iter().filter(|app| !previous_apps.contains(app)).cloned().collect(), removed_apps: previous_apps.iter().filter(|app| !next_apps.contains(app)).cloned().collect() }
        }

        pub fn hot_swap_plugin(&mut self, program: LoadedProgram) -> ProgramHotSwapEvent {
            let plugin_id = program.plugin_id.clone();
            let rollback = HotSwapRollback { previous_plugin: self.programs.get(&plugin_id).cloned(), instance_generations: self.instances.iter().map(|(id, state)| (*id, state.generation)).collect() };

            if let Err(error) = validate_plugin_manifest(&program) {
                self.supervisor.insert(plugin_id.clone(), ProgramSupervisorState::Loaded);
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
            self.supervisor.insert(plugin_id.clone(), ProgramSupervisorState::Running);
            ProgramHotSwapEvent { plugin_id, version, added_apps: next_apps.iter().filter(|app| !previous_apps.contains(app)).cloned().collect(), removed_apps: previous_apps.iter().filter(|app| !next_apps.contains(app)).cloned().collect() }
        }

        pub fn apps(&self) -> Vec<AppDefinition> {
            self.registry.apps()
        }

        pub fn contributions(&self) -> Vec<ProgramContributionEntry> {
            let mut entries = Vec::new();
            for loaded in self.programs.values() {
                for contribution in &loaded.manifest.contributions {
                    entries.push(ProgramContributionEntry { plugin_id: loaded.plugin_id.clone(), contribution: contribution.clone() });
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
            self.instances.insert(id, OsInstanceState { id, app_id: app.id.clone(), controller_id: app.controller_id.clone(), document_json, view_state: ViewState::default(), generation: 0 });
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
        /// has no locale on hand at this call site (no `ViewState` threaded into `recovery_ui`), so
        /// `is_de` is pinned to `false` (English) until a locale source is plumbed through.
        pub fn recovery_ui(&self, plugin_id: &str) -> UiNode {
            let quarantined = self.supervisor.get(plugin_id).copied() == Some(ProgramSupervisorState::Quarantined);
            ui_recovery_panel(plugin_id, quarantined, false)
        }
        //#endregion 🔖️ActionKernel

        pub fn set_view_state(&mut self, instance_id: u32, view_state: ViewState) {
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
            self.supervisor.insert(plugin_id.clone(), ProgramSupervisorState::Loaded);
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
    /// (schema/id/name/vcs/applied_edit_ids/backbone), parametrized over any `<P, Op>` pair
    /// `store::create_document_envelope`/`materialize_document_projection`/`print_document_pack`/
    /// `parse_document_pack` already support generically — nothing OS-specific left to hardcode. See
    /// `## The inversion` in the plan: `OsProjection`/`OsOperation`/`OsDocument` dissolve into the three
    /// type aliases below instead of one bespoke studio-only document type.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BackboneDocument<P, Op> {
        pub schema: String,
        pub id: String,
        pub name: String,
        pub vcs: DocumentVcs<P, Op>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub applied_edit_ids: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backbone: Option<DocumentBackboneRef>,
    }

    /// 🏠️ A space's manifest document — the space-catalog half of the dissolved `OsProjection`.
    pub type OsSpaceDocument = BackboneDocument<space::SpaceProjection, space::SpaceOperation>;
    /// 🗂️ One collection's folder/entry tree document.
    pub type OsCollectionDocument = BackboneDocument<space::CollectionProjection, space::CollectionOperation>;
    /// 🕸️ One `s.workflow` artifact document — the workflow-graph half of the dissolved `OsProjection`
    /// (see the kernel `workflow` crate's `WorkflowDocument`).
    pub type OsWorkflowArtifactDocument = BackboneDocument<workflow::WorkflowDocument, workflow::WorkflowOperation>;

    /// 🌉️ Live `DocumentStore` handle for a space-manifest session — no bespoke wrapper needed (unlike
    /// `OsWorkflowStore` below, whose only extra logic is workflow-specific node/parameter id-minting);
    /// every generic `DocumentStore` method (`projection`/`dispatch`/`attach_backbone`/...) already
    /// applies directly. `OsWorkflowStore::add_workflow_node` dispatches into one of these to install
    /// the spawned app's plugin into the owning space's `programs` list.
    pub type OsSpaceStore = DocumentStore<space::SpaceProjection, space::SpaceOperation>;

    /// @emoji 🌱️ Mints a fresh backbone document wrapping `initial_projection` with empty edit history.
    pub fn create_backbone_document<P, Op>(schema: &str, id: &str, name: &str, initial_projection: P) -> BackboneDocument<P, Op>
    where
        P: Clone,
    {
        BackboneDocument { schema: schema.into(), id: id.into(), name: name.into(), vcs: create_document_envelope::<P, Op>(schema, id, initial_projection, None).vcs, applied_edit_ids: Vec::new(), backbone: None }
    }

    /// @emoji 🌉️ Builds the bare `DocumentEnvelope` a `BackboneDocument` wraps (dropping the app-level
    /// `name`/`applied_edit_ids` fields) — shared by every typed pack/text export path below.
    fn backbone_envelope_of<P, Op>(document: &BackboneDocument<P, Op>) -> DocumentEnvelope<P, Op>
    where
        P: Clone,
        Op: Clone,
    {
        DocumentEnvelope { schema: document.schema.clone(), id: document.id.clone(), vcs: document.vcs.clone(), backbone: document.backbone.clone(), active_alternative_id: None, cursor: None }
    }

    pub fn materialize_backbone_projection<P, Op>(document: &BackboneDocument<P, Op>, applied_edit_ids: &[String]) -> Result<P, VcsError>
    where
        P: Clone,
        Op: Clone + Operation<P>,
    {
        let envelope = backbone_envelope_of(document);
        materialize_document_projection(&envelope, applied_edit_ids)
    }

    /// @emoji 📤️ Exports an already-loaded backbone document as pack bytes + ops text.
    pub fn export_backbone_pack<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::DocumentPackFiles, VcsError>
    where
        P: Clone + store::DocumentPack,
        Op: Clone + protocol::OpText + protocol::OpBinary,
    {
        store::print_document_pack(&backbone_envelope_of(document))
    }

    /// @emoji 📤️ DSL-text counterpart of `export_backbone_pack`.
    pub fn export_backbone_dsl<P, Op>(document: &BackboneDocument<P, Op>) -> Result<store::DocumentTextFiles, VcsError>
    where
        P: Clone + store::DocumentDsl,
        Op: Clone + protocol::OpText,
    {
        store::print_document_text(&backbone_envelope_of(document))
    }

    /// @emoji 📦️ Binary pack+spr payload for the whole `BackboneDocument` (name + applied-edit cursor +
    /// vcs) — the persisted/synced form. `name` rides as a `store::encode_document_pack_bytes`-framed
    /// blob wrapping a nested `pack`+`spr` pair, and `applied_edit_ids` rides through the envelope's
    /// `cursor` so `spr`'s cursor line restores the exact undo/redo position.
    pub fn encode_backbone_payload<P, Op>(document: &BackboneDocument<P, Op>) -> Result<Vec<u8>, VcsError>
    where
        P: Clone + store::DocumentPack,
        Op: Clone + protocol::OpText + protocol::OpBinary,
    {
        let mut envelope = backbone_envelope_of(document);
        envelope.cursor = Some(store::DocumentCursor { applied_edit_ids: document.applied_edit_ids.clone(), redo_edit_ids: Vec::new(), checkpoint_id: None });
        let files = store::print_document_pack(&envelope)?;
        let inner = store::encode_document_pack_bytes(&files.pack, &files.spr);
        Ok(store::encode_document_pack_bytes(document.name.as_bytes(), &inner))
    }

    /// @emoji 📥️ Inverse of `encode_backbone_payload` — `expected_schema` guards against decoding one
    /// document kind's bytes as another.
    pub fn decode_backbone_payload<P, Op>(bytes: &[u8], expected_schema: &str) -> Result<BackboneDocument<P, Op>, VcsError>
    where
        P: Clone + store::DocumentPack,
        Op: Clone + protocol::OpText + protocol::OpBinary + Operation<P>,
    {
        let (name_bytes, inner) = store::decode_document_pack_bytes(bytes)?;
        let name = String::from_utf8(name_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let (pack, spr) = store::decode_document_pack_bytes(&inner)?;
        let parsed: store::ParsedDocumentText<P, Op> = store::parse_document_pack(&pack, &spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        if parsed.envelope.schema != expected_schema {
            return Err(VcsError::Deserialize(format!("expected schema {expected_schema}")));
        }
        let applied_edit_ids = parsed.envelope.cursor.as_ref().map(|cursor| cursor.applied_edit_ids.clone()).unwrap_or_default();
        Ok(BackboneDocument { schema: parsed.envelope.schema, id: parsed.envelope.id, name, vcs: parsed.envelope.vcs, applied_edit_ids, backbone: parsed.envelope.backbone })
    }
    //#endregion 🔖️BackboneDocument

    //#region 🔖️GraphReconcile
    /// @emoji 🧵️ Post-materialization workflow integrity pass, invoked explicitly by
    /// `OsWorkflowStore::projection_with_conflicts` (NOT through `Operation::reconcile` — the kernel
    /// `workflow::WorkflowOperation` inherits that trait hook's no-op default, since the two rules that
    /// used to run alongside these purely-structural ones need the os-core plugin/artifact registry the
    /// kernel crate doesn't have; see `workflow::WorkflowOperation`'s own doc). Runs, in order: (1) drop
    /// edges whose source/target node or port no longer exists (a concurrent delete tombstone wins over
    /// the wiring), (2) drop edges whose port types no longer match (a concurrent re-typing wins over
    /// the wiring), (3) dedupe edges with identical endpoints down to the lexicographically smallest id
    /// (deterministic across peers replaying the same operation log), (4) break any cycle the previous
    /// rules left behind, (5) drop parameter bindings whose target config field or parameter type no
    /// longer validates. Each rule operates on the edge/binding set the previous one produced.
    fn reconcile_workflow_document(mut document: workflow::WorkflowDocument) -> (workflow::WorkflowDocument, Vec<SpaceConflict>) {
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
                conflicts.push(SpaceConflict {
                    kind: "workflow/edge-orphaned".into(),
                    uri: edge.id.clone(),
                    message: format!("edge {} references a node or port that no longer exists ({}:{} -> {}:{})", edge.id, edge.source_node_id, edge.source_port_id, edge.target_node_id, edge.target_port_id),
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
            match crate::workflow::negotiate_media_contract(source_port, target_port) {
                Ok(contract) if contract == edge.contract => true,
                Ok(_) => {
                    conflicts.push(SpaceConflict { kind: "workflow/edge-type-mismatch".into(), uri: edge.id.clone(), message: format!("edge {} contract stale: no longer matches negotiated port types", edge.id) });
                    false
                }
                Err(reason) => {
                    conflicts.push(SpaceConflict { kind: "workflow/edge-type-mismatch".into(), uri: edge.id.clone(), message: format!("edge {} connects ports whose types no longer match: {reason}", edge.id) });
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
        // as `TypeMismatchDrop`, ported from os-core's dissolved `OsProjection`-based reconcile.
        let parameters = document.parameters.clone();
        let nodes = document.graph.nodes.clone();
        document.parameter_bindings.retain(|binding| {
            let Some(node) = nodes.iter().find(|node| node.id == binding.node_id) else { return true };
            let Some(registration) = os_app_registration(&node.plugin_id, &node.app_id) else { return true };
            let Some(parameter_type) = parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == binding.parameter_id).map(workflow_parameter_type_of) else { return true };
            match workflow::validate_workflow_parameter_config_binding(binding, &parameter_type, &registration.config) {
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
    /// `reconcile_workflow_document` only receives the materialized `WorkflowDocument` by value, not
    /// per-edge `HybridLogicalTimestamp`s from the edit log. `apply_workflow_operation`'s `ConnectPorts`
    /// handler appends new edges to the end of the vec, so a higher index approximates a later edit;
    /// true HLT-based tie-breaking would need this pass to also see edit history, not just the document.
    fn drop_workflow_cycle_edges(mut edges: Vec<workflow::WorkflowEdge>, conflicts: &mut Vec<SpaceConflict>) -> Vec<workflow::WorkflowEdge> {
        while let Some(cycle_node_ids) = find_workflow_cycle_participants(&edges) {
            let newest_cycle_edge_index = edges.iter().enumerate().filter(|(_, edge)| cycle_node_ids.contains(&edge.source_node_id) && cycle_node_ids.contains(&edge.target_node_id)).map(|(index, _)| index).max();
            let Some(newest_cycle_edge_index) = newest_cycle_edge_index else { break };
            let dropped = edges.remove(newest_cycle_edge_index);
            conflicts.push(SpaceConflict { kind: "workflow/edge-cycle".into(), uri: dropped.id.clone(), message: format!("edge {} was dropped to break a cycle in the workflow", dropped.id) });
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
        inner: DocumentStore<workflow::WorkflowDocument, workflow::WorkflowOperation>,
        name: String,
    }

    impl OsWorkflowStore {
        pub fn new(document: OsWorkflowArtifactDocument) -> Self {
            let applied_edit_ids = document.applied_edit_ids.clone();
            let envelope = DocumentEnvelope { schema: document.schema, id: document.id, vcs: document.vcs, backbone: document.backbone, active_alternative_id: None, cursor: None };
            let mut inner = DocumentStore::new(envelope);
            if !applied_edit_ids.is_empty() {
                let snapshot = inner.envelope().clone();
                inner.reset(snapshot, applied_edit_ids, Vec::new()).expect("reset snapshot");
            }
            Self { inner, name: document.name }
        }

        pub fn generation(&self) -> u64 {
            self.inner.generation()
        }

        pub fn projection(&self) -> Result<workflow::WorkflowDocument, VcsError> {
            self.inner.projection()
        }

        /// @emoji 🤝️ Fresh replay plus `reconcile_workflow_document`'s whole 4(+1)-rule pipeline —
        /// invoked explicitly here rather than through `Operation::reconcile` (a no-op default at the
        /// kernel-crate layer, since two of those rules need the os-core plugin/artifact registry).
        pub fn projection_with_conflicts(&self) -> Result<(workflow::WorkflowDocument, Vec<SpaceConflict>), VcsError> {
            let document = self.inner.projection()?;
            Ok(reconcile_workflow_document(document))
        }

        pub fn document(&self) -> OsWorkflowArtifactDocument {
            let envelope = self.inner.envelope();
            BackboneDocument { schema: envelope.schema.clone(), id: envelope.id.clone(), name: self.name.clone(), vcs: envelope.vcs.clone(), applied_edit_ids: self.inner.applied_edit_ids().to_vec(), backbone: envelope.backbone.clone() }
        }

        pub fn dispatch_text(&mut self, command_text: &str) -> Result<(), VcsError> {
            self.inner.dispatch_text(command_text)
        }

        pub fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<(), VcsError> {
            self.inner.dispatch_binary(command_bytes)
        }

        pub fn dispatch_apply(&mut self, operations: Vec<workflow::WorkflowOperation>) -> Result<(), VcsError> {
            self.inner.dispatch(DocumentCommand::Apply { operations, description: None })
        }

        pub fn set_workflow_name(&mut self, name: &str) {
            self.name = name.into();
            let _ = self.inner.generation();
        }

        /// @emoji 🆔️ Mints a fresh `WorkflowNode` (id, ports, document/config refs — everything) via
        /// `workflow::workflow_node_for_app`, at dispatch time, so replay never re-derives it. Also
        /// dispatches `space::SpaceOperation::InstallProgram` against `space_store` — the owning space's
        /// `programs` list moved off the dissolved `OsProjection` onto `space::SpaceProjection` (see
        /// `## The inversion` in the plan), so spawning a node into the workflow graph and installing its
        /// plugin into the space are now two operations against two separate documents.
        pub fn add_workflow_node(&mut self, plugin_id: &str, app_id: &str, label: Option<&str>, x: f64, y: f64, space_store: &mut OsSpaceStore) -> Result<String, VcsError> {
            let app = resolve_os_app_definition(plugin_id, app_id).ok_or_else(|| VcsError::Deserialize(format!("unknown app {plugin_id}/{app_id}")))?;
            let node_id = create_os_id("node");
            let position = workflow::WorkflowPosition { x, y, width: 0.0, height: 0.0 };
            let mut node = workflow::workflow_node_for_app(&app, plugin_id, &node_id, &position);
            if let Some(label) = label {
                node.label = label.into();
            }
            self.dispatch_apply(vec![workflow::WorkflowOperation::AddNode { node }])?;
            space_store.dispatch(DocumentCommand::Apply { operations: vec![space::SpaceOperation::InstallProgram { plugin_id: plugin_id.into() }], description: None })?;
            Ok(node_id)
        }

        pub fn add_parameter(&mut self, parameter_type: &workflow::WorkflowParameterType, name: &str) -> Result<String, VcsError> {
            let parameter = workflow::create_default_workflow_parameter(parameter_type, name, None);
            let parameter_id_value = workflow::workflow_parameter_id(&parameter).to_string();
            self.dispatch_apply(vec![workflow::WorkflowOperation::AddParameter { parameter }])?;
            Ok(parameter_id_value)
        }

        pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &workflow::WorkflowParameterPatch) -> Result<(), VcsError> {
            let document = self.projection()?;
            let current = document.parameters.iter().find(|parameter| workflow::workflow_parameter_id(parameter) == target_parameter_id).cloned().ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
            let next = workflow::patch_workflow_parameter(&current, patch);
            self.dispatch_apply(vec![workflow::WorkflowOperation::PatchParameter { parameter_id: target_parameter_id.into(), parameter: next }])
        }

        /// @emoji 📡️ Pumps any queued inbound backbone messages into the edit timeline.
        pub fn tick(&mut self) -> Result<bool, VcsError> {
            self.inner.tick()
        }

        /// @emoji 🔗️ Resolves and attaches a backbone by uri. Only available inside the wasm sandbox
        /// (every scheme forwards to the host over the injected `BackboneChannelPort`, a pure queue) —
        /// see {@link attach_backbone} for the native counterpart, which takes an explicit
        /// `Box<dyn store::Backbone>` since native has no URI→IO auto-resolution anymore (`framework/sync`'s
        /// `host_runtime` module owns constructing the real endpoint via `DocumentHost`).
        #[cfg(target_arch = "wasm32")]
        pub fn attach_backbone(&mut self, uri: &str) -> Result<(), VcsError> {
            self.inner.attach_backbone_uri(uri)
        }

        /// @emoji 🔗️ Attaches an explicit native backbone channel (typically a `channel_backbone` handed
        /// out by `framework/sync`'s `DocumentHost::open`, per `host_runtime`'s canonical sequence).
        #[cfg(not(target_arch = "wasm32"))]
        pub fn attach_backbone(&mut self, backbone: Box<dyn store::Backbone>) -> Result<(), VcsError> {
            self.inner.attach_backbone(backbone)
        }

        pub fn detach_backbone(&mut self) {
            self.inner.detach_backbone();
        }

        pub fn backbone_ref(&self) -> Option<&DocumentBackboneRef> {
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
    impl<T: store::BackbonePort> OsBackbonePort for T {
        fn read(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
            use base64::Engine;
            let text = store::BackbonePort::read(self, uri)?;
            if text.is_empty() {
                return Ok(Vec::new());
            }
            base64::engine::general_purpose::STANDARD.decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
        }

        fn write(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
            use base64::Engine;
            if payload.is_empty() {
                return store::BackbonePort::write(self, uri, "");
            }
            store::BackbonePort::write(self, uri, &base64::engine::general_purpose::STANDARD.encode(payload))
        }
    }

    /// @emoji 🌉️ Writes any `BackboneDocument<P, Op>` to `uri`, stamping its own `backbone` ref first —
    /// shared by every catalog write path below (space manifests, collections).
    fn sync_backbone_document<P, Op>(document: &BackboneDocument<P, Op>, backbone_uri: &str, port: &Arc<dyn OsBackbonePort>) -> Result<(), VcsError>
    where
        P: Clone + store::DocumentPack,
        Op: Clone + protocol::OpText + protocol::OpBinary,
    {
        let mut synced = document.clone();
        synced.backbone = Some(document_backbone_ref(backbone_uri));
        port.write(backbone_uri, &encode_backbone_payload(&synced)?)
    }
    //#endregion 🔖️Backbone

    // 🫀️ Presence used to be a `presence:` backbone-URI polling hack (`OS_PRESENCE_URI_PREFIX` /
    // `write_os_presence` / `read_os_presence_peers`) — deleted. Presence now flows through the semio_hub's
    // duplex `PresencePeer`/`HubServerFrame::Presence` frames (`framework/core/rs`'s 🔖️HubProtocol
    // region) via `framework/sync`'s `DocumentHost::subscribe` yielding `DocumentEvent::Presence`; the
    // `host_runtime` module below is where a native host translates that event into
    // `ViewState.presence_peers_json` — the plugin read-side contract is unchanged.

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

    fn port_key(port: &Arc<dyn OsBackbonePort>) -> usize {
        Arc::as_ptr(port) as *const () as usize
    }

    fn track_os_space_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
        SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(port_key(port)).or_default().insert(uri.into());
    }

    fn untrack_os_space_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
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
        let projection = materialize_backbone_projection(document, &[])?;
        let updated_at = document.vcs.changes.last().map(|change| change.saved_at.clone()).unwrap_or_else(|| "0".into());
        Ok(OsSpaceCatalogEntry { id: space_id, name: document.name.clone(), backbone_uri: backbone_uri.into(), kind: projection.kind, visibility: projection.visibility, collection_count: projection.collections.len(), updated_at })
    }

    /// @emoji 📚️ Lists persisted space manifests from the dev backbone namespace.
    pub fn list_os_space_catalog_entries(port: Arc<dyn OsBackbonePort>) -> Result<Vec<OsSpaceCatalogEntry>, VcsError> {
        let mut entries = Vec::new();
        let uris: Vec<String> = SPACE_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&port_key(&port)).cloned().unwrap_or_default().into_iter().collect();
        for uri in uris {
            if os_space_id_from_backbone_uri(&uri).is_none() {
                continue;
            }
            let payload = port.read(&uri)?;
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
    pub fn create_os_space(name: &str, kind: space::SpaceKind, visibility: space::SpaceVisibility, owner: space::SpaceUser, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let space_id = create_os_id("space");
        let collection_id = create_os_id("collection");
        let mut space_projection = space::empty_space_projection(name.trim(), kind, visibility);
        space_projection.users.push(owner);
        space_projection.collections.push(space::CollectionRef { id: collection_id.clone(), name: "main".into(), document_id: collection_id.clone() });
        let space_document: OsSpaceDocument = create_backbone_document(space::S_SPACE_SCHEMA, &space_id, name.trim(), space_projection);
        let collection_document: OsCollectionDocument = create_backbone_document(space::S_COLLECTION_SCHEMA, &collection_id, "main", space::empty_collection_projection("main"));

        let space_uri = space::space_backbone_uri(&space_id);
        let collection_uri = space::collection_backbone_uri(&space_id, &collection_id);
        sync_backbone_document(&space_document, &space_uri, &port)?;
        sync_backbone_document(&collection_document, &collection_uri, &port)?;
        track_os_space_backbone_uri(&port, &space_uri);
        track_os_space_backbone_uri(&port, &collection_uri);
        os_space_catalog_entry_from_document(&space_uri, &space_document)
    }

    /// @emoji 🗑️ Deletes a space manifest and every collection it references from the dev backbone.
    pub fn delete_os_space(space_id: &str, port: Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
        let uri = space::space_backbone_uri(space_id);
        if let Ok(document) = load_os_space_document(space_id, port.clone()) {
            if let Ok(projection) = materialize_backbone_projection(&document, &[]) {
                for collection in &projection.collections {
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
    fn admit_os_space_document(mut document: OsSpaceDocument, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
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
    pub fn import_os_space_from_dsl(dsl: &str, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let projection = <space::SpaceProjection as store::DocumentDsl>::parse_dsl(dsl).map_err(|error| VcsError::Deserialize(error.message))?;
        let vcs = create_document_envelope::<space::SpaceProjection, space::SpaceOperation>(space::S_SPACE_SCHEMA, "", projection, None).vcs;
        admit_os_space_document(BackboneDocument { schema: space::S_SPACE_SCHEMA.into(), id: String::new(), name: String::new(), vcs, applied_edit_ids: Vec::new(), backbone: None }, port)
    }

    /// @emoji 📦️ Pack counterpart of `import_os_space_from_dsl`.
    pub fn import_os_space_from_pack(pack: &[u8], spr: &[u8], port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let parsed: store::ParsedDocumentText<space::SpaceProjection, space::SpaceOperation> = store::parse_document_pack(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let applied_edit_ids = parsed.envelope.cursor.as_ref().map(|cursor| cursor.applied_edit_ids.clone()).unwrap_or_default();
        let document = BackboneDocument { schema: parsed.envelope.schema, id: parsed.envelope.id, name: String::new(), vcs: parsed.envelope.vcs, applied_edit_ids, backbone: parsed.envelope.backbone };
        admit_os_space_document(document, port)
    }

    /// @emoji 📤️ Exports an already-loaded space manifest as pack bytes + ops text.
    pub fn export_os_space_pack(document: &OsSpaceDocument) -> Result<store::DocumentPackFiles, VcsError> {
        export_backbone_pack(document)
    }

    /// @emoji 📤️ DSL-text counterpart of `export_os_space_pack`.
    pub fn export_os_space_dsl(document: &OsSpaceDocument) -> Result<store::DocumentTextFiles, VcsError> {
        export_backbone_dsl(document)
    }

    /// @emoji 📂️ Loads a space manifest from the dev backbone.
    pub fn load_os_space_document(space_id: &str, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceDocument, VcsError> {
        let backbone_uri = space::space_backbone_uri(space_id);
        let payload = port.read(&backbone_uri)?;
        if payload.is_empty() {
            return Err(VcsError::Backbone(format!("unknown os space: {space_id}")));
        }
        decode_backbone_payload(&payload, space::S_SPACE_SCHEMA)
    }

    /// @emoji 🌱️ Seeds the demo space when the catalog is empty.
    pub fn seed_os_space_catalog_if_empty(seed_document: OsSpaceDocument, port: Arc<dyn OsBackbonePort>) -> Result<Option<OsSpaceCatalogEntry>, VcsError> {
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

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::workflow::{empty_workflow, placeholder_media_contract, validate_workflow, MediaContract, WorkflowEdge, WorkflowPosition};
        use semio_framework_core::{MediaClass, MediaForm, MediaType, MediaWireFormat, ModeDefinition, OsMediaFormat, PluginManifest, WindowKindDefinition};
        use std::sync::Arc;
        use store::{MemoryBackbone, MemoryBackbonePort};
        use ui_wgpu::wgpu::SurfaceKind;

        #[test]
        fn loads_plugin_apps_into_registry() {
            let mut host = PluginHost::new();
            let manifest = PluginManifest {
                plugin_id: "draw".into(),
                label: "Draw".into(),
                version: "0.1.0".into(),
                apps: vec![AppDefinition {
                    id: "draw-play".into(),
                    label: "Draw".into(),
                    document: vec!["semio".into(), "draw".into()],
                    icon_id: None,
                    controller_id: "draw-play".into(),
                    modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                    default_mode_id: "edit".into(),
                    window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                        id: "composite".into(),
                        label: "Canvas".into(),
                        body_key: "composite".into(),
                        surface_kind: SurfaceKind::Canvas2d,
                        icon_id: "pen-tool".into(),
                        options: ui_wgpu::wgpu::WindowOptions::default(),
                        actions: Vec::new(),
                        utilities: Vec::new(),
                        params_schema: None,
                        document_projection_schema: None,
                        input_event_schema: None,
                        output_schema: None,
                        capabilities: vec![],
                    }),
                    panel_tabs: vec![],
                    keybindings: vec![],
                    actions: vec![],
                    utilities: Vec::new(),
                    tools: Vec::new(),
                    commands: Vec::new(),
                    named_layouts: Vec::new(),
                    default_layout: None,
                    terminologies: Vec::new(),
                    terminology_documents: std::collections::HashMap::new(),
                    introduction: None,
                    dialogs: Vec::new(),
                    media_inputs: Vec::new(),
                    media_outputs: Vec::new(),
                    artifact_kinds: Vec::new(),
                    config: semio_framework_core::ConfigSpec::empty(),
                    command_grammar: semio_framework_core::CommandGrammar::empty(),
                    io: semio_framework_core::AppIo::default(),
                    tutorials: Vec::new(),
                }],
                capabilities: vec![],
                contributions: vec![],
                examples: vec![],
                commands: vec![],
            };
            host.load_plugin(LoadedProgram { plugin_id: "draw".into(), manifest, artifact_uri: "program://draw".into() });
            assert_eq!(host.apps().len(), 1);
        }

        #[test]
        fn hot_swap_bumps_instance_generation_and_tracks_app_changes() {
            let mut host = PluginHost::new();
            let draw_app = AppDefinition {
                id: "draw-play".into(),
                label: "Draw".into(),
                document: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_documents: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework_core::ConfigSpec::empty(),
                command_grammar: semio_framework_core::CommandGrammar::empty(),
                io: semio_framework_core::AppIo::default(),
                tutorials: Vec::new(),
            };
            let note_app = AppDefinition {
                id: "note-play".into(),
                label: "Note".into(),
                document: vec!["semio".into(), "note".into()],
                icon_id: None,
                controller_id: "note-play".into(),
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_documents: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework_core::ConfigSpec::empty(),
                command_grammar: semio_framework_core::CommandGrammar::empty(),
                io: semio_framework_core::AppIo::default(),
                tutorials: Vec::new(),
            };
            host.load_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "0.1.0".into(), apps: vec![draw_app.clone()], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
                artifact_uri: "program://draw".into(),
            });
            let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
            let generation_before = host.instance(instance_id).expect("instance").generation;
            let event = host.hot_swap_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "0.2.0".into(), apps: vec![draw_app, note_app], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
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
                label: "Draw".into(),
                document: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: vec![],
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_documents: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework_core::ConfigSpec::empty(),
                command_grammar: semio_framework_core::CommandGrammar::empty(),
                io: semio_framework_core::AppIo::default(),
                tutorials: Vec::new(),
            };
            host.load_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "0.1.0".into(), apps: vec![draw_app], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
                artifact_uri: "program://draw".into(),
            });
            let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
            let generation_before = host.instance(instance_id).expect("instance").generation;
            let event = host.hot_swap_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "".into(), apps: vec![], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
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
            let contribution = Contribution::PlaybookBlockKind {
                app_id: "playbook-module-procedural".into(),
                block_kind: "buildingComponent".into(),
                label: "Building Component".into(),
                icon_id: "building".into(),
                default_value_json: "{}".into(),
                params_body_key: "params".into(),
                preview_body_key: "preview".into(),
            };
            host.load_plugin(LoadedProgram {
                plugin_id: "playbook-module-procedural".into(),
                manifest: PluginManifest {
                    plugin_id: "playbook-module-procedural".into(),
                    label: "Playbook Module Procedural".into(),
                    version: "0.1.0".into(),
                    apps: vec![],
                    capabilities: vec![],
                    contributions: vec![contribution.clone()],
                    examples: vec![],
                    commands: vec![],
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
                    contributions: vec![],
                    examples: vec![],
                    commands: vec![],
                },
                artifact_uri: "program://playbook-module-procedural".into(),
            });
            assert!(host.contributions().is_empty());
        }

        #[test]
        fn recovery_ui_renders_actions_for_quarantined_plugin() {
            let mut host = PluginHost::new();
            host.supervisor.insert("draw".into(), ProgramSupervisorState::Quarantined);
            let ui = host.recovery_ui("draw");
            match ui {
                UiNode::Stack(stack) => assert_eq!(stack.children.len(), 5, "title + message + restart/disable/showDiagnostics buttons"),
                other => panic!("expected recovery stack, got {other:?}"),
            }
        }

        /// 🧷️ Minimal `AppDefinition` for registry tests — every field but `io`/`document` is filler;
        /// `register_app_io` only reads `.id`/`.label`/`.io` (see `workflow::workflow_node_for_app`).
        fn test_app_definition(id: &str, label: &str, document_schema: &str, ports: Vec<semio_framework_core::MediaPortSpec>) -> AppDefinition {
            AppDefinition {
                id: id.into(),
                label: label.into(),
                document: vec!["semio".into(), id.into()],
                icon_id: None,
                controller_id: format!("{id}-play"),
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: id.into(),
                    label: label.into(),
                    body_key: id.into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "app-window".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: Vec::new(),
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_documents: std::collections::HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: semio_framework_core::ConfigSpec::empty(),
                command_grammar: semio_framework_core::CommandGrammar::empty(),
                io: semio_framework_core::AppIo::from_document(
                    document_schema,
                    MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    semio_framework_core::ArtifactPresentation { id: id.into(), name: label.into(), dimension: "2d".into(), component_kind: id.into() },
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
            let envelope = create_document_envelope(space::S_SPACE_SCHEMA, "space", space::empty_space_projection("Space", space::SpaceKind::Studio, space::SpaceVisibility::Private), None);
            DocumentStore::new(envelope)
        }

        fn test_workflow_store() -> OsWorkflowStore {
            OsWorkflowStore::new(create_backbone_document(workflow::S_WORKFLOW_SCHEMA, "workflow", "Workflow", workflow::empty_workflow_document()))
        }

        #[test]
        fn spawns_and_removes_app_instances() {
            seed_draw_plugin();
            let mut space_store = test_space_store();
            let mut store = test_workflow_store();
            store.add_workflow_node("draw", "draw", None, 40.0, 40.0, &mut space_store).expect("spawn");
            assert_eq!(store.projection().expect("projection").graph.nodes.len(), 1);
            assert!(space_store.projection().expect("projection").programs.contains(&"draw".to_string()), "spawning a node must install its plugin into the owning space");
            store.dispatch_text("undo").expect("undo");
            assert_eq!(store.projection().expect("projection").graph.nodes.len(), 0);
        }

        #[test]
        fn adds_and_patches_studio_parameters() {
            let mut store = test_workflow_store();
            let parameter_id = store.add_parameter(&workflow::WorkflowParameterType::Numeric, "Zoom").expect("add");
            store.patch_parameter(&parameter_id, &serde_json::json!({ "value": 12.0, "max": 10.0 })).expect("patch");
            match &store.projection().expect("projection").parameters[0] {
                workflow::WorkflowParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
                _ => panic!("expected numeric"),
            }
        }

        #[test]
        fn creates_and_lists_space_catalog_entries() {
            let port = Arc::new(MemoryBackbonePort::new());
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
            let mut store_b = OsWorkflowStore::new(store_a.document());

            let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://reconcile-race", "mem://reconcile-race");
            store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

            let document = store_a.projection().expect("projection");
            let node_a = document.graph.nodes.iter().find(|node| node.id == node_a_id).expect("node a");
            let node_b = document.graph.nodes.iter().find(|node| node.id == node_b_id).expect("node b");
            let source_node_id = node_a.id.clone();
            let source_port_id = node_a.outputs.first().expect("node a output port").id.clone();
            let target_node_id = node_b.id.clone();
            let target_port_id = node_b.inputs.first().expect("node b input port").id.clone();

            // 🏃️ Actor A deletes node B; actor B (unaware of the delete) concurrently wires a new edge
            // to a port on node B — the classic delete/wire race `reconcile` must clean up post-merge.
            store_a.dispatch_apply(vec![workflow::WorkflowOperation::RemoveNode { node_id: node_b_id.clone() }]).expect("remove node b");
            store_b
                .dispatch_apply(vec![workflow::WorkflowOperation::ConnectPorts {
                    edge: WorkflowEdge { id: "edge-race".into(), source_node_id: source_node_id.clone(), source_port_id, target_node_id: target_node_id.clone(), target_port_id, contract: placeholder_media_contract("draw") },
                }])
                .expect("wire edge to node b");
            store_a.tick().expect("pump a");
            store_b.tick().expect("pump b");

            let (converged_a, conflicts_a) = store_a.projection_with_conflicts().expect("projection with conflicts a");
            let (converged_b, conflicts_b) = store_b.projection_with_conflicts().expect("projection with conflicts b");
            assert_eq!(converged_a, converged_b, "both peers must converge on the same reconciled document");
            assert!(converged_a.graph.nodes.iter().all(|node| node.id != node_b_id), "node b must stay removed");
            assert!(converged_a.graph.edges.iter().all(|edge| edge.target_node_id != target_node_id), "the edge wired to the deleted node must be dropped, not dangling");
            assert!(conflicts_a.iter().any(|conflict| conflict.kind == "workflow/edge-orphaned"), "dropping the dangling edge must surface a conflict");
            assert_eq!(conflicts_a, conflicts_b, "both peers must report the same reconciliation conflicts");
        }

        // 🫀️ The old `presence_upserts_prunes_and_excludes_self` test exercised the deleted `presence:`
        // backbone-URI hack (`write_os_presence`/`read_os_presence_peers`). Presence now flows through
        // the semio_hub's `PresencePeer`/`HubServerFrame::Presence` frames and `framework/sync`'s
        // `DocumentEvent::Presence` — see `framework/product/os/semio_hub/rs/bin.rs` and
        // `framework/sync/rs/lib.rs` for that layer's own coverage.

        // #region 🔖️DslAndOpText
        /// 🧵️ A representative `WorkflowDocument` exercising every collection: two workflow nodes wired
        /// by one edge, one of each `WorkflowParameter` variant, and one parameter binding — so the DSL
        /// round trip actually covers the workflow encoding, not just an empty-document fixpoint.
        fn sample_workflow_document() -> workflow::WorkflowDocument {
            let node_a = workflow::WorkflowNode {
                id: "app-1".into(),
                plugin_id: "puzzle".into(),
                app_id: "puzzle2d".into(),
                label: "Puzzle Board \"3D\"".into(),
                yields: "puzzle.2d.fixture".into(),
                document_ref: "documents/app-1".into(),
                config_ref: "config/app-1".into(),
                x: 0.0,
                y: 0.0,
                width: 220.0,
                height: 92.0,
                inputs: Vec::new(),
                outputs: vec![workflow::WorkflowMediaPort {
                    id: "app-1:puzzle.out:out".into(),
                    spec: semio_framework_core::MediaPortSpec {
                        id: "puzzle.out".into(),
                        label: "Out".into(),
                        direction: semio_framework_core::MediaPortDirection::Out,
                        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                        kind_id: Some("puzzle.2d.fixture".into()),
                        required: false,
                        multiplicity: semio_framework_core::PortMultiplicity::One,
                    },
                }],
            };
            let node_b = workflow::WorkflowNode {
                id: "app-2".into(),
                plugin_id: "draw".into(),
                app_id: "draw".into(),
                label: "Draw Sink".into(),
                yields: "draw.document".into(),
                document_ref: "documents/app-2".into(),
                config_ref: "config/app-2".into(),
                x: 240.0,
                y: 0.0,
                width: 220.0,
                height: 92.0,
                inputs: vec![workflow::WorkflowMediaPort {
                    id: "app-2:draw.in:in".into(),
                    spec: semio_framework_core::MediaPortSpec {
                        id: "draw.in".into(),
                        label: "In".into(),
                        direction: semio_framework_core::MediaPortDirection::In,
                        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                        kind_id: Some("puzzle.2d.fixture".into()),
                        required: false,
                        multiplicity: semio_framework_core::PortMultiplicity::One,
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
            workflow::WorkflowDocument {
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

        /// 📜️ `📚️examples/🎬️demo.workflow-document` is the handcrafted DSL-text fixture for `WorkflowDocument`
        /// (the `s.workflow` artifact document, replacing the dissolved `📚️examples/🎬️demo.os`).
        #[test]
        fn dsl_round_trips_demo_workflow_example() {
            let text = include_str!("../../📚️examples/🎬️demo.workflow-document");
            let document = <workflow::WorkflowDocument as store::DocumentDsl>::parse_dsl(text).expect("🎬️demo.workflow-document must parse as WorkflowDocument");
            store::test_support::assert_dsl_round_trip(&document);
            store::test_support::assert_dsl_pack_equivalence(&document);
        }

        #[test]
        fn dsl_round_trips_default_workflow_document() {
            store::test_support::assert_dsl_round_trip(&workflow::empty_workflow_document());
            store::test_support::assert_dsl_pack_equivalence(&workflow::empty_workflow_document());
        }

        #[test]
        fn dsl_round_trips_workflow_document_with_graph_and_parameters() {
            store::test_support::assert_dsl_round_trip(&sample_workflow_document());
            store::test_support::assert_dsl_pack_equivalence(&sample_workflow_document());
        }

        #[test]
        fn op_text_round_trips_add_workflow_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::AddNode {
                node: workflow::WorkflowNode {
                    id: "node-1".into(),
                    plugin_id: "puzzle".into(),
                    app_id: "puzzle2d".into(),
                    label: "Puzzle Board".into(),
                    yields: "puzzle.2d.fixture".into(),
                    document_ref: "documents/node-1".into(),
                    config_ref: "config/node-1".into(),
                    x: 10.0,
                    y: -20.5,
                    width: 220.0,
                    height: 92.0,
                    inputs: Vec::new(),
                    outputs: Vec::new(),
                },
            });
        }

        #[test]
        fn op_text_round_trips_remove_workflow_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::RemoveNode { node_id: "app-1".into() });
        }

        #[test]
        fn op_text_round_trips_connect_media_ports() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::ConnectPorts {
                edge: workflow::WorkflowEdge {
                    id: "edge-1".into(),
                    source_node_id: "node-1".into(),
                    source_port_id: "app-1:out:out".into(),
                    target_node_id: "node-2".into(),
                    target_port_id: "app-2:in:in".into(),
                    contract: MediaContract {
                        kind_id: "puzzle.2d.fixture".into(),
                        media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                        wire: MediaWireFormat::Binary { format: OsMediaFormat::Stl },
                        conversion: Some((MediaForm::Brep, MediaForm::Mesh)),
                    },
                },
            });
        }

        #[test]
        fn op_text_round_trips_disconnect_media_edge() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::DisconnectEdge { edge_id: "edge-1".into() });
        }

        #[test]
        fn op_text_round_trips_move_media_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::MoveNode { node_id: "node-1".into(), x: 5.5, y: -6.25 });
        }

        #[test]
        fn op_text_round_trips_patch_workflow_node() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::PatchNode { node_id: "app-1".into(), label: "Renamed \"Board\"".into() });
        }

        #[test]
        fn op_text_round_trips_add_parameter() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::AddParameter { parameter: workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) } });
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::AddParameter { parameter: workflow::WorkflowParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] } });
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::AddParameter { parameter: workflow::WorkflowParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: false } });
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::AddParameter { parameter: workflow::WorkflowParameter::Text { id: "p4".into(), name: "Label".into(), value: "hi there".into() } });
        }

        #[test]
        fn op_text_round_trips_remove_parameter() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::RemoveParameter { parameter_id: "p1".into() });
        }

        #[test]
        fn op_text_round_trips_patch_parameter() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::PatchParameter { parameter_id: "p1".into(), parameter: workflow::WorkflowParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 20.0, min: None, max: None, step: None } });
        }

        #[test]
        fn op_text_round_trips_bind_parameter_field() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::BindParameterField { binding: workflow::WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "app-1".into(), field_path: "/zoom".into() } });
        }

        #[test]
        fn op_text_round_trips_unbind_parameter_field() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::UnbindParameterField { node_id: "app-1".into(), field_path: "/zoom".into() });
        }

        #[test]
        fn op_text_round_trips_sync_node_ports() {
            store::test_support::assert_op_line_round_trip(&workflow::WorkflowOperation::SyncNodePorts);
        }

        #[test]
        fn document_text_round_trips_store_with_applied_operation() {
            let envelope = create_document_envelope(workflow::S_WORKFLOW_SCHEMA, "workflow-text-test", workflow::empty_workflow_document(), None);
            let mut store = DocumentStore::new(envelope);
            store.dispatch(DocumentCommand::Apply { operations: vec![workflow::WorkflowOperation::SyncNodePorts], description: None }).expect("apply");
            store::test_support::assert_document_text_round_trip(&store);
            store::test_support::assert_document_pack_round_trip(&store);
        }
        // #endregion 🔖️DslAndOpText
    }
    // #endregion host
}

pub mod backbone {
    // #region backbone
    //! 🗄️ Trusted host-side backbone ports for local studio storage — reads/writes the raw persisted
    //! json directly, bypassing the duplex `Backbone` channel since there is no other process here.

    use crate::host::OsBackbonePort;
    #[cfg(not(target_arch = "wasm32"))]
    use std::sync::Arc;
    use store::MemoryBackbonePort;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::store_sync::{FolderSqliteStorage, FolderTextStorage};
    use vcs::VcsError;

    /// @emoji 🗂️ Conventional single-document id used inside a folder-backed studio backbone — a studio
    /// folder holds exactly one os document at its root (app documents get their own document ids once
    /// {@link OsDocumentRef} routes them through `framework/sync`'s multi-document `DocumentHost`).
    #[cfg(not(target_arch = "wasm32"))]
    const SPACE_FOLDER_DOCUMENT_ID: &str = "studio";

    enum SpacePortKind {
        /// @emoji 🗃️ A single document's pack blob addressed by an arbitrary `file://` path —
        /// `<folder>/<document_id>.<extension>.pack` (authoritative) + `.ops` + a DSL mirror, via
        /// `FolderTextStorage::write_pack`/`read_pack` and the typed `store::parse_document_pack`/
        /// `print_document_pack::<OsProjection, OsOperation>` (this crate is fully typed, no
        /// `store::DocumentCodec` indirection needed).
        #[cfg(not(target_arch = "wasm32"))]
        File { uri: String, storage: FolderTextStorage, document_id: String, extension: String },
        #[cfg(not(target_arch = "wasm32"))]
        Folder(String, FolderSqliteStorage),
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
            Ok(Self { kind: Some(SpacePortKind::Folder(uri, FolderSqliteStorage::new(std::path::PathBuf::from(folder_path)))), memory: MemoryBackbonePort::new() })
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
                                    let projection = <space::SpaceProjection as store::DocumentDsl>::parse_dsl(&text_files.dsl).map_err(|error| VcsError::Deserialize(error.message))?;
                                    let envelope = store::create_document_envelope::<space::SpaceProjection, space::SpaceOperation>(space::S_SPACE_SCHEMA, document_id, projection, None);
                                    let pack_files = store::print_document_pack(&envelope)?;
                                    (pack_files.pack, pack_files.spr)
                                }
                                None => return Err(VcsError::Backbone(format!("missing backbone file {uri}"))),
                            }
                        };
                        let inner = store::encode_document_pack_bytes(&pack, &spr);
                        return Ok(store::encode_document_pack_bytes(&[], &inner));
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    SpacePortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                        let (pack, spr) = storage.read(SPACE_FOLDER_DOCUMENT_ID)?.ok_or_else(|| VcsError::Backbone(format!("missing backbone file {uri}")))?;
                        let inner = store::encode_document_pack_bytes(&pack, &spr);
                        return Ok(store::encode_document_pack_bytes(&[], &inner));
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
                        let parsed: store::ParsedDocumentText<space::SpaceProjection, space::SpaceOperation> = store::parse_document_pack(&pack, &spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                        let dsl_mirror = store::DocumentDsl::print_dsl(&parsed.envelope.vcs.initial_projection);
                        let pack_files = store::DocumentPackFiles { pack, spr, ops: String::new() };
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
        let (_name, inner) = store::decode_document_pack_bytes(payload)?;
        store::decode_document_pack_bytes(&inner)
    }

    impl SpaceBackbonePort {
        /// @emoji 🌉️ `self.memory` is a plain `store::BackbonePort` (string-typed) fallback for any uri
        /// that isn't this port's own configured file/folder uri (e.g. the space catalog uri) — bridge
        /// bytes↔string via base64, same as the blanket `impl<T: store::BackbonePort> OsBackbonePort`.
        fn read_via_memory(&self, uri: &str) -> Result<Vec<u8>, VcsError> {
            use base64::Engine;
            let text = store::BackbonePort::read(&self.memory, uri)?;
            if text.is_empty() {
                return Ok(Vec::new());
            }
            base64::engine::general_purpose::STANDARD.decode(text).map_err(|error| VcsError::Deserialize(error.to_string()))
        }

        fn write_via_memory(&self, uri: &str, payload: &[u8]) -> Result<(), VcsError> {
            use base64::Engine;
            if payload.is_empty() {
                return store::BackbonePort::write(&self.memory, uri, "");
            }
            store::BackbonePort::write(&self.memory, uri, &base64::engine::general_purpose::STANDARD.encode(payload))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_folder_space_backbone(folder_path: &str) -> Result<Arc<dyn OsBackbonePort>, VcsError> {
        Ok(Arc::new(SpaceBackbonePort::folder(folder_path)?))
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_file_space_backbone(file_path: &str) -> Result<Arc<dyn OsBackbonePort>, VcsError> {
        Ok(Arc::new(SpaceBackbonePort::file(file_path)?))
    }
    // #endregion backbone
}

#[cfg(not(target_arch = "wasm32"))]
pub mod host_runtime {
    // #region host_runtime
    //! 🧵️ Canonical native document-open sequencing shared by every native host that links this crate
    //! (currently the wgpu shell). Native-only: it depends on `framework/sync`'s `DocumentHost`, whose
    //! actor is a native-thread (or wasm `spawn_local`) concern — WASI-P2 plugins never see it, and the
    //! browser React shell talks to its own TS twin (`framework/product/os/core/js/🟦️backbone-worker.ts`)
    //! through a different FFI boundary (the WIT program sandbox), not through this Rust module. Keeping
    //! this doc-comment as the single canonical description of the sequence — referenced from both
    //! `os-shell.tsx`'s `openDocument` and `framework/renderer/wgpu/rs/lib.rs` — is how the two stay in
    //! lockstep without a literal shared code path across the Rust/TS boundary.
    //!
    //! ## Canonical open/spawn/effect sequence (mirrored in TS by `os-shell.tsx`'s `openDocument`):
    //! 1. Build a `DocumentActorConfig{document_id, schema, bindings, watch_external, actor}` for the
    //!    document being opened — either the os/studio document itself, or one app instance's
    //!    {@link crate::instance::OsDocumentRef}.
    //! 2. `DocumentHost::open(config)` → `DocumentChannels{cmd_tx, channel_backbone}`.
    //! 3. Attach `channel_backbone` to the document's own store: `store.attach_backbone(Box::new(...))`.
    //!    For a native WASM plugin instance this ALSO means calling `framework/plugin/host`'s
    //!    `WasmPluginRuntime::register_host_backbone(uri, Box::new(channel_backbone))` so the sandboxed
    //!    plugin's `backbone-send`/`backbone-poll` host imports reach the same channel — this crate does
    //!    not link `framework/plugin/host` directly (no existing dependency edge), so the wgpu shell,
    //!    which links both, is the one that actually performs that registration call using the
    //!    {@link OpenedDocument} this module hands back.
    //! 4. `DocumentHost::subscribe(&document_id)` → `broadcast::Receiver<DocumentEvent>`; on each event:
    //!    - `RemoteOperations`/`SnapshotReplaced` are already pushed into the store's inbound queue by the actor
    //!      — the caller just needs to call `store.tick()` (step 5) to materialize them.
    //!    - `Presence{peers}` translates into `ViewState.presence_peers_json` via
    //!      {@link presence_peers_json} — the ONLY place presence now flows through; the old `presence:`
    //!      backbone-URI hack is gone entirely.
    //!    - `Status`/`Conflict` surface on the shell's sync-status badge / conflict card.
    //! 5. Every tick/frame: `store.tick()` drains the attached backbone's inbound queue into the store.
    //! 6. On `HostEffect::SpawnPluginInstance`/`OpenPluginInstance` from an action result: mint (if
    //!    needed) a fresh `OsDocumentRef` (see {@link crate::instance::create_os_document_id}), then repeat
    //!    steps 1-5 for that app's own document.
    //! 7. On close: send `DocumentActorMsg::Detach` (flushes pending operations) via `host.send(id, Detach)`, then
    //!    `DocumentHost::close(&id)`, then `store.detach_backbone()` /
    //!    `WasmPluginRuntime::deregister_host_backbone(uri)`.

    use crate::instance::OsDocumentRef;
    use crate::store_sync::{DocumentActorConfig, DocumentActorMsg, DocumentChannels, DocumentEvent, DocumentHost, PersistenceBinding};

    /// @emoji 📌️ The local persistence binding for a folder-backed document (one row per `document_id`
    /// in the folder's `.semio` sqlite store — see `FolderSqliteStorage`).
    pub fn folder_binding(folder_path: std::path::PathBuf) -> PersistenceBinding {
        PersistenceBinding::Folder { path: folder_path }
    }

    /// @emoji ☁️ The semio_hub persistence binding for a document.
    pub fn hub_binding(base_url: impl Into<String>, space_id: impl Into<String>, token: Option<String>) -> PersistenceBinding {
        PersistenceBinding::Hub { base_url: base_url.into(), space_id: space_id.into(), token }
    }

    /// @emoji 🔗️ Builds the `DocumentActorConfig` to open an app instance's own document, from its
    /// `OsDocumentRef` — step 1 of the canonical sequence.
    pub fn app_document_config(document: &OsDocumentRef, bindings: Vec<PersistenceBinding>, actor: &str) -> DocumentActorConfig {
        DocumentActorConfig { document_id: document.document_id.clone(), schema: document.schema.clone(), bindings, watch_external: true, actor: actor.to_string() }
    }

    /// @emoji 🧵️ Channels + a fresh event receiver for one opened document — steps 2 and 4 of the
    /// canonical sequence.
    pub struct OpenedDocument {
        pub channels: DocumentChannels,
        pub events: tokio::sync::broadcast::Receiver<DocumentEvent>,
    }

    /// @emoji 🚀️ Opens a document on `host` and subscribes to its events in one call (steps 1-2 & 4).
    pub fn open_document(host: &DocumentHost, document_id: &str, schema: &str, bindings: Vec<PersistenceBinding>, actor: &str) -> OpenedDocument {
        let channels = host.open(DocumentActorConfig { document_id: document_id.to_string(), schema: schema.to_string(), bindings, watch_external: true, actor: actor.to_string() });
        let events = host.subscribe(document_id);
        OpenedDocument { channels, events }
    }

    /// @emoji ✂️ Detaches and closes a document's actor (step 7's `DocumentHost` half).
    pub fn close_document(host: &DocumentHost, document_id: &str) {
        host.send(document_id, DocumentActorMsg::Detach);
        host.close(document_id);
    }

    /// @emoji 👥️ Translates a `DocumentEvent::Presence` into the `ViewState.presence_peers_json` contract
    /// plugins already read (`semio_framework_core::PresencePeer` → JSON array) — the new (only) source
    /// of presence data; the deleted `presence:` backbone hack used to be it.
    pub fn presence_peers_json(event: &DocumentEvent) -> Option<String> {
        match event {
            DocumentEvent::Presence { peers } => serde_json::to_string(peers).ok(),
            _ => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn opens_a_document_and_subscribes_to_its_events() {
            let host = DocumentHost::new();
            let opened = open_document(&host, "doc-1", "test.schema", vec![], "actor-1");
            drop(opened.events);
            close_document(&host, "doc-1");
        }

        #[test]
        fn app_document_config_carries_the_document_ref_through() {
            let document = OsDocumentRef { document_id: "doc-2".into(), schema: "draw.document".into() };
            let config = app_document_config(&document, vec![], "actor-1");
            assert_eq!(config.document_id, "doc-2");
            assert_eq!(config.schema, "draw.document");
        }

        #[test]
        fn presence_peers_json_only_matches_presence_events() {
            use semio_framework_core::PresencePeer;
            let peers = vec![PresencePeer { actor: "a".into(), label: Some("Ada".into()), selection_json: None, connected_at_ms: 0, user_id: None, role: None, cursor: None, viewport: None, drag_ghost_json: None }];
            let json = presence_peers_json(&DocumentEvent::Presence { peers: peers.clone() }).expect("json");
            assert!(json.contains("\"actor\":\"a\""));
            assert!(presence_peers_json(&DocumentEvent::Status(Default::default())).is_none());
        }
    }
    // #endregion host_runtime
}

pub mod instance {
    // #region instance
    //! 📦️ App instance schemas, parameters, and studio bindings.

    use semio_framework_core::{ConfigFieldShape, ConfigSpec};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::atomic::{AtomicU64, Ordering};
    use store::SpaceConflict;

    pub const OS_PARAMETER_PORT_PREFIX: &str = "param.";

    //#region 🔖️Schemas
    /// @emoji 🔗️ Handle to an app's own `framework/sync`-hosted vcs document — the os document never
    /// embeds app content, only this reference (mirrors `framework/sync`'s `DocumentActorConfig`).
    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsDocumentRef {
        pub document_id: String,
        pub schema: String,
    }

    /// @emoji 🆔️ Mints a fresh app document id — uuid-v7 (time-ordered), matching the id shape semio_hub already
    /// uses for its own entities (`framework/product/os/semio_hub/rs/bin.rs`'s `Uuid::now_v7()`).
    pub fn create_os_document_id() -> String {
        uuid::Uuid::now_v7().to_string()
    }

    // 🧷️ `OsAppInstance` is deleted — `workflow::WorkflowNode` (kernel crate) absorbs it entirely;
    // `WorkflowNode.id` IS the app-instance identity now (see the kernel crate's `🔖️InstanceIdentity`
    // region doc). `OsDocumentRef` stays (still used generically by `host_runtime`'s document-open
    // sequence), just no longer nested inside a per-instance record here.

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsInstanceState {
        pub id: u32,
        pub app_id: String,
        pub controller_id: String,
        pub document_json: String,
        pub view_state: semio_framework_core::ViewState,
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
        /// `validate_parameter_config_binding` (type-checks this against the field's
        /// `ConfigFieldShape`) and `build_configure_config` (overlays the bound parameter's value onto
        /// that config field for an `AppCommand::Configure` payload). Historically a JSON pointer into
        /// the node's live document (`apply_parameter_values_to_projection`'s still-live overlay,
        /// used only by the media-export path today) — that document-projection sense is now
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

    /// @emoji 🎛️ Deep-sets a JSON-pointer path on a plain object projection.
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

    /// @emoji 🎛️ Applies bound space parameter values onto an app projection via JSON pointers. 🩹️
    /// Pre-`ConfigSpec` document-projection overlay, kept for its one remaining live caller
    /// (`app_instance_document_patches_for_binding`, the media-export path's synthetic-document seed)
    /// — `field_path` here is still read as a JSON pointer into that bare document, distinct from the
    /// `ConfigFieldSpec.key` sense `validate_parameter_config_binding`/`build_configure_config` give it
    /// for driving a running app instance's config (see `OsParameterFieldBinding::field_path`'s doc).
    pub fn apply_parameter_values_to_projection(projection: Value, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], node_id: &str) -> Value {
        let node_bindings: Vec<_> = bindings.iter().filter(|binding| binding.node_id == node_id).collect();
        if node_bindings.is_empty() {
            return projection;
        }
        let mut clone = projection;
        for binding in node_bindings {
            let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
                continue;
            };
            set_json_pointer_value(&mut clone, &binding.field_path, os_parameter_value(parameter));
        }
        clone
    }

    /// @emoji ✅️ Type-checks one binding's `field_path` against the target app's declared
    /// `ConfigSpec`: the field must exist, and `parameter_type` must be able to drive its
    /// `ConfigFieldShape` (`Numeric`↔`Number`, `Categorical`↔`Select`, `Toggle`↔`Toggle`,
    /// `Text`↔`Text`; anything else — including an unknown `field_path` — is a conflict). Raised the
    /// same way `host::reconcile_os_workflow` raises `"workflow/edge-type-mismatch"`, as
    /// `"workflow/parameter-binding-invalid"`, so callers fold this into that same conflict-collecting
    /// pass instead of a bespoke error type.
    pub fn validate_parameter_config_binding(binding: &OsParameterFieldBinding, parameter_type: &OsParameterType, config_spec: &ConfigSpec) -> Result<(), SpaceConflict> {
        let uri = format!("{}#{}", binding.node_id, binding.field_path);
        let Some(field) = config_spec.fields.iter().find(|field| field.key == binding.field_path) else {
            return Err(SpaceConflict { kind: "workflow/parameter-binding-invalid".into(), uri, message: format!("binding targets config field '{}', which the app's ConfigSpec does not declare", binding.field_path) });
        };
        let compatible = matches!(
            (parameter_type, &field.shape),
            (OsParameterType::Numeric, ConfigFieldShape::Number { .. }) | (OsParameterType::Categorical, ConfigFieldShape::Select { .. }) | (OsParameterType::Toggle, ConfigFieldShape::Toggle) | (OsParameterType::Text, ConfigFieldShape::Text)
        );
        if compatible {
            Ok(())
        } else {
            Err(SpaceConflict { kind: "workflow/parameter-binding-invalid".into(), uri, message: format!("parameter type {parameter_type:?} cannot drive config field '{}' ({:?})", binding.field_path, field.shape) })
        }
    }

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
    /// {@link OsDocumentRef} creation time (see `host_runtime`), not on every materialize/read.
    pub fn os_fixture_json(slug: &str) -> Option<String> {
        os_fixture_json_registry().lock().ok().and_then(|registry| registry.get(slug).cloned())
    }

    /// @emoji 🎚️ Default config value seeded from `config_spec.fields[].default` — what a freshly
    /// spawned instance's config resolves to before any explicit `instance.config`/binding overlay.
    fn config_spec_default_value(config_spec: &ConfigSpec) -> Value {
        let mut defaults = serde_json::Map::new();
        for field in &config_spec.fields {
            if let Some(default) = &field.default {
                let json_default = semio_framework_core::from_dsl_value::<Value>(default.clone()).unwrap_or(Value::Null);
                defaults.insert(field.key.clone(), json_default);
            }
        }
        Value::Object(defaults)
    }

    /// @emoji 🧩️ Builds the dynamic config value for an `AppCommand::Configure` payload: starts from the
    /// app's own `ConfigSpec` defaults, then overlays every parameter bound to one of `config_spec`'s
    /// fields with that parameter's current value — the config-driving counterpart to
    /// `apply_parameter_values_to_projection`'s document-JSON-pointer overlay (see
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
        let mut config = dsl::to_dsl_value(&config_spec_default_value(config_spec)).unwrap_or(dsl::DslValue::Object(vec![]));
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
            let value = dsl::to_dsl_value(&os_parameter_value(parameter)).unwrap_or(dsl::DslValue::Null);
            if let Some((_, slot)) = entries.iter_mut().find(|(key, _)| key == &field.key) {
                *slot = value;
            } else {
                entries.push((field.key.clone(), value));
            }
        }
        config
    }

    /// @emoji 🧩️ Overlays bound parameter values onto an app instance's current document projection.
    /// Content itself lives in the app's own `framework/sync`-hosted document (referenced by
    /// {@link OsDocumentRef}, read host-side and passed in as `current_document_json`) — this function
    /// no longer resolves embedded/upstream source documents; that concept was deleted with
    /// `OsSourceDocument`. Cross-instance ("upstream") dataflow through workflow edges is deferred
    /// (see `host_runtime` doc-comment) to a follow-up that reads the upstream app's live document.
    pub fn materialize_os_app_instance_document_json(current_document_json: &str, node_id: &str, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter]) -> String {
        let projection: Value = serde_json::from_str(current_document_json).unwrap_or_else(|_| json!({}));
        let with_params = apply_parameter_values_to_projection(projection, bindings, parameters, node_id);
        serde_json::to_string(&with_params).unwrap_or_else(|_| "{}".into())
    }

    /// @emoji 🔀️ Host-side hook for the common case: when a bound parameter's value changes, computes the
    /// patched document JSON for every app instance with a field bound to it, keyed by document id — the
    /// host dispatches each as a snapshot replace into that app's own document store (e.g. via the program
    /// WIT boundary's `load-app-document`, or `framework/sync`'s document actor once the app is wired onto
    /// `DocumentHost`). This covers the "common/simple case" per the JSON-pointer overlay convention
    /// {@link apply_parameter_values_to_projection} already established — a true typed operation into the bound
    /// app's own `Operation` vocabulary requires that app's real (non-opaque) Operation type and is left to each app's
    /// own `DocumentApp` migration (WS-F); until then this snapshot-replace path is the host's only lever.
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
                let current_json = current_document_json(&node.document_ref)?;
                let patched = materialize_os_app_instance_document_json(&current_json, &node.id, bindings, parameters);
                Some((node.document_ref.clone(), patched))
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
            let projection = serde_json::json!({ "brushSize": 8 });
            let overridden = apply_parameter_values_to_projection(
                projection,
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
                    semio_framework_core::ConfigFieldSpec {
                        key: "zoom".into(),
                        label: "Zoom".into(),
                        shape: ConfigFieldShape::Number { min: None, max: None, step: None },
                        default: Some(dsl::to_dsl_value(&serde_json::json!(1.0)).expect("dsl value")),
                    },
                    semio_framework_core::ConfigFieldSpec {
                        key: "mode".into(),
                        label: "Mode".into(),
                        shape: ConfigFieldShape::Select { options: vec!["A".into(), "B".into()] },
                        default: Some(dsl::to_dsl_value(&serde_json::json!("A")).expect("dsl value")),
                    },
                    semio_framework_core::ConfigFieldSpec { key: "flag".into(), label: "Flag".into(), shape: ConfigFieldShape::Toggle, default: None },
                    semio_framework_core::ConfigFieldSpec { key: "label".into(), label: "Label".into(), shape: ConfigFieldShape::Text, default: None },
                ],
            }
        }

        #[test]
        fn validates_matching_parameter_config_bindings() {
            let config_spec = sample_config_spec();
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "zoom".into() }, &OsParameterType::Numeric, &config_spec).is_ok());
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p2".into(), node_id: "i1".into(), field_path: "mode".into() }, &OsParameterType::Categorical, &config_spec).is_ok());
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p3".into(), node_id: "i1".into(), field_path: "flag".into() }, &OsParameterType::Toggle, &config_spec).is_ok());
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p4".into(), node_id: "i1".into(), field_path: "label".into() }, &OsParameterType::Text, &config_spec).is_ok());
        }

        #[test]
        fn rejects_mismatched_parameter_config_bindings() {
            let config_spec = sample_config_spec();
            let mismatch =
                validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "zoom".into() }, &OsParameterType::Toggle, &config_spec).expect_err("toggle cannot drive a Number field");
            assert_eq!(mismatch.kind, "workflow/parameter-binding-invalid");
            let mismatch =
                validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p2".into(), node_id: "i1".into(), field_path: "mode".into() }, &OsParameterType::Text, &config_spec).expect_err("text cannot drive a Select field");
            assert_eq!(mismatch.kind, "workflow/parameter-binding-invalid");
        }

        #[test]
        fn rejects_parameter_config_binding_to_unknown_field() {
            let config_spec = sample_config_spec();
            let error = validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "nonexistent".into() }, &OsParameterType::Numeric, &config_spec)
                .expect_err("field does not exist on the ConfigSpec");
            assert_eq!(error.kind, "workflow/parameter-binding-invalid");
        }

        #[test]
        fn build_configure_config_starts_from_config_spec_defaults() {
            let config_spec = sample_config_spec();
            let config = build_configure_config("i1", &[], &[], &config_spec);
            let config: Value = dsl::from_dsl_value(config).expect("config json");
            assert_eq!(config["zoom"], 1.0);
            assert_eq!(config["mode"], "A");
        }

        #[test]
        fn build_configure_config_overlays_bound_parameter_values() {
            let config_spec = sample_config_spec();
            let parameters = vec![OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 42.0, min: None, max: None, step: None }];
            let bindings = vec![OsParameterFieldBinding { parameter_id: "p1".into(), node_id: "i1".into(), field_path: "zoom".into() }];
            let config = build_configure_config("i1", &parameters, &bindings, &config_spec);
            let config: Value = dsl::from_dsl_value(config).expect("config json");
            assert_eq!(config["zoom"], 42.0);
            assert_eq!(config["mode"], "A");
        }
    }
    //#endregion 🧪️Tests
    // #endregion instance
}

pub mod media_export_raster {
    // #region media_export_raster
    //! 🖼️ SVG rasterization, DWG flattening, and media-export registration helpers.

    use crate::workflow::{register_os_media_export_handler, register_os_media_import_handler, OsMediaExportResult, OsMediaFormat};
    use base64::Engine;
    use png::{BitDepth, ColorType, Encoder};
    use semio_framework_core::{DwgColor, DwgDrawing, DwgEntity, DwgGeometry};
    use serde_json::Value;
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    /// @emoji 🖼️ Rasterizes SVG markup to a base64-encoded PNG payload.
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
    pub fn svg_to_dwg_bytes(svg: &str) -> Result<Vec<u8>, String> {
        let tree = usvg::Tree::from_str(svg, &usvg::Options::default()).map_err(|error| error.to_string())?;
        let mut drawing = DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        let height = tree.size().height() as f64;
        collect_svg_children(tree.root().children(), &mut drawing, layer, height);
        semio_framework_core::dwg_to_bytes(&drawing)
    }

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

    fn transformed_svg_point(transform: usvg::Transform, point: usvg::tiny_skia_path::Point, height: f64) -> [f64; 2] {
        let mut p = point;
        transform.map_point(&mut p);
        [p.x as f64, height - p.y as f64]
    }

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
        register_os_media_export_handler(artifact_kind, OsMediaFormat::Svg, move |doc| {
            let (svg, _width, _height) = document_to_svg(doc)?;
            Ok(OsMediaExportResult { data: svg, mime_type: OsMediaFormat::Svg.mime_type().into(), file_name: format!("{file_stem}.svg"), encoding: None })
        });
        register_os_media_export_handler(artifact_kind, OsMediaFormat::Png, move |doc| {
            let (svg, width, height) = document_to_svg(doc)?;
            let data = rasterize_svg_to_png_base64(&svg, width, height)?;
            Ok(OsMediaExportResult { data, mime_type: OsMediaFormat::Png.mime_type().into(), file_name: format!("{file_stem}.png"), encoding: Some("base64".into()) })
        });
        register_os_media_export_handler(artifact_kind, OsMediaFormat::Dwg, move |doc| {
            let (svg, _width, _height) = document_to_svg(doc)?;
            let bytes = svg_to_dwg_bytes(&svg)?;
            Ok(OsMediaExportResult { data: base64::engine::general_purpose::STANDARD.encode(bytes), mime_type: OsMediaFormat::Dwg.mime_type().into(), file_name: format!("{file_stem}.dwg"), encoding: Some("base64".into()) })
        });
    }

    /// @emoji 📥️ Registers a DWG import handler for one 2D resource kind, rasterizing DWG geometry into flat SVG first.
    pub fn register_dwg_import_handler(artifact_kind: &'static str, from_dwg: fn(&DwgDrawing) -> Result<Value, String>) {
        register_os_media_import_handler(artifact_kind, OsMediaFormat::Dwg, move |bytes| {
            let drawing = semio_framework_core::dwg_from_bytes(bytes)?;
            from_dwg(&drawing)
        });
    }

    /// @emoji 🧵️ Registers one `MeshExporter` format (Obj/Glb/Stl/…) for a mesh resource kind; call once per format — `mesh_from_document` bridges the OS workflow's per-document export pipeline down to the format-agnostic `MeshData` the exporter instance actually encodes. DWG stays on `register_mesh_dwg_import_handler`'s sibling below; it is not part of the `MeshExporter` mechanism.
    pub fn register_mesh_exporter(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>, exporter: Box<dyn semio_framework_plugin::MeshExporter>) {
        let format = exporter.format();
        let ext = format.as_str();
        let mime_type = format.mime_type().to_string();
        let binary = format.is_binary();
        register_os_media_export_handler(artifact_kind, format, move |doc| {
            let mesh = mesh_from_document(doc)?;
            let bytes = exporter.export(&mesh)?;
            let data = if binary { base64::engine::general_purpose::STANDARD.encode(&bytes) } else { String::from_utf8(bytes).map_err(|error| error.to_string())? };
            Ok(OsMediaExportResult { data, mime_type: mime_type.clone(), file_name: format!("{file_stem}.{ext}"), encoding: if binary { Some("base64".into()) } else { None } })
        });
    }

    /// @emoji 🧵️ Registers one `MeshImporter` format (Obj/Glb/Stl/…) for a mesh resource kind; `document_from_mesh` bridges the decoded `MeshData` back into the app's own document shape.
    pub fn register_mesh_importer(artifact_kind: &'static str, document_from_mesh: fn(&semio_framework_plugin::MeshData) -> Result<Value, String>, importer: Box<dyn semio_framework_plugin::MeshImporter>) {
        let format = importer.format();
        register_os_media_import_handler(artifact_kind, format, move |bytes| {
            let mesh = importer.import(bytes)?;
            document_from_mesh(&mesh)
        });
    }

    /// @emoji 📥️ Registers a DWG import handler for one mesh resource kind.
    pub fn register_mesh_dwg_import_handler(artifact_kind: &'static str, document_from_mesh: fn(&semio_framework_plugin::MeshData) -> Result<Value, String>) {
        register_os_media_import_handler(artifact_kind, OsMediaFormat::Dwg, move |bytes| {
            let drawing = semio_framework_core::dwg_from_bytes(bytes)?;
            let mesh = semio_framework_core::dwg_drawing_to_mesh(&drawing);
            document_from_mesh(&mesh)
        });
    }

    /// @emoji 💾️ Registers a DWG export handler for one mesh resource kind; DWG is not part of the `MeshExporter` mechanism (it flattens a mesh into a DWG drawing, not a mesh codec), so it stays a dedicated registrar alongside `register_mesh_exporter`.
    pub fn register_mesh_dwg_export_handler(artifact_kind: &'static str, file_stem: &'static str, mesh_from_document: fn(&Value) -> Result<semio_framework_plugin::MeshData, String>) {
        register_os_media_export_handler(artifact_kind, OsMediaFormat::Dwg, move |doc| {
            let mesh = mesh_from_document(doc)?;
            let drawing = semio_framework_core::mesh_to_dwg_drawing(&mesh);
            let bytes = semio_framework_core::dwg_to_bytes(&drawing)?;
            Ok(OsMediaExportResult { data: base64::engine::general_purpose::STANDARD.encode(bytes), mime_type: OsMediaFormat::Dwg.mime_type().into(), file_name: format!("{file_stem}.dwg"), encoding: Some("base64".into()) })
        });
    }

    //#region SolidMediaExport
    type SolidExporterRegistry = HashMap<String, Box<dyn semio_s_3d::brep::kernel::SolidExporter>>;

    fn solid_exporters() -> &'static Mutex<SolidExporterRegistry> {
        static HANDLERS: OnceLock<Mutex<SolidExporterRegistry>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    type SolidImporterRegistry = HashMap<String, Box<dyn semio_s_3d::brep::kernel::SolidImporter>>;

    fn solid_importers() -> &'static Mutex<SolidImporterRegistry> {
        static HANDLERS: OnceLock<Mutex<SolidImporterRegistry>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn solid_registry_key(artifact_kind: &str, format: &OsMediaFormat) -> String {
        format!("{}:{}", artifact_kind, format.as_str())
    }

    /// @emoji 🧊️ Registers a B-Rep solid exporter (STEP/STL/OBJ/GLB, operating on `GeometryHandle` via `semio_s_3d::brep::kernel::BrepkitKernel` rather than a tessellated `MeshData`) for a resource kind; call once per format.
    pub fn register_solid_exporter(artifact_kind: &str, exporter: Box<dyn semio_s_3d::brep::kernel::SolidExporter>) {
        let key = solid_registry_key(artifact_kind, &exporter.format());
        solid_exporters().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key, exporter);
    }

    /// @emoji 🧊️ Registers a B-Rep solid importer for a resource kind; see `register_solid_exporter`.
    pub fn register_solid_importer(artifact_kind: &str, importer: Box<dyn semio_s_3d::brep::kernel::SolidImporter>) {
        let key = solid_registry_key(artifact_kind, &importer.format());
        solid_importers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key, importer);
    }

    /// @emoji 🧊️ Looks up a previously registered solid exporter for a resource kind + format.
    pub fn solid_exporter_for(artifact_kind: &str, format: &OsMediaFormat) -> bool {
        solid_exporters().lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&solid_registry_key(artifact_kind, format))
    }

    /// @emoji 🧊️ Exports `shapes` from `kernel` through the solid exporter registered for `artifact_kind` + `format`.
    pub fn export_registered_solid(artifact_kind: &str, format: &OsMediaFormat, kernel: &semio_s_3d::brep::kernel::BrepkitKernel, shapes: &[semio_s_3d::brep::engine::GeometryHandle], deflection: f64) -> Result<Vec<u8>, String> {
        let key = solid_registry_key(artifact_kind, format);
        let handlers = solid_exporters().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let exporter = handlers.get(&key).ok_or_else(|| format!("no solid export handler for {key}"))?;
        exporter.export(kernel, shapes, deflection).map_err(|error| error.to_string())
    }

    /// @emoji 🧊️ Imports bytes into `kernel` through the solid importer registered for `artifact_kind` + `format`.
    pub fn import_registered_solid(artifact_kind: &str, format: &OsMediaFormat, kernel: &mut semio_s_3d::brep::kernel::BrepkitKernel, data: &[u8], tolerance: f64) -> Result<Vec<semio_s_3d::brep::engine::GeometryHandle>, String> {
        let key = solid_registry_key(artifact_kind, format);
        let handlers = solid_importers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let importer = handlers.get(&key).ok_or_else(|| format!("no solid import handler for {key}"))?;
        importer.import(kernel, data, tolerance).map_err(|error| error.to_string())
    }
    //#endregion SolidMediaExport
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

pub mod workflow {
    // #region workflow
    //! 🎬️ Workflow, VFS projection types, and media export registry.

    // 🧬️ Kernel re-exports — the persisted graph model itself (`Workflow`/`WorkflowNode`/`WorkflowEdge`/
    // `WorkflowMediaPort`/`WorkflowPosition`/`MediaContract`/`WorkflowDelivery`/`WorkflowFixture`/
    // `plan_workflow`/`workflow_node_for_app`/`placeholder_media_contract`/`empty_workflow`) lives in
    // the `semio-framework-os-kernel-workflow` crate (dependency name `workflow`) — see its
    // `🔖️InstanceIdentity` doc. Re-exported here so every existing `crate::workflow::X` call site in
    // this file keeps working unchanged. `workflow::validate_workflow` (dangling-edge + cycle checks
    // only) is re-exported under a different name because this module's own `validate_workflow` (below)
    // wraps it with the contract-renegotiation check that still needs the artifact registry, which only
    // exists at this layer.
    // 🧬️ `WorkflowDocument`/`WorkflowOperation`/`WorkflowParameter*`/`WorkflowInput*`/`WorkflowOutputBinding`
    // absorb os-core's dissolved `OsProjection`/`OsOperation`/`instance::OsParameter*` (see `## The
    // inversion` in the plan) — re-exported here too so every `crate::workflow::X` call site (and every
    // downstream crate importing via `semio_framework_os::workflow::X`/`semio_framework_os::X`) keeps a
    // single source of truth for the workflow document vocabulary.
    pub use crate::workflow_kernel::{
        apply_workflow_operation, create_default_workflow_parameter, empty_workflow, empty_workflow_document, patch_workflow_parameter, placeholder_media_contract, plan_workflow, sync_workflow_parameter_ports,
        validate_workflow as kernel_validate_workflow, validate_workflow_document, validate_workflow_parameter_config_binding, workflow_node_for_app, workflow_parameter_id, workflow_parameter_id_from_port_id, workflow_parameter_name,
        workflow_parameter_types_compatible, workflow_parameter_value, MediaContract, Workflow, WorkflowDelivery, WorkflowDocument, WorkflowEdge, WorkflowFixture, WorkflowInput, WorkflowInputBinding, WorkflowMediaPort, WorkflowNode, WorkflowOperation,
        WorkflowOutputBinding, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterPatch, WorkflowParameterType, WorkflowPosition, WorkflowValidation, S_WORKFLOW_SCHEMA, WORKFLOW_SCHEMA,
    };

    use crate::instance::create_os_id;
    use crate::registry::{os_app_registration, os_artifact_descriptor, OsArtifactDescriptor};
    use semio_framework_core::{media_types_compatible, MediaCompat, MediaWireFormat};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::{Mutex, OnceLock};

    pub const OS_SPACE_SCHEMA: &str = "s.space";
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
        let conversion = match media_types_compatible(&source_port.spec.media_type, &target_port.spec.media_type) {
            MediaCompat::Direct => None,
            MediaCompat::Convert { from, to } => Some((from, to)),
            MediaCompat::Reject => {
                return Err(format!(
                    "cannot connect `{}` ({:?}/{:?}) to `{}` ({:?}/{:?}): incompatible media types",
                    source_port.spec.id, source_port.spec.media_type.class, source_port.spec.media_type.form, target_port.spec.id, target_port.spec.media_type.class, target_port.spec.media_type.form
                ));
            }
        };
        let wire = negotiate_wire_format(&source_descriptor, &target_descriptor).ok_or_else(|| format!("cannot connect `{}` to `{}`: no shared wire format", source_port.spec.id, target_port.spec.id))?;
        let kind_id = target_port.spec.kind_id.clone().unwrap_or_else(|| target_descriptor.kind.clone());
        Ok(MediaContract { kind_id, media_type: target_port.spec.media_type, wire, conversion })
    }

    /// 🔀️ Prefers a shared `Document{schema}` wire (structured payloads round-trip losslessly) over a shared
    /// `Binary{format}` wire (the first common `OsMediaFormat` between the two descriptors' export/import
    /// lists) — see `MediaWireFormat`.
    fn negotiate_wire_format(source: &OsArtifactDescriptor, target: &OsArtifactDescriptor) -> Option<MediaWireFormat> {
        if !source.schema.is_empty() && source.schema == target.schema {
            return Some(MediaWireFormat::Document { schema: source.schema.clone() });
        }
        source.export_formats.iter().find(|format| target.import_formats.contains(format)).map(|format| MediaWireFormat::Binary { format: *format })
    }

    /// @emoji ✅️ Validates workflow connectivity, cycle freedom (via `workflow::validate_workflow`,
    /// re-exported as `kernel_validate_workflow`), and edge-contract consistency (this layer's own
    /// pass, since it needs the artifact registry the kernel crate doesn't have).
    pub fn validate_workflow(graph: &Workflow) -> WorkflowValidation {
        let mut validation = kernel_validate_workflow(graph);

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
    pub fn apply_flow_fixture_to_os_workflow(graph: &Workflow, fixture_json: &str) -> Vec<WorkflowOperation> {
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
                    operations.push(WorkflowOperation::MoveNode { node_id: node.id.clone(), x, y });
                }
            }
        }
        let mut removed_node_ids = HashSet::new();
        if let Some(widgets) = fixture.get("widgets").and_then(Value::as_array) {
            let widget_ids: HashSet<&str> = widgets.iter().filter_map(|widget| widget.get("id").and_then(Value::as_str)).collect();
            for node in &graph.nodes {
                if !widget_ids.contains(node.id.as_str()) {
                    removed_node_ids.insert(node.id.clone());
                    operations.push(WorkflowOperation::RemoveNode { node_id: node.id.clone() });
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
            operations.push(WorkflowOperation::ConnectPorts { edge: WorkflowEdge { id, source_node_id, source_port_id, target_node_id, target_port_id, contract } });
        }
        if fixture.get("synapses").and_then(Value::as_array).is_some() {
            for edge in &graph.edges {
                if fixture_endpoints.contains(&edge_endpoints(edge)) {
                    continue;
                }
                if removed_node_ids.contains(&edge.source_node_id) || removed_node_ids.contains(&edge.target_node_id) {
                    continue;
                }
                operations.push(WorkflowOperation::DisconnectEdge { edge_id: edge.id.clone() });
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowChannelSpec {
        pub name: String,
        pub code: String,
        pub abbreviation: String,
        pub full_name: String,
        pub operators: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
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
    /// `workflow::workflow_parameter_id_from_port_id`) — `WorkflowDocument.parameters` absorbed the
    /// dissolved `OsProjection.parameters` in W3, see `## The inversion`.
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
                            let label = parameter_id.as_ref().and_then(|id| parameter_by_id.get(id)).map(|parameter| workflow_parameter_name(parameter)).or_else(|| workflow::media_port_spec_id(&port.id)).unwrap_or_else(|| port.id.clone());
                            os_workflow_channel_spec(port, &label)
                        })
                        .collect(),
                    outputs: node
                        .outputs
                        .iter()
                        .map(|port| {
                            let label = workflow::media_port_spec_id(&port.id).unwrap_or_else(|| port.id.clone());
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
    /// 🗂️ Defined in `semio_framework_core` (below this crate in the dependency graph) so `MeshExporter`/`MeshImporter` there can name it too; re-exported here verbatim.
    pub use semio_framework_core::OsMediaFormat;

    //#region 🔖️MediaCapability
    pub use crate::registry::os_resource_media_capability;
    /// 🗂️ Defined in `semio_framework_core` alongside `OsMediaFormat`/`ArtifactKindSpec`; re-exported here
    /// verbatim. `os_resource_media_capability` is a registry lookup (see `crate::registry`) driven by each
    /// app's declared `ArtifactKindSpec.media_capability` instead of a hardcoded per-app match.
    pub use semio_framework_core::OsMediaCapability;
    //#endregion 🔖️MediaCapability

    #[derive(Clone, Debug, PartialEq)]
    pub struct OsMediaExportResult {
        pub data: String,
        pub mime_type: String,
        pub file_name: String,
        pub encoding: Option<String>,
    }

    type OsMediaExportHandler = Box<dyn Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync>;

    fn export_handlers() -> &'static Mutex<HashMap<String, OsMediaExportHandler>> {
        static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaExportHandler>>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn os_media_export_key(artifact_kind: &str, format: &OsMediaFormat) -> String {
        format!("{}:{}", artifact_kind, format.as_str())
    }

    /// @emoji 💾️ Registers an export handler for a media resource kind and format.
    pub fn register_os_media_export_handler(artifact_kind: &str, format: OsMediaFormat, handler: impl Fn(&Value) -> Result<OsMediaExportResult, String> + Send + Sync + 'static) {
        export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(os_media_export_key(artifact_kind, &format), Box::new(handler));
    }

    /// 📐️ Required export formats per dimension; 3D/5D mesh-only apps stop at OBJ/GLB/STL/DWG, B-Rep apps (`os_resource_media_capability`) additionally require STEP.
    pub fn required_os_media_export_formats(dimension: &str, capability: OsMediaCapability) -> Vec<OsMediaFormat> {
        match dimension {
            "2d" => vec![OsMediaFormat::Svg, OsMediaFormat::Png, OsMediaFormat::Dwg],
            "3d" | "5d" => match capability {
                OsMediaCapability::Brep => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Step, OsMediaFormat::Dwg],
                OsMediaCapability::MeshOnly => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Dwg],
            },
            _ => Vec::new(),
        }
    }

    /// @emoji ✅️ Ensures every known resource kind has required export handlers.
    pub fn assert_os_media_export_coverage() -> Result<(), String> {
        let handlers = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut missing = Vec::new();
        for descriptor in crate::registry::list_os_artifact_descriptors() {
            let capability = os_resource_media_capability(&descriptor.kind);
            for format in required_os_media_export_formats(&descriptor.dimension, capability) {
                if !handlers.contains_key(&os_media_export_key(&descriptor.kind, &format)) {
                    missing.push(format!("{}:{}", descriptor.kind, format.as_str()));
                }
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!("missing os media export handlers: {}", missing.join(", ")))
        }
    }

    pub fn export_os_app_instance_media(node: &WorkflowNode, source_document: &Value, format: OsMediaFormat) -> Result<OsMediaExportResult, String> {
        let handlers = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers.get(&os_media_export_key(&node.yields, &format)).ok_or_else(|| format!("no export handler for {}:{}", node.yields, format.as_str()))?;
        handler(source_document)
    }

    pub fn os_media_export_extension_for_format(format: &OsMediaFormat) -> &'static str {
        format.as_str()
    }

    type OsMediaImportHandler = Box<dyn Fn(&[u8]) -> Result<Value, String> + Send + Sync>;

    fn import_handlers() -> &'static Mutex<HashMap<String, OsMediaImportHandler>> {
        static HANDLERS: OnceLock<Mutex<HashMap<String, OsMediaImportHandler>>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    /// @emoji 📥️ Registers an import handler for a media resource kind and format; the handler turns raw bytes into a complete source document.
    pub fn register_os_media_import_handler(artifact_kind: &str, format: OsMediaFormat, handler: impl Fn(&[u8]) -> Result<Value, String> + Send + Sync + 'static) {
        import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(os_media_export_key(artifact_kind, &format), Box::new(handler));
    }

    /// @emoji 📥️ Formats every resource kind of the given dimension must accept for import; 2D stays DWG-only, 3D/5D mirrors `required_os_media_export_formats`.
    pub fn required_os_media_import_formats(dimension: &str, capability: OsMediaCapability) -> Vec<OsMediaFormat> {
        match dimension {
            "2d" => vec![OsMediaFormat::Dwg],
            "3d" | "5d" => match capability {
                OsMediaCapability::Brep => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Step, OsMediaFormat::Dwg],
                OsMediaCapability::MeshOnly => vec![OsMediaFormat::Obj, OsMediaFormat::Glb, OsMediaFormat::Stl, OsMediaFormat::Dwg],
            },
            _ => Vec::new(),
        }
    }

    /// @emoji ✅️ Ensures every known resource kind has required import handlers.
    pub fn assert_os_media_import_coverage() -> Result<(), String> {
        let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut missing = Vec::new();
        for descriptor in crate::registry::list_os_artifact_descriptors() {
            let capability = os_resource_media_capability(&descriptor.kind);
            for format in required_os_media_import_formats(&descriptor.dimension, capability) {
                if !handlers.contains_key(&os_media_export_key(&descriptor.kind, &format)) {
                    missing.push(format!("{}:{}", descriptor.kind, format.as_str()));
                }
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(format!("missing os media import handlers: {}", missing.join(", ")))
        }
    }

    /// @emoji 📥️ Imports raw bytes for an app instance's resource kind, returning the new inline source document.
    pub fn import_os_app_instance_media(node: &WorkflowNode, data: &[u8], format: OsMediaFormat) -> Result<Value, String> {
        let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers.get(&os_media_export_key(&node.yields, &format)).ok_or_else(|| format!("no import handler for {}:{}", node.yields, format.as_str()))?;
        handler(data)
    }
    //#endregion 🔖️MediaExport

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn validates_empty_workflow() {
            assert!(validate_workflow(&empty_workflow()).ok);
        }

        #[test]
        fn export_coverage_accepts_registered_handlers() {
            for descriptor in crate::registry::list_os_artifact_descriptors() {
                for format in required_os_media_export_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind)) {
                    register_os_media_export_handler(&descriptor.kind, format, |_| Ok(OsMediaExportResult { data: "export".into(), mime_type: "application/octet-stream".into(), file_name: "export.bin".into(), encoding: None }));
                }
            }
            assert!(assert_os_media_export_coverage().is_ok());
        }

        #[test]
        fn import_coverage_accepts_registered_handlers() {
            for descriptor in crate::registry::list_os_artifact_descriptors() {
                for format in required_os_media_import_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind)) {
                    register_os_media_import_handler(&descriptor.kind, format, |_| Ok(serde_json::json!({})));
                }
            }
            assert!(assert_os_media_import_coverage().is_ok());
        }

        #[test]
        fn svg_to_dwg_round_trip_produces_a_polyline() {
            let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10"><rect x="1" y="1" width="4" height="4"/></svg>"#;
            let bytes = crate::media_export_raster::svg_to_dwg_bytes(svg).expect("svg to dwg");
            let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("dwg from bytes");
            assert!(!drawing.entities.is_empty());
        }

        #[test]
        fn mesh_dwg_registrar_round_trips_a_box() {
            use base64::Engine;
            crate::media_export_raster::register_mesh_dwg_export_handler("3d.__dwg_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
            let result = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&os_media_export_key("3d.__dwg_test", &OsMediaFormat::Dwg)).expect("dwg handler registered")(&serde_json::json!({})).expect("export dwg");
            let bytes = base64::engine::general_purpose::STANDARD.decode(result.data).expect("decode base64");
            let drawing = semio_framework_core::dwg_from_bytes(&bytes).expect("dwg from bytes");
            assert!(!drawing.entities.is_empty());
        }

        #[test]
        fn mesh_exporter_registrar_round_trips_a_box_through_glb() {
            use base64::Engine;
            crate::media_export_raster::register_mesh_exporter("3d.__mesh_exporter_test", "box", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
            let result =
                export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&os_media_export_key("3d.__mesh_exporter_test", &OsMediaFormat::Glb)).expect("glb handler registered")(&serde_json::json!({})).expect("export glb");
            let bytes = base64::engine::general_purpose::STANDARD.decode(result.data).expect("decode base64");
            let mesh = semio_framework_core::mesh_from_glb(&bytes).expect("glb decodes back to a mesh");
            assert!(mesh.vertex_count() > 0);
        }

        #[test]
        fn mesh_importer_registrar_round_trips_a_box_through_obj() {
            crate::media_export_raster::register_mesh_importer("3d.__mesh_importer_test", |mesh| Ok(serde_json::json!({ "vertexCount": mesh.vertex_count() })), Box::new(semio_framework_plugin::ObjImporter));
            let obj_bytes = semio_framework_core::mesh_to_obj(&semio_framework_plugin::mesh_from_kind("box"), "box").into_bytes();
            let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            let handler = handlers.get(&os_media_export_key("3d.__mesh_importer_test", &OsMediaFormat::Obj)).expect("obj handler registered");
            let document = handler(&obj_bytes).expect("import obj");
            assert!(document["vertexCount"].as_u64().expect("vertex count") > 0);
        }

        #[test]
        fn solid_exporter_and_importer_registrars_round_trip_a_box_through_step() {
            let mut kernel = semio_s_3d::brep::kernel::BrepkitKernel::new();
            let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).expect("box");
            crate::media_export_raster::register_solid_exporter("3d.__solid_test", Box::new(semio_s_3d::brep::kernel::StepSolidExporter));
            crate::media_export_raster::register_solid_importer("3d.__solid_test", Box::new(semio_s_3d::brep::kernel::StepSolidImporter));
            assert!(crate::media_export_raster::solid_exporter_for("3d.__solid_test", &OsMediaFormat::Step));
            let bytes = crate::media_export_raster::export_registered_solid("3d.__solid_test", &OsMediaFormat::Step, &kernel, &[solid], 0.1).expect("export step");
            assert!(!bytes.is_empty());
            let imported = crate::media_export_raster::import_registered_solid("3d.__solid_test", &OsMediaFormat::Step, &mut kernel, &bytes, 0.1).expect("import step");
            assert!(!imported.is_empty());
        }

        /// 🧷️ Hand-built node for tests that don't need a real app registration — `os_workflow_to_flow_fixture`/
        /// `build_os_workflow_operator_infos`/VFS listing all read straight off the node now (no more
        /// separate `OsAppInstance` join), so a plain struct literal is enough.
        fn media_node(id: &str, x: f64, y: f64) -> WorkflowNode {
            let port = |direction: semio_framework_core::MediaPortDirection| WorkflowMediaPort {
                id: format!("{id}:{}", if direction == semio_framework_core::MediaPortDirection::In { "in" } else { "out" }),
                spec: semio_framework_core::MediaPortSpec {
                    id: if direction == semio_framework_core::MediaPortDirection::In { "in".into() } else { "out".into() },
                    label: "Port".into(),
                    direction,
                    media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Vector },
                    kind_id: Some("2d.drawing".into()),
                    required: false,
                    multiplicity: semio_framework_core::PortMultiplicity::One,
                },
            };
            WorkflowNode {
                id: id.into(),
                plugin_id: "draw".into(),
                app_id: "draw".into(),
                label: id.into(),
                yields: "2d.drawing".into(),
                document_ref: format!("documents/{id}"),
                config_ref: format!("config/{id}"),
                x,
                y,
                width: 160.0,
                height: 72.0,
                inputs: vec![port(semio_framework_core::MediaPortDirection::In)],
                outputs: vec![port(semio_framework_core::MediaPortDirection::Out)],
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
            assert_eq!(operations, vec![WorkflowOperation::MoveNode { node_id: "node-1".into(), x: 140.0, y: 120.0 }]);
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
                WorkflowOperation::ConnectPorts { edge } if edge.source_node_id == "node-2" && edge.target_port_id == "node-1:in" && !edge.id.is_empty()
            ));
            assert!(operations.contains(&WorkflowOperation::DisconnectEdge { edge_id: "edge-1".into() }));
            let mut removal = os_workflow_to_flow_fixture(&graph, &OsWorkflowCamera::default());
            removal["widgets"] = json!([{ "id": "node-1" }]);
            removal["synapses"] = json!([]);
            let removal_operations = apply_flow_fixture_to_os_workflow(&graph, &removal.to_string());
            assert!(removal_operations.contains(&WorkflowOperation::RemoveNode { node_id: "node-2".into() }));
            assert!(!removal_operations.iter().any(|operation| matches!(operation, WorkflowOperation::DisconnectEdge { .. })));
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
                let fixture = <WorkflowFixture as store::DocumentDsl>::parse_dsl(&contents).unwrap_or_else(|error| panic!("parse fixture {path:?}: {error}"));
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
                let via_dsl = <WorkflowFixture as store::DocumentDsl>::parse_dsl(&dsl_text).unwrap_or_else(|error| panic!("parse {dsl_path:?}: {error}"));
                let via_pack = <WorkflowFixture as store::DocumentPack>::decode_pack(&spk_bytes).unwrap_or_else(|error| panic!("decode {spk_path:?}: {error}"));
                assert_eq!(via_dsl, via_pack, "{dsl_path:?} and {spk_path:?} decode to different documents");
                assert_eq!(store::DocumentDsl::print_dsl(&via_dsl), dsl_text, "{dsl_path:?} is not its own canonical print_dsl fixpoint");
                assert_eq!(store::DocumentPack::encode_pack(&via_dsl), spk_bytes, "{spk_path:?} does not match a fresh canonical encode_pack()");
                store::test_support::assert_dsl_pack_equivalence(&via_dsl);
            }
        }
        //#endregion 🔖️WorkflowPlanner
    }
    //#endregion 🧪️Tests
    // #endregion workflow
}

/// 🌉️ Wasm bindings so the TS twin (`framework/product/os/core/js/index.ts`) decodes the shared
/// `WorkflowFixture` corpus through the same `dsl`/`pack` codepaths Rust uses, instead of `JSON.parse`.
/// Built via `bun ./📜️script.ts wasm` (`s/kernel/store/rs/script.ts`'s `runWasmPackWebBuild` pattern).
#[cfg(target_arch = "wasm32")]
pub mod wasm_exports {
    // #region wasm_exports
    use crate::workflow::WorkflowFixture;
    use wasm_bindgen::prelude::*;

    /// 📦️ Decodes a `WorkflowFixture` from its binary `.spk` pack form into a plain JS object.
    #[wasm_bindgen(js_name = decodeWorkflowFixturePack)]
    pub fn decode_workflow_fixture_pack(bytes: &[u8]) -> Result<JsValue, JsValue> {
        let fixture = <WorkflowFixture as store::DocumentPack>::decode_pack(bytes).map_err(|error| JsValue::from_str(&error.to_string()))?;
        serde_wasm_bindgen::to_value(&fixture).map_err(|error| JsValue::from_str(&error.to_string()))
    }

    /// 📖️ Parses a `WorkflowFixture` from its `.dsl` text form into a plain JS object.
    #[wasm_bindgen(js_name = parseWorkflowFixtureDsl)]
    pub fn parse_workflow_fixture_dsl(text: &str) -> Result<JsValue, JsValue> {
        let fixture = <WorkflowFixture as store::DocumentDsl>::parse_dsl(text).map_err(|error| JsValue::from_str(&error.message))?;
        serde_wasm_bindgen::to_value(&fixture).map_err(|error| JsValue::from_str(&error.to_string()))
    }
    // #endregion wasm_exports
}

pub mod registry {
    // #region registry
    //! 🗂️ Plugin manifest registry and OS plugin/artifact catalog.

    use crate::instance::OsParameterFieldSpec;
    use semio_framework_core::{AppDefinition, ArtifactKindSpec, ConfigSpec, MediaClass, MediaForm, MediaType, ModeDefinition, OsMediaCapability, OsMediaFormat, PluginManifest, WindowKindDefinition};
    use semio_framework_core::{Locale, Terminology};
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
        /// `semio_framework_core::media_types_compatible`.
        pub media_type: MediaType,
        /// 🔌️ Structured-payload schema id, mirrored from `ArtifactKindSpec::schema` — see
        /// `crate::workflow::negotiate_media_contract`, which prefers a matching schema over a shared
        /// binary `OsMediaFormat`.
        pub schema: String,
        pub export_formats: Vec<OsMediaFormat>,
        pub import_formats: Vec<OsMediaFormat>,
    }

    /// 🗂️ One registered resource kind's full catalog entry — the descriptor plus the media capability
    /// its exporters/importers target (kept alongside rather than in `OsArtifactDescriptor` itself since
    /// the descriptor is also the wire-facing presentation shape).
    struct ArtifactKindEntry {
        descriptor: OsArtifactDescriptor,
        media_capability: OsMediaCapability,
    }

    fn artifact_kind_entry_from_spec(spec: &ArtifactKindSpec) -> ArtifactKindEntry {
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
        })
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
    // `MediaType`s via `semio_framework_core::media_types_compatible`, or go through
    // `crate::workflow::negotiate_media_contract` for a full connect-time decision.
    //#endregion 🔖️ResourceDescriptors

    //#region 🔖️PluginRegistry
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsAppRegistration {
        pub id: String,
        pub label: LocalizedLabel,
        pub document: Vec<String>,
        pub controller_id: String,
        pub inputs: Vec<semio_framework_core::MediaPortSpec>,
        pub outputs: Vec<semio_framework_core::MediaPortSpec>,
        pub source_format: String,
        pub component_kind: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_mode_id: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub parameter_fields: Vec<OsParameterFieldSpec>,
        pub modes: Vec<ModeDefinition>,
        /// 🧮️ The app's declared `AppDefinition.config` — how `host::reconcile_os_workflow` resolves
        /// a `plugin_id`/`app_id` app instance's `ConfigSpec` to type-check/materialize its parameter
        /// bindings (`instance::validate_parameter_config_binding`/`build_configure_config`).
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
        let ports = app.io.all_ports();
        let (inputs, outputs): (Vec<_>, Vec<_>) = ports.into_iter().partition(|port| port.direction == semio_framework_core::MediaPortDirection::In);
        let registration = OsAppRegistration {
            id: app.id.clone(),
            label: app.label.clone(),
            document: app.document.clone(),
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
    fn app_io_for_registration(registration: &OsAppRegistration) -> semio_framework_core::AppIo {
        let document_media_type = registration
            .inputs
            .iter()
            .find(|port| port.id == "document:in")
            .or_else(|| registration.outputs.iter().find(|port| port.id == "document:out"))
            .map(|port| port.media_type)
            .unwrap_or(MediaType { class: MediaClass::Data, form: MediaForm::Value });
        let declared_ports: Vec<_> = registration.inputs.iter().chain(registration.outputs.iter()).filter(|port| port.id != "document:in" && port.id != "document:out").cloned().collect();
        semio_framework_core::AppIo::from_document(
            registration.source_format.clone(),
            document_media_type,
            semio_framework_core::ArtifactPresentation {
                id: registration.id.clone(),
                // 🚧️ No locale context reaches this reconstruction path — resolves native/English
                // pending a documented follow-up (same gap as Menu::action_with_args in the plugin SDK).
                name: registration.label.resolve(Terminology::Native, Locale::En).to_string(),
                dimension: String::new(),
                component_kind: registration.component_kind.clone(),
            },
        )
        .with_ports(declared_ports)
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
        let modes = semio_framework_core::Modes::try_from(registration.modes.clone()).ok()?;
        let default_mode_id = registration.default_mode_id.clone().unwrap_or_else(|| modes.first().id.clone());
        let window_kinds = semio_framework_core::WindowKinds::one(WindowKindDefinition {
            id: registration.component_kind.clone(),
            label: registration.label.clone(),
            body_key: registration.component_kind.clone(),
            surface_kind: SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: ui_wgpu::wgpu::WindowOptions::default(),
            actions: Vec::new(),
            utilities: Vec::new(),
            params_schema: None,
            document_projection_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        });
        let io = app_io_for_registration(&registration);
        Some(AppDefinition {
            id: registration.id,
            label: registration.label,
            document: registration.document,
            icon_id: None,
            controller_id: registration.controller_id,
            modes,
            default_mode_id,
            window_kinds,
            panel_tabs: Vec::new(),
            keybindings: Vec::new(),
            actions: Vec::new(),
            utilities: Vec::new(),
            tools: Vec::new(),
            commands: Vec::new(),
            named_layouts: Vec::new(),
            default_layout: None,
            terminologies: Vec::new(),
            terminology_documents: HashMap::new(),
            introduction: None,
            dialogs: Vec::new(),
            media_inputs: Vec::new(),
            media_outputs: Vec::new(),
            artifact_kinds: Vec::new(),
            config: registration.config,
            command_grammar: semio_framework_core::CommandGrammar::empty(),
            io,
            tutorials: Vec::new(),
        })
    }

    /// @emoji 🎨️ One palette entry the browser shell can spawn a workflow node from — a thin,
    /// wire-friendly projection of `OsAppRegistration` (drops `ConfigSpec`/`ModeDefinition`s the
    /// palette doesn't need). `ports` is `app.io.all_ports()` so the palette UI can preview a node's
    /// wiring before it's spawned.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AppPaletteEntry {
        pub plugin_id: String,
        pub app_id: String,
        pub label: LocalizedLabel,
        pub icon_id: String,
        pub ports: Vec<semio_framework_core::MediaPortSpec>,
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
                label: "Draw".into(),
                document: vec!["semio".into(), "draw".into()],
                icon_id: None,
                controller_id: "draw-play".into(),
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "draw".into(),
                    label: "Draw".into(),
                    body_key: "draw".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::wgpu::WindowOptions::default(),
                    actions: Vec::new(),
                    utilities: Vec::new(),
                    params_schema: None,
                    document_projection_schema: None,
                    input_event_schema: None,
                    output_schema: None,
                    capabilities: Vec::new(),
                }),
                panel_tabs: vec![],
                keybindings: vec![],
                actions: vec![],
                utilities: Vec::new(),
                tools: Vec::new(),
                commands: Vec::new(),
                named_layouts: Vec::new(),
                default_layout: None,
                terminologies: Vec::new(),
                terminology_documents: HashMap::new(),
                introduction: None,
                dialogs: Vec::new(),
                media_inputs: Vec::new(),
                media_outputs: Vec::new(),
                artifact_kinds: Vec::new(),
                config: ConfigSpec::empty(),
                command_grammar: semio_framework_core::CommandGrammar::empty(),
                io: semio_framework_core::AppIo::from_document(
                    "draw.document",
                    MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
                    semio_framework_core::ArtifactPresentation { id: "draw".into(), name: "Draw".into(), dimension: "2d".into(), component_kind: "draw".into() },
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

#[cfg(not(target_arch = "wasm32"))]
pub use backbone::{open_file_space_backbone, open_folder_space_backbone};
pub use host::{
    create_backbone_document, create_os_space, decode_backbone_payload, delete_os_space, encode_backbone_payload, export_backbone_dsl, export_backbone_pack, export_os_space_dsl, export_os_space_pack, import_os_space_from_dsl,
    import_os_space_from_pack, list_os_space_catalog_entries, load_os_space_document, materialize_backbone_projection, seed_os_space_catalog_if_empty, BackboneDocument, LoadedProgram, OsBackbonePort, OsCollectionDocument, OsSpaceCatalogEntry,
    OsSpaceDocument, OsSpaceStore, OsWorkflowArtifactDocument, OsWorkflowStore, PluginHost, ProgramHotSwapEvent, ProgramSupervisorState, OS_HOME_VFS_ROOT_ID, OS_SPACE_BACKBONE_URI_PREFIX,
};
pub use instance::{
    apply_parameter_values_to_projection, create_default_os_parameter, create_os_document_id, create_os_id, is_parameter_port_id, materialize_os_app_instance_document_json, media_port_id_for_spec, media_port_spec_id, os_fixture_json,
    os_parameter_types_compatible, os_parameter_value, parameter_id_from_port_id, parameter_port_id, patch_os_parameter, register_os_fixture_json, resolve_parameter_values_for_instance, set_json_pointer_value, OsDocumentRef, OsInstanceState,
    OsParameter, OsParameterFieldBinding, OsParameterFieldSpec, OsParameterType, OS_PARAMETER_PORT_PREFIX,
};
pub use media_export_raster::{
    dwg_drawing_to_svg, export_registered_solid, import_registered_solid, rasterize_svg_to_png_base64, register_2d_export_handlers, register_dwg_import_handler, register_mesh_dwg_export_handler, register_mesh_dwg_import_handler,
    register_mesh_exporter, register_mesh_importer, register_solid_exporter, register_solid_importer, solid_exporter_for, svg_to_dwg_bytes,
};
pub use media_export_simple::{map_points_svg, pages_rects_svg, title_card_svg, wrap_svg};
pub use registry::{
    list_os_artifact_descriptors, os_app_primary_output_kind, os_app_registration, os_artifact_descriptor, register_app_io, register_artifact_descriptor, register_artifact_descriptors, resolve_os_app_definition, try_os_artifact_descriptor,
    workflow_palette, AppPaletteEntry, OsAppRegistration, OsArtifactDescriptor, OsArtifactKindId, PluginRegistry,
};
pub use semio_framework_core::*;
pub use crate::space::*;
pub use store::{document_backbone_ref, set_host_backbone_port, DocumentBackboneRef, DocumentCommand, LocalStorageBackbonePort, MemoryBackbonePort};
pub use ui_wgpu::wgpu::*;
pub use vcs::{Author, Checkpoint, VcsError};
pub use crate::workflow_kernel::{
    apply_flow_fixture_to_os_workflow, apply_workflow_operation, assert_os_media_export_coverage, assert_os_media_import_coverage, build_os_workflow_operator_infos, create_default_workflow_parameter, empty_workflow, empty_workflow_document,
    export_os_app_instance_media, import_os_app_instance_media, negotiate_media_contract, os_media_export_extension_for_format, os_media_neuron_kind_for_node, os_resource_media_capability, os_workflow_to_flow_fixture,
    os_workflow_to_node_graph_payload, patch_workflow_parameter, placeholder_media_contract, plan_workflow, register_os_media_export_handler, register_os_media_import_handler, required_os_media_export_formats, required_os_media_import_formats,
    sync_workflow_parameter_ports, validate_workflow, validate_workflow_document, validate_workflow_parameter_config_binding, workflow_node_for_app, workflow_parameter_id, workflow_parameter_id_from_port_id, workflow_parameter_name,
    workflow_parameter_types_compatible, workflow_parameter_value, MediaContract, OsMediaCapability, OsMediaExportResult, OsMediaFormat, OsWorkflowCamera, OsWorkflowNodeGraphPayload, OsWorkflowOperatorInfo, Workflow, WorkflowDelivery, WorkflowDocument,
    WorkflowEdge, WorkflowFixture, WorkflowInput, WorkflowInputBinding, WorkflowMediaPort, WorkflowNode, WorkflowOperation, WorkflowOutputBinding, WorkflowParameter, WorkflowParameterBinding, WorkflowParameterPatch, WorkflowParameterType,
    WorkflowPosition, WorkflowValidation, OS_MEDIA_FLOW_MODULE_ID, OS_SPACE_SCHEMA, OS_WORKFLOW_VFS_ROOT_ID, S_WORKFLOW_SCHEMA, WORKFLOW_SCHEMA,
};

//#region 🔖️PluginBundleInstallerShim
/// 🛡️ Fallback installer stub inlined — external shim path removed during crate consolidation.
mod plugin_bundle_installer_shim {
    #[allow(dead_code)]
    pub fn install_noop() {}
}
//#endregion 🔖️PluginBundleInstallerShim
