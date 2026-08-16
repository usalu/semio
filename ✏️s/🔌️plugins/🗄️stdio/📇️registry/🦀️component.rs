//! 🧾️ Schema-owned stdio artifact-definition assembly.

use semio_framework_plugin::{
    ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, ArtifactIdentity,
    ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, PluginAssemblyError,
};
use semio_framework_plugin::io::FormatDescriptor;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

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
struct Ledger {
    normative_source: Option<String>, publication_date: Option<String>, source_checksum: Option<String>,
    redistribution_status: String, clauses_or_features: Vec<String>, profiles: Vec<String>,
    registered_code_points: Vec<String>, read: String, write: String, lossless: String, canonical: String,
    validators: Vec<String>, mutations: Vec<String>, inferences: Vec<String>, fixtures: Vec<String>,
}
//#endregion SourceSchema

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
    if source.artifact == "epw" && source.representations.iter().any(|item| !item.mimes.is_empty()) { return Err(failure("EPW must remain MIME-unregistered")); }
    let locales = source.localized_descriptors.iter().map(|item| item.locale.as_str()).collect::<BTreeSet<_>>();
    if locales != BTreeSet::from(["de", "en"]) { return Err(failure(format!("{owner} must own English and German descriptors"))); }
    for item in &source.localized_descriptors { if item.id != format!("{owner}.localization.{}", item.locale) || item.name.is_empty() || item.description.is_empty() { return Err(failure(format!("invalid localization {}", item.id))); } id(&item.id)?; }
    for item in &source.resources { if item.status != "unimplemented" || item.external_reference_policy.is_empty() { return Err(failure(format!("invalid resource {}", item.id))); } child(&item.id, &owner, "resource")?; id(&item.id)?; }
    for item in &source.conformance_suites { if item.status != "unimplemented" { return Err(failure(format!("invalid conformance suite {}", item.id))); } child(&item.id, &owner, "conformance-suite")?; id(&item.id)?; for fixture in &item.fixtures { id(fixture)?; } }
    for item in &source.codecs { if !item.executable_registration || item.status != "implemented" || item.from.is_empty() || item.to.is_empty() { return Err(failure(format!("invalid executable codec {}", item.id))); } return Err(failure(format!("{} needs a typed executable codec mapping", item.id))); }
    for item in source.mutations.iter().chain(&source.inferences) { if !item.executable_registration || item.status != "implemented" { return Err(failure(format!("invalid executable leaf {}", item.id))); } return Err(failure(format!("{} needs a typed executable mapping", item.id))); }
    let ledger = &source.support_ledger;
    let states = [&ledger.read, &ledger.write, &ledger.lossless, &ledger.canonical];
    if !states.into_iter().all(|state| matches!(*state, "unimplemented" | "opaque" | "implemented")) { return Err(failure(format!("{owner} has an invalid support state"))); }
    if states.into_iter().any(|state| *state == "implemented") && (ledger.normative_source.is_none() || ledger.publication_date.is_none() || ledger.source_checksum.is_none() || ledger.redistribution_status == "unknown" || ledger.clauses_or_features.is_empty() || ledger.validators.is_empty() || ledger.fixtures.is_empty()) { return Err(failure(format!("{owner} claims implementation without normative, validator, and fixture evidence"))); }
    same("ledger profiles", ledger.profiles.clone(), source.profiles.iter().map(|item| item.id.clone()))?;
    same("ledger code points", ledger.registered_code_points.clone(), source.source_dialects.iter().flat_map(|item| item.registered_code_points.clone()))?;
    same("ledger mutations", ledger.mutations.clone(), source.mutations.iter().map(|item| item.id.clone()))?;
    same("ledger inferences", ledger.inferences.clone(), source.inferences.iter().map(|item| item.id.clone()))?;
    same("ledger fixtures", ledger.fixtures.clone(), source.conformance_suites.iter().flat_map(|item| item.fixtures.clone()))?;
    let local = source.profiles.iter().map(|item| item.id.clone()).chain(source.resources.iter().map(|item| item.id.clone())).chain(source.conformance_suites.iter().flat_map(|item| std::iter::once(item.id.clone()).chain(item.fixtures.clone()))).collect::<BTreeSet<_>>();
    for reference in ledger.validators.iter().chain(&ledger.mutations).chain(&ledger.inferences).chain(&ledger.fixtures) { if !local.contains(reference) { return Err(failure(format!("{owner} ledger reference {reference:?} does not resolve locally"))); } }
    Ok(())
}

fn validate_catalog(values: &[Source]) -> Result<(), PluginAssemblyError> {
    if values.len() != 36 { return Err(failure(format!("expected 36 artifact definitions, got {}", values.len()))); }
    let mut identities = BTreeSet::new(); let mut directories = BTreeSet::new(); let mut mimes = BTreeSet::new(); let mut extensions = BTreeSet::new(); let mut dialects = BTreeSet::new();
    for source in values {
        validate(source)?;
        if !identities.insert(source.id.clone()) || !directories.insert(source.directory.clone()) { return Err(failure(format!("duplicate artifact {}", source.id))); }
        for representation in &source.representations {
            for extension in &representation.extensions { if !extensions.insert(extension.clone()) { return Err(failure(format!("duplicate extension {extension}"))); } }
            for mime in &representation.mimes { if !mimes.insert(mime.clone()) { return Err(failure(format!("duplicate MIME {mime}"))); } }
        }
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

fn build(source: &Source) -> Result<ArtifactDefinition, PluginAssemblyError> {
    let mut definition = ArtifactDefinition::stdio(&source.artifact).map_err(PluginAssemblyError::definition)?;
    for item in &source.standards { definition = definition.standard(&item.revision, descriptor(item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.profiles { definition = definition.profile(child(&item.standard, &source.id, "standard")?, &item.profile, descriptor(item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.source_dialects { definition = definition.source_dialect(child(&item.standard, &source.id, "standard")?, &item.dialect, descriptor(item)?).map_err(PluginAssemblyError::definition)?; }
    for item in &source.representations {
        let identity = ArtifactIdentity::stdio_artifact(&source.artifact).and_then(|identity| identity.standard(child(&item.standard, &source.id, "standard")?)).and_then(|identity| identity.representation(&item.representation)).map_err(PluginAssemblyError::definition)?;
        let mut capability = ArtifactCapability::new(identity, ArtifactCapabilityKind::representation()).descriptor(descriptor(item)?).map_err(PluginAssemblyError::definition)?;
        for mime in &item.mimes { capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::mime(), mime)?).map_err(PluginAssemblyError::definition)?; }
        definition = definition.capability(capability).map_err(PluginAssemblyError::definition)?;
        for extension in &item.extensions {
            let segment = extension.strip_prefix('.').ok_or_else(|| failure(format!("{} has no extension", item.id)))?;
            let identity = ArtifactIdentity::stdio_artifact(&source.artifact).and_then(|identity| identity.child("extension")).and_then(|identity| identity.child(segment)).map_err(PluginAssemblyError::definition)?;
            let capability = ArtifactCapability::new(identity, ArtifactCapabilityKind::extension()).descriptor(descriptor(item)?).and_then(|value| value.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), extension)?)).map_err(PluginAssemblyError::definition)?;
            definition = definition.capability(capability).map_err(PluginAssemblyError::definition)?;
        }
    }
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
pub fn artifact_assemblies() -> Result<Vec<ArtifactAssembly>, PluginAssemblyError> {
    let factories: [fn(ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError>; 36] = [
        crate::artifacts::binary::assembly, crate::artifacts::txt::assembly, crate::artifacts::xml::assembly,
        crate::artifacts::deflate::assembly, crate::artifacts::zip::assembly, crate::artifacts::json::assembly,
        crate::artifacts::csv::assembly, crate::artifacts::md::assembly, crate::artifacts::gltf::assembly,
        crate::artifacts::obj::assembly, crate::artifacts::stl::assembly, crate::artifacts::ply::assembly,
        crate::artifacts::las::assembly, crate::artifacts::step::assembly, crate::artifacts::ifc::assembly,
        crate::artifacts::dwg::assembly, crate::artifacts::dxf::assembly, crate::artifacts::svg::assembly,
        crate::artifacts::png::assembly, crate::artifacts::jpg::assembly, crate::artifacts::gif::assembly,
        crate::artifacts::bmp::assembly, crate::artifacts::tiff::assembly, crate::artifacts::pdf::assembly,
        crate::artifacts::docx::assembly, crate::artifacts::pptx::assembly, crate::artifacts::xlsx::assembly,
        crate::artifacts::bcf::assembly, crate::artifacts::semio::assembly, crate::artifacts::mp4::assembly,
        crate::artifacts::avi::assembly, crate::artifacts::mp3::assembly, crate::artifacts::wav::assembly,
        crate::artifacts::epw::assembly, crate::artifacts::tsv::assembly, crate::artifacts::html::assembly,
    ];
    artifact_definitions()?.into_iter().zip(factories).map(|(definition, factory)| factory(definition)).collect()
}

/// 🛂️ Derives every runtime format descriptor from schema-owned representations.
pub fn format_descriptors() -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    let values = sources()?; validate_catalog(&values)?;
    values.iter().flat_map(|source| source.representations.iter().map(move |representation| (source, representation))).map(|(source, representation)| {
        let english = source.localized_descriptors.iter().find(|item| item.locale == "en").ok_or_else(|| failure(format!("{} has no English descriptor", source.id)))?;
        Ok(FormatDescriptor { kind_id: representation.id.clone(), short_id: representation.id.clone(), aliases: representation.aliases.clone(), mimes: representation.mimes.clone(), extensions: representation.extensions.clone(), name: english.name.clone(), full_name: english.description.clone(), neutral: representation.neutral, dir_name: source.directory.clone(), is_binary: representation.is_binary })
    }).collect()
}
//#endregion Assembly
