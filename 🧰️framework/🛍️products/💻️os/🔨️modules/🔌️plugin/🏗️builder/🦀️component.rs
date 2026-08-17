//! 🏗️ Typestate `PluginBuilder` — missing label/version is a compile error.

use crate::app::{
    App, ArtifactApp, ArtifactContribution, ArtifactDeclaration, ArtifactDefinitionRegistry, FlowExtensionDeclaration, FlowExtensionExecutableIdentity, FlowExtensionManifest, HostMediaHandlerDeclaration, Plugin, PluginApp, PluginAssemblyError,
    PluginCommandHandler,
};
use semio_framework::{kernel::CapabilityRequirement, CommandDefinition};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::marker::PhantomData;

/// 🏷️ Builder has plugin id only — next call must be `.label(...)`.
pub struct NeedsLabel;
/// 🏷️ Builder has id + label — next call must be `.version(...)`.
pub struct NeedsVersion;
/// ✅️ Builder has id + label + version — ready for apps, capabilities, and typed assembly.
pub struct Ready;

/// 🏗️ Fluent plugin constructor with typestate gates for identity fields.
pub struct PluginBuilder<State> {
    plugin_id: String,
    label: Option<String>,
    version: Option<String>,
    artifacts: Vec<ArtifactDeclaration>,
    artifact_definitions: Vec<crate::app::ArtifactDefinition>,
    capabilities: Vec<CapabilityRequirement>,
    commands: Vec<(CommandDefinition, PluginCommandHandler)>,
    artifact_kinds: Vec<semio_framework::ArtifactKindSpec>,
    host_media_handlers: Vec<HostMediaHandlerDeclaration>,
    flow_extensions: Vec<FlowExtensionDeclaration>,
    foreign_document_codecs: Vec<crate::app::DocumentCodecSpec>,
    /// 🔗️ Direct plugin dependencies — contract freeze §3/§4; gate-checked in `try_build` via
    /// `crate::app::register_contributions`.
    dependencies: Vec<semio_framework::PluginDependency>,
    /// 🗂️ Contributions onto artifact kinds owned by a dependency — resolved against `plugin_id` in
    /// `try_build`, once it is known to be final.
    contributions: Vec<ArtifactContribution>,
    /// 📖️ One non-capturing `(document_schema, kinds)` provider per `.document_app_mutation_roster::
    /// <A>()`/`.viewer_mutation_roster::<V>()`/`.editor_mutation_roster::<E>()` call — committed into
    /// the process-wide owner mutation roster by `try_build`.
    owner_mutation_rosters: Vec<fn() -> (&'static str, &'static [protocol::SemanticDescriptor])>,
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
    app_defs: Vec<(App, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>)>,
    app_schema_descriptors: Vec<fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>>,
    document_app_ids: Vec<&'static str>,
    _state: PhantomData<State>,
}

impl PluginBuilder<NeedsLabel> {
    /// 🪪 Starts a plugin builder from a stable plugin id.
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            label: None,
            version: None,
            artifacts: Vec::new(),
            artifact_definitions: Vec::new(),
            capabilities: Vec::new(),
            commands: Vec::new(),
            artifact_kinds: Vec::new(),
            host_media_handlers: Vec::new(),
            flow_extensions: Vec::new(),
            foreign_document_codecs: Vec::new(),
            dependencies: Vec::new(),
            contributions: Vec::new(),
            owner_mutation_rosters: Vec::new(),
            apps: HashMap::new(),
            app_defs: Vec::new(),
            app_schema_descriptors: Vec::new(),
            document_app_ids: Vec::new(),
            _state: PhantomData,
        }
    }

    /// 🏷️ Sets the human-readable plugin label.
    pub fn label(self, label: impl Into<String>) -> PluginBuilder<NeedsVersion> {
        PluginBuilder {
            plugin_id: self.plugin_id,
            label: Some(label.into()),
            version: None,
            artifacts: self.artifacts,
            artifact_definitions: self.artifact_definitions,
            capabilities: self.capabilities,
            commands: self.commands,
            artifact_kinds: self.artifact_kinds,
            host_media_handlers: self.host_media_handlers,
            flow_extensions: self.flow_extensions,
            foreign_document_codecs: self.foreign_document_codecs,
            dependencies: self.dependencies,
            contributions: self.contributions,
            owner_mutation_rosters: self.owner_mutation_rosters,
            apps: self.apps,
            app_defs: self.app_defs,
            app_schema_descriptors: self.app_schema_descriptors,
            document_app_ids: self.document_app_ids,
            _state: PhantomData,
        }
    }
}

impl PluginBuilder<NeedsVersion> {
    /// 🏷️ Sets the plugin version string.
    pub fn version(self, version: impl Into<String>) -> PluginBuilder<Ready> {
        PluginBuilder {
            plugin_id: self.plugin_id,
            label: self.label,
            version: Some(version.into()),
            artifacts: self.artifacts,
            artifact_definitions: self.artifact_definitions,
            capabilities: self.capabilities,
            commands: self.commands,
            artifact_kinds: self.artifact_kinds,
            host_media_handlers: self.host_media_handlers,
            flow_extensions: self.flow_extensions,
            foreign_document_codecs: self.foreign_document_codecs,
            dependencies: self.dependencies,
            contributions: self.contributions,
            owner_mutation_rosters: self.owner_mutation_rosters,
            apps: self.apps,
            app_defs: self.app_defs,
            app_schema_descriptors: self.app_schema_descriptors,
            document_app_ids: self.document_app_ids,
            _state: PhantomData,
        }
    }
}

impl PluginBuilder<Ready> {
    /// 🗿️ Declares one artifact this plugin owns. Repeatable. `try_build()` walks every
    /// declared artifact in a fixed deterministic order and validates that it owns everything it
    /// declares — see `ArtifactDeclaration::preflight`.
    pub fn artifact(mut self, declaration: ArtifactDeclaration) -> Self {
        self.artifacts.push(declaration);
        self
    }

    /// 🧾️ Registers one definition-only artifact through the same typed preflight registry.
    pub fn artifact_definition(mut self, definition: crate::app::ArtifactDefinition) -> Self {
        self.artifact_definitions.push(definition);
        self
    }

    /// 🔒️ Declares a capability requirement.
    pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self
    }

    /// 🎲️ Declares local backbone read+write at plugin scope.
    pub fn local_backbone_storage(self) -> Self {
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        self.capability(CapabilityRequirement { artifact: ArtifactKind::Backbone, rights: Rights::Read, scope: Scope::Plugin }).capability(CapabilityRequirement { artifact: ArtifactKind::Backbone, rights: Rights::Write, scope: Scope::Plugin })
    }

    /// 🎮️ Declares a plugin-owned command and its program-level handler.
    pub fn plugin_command(mut self, command: CommandDefinition, handler: PluginCommandHandler) -> Self {
        self.commands.push((command, handler));
        self
    }

    /// 🗂️ Declares one plugin-level artifact kind for library (zero-app) plugins. Repeatable.
    pub fn artifact_kind(mut self, spec: semio_framework::ArtifactKindSpec) -> Self {
        self.artifact_kinds.push(spec);
        self
    }

    /// 🧭️ Declares an owned OS-media bridge or export renderer as frozen runtime authority.
    pub fn host_media_handler(mut self, declaration: HostMediaHandlerDeclaration) -> Self {
        self.host_media_handlers.push(declaration);
        self
    }

    /// 🌊️ Declares one immutable `flow.extension` executable descriptor for runtime catalogue merging.
    pub fn flow_extension(mut self, declaration: FlowExtensionDeclaration) -> Self {
        self.flow_extensions.push(declaration);
        self
    }

    /// 🗂️ Declares an app-owned codec under a foreign document schema for the aggregate codec commit.
    pub fn foreign_document_codec<A: ArtifactApp>(mut self, schema: impl Into<String>) -> Self {
        self.foreign_document_codecs.push(crate::app::DocumentCodecSpec::foreign::<A>(schema));
        self
    }

    /// 🔗️ Declares a direct plugin dependency this plugin requires to load — contract freeze §3/§4.
    /// Repeatable; order matters only for extensions (`ExtensionBundle::extends` must equal
    /// `dependencies[0].plugin_id`), which plain plugins have no equivalent constraint for.
    pub fn depends_on(mut self, plugin_id: impl Into<String>, version: semio_framework::VersionReq) -> Self {
        self.dependencies.push(semio_framework::PluginDependency::new(plugin_id, version));
        self
    }

    /// 🗂️ Declares one contribution of mutations/inferences onto an artifact kind owned by a
    /// dependency. Resolved against this plugin's own id and gate-checked (contract freeze §4) at
    /// `try_build()`, once every declared dependency is final.
    pub fn contributes(mut self, contribution: ArtifactContribution) -> Self {
        self.contributions.push(contribution);
        self
    }

    /// 🧬️ Declares a typed document app factory and app-schema descriptor for transactional assembly.
    /// No `SemanticMutation` bound here — `ArtifactApp` itself only requires plain `protocol::
    /// Mutation` (mirrors `.editor()`/`.viewer()`, ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-
    /// SUBSET contract §2.2). A framework-owned document app (e.g. a workflow-backed studio) may have
    /// a `Mutation` type with no `SemanticMutation` impl at all; `document_app` still registers and
    /// routes it. See `document_app_mutation_roster` for the separate opt-in `contributor.list-
    /// artifact-mutations` capability (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-
    /// STUDIOS, lane 2-0 — the bound was blocking every non-`SemanticMutation` document app, e.g.
    /// `semio-s-plugin-space`'s `SpaceApp`/`WorkflowMutation` and `semio-s-plugin-playbook-procedural`'s
    /// `ModuleApp`/`ModulePayloadMutation`, from linking at all).
    pub fn document_app<A: ArtifactApp>(mut self, app: App) -> Self {
        fn app_schema<A: ArtifactApp>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            A::app_schema()
        }
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> = Box::new(move || Box::new(crate::app::VcsArtifactApp::with_registry(A::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self.app_schema_descriptors.push(app_schema::<A>);
        self.document_app_ids.push(A::APP_ID);
        self
    }

    /// 🗂️ Opt-in: registers `A`'s owner-mutation roster with `contributor.list-artifact-mutations`
    /// (the "owner half", `crate::app::commit_owner_mutation_roster`) — see `viewer_mutation_roster`/
    /// `editor_mutation_roster`. Requires `A::Mutation: SemanticMutation<A::Snapshot>`. Chain right
    /// after `.document_app::<A>(app)` for a document app whose `Mutation` already derives it; skip
    /// it for the rest — they still register and route through `.document_app::<A>(app)` alone, they
    /// just do not contribute a roster row.
    pub fn document_app_mutation_roster<A: ArtifactApp>(mut self) -> Self
    where
        A::Mutation: protocol::SemanticMutation<A::Snapshot>,
    {
        /// 📖️ Non-capturing thunk pairing `A::DOCUMENT_SCHEMA` with its `SemanticMutation::kinds()`
        /// table — `try_build()` commits these into the process-wide owner mutation roster
        /// (`crate::app::commit_owner_mutation_roster`), the "owner half" of
        /// `contributor.list-artifact-mutations`.
        fn owner_mutation_roster<A: ArtifactApp>() -> (&'static str, &'static [protocol::SemanticDescriptor])
        where
            A::Mutation: protocol::SemanticMutation<A::Snapshot>,
        {
            (A::DOCUMENT_SCHEMA, <A::Mutation as protocol::SemanticMutation<A::Snapshot>>::kinds())
        }
        self.owner_mutation_rosters.push(owner_mutation_roster::<A>);
        self
    }

    //#region 🔖️Surfaces
    /// 👁️ Declares a typed viewer app factory (read-only surface) — the `ArtifactViewer` twin of
    /// `document_app` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4/§2.6).
    /// `def` is `Viewer::builder(V::DIALECT)...build_definition()` — already carries the derived id,
    /// `role`, and `dialect`. No `SemanticMutation` bound here — `ArtifactViewer` itself only
    /// requires plain `protocol::Mutation` (contract §2.2, decode-only); a surface always registers
    /// and routes regardless of what `V::Mutation` is. See `viewer_mutation_roster` for the separate
    /// opt-in `contributor.list-artifact-mutations` capability (ticket 26/08/16/ARTIFACT-VIEWERS-
    /// AND-EDITORS-PER-SUBSET report `📓️w2-sdk2-report.md`).
    pub fn viewer<V: crate::app::ArtifactViewer>(mut self, def: crate::app::AppDefinition) -> Self {
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        fn app_schema<V: crate::app::ArtifactViewer>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            V::app_schema()
        }
        let app = App { definition: def, examples: Vec::new() };
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> = Box::new(move || Box::new(crate::app::VcsArtifactApp::with_registry(crate::app::ViewerApp::<V>::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self.app_schema_descriptors.push(app_schema::<V>);
        // 🔒️ Contract §2.3 clause 4 — a viewer's document store attaches Read only, never Write.
        self.capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Read, scope: Scope::App })
    }

    /// 🗂️ Opt-in: registers `V`'s owner-mutation roster with `contributor.list-artifact-mutations`
    /// (the "owner half", `crate::app::commit_owner_mutation_roster`). Requires
    /// `V::Mutation: SemanticMutation<V::Snapshot>` — implemented only by `#[derive(Mutations)]`,
    /// not yet every dispatch enum. Chain right after `.viewer::<V>(def)` for a subset whose
    /// `Mutation` already derives it; skip it for the rest — they still register and route through
    /// `.viewer::<V>(def)` alone, they just do not contribute a roster row.
    pub fn viewer_mutation_roster<V: crate::app::ArtifactViewer>(mut self) -> Self
    where
        V::Mutation: protocol::SemanticMutation<V::Snapshot>,
    {
        fn owner_mutation_roster<V: crate::app::ArtifactViewer>() -> (&'static str, &'static [protocol::SemanticDescriptor])
        where
            V::Mutation: protocol::SemanticMutation<V::Snapshot>,
        {
            (V::DOCUMENT_SCHEMA, <V::Mutation as protocol::SemanticMutation<V::Snapshot>>::kinds())
        }
        self.owner_mutation_rosters.push(owner_mutation_roster::<V>);
        self
    }

    /// ✏️ Declares a typed editor app factory (mutation-capable surface) — the `ArtifactEditor` twin
    /// of `document_app`. `def` is `Editor::builder(E::DIALECT)...build_definition()`. No
    /// `SemanticMutation` bound — see `viewer` above and `editor_mutation_roster` below.
    pub fn editor<E: crate::app::ArtifactEditor>(mut self, def: crate::app::AppDefinition) -> Self {
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        fn app_schema<E: crate::app::ArtifactEditor>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            E::app_schema()
        }
        let app = App { definition: def, examples: Vec::new() };
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> = Box::new(move || Box::new(crate::app::VcsArtifactApp::with_registry(crate::app::EditorApp::<E>::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self.app_schema_descriptors.push(app_schema::<E>);
        // 🔒️ Contract §2.3 clause 4 — an editor's document store attaches both Read and Write.
        self.capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Read, scope: Scope::App }).capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Write, scope: Scope::App })
    }

    /// 🗂️ Opt-in: registers `E`'s owner-mutation roster — see `viewer_mutation_roster`.
    pub fn editor_mutation_roster<E: crate::app::ArtifactEditor>(mut self) -> Self
    where
        E::Mutation: protocol::SemanticMutation<E::Snapshot>,
    {
        fn owner_mutation_roster<E: crate::app::ArtifactEditor>() -> (&'static str, &'static [protocol::SemanticDescriptor])
        where
            E::Mutation: protocol::SemanticMutation<E::Snapshot>,
        {
            (E::DOCUMENT_SCHEMA, <E::Mutation as protocol::SemanticMutation<E::Snapshot>>::kinds())
        }
        self.owner_mutation_rosters.push(owner_mutation_roster::<E>);
        self
    }
    //#endregion 🔖️Surfaces

    /// 📚️ Assembles a library-only plugin through the typed boundary.
    pub fn try_library(self) -> Result<Plugin, PluginAssemblyError> {
        self.try_build()
    }

    /// ✅️ Builds plugin-local runtime authority before one all-registry commit.
    pub fn try_build(self) -> Result<Plugin, PluginAssemblyError> {
        let Self {
            plugin_id,
            label,
            version,
            artifacts,
            artifact_definitions,
            capabilities,
            commands,
            artifact_kinds,
            host_media_handlers,
            flow_extensions,
            foreign_document_codecs,
            dependencies,
            contributions,
            owner_mutation_rosters,
            apps: _,
            app_defs,
            app_schema_descriptors,
            document_app_ids,
            _state: _,
        } = self;
        let label = label.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.label", "typestate-ready builder has no label"))?;
        let version = version.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.version", "typestate-ready builder has no version"))?;
        let mut definitions = ArtifactDefinitionRegistry::new();
        for definition in artifact_definitions {
            definitions.register(definition).map_err(PluginAssemblyError::definition)?;
        }
        for declaration in &artifacts {
            declaration.preflight(&plugin_id, &mut definitions)?;
        }
        let mut declared_media_kinds = BTreeMap::new();
        for spec in artifact_kinds.iter().chain(app_defs.iter().flat_map(|(app, _)| app.definition.artifact_kinds.iter())) {
            if spec.id.trim().is_empty() || spec.schema.trim().is_empty() {
                return Err(PluginAssemblyError::new("plugin-assembly.media-kind", "artifact-kind contributions require non-empty id and schema"));
            }
            if let Some(existing) = declared_media_kinds.get(&spec.id) {
                if existing != spec {
                    return Err(PluginAssemblyError::new("plugin-assembly.media-kind", format!("artifact kind {:?} has conflicting descriptors", spec.id)));
                }
            } else {
                declared_media_kinds.insert(spec.id.clone(), spec.clone());
            }
        }
        for declaration in &host_media_handlers {
            declaration.preflight(&plugin_id, &declared_media_kinds)?;
        }
        for declaration in &flow_extensions {
            declaration.preflight(&plugin_id)?;
        }
        let document_app_ids: BTreeSet<_> = document_app_ids.into_iter().collect();
        for codec in &foreign_document_codecs {
            codec.preflight_foreign(&document_app_ids)?;
        }
        let mut app_schemas = Vec::new();
        for get_schema in app_schema_descriptors {
            if let Some(descriptor) = get_schema() {
                app_schemas.push(descriptor);
            }
        }
        let plan = crate::app::ArtifactRegistrationPlan::from_declarations(&artifacts, app_schemas, foreign_document_codecs, &plugin_id, host_media_handlers, flow_extensions);
        let (mut runtime, registry_plan) = plan.into_runtime(definitions)?;

        // 🗂️ Resolve every declared contribution against this plugin's own (now-final) id — pure,
        // no registry side effects — then gate-check the WHOLE candidate set (contract freeze §4)
        // before anything commits.
        let mut contribution_descriptors = Vec::with_capacity(contributions.len());
        let mut contributed_inference_services = Vec::new();
        let mut contributed_mutation_runtime = Vec::new();
        for contribution in contributions {
            let (descriptor, inference_services, mutation_runtime) = contribution.resolve(&plugin_id);
            contribution_descriptors.push(descriptor);
            contributed_inference_services.extend(inference_services);
            contributed_mutation_runtime.extend(mutation_runtime);
        }
        crate::app::register_contributions(&plugin_id, &dependencies, &contribution_descriptors).map_err(|error| PluginAssemblyError::new("plugin-assembly.contribution-gate", error.to_string()))?;
        runtime.extend_contributions(contributed_inference_services, &owner_mutation_rosters, contributed_mutation_runtime)?;

        let mut plugin = Plugin::new(plugin_id.clone(), label, version).with_runtime_registry(runtime);
        plugin.manifest.dependencies = dependencies;
        plugin.manifest.contributions = contribution_descriptors;
        for declaration in artifacts {
            plugin = declaration.apply_to(plugin);
        }
        for capability in capabilities {
            plugin = plugin.capability(capability);
        }
        for (command, handler) in commands {
            plugin = plugin.plugin_command(command, handler);
        }
        for kind in artifact_kinds {
            plugin = plugin.artifact_kind(kind);
        }
        for (app, factory) in app_defs {
            plugin = plugin.register_app_factory(app, factory);
        }
        let assembly = store::begin_artifact_assembly().map_err(|error| PluginAssemblyError::new("plugin-assembly.unavailable", error.to_string()))?;
        crate::app::commit_artifact_registration_plan(&assembly, registry_plan)?;
        Ok(plugin)
    }
}

impl Plugin {
    /// 🏗️ Starts a typestate plugin builder from a stable plugin id.
    pub fn builder(plugin_id: impl Into<String>) -> PluginBuilder<NeedsLabel> {
        PluginBuilder::new(plugin_id)
    }
}

#[cfg(test)]
mod plugin_builder_dependency_tests {
    use super::*;
    use crate::app::ArtifactContribution;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static MESH_DWG_EXECUTIONS: AtomicUsize = AtomicUsize::new(0);

    fn host_media_kind() -> semio_framework::ArtifactKindSpec {
        semio_framework::ArtifactKindSpec {
            id: "3d.builder-test".into(),
            name: "Builder Test 3D".into(),
            source_format: "semio.builder-test.mesh/v1".into(),
            component_kind: "builder-test".into(),
            dimension: "3d".into(),
            media_capability: semio_framework::OsMediaCapability::MeshOnly,
            media_type: semio_framework::MediaType { class: semio_framework::MediaClass::ThreeD, form: semio_framework::MediaForm::Mesh },
            schema: "semio.builder-test.mesh/v1".into(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            export_stdio_kinds: Vec::new(),
            import_stdio_kinds: Vec::new(),
        }
    }

    fn counting_mesh_dwg_importer(_mesh: &semio_framework::MeshData) -> Result<serde_json::Value, String> {
        MESH_DWG_EXECUTIONS.fetch_add(1, Ordering::SeqCst);
        Ok(serde_json::json!({ "bridge": "counting" }))
    }

    fn alternate_mesh_dwg_importer(_mesh: &semio_framework::MeshData) -> Result<serde_json::Value, String> {
        MESH_DWG_EXECUTIONS.fetch_add(100, Ordering::SeqCst);
        Ok(serde_json::json!({ "bridge": "alternate" }))
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct DependencyTestSnapshot {
        value: i32,
    }
    impl store::ArtifactPack for DependencyTestSnapshot {
        fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
            serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
        }
        fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
            serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
    struct DependencyTestDiff {
        delta: i32,
    }
    impl protocol::MutationDiff<DependencyTestSnapshot> for DependencyTestDiff {
        fn apply(&self, base: &DependencyTestSnapshot) -> protocol::MutationApplyResult<DependencyTestSnapshot> {
            Ok(DependencyTestSnapshot { value: base.value + self.delta })
        }
        fn absorb(&mut self, other: Self) {
            self.delta += other.delta;
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    enum DependencyTestOp {
        Add(i32),
    }
    impl protocol::Mutation<DependencyTestSnapshot> for DependencyTestOp {
        type Diff = DependencyTestDiff;
        fn diff(&self, _base: &DependencyTestSnapshot) -> protocol::MutationOutcome<DependencyTestDiff> {
            let DependencyTestOp::Add(delta) = self;
            protocol::MutationOutcome::new(DependencyTestDiff { delta: *delta })
        }
        fn inverse(&self, _base: &DependencyTestSnapshot) -> Vec<Self> {
            let DependencyTestOp::Add(delta) = self;
            vec![DependencyTestOp::Add(-delta)]
        }
    }
    impl protocol::OpBinary for DependencyTestOp {
        fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
            Ok(serde_json::to_vec(self).expect("dependency test op always encodes"))
        }
        fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
            serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()).into())
        }
    }

    #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct DependencyTestMutationKind {
        delta: i32,
    }
    impl protocol::CompositeMutationKind<DependencyTestSnapshot, DependencyTestOp> for DependencyTestMutationKind {
        const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "value", kind: "add-value", record: "AddedValue" };
        fn plan(&self, _base: &DependencyTestSnapshot, planner: &mut protocol::Planner<DependencyTestSnapshot, DependencyTestOp>) -> Result<(), protocol::PlanError> {
            planner.call(DependencyTestOp::Add(self.delta))
        }
        fn label(&self) -> String {
            format!("Add {} to value", self.delta)
        }
    }

    fn contribution(target_artifact_kind: &str) -> ArtifactContribution {
        ArtifactContribution::builder(target_artifact_kind).mutation::<DependencyTestSnapshot, DependencyTestOp, DependencyTestMutationKind>("dep-target.document", 1, 1).build()
    }

    #[test]
    fn dependency_gating_rejects_a_contribution_onto_a_non_dependency() {
        let error = Plugin::builder("builder-test-contributor-missing-dep")
            .label("Builder Test Contributor Missing Dep")
            .version("0.1.0")
            .contributes(contribution("s.builder-test-dep-target.thing"))
            .try_build()
            .err()
            .expect("a contribution with no matching declared dependency must be rejected");
        assert_eq!(error.code, "plugin-assembly.contribution-gate");
        assert!(error.message.contains("not a direct dependency"), "unexpected message: {}", error.message);
    }

    #[test]
    fn a_direct_dependency_permits_its_contribution_and_lands_on_the_manifest() {
        let plugin = Plugin::builder("builder-test-contributor-ok")
            .label("Builder Test Contributor Ok")
            .version("0.1.0")
            .depends_on("builder-test-dep-target-ok", semio_framework::VersionReq::Any)
            .contributes(contribution("s.builder-test-dep-target-ok.thing"))
            .try_build()
            .expect("a contribution onto a direct dependency must be accepted");
        assert_eq!(plugin.manifest.dependencies.len(), 1);
        assert_eq!(plugin.manifest.dependencies[0].plugin_id, "builder-test-dep-target-ok");
        assert_eq!(plugin.manifest.contributions.len(), 1);
        assert_eq!(plugin.manifest.contributions[0].artifact_kind, "s.builder-test-dep-target-ok.thing");
        assert_eq!(plugin.manifest.contributions[0].mutations[0].mutation_id, "dep-target.document#builder-test-contributor-ok:add-value");
    }

    #[test]
    fn host_media_contributions_are_idempotent_and_execute_only_at_runtime() {
        MESH_DWG_EXECUTIONS.store(0, Ordering::SeqCst);
        let kind = host_media_kind();
        let bridge = HostMediaHandlerDeclaration::mesh_dwg_bridge("builder-test.media.mesh-dwg", kind.clone(), kind.schema.clone(), counting_mesh_dwg_importer).expect("typed bridge declaration");
        let plugin = Plugin::builder("builder-test-media")
            .label("Builder Test Media")
            .version("0.1.0")
            .artifact_kind(kind.clone())
            .host_media_handler(bridge.clone())
            .host_media_handler(bridge)
            .try_build()
            .expect("identical frozen host-media declarations are idempotent");
        assert_eq!(MESH_DWG_EXECUTIONS.load(Ordering::SeqCst), 0, "assembly must never execute a media converter");
        assert_eq!(plugin.host_media_handlers().len(), 1);
        let result = plugin.import_mesh_dwg(crate::MeshDwgBridgeRequest { artifact_kind: kind.id.clone(), document_schema: kind.schema.clone(), mesh: semio_framework::MeshData::default() }).expect("runtime bridge execution");
        assert_eq!(result.document, serde_json::json!({ "bridge": "counting" }));
        assert_eq!(MESH_DWG_EXECUTIONS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn host_media_conflicts_reject_the_whole_candidate_before_execution() {
        MESH_DWG_EXECUTIONS.store(0, Ordering::SeqCst);
        let kind = host_media_kind();
        let first = HostMediaHandlerDeclaration::mesh_dwg_bridge("builder-test.media.first", kind.clone(), kind.schema.clone(), counting_mesh_dwg_importer).expect("first bridge");
        let second = HostMediaHandlerDeclaration::mesh_dwg_bridge("builder-test.media.second", kind.clone(), kind.schema.clone(), alternate_mesh_dwg_importer).expect("second bridge");
        let error = Plugin::builder("builder-test-media-conflict")
            .label("Builder Test Media Conflict")
            .version("0.1.0")
            .artifact_kind(kind)
            .host_media_handler(first)
            .host_media_handler(second)
            .try_build()
            .err()
            .expect("two executable identities may not own one host-media target");
        assert_eq!(error.code, "plugin-assembly.host-media-target");
        assert_eq!(MESH_DWG_EXECUTIONS.load(Ordering::SeqCst), 0, "a rejected aggregate must have no runtime side effect");
    }

    #[test]
    fn flow_extension_descriptors_are_idempotent_and_conflict_rejecting() {
        let manifest = FlowExtensionManifest::new("builder-test-flow", "Builder Test Flow", "0.1.0").expect("typed manifest");
        let executable = FlowExtensionExecutableIdentity::native("semio.builder-test.flow", "semio.builder-test.flow.module", "activate").expect("typed executable identity");
        let declaration = FlowExtensionDeclaration::new("builder-test.flow.contribution", manifest.clone(), executable.clone()).expect("flow declaration");
        let plugin = Plugin::builder("builder-test-flow").label("Builder Test Flow").version("0.1.0").flow_extension(declaration.clone()).flow_extension(declaration).try_build().expect("identical frozen flow declarations are idempotent");
        assert_eq!(plugin.flow_extensions().len(), 1);
        let conflict = FlowExtensionDeclaration::new("builder-test.flow.other", manifest, executable).expect("conflicting target descriptor");
        let error = Plugin::builder("builder-test-flow-conflict")
            .label("Builder Test Flow Conflict")
            .version("0.1.0")
            .flow_extension(
                plugin
                    .flow_extensions()
                    .into_iter()
                    .next()
                    .map(|descriptor| FlowExtensionDeclaration::new(descriptor.id, descriptor.manifest, descriptor.executable_identity))
                    .expect("a flow extension to rebuild from")
                    .expect("the rebuilt descriptor to be valid"),
            )
            .flow_extension(conflict)
            .try_build()
            .err()
            .expect("one flow extension id may have exactly one contribution owner");
        assert_eq!(error.code, "plugin-assembly.flow-extension-target");
    }
}
