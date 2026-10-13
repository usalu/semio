//! 🔌️ The assembled plugin program and its app instances.
use semio_framework::AppDefinition;
use semio_framework::manifest::{CommandInvocation as ManifestCommandInvocation, CommandOwnerAddress as ManifestCommandOwnerAddress};
use semio_framework::{kernel::{ActorId, ArtifactKind, CapabilityRequirement, InvocationResult, Rights, Scope}, ActionKind, CommandDefinition, Fault, PluginManifest};
use std::collections::{HashMap, HashSet};
/// 🗂️ `OsMediaCapability`/`ArtifactKindSpec` now live in `semio-framework-core` (both this crate and
/// `semio-framework-os` already depend on it) the same way the legacy format enum used to — re-exported
/// here verbatim instead of duplicated, so `AppBuilder::artifact_kind(...)` and
/// `semio_framework_os`'s artifact catalog registry share one definition.
use semio_framework::ArtifactKindSpec;
use crate::plugin_app::NoPluginApp;
use crate::app_builder::App;
use crate::plugin_app::PluginApp;
use crate::app_emission::ActionMeta;
use crate::artifact_declaration::FlowExtensionDescriptor;
use crate::artifact_declaration::HostMediaHandlerDescriptor;
use crate::artifact_declaration::HostMediaRuntimeError;
use crate::artifact_declaration::MeshImportRequest;
use crate::artifact_declaration::MeshImportResult;
use crate::artifact_declaration::TwoDSvgExportRequest;
use crate::artifact_declaration::TwoDSvgExportResult;
use crate::artifact_identity::ArtifactDefinitionRegistry;
use crate::artifact_inference::ArtifactInferenceExecutionError;
use crate::inference_gateway::wire_artifact_infer_from_cold;
use crate::artifact_inference::wire_list_artifact_inference_services_with_routes;
use crate::artifact_registration::PluginRuntimeRegistry;
use crate::artifact_registration::WireArtifactMutationPlanRequest;
use crate::artifact_registration::WireArtifactMutationPlanResult;
use crate::reserved_jobs::join_framework_shared_action_dispositions;
use crate::sdk_fault::plugin_sdk_fault;
use crate::surface_contract::ArtifactCodecTableV1;



/// 🗃️ sdk-dedyn (O1/§1.5): generic over the per-plugin app enum `PA: PluginApp` — replaces
/// `Box<dyn PluginApp>`.
pub struct AppInstance<PA: PluginApp = NoPluginApp> {
    pub id: u32,
    pub app: PA,
    pub(crate) surface_contexts: crate::reactor::surface_context::SurfaceContexts,
}

/// 🔌️ A plugin-owned command executes at the program boundary, never through an arbitrary
/// focused app. The manifest invocation carries its structural owner address; the handler may return
/// the same host effects/events as app command dispatch without acquiring app document state.
pub type PluginCommandHandler = Box<dyn Fn(&ManifestCommandInvocation, &ActionMeta) -> Result<InvocationResult, Fault> + Send>;

/// 🗃️ sdk-dedyn (O1/§1.5): `Self::App: PluginApp` replaces `Box<dyn PluginApp>` — the single
/// real impl (`Plugin<PA>` below) is never itself `dyn`-used (verified repo-wide), so this trait
/// stays a plain AFIT contract, not a `#[dyn_enum]` family.
pub trait PluginProgram: Send {
    type App: PluginApp;
    fn manifest(&self) -> PluginManifest;
    fn create_app(&self, app_id: &str, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Option<Self::App>;
    fn handle_plugin_command(&self, invocation: &ManifestCommandInvocation, meta: &ActionMeta) -> Result<InvocationResult, Fault>;
}

pub struct Plugin<PA: PluginApp = NoPluginApp> {
    pub manifest: PluginManifest,
    descriptor_extras: crate::plugin_runtime::PluginDescriptorExtras,
    /// 📦️ Descriptor declarations for the example bodies this plugin registered as DEFERRED —
    /// collected at `register_app_factory`, where the leaf and its dialect are both in hand, and
    /// merged into the descriptor by `plugin_runtime::plugin_descriptor_extras`.
    deferred_example_assets: Vec<semio_framework::AssetDeclaration>,
    apps: HashMap<String, AppFactory<PA>>,
    command_handlers: HashMap<String, PluginCommandHandler>,
    runtime: PluginRuntimeRegistry,
}

impl<PA: PluginApp> Plugin<PA> {
    pub fn new(plugin_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            manifest: PluginManifest {
                plugin_id: plugin_id.into(),
                label: label.into(),
                version: version.into(),
                apps: Vec::new(),
                examples: Vec::new(),
                capabilities: Vec::new(),
                topic_contributions: Vec::new(),
                commands: Vec::new(),
                artifact_kinds: Vec::new(),
                hosted_artifact_kinds: Vec::new(),
                dependencies: Vec::new(),
                contributions: Vec::new(),
            },
            apps: HashMap::new(),
            command_handlers: HashMap::new(),
            runtime: PluginRuntimeRegistry::empty(),
            descriptor_extras: crate::plugin_runtime::PluginDescriptorExtras::default(),
            deferred_example_assets: Vec::new(),
        }
    }

    /// 🛂️ Keeps descriptor authority on the exact immutable plugin assembly.
    pub(crate) fn with_descriptor_extras(mut self, extras: crate::plugin_runtime::PluginDescriptorExtras) -> Self {
        self.descriptor_extras = extras;
        self
    }

    /// 🪪️ Reads descriptor fields belonging to this plugin, never the last assembled plugin.
    pub(crate) fn descriptor_extras(&self) -> &crate::plugin_runtime::PluginDescriptorExtras {
        &self.descriptor_extras
    }

    /// 📦️ Descriptor declarations for this plugin's DEFERRED example bodies — see the field.
    pub(crate) fn deferred_example_assets(&self) -> &[semio_framework::AssetDeclaration] {
        &self.deferred_example_assets
    }

    /// 🧬️ Installs the fully preflighted declaration authority before external rows commit.
    pub(crate) fn with_runtime_registry(mut self, runtime: PluginRuntimeRegistry) -> Self {
        self.runtime = runtime;
        self
    }

    /// 🧾️ Reads this plugin's immutable artifact-definition authority.
    pub fn artifact_definitions(&self) -> &ArtifactDefinitionRegistry {
        self.runtime.definitions()
    }

    /// 🪪️ Reads every prefilter codec binding frozen by the plugin declaration plan.
    pub fn artifact_codec_bindings(&self) -> &[semio_framework_os_kernel::io::ArtifactCodecBinding] {
        self.runtime.document_bindings()
    }

    /// 🧭️ Lists frozen host-media declarations in stable contribution-id order.
    pub fn host_media_handlers(&self) -> Vec<HostMediaHandlerDescriptor> {
        self.runtime.host_media_handlers.descriptors()
    }

    /// 🌊️ Lists frozen flow-extension descriptors in stable contribution-id order.
    pub fn flow_extensions(&self) -> Vec<FlowExtensionDescriptor> {
        self.runtime.flow_extensions.descriptors()
    }

    /// 📥️ Executes only the declared typed mesh bridge selected by the request.
    pub fn import_mesh(&self, request: &MeshImportRequest) -> Result<MeshImportResult, HostMediaRuntimeError> {
        self.runtime.host_media_handlers.mesh_import(request)
    }

    /// 📤️ Executes only the declared typed 2D SVG renderer selected by the request.
    pub fn export_two_d_svg(&self, request: &TwoDSvgExportRequest) -> Result<TwoDSvgExportResult, HostMediaRuntimeError> {
        self.runtime.host_media_handlers.two_d_svg_export(request)
    }

    pub(crate) fn wire_list_artifact_inference_services(&self) -> Vec<u8> {
        wire_list_artifact_inference_services_with_routes(self.runtime.inference_services(), self.runtime.routed_inferences())
    }

    pub(crate) fn wire_artifact_infer_cold(&self, request: &[u8],operation:u64,generation:u64) -> Result<Vec<u8>, ArtifactInferenceExecutionError> {
        wire_artifact_infer_from_cold(self.runtime.inference_services(), request,operation,generation)
    }

    pub(crate) fn wire_list_artifact_mutations(&self) -> Vec<u8> {
        store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&self.runtime.mutation_roster_entries()))
    }

    pub(crate) fn wire_artifact_mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, Fault> {
        let request: WireArtifactMutationPlanRequest =
            store::pack_rt::decode_wire_value(request).map_err(|error| plugin_sdk_fault(error.to_string())).and_then(|value| semio_framework_value::FromValue::from_value(value).map_err(|error| plugin_sdk_fault(error.to_string())))?;
        if request.mutation_id.trim().is_empty() {
            return Err(plugin_sdk_fault("artifact-mutation-plan requires a non-empty mutation_id"));
        }
        let output = self.runtime.contributed_mutation_plan(&request.mutation_id, &request.artifact_kind, &request.snapshot_pack, &request.payload).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let result = WireArtifactMutationPlanResult {
            artifact_kind: request.artifact_kind,
            mutation_id: request.mutation_id,
            revision: request.revision,
            generation: request.generation,
            owner_ops: output.owner_ops,
            label: output.label,
            foreign: output.foreign,
        };
        Ok(store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&result)))
    }

    /// 🎛️ Declares a plugin-owned command and its program-level handler.
    pub fn plugin_command(mut self, command: CommandDefinition, handler: PluginCommandHandler) -> Self {
        assert!(!command.id.trim().is_empty(), "plugin {} command id must be non-empty", self.manifest.plugin_id);
        assert!(!self.command_handlers.contains_key(&command.id), "plugin {} duplicate command id {}", self.manifest.plugin_id, command.id);
        let mut arg_ids = HashSet::new();
        for arg in &command.args {
            assert!(!arg.id.trim().is_empty(), "plugin {} command {} has an empty arg id", self.manifest.plugin_id, command.id);
            assert!(arg_ids.insert(arg.id.clone()), "plugin {} command {} declares duplicate arg id {}", self.manifest.plugin_id, command.id, arg.id);
            if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
                assert!(!options.is_empty(), "plugin {} command {} arg {} is a Select with no options", self.manifest.plugin_id, command.id, arg.id);
            }
        }
        self.command_handlers.insert(command.id.clone(), handler);
        self.manifest.commands.push(command);
        self
    }

    /// 🔌️ Dispatches only commands structurally owned by this plugin.
    pub fn handle_plugin_command(&self, invocation: &ManifestCommandInvocation, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let ManifestCommandOwnerAddress::Plugin { plugin_id } = &invocation.address.owner else {
            return Err(Fault::from("plugin command handler received a non-plugin owner"));
        };
        if plugin_id != &self.manifest.plugin_id {
            return Err(Fault::from(format!("plugin command owner {plugin_id} does not match {}", self.manifest.plugin_id)));
        }
        let handler = self.command_handlers.get(&invocation.address.command_id).ok_or_else(|| Fault::from(format!("unknown plugin command: {}", invocation.address.command_id)))?;
        let definition = self.manifest.commands.iter().find(|command| command.id == invocation.address.command_id).ok_or_else(|| Fault::from(format!("plugin command definition missing: {}", invocation.address.command_id)))?;
        let result = handler(invocation, meta)?;
        if matches!(definition.kind, ActionKind::View | ActionKind::Shell) && !result.mutations.is_empty() {
            return Err(Fault::from(format!("{:?}-kind plugin command '{}' must not emit mutations", definition.kind, definition.id)));
        }
        Ok(result)
    }

    pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
        if !self.manifest.capabilities.contains(&capability) {
            self.manifest.capabilities.push(capability);
        }
        self
    }

    /// 🗂️ Declares one plugin-level artifact kind (library plugins use this; repeatable).
    pub fn artifact_kind(mut self, spec: ArtifactKindSpec) -> Self {
        self.manifest.artifact_kinds.push(spec);
        self
    }

    pub fn local_backbone_storage(self) -> Self {
        self.capability(CapabilityRequirement { artifact: ArtifactKind::Backbone, rights: Rights::Read, scope: Scope::Plugin }).capability(CapabilityRequirement { artifact: ArtifactKind::Backbone, rights: Rights::Write, scope: Scope::Plugin })
    }

    /// 🧬️ Registers an already-wrapped app factory (definition + bare fn pointer — see
    /// `crate::AppFactory`'s doc for why the definition travels with it)
    /// after typed assembly has completed.
    ///
    /// 📚️ An example is a document of the registering surface's DIALECT, shared by every app
    /// bound to it (editor and viewer alike) — never a property of this one app id.
    ///
    /// 📦️ A deferred body never becomes a manifest string, so its descriptor declaration
    /// has to be taken from the leaf HERE — `describe` only ever sees the (empty) row.
    pub(crate) fn register_app_factory(mut self, mut app: App, factory: AppFactory<PA>) -> Self {
        let mut factory = factory;
        join_framework_shared_action_dispositions(&mut app.definition);
        join_framework_shared_action_dispositions(&mut factory.definition);
        let dialect = app.definition.dialect.clone();
        let media_type = app.definition.io.artifact_media_type;
        self.manifest.apps.push(app.definition);
        for source in app.examples {
            if let Some(declaration) = source.deferred_body_asset(&dialect, media_type) {
                if !self.deferred_example_assets.iter().any(|existing| existing.name == declaration.name) {
                    self.deferred_example_assets.push(declaration);
                }
            }
            self.manifest.examples.push(source.into_example_definition(dialect.clone()));
        }
        self.apps.insert(self.manifest.apps.last().unwrap().id.clone(), factory);
        self
    }

    pub fn create_app(&self, app_id: &str, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Option<PA> {
        self.apps.get(app_id).map(|factory| (factory.create)(&factory.definition, actor, mounted_policy, identity))
    }

    /// 🪪️ The document schema the registered app opens — its type's own `DOCUMENT_SCHEMA`, recorded
    /// at registration, so nothing has to be constructed to learn it.
    pub fn app_document_schema(&self, app_id: &str) -> Option<&'static str> {
        self.apps.get(app_id).map(|factory| factory.document_schema)
    }

    /// 🧬️ The registered app's `codec` answers — functions of its type, recorded at registration, so a codec
    /// call constructs nothing.
    pub fn app_codec(&self, app_id: &str) -> Option<ArtifactCodecTableV1> {
        self.apps.get(app_id).map(|factory| factory.codec)
    }
}

impl<PA: PluginApp> PluginProgram for Plugin<PA> {
    type App = PA;

    fn manifest(&self) -> PluginManifest {
        self.manifest.clone()
    }

    fn create_app(&self, app_id: &str, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Option<PA> {
        Plugin::create_app(self, app_id, actor, mounted_policy, identity)
    }

    fn handle_plugin_command(&self, invocation: &ManifestCommandInvocation, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        Plugin::handle_plugin_command(self, invocation, meta)
    }
}

/// 🎭️ The definition travels WITH the bare fn pointer — same reason `SurfaceDeclaration.factory`
/// does (a monomorphized non-capturing `fn` item cannot close over `def`).
pub(crate) struct AppFactory<PA> {
    pub definition: AppDefinition,
    /// 🚫️async: E4 fn-pointer slot
    pub create: for<'a, 'b> fn(&AppDefinition, ActorId, crate::MountedOwnerPolicyV1, &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> PA,
    pub document_schema: &'static str,
    pub codec: ArtifactCodecTableV1,
}
