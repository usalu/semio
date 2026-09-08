//! 📇 Component-level composition and validation for selected artifact packages.

use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::{ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};
use semio_s_artifact_stdio_contract::{
    capability_ledger as artifact_capability_ledger, kernel, native_codec_factory_receipts as artifact_native_codec_factory_receipts, native_codec_hash, pack, schema_summary, ArtifactContribution, NativeCodecFactory,
};
pub use semio_s_artifact_stdio_contract::{ArtifactAssembly, CapabilityCounts, CapabilityLedger, NativeCodecFactoryReceipt};
use std::collections::{BTreeMap, BTreeSet};

/// 📦 Reads the guest's canonical component identity from its Cargo component contract.
#[cfg(feature = "full-artifact-catalog")]
pub(crate) fn component_package_id() -> Result<&'static str, PluginAssemblyError> {
    let manifest = include_str!("../📦️packages/🦀️rust/Cargo.toml");
    if manifest.len() > 64 * 1024 {
        return Err(PluginAssemblyError::new("plugin-assembly.package-id", "component Cargo contract exceeds 64 KiB"));
    }
    let mut component = false;
    let mut component_seen = false;
    let mut package_id = None;
    for raw in manifest.lines() {
        let line = raw.trim();
        if line.starts_with('[') {
            component = line == "[package.metadata.component]";
            if component && std::mem::replace(&mut component_seen, true) {
                return Err(PluginAssemblyError::new("plugin-assembly.package-id", "component Cargo contract repeats its component section"));
            }
        } else if component {
            let Some((key, raw_value)) = line.split_once('=') else { continue };
            if key.trim() != "package" {
                continue;
            }
            if package_id.is_some() {
                return Err(PluginAssemblyError::new("plugin-assembly.package-id", "component Cargo contract repeats its package key"));
            }
            package_id = raw_value.trim().strip_prefix('"').and_then(|value| value.strip_suffix('"'));
        }
    }
    let package_id = package_id.ok_or_else(|| PluginAssemblyError::new("plugin-assembly.package-id", "component package identity is missing"))?;
    let suffix = package_id.strip_prefix("semio:").ok_or_else(|| PluginAssemblyError::new("plugin-assembly.package-id", "component package identity must use the semio namespace"))?;
    if suffix.is_empty() || suffix.starts_with('-') || suffix.ends_with('-') || suffix.contains("--") || !suffix.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
        return Err(PluginAssemblyError::new("plugin-assembly.package-id", "component package identity is not canonical semio:<lowercase-alnum-hyphen>"));
    }
    Ok(package_id)
}

#[cfg(all(test, feature = "full-artifact-catalog"))]
mod component_package_id_tests {
    #[test]
    fn component_package_identity_comes_from_the_canonical_cargo_contract() {
        assert_eq!(super::component_package_id().expect("stdio component package identity"), "semio:stdio");
    }
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeOpenableProviderSourceV1 {
    schema: String,
    provider_id: String,
    plugin_id: String,
    package_id: String,
    receipts: Vec<NativeOpenableReceiptSourceV1>,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeOpenableReceiptSourceV1 {
    artifact: String,
    factory_id: String,
    descriptor_codec_id: String,
    runtime_capability_id: String,
    artifact_kind: String,
    document_schema: String,
    extension: String,
    pack_schema_sha256: String,
    protocol_path: String,
}

fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.definition", message)
}

#[cfg(feature = "full-artifact-catalog")]
fn selected_contributions() -> Vec<ArtifactContribution> {
    vec![
        semio_s_artifact_stdio_binary::contribution(),
        semio_s_artifact_stdio_txt::contribution(),
        semio_s_artifact_stdio_xml::contribution(),
        semio_s_artifact_stdio_deflate::contribution(),
        semio_s_artifact_stdio_zip::contribution(),
        semio_s_artifact_stdio_json::contribution(),
        semio_s_artifact_stdio_csv::contribution(),
        semio_s_artifact_stdio_md::contribution(),
        semio_s_artifact_stdio_gltf::contribution(),
        semio_s_artifact_stdio_obj::contribution(),
        semio_s_artifact_stdio_stl::contribution(),
        semio_s_artifact_stdio_ply::contribution(),
        semio_s_artifact_stdio_las::contribution(),
        semio_s_artifact_stdio_step::contribution(),
        semio_s_artifact_stdio_ifc::contribution(),
        semio_s_artifact_stdio_dwg::contribution(),
        semio_s_artifact_stdio_dxf::contribution(),
        semio_s_artifact_stdio_svg::contribution(),
        semio_s_artifact_stdio_png::contribution(),
        semio_s_artifact_stdio_jpg::contribution(),
        semio_s_artifact_stdio_gif::contribution(),
        semio_s_artifact_stdio_bmp::contribution(),
        semio_s_artifact_stdio_tiff::contribution(),
        semio_s_artifact_stdio_pdf::contribution(),
        semio_s_artifact_stdio_docx::contribution(),
        semio_s_artifact_stdio_pptx::contribution(),
        semio_s_artifact_stdio_xlsx::contribution(),
        semio_s_artifact_stdio_bcf::contribution(),
        semio_s_artifact_stdio_semio::contribution(),
        semio_s_artifact_stdio_mp4::contribution(),
        semio_s_artifact_stdio_avi::contribution(),
        semio_s_artifact_stdio_mp3::contribution(),
        semio_s_artifact_stdio_wav::contribution(),
        semio_s_artifact_stdio_epw::contribution(),
        semio_s_artifact_stdio_tsv::contribution(),
        semio_s_artifact_stdio_html::contribution(),
    ]
}

#[cfg(all(feature = "home-io", not(feature = "full-artifact-catalog")))]
fn selected_contributions() -> Vec<ArtifactContribution> {
    vec![
        semio_s_artifact_stdio_binary::contribution(),
        semio_s_artifact_stdio_txt::contribution(),
        semio_s_artifact_stdio_xml::contribution(),
        semio_s_artifact_stdio_deflate::contribution(),
        semio_s_artifact_stdio_zip::contribution(),
        semio_s_artifact_stdio_json::contribution(),
        semio_s_artifact_stdio_csv::contribution(),
        semio_s_artifact_stdio_xlsx::contribution(),
    ]
}

#[cfg(not(any(feature = "full-artifact-catalog", feature = "home-io")))]
fn selected_contributions() -> Vec<ArtifactContribution> {
    Vec::new()
}

fn expected_artifact_count() -> usize {
    if cfg!(feature = "full-artifact-catalog") {
        36
    } else if cfg!(feature = "home-io") {
        8
    } else {
        0
    }
}

fn validate_catalog(contributions: &[ArtifactContribution]) -> Result<(), PluginAssemblyError> {
    let expected = expected_artifact_count();
    if contributions.len() != expected {
        return Err(failure(format!("expected {expected} artifact contributions, got {}", contributions.len())));
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
            for extension in &representation.extensions {
                if let Some(existing) = extensions.insert(extension.clone(), summary.identity.clone()) {
                    if existing != summary.identity {
                        return Err(failure(format!("extension {extension} is claimed by both {existing} and {}", summary.identity)));
                    }
                }
            }
            for mime in &representation.mimes {
                if let Some(existing) = mimes.insert(mime.clone(), summary.identity.clone()) {
                    if existing != summary.identity {
                        return Err(failure(format!("MIME {mime} is claimed by both {existing} and {}", summary.identity)));
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
    if expected > 0 && !summaries.iter().find(|summary| summary.artifact == "txt").is_some_and(|summary| summary.representations.iter().any(|item| item.mimes.iter().any(|mime| mime == "text/plain"))) {
        return Err(failure("TXT must own text/plain"));
    }
    Ok(())
}

/// 📊 Returns the combined ledger of the selected artifact schemas.
pub fn capability_ledger() -> Result<CapabilityLedger, PluginAssemblyError> {
    let contributions = selected_contributions();
    validate_catalog(&contributions)?;
    let mut ledger = CapabilityLedger::default();
    for contribution in contributions {
        ledger.include(artifact_capability_ledger(contribution.schema)?);
    }
    Ok(ledger)
}

/// 🧾 Builds every selected artifact definition in contribution order.
pub fn artifact_definitions() -> Result<Vec<ArtifactDefinition>, PluginAssemblyError> {
    let contributions = selected_contributions();
    validate_catalog(&contributions)?;
    contributions.iter().map(|contribution| (contribution.definition)()).collect()
}

/// 🧭 Assembles every selected artifact package in contribution order.
pub fn artifact_assemblies() -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {
    let contributions = selected_contributions();
    validate_catalog(&contributions)?;
    contributions
        .iter()
        .map(|contribution| {
            let assembly = (contribution.assembly)()?;
            if assembly.definition().identity().as_str() != format!("s.stdio.{}", contribution.identity) {
                return Err(failure(format!("artifact contribution {} returned a foreign assembly", contribution.identity)));
            }
            Ok(assembly)
        })
        .collect()
}

/// 🗂 Derives one selected artifact's formats from its local schema.
pub fn format_descriptors_for(artifact: &str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    selected_contributions()
        .into_iter()
        .find(|contribution| contribution.identity == artifact)
        .ok_or_else(|| ArtifactDefinitionError::new("stdio.format", format!("unknown stdio artifact {artifact}")))
        .and_then(|contribution| (contribution.formats)())
}

/// 🛂 Derives every selected runtime format descriptor from artifact-local schemas.
pub fn format_descriptors() -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    let contributions = selected_contributions();
    validate_catalog(&contributions)?;
    contributions.iter().map(|contribution| (contribution.formats)().map_err(PluginAssemblyError::definition)).collect::<Result<Vec<_>, _>>().map(|groups| groups.into_iter().flatten().collect())
}

fn native_codec_factories() -> Vec<NativeCodecFactory> {
    selected_contributions().into_iter().flat_map(|contribution| (contribution.native_codecs)()).collect()
}

/// 🧬 Lists only selected native runtime roots; definition-only artifacts are absent.
pub fn native_codec_artifact_kinds() -> Vec<semio_framework_plugin::ArtifactKindSpec> {
    native_codec_factories().into_iter().map(|factory| (factory.kind)()).collect()
}

#[cfg(feature = "full-artifact-catalog")]
fn validate_native_openable_projection(receipts: &[NativeCodecFactoryReceipt]) -> Result<(), PluginAssemblyError> {
    let provider: NativeOpenableProviderSourceV1 = pack::from_json_str(include_str!("🧬️schema/📜️native-codec-factories.json")).map_err(|error| failure(format!("cannot parse native codec receipt projection: {error}")))?;
    if provider.schema != "semio.stdio.native-openable-catalog-provider/v1" || provider.provider_id != "stdio/native-codecs/v1" || provider.plugin_id != "stdio" || provider.package_id != "semio:stdio" || provider.receipts.len() != receipts.len() {
        return Err(failure("native codec receipt projection identity or closure is invalid"));
    }
    let mut ordered = receipts.iter().collect::<Vec<_>>();
    ordered.sort_by(|left, right| left.factory_id.cmp(&right.factory_id));
    for (projected, receipt) in provider.receipts.iter().zip(ordered) {
        let factory = native_codec_factories().into_iter().find(|factory| factory.id == receipt.factory_id).ok_or_else(|| failure("native codec projection has no exact verified private factory"))?;
        let artifact = factory.artifact;
        if projected.artifact != artifact
            || projected.factory_id != receipt.factory_id
            || projected.descriptor_codec_id != receipt.descriptor_codec_id
            || projected.runtime_capability_id != receipt.runtime_capability_id
            || projected.artifact_kind != receipt.artifact_kind
            || projected.document_schema != receipt.schema
            || projected.extension != receipt.extension
            || native_codec_hash(&projected.pack_schema_sha256)? != receipt.pack_schema_hash
            || !projected.protocol_path.starts_with("🗿️artifacts/")
            || !projected.protocol_path.ends_with("📡️.protocol.semio")
        {
            return Err(failure(format!("native codec receipt projection differs from artifact-owned authority for {}", receipt.factory_id)));
        }
    }
    Ok(())
}

#[cfg(feature = "full-artifact-catalog")]
const NATIVE_ARTIFACT_CATALOG_TOPIC: &str = "stdio.artifact-catalog.v1";

#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct NativeArtifactCatalogV1 {
    schema: String,
    plugin_id: String,
    package_id: String,
    package_version: String,
    definitions: Vec<NativeCatalogDefinitionV1>,
    codecs: Vec<NativeCatalogCodecV1>,
}

#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct NativeCatalogDefinitionV1 {
    identity: String,
    capabilities: Vec<NativeCatalogCapabilityV1>,
}

#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct NativeCatalogCapabilityV1 {
    identity: String,
    kind: String,
    descriptor_sha256: String,
    executable: bool,
    claims: Vec<NativeCatalogClaimV1>,
    localizations: Vec<NativeCatalogLocalizationV1>,
}

#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeCatalogClaimV1 {
    namespace: String,
    value: String,
}

#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeCatalogLocalizationV1 {
    locale: String,
    text: String,
}

#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct NativeCatalogCodecV1 {
    factory_id: String,
    definition_identity: String,
    descriptor_codec_id: String,
    runtime_capability_id: String,
    artifact_kind: String,
    schema: String,
    extension: String,
    pack_schema_sha256: String,
}

#[cfg(feature = "full-artifact-catalog")]
fn catalog_hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(feature = "full-artifact-catalog")]
struct NativeCatalogProjectionBudget {
    projection: usize,
    descriptors: usize,
}

#[cfg(feature = "full-artifact-catalog")]
impl NativeCatalogProjectionBudget {
    fn new() -> Self {
        Self { projection: 0, descriptors: 0 }
    }

    fn charge(&mut self, bytes: usize) -> Result<(), PluginAssemblyError> {
        self.projection = self.projection.checked_add(bytes).filter(|value| *value <= 2 * 1024 * 1024).ok_or_else(|| failure("native catalog semantic commitment exceeds 2 MiB"))?;
        Ok(())
    }

    fn descriptor(&mut self, bytes: usize) -> Result<(), PluginAssemblyError> {
        self.descriptors = self.descriptors.checked_add(bytes).filter(|value| *value <= 16 * 1024 * 1024).ok_or_else(|| failure("native catalog descriptor hashing exceeds 16 MiB"))?;
        Ok(())
    }

    fn object(&mut self, fields: &[&str]) -> Result<(), PluginAssemblyError> {
        self.charge(2 + fields.len().saturating_sub(1))?;
        for field in fields {
            self.string(&[field], 64)?;
            self.charge(1)?;
        }
        Ok(())
    }

    fn array(&mut self, length: usize) -> Result<(), PluginAssemblyError> {
        self.charge(2 + length.saturating_sub(1))
    }

    fn string(&mut self, parts: &[&str], maximum: usize) -> Result<(), PluginAssemblyError> {
        let length = parts.iter().try_fold(0usize, |sum, part| sum.checked_add(part.len())).filter(|length| *length > 0 && *length <= maximum).ok_or_else(|| failure("native catalog string exceeds its owned schema bound"))?;
        let mut encoded = 2 + length;
        for byte in parts.iter().flat_map(|part| part.bytes()) {
            encoded += match byte {
                b'"' | b'\\' | 8 | 9 | 10 | 12 | 13 => 1,
                0..=31 => 5,
                _ => 0,
            };
        }
        self.charge(encoded)
    }
}

#[cfg(feature = "full-artifact-catalog")]
fn preflight_native_catalog_projection(assemblies: &[ArtifactAssembly], receipts: &[NativeCodecFactoryReceipt]) -> Result<NativeCatalogProjectionBudget, PluginAssemblyError> {
    if assemblies.len() != 36 || receipts.len() != 26 {
        return Err(failure("native catalog projection requires 36 definitions and 26 codecs"));
    }
    let mut budget = NativeCatalogProjectionBudget::new();
    budget.object(&["schema", "pluginId", "packageId", "packageVersion", "definitions", "codecs"])?;
    for value in ["semio.stdio.artifact-catalog/v1", "stdio", component_package_id()?, env!("CARGO_PKG_VERSION")] {
        budget.string(&[value], 16384)?;
    }
    budget.array(assemblies.len())?;
    for assembly in assemblies {
        let definition = assembly.definition();
        let count = definition.capabilities().count();
        if count == 0 || count > 2048 {
            return Err(failure("native catalog capability count exceeds its owned commitment bounds"));
        }
        budget.object(&["identity", "capabilities"])?;
        budget.string(&[definition.identity().as_str()], 4096)?;
        budget.array(count)?;
        for capability in definition.capabilities() {
            if capability.descriptor_bytes().len() > 2 * 1024 * 1024 || capability.claims().len() > 64 || capability.localizations().len() > 64 {
                return Err(failure("native catalog capability exceeds its owned commitment bounds"));
            }
            budget.descriptor(capability.descriptor_bytes().len())?;
            budget.object(&["identity", "kind", "descriptorSha256", "executable", "claims", "localizations"])?;
            budget.string(&[capability.identity().as_str()], 4096)?;
            budget.string(&[capability.kind().as_str()], 16384)?;
            budget.charge(66 + if capability.executable_identity().is_some() { 4 } else { 5 })?;
            budget.array(capability.claims().len())?;
            for claim in capability.claims() {
                budget.object(&["namespace", "value"])?;
                budget.string(&[claim.namespace().as_str()], 16384)?;
                budget.string(&[claim.value()], 16384)?;
            }
            budget.array(capability.localizations().len())?;
            for localization in capability.localizations() {
                budget.object(&["locale", "text"])?;
                budget.string(&[localization.locale().as_str()], 16384)?;
                budget.string(&[localization.text()], 16384)?;
            }
        }
    }
    budget.array(receipts.len())?;
    for receipt in receipts {
        let factory = native_codec_factories().into_iter().find(|factory| factory.id == receipt.factory_id).ok_or_else(|| failure("native catalog codec has no private artifact owner"))?;
        budget.object(&["factoryId", "definitionIdentity", "descriptorCodecId", "runtimeCapabilityId", "artifactKind", "schema", "extension", "packSchemaSha256"])?;
        budget.string(&[&receipt.factory_id], 16384)?;
        budget.string(&["s.stdio.", factory.artifact], 4096)?;
        budget.string(&[&receipt.descriptor_codec_id], 4096)?;
        budget.string(&[&receipt.runtime_capability_id], 4096)?;
        budget.string(&[&receipt.artifact_kind], 4096)?;
        budget.string(&[&receipt.schema], 16384)?;
        budget.string(&[&receipt.extension], 16384)?;
        budget.charge(66)?;
    }
    Ok(budget)
}

#[cfg(all(test, feature = "full-artifact-catalog"))]
mod catalog_projection_budget_tests {
    use super::*;

    #[test]
    fn catalog_projection_budget_matches_serde_and_refuses_before_overdraw() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/📇️native-catalog-surface/🧪️budget.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap() {
            let value = row["text"].as_str().unwrap();
            let mut budget = NativeCatalogProjectionBudget::new();
            budget.string(&[value], 16384).unwrap();
            let expected = row["encodedBytes"].as_u64().unwrap() as usize;
            assert_eq!(serde_json::to_vec(value).unwrap().len(), expected, "{}", row["id"]);
            assert_eq!(budget.projection, expected, "{}", row["id"]);
        }
        for row in fixture["aggregateCases"].as_array().unwrap() {
            let mut budget = NativeCatalogProjectionBudget::new();
            let projection = row["projectionCharges"].as_array().unwrap();
            let descriptors = row["descriptorCharges"].as_array().unwrap();
            let independent =
                projection.iter().map(|v| v.as_u64().unwrap()).sum::<u64>() <= fixture["projectionLimitBytes"].as_u64().unwrap() && descriptors.iter().map(|v| v.as_u64().unwrap()).sum::<u64>() <= fixture["descriptorLimitBytes"].as_u64().unwrap();
            let result = projection.iter().try_for_each(|value| budget.charge(value.as_u64().unwrap() as usize)).and_then(|()| descriptors.iter().try_for_each(|value| budget.descriptor(value.as_u64().unwrap() as usize)));
            assert_eq!(independent, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
            assert_eq!(result.is_ok(), independent, "{}", row["id"]);
            assert!(budget.projection <= 2 * 1024 * 1024 && budget.descriptors <= 16 * 1024 * 1024);
        }
        let mut budget = NativeCatalogProjectionBudget::new();
        assert!(budget.charge(usize::MAX).is_err());
        assert_eq!(budget.projection, 0);
        assert!(budget.descriptor(usize::MAX).is_err());
        assert_eq!(budget.descriptors, 0);
        assert!(budget.string(&[""], 16).is_err());
        assert!(budget.string(&["éé"], 3).is_err());
    }

    #[test]
    fn catalog_projection_preflight_matches_actual_serde_payload_bytes() {
        let assemblies = artifact_assemblies().unwrap();
        let receipts = native_codec_factory_receipts().unwrap();
        let expected = preflight_native_catalog_projection(&assemblies, &receipts).unwrap();
        let contribution = artifact_catalog_contribution(&assemblies).unwrap();
        let payload = serde_json::to_value(&contribution).unwrap()["payload"].clone();
        assert_eq!(serde_json::to_vec(&payload).unwrap().len(), expected.projection);
        assert!(expected.descriptors > 0);
    }
}

#[cfg(feature = "full-artifact-catalog")]
fn native_artifact_catalog(assemblies: &[ArtifactAssembly]) -> Result<NativeArtifactCatalogV1, PluginAssemblyError> {
    if assemblies.len() != 36 {
        return Err(failure("native catalog commitment requires all 36 artifact definitions"));
    }
    let receipts = native_codec_factory_receipts()?;
    preflight_native_catalog_projection(assemblies, &receipts)?;
    let mut definitions = Vec::with_capacity(36);
    for assembly in assemblies {
        let definition = assembly.definition();
        let mut capabilities = Vec::new();
        for capability in definition.capabilities() {
            if capabilities.len() == 2048 || capability.descriptor_bytes().len() > 2 * 1024 * 1024 || capability.claims().len() > 64 || capability.localizations().len() > 64 {
                return Err(failure("native catalog capability exceeds its owned commitment bounds"));
            }
            capabilities.push(NativeCatalogCapabilityV1 {
                identity: capability.identity().as_str().into(),
                kind: capability.kind().as_str().into(),
                descriptor_sha256: catalog_hex(&semio_framework_hash::Sha256::digest(capability.descriptor_bytes())),
                executable: capability.executable_identity().is_some(),
                claims: capability.claims().iter().map(|claim| NativeCatalogClaimV1 { namespace: claim.namespace().as_str().into(), value: claim.value().into() }).collect(),
                localizations: capability.localizations().iter().map(|localization| NativeCatalogLocalizationV1 { locale: localization.locale().as_str().into(), text: localization.text().into() }).collect(),
            });
        }
        if capabilities.is_empty() {
            return Err(failure("native catalog definition has no semantic capabilities"));
        }
        definitions.push(NativeCatalogDefinitionV1 { identity: definition.identity().as_str().into(), capabilities });
    }
    definitions.sort_by(|left, right| left.identity.cmp(&right.identity));
    let identities = definitions.iter().map(|definition| definition.identity.as_str()).collect::<BTreeSet<_>>();
    if identities.len() != 36 || receipts.len() != 26 {
        return Err(failure("native catalog definition and codec ownership is incomplete"));
    }
    let mut codecs = Vec::with_capacity(26);
    for receipt in receipts {
        let factory = native_codec_factories().into_iter().find(|factory| factory.id == receipt.factory_id).ok_or_else(|| failure("native catalog codec has no private artifact owner"))?;
        let definition_identity = format!("s.stdio.{}", factory.artifact);
        if !identities.contains(definition_identity.as_str()) {
            return Err(failure("native catalog codec omits its declared definition owner"));
        }
        codecs.push(NativeCatalogCodecV1 {
            factory_id: receipt.factory_id,
            definition_identity,
            descriptor_codec_id: receipt.descriptor_codec_id,
            runtime_capability_id: receipt.runtime_capability_id,
            artifact_kind: receipt.artifact_kind,
            schema: receipt.schema,
            extension: receipt.extension,
            pack_schema_sha256: catalog_hex(&receipt.pack_schema_hash),
        });
    }
    codecs.sort_by(|left, right| left.factory_id.cmp(&right.factory_id));
    Ok(NativeArtifactCatalogV1 { schema: "semio.stdio.artifact-catalog/v1".into(), plugin_id: "stdio".into(), package_id: component_package_id()?.into(), package_version: env!("CARGO_PKG_VERSION").into(), definitions, codecs })
}

/// 📇️ Commits the exact definitions consumed by this guest assembly, without executable addresses.
#[cfg(feature = "full-artifact-catalog")]
pub(crate) fn artifact_catalog_contribution(assemblies: &[ArtifactAssembly]) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    let payload = kernel::ToValue::to_value(&native_artifact_catalog(assemblies)?);
    if pack::json_to_string(&pack::json_from_dsl_value(&payload)).len() > 2 * 1024 * 1024 {
        return Err(failure("native catalog semantic commitment exceeds 2 MiB"));
    }
    Ok(semio_framework::TopicContribution::new(NATIVE_ARTIFACT_CATALOG_TOPIC, payload))
}

/// 🔗️ Binds consumers to the exact version of their statically linked Stdio catalog.
#[cfg(feature = "full-artifact-catalog")]
pub fn native_artifact_catalog_dependency() -> Result<semio_framework::PluginDependency, PluginAssemblyError> {
    let version = semio_framework::Version::parse(env!("CARGO_PKG_VERSION")).map_err(|error| failure(format!("compiled Stdio catalog version is invalid: {error}")))?;
    Ok(semio_framework::PluginDependency::new("stdio", semio_framework::VersionReq::Exact(version)))
}

/// 🪢️ Requires the consumer's sole dependency to equal its compiled catalog owner.
#[cfg(feature = "full-artifact-catalog")]
pub fn validate_native_artifact_catalog_dependency(dependencies: &[semio_framework::PluginDependency]) -> Result<(), PluginAssemblyError> {
    if dependencies != [native_artifact_catalog_dependency()?].as_slice() {
        return Err(failure("decoded consumer dependencies differ from the exact compiled Stdio catalog"));
    }
    Ok(())
}

/// 🧮️ Recomputes the complete headless projection from the same artifact-owned assemblies.
#[cfg(feature = "full-artifact-catalog")]
pub fn native_artifact_catalog_contribution() -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    artifact_catalog_contribution(&artifact_assemblies()?)
}

#[cfg(feature = "full-artifact-catalog")]
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

#[cfg(feature = "full-artifact-catalog")]
struct CompiledNativeCatalogExpectation {
    payload: semio_framework::DslValue,
    catalog: NativeArtifactCatalogV1,
}

#[cfg(feature = "full-artifact-catalog")]
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

/// 🔐️ Admits only the exact guest-committed 36-definition/26-codec semantic projection.
#[cfg(feature = "full-artifact-catalog")]
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
#[cfg(feature = "full-artifact-catalog")]
pub fn validate_native_codec_artifact_kinds(kinds: &[semio_framework_plugin::ArtifactKindSpec]) -> Result<(), PluginAssemblyError> {
    let expected = native_codec_artifact_kinds();
    if expected.len() != 26 || kinds.len() != expected.len() {
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
#[cfg(feature = "full-artifact-catalog")]
pub fn native_codec_factory_receipts() -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
    let contributions = selected_contributions();
    validate_catalog(&contributions)?;
    let runtime_artifacts = artifact_assemblies()?
        .into_iter()
        .filter_map(|assembly| match assembly {
            ArtifactAssembly::Runtime(declaration) => declaration.definition().identity().as_str().strip_prefix("s.stdio.").map(str::to_owned),
            ArtifactAssembly::Definition(_) => None,
        })
        .collect::<BTreeSet<_>>();
    let mut receipts = Vec::new();
    for contribution in &contributions {
        receipts.extend(artifact_native_codec_factory_receipts(contribution, "stdio", component_package_id()?, env!("CARGO_PKG_VERSION"))?);
    }
    let factories = native_codec_factories();
    let mut factory_ids = BTreeSet::new();
    let mut descriptor_ids = BTreeSet::new();
    let mut receipt_keys = BTreeSet::new();
    for receipt in &receipts {
        let factory = factories.iter().find(|factory| factory.id == receipt.factory_id).ok_or_else(|| failure(format!("receipt {} has no selected artifact factory", receipt.factory_id)))?;
        if !runtime_artifacts.contains(factory.artifact)
            || !factory_ids.insert(receipt.factory_id.as_str())
            || !descriptor_ids.insert(receipt.descriptor_codec_id.as_str())
            || !receipt_keys.insert((receipt.artifact_kind.as_str(), receipt.schema.as_str()))
        {
            return Err(failure(format!("receipt {} is not bijective with one selected runtime artifact", receipt.factory_id)));
        }
    }
    if receipts.len() != 26 || factories.len() != 26 || factory_ids.len() != 26 || descriptor_ids.len() != 26 || receipt_keys.len() != 26 {
        return Err(failure("native codec receipts and selected artifact factories are not a complete bijection"));
    }
    validate_native_openable_projection(&receipts)?;
    Ok(receipts)
}
//#endregion NativeCodecFactoryReceipts

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_contribution_identities_are_unique_and_schema_owned() {
        let contributions = selected_contributions();
        validate_catalog(&contributions).expect("selected contribution catalog");
        assert_eq!(contributions.iter().map(|item| item.identity).collect::<BTreeSet<_>>().len(), expected_artifact_count());
    }

    #[cfg(feature = "full-artifact-catalog")]
    #[test]
    fn full_catalog_preserves_definition_codec_and_ledger_counts() {
        assert_eq!(artifact_assemblies().expect("artifact assemblies").len(), 36);
        assert_eq!(native_codec_factory_receipts().expect("native codec receipts").len(), 26);
        let ledger = capability_ledger().expect("capability ledger");
        assert_eq!(ledger.declared, CapabilityCounts { codecs: 32, mutations: 3, inferences: 67 });
        assert_eq!(ledger.registered, CapabilityCounts { codecs: 26, mutations: 3, inferences: 67 });
        assert_eq!(ledger.implemented, CapabilityCounts { codecs: 26, mutations: 0, inferences: 0 });
        assert_eq!(ledger.verified, CapabilityCounts::default());
    }
}
