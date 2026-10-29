//! 📇 Component-level composition and validation for selected artifact packages: the feature-free full stdio catalog
//! that the stdio component, the native hub and every composition validating stdio contributions link.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

#[path = "../../🛂️manifest/🦀️.rs"]
pub mod manifest;

use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::{ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};
use semio_s_artifact_stdio_contract::registry::ContributionRegistry;
use semio_s_artifact_stdio_contract::{ArtifactContribution, NativeCodecFactory};
pub use semio_s_artifact_stdio_contract::{ArtifactAssembly, CapabilityCounts, CapabilityLedger, NativeCodecFactoryReceipt};
use semio_hub_stdio_catalog_projection::{NativeArtifactCatalogV1, NATIVE_ARTIFACT_CATALOG_TOPIC};
pub use semio_hub_stdio_catalog_projection::{artifact_catalog_contribution_for, component_package_id, native_artifact_catalog_dependency, validate_native_artifact_catalog_dependency};
use std::collections::{BTreeMap, BTreeSet};

/// 📚️ Schema exports authored by this concrete deployment owner.
pub const CATALOG_SCHEMA_DOCUMENTS: semio_framework_schema_registry::ScopeSchemaExports = semio_framework_schema_registry::ScopeSchemaExports {
    scope: "hub.compositions.stdio.catalog",
    exports: &[semio_framework_schema_registry::SchemaExport { id: "schema", leaves: semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: include_str!("../../📇️catalog/🧬️schema/🔣️.json"), proto: "" } }],
};

fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.definition", message)
}

include!("../../🤖️generated/🧩️composition/🦀️.rs");

pub fn selected_registry() -> Result<ContributionRegistry, PluginAssemblyError> {
    let registry = ContributionRegistry::new(selected_contributions())?;
    if let Some(contribution) = registry.contributions().iter().find(|contribution| contribution.identity == "txt") {
        let formats = (contribution.formats)().map_err(PluginAssemblyError::definition)?;
        if !formats.iter().any(|format| format.mimes.iter().any(|mime| mime == "text/plain")) {
            return Err(failure("TXT must own text/plain"));
        }
    }
    Ok(registry)
}

/// 📊 Returns the selected artifact schemas' validated ledger.
pub fn capability_ledger() -> Result<CapabilityLedger, PluginAssemblyError> {
    selected_registry()?.capability_ledger()
}

/// 🧾 Builds every selected artifact definition in authored order.
pub fn artifact_definitions() -> Result<Vec<ArtifactDefinition>, PluginAssemblyError> {
    selected_registry()?.artifact_definitions()
}

/// 🧭 Assembles every selected artifact package in authored order.
pub fn artifact_assemblies() -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {
    selected_registry()?.artifact_assemblies()
}

/// 🗂 Derives one selected artifact's formats from its own schema.
pub fn format_descriptors_for(artifact: &str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    selected_registry().map_err(|error| ArtifactDefinitionError::new("stdio.format", error.to_string()))?.format_descriptors_for(artifact)
}

/// 🛂 Derives the selected runtime formats from artifact-owned schemas.
pub fn format_descriptors() -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    selected_registry()?.format_descriptors()
}

fn native_codec_factories() -> Vec<NativeCodecFactory> {
    selected_contributions().into_iter().flat_map(|contribution| (contribution.native_codecs)()).collect()
}

/// 🧬 Lists only selected native runtime roots; definition-only artifacts are absent.
pub fn native_codec_artifact_kinds() -> Vec<semio_framework_plugin::ArtifactKindSpec> {
    let mut kinds = Vec::new();
    for factory in native_codec_factories() {
        let kind = (factory.kind)();
        if !kinds.contains(&kind) { kinds.push(kind); }
    }
    kinds
}

/// 📇️ Commits the exact definitions consumed by this guest assembly, without executable addresses.
pub fn artifact_catalog_contribution(assemblies: &[ArtifactAssembly]) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    artifact_catalog_contribution_for(&selected_registry()?, assemblies)
}

/// 🧮️ Recomputes the complete headless projection from the same artifact-owned assemblies.
pub fn native_artifact_catalog_contribution() -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    artifact_catalog_contribution(&artifact_assemblies()?)
}

fn catalog_value_matches(expected: &semio_framework::DslValue, actual: &semio_framework::DslValue) -> bool {
    use semio_framework::DslValue;
    match (expected, actual) {
        (DslValue::String(left), DslValue::String(right)) => left == right,
        (DslValue::Bool(left), DslValue::Bool(right)) => left == right,
        (DslValue::Array(left), DslValue::Array(right)) => left.len() == right.len() && left.iter().zip(right).all(|(a, b)| catalog_value_matches(a, b)),
        (DslValue::Object(left), DslValue::Object(right)) => {
            left.len() == right.len()
                && left.iter().all(|(key, value)| {
                    let mut matches = right.iter().filter(|(candidate, _)| candidate == key);
                    let Some((_, candidate)) = matches.next() else { return false };
                    matches.next().is_none() && catalog_value_matches(value, candidate)
                })
        }
        _ => false,
    }
}

struct CompiledNativeCatalogExpectation {
    payload: semio_framework::DslValue,
    catalog: NativeArtifactCatalogV1,
}

fn compiled_native_catalog_expectation() -> Result<&'static CompiledNativeCatalogExpectation, PluginAssemblyError> {
    static EXPECTED: std::sync::OnceLock<Result<CompiledNativeCatalogExpectation, PluginAssemblyError>> = std::sync::OnceLock::new();
    EXPECTED
        .get_or_init(|| {
            let contribution = native_artifact_catalog_contribution()?;
            let catalog = contribution.decode().map_err(|error| failure(format!("native catalog owner is not its closed schema: {error}")))?;
            Ok(CompiledNativeCatalogExpectation { payload: contribution.payload, catalog })
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// 🔐️ Admits only the exact guest-committed selected semantic projection.
pub fn validate_native_artifact_catalog_contributions(contributions: &[semio_framework::TopicContribution]) -> Result<(), PluginAssemblyError> {
    if contributions.len() > 256 {
        return Err(failure("native catalog contribution inventory exceeds 256 topics"));
    }
    let mut matches = contributions.iter().filter(|contribution| contribution.topic == "stdio.artifact-catalog" || contribution.topic.starts_with("stdio.artifact-catalog."));
    let actual = matches.next().ok_or_else(|| failure("decoded Stdio descriptor omits its semantic catalog commitment"))?;
    if matches.next().is_some() || actual.topic != NATIVE_ARTIFACT_CATALOG_TOPIC {
        return Err(failure("decoded Stdio descriptor repeats or replaces its semantic catalog topic"));
    }
    let expected = compiled_native_catalog_expectation()?;
    if !catalog_value_matches(&expected.payload, &actual.payload) {
        return Err(failure("decoded Stdio descriptor differs from native artifact semantics"));
    }
    let decoded: NativeArtifactCatalogV1 = actual.decode().map_err(|error| failure(format!("native catalog commitment is not its closed schema: {error}")))?;
    if decoded != expected.catalog {
        return Err(failure("decoded Stdio descriptor differs from its exact native commitment"));
    }
    Ok(())
}

/// 🔎️ Requires every decoded kind to equal the complete artifact-owned native catalog.
pub fn validate_native_codec_artifact_kinds(kinds: &[semio_framework_plugin::ArtifactKindSpec]) -> Result<(), PluginAssemblyError> {
    let expected = native_codec_artifact_kinds();
    if kinds.len() != expected.len() {
        return Err(failure("decoded descriptor omits or adds native artifact kinds"));
    }
    let actual = kinds.iter().map(|kind| (kind.id.as_str(), kind)).collect::<BTreeMap<_, _>>();
    let identities = expected.iter().map(|kind| kind.id.as_str()).collect::<BTreeSet<_>>();
    if actual.len() != kinds.len() || identities.len() != expected.len() || expected.iter().any(|kind| actual.get(kind.id.as_str()).copied() != Some(kind)) {
        return Err(failure("decoded descriptor differs from the complete native artifact catalog"));
    }
    Ok(())
}

/// 🧷 Emits receipts only when schema data explicitly authorizes the exact native factory.
pub fn native_codec_factory_receipts() -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
    semio_hub_stdio_catalog_projection::native_codec_factory_receipts_for(&selected_registry()?, &artifact_assemblies()?)
}

/// 🧬️ The live receipts the committed `📜️native-codec-factories.json` projection is GENERATED from — the same
/// bijection as [`native_codec_factory_receipts`], but before that projection is compared against them, so the
/// `native-codec-projection` verb can regenerate a projection a schema-hash change made stale.
pub fn live_native_codec_factory_receipts() -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
    selected_registry()?.native_codec_factory_receipts("stdio", component_package_id()?, env!("CARGO_PKG_VERSION"))
}
//#endregion NativeCodecFactoryReceipts

#[cfg(test)]
#[path = "🧪️tests/🔬️definition-roster/🦀️.rs"]
mod definition_roster_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
