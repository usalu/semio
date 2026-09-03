//! 🗂️ Immutable trusted-catalog bundle verification for headless hub authority startup.

use super::adapters::{bounded_message, AUTHORITY_MAX_CODEC_TEXT_BYTES, TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_PACKAGES};
use super::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactIdentity};
use directory::os_directory::hex_lower;
use directory::os_store::{self, ArtifactCodec};
use semio_framework::{PackageDescriptor, PackageRole, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use tokio::io::AsyncReadExt;

/// 🧯️ Maximum accepted serialized bundle bytes.
pub const TRUSTED_BUNDLE_MAX_BYTES: u64 = 4 * 1024 * 1024;
/// 🧯️ Maximum accepted committed package-descriptor bytes.
pub const TRUSTED_DESCRIPTOR_MAX_BYTES: u64 = 4 * 1024 * 1024;
/// 🧯️ Maximum accepted bytes for one retained component.
pub const TRUSTED_COMPONENT_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// 🧯️ Maximum retained component bytes across one selected closure.
pub const TRUSTED_COMPONENT_CLOSURE_MAX_BYTES: u64 = 512 * 1024 * 1024;
/// 🧯️ Maximum retained descriptor bytes across one selected closure.
pub const TRUSTED_DESCRIPTOR_CLOSURE_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// 🧯️ Maximum UTF-8 bytes retained for one identity or version field.
pub const TRUSTED_IDENTITY_MAX_BYTES: usize = 256;
/// 🧯️ Maximum UTF-8 bytes accepted for one bundle-relative path.
pub const TRUSTED_RELATIVE_PATH_MAX_BYTES: usize = 1024;
/// 🧯️ Maximum direct dependencies accepted for one package.
pub const TRUSTED_PACKAGE_MAX_DEPENDENCIES: usize = 256;
/// 🧯️ Maximum selectable profiles in one bundle.
pub const TRUSTED_BUNDLE_MAX_PROFILES: usize = 256;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
enum BundleRole {
    Plugin,
    Extension,
}

impl BundleRole {
    fn matches(self, role: PackageRole) -> bool {
        matches!((self, role), (Self::Plugin, PackageRole::Plugin) | (Self::Extension, PackageRole::Extension))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleIdentity {
    plugin_id: String,
    package_id: String,
    version: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleCodec {
    artifact_kind: String,
    artifact_schema: String,
    pack_schema_hash: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleFile {
    path: String,
    byte_length: u64,
    sha256: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleComponent {
    path: String,
    byte_length: u64,
    sha256: String,
    blake3: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundlePackage {
    plugin_id: String,
    package_id: String,
    version: String,
    role: BundleRole,
    dependencies: Vec<BundleIdentity>,
    component: BundleComponent,
    descriptor: BundleFile,
    native_codecs: Vec<BundleCodec>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleProfile {
    id: String,
    roots: Vec<BundleIdentity>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Bundle {
    schema_version: u32,
    profiles: Vec<BundleProfile>,
    packages: Vec<BundlePackage>,
}

/// 🔗️ One explicitly linked native executable for an exact bundle artifact identity.
#[derive(Clone)]
pub struct NativeCodecBinding {
    plugin_id: String,
    package_id: String,
    artifact_kind: String,
    codec: ArtifactCodec,
}

impl NativeCodecBinding {
    /// 🪢️ Binds a native executable without deriving package identity from plugin identity.
    pub fn new(plugin_id: impl Into<String>, package_id: impl Into<String>, artifact_kind: impl Into<String>, codec: ArtifactCodec) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package_id.into(), artifact_kind: artifact_kind.into(), codec }
    }
}

/// 🧬️ One fully verified package retained in dependency-first order.
pub struct VerifiedTrustedPackage {
    plugin_id: String,
    package: PackageRef,
    version: String,
    component_sha256: [u8; 32],
    descriptor_sha256: [u8; 32],
    component_bytes: Arc<[u8]>,
    descriptor_bytes: Arc<[u8]>,
    descriptor: Arc<PackageDescriptor>,
}

impl VerifiedTrustedPackage {
    /// 🪪️ Returns the registry plugin identity, kept separate from `package_ref().package`.
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// 📦️ Returns the independently attested package id and component BLAKE3 identity.
    pub fn package_ref(&self) -> &PackageRef {
        &self.package
    }

    /// 🏷️ Returns the exact decoded descriptor version string.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// 🔐️ Returns the independently checked component SHA-256.
    pub const fn component_sha256(&self) -> &[u8; 32] {
        &self.component_sha256
    }

    /// 🔐️ Returns the raw committed descriptor-byte SHA-256.
    pub const fn descriptor_sha256(&self) -> &[u8; 32] {
        &self.descriptor_sha256
    }

    /// 🧱️ Returns the exact component bytes used to derive both retained hashes.
    pub fn component_bytes(&self) -> &[u8] {
        &self.component_bytes
    }

    /// 📜️ Returns the exact bytes decoded into `descriptor()`.
    pub fn descriptor_bytes(&self) -> &[u8] {
        &self.descriptor_bytes
    }

    /// 🗂️ Returns the decoded existing `PackageDescriptor`.
    pub fn descriptor(&self) -> &PackageDescriptor {
        &self.descriptor
    }
}

/// 🧪️ Exact native executable plus immutable authority identity.
pub struct VerifiedNativeArtifactCodec {
    identity: TrustedArtifactIdentity,
    codec: ArtifactCodec,
}

impl TrustedArtifactCodec for VerifiedNativeArtifactCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let mirror = (self.codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()
    }

    async fn apply_operation(&self, pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let encoded = directory::os_spr::encode_ops_vec(std::slice::from_ref(&operation.encoded));
        let (pack, spr, ops) = (self.codec.apply_ops_binary)(&pair.pack, &pair.spr, &encoded).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

/// 🗂️ Process-lifetime snapshot produced only after complete bundle verification and codec activation.
pub struct VerifiedTrustedCatalog {
    packages: Box<[VerifiedTrustedPackage]>,
    codecs: Box<[VerifiedNativeArtifactCodec]>,
}

impl VerifiedTrustedCatalog {
    /// 📦️ Returns selected packages in deterministic dependency-first order.
    pub fn packages(&self) -> &[VerifiedTrustedPackage] {
        &self.packages
    }

    /// 🧪️ Returns the exact number of activated artifact identities.
    pub fn codec_count(&self) -> usize {
        self.codecs.len()
    }
}

impl TrustedArtifactCatalog for VerifiedTrustedCatalog {
    type Codec = VerifiedNativeArtifactCodec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.codecs.iter().find(|entry| &entry.identity == required).ok_or_else(|| AuthorityError::Catalog("descriptor identity is absent from the verified trusted catalog".to_string()))
    }
}

impl TrustedArtifactCatalog for Arc<VerifiedTrustedCatalog> {
    type Codec = VerifiedNativeArtifactCodec;

    async fn resolve<'a>(&'a self, required: &TrustedArtifactIdentity) -> Result<&'a Self::Codec, AuthorityError> {
        self.as_ref().resolve(required).await
    }
}

/// 🏗️ Stateless verifier for one explicitly selected immutable trust bundle.
pub struct TrustedCatalogLoader;

impl TrustedCatalogLoader {
    /// 🛡️ Verifies the complete selected closure before atomically registering any native codec.
    pub async fn load(bundle_path: &Path, profile_id: &str, native_bindings: &[NativeCodecBinding], context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 0, total_units: 1 })?;
        let bundle_path = tokio::fs::canonicalize(bundle_path).await.map_err(|error| catalog_error(error))?;
        let root = bundle_path.parent().ok_or_else(|| catalog("bundle has no containing directory"))?.to_path_buf();
        let bundle_bytes = read_bounded(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        let bundle: Bundle = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let order = validate_bundle(&bundle, profile_id)?;
        let order_len = u64::try_from(order.len()).map_err(|error| catalog_error(error))?;
        let total_units = order_len.checked_mul(3).and_then(|units| units.checked_add(1)).ok_or_else(|| catalog("catalog progress total overflow"))?;
        let binding_map = validate_native_bindings(native_bindings)?;
        let mut retained_component_bytes = 0u64;
        let mut retained_descriptor_bytes = 0u64;
        let mut packages = Vec::with_capacity(order.len());
        let mut codecs = Vec::new();
        let mut registration_codecs = Vec::new();
        let mut resolved_paths = BTreeSet::from([bundle_path.clone()]);
        let mut consumed_bindings = BTreeSet::new();

        for (position, index) in order.into_iter().enumerate() {
            context.checkpoint()?;
            let record = &bundle.packages[index];
            let component_path = contained_path(&root, &record.component.path).await?;
            if !resolved_paths.insert(component_path.clone()) {
                return Err(catalog("trusted file resolves to a path already used by the selected closure"));
            }
            let component_bytes = read_bounded(&component_path, TRUSTED_COMPONENT_MAX_BYTES, context).await?;
            retained_component_bytes = retained_component_bytes.checked_add(u64::try_from(component_bytes.len()).map_err(catalog_error)?).filter(|bytes| *bytes <= TRUSTED_COMPONENT_CLOSURE_MAX_BYTES).ok_or_else(|| AuthorityError::ResourceLimit("trusted component closure byte"))?;
            verify_length(record.component.byte_length, component_bytes.len())?;
            let (component_sha256, component_blake3) = dual_hash(&component_bytes, context).await?;
            verify_digest(&record.component.sha256, component_sha256, "component sha256")?;
            verify_digest(&record.component.blake3, component_blake3, "component blake3")?;
            report_package_progress(context, position, 1, total_units)?;

            let descriptor_path = contained_path(&root, &record.descriptor.path).await?;
            if !resolved_paths.insert(descriptor_path.clone()) {
                return Err(catalog("trusted file resolves to a path already used by the selected closure"));
            }
            let descriptor_bytes = read_bounded(&descriptor_path, TRUSTED_DESCRIPTOR_MAX_BYTES, context).await?;
            retained_descriptor_bytes = retained_descriptor_bytes.checked_add(u64::try_from(descriptor_bytes.len()).map_err(catalog_error)?).filter(|bytes| *bytes <= TRUSTED_DESCRIPTOR_CLOSURE_MAX_BYTES).ok_or_else(|| AuthorityError::ResourceLimit("trusted descriptor closure byte"))?;
            verify_length(record.descriptor.byte_length, descriptor_bytes.len())?;
            let descriptor_sha256 = sha256(&descriptor_bytes, context).await?;
            verify_digest(&record.descriptor.sha256, descriptor_sha256, "descriptor sha256")?;
            let descriptor = decode_package_descriptor(&descriptor_bytes)?;
            validate_descriptor(record, &descriptor, &bundle.packages)?;
            report_package_progress(context, position, 2, total_units)?;

            for expected in &record.native_codecs {
                if codecs.len() >= TRUSTED_CATALOG_MAX_CODECS {
                    return Err(AuthorityError::ResourceLimit("trusted codec count"));
                }
                let key = CodecKey::from_parts(&record.plugin_id, &record.package_id, &expected.artifact_kind, &expected.artifact_schema);
                let binding = binding_map.get(&key).ok_or_else(|| catalog("selected artifact kind has no explicit native codec binding"))?;
                consumed_bindings.insert(key);
                let expected_hash = decode_digest(&expected.pack_schema_hash, "pack schema hash")?;
                if expected_hash == [0; 32] || binding.codec.pack_schema_hash == [0; 32] || binding.codec.pack_schema_hash != expected_hash || binding.codec.schema != expected.artifact_schema {
                    return Err(catalog("native codec schema hash is zero or mismatched"));
                }
                let identity = TrustedArtifactIdentity {
                    plugin_id: record.plugin_id.clone(),
                    package_id: record.package_id.clone(),
                    version: record.version.clone(),
                    package_hash: hex_lower(&component_blake3),
                    artifact_kind: expected.artifact_kind.clone(),
                    artifact_schema: expected.artifact_schema.clone(),
                    pack_schema_hash: expected.pack_schema_hash.clone(),
                };
                if codecs.iter().any(|entry: &VerifiedNativeArtifactCodec| entry.identity == identity) {
                    return Err(catalog("duplicate exact trusted artifact identity"));
                }
                registration_codecs.push(binding.codec.clone());
                codecs.push(VerifiedNativeArtifactCodec { identity, codec: binding.codec.clone() });
            }
            if binding_map.keys().any(|key| key.plugin_id == record.plugin_id && key.package_id == record.package_id && !record.native_codecs.iter().any(|codec| key.artifact_kind == codec.artifact_kind && key.artifact_schema == codec.artifact_schema)) {
                return Err(catalog("selected package has an undeclared native codec binding"));
            }
            report_package_progress(context, position, 3, total_units)?;
            packages.push(VerifiedTrustedPackage {
                plugin_id: record.plugin_id.clone(),
                package: PackageRef { package: PackageId(record.package_id.clone()), hash: PackageHash(component_blake3) },
                version: record.version.clone(),
                component_sha256,
                descriptor_sha256,
                component_bytes: component_bytes.into(),
                descriptor_bytes: descriptor_bytes.into(),
                descriptor: Arc::new(descriptor),
            });
        }
        if consumed_bindings.len() != binding_map.len() {
            return Err(catalog("native codec binding is outside the selected declared closure"));
        }
        if codecs.is_empty() {
            return Err(catalog("selected profile exposes no executable artifact codec"));
        }
        let catalog = VerifiedTrustedCatalog { packages: packages.into_boxed_slice(), codecs: codecs.into_boxed_slice() };
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: total_units, total_units })?;
        let assembly = os_store::begin_artifact_assembly().map_err(catalog_error)?;
        os_store::preflight_document_codecs_in_assembly(&assembly, &registration_codecs).map_err(catalog_error)?;
        context.checkpoint()?;
        os_store::register_document_codecs_in_assembly(&assembly, registration_codecs).map_err(catalog_error)?;
        Ok(catalog)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CodecKey {
    plugin_id: String,
    package_id: String,
    artifact_kind: String,
    artifact_schema: String,
}

impl CodecKey {
    fn from_parts(plugin_id: &str, package_id: &str, artifact_kind: &str, artifact_schema: &str) -> Self {
        Self { plugin_id: plugin_id.to_string(), package_id: package_id.to_string(), artifact_kind: artifact_kind.to_string(), artifact_schema: artifact_schema.to_string() }
    }
}

fn catalog(message: &str) -> AuthorityError {
    AuthorityError::Catalog(bounded_message(message))
}

fn catalog_error(error: impl std::fmt::Display) -> AuthorityError {
    AuthorityError::Catalog(bounded_message(error))
}

fn valid_identity(value: &str) -> bool {
    !value.is_empty() && value.len() <= TRUSTED_IDENTITY_MAX_BYTES && value.trim() == value
}

fn ensure_count(actual: usize, maximum: usize, resource: &'static str) -> Result<(), AuthorityError> {
    if actual > maximum {
        return Err(AuthorityError::ResourceLimit(resource));
    }
    Ok(())
}

fn validate_identity(identity: &BundleIdentity) -> Result<(), AuthorityError> {
    if !valid_identity(&identity.plugin_id) || !valid_identity(&identity.package_id) || !valid_identity(&identity.version) {
        return Err(catalog("trusted package identity is empty, padded, or oversized"));
    }
    Ok(())
}

fn validate_file(file: &BundleFile, maximum: u64) -> Result<(), AuthorityError> {
    if file.path.is_empty() || file.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES || file.byte_length == 0 || file.byte_length > maximum {
        return Err(catalog("trusted file record is empty or exceeds its fixed boundary"));
    }
    decode_digest(&file.sha256, "sha256")?;
    Ok(())
}

fn validate_bundle(bundle: &Bundle, profile_id: &str) -> Result<Vec<usize>, AuthorityError> {
    if bundle.schema_version != 1 || bundle.packages.is_empty() || bundle.packages.len() > TRUSTED_CATALOG_MAX_PACKAGES || bundle.profiles.is_empty() || bundle.profiles.len() > TRUSTED_BUNDLE_MAX_PROFILES || !valid_identity(profile_id) {
        return Err(catalog("trusted bundle shape or version is invalid"));
    }
    let mut plugins = BTreeMap::new();
    let mut package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for (index, package) in bundle.packages.iter().enumerate() {
        let identity = BundleIdentity { plugin_id: package.plugin_id.clone(), package_id: package.package_id.clone(), version: package.version.clone() };
        validate_identity(&identity)?;
        if plugins.insert(package.plugin_id.as_str(), index).is_some() || !package_ids.insert(package.package_id.as_str()) {
            return Err(catalog("trusted plugin or package identity is duplicated"));
        }
        Version::parse(&package.version).map_err(catalog_error)?;
        ensure_count(package.dependencies.len(), TRUSTED_PACKAGE_MAX_DEPENDENCIES, "trusted dependency count")?;
        ensure_count(package.native_codecs.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
        validate_file(&BundleFile { path: package.component.path.clone(), byte_length: package.component.byte_length, sha256: package.component.sha256.clone() }, TRUSTED_COMPONENT_MAX_BYTES)?;
        decode_digest(&package.component.blake3, "component blake3")?;
        validate_file(&package.descriptor, TRUSTED_DESCRIPTOR_MAX_BYTES)?;
        if !paths.insert(package.component.path.as_str()) || !paths.insert(package.descriptor.path.as_str()) {
            return Err(catalog("trusted file path is reused across package records"));
        }
        let mut dependencies = BTreeSet::new();
        for dependency in &package.dependencies {
            validate_identity(dependency)?;
            Version::parse(&dependency.version).map_err(catalog_error)?;
            if !dependencies.insert(dependency) || dependency.plugin_id == package.plugin_id {
                return Err(catalog("trusted dependency identity is duplicated or self-referential"));
            }
        }
        let mut codec_kinds = BTreeSet::new();
        for codec in &package.native_codecs {
            if !valid_identity(&codec.artifact_kind) || !valid_identity(&codec.artifact_schema) || decode_digest(&codec.pack_schema_hash, "pack schema hash")? == [0; 32] || !codec_kinds.insert(codec.artifact_kind.as_str()) {
                return Err(catalog("trusted native codec identity is empty, zero, or duplicated"));
            }
        }
    }
    for package in &bundle.packages {
        for dependency in &package.dependencies {
            let index = *plugins.get(dependency.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle dependency is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != dependency.package_id || expected.version != dependency.version {
                return Err(catalog("trusted bundle dependency identity conflicts with its package record"));
            }
        }
    }
    let mut profiles = BTreeSet::new();
    let mut selected = None;
    for profile in &bundle.profiles {
        if !valid_identity(&profile.id) || profile.roots.is_empty() || profile.roots.len() > TRUSTED_CATALOG_MAX_PACKAGES || !profiles.insert(profile.id.as_str()) {
            return Err(catalog("trusted bundle profile is empty, oversized, or duplicated"));
        }
        let mut roots = BTreeSet::new();
        for root in &profile.roots {
            validate_identity(root)?;
            if !roots.insert(root) {
                return Err(catalog("trusted bundle profile root is duplicated"));
            }
            let index = *plugins.get(root.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle profile root is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != root.package_id || expected.version != root.version {
                return Err(catalog("trusted bundle profile root conflicts with its package record"));
            }
        }
        if profile.id == profile_id {
            selected = Some(profile);
        }
    }
    let profile = selected.ok_or_else(|| catalog("selected trusted bundle profile is missing"))?;
    let mut closure = BTreeSet::new();
    let mut queue = VecDeque::new();
    for root in &profile.roots {
        queue.push_back(root.clone());
    }
    while let Some(required) = queue.pop_front() {
        let index = *plugins.get(required.plugin_id.as_str()).ok_or_else(|| catalog("selected dependency closure is incomplete"))?;
        let package = &bundle.packages[index];
        if package.package_id != required.package_id || package.version != required.version {
            return Err(catalog("selected dependency identity conflicts with its package record"));
        }
        if closure.insert(index) {
            queue.extend(package.dependencies.iter().cloned());
        }
    }
    let mut indegree = BTreeMap::new();
    let mut dependents: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &index in &closure {
        let package = &bundle.packages[index];
        indegree.insert(index, package.dependencies.len());
        for dependency in &package.dependencies {
            let dependency_index = *plugins.get(dependency.plugin_id.as_str()).ok_or_else(|| catalog("selected dependency closure is incomplete"))?;
            let dependency_package = &bundle.packages[dependency_index];
            if !closure.contains(&dependency_index) || dependency_package.package_id != dependency.package_id || dependency_package.version != dependency.version {
                return Err(catalog("selected dependency identity conflicts with its package record"));
            }
            dependents.entry(dependency_index).or_default().push(index);
        }
    }
    let mut ready: BTreeSet<(String, usize)> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(index, _)| (bundle.packages[*index].plugin_id.clone(), *index)).collect();
    let mut order = Vec::with_capacity(closure.len());
    while let Some((_, index)) = ready.pop_first() {
        order.push(index);
        for dependent in dependents.get(&index).into_iter().flatten() {
            let degree = indegree.get_mut(dependent).expect("selected dependent has indegree");
            *degree -= 1;
            if *degree == 0 {
                ready.insert((bundle.packages[*dependent].plugin_id.clone(), *dependent));
            }
        }
    }
    if order.len() != closure.len() {
        return Err(catalog("selected dependency closure contains a cycle"));
    }
    Ok(order)
}

fn validate_native_bindings(bindings: &[NativeCodecBinding]) -> Result<BTreeMap<CodecKey, &NativeCodecBinding>, AuthorityError> {
    ensure_count(bindings.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
    let mut map = BTreeMap::new();
    for binding in bindings {
        if !valid_identity(&binding.plugin_id) || !valid_identity(&binding.package_id) || !valid_identity(&binding.artifact_kind) || !valid_identity(&binding.codec.schema) {
            return Err(catalog("native codec binding identity is invalid"));
        }
        let key = CodecKey::from_parts(&binding.plugin_id, &binding.package_id, &binding.artifact_kind, &binding.codec.schema);
        if map.insert(key, binding).is_some() {
            return Err(catalog("native codec binding identity is duplicated"));
        }
    }
    Ok(map)
}

fn validate_descriptor(record: &BundlePackage, descriptor: &PackageDescriptor, packages: &[BundlePackage]) -> Result<(), AuthorityError> {
    if descriptor.descriptor_version != 1 || !record.role.matches(descriptor.role) || descriptor.manifest.plugin_id != record.plugin_id || descriptor.manifest.version != record.version || descriptor.hashes.wasm_sha256 != record.component.sha256 {
        return Err(catalog("decoded package descriptor identity does not exactly match its trust record"));
    }
    if decode_digest(&descriptor.hashes.core_wasm_sha256, "descriptor core wasm sha256")? == [0; 32] || decode_digest(&descriptor.hashes.descriptor_sha256, "descriptor metadata sha256")? == [0; 32] {
        return Err(catalog("decoded package descriptor hash metadata is zero"));
    }
    if descriptor.manifest.dependencies.len() != record.dependencies.len() || descriptor.manifest.artifact_kinds.len() != record.native_codecs.len() {
        return Err(catalog("decoded manifest declaration counts do not match the trust record"));
    }
    let records: BTreeMap<&str, &BundlePackage> = packages.iter().map(|package| (package.plugin_id.as_str(), package)).collect();
    let mut manifest_dependencies = BTreeSet::new();
    for dependency in &descriptor.manifest.dependencies {
        if !manifest_dependencies.insert(dependency.plugin_id.as_str()) {
            return Err(catalog("decoded manifest dependency is duplicated"));
        }
        let expected = record.dependencies.iter().find(|entry| entry.plugin_id == dependency.plugin_id).ok_or_else(|| catalog("decoded manifest dependency is absent from the trust record"))?;
        let package = records.get(expected.plugin_id.as_str()).ok_or_else(|| catalog("decoded manifest dependency package is missing"))?;
        let version = Version::parse(&package.version).map_err(catalog_error)?;
        if expected.package_id != package.package_id || expected.version != package.version || !dependency.version.matches(&version) {
            return Err(catalog("decoded manifest dependency version or package identity conflicts"));
        }
    }
    let mut manifest_kinds = BTreeSet::new();
    for kind in &descriptor.manifest.artifact_kinds {
        if !manifest_kinds.insert(kind.id.as_str()) {
            return Err(catalog("decoded manifest artifact kind is duplicated"));
        }
        if !record.native_codecs.iter().any(|codec| codec.artifact_kind == kind.id && codec.artifact_schema == kind.schema) {
            return Err(catalog("decoded manifest artifact kind is absent from the trust record"));
        }
    }
    Ok(())
}

async fn contained_path(root: &Path, relative: &str) -> Result<PathBuf, AuthorityError> {
    if relative.is_empty()
        || relative.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES
        || relative.contains('\\')
        || Path::new(relative).is_absolute()
        || Path::new(relative).components().any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
    {
        return Err(catalog("trusted file path is not a bounded relative path"));
    }
    let path = tokio::fs::canonicalize(root.join(relative)).await.map_err(catalog_error)?;
    if !path.starts_with(root) {
        return Err(catalog("trusted file path escapes the bundle root"));
    }
    Ok(path)
}

async fn read_bounded(path: &Path, maximum: u64, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
    context.checkpoint()?;
    let metadata = tokio::fs::metadata(path).await.map_err(catalog_error)?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > maximum {
        return Err(catalog("trusted file is empty, non-regular, or exceeds its fixed byte boundary"));
    }
    let capacity = usize::try_from(metadata.len()).map_err(catalog_error)?;
    let mut file = tokio::fs::File::open(path).await.map_err(catalog_error)?;
    let mut bytes = Vec::with_capacity(capacity);
    let mut chunk = [0u8; 64 * 1024];
    loop {
        context.checkpoint()?;
        let read = file.read(&mut chunk).await.map_err(catalog_error)?;
        if read == 0 {
            break;
        }
        let next = bytes.len().checked_add(read).ok_or_else(|| AuthorityError::ResourceLimit("trusted file byte"))?;
        if u64::try_from(next).map_err(catalog_error)? > maximum {
            return Err(catalog("trusted file changed beyond its fixed byte boundary while reading"));
        }
        bytes.extend_from_slice(&chunk[..read]);
        semio_framework_async::yield_once().await;
    }
    context.checkpoint()?;
    if bytes.is_empty() {
        return Err(catalog("trusted file became empty while reading"));
    }
    Ok(bytes)
}

async fn dual_hash(bytes: &[u8], context: &OperationContext<'_>) -> Result<([u8; 32], [u8; 32]), AuthorityError> {
    let mut sha256 = Sha256::new();
    let mut blake3 = Hasher::new();
    for chunk in bytes.chunks(64 * 1024) {
        context.checkpoint()?;
        sha256.update(chunk);
        blake3.update(chunk);
        semio_framework_async::yield_once().await;
    }
    context.checkpoint()?;
    Ok((sha256.finalize(), *blake3.finalize().as_bytes()))
}

async fn sha256(bytes: &[u8], context: &OperationContext<'_>) -> Result<[u8; 32], AuthorityError> {
    let mut hash = Sha256::new();
    for chunk in bytes.chunks(64 * 1024) {
        context.checkpoint()?;
        hash.update(chunk);
        semio_framework_async::yield_once().await;
    }
    context.checkpoint()?;
    Ok(hash.finalize())
}

fn decode_package_descriptor(bytes: &[u8]) -> Result<PackageDescriptor, AuthorityError> {
    let value = directory::os_store::pack_rt::decode_wire_value(bytes).map_err(catalog_error)?;
    let mut value = serde_json::Value::from(value);
    normalize_integral_json_numbers(&mut value);
    serde_json::from_value(value).map_err(catalog_error)
}

fn normalize_integral_json_numbers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) if number.is_f64() => {
            let Some(float) = number.as_f64() else { return };
            if float.fract() != 0.0 || !float.is_finite() {
                return;
            }
            if float >= 0.0 && float <= 9_007_199_254_740_991.0 {
                *value = serde_json::Value::Number(serde_json::Number::from(float as u64));
            } else if float >= -9_007_199_254_740_991.0 && float < 0.0 {
                *value = serde_json::Value::Number(serde_json::Number::from(float as i64));
            }
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(normalize_integral_json_numbers),
        serde_json::Value::Object(entries) => entries.values_mut().for_each(normalize_integral_json_numbers),
        _ => {}
    }
}

fn verify_length(expected: u64, actual: usize) -> Result<(), AuthorityError> {
    if u64::try_from(actual).map_err(catalog_error)? != expected {
        return Err(catalog("trusted file byte length does not match its trust record"));
    }
    Ok(())
}

fn verify_digest(expected: &str, actual: [u8; 32], label: &str) -> Result<(), AuthorityError> {
    if decode_digest(expected, label)? != actual {
        return Err(catalog("trusted file digest does not match its trust record"));
    }
    Ok(())
}

fn decode_digest(value: &str, label: &str) -> Result<[u8; 32], AuthorityError> {
    if value.len() != 64 || value.bytes().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(catalog(&format!("{label} is not canonical lowercase hexadecimal")));
    }
    let mut result = [0; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let nibble = |byte: u8| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        result[index] = (nibble(pair[0]) << 4) | nibble(pair[1]);
    }
    Ok(result)
}

fn report_package_progress(context: &OperationContext<'_>, package_position: usize, package_phase: u64, total_units: u64) -> Result<(), AuthorityError> {
    let position = u64::try_from(package_position).map_err(catalog_error)?;
    let completed_units = position.checked_mul(3).and_then(|units| units.checked_add(package_phase)).ok_or_else(|| catalog("catalog progress overflow"))?;
    context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogLoading, completed_units, total_units })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact_authority::adapters::AUTHORITY_MAX_DIAGNOSTIC_BYTES;
    use crate::artifact_authority::{AuthorityLimits, AuthorityOperationControl};
    use directory::os_store::{document_codec, ArtifactPackFiles, ArtifactTextFiles, VcsError};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Mutex;

    static FIXTURE_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

    struct TestControl {
        cancelled: AtomicBool,
        progress: Mutex<Vec<AuthorityProgress>>,
    }

    impl TestControl {
        fn new() -> Self {
            Self { cancelled: AtomicBool::new(false), progress: Mutex::new(Vec::new()) }
        }

        fn context(&self) -> OperationContext<'_> {
            OperationContext::new(u64::MAX, AuthorityLimits::maximum(), self)
        }
    }

    impl AuthorityOperationControl for TestControl {
        fn now_ms(&self) -> u64 {
            0
        }

        fn is_cancelled(&self) -> bool {
            self.cancelled.load(Ordering::SeqCst)
        }

        fn report(&self, progress: AuthorityProgress) {
            self.progress.lock().expect("progress lock").push(progress);
        }
    }

    struct FixtureDirectory {
        root: PathBuf,
        bundle_path: PathBuf,
        bundle: serde_json::Value,
        schema: String,
    }

    impl FixtureDirectory {
        fn persist_bundle(&self) {
            std::fs::write(&self.bundle_path, serde_json::to_vec_pretty(&self.bundle).expect("bundle json")).expect("write bundle");
        }

        fn component_path(&self, package: usize) -> PathBuf {
            self.root.join(self.bundle["packages"][package]["component"]["path"].as_str().expect("component path"))
        }

        fn binding(&self) -> NativeCodecBinding {
            NativeCodecBinding::new("fixture.editor", "fixture.editor.native", "fixture.document", fixture_codec(&self.schema, [0x11; 32]))
        }
    }

    impl Drop for FixtureDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn fixture_json() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/🧬️two-package/🔣️.json")).expect("trusted-catalog fixture")
    }

    fn descriptor_bytes(plugin_id: &str, version: &str, component_sha256: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> Vec<u8> {
        let artifact_kinds = schema.map_or_else(Vec::new, |schema| {
            vec![serde_json::json!({
                "id": "fixture.document",
                "name": "Fixture Document",
                "sourceFormat": "fixture",
                "componentKind": "document",
                "dimension": "data",
                "mediaCapability": "meshOnly",
                "mediaType": { "class": "data", "form": "value" },
                "schema": schema,
                "exportFormats": [],
                "importFormats": []
            })]
        });
        let dependencies = dependency.map_or_else(Vec::new, |(plugin_id, version)| vec![serde_json::json!({ "pluginId": plugin_id, "version": format!("={version}") })]);
        let json = serde_json::json!({
            "descriptorVersion": 1,
            "role": "plugin",
            "manifest": {
                "pluginId": plugin_id,
                "label": plugin_id,
                "version": version,
                "apps": [],
                "examples": [],
                "artifactKinds": artifact_kinds,
                "dependencies": dependencies
            },
            "execution": "isolated",
            "hashes": {
                "wasmSha256": component_sha256,
                "coreWasmSha256": "22".repeat(32),
                "descriptorSha256": "33".repeat(32)
            }
        });
        let descriptor: PackageDescriptor = serde_json::from_value(json).expect("package descriptor");
        let json = serde_json::to_value(descriptor).expect("serialize descriptor");
        os_store::pack_rt::encode_wire_value(&semio_framework::DslValue::from(&json))
    }

    fn prepared_fixture() -> FixtureDirectory {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst);
        let schema = format!("fixture.document.catalog.{}.{}@1", std::process::id(), sequence);
        let root = std::env::temp_dir().join(format!("semio-hub-trusted-catalog-{}-{sequence}", std::process::id()));
        std::fs::create_dir_all(root.join("components")).expect("component directory");
        std::fs::create_dir_all(root.join("descriptors")).expect("descriptor directory");
        let mut fixture = fixture_json();
        let mut bundle = fixture["bundle"].take();
        bundle["packages"][0]["nativeCodecs"][0]["artifactSchema"] = schema.clone().into();
        let component = [b'a', b'b', b'c'];
        for index in 0..2 {
            let component_path = root.join(bundle["packages"][index]["component"]["path"].as_str().expect("component path"));
            std::fs::write(component_path, component).expect("write component");
        }
        let root_descriptor = descriptor_bytes("fixture.editor", "1.2.3", fixture["componentSha256"].as_str().expect("sha256"), Some(&schema), Some(("fixture.base", "1.0.0")));
        let base_descriptor = descriptor_bytes("fixture.base", "1.0.0", fixture["componentSha256"].as_str().expect("sha256"), None, None);
        for (index, bytes) in [(0, root_descriptor), (1, base_descriptor)] {
            bundle["packages"][index]["descriptor"]["byteLength"] = bytes.len().into();
            bundle["packages"][index]["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
            let path = root.join(bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));
            std::fs::write(path, bytes).expect("write descriptor");
        }
        let bundle_path = root.join("trusted-catalog.json");
        let fixture = FixtureDirectory { root, bundle_path, bundle, schema };
        fixture.persist_bundle();
        fixture
    }

    fn fixture_compile<'a>(_dsl: &'a str, _ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>> {
        Box::pin(async { Err(VcsError::Deserialize("fixture compile is not exercised".to_string())) })
    }

    fn fixture_print<'a>(_pack: &'a [u8], _spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>> {
        Box::pin(async { Ok(ArtifactTextFiles { dsl: String::new(), ops: String::new() }) })
    }

    fn fixture_edit<'a>(_envelope: &'a directory::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>> {
        Box::pin(async { Ok(String::new()) })
    }

    fn fixture_apply<'a>(pack: &'a [u8], spr: &'a [u8], _operations: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(Vec<u8>, Vec<u8>, String), VcsError>> + 'a>> {
        Box::pin(async move { Ok((pack.to_vec(), spr.to_vec(), String::new())) })
    }

    fn fixture_codec(schema: &str, pack_schema_hash: [u8; 32]) -> ArtifactCodec {
        ArtifactCodec {
            schema: schema.to_string(),
            extension: "fixture",
            pack_schema_hash,
            compile_dsl: fixture_compile,
            print_mirror: fixture_print,
            edit_text_from_envelope: fixture_edit,
            apply_ops_binary: fixture_apply,
        }
    }

    async fn expect_load_error(fixture: &FixtureDirectory, bindings: &[NativeCodecBinding], control: &TestControl) -> AuthorityError {
        match TrustedCatalogLoader::load(&fixture.bundle_path, "fixture", bindings, &control.context()).await {
            Ok(_) => panic!("trusted catalog load unexpectedly succeeded"),
            Err(error) => error,
        }
    }

    #[tokio::test]
    async fn neutral_fixture_proves_dependency_order_hash_oracles_and_exact_limit_edges() {
        let fixture = fixture_json();
        let bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle shape");
        assert_eq!(validate_bundle(&bundle, "fixture").expect("valid closure"), vec![1, 0]);
        let bytes = fixture["componentHex"].as_str().expect("component hex").as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")).collect::<Vec<_>>();
        let control = TestControl::new();
        let (sha, internal_blake3) = dual_hash(&bytes, &control.context()).await.expect("dual hash");
        assert_eq!(hex_lower(&sha), fixture["componentSha256"]);
        assert_eq!(hex_lower(&internal_blake3), fixture["componentBlake3"]);
        assert_eq!(blake3::hash(&bytes).to_hex().as_str(), fixture["componentBlake3"]);

        let limits = &fixture["limits"];
        assert_eq!(limits["componentBytesMax"], TRUSTED_COMPONENT_MAX_BYTES);
        assert_eq!(limits["componentBytesMaxPlusOne"], TRUSTED_COMPONENT_MAX_BYTES + 1);
        assert!(validate_file(&BundleFile { path: "component.wasm".to_string(), byte_length: TRUSTED_COMPONENT_MAX_BYTES, sha256: "11".repeat(32) }, TRUSTED_COMPONENT_MAX_BYTES).is_ok());
        assert!(validate_file(&BundleFile { path: "component.wasm".to_string(), byte_length: TRUSTED_COMPONENT_MAX_BYTES + 1, sha256: "11".repeat(32) }, TRUSTED_COMPONENT_MAX_BYTES).is_err());
        assert_eq!(limits["descriptorBytesMax"], TRUSTED_DESCRIPTOR_MAX_BYTES);
        assert!(validate_file(&BundleFile { path: "descriptor.semio".to_string(), byte_length: TRUSTED_DESCRIPTOR_MAX_BYTES, sha256: "11".repeat(32) }, TRUSTED_DESCRIPTOR_MAX_BYTES).is_ok());
        assert!(validate_file(&BundleFile { path: "descriptor.semio".to_string(), byte_length: TRUSTED_DESCRIPTOR_MAX_BYTES + 1, sha256: "11".repeat(32) }, TRUSTED_DESCRIPTOR_MAX_BYTES).is_err());
        assert!(valid_identity(&"a".repeat(TRUSTED_IDENTITY_MAX_BYTES)));
        assert!(!valid_identity(&"a".repeat(TRUSTED_IDENTITY_MAX_BYTES + 1)));
        assert!(ensure_count(TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_CODECS, "trusted codec count").is_ok());
        assert!(ensure_count(TRUSTED_CATALOG_MAX_CODECS + 1, TRUSTED_CATALOG_MAX_CODECS, "trusted codec count").is_err());
    }

    #[tokio::test]
    async fn loader_retains_exact_bytes_and_independent_identities_before_atomic_codec_activation() {
        let fixture = prepared_fixture();
        let binding = fixture.binding();
        let control = TestControl::new();
        let catalog = TrustedCatalogLoader::load(&fixture.bundle_path, "fixture", &[binding], &control.context()).await.expect("verified catalog");
        assert_eq!(catalog.packages().iter().map(VerifiedTrustedPackage::plugin_id).collect::<Vec<_>>(), vec!["fixture.base", "fixture.editor"]);
        let editor = &catalog.packages()[1];
        assert_eq!(editor.package_ref().package.0, "fixture.editor.native");
        assert_ne!(editor.plugin_id(), editor.package_ref().package.0);
        assert_eq!(editor.component_bytes(), b"abc");
        assert_eq!(hex_lower(editor.component_sha256()), fixture_json()["componentSha256"]);
        assert_eq!(editor.descriptor().manifest.plugin_id, editor.plugin_id());
        assert_eq!(editor.descriptor().manifest.version, editor.version());
        assert_eq!(Sha256::digest(editor.descriptor_bytes()), *editor.descriptor_sha256());
        assert_eq!(catalog.codec_count(), 1);
        assert!(document_codec(&fixture.schema).await.expect("codec registry").is_some());
        let progress = control.progress.lock().expect("progress lock");
        assert_eq!(progress.first().map(|entry| entry.stage), Some(AuthorityProgressStage::Preflight));
        assert_eq!(progress.last().map(|entry| entry.stage), Some(AuthorityProgressStage::CatalogResolved));
        assert!(progress.iter().all(|entry| entry.completed_units <= entry.total_units));
    }

    #[tokio::test]
    async fn all_trust_failures_precede_activation_and_have_bounded_diagnostics() {
        let mut mutated = prepared_fixture();
        std::fs::write(mutated.component_path(0), b"abd").expect("mutate component");
        let control = TestControl::new();
        let error = expect_load_error(&mutated, &[mutated.binding()], &control).await;
        assert!(error.to_string().contains("digest"));
        assert!(document_codec(&mutated.schema).await.expect("codec registry").is_none());

        let mut descriptor_mutation = prepared_fixture();
        let descriptor_path = descriptor_mutation.root.join(descriptor_mutation.bundle["packages"][0]["descriptor"]["path"].as_str().expect("descriptor path"));
        let mut bytes = std::fs::read(&descriptor_path).expect("descriptor bytes");
        let last = bytes.last_mut().expect("descriptor byte");
        *last ^= 1;
        std::fs::write(descriptor_path, bytes).expect("mutate descriptor");
        let error = expect_load_error(&descriptor_mutation, &[descriptor_mutation.binding()], &control).await;
        assert!(error.to_string().contains("digest"));
        assert!(document_codec(&descriptor_mutation.schema).await.expect("codec registry").is_none());

        let missing = prepared_fixture();
        let error = expect_load_error(&missing, &[], &control).await;
        assert!(error.to_string().contains("no explicit native codec"));
        assert!(document_codec(&missing.schema).await.expect("codec registry").is_none());

        let wrong_package = prepared_fixture();
        let lossy = NativeCodecBinding::new("fixture.editor", "fixture.editor", "fixture.document", fixture_codec(&wrong_package.schema, [0x11; 32]));
        let error = expect_load_error(&wrong_package, &[lossy], &control).await;
        assert!(error.to_string().contains("no explicit native codec"));
        assert!(document_codec(&wrong_package.schema).await.expect("codec registry").is_none());

        let mut zero = prepared_fixture();
        zero.bundle["packages"][0]["nativeCodecs"][0]["packSchemaHash"] = "00".repeat(32).into();
        zero.persist_bundle();
        let error = expect_load_error(&zero, &[zero.binding()], &control).await;
        assert!(error.to_string().contains("zero"));
        assert!(document_codec(&zero.schema).await.expect("codec registry").is_none());

        let mismatch = prepared_fixture();
        let binding = NativeCodecBinding::new("fixture.editor", "fixture.editor.native", "fixture.document", fixture_codec(&mismatch.schema, [0x12; 32]));
        let error = expect_load_error(&mismatch, &[binding], &control).await;
        assert!(error.to_string().contains("mismatched"));
        assert!(document_codec(&mismatch.schema).await.expect("codec registry").is_none());

        let cancelled = prepared_fixture();
        let control = TestControl::new();
        control.cancelled.store(true, Ordering::SeqCst);
        assert_eq!(expect_load_error(&cancelled, &[cancelled.binding()], &control).await, AuthorityError::Cancelled);
        assert!(document_codec(&cancelled.schema).await.expect("codec registry").is_none());
        assert!(catalog_error("x".repeat(AUTHORITY_MAX_DIAGNOSTIC_BYTES * 2)).to_string().len() <= AUTHORITY_MAX_DIAGNOSTIC_BYTES + 40);
    }

    #[test]
    fn bundle_rejects_incomplete_duplicate_conflicting_and_escaping_declarations() {
        let fixture = fixture_json();
        let mut bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        bundle.packages.pop();
        assert!(validate_bundle(&bundle, "fixture").expect_err("incomplete closure").to_string().contains("incomplete"));

        let mut bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        bundle.packages[1].package_id = bundle.packages[0].package_id.clone();
        assert!(validate_bundle(&bundle, "fixture").expect_err("duplicate package identity").to_string().contains("duplicated"));

        let mut bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        bundle.profiles[0].roots[0].version = "9.9.9".to_string();
        assert!(validate_bundle(&bundle, "fixture").expect_err("conflicting root identity").to_string().contains("conflicts"));

        let mut bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        bundle.packages[0].component.path = "../escape.wasm".to_string();
        let control = TestControl::new();
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        let error = runtime.block_on(contained_path(Path::new("."), &bundle.packages[0].component.path)).expect_err("escaping path");
        assert!(error.to_string().contains("relative path"));
    }
}
