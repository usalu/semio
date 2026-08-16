//! 🏗️ Typestate `PluginBuilder` — missing label/version is a compile error.

use crate::app::{App, ArtifactApp, ArtifactDeclaration, ArtifactDefinitionRegistry, Plugin, PluginApp, PluginAssemblyError, PluginCommandHandler};
use semio_framework::{kernel::CapabilityRequirement, CommandDefinition};
use std::collections::HashMap;
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
    setup: Vec<fn()>,
    artifacts: Vec<ArtifactDeclaration>,
    capabilities: Vec<CapabilityRequirement>,
    commands: Vec<(CommandDefinition, PluginCommandHandler)>,
    artifact_kinds: Vec<semio_framework::ArtifactKindSpec>,
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
    app_defs: Vec<(App, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>)>,
    app_schema_registrations: Vec<fn()>,
    _state: PhantomData<State>,
}

impl PluginBuilder<NeedsLabel> {
    /// 🪪 Starts a plugin builder from a stable plugin id.
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            label: None,
            version: None,
            setup: Vec::new(),
            artifacts: Vec::new(),
            capabilities: Vec::new(),
            commands: Vec::new(),
            artifact_kinds: Vec::new(),
            apps: HashMap::new(),
            app_defs: Vec::new(),
            app_schema_registrations: Vec::new(),
            _state: PhantomData,
        }
    }

    /// 🏷️ Sets the human-readable plugin label.
    pub fn label(self, label: impl Into<String>) -> PluginBuilder<NeedsVersion> {
        PluginBuilder {
            plugin_id: self.plugin_id,
            label: Some(label.into()),
            version: None,
            setup: self.setup,
            artifacts: self.artifacts,
            capabilities: self.capabilities,
            commands: self.commands,
            artifact_kinds: self.artifact_kinds,
            apps: self.apps,
            app_defs: self.app_defs,
            app_schema_registrations: self.app_schema_registrations,
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
            setup: self.setup,
            artifacts: self.artifacts,
            capabilities: self.capabilities,
            commands: self.commands,
            artifact_kinds: self.artifact_kinds,
            apps: self.apps,
            app_defs: self.app_defs,
            app_schema_registrations: self.app_schema_registrations,
            _state: PhantomData,
        }
    }
}

impl PluginBuilder<Ready> {
    /// 🔧️ Registers a one-shot setup callback (codecs / languages / importers). Repeatable — accumulates,
    /// runs in call order, does NOT overwrite an earlier `.setup(...)` call (see the field doc). Retiring
    /// in favor of `.artifact()`/`app_schema()` — see `PluginBuilder::setup`'s own doc for why this is
    /// still here.
    pub fn setup(mut self, setup: fn()) -> Self {
        self.setup.push(setup);
        self
    }

    /// 🗿️ Declares one artifact this plugin owns — the declarative replacement for `.setup()`
    /// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1). Repeatable. `build()` walks every
    /// declared artifact in a fixed deterministic order and validates that it owns everything it
    /// declares — see `ArtifactDeclaration::register_all`.
    pub fn artifact(mut self, declaration: ArtifactDeclaration) -> Self {
        self.artifacts.push(declaration);
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

    /// 🧬️ Declares a typed document app factory and defers app-schema registration until commit.
    pub fn document_app<A: ArtifactApp>(mut self, app: App) -> Self {
        fn register_schema<A: ArtifactApp>() {
            if let Some(descriptor) = A::app_schema() {
                ::semio_framework_schema::register_app_schema_descriptor(descriptor);
            }
        }
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> = Box::new(move || Box::new(crate::app::VcsArtifactApp::with_registry(A::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self.app_schema_registrations.push(register_schema::<A>);
        self
    }

    /// 🧬️ Alias for [`Self::document_app`] — matches the retired `Plugin` method name used by `semio_plugin!`.
    pub fn register_document_app<A: ArtifactApp>(self, app: App) -> Self {
        self.document_app::<A>(app)
    }

    /// 📚️ Assembles a library-only plugin through the typed boundary.
    pub fn try_library(self) -> Result<Plugin, PluginAssemblyError> {
        self.try_build()
    }

    /// ✅️ Preflights all declarations before any setup or registry side effect, then commits once.
    pub fn try_build(self) -> Result<Plugin, PluginAssemblyError> {
        let Self {
            plugin_id,
            label,
            version,
            setup,
            artifacts,
            capabilities,
            commands,
            artifact_kinds,
            apps: _,
            app_defs,
            app_schema_registrations,
            _state: _,
        } = self;
        let label = label.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.label", "typestate-ready builder has no label"))?;
        let version = version.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.version", "typestate-ready builder has no version"))?;
        let mut definitions = ArtifactDefinitionRegistry::new();
        for declaration in &artifacts {
            declaration.preflight(&plugin_id, &mut definitions)?;
        }
        for setup in &setup {
            setup();
        }
        let mut plugin = Plugin::new(plugin_id.clone(), label, version);
        for declaration in artifacts {
            plugin = declaration.try_register_all(&plugin_id, plugin)?;
        }
        for register_schema in app_schema_registrations {
            register_schema();
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
