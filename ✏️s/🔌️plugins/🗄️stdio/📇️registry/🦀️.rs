//! 🧾️ Schema-owned stdio artifact-definition assembly.

use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::{
    ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, ArtifactExecutableIdentity, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, PluginAssemblyError,
};
use std::collections::{BTreeMap, BTreeSet};

//#region SourceSchema
#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Source {
    definition_version: u8,
    id: String,
    artifact: String,
    directory: String,
    dependencies: Vec<String>,
    standards: Vec<Standard>,
    profiles: Vec<Profile>,
    source_dialects: Vec<Dialect>,
    representations: Vec<Representation>,
    codecs: Vec<Codec>,
    mutations: Vec<ExecutableLeaf>,
    inferences: Vec<ExecutableLeaf>,
    resources: Vec<Resource>,
    localized_descriptors: Vec<Localized>,
    conformance_suites: Vec<Conformance>,
    runtime_capabilities: Vec<RuntimeCapability>,
    support_ledger: Ledger,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Standard {
    id: String,
    revision: String,
    normative_source: Option<String>,
    publication_date: Option<String>,
    source_checksum: Option<String>,
    redistribution_status: String,
    clauses_or_features: Vec<String>,
    status: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Profile {
    id: String,
    standard: String,
    profile: String,
    status: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Dialect {
    id: String,
    standard: String,
    dialect: String,
    registered_code_points: Vec<String>,
    status: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Representation {
    id: String,
    standard: String,
    representation: String,
    mimes: Vec<String>,
    extensions: Vec<String>,
    is_binary: bool,
    aliases: Vec<String>,
    neutral: bool,
    status: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Codec {
    id: String,
    status: String,
    from: String,
    to: String,
    executable_registration: bool,
    #[value(default, skip_serializing_if = "Option::is_none")]
    native_factory: Option<NativeCodecBinding>,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct NativeCodecBinding {
    factory_id: String,
    artifact_kind: String,
    document_schema: String,
    extension: String,
    pack_schema_hash: String,
    runtime_capability_id: String,
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

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct ExecutableLeaf {
    id: String,
    status: String,
    executable_registration: bool,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Resource {
    id: String,
    external_reference_policy: String,
    status: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Localized {
    id: String,
    locale: String,
    name: String,
    description: String,
    status: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Conformance {
    id: String,
    status: String,
    fixtures: Vec<String>,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct RuntimeCapability {
    id: String,
    category: String,
    descriptor: String,
    claims: Vec<RuntimeClaim>,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct RuntimeClaim {
    namespace: String,
    value: String,
}

#[derive(Clone, value_derive::FromValue, value_derive::ToValue)]
#[value(deny_unknown_fields)]
struct Ledger {
    normative_source: Option<String>,
    publication_date: Option<String>,
    source_checksum: Option<String>,
    redistribution_status: String,
    clauses_or_features: Vec<String>,
    profiles: Vec<String>,
    registered_code_points: Vec<String>,
    read: String,
    write: String,
    lossless: String,
    canonical: String,
    validators: Vec<String>,
    mutations: Vec<String>,
    inferences: Vec<String>,
    fixtures: Vec<String>,
}
//#endregion SourceSchema

//#region CapabilityLedger
/// 📊️ Category counts never conflate declaration, registration, implementation, or verification.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapabilityCounts {
    pub codecs: usize,
    pub mutations: usize,
    pub inferences: usize,
}

/// 📒️ Honest capability status ledger derived from schema leaves.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapabilityLedger {
    pub declared: CapabilityCounts,
    pub registered: CapabilityCounts,
    pub implemented: CapabilityCounts,
    pub verified: CapabilityCounts,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn capability_counts<T>(items: &[T], status: impl Fn(&T) -> &str, registered: impl Fn(&T) -> bool) -> (usize, usize, usize, usize) {
    (items.len(), items.iter().filter(|item| registered(item)).count(), items.iter().filter(|item| status(item) == "implemented").count(), items.iter().filter(|item| status(item) == "verified").count())
}

/// 📊️ Returns separate schema declaration, runtime registration, implementation, and verification counts.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn capability_ledger() -> Result<CapabilityLedger, PluginAssemblyError> {
    let values = sources()?;
    validate_catalog(&values)?;
    let mut ledger = CapabilityLedger::default();
    for source in &values {
        let (declared, registered, implemented, verified) = capability_counts(&source.codecs, |item| &item.status, |item| item.executable_registration);
        ledger.declared.codecs += declared;
        ledger.registered.codecs += registered;
        ledger.implemented.codecs += implemented;
        ledger.verified.codecs += verified;
        let (declared, registered, implemented, verified) = capability_counts(&source.mutations, |item| &item.status, |item| item.executable_registration);
        ledger.declared.mutations += declared;
        ledger.registered.mutations += registered;
        ledger.implemented.mutations += implemented;
        ledger.verified.mutations += verified;
        let (declared, registered, implemented, verified) = capability_counts(&source.inferences, |item| &item.status, |item| item.executable_registration);
        ledger.declared.inferences += declared;
        ledger.registered.inferences += registered;
        ledger.implemented.inferences += implemented;
        ledger.verified.inferences += verified;
    }
    Ok(ledger)
}
//#endregion CapabilityLedger

//#region SourceLoading
#[cfg(feature = "full-artifact-catalog")]
const SOURCES: [&str; 36] = [
    include_str!("../🗿️artifacts/💾️binary/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🔤️txt/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📰️xml/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🗜️deflate/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎒️zip/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧾️json/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📊️csv/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📝️md/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🗽️obj/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🔺️stl/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧱️ply/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/☁️las/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📐️step/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🏗️ifc/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖊️dwg/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖋️dxf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎨️svg/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📷️png/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📸️jpg/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎞️gif/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🪟️bmp/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖼️tiff/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📖️pdf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📜️docx/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📽️pptx/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📕️xlsx/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/💬️bcf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧿️semio/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎥️mp4/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📼️avi/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎵️mp3/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🔊️wav/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🌦️epw/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📑️tsv/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🌐️html/🧬️schema/📜️artifact-definition.json"),
];

#[cfg(all(feature = "home-io", not(feature = "full-artifact-catalog")))]
const SOURCES: [&str; 8] = [
    include_str!("../🗿️artifacts/💾️binary/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🔤️txt/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📰️xml/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🗜️deflate/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎒️zip/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧾️json/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📊️csv/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📕️xlsx/🧬️schema/📜️artifact-definition.json"),
];

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.definition", message)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn descriptor<T: dsl::ToValue>(value: &T) -> Result<Vec<u8>, PluginAssemblyError> {
    Ok(pack::to_json_string(value).into_bytes())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn sources() -> Result<Vec<Source>, PluginAssemblyError> {
    SOURCES.into_iter().map(|value| pack::from_json_str(value).map_err(|error| failure(format!("cannot parse artifact definition: {error}")))).collect()
}
//#endregion SourceLoading

//#region Validation
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn id(value: &str) -> Result<(), PluginAssemblyError> {
    ArtifactIdentity::parse(value).map(|_| ()).map_err(PluginAssemblyError::definition)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn child<'a>(id: &'a str, owner: &str, namespace: &str) -> Result<&'a str, PluginAssemblyError> {
    id.strip_prefix(&format!("{owner}.{namespace}.")).filter(|value| !value.contains('.')).ok_or_else(|| failure(format!("{id:?} is not a direct {namespace} leaf of {owner}")))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn versioned_leaf(id: &str, prefix: &str) -> Result<(), PluginAssemblyError> {
    let leaf = id.strip_prefix(prefix).ok_or_else(|| failure(format!("{id:?} is not owned by {prefix:?}")))?;
    let (semantic, version) = leaf.rsplit_once(".v").ok_or_else(|| failure(format!("{id:?} must end in a canonical vN leaf")))?;
    if semantic.is_empty() || semantic.contains('.') || version.is_empty() || !version.bytes().all(|byte| byte.is_ascii_digit()) || version.starts_with('0') {
        return Err(failure(format!("{id:?} must end in a canonical vN leaf")));
    }
    ArtifactIdentity::parse(id).map(|_| ()).map_err(PluginAssemblyError::definition)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn leaf_kind(category: &str) -> Result<ArtifactCapabilityKind, PluginAssemblyError> {
    match category {
        "schema" => Ok(ArtifactCapabilityKind::schema()),
        "inference" => Ok(ArtifactCapabilityKind::inference()),
        "codec" => Ok(ArtifactCapabilityKind::codec()),
        "representation" => Ok(ArtifactCapabilityKind::representation()),
        "grammar" => Ok(ArtifactCapabilityKind::grammar()),
        "composer" => Ok(ArtifactCapabilityKind::composer()),
        "subset-validator" => Ok(ArtifactCapabilityKind::subset_validator()),
        _ => Err(failure(format!("unknown runtime capability category {category:?}"))),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn representation_claims(item: &Representation) -> BTreeSet<(String, String)> {
    item.mimes.iter().map(|value| ("mime".into(), value.clone())).chain(item.extensions.iter().map(|value| ("extension".into(), value.clone()))).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn runtime_claims(item: &RuntimeCapability) -> BTreeSet<(String, String)> {
    item.claims.iter().map(|claim| (claim.namespace.clone(), claim.value.clone())).collect()
}

/// 🧷️ Maps exactly the schema leaves that declare a native executable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn executable_mappings(source: &Source) -> Result<BTreeMap<String, ArtifactExecutableIdentity>, PluginAssemblyError> {
    let mut mappings = BTreeMap::new();
    #[cfg(feature = "full-artifact-catalog")]
    {
        let services = match source.artifact.as_str() {
            "gltf" => crate::artifacts::gltf::gltf_inference_services(),
            _ => Vec::new(),
        };
        for service in services {
            let id = service.metadata().inference_schema.to_owned();
            if mappings.insert(id.clone(), service.executable_identity()).is_some() {
                return Err(failure(format!("{} repeats executable mapping {id}", source.id)));
            }
        }
        if source.artifact == "gltf" {
            use protocol::SemanticMutation;
            let registered: BTreeSet<&str> = source.mutations.iter().filter(|item| item.executable_registration).map(|item| item.id.as_str()).collect();
            for descriptor in crate::artifacts::gltf::schema::mutations::GltfMutation::kinds() {
                let id = format!("s.stdio.gltf.mutation.{}.v1", descriptor.kind);
                if !registered.contains(id.as_str()) {
                    continue;
                }
                let identity = ArtifactExecutableIdentity::from_function_pointer(crate::artifacts::gltf::schema::mutations::apply_gltf_mutation as *const ());
                if mappings.insert(id.clone(), identity).is_some() {
                    return Err(failure(format!("{} repeats executable mapping {id}", source.id)));
                }
            }
        }
    }
    for item in source.codecs.iter().filter(|item| item.executable_registration) {
        let binding = item.native_factory.as_ref().ok_or_else(|| failure(format!("executable codec {} omits its native factory binding", item.id)))?;
        let factory = native_codec_factories()
            .into_iter()
            .find(|factory| factory.id == binding.factory_id)
            .ok_or_else(|| failure(format!("executable codec {} names unknown native factory {}", item.id, binding.factory_id)))?;
        let identity = ArtifactExecutableIdentity::from_function_pointer(factory.codec as *const ());
        if mappings.insert(item.id.clone(), identity).is_some() {
            return Err(failure(format!("{} repeats executable mapping {}", source.id, item.id)));
        }
    }
    Ok(mappings)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn expected_executable_ids(source: &Source) -> BTreeSet<String> {
    source
        .codecs
        .iter()
        .map(|item| (&item.id, item.executable_registration))
        .chain(source.mutations.iter().map(|item| (&item.id, item.executable_registration)))
        .chain(source.inferences.iter().map(|item| (&item.id, item.executable_registration)))
        .filter(|(_, registered)| *registered)
        .map(|(id, _)| id.clone())
        .collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn same(label: &str, left: impl IntoIterator<Item = String>, right: impl IntoIterator<Item = String>) -> Result<(), PluginAssemblyError> {
    if left.into_iter().collect::<BTreeSet<_>>() != right.into_iter().collect::<BTreeSet<_>>() {
        return Err(failure(format!("{label} diverges from its schema collection")));
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate(source: &Source) -> Result<(), PluginAssemblyError> {
    let owner = format!("s.stdio.{}", source.artifact);
    if source.definition_version != 1 || source.id != owner {
        return Err(failure(format!("{owner} must use definition_version 1")));
    }
    if source.standards.is_empty()
        || source.profiles.is_empty()
        || source.source_dialects.is_empty()
        || source.representations.is_empty()
        || source.resources.is_empty()
        || source.localized_descriptors.len() != 2
        || source.conformance_suites.is_empty()
    {
        return Err(failure(format!("{owner} omits a required collection")));
    }
    id(&source.id)?;
    let standards = source.standards.iter().map(|item| item.id.clone()).collect::<BTreeSet<_>>();
    for item in &source.standards {
        if item.id != format!("{owner}.standard.{}", item.revision) {
            return Err(failure(format!("invalid standard {}", item.id)));
        }
        if item.status == "unverified" && (item.normative_source.is_some() || item.publication_date.is_some() || item.source_checksum.is_some() || item.redistribution_status != "unknown" || !item.clauses_or_features.is_empty()) {
            return Err(failure(format!("unverified standard {} carries unverifiable provenance", item.id)));
        }
        if item.status == "verified" && (item.normative_source.is_none() || item.publication_date.is_none() || item.source_checksum.is_none() || item.redistribution_status == "unknown" || item.clauses_or_features.is_empty()) {
            return Err(failure(format!("verified standard {} lacks provenance", item.id)));
        }
        if !matches!(item.status.as_str(), "unverified" | "verified") {
            return Err(failure(format!("invalid standard status {}", item.status)));
        }
        id(&item.id)?;
    }
    for item in &source.profiles {
        if !standards.contains(&item.standard) || item.id != format!("{}.profile.{}", item.standard, item.profile) || !matches!(item.status.as_str(), "unimplemented" | "opaque" | "implemented") {
            return Err(failure(format!("invalid profile {}", item.id)));
        }
        id(&item.id)?;
    }
    for item in &source.source_dialects {
        if !standards.contains(&item.standard) || item.id != format!("{}.dialect.{}", item.standard, item.dialect) || !matches!(item.status.as_str(), "unimplemented" | "opaque" | "implemented") {
            return Err(failure(format!("invalid source dialect {}", item.id)));
        }
        id(&item.id)?;
    }
    for item in &source.representations {
        if !standards.contains(&item.standard)
            || item.id != format!("{}.representation.{}", item.standard, item.representation)
            || item.extensions.is_empty()
            || item.extensions.iter().any(|extension| !extension.starts_with('.'))
            || item.status != "declared"
        {
            return Err(failure(format!("invalid representation {}", item.id)));
        }
        if item.mimes.iter().collect::<BTreeSet<_>>().len() != item.mimes.len() || item.extensions.iter().collect::<BTreeSet<_>>().len() != item.extensions.len() {
            return Err(failure(format!("duplicate representation claim {}", item.id)));
        }
        id(&item.id)?;
    }
    if standards != source.representations.iter().map(|item| item.standard.clone()).collect::<BTreeSet<_>>() {
        return Err(failure(format!("{owner} must give every declared standard its own representation")));
    }
    if source.artifact == "epw" && source.representations.iter().any(|item| !item.mimes.is_empty()) {
        return Err(failure("EPW must remain MIME-unregistered"));
    }
    let locales = source.localized_descriptors.iter().map(|item| item.locale.as_str()).collect::<BTreeSet<_>>();
    if locales != BTreeSet::from(["de", "en"]) {
        return Err(failure(format!("{owner} must own English and German descriptors")));
    }
    for item in &source.localized_descriptors {
        if item.id != format!("{owner}.localization.{}", item.locale) || item.name.is_empty() || item.description.is_empty() {
            return Err(failure(format!("invalid localization {}", item.id)));
        }
        id(&item.id)?;
    }
    for item in &source.resources {
        if item.status != "unimplemented" || item.external_reference_policy.is_empty() {
            return Err(failure(format!("invalid resource {}", item.id)));
        }
        child(&item.id, &owner, "resource")?;
        id(&item.id)?;
    }
    for item in &source.conformance_suites {
        if item.status != "unimplemented" {
            return Err(failure(format!("invalid conformance suite {}", item.id)));
        }
        child(&item.id, &owner, "conformance-suite")?;
        id(&item.id)?;
        for fixture in &item.fixtures {
            id(fixture)?;
        }
    }
    for item in &source.codecs {
        let standard = source.standards.iter().find(|standard| item.id.starts_with(&format!("{}.codec.", standard.id))).ok_or_else(|| failure(format!("invalid codec {}", item.id)))?;
        versioned_leaf(&item.id, &format!("{}.codec.", standard.id))?;
        if !source.source_dialects.iter().any(|dialect| dialect.id == item.from)
            || !source.source_dialects.iter().any(|dialect| dialect.id == item.to)
            || !matches!(item.status.as_str(), "unimplemented" | "implemented" | "verified")
        {
            return Err(failure(format!("invalid codec {}", item.id)));
        }
        match (&item.native_factory, item.executable_registration) {
            (None, false) => {}
            (Some(binding), true) if matches!(item.status.as_str(), "implemented" | "verified") => validate_native_codec_binding(source, item, binding)?,
            _ => return Err(failure(format!("codec {} must bind an exact native factory if and only if it is executable and implemented", item.id))),
        }
    }
    for (category, item) in source.mutations.iter().map(|item| ("mutation", item)).chain(source.inferences.iter().map(|item| ("inference", item))) {
        versioned_leaf(&item.id, &format!("{owner}.{category}."))?;
        if source.artifact == "gltf" && (item.id.contains(".no-mutation.") || item.id.contains(".set-snapshot.") || item.id.contains(".set-")) {
            return Err(failure(format!("GLTF capability {} is not a specific semantic command", item.id)));
        }
        if !matches!(item.status.as_str(), "unimplemented" | "implemented" | "verified") {
            return Err(failure(format!("invalid {category} {}", item.id)));
        }
    }
    let mappings = executable_mappings(source)?;
    if mappings.keys().cloned().collect::<BTreeSet<_>>() != expected_executable_ids(source) {
        return Err(failure(format!("{} executable mapping keys diverge from schema registrations", source.id)));
    }
    let mut runtime_ids = BTreeSet::new();
    let mut runtime_claim_sets = BTreeSet::new();
    for item in &source.runtime_capabilities {
        let prefix = match item.category.as_str() {
            "codec" | "representation" => format!("{}.{}.", source.standards.first().ok_or_else(|| failure("runtime capability requires an owning standard"))?.id, item.category),
            _ => format!("{owner}.{}.", item.category),
        };
        leaf_kind(&item.category)?;
        if item.category == "representation" {
            child(&item.id, &source.standards.first().ok_or_else(|| failure("runtime representation requires an owning standard"))?.id, "representation")?;
        } else {
            versioned_leaf(&item.id, &prefix)?;
        }
        if item.descriptor.trim().is_empty() || item.claims.is_empty() || !runtime_ids.insert(item.id.clone()) {
            return Err(failure(format!("invalid runtime capability {}", item.id)));
        }
        let claims = runtime_claims(item);
        if claims.len() != item.claims.len()
            || !item.claims.iter().all(|claim| matches!(claim.namespace.as_str(), "schema" | "codec" | "codec-extension" | "extension" | "mime" | "dialect" | "validated-dialect" | "grammar") && !claim.value.trim().is_empty())
            || (item.category == "subset-validator" && item.claims.iter().any(|claim| claim.namespace != "validated-dialect"))
            || !runtime_claim_sets.insert((item.category.clone(), claims.clone()))
        {
            return Err(failure(format!("invalid runtime capability claims for {}", item.id)));
        }
        if item.category == "representation" && !source.representations.iter().any(|representation| representation_claims(representation) == claims) {
            return Err(failure(format!("runtime representation {} does not claim a representation leaf", item.id)));
        }
    }
    let ledger = &source.support_ledger;
    let states = [&ledger.read, &ledger.write, &ledger.lossless, &ledger.canonical];
    if !states.into_iter().all(|state| matches!(state.as_str(), "unimplemented" | "opaque" | "implemented")) {
        return Err(failure(format!("{owner} has an invalid support state")));
    }
    if states.into_iter().any(|state| state == "implemented")
        && (ledger.normative_source.is_none()
            || ledger.publication_date.is_none()
            || ledger.source_checksum.is_none()
            || ledger.redistribution_status == "unknown"
            || ledger.clauses_or_features.is_empty()
            || ledger.validators.is_empty()
            || ledger.fixtures.is_empty())
    {
        return Err(failure(format!("{owner} claims implementation without normative, validator, and fixture evidence")));
    }
    same("ledger profiles", ledger.profiles.clone(), source.profiles.iter().map(|item| item.id.clone()))?;
    same("ledger code points", ledger.registered_code_points.clone(), source.source_dialects.iter().flat_map(|item| item.registered_code_points.clone()))?;
    same("ledger mutations", ledger.mutations.clone(), source.mutations.iter().map(|item| item.id.clone()))?;
    same("ledger inferences", ledger.inferences.clone(), source.inferences.iter().map(|item| item.id.clone()))?;
    same("ledger fixtures", ledger.fixtures.clone(), source.conformance_suites.iter().flat_map(|item| item.fixtures.clone()))?;
    let local = source
        .profiles
        .iter()
        .map(|item| item.id.clone())
        .chain(source.resources.iter().map(|item| item.id.clone()))
        .chain(source.codecs.iter().map(|item| item.id.clone()))
        .chain(source.mutations.iter().map(|item| item.id.clone()))
        .chain(source.inferences.iter().map(|item| item.id.clone()))
        .chain(source.conformance_suites.iter().flat_map(|item| std::iter::once(item.id.clone()).chain(item.fixtures.clone())))
        .collect::<BTreeSet<_>>();
    for reference in ledger.validators.iter().chain(&ledger.mutations).chain(&ledger.inferences).chain(&ledger.fixtures) {
        if !local.contains(reference) {
            return Err(failure(format!("{owner} ledger reference {reference:?} does not resolve locally")));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_catalog(values: &[Source]) -> Result<(), PluginAssemblyError> {
    let expected = if cfg!(feature = "full-artifact-catalog") { 36 } else { 8 };
    if values.len() != expected {
        return Err(failure(format!("expected {expected} artifact definitions, got {}", values.len())));
    }
    let mut identities = BTreeSet::new();
    let mut directories = BTreeSet::new();
    let mut mimes = BTreeMap::new();
    let mut extensions = BTreeMap::new();
    let mut dialects = BTreeSet::new();
    let mut runtime_capabilities = BTreeSet::new();
    for source in values {
        validate(source)?;
        if !identities.insert(source.id.clone()) || !directories.insert(source.directory.clone()) {
            return Err(failure(format!("duplicate artifact {}", source.id)));
        }
        for representation in &source.representations {
            for extension in &representation.extensions {
                if let Some(existing) = extensions.insert(extension.clone(), source.id.clone()) {
                    if existing != source.id {
                        return Err(failure(format!("extension {extension} is claimed by both {existing} and {}", source.id)));
                    }
                }
            }
            for mime in &representation.mimes {
                if let Some(existing) = mimes.insert(mime.clone(), source.id.clone()) {
                    if existing != source.id {
                        return Err(failure(format!("MIME {mime} is claimed by both {existing} and {}", source.id)));
                    }
                }
            }
        }
        for capability in &source.runtime_capabilities {
            if !runtime_capabilities.insert(capability.id.clone()) {
                return Err(failure(format!("duplicate runtime capability {}", capability.id)));
            }
        }
        for dialect in &source.source_dialects {
            if !dialects.insert(dialect.id.clone()) {
                return Err(failure(format!("duplicate dialect {}", dialect.id)));
            }
        }
        for dependency in &source.dependencies {
            if dependency == &source.id || !values.iter().any(|candidate| candidate.id == *dependency) {
                return Err(failure(format!("{} has unresolved dependency {dependency}", source.id)));
            }
        }
    }
    if !values.iter().find(|source| source.artifact == "txt").is_some_and(|source| source.representations.iter().any(|item| item.mimes.iter().any(|mime| mime == "text/plain"))) {
        return Err(failure("TXT must own text/plain"));
    }
    Ok(())
}
//#endregion Validation

//#region Assembly
/// 🧩️ One schema definition paired with its optional executable declaration.
pub enum ArtifactAssembly {
    Definition(ArtifactDefinition),
    Runtime(ArtifactDeclaration),
}

/// 🧷️ Binds an executable artifact root to the definition it owns.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn runtime_assembly(artifact: &str, definition: ArtifactDefinition, factory: fn(ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError>) -> Result<ArtifactAssembly, PluginAssemblyError> {
    if definition.identity().as_str() != format!("s.stdio.{artifact}") {
        return Err(failure(format!("runtime artifact {artifact} received definition {}", definition.identity())));
    }
    factory(definition).map(ArtifactAssembly::Runtime).map_err(PluginAssemblyError::definition)
}

/// 🧾️ Preserves a schema-only artifact without fabricating runtime capabilities.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition_only_assembly(artifact: &str, definition: ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError> {
    if definition.identity().as_str() != format!("s.stdio.{artifact}") {
        return Err(failure(format!("definition-only artifact {artifact} received definition {}", definition.identity())));
    }
    Ok(ArtifactAssembly::Definition(definition))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn declared_capability<T: dsl::ToValue>(mappings: &BTreeMap<String, ArtifactExecutableIdentity>, id: &str, kind: ArtifactCapabilityKind, value: &T) -> Result<ArtifactCapability, PluginAssemblyError> {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?, kind).descriptor(descriptor(value)?).map_err(PluginAssemblyError::definition)?;
    if capability.kind() == &ArtifactCapabilityKind::inference() {
        capability = capability
            .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), id).map_err(PluginAssemblyError::definition)?)
            .map_err(PluginAssemblyError::definition)?;
    }
    if let Some(executable) = mappings.get(id) {
        capability = capability.executable(*executable);
    }
    Ok(capability)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn runtime_capability(item: &RuntimeCapability) -> Result<ArtifactCapability, PluginAssemblyError> {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(&item.id).map_err(PluginAssemblyError::definition)?, leaf_kind(&item.category)?).descriptor(item.descriptor.as_bytes().to_vec()).map_err(PluginAssemblyError::definition)?;
    for claim in &item.claims {
        capability = capability
            .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(&claim.namespace).map_err(PluginAssemblyError::definition)?, &claim.value).map_err(PluginAssemblyError::definition)?)
            .map_err(PluginAssemblyError::definition)?;
    }
    Ok(capability)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn build(source: &Source) -> Result<ArtifactDefinition, PluginAssemblyError> {
    let mappings = executable_mappings(source)?;
    let mut definition = ArtifactDefinition::stdio(&source.artifact).map_err(PluginAssemblyError::definition)?;
    for item in &source.standards {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::standard(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.profiles {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::profile(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.source_dialects {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::source_dialect(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.representations {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::representation(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.codecs {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::codec(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.mutations {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::mutation(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.inferences {
        definition = definition.capability(declared_capability(&mappings, &item.id, ArtifactCapabilityKind::inference(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.runtime_capabilities {
        definition = definition.capability(runtime_capability(item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.resources {
        definition = definition.resource(child(&item.id, &source.id, "resource")?, descriptor(item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.localized_descriptors {
        definition = definition.localization(ArtifactLocale::parse(&item.locale).map_err(PluginAssemblyError::definition)?, format!("{}\n{}", item.name, item.description), descriptor(item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.conformance_suites {
        definition = definition.conformance_suite(child(&item.id, &source.id, "conformance-suite")?, descriptor(item)?).map_err(PluginAssemblyError::definition)?;
    }
    Ok(definition)
}

/// 🧾️ Builds every schema-owned artifact definition in catalog order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_definitions() -> Result<Vec<ArtifactDefinition>, PluginAssemblyError> {
    let values = sources()?;
    validate_catalog(&values)?;
    values.iter().map(build).collect()
}

/// 🧭️ Assembles every artifact root in schema-catalog order.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn artifact_factories() -> BTreeMap<&'static str, fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>> {
    let mut factories = BTreeMap::from([
        ("binary", crate::artifacts::binary::assembly as fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>),
        ("txt", crate::artifacts::txt::assembly),
        ("xml", crate::artifacts::xml::assembly),
        ("deflate", crate::artifacts::deflate::assembly),
        ("zip", crate::artifacts::zip::assembly),
        ("json", crate::artifacts::json::assembly),
        ("csv", crate::artifacts::csv::assembly),
        ("xlsx", crate::artifacts::xlsx::assembly),
    ]);
    #[cfg(feature = "full-artifact-catalog")]
    factories.extend(BTreeMap::from([
        ("md", crate::artifacts::md::assembly as fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>),
        ("gltf", crate::artifacts::gltf::assembly),
        ("obj", crate::artifacts::obj::assembly),
        ("stl", crate::artifacts::stl::assembly),
        ("ply", crate::artifacts::ply::assembly),
        ("las", crate::artifacts::las::assembly),
        ("step", crate::artifacts::step::assembly),
        ("ifc", crate::artifacts::ifc::assembly),
        ("dwg", crate::artifacts::dwg::assembly),
        ("dxf", crate::artifacts::dxf::assembly),
        ("svg", crate::artifacts::svg::assembly),
        ("png", crate::artifacts::png::assembly),
        ("jpg", crate::artifacts::jpg::assembly),
        ("gif", crate::artifacts::gif::assembly),
        ("bmp", crate::artifacts::bmp::assembly),
        ("tiff", crate::artifacts::tiff::assembly),
        ("pdf", crate::artifacts::pdf::assembly),
        ("docx", crate::artifacts::docx::assembly),
        ("pptx", crate::artifacts::pptx::assembly),
        ("bcf", crate::artifacts::bcf::assembly),
        ("semio", crate::artifacts::semio::assembly),
        ("mp4", crate::artifacts::mp4::assembly),
        ("avi", crate::artifacts::avi::assembly),
        ("mp3", crate::artifacts::mp3::assembly),
        ("wav", crate::artifacts::wav::assembly),
        ("epw", crate::artifacts::epw::assembly),
        ("tsv", crate::artifacts::tsv::assembly),
        ("html", crate::artifacts::html::assembly),
    ]));
    factories
}

/// 🧭️ Assembles every artifact root by its schema-owned artifact key.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_assemblies() -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {
    let factories = artifact_factories();
    let values = sources()?;
    validate_catalog(&values)?;
    if factories.keys().copied().collect::<BTreeSet<_>>() != values.iter().map(|source| source.artifact.as_str()).collect() {
        return Err(failure("artifact factory keys diverge from schema artifacts"));
    }
    values.iter().map(|source| factories.get(source.artifact.as_str()).ok_or_else(|| failure(format!("missing factory for {}", source.artifact)))?(build(source)?)).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn source_format_descriptors(source: &Source) -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    source
        .runtime_capabilities
        .iter()
        .filter(|capability| capability.category == "representation")
        .filter_map(|capability| source.representations.iter().filter(|representation| representation_claims(representation) == runtime_claims(capability)).min_by(|left, right| left.id.cmp(&right.id)).map(|representation| (source, representation)))
        .map(|(source, representation)| {
            let english = source.localized_descriptors.iter().find(|item| item.locale == "en").ok_or_else(|| failure(format!("{} has no English descriptor", source.id)))?;
            Ok(FormatDescriptor {
                kind_id: representation.id.clone(),
                short_id: representation.id.clone(),
                aliases: representation.aliases.clone(),
                mimes: representation.mimes.clone(),
                extensions: representation.extensions.clone(),
                name: english.name.clone(),
                full_name: english.description.clone(),
                neutral: representation.neutral,
                dir_name: source.directory.clone(),
                is_binary: representation.is_binary,
            })
        })
        .collect()
}

/// 🗂️ Derives one runtime root's format descriptors from its exact representation capability records.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn format_descriptors_for(artifact: &str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    let values = sources().and_then(|values| {
        validate_catalog(&values)?;
        let source = values.iter().find(|source| source.artifact == artifact).ok_or_else(|| failure(format!("unknown stdio artifact {artifact}")))?;
        source_format_descriptors(source)
    });
    values.map_err(|error| ArtifactDefinitionError::new("stdio.format", error.to_string()))
}

/// 🛂️ Derives every runtime format descriptor from schema-owned representations.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn format_descriptors() -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    let values = sources()?;
    validate_catalog(&values)?;
    values.iter().map(source_format_descriptors).collect::<Result<Vec<_>, _>>().map(|groups| groups.into_iter().flatten().collect())
}

//#region NativeCodecFactoryReceipts
/// 🪢 One native codec factory bound to exact descriptor and Cargo component identities.
#[cfg(feature = "full-artifact-catalog")]
#[derive(Clone)]
pub struct NativeCodecFactoryReceipt {
    pub plugin_id: &'static str,
    pub package_id: String,
    pub package_version: &'static str,
    pub factory_id: String,
    pub descriptor_codec_id: String,
    pub runtime_capability_id: String,
    pub artifact_kind: String,
    pub schema: String,
    pub pack_schema_hash: [u8; 32],
    pub extension: String,
    pub factory: fn() -> store::ArtifactCodec,
}

#[cfg(feature = "full-artifact-catalog")]
impl NativeCodecFactoryReceipt {
    /// 🔐 Rechecks the immutable factory result before a trusted loader can bind it.
    pub fn instantiate(&self) -> Result<store::ArtifactCodec, PluginAssemblyError> {
        let codec = (self.factory)();
        if self.plugin_id != "stdio"
            || self.package_id != crate::plugin::component_package_id()?
            || self.package_version != env!("CARGO_PKG_VERSION")
            || codec.schema != self.schema
            || codec.extension != self.extension
            || codec.pack_schema_hash != self.pack_schema_hash
            || codec.pack_schema_hash == [0; 32]
        {
            return Err(failure(format!("native codec receipt {} failed factory verification", self.artifact_kind)));
        }
        Ok(codec)
    }
}

#[derive(Clone, Copy)]
struct NativeCodecFactory {
    id: &'static str,
    artifact: &'static str,
    kind: fn() -> semio_framework_plugin::ArtifactKindSpec,
    codec: fn() -> store::ArtifactCodec,
}

macro_rules! native_codec_factory {
    ($factory:ident, $module:ident, $snapshot:ident, $mutation:ident, $schema:ident, $extension:literal, $protocol:literal) => {
        fn $factory() -> store::ArtifactCodec {
            let mut codec = store::ArtifactCodec::of::<crate::artifacts::$module::$snapshot, crate::artifacts::$module::$mutation>(crate::artifacts::$module::$schema);
            codec.extension = $extension;
            codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!($protocol));
            codec
        }
    };
}

#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(ply_codec, ply, PlySnapshot, PlyMutation, STDIO_PLY_DOCUMENT_SCHEMA, "ply", "../🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(stl_codec, stl, StlSnapshot, StlMutation, STDIO_STL_DOCUMENT_SCHEMA, "stl", "../🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(las_codec, las, LasSnapshot, LasMutation, STDIO_LAS_DOCUMENT_SCHEMA, "las", "../🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(dxf_codec, dxf, DxfSnapshot, DxfMutation, STDIO_DXF_DOCUMENT_SCHEMA, "dxf", "../🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(mp3_codec, mp3, Mp3Snapshot, Mp3Mutation, STDIO_MP3_DOCUMENT_SCHEMA, "mp3", "../🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
native_codec_factory!(xlsx_codec, xlsx, XlsxSnapshot, XlsxMutation, STDIO_XLSX_DOCUMENT_SCHEMA, "xlsx", "../🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(tiff_codec, tiff, TiffSnapshot, TiffMutation, STDIO_TIFF_DOCUMENT_SCHEMA, "tiff", "../🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(jpg_codec, jpg, JpgSnapshot, JpgMutation, STDIO_JPG_DOCUMENT_SCHEMA, "jpg", "../🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(avi_codec, avi, AviSnapshot, AviMutation, STDIO_AVI_DOCUMENT_SCHEMA, "semio", "../🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(png_codec, png, PngSnapshot, PngMutation, STDIO_PNG_DOCUMENT_SCHEMA, "png", "../🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
native_codec_factory!(csv_codec, csv, CsvSnapshot, CsvMutation, STDIO_CSV_DOCUMENT_SCHEMA, "csv", "../🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(md_codec, md, MdSnapshot, MdMutation, STDIO_MD_DOCUMENT_SCHEMA, "md", "../🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(docx_codec, docx, DocxSnapshot, DocxMutation, STDIO_DOCX_DOCUMENT_SCHEMA, "docx", "../🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(mp4_codec, mp4, Mp4Snapshot, Mp4Mutation, STDIO_MP4_DOCUMENT_SCHEMA, "semio", "../🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
native_codec_factory!(json_codec, json, JsonSnapshot, JsonMutation, STDIO_JSON_DOCUMENT_SCHEMA, "json", "../🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(gltf_codec, gltf, GltfSnapshot, GltfMutation, STDIO_GLTF_DOCUMENT_SCHEMA, "gltf", "../🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(bcf_codec, bcf, BcfSnapshot, BcfMutation, STDIO_BCF_DOCUMENT_SCHEMA, "bcf", "../🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
native_codec_factory!(zip_codec, zip, ZipSnapshot, ZipMutation, STDIO_ZIP_DOCUMENT_SCHEMA, "zip", "../🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
native_codec_factory!(xml_codec, xml, XmlSnapshot, XmlMutation, STDIO_XML_DOCUMENT_SCHEMA, "xml", "../🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
native_codec_factory!(deflate_codec, deflate, DeflateSnapshot, DeflateMutation, STDIO_DEFLATE_DOCUMENT_SCHEMA, "zz", "../🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(obj_codec, obj, ObjSnapshot, ObjMutation, STDIO_OBJ_DOCUMENT_SCHEMA, "obj", "../🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(pptx_codec, pptx, PptxSnapshot, PptxMutation, STDIO_PPTX_DOCUMENT_SCHEMA, "pptx", "../🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(step_codec, step, StepSnapshot, StepMutation, STDIO_STEP_DOCUMENT_SCHEMA, "step", "../🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(dwg_codec, dwg, DwgSnapshot, DwgMutation, STDIO_DWG_DOCUMENT_SCHEMA, "dwg", "../🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");
#[cfg(feature = "full-artifact-catalog")]
native_codec_factory!(svg_codec, svg, SvgSnapshot, SvgMutation, STDIO_SVG_DOCUMENT_SCHEMA, "svg", "../🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio");

#[cfg(feature = "full-artifact-catalog")]
fn pdf_codec() -> store::ArtifactCodec {
    let mut codec = store::ArtifactCodec::of::<
        crate::artifacts::pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot,
        crate::artifacts::pdf::standards::v1_4::subsets::base::schema::mutations::PdfMutation,
    >(crate::artifacts::pdf::STDIO_PDF_DOCUMENT_SCHEMA);
    codec.extension = "pdf";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("../🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio"));
    codec
}

#[cfg(feature = "full-artifact-catalog")]
fn native_codec_factories() -> [NativeCodecFactory; 26] {
    [
        NativeCodecFactory { id: "stdio.native.ply.v1", artifact: "ply", kind: crate::artifacts::ply::artifact_kind, codec: ply_codec },
        NativeCodecFactory { id: "stdio.native.stl.v1", artifact: "stl", kind: crate::artifacts::stl::artifact_kind, codec: stl_codec },
        NativeCodecFactory { id: "stdio.native.las.v1", artifact: "las", kind: crate::artifacts::las::artifact_kind, codec: las_codec },
        NativeCodecFactory { id: "stdio.native.dxf.v1", artifact: "dxf", kind: crate::artifacts::dxf::artifact_kind, codec: dxf_codec },
        NativeCodecFactory { id: "stdio.native.mp3.v1", artifact: "mp3", kind: crate::artifacts::mp3::artifact_kind, codec: mp3_codec },
        NativeCodecFactory { id: "stdio.native.xlsx.v1", artifact: "xlsx", kind: crate::artifacts::xlsx::artifact_kind, codec: xlsx_codec },
        NativeCodecFactory { id: "stdio.native.tiff.v1", artifact: "tiff", kind: crate::artifacts::tiff::artifact_kind, codec: tiff_codec },
        NativeCodecFactory { id: "stdio.native.jpg.v1", artifact: "jpg", kind: crate::artifacts::jpg::artifact_kind, codec: jpg_codec },
        NativeCodecFactory { id: "stdio.native.avi.v1", artifact: "avi", kind: crate::artifacts::avi::artifact_kind, codec: avi_codec },
        NativeCodecFactory { id: "stdio.native.png.v1", artifact: "png", kind: crate::artifacts::png::artifact_kind, codec: png_codec },
        NativeCodecFactory { id: "stdio.native.csv.v1", artifact: "csv", kind: crate::artifacts::csv::artifact_kind, codec: csv_codec },
        NativeCodecFactory { id: "stdio.native.md.v1", artifact: "md", kind: crate::artifacts::md::artifact_kind, codec: md_codec },
        NativeCodecFactory { id: "stdio.native.docx.v1", artifact: "docx", kind: crate::artifacts::docx::artifact_kind, codec: docx_codec },
        NativeCodecFactory { id: "stdio.native.mp4.v1", artifact: "mp4", kind: crate::artifacts::mp4::artifact_kind, codec: mp4_codec },
        NativeCodecFactory { id: "stdio.native.json.v1", artifact: "json", kind: crate::artifacts::json::artifact_kind, codec: json_codec },
        NativeCodecFactory { id: "stdio.native.gltf.v1", artifact: "gltf", kind: crate::artifacts::gltf::artifact_kind, codec: gltf_codec },
        NativeCodecFactory { id: "stdio.native.bcf.v1", artifact: "bcf", kind: crate::artifacts::bcf::artifact_kind, codec: bcf_codec },
        NativeCodecFactory { id: "stdio.native.zip.v1", artifact: "zip", kind: crate::artifacts::zip::artifact_kind, codec: zip_codec },
        NativeCodecFactory { id: "stdio.native.xml.v1", artifact: "xml", kind: crate::artifacts::xml::artifact_kind, codec: xml_codec },
        NativeCodecFactory { id: "stdio.native.deflate.v1", artifact: "deflate", kind: crate::artifacts::deflate::artifact_kind, codec: deflate_codec },
        NativeCodecFactory { id: "stdio.native.obj.v1", artifact: "obj", kind: crate::artifacts::obj::artifact_kind, codec: obj_codec },
        NativeCodecFactory { id: "stdio.native.pdf.v1", artifact: "pdf", kind: crate::artifacts::pdf::artifact_kind, codec: pdf_codec },
        NativeCodecFactory { id: "stdio.native.pptx.v1", artifact: "pptx", kind: crate::artifacts::pptx::artifact_kind, codec: pptx_codec },
        NativeCodecFactory { id: "stdio.native.step.v1", artifact: "step", kind: crate::artifacts::step::artifact_kind, codec: step_codec },
        NativeCodecFactory { id: "stdio.native.dwg.v1", artifact: "dwg", kind: crate::artifacts::dwg::artifact_kind, codec: dwg_codec },
        NativeCodecFactory { id: "stdio.native.svg.v1", artifact: "svg", kind: crate::artifacts::svg::artifact_kind, codec: svg_codec },
    ]
}

#[cfg(all(feature = "home-io", not(feature = "full-artifact-catalog")))]
fn native_codec_factories() -> [NativeCodecFactory; 6] {
    [
        NativeCodecFactory { id: "stdio.native.xlsx.v1", artifact: "xlsx", kind: crate::artifacts::xlsx::artifact_kind, codec: xlsx_codec },
        NativeCodecFactory { id: "stdio.native.csv.v1", artifact: "csv", kind: crate::artifacts::csv::artifact_kind, codec: csv_codec },
        NativeCodecFactory { id: "stdio.native.json.v1", artifact: "json", kind: crate::artifacts::json::artifact_kind, codec: json_codec },
        NativeCodecFactory { id: "stdio.native.zip.v1", artifact: "zip", kind: crate::artifacts::zip::artifact_kind, codec: zip_codec },
        NativeCodecFactory { id: "stdio.native.xml.v1", artifact: "xml", kind: crate::artifacts::xml::artifact_kind, codec: xml_codec },
        NativeCodecFactory { id: "stdio.native.deflate.v1", artifact: "deflate", kind: crate::artifacts::deflate::artifact_kind, codec: deflate_codec },
    ]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via equality checks) — see R9
fn native_codec_hash(value: &str) -> Result<[u8; 32], PluginAssemblyError> {
    if value.len() != 64 {
        return Err(failure("native codec pack schema hash must contain exactly 64 lowercase hexadecimal digits"));
    }
    let mut hash = [0u8; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let digit = |byte| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        let high = digit(chunk[0]).ok_or_else(|| failure("native codec pack schema hash must use lowercase hexadecimal"))?;
        let low = digit(chunk[1]).ok_or_else(|| failure("native codec pack schema hash must use lowercase hexadecimal"))?;
        hash[index] = high * 16 + low;
    }
    if hash == [0; 32] {
        return Err(failure("native codec pack schema hash must be nonzero"));
    }
    Ok(hash)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via schema validation) — see R9
fn validate_native_codec_binding(source: &Source, item: &Codec, binding: &NativeCodecBinding) -> Result<(), PluginAssemblyError> {
    let factory = native_codec_factories()
        .into_iter()
        .find(|factory| factory.id == binding.factory_id)
        .ok_or_else(|| failure(format!("codec {} names unknown native factory {}", item.id, binding.factory_id)))?;
    let kind = (factory.kind)();
    let hash = native_codec_hash(&binding.pack_schema_hash)?;
    let runtime = source
        .runtime_capabilities
        .iter()
        .find(|capability| capability.id == binding.runtime_capability_id)
        .ok_or_else(|| failure(format!("codec {} names missing runtime capability {}", item.id, binding.runtime_capability_id)))?;
    let extension_claim = ArtifactIdentityClaim::codec_extension(&binding.document_schema, &binding.extension).map_err(PluginAssemblyError::definition)?;
    let expected_claims = BTreeSet::from([("codec".to_owned(), binding.document_schema.clone()), (extension_claim.namespace().as_str().to_owned(), extension_claim.value().to_owned())]);
    let codec = (factory.codec)();
    if factory.artifact != source.artifact
        || kind.id != binding.artifact_kind
        || runtime.category != "codec"
        || runtime_claims(runtime) != expected_claims
        || codec.schema != binding.document_schema
        || codec.extension != binding.extension
        || codec.pack_schema_hash != hash
    {
        return Err(failure(format!("codec {} native factory binding does not exactly match its artifact kind, runtime capability, schema, extension, and pack hash", item.id)));
    }
    Ok(())
}

/// 🧬 Lists only explicit native runtime roots; definition-only artifacts are unrepresentable here.
pub fn native_codec_artifact_kinds() -> Vec<semio_framework_plugin::ArtifactKindSpec> {
    native_codec_factories().into_iter().map(|factory| (factory.kind)()).collect()
}

#[cfg(feature = "full-artifact-catalog")]
fn validate_native_openable_projection(receipts: &[NativeCodecFactoryReceipt]) -> Result<(), PluginAssemblyError> {
    let provider: NativeOpenableProviderSourceV1 = pack::from_json_str(include_str!("🧬️schema/📜️native-codec-factories.json"))
        .map_err(|error| failure(format!("cannot parse native codec receipt projection: {error}")))?;
    if provider.schema != "semio.stdio.native-openable-catalog-provider/v1"
        || provider.provider_id != "stdio/native-codecs/v1"
        || provider.plugin_id != "stdio"
        || provider.package_id != "semio:stdio"
        || provider.receipts.len() != receipts.len()
    {
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

/// 🧷 Emits receipts only when schema data explicitly authorizes the exact native factory.
#[cfg(feature = "full-artifact-catalog")]
pub fn native_codec_factory_receipts() -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
    let values = sources()?;
    validate_catalog(&values)?;
    let executable_codecs = values.iter().flat_map(|source| source.codecs.iter()).filter(|codec| codec.executable_registration).count();
    if executable_codecs != 26 {
        return Err(failure(format!("withholding native codec factories: artifact owners authorize {executable_codecs} executable registrations, expected 26")));
    }
    let package_id = crate::plugin::component_package_id()?.to_owned();
    if package_id != "semio:stdio" {
        return Err(failure("native codec receipt package differs from semio:stdio"));
    }
    let runtime_artifacts = artifact_assemblies()?
        .into_iter()
        .filter_map(|assembly| match assembly {
            ArtifactAssembly::Runtime(declaration) => declaration.definition().identity().as_str().strip_prefix("s.stdio.").map(str::to_owned),
            ArtifactAssembly::Definition(_) => None,
        })
        .collect::<BTreeSet<_>>();
    let descriptor = crate::plugin()?.manifest;
    let descriptor_kinds = descriptor.artifact_kinds.into_iter().map(|kind| (kind.id.clone(), kind)).collect::<BTreeMap<_, _>>();
    let mut factory_ids = BTreeSet::new();
    let mut descriptor_codec_ids = BTreeSet::new();
    let mut receipt_keys = BTreeSet::new();
    let mut receipts = Vec::with_capacity(executable_codecs);
    for source in &values {
        for item in source.codecs.iter().filter(|codec| codec.executable_registration) {
            let binding = item.native_factory.as_ref().ok_or_else(|| failure(format!("executable codec {} omits its native factory binding", item.id)))?;
            let factory = native_codec_factories()
                .into_iter()
                .find(|factory| factory.id == binding.factory_id)
                .ok_or_else(|| failure(format!("executable codec {} names unknown native factory {}", item.id, binding.factory_id)))?;
            if !runtime_artifacts.contains(factory.artifact)
                || !factory_ids.insert(factory.id.to_owned())
                || !descriptor_codec_ids.insert(item.id.clone())
                || !receipt_keys.insert((binding.artifact_kind.clone(), binding.document_schema.clone()))
            {
                return Err(failure(format!("executable codec {} is not bijective with one runtime artifact and private factory", item.id)));
            }
            let kind = (factory.kind)();
            if descriptor_kinds.get(&binding.artifact_kind) != Some(&kind) {
                return Err(failure(format!("descriptor artifact kind {} differs from executable codec {}", binding.artifact_kind, item.id)));
            }
            let receipt = NativeCodecFactoryReceipt {
                plugin_id: "stdio",
                package_id: package_id.clone(),
                package_version: env!("CARGO_PKG_VERSION"),
                factory_id: binding.factory_id.clone(),
                descriptor_codec_id: item.id.clone(),
                runtime_capability_id: binding.runtime_capability_id.clone(),
                artifact_kind: binding.artifact_kind.clone(),
                schema: binding.document_schema.clone(),
                pack_schema_hash: native_codec_hash(&binding.pack_schema_hash)?,
                extension: binding.extension.clone(),
                factory: factory.codec,
            };
            receipt.instantiate()?;
            receipts.push(receipt);
        }
    }
    if receipts.len() != 26
        || factory_ids.len() != 26
        || descriptor_codec_ids.len() != 26
        || receipt_keys.len() != 26
        || native_codec_factories().into_iter().any(|factory| !factory_ids.contains(factory.id))
    {
        return Err(failure("native codec receipt manifest, descriptor identities, runtime artifacts, and private factories are not a complete bijection"));
    }
    validate_native_openable_projection(&receipts)?;
    Ok(receipts)
}
//#endregion NativeCodecFactoryReceipts

#[cfg(test)]
mod tests {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn runtime_key(category: &str, claims: impl IntoIterator<Item = (String, String)>) -> String {
        format!("{category}|{}", claims.into_iter().map(|(namespace, value)| format!("{namespace}:{value}")).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>().join("|"))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn schema_runtime_keys(source: &Source) -> BTreeSet<String> {
        source.runtime_capabilities.iter().map(|capability| runtime_key(&capability.category, runtime_claims(capability))).collect()
    }

    /// ⏳️ `ArtifactRuntimeCapabilityRequirement::kind`/`claims` are declared `async` by the plugin
    /// SDK, so this reader suspends once per requirement and cannot be a `🚫️async` helper.
    async fn declaration_runtime_keys(declaration: &ArtifactDeclaration) -> BTreeSet<String> {
        let mut keys = BTreeSet::new();
        for requirement in declaration.runtime_capability_requirements().unwrap() {
            let kind = requirement.kind().await.as_str();
            let claims = requirement.claims().await.iter().map(|claim| (claim.namespace().as_str().to_owned(), claim.value().to_owned()));
            keys.insert(runtime_key(kind, claims));
        }
        keys
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_runtime_capabilities_exactly_match_registered_declarations() {
        let factories = artifact_factories();
        for source in sources().unwrap() {
            let assembly = factories.get(source.artifact.as_str()).expect("schema artifact factory")(build(&source).expect("schema definition")).expect("runtime parity");
            match assembly {
                ArtifactAssembly::Runtime(declaration) => assert_eq!(schema_runtime_keys(&source), declaration_runtime_keys(&declaration).await, "runtime capability rows diverge for {}", source.artifact),
                ArtifactAssembly::Definition(_) => assert!(source.runtime_capabilities.is_empty(), "definition-only {} declares unregistered runtime capabilities", source.artifact),
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn gltf_capability_ledger_is_honest() {
        let ledger = capability_ledger().unwrap();
        assert_eq!(ledger.declared, CapabilityCounts { codecs: 32, mutations: 3, inferences: 67 });
        assert_eq!(ledger.registered, CapabilityCounts { codecs: 26, mutations: 3, inferences: 67 });
        assert_eq!(ledger.implemented, CapabilityCounts { codecs: 26, mutations: 0, inferences: 0 });
        assert_eq!(ledger.verified, CapabilityCounts::default());
    }

    #[semio_framework_async_macros::async_test]
    async fn gltf_executable_identity_keys_are_exactly_the_registered_services() {
        let source = sources().unwrap().into_iter().find(|source| source.artifact == "gltf").expect("gltf source");
        let mappings = executable_mappings(&source).expect("gltf executable mappings");
        assert_eq!(mappings.keys().cloned().collect::<BTreeSet<_>>(), expected_executable_ids(&source));
        assert_eq!(mappings.len(), 71);
        let definition = build(&source).expect("gltf definition");
        assert_eq!(
            definition
                .capabilities()
                .filter(|capability| { matches!(capability.kind().as_str(), "codec" | "inference" | "mutation") && capability.executable_identity().is_some() })
                .map(|capability| capability.identity().as_str().to_owned())
                .collect::<BTreeSet<_>>(),
            expected_executable_ids(&source)
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn schema_keys_and_runtime_factories_are_exact() {
        let sources = sources().unwrap();
        assert_eq!(artifact_factories().keys().copied().collect::<BTreeSet<_>>(), sources.iter().map(|source| source.artifact.as_str()).collect());
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_owned_native_codec_receipts_form_one_complete_static_bijection() {
        assert_eq!(artifact_assemblies().unwrap().into_iter().filter(|assembly| matches!(assembly, ArtifactAssembly::Runtime(_))).count(), 26);
        let declared_kinds = native_codec_artifact_kinds().into_iter().map(|kind| kind.id).collect::<BTreeSet<_>>();
        assert_eq!(declared_kinds.len(), 26);
        assert!(!declared_kinds.contains("stdio.binary"));
        assert_eq!(native_codec_factories().into_iter().map(|factory| factory.id).collect::<BTreeSet<_>>().len(), 26);
        let receipts = native_codec_factory_receipts().expect("artifact-owned native codec receipts");
        assert_eq!(receipts.len(), 26);
        assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
        assert!(receipts.iter().all(|receipt| receipt.pack_schema_hash != [0; 32] && receipt.instantiate().is_ok()));
    }

    #[semio_framework_async_macros::async_test]
    async fn gltf_runtime_codec_capability_is_authorized_only_by_its_exact_artifact_owned_receipt() {
        let source = sources().unwrap().into_iter().find(|source| source.artifact == "gltf").expect("gltf source");
        let declared = source.codecs.iter().find(|codec| codec.executable_registration).expect("artifact-owned executable codec");
        let binding = declared.native_factory.as_ref().expect("exact native factory binding");
        let runtime = source.runtime_capabilities.iter().find(|capability| capability.id == "s.stdio.gltf.standard.2-0.codec.codec-stdio-gltf-extension-gltf.v1").expect("gltf runtime codec capability");
        let codec = gltf_codec();
        assert_eq!(codec.schema, "stdio.gltf");
        assert_eq!(codec.extension, "gltf");
        assert_ne!(codec.pack_schema_hash, [0; 32]);
        let extension_claim = ArtifactIdentityClaim::codec_extension(&codec.schema, codec.extension).unwrap();
        assert_eq!(runtime_claims(runtime), BTreeSet::from([("codec".to_owned(), codec.schema.clone()), (extension_claim.namespace().as_str().to_owned(), extension_claim.value().to_owned())]));
        assert_eq!(binding.factory_id, "stdio.native.gltf.v1");
        assert_eq!(native_codec_hash(&binding.pack_schema_hash).expect("pack schema hash"), codec.pack_schema_hash);
    }

    #[semio_framework_async_macros::async_test]
    async fn executable_codec_without_a_complete_native_factory_binding_is_rejected() {
        let mut source = sources().unwrap().into_iter().find(|source| source.artifact == "gltf").expect("gltf source");
        source.codecs[0].status = "implemented".into();
        source.codecs[0].executable_registration = true;
        let error = validate(&source).expect_err("executable codec without exact factory binding must fail closed");
        assert!(error.to_string().contains("must bind an exact native factory"));
    }

    #[semio_framework_async_macros::async_test]
    async fn gltf_representation_capability_has_exact_format_claims() {
        let definition = artifact_definitions().unwrap().into_iter().find(|definition| definition.identity().as_str() == "s.stdio.gltf").expect("gltf definition");
        let capability = definition.capabilities().find(|capability| capability.identity().as_str() == "s.stdio.gltf.standard.2-0.representation.mime-model-gltf-json-extension-gltf").expect("gltf runtime representation");
        assert_eq!(capability.kind().as_str(), "representation");
        assert_eq!(capability.claims().iter().map(|claim| (claim.namespace().as_str(), claim.value())).collect::<Vec<_>>(), vec![("extension", ".gltf"), ("mime", "model/gltf+json")]);
        let formats = format_descriptors_for("gltf").unwrap();
        assert_eq!(formats.len(), 1);
        assert_eq!(formats[0].kind_id, "s.stdio.gltf.standard.2-0.representation.document");
        assert_eq!(formats[0].mimes, ["model/gltf+json"]);
        assert_eq!(formats[0].extensions, [".gltf"]);
        assert_eq!(artifact_assemblies().unwrap().len(), 36);
        assert!(crate::plugin().is_ok());
    }
}
//#endregion Assembly
