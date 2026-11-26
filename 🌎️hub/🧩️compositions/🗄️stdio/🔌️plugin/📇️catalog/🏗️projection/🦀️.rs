//! 🏗️ The roster-independent projection of the stdio artifact catalog: the native catalog commitment, its budget, the
//! native-openable receipt closure and the compiled dependency, computed from any validated contribution roster.


extern crate semio_framework_value_derive as value_derive;

use semio_plugin::{ArtifactCapability, ArtifactDefinition, PluginAssemblyError};
use semio_s_artifact_stdio_contract_assembly::registry::ContributionRegistry;
use semio_s_artifact_stdio_contract_assembly::{catalog_definition_from_schema, native_codec_hash, ArtifactAssembly, NativeCodecFactory, NativeCodecFactoryReceipt};
use std::collections::BTreeSet;

/// 📦 Reads the guest's canonical component identity from its Cargo component contract.
pub fn component_package_id() -> Result<&'static str, PluginAssemblyError> {
    let manifest = include_str!("../../../🧩️component/📦️packages/🦀️rust/Cargo.toml");
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

#[cfg(test)]
#[path = "../🧪️tests/🔬️component-package-id/🦀️.rs"]
mod component_package_id_tests;

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
    artifact_schema: String,
    extension: String,
    pack_schema_hash: String,
    protocol_source_sha256: String,
    protocol_path: String,
    definition_path: String,
}

#[derive(value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeCatalogBundleV1 {
    schema: String,
    definitions: Vec<NativeCatalogBundleDefinitionV1>,
    provider: NativeOpenableProviderSourceV1,
}

#[derive(value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeCatalogBundleDefinitionV1 {
    artifact: String,
    definition: String,
}

fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.definition", message)
}

fn validate_native_openable_projection_provider(provider: &NativeOpenableProviderSourceV1) -> Result<(), PluginAssemblyError> {
    if provider.schema != "semio.stdio.native-openable-catalog-provider/v1" || provider.provider_id != "stdio/native-codecs/v1" || provider.plugin_id != "stdio" || provider.package_id != component_package_id()? || provider.receipts.len() > 4096 {
        return Err(failure("native codec receipt projection identity or closure is invalid"));
    }
    for receipt in &provider.receipts {
        if native_codec_hash(&receipt.protocol_source_sha256).is_err()
            || !receipt.definition_path.starts_with("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/")
            || !receipt.definition_path.ends_with("/📜️artifact-definition.json")
            || !receipt.protocol_path.starts_with("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/")
            || !receipt.protocol_path.ends_with("📡️.protocol.semio")
        {
            return Err(failure(format!("native codec receipt projection names invalid artifact-owned authority for {}", receipt.factory_id)));
        }
    }
    Ok(())
}

fn validate_native_openable_projection(receipts: &[NativeCodecFactoryReceipt], factories: &[NativeCodecFactory]) -> Result<(), PluginAssemblyError> {
    let provider = semio_framework_pack_json::from_json_str(include_str!("../📜️native-codec-factories.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| failure(format!("cannot parse native codec receipt projection: {error}")))?;
    validate_native_openable_projection_provider(&provider)?;
    let projected_ids = provider.receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>();
    if projected_ids.len() != provider.receipts.len() {
        return Err(failure("native codec receipt projection repeats a factory identity"));
    }
    let authored = factories.iter().map(|factory| factory.id).collect::<BTreeSet<_>>();
    if projected_ids != authored {
        let join = |keys: Vec<&str>| if keys.is_empty() { "none".to_owned() } else { keys.join(", ") };
        return Err(failure(format!(
            "native codec receipt projection differs from its authored composition roster: projected without authored factory [{}]; authored factory without projection [{}]",
            join(projected_ids.difference(&authored).copied().collect()),
            join(authored.difference(&projected_ids).copied().collect())
        )));
    }
    for receipt in receipts {
        let projected = provider.receipts.iter().find(|projected| projected.factory_id == receipt.factory_id).ok_or_else(|| failure("native codec projection omits a selected factory"))?;
        let factory = factories.iter().find(|factory| factory.id == receipt.factory_id).ok_or_else(|| failure("native codec projection has no exact verified private factory"))?;
        let artifact = factory.artifact;
        if projected.artifact != artifact
            || projected.factory_id != receipt.factory_id
            || projected.descriptor_codec_id != receipt.descriptor_codec_id
            || projected.runtime_capability_id != receipt.runtime_capability_id
            || projected.artifact_kind != receipt.artifact_kind
            || projected.artifact_schema != receipt.schema
            || projected.extension != receipt.extension
            || native_codec_hash(&projected.pack_schema_hash)? != receipt.pack_schema_hash
        {
            return Err(failure(format!("native codec receipt projection differs from artifact-owned authority for {}", receipt.factory_id)));
        }
    }
    Ok(())
}

pub const NATIVE_ARTIFACT_CATALOG_TOPIC: &str = "stdio.artifact-catalog.v1";

/// 🏠️ The catalog topic one runtime host publishes for the artifacts whose complete runtime it hosts.
pub fn hosted_artifact_catalog_topic(host_plugin_id: &str) -> String {
    format!("stdio.artifact-catalog.{host_plugin_id}")
}

/// 🪪️ The plugin and component package one catalog commitment is published under.
pub struct CatalogIdentity<'a> {
    pub plugin_id: &'a str,
    pub package_id: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeArtifactCatalogV1 {
    schema: String,
    plugin_id: String,
    package_id: String,
    package_version: String,
    definitions: Vec<NativeCatalogDefinitionV1>,
    codecs: Vec<NativeCatalogCodecV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeCatalogDefinitionV1 {
    identity: String,
    capabilities: Vec<NativeCatalogCapabilityV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeCatalogCapabilityV1 {
    identity: String,
    kind: String,
    descriptor_sha256: String,
    executable: bool,
    claims: Vec<NativeCatalogClaimV1>,
    localizations: Vec<NativeCatalogLocalizationV1>,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
pub struct NativeCatalogClaimV1 {
    namespace: String,
    value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
pub struct NativeCatalogLocalizationV1 {
    locale: String,
    text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, value_derive::FromValue, value_derive::ToValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct NativeCatalogCodecV1 {
    factory_id: String,
    definition_identity: String,
    descriptor_codec_id: String,
    runtime_capability_id: String,
    artifact_kind: String,
    schema: String,
    extension: String,
    pack_schema_hash: String,
}

fn catalog_hex(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

struct NativeCatalogProjectionBudget {
    projection: usize,
    descriptors: usize,
}

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

struct ProjectionDefinition<'a> {
    definition: &'a ArtifactDefinition,
    authored_executables: Option<&'a BTreeSet<String>>,
}

impl ProjectionDefinition<'_> {
    fn executable(&self, capability: &ArtifactCapability) -> bool {
        self.authored_executables.map_or_else(|| capability.executable_identity().is_some(), |identities| identities.contains(capability.identity().as_str()))
    }
}

fn preflight_native_catalog_projection(identity: &CatalogIdentity<'_>, definitions: &[ProjectionDefinition<'_>], codecs: &[NativeCatalogCodecV1]) -> Result<NativeCatalogProjectionBudget, PluginAssemblyError> {
    if definitions.is_empty() || definitions.len() > 4096 || codecs.len() > 4096 {
        return Err(failure("native catalog projection exceeds its bounds"));
    }
    let mut budget = NativeCatalogProjectionBudget::new();
    budget.object(&["schema", "pluginId", "packageId", "packageVersion", "definitions", "codecs"])?;
    for value in ["semio.stdio.artifact-catalog/v1", identity.plugin_id, identity.package_id, env!("CARGO_PKG_VERSION")] {
        budget.string(&[value], 16384)?;
    }
    budget.array(definitions.len())?;
    for projected in definitions {
        let definition = projected.definition;
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
            budget.charge(66 + if projected.executable(capability) { 4 } else { 5 })?;
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
    budget.array(codecs.len())?;
    for codec in codecs {
        budget.object(&["factoryId", "definitionIdentity", "descriptorCodecId", "runtimeCapabilityId", "artifactKind", "schema", "extension", "packSchemaHash"])?;
        budget.string(&[&codec.factory_id], 16384)?;
        budget.string(&[&codec.definition_identity], 4096)?;
        budget.string(&[&codec.descriptor_codec_id], 4096)?;
        budget.string(&[&codec.runtime_capability_id], 4096)?;
        budget.string(&[&codec.artifact_kind], 4096)?;
        budget.string(&[&codec.schema], 16384)?;
        budget.string(&[&codec.extension], 16384)?;
        budget.charge(66)?;
    }
    Ok(budget)
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️catalog-projection-budget/🦀️.rs"]
mod catalog_projection_budget_tests;

fn native_artifact_catalog(identity: &CatalogIdentity<'_>, projected: &[ProjectionDefinition<'_>], mut codecs: Vec<NativeCatalogCodecV1>) -> Result<NativeArtifactCatalogV1, PluginAssemblyError> {
    preflight_native_catalog_projection(identity, projected, &codecs)?;
    let mut definitions = Vec::with_capacity(projected.len());
    for owner in projected {
        let definition = owner.definition;
        let capabilities = definition.capabilities().map(|capability| NativeCatalogCapabilityV1 {
            identity: capability.identity().as_str().into(),
            kind: capability.kind().as_str().into(),
            descriptor_sha256: catalog_hex(&semio_framework_hash::Sha256::digest(capability.descriptor_bytes())),
            executable: owner.executable(capability),
            claims: capability.claims().iter().map(|claim| NativeCatalogClaimV1 { namespace: claim.namespace().as_str().into(), value: claim.value().into() }).collect(),
            localizations: capability.localizations().iter().map(|localization| NativeCatalogLocalizationV1 { locale: localization.locale().as_str().into(), text: localization.text().into() }).collect(),
        }).collect();
        definitions.push(NativeCatalogDefinitionV1 { identity: definition.identity().as_str().into(), capabilities });
    }
    definitions.sort_by(|left, right| left.identity.cmp(&right.identity));
    let identities = definitions.iter().map(|definition| definition.identity.as_str()).collect::<BTreeSet<_>>();
    if identities.len() != projected.len() || codecs.iter().any(|codec| !identities.contains(codec.definition_identity.as_str())) {
        return Err(failure("native catalog definition and codec ownership is incomplete"));
    }
    codecs.sort_by(|left, right| left.factory_id.cmp(&right.factory_id));
    Ok(NativeArtifactCatalogV1 { schema: "semio.stdio.artifact-catalog/v1".into(), plugin_id: identity.plugin_id.into(), package_id: identity.package_id.into(), package_version: env!("CARGO_PKG_VERSION").into(), definitions, codecs })
}

/// 🧩 Commits the exact authored selection and verifies its native receipt closure.
pub fn artifact_catalog_contribution_for(registry: &ContributionRegistry, assemblies: &[ArtifactAssembly]) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    let selected = registry.contributions().iter().map(|contribution| format!("s.stdio.{}", contribution.identity)).collect::<BTreeSet<_>>();
    let actual = assemblies.iter().map(|assembly| assembly.definition().identity().as_str()).collect::<BTreeSet<_>>();
    if actual.len() != assemblies.len() || selected.len() != actual.len() || !actual.iter().all(|identity| selected.contains(*identity)) {
        return Err(failure("native catalog assemblies differ from the exact selected roster"));
    }
    let receipts = native_codec_factory_receipts_for(registry, assemblies)?;
    let factories = registry.native_codec_factories();
    let authored = assemblies.iter().map(|assembly| match registry.runtime_host_of(assembly.definition().identity().as_str().trim_start_matches("s.stdio.")) {
        Some(_) => registry.contributions().iter().find(|contribution| format!("s.stdio.{}", contribution.identity) == assembly.definition().identity().as_str()).map(|contribution| catalog_definition_from_schema(contribution.schema).map(|authored| authored.executable_ids)).transpose(),
        None => Ok(None),
    }).collect::<Result<Vec<_>, _>>()?;
    let definitions = assemblies.iter().zip(&authored).map(|(assembly, authored)| ProjectionDefinition { definition: assembly.definition(), authored_executables: authored.as_ref() }).collect::<Vec<_>>();
    let codecs = receipts.into_iter().map(|receipt| {
        let factory = factories.iter().find(|factory| factory.id == receipt.factory_id).ok_or_else(|| failure("native catalog codec has no private artifact owner"))?;
        Ok(NativeCatalogCodecV1 {
            factory_id: receipt.factory_id,
            definition_identity: format!("s.stdio.{}", factory.artifact),
            descriptor_codec_id: receipt.descriptor_codec_id,
            runtime_capability_id: receipt.runtime_capability_id,
            artifact_kind: receipt.artifact_kind,
            schema: receipt.schema,
            extension: receipt.extension,
            pack_schema_hash: catalog_hex(&receipt.pack_schema_hash),
        })
    }).collect::<Result<_, PluginAssemblyError>>()?;
    catalog_contribution(NATIVE_ARTIFACT_CATALOG_TOPIC, native_artifact_catalog(&CatalogIdentity { plugin_id: "stdio", package_id: component_package_id()? }, &definitions, codecs)?)
}

/// 🏠️ Commits the runtime catalog of the artifacts one host plugin runs while the catalog owner assembles only their definitions.
pub fn hosted_artifact_catalog_contribution_for(host_plugin_id: &str, host_package_id: &str, hosted: &[semio_s_artifact_stdio_contract_assembly::ArtifactContribution]) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    let authored = hosted.iter().map(|contribution| catalog_definition_from_schema(contribution.schema)).collect::<Result<Vec<_>, _>>()?;
    let definitions = authored.iter().map(|authored| ProjectionDefinition { definition: &authored.definition, authored_executables: Some(&authored.executable_ids) }).collect::<Vec<_>>();
    catalog_contribution(&hosted_artifact_catalog_topic(host_plugin_id), native_artifact_catalog(&CatalogIdentity { plugin_id: host_plugin_id, package_id: host_package_id }, &definitions, Vec::new())?)
}

/// 📜 Projects the composition's bounded schema bundle and verifies its authored native codec receipts.
pub fn artifact_catalog_contribution_from_bundle(stored: &[u8]) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    if stored.len() > 128 * 1024 {
        return Err(failure("compressed catalog definition projection exceeds 128 KiB"));
    }
    let bytes = semio_framework_deflate::inflate(stored, 1024 * 1024).map_err(|error| failure(format!("cannot inflate catalog definition projection: {error:?}")))?;
    let source = std::str::from_utf8(&bytes).map_err(|error| failure(format!("catalog definition projection is not UTF-8: {error}")))?;
    let bundle: NativeCatalogBundleV1 = semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| failure(format!("cannot parse catalog definition projection: {error}")))?;
    if bundle.schema != "semio.stdio.catalog-input/v1" || bundle.definitions.is_empty() || bundle.definitions.len() > 4096 {
        return Err(failure("catalog definition projection identity or closure is invalid"));
    }
    validate_native_openable_projection_provider(&bundle.provider)?;
    catalog_from_schema_bundle(bundle)
}

fn catalog_from_schema_bundle(bundle: NativeCatalogBundleV1) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    let NativeCatalogBundleV1 { definitions: schemas, provider, .. } = bundle;
    let mut definitions = Vec::with_capacity(schemas.len());
    let mut codecs = Vec::new();
    let mut receipt_ids = BTreeSet::new();
    let mut receipt_keys = BTreeSet::new();
    for NativeCatalogBundleDefinitionV1 { artifact, definition } in schemas {
        let authored = catalog_definition_from_schema(&definition)?;
        let identity = format!("s.stdio.{artifact}");
        if authored.definition.identity().as_str() != identity {
            return Err(failure("selected catalog schema differs from its artifact identity"));
        }
        for codec in &authored.native_codecs {
            let projected = provider.receipts.iter().find(|receipt| receipt.factory_id == codec.factory_id).ok_or_else(|| failure(format!("native codec receipt projection omits {}", codec.factory_id)))?;
            if projected.artifact != artifact
                || projected.descriptor_codec_id != codec.descriptor_codec_id
                || projected.runtime_capability_id != codec.runtime_capability_id
                || projected.artifact_kind != codec.artifact_kind
                || projected.artifact_schema != codec.schema
                || projected.extension != codec.extension
                || native_codec_hash(&projected.pack_schema_hash)? != codec.pack_schema_hash
                || !receipt_ids.insert(projected.factory_id.as_str())
                || !receipt_keys.insert((projected.artifact_kind.as_str(), projected.artifact_schema.as_str()))
            {
                return Err(failure(format!("native codec receipt projection differs from authored definition for {}", codec.factory_id)));
            }
            codecs.push(NativeCatalogCodecV1 {
                factory_id: codec.factory_id.clone(),
                definition_identity: identity.clone(),
                descriptor_codec_id: codec.descriptor_codec_id.clone(),
                runtime_capability_id: codec.runtime_capability_id.clone(),
                artifact_kind: codec.artifact_kind.clone(),
                schema: codec.schema.clone(),
                extension: codec.extension.clone(),
                pack_schema_hash: catalog_hex(&codec.pack_schema_hash),
            });
        }
        definitions.push(authored);
    }
    if receipt_ids.len() != provider.receipts.len() {
        return Err(failure("native codec receipt projection includes an unselected or repeated factory"));
    }
    let definitions = definitions.iter().map(|authored| ProjectionDefinition { definition: &authored.definition, authored_executables: Some(&authored.executable_ids) }).collect::<Vec<_>>();
    catalog_contribution(NATIVE_ARTIFACT_CATALOG_TOPIC, native_artifact_catalog(&CatalogIdentity { plugin_id: "stdio", package_id: component_package_id()? }, &definitions, codecs)?)
}

fn catalog_contribution(topic: &str, catalog: NativeArtifactCatalogV1) -> Result<semio_framework::TopicContribution, PluginAssemblyError> {
    let payload = semio_framework_value::ToValue::to_value(&catalog);
    if semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&payload)).len() > 2 * 1024 * 1024 {
        return Err(failure("native catalog semantic commitment exceeds 2 MiB"));
    }
    Ok(semio_framework::TopicContribution::new(topic, payload))
}

/// 🔗️ Binds consumers to the exact version of their statically linked Stdio catalog.
pub fn native_artifact_catalog_dependency() -> Result<semio_framework::PluginDependency, PluginAssemblyError> {
    Ok(semio_framework::PluginDependency::new("stdio", semio_framework::tree_pin!()))
}

/// 🪢️ Requires the consumer's sole dependency to equal its compiled catalog owner.
pub fn validate_native_artifact_catalog_dependency(dependencies: &[semio_framework::PluginDependency]) -> Result<(), PluginAssemblyError> {
    if dependencies != [native_artifact_catalog_dependency()?].as_slice() {
        return Err(failure("decoded consumer dependencies differ from the exact compiled Stdio catalog"));
    }
    Ok(())
}

/// 🧷 The same receipts, derived from assemblies the caller ALREADY owns.
///
/// The only thing the assemblies are read for here is which artifacts carry a runtime declaration,
/// and `native_artifact_catalog` is handed the complete 36-assembly slice by `plugin()`. Building a
/// second, identical set just to answer that question re-parsed, re-validated and re-BUILT every
/// definition a second time — the single largest avoidable cost of the guest's `describe()`.
pub fn native_codec_factory_receipts_for(registry: &ContributionRegistry, assemblies: &[ArtifactAssembly]) -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
    let receipts = registry.native_codec_factory_receipts("stdio", component_package_id()?, env!("CARGO_PKG_VERSION"))?;
    let runtime_artifacts = assemblies
        .iter()
        .filter_map(|assembly| match assembly {
            ArtifactAssembly::Runtime(declaration) => declaration.definition().identity().as_str().strip_prefix("s.stdio."),
            ArtifactAssembly::Definition(_) => None,
        })
        .collect::<BTreeSet<_>>();
    let factories = registry.native_codec_factories();
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
    if [receipts.len(), factories.len(), factory_ids.len(), descriptor_ids.len(), receipt_keys.len()].iter().any(|count| *count != factories.len()) {
        return Err(failure("native codec receipts and selected artifact factories are not a complete bijection"));
    }
    validate_native_openable_projection(&receipts, &factories)?;
    Ok(receipts)
}

