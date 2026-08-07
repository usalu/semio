//! 🏗️ Typestate `PluginBuilder` — missing label/version is a compile error.

use crate::app::{App, DocumentApp, Plugin, PluginApp};
use semio_framework_core::{kernel::CapabilityRequirement, CommandDefinition, Contribution};
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
    setup: Option<fn()>,
    capabilities: Vec<CapabilityRequirement>,
    contributions: Vec<Contribution>,
    commands: Vec<CommandDefinition>,
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
            capabilities: Vec::new(),
            contributions: Vec::new(),
            commands: Vec::new(),
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
            capabilities: self.capabilities,
            contributions: self.contributions,
            commands: self.commands,
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
            capabilities: self.capabilities,
            contributions: self.contributions,
            commands: self.commands,
            apps: self.apps,
            app_defs: self.app_defs,
            _state: PhantomData,
        }
    }
}

impl PluginBuilder<Ready> {
    /// 🔧️ Registers a one-shot setup callback (codecs / languages / importers).
    pub fn setup(mut self, setup: fn()) -> Self {
        self.setup = Some(setup);
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
        use semio_framework_core::kernel::{ArtifactKind, Rights, Scope};
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

    /// 🧩️ Adds a contribution declaration.
    pub fn contributes(mut self, contribution: Contribution) -> Self {
        self.contributions.push(contribution);
        self
    }

    /// 🎮️ Declares a plugin-scope command.
    pub fn plugin_command(mut self, command: CommandDefinition) -> Self {
        self.commands.push(command);
        self
    }

    /// 🧬️ Registers a typed document app factory.
    pub fn document_app<A: DocumentApp>(mut self, app: App) -> Self {
        let registry = crate::app::AppActionRegistry::from_definition(&app.definition);
        let factory: Box<dyn Fn() -> Box<dyn PluginApp> + Send + 'static> =
            Box::new(move || Box::new(crate::app::VcsDocumentApp::with_registry(A::default(), registry.clone())));
        self.app_defs.push((app, factory));
        self
    }

    /// 🧬️ Alias for [`Self::document_app`] — matches the retired `Plugin` method name used by `semio_plugin!`.
    pub fn register_document_app<A: DocumentApp>(self, app: App) -> Self {
        self.document_app::<A>(app)
    }

    /// 📚️ Finishes a library-only plugin (no apps) — used by headless crates like energy.
    pub fn library(self) -> Plugin {
        self.build()
    }

    /// ✅️ Runs setup (if any) and materializes a [`Plugin`].
    pub fn build(self) -> Plugin {
        if let Some(setup) = self.setup {
            setup();
        }
        let mut plugin = Plugin::new(
            self.plugin_id,
            self.label.expect("typestate Ready implies label"),
            self.version.expect("typestate Ready implies version"),
        );
        for capability in self.capabilities {
            plugin = plugin.capability(capability);
        }
        for contribution in self.contributions {
            plugin = plugin.contributes(contribution);
        }
        for command in self.commands {
            plugin = plugin.plugin_command(command);
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
