//! 🖥️ Plugin-based OS kernel: hot-swappable WASM plugins, workflow, document VCS.

pub mod host {
    // #region host
    //! 🔌️ Plugin host, studio document VCS store, backbone, and catalog.

    use crate::instance::{create_default_os_parameter, create_os_document_id, create_os_id, patch_os_parameter, OsAppInstance, OsDocumentRef, OsInstanceState, OsParameter, OsParameterFieldBinding, OsParameterType};
    use crate::workflow::{empty_workflow, workflow_node_for_instance, sync_workflow_parameter_ports, WorkflowPosition, OsWorkflow, OsWorkflowEdge, OsWorkflowNode, OS_WORKFLOW_SCHEMA, OS_SPACE_SCHEMA};
    use crate::registry::{os_app_primary_output_kind, os_app_registration, PluginRegistry};
    use semio_framework_core::{AppDefinition, Contribution, PluginManifest, ViewState};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::{HashMap, HashSet};
    use std::sync::{Arc, LazyLock, Mutex};
    use ui_wgpu::{ui_recovery_panel, UiNode};
    use protocol::{Operation, OperationDiff};
    use vcs::{DocumentVcs, VcsError};
use store::{create_document_envelope, document_backbone_ref, materialize_document_projection, DocumentBackboneRef, DocumentCommand, DocumentEnvelope, DocumentStore, SpaceConflict};

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
            }
            for workflow in &program.manifest.workflows {
                self.registry.register_workflow(workflow.clone());
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
            }
            for workflow in &program.manifest.workflows {
                self.registry.register_workflow(workflow.clone());
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

        /// @emoji 🩺️ Delegates to `ui_wgpu::ui_recovery_panel`'s `🔖️StatusBuilders` builder — this host
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
                for workflow in &previous.manifest.workflows {
                    host.registry.register_workflow(workflow.clone());
                }
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

    //#region 🔖️OsDocument
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
    #[serde(rename_all = "camelCase")]
    #[dsl(extension = "os")]
    pub struct OsProjection {
        pub programs: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub active_plugin_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub active_alternative_id: Option<String>,
        #[dsl(table)]
        pub app_instances: Vec<OsAppInstance>,
        #[dsl(block)]
        pub workflow: OsWorkflow,
        #[serde(default)]
        #[dsl(statements)]
        pub parameters: Vec<OsParameter>,
        #[serde(default)]
        #[dsl(table)]
        pub parameter_bindings: Vec<OsParameterFieldBinding>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "operation", rename_all = "camelCase")]
    pub enum OsOperation {
        SetActiveProgram {
            #[serde(skip_serializing_if = "Option::is_none")]
            plugin_id: Option<String>,
        },
        SetActiveAlternative {
            #[serde(skip_serializing_if = "Option::is_none")]
            alternative_id: Option<String>,
        },
        SpawnAppInstance {
            instance: OsAppInstance,
            position: WorkflowPosition,
            /// 🆔️ Minted once at dispatch time (`OsStore::spawn_app_instance`) and carried in the op
            /// itself so replay never re-mints it — see `apply_os_operation`'s `SpawnAppInstance` arm.
            node_id: String,
        },
        RemoveAppInstance {
            instance_id: String,
        },
        ConnectWorkflowPorts {
            edge: OsWorkflowEdge,
        },
        DisconnectWorkflowEdge {
            edge_id: String,
        },
        MoveWorkflowNode {
            node_id: String,
            x: f64,
            y: f64,
        },
        PatchAppInstance {
            instance_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        },
        AddParameter {
            parameter: OsParameter,
        },
        RemoveParameter {
            parameter_id: String,
        },
        PatchParameter {
            parameter_id: String,
            parameter: OsParameter,
        },
        BindParameterField {
            binding: OsParameterFieldBinding,
        },
        UnbindParameterField {
            instance_id: String,
            field_path: String,
        },
        SyncParameterPorts,
        /// 🧮️ Sets (or, with `config: None`, clears back to the app's own defaults) an app instance's
        /// dynamic `OsAppInstance::config` — the studio-document-side counterpart to dispatching
        /// `AppCommand::Configure` at the running app instance (see `build_configure_config`).
        SetAppInstanceConfig {
            instance_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            config: Option<Value>,
        },
    }

    pub type OsVcs = DocumentVcs<OsProjection, OsOperation>;

    /// @emoji 🩹️ Explicit reexport of `serde_json::Value` — the shape of a parameter patch, so callers
    /// don't reach across the crate boundary into `serde_json` directly.
    pub type OsParameterPatch = Value;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsDocument {
        pub schema: String,
        pub id: String,
        pub name: String,
        pub vcs: OsVcs,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub applied_edit_ids: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backbone: Option<DocumentBackboneRef>,
    }

    pub type OsEnvelope = DocumentEnvelope<OsProjection, OsOperation>;

    pub fn default_os_projection() -> OsProjection {
        OsProjection { programs: Vec::new(), active_plugin_id: None, active_alternative_id: None, app_instances: Vec::new(), workflow: empty_workflow(), parameters: Vec::new(), parameter_bindings: Vec::new() }
    }

    pub fn create_empty_os_document(id: &str, name: &str) -> OsDocument {
        OsDocument { schema: OS_SPACE_SCHEMA.into(), id: id.into(), name: name.into(), vcs: create_document_envelope(OS_SPACE_SCHEMA, id, default_os_projection(), None).vcs, applied_edit_ids: Vec::new(), backbone: None }
    }

    pub fn apply_os_operation(projection: &OsProjection, operation: &OsOperation) -> OsProjection {
        let mut next = projection.clone();
        match operation {
            OsOperation::SetActiveProgram { plugin_id } => {
                next.active_plugin_id = plugin_id.clone();
            }
            OsOperation::SetActiveAlternative { alternative_id } => {
                next.active_alternative_id = alternative_id.clone();
            }
            OsOperation::SpawnAppInstance { instance, position, node_id } => {
                if !next.programs.contains(&instance.plugin_id) {
                    next.programs.push(instance.plugin_id.clone());
                }
                if let Some(registration) = os_app_registration(&instance.plugin_id, &instance.app_id) {
                    let node = sync_workflow_node_parameter_ports(&workflow_node_for_instance(instance, &registration, position, node_id), &next.parameter_bindings);
                    next.workflow.nodes.push(node);
                }
                next.app_instances.push(instance.clone());
            }
            OsOperation::RemoveAppInstance { instance_id } => {
                let node_id = next.workflow.nodes.iter().find(|node| node.instance_id == *instance_id).map(|node| node.id.clone());
                next.app_instances.retain(|instance| instance.id != *instance_id);
                next.parameter_bindings.retain(|binding| binding.instance_id != *instance_id);
                next.workflow.nodes.retain(|node| node.instance_id != *instance_id);
                if let Some(node_id) = node_id {
                    next.workflow.edges.retain(|edge| edge.source_node_id != node_id && edge.target_node_id != node_id);
                }
            }
            OsOperation::ConnectWorkflowPorts { edge } => next.workflow.edges.push(edge.clone()),
            OsOperation::DisconnectWorkflowEdge { edge_id } => next.workflow.edges.retain(|edge| edge.id != *edge_id),
            OsOperation::MoveWorkflowNode { node_id, x, y } => {
                for node in &mut next.workflow.nodes {
                    if node.id == *node_id {
                        node.x = *x;
                        node.y = *y;
                    }
                }
            }
            OsOperation::PatchAppInstance { instance_id, label } => {
                if let Some(label) = label {
                    for instance in &mut next.app_instances {
                        if instance.id == *instance_id {
                            instance.label = label.clone();
                        }
                    }
                }
            }
            OsOperation::AddParameter { parameter } => next.parameters.push(parameter.clone()),
            OsOperation::RemoveParameter { parameter_id } => {
                next.parameters.retain(|parameter| parameter_entity_id(parameter) != *parameter_id);
                next.parameter_bindings.retain(|binding| binding.parameter_id != *parameter_id);
                next.workflow = sync_workflow_parameter_ports(&next.workflow, &next.parameter_bindings);
            }
            OsOperation::PatchParameter { parameter_id, parameter } => {
                for entry in &mut next.parameters {
                    if parameter_entity_id(entry) == *parameter_id {
                        *entry = parameter.clone();
                    }
                }
            }
            OsOperation::BindParameterField { binding } => {
                next.parameter_bindings.retain(|entry| !(entry.instance_id == binding.instance_id && entry.field_path == binding.field_path));
                next.parameter_bindings.push(binding.clone());
                next.workflow = sync_workflow_parameter_ports(&next.workflow, &next.parameter_bindings);
            }
            OsOperation::UnbindParameterField { instance_id, field_path } => {
                next.parameter_bindings.retain(|binding| !(binding.instance_id == *instance_id && binding.field_path == *field_path));
                next.workflow = sync_workflow_parameter_ports(&next.workflow, &next.parameter_bindings);
            }
            OsOperation::SyncParameterPorts => {
                next.workflow = sync_workflow_parameter_ports(&next.workflow, &next.parameter_bindings);
            }
            OsOperation::SetAppInstanceConfig { instance_id, config } => {
                for instance in &mut next.app_instances {
                    if instance.id == *instance_id {
                        instance.config = config.clone();
                    }
                }
            }
        }
        next
    }

    fn sync_workflow_node_parameter_ports(node: &crate::workflow::OsWorkflowNode, bindings: &[OsParameterFieldBinding]) -> crate::workflow::OsWorkflowNode {
        sync_workflow_parameter_ports(&OsWorkflow { schema: OS_WORKFLOW_SCHEMA.into(), nodes: vec![node.clone()], edges: Vec::new() }, bindings).nodes.into_iter().next().unwrap_or_else(|| node.clone())
    }

    fn parameter_entity_id(parameter: &OsParameter) -> &str {
        match parameter {
            OsParameter::Numeric { id, .. } | OsParameter::Categorical { id, .. } | OsParameter::Toggle { id, .. } | OsParameter::Text { id, .. } => id,
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "kind", rename_all = "camelCase")]
    pub enum OsDiff {
        #[default]
        Empty,
        SetActiveProgram {
            #[serde(skip_serializing_if = "Option::is_none")]
            plugin_id: Option<String>,
        },
        SetActiveAlternative {
            #[serde(skip_serializing_if = "Option::is_none")]
            alternative_id: Option<String>,
        },
        SpawnAppInstance {
            instance: OsAppInstance,
            position: WorkflowPosition,
            node_id: String,
        },
        RemoveAppInstance {
            instance_id: String,
        },
        ConnectWorkflowPorts {
            edge: OsWorkflowEdge,
        },
        DisconnectWorkflowEdge {
            edge_id: String,
        },
        MoveWorkflowNode {
            node_id: String,
            x: f64,
            y: f64,
        },
        PatchAppInstance {
            instance_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            label: Option<String>,
        },
        AddParameter {
            parameter: OsParameter,
        },
        RemoveParameter {
            parameter_id: String,
        },
        PatchParameter {
            parameter_id: String,
            parameter: OsParameter,
        },
        BindParameterField {
            binding: OsParameterFieldBinding,
        },
        UnbindParameterField {
            instance_id: String,
            field_path: String,
        },
        SyncParameterPorts,
        SetAppInstanceConfig {
            instance_id: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            config: Option<Value>,
        },
    }

    impl OperationDiff<OsProjection> for OsDiff {
        fn apply(&self, projection: &OsProjection) -> OsProjection {
            let operation = match self {
                OsDiff::Empty => return projection.clone(),
                OsDiff::SetActiveProgram { plugin_id } => OsOperation::SetActiveProgram { plugin_id: plugin_id.clone() },
                OsDiff::SetActiveAlternative { alternative_id } => OsOperation::SetActiveAlternative { alternative_id: alternative_id.clone() },
                OsDiff::SpawnAppInstance { instance, position, node_id } => OsOperation::SpawnAppInstance { instance: instance.clone(), position: position.clone(), node_id: node_id.clone() },
                OsDiff::RemoveAppInstance { instance_id } => OsOperation::RemoveAppInstance { instance_id: instance_id.clone() },
                OsDiff::ConnectWorkflowPorts { edge } => OsOperation::ConnectWorkflowPorts { edge: edge.clone() },
                OsDiff::DisconnectWorkflowEdge { edge_id } => OsOperation::DisconnectWorkflowEdge { edge_id: edge_id.clone() },
                OsDiff::MoveWorkflowNode { node_id, x, y } => OsOperation::MoveWorkflowNode { node_id: node_id.clone(), x: *x, y: *y },
                OsDiff::PatchAppInstance { instance_id, label } => OsOperation::PatchAppInstance { instance_id: instance_id.clone(), label: label.clone() },
                OsDiff::AddParameter { parameter } => OsOperation::AddParameter { parameter: parameter.clone() },
                OsDiff::RemoveParameter { parameter_id } => OsOperation::RemoveParameter { parameter_id: parameter_id.clone() },
                OsDiff::PatchParameter { parameter_id, parameter } => OsOperation::PatchParameter { parameter_id: parameter_id.clone(), parameter: parameter.clone() },
                OsDiff::BindParameterField { binding } => OsOperation::BindParameterField { binding: binding.clone() },
                OsDiff::UnbindParameterField { instance_id, field_path } => OsOperation::UnbindParameterField { instance_id: instance_id.clone(), field_path: field_path.clone() },
                OsDiff::SyncParameterPorts => OsOperation::SyncParameterPorts,
                OsDiff::SetAppInstanceConfig { instance_id, config } => OsOperation::SetAppInstanceConfig { instance_id: instance_id.clone(), config: config.clone() },
            };
            apply_os_operation(projection, &operation)
        }

        fn absorb(&mut self, other: Self) {
            if !matches!(other, OsDiff::Empty) {
                *self = other;
            }
        }
    }

    impl Operation<OsProjection> for OsOperation {
        type Diff = OsDiff;

        fn diff(&self, _projection: &OsProjection) -> OsDiff {
            match self {
                OsOperation::SetActiveProgram { plugin_id } => OsDiff::SetActiveProgram { plugin_id: plugin_id.clone() },
                OsOperation::SetActiveAlternative { alternative_id } => OsDiff::SetActiveAlternative { alternative_id: alternative_id.clone() },
                OsOperation::SpawnAppInstance { instance, position, node_id } => OsDiff::SpawnAppInstance { instance: instance.clone(), position: position.clone(), node_id: node_id.clone() },
                OsOperation::RemoveAppInstance { instance_id } => OsDiff::RemoveAppInstance { instance_id: instance_id.clone() },
                OsOperation::ConnectWorkflowPorts { edge } => OsDiff::ConnectWorkflowPorts { edge: edge.clone() },
                OsOperation::DisconnectWorkflowEdge { edge_id } => OsDiff::DisconnectWorkflowEdge { edge_id: edge_id.clone() },
                OsOperation::MoveWorkflowNode { node_id, x, y } => OsDiff::MoveWorkflowNode { node_id: node_id.clone(), x: *x, y: *y },
                OsOperation::PatchAppInstance { instance_id, label } => OsDiff::PatchAppInstance { instance_id: instance_id.clone(), label: label.clone() },
                OsOperation::AddParameter { parameter } => OsDiff::AddParameter { parameter: parameter.clone() },
                OsOperation::RemoveParameter { parameter_id } => OsDiff::RemoveParameter { parameter_id: parameter_id.clone() },
                OsOperation::PatchParameter { parameter_id, parameter } => OsDiff::PatchParameter { parameter_id: parameter_id.clone(), parameter: parameter.clone() },
                OsOperation::BindParameterField { binding } => OsDiff::BindParameterField { binding: binding.clone() },
                OsOperation::UnbindParameterField { instance_id, field_path } => OsDiff::UnbindParameterField { instance_id: instance_id.clone(), field_path: field_path.clone() },
                OsOperation::SyncParameterPorts => OsDiff::SyncParameterPorts,
                OsOperation::SetAppInstanceConfig { instance_id, config } => OsDiff::SetAppInstanceConfig { instance_id: instance_id.clone(), config: config.clone() },
            }
        }

        fn backwards(&self, projection: &OsProjection) -> Vec<Self> {
            match self {
                OsOperation::SetActiveProgram { .. } => vec![OsOperation::SetActiveProgram { plugin_id: projection.active_plugin_id.clone() }],
                OsOperation::SetActiveAlternative { .. } => vec![OsOperation::SetActiveAlternative { alternative_id: projection.active_alternative_id.clone() }],
                OsOperation::SpawnAppInstance { instance, .. } => vec![OsOperation::RemoveAppInstance { instance_id: instance.id.clone() }],
                OsOperation::RemoveAppInstance { instance_id } => projection
                    .app_instances
                    .iter()
                    .find(|instance| instance.id == *instance_id)
                    .map(|instance| {
                        let node = projection.workflow.nodes.iter().find(|entry| entry.instance_id == *instance_id);
                        vec![OsOperation::SpawnAppInstance {
                            instance: instance.clone(),
                            position: WorkflowPosition { x: node.map(|entry| entry.x).unwrap_or(0.0), y: node.map(|entry| entry.y).unwrap_or(0.0) },
                            node_id: node.map(|entry| entry.id.clone()).unwrap_or_else(|| create_os_id("node")),
                        }]
                    })
                    .unwrap_or_default(),
                OsOperation::ConnectWorkflowPorts { edge } => vec![OsOperation::DisconnectWorkflowEdge { edge_id: edge.id.clone() }],
                OsOperation::DisconnectWorkflowEdge { edge_id } => projection.workflow.edges.iter().find(|edge| edge.id == *edge_id).map(|edge| vec![OsOperation::ConnectWorkflowPorts { edge: edge.clone() }]).unwrap_or_default(),
                OsOperation::MoveWorkflowNode { node_id, .. } => projection.workflow.nodes.iter().find(|node| node.id == *node_id).map(|node| vec![OsOperation::MoveWorkflowNode { node_id: node_id.clone(), x: node.x, y: node.y }]).unwrap_or_default(),
                OsOperation::PatchAppInstance { instance_id, .. } => {
                    projection.app_instances.iter().find(|instance| instance.id == *instance_id).map(|instance| vec![OsOperation::PatchAppInstance { instance_id: instance_id.clone(), label: Some(instance.label.clone()) }]).unwrap_or_default()
                }
                OsOperation::AddParameter { parameter } => vec![OsOperation::RemoveParameter { parameter_id: parameter_entity_id(parameter).into() }],
                OsOperation::RemoveParameter { parameter_id } => projection.parameters.iter().find(|parameter| parameter_entity_id(parameter) == *parameter_id).map(|parameter| vec![OsOperation::AddParameter { parameter: parameter.clone() }]).unwrap_or_default(),
                OsOperation::PatchParameter { parameter_id, parameter } => projection
                    .parameters
                    .iter()
                    .find(|entry| parameter_entity_id(entry) == *parameter_id)
                    .map(|current| vec![OsOperation::PatchParameter { parameter_id: parameter_id.clone(), parameter: current.clone() }])
                    .unwrap_or_else(|| vec![OsOperation::PatchParameter { parameter_id: parameter_id.clone(), parameter: parameter.clone() }]),
                OsOperation::BindParameterField { binding } => vec![OsOperation::UnbindParameterField { instance_id: binding.instance_id.clone(), field_path: binding.field_path.clone() }],
                OsOperation::UnbindParameterField { instance_id, field_path } => {
                    projection.parameter_bindings.iter().find(|binding| binding.instance_id == *instance_id && binding.field_path == *field_path).map(|binding| vec![OsOperation::BindParameterField { binding: binding.clone() }]).unwrap_or_default()
                }
                OsOperation::SyncParameterPorts => Vec::new(),
                OsOperation::SetAppInstanceConfig { instance_id, .. } => projection
                    .app_instances
                    .iter()
                    .find(|instance| instance.id == *instance_id)
                    .map(|instance| vec![OsOperation::SetAppInstanceConfig { instance_id: instance_id.clone(), config: instance.config.clone() }])
                    .unwrap_or_default(),
            }
        }

        /// @emoji 🤝️ Workflow referential-integrity pass — see `reconcile_os_workflow` (region
        /// 🔖️GraphReconcile below) for the four ordered rules it runs.
        ///
        /// 🎞️ CW3 kernel cut-over ripple fix: `Operation::reconcile` moved to `protocol_command` with
        /// a changed signature (was an associated fn, `fn reconcile(projection: P) -> (P,
        /// Vec<SpaceConflict>)`; now an instance method returning `protocol`-owned
        /// `Vec<ReconcileReport>`, per that trait's own doc comment: "`vcs` maps `ReconcileReport ->
        /// SpaceConflict` at its own edge instead of this crate knowing about space types"). The
        /// workflow rules themselves (`reconcile_os_workflow`) are untouched — only this thin
        /// trait-facing wrapper adapts to the new signature, converting each `SpaceConflict` to a
        /// `ReconcileReport` at the boundary.
        fn reconcile(&self, projection: OsProjection) -> (OsProjection, Vec<protocol::ReconcileReport>) {
            let (projection, conflicts) = reconcile_os_workflow(projection);
            let reports = conflicts
                .into_iter()
                .map(|conflict| protocol::ReconcileReport { id: conflict.kind, message: conflict.message, severity: protocol::ReconcileSeverity::Warning })
                .collect();
            (projection, reports)
        }
    }

    //#region 🔖️GraphReconcile
    /// @emoji 🧵️ Post-materialization workflow integrity pass run by `OsOperation::reconcile`. Runs, in
    /// order: (1) drop edges whose source/target node or port no longer exists (a concurrent delete
    /// tombstone wins over the wiring), (2) drop edges whose port types no longer match (a concurrent
    /// re-typing wins over the wiring), (3) dedupe edges with identical endpoints down to the
    /// lexicographically smallest id (deterministic across peers replaying the same operation log), (4) break
    /// any cycle the previous rules left behind. Each rule operates on the edge set the previous one
    /// produced.
    fn reconcile_os_workflow(mut projection: OsProjection) -> (OsProjection, Vec<SpaceConflict>) {
        let mut conflicts = Vec::new();
        let mut edges = std::mem::take(&mut projection.workflow.edges);
        let node_by_id: HashMap<&str, &OsWorkflowNode> = projection.workflow.nodes.iter().map(|node| (node.id.as_str(), node)).collect();

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
        // 🩹️ Baseline comparison: straight `artifact_kind` string equality, since `OsWorkflowEdge` has
        // no `contract` field yet. Once edge contracts land, prefer the contract's `kind_id` against the
        // live port types (contract wins if present), falling back to this comparison otherwise.
        edges.retain(|edge| {
            let source_kind = node_by_id.get(edge.source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id)).map(|port| port.artifact_kind.as_str());
            let target_kind = node_by_id.get(edge.target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id)).map(|port| port.artifact_kind.as_str());
            match (source_kind, target_kind) {
                (Some(source), Some(target)) if source == target => true,
                _ => {
                    conflicts.push(SpaceConflict {
                        kind: "workflow/edge-type-mismatch".into(),
                        uri: edge.id.clone(),
                        message: format!("edge {} connects ports whose types no longer match", edge.id),
                    });
                    false
                }
            }
        });
        //#endregion TypeMismatchDrop

        //#region DuplicateWireDedupe
        // 🧮️ No conflict reported here — identical wiring intent from two peers isn't a disagreement.
        let mut smallest_id_for_wire: HashMap<(String, String, String, String), String> = HashMap::new();
        for edge in &edges {
            let wire = (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
            smallest_id_for_wire.entry(wire).and_modify(|smallest| if edge.id < *smallest { *smallest = edge.id.clone() }).or_insert_with(|| edge.id.clone());
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
        // 🛡️ Same defense-in-depth spirit as `TypeMismatchDrop`, for the OTHER thing a concurrent edit
        // can invalidate: a binding's target `ConfigSpec` field (a concurrent app swap/downgrade
        // removed or re-shaped the field) or the bound parameter's own type (a concurrent
        // `PatchParameter` re-typed it). Missing instance/registration/parameter is left alone here —
        // those are separate lifecycle concerns already handled at op-apply time (`RemoveAppInstance`/
        // `RemoveParameter` retain the dependent bindings themselves) or by a plugin simply not being
        // loaded yet, neither of which this binding is at fault for.
        let parameters = projection.parameters.clone();
        let app_instances = projection.app_instances.clone();
        projection.parameter_bindings.retain(|binding| {
            let Some(instance) = app_instances.iter().find(|instance| instance.id == binding.instance_id) else { return true };
            let Some(registration) = os_app_registration(&instance.plugin_id, &instance.app_id) else { return true };
            let Some(parameter_type) = parameters.iter().find(|parameter| parameter_entity_id(parameter) == binding.parameter_id).map(parameter_type_of) else { return true };
            match crate::instance::validate_parameter_config_binding(binding, &parameter_type, &registration.config) {
                Ok(()) => true,
                Err(conflict) => {
                    conflicts.push(conflict);
                    false
                }
            }
        });
        projection.workflow = sync_workflow_parameter_ports(&projection.workflow, &projection.parameter_bindings);
        //#endregion ParameterBindingValidation

        projection.workflow.edges = edges;
        (projection, conflicts)
    }

    /// @emoji 🎛️ Maps an `OsParameter` to its `OsParameterType` tag — the `reconcile_os_workflow`-local
    /// twin of the space plugin UI's own inline match (`os_parameter_types_compatible`'s caller),
    /// needed here to type-check a binding's parameter against its target `ConfigFieldShape`.
    fn parameter_type_of(parameter: &OsParameter) -> OsParameterType {
        match parameter {
            OsParameter::Numeric { .. } => OsParameterType::Numeric,
            OsParameter::Categorical { .. } => OsParameterType::Categorical,
            OsParameter::Toggle { .. } => OsParameterType::Toggle,
            OsParameter::Text { .. } => OsParameterType::Text,
        }
    }

    /// @emoji 🌀️ Repeatedly finds a cycle in `edges` (by node-id adjacency) and drops the participating
    /// edge with the highest array index — a deterministic proxy for "newest edit" since
    /// `Operation::reconcile` only receives the materialized `OsProjection` by value, not per-edge
    /// `HybridLogicalTimestamp`s from the edit log. `apply_os_operation`'s `ConnectWorkflowPorts` handler
    /// appends new edges to the end of the vec, so a higher index approximates a later edit; true
    /// HLT-based tie-breaking would need `reconcile` to also see edit history, not just the projection.
    fn drop_workflow_cycle_edges(mut edges: Vec<OsWorkflowEdge>, conflicts: &mut Vec<SpaceConflict>) -> Vec<OsWorkflowEdge> {
        while let Some(cycle_node_ids) = find_workflow_cycle_participants(&edges) {
            let newest_cycle_edge_index = edges.iter().enumerate().filter(|(_, edge)| cycle_node_ids.contains(&edge.source_node_id) && cycle_node_ids.contains(&edge.target_node_id)).map(|(index, _)| index).max();
            let Some(newest_cycle_edge_index) = newest_cycle_edge_index else { break };
            let dropped = edges.remove(newest_cycle_edge_index);
            conflicts.push(SpaceConflict {
                kind: "workflow/edge-cycle".into(),
                uri: dropped.id.clone(),
                message: format!("edge {} was dropped to break a cycle in the workflow", dropped.id),
            });
        }
        edges
    }

    /// @emoji 🔍️ DFS cycle detection adapted from `workflow::validate_workflow`'s check, but
    /// returning the participant node ids of the first cycle found (rather than just an error string)
    /// so the caller can identify which edges are eligible for dropping.
    fn find_workflow_cycle_participants(edges: &[OsWorkflowEdge]) -> Option<HashSet<String>> {
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

    //#region 🔖️Dsl
    /// @emoji 🧬️ `OsProjection`'s `.os` studio DSL now derives from `#[derive(dsl::DslDocument)]` on
    /// `OsProjection` itself (see its declaration above, in `🔖️OsDocument`) — none of its own fields
    /// need boxing (no bare nested tagged-enum field), so the real type derives the grammar directly,
    /// no local mirror type needed. `MediaContract` (`workflow` module, `🔖️MediaContractDsl` region)
    /// is the one exception, hand-bridged instead of derived. `store::DocumentDsl for OsProjection` is
    /// therefore generated entirely by the derive macro; nothing further to implement here.
    //#endregion 🔖️Dsl

    //#region 🔖️OpText
    //#region 🔖️OpTextMirror
    /// 🧬️ Local structural twin of {@link OsOperation} for the `dsl::DslOps` derive — every variant
    /// identical except `AddParameter`/`PatchParameter`'s `parameter` field, boxed only here
    /// (`#[dsl(statements)] Box<OsParameter>`, the derive engine's "exactly one required tagged value"
    /// shape) because `OsParameter` derives `dsl::DslEnum` (only `dsl::DslVariants`, not `dsl::DslField`)
    /// and the engine's `#[dsl(statements)]` recognizes only `Vec`/`Option`/`Box` wrappers, never a bare
    /// tagged-enum field. `OsOperation` itself keeps its original unboxed `parameter: OsParameter`
    /// shape — downstream crates matching on it by name see zero change — with conversion happening
    /// only at this boundary, mirroring `imperative_core`'s `ImperativeOperationDsl` and
    /// `infinite_board_port_directed_dag`'s `DagOperationDsl`.
    #[derive(Clone, Debug, PartialEq, dsl::DslOps)]
    enum OsOperationDsl {
        SetActiveProgram {
            #[dsl(key = "id")]
            plugin_id: Option<String>,
        },
        SetActiveAlternative {
            #[dsl(key = "id")]
            alternative_id: Option<String>,
        },
        SpawnAppInstance { instance: OsAppInstance, position: WorkflowPosition, #[dsl(key = "node")] node_id: String },
        RemoveAppInstance {
            #[dsl(key = "id")]
            instance_id: String,
        },
        ConnectWorkflowPorts { edge: OsWorkflowEdge },
        DisconnectWorkflowEdge {
            #[dsl(key = "id")]
            edge_id: String,
        },
        MoveWorkflowNode {
            #[dsl(key = "id")]
            node_id: String,
            x: f64,
            y: f64,
        },
        PatchAppInstance {
            #[dsl(key = "id")]
            instance_id: String,
            label: Option<String>,
        },
        AddParameter {
            #[dsl(statements)]
            parameter: Box<OsParameter>,
        },
        RemoveParameter {
            #[dsl(key = "id")]
            parameter_id: String,
        },
        PatchParameter {
            #[dsl(key = "target")]
            parameter_id: String,
            #[dsl(statements)]
            parameter: Box<OsParameter>,
        },
        BindParameterField { binding: OsParameterFieldBinding },
        UnbindParameterField { instance_id: String, field_path: String },
        SyncParameterPorts,
        SetAppInstanceConfig {
            #[dsl(key = "id")]
            instance_id: String,
            config: Option<Value>,
        },
    }

    fn os_operation_to_dsl(operation: &OsOperation) -> OsOperationDsl {
        match operation {
            OsOperation::SetActiveProgram { plugin_id } => OsOperationDsl::SetActiveProgram { plugin_id: plugin_id.clone() },
            OsOperation::SetActiveAlternative { alternative_id } => OsOperationDsl::SetActiveAlternative { alternative_id: alternative_id.clone() },
            OsOperation::SpawnAppInstance { instance, position, node_id } => OsOperationDsl::SpawnAppInstance { instance: instance.clone(), position: position.clone(), node_id: node_id.clone() },
            OsOperation::RemoveAppInstance { instance_id } => OsOperationDsl::RemoveAppInstance { instance_id: instance_id.clone() },
            OsOperation::ConnectWorkflowPorts { edge } => OsOperationDsl::ConnectWorkflowPorts { edge: edge.clone() },
            OsOperation::DisconnectWorkflowEdge { edge_id } => OsOperationDsl::DisconnectWorkflowEdge { edge_id: edge_id.clone() },
            OsOperation::MoveWorkflowNode { node_id, x, y } => OsOperationDsl::MoveWorkflowNode { node_id: node_id.clone(), x: *x, y: *y },
            OsOperation::PatchAppInstance { instance_id, label } => OsOperationDsl::PatchAppInstance { instance_id: instance_id.clone(), label: label.clone() },
            OsOperation::AddParameter { parameter } => OsOperationDsl::AddParameter { parameter: Box::new(parameter.clone()) },
            OsOperation::RemoveParameter { parameter_id } => OsOperationDsl::RemoveParameter { parameter_id: parameter_id.clone() },
            OsOperation::PatchParameter { parameter_id, parameter } => OsOperationDsl::PatchParameter { parameter_id: parameter_id.clone(), parameter: Box::new(parameter.clone()) },
            OsOperation::BindParameterField { binding } => OsOperationDsl::BindParameterField { binding: binding.clone() },
            OsOperation::UnbindParameterField { instance_id, field_path } => OsOperationDsl::UnbindParameterField { instance_id: instance_id.clone(), field_path: field_path.clone() },
            OsOperation::SyncParameterPorts => OsOperationDsl::SyncParameterPorts,
            OsOperation::SetAppInstanceConfig { instance_id, config } => OsOperationDsl::SetAppInstanceConfig { instance_id: instance_id.clone(), config: config.clone() },
        }
    }

    fn os_operation_from_dsl(operation: OsOperationDsl) -> OsOperation {
        match operation {
            OsOperationDsl::SetActiveProgram { plugin_id } => OsOperation::SetActiveProgram { plugin_id },
            OsOperationDsl::SetActiveAlternative { alternative_id } => OsOperation::SetActiveAlternative { alternative_id },
            OsOperationDsl::SpawnAppInstance { instance, position, node_id } => OsOperation::SpawnAppInstance { instance, position, node_id },
            OsOperationDsl::RemoveAppInstance { instance_id } => OsOperation::RemoveAppInstance { instance_id },
            OsOperationDsl::ConnectWorkflowPorts { edge } => OsOperation::ConnectWorkflowPorts { edge },
            OsOperationDsl::DisconnectWorkflowEdge { edge_id } => OsOperation::DisconnectWorkflowEdge { edge_id },
            OsOperationDsl::MoveWorkflowNode { node_id, x, y } => OsOperation::MoveWorkflowNode { node_id, x, y },
            OsOperationDsl::PatchAppInstance { instance_id, label } => OsOperation::PatchAppInstance { instance_id, label },
            OsOperationDsl::AddParameter { parameter } => OsOperation::AddParameter { parameter: *parameter },
            OsOperationDsl::RemoveParameter { parameter_id } => OsOperation::RemoveParameter { parameter_id },
            OsOperationDsl::PatchParameter { parameter_id, parameter } => OsOperation::PatchParameter { parameter_id, parameter: *parameter },
            OsOperationDsl::BindParameterField { binding } => OsOperation::BindParameterField { binding },
            OsOperationDsl::UnbindParameterField { instance_id, field_path } => OsOperation::UnbindParameterField { instance_id, field_path },
            OsOperationDsl::SyncParameterPorts => OsOperation::SyncParameterPorts,
            OsOperationDsl::SetAppInstanceConfig { instance_id, config } => OsOperation::SetAppInstanceConfig { instance_id, config },
        }
    }

    impl protocol::OpText for OsOperation {
        fn parse_op(line: &str) -> Result<Self, store::TextError> {
            Ok(os_operation_from_dsl(<OsOperationDsl as protocol::OpText>::parse_op(line)?))
        }

        fn print_op(&self) -> String {
            <OsOperationDsl as protocol::OpText>::print_op(&os_operation_to_dsl(self))
        }
    }

    /// @emoji 🎞️ Binary mirror of the `OpText` bridge above — `OsOperationDsl` already derives
    /// `OpBinary` via `#[derive(dsl::DslOps)]`, so this is a pure to/from-dsl forward.
    impl protocol::OpBinary for OsOperation {
        fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
            os_operation_to_dsl(self).encode_op()
        }

        fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
            Ok(os_operation_from_dsl(OsOperationDsl::decode_op(bytes)?))
        }
    }
    //#endregion 🔖️OpTextMirror
    //#endregion 🔖️OpText

    /// @emoji 🌉️ Builds the bare `OsEnvelope` an `OsDocument` wraps (dropping the app-level `name`/
    /// `applied_edit_ids` fields) — shared by every typed pack/text export path below.
    fn os_envelope_of(document: &OsDocument) -> OsEnvelope {
        OsEnvelope { schema: document.schema.clone(), id: document.id.clone(), vcs: document.vcs.clone(), backbone: document.backbone.clone(), active_alternative_id: document.vcs.initial_projection.active_alternative_id.clone(), cursor: None }
    }

    pub fn materialize_os_projection(document: &OsDocument, applied_edit_ids: &[String]) -> Result<OsProjection, VcsError> {
        let envelope = os_envelope_of(document);
        materialize_document_projection(&envelope, applied_edit_ids)
    }

    pub fn os_document_to_json(document: &OsDocument) -> Result<String, VcsError> {
        serde_json::to_string_pretty(document).map_err(|error| VcsError::Serialize(error.to_string()))
    }

    pub fn os_document_from_json(json: &str) -> Result<OsDocument, VcsError> {
        let document: OsDocument = serde_json::from_str(json).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        if document.schema != OS_SPACE_SCHEMA {
            return Err(VcsError::Deserialize(format!("expected schema {OS_SPACE_SCHEMA}")));
        }
        Ok(document)
    }

    /// @emoji 📦️ Binary pack+spr payload for the whole `OsDocument` (name + applied-edit cursor +
    /// vcs) — the persisted/synced form, replacing `os_document_to_json`. Unlike `export_os_space_pack`
    /// (a one-shot "download a copy" that intentionally drops `name`/history position per
    /// `os_envelope_of`'s doc), this is the round-trip-faithful codec: `name` rides as a
    /// `store::encode_document_pack_bytes`-framed blob wrapping a nested `pack`+`spr` pair (the same
    /// length-prefixed-blob codec, reused twice instead of a third hand-rolled format), and
    /// `applied_edit_ids` rides through the envelope's `cursor` so `spr`'s cursor line restores the
    /// exact undo/redo position (see `store::print_document_pack`'s doc on `cursor`).
    pub fn encode_os_space_payload(document: &OsDocument) -> Result<Vec<u8>, VcsError> {
        let mut envelope = os_envelope_of(document);
        envelope.cursor = Some(store::DocumentCursor { applied_edit_ids: document.applied_edit_ids.clone(), redo_edit_ids: Vec::new(), checkpoint_id: None });
        let files = store::print_document_pack(&envelope)?;
        let inner = store::encode_document_pack_bytes(&files.pack, &files.spr);
        Ok(store::encode_document_pack_bytes(document.name.as_bytes(), &inner))
    }

    /// @emoji 📥️ Inverse of `encode_os_space_payload`.
    pub fn decode_os_space_payload(bytes: &[u8]) -> Result<OsDocument, VcsError> {
        let (name_bytes, inner) = store::decode_document_pack_bytes(bytes)?;
        let name = String::from_utf8(name_bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let (pack, spr) = store::decode_document_pack_bytes(&inner)?;
        let parsed: store::ParsedDocumentText<OsProjection, OsOperation> = store::parse_document_pack(&pack, &spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        if parsed.envelope.schema != OS_SPACE_SCHEMA {
            return Err(VcsError::Deserialize(format!("expected schema {OS_SPACE_SCHEMA}")));
        }
        let applied_edit_ids = parsed.envelope.cursor.as_ref().map(|cursor| cursor.applied_edit_ids.clone()).unwrap_or_default();
        Ok(OsDocument { schema: parsed.envelope.schema, id: parsed.envelope.id, name, vcs: parsed.envelope.vcs, applied_edit_ids, backbone: parsed.envelope.backbone })
    }
    //#endregion 🔖️OsDocument

    //#region 🔖️OsStore
    pub struct OsStore {
        inner: DocumentStore<OsProjection, OsOperation>,
        name: String,
    }

    impl OsStore {
        pub fn new(document: OsDocument) -> Self {
            let applied_edit_ids = document.applied_edit_ids.clone();
            let envelope = OsEnvelope { schema: document.schema, id: document.id, vcs: document.vcs, backbone: document.backbone, active_alternative_id: None, cursor: None };
            let mut inner = DocumentStore::new(envelope);
            if !applied_edit_ids.is_empty() {
                let snapshot = inner.envelope().clone();
                inner.set_envelope(snapshot, applied_edit_ids);
            }
            Self { inner, name: document.name }
        }

        pub fn generation(&self) -> u64 {
            self.inner.generation()
        }

        pub fn projection(&self) -> Result<OsProjection, VcsError> {
            self.inner.projection()
        }

        /// @emoji 🤝️ Fresh replay plus whatever `OsOperation::reconcile`'s workflow pass reports. See
        /// `store::DocumentStore::projection_with_conflicts`.
        pub fn projection_with_conflicts(&self) -> Result<(OsProjection, Vec<SpaceConflict>), VcsError> {
            self.inner.projection_with_conflicts()
        }

        pub fn document(&self) -> OsDocument {
            let envelope = self.inner.envelope();
            OsDocument { schema: envelope.schema.clone(), id: envelope.id.clone(), name: self.name.clone(), vcs: envelope.vcs.clone(), applied_edit_ids: self.inner.applied_edit_ids().to_vec(), backbone: envelope.backbone.clone() }
        }

        pub fn dispatch_text(&mut self, command_text: &str) -> Result<(), VcsError> {
            self.inner.dispatch_text(command_text)
        }

        pub fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<(), VcsError> {
            self.inner.dispatch_binary(command_bytes)
        }

        pub fn dispatch_apply(&mut self, operations: Vec<OsOperation>) -> Result<(), VcsError> {
            self.inner.dispatch(DocumentCommand::Apply { operations, description: None })
        }

        pub fn set_space_name(&mut self, name: &str) {
            self.name = name.into();
            let _ = self.inner.generation();
        }

        pub fn spawn_app_instance(&mut self, plugin_id: &str, app_id: &str, label: Option<&str>, position: WorkflowPosition) -> Result<String, VcsError> {
            let registration = os_app_registration(plugin_id, app_id).ok_or_else(|| VcsError::Deserialize(format!("unknown app {plugin_id}/{app_id}")))?;
            let instance_id = create_os_id("app");
            // 🆔️ Minted once, here, at dispatch time; the id is embedded in the stored `OsOperation` itself so
            // replay is deterministic (it never re-mints) — same idempotency property `create_os_id`
            // already relies on for `instance_id`.
            let document_id = create_os_document_id();
            let node_id = create_os_id("node");
            let instance = OsAppInstance {
                id: instance_id.clone(),
                plugin_id: plugin_id.into(),
                app_id: app_id.into(),
                label: label.map(str::to_string).unwrap_or_else(|| registration.label.clone()),
                yields: os_app_primary_output_kind(&registration),
                document: OsDocumentRef { document_id, schema: registration.source_format.clone() },
                config: None,
            };
            self.dispatch_apply(vec![OsOperation::SpawnAppInstance { instance, position, node_id }])?;
            Ok(instance_id)
        }

        pub fn add_parameter(&mut self, parameter_type: &OsParameterType, name: &str) -> Result<String, VcsError> {
            let parameter = create_default_os_parameter(parameter_type, name, None);
            let parameter_id_value = parameter_entity_id(&parameter).to_string();
            self.dispatch_apply(vec![OsOperation::AddParameter { parameter }])?;
            Ok(parameter_id_value)
        }

        pub fn patch_parameter(&mut self, target_parameter_id: &str, patch: &OsParameterPatch) -> Result<(), VcsError> {
            let projection = self.projection()?;
            let current = projection.parameters.iter().find(|parameter| parameter_entity_id(parameter) == target_parameter_id).cloned().ok_or_else(|| VcsError::Deserialize(format!("unknown parameter {target_parameter_id}")))?;
            let next = patch_os_parameter(&current, patch);
            self.dispatch_apply(vec![OsOperation::PatchParameter { parameter_id: target_parameter_id.into(), parameter: next }])
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
    //#endregion 🔖️OsStore

    //#region 🔖️Backbone
    /// @emoji 🔌️ Byte-oriented studio persistence port — `read`/`write` carry `encode_os_space_payload`'s
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

    fn sync_os_space_document(document: &OsDocument, backbone_uri: &str, port: &Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
        let mut synced = document.clone();
        synced.backbone = Some(document_backbone_ref(backbone_uri));
        port.write(backbone_uri, &encode_os_space_payload(&synced)?)
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
        pub app_count: usize,
        pub node_count: usize,
        pub updated_at: String,
    }

    static STUDIO_CATALOG_URIS: LazyLock<Mutex<HashMap<usize, HashSet<String>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

    fn port_key(port: &Arc<dyn OsBackbonePort>) -> usize {
        Arc::as_ptr(port) as *const () as usize
    }

    fn track_os_space_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
        STUDIO_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).entry(port_key(port)).or_default().insert(uri.into());
    }

    fn untrack_os_space_backbone_uri(port: &Arc<dyn OsBackbonePort>, uri: &str) {
        if let Some(uris) = STUDIO_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get_mut(&port_key(port)) {
            uris.remove(uri);
        }
    }

    fn os_space_backbone_uri(space_id: &str) -> String {
        format!("{OS_SPACE_BACKBONE_URI_PREFIX}{space_id}")
    }

    fn os_space_id_from_backbone_uri(uri: &str) -> Option<String> {
        uri.strip_prefix(OS_SPACE_BACKBONE_URI_PREFIX).filter(|id| !id.is_empty()).map(str::to_string)
    }

    fn os_space_catalog_entry_from_document(backbone_uri: &str, document: &OsDocument) -> Result<OsSpaceCatalogEntry, VcsError> {
        let space_id = os_space_id_from_backbone_uri(backbone_uri).unwrap_or_else(|| document.id.clone());
        let projection = materialize_os_projection(document, &[])?;
        let updated_at = document.vcs.changes.last().map(|change| change.saved_at.clone()).unwrap_or_else(|| "0".into());
        Ok(OsSpaceCatalogEntry { id: space_id, name: document.name.clone(), backbone_uri: backbone_uri.into(), app_count: projection.app_instances.len(), node_count: projection.workflow.nodes.len(), updated_at })
    }

    /// @emoji 📚️ Lists persisted studio documents from the dev backbone namespace.
    pub fn list_os_space_catalog_entries(port: Arc<dyn OsBackbonePort>) -> Result<Vec<OsSpaceCatalogEntry>, VcsError> {
        let mut entries = Vec::new();
        let uris: Vec<String> = STUDIO_CATALOG_URIS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).get(&port_key(&port)).cloned().unwrap_or_default().into_iter().collect();
        for uri in uris {
            let payload = port.read(&uri)?;
            if payload.is_empty() {
                continue;
            }
            let document = decode_os_space_payload(&payload)?;
            entries.push(os_space_catalog_entry_from_document(&uri, &document)?);
        }
        entries.sort_by(|left, right| right.updated_at.cmp(&left.updated_at).then_with(|| left.name.cmp(&right.name)));
        Ok(entries)
    }

    /// @emoji 🫧️ Mints an empty session-local studio document — no backbone URI and no catalog port write.
    pub fn create_ephemeral_os_space(name: &str) -> OsDocument {
        let id = create_os_id("space");
        create_empty_os_document(&id, name.trim())
    }

    /// @emoji 🆕️ Creates an empty studio document on the dev backbone.
    pub fn create_os_space(name: &str, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let id = create_os_id("space");
        let document = create_empty_os_document(&id, name.trim());
        let backbone_uri = os_space_backbone_uri(&id);
        sync_os_space_document(&document, &backbone_uri, &port)?;
        track_os_space_backbone_uri(&port, &backbone_uri);
        os_space_catalog_entry_from_document(&backbone_uri, &document)
    }

    /// @emoji 🗑️ Deletes a studio document from the dev backbone.
    pub fn delete_os_space(space_id: &str, port: Arc<dyn OsBackbonePort>) -> Result<(), VcsError> {
        let uri = os_space_backbone_uri(space_id);
        untrack_os_space_backbone_uri(&port, &uri);
        port.write(&uri, &[])
    }

    /// @emoji 🌉️ Shared admission tail for `import_os_space_from_dsl`/`import_os_space_from_pack`:
    /// mints a fresh id when the source carried none, syncs, and tracks the catalog uri.
    fn admit_os_space_document(mut document: OsDocument, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let space_id = if document.id.is_empty() { create_os_id("space") } else { document.id.clone() };
        let backbone_uri = os_space_backbone_uri(&space_id);
        document.id = space_id;
        sync_os_space_document(&document, &backbone_uri, &port)?;
        track_os_space_backbone_uri(&port, &backbone_uri);
        os_space_catalog_entry_from_document(&backbone_uri, &document)
    }

    /// @emoji 📥️ Imports a studio document dsl text (`export_os_space_dsl`'s counterpart — the
    /// projection's own dsl only, so the admitted document has no name and no edit history) onto the
    /// dev backbone.
    pub fn import_os_space_from_dsl(dsl: &str, port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let projection = <OsProjection as store::DocumentDsl>::parse_dsl(dsl).map_err(|error| VcsError::Deserialize(error.message))?;
        let vcs = create_document_envelope::<OsProjection, OsOperation>(OS_SPACE_SCHEMA, "", projection, None).vcs;
        admit_os_space_document(OsDocument { schema: OS_SPACE_SCHEMA.into(), id: String::new(), name: String::new(), vcs, applied_edit_ids: Vec::new(), backbone: None }, port)
    }

    /// @emoji 📦️ Pack counterpart of `import_os_space_from_dsl`: decodes `pack`+`spr` directly via the
    /// typed `store::parse_document_pack::<OsProjection, OsOperation>` (this crate is fully typed —
    /// no `store::DocumentCodec` indirection needed), restoring `applied_edit_ids` from the parsed
    /// cursor when present, then follows the identical admission flow.
    pub fn import_os_space_from_pack(pack: &[u8], spr: &[u8], port: Arc<dyn OsBackbonePort>) -> Result<OsSpaceCatalogEntry, VcsError> {
        let parsed: store::ParsedDocumentText<OsProjection, OsOperation> = store::parse_document_pack(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
        let applied_edit_ids = parsed.envelope.cursor.as_ref().map(|cursor| cursor.applied_edit_ids.clone()).unwrap_or_default();
        let document = OsDocument { schema: parsed.envelope.schema, id: parsed.envelope.id, name: String::new(), vcs: parsed.envelope.vcs, applied_edit_ids, backbone: parsed.envelope.backbone };
        admit_os_space_document(document, port)
    }

    /// @emoji 📤️ Exports an already-loaded studio document as pack bytes + ops text. This crate never
    /// constructs `HostEffect` itself (it has no command dispatcher of its own — see
    /// `wave1-progress.txt` in the pack-rollout ticket for why `exportDocumentPack`/`exportDocumentDsl`/
    /// `importDocumentPack` couldn't be wired as literal os/core "commands"); mirrors
    /// `export_os_app_instance_media`'s data-returning shape so `s/plugin`'s `handle_action` can wrap
    /// the result into a `HostEffect::DownloadMediaExport` exactly the way `exportMedia` already wraps
    /// `export_os_app_instance_media`'s `OsMediaExportResult`.
    pub fn export_os_space_pack(document: &OsDocument) -> Result<store::DocumentPackFiles, VcsError> {
        store::print_document_pack(&os_envelope_of(document))
    }

    /// @emoji 📤️ DSL-text counterpart of `export_os_space_pack` — exercises the text export path
    /// (`exportDocumentDsl`) via the same typed `store::print_document_text`.
    pub fn export_os_space_dsl(document: &OsDocument) -> Result<store::DocumentTextFiles, VcsError> {
        store::print_document_text(&os_envelope_of(document))
    }

    /// @emoji 📂️ Loads a studio document from the dev backbone.
    pub fn load_os_space_document(space_id: &str, port: Arc<dyn OsBackbonePort>) -> Result<OsDocument, VcsError> {
        let backbone_uri = os_space_backbone_uri(space_id);
        let payload = port.read(&backbone_uri)?;
        if payload.is_empty() {
            return Err(VcsError::Backbone(format!("unknown os space: {space_id}")));
        }
        decode_os_space_payload(&payload)
    }

    /// @emoji 🌱️ Seeds the demo studio when the catalog is empty.
    pub fn seed_os_space_catalog_if_empty(seed_document: OsDocument, port: Arc<dyn OsBackbonePort>) -> Result<Option<OsSpaceCatalogEntry>, VcsError> {
        if !list_os_space_catalog_entries(port.clone())?.is_empty() {
            return Ok(None);
        }
        let space_id = if seed_document.id.is_empty() { "default".into() } else { seed_document.id.clone() };
        let backbone_uri = os_space_backbone_uri(&space_id);
        let mut seeded = seed_document;
        seeded.id = space_id;
        sync_os_space_document(&seeded, &backbone_uri, &port)?;
        track_os_space_backbone_uri(&port, &backbone_uri);
        Ok(Some(os_space_catalog_entry_from_document(&backbone_uri, &seeded)?))
    }
    //#endregion 🔖️SpaceCatalog

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::workflow::{empty_workflow, placeholder_media_contract, validate_workflow, MediaContract, OsMediaPort};
        use crate::registry::{merge_os_plugin_definition, os_baseline_resource, os_in_port, OsAppResourceSpec, OsPlatformAppInput, OsPlatformInput};
        use semio_framework_core::{MediaClass, MediaForm, MediaType, MediaWireFormat, ModeDefinition, OsMediaFormat, PluginManifest, WindowKindDefinition};
        use std::sync::Arc;
        use ui_wgpu::SurfaceKind;
        use store::{MemoryBackbone, MemoryBackbonePort};

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
                    modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                    default_mode_id: "edit".into(),
                    window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                        id: "composite".into(),
                        label: "Canvas".into(),
                        body_key: "composite".into(),
                        surface_kind: SurfaceKind::Canvas2d,
                        icon_id: "pen-tool".into(),
                        options: ui_wgpu::WindowOptions::default(),
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
                workflows: vec![],
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
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::WindowOptions::default(),
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
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::WindowOptions::default(),
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
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "0.1.0".into(), apps: vec![draw_app.clone()], workflows: vec![], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
                artifact_uri: "program://draw".into(),
            });
            let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
            let generation_before = host.instance(instance_id).expect("instance").generation;
            let event = host.hot_swap_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "0.2.0".into(), apps: vec![draw_app, note_app], workflows: vec![], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
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
                modes: semio_framework_core::Modes::one(ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }),
                default_mode_id: "edit".into(),
                window_kinds: semio_framework_core::WindowKinds::one(WindowKindDefinition {
                    id: "composite".into(),
                    label: "Canvas".into(),
                    body_key: "composite".into(),
                    surface_kind: SurfaceKind::Canvas2d,
                    icon_id: "pen-tool".into(),
                    options: ui_wgpu::WindowOptions::default(),
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
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "0.1.0".into(), apps: vec![draw_app], workflows: vec![], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
                artifact_uri: "program://draw".into(),
            });
            let instance_id = host.create_instance("draw-play", "{}".into()).expect("instance");
            let generation_before = host.instance(instance_id).expect("instance").generation;
            let event = host.hot_swap_plugin(LoadedProgram {
                plugin_id: "draw".into(),
                manifest: PluginManifest { plugin_id: "draw".into(), label: "Draw".into(), version: "".into(), apps: vec![], workflows: vec![], capabilities: vec![], contributions: vec![], examples: vec![], commands: vec![] },
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
                    workflows: vec![],
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
                    workflows: vec![],
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

        fn seed_draw_plugin() {
            let mut resources = HashMap::new();
            resources.insert("draw".into(), os_baseline_resource("2d.drawing", "draw.document", "draw"));
            merge_os_plugin_definition(
                "draw",
                &OsPlatformInput {
                    id: "draw".into(),
                    name: "Draw".into(),
                    api_version: "1".into(),
                    apps: vec![OsPlatformAppInput {
                        id: "draw".into(),
                        label: "Draw".into(),
                        document: vec!["semio".into(), "draw".into()],
                        controller_id: "draw-play".into(),
                        modes: vec![ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }],
                        default_mode_id: None,
                    }],
                },
                &resources,
            )
            .expect("merge");
        }

        /// 🧲️ `draw` is a pure source app (`os_baseline_resource` gives it zero input ports), so tests
        /// that need to wire an edge *into* a spawned instance register this minimal sink alongside it.
        fn seed_sink_plugin() {
            let mut resources = HashMap::new();
            resources.insert(
                "sink".into(),
                OsAppResourceSpec {
                    inputs: vec![os_in_port("2d.drawing", "in", "In", false)],
                    outputs: Vec::new(),
                    source_format: "sink.document".into(),
                    component_kind: "sink".into(),
                    modes: vec![ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }],
                    default_mode_id: None,
                    parameter_fields: Vec::new(),
                    config: semio_framework_core::ConfigSpec::empty(),
                },
            );
            merge_os_plugin_definition(
                "sink",
                &OsPlatformInput {
                    id: "sink".into(),
                    name: "Sink".into(),
                    api_version: "1".into(),
                    apps: vec![OsPlatformAppInput {
                        id: "sink".into(),
                        label: "Sink".into(),
                        document: vec!["semio".into(), "sink".into()],
                        controller_id: "sink-play".into(),
                        modes: vec![ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }],
                        default_mode_id: None,
                    }],
                },
                &resources,
            )
            .expect("merge");
        }

        #[test]
        fn spawns_and_removes_app_instances() {
            seed_draw_plugin();
            let mut store = OsStore::new(create_empty_os_document("space", "Space"));
            store.spawn_app_instance("draw", "draw", None, WorkflowPosition { x: 40.0, y: 40.0 }).expect("spawn");
            assert_eq!(store.projection().expect("projection").app_instances.len(), 1);
            store.dispatch_text("undo").expect("undo");
            assert_eq!(store.projection().expect("projection").app_instances.len(), 0);
        }

        #[test]
        fn adds_and_patches_studio_parameters() {
            let mut store = OsStore::new(create_empty_os_document("space", "Space"));
            let parameter_id = store.add_parameter(&OsParameterType::Numeric, "Zoom").expect("add");
            store.patch_parameter(&parameter_id, &serde_json::json!({ "value": 12.0, "max": 10.0 })).expect("patch");
            match &store.projection().expect("projection").parameters[0] {
                OsParameter::Numeric { value, .. } => assert_eq!(*value, 10.0),
                _ => panic!("expected numeric"),
            }
        }

        #[test]
        fn create_ephemeral_os_space_has_no_backbone() {
            let document = create_ephemeral_os_space("Ephemeral Space");
            assert!(document.id.starts_with("space-"));
            assert_eq!(document.name, "Ephemeral Space");
            assert!(document.backbone.is_none());
            assert!(document.vcs.initial_projection.app_instances.is_empty());
        }

        #[test]
        fn creates_and_lists_studio_catalog_entries() {
            let port = Arc::new(MemoryBackbonePort::new());
            let entry = create_os_space("Catalog Space", port.clone()).expect("create");
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
            let mut store_a = OsStore::new(create_empty_os_document("space", "Space"));
            let node_a_instance = store_a.spawn_app_instance("draw", "draw", None, WorkflowPosition { x: 0.0, y: 0.0 }).expect("spawn a");
            let node_b_instance = store_a.spawn_app_instance("sink", "sink", None, WorkflowPosition { x: 200.0, y: 0.0 }).expect("spawn b");
            let mut store_b = OsStore::new(store_a.document());

            let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://reconcile-race", "mem://reconcile-race");
            store_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
            store_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

            let projection = store_a.projection().expect("projection");
            let node_a = projection.workflow.nodes.iter().find(|node| node.instance_id == node_a_instance).expect("node a");
            let node_b = projection.workflow.nodes.iter().find(|node| node.instance_id == node_b_instance).expect("node b");
            let source_node_id = node_a.id.clone();
            let source_port_id = node_a.outputs.first().expect("node a output port").id.clone();
            let target_node_id = node_b.id.clone();
            let target_port_id = node_b.inputs.first().expect("node b input port").id.clone();

            // 🏃️ Actor A deletes node B; actor B (unaware of the delete) concurrently wires a new edge
            // to a port on node B — the classic delete/wire race `reconcile` must clean up post-merge.
            store_a.dispatch_apply(vec![OsOperation::RemoveAppInstance { instance_id: node_b_instance.clone() }]).expect("remove node b");
            store_b
                .dispatch_apply(vec![OsOperation::ConnectWorkflowPorts {
                    edge: OsWorkflowEdge { id: "edge-race".into(), source_node_id: source_node_id.clone(), source_port_id, target_node_id: target_node_id.clone(), target_port_id, contract: placeholder_media_contract("draw") },
                }])
                .expect("wire edge to node b");
            store_a.tick().expect("pump a");
            store_b.tick().expect("pump b");

            let (converged_a, conflicts_a) = store_a.projection_with_conflicts().expect("projection with conflicts a");
            let (converged_b, conflicts_b) = store_b.projection_with_conflicts().expect("projection with conflicts b");
            assert_eq!(converged_a, converged_b, "both peers must converge on the same reconciled projection");
            assert!(converged_a.workflow.nodes.iter().all(|node| node.instance_id != node_b_instance), "node b must stay removed");
            assert!(converged_a.workflow.edges.iter().all(|edge| edge.target_node_id != target_node_id), "the edge wired to the deleted node must be dropped, not dangling");
            assert!(conflicts_a.iter().any(|conflict| conflict.kind == "workflow/edge-orphaned"), "dropping the dangling edge must surface a conflict");
            assert_eq!(conflicts_a, conflicts_b, "both peers must report the same reconciliation conflicts");
        }

        // 🫀️ The old `presence_upserts_prunes_and_excludes_self` test exercised the deleted `presence:`
        // backbone-URI hack (`write_os_presence`/`read_os_presence_peers`). Presence now flows through
        // the semio_hub's `PresencePeer`/`HubServerFrame::Presence` frames and `framework/sync`'s
        // `DocumentEvent::Presence` — see `framework/product/os/semio_hub/rs/bin.rs` and
        // `framework/sync/rs/lib.rs` for that layer's own coverage.

        // #region 🔖️DslAndOpText
        /// 🧵️ A representative `OsProjection` exercising every collection: two app instances, two media
        /// graph nodes (one with an input port, one with an output port) wired by one edge, one of each
        /// `OsParameter` variant, and one parameter binding — so the DSL round trip actually covers the
        /// workflow encoding, not just an empty-projection fixpoint.
        fn sample_os_projection() -> OsProjection {
            let node_a = OsWorkflowNode {
                id: "node-1".into(),
                instance_id: "app-1".into(),
                x: 0.0,
                y: 0.0,
                width: 220.0,
                height: 92.0,
                inputs: Vec::new(),
                outputs: vec![OsMediaPort { id: "app-1:puzzle.out:out".into(), artifact_kind: "puzzle.2d.fixture".into(), direction: "out".into() }],
            };
            let node_b = OsWorkflowNode {
                id: "node-2".into(),
                instance_id: "app-2".into(),
                x: 240.0,
                y: 0.0,
                width: 220.0,
                height: 92.0,
                inputs: vec![OsMediaPort { id: "app-2:draw.in:in".into(), artifact_kind: "puzzle.2d.fixture".into(), direction: "in".into() }],
                outputs: Vec::new(),
            };
            let edge = OsWorkflowEdge {
                id: "edge-1".into(),
                source_node_id: "node-1".into(),
                source_port_id: "app-1:puzzle.out:out".into(),
                target_node_id: "node-2".into(),
                target_port_id: "app-2:draw.in:in".into(),
                contract: MediaContract { kind_id: "puzzle.2d.fixture".into(), media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, wire: MediaWireFormat::Document { schema: "puzzle.2d.fixture".into() }, conversion: None },
            };
            OsProjection {
                programs: vec!["puzzle".into(), "draw".into()],
                active_plugin_id: Some("puzzle".into()),
                active_alternative_id: Some("alt-1".into()),
                app_instances: vec![
                    OsAppInstance {
                        id: "app-1".into(),
                        plugin_id: "puzzle".into(),
                        app_id: "puzzle2d".into(),
                        label: "Puzzle Board \"3D\"".into(),
                        yields: "puzzle.2d.fixture".into(),
                        document: OsDocumentRef { document_id: "doc-1".into(), schema: "puzzle.2d.fixture".into() },
                        config: Some(serde_json::json!({ "brushSize": 4.0 })),
                    },
                    OsAppInstance {
                        id: "app-2".into(),
                        plugin_id: "draw".into(),
                        app_id: "draw".into(),
                        label: "Draw Sink".into(),
                        yields: "draw.document".into(),
                        document: OsDocumentRef { document_id: "doc-2".into(), schema: "draw.document".into() },
                        config: None,
                    },
                ],
                workflow: OsWorkflow { schema: OS_WORKFLOW_SCHEMA.into(), nodes: vec![node_a, node_b], edges: vec![edge] },
                parameters: vec![
                    OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) },
                    OsParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B, with comma".into()] },
                    OsParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: true },
                    OsParameter::Text { id: "p4".into(), name: "Label".into(), value: "hello \"world\"\nnewline".into() },
                ],
                parameter_bindings: vec![OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "app-1".into(), field_path: "/zoom".into() }],
            }
        }

        /// 📜️ `📚️example/🎬️demo.os` is the handcrafted `.os` DSL-text fixture for `OsProjection` (the
        /// `s.space` studio document) — a two-app workflow (a `cad` scene feeding an `en1995` timber
        /// norm check over a direct `cad.scene` wire) with one of each `OsParameter` variant and two
        /// parameter bindings, so this covers the full grammar `sample_os_projection` exercises, from a
        /// static file instead of only from inline Rust construction.
        #[test]
        fn dsl_round_trips_demo_os_example() {
            let text = include_str!("../../📚️example/🎬️demo.os");
            let projection = <OsProjection as store::DocumentDsl>::parse_dsl(text).expect("🎬️demo.os must parse as OsProjection");
            store::test_support::assert_dsl_round_trip(&projection);
            store::test_support::assert_dsl_pack_equivalence(&projection);
        }

        #[test]
        fn dsl_round_trips_default_projection() {
            store::test_support::assert_dsl_round_trip(&default_os_projection());
            store::test_support::assert_dsl_pack_equivalence(&default_os_projection());
        }

        #[test]
        fn dsl_round_trips_projection_with_workflow_and_parameters() {
            store::test_support::assert_dsl_round_trip(&sample_os_projection());
            store::test_support::assert_dsl_pack_equivalence(&sample_os_projection());
        }

        #[test]
        fn op_text_round_trips_set_active_plugin() {
            store::test_support::assert_op_line_round_trip(&OsOperation::SetActiveProgram { plugin_id: Some("puzzle".into()) });
            store::test_support::assert_op_line_round_trip(&OsOperation::SetActiveProgram { plugin_id: None });
        }

        #[test]
        fn op_text_round_trips_set_active_alternative() {
            store::test_support::assert_op_line_round_trip(&OsOperation::SetActiveAlternative { alternative_id: Some("alt-1".into()) });
            store::test_support::assert_op_line_round_trip(&OsOperation::SetActiveAlternative { alternative_id: None });
        }

        #[test]
        fn op_text_round_trips_spawn_app_instance() {
            store::test_support::assert_op_line_round_trip(&OsOperation::SpawnAppInstance {
                instance: OsAppInstance {
                    id: "app-1".into(),
                    plugin_id: "puzzle".into(),
                    app_id: "puzzle2d".into(),
                    label: "Puzzle Board".into(),
                    yields: "puzzle.2d.fixture".into(),
                    document: OsDocumentRef { document_id: "doc-1".into(), schema: "puzzle.2d.fixture".into() },
                    config: None,
                },
                position: WorkflowPosition { x: 10.0, y: -20.5 },
                node_id: "node-1".into(),
            });
        }

        #[test]
        fn op_text_round_trips_remove_app_instance() {
            store::test_support::assert_op_line_round_trip(&OsOperation::RemoveAppInstance { instance_id: "app-1".into() });
        }

        #[test]
        fn op_text_round_trips_connect_media_ports() {
            store::test_support::assert_op_line_round_trip(&OsOperation::ConnectWorkflowPorts {
                edge: OsWorkflowEdge {
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
            store::test_support::assert_op_line_round_trip(&OsOperation::DisconnectWorkflowEdge { edge_id: "edge-1".into() });
        }

        #[test]
        fn op_text_round_trips_move_media_node() {
            store::test_support::assert_op_line_round_trip(&OsOperation::MoveWorkflowNode { node_id: "node-1".into(), x: 5.5, y: -6.25 });
        }

        #[test]
        fn op_text_round_trips_patch_app_instance() {
            store::test_support::assert_op_line_round_trip(&OsOperation::PatchAppInstance { instance_id: "app-1".into(), label: Some("Renamed \"Board\"".into()) });
            store::test_support::assert_op_line_round_trip(&OsOperation::PatchAppInstance { instance_id: "app-1".into(), label: None });
        }

        #[test]
        fn op_text_round_trips_add_parameter() {
            store::test_support::assert_op_line_round_trip(&OsOperation::AddParameter { parameter: OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 10.0, min: Some(0.0), max: Some(100.0), step: Some(1.0) } });
            store::test_support::assert_op_line_round_trip(&OsOperation::AddParameter { parameter: OsParameter::Categorical { id: "p2".into(), name: "Mode".into(), value: "Option A".into(), options: vec!["Option A".into(), "Option B".into()] } });
            store::test_support::assert_op_line_round_trip(&OsOperation::AddParameter { parameter: OsParameter::Toggle { id: "p3".into(), name: "Flag".into(), value: false } });
            store::test_support::assert_op_line_round_trip(&OsOperation::AddParameter { parameter: OsParameter::Text { id: "p4".into(), name: "Label".into(), value: "hi there".into() } });
        }

        #[test]
        fn op_text_round_trips_remove_parameter() {
            store::test_support::assert_op_line_round_trip(&OsOperation::RemoveParameter { parameter_id: "p1".into() });
        }

        #[test]
        fn op_text_round_trips_patch_parameter() {
            store::test_support::assert_op_line_round_trip(&OsOperation::PatchParameter { parameter_id: "p1".into(), parameter: OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 20.0, min: None, max: None, step: None } });
        }

        #[test]
        fn op_text_round_trips_bind_parameter_field() {
            store::test_support::assert_op_line_round_trip(&OsOperation::BindParameterField { binding: OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "app-1".into(), field_path: "/zoom".into() } });
        }

        #[test]
        fn op_text_round_trips_unbind_parameter_field() {
            store::test_support::assert_op_line_round_trip(&OsOperation::UnbindParameterField { instance_id: "app-1".into(), field_path: "/zoom".into() });
        }

        #[test]
        fn op_text_round_trips_sync_parameter_ports() {
            store::test_support::assert_op_line_round_trip(&OsOperation::SyncParameterPorts);
        }

        #[test]
        fn op_text_round_trips_set_app_instance_config() {
            store::test_support::assert_op_line_round_trip(&OsOperation::SetAppInstanceConfig { instance_id: "app-1".into(), config: Some(serde_json::json!({ "zoom": 2.0, "label": "Roof Beam \"B12\"" })) });
            store::test_support::assert_op_line_round_trip(&OsOperation::SetAppInstanceConfig { instance_id: "app-1".into(), config: None });
        }

        #[test]
        fn document_text_round_trips_store_with_applied_operation() {
            let envelope = create_document_envelope(OS_SPACE_SCHEMA, "space-text-test", default_os_projection(), None);
            let mut store = DocumentStore::new(envelope);
            store
                .dispatch(DocumentCommand::Apply { operations: vec![OsOperation::SetActiveProgram { plugin_id: Some("puzzle".into()) }], description: None })
                .expect("apply");
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
    use crate::workflow::OS_SPACE_SCHEMA;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::{OsEnvelope, OsOperation, OsProjection};
    #[cfg(not(target_arch = "wasm32"))]
    use std::sync::Arc;
    use vcs::VcsError;
    use store::{DocumentDsl, MemoryBackbonePort};
    #[cfg(not(target_arch = "wasm32"))]
    use store_sync::{FolderSqliteStorage, FolderTextStorage};

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
                                    let projection = <OsProjection as store::DocumentDsl>::parse_dsl(&text_files.dsl).map_err(|error| VcsError::Deserialize(error.message))?;
                                    let envelope = store::create_document_envelope::<OsProjection, OsOperation>(OS_SPACE_SCHEMA, document_id, projection, None);
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
                        let parsed: store::ParsedDocumentText<OsProjection, OsOperation> = store::parse_document_pack(&pack, &spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                        let dsl_mirror = store::DocumentDsl::print_dsl(&parsed.envelope.vcs.initial_projection);
                        let pack_files = store::DocumentPackFiles { pack, spr, ops: String::new() };
                        return storage.write_pack(document_id, extension, &pack_files, &dsl_mirror);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    SpacePortKind::Folder(folder_uri, storage) if uri == folder_uri => {
                        let (pack, spr) = decode_os_space_pack_payload(payload)?;
                        return storage.write(SPACE_FOLDER_DOCUMENT_ID, OS_SPACE_SCHEMA, &pack, &spr);
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
    use store_sync::{DocumentActorConfig, DocumentActorMsg, DocumentChannels, DocumentEvent, DocumentHost, PersistenceBinding};

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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsAppInstance {
        pub id: String,
        pub plugin_id: String,
        pub app_id: String,
        pub label: String,
        pub yields: String,
        #[dsl(block)]
        pub document: OsDocumentRef,
        /// 🧮️ Dynamic wire-value-encodable config for this instance, validated against the app's
        /// declared `ConfigSpec` (`AppDefinition.config`) at apply time (`build_configure_config`) —
        /// `None` means "use the app's own config defaults". Mirrors the `serde_json::Value`
        /// schema-less-escape-hatch convention this crate's `dsl_derive` already uses elsewhere (see
        /// `dsl::DslField for serde_json::Value`) rather than `dsl_schema::DslValue` directly, which
        /// has no `DslField` impl of its own. New field appended last per this struct's `dsl_derive`
        /// field-id-by-declaration-order convention (field ids are never renumbered) — see
        /// `dsl_derive::plan_fields`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<Value>,
    }

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
        pub instance_id: String,
        /// 🎯️ Names a `ConfigFieldSpec.key` in the target `instance`'s app's declared `ConfigSpec`
        /// (resolved via `registry::os_app_registration(instance.plugin_id, instance.app_id).config`)
        /// — see `validate_parameter_config_binding` (type-checks this against the field's
        /// `ConfigFieldShape`) and `build_configure_config` (overlays the bound parameter's value onto
        /// that config field for an `AppCommand::Configure` payload). Historically a JSON pointer into
        /// the instance's live document (`apply_parameter_values_to_projection`'s still-live overlay,
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
    pub fn apply_parameter_values_to_projection(projection: Value, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], instance_id: &str) -> Value {
        let instance_bindings: Vec<_> = bindings.iter().filter(|binding| binding.instance_id == instance_id).collect();
        if instance_bindings.is_empty() {
            return projection;
        }
        let mut clone = projection;
        for binding in instance_bindings {
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
        let uri = format!("{}#{}", binding.instance_id, binding.field_path);
        let Some(field) = config_spec.fields.iter().find(|field| field.key == binding.field_path) else {
            return Err(SpaceConflict {
                kind: "workflow/parameter-binding-invalid".into(),
                uri,
                message: format!("binding targets config field '{}', which the app's ConfigSpec does not declare", binding.field_path),
            });
        };
        let compatible = matches!(
            (parameter_type, &field.shape),
            (OsParameterType::Numeric, ConfigFieldShape::Number { .. }) | (OsParameterType::Categorical, ConfigFieldShape::Select { .. }) | (OsParameterType::Toggle, ConfigFieldShape::Toggle) | (OsParameterType::Text, ConfigFieldShape::Text)
        );
        if compatible {
            Ok(())
        } else {
            Err(SpaceConflict {
                kind: "workflow/parameter-binding-invalid".into(),
                uri,
                message: format!("parameter type {parameter_type:?} cannot drive config field '{}' ({:?})", binding.field_path, field.shape),
            })
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

    /// @emoji 🎛️ Resolves bound parameter values for an app instance as a field-path map.
    pub fn resolve_parameter_values_for_instance(bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], instance_id: &str) -> HashMap<String, Value> {
        let mut values = HashMap::new();
        for binding in bindings.iter().filter(|entry| entry.instance_id == instance_id) {
            let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
                continue;
            };
            values.insert(binding.field_path.clone(), os_parameter_value(parameter));
        }
        values
    }

    /// @emoji 🎛️ Builds the workflow input port id for a bound space parameter.
    pub fn parameter_port_id(instance_id: &str, parameter_id: &str) -> String {
        media_port_id_for_spec(instance_id, &format!("{OS_PARAMETER_PORT_PREFIX}{parameter_id}"), "in")
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
                defaults.insert(field.key.clone(), default.clone());
            }
        }
        Value::Object(defaults)
    }

    /// @emoji 🧩️ Builds the dynamic config value for an `AppCommand::Configure` payload: starts from
    /// `instance.config` (falling back to the app's own `ConfigSpec` defaults when `None`), then
    /// overlays every parameter bound to one of `config_spec`'s fields with that parameter's current
    /// value — the config-driving counterpart to `apply_parameter_values_to_projection`'s
    /// document-JSON-pointer overlay (see `OsParameterFieldBinding::field_path`'s doc for how the two
    /// diverge). Callers (the renderer/headless-runner drivers dispatching `AppCommand::Configure`,
    /// both out of this crate's scope) `store::pack_rt::encode_wire_value` the result themselves —
    /// this function only builds the value, it never sends anything over a channel.
    pub fn build_configure_config(instance: &OsAppInstance, parameters: &[OsParameter], bindings: &[OsParameterFieldBinding], config_spec: &ConfigSpec) -> Value {
        let mut config = instance.config.clone().unwrap_or_else(|| config_spec_default_value(config_spec));
        if !config.is_object() {
            config = Value::Object(serde_json::Map::new());
        }
        // infallible: forced to `Value::Object(_)` immediately above.
        let object = config.as_object_mut().expect("config is always an object here");
        for binding in bindings.iter().filter(|binding| binding.instance_id == instance.id) {
            let Some(field) = config_spec.fields.iter().find(|field| field.key == binding.field_path) else {
                continue;
            };
            let Some(parameter) = parameters.iter().find(|entry| entry.id() == binding.parameter_id) else {
                continue;
            };
            object.insert(field.key.clone(), os_parameter_value(parameter));
        }
        config
    }

    /// @emoji 🧩️ Overlays bound parameter values onto an app instance's current document projection.
    /// Content itself lives in the app's own `framework/sync`-hosted document (referenced by
    /// {@link OsDocumentRef}, read host-side and passed in as `current_document_json`) — this function
    /// no longer resolves embedded/upstream source documents; that concept was deleted with
    /// `OsSourceDocument`. Cross-instance ("upstream") dataflow through workflow edges is deferred
    /// (see `host_runtime` doc-comment) to a follow-up that reads the upstream app's live document.
    pub fn materialize_os_app_instance_document_json(current_document_json: &str, instance_id: &str, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter]) -> String {
        let projection: Value = serde_json::from_str(current_document_json).unwrap_or_else(|_| json!({}));
        let with_params = apply_parameter_values_to_projection(projection, bindings, parameters, instance_id);
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
    pub fn app_instance_document_patches_for_binding(parameter_id: &str, instances: &[OsAppInstance], bindings: &[OsParameterFieldBinding], parameters: &[OsParameter], current_document_json: impl Fn(&str) -> Option<String>) -> Vec<(String, String)> {
        let bound_instance_ids: HashSet<String> = bindings.iter().filter(|binding| binding.parameter_id == parameter_id).map(|binding| binding.instance_id.clone()).collect();
        instances
            .iter()
            .filter(|instance| bound_instance_ids.contains(&instance.id))
            .filter_map(|instance| {
                let current_json = current_document_json(&instance.document.document_id)?;
                let patched = materialize_os_app_instance_document_json(&current_json, &instance.id, bindings, parameters);
                Some((instance.document.document_id.clone(), patched))
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
                &[OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "i1".into(), field_path: "/brushSize".into() }],
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
                    semio_framework_core::ConfigFieldSpec { key: "zoom".into(), label: "Zoom".into(), shape: ConfigFieldShape::Number { min: None, max: None, step: None }, default: Some(serde_json::json!(1.0)) },
                    semio_framework_core::ConfigFieldSpec { key: "mode".into(), label: "Mode".into(), shape: ConfigFieldShape::Select { options: vec!["A".into(), "B".into()] }, default: Some(serde_json::json!("A")) },
                    semio_framework_core::ConfigFieldSpec { key: "flag".into(), label: "Flag".into(), shape: ConfigFieldShape::Toggle, default: None },
                    semio_framework_core::ConfigFieldSpec { key: "label".into(), label: "Label".into(), shape: ConfigFieldShape::Text, default: None },
                ],
            }
        }

        #[test]
        fn validates_matching_parameter_config_bindings() {
            let config_spec = sample_config_spec();
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "i1".into(), field_path: "zoom".into() }, &OsParameterType::Numeric, &config_spec).is_ok());
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p2".into(), instance_id: "i1".into(), field_path: "mode".into() }, &OsParameterType::Categorical, &config_spec).is_ok());
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p3".into(), instance_id: "i1".into(), field_path: "flag".into() }, &OsParameterType::Toggle, &config_spec).is_ok());
            assert!(validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p4".into(), instance_id: "i1".into(), field_path: "label".into() }, &OsParameterType::Text, &config_spec).is_ok());
        }

        #[test]
        fn rejects_mismatched_parameter_config_bindings() {
            let config_spec = sample_config_spec();
            let mismatch = validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "i1".into(), field_path: "zoom".into() }, &OsParameterType::Toggle, &config_spec).expect_err("toggle cannot drive a Number field");
            assert_eq!(mismatch.kind, "workflow/parameter-binding-invalid");
            let mismatch = validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p2".into(), instance_id: "i1".into(), field_path: "mode".into() }, &OsParameterType::Text, &config_spec).expect_err("text cannot drive a Select field");
            assert_eq!(mismatch.kind, "workflow/parameter-binding-invalid");
        }

        #[test]
        fn rejects_parameter_config_binding_to_unknown_field() {
            let config_spec = sample_config_spec();
            let error = validate_parameter_config_binding(&OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "i1".into(), field_path: "nonexistent".into() }, &OsParameterType::Numeric, &config_spec).expect_err("field does not exist on the ConfigSpec");
            assert_eq!(error.kind, "workflow/parameter-binding-invalid");
        }

        #[test]
        fn build_configure_config_starts_from_config_spec_defaults() {
            let config_spec = sample_config_spec();
            let instance = OsAppInstance { id: "i1".into(), plugin_id: "p".into(), app_id: "a".into(), label: "A".into(), yields: "a.document".into(), document: OsDocumentRef { document_id: "d1".into(), schema: "a.document".into() }, config: None };
            let config = build_configure_config(&instance, &[], &[], &config_spec);
            assert_eq!(config["zoom"], 1.0);
            assert_eq!(config["mode"], "A");
        }

        #[test]
        fn build_configure_config_overlays_bound_parameter_values() {
            let config_spec = sample_config_spec();
            let instance = OsAppInstance { id: "i1".into(), plugin_id: "p".into(), app_id: "a".into(), label: "A".into(), yields: "a.document".into(), document: OsDocumentRef { document_id: "d1".into(), schema: "a.document".into() }, config: Some(serde_json::json!({ "zoom": 1.0, "mode": "A" })) };
            let parameters = vec![OsParameter::Numeric { id: "p1".into(), name: "Zoom".into(), value: 42.0, min: None, max: None, step: None }];
            let bindings = vec![OsParameterFieldBinding { parameter_id: "p1".into(), instance_id: "i1".into(), field_path: "zoom".into() }];
            let config = build_configure_config(&instance, &parameters, &bindings, &config_spec);
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
    pub fn dwg_drawing_to_svg(drawing: &semio_framework_core::DwgDrawing) -> Result<(String, u32, u32), String> {
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
    type SolidExporterRegistry = HashMap<String, Box<dyn kernel_3d_brepkit::SolidExporter>>;

    fn solid_exporters() -> &'static Mutex<SolidExporterRegistry> {
        static HANDLERS: OnceLock<Mutex<SolidExporterRegistry>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    type SolidImporterRegistry = HashMap<String, Box<dyn kernel_3d_brepkit::SolidImporter>>;

    fn solid_importers() -> &'static Mutex<SolidImporterRegistry> {
        static HANDLERS: OnceLock<Mutex<SolidImporterRegistry>> = OnceLock::new();
        HANDLERS.get_or_init(|| Mutex::new(HashMap::new()))
    }

    fn solid_registry_key(artifact_kind: &str, format: &OsMediaFormat) -> String {
        format!("{}:{}", artifact_kind, format.as_str())
    }

    /// @emoji 🧊️ Registers a B-Rep solid exporter (STEP/STL/OBJ/GLB, operating on `GeometryHandle` via `kernel_3d_brepkit::BrepkitKernel` rather than a tessellated `MeshData`) for a resource kind; call once per format.
    pub fn register_solid_exporter(artifact_kind: &str, exporter: Box<dyn kernel_3d_brepkit::SolidExporter>) {
        let key = solid_registry_key(artifact_kind, &exporter.format());
        solid_exporters().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key, exporter);
    }

    /// @emoji 🧊️ Registers a B-Rep solid importer for a resource kind; see `register_solid_exporter`.
    pub fn register_solid_importer(artifact_kind: &str, importer: Box<dyn kernel_3d_brepkit::SolidImporter>) {
        let key = solid_registry_key(artifact_kind, &importer.format());
        solid_importers().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(key, importer);
    }

    /// @emoji 🧊️ Looks up a previously registered solid exporter for a resource kind + format.
    pub fn solid_exporter_for(artifact_kind: &str, format: &OsMediaFormat) -> bool {
        solid_exporters().lock().unwrap_or_else(std::sync::PoisonError::into_inner).contains_key(&solid_registry_key(artifact_kind, format))
    }

    /// @emoji 🧊️ Exports `shapes` from `kernel` through the solid exporter registered for `artifact_kind` + `format`.
    pub fn export_registered_solid(artifact_kind: &str, format: &OsMediaFormat, kernel: &kernel_3d_brepkit::BrepkitKernel, shapes: &[kernel_3d_engine::GeometryHandle], deflection: f64) -> Result<Vec<u8>, String> {
        let key = solid_registry_key(artifact_kind, format);
        let handlers = solid_exporters().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let exporter = handlers.get(&key).ok_or_else(|| format!("no solid export handler for {key}"))?;
        exporter.export(kernel, shapes, deflection).map_err(|error| error.to_string())
    }

    /// @emoji 🧊️ Imports bytes into `kernel` through the solid importer registered for `artifact_kind` + `format`.
    pub fn import_registered_solid(artifact_kind: &str, format: &OsMediaFormat, kernel: &mut kernel_3d_brepkit::BrepkitKernel, data: &[u8], tolerance: f64) -> Result<Vec<kernel_3d_engine::GeometryHandle>, String> {
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

    pub use workflow::workflow_node_for_app;

    use crate::host::OsOperation;
    use crate::instance::{create_os_id, is_parameter_port_id, media_port_spec_id, parameter_id_from_port_id, parameter_port_id, OsAppInstance, OsParameter, OsParameterFieldBinding};
    use crate::registry::{os_app_primary_output_kind, os_app_registration, os_artifact_descriptor, OsAppRegistration, OsArtifactDescriptor};
    use semio_framework_core::{media_types_compatible, MediaClass, MediaCompat, MediaForm, MediaType, MediaWireFormat};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::collections::{HashMap, HashSet};
    use std::sync::{Mutex, OnceLock};

    pub const OS_SPACE_SCHEMA: &str = "s.space";
    pub const OS_WORKFLOW_SCHEMA: &str = "s.workflow";
    pub const OS_WORKFLOW_VFS_ROOT_ID: &str = "os-workflow-root";
    pub const OS_MEDIA_FLOW_MODULE_ID: &str = "os-media";

    //#region 🔖️Workflow
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsMediaPort {
        pub id: String,
        pub artifact_kind: String,
        pub direction: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowNode {
        pub id: String,
        pub instance_id: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        pub inputs: Vec<OsMediaPort>,
        pub outputs: Vec<OsMediaPort>,
    }

    //#region 🔖️MediaContract
    /// 🤝️ A connect-time negotiated wire contract between two `OsMediaPort`s — produced by
    /// `negotiate_media_contract` and stored on `OsWorkflowEdge` so later passes (`validate_workflow`,
    /// merge reconciliation) can re-check it without re-resolving the artifact registry. `kind_id`/`media_type`
    /// describe the *accepted* (target) side — see `semio_framework_core::media_types_compatible`.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct MediaContract {
        pub kind_id: String,
        pub media_type: MediaType,
        pub wire: MediaWireFormat,
        pub conversion: Option<(MediaForm, MediaForm)>,
    }

    /// @emoji 🤝️ Negotiates the wire contract for connecting `source_port` (a producer/output) to
    /// `target_port` (a consumer/input): resolves each port's `OsArtifactDescriptor` from the resource
    /// registry, checks `media_types_compatible`, and picks a shared wire format. `Err` means the connect
    /// must be rejected outright (see the `s::plugin` connect handlers).
    pub fn negotiate_media_contract(source_port: &OsMediaPort, target_port: &OsMediaPort) -> Result<MediaContract, String> {
        let source_descriptor = os_artifact_descriptor(&source_port.artifact_kind);
        let target_descriptor = os_artifact_descriptor(&target_port.artifact_kind);
        let conversion = match media_types_compatible(&source_descriptor.media_type, &target_descriptor.media_type) {
            MediaCompat::Direct => None,
            MediaCompat::Convert { from, to } => Some((from, to)),
            MediaCompat::Reject => {
                return Err(format!(
                    "cannot connect `{}` ({:?}/{:?}) to `{}` ({:?}/{:?}): incompatible media types",
                    source_port.artifact_kind, source_descriptor.media_type.class, source_descriptor.media_type.form, target_port.artifact_kind, target_descriptor.media_type.class, target_descriptor.media_type.form
                ));
            }
        };
        let wire = negotiate_wire_format(&source_descriptor, &target_descriptor)
            .ok_or_else(|| format!("cannot connect `{}` to `{}`: no shared wire format", source_port.artifact_kind, target_port.artifact_kind))?;
        Ok(MediaContract { kind_id: target_descriptor.kind.clone(), media_type: target_descriptor.media_type, wire, conversion })
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

    /// 🧪️ Placeholder contract for test/fixture edges built without a real port-negotiation context — mirrors
    /// `registry::os_artifact_descriptor`'s unregistered-kind fallback (`Data`/`Value`, schema pinned to
    /// `kind_id` itself).
    pub fn placeholder_media_contract(kind_id: &str) -> MediaContract {
        MediaContract { kind_id: kind_id.into(), media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, wire: MediaWireFormat::Document { schema: kind_id.into() }, conversion: None }
    }
    //#endregion 🔖️MediaContract

    //#region 🔖️MediaContractDsl
    /// 🧬️ Hand-crafted `dsl::DslField` for `MediaContract` (instead of `#[derive(dsl::DslRecord)]`) —
    /// see the `dsl::` conversion cheat sheet's tuple-field guidance. `conversion: Option<(MediaForm,
    /// MediaForm)>` has no derivable shape (raw Rust tuples don't implement `dsl::DslField`), and
    /// `media_type`/`wire` point at plain-data types from `semio_framework_core` that this crate can't
    /// implement `dsl::DslField` for under the orphan rule (neither the trait nor the type is local
    /// here). Since `MediaContract` itself IS local, hand-writing its own impl sidesteps both problems
    /// at once: every foreign sub-value (`MediaClass`/`MediaForm`/`OsMediaFormat`) is bridged directly
    /// to/from a scalar `dsl::FieldValue::Enum`/`Ident` right here, so none of them ever need their own
    /// `DslField` impl or a local-twin type. `media_contract_spec()`'s `keyword: None` makes
    /// `Shape::Record` splice these eight fields inline wherever `MediaContract` is used as a
    /// `#[dsl(block)]` field (see `OsWorkflowEdge.contract`), with no keyword of its own repeated
    /// inside the braces.
    fn media_class_ordinal(class: MediaClass) -> u32 {
        match class {
            MediaClass::TwoD => 0,
            MediaClass::ThreeD => 1,
            MediaClass::Text => 2,
            MediaClass::Data => 3,
            MediaClass::Graph => 4,
            MediaClass::Kit => 5,
            MediaClass::Computation => 6,
            MediaClass::Presentation => 7,
        }
    }

    fn media_class_from_ordinal(ordinal: u32) -> Result<MediaClass, String> {
        Ok(match ordinal {
            0 => MediaClass::TwoD,
            1 => MediaClass::ThreeD,
            2 => MediaClass::Text,
            3 => MediaClass::Data,
            4 => MediaClass::Graph,
            5 => MediaClass::Kit,
            6 => MediaClass::Computation,
            7 => MediaClass::Presentation,
            other => return Err(format!("unknown media class ordinal {other}")),
        })
    }

    fn media_class_variants() -> Vec<(String, u32)> {
        vec![
            ("twoD".to_string(), 0),
            ("threeD".to_string(), 1),
            ("text".to_string(), 2),
            ("data".to_string(), 3),
            ("graph".to_string(), 4),
            ("kit".to_string(), 5),
            ("computation".to_string(), 6),
            ("presentation".to_string(), 7),
        ]
    }

    fn media_form_ordinal(form: MediaForm) -> u32 {
        match form {
            MediaForm::Any => 0,
            MediaForm::Vector => 1,
            MediaForm::Raster => 2,
            MediaForm::Brep => 3,
            MediaForm::Mesh => 4,
            MediaForm::Document => 5,
            MediaForm::Value => 6,
            MediaForm::Dag => 7,
            MediaForm::Trinity => 8,
            MediaForm::Type => 9,
            MediaForm::Design => 10,
            MediaForm::Kit => 11,
            MediaForm::Flow => 12,
            MediaForm::Sequence => 13,
            MediaForm::Imperative => 14,
            MediaForm::Deck => 15,
        }
    }

    fn media_form_from_ordinal(ordinal: u32) -> Result<MediaForm, String> {
        Ok(match ordinal {
            0 => MediaForm::Any,
            1 => MediaForm::Vector,
            2 => MediaForm::Raster,
            3 => MediaForm::Brep,
            4 => MediaForm::Mesh,
            5 => MediaForm::Document,
            6 => MediaForm::Value,
            7 => MediaForm::Dag,
            8 => MediaForm::Trinity,
            9 => MediaForm::Type,
            10 => MediaForm::Design,
            11 => MediaForm::Kit,
            12 => MediaForm::Flow,
            13 => MediaForm::Sequence,
            14 => MediaForm::Imperative,
            15 => MediaForm::Deck,
            other => return Err(format!("unknown media form ordinal {other}")),
        })
    }

    fn media_form_variants() -> Vec<(String, u32)> {
        vec![
            ("any".to_string(), 0),
            ("vector".to_string(), 1),
            ("raster".to_string(), 2),
            ("brep".to_string(), 3),
            ("mesh".to_string(), 4),
            ("document".to_string(), 5),
            ("value".to_string(), 6),
            ("dag".to_string(), 7),
            ("trinity".to_string(), 8),
            ("type".to_string(), 9),
            ("design".to_string(), 10),
            ("kit".to_string(), 11),
            ("flow".to_string(), 12),
            ("sequence".to_string(), 13),
            ("imperative".to_string(), 14),
            ("deck".to_string(), 15),
        ]
    }

    fn media_contract_spec() -> dsl::RecordSpec {
        dsl::RecordSpec::new(
            None,
            dsl::RecordLayout::Inline,
            vec![
                dsl::FieldSpec::new(0, "kind_id", dsl::Shape::Text),
                dsl::FieldSpec::new(1, "class", dsl::Shape::Enum(media_class_variants())),
                dsl::FieldSpec::new(2, "form", dsl::Shape::Enum(media_form_variants())),
                dsl::FieldSpec::new(3, "wire_kind", dsl::Shape::Text),
                dsl::FieldSpec::new(4, "wire_format", dsl::Shape::Text).optional(),
                dsl::FieldSpec::new(5, "wire_schema", dsl::Shape::Text).optional(),
                dsl::FieldSpec::new(6, "conversion_from", dsl::Shape::Enum(media_form_variants())).optional(),
                dsl::FieldSpec::new(7, "conversion_to", dsl::Shape::Enum(media_form_variants())).optional(),
            ],
        )
    }

    fn media_contract_to_record(contract: &MediaContract) -> dsl::RecordValue {
        let mut record = dsl::RecordValue::default();
        record.fields.insert(0, dsl::FieldValue::Text(contract.kind_id.clone()));
        record.fields.insert(1, dsl::FieldValue::Enum(media_class_ordinal(contract.media_type.class)));
        record.fields.insert(2, dsl::FieldValue::Enum(media_form_ordinal(contract.media_type.form)));
        match &contract.wire {
            MediaWireFormat::Binary { format } => {
                record.fields.insert(3, dsl::FieldValue::Text("binary".to_string()));
                record.fields.insert(4, dsl::FieldValue::Text(format.as_str().to_string()));
                record.fields.insert(5, dsl::FieldValue::Absent);
            }
            MediaWireFormat::Document { schema } => {
                record.fields.insert(3, dsl::FieldValue::Text("document".to_string()));
                record.fields.insert(4, dsl::FieldValue::Absent);
                record.fields.insert(5, dsl::FieldValue::Text(schema.clone()));
            }
        }
        match contract.conversion {
            Some((from, to)) => {
                record.fields.insert(6, dsl::FieldValue::Enum(media_form_ordinal(from)));
                record.fields.insert(7, dsl::FieldValue::Enum(media_form_ordinal(to)));
            }
            None => {
                record.fields.insert(6, dsl::FieldValue::Absent);
                record.fields.insert(7, dsl::FieldValue::Absent);
            }
        }
        record
    }

    fn media_contract_from_record(record: &dsl::RecordValue) -> Result<MediaContract, store::TextError> {
        let kind_id = match record.get(0) {
            Some(dsl::FieldValue::Text(s)) => s.clone(),
            other => return Err(dsl::__rt::field_error(format!("expected kind_id, found {other:?}"))),
        };
        let class = match record.get(1) {
            Some(dsl::FieldValue::Enum(ordinal)) => media_class_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
            other => return Err(dsl::__rt::field_error(format!("expected class, found {other:?}"))),
        };
        let form = match record.get(2) {
            Some(dsl::FieldValue::Enum(ordinal)) => media_form_from_ordinal(*ordinal).map_err(dsl::__rt::field_error)?,
            other => return Err(dsl::__rt::field_error(format!("expected form, found {other:?}"))),
        };
        let wire_kind = match record.get(3) {
            Some(dsl::FieldValue::Text(s)) => s.clone(),
            other => return Err(dsl::__rt::field_error(format!("expected wire_kind, found {other:?}"))),
        };
        let wire = match wire_kind.as_str() {
            "binary" => {
                let format_word = match record.get(4) {
                    Some(dsl::FieldValue::Text(s)) => s.clone(),
                    other => return Err(dsl::__rt::field_error(format!("expected wire_format, found {other:?}"))),
                };
                let format = semio_framework_core::OsMediaFormat::parse(&format_word).ok_or_else(|| dsl::__rt::field_error(format!("unknown wire format '{format_word}'")))?;
                MediaWireFormat::Binary { format }
            }
            "document" => {
                let schema = match record.get(5) {
                    Some(dsl::FieldValue::Text(s)) => s.clone(),
                    other => return Err(dsl::__rt::field_error(format!("expected wire_schema, found {other:?}"))),
                };
                MediaWireFormat::Document { schema }
            }
            other => return Err(dsl::__rt::field_error(format!("unknown wire kind '{other}'"))),
        };
        let conversion = match (record.get(6), record.get(7)) {
            (Some(dsl::FieldValue::Enum(from)), Some(dsl::FieldValue::Enum(to))) => Some((media_form_from_ordinal(*from).map_err(dsl::__rt::field_error)?, media_form_from_ordinal(*to).map_err(dsl::__rt::field_error)?)),
            _ => None,
        };
        Ok(MediaContract { kind_id, media_type: MediaType { class, form }, wire, conversion })
    }

    impl dsl::DslField for MediaContract {
        fn shape() -> dsl::Shape {
            dsl::Shape::Record(media_contract_spec)
        }
        fn to_value(&self) -> dsl::FieldValue {
            dsl::FieldValue::Record(media_contract_to_record(self))
        }
        fn from_value(value: &dsl::FieldValue) -> Result<Self, String> {
            match value {
                dsl::FieldValue::Record(record) => media_contract_from_record(record).map_err(|e| e.message),
                other => Err(format!("expected Record, found {other:?}")),
            }
        }
    }
    //#endregion 🔖️MediaContractDsl

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowEdge {
        pub id: String,
        pub source_node_id: String,
        pub source_port_id: String,
        pub target_node_id: String,
        pub target_port_id: String,
        #[dsl(block)]
        pub contract: MediaContract,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflow {
        pub schema: String,
        #[dsl(table)]
        pub nodes: Vec<OsWorkflowNode>,
        #[dsl(table)]
        pub edges: Vec<OsWorkflowEdge>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct WorkflowPosition {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct WorkflowValidation {
        pub ok: bool,
        pub errors: Vec<String>,
    }

    pub fn empty_workflow() -> OsWorkflow {
        OsWorkflow { schema: OS_WORKFLOW_SCHEMA.into(), nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn workflow_node_for_instance(instance: &OsAppInstance, registration: &OsAppRegistration, position: &WorkflowPosition, node_id: &str) -> OsWorkflowNode {
        let (inputs, outputs) = crate::registry::workflow_node_ports_for_registration(&instance.id, registration);
        let port_count = inputs.len().max(outputs.len()).max(1);
        OsWorkflowNode { id: node_id.into(), instance_id: instance.id.clone(), x: position.x, y: position.y, width: 220.0, height: 56.0 + port_count as f64 * 18.0, inputs, outputs }
    }

    fn sync_workflow_node_parameter_ports(node: &OsWorkflowNode, bindings: &[OsParameterFieldBinding]) -> OsWorkflowNode {
        let instance_bindings: Vec<_> = bindings.iter().filter(|binding| binding.instance_id == node.instance_id).collect();
        let base_inputs: Vec<_> = node.inputs.iter().filter(|port| !is_parameter_port_id(&port.id)).cloned().collect();
        let parameter_inputs: Vec<_> = instance_bindings.iter().map(|binding| OsMediaPort { id: parameter_port_id(&node.instance_id, &binding.parameter_id), artifact_kind: "parameter.value".into(), direction: "in".into() }).collect();
        let inputs: Vec<_> = base_inputs.into_iter().chain(parameter_inputs).collect();
        let port_count = inputs.len().max(node.outputs.len()).max(1);
        OsWorkflowNode { inputs, height: 56.0 + port_count as f64 * 18.0, ..node.clone() }
    }

    pub fn sync_workflow_parameter_ports(graph: &OsWorkflow, bindings: &[OsParameterFieldBinding]) -> OsWorkflow {
        OsWorkflow { schema: OS_WORKFLOW_SCHEMA.into(), nodes: graph.nodes.iter().map(|node| sync_workflow_node_parameter_ports(node, bindings)).collect(), edges: graph.edges.clone() }
    }

    /// @emoji ✅️ Validates workflow connectivity, cycle freedom, and edge-contract consistency.
    pub fn validate_workflow(graph: &OsWorkflow) -> WorkflowValidation {
        let mut errors = Vec::new();
        let node_ids: HashSet<_> = graph.nodes.iter().map(|node| node.id.clone()).collect();
        for edge in &graph.edges {
            if !node_ids.contains(&edge.source_node_id) {
                errors.push(format!("missing source node {}", edge.source_node_id));
            }
            if !node_ids.contains(&edge.target_node_id) {
                errors.push(format!("missing target node {}", edge.target_node_id));
            }
        }

        //#region ContractConsistency
        // 🛡️ Defense in depth for merged/imported studio documents: re-negotiate each edge's endpoints
        // against the *current* artifact registry and flag any edge whose stored `contract` no longer
        // matches — a concurrent re-typing or a stale import can leave a wire's contract behind.
        let node_by_id: HashMap<&str, &OsWorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        for edge in &graph.edges {
            let Some(source_port) = node_by_id.get(edge.source_node_id.as_str()).and_then(|node| node.outputs.iter().find(|port| port.id == edge.source_port_id)) else { continue };
            let Some(target_port) = node_by_id.get(edge.target_node_id.as_str()).and_then(|node| node.inputs.iter().find(|port| port.id == edge.target_port_id)) else { continue };
            match negotiate_media_contract(source_port, target_port) {
                Ok(contract) if contract == edge.contract => {}
                Ok(_) => errors.push(format!("edge {} contract stale: no longer matches negotiated port types", edge.id)),
                Err(reason) => errors.push(format!("edge {} contract invalid: {reason}", edge.id)),
            }
        }
        //#endregion ContractConsistency

        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &graph.edges {
            adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
        }
        let mut visiting = HashSet::new();
        let mut visited = HashSet::new();
        fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visiting: &mut HashSet<String>, visited: &mut HashSet<String>, errors: &mut Vec<String>) {
            if visited.contains(node_id) {
                return;
            }
            if visiting.contains(node_id) {
                errors.push(format!("cycle detected at {node_id}"));
                return;
            }
            visiting.insert(node_id.to_string());
            for next in adjacency.get(node_id).into_iter().flatten() {
                dfs(next, adjacency, visiting, visited, errors);
            }
            visiting.remove(node_id);
            visited.insert(node_id.to_string());
        }
        for node in &graph.nodes {
            dfs(&node.id, &adjacency, &mut visiting, &mut visited, &mut errors);
        }
        WorkflowValidation { ok: errors.is_empty(), errors }
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

    pub fn os_workflow_to_flow_fixture(graph: &OsWorkflow, instances: &[OsAppInstance], camera: &OsWorkflowCamera) -> Value {
        let instance_by_id: HashMap<_, _> = instances.iter().map(|instance| (instance.id.clone(), instance)).collect();
        let widgets: Vec<_> = graph
            .nodes
            .iter()
            .map(|node| {
                let instance = instance_by_id.get(&node.instance_id);
                json!({
                    "kind": "neuron",
                    "id": node.id,
                    "neuronKind": os_media_neuron_kind_for_node(&node.id),
                    "inputPorts": node.inputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                    "outputPorts": node.outputs.iter().map(|port| &port.id).collect::<Vec<_>>(),
                    "params": {
                        "instanceId": node.instance_id,
                        "pluginId": instance.map(|entry| &entry.plugin_id).unwrap_or(&String::new()),
                        "appId": instance.map(|entry| &entry.app_id).unwrap_or(&String::new()),
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
    pub fn apply_flow_fixture_to_os_workflow(graph: &OsWorkflow, fixture_json: &str) -> Vec<OsOperation> {
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
                    operations.push(OsOperation::MoveWorkflowNode { node_id: node.id.clone(), x, y });
                }
            }
        }
        let mut removed_node_ids = HashSet::new();
        if let Some(widgets) = fixture.get("widgets").and_then(Value::as_array) {
            let widget_ids: HashSet<&str> = widgets.iter().filter_map(|widget| widget.get("id").and_then(Value::as_str)).collect();
            for node in &graph.nodes {
                if !widget_ids.contains(node.id.as_str()) {
                    removed_node_ids.insert(node.id.clone());
                    operations.push(OsOperation::RemoveAppInstance { instance_id: node.instance_id.clone() });
                }
            }
        }
        let synapse_endpoints = |synapse: &Value| -> Option<(String, String, String, String)> {
            Some((synapse.get("from").and_then(Value::as_str)?.into(), synapse.get("fromPort").and_then(Value::as_str)?.into(), synapse.get("to").and_then(Value::as_str)?.into(), synapse.get("toPort").and_then(Value::as_str)?.into()))
        };
        let edge_endpoints = |edge: &OsWorkflowEdge| (edge.source_node_id.clone(), edge.source_port_id.clone(), edge.target_node_id.clone(), edge.target_port_id.clone());
        let synapses = fixture.get("synapses").and_then(Value::as_array).cloned().unwrap_or_default();
        let fixture_endpoints: HashSet<_> = synapses.iter().filter_map(synapse_endpoints).collect();
        let graph_endpoints: HashSet<_> = graph.edges.iter().map(edge_endpoints).collect();
        let node_by_id: HashMap<&str, &OsWorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
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
            operations.push(OsOperation::ConnectWorkflowPorts { edge: OsWorkflowEdge { id, source_node_id, source_port_id, target_node_id, target_port_id, contract } });
        }
        if fixture.get("synapses").and_then(Value::as_array).is_some() {
            for edge in &graph.edges {
                if fixture_endpoints.contains(&edge_endpoints(edge)) {
                    continue;
                }
                if removed_node_ids.contains(&edge.source_node_id) || removed_node_ids.contains(&edge.target_node_id) {
                    continue;
                }
                operations.push(OsOperation::DisconnectWorkflowEdge { edge_id: edge.id.clone() });
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

    /** @emoji 🕸️ Serializes an OS workflow into generic node-graph scene payloads. */
    pub fn os_workflow_to_node_graph_payload(graph: &OsWorkflow, instances: &[OsAppInstance]) -> OsWorkflowNodeGraphPayload {
        let instance_by_id: HashMap<_, _> = instances.iter().map(|instance| (instance.id.clone(), instance)).collect();
        let nodes: Vec<_> = graph
            .nodes
            .iter()
            .map(|node| {
                let instance = instance_by_id.get(&node.instance_id);
                let label = instance.map(|entry| format!("{} / {}", entry.plugin_id, entry.app_id)).unwrap_or_else(|| node.instance_id.clone());
                json!({
                    "id": node.id,
                    "instanceId": node.instance_id,
                    "label": label,
                    "x": node.x,
                    "y": node.y,
                    "width": node.width,
                    "height": node.height,
                    "inputs": node.inputs.iter().map(|port| json!({
                        "id": port.id,
                        "resourceKind": port.artifact_kind,
                        "direction": port.direction,
                        "label": port.id,
                    })).collect::<Vec<_>>(),
                    "outputs": node.outputs.iter().map(|port| json!({
                        "id": port.id,
                        "resourceKind": port.artifact_kind,
                        "direction": port.direction,
                        "label": port.id,
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
        let find_items: Vec<_> = graph
            .nodes
            .iter()
            .map(|node| {
                let instance = instance_by_id.get(&node.instance_id);
                json!({
                    "id": node.instance_id,
                    "label": instance
                        .map(|entry| format!("{} / {}", entry.plugin_id, entry.app_id))
                        .unwrap_or_else(|| node.instance_id.clone()),
                    "category": "Workflow",
                })
            })
            .collect();
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

    fn os_workflow_channel_spec(port_id: &str, artifact_kind: &str, label: &str) -> OsWorkflowChannelSpec {
        let code = port_id.chars().next().map(|ch| ch.to_uppercase().collect::<String>()).unwrap_or_else(|| "P".into());
        let abbreviation = if label.chars().count() <= 3 { label.into() } else { label.chars().take(3).collect() };
        OsWorkflowChannelSpec { name: port_id.into(), code, abbreviation, full_name: label.into(), operators: vec![artifact_kind.into()] }
    }

    fn parameter_label(parameter: &OsParameter) -> &str {
        match parameter {
            OsParameter::Numeric { name, .. } | OsParameter::Categorical { name, .. } | OsParameter::Toggle { name, .. } | OsParameter::Text { name, .. } => name,
        }
    }

    fn parameter_entity_id(parameter: &OsParameter) -> &str {
        match parameter {
            OsParameter::Numeric { id, .. } | OsParameter::Categorical { id, .. } | OsParameter::Toggle { id, .. } | OsParameter::Text { id, .. } => id,
        }
    }

    /// @emoji 🧩️ Registers per-node neuron metadata for the OS workflow flow extension.
    pub fn build_os_workflow_operator_infos(graph: &OsWorkflow, instances: &[OsAppInstance], parameters: &[OsParameter]) -> Vec<OsWorkflowOperatorInfo> {
        let instance_by_id: HashMap<_, _> = instances.iter().map(|row| (row.id.clone(), row)).collect();
        let parameter_by_id: HashMap<_, _> = parameters.iter().map(|row| (parameter_entity_id(row).to_string(), row)).collect();
        graph
            .nodes
            .iter()
            .map(|node| {
                let instance = instance_by_id.get(&node.instance_id);
                let registration = instance.and_then(|row| os_app_registration(&row.plugin_id, &row.app_id));
                let neuron_kind = os_media_neuron_kind_for_node(&node.id);
                OsWorkflowOperatorInfo {
                    id: neuron_kind,
                    module: OS_MEDIA_FLOW_MODULE_ID.into(),
                    name: instance.map(|row| row.label.clone()).unwrap_or_else(|| node.instance_id.clone()),
                    abbreviation: instance.map(|row| if row.app_id.chars().count() <= 3 { row.app_id.clone() } else { row.app_id.chars().take(3).collect() }).unwrap_or_else(|| "app".into()),
                    icon: format!("emoji:{}", registration.map(|row| row.component_kind.clone()).unwrap_or_else(|| "s".into())),
                    summary: instance.map(|row| format!("{}/{}", row.plugin_id, row.app_id)).unwrap_or_else(|| "App instance".into()),
                    inputs: node
                        .inputs
                        .iter()
                        .map(|port| {
                            let parameter_id = parameter_id_from_port_id(&port.id);
                            let label = parameter_id.as_ref().and_then(|id| parameter_by_id.get(id)).map(|parameter| parameter_label(parameter).to_string()).or_else(|| media_port_spec_id(&port.id)).unwrap_or_else(|| port.id.clone());
                            os_workflow_channel_spec(&port.id, &port.artifact_kind, &label)
                        })
                        .collect(),
                    outputs: node
                        .outputs
                        .iter()
                        .map(|port| {
                            let label = media_port_spec_id(&port.id).unwrap_or_else(|| port.id.clone());
                            os_workflow_channel_spec(&port.id, &port.artifact_kind, &label)
                        })
                        .collect(),
                }
            })
            .collect()
    }
    //#endregion 🔖️Workflow

    //#region 🔖️WorkflowPlanner
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[serde(rename_all = "camelCase")]
    pub struct WorkflowDelivery {
        pub edge_id: String,
        pub producer_instance_id: String,
        pub producer_port_id: String,
        pub consumer_instance_id: String,
        pub consumer_port_id: String,
        // 🩹️ `OsWorkflowEdge` has no `contract` field yet (see `reconcile_os_workflow`'s baseline
        // comment above). Once a sibling ticket lands `contract: MediaContract` on the edge, carry it
        // through here too so delivery-execution knows what to transcode.
    }

    /// 🔬️ One planner test vector: a workflow graph, the instances marked dirty, and the deliveries
    /// `plan_workflow` must produce for them. Ships as a `dsl`+`pack` document — see
    /// `framework/product/os/core/fixtures/*.dsl`/`*.spk` and `README.md` — so the fixture corpus
    /// itself proves the dsl≡pack law instead of riding untyped JSON.
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
    #[serde(rename_all = "camelCase")]
    #[dsl(extension = "workflow-fixture")]
    pub struct WorkflowFixture {
        pub name: String,
        #[dsl(block)]
        pub graph: OsWorkflow,
        pub dirty_instance_ids: Vec<String>,
        #[dsl(table)]
        pub expected_deliveries: Vec<WorkflowDelivery>,
    }

    /// @emoji 🧭️ Post-order DFS reversed into a topological node order (source before target); same
    /// recursive shape as `validate_workflow`'s cycle-detection DFS, but collects the traversal
    /// order instead of flagging revisits (the graph is validated acyclic before planning runs).
    fn workflow_topological_node_order(graph: &OsWorkflow) -> Vec<String> {
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &graph.edges {
            adjacency.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
        }
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        fn dfs(node_id: &str, adjacency: &HashMap<String, Vec<String>>, visited: &mut HashSet<String>, order: &mut Vec<String>) {
            if !visited.insert(node_id.to_string()) {
                return;
            }
            for next in adjacency.get(node_id).into_iter().flatten() {
                dfs(next, adjacency, visited, order);
            }
            order.push(node_id.to_string());
        }
        for node in &graph.nodes {
            dfs(&node.id, &adjacency, &mut visited, &mut order);
        }
        order.reverse();
        order
    }

    /// @emoji 🚚️ Plans one [`WorkflowDelivery`] per edge in the downstream closure of `dirty_instance_ids`,
    /// propagating dirtiness onto each edge's consumer instance so multi-hop chains (A→B→C) resolve in a
    /// single topological pass. Pure/side-effect-free — callers own applying the deliveries.
    pub fn plan_workflow(graph: &OsWorkflow, dirty_instance_ids: &HashSet<String>) -> Vec<WorkflowDelivery> {
        let node_by_id: HashMap<&str, &OsWorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let mut edges_by_source: HashMap<&str, Vec<&OsWorkflowEdge>> = HashMap::new();
        for edge in &graph.edges {
            edges_by_source.entry(edge.source_node_id.as_str()).or_default().push(edge);
        }
        let order = workflow_topological_node_order(graph);
        let mut dirty = dirty_instance_ids.clone();
        let mut deliveries = Vec::new();
        for node_id in &order {
            let Some(node) = node_by_id.get(node_id.as_str()) else { continue };
            if !dirty.contains(&node.instance_id) {
                continue;
            }
            for edge in edges_by_source.get(node_id.as_str()).into_iter().flatten() {
                let Some(target_node) = node_by_id.get(edge.target_node_id.as_str()) else { continue };
                deliveries.push(WorkflowDelivery {
                    edge_id: edge.id.clone(),
                    producer_instance_id: node.instance_id.clone(),
                    producer_port_id: edge.source_port_id.clone(),
                    consumer_instance_id: target_node.instance_id.clone(),
                    consumer_port_id: edge.target_port_id.clone(),
                });
                dirty.insert(target_node.instance_id.clone());
            }
        }
        deliveries
    }
    //#endregion 🔖️WorkflowPlanner

    //#region 🔖️WorkflowInstanceRegistry
    #[derive(Clone, Debug, Default)]
    pub struct WorkflowInstanceRegistry {
        instances: HashMap<String, OsAppInstance>,
    }

    impl WorkflowInstanceRegistry {
        pub fn materialize_instance(&mut self, instance: OsAppInstance) {
            self.instances.insert(instance.id.clone(), instance);
        }

        pub fn get_instance(&self, instance_id: &str) -> Option<&OsAppInstance> {
            self.instances.get(instance_id)
        }
    }
    //#endregion 🔖️WorkflowInstanceRegistry

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

    pub fn export_os_app_instance_media(instance: &OsAppInstance, source_document: &Value, format: OsMediaFormat) -> Result<OsMediaExportResult, String> {
        let handlers = export_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers.get(&os_media_export_key(&instance.yields, &format)).ok_or_else(|| format!("no export handler for {}:{}", instance.yields, format.as_str()))?;
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
    pub fn import_os_app_instance_media(instance: &OsAppInstance, data: &[u8], format: OsMediaFormat) -> Result<Value, String> {
        let handlers = import_handlers().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let handler = handlers.get(&os_media_export_key(&instance.yields, &format)).ok_or_else(|| format!("no import handler for {}:{}", instance.yields, format.as_str()))?;
        handler(data)
    }
    //#endregion 🔖️MediaExport

    //#region 🔖️WorkflowVfs
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowVfsDescriptorKind {
        pub id: String,
        pub name: String,
        pub presentation: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowVfsFileNodeDescriptor {
        pub id: String,
        pub descriptor_kind_id: String,
        pub label: String,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowVfsFileNodeKind {
        pub id: String,
        pub name: String,
        pub descriptors: Vec<OsWorkflowVfsFileNodeDescriptor>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowVfsSchema {
        pub descriptor_kinds: HashMap<String, OsWorkflowVfsDescriptorKind>,
        pub file_node_kinds: HashMap<String, OsWorkflowVfsFileNodeKind>,
        pub descriptor_column_ids: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowVfsNodeRecord {
        pub id: String,
        pub file_node_kind_id: String,
        pub name: String,
        pub path: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        pub has_children: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub icon: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub navigate_uri: Option<String>,
        #[serde(default, skip_serializing_if = "HashMap::is_empty")]
        pub descriptor_values: HashMap<String, String>,
    }

    pub fn os_workflow_vfs_schema() -> OsWorkflowVfsSchema {
        let mut descriptor_kinds = HashMap::new();
        descriptor_kinds.insert("text".into(), OsWorkflowVfsDescriptorKind { id: "text".into(), name: "Text".into(), presentation: "text".into() });
        let binding = OsWorkflowVfsFileNodeDescriptor { id: "binding".into(), descriptor_kind_id: "text".into(), label: "Binding".into() };
        let mut file_node_kinds = HashMap::new();
        for kind in ["root", "instance", "folder", "source", "input"] {
            file_node_kinds.insert(kind.into(), OsWorkflowVfsFileNodeKind { id: kind.into(), name: kind.into(), descriptors: vec![binding.clone()] });
        }
        file_node_kinds.insert(
            "export".into(),
            OsWorkflowVfsFileNodeKind { id: "export".into(), name: "Export".into(), descriptors: vec![binding.clone(), OsWorkflowVfsFileNodeDescriptor { id: "format".into(), descriptor_kind_id: "text".into(), label: "Format".into() }] },
        );
        OsWorkflowVfsSchema { descriptor_kinds, file_node_kinds, descriptor_column_ids: vec!["binding".into(), "format".into()] }
    }

    pub fn os_workflow_vfs_instance_id(node_id: &str) -> Option<String> {
        regex_lite(node_id, r"^inst:([^:]+)(?::|$)")
    }

    pub fn os_workflow_vfs_instance_folder_id(instance_id: &str) -> String {
        format!("inst:{instance_id}")
    }

    pub fn os_workflow_vfs_source_id(instance_id: &str) -> String {
        format!("inst:{instance_id}:source")
    }

    pub fn os_workflow_vfs_inputs_folder_id(instance_id: &str) -> String {
        format!("inst:{instance_id}:inputs")
    }

    pub fn os_workflow_vfs_outputs_folder_id(instance_id: &str) -> String {
        format!("inst:{instance_id}:outputs")
    }

    pub fn os_workflow_vfs_input_port_id(instance_id: &str, port_spec_id: &str) -> String {
        format!("inst:{instance_id}:input:{port_spec_id}")
    }

    pub fn os_workflow_vfs_export_id(instance_id: &str, port_spec_id: &str, format: &OsMediaFormat) -> String {
        format!("inst:{instance_id}:export:{port_spec_id}:{}", format.as_str())
    }

    pub fn os_workflow_vfs_import_id(instance_id: &str, format: &OsMediaFormat) -> String {
        format!("inst:{instance_id}:import:{}", format.as_str())
    }

    fn regex_lite(input: &str, pattern: &str) -> Option<String> {
        if pattern == r"^inst:([^:]+)(?::|$)" {
            let rest = input.strip_prefix("inst:")?;
            let instance_id = rest.split(':').next()?;
            if instance_id.is_empty() {
                None
            } else {
                Some(instance_id.to_string())
            }
        } else {
            None
        }
    }

    /// @emoji 📁️ Lists VFS children for one workflow folder node.
    pub fn list_os_workflow_vfs_children(parent_id: &str, instances: &[OsAppInstance], graph: &OsWorkflow, bindings: &[OsParameterFieldBinding], parameters: &[OsParameter]) -> Vec<OsWorkflowVfsNodeRecord> {
        if parent_id == OS_WORKFLOW_VFS_ROOT_ID {
            return instances
                .iter()
                .map(|instance| {
                    let registration = os_app_registration(&instance.plugin_id, &instance.app_id);
                    OsWorkflowVfsNodeRecord {
                        id: os_workflow_vfs_instance_folder_id(&instance.id),
                        file_node_kind_id: "instance".into(),
                        name: format!("{} ({}.{}))", instance.label, instance.plugin_id, instance.app_id),
                        path: format!("/{}", instance.label),
                        parent_id: Some(OS_WORKFLOW_VFS_ROOT_ID.into()),
                        has_children: true,
                        icon: registration.as_ref().map(|entry| entry.component_kind.clone()),
                        navigate_uri: None,
                        descriptor_values: HashMap::from([("binding".into(), instance.yields.clone())]),
                    }
                })
                .collect();
        }
        let Some(instance_id) = os_workflow_vfs_instance_id(parent_id) else {
            return Vec::new();
        };
        let Some(instance) = instances.iter().find(|entry| entry.id == instance_id) else {
            return Vec::new();
        };
        let registration = os_app_registration(&instance.plugin_id, &instance.app_id);
        if parent_id == os_workflow_vfs_instance_folder_id(&instance_id) {
            return vec![
                OsWorkflowVfsNodeRecord {
                    id: os_workflow_vfs_source_id(&instance_id),
                    file_node_kind_id: "source".into(),
                    name: "source.json".into(),
                    path: format!("/{}/source.json", instance.label),
                    parent_id: Some(parent_id.into()),
                    has_children: false,
                    icon: Some("json".into()),
                    navigate_uri: Some(format!("os://instance/{}", instance.id)),
                    descriptor_values: HashMap::from([("binding".into(), registration.as_ref().map(|entry| entry.source_format.clone()).unwrap_or_else(|| instance.yields.clone()))]),
                },
                OsWorkflowVfsNodeRecord {
                    id: os_workflow_vfs_inputs_folder_id(&instance_id),
                    file_node_kind_id: "folder".into(),
                    name: "inputs".into(),
                    path: format!("/{}/inputs", instance.label),
                    parent_id: Some(parent_id.into()),
                    has_children: registration.as_ref().map(|entry| !entry.inputs.is_empty()).unwrap_or(false) || bindings.iter().any(|binding| binding.instance_id == instance_id),
                    icon: Some("folder-input".into()),
                    navigate_uri: None,
                    descriptor_values: HashMap::new(),
                },
                OsWorkflowVfsNodeRecord {
                    id: os_workflow_vfs_outputs_folder_id(&instance_id),
                    file_node_kind_id: "folder".into(),
                    name: "outputs".into(),
                    path: format!("/{}/outputs", instance.label),
                    parent_id: Some(parent_id.into()),
                    has_children: registration.as_ref().map(|entry| !entry.outputs.is_empty()).unwrap_or(false),
                    icon: Some("folder-output".into()),
                    navigate_uri: None,
                    descriptor_values: HashMap::new(),
                },
            ];
        }
        if parent_id == os_workflow_vfs_inputs_folder_id(&instance_id) {
            let mut rows = Vec::new();
            if let Some(registration) = registration.as_ref() {
                for spec in &registration.inputs {
                    rows.push(OsWorkflowVfsNodeRecord {
                        id: os_workflow_vfs_input_port_id(&instance_id, &spec.id),
                        file_node_kind_id: "input".into(),
                        name: spec.id.clone(),
                        path: format!("/{}/inputs/{}", instance.label, spec.id),
                        parent_id: Some(parent_id.into()),
                        has_children: false,
                        icon: Some("plug".into()),
                        navigate_uri: None,
                        descriptor_values: HashMap::from([("binding".into(), spec.artifact_kind.clone())]),
                    });
                }
            }
            for binding in bindings.iter().filter(|entry| entry.instance_id == instance_id) {
                let parameter = parameters.iter().find(|entry| match entry {
                    crate::instance::OsParameter::Numeric { id, .. } | crate::instance::OsParameter::Categorical { id, .. } | crate::instance::OsParameter::Toggle { id, .. } | crate::instance::OsParameter::Text { id, .. } => {
                        id == &binding.parameter_id
                    }
                });
                rows.push(OsWorkflowVfsNodeRecord {
                    id: os_workflow_vfs_input_port_id(&instance_id, &format!("param.{}", binding.parameter_id)),
                    file_node_kind_id: "input".into(),
                    name: parameter
                        .map(|entry| match entry {
                            crate::instance::OsParameter::Numeric { name, .. } | crate::instance::OsParameter::Categorical { name, .. } | crate::instance::OsParameter::Toggle { name, .. } | crate::instance::OsParameter::Text { name, .. } => {
                                name.clone()
                            }
                        })
                        .unwrap_or_else(|| binding.field_path.clone()),
                    path: format!("/{}/inputs/param.{}", instance.label, binding.parameter_id),
                    parent_id: Some(parent_id.into()),
                    has_children: false,
                    icon: Some("sliders-horizontal".into()),
                    navigate_uri: None,
                    descriptor_values: HashMap::from([(
                        "binding".into(),
                        parameter
                            .map(|entry| match entry {
                                crate::instance::OsParameter::Numeric { name, .. } | crate::instance::OsParameter::Categorical { name, .. } | crate::instance::OsParameter::Toggle { name, .. } | crate::instance::OsParameter::Text { name, .. } => {
                                    name.clone()
                                }
                            })
                            .unwrap_or_else(|| binding.parameter_id.clone()),
                    )]),
                });
            }
            let descriptor = crate::registry::os_artifact_descriptor(&instance.yields);
            for format in required_os_media_import_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind)) {
                let ext = format.as_str();
                rows.push(OsWorkflowVfsNodeRecord {
                    id: os_workflow_vfs_import_id(&instance_id, &format),
                    file_node_kind_id: "import".into(),
                    name: format!("import.{ext}"),
                    path: format!("/{}/inputs/import.{ext}", instance.label),
                    parent_id: Some(parent_id.into()),
                    has_children: false,
                    icon: Some(ext.into()),
                    navigate_uri: Some(format!("os://import/{}/{}/{}", instance.id, descriptor.kind, format.as_str())),
                    descriptor_values: HashMap::from([("format".into(), format.as_str().into())]),
                });
            }
            return rows;
        }
        if parent_id == os_workflow_vfs_outputs_folder_id(&instance_id) {
            let descriptor = crate::registry::os_artifact_descriptor(&instance.yields);
            let formats = required_os_media_export_formats(&descriptor.dimension, os_resource_media_capability(&descriptor.kind));
            let mut rows = Vec::new();
            if let Some(registration) = registration.as_ref() {
                for spec in &registration.outputs {
                    for format in &formats {
                        let ext = os_media_export_extension_for_format(format);
                        rows.push(OsWorkflowVfsNodeRecord {
                            id: os_workflow_vfs_export_id(&instance_id, &spec.id, format),
                            file_node_kind_id: "export".into(),
                            name: format!("{}.{}", spec.id, ext),
                            path: format!("/{}/outputs/{}.{}", instance.label, spec.id, ext),
                            parent_id: Some(parent_id.into()),
                            has_children: false,
                            icon: Some(ext.into()),
                            navigate_uri: Some(format!("os://export/{}/{}/{}", instance.id, spec.id, format.as_str())),
                            descriptor_values: HashMap::from([("binding".into(), spec.artifact_kind.clone()), ("format".into(), format.as_str().into())]),
                        });
                    }
                }
            }
            return rows;
        }
        let _ = graph;
        let _ = parameter_id_from_port_id;
        let _ = media_port_spec_id;
        let _ = os_app_primary_output_kind;
        Vec::new()
    }
    //#endregion 🔖️WorkflowVfs

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::instance::OsDocumentRef;
        use crate::registry::{merge_os_plugin_definition, os_baseline_resource, OsPlatformAppInput, OsPlatformInput};

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
            let mut kernel = kernel_3d_brepkit::BrepkitKernel::new();
            let solid = kernel.box_prim_sync(2.0, 3.0, 4.0).expect("box");
            crate::media_export_raster::register_solid_exporter("3d.__solid_test", Box::new(kernel_3d_brepkit::StepSolidExporter));
            crate::media_export_raster::register_solid_importer("3d.__solid_test", Box::new(kernel_3d_brepkit::StepSolidImporter));
            assert!(crate::media_export_raster::solid_exporter_for("3d.__solid_test", &OsMediaFormat::Step));
            let bytes = crate::media_export_raster::export_registered_solid("3d.__solid_test", &OsMediaFormat::Step, &kernel, &[solid], 0.1).expect("export step");
            assert!(!bytes.is_empty());
            let imported = crate::media_export_raster::import_registered_solid("3d.__solid_test", &OsMediaFormat::Step, &mut kernel, &bytes, 0.1).expect("import step");
            assert!(!imported.is_empty());
        }

        #[test]
        fn flow_fixture_projects_neuron_preview() {
            let mut resources = HashMap::new();
            resources.insert("draw".into(), os_baseline_resource("2d.drawing", "draw.document", "draw"));
            let platform = OsPlatformInput {
                id: "draw".into(),
                name: "Draw".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput { id: "draw".into(), label: "Draw".into(), document: vec!["semio".into(), "draw".into()], controller_id: "draw-play".into(), modes: vec![], default_mode_id: None }],
            };
            merge_os_plugin_definition("draw", &platform, &resources).expect("merge");
            let registration = os_app_registration("draw", "draw").expect("registration");
            let instance = OsAppInstance {
                id: "app-1".into(),
                plugin_id: "draw".into(),
                app_id: "draw".into(),
                label: "Draw".into(),
                yields: os_app_primary_output_kind(&registration),
                document: OsDocumentRef { document_id: "doc-app-1".into(), schema: "draw.document".into() },
                config: None,
            };
            let mut graph = empty_workflow();
            graph.nodes.push(workflow_node_for_instance(&instance, &registration, &WorkflowPosition { x: 0.0, y: 0.0 }, "node-1"));
            let fixture = os_workflow_to_flow_fixture(&graph, std::slice::from_ref(&instance), &OsWorkflowCamera::default());
            assert_eq!(fixture["schema"], "flow.fixture");
            assert_eq!(fixture["widgets"][0]["preview"], true);
            assert_eq!(fixture["widgets"][0]["params"]["instanceId"], "app-1");
            assert_eq!(fixture["widgets"][0]["params"]["pluginId"], "draw");
            assert_eq!(fixture["widgets"][0]["params"]["appId"], "draw");
            let operators = build_os_workflow_operator_infos(&graph, &[instance], &[]);
            assert_eq!(operators.len(), 1);
            assert_eq!(operators[0].id, "os.media.node.node-1");
            assert_eq!(operators[0].module, OS_MEDIA_FLOW_MODULE_ID);
            assert_eq!(operators[0].name, "Draw");
        }

        #[test]
        fn vfs_inputs_folder_lists_a_dwg_import_row_for_2d_kinds() {
            crate::registry::register_artifact_descriptor(&semio_framework_core::ArtifactKindSpec {
                id: "2d.drawing".into(),
                name: "2D Drawing".into(),
                source_format: "draw.document".into(),
                component_kind: "draw".into(),
                dimension: "2d".into(),
                media_capability: semio_framework_core::OsMediaCapability::MeshOnly,
                media_type: semio_framework_core::MediaType { class: semio_framework_core::MediaClass::TwoD, form: semio_framework_core::MediaForm::Vector },
                schema: "draw.document".into(),
                export_formats: vec![semio_framework_core::OsMediaFormat::Svg, semio_framework_core::OsMediaFormat::Png],
                import_formats: vec![semio_framework_core::OsMediaFormat::Svg, semio_framework_core::OsMediaFormat::Png],
            });
            let mut resources = HashMap::new();
            resources.insert("draw".into(), os_baseline_resource("2d.drawing", "draw.document", "draw"));
            let platform = OsPlatformInput {
                id: "draw-vfs".into(),
                name: "Draw".into(),
                api_version: "1".into(),
                apps: vec![OsPlatformAppInput { id: "draw".into(), label: "Draw".into(), document: vec!["semio".into(), "draw".into()], controller_id: "draw-play".into(), modes: vec![], default_mode_id: None }],
            };
            merge_os_plugin_definition("draw-vfs", &platform, &resources).expect("merge");
            let registration = os_app_registration("draw-vfs", "draw").expect("registration");
            let instance = OsAppInstance {
                id: "app-vfs-1".into(),
                plugin_id: "draw-vfs".into(),
                app_id: "draw".into(),
                label: "Draw".into(),
                yields: os_app_primary_output_kind(&registration),
                document: OsDocumentRef { document_id: "doc-app-vfs-1".into(), schema: "draw.document".into() },
                config: None,
            };
            let graph = empty_workflow();
            let inputs_folder = os_workflow_vfs_inputs_folder_id(&instance.id);
            let rows = list_os_workflow_vfs_children(&inputs_folder, std::slice::from_ref(&instance), &graph, &[], &[]);
            let import_row = rows.iter().find(|row| row.file_node_kind_id == "import").expect("import row present");
            assert_eq!(import_row.name, "import.dwg");
            assert_eq!(import_row.navigate_uri, Some(format!("os://import/{}/2d.drawing/dwg", instance.id)));
        }

        fn media_node(id: &str, instance_id: &str, x: f64, y: f64) -> OsWorkflowNode {
            OsWorkflowNode {
                id: id.into(),
                instance_id: instance_id.into(),
                x,
                y,
                width: 160.0,
                height: 72.0,
                inputs: vec![OsMediaPort { id: format!("{instance_id}:in"), artifact_kind: "2d.drawing".into(), direction: "in".into() }],
                outputs: vec![OsMediaPort { id: format!("{instance_id}:out"), artifact_kind: "2d.drawing".into(), direction: "out".into() }],
            }
        }

        #[test]
        fn flow_fixture_round_trips_camera_and_diffs_back_to_operations() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-1", 40.0, 80.0));
            graph.nodes.push(media_node("node-2", "app-2", 300.0, 80.0));
            graph.edges.push(OsWorkflowEdge { id: "edge-1".into(), source_node_id: "node-1".into(), source_port_id: "app-1:out".into(), target_node_id: "node-2".into(), target_port_id: "app-2:in".into(), contract: placeholder_media_contract("2d.drawing") });
            let camera = OsWorkflowCamera { x: 12.0, y: -8.0, zoom: 1.5 };
            let fixture = os_workflow_to_flow_fixture(&graph, &[], &camera);
            assert_eq!(fixture["camera"]["x"], 12.0);
            assert_eq!(fixture["camera"]["zoom"], 1.5);
            let unchanged = apply_flow_fixture_to_os_workflow(&graph, &fixture.to_string());
            assert!(unchanged.is_empty());
            let mut moved = fixture.clone();
            moved["layout"]["node-1"] = json!({ "x": 220.0, "y": 156.0 });
            let operations = apply_flow_fixture_to_os_workflow(&graph, &moved.to_string());
            assert_eq!(operations, vec![OsOperation::MoveWorkflowNode { node_id: "node-1".into(), x: 140.0, y: 120.0 }]);
        }

        #[test]
        fn flow_fixture_diff_connects_disconnects_and_removes() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", "app-2", 200.0, 0.0));
            graph.edges.push(OsWorkflowEdge { id: "edge-1".into(), source_node_id: "node-1".into(), source_port_id: "app-1:out".into(), target_node_id: "node-2".into(), target_port_id: "app-2:in".into(), contract: placeholder_media_contract("2d.drawing") });
            let mut fixture = os_workflow_to_flow_fixture(&graph, &[], &OsWorkflowCamera::default());
            fixture["synapses"] = json!([
                { "id": "", "from": "node-2", "fromPort": "app-2:out", "to": "node-1", "toPort": "app-1:in" }
            ]);
            let operations = apply_flow_fixture_to_os_workflow(&graph, &fixture.to_string());
            assert!(matches!(
                &operations[0],
                OsOperation::ConnectWorkflowPorts { edge } if edge.source_node_id == "node-2" && edge.target_port_id == "app-1:in" && !edge.id.is_empty()
            ));
            assert!(operations.contains(&OsOperation::DisconnectWorkflowEdge { edge_id: "edge-1".into() }));
            let mut removal = os_workflow_to_flow_fixture(&graph, &[], &OsWorkflowCamera::default());
            removal["widgets"] = json!([{ "id": "node-1" }]);
            removal["synapses"] = json!([]);
            let removal_operations = apply_flow_fixture_to_os_workflow(&graph, &removal.to_string());
            assert!(removal_operations.contains(&OsOperation::RemoveAppInstance { instance_id: "app-2".into() }));
            assert!(!removal_operations.iter().any(|operation| matches!(operation, OsOperation::DisconnectWorkflowEdge { .. })));
        }

        //#region 🔖️WorkflowPlanner
        fn dirty_set(instance_ids: &[&str]) -> HashSet<String> {
            instance_ids.iter().map(|id| id.to_string()).collect()
        }

        #[test]
        fn plans_a_single_delivery_across_one_dirty_edge() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", "app-2", 200.0, 0.0));
            graph.edges.push(OsWorkflowEdge { id: "edge-1".into(), source_node_id: "node-1".into(), source_port_id: "app-1:out".into(), target_node_id: "node-2".into(), target_port_id: "app-2:in".into(), contract: placeholder_media_contract("2d.drawing") });
            let deliveries = plan_workflow(&graph, &dirty_set(&["app-1"]));
            assert_eq!(
                deliveries,
                vec![WorkflowDelivery { edge_id: "edge-1".into(), producer_instance_id: "app-1".into(), producer_port_id: "app-1:out".into(), consumer_instance_id: "app-2".into(), consumer_port_id: "app-2:in".into() }]
            );
        }

        #[test]
        fn plans_a_chain_in_topological_order_when_only_the_root_is_dirty() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", "app-2", 200.0, 0.0));
            graph.nodes.push(media_node("node-3", "app-3", 400.0, 0.0));
            graph.edges.push(OsWorkflowEdge { id: "edge-ab".into(), source_node_id: "node-1".into(), source_port_id: "app-1:out".into(), target_node_id: "node-2".into(), target_port_id: "app-2:in".into(), contract: placeholder_media_contract("2d.drawing") });
            graph.edges.push(OsWorkflowEdge { id: "edge-bc".into(), source_node_id: "node-2".into(), source_port_id: "app-2:out".into(), target_node_id: "node-3".into(), target_port_id: "app-3:in".into(), contract: placeholder_media_contract("2d.drawing") });
            let deliveries = plan_workflow(&graph, &dirty_set(&["app-1"]));
            assert_eq!(deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect::<Vec<_>>(), vec!["edge-ab", "edge-bc"], "A→B must be planned before B→C");
        }

        #[test]
        fn plans_a_diamond_with_one_delivery_per_incoming_edge() {
            // 🔀️ One delivery per edge, not per node: D has two producers (B and C), so D is the
            // target of two separate deliveries rather than a single merged one.
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-a", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", "app-b", 200.0, -80.0));
            graph.nodes.push(media_node("node-3", "app-c", 200.0, 80.0));
            graph.nodes.push(media_node("node-4", "app-d", 400.0, 0.0));
            graph.edges.push(OsWorkflowEdge { id: "edge-ab".into(), source_node_id: "node-1".into(), source_port_id: "app-a:out".into(), target_node_id: "node-2".into(), target_port_id: "app-b:in".into(), contract: placeholder_media_contract("2d.drawing") });
            graph.edges.push(OsWorkflowEdge { id: "edge-ac".into(), source_node_id: "node-1".into(), source_port_id: "app-a:out".into(), target_node_id: "node-3".into(), target_port_id: "app-c:in".into(), contract: placeholder_media_contract("2d.drawing") });
            graph.edges.push(OsWorkflowEdge { id: "edge-bd".into(), source_node_id: "node-2".into(), source_port_id: "app-b:out".into(), target_node_id: "node-4".into(), target_port_id: "app-d:in".into(), contract: placeholder_media_contract("2d.drawing") });
            graph.edges.push(OsWorkflowEdge { id: "edge-cd".into(), source_node_id: "node-3".into(), source_port_id: "app-c:out".into(), target_node_id: "node-4".into(), target_port_id: "app-d:in".into(), contract: placeholder_media_contract("2d.drawing") });
            let deliveries = plan_workflow(&graph, &dirty_set(&["app-a"]));
            let edge_ids: Vec<&str> = deliveries.iter().map(|delivery| delivery.edge_id.as_str()).collect();
            assert_eq!(edge_ids.len(), 4);
            let index_of = |id: &str| edge_ids.iter().position(|candidate| *candidate == id).unwrap();
            assert!(index_of("edge-bd") > index_of("edge-ab"), "B→D must be planned after A→B");
            assert!(index_of("edge-cd") > index_of("edge-ac"), "C→D must be planned after A→C");
        }

        #[test]
        fn plans_nothing_when_no_instance_is_dirty() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-1", 0.0, 0.0));
            graph.nodes.push(media_node("node-2", "app-2", 200.0, 0.0));
            graph.edges.push(OsWorkflowEdge { id: "edge-1".into(), source_node_id: "node-1".into(), source_port_id: "app-1:out".into(), target_node_id: "node-2".into(), target_port_id: "app-2:in".into(), contract: placeholder_media_contract("2d.drawing") });
            assert!(plan_workflow(&graph, &dirty_set(&[])).is_empty());
        }

        #[test]
        fn plans_nothing_for_a_dirty_node_with_no_outgoing_edges() {
            let mut graph = empty_workflow();
            graph.nodes.push(media_node("node-1", "app-1", 0.0, 0.0));
            assert!(plan_workflow(&graph, &dirty_set(&["app-1"])).is_empty());
        }

        /// 🔬️ Shared fixtures replay (`framework/product/os/core/fixtures/*.dsl`) — the same files
        /// drive `planWorkflow`'s vitest harness in `js/index.ts` (decoded there via the sibling
        /// `.spk` through a wasm export), keeping the two implementations in lockstep. See
        /// `framework/product/os/core/fixtures/README.md`.
        fn workflow_fixture_dsl_paths() -> Vec<std::path::PathBuf> {
            let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧫️fixtures");
            let entries = std::fs::read_dir(&fixtures_dir).unwrap_or_else(|error| panic!("read fixtures dir {fixtures_dir:?}: {error}"));
            let mut paths: Vec<std::path::PathBuf> = entries
                .map(|entry| entry.expect("dir entry").path())
                .filter(|path| path.extension().and_then(|extension| extension.to_str()) == Some("dsl"))
                .collect();
            paths.sort();
            paths
        }

        #[test]
        fn workflow_fixtures_match_expected_deliveries() {
            let paths = workflow_fixture_dsl_paths();
            for path in &paths {
                let contents = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read fixture {path:?}: {error}"));
                let fixture = <WorkflowFixture as store::DocumentDsl>::parse_dsl(&contents).unwrap_or_else(|error| panic!("parse fixture {path:?}: {error}"));
                let dirty: HashSet<String> = fixture.dirty_instance_ids.iter().cloned().collect();
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
                let spk_name = if file_name.starts_with('🗣️') { file_name.replacen('🗣️', "📦️", 1).replace(".dsl", ".spk") } else { file_name.replace(".dsl", ".spk") };
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

    use crate::instance::{media_port_id_for_spec, OsParameterFieldSpec};
    use semio_framework_core::{AppDefinition, ConfigSpec, MediaClass, MediaForm, MediaType, ModeDefinition, OsMediaCapability, OsMediaFormat, PluginManifest, WorkflowDefinition, ArtifactKindSpec, WindowKindDefinition};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::sync::{LazyLock, Mutex};
    use ui_wgpu::SurfaceKind;

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

    /// @emoji 🔗️ Returns whether two resource kinds are interchangeable.
    pub fn resources_compatible(left: &str, right: &str) -> bool {
        left == right
    }
    //#endregion 🔖️ResourceDescriptors

    //#region 🔖️PluginRegistry
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsPortSpec {
        pub id: String,
        pub label: String,
        pub artifact_kind: OsArtifactKindId,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub required: Option<bool>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsAppRegistration {
        pub id: String,
        pub label: String,
        pub document: Vec<String>,
        pub controller_id: String,
        pub inputs: Vec<OsPortSpec>,
        pub outputs: Vec<OsPortSpec>,
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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsWorkflowDefinition {
        pub id: String,
        pub name: String,
        pub api_version: String,
        pub apps: Vec<OsAppRegistration>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsPlatformAppInput {
        pub id: String,
        pub label: String,
        pub document: Vec<String>,
        pub controller_id: String,
        pub modes: Vec<ModeDefinition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_mode_id: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsPlatformInput {
        pub id: String,
        pub name: String,
        #[serde(default = "default_api_version")]
        pub api_version: String,
        pub apps: Vec<OsPlatformAppInput>,
    }

    fn default_api_version() -> String {
        "1".into()
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OsAppResourceSpec {
        pub inputs: Vec<OsPortSpec>,
        pub outputs: Vec<OsPortSpec>,
        pub source_format: String,
        pub component_kind: String,
        pub modes: Vec<ModeDefinition>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub default_mode_id: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub parameter_fields: Vec<OsParameterFieldSpec>,
        /// 🧮️ Threaded straight through into `OsAppRegistration::config` by `merge_os_plugin_definition`.
        #[serde(default)]
        pub config: ConfigSpec,
    }

    pub fn os_out_port(artifact_kind: &str, id: &str, label: &str) -> OsPortSpec {
        OsPortSpec { id: id.into(), label: label.into(), artifact_kind: artifact_kind.into(), required: None }
    }

    pub fn os_in_port(artifact_kind: &str, id: &str, label: &str, required: bool) -> OsPortSpec {
        OsPortSpec { id: id.into(), label: label.into(), artifact_kind: artifact_kind.into(), required: Some(required) }
    }

    pub fn os_app_primary_output_kind(registration: &OsAppRegistration) -> OsArtifactKindId {
        registration.outputs.first().map(|port| port.artifact_kind.clone()).unwrap_or_else(|| "graph.dag".into())
    }

    pub fn os_baseline_resource(artifact_kind: &str, source_format: &str, component_kind: &str) -> OsAppResourceSpec {
        OsAppResourceSpec {
            inputs: Vec::new(),
            outputs: vec![os_out_port(artifact_kind, "out", "Out")],
            source_format: source_format.into(),
            component_kind: component_kind.into(),
            modes: vec![ModeDefinition { id: "edit".into(), label: "Edit".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }],
            default_mode_id: None,
            parameter_fields: Vec::new(),
            config: ConfigSpec::empty(),
        }
    }

    static BUILTIN_WORKFLOWS: LazyLock<Mutex<Vec<OsWorkflowDefinition>>> = LazyLock::new(|| Mutex::new(Vec::new()));
    static EXTENSION_WORKFLOWS: LazyLock<Mutex<HashMap<String, OsWorkflowDefinition>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

    /// @emoji 📚️ Registers a built-in os program prepended to list_os_workflows.
    pub fn register_os_builtin_workflow(program: OsWorkflowDefinition) {
        let mut registry = BUILTIN_WORKFLOWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.iter().any(|entry| entry.id == program.id) {
            return;
        }
        registry.push(program);
    }

    /// @emoji 📚️ Registers a fully materialized os program definition.
    pub fn register_os_workflow_definition(program: OsWorkflowDefinition) {
        EXTENSION_WORKFLOWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(program.id.clone(), program);
    }

    /// @emoji 🧩️ Merges a platform definition into the os plugin registry with port metadata.
    pub fn merge_os_plugin_definition(plugin_id: &str, definition: &OsPlatformInput, artifact_by_app_id: &HashMap<String, OsAppResourceSpec>) -> Result<(), String> {
        let fallback_artifact = artifact_by_app_id.values().next().ok_or_else(|| format!("merge_os_plugin_definition requires resourceByAppId for {plugin_id}"))?.clone();
        let apps = definition
            .apps
            .iter()
            .map(|app| {
                let resource = artifact_by_app_id.get(&app.id).cloned().unwrap_or_else(|| fallback_artifact.clone());
                OsAppRegistration {
                    id: app.id.clone(),
                    label: app.label.clone(),
                    document: app.document.clone(),
                    controller_id: app.controller_id.clone(),
                    inputs: resource.inputs,
                    outputs: resource.outputs,
                    source_format: resource.source_format,
                    component_kind: resource.component_kind,
                    parameter_fields: resource.parameter_fields,
                    modes: if app.modes.is_empty() { resource.modes } else { app.modes.clone() },
                    default_mode_id: app.default_mode_id.clone().or(resource.default_mode_id),
                    config: resource.config,
                }
            })
            .collect();
        register_os_workflow_definition(OsWorkflowDefinition { id: plugin_id.into(), name: definition.name.clone(), api_version: definition.api_version.clone(), apps });
        Ok(())
    }

    /// @emoji 🌱️ Seeds the extension registry from a artifact map for tests and offline tooling.
    pub fn seed_os_plugin_registry_from_resource_map(resource_by_plugin: &HashMap<String, HashMap<String, OsAppResourceSpec>>) {
        let mut registry = EXTENSION_WORKFLOWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        for (plugin_id, resources) in resource_by_plugin {
            if registry.contains_key(plugin_id) {
                continue;
            }
            let name = plugin_id
                .split('.')
                .map(|segment| {
                    let mut chars = segment.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<Vec<_>>()
                .join(" ");
            let apps = resources
                .iter()
                .map(|(app_id, resource)| OsPlatformAppInput {
                    id: app_id.clone(),
                    label: {
                        let mut chars = app_id.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    },
                    document: vec!["semio".into(), app_id.to_lowercase()],
                    controller_id: format!("{}-play", plugin_id.replace('.', "-")),
                    modes: resource.modes.clone(),
                    default_mode_id: resource.default_mode_id.clone(),
                })
                .collect();
            let platform = OsPlatformInput { id: plugin_id.clone(), name, api_version: "1".into(), apps };
            drop(registry);
            let _ = merge_os_plugin_definition(plugin_id, &platform, resources);
            registry = EXTENSION_WORKFLOWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    pub fn list_os_workflows() -> Vec<OsWorkflowDefinition> {
        let builtins = BUILTIN_WORKFLOWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let extensions = EXTENSION_WORKFLOWS.lock().unwrap_or_else(std::sync::PoisonError::into_inner).values().cloned().collect::<Vec<_>>();
        builtins.into_iter().chain(extensions).collect()
    }

    pub fn os_workflow_by_id(plugin_id: &str) -> Option<OsWorkflowDefinition> {
        list_os_workflows().into_iter().find(|program| program.id == plugin_id)
    }

    pub fn os_app_registration(plugin_id: &str, app_id: &str) -> Option<OsAppRegistration> {
        os_workflow_by_id(plugin_id)?.apps.into_iter().find(|app| app.id == app_id)
    }

    /// @emoji 🧩️ Resolves the AppDefinition backing an embedded os app instance. Returns `None` if the
    /// registration declares zero modes — every app must declare at least one, so an ad hoc "inject a
    /// fake edit mode" fallback would just hide a mis-registered app instead of surfacing it. An embedded
    /// os app instance renders through exactly one component surface, so this synthesizes the single
    /// window kind that represents it rather than leaving `window_kinds` empty (now impossible).
    pub fn resolve_os_app_definition(plugin_id: &str, app_id: &str) -> Option<AppDefinition> {
        let registration = os_app_registration(plugin_id, app_id)?;
        let program = os_workflow_by_id(plugin_id)?;
        let app = program.apps.iter().find(|entry| entry.id == app_id)?;
        let modes = semio_framework_core::Modes::try_from(app.modes.clone()).ok()?;
        let default_mode_id = app.default_mode_id.clone().or_else(|| registration.default_mode_id.clone()).unwrap_or_else(|| modes.first().id.clone());
        let window_kinds = semio_framework_core::WindowKinds::one(WindowKindDefinition {
            id: registration.component_kind.clone(),
            label: registration.label.clone(),
            body_key: registration.component_kind.clone(),
            surface_kind: SurfaceKind::Canvas2d,
            icon_id: "app-window".into(),
            options: ui_wgpu::WindowOptions::default(),
            actions: Vec::new(),
            utilities: Vec::new(),
            params_schema: None,
            document_projection_schema: None,
            input_event_schema: None,
            output_schema: None,
            capabilities: Vec::new(),
        });
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
        })
    }

    pub fn workflow_node_ports_for_registration(instance_id: &str, registration: &OsAppRegistration) -> (Vec<crate::workflow::OsMediaPort>, Vec<crate::workflow::OsMediaPort>) {
        let inputs = registration.inputs.iter().map(|spec| crate::workflow::OsMediaPort { id: media_port_id_for_spec(instance_id, &spec.id, "in"), artifact_kind: spec.artifact_kind.clone(), direction: "in".into() }).collect();
        let outputs = registration.outputs.iter().map(|spec| crate::workflow::OsMediaPort { id: media_port_id_for_spec(instance_id, &spec.id, "out"), artifact_kind: spec.artifact_kind.clone(), direction: "out".into() }).collect();
        (inputs, outputs)
    }
    //#endregion 🔖️PluginRegistry

    //#region 🔖️PluginRegistry
    pub struct PluginRegistry {
        apps: HashMap<String, AppDefinition>,
        workflows: HashMap<String, WorkflowDefinition>,
    }

    impl Default for PluginRegistry {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PluginRegistry {
        pub fn new() -> Self {
            Self { apps: HashMap::new(), workflows: HashMap::new() }
        }

        pub fn register_app(&mut self, app: AppDefinition) {
            self.apps.insert(app.id.clone(), app);
        }

        pub fn register_workflow(&mut self, workflow: WorkflowDefinition) {
            self.workflows.insert(workflow.workflow_step_id.clone(), workflow);
        }

        pub fn find_app(&self, app_id: &str) -> Option<&AppDefinition> {
            self.apps.get(app_id)
        }

        pub fn find_workflow(&self, workflow_step_id: &str) -> Option<&WorkflowDefinition> {
            self.workflows.get(workflow_step_id)
        }

        pub fn apps(&self) -> Vec<AppDefinition> {
            self.apps.values().cloned().collect()
        }

        pub fn workflows(&self) -> Vec<WorkflowDefinition> {
            self.workflows.values().cloned().collect()
        }
    }
    //#endregion 🔖️PluginRegistry

    //#region 🧪️Tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn merges_plugin_definition_with_resource_map() {
            let mut resources = HashMap::new();
            resources.insert("draw".into(), os_baseline_resource("2d.drawing", "draw.document", "draw"));
            let mut by_plugin = HashMap::new();
            by_plugin.insert("draw".into(), resources);
            seed_os_plugin_registry_from_resource_map(&by_plugin);
            let registration = os_app_registration("draw", "draw").expect("registration");
            assert_eq!(registration.source_format, "draw.document");
        }
    }
    //#endregion 🧪️Tests
    // #endregion registry
}

#[cfg(not(target_arch = "wasm32"))]
pub use backbone::{open_file_space_backbone, open_folder_space_backbone};
pub use host::{
    apply_os_operation, create_empty_os_document, create_ephemeral_os_space, create_os_space, decode_os_space_payload, default_os_projection, delete_os_space, encode_os_space_payload, export_os_space_pack, import_os_space_from_dsl,
    list_os_space_catalog_entries, load_os_space_document, materialize_os_projection, os_document_from_json,
    os_document_to_json, seed_os_space_catalog_if_empty, LoadedProgram, OsBackbonePort, OsDiff, OsDocument, OsEnvelope, OsOperation, OsProjection, OsStore, OsSpaceCatalogEntry, OsVcs, PluginHost, ProgramHotSwapEvent, ProgramSupervisorState,
    OS_HOME_VFS_ROOT_ID, OS_SPACE_BACKBONE_URI_PREFIX,
};
pub use instance::{
    apply_parameter_values_to_projection, create_default_os_parameter, create_os_document_id, create_os_id, is_parameter_port_id, materialize_os_app_instance_document_json, media_port_id_for_spec, media_port_spec_id, os_fixture_json,
    os_parameter_types_compatible, os_parameter_value, parameter_id_from_port_id, parameter_port_id, patch_os_parameter, register_os_fixture_json, resolve_parameter_values_for_instance, set_json_pointer_value, OsAppInstance, OsDocumentRef,
    OsInstanceState, OsParameter, OsParameterFieldBinding, OsParameterFieldSpec, OsParameterType, OS_PARAMETER_PORT_PREFIX,
};
pub use media_export_raster::{
    dwg_drawing_to_svg, export_registered_solid, import_registered_solid, rasterize_svg_to_png_base64, register_2d_export_handlers, register_dwg_import_handler, register_mesh_dwg_export_handler, register_mesh_dwg_import_handler,
    register_mesh_exporter, register_mesh_importer, register_solid_exporter, register_solid_importer, solid_exporter_for, svg_to_dwg_bytes,
};
pub use media_export_simple::{map_points_svg, pages_rects_svg, title_card_svg, wrap_svg};
pub use workflow::{
    apply_flow_fixture_to_os_workflow, assert_os_media_export_coverage, assert_os_media_import_coverage, build_os_workflow_operator_infos, empty_workflow, export_os_app_instance_media, import_os_app_instance_media,
    list_os_workflow_vfs_children, workflow_node_for_instance, negotiate_media_contract, os_media_export_extension_for_format, os_workflow_to_flow_fixture, os_workflow_to_node_graph_payload, os_workflow_vfs_export_id,
    os_workflow_vfs_import_id, os_workflow_vfs_instance_folder_id, os_workflow_vfs_instance_id, os_workflow_vfs_schema, os_workflow_vfs_source_id, os_media_neuron_kind_for_node, os_resource_media_capability,
    placeholder_media_contract, register_os_media_export_handler, register_os_media_import_handler, required_os_media_export_formats, required_os_media_import_formats, sync_workflow_parameter_ports, validate_workflow, MediaContract,
    WorkflowPosition, WorkflowValidation, OsMediaCapability, OsMediaExportResult, OsWorkflowOperatorInfo, OsMediaFormat, OsWorkflow, OsWorkflowCamera, OsWorkflowEdge, OsWorkflowNode, OsWorkflowVfsNodeRecord, OsWorkflowVfsSchema,
    OsWorkflowNodeGraphPayload, OsMediaPort, WorkflowInstanceRegistry, OS_MEDIA_FLOW_MODULE_ID, OS_WORKFLOW_SCHEMA, OS_WORKFLOW_VFS_ROOT_ID, OS_SPACE_SCHEMA,
    WorkflowDelivery, WorkflowFixture,
};
pub use registry::{
    list_os_workflows, list_os_artifact_descriptors, merge_os_plugin_definition, os_app_primary_output_kind, os_app_registration, os_baseline_resource, os_in_port, os_out_port, os_workflow_by_id, os_artifact_descriptor, register_os_builtin_workflow,
    register_os_workflow_definition, register_artifact_descriptor, register_artifact_descriptors, resolve_os_app_definition, resources_compatible, seed_os_plugin_registry_from_resource_map, OsAppRegistration, OsAppResourceSpec, OsPlatformAppInput,
    OsPlatformInput, OsPortSpec, OsWorkflowDefinition, OsArtifactDescriptor, OsArtifactKindId, PluginRegistry,
};
pub use semio_framework_core::*;
pub use ui_wgpu::*;
pub use vcs::{Author, Checkpoint, VcsError};
pub use store::{document_backbone_ref, set_host_backbone_port, DocumentBackboneRef, DocumentCommand, LocalStorageBackbonePort, MemoryBackbonePort};
