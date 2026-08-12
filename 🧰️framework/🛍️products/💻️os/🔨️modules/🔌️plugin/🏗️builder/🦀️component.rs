//! 🏗️ Typestate `PluginBuilder` — missing label/version is a compile error.

use crate::app::{App, ArtifactApp, ArtifactDeclaration, Plugin, PluginApp};
use semio_framework::{kernel::CapabilityRequirement, CommandDefinition};
use std::collections::HashMap;
use std::marker::PhantomData;

/// 🏷️ Builder has plugin id only — next call must be `.label(...)`.
pub struct NeedsLabel;
/// 🏷️ Builder has id + label — next call must be `.version(...)`.
pub struct NeedsVersion;
/// ✅️ Builder has id + label + version — ready for apps / capabilities / `.build()` / `.library()`.
pub struct Ready;

/// 🏗️ Fluent plugin constructor with typestate gates for identity fields.
pub struct PluginBuilder<State> {
    plugin_id: String,
    label: Option<String>,
    version: Option<String>,
    // 🚧️ ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1: `setup` is NOT deleted yet — 31 of
    // 33 plugins still call `.setup(...)`, and deleting the method now would break every one of them
    // at once (SCOPE DISCIPLINE in the W1 packet: "land the mechanism, keep the tree compiling").
    // `.artifact()`/`artifacts` is the new, declarative surface; `setup` retires plugin-by-plugin as
    // each converts, and the field/method/call below are deleted together once none remain.
    setup: Option<fn()>,
    artifacts: Vec<ArtifactDeclaration>,
    capabilities: Vec<CapabilityRequirement>,
    commands: Vec<CommandDefinition>,
    artifact_kinds: Vec<semio_framework::ArtifactKindSpec>,
    apps: HashMap<String, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>>,
    app_defs: Vec<(App, Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static>)>,
    _state: PhantomData<State>,
}

impl PluginBuilder<NeedsLabel> {
    /// 🪪 Starts a plugin builder from a stable plugin id.
    pub fn new(plugin_id: impl Into<String>) -> Self {
        Self {
            plugin_id: plugin_id.into(),
            label: None,
            version: None,
            setup: None,
            artifacts: Vec::new(),
            capabilities: Vec::new(),
            commands: Vec::new(),
            artifact_kinds: Vec::new(),
            apps: HashMap::new(),
            app_defs: Vec::new(),
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
            _state: PhantomData,
        }
    }
}

impl PluginBuilder<Ready> {
    /// 🔧️ Registers a one-shot setup callback (codecs / languages / importers). Retiring in favor
    /// of `.artifact()` — see the field doc on `PluginBuilder::setup` for why this is still here.
    pub fn setup(mut self, setup: fn()) -> Self {
        self.setup = Some(setup);
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
        self.capability(CapabilityRequirement {
            artifact: ArtifactKind::Backbone,
            rights: Rights::Read,
            scope: Scope::Plugin,
        })
        .capability(CapabilityRequirement {
            artifact: ArtifactKind::Backbone,
            rights: Rights::Write,
            scope: Scope::Plugin,
        })
    }

    /// 🎮️ Declares a plugin-scope command.
    pub fn plugin_command(mut self, command: CommandDefinition) -> Self {
        self.commands.push(command);
        self
    }

    /// 🗂️ Declares one plugin-level artifact kind for library (zero-app) plugins. Repeatable.
    pub fn artifact_kind(mut self, spec: semio_framework::ArtifactKindSpec) -> Self {
        self.artifact_kinds.push(spec);
        self
    }

    /// 🧬️ Registers a typed document app factory.
    pub fn document_app<A: ArtifactApp>(mut self, app: App) -> Self {
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> =
            Box::new(move || Box::new(crate::app::VcsArtifactApp::with_registry(A::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self
    }

    /// 🧬️ Alias for [`Self::document_app`] — matches the retired `Plugin` method name used by `semio_plugin!`.
    pub fn register_document_app<A: ArtifactApp>(self, app: App) -> Self {
        self.document_app::<A>(app)
    }

    /// 📚️ Finishes a library-only plugin (no apps) — used by headless crates like energy.
    pub fn library(self) -> Plugin {
        self.build()
    }

    /// ✅️ Runs `.setup()` (if any — retiring, see the field doc), then walks every `.artifact()`
    /// declaration (fixed order, ownership-checked — see `ArtifactDeclaration::register_all`), and
    /// materializes a [`Plugin`].
    pub fn build(self) -> Plugin {
        if let Some(setup) = self.setup {
            setup();
        }
        let plugin_id = self.plugin_id.clone();
        let mut plugin = Plugin::new(
            self.plugin_id,
            self.label.expect("typestate Ready implies label"),
            self.version.expect("typestate Ready implies version"),
        );
        for declaration in self.artifacts {
            plugin = declaration.register_all(&plugin_id, plugin);
        }
        for capability in self.capabilities {
            plugin = plugin.capability(capability);
        }
        for command in self.commands {
            plugin = plugin.plugin_command(command);
        }
        for kind in self.artifact_kinds {
            plugin = plugin.artifact_kind(kind);
        }
        for (app, factory) in self.app_defs {
            plugin = plugin.register_app_factory(app, factory);
        }
        plugin
    }
}

impl Plugin {
    /// 🏗️ Starts a typestate plugin builder from a stable plugin id.
    pub fn builder(plugin_id: impl Into<String>) -> PluginBuilder<NeedsLabel> {
        PluginBuilder::new(plugin_id)
    }
}
