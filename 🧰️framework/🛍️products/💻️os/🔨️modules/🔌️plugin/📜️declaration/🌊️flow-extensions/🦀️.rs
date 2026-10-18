//! 🌊️ Flow extensions: the extension executable identity, contributions, manifest, descriptor and declaration and the flow extension registry. See [`super`].

use super::*;

/// ⚙️ Immutable executable identity for a flow-extension contribution.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlowExtensionExecutableIdentity {
    pub package_id: String,
    pub module_id: String,
    pub entrypoint: String,
    pub abi: String,
}

impl FlowExtensionExecutableIdentity {
    /// 🏗️ Names a native executable entry without accepting an installer callback.
    pub fn native(package_id: impl Into<String>, module_id: impl Into<String>, entrypoint: impl Into<String>) -> Result<Self, PluginAssemblyError> {
        let identity = Self { package_id: package_id.into(), module_id: module_id.into(), entrypoint: entrypoint.into(), abi: "semio.flow.extension.native.v1".into() };
        for value in [&identity.package_id, &identity.module_id, &identity.entrypoint, &identity.abi] {
            ArtifactIdentity::parse(value.clone()).map_err(PluginAssemblyError::definition)?;
        }
        Ok(identity)
    }
}

/// 🧾️ One closed contribution category inside a `flow.extension` manifest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FlowExtensionContributionKind {
    Schema,
    Operator,
    Widget,
    Command,
    Setting,
}

/// 🧾️ One externally addressable contribution declared by a flow extension.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlowExtensionContribution {
    pub kind: FlowExtensionContributionKind,
    pub id: String,
}

impl FlowExtensionContribution {
    /// 🏗️ Creates one non-empty typed flow-extension contribution identity.
    pub fn new(kind: FlowExtensionContributionKind, id: impl Into<String>) -> Result<Self, PluginAssemblyError> {
        let id = id.into();
        ArtifactIdentity::parse(id.clone()).map_err(PluginAssemblyError::definition)?;
        Ok(Self { kind, id })
    }
}

/// 📋️ Schema-owned, immutable `flow.extension` manifest descriptor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub activation_events: Vec<String>,
    pub contributions: Vec<FlowExtensionContribution>,
}

impl FlowExtensionManifest {
    /// 🏗️ Creates a typed flow-extension manifest with an explicit startup activation.
    pub fn new(id: impl Into<String>, name: impl Into<String>, version: impl Into<String>) -> Result<Self, PluginAssemblyError> {
        let manifest = Self { id: id.into(), name: name.into(), version: version.into(), activation_events: vec!["onStartup".into()], contributions: Vec::new() };
        manifest.validate()?;
        Ok(manifest)
    }

    /// 🧩️ Adds one typed contribution to this immutable descriptor.
    pub fn contribution(mut self, contribution: FlowExtensionContribution) -> Result<Self, PluginAssemblyError> {
        if self.contributions.iter().any(|candidate| candidate.kind == contribution.kind && candidate.id == contribution.id) {
            return Ok(self);
        }
        self.contributions.push(contribution);
        Ok(self)
    }

    fn validate(&self) -> Result<(), PluginAssemblyError> {
        ArtifactIdentity::parse(self.id.clone()).map_err(PluginAssemblyError::definition)?;
        if self.name.trim().is_empty() || self.version.trim().is_empty() || self.activation_events.iter().any(|event| event.trim().is_empty()) {
            return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-manifest", "flow extension manifests require non-empty name, version, and activation events"));
        }
        let mut contributions = BTreeSet::new();
        for contribution in &self.contributions {
            ArtifactIdentity::parse(contribution.id.clone()).map_err(PluginAssemblyError::definition)?;
            if !contributions.insert((contribution.kind, contribution.id.clone())) {
                return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-contribution", format!("duplicate flow extension contribution {:?}", contribution.id)));
            }
        }
        Ok(())
    }
}

/// 📋️ Immutable runtime descriptor for one linked flow extension.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowExtensionDescriptor {
    pub id: String,
    pub owner: String,
    pub domain: String,
    pub schema: String,
    pub extension_id: String,
    pub manifest: FlowExtensionManifest,
    pub executable_identity: FlowExtensionExecutableIdentity,
}

/// 🌊️ Immutable `flow.extension` contribution; it carries no mutating installer function.
#[derive(Clone, Debug)]
pub struct FlowExtensionDeclaration {
    pub(super) id: ArtifactIdentity,
    extension_id: ArtifactIdentity,
    pub(super) manifest: FlowExtensionManifest,
    pub(super) executable_identity: FlowExtensionExecutableIdentity,
}

impl FlowExtensionDeclaration {
    /// 🏗️ Declares a `flow.extension` manifest and its immutable native executable identity.
    pub fn new(id: impl Into<String>, manifest: FlowExtensionManifest, executable_identity: FlowExtensionExecutableIdentity) -> Result<Self, PluginAssemblyError> {
        let declaration = Self { id: ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?, extension_id: ArtifactIdentity::parse(manifest.id.clone()).map_err(PluginAssemblyError::definition)?, manifest, executable_identity };
        declaration.validate_shape()?;
        Ok(declaration)
    }

    /// 🪪️ Returns the stable contribution id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub(super) fn validate_shape(&self) -> Result<(), PluginAssemblyError> {
        self.manifest.validate()
    }

    pub fn preflight(&self, owner: &str) -> Result<(), PluginAssemblyError> {
        ArtifactIdentity::parse(owner.to_string()).map_err(PluginAssemblyError::definition)?;
        self.validate_shape()
    }

    pub(super) fn descriptor(&self, owner: &str) -> FlowExtensionDescriptor {
        FlowExtensionDescriptor {
            id: self.id.as_str().to_string(),
            owner: owner.to_string(),
            domain: "flow.extension".into(),
            schema: "flow.extension".into(),
            extension_id: self.extension_id.as_str().to_string(),
            manifest: self.manifest.clone(),
            executable_identity: self.executable_identity.clone(),
        }
    }
}

pub struct FlowExtensionRegistry {
    pub(super) by_id: BTreeMap<String, FlowExtensionDescriptor>,
    by_extension_id: BTreeMap<String, String>,
}

impl FlowExtensionRegistry {
    pub fn new() -> Self {
        Self { by_id: BTreeMap::new(), by_extension_id: BTreeMap::new() }
    }

    pub fn from_declarations(owner: &str, declarations: &[FlowExtensionDeclaration]) -> Result<Self, PluginAssemblyError> {
        let mut registry = Self::new();
        for declaration in declarations {
            registry.register(owner, declaration)?;
        }
        Ok(registry)
    }

    pub(super) fn register(&mut self, owner: &str, declaration: &FlowExtensionDeclaration) -> Result<(), PluginAssemblyError> {
        let descriptor = declaration.descriptor(owner);
        if let Some(existing) = self.by_id.get(&descriptor.id) {
            if existing == &descriptor {
                return Ok(());
            }
            return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-id", format!("flow extension contribution {:?} has a conflicting descriptor or executable identity", descriptor.id)));
        }
        if let Some(existing) = self.by_extension_id.get(&descriptor.extension_id) {
            return Err(PluginAssemblyError::new("plugin-assembly.flow-extension-target", format!("flow extension {:?} is already owned by {:?}", descriptor.extension_id, existing)));
        }
        self.by_extension_id.insert(descriptor.extension_id.clone(), descriptor.id.clone());
        self.by_id.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    pub fn descriptors(&self) -> Vec<FlowExtensionDescriptor> {
        self.by_id.values().cloned().collect()
    }
}
