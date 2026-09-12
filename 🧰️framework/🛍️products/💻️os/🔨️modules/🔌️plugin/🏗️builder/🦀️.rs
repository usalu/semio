//! 🏗️ Typestate `PluginBuilder` — missing label/version is a compile error.

use crate::app::{
    resolve_ready, App, ArtifactApp, ArtifactContribution, ArtifactDeclaration, ArtifactDefinitionRegistry, ArtifactInferenceServiceMetadata, FlowExtensionDeclaration, HostMediaHandlerDeclaration, Plugin, PluginApp, PluginAssemblyError,
    PluginCommandHandler,
};
use semio_framework::{
    kernel::{ActivationEvent, CapabilityRequest, CapabilityRequirement, QuotaSchema},
    AssetDeclaration, CommandDefinition, ExecutionMode, ExtensionPointDeclaration,
};
use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

/// 📏️ Field-by-field `QuotaSchema` merge — `incoming`'s `Some` fields win, `None` fields defer to
/// `base`. Mirrors `📓️design-abi.md` §5's own `QuotaTree` inherit rule (`None` at any level defers
/// to the next), applied here across repeated `PluginBuilder::quota(..)` calls on one builder
/// instance instead of across the os → plugin → extension → instance tree.
fn merge_quota_schema(base: QuotaSchema, incoming: QuotaSchema) -> QuotaSchema {
    QuotaSchema {
        memory_bytes: incoming.memory_bytes.or(base.memory_bytes),
        fuel_per_turn: incoming.fuel_per_turn.or(base.fuel_per_turn),
        turn_deadline_ms: incoming.turn_deadline_ms.or(base.turn_deadline_ms),
        tables: incoming.tables.or(base.tables),
        mailbox_len: incoming.mailbox_len.or(base.mailbox_len),
        message_bytes: incoming.message_bytes.or(base.message_bytes),
        outstanding_requests: incoming.outstanding_requests.or(base.outstanding_requests),
        timers: incoming.timers.or(base.timers),
        storage_bytes: incoming.storage_bytes.or(base.storage_bytes),
        network_bytes_per_min: incoming.network_bytes_per_min.or(base.network_bytes_per_min),
        ui_nodes: incoming.ui_nodes.or(base.ui_nodes),
        patch_bytes: incoming.patch_bytes.or(base.patch_bytes),
        patch_hz: incoming.patch_hz.or(base.patch_hz),
        blob_resident_bytes: incoming.blob_resident_bytes.or(base.blob_resident_bytes),
        gpu_ms_per_frame: incoming.gpu_ms_per_frame.or(base.gpu_ms_per_frame),
        background_ms_per_min: incoming.background_ms_per_min.or(base.background_ms_per_min),
        log_bytes_per_min: incoming.log_bytes_per_min.or(base.log_bytes_per_min),
    }
}

/// 🏷️ Builder has plugin id only — next call must be `.label(...)`.
pub struct NeedsLabel;
/// 🏷️ Builder has id + label — next call must be `.version(...)`.
pub struct NeedsVersion;
/// ✅️ Builder has id + label + version — ready for apps, capabilities, and typed assembly.
pub struct Ready;

/// 🏗️ Fluent plugin constructor with typestate gates for identity fields.
/// 🗃️ sdk-dedyn (O1/§1.5): generic over the per-plugin app enum `PA: PluginApp` — replaces
/// `Box<dyn Fn() -> Box<dyn PluginApp> + Send>`'s capturing closures throughout with the same
/// `AppFactory<PA>` (definition + bare fn pointer) shape `crate::app::declarations::SurfaceDeclaration`
/// already uses (see that struct's doc for why a bare, non-capturing `fn` cannot close over `def`).
pub struct PluginBuilder<State, PA: PluginApp = crate::app::NoPluginApp> {
    plugin_id: String,
    package_id: Option<String>,
    label: Option<String>,
    version: Option<String>,
    artifacts: Vec<ArtifactDeclaration>,
    artifact_definitions: Vec<crate::app::ArtifactDefinition>,
    capabilities: Vec<CapabilityRequirement>,
    commands: Vec<(CommandDefinition, PluginCommandHandler)>,
    /// 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jobs-runtime, design-abi.md §4/§6) —
    /// `.job(kind, run)` declarations, folded into `⚛️reactor/💼️jobs::register_job_kind` at the end
    /// of `try_build()` ("registered on bundle install like other builder registrations" per this
    /// packet's brief) rather than stored on `Plugin` itself — the job registry is a thread-local
    /// keyed by `kind`, not a `Plugin`-scoped table, since `step_job`/`start_job` never carry a
    /// plugin id to look one up by.
    jobs: Vec<(&'static str, crate::reactor::jobs::JobFn)>,
    /// 🧭️ Metadata-only routes whose executable is an ActionBus-owned cold job rather than a
    /// synchronous `ArtifactInferenceService` facade.
    routed_inferences: Vec<ArtifactInferenceServiceMetadata>,
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
    topic_contributions: Vec<semio_framework::TopicContribution>,
    /// 📖️ One non-capturing `(document_schema, kinds)` provider per `.document_app_mutation_roster::
    /// <A>()`/`.viewer_mutation_roster::<V>()`/`.editor_mutation_roster::<E>()` call — committed into
    /// the process-wide owner mutation roster by `try_build`.
    owner_mutation_rosters: Vec<crate::app::OwnerMutationRoster>,
    app_defs: Vec<(App, crate::app::declarations::AppFactory<PA>)>,
    app_schema_descriptors: Vec<fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>>,
    document_app_ids: Vec<&'static str>,
    /// 🌳️ Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-C — the new declaration tree,
    /// walked by `.declare_artifact(...)`/`try_build()` alongside (never instead of) `artifacts`
    /// above, which stays bound to the OLD `ArtifactDeclaration` type (debt D1).
    declared_artifacts: Vec<crate::app::declarations::ArtifactDeclaration<PA>>,
    /// 🚀️ Ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME E2 (`📓️design-abi.md` §3) —
    /// `.activation(..)`/`.extension_point(..)`/`.requests(..)`/`.quota(..)`/`.execution(..)`/
    /// `.asset(..)`'s own storage, installed into `plugin_runtime::PluginDescriptorExtras` at the
    /// end of `try_build()` for `describe::describe_plugin()` to read — see that install call's own
    /// doc for why this is a side channel rather than new `Plugin`/`PluginManifest` fields.
    activation_events: Vec<ActivationEvent>,
    capability_requests: Vec<CapabilityRequest>,
    extension_points: Vec<ExtensionPointDeclaration>,
    execution: ExecutionMode,
    quotas: QuotaSchema,
    assets: Vec<AssetDeclaration>,
    _state: PhantomData<State>,
}

impl<PA: PluginApp> PluginBuilder<NeedsLabel, PA> {
    /// 🪪 Starts a plugin builder from a stable plugin id.
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            package_id: None,
            label: None,
            version: None,
            artifacts: Vec::new(),
            artifact_definitions: Vec::new(),
            capabilities: Vec::new(),
            commands: Vec::new(),
            jobs: Vec::new(),
            routed_inferences: Vec::new(),
            artifact_kinds: Vec::new(),
            host_media_handlers: Vec::new(),
            flow_extensions: Vec::new(),
            foreign_document_codecs: Vec::new(),
            dependencies: Vec::new(),
            contributions: Vec::new(),
            topic_contributions: Vec::new(),
            owner_mutation_rosters: Vec::new(),
            app_defs: Vec::new(),
            app_schema_descriptors: Vec::new(),
            document_app_ids: Vec::new(),
            declared_artifacts: Vec::new(),
            activation_events: Vec::new(),
            capability_requests: Vec::new(),
            extension_points: Vec::new(),
            execution: ExecutionMode::default(),
            quotas: QuotaSchema::default(),
            assets: Vec::new(),
            _state: PhantomData,
        }
    }

    /// 🏷️ Sets the human-readable plugin label.
    pub fn label(self, label: impl Into<String>) -> PluginBuilder<NeedsVersion, PA> {
        PluginBuilder {
            plugin_id: self.plugin_id,
            package_id: self.package_id,
            label: Some(label.into()),
            version: None,
            artifacts: self.artifacts,
            artifact_definitions: self.artifact_definitions,
            capabilities: self.capabilities,
            commands: self.commands,
            jobs: self.jobs,
            routed_inferences: self.routed_inferences,
            artifact_kinds: self.artifact_kinds,
            host_media_handlers: self.host_media_handlers,
            flow_extensions: self.flow_extensions,
            foreign_document_codecs: self.foreign_document_codecs,
            dependencies: self.dependencies,
            contributions: self.contributions,
            topic_contributions: self.topic_contributions,
            owner_mutation_rosters: self.owner_mutation_rosters,
            app_defs: self.app_defs,
            app_schema_descriptors: self.app_schema_descriptors,
            document_app_ids: self.document_app_ids,
            declared_artifacts: self.declared_artifacts,
            activation_events: self.activation_events,
            capability_requests: self.capability_requests,
            extension_points: self.extension_points,
            execution: self.execution,
            quotas: self.quotas,
            assets: self.assets,
            _state: PhantomData,
        }
    }
}

impl<PA: PluginApp> PluginBuilder<NeedsVersion, PA> {
    /// 🏷️ Sets the plugin version string.
    pub fn version(self, version: impl Into<String>) -> PluginBuilder<Ready, PA> {
        PluginBuilder {
            plugin_id: self.plugin_id,
            package_id: self.package_id,
            label: self.label,
            version: Some(version.into()),
            artifacts: self.artifacts,
            artifact_definitions: self.artifact_definitions,
            capabilities: self.capabilities,
            commands: self.commands,
            jobs: self.jobs,
            routed_inferences: self.routed_inferences,
            artifact_kinds: self.artifact_kinds,
            host_media_handlers: self.host_media_handlers,
            flow_extensions: self.flow_extensions,
            foreign_document_codecs: self.foreign_document_codecs,
            dependencies: self.dependencies,
            contributions: self.contributions,
            topic_contributions: self.topic_contributions,
            owner_mutation_rosters: self.owner_mutation_rosters,
            app_defs: self.app_defs,
            app_schema_descriptors: self.app_schema_descriptors,
            document_app_ids: self.document_app_ids,
            declared_artifacts: self.declared_artifacts,
            activation_events: self.activation_events,
            capability_requests: self.capability_requests,
            extension_points: self.extension_points,
            execution: self.execution,
            quotas: self.quotas,
            assets: self.assets,
            _state: PhantomData,
        }
    }
}

impl<PA: PluginApp> PluginBuilder<Ready, PA> {
    /// 📦️ Declares the canonical component package identity emitted by `describe()`.
    pub fn package_id(mut self, package_id: impl Into<String>) -> Self {
        self.package_id = Some(package_id.into());
        self
    }

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

    /// 🌳️ Declares one artifact through the NEW declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-
    /// STANDARD-SUBSET-MECHANISM, design.md §2). Sibling of `.artifact(...)` above (bound to the OLD
    /// `ArtifactDeclaration`, debt D1) — a distinct method name because Rust has no overloading and
    /// both types share the bare name `ArtifactDeclaration` at their own module scope. `try_build()`
    /// walks every declared tree and registers it atomically (preflight across every channel, then
    /// commit), additive alongside every old registration path — see `crate::app::declarations`.
    pub fn declare_artifact(mut self, declaration: crate::app::declarations::ArtifactDeclaration<PA>) -> Self {
        self.declared_artifacts.push(declaration);
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

    /// 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jobs-runtime, design-abi.md §4/§6) —
    /// declares one cold job kind this plugin authors, resolved by `⚛️reactor/💼️jobs::start_job`
    /// through the SAME `kind` string a `spawn-job` effect names. `try_build()` folds every
    /// declared entry into `⚛️reactor/💼️jobs::register_job_kind` at bundle-install time — see that
    /// field's own doc comment for why the registry lives there rather than on `Plugin`.
    pub fn job(mut self, kind: &'static str, run: crate::reactor::jobs::JobFn) -> Self {
        self.jobs.push((kind, run));
        self
    }

    /// 🧭️ Advertises one inference implemented by the exact `semio.infer/<schema>` cold-job
    /// route without manufacturing a second synchronous executable.
    pub fn routed_inference(mut self, metadata: ArtifactInferenceServiceMetadata) -> Self {
        self.routed_inferences.push(metadata);
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
    pub fn foreign_document_codec<A: ArtifactApp>(mut self, schema: impl Into<String>) -> Self
    where
        A::Mutation: Sync,
    {
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

    /// 📇️ Retains domain-owned metadata in the final manifest before assembly completes.
    pub fn contributes_topic(mut self, contribution: semio_framework::TopicContribution) -> Self {
        self.topic_contributions.push(contribution);
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
    pub fn document_app<A: ArtifactApp>(mut self, app: App) -> Self
    where
        PA: From<crate::app::VcsArtifactApp<A>>,
    {
        // 🚫️async: E4 fn-pointer slot
        fn app_schema<A: ArtifactApp>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            resolve_ready(A::app_schema())
        }
        // 🚫️async: E4 fn-pointer slot — bare, non-capturing (see `PluginBuilder`'s own doc); rebuilds
        // the registry from `def` inside the fn body instead of capturing it, same trick
        // `crate::app::declarations::editor_surface`'s inner `factory` uses.
        fn factory<A: ArtifactApp, PA: PluginApp + From<crate::app::VcsArtifactApp<A>>>(def: &crate::app::AppDefinition) -> PA {
            PA::from(resolve_ready(crate::app::VcsArtifactApp::with_registry(A::default(), crate::app::AppActionRegistry::from_definition(def))))
        }
        let definition = app.definition.clone();
        self.app_defs.push((app, (definition, factory::<A, PA>)));
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
        // 🚫️async: E4 fn-pointer slot — `owner_mutation_rosters: Vec<fn() -> ...>` is a bare fn
        // pointer table; `kinds()` is a pure static-table accessor.
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
    pub fn viewer<V: crate::app::ArtifactViewer>(self, def: crate::app::AppDefinition) -> Self
    where
        PA: From<crate::app::VcsArtifactApp<crate::app::ViewerApp<V>>>,
    {
        self.viewer_with_members::<V, store::NoMembers>(def)
    }

    /// 🧸️ A read-only surface retains its exact typed child fleet without acquiring document write authority.
    pub fn viewer_with_members<V, M>(mut self, mut def: crate::app::AppDefinition) -> Self
    where
        V: crate::app::ArtifactViewer,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        PA: From<crate::app::VcsArtifactApp<crate::app::ViewerApp<V>, M>>,
    {
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        // 🚫️async: E4 fn-pointer slot
        fn app_schema<V: crate::app::ArtifactViewer>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            V::app_schema()
        }
        // 🚫️async: E4 fn-pointer slot — see `document_app`'s `factory` doc.
        fn factory<V, M, PA>(def: &crate::app::AppDefinition) -> PA
        where
            V: crate::app::ArtifactViewer,
            M: store::SpaceMember + store::MemberFactory + Send + 'static,
            PA: PluginApp + From<crate::app::VcsArtifactApp<crate::app::ViewerApp<V>, M>>,
        {
            PA::from(resolve_ready(crate::app::VcsArtifactApp::<crate::app::ViewerApp<V>, M>::with_registry(crate::app::ViewerApp::<V>::default(), crate::app::AppActionRegistry::from_definition(def))))
        }
        // 🎯️ C8.2 — schema-first: `io.document_schema` names the schema this surface opens without
        // relying on the `artifact_kinds[0].schema` convention. Stamped only when the app left it
        // empty, so an app that already set a different `io.document_schema` keeps its own choice.
        if def.io.document_schema.is_empty() {
            def.io.document_schema = V::DOCUMENT_SCHEMA.to_string();
        }
        let app = App { definition: def.clone(), examples: Vec::new() };
        self.app_defs.push((app, (def, factory::<V, M, PA>)));
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
        // 🚫️async: E4 fn-pointer slot — see `document_app`'s `owner_mutation_roster` doc.
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
    /// `SemanticMutation` bound — see `viewer` above and `editor_mutation_roster` below. Delegates to
    /// `editor_app` with an empty example set — see `editor_with_examples` for the variant that
    /// registers `📚️examples/🎬️<slug>/🦀️.rs` fixtures onto the manifest.
    pub fn editor<E: crate::app::ArtifactEditor>(self, def: crate::app::AppDefinition) -> Self
    where
        PA: From<crate::app::VcsArtifactApp<crate::app::EditorApp<E>>>,
    {
        self.editor_app::<E>(def, Vec::new())
    }

    /// 📚️ `editor` twin that also declares this subset's `ExampleSource`s on the manifest — each
    /// becomes an `ExampleDefinition` stamped with `E::DIALECT` by `register_app_factory` at
    /// `try_build`, the same path `App::example_source` uses. The examples belong to the DIALECT, not
    /// to this editor: every surface of the same dialect (its viewer included) resolves them through
    /// `manifest::examples_for_app`, which is what feeds the react shell's example dropdown
    /// (`activePluginManifest.examples`, `NavbarExampleSelect/🟦️.tsx`) — hidden only while no example
    /// of the open app's dialect exists. Declared here rather than on a standalone builder step
    /// because the editor row is the one registration the subset's fixtures travel with
    /// (`SubsetDeclaration.examples` does the same, ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn editor_with_examples<E: crate::app::ArtifactEditor>(self, def: crate::app::AppDefinition, examples: Vec<crate::app::ExampleSource>) -> Self
    where
        PA: From<crate::app::VcsArtifactApp<crate::app::EditorApp<E>>>,
    {
        self.editor_app::<E>(def, examples)
    }

    fn editor_app<E: crate::app::ArtifactEditor>(mut self, mut def: crate::app::AppDefinition, examples: Vec<crate::app::ExampleSource>) -> Self
    where
        PA: From<crate::app::VcsArtifactApp<crate::app::EditorApp<E>>>,
    {
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        // 🚫️async: E4 fn-pointer slot
        fn app_schema<E: crate::app::ArtifactEditor>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            E::app_schema()
        }
        // 🚫️async: E4 fn-pointer slot — see `document_app`'s `factory` doc.
        fn factory<E: crate::app::ArtifactEditor, PA: PluginApp + From<crate::app::VcsArtifactApp<crate::app::EditorApp<E>>>>(def: &crate::app::AppDefinition) -> PA {
            PA::from(resolve_ready(crate::app::VcsArtifactApp::with_registry(crate::app::EditorApp::<E>::default(), crate::app::AppActionRegistry::from_definition(def))))
        }
        // 🎯️ C8.2 — schema-first: `io.document_schema` names the schema this surface opens without
        // relying on the `artifact_kinds[0].schema` convention. Stamped only when the app left it
        // empty, so an app that already set a different `io.document_schema` keeps its own choice.
        if def.io.document_schema.is_empty() {
            def.io.document_schema = E::DOCUMENT_SCHEMA.to_string();
        }
        let app = App { definition: def.clone(), examples };
        self.app_defs.push((app, (def, factory::<E, PA>)));
        self.app_schema_descriptors.push(app_schema::<E>);
        // 🔒️ Contract §2.3 clause 4 — an editor's document store attaches both Read and Write.
        self.capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Read, scope: Scope::App }).capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Write, scope: Scope::App })
    }

    /// 🧩️ Declares an editor whose document owns typed composed children. The member fleet is
    /// part of the concrete plugin-app variant, so production factories retain child resolution
    /// instead of erasing it to `NoMembers` at the app-bus boundary.
    pub fn editor_with_members<E, M>(mut self, mut def: crate::app::AppDefinition) -> Self
    where
        E: crate::app::ArtifactEditor,
        M: store::SpaceMember + store::MemberFactory + Send + 'static,
        PA: From<crate::app::VcsArtifactApp<crate::app::EditorApp<E>, M>>,
    {
        use semio_framework::kernel::{ArtifactKind, Rights, Scope};
        fn app_schema<E: crate::app::ArtifactEditor>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            E::app_schema()
        }
        fn factory<E, M, PA>(def: &crate::app::AppDefinition) -> PA
        where
            E: crate::app::ArtifactEditor,
            M: store::SpaceMember + store::MemberFactory + Send + 'static,
            PA: PluginApp + From<crate::app::VcsArtifactApp<crate::app::EditorApp<E>, M>>,
        {
            PA::from(resolve_ready(crate::app::VcsArtifactApp::<crate::app::EditorApp<E>, M>::with_registry(crate::app::EditorApp::<E>::default(), crate::app::AppActionRegistry::from_definition(def))))
        }
        if def.io.document_schema.is_empty() {
            def.io.document_schema = E::DOCUMENT_SCHEMA.to_string();
        }
        let app = App { definition: def.clone(), examples: Vec::new() };
        self.app_defs.push((app, (def, factory::<E, M, PA>)));
        self.app_schema_descriptors.push(app_schema::<E>);
        self.capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Read, scope: Scope::App }).capability(CapabilityRequirement { artifact: ArtifactKind::Document, rights: Rights::Write, scope: Scope::App })
    }

    /// 🗂️ Opt-in: registers `E`'s owner-mutation roster — see `viewer_mutation_roster`.
    pub fn editor_mutation_roster<E: crate::app::ArtifactEditor>(mut self) -> Self
    where
        E::Mutation: protocol::SemanticMutation<E::Snapshot>,
    {
        // 🚫️async: E4 fn-pointer slot — see `document_app`'s `owner_mutation_roster` doc.
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

    //#region 🔖️Descriptor
    /// 🚀️ Declares one activation event — when the host should spin up an instance of this plugin
    /// (`📓️design-abi.md` §2/§3). Repeatable; idempotent for a byte-identical event.
    pub fn activation(mut self, event: ActivationEvent) -> Self {
        if !self.activation_events.contains(&event) {
            self.activation_events.push(event);
        }
        self
    }

    /// 🧩️ Publishes one extension point other packages may attach to (`📓️design-abi.md` §5) —
    /// replaces the old Cargo `consumes` tag. Repeatable; idempotent for a byte-identical row.
    pub fn extension_point(mut self, declaration: ExtensionPointDeclaration) -> Self {
        if !self.extension_points.contains(&declaration) {
            self.extension_points.push(declaration);
        }
        self
    }

    /// 🙏️ Declares one capability ask the broker resolves at install/link/runtime
    /// (`📓️design-abi.md` §5) — the NEW broker-scoped `kernel::CapabilityRequest`, not the older
    /// kernel-level `CapabilityRequirement` `.capability(..)` declares. Repeatable; idempotent for a
    /// byte-identical ask.
    pub fn requests(mut self, request: CapabilityRequest) -> Self {
        if !self.capability_requests.contains(&request) {
            self.capability_requests.push(request);
        }
        self
    }

    /// 📏️ Merges one resource ceiling into this plugin's own (`📓️design-abi.md` §5) — only
    /// `schema`'s `Some` fields override; repeated calls layer without clobbering fields a previous
    /// call already set (see `merge_quota_schema`).
    pub fn quota(mut self, schema: QuotaSchema) -> Self {
        self.quotas = merge_quota_schema(self.quotas, schema);
        self
    }

    /// 🚦️ Sets how this plugin's actor runs — default `Isolated` (`📓️design-abi.md` §5).
    pub fn execution(mut self, mode: ExecutionMode) -> Self {
        self.execution = mode;
        self
    }

    /// 📦️ Declares one asset bundled with this plugin, preloaded into `kernel::Event::InstanceOpen.
    /// assets` (`📓️design-abi.md` §2). Repeatable; idempotent for a byte-identical declaration.
    pub fn asset(mut self, declaration: AssetDeclaration) -> Self {
        if !self.assets.contains(&declaration) {
            self.assets.push(declaration);
        }
        self
    }
    //#endregion 🔖️Descriptor

    /// 📚️ Assembles a library-only plugin through the typed boundary.
    pub fn try_library(self) -> Result<Plugin<PA>, PluginAssemblyError> {
        self.try_build()
    }

    /// ✅️ Builds plugin-local runtime authority before one all-registry commit.
    pub fn try_build(self) -> Result<Plugin<PA>, PluginAssemblyError> {
        let Self {
            plugin_id,
            package_id,
            label,
            version,
            artifacts,
            artifact_definitions,
            mut capabilities,
            commands,
            jobs,
            routed_inferences,
            artifact_kinds,
            host_media_handlers,
            flow_extensions,
            foreign_document_codecs,
            dependencies,
            contributions,
            topic_contributions,
            owner_mutation_rosters,
            mut app_defs,
            mut app_schema_descriptors,
            document_app_ids,
            declared_artifacts,
            activation_events,
            capability_requests,
            extension_points,
            execution,
            quotas,
            assets,
            _state: _,
        } = self;
        let label = label.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.label", "typestate-ready builder has no label"))?;
        let version = version.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.version", "typestate-ready builder has no version"))?;
        let package_id = package_id.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.package-id", "component package identity was not declared"))?;
        let package_suffix = package_id.strip_prefix("semio:").ok_or_else(|| PluginAssemblyError::new("plugin-assembly.package-id", "component package identity must use the semio namespace"))?;
        if package_suffix != plugin_id
            || package_suffix.is_empty()
            || package_suffix.starts_with('-')
            || package_suffix.ends_with('-')
            || package_suffix.contains("--")
            || !package_suffix.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(PluginAssemblyError::new("plugin-assembly.package-id", "component package identity must exactly match semio:<plugin-id> in canonical lowercase form"));
        }
        crate::app::declarations::preflight_artifact_declarations(&plugin_id, &declared_artifacts)?;
        let declared_registration = crate::app::declarations::project_artifact_declarations(&declared_artifacts);
        app_defs.extend(declared_registration.app_defs);
        app_schema_descriptors.extend(declared_registration.app_schema_descriptors);
        capabilities.extend(declared_registration.capabilities);
        let mut definitions = ArtifactDefinitionRegistry::new();
        for definition in artifact_definitions {
            crate::app::preflight_artifact_identity(&plugin_id, definition.identity().as_str())?;
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
        let plan = crate::app::ArtifactRegistrationPlan::from_declarations(&artifacts, app_schemas, &foreign_document_codecs, &plugin_id, host_media_handlers, flow_extensions, routed_inferences);
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

        crate::app::declarations::commit_artifact_declarations(&plugin_id, &declared_artifacts)?;

        let mut plugin = Plugin::new(plugin_id.clone(), label, version).with_runtime_registry(runtime);
        plugin.manifest.dependencies = dependencies;
        plugin.manifest.contributions = contribution_descriptors;
        plugin.manifest.topic_contributions = topic_contributions;
        for declaration in artifacts {
            plugin = declaration.apply_to(plugin);
        }
        for capability in capabilities {
            plugin = plugin.capability(capability);
        }
        for (command, handler) in commands {
            plugin = plugin.plugin_command(command, handler);
        }
        // 💼️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jobs-runtime) — "registered on bundle
        // install like other builder registrations" per this packet's brief: folded here, in the
        // SAME `try_build()` call that installs every other builder registration, rather than
        // deferred to a separate hook.
        for (kind, run) in jobs {
            crate::reactor::jobs::register_job_kind(kind, run);
        }
        for kind in artifact_kinds {
            plugin = plugin.artifact_kind(kind);
        }
        for (app, factory) in app_defs {
            plugin = plugin.register_app_factory(app, factory);
        }
        if let Some(breach) = crate::app::surface_dependency_breaches(&plugin.manifest).into_iter().next() {
            return Err(PluginAssemblyError::new("plugin-assembly.surface-dependency-gate", breach));
        }
        let assembly = store::begin_artifact_assembly().map_err(|error| PluginAssemblyError::new("plugin-assembly.unavailable", error.to_string()))?;
        crate::app::commit_artifact_registration_plan(&assembly, registry_plan)?;
        Ok(plugin.with_descriptor_extras(crate::plugin_runtime::PluginDescriptorExtras { package_id, activation_events, capability_requests, extension_points, execution, quotas, assets }))
    }
}

impl<PA: PluginApp> Plugin<PA> {
    /// 🏗️ Starts a typestate plugin builder from a stable plugin id.
    pub fn builder(plugin_id: impl Into<String>) -> PluginBuilder<NeedsLabel, PA> {
        PluginBuilder::new(plugin_id)
    }
}

//#region 🧪️DependencyContributionFixtureMount
#[cfg(test)]
#[path = "🧪️tests/🪪️artifact-admission/🦀️.rs"]
mod artifact_admission_tests;

#[cfg(test)]
#[path = "🧪️tests/🔗️dependency-contribution/🦀️.rs"]
mod dependency_fixture;
//#endregion 🧪️DependencyContributionFixtureMount

#[cfg(test)]
#[path = "🧪️tests/🔬️plugin-builder-dependency/🦀️.rs"]
mod plugin_builder_dependency_tests;

/// 🧪️ Ticket 26/08/17/SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION contract §C8.2 —
/// `PluginBuilder::editor::<E>`/`viewer::<V>` stamp `def.io.document_schema` from `E::DOCUMENT_SCHEMA`/
/// `V::DOCUMENT_SCHEMA` when the app left it empty, so "which document schema does this surface open"
/// is schema-first instead of the `artifact_kinds[0].schema` convention.
#[cfg(test)]
#[path = "🧪️tests/🔬️schema-stamping/🦀️.rs"]
mod schema_stamping_tests;
