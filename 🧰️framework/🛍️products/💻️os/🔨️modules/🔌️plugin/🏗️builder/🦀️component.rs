//! 🏗️ Typestate `PluginBuilder` — missing label/version is a compile error.

use crate::app::{App, ArtifactApp, ArtifactDeclaration, ArtifactDefinitionRegistry, Plugin, PluginApp, PluginAssemblyError, PluginCommandHandler, PluginRegistration};
use semio_framework::{kernel::CapabilityRequirement, CommandDefinition};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Mutex, OnceLock};

static PLUGIN_ASSEMBLY_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn plugin_assembly_mutex() -> &'static Mutex<()> {
    PLUGIN_ASSEMBLY_MUTEX.get_or_init(|| Mutex::new(()))
}

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
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
    app_defs: Vec<(App, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>)>,
    app_schema_descriptors: Vec<fn() -> Option<::semio_framework_schema::AppSchemaDescriptor>>,
    registrations: Vec<PluginRegistration>,
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
            apps: HashMap::new(),
            app_defs: Vec::new(),
            app_schema_descriptors: Vec::new(),
            registrations: Vec::new(),
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
            apps: self.apps,
            app_defs: self.app_defs,
            app_schema_descriptors: self.app_schema_descriptors,
            registrations: self.registrations,
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
            apps: self.apps,
            app_defs: self.app_defs,
            app_schema_descriptors: self.app_schema_descriptors,
            registrations: self.registrations,
            _state: PhantomData,
        }
    }
}

impl PluginBuilder<Ready> {
    /// 🗿️ Declares one artifact this plugin owns. Repeatable. `try_build()` walks every
    /// declared artifact in a fixed deterministic order and validates that it owns everything it
    /// declares — see `ArtifactDeclaration::register_all`.
    pub fn artifact(mut self, declaration: ArtifactDeclaration) -> Self {
        self.artifacts.push(declaration);
        self
    }

    /// 🧾️ Registers one definition-only artifact through the same typed preflight registry.
    pub fn artifact_definition(mut self, definition: crate::app::ArtifactDefinition) -> Self {
        self.artifact_definitions.push(definition);
        self
    }

    /// 🧩️ Declares one typed plugin-wide registration in the transactional assembly plan.
    pub fn registration(mut self, registration: PluginRegistration) -> Self {
        self.registrations.push(registration);
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

    /// 🧬️ Declares a typed document app factory and app-schema descriptor for transactional assembly.
    pub fn document_app<A: ArtifactApp>(mut self, app: App) -> Self {
        fn app_schema<A: ArtifactApp>() -> Option<::semio_framework_schema::AppSchemaDescriptor> {
            A::app_schema()
        }
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> = Box::new(move || Box::new(crate::app::VcsArtifactApp::with_registry(A::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self.app_schema_descriptors.push(app_schema::<A>);
        self
    }

    /// 📚️ Assembles a library-only plugin through the typed boundary.
    pub fn try_library(self) -> Result<Plugin, PluginAssemblyError> {
        self.try_build()
    }

    /// ✅️ Preflights all declarations before any registry side effect, then commits once.
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
            apps: _,
            app_defs,
            app_schema_descriptors,
            registrations,
            _state: _,
        } = self;
        let label = label.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.label", "typestate-ready builder has no label"))?;
        let version = version.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.version", "typestate-ready builder has no version"))?;
        let _assembly = plugin_assembly_mutex()
            .lock()
            .map_err(|_| PluginAssemblyError::new("plugin-assembly.unavailable", "plugin assembly mutex is poisoned"))?;
        let mut definitions = ArtifactDefinitionRegistry::new();
        for definition in artifact_definitions {
            definitions.register(definition).map_err(PluginAssemblyError::definition)?;
        }
        for declaration in &artifacts {
            declaration.preflight(&plugin_id, &mut definitions)?;
        }
        let mut app_schemas = Vec::new();
        for get_schema in app_schema_descriptors {
            if let Some(descriptor) = get_schema() {
                app_schemas.push(descriptor);
            }
        }
        let plan = crate::app::ArtifactRegistrationPlan::from_declarations(&artifacts, app_schemas, registrations);
        plan.preflight()?;
        plan.commit()?;

        let mut plugin = Plugin::new(plugin_id.clone(), label, version);
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
        Ok(plugin)
    }
}

impl Plugin {
    /// 🏗️ Starts a typestate plugin builder from a stable plugin id.
    pub fn builder(plugin_id: impl Into<String>) -> PluginBuilder<NeedsLabel> {
        PluginBuilder::new(plugin_id)
    }
}
