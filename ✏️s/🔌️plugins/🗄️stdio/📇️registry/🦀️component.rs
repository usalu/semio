//! 🧾️ Schema-owned stdio artifact-definition assembly.

use semio_framework_plugin::{
    ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, ArtifactExecutableIdentity,
    ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, PluginAssemblyError,
};
use semio_framework_plugin::io::FormatDescriptor;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

//#region SourceSchema
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Standard {
    id: String, revision: String, normative_source: Option<String>, publication_date: Option<String>,
    source_checksum: Option<String>, redistribution_status: String, clauses_or_features: Vec<String>, status: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Profile { id: String, standard: String, profile: String, status: String }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Dialect { id: String, standard: String, dialect: String, registered_code_points: Vec<String>, status: String }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Representation {
    id: String, standard: String, representation: String, mimes: Vec<String>, extensions: Vec<String>,
    is_binary: bool, aliases: Vec<String>, neutral: bool, status: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Codec { id: String, status: String, from: String, to: String, executable_registration: bool }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ExecutableLeaf { id: String, status: String, executable_registration: bool }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Resource { id: String, external_reference_policy: String, status: String }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Localized { id: String, locale: String, name: String, description: String, status: String }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Conformance { id: String, status: String, fixtures: Vec<String> }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RuntimeCapability { id: String, category: String, descriptor: String, claims: Vec<RuntimeClaim> }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RuntimeClaim { namespace: String, value: String }

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Ledger {
    normative_source: Option<String>, publication_date: Option<String>, source_checksum: Option<String>,
    redistribution_status: String, clauses_or_features: Vec<String>, profiles: Vec<String>,
    registered_code_points: Vec<String>, read: String, write: String, lossless: String, canonical: String,
    validators: Vec<String>, mutations: Vec<String>, inferences: Vec<String>, fixtures: Vec<String>,
}
//#endregion SourceSchema

//#region CapabilityLedger
/// 📊️ Category counts never conflate declaration, registration, implementation, or verification.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapabilityCounts { pub codecs: usize, pub mutations: usize, pub inferences: usize }

/// 📒️ Honest capability status ledger derived from schema leaves.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapabilityLedger { pub declared: CapabilityCounts, pub registered: CapabilityCounts, pub implemented: CapabilityCounts, pub verified: CapabilityCounts }

fn capability_counts<T>(items: &[T], status: impl Fn(&T) -> &str, registered: impl Fn(&T) -> bool) -> (usize, usize, usize, usize) {
    (items.len(), items.iter().filter(|item| registered(item)).count(), items.iter().filter(|item| status(item) == "implemented").count(), items.iter().filter(|item| status(item) == "verified").count())
}

/// 📊️ Returns separate schema declaration, runtime registration, implementation, and verification counts.
pub fn capability_ledger() -> Result<CapabilityLedger, PluginAssemblyError> {
    let values = sources()?;
    validate_catalog(&values)?;
    let mut ledger = CapabilityLedger::default();
    for source in &values {
        let (declared, registered, implemented, verified) = capability_counts(&source.codecs, |item| &item.status, |item| item.executable_registration);
        ledger.declared.codecs += declared; ledger.registered.codecs += registered; ledger.implemented.codecs += implemented; ledger.verified.codecs += verified;
        let (declared, registered, implemented, verified) = capability_counts(&source.mutations, |item| &item.status, |item| item.executable_registration);
        ledger.declared.mutations += declared; ledger.registered.mutations += registered; ledger.implemented.mutations += implemented; ledger.verified.mutations += verified;
        let (declared, registered, implemented, verified) = capability_counts(&source.inferences, |item| &item.status, |item| item.executable_registration);
        ledger.declared.inferences += declared; ledger.registered.inferences += registered; ledger.implemented.inferences += implemented; ledger.verified.inferences += verified;
    }
    Ok(ledger)
}
//#endregion CapabilityLedger

//#region SourceLoading
const SOURCES: [&str; 36] = [
    include_str!("../🗿️artifacts/💾️binary/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📄txt/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📰xml/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🗜️deflate/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎒️zip/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🔣️json/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📊️csv/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📝️md/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧊️gltf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🧊️obj/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🟪️stl/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/☁️ply/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/☁️las/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📐️step/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🏗️ifc/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖊️dwg/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖊️dxf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎨️svg/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📷️png/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📷️jpg/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎞️gif/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖼️bmp/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🖼️tiff/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📄️pdf/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/📜️docx/🧬️schema/📜️artifact-definition.json"),
    include_str!("../🗿️artifacts/🎞️pptx/🧬️schema/📜️artifact-definition.json"),
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

fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.definition", message)
}

fn descriptor<T: Serialize>(value: &T) -> Result<Vec<u8>, PluginAssemblyError> {
    serde_json::to_vec(value).map_err(|error| failure(format!("cannot serialize definition descriptor: {error}")))
}

fn sources() -> Result<Vec<Source>, PluginAssemblyError> {
    SOURCES.into_iter().map(|value| serde_json::from_str(value).map_err(|error| failure(format!("cannot parse artifact definition: {error}")))).collect()
}
//#endregion SourceLoading

//#region Validation
fn id(value: &str) -> Result<(), PluginAssemblyError> {
    ArtifactIdentity::parse(value).map(|_| ()).map_err(PluginAssemblyError::definition)
}

fn child<'a>(id: &'a str, owner: &str, namespace: &str) -> Result<&'a str, PluginAssemblyError> {
    id.strip_prefix(&format!("{owner}.{namespace}.")).filter(|value| !value.contains('.')).ok_or_else(|| failure(format!("{id:?} is not a direct {namespace} leaf of {owner}")))
}

fn versioned_leaf(id: &str, prefix: &str) -> Result<(), PluginAssemblyError> {
    let leaf = id.strip_prefix(prefix).ok_or_else(|| failure(format!("{id:?} is not owned by {prefix:?}")))?;
    let (semantic, version) = leaf.rsplit_once(".v").ok_or_else(|| failure(format!("{id:?} must end in a canonical vN leaf")))?;
    if semantic.is_empty() || semantic.contains('.') || version.is_empty() || !version.bytes().all(|byte| byte.is_ascii_digit()) || version.starts_with('0') { return Err(failure(format!("{id:?} must end in a canonical vN leaf"))); }
    ArtifactIdentity::parse(id).map(|_| ()).map_err(PluginAssemblyError::definition)
}

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

fn representation_claims(item: &Representation) -> BTreeSet<(String, String)> {
    item.mimes.iter().map(|value| ("mime".into(), value.clone())).chain(item.extensions.iter().map(|value| ("extension".into(), value.clone()))).collect()
}

fn runtime_claims(item: &RuntimeCapability) -> BTreeSet<(String, String)> {
    item.claims.iter().map(|claim| (claim.namespace.clone(), claim.value.clone())).collect()
}

type CapabilityExecutable = fn();

/// 🧷️ Maps exactly the schema leaves that declare a native executable.
fn executable_mappings(_source: &Source) -> BTreeMap<String, CapabilityExecutable> { BTreeMap::new() }

fn executable_identity(source: &Source, id: &str) -> Result<Option<ArtifactExecutableIdentity>, PluginAssemblyError> {
    let mappings = executable_mappings(source);
    let expected = source.codecs.iter().map(|item| (&item.id, item.executable_registration)).chain(source.mutations.iter().map(|item| (&item.id, item.executable_registration))).chain(source.inferences.iter().map(|item| (&item.id, item.executable_registration))).filter(|(_, registered)| *registered).map(|(id, _)| id.clone()).collect::<BTreeSet<_>>();
    if mappings.keys().cloned().collect::<BTreeSet<_>>() != expected { return Err(failure(format!("{} executable mapping keys diverge from schema registrations", source.id))); }
    Ok(mappings.get(id).copied().map(ArtifactExecutableIdentity::from_function))
}

fn same(label: &str, left: impl IntoIterator<Item = String>, right: impl IntoIterator<Item = String>) -> Result<(), PluginAssemblyError> {
    if left.into_iter().collect::<BTreeSet<_>>() != right.into_iter().collect::<BTreeSet<_>>() {
        return Err(failure(format!("{label} diverges from its schema collection")));
    }
    Ok(())
}

fn validate(source: &Source) -> Result<(), PluginAssemblyError> {
    let owner = format!("s.stdio.{}", source.artifact);
    if source.definition_version != 1 || source.id != owner { return Err(failure(format!("{owner} must use definition_version 1"))); }
    if source.standards.is_empty() || source.profiles.is_empty() || source.source_dialects.is_empty() || source.representations.is_empty() || source.resources.is_empty() || source.localized_descriptors.len() != 2 || source.conformance_suites.is_empty() { return Err(failure(format!("{owner} omits a required collection"))); }
    id(&source.id)?;
    let standards = source.standards.iter().map(|item| item.id.clone()).collect::<BTreeSet<_>>();
    for item in &source.standards {
        if item.id != format!("{owner}.standard.{}", item.revision) { return Err(failure(format!("invalid standard {}", item.id))); }
        if item.status == "unverified" && (item.normative_source.is_some() || item.publication_date.is_some() || item.source_checksum.is_some() || item.redistribution_status != "unknown" || !item.clauses_or_features.is_empty()) { return Err(failure(format!("unverified standard {} carries unverifiable provenance", item.id))); }
        if item.status == "verified" && (item.normative_source.is_none() || item.publication_date.is_none() || item.source_checksum.is_none() || item.redistribution_status == "unknown" || item.clauses_or_features.is_empty()) { return Err(failure(format!("verified standard {} lacks provenance", item.id))); }
        if !matches!(item.status.as_str(), "unverified" | "verified") { return Err(failure(format!("invalid standard status {}", item.status))); }
        id(&item.id)?;
    }
    for item in &source.profiles { if !standards.contains(&item.standard) || item.id != format!("{}.profile.{}", item.standard, item.profile) || !matches!(item.status.as_str(), "unimplemented" | "opaque" | "implemented") { return Err(failure(format!("invalid profile {}", item.id))); } id(&item.id)?; }
    for item in &source.source_dialects { if !standards.contains(&item.standard) || item.id != format!("{}.dialect.{}", item.standard, item.dialect) || !matches!(item.status.as_str(), "unimplemented" | "opaque" | "implemented") { return Err(failure(format!("invalid source dialect {}", item.id))); } id(&item.id)?; }
    for item in &source.representations {
        if !standards.contains(&item.standard) || item.id != format!("{}.representation.{}", item.standard, item.representation) || item.extensions.is_empty() || item.extensions.iter().any(|extension| !extension.starts_with('.')) || item.status != "declared" { return Err(failure(format!("invalid representation {}", item.id))); }
        if item.mimes.iter().collect::<BTreeSet<_>>().len() != item.mimes.len() || item.extensions.iter().collect::<BTreeSet<_>>().len() != item.extensions.len() { return Err(failure(format!("duplicate representation claim {}", item.id))); }
        id(&item.id)?;
    }
    if standards != source.representations.iter().map(|item| item.standard.clone()).collect::<BTreeSet<_>>() { return Err(failure(format!("{owner} must give every declared standard its own representation"))); }
    if source.artifact == "epw" && source.representations.iter().any(|item| !item.mimes.is_empty()) { return Err(failure("EPW must remain MIME-unregistered")); }
    let locales = source.localized_descriptors.iter().map(|item| item.locale.as_str()).collect::<BTreeSet<_>>();
    if locales != BTreeSet::from(["de", "en"]) { return Err(failure(format!("{owner} must own English and German descriptors"))); }
    for item in &source.localized_descriptors { if item.id != format!("{owner}.localization.{}", item.locale) || item.name.is_empty() || item.description.is_empty() { return Err(failure(format!("invalid localization {}", item.id))); } id(&item.id)?; }
    for item in &source.resources { if item.status != "unimplemented" || item.external_reference_policy.is_empty() { return Err(failure(format!("invalid resource {}", item.id))); } child(&item.id, &owner, "resource")?; id(&item.id)?; }
    for item in &source.conformance_suites { if item.status != "unimplemented" { return Err(failure(format!("invalid conformance suite {}", item.id))); } child(&item.id, &owner, "conformance-suite")?; id(&item.id)?; for fixture in &item.fixtures { id(fixture)?; } }
    for item in &source.codecs {
        let standard = source.standards.iter().find(|standard| item.id.starts_with(&format!("{}.codec.", standard.id))).ok_or_else(|| failure(format!("invalid codec {}", item.id)))?;
        versioned_leaf(&item.id, &format!("{}.codec.", standard.id))?;
        if item.from.is_empty() || item.to.is_empty() || !matches!(item.status.as_str(), "unimplemented" | "implemented" | "verified") || item.executable_registration != matches!(item.status.as_str(), "implemented" | "verified") { return Err(failure(format!("invalid codec {}", item.id))); }
    }
    for (category, item) in source.mutations.iter().map(|item| ("mutation", item)).chain(source.inferences.iter().map(|item| ("inference", item))) {
        versioned_leaf(&item.id, &format!("{owner}.{category}."))?;
        if source.artifact == "gltf" && (item.id.contains(".no-mutation.") || item.id.contains(".set-snapshot.") || item.id.contains(".set-")) { return Err(failure(format!("GLTF capability {} is not a specific semantic command", item.id))); }
        if !matches!(item.status.as_str(), "unimplemented" | "implemented" | "verified") || item.executable_registration != matches!(item.status.as_str(), "implemented" | "verified") { return Err(failure(format!("invalid {category} {}", item.id))); }
    }
    for item in source.codecs.iter().map(|item| &item.id).chain(source.mutations.iter().map(|item| &item.id)).chain(source.inferences.iter().map(|item| &item.id)) { executable_identity(source, item)?; }
    let mut runtime_ids = BTreeSet::new(); let mut runtime_claim_sets = BTreeSet::new();
    for item in &source.runtime_capabilities {
        let prefix = format!("{owner}.runtime.{}.", item.category);
        leaf_kind(&item.category)?;
        versioned_leaf(&item.id, &prefix)?;
        if item.descriptor.trim().is_empty() || item.claims.is_empty() || !runtime_ids.insert(item.id.clone()) { return Err(failure(format!("invalid runtime capability {}", item.id))); }
        let claims = runtime_claims(item);
        if claims.len() != item.claims.len() || !item.claims.iter().all(|claim| matches!(claim.namespace.as_str(), "schema" | "codec" | "extension" | "mime" | "dialect" | "grammar") && !claim.value.trim().is_empty()) || !runtime_claim_sets.insert((item.category.clone(), claims.clone())) { return Err(failure(format!("invalid runtime capability claims for {}", item.id))); }
        if item.category == "representation" && !source.representations.iter().any(|representation| representation_claims(representation) == claims) { return Err(failure(format!("runtime representation {} does not claim a representation leaf", item.id))); }
    }
    let ledger = &source.support_ledger;
    let states = [&ledger.read, &ledger.write, &ledger.lossless, &ledger.canonical];
    if !states.into_iter().all(|state| matches!(state.as_str(), "unimplemented" | "opaque" | "implemented")) { return Err(failure(format!("{owner} has an invalid support state"))); }
    if states.into_iter().any(|state| state == "implemented") && (ledger.normative_source.is_none() || ledger.publication_date.is_none() || ledger.source_checksum.is_none() || ledger.redistribution_status == "unknown" || ledger.clauses_or_features.is_empty() || ledger.validators.is_empty() || ledger.fixtures.is_empty()) { return Err(failure(format!("{owner} claims implementation without normative, validator, and fixture evidence"))); }
    same("ledger profiles", ledger.profiles.clone(), source.profiles.iter().map(|item| item.id.clone()))?;
    same("ledger code points", ledger.registered_code_points.clone(), source.source_dialects.iter().flat_map(|item| item.registered_code_points.clone()))?;
    same("ledger mutations", ledger.mutations.clone(), source.mutations.iter().map(|item| item.id.clone()))?;
    same("ledger inferences", ledger.inferences.clone(), source.inferences.iter().map(|item| item.id.clone()))?;
    same("ledger fixtures", ledger.fixtures.clone(), source.conformance_suites.iter().flat_map(|item| item.fixtures.clone()))?;
    let local = source.profiles.iter().map(|item| item.id.clone()).chain(source.resources.iter().map(|item| item.id.clone())).chain(source.codecs.iter().map(|item| item.id.clone())).chain(source.mutations.iter().map(|item| item.id.clone())).chain(source.inferences.iter().map(|item| item.id.clone())).chain(source.conformance_suites.iter().flat_map(|item| std::iter::once(item.id.clone()).chain(item.fixtures.clone()))).collect::<BTreeSet<_>>();
    for reference in ledger.validators.iter().chain(&ledger.mutations).chain(&ledger.inferences).chain(&ledger.fixtures) { if !local.contains(reference) { return Err(failure(format!("{owner} ledger reference {reference:?} does not resolve locally"))); } }
    Ok(())
}

fn validate_catalog(values: &[Source]) -> Result<(), PluginAssemblyError> {
    if values.len() != 36 { return Err(failure(format!("expected 36 artifact definitions, got {}", values.len()))); }
    let mut identities = BTreeSet::new(); let mut directories = BTreeSet::new(); let mut mimes = BTreeMap::new(); let mut extensions = BTreeMap::new(); let mut dialects = BTreeSet::new(); let mut runtime_capabilities = BTreeSet::new();
    for source in values {
        validate(source)?;
        if !identities.insert(source.id.clone()) || !directories.insert(source.directory.clone()) { return Err(failure(format!("duplicate artifact {}", source.id))); }
        for representation in &source.representations {
            for extension in &representation.extensions { if let Some(existing) = extensions.insert(extension.clone(), source.id.clone()) { if existing != source.id { return Err(failure(format!("extension {extension} is claimed by both {existing} and {}", source.id))); } } }
            for mime in &representation.mimes { if let Some(existing) = mimes.insert(mime.clone(), source.id.clone()) { if existing != source.id { return Err(failure(format!("MIME {mime} is claimed by both {existing} and {}", source.id))); } } }
        }
        for capability in &source.runtime_capabilities { if !runtime_capabilities.insert(capability.id.clone()) { return Err(failure(format!("duplicate runtime capability {}", capability.id))); } }
        for dialect in &source.source_dialects { if !dialects.insert(dialect.id.clone()) { return Err(failure(format!("duplicate dialect {}", dialect.id))); } }
        for dependency in &source.dependencies { if dependency == &source.id || !values.iter().any(|candidate| candidate.id == *dependency) { return Err(failure(format!("{} has unresolved dependency {dependency}", source.id))); } }
    }
    if !values.iter().find(|source| source.artifact == "txt").is_some_and(|source| source.representations.iter().any(|item| item.mimes.iter().any(|mime| mime == "text/plain"))) { return Err(failure("TXT must own text/plain")); }
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
pub fn runtime_assembly(artifact: &str, definition: ArtifactDefinition, factory: fn(ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError>) -> Result<ArtifactAssembly, PluginAssemblyError> {
    if definition.identity().as_str() != format!("s.stdio.{artifact}") {
        return Err(failure(format!("runtime artifact {artifact} received definition {}", definition.identity())));
    }
    factory(definition).map(ArtifactAssembly::Runtime).map_err(PluginAssemblyError::definition)
}

/// 🧾️ Preserves a schema-only artifact without fabricating runtime capabilities.
pub fn definition_only_assembly(artifact: &str, definition: ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError> {
    if definition.identity().as_str() != format!("s.stdio.{artifact}") {
        return Err(failure(format!("definition-only artifact {artifact} received definition {}", definition.identity())));
    }
    Ok(ArtifactAssembly::Definition(definition))
}

fn declared_capability<T: Serialize>(source: &Source, id: &str, kind: ArtifactCapabilityKind, value: &T) -> Result<ArtifactCapability, PluginAssemblyError> {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?, kind).descriptor(descriptor(value)?).map_err(PluginAssemblyError::definition)?;
    if let Some(executable) = executable_identity(source, id)? { capability = capability.executable(executable); }
    Ok(capability)
}

fn runtime_capability(item: &RuntimeCapability) -> Result<ArtifactCapability, PluginAssemblyError> {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(&item.id).map_err(PluginAssemblyError::definition)?, leaf_kind(&item.category)?).descriptor(item.descriptor.as_bytes().to_vec()).map_err(PluginAssemblyError::definition)?;
    for claim in &item.claims {
        capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(&claim.namespace).map_err(PluginAssemblyError::definition)?, &claim.value).map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    }
    Ok(capability)
}

fn build(source: &Source) -> Result<ArtifactDefinition, PluginAssemblyError> {
    let mut definition = ArtifactDefinition::stdio(&source.artifact).map_err(PluginAssemblyError::definition)?;
    for item in &source.standards { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::standard(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.profiles { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::profile(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.source_dialects { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::source_dialect(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.representations { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::representation(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.codecs { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::codec(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.mutations { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::mutation(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.inferences { definition = definition.capability(declared_capability(source, &item.id, ArtifactCapabilityKind::inference(), item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.runtime_capabilities { definition = definition.capability(runtime_capability(item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.resources { definition = definition.resource(child(&item.id, &source.id, "resource")?, descriptor(item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.localized_descriptors { definition = definition.localization(ArtifactLocale::parse(&item.locale).map_err(PluginAssemblyError::definition)?, format!("{}\n{}", item.name, item.description), descriptor(item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.conformance_suites { definition = definition.conformance_suite(child(&item.id, &source.id, "conformance-suite")?, descriptor(item)?).map_err(PluginAssemblyError::definition)?; }
    Ok(definition)
}

/// 🧾️ Builds every schema-owned artifact definition in catalog order.
pub fn artifact_definitions() -> Result<Vec<ArtifactDefinition>, PluginAssemblyError> {
    let values = sources()?; validate_catalog(&values)?; values.iter().map(build).collect()
}

/// 🧭️ Assembles every artifact root in schema-catalog order.
fn artifact_factories() -> BTreeMap<&'static str, fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>> {
    BTreeMap::from([
        ("binary", crate::artifacts::binary::assembly as fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>), ("txt", crate::artifacts::txt::assembly), ("xml", crate::artifacts::xml::assembly), ("deflate", crate::artifacts::deflate::assembly), ("zip", crate::artifacts::zip::assembly), ("json", crate::artifacts::json::assembly), ("csv", crate::artifacts::csv::assembly), ("md", crate::artifacts::md::assembly), ("gltf", crate::artifacts::gltf::assembly), ("obj", crate::artifacts::obj::assembly), ("stl", crate::artifacts::stl::assembly), ("ply", crate::artifacts::ply::assembly), ("las", crate::artifacts::las::assembly), ("step", crate::artifacts::step::assembly), ("ifc", crate::artifacts::ifc::assembly), ("dwg", crate::artifacts::dwg::assembly), ("dxf", crate::artifacts::dxf::assembly), ("svg", crate::artifacts::svg::assembly), ("png", crate::artifacts::png::assembly), ("jpg", crate::artifacts::jpg::assembly), ("gif", crate::artifacts::gif::assembly), ("bmp", crate::artifacts::bmp::assembly), ("tiff", crate::artifacts::tiff::assembly), ("pdf", crate::artifacts::pdf::assembly), ("docx", crate::artifacts::docx::assembly), ("pptx", crate::artifacts::pptx::assembly), ("xlsx", crate::artifacts::xlsx::assembly), ("bcf", crate::artifacts::bcf::assembly), ("semio", crate::artifacts::semio::assembly), ("mp4", crate::artifacts::mp4::assembly), ("avi", crate::artifacts::avi::assembly), ("mp3", crate::artifacts::mp3::assembly), ("wav", crate::artifacts::wav::assembly), ("epw", crate::artifacts::epw::assembly), ("tsv", crate::artifacts::tsv::assembly), ("html", crate::artifacts::html::assembly),
    ])
}

/// 🧭️ Assembles every artifact root by its schema-owned artifact key.
pub fn artifact_assemblies() -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {
    let factories = artifact_factories();
    let values = sources()?; validate_catalog(&values)?;
    if factories.keys().copied().collect::<BTreeSet<_>>() != values.iter().map(|source| source.artifact.as_str()).collect() { return Err(failure("artifact factory keys diverge from schema artifacts")); }
    values.iter().map(|source| factories.get(source.artifact.as_str()).ok_or_else(|| failure(format!("missing factory for {}", source.artifact)))?(build(source)?)).collect()
}

fn source_format_descriptors(source: &Source) -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    source.runtime_capabilities.iter().filter(|capability| capability.category == "representation").filter_map(|capability| source.representations.iter().filter(|representation| representation_claims(representation) == runtime_claims(capability)).min_by(|left, right| left.id.cmp(&right.id)).map(|representation| (source, representation))).map(|(source, representation)| {
        let english = source.localized_descriptors.iter().find(|item| item.locale == "en").ok_or_else(|| failure(format!("{} has no English descriptor", source.id)))?;
        Ok(FormatDescriptor { kind_id: representation.id.clone(), short_id: representation.id.clone(), aliases: representation.aliases.clone(), mimes: representation.mimes.clone(), extensions: representation.extensions.clone(), name: english.name.clone(), full_name: english.description.clone(), neutral: representation.neutral, dir_name: source.directory.clone(), is_binary: representation.is_binary })
    }).collect()
}

/// 🗂️ Derives one runtime root's format descriptors from its exact representation capability records.
pub fn format_descriptors_for(artifact: &str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    let values = sources().and_then(|values| {
        validate_catalog(&values)?;
        let source = values.iter().find(|source| source.artifact == artifact).ok_or_else(|| failure(format!("unknown stdio artifact {artifact}")))?;
        source_format_descriptors(source)
    });
    values.map_err(|error| ArtifactDefinitionError::new("stdio.format", error.to_string()))
}

/// 🛂️ Derives every runtime format descriptor from schema-owned representations.
pub fn format_descriptors() -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    let values = sources()?; validate_catalog(&values)?;
    values.iter().map(source_format_descriptors).collect::<Result<Vec<_>, _>>().map(|groups| groups.into_iter().flatten().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runtime_key(category: &str, claims: impl IntoIterator<Item = (String, String)>) -> String {
        format!("{category}|{}", claims.into_iter().map(|(namespace, value)| format!("{namespace}:{value}")).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>().join("|"))
    }

    fn schema_runtime_keys(source: &Source) -> BTreeSet<String> {
        source.runtime_capabilities.iter().map(|capability| runtime_key(&capability.category, runtime_claims(capability))).collect()
    }

    fn declaration_runtime_keys(declaration: &ArtifactDeclaration) -> BTreeSet<String> {
        declaration.runtime_capability_requirements().unwrap().into_iter().map(|requirement| runtime_key(requirement.kind().as_str(), requirement.claims().iter().map(|claim| (claim.namespace().as_str().to_owned(), claim.value().to_owned())))).collect()
    }

    #[test]
    fn schema_runtime_capabilities_exactly_match_registered_declarations() {
        let factories = artifact_factories();
        for source in sources().unwrap() {
            let assembly = factories.get(source.artifact.as_str()).expect("schema artifact factory")(build(&source).expect("schema definition")).expect("runtime parity");
            match assembly {
                ArtifactAssembly::Runtime(declaration) => assert_eq!(schema_runtime_keys(&source), declaration_runtime_keys(&declaration), "runtime capability rows diverge for {}", source.artifact),
                ArtifactAssembly::Definition(_) => assert!(source.runtime_capabilities.is_empty(), "definition-only {} declares unregistered runtime capabilities", source.artifact),
            }
        }
    }

    #[test]
    fn gltf_capability_ledger_is_honest() {
        let ledger = capability_ledger().unwrap();
        assert_eq!(ledger.declared, CapabilityCounts { codecs: 6, mutations: 18, inferences: 15 });
        assert_eq!(ledger.registered, CapabilityCounts::default());
        assert_eq!(ledger.implemented, CapabilityCounts::default());
        assert_eq!(ledger.verified, CapabilityCounts::default());
    }

    #[test]
    fn schema_keys_and_runtime_factories_are_exact() {
        let sources = sources().unwrap();
        assert_eq!(artifact_factories().keys().copied().collect::<BTreeSet<_>>(), sources.iter().map(|source| source.artifact.as_str()).collect());
    }

    #[test]
    fn gltf_representation_capability_has_exact_format_claims() {
        let definition = artifact_definitions().unwrap().into_iter().find(|definition| definition.identity().as_str() == "s.stdio.gltf").expect("gltf definition");
        let capability = definition.capabilities().find(|capability| capability.identity().as_str() == "s.stdio.gltf.runtime.representation.mime-model-gltf-json-extension-gltf.v1").expect("gltf runtime representation");
        assert_eq!(capability.kind().as_str(), "representation");
        assert_eq!(capability.claims().iter().map(|claim| (claim.namespace().as_str(), claim.value())).collect::<Vec<_>>(), vec![("extension", ".gltf"), ("mime", "model/gltf+json")]);
        let formats = format_descriptors_for("gltf").unwrap();
        assert_eq!(formats.len(), 1);
        assert_eq!(formats[0].kind_id, "s.stdio.gltf.standard.2.0.representation.document");
        assert_eq!(formats[0].mimes, ["model/gltf+json"]);
        assert_eq!(formats[0].extensions, [".gltf"]);
        assert_eq!(artifact_assemblies().unwrap().len(), 36);
        assert!(crate::plugin().is_ok());
    }
}
//#endregion Assembly
