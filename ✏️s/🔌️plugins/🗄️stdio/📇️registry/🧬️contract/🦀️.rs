//! 🧩 Shared, catalog-independent contracts for one stdio artifact package.

pub use pack;
pub use semio_framework_os_kernel as kernel;

use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::{
    ArtifactCapability, ArtifactCapabilityKind, ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, ArtifactExecutableIdentity, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactKindSpec, ArtifactLocale,
    PluginAssemblyError,
};
use semio_framework_value_derive as value_derive;
use std::collections::{BTreeMap, BTreeSet};

/// 🧩 One schema definition paired with its optional executable declaration.
pub enum ArtifactAssembly {
    Definition(ArtifactDefinition),
    Runtime(Box<ArtifactDeclaration>),
}

impl ArtifactAssembly {
    /// 🧾 Borrows the schema definition represented by this assembly.
    pub fn definition(&self) -> &ArtifactDefinition {
        match self {
            Self::Definition(definition) => definition,
            Self::Runtime(declaration) => declaration.definition(),
        }
    }
}

/// ⚙️ One schema capability bound to its artifact-owned executable address.
pub struct ArtifactExecutable {
    pub identity: String,
    pub executable: ArtifactExecutableIdentity,
}

impl ArtifactExecutable {
    /// 🧷 Captures a typed artifact executable without exporting its implementation type.
    pub fn from_function_pointer(identity: impl Into<String>, function: *const ()) -> Self {
        Self { identity: identity.into(), executable: ArtifactExecutableIdentity::from_function_pointer(function) }
    }
}

/// 🏭 One artifact-owned native document codec constructor.
#[derive(Clone, Copy)]
pub struct NativeCodecFactory {
    pub id: &'static str,
    pub artifact: &'static str,
    pub kind: fn() -> ArtifactKindSpec,
    pub codec: fn() -> kernel::ArtifactCodec,
}

/// 🪢 One verified native codec factory bound to schema and component identities.
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
    pub factory: fn() -> kernel::ArtifactCodec,
}

impl NativeCodecFactoryReceipt {
    /// 🔐 Rechecks the immutable factory result before a loader can bind it.
    pub fn instantiate(&self) -> Result<kernel::ArtifactCodec, PluginAssemblyError> {
        let codec = (self.factory)();
        if self.plugin_id.is_empty()
            || self.package_id.is_empty()
            || self.package_version.is_empty()
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

/// 📦 Artifact-owned functions consumed by a selected component catalog.
#[derive(Clone, Copy)]
pub struct ArtifactContribution {
    pub identity: &'static str,
    pub schema: &'static str,
    pub definition: fn() -> Result<ArtifactDefinition, PluginAssemblyError>,
    pub assembly: fn() -> Result<ArtifactAssembly, PluginAssemblyError>,
    pub formats: fn() -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError>,
    pub native_codecs: fn() -> Vec<NativeCodecFactory>,
}

/// 📊 Category counts keep declaration, registration, implementation, and verification distinct.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapabilityCounts {
    pub codecs: usize,
    pub mutations: usize,
    pub inferences: usize,
}

/// 📒 Honest capability status ledger for one or more artifact schemas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapabilityLedger {
    pub declared: CapabilityCounts,
    pub registered: CapabilityCounts,
    pub implemented: CapabilityCounts,
    pub verified: CapabilityCounts,
}

impl CapabilityLedger {
    /// ➕ Adds an independently validated artifact ledger.
    pub fn include(&mut self, other: Self) {
        self.declared.codecs += other.declared.codecs;
        self.declared.mutations += other.declared.mutations;
        self.declared.inferences += other.declared.inferences;
        self.registered.codecs += other.registered.codecs;
        self.registered.mutations += other.registered.mutations;
        self.registered.inferences += other.registered.inferences;
        self.implemented.codecs += other.implemented.codecs;
        self.implemented.mutations += other.implemented.mutations;
        self.implemented.inferences += other.implemented.inferences;
        self.verified.codecs += other.verified.codecs;
        self.verified.mutations += other.verified.mutations;
        self.verified.inferences += other.verified.inferences;
    }
}

/// 🗂 Catalog metadata exposed without leaking the private artifact schema representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactSchemaSummary {
    pub identity: String,
    pub artifact: String,
    pub directory: String,
    pub dependencies: Vec<String>,
    pub representations: Vec<ArtifactRepresentationClaims>,
    pub source_dialects: Vec<String>,
    pub runtime_capabilities: Vec<String>,
    pub formats: Vec<FormatDescriptor>,
}

/// 🏷️ Raw MIME and extension ownership retained for catalog-wide collision checks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactRepresentationClaims {
    pub mimes: Vec<String>,
    pub extensions: Vec<String>,
}

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

fn failure(message: impl Into<String>) -> PluginAssemblyError {
    PluginAssemblyError::new("stdio.definition", message)
}

fn source(schema: &'static str) -> Result<Source, PluginAssemblyError> {
    if schema.len() > 2 * 1024 * 1024 {
        return Err(failure("artifact definition exceeds 2 MiB"));
    }
    pack::from_json_str(schema).map_err(|error| failure(format!("cannot parse artifact definition: {error}")))
}

fn descriptor<T: kernel::ToValue>(value: &T) -> Vec<u8> {
    pack::to_json_string(value).into_bytes()
}

fn id(value: &str) -> Result<(), PluginAssemblyError> {
    ArtifactIdentity::parse(value).map(|_| ()).map_err(PluginAssemblyError::definition)
}

fn child<'a>(identity: &'a str, owner: &str, namespace: &str) -> Result<&'a str, PluginAssemblyError> {
    identity.strip_prefix(&format!("{owner}.{namespace}.")).filter(|value| !value.contains('.')).ok_or_else(|| failure(format!("{identity:?} is not a direct {namespace} leaf of {owner}")))
}

fn versioned_leaf(identity: &str, prefix: &str) -> Result<(), PluginAssemblyError> {
    let leaf = identity.strip_prefix(prefix).ok_or_else(|| failure(format!("{identity:?} is not owned by {prefix:?}")))?;
    let (semantic, version) = leaf.rsplit_once(".v").ok_or_else(|| failure(format!("{identity:?} must end in a canonical vN leaf")))?;
    if semantic.is_empty() || semantic.contains('.') || version.is_empty() || !version.bytes().all(|byte| byte.is_ascii_digit()) || version.starts_with('0') {
        return Err(failure(format!("{identity:?} must end in a canonical vN leaf")));
    }
    id(identity)
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

fn expected_executable_ids(source: &Source) -> BTreeSet<String> {
    source
        .codecs
        .iter()
        .map(|item| (&item.id, item.executable_registration))
        .chain(source.mutations.iter().map(|item| (&item.id, item.executable_registration)))
        .chain(source.inferences.iter().map(|item| (&item.id, item.executable_registration)))
        .filter(|(_, registered)| *registered)
        .map(|(identity, _)| identity.clone())
        .collect()
}

fn same(label: &str, left: impl IntoIterator<Item = String>, right: impl IntoIterator<Item = String>) -> Result<(), PluginAssemblyError> {
    if left.into_iter().collect::<BTreeSet<_>>() != right.into_iter().collect::<BTreeSet<_>>() {
        return Err(failure(format!("{label} diverges from its schema collection")));
    }
    Ok(())
}

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
        if !source.source_dialects.iter().any(|dialect| dialect.id == item.from) || !source.source_dialects.iter().any(|dialect| dialect.id == item.to) || !matches!(item.status.as_str(), "unimplemented" | "implemented" | "verified") {
            return Err(failure(format!("invalid codec {}", item.id)));
        }
        match (&item.native_factory, item.executable_registration) {
            (None, false) => {}
            (Some(binding), true) if matches!(item.status.as_str(), "implemented" | "verified") => {
                native_codec_hash(&binding.pack_schema_hash)?;
                if binding.factory_id.is_empty() || binding.artifact_kind.is_empty() || binding.document_schema.is_empty() || binding.extension.is_empty() || binding.runtime_capability_id.is_empty() {
                    return Err(failure(format!("codec {} has an incomplete native factory binding", item.id)));
                }
            }
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
    let mut runtime_ids = BTreeSet::new();
    let mut runtime_claim_sets = BTreeSet::new();
    for item in &source.runtime_capabilities {
        let standard = source.standards.first().ok_or_else(|| failure("runtime capability requires an owning standard"))?;
        let prefix = match item.category.as_str() {
            "codec" | "representation" => format!("{}.{}.", standard.id, item.category),
            _ => format!("{owner}.{}.", item.category),
        };
        leaf_kind(&item.category)?;
        if item.category == "representation" {
            child(&item.id, &standard.id, "representation")?;
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

fn executable_mappings(executables: impl IntoIterator<Item = ArtifactExecutable>) -> Result<BTreeMap<String, ArtifactExecutableIdentity>, PluginAssemblyError> {
    let mut mappings = BTreeMap::new();
    for executable in executables {
        if mappings.insert(executable.identity.clone(), executable.executable).is_some() {
            return Err(failure(format!("repeated executable mapping {}", executable.identity)));
        }
    }
    Ok(mappings)
}

fn declared_capability<T: kernel::ToValue>(mappings: &BTreeMap<String, ArtifactExecutableIdentity>, identity: &str, kind: ArtifactCapabilityKind, value: &T) -> Result<ArtifactCapability, PluginAssemblyError> {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(identity).map_err(PluginAssemblyError::definition)?, kind).descriptor(descriptor(value)).map_err(PluginAssemblyError::definition)?;
    if capability.kind() == &ArtifactCapabilityKind::inference() {
        capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), identity).map_err(PluginAssemblyError::definition)?).map_err(PluginAssemblyError::definition)?;
    }
    if let Some(executable) = mappings.get(identity) {
        capability = capability.executable(*executable);
    }
    Ok(capability)
}

fn runtime_capability(item: &RuntimeCapability) -> Result<ArtifactCapability, PluginAssemblyError> {
    let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(&item.id).map_err(PluginAssemblyError::definition)?, leaf_kind(&item.category)?).descriptor(item.descriptor.as_bytes().to_vec()).map_err(PluginAssemblyError::definition)?;
    for claim in &item.claims {
        capability = capability
            .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(&claim.namespace).map_err(PluginAssemblyError::definition)?, &claim.value).map_err(PluginAssemblyError::definition)?)
            .map_err(PluginAssemblyError::definition)?;
    }
    Ok(capability)
}

fn build(source: &Source, mappings: &BTreeMap<String, ArtifactExecutableIdentity>) -> Result<ArtifactDefinition, PluginAssemblyError> {
    let mut definition = ArtifactDefinition::stdio(&source.artifact).map_err(PluginAssemblyError::definition)?;
    for item in &source.standards {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::standard(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.profiles {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::profile(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.source_dialects {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::source_dialect(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.representations {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::representation(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.codecs {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::codec(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.mutations {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::mutation(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.inferences {
        definition = definition.capability(declared_capability(mappings, &item.id, ArtifactCapabilityKind::inference(), item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.runtime_capabilities {
        definition = definition.capability(runtime_capability(item)?).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.resources {
        definition = definition.resource(child(&item.id, &source.id, "resource")?, descriptor(item)).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.localized_descriptors {
        definition = definition.localization(ArtifactLocale::parse(&item.locale).map_err(PluginAssemblyError::definition)?, format!("{}\n{}", item.name, item.description), descriptor(item)).map_err(PluginAssemblyError::definition)?;
    }
    for item in &source.conformance_suites {
        definition = definition.conformance_suite(child(&item.id, &source.id, "conformance-suite")?, descriptor(item)).map_err(PluginAssemblyError::definition)?;
    }
    Ok(definition)
}

/// 🧾 Parses and builds a schema with no executable registrations.
pub fn definition_from_schema(schema: &'static str) -> Result<ArtifactDefinition, PluginAssemblyError> {
    definition_from_schema_with_executables(schema, [])
}

/// 🧷 Parses one schema and binds its complete declared executable set.
pub fn definition_from_schema_with_executables(schema: &'static str, executables: impl IntoIterator<Item = ArtifactExecutable>) -> Result<ArtifactDefinition, PluginAssemblyError> {
    let source = source(schema)?;
    validate(&source)?;
    let mappings = executable_mappings(executables)?;
    let expected = expected_executable_ids(&source);
    if mappings.keys().cloned().collect::<BTreeSet<_>>() != expected {
        return Err(failure(format!("{} executable mappings diverge from schema registrations", source.id)));
    }
    build(&source, &mappings)
}

/// 🧷 Binds an executable artifact root to the definition it owns.
pub fn runtime_assembly(artifact: &'static str, definition: ArtifactDefinition, declaration: fn(ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError>) -> Result<ArtifactAssembly, PluginAssemblyError> {
    if definition.identity().as_str() != format!("s.stdio.{artifact}") {
        return Err(failure(format!("runtime artifact {artifact} received definition {}", definition.identity())));
    }
    declaration(definition).map(|declaration| ArtifactAssembly::Runtime(Box::new(declaration))).map_err(PluginAssemblyError::definition)
}

/// 🧾 Preserves a schema-only artifact without fabricating runtime capabilities.
pub fn definition_only_assembly(artifact: &'static str, definition: ArtifactDefinition) -> Result<ArtifactAssembly, PluginAssemblyError> {
    if definition.identity().as_str() != format!("s.stdio.{artifact}") {
        return Err(failure(format!("definition-only artifact {artifact} received definition {}", definition.identity())));
    }
    Ok(ArtifactAssembly::Definition(definition))
}

fn source_format_descriptors(source: &Source) -> Result<Vec<FormatDescriptor>, PluginAssemblyError> {
    source
        .runtime_capabilities
        .iter()
        .filter(|capability| capability.category == "representation")
        .filter_map(|capability| source.representations.iter().filter(|representation| representation_claims(representation) == runtime_claims(capability)).min_by(|left, right| left.id.cmp(&right.id)))
        .map(|representation| {
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

/// 🗂 Derives one artifact's formats strictly from its local schema.
pub fn format_descriptors(schema: &'static str) -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    let values = source(schema).and_then(|source| {
        validate(&source)?;
        source_format_descriptors(&source)
    });
    values.map_err(|error| ArtifactDefinitionError::new("stdio.format", error.to_string()))
}

/// 📋 Returns catalog validation data derived from one local schema.
pub fn schema_summary(schema: &'static str) -> Result<ArtifactSchemaSummary, PluginAssemblyError> {
    let source = source(schema)?;
    validate(&source)?;
    Ok(ArtifactSchemaSummary {
        identity: source.id.clone(),
        artifact: source.artifact.clone(),
        directory: source.directory.clone(),
        dependencies: source.dependencies.clone(),
        representations: source.representations.iter().map(|item| ArtifactRepresentationClaims { mimes: item.mimes.clone(), extensions: item.extensions.clone() }).collect(),
        source_dialects: source.source_dialects.iter().map(|item| item.id.clone()).collect(),
        runtime_capabilities: source.runtime_capabilities.iter().map(|item| item.id.clone()).collect(),
        formats: source_format_descriptors(&source)?,
    })
}

fn capability_counts<T>(items: &[T], status: impl Fn(&T) -> &str, registered: impl Fn(&T) -> bool) -> (usize, usize, usize, usize) {
    (items.len(), items.iter().filter(|item| registered(item)).count(), items.iter().filter(|item| status(item) == "implemented").count(), items.iter().filter(|item| status(item) == "verified").count())
}

/// 📊 Derives an honest capability ledger from one local schema.
pub fn capability_ledger(schema: &'static str) -> Result<CapabilityLedger, PluginAssemblyError> {
    let source = source(schema)?;
    validate(&source)?;
    let mut ledger = CapabilityLedger::default();
    let (declared, registered, implemented, verified) = capability_counts(&source.codecs, |item| &item.status, |item| item.executable_registration);
    ledger.declared.codecs = declared;
    ledger.registered.codecs = registered;
    ledger.implemented.codecs = implemented;
    ledger.verified.codecs = verified;
    let (declared, registered, implemented, verified) = capability_counts(&source.mutations, |item| &item.status, |item| item.executable_registration);
    ledger.declared.mutations = declared;
    ledger.registered.mutations = registered;
    ledger.implemented.mutations = implemented;
    ledger.verified.mutations = verified;
    let (declared, registered, implemented, verified) = capability_counts(&source.inferences, |item| &item.status, |item| item.executable_registration);
    ledger.declared.inferences = declared;
    ledger.registered.inferences = registered;
    ledger.implemented.inferences = implemented;
    ledger.verified.inferences = verified;
    Ok(ledger)
}

/// 🔏 Parses a nonzero lowercase SHA-256 value used by a codec schema binding.
pub fn native_codec_hash(value: &str) -> Result<[u8; 32], PluginAssemblyError> {
    if value.len() != 64 {
        return Err(failure("native codec pack schema hash must contain exactly 64 lowercase hexadecimal digits"));
    }
    let mut hash = [0u8; 32];
    for (index, chunk) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
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

fn native_codec_binding<'a>(source: &'a Source, factory: &NativeCodecFactory) -> Result<(&'a Codec, &'a NativeCodecBinding), PluginAssemblyError> {
    let item = source.codecs.iter().find(|codec| codec.native_factory.as_ref().is_some_and(|binding| binding.factory_id == factory.id)).ok_or_else(|| failure(format!("native factory {} has no schema binding", factory.id)))?;
    let binding = item.native_factory.as_ref().expect("filtered native binding");
    let kind = (factory.kind)();
    let codec = (factory.codec)();
    let hash = native_codec_hash(&binding.pack_schema_hash)?;
    let runtime = source.runtime_capabilities.iter().find(|capability| capability.id == binding.runtime_capability_id).ok_or_else(|| failure(format!("codec {} names missing runtime capability {}", item.id, binding.runtime_capability_id)))?;
    let extension_claim = ArtifactIdentityClaim::codec_extension(&binding.document_schema, &binding.extension).map_err(PluginAssemblyError::definition)?;
    let expected_claims = BTreeSet::from([("codec".to_owned(), binding.document_schema.clone()), (extension_claim.namespace().as_str().to_owned(), extension_claim.value().to_owned())]);
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
    Ok((item, binding))
}

/// 🧷 Derives actual executable identities for an artifact's schema-authorized codecs.
pub fn native_codec_executables(schema: &'static str, factories: &[NativeCodecFactory]) -> Result<Vec<ArtifactExecutable>, PluginAssemblyError> {
    let source = source(schema)?;
    validate(&source)?;
    let expected = source.codecs.iter().filter(|codec| codec.executable_registration).count();
    if factories.len() != expected {
        return Err(failure(format!("{} exposes {} native factories, expected {expected}", source.id, factories.len())));
    }
    factories
        .iter()
        .map(|factory| {
            let (codec, _) = native_codec_binding(&source, factory)?;
            Ok(ArtifactExecutable::from_function_pointer(codec.id.clone(), factory.codec as *const ()))
        })
        .collect()
}

/// 🪢 Validates and emits native codec receipts for one artifact contribution.
pub fn native_codec_factory_receipts(contribution: &ArtifactContribution, plugin_id: &'static str, package_id: impl Into<String>, package_version: &'static str) -> Result<Vec<NativeCodecFactoryReceipt>, PluginAssemblyError> {
    let source = source(contribution.schema)?;
    validate(&source)?;
    if contribution.identity != source.artifact {
        return Err(failure(format!("contribution {} differs from schema artifact {}", contribution.identity, source.artifact)));
    }
    let factories = (contribution.native_codecs)();
    let expected = source.codecs.iter().filter(|codec| codec.executable_registration).count();
    if factories.len() != expected {
        return Err(failure(format!("{} exposes {} native factories, expected {expected}", source.id, factories.len())));
    }
    let package_id = package_id.into();
    let mut ids = BTreeSet::new();
    let mut descriptor_ids = BTreeSet::new();
    let mut receipt_keys = BTreeSet::new();
    let mut receipts = Vec::with_capacity(factories.len());
    for factory in factories {
        let (item, binding) = native_codec_binding(&source, &factory)?;
        if !ids.insert(factory.id) || !descriptor_ids.insert(item.id.clone()) || !receipt_keys.insert((binding.artifact_kind.clone(), binding.document_schema.clone())) {
            return Err(failure(format!("{} native codec factories are not bijective", source.id)));
        }
        let receipt = NativeCodecFactoryReceipt {
            plugin_id,
            package_id: package_id.clone(),
            package_version,
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
    Ok(receipts)
}

/// 🔡 Encodes bytes with the padded RFC 4648 standard Base64 alphabet.
pub fn base64_standard(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::with_capacity(bytes.len().div_ceil(3).saturating_mul(4));
    let (chunks, remainder) = bytes.as_chunks::<3>();
    for chunk in chunks {
        let value = u32::from_be_bytes([0, chunk[0], chunk[1], chunk[2]]);
        output.push(ALPHABET[((value >> 18) & 63) as usize] as char);
        output.push(ALPHABET[((value >> 12) & 63) as usize] as char);
        output.push(ALPHABET[((value >> 6) & 63) as usize] as char);
        output.push(ALPHABET[(value & 63) as usize] as char);
    }
    match remainder {
        [first] => {
            output.push(ALPHABET[(first >> 2) as usize] as char);
            output.push(ALPHABET[((first & 3) << 4) as usize] as char);
            output.push_str("==");
        }
        [first, second] => {
            output.push(ALPHABET[(first >> 2) as usize] as char);
            output.push(ALPHABET[(((first & 3) << 4) | (second >> 4)) as usize] as char);
            output.push(ALPHABET[((second & 15) << 2) as usize] as char);
            output.push('=');
        }
        _ => {}
    }
    output
}

fn hash_hex_bytes(hash: &str) -> Vec<u8> {
    fn nibble(byte: u8) -> u8 {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => unreachable!("framework hash must be lowercase hexadecimal"),
        }
    }
    hash.as_bytes().as_chunks::<2>().0.iter().map(|pair| nibble(pair[0]) << 4 | nibble(pair[1])).collect()
}

/// 🪪 Computes the stable BLAKE3 identity of a semantic projection.
pub fn semantic_fingerprint<T: kernel::ToValue>(projection: &T) -> Result<Vec<u8>, String> {
    let encoded = pack::json_to_string(&pack::json_from_dsl_value(&kernel::ToValue::to_value(projection))).into_bytes();
    Ok(hash_hex_bytes(&semio_framework_hash::hash_bytes(&encoded)))
}

/// 📡 Implements the canonical JSON text and binary mutation wire codecs.
#[macro_export]
macro_rules! impl_serde_op_codec {
    ($mutation:ty, $what:literal) => {
        impl $crate::kernel::OpText for $mutation {
            fn print_op(&self) -> String {
                $crate::pack::json_to_string(&$crate::pack::json_from_dsl_value(&$crate::kernel::ToValue::to_value(self)))
            }

            fn parse_op(line: &str) -> Result<Self, $crate::kernel::TextError> {
                let parsed = $crate::pack::parse_json(line).map_err(|error| $crate::kernel::TextError::new(error.to_string(), $crate::kernel::TextSpan::at(1, 1)))?;
                <Self as $crate::kernel::FromValue>::from_value($crate::pack::json_to_dsl_value(&parsed)).map_err(|error| $crate::kernel::TextError::new(error.to_string(), $crate::kernel::TextSpan::at(1, 1)))
            }
        }

        impl $crate::kernel::OpBinary for $mutation {
            fn encode_op(&self) -> Result<Vec<u8>, $crate::kernel::ProtocolError> {
                Ok($crate::pack::json_to_string(&$crate::pack::json_from_dsl_value(&$crate::kernel::ToValue::to_value(self))).into_bytes())
            }

            fn decode_op(bytes: &[u8]) -> Result<Self, $crate::kernel::ProtocolError> {
                let parsed = $crate::pack::parse_json_bytes(bytes).map_err(|error| $crate::kernel::ProtocolError::Malformed { what: $what, offset: 0, detail: error.to_string() })?;
                <Self as $crate::kernel::FromValue>::from_value($crate::pack::json_to_dsl_value(&parsed)).map_err(|error| $crate::kernel::ProtocolError::Malformed { what: $what, offset: 0, detail: error.to_string() })
            }
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
