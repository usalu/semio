//! 📇 Validated assembly of caller-authored artifact contributions.

use crate::{capability_ledger, native_codec_factory_receipts, schema_summary, ArtifactAssembly, ArtifactContribution, CapabilityLedger, NativeCodecFactory, NativeCodecFactoryReceipt};
use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::{ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};
use std::collections::{BTreeMap, BTreeSet};

fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.contributions", message)
}

fn validate_catalog(contributions: &[ArtifactContribution]) -> Result<(), PluginAssemblyError> {
    if contributions.len() > 4096 {
        return Err(failure("selected contribution roster exceeds 4096 owners"));
    }
    for contribution in contributions {
        if let Some(constraint) = contribution.definition_constraint {
            crate::validate_definition_constraint(contribution.schema, constraint)?;
        }
    }
    let summaries = contributions.iter().map(|contribution| schema_summary(contribution.schema)).collect::<Result<Vec<_>, _>>()?;
    let available = summaries.iter().map(|summary| summary.identity.as_str()).collect::<BTreeSet<_>>();
    let mut identities = BTreeSet::new();
    let mut directories = BTreeSet::new();
    let mut mimes = BTreeMap::new();
    let mut extensions = BTreeMap::new();
    let mut dialects = BTreeSet::new();
    let mut runtime_capabilities = BTreeSet::new();
    for (contribution, summary) in contributions.iter().zip(&summaries) {
        if contribution.identity != summary.artifact || summary.identity != format!("s.stdio.{}", contribution.identity) || !identities.insert(summary.identity.clone()) || !directories.insert(summary.directory.clone()) {
            return Err(failure(format!("invalid or duplicate artifact contribution {}", contribution.identity)));
        }
        for representation in &summary.representations {
            for (namespace, claims, owners) in [("extension", &representation.extensions, &mut extensions), ("MIME", &representation.mimes, &mut mimes)] {
                for claim in claims {
                    if let Some(existing) = owners.insert(claim.clone(), summary.identity.clone()) {
                        if existing != summary.identity {
                            return Err(failure(format!("{namespace} {claim} is claimed by both {existing} and {}", summary.identity)));
                        }
                    }
                }
            }
        }
        for capability in &summary.runtime_capabilities {
            if !runtime_capabilities.insert(capability) {
                return Err(failure(format!("duplicate runtime capability {capability}")));
            }
        }
        for dialect in &summary.source_dialects {
            if !dialects.insert(dialect) {
                return Err(failure(format!("duplicate dialect {dialect}")));
            }
        }
        for dependency in &summary.dependencies {
            if dependency == &summary.identity || !available.contains(dependency.as_str()) {
                return Err(failure(format!("{} has unresolved dependency {dependency}", summary.identity)));
            }
        }
    }
    Ok(())
}

/// 🪢 Validated contribution roster with atomic registration and removal.
#[derive(Clone)]
pub struct ContributionRegistry {
    contributions: Vec<ArtifactContribution>,
}

impl ContributionRegistry {
    /// 📥 Accepts the exact authored roster without catalog defaults.
    pub fn new(contributions: Vec<ArtifactContribution>) -> Result<Self, PluginAssemblyError> {
        validate_catalog(&contributions)?;
        Ok(Self { contributions })
    }

    /// 📋 Borrows the selected contributions in authored assembly order.
    pub fn contributions(&self) -> &[ArtifactContribution] {
        &self.contributions
    }

    /// ➕ Registers one contribution after validating the complete resulting roster.
    pub fn register(&mut self, contribution: ArtifactContribution) -> Result<(), PluginAssemblyError> {
        let mut selected = self.contributions.clone();
        selected.push(contribution);
        validate_catalog(&selected)?;
        self.contributions = selected;
        Ok(())
    }

    /// ➖ Removes an owner only when remaining declared dependencies still resolve.
    pub fn remove(&mut self, identity: &str) -> Result<bool, PluginAssemblyError> {
        let selected: Vec<_> = self.contributions.iter().copied().filter(|contribution| contribution.identity != identity).collect();
        if selected.len() == self.contributions.len() {
            return Ok(false);
        }
        validate_catalog(&selected)?;
        self.contributions = selected;
        Ok(true)
    }

    /// 📊 Combines only the selected schemas' capability ledgers.
    pub fn capability_ledger(&self) -> Result<CapabilityLedger, PluginAssemblyError> {
        let mut ledger = CapabilityLedger::default();
        for contribution in &self.contributions {
            ledger.include(capability_ledger(contribution.schema)?);
        }
        Ok(ledger)
    }

    /// 🧾 Builds definitions and rejects callbacks returning another owner.
    pub fn artifact_definitions(&self) -> Result<Vec<ArtifactDefinition>, PluginAssemblyError> {
        self.contributions
            .iter()
            .map(|contribution| {
                let definition = (contribution.definition)()?;
                if definition.identity().as_str() != format!("s.stdio.{}", contribution.identity) {
                    return Err(failure(format!("{} returned a foreign definition", contribution.identity)));
                }
                Ok(definition)
            })
            .collect()
    }

    /// 🧭 Builds executable declarations in selected contribution order.
    pub fn artifact_assemblies(&self) -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {
        self.contributions
            .iter()
            .map(|contribution| {
                let assembly = (contribution.assembly)()?;
                if assembly.definition().identity().as_str() != format!("s.stdio.{}", contribution.identity) {
                    return Err(failure(format!("{} returned a foreign assembly", contribution.identity)));
                }
                Ok(assembly)
            })
            .collect()
    }

    /// 🗂 Resolves formats solely within the selected roster.
    pub fn format_descriptors_for(&self, artifact: &str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
        let contribution = self.contributions.iter().find(|contribution| contribution.identity == artifact).ok_or_else(|| ArtifactDefinitionError::new("stdio.format", format!("unknown selected artifact {artifact}")))?;
        let formats = (contribution.formats)()?;
        let expected = schema_summary(contribution.schema).map_err(|error| ArtifactDefinitionError::new("stdio.format", error.to_string()))?.formats;
        if formats != expected {
            return Err(ArtifactDefinitionError::new("stdio.format", format!("{artifact} returned formats differing from its owned schema")));
        }
        Ok(formats)
    }

    /// 🛂 Builds the exact selected runtime formats without fallback lookups.
    pub fn format_descriptors(&self) -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
        self.contributions.iter().map(|contribution| self.format_descriptors_for(contribution.identity).map_err(PluginAssemblyError::definition)).collect::<Result<Vec<_>, _>>().map(|groups| groups.into_iter().flatten().collect())
    }

    /// 🏭 Lists artifact-owned native codec factories in contribution order.
    pub fn native_codec_factories(&self) -> Vec<NativeCodecFactory> {
        self.contributions.iter().flat_map(|contribution| (contribution.native_codecs)()).collect()
    }

    /// 🧷 Verifies schema-authorized factory receipts and catalog-wide ownership uniqueness.
    pub fn native_codec_factory_receipts(&self, plugin_id: &'static str, package_id: &str, package_version: &'static str) -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
        let mut receipts = Vec::new();
        let mut factory_ids = BTreeSet::new();
        let mut descriptor_ids = BTreeSet::new();
        let mut receipt_keys = BTreeSet::new();
        for contribution in &self.contributions {
            for receipt in native_codec_factory_receipts(contribution, plugin_id, package_id, package_version)? {
                if !factory_ids.insert(receipt.factory_id.clone()) || !descriptor_ids.insert(receipt.descriptor_codec_id.clone()) || !receipt_keys.insert((receipt.artifact_kind.clone(), receipt.schema.clone())) {
                    return Err(failure(format!("receipt {} is not unique within the selected roster", receipt.factory_id)));
                }
                receipts.push(receipt);
            }
        }
        Ok(receipts)
    }
}
