//! 🗄️ Caller-authored Stdio contribution assembly.

use semio_framework_plugin::plugin_app_close_prelude::{PluginBuilder, Ready};
use semio_framework_plugin::{PluginApp, PluginAssemblyError};
use semio_s_artifact_stdio_contract::registry::ContributionRegistry;
use semio_s_artifact_stdio_contract::{ArtifactAssembly, NativeCodecFactoryReceipt};
use std::collections::BTreeSet;

/// 🪪️ Component identity authored by the caller that owns this deployment.
pub struct AssemblyOwner<'a> {
    pub plugin_id: &'static str,
    pub package_id: &'a str,
    pub package_version: &'static str,
}

/// 📦️ A validated, caller-selected assembly with no concrete catalog defaults.
pub struct AssemblyPlan {
    assemblies: Vec<ArtifactAssembly>,
    receipts: Vec<NativeCodecFactoryReceipt>,
    kinds: Vec<semio_framework_plugin::ArtifactKindSpec>,
}

impl AssemblyPlan {
    /// 🛂️ Checks owner-authorized native factories against the exact runtime roster.
    pub fn new(registry: &ContributionRegistry, owner: AssemblyOwner<'_>) -> Result<Self, PluginAssemblyError> {
        let assemblies = registry.artifact_assemblies()?;
        let receipts = registry.native_codec_factory_receipts(owner.plugin_id, owner.package_id, owner.package_version)?;
        let runtime = assemblies.iter().filter_map(|assembly| match assembly {
            ArtifactAssembly::Runtime(declaration) => Some(declaration.definition().identity().as_str()),
            ArtifactAssembly::Definition(_) => None,
        }).collect::<BTreeSet<_>>();
        let factories = registry.native_codec_factories();
        for factory in &factories {
            let kind = (factory.kind)();
            if !runtime.contains(kind.id.as_str()) || !receipts.iter().any(|receipt| receipt.factory_id == factory.id && receipt.artifact_kind == kind.id) {
                return Err(PluginAssemblyError::new("stdio.assembly", format!("factory {} has no exact selected runtime declaration and receipt", factory.id)));
            }
        }
        if receipts.len() != factories.len() {
            return Err(PluginAssemblyError::new("stdio.assembly", "native factories and receipts are not a complete bijection"));
        }
        let kinds = factories.into_iter().map(|factory| (factory.kind)()).collect();
        Ok(Self { assemblies, receipts, kinds })
    }

    /// 📇️ Exposes the validated assembly for owner-authored publication.
    pub fn assemblies(&self) -> &[ArtifactAssembly] {
        &self.assemblies
    }

    /// 🧷️ Exposes verified receipts for owner-authored publication.
    pub fn receipts(&self) -> &[NativeCodecFactoryReceipt] {
        &self.receipts
    }

    /// 🧩️ Adds the selected definitions and runtime declarations to the caller's app builder.
    pub fn apply<PA: PluginApp>(self, mut builder: PluginBuilder<Ready, PA>) -> PluginBuilder<Ready, PA> {
        for assembly in self.assemblies {
            builder = match assembly {
                ArtifactAssembly::Definition(definition) => builder.artifact_definition(definition),
                ArtifactAssembly::Runtime(declaration) => builder.artifact(*declaration),
            };
        }
        for kind in self.kinds {
            builder = builder.artifact_kind(kind);
        }
        builder
    }
}

/// 🧩️ Assembles an explicitly supplied contribution registry into the caller's app builder.
pub fn assemble<PA: PluginApp>(builder: PluginBuilder<Ready, PA>, registry: &ContributionRegistry, owner: AssemblyOwner<'_>) -> Result<PluginBuilder<Ready, PA>, PluginAssemblyError> {
    Ok(AssemblyPlan::new(registry, owner)?.apply(builder))
}
