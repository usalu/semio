//! 🗂️ Immutable trusted-catalog bundle verification for headless hub authority startup.

use super::adapters::{bounded_message, AUTHORITY_MAX_CODEC_TEXT_BYTES, TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_PACKAGES};
use super::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactIdentity};
use directory::os_directory::{hex_lower, DocumentDescriptor, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use directory::os_store::{self, ArtifactCodec};
use semio_framework::{from_dsl_value, to_dsl_value, DslValue, PackageDescriptor, PackageRole, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
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
/// 🧯 Maximum immutable document-open selections retained by one catalog generation.
pub const TRUSTED_CATALOG_MAX_OPEN_TARGETS: usize = 1024;

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

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum BundleOpenRole {
    Viewer,
    Editor,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum BundleRendererTarget {
    React,
    Wgpu,
    Wasm,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleOpenTarget {
    artifact_kind: String,
    artifact_schema: String,
    pack_schema_hash: String,
    surface_id: String,
    app_id: String,
    window_kind_id: String,
    role: BundleOpenRole,
    renderer_target: BundleRendererTarget,
    parent_dialect: semio_framework::ArtifactDialect,
    grant: BundleGrant,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleGrant {
    read: bool,
    write: bool,
    observe: bool,
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
    open_targets: Vec<BundleOpenTarget>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleProfile {
    id: String,
    selected_closure: Vec<BundleIdentity>,
    selected_closure_sha256: String,
    open_target: BundleProfileOpenTarget,
    generation_id: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct BundleProfileOpenTarget {
    package: BundleIdentity,
    target: BundleOpenTarget,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Bundle {
    schema_version: u32,
    profiles: Vec<BundleProfile>,
    packages: Vec<BundlePackage>,
}

#[derive(Debug)]
struct SelectedTrustedBundleV1 {
    package_indices: Vec<usize>,
    profile: BundleProfile,
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

/// 🪪️ Borrowed immutable package identity passed from the trusted loader to one native provider.
#[derive(Clone, Copy)]
pub struct NativeCodecProviderPackageV1<'a> {
    pub plugin_id: &'a str,
    pub package_id: &'a str,
    pub version: &'a str,
}

/// 🧬️ One descriptor-committed codec requirement exposed without product-plugin types.
#[derive(Clone, Copy)]
pub struct NativeCodecProviderRequirementV1<'a> {
    pub package: NativeCodecProviderPackageV1<'a>,
    pub artifact_kind: &'a str,
    pub artifact_schema: &'a str,
    pub pack_schema_hash: &'a str,
}

/// 🔌️ Headless Hub provider port. Implementations may supply executable codecs only for the exact
/// verified package closure and cannot publish them outside the loader's atomic registration.
pub trait NativeCodecProviderSourceV1: Sync {
    fn preflight_selection(&self, _selected: &[NativeCodecProviderRequirementV1<'_>]) -> Result<(), AuthorityError> {
        Ok(())
    }

    fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &PackageDescriptor, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError>;
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
    open_targets: Box<[VerifiedDocumentOpenSelectionV1]>,
    generation_id: String,
}

/// 🧱 The exact verified component and raw descriptor bytes bound to one current selection. It is
/// produced only by [`VerifiedTrustedCatalog::assets_for_current_selection`] and carries no path,
/// origin or catalog handle.
pub struct VerifiedExecutionTargetAssets {
    pub selection: VerifiedDocumentOpenSelectionV1,
    pub component: Arc<[u8]>,
    pub descriptor: Arc<[u8]>,
}

/// 🧬 One exact document-open choice retained only after the complete catalog verifies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedDocumentOpenSelectionV1 {
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: semio_framework::ArtifactDialect,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
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

    /// 🪪 Returns the number of exact catalog-backed open choices in this immutable generation.
    pub fn open_target_count(&self) -> usize {
        self.open_targets.len()
    }

    /// 🧬 Returns the neutral SHA-256 identity of the sorted immutable open-target projection.
    pub fn generation_id(&self) -> &str {
        &self.generation_id
    }

    /// 🎯 Returns the profile's sole completely verified document-open choice without reconstructing it from public plan bytes.
    pub fn selected_document_open(&self) -> Option<&VerifiedDocumentOpenSelectionV1> {
        (self.open_targets.len() == 1).then(|| &self.open_targets[0])
    }

    /// 🧱 Returns the verified bytes of the current selection only. It is deliberately not a package
    /// lookup: it accepts no package id, digest, path or generation selector from a caller, resolves
    /// the selection from the durable descriptor and subject role alone, and answers only while the
    /// caller-observed generation is still this immutable catalog's own.
    pub fn assets_for_current_selection(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool, current_generation: &str) -> Option<VerifiedExecutionTargetAssets> {
        if current_generation != self.generation_id {
            return None;
        }
        let selection = self.resolve_document_open(descriptor, requested_surface_id, writable)?;
        let package = self.packages.iter().find(|retained| {
            retained.plugin_id == selection.package.plugin_id
                && retained.package.package.0 == selection.package.package_id
                && retained.version == selection.package.version
                && hex_lower(&retained.component_sha256) == selection.package.component_sha256
                && hex_lower(&retained.package.hash.0) == selection.package.component_blake3
                && hex_lower(&retained.descriptor_sha256) == selection.package.descriptor_byte_sha256
        })?;
        if package.component_bytes.is_empty() || package.descriptor_bytes.is_empty() || package.component_bytes.len() as u64 > TRUSTED_COMPONENT_MAX_BYTES || package.descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
            return None;
        }
        Some(VerifiedExecutionTargetAssets { selection, component: Arc::clone(&package.component_bytes), descriptor: Arc::clone(&package.descriptor_bytes) })
    }

    /// 🎯 Resolves one exact descriptor, subject role, and optional surface preference without fallback.
    pub fn resolve_document_open(&self, descriptor: &DocumentDescriptor, requested_surface_id: Option<&str>, writable: bool) -> Option<VerifiedDocumentOpenSelectionV1> {
        let role = if writable { DocumentOpenSurfaceRoleV1::Editor } else { DocumentOpenSurfaceRoleV1::Viewer };
        let mut matches = self.open_targets.iter().filter(|selection| {
            selection.package.plugin_id == descriptor.owner.plugin_id
                && selection.package.package_id == descriptor.owner.package_id
                && selection.package.version == descriptor.owner.version
                && selection.package.component_sha256 == descriptor.owner.package_hash
                && selection.artifact.kind == descriptor.artifact_kind
                && selection.artifact.schema == descriptor.artifact_schema
                && selection.artifact.pack_schema_hash == descriptor.pack_schema_hash
                && selection.surface.role == role
                && requested_surface_id.is_none_or(|requested| selection.surface.surface_id == requested)
        });
        let selected = matches.next()?.clone();
        matches.next().is_none().then_some(selected)
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
    pub async fn load(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        Self::load_selected(bundle_path, profile_id, providers, context).await
    }

    async fn load_selected(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 0, total_units: 1 })?;
        let bundle_path = tokio::fs::canonicalize(bundle_path).await.map_err(|error| catalog_error(error))?;
        let root = bundle_path.parent().ok_or_else(|| catalog("bundle has no containing directory"))?.to_path_buf();
        let bundle_bytes = read_bounded(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        let bundle: Bundle = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let SelectedTrustedBundleV1 { package_indices: order, profile } = validate_bundle(&bundle, profile_id)?;
        let order_len = u64::try_from(order.len()).map_err(|error| catalog_error(error))?;
        let total_units = order_len.checked_mul(3).and_then(|units| units.checked_add(1)).ok_or_else(|| catalog("catalog progress total overflow"))?;
        let requirements = order
            .iter()
            .flat_map(|index| {
                let record = &bundle.packages[*index];
                record.native_codecs.iter().map(move |codec| NativeCodecProviderRequirementV1 {
                    package: NativeCodecProviderPackageV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version },
                    artifact_kind: &codec.artifact_kind,
                    artifact_schema: &codec.artifact_schema,
                    pack_schema_hash: &codec.pack_schema_hash,
                })
            })
            .collect::<Vec<_>>();
        providers.preflight_selection(&requirements)?;
        drop(requirements);
        let mut retained_component_bytes = 0u64;
        let mut retained_descriptor_bytes = 0u64;
        let mut packages = Vec::with_capacity(order.len());
        let mut codecs = Vec::new();
        let mut open_targets = Vec::new();
        let mut registration_codecs = Vec::new();
        let mut resolved_paths = BTreeSet::from([bundle_path.clone()]);

        for (position, index) in order.into_iter().enumerate() {
            context.checkpoint()?;
            let record = &bundle.packages[index];
            let component_path = contained_path(&root, &record.component.path).await?;
            if !resolved_paths.insert(component_path.clone()) {
                return Err(catalog("trusted file resolves to a path already used by the selected closure"));
            }
            let component_bytes = read_bounded(&component_path, TRUSTED_COMPONENT_MAX_BYTES, context).await?;
            retained_component_bytes = retained_component_bytes
                .checked_add(u64::try_from(component_bytes.len()).map_err(catalog_error)?)
                .filter(|bytes| *bytes <= TRUSTED_COMPONENT_CLOSURE_MAX_BYTES)
                .ok_or_else(|| AuthorityError::ResourceLimit("trusted component closure byte"))?;
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
            retained_descriptor_bytes = retained_descriptor_bytes
                .checked_add(u64::try_from(descriptor_bytes.len()).map_err(catalog_error)?)
                .filter(|bytes| *bytes <= TRUSTED_DESCRIPTOR_CLOSURE_MAX_BYTES)
                .ok_or_else(|| AuthorityError::ResourceLimit("trusted descriptor closure byte"))?;
            verify_length(record.descriptor.byte_length, descriptor_bytes.len())?;
            let descriptor_sha256 = sha256(&descriptor_bytes, context).await?;
            verify_digest(&record.descriptor.sha256, descriptor_sha256, "descriptor sha256")?;
            let descriptor = decode_package_descriptor(&descriptor_bytes)?;
            validate_descriptor(record, &descriptor, &bundle.packages)?;
            report_package_progress(context, position, 2, total_units)?;

            context.checkpoint()?;
            let native_bindings = providers.preview(NativeCodecProviderPackageV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version }, &descriptor, context)?;
            context.checkpoint()?;
            let binding_map = validate_native_bindings(&native_bindings)?;
            let mut consumed_bindings = BTreeSet::new();

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
                    package_hash: hex_lower(&component_sha256),
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
            if consumed_bindings.len() != binding_map.len() {
                return Err(catalog("selected provider returned a binding outside its exact declared package closure"));
            }
            for target in &record.open_targets {
                if open_targets.len() >= TRUSTED_CATALOG_MAX_OPEN_TARGETS {
                    return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
                }
                let parent_dialect = validate_descriptor_open_target(&descriptor, target)?;
                if profile.open_target.package.plugin_id != record.plugin_id || profile.open_target.package.package_id != record.package_id || profile.open_target.package.version != record.version || profile.open_target.target != *target {
                    continue;
                }
                let declared = record.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash);
                if !declared {
                    return Err(catalog("document-open target has no exact verified native codec"));
                }
                let role = match target.role {
                    BundleOpenRole::Viewer => DocumentOpenSurfaceRoleV1::Viewer,
                    BundleOpenRole::Editor => DocumentOpenSurfaceRoleV1::Editor,
                };
                let renderer_target = match target.renderer_target {
                    BundleRendererTarget::React => DocumentOpenRendererTargetV1::React,
                    BundleRendererTarget::Wgpu => DocumentOpenRendererTargetV1::Wgpu,
                    BundleRendererTarget::Wasm => DocumentOpenRendererTargetV1::Wasm,
                };
                let selection = VerifiedDocumentOpenSelectionV1 {
                    parent_dialect,
                    package: DocumentOpenPackageV1 {
                        plugin_id: record.plugin_id.clone(),
                        package_id: record.package_id.clone(),
                        version: record.version.clone(),
                        component_sha256: hex_lower(&component_sha256),
                        component_blake3: hex_lower(&component_blake3),
                        descriptor_byte_sha256: hex_lower(&descriptor_sha256),
                    },
                    artifact: DocumentOpenArtifactV1 { kind: target.artifact_kind.clone(), schema: target.artifact_schema.clone(), pack_schema_hash: target.pack_schema_hash.clone() },
                    surface: DocumentOpenSurfaceV1 { surface_id: target.surface_id.clone(), app_id: target.app_id.clone(), window_kind_id: target.window_kind_id.clone(), role, renderer_target },
                    grant: DocumentOpenGrantV1 { read: target.grant.read, write: target.grant.write, observe: target.grant.observe },
                };
                if open_targets.iter().any(|existing| document_open_target_sort_key(existing) == document_open_target_sort_key(&selection)) {
                    return Err(catalog("document-open target identity is duplicated"));
                }
                open_targets.push(selection);
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
        if codecs.is_empty() {
            return Err(catalog("selected profile exposes no executable artifact codec"));
        }
        sort_open_targets(&mut open_targets);
        if open_targets.len() != 1 {
            return Err(catalog("selected profile must resolve exactly one document-open target"));
        }
        let generation_id = trusted_profile_generation(&bundle, &profile)?;
        if generation_id != profile.generation_id {
            return Err(catalog("trusted profile generation differs from the completely verified package, codec, and target closure"));
        }
        let catalog = VerifiedTrustedCatalog { packages: packages.into_boxed_slice(), codecs: codecs.into_boxed_slice(), open_targets: open_targets.into_boxed_slice(), generation_id };
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

fn valid_open_identity(value: &str) -> bool {
    valid_identity(value) && !value.chars().any(char::is_control)
}

fn validate_descriptor_open_target(descriptor: &PackageDescriptor, target: &BundleOpenTarget) -> Result<semio_framework::ArtifactDialect, AuthorityError> {
    let expected_role = match target.role {
        BundleOpenRole::Viewer => semio_framework::AppRole::Viewer,
        BundleOpenRole::Editor => semio_framework::AppRole::Editor,
    };
    let app = descriptor.manifest.apps.iter().find(|app| app.id == target.app_id).ok_or_else(|| catalog("document-open target app is absent from the verified descriptor"))?;
    let discoverable = descriptor.manifest.artifact_kinds.iter().any(|kind| kind.id == target.artifact_kind && kind.schema == target.artifact_schema);
    if !discoverable
        || app.id != target.surface_id
        || app.id != semio_framework::surface_app_id(&app.dialect, app.role)
        || app.role != expected_role
        || app.dialect != target.parent_dialect
        || [&app.dialect.artifact_kind, &app.dialect.standard, &app.dialect.subset].into_iter().any(|value| !valid_open_identity(value))
        || !app.window_kinds.iter().any(|window| window.id == target.window_kind_id)
        || descriptor.execution != semio_framework::ExecutionMode::Isolated
        || target.renderer_target != BundleRendererTarget::Wasm
    {
        return Err(catalog("document-open target surface, app, window, role, renderer, or artifact differs from the verified descriptor"));
    }
    Ok(app.dialect.clone())
}

fn document_open_target_sort_key(target: &VerifiedDocumentOpenSelectionV1) -> [&str; 18] {
    let role = match target.surface.role {
        DocumentOpenSurfaceRoleV1::Viewer => "viewer",
        DocumentOpenSurfaceRoleV1::Editor => "editor",
    };
    let renderer = match target.surface.renderer_target {
        DocumentOpenRendererTargetV1::React => "react",
        DocumentOpenRendererTargetV1::Wgpu => "wgpu",
        DocumentOpenRendererTargetV1::Wasm => "wasm",
    };
    [
        &target.package.plugin_id,
        &target.package.package_id,
        &target.package.version,
        &target.package.component_sha256,
        &target.package.component_blake3,
        &target.package.descriptor_byte_sha256,
        &target.artifact.kind,
        &target.artifact.schema,
        &target.artifact.pack_schema_hash,
        &target.parent_dialect.artifact_kind,
        &target.parent_dialect.standard,
        &target.parent_dialect.subset,
        &target.surface.surface_id,
        &target.surface.app_id,
        &target.surface.window_kind_id,
        role,
        renderer,
        if target.grant.write { "111" } else { "101" },
    ]
}

fn sort_open_targets(targets: &mut [VerifiedDocumentOpenSelectionV1]) {
    targets.sort_by(|left, right| document_open_target_sort_key(left).cmp(&document_open_target_sort_key(right)));
}

fn append_document_open_catalog_field(output: &mut Vec<u8>, value: &[u8]) -> Result<(), AuthorityError> {
    let length = u64::try_from(value.len()).map_err(catalog_error)?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn selected_closure_digest(identities: &[BundleIdentity]) -> Result<[u8; 32], AuthorityError> {
    let mut encoded = b"semio/hub/trusted-profile-selected-closure/v1\0".to_vec();
    encoded.extend_from_slice(&u32::try_from(identities.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in identities {
        for value in [identity.plugin_id.as_bytes(), identity.package_id.as_bytes(), identity.version.as_bytes()] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
    }
    Ok(Sha256::digest(&encoded))
}

fn trusted_profile_generation(bundle: &Bundle, profile: &BundleProfile) -> Result<String, AuthorityError> {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(b"semio/hub/trusted-profile-generation/v1\0");
    append_document_open_catalog_field(&mut encoded, profile.id.as_bytes())?;
    encoded.extend_from_slice(&u32::try_from(profile.selected_closure.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in &profile.selected_closure {
        let package = bundle
            .packages
            .iter()
            .find(|package| package.plugin_id == identity.plugin_id && package.package_id == identity.package_id && package.version == identity.version)
            .ok_or_else(|| catalog("profile generation package is outside its selected closure"))?;
        let role = match package.role {
            BundleRole::Plugin => b"plugin".as_slice(),
            BundleRole::Extension => b"extension".as_slice(),
        };
        for value in [
            package.plugin_id.as_bytes(),
            package.package_id.as_bytes(),
            package.version.as_bytes(),
            role,
            decode_digest(&package.component.sha256, "profile component sha256")?.as_slice(),
            decode_digest(&package.component.blake3, "profile component blake3")?.as_slice(),
            decode_digest(&package.descriptor.sha256, "profile descriptor sha256")?.as_slice(),
        ] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
        let mut dependencies = package.dependencies.iter().collect::<Vec<_>>();
        dependencies.sort();
        encoded.extend_from_slice(&u32::try_from(dependencies.len()).map_err(catalog_error)?.to_be_bytes());
        for dependency in dependencies {
            for value in [dependency.plugin_id.as_bytes(), dependency.package_id.as_bytes(), dependency.version.as_bytes()] {
                append_document_open_catalog_field(&mut encoded, value)?;
            }
        }
        let mut codecs = package.native_codecs.iter().collect::<Vec<_>>();
        codecs.sort_by(|left, right| (&left.artifact_kind, &left.artifact_schema, &left.pack_schema_hash).cmp(&(&right.artifact_kind, &right.artifact_schema, &right.pack_schema_hash)));
        encoded.extend_from_slice(&u32::try_from(codecs.len()).map_err(catalog_error)?.to_be_bytes());
        for codec in codecs {
            append_document_open_catalog_field(&mut encoded, codec.artifact_kind.as_bytes())?;
            append_document_open_catalog_field(&mut encoded, codec.artifact_schema.as_bytes())?;
            append_document_open_catalog_field(&mut encoded, decode_digest(&codec.pack_schema_hash, "profile codec pack schema hash")?.as_slice())?;
        }
    }
    encoded.extend_from_slice(&1u32.to_be_bytes());
    let package = bundle
        .packages
        .iter()
        .find(|package| package.plugin_id == profile.open_target.package.plugin_id && package.package_id == profile.open_target.package.package_id && package.version == profile.open_target.package.version)
        .ok_or_else(|| catalog("profile generation open-target package is absent"))?;
    let target = &profile.open_target.target;
    let role = match target.role {
        BundleOpenRole::Viewer => b"viewer".as_slice(),
        BundleOpenRole::Editor => b"editor".as_slice(),
    };
    let renderer = match target.renderer_target {
        BundleRendererTarget::React => b"react".as_slice(),
        BundleRendererTarget::Wgpu => b"wgpu".as_slice(),
        BundleRendererTarget::Wasm => b"wasm".as_slice(),
    };
    for value in [
        package.plugin_id.as_bytes(),
        package.package_id.as_bytes(),
        package.version.as_bytes(),
        decode_digest(&package.component.sha256, "open target component sha256")?.as_slice(),
        decode_digest(&package.component.blake3, "open target component blake3")?.as_slice(),
        decode_digest(&package.descriptor.sha256, "open target descriptor sha256")?.as_slice(),
        target.artifact_kind.as_bytes(),
        target.artifact_schema.as_bytes(),
        decode_digest(&target.pack_schema_hash, "open target pack schema hash")?.as_slice(),
        target.parent_dialect.artifact_kind.as_bytes(),
        target.parent_dialect.standard.as_bytes(),
        target.parent_dialect.subset.as_bytes(),
        target.surface_id.as_bytes(),
        target.app_id.as_bytes(),
        target.window_kind_id.as_bytes(),
        role,
        renderer,
        [u8::from(target.grant.read), u8::from(target.grant.write), u8::from(target.grant.observe)].as_slice(),
    ] {
        append_document_open_catalog_field(&mut encoded, value)?;
    }
    Ok(hex_lower(&Sha256::digest(&encoded)))
}

/// 🧬️ Recomputes the two canonical profile digests without trusting carried digest fields.
pub fn trusted_profile_digests_json(bundle_bytes: &[u8], profile_id: &str) -> Result<(String, String), AuthorityError> {
    let bundle: Bundle = serde_json::from_slice(bundle_bytes).map_err(catalog_error)?;
    let profile = bundle.profiles.iter().find(|profile| profile.id == profile_id).ok_or_else(|| catalog("trusted profile digest source is missing"))?;
    Ok((hex_lower(&selected_closure_digest(&profile.selected_closure)?), trusted_profile_generation(&bundle, profile)?))
}

fn valid_package_id(value: &str) -> bool {
    let Some(name) = value.strip_prefix("semio:") else { return false };
    !name.is_empty() && value.len() <= TRUSTED_IDENTITY_MAX_BYTES && !name.starts_with('-') && !name.ends_with('-') && !name.contains("--") && name.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn ensure_count(actual: usize, maximum: usize, resource: &'static str) -> Result<(), AuthorityError> {
    if actual > maximum {
        return Err(AuthorityError::ResourceLimit(resource));
    }
    Ok(())
}

fn validate_identity(identity: &BundleIdentity) -> Result<(), AuthorityError> {
    if !valid_identity(&identity.plugin_id) || !valid_package_id(&identity.package_id) || !valid_identity(&identity.version) {
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

fn validate_bundle(bundle: &Bundle, profile_id: &str) -> Result<SelectedTrustedBundleV1, AuthorityError> {
    if bundle.schema_version != 2 || bundle.packages.is_empty() || bundle.packages.len() > TRUSTED_CATALOG_MAX_PACKAGES || bundle.profiles.is_empty() || bundle.profiles.len() > TRUSTED_BUNDLE_MAX_PROFILES || !valid_identity(profile_id) {
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
        ensure_count(package.open_targets.len(), TRUSTED_CATALOG_MAX_OPEN_TARGETS, "trusted document-open target count")?;
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
        let mut open_target_keys = BTreeSet::new();
        for target in &package.open_targets {
            let expected_grant = BundleGrant { read: true, write: matches!(target.role, BundleOpenRole::Editor), observe: true };
            if !valid_open_identity(&target.artifact_kind)
                || !valid_open_identity(&target.artifact_schema)
                || !valid_open_identity(&target.surface_id)
                || !valid_open_identity(&target.app_id)
                || !valid_open_identity(&target.window_kind_id)
                || [&target.parent_dialect.artifact_kind, &target.parent_dialect.standard, &target.parent_dialect.subset].into_iter().any(|value| !valid_open_identity(value))
                || target.parent_dialect.artifact_kind != target.artifact_kind
                || target.grant != expected_grant
                || decode_digest(&target.pack_schema_hash, "open target pack schema hash")? == [0; 32]
                || !package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash)
                || !open_target_keys.insert((target.artifact_kind.as_str(), target.artifact_schema.as_str(), target.surface_id.as_str(), target.role as u8))
            {
                return Err(catalog("trusted document-open target is invalid, unbound, or duplicated"));
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
        if !valid_identity(&profile.id)
            || profile.selected_closure.is_empty()
            || profile.selected_closure.len() > TRUSTED_CATALOG_MAX_PACKAGES
            || !profiles.insert(profile.id.as_str())
            || decode_digest(&profile.selected_closure_sha256, "selected closure sha256")? == [0; 32]
            || decode_digest(&profile.generation_id, "profile generation id")? == [0; 32]
        {
            return Err(catalog("trusted bundle profile is empty, oversized, or duplicated"));
        }
        let mut prior_plugin = None;
        let mut closure = BTreeSet::new();
        for identity in &profile.selected_closure {
            validate_identity(identity)?;
            if prior_plugin.is_some_and(|prior: &str| prior >= identity.plugin_id.as_str()) {
                return Err(catalog("trusted bundle selected closure is not in canonical plugin order"));
            }
            prior_plugin = Some(identity.plugin_id.as_str());
            let index = *plugins.get(identity.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle selected closure is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != identity.package_id || expected.version != identity.version || !closure.insert(index) {
                return Err(catalog("trusted bundle selected closure conflicts with its package record"));
            }
        }
        if hex_lower(&selected_closure_digest(&profile.selected_closure)?) != profile.selected_closure_sha256 {
            return Err(catalog("trusted bundle selected closure digest differs"));
        }
        validate_identity(&profile.open_target.package)?;
        let target_index = *plugins.get(profile.open_target.package.plugin_id.as_str()).ok_or_else(|| catalog("trusted profile open target package is absent"))?;
        let target_package = &bundle.packages[target_index];
        if !closure.contains(&target_index) || target_package.package_id != profile.open_target.package.package_id || target_package.version != profile.open_target.package.version || !target_package.open_targets.contains(&profile.open_target.target)
        {
            return Err(catalog("trusted profile open target is outside its selected closure"));
        }
        if profile.id == "local-stdio-gis-open-v1" {
            let identities = profile.selected_closure.iter().map(|identity| (identity.plugin_id.as_str(), identity.package_id.as_str())).collect::<Vec<_>>();
            let target_count = bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>();
            let gis = bundle.packages.iter().find(|package| package.plugin_id == "gis");
            let stdio = bundle.packages.iter().find(|package| package.plugin_id == "stdio");
            let target = &profile.open_target.target;
            if identities != [("gis", "semio:gis"), ("stdio", "semio:stdio")]
                || bundle.packages.len() != 2
                || target_count != 1
                || gis.is_none_or(|package| {
                    package.native_codecs.len() != 2
                        || package.open_targets.len() != 1
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gismap" && codec.artifact_schema == "gis.map")
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gisterrain" && codec.artifact_schema == "gis.terrain")
                })
                || stdio.is_none_or(|package| package.native_codecs.len() != 26 || !package.open_targets.is_empty())
                || profile.open_target.package.plugin_id != "gis"
                || target.artifact_kind != "s.gis.gismap"
                || target.artifact_schema != "gis.map"
                || target.surface_id != "s.gis.gismap@1/*#editor"
                || target.app_id != "s.gis.gismap@1/*#editor"
                || target.window_kind_id != "gis2d-main"
                || target.parent_dialect != (semio_framework::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() })
                || target.role != BundleOpenRole::Editor
                || target.renderer_target != BundleRendererTarget::Wasm
                || target.grant != (BundleGrant { read: true, write: true, observe: true })
            {
                return Err(catalog("local stdio plus GIS profile is not its exact closed two-package map-editor authority"));
            }
        }
        if profile.id == profile_id {
            selected = Some((profile, closure));
        }
    }
    let (profile, closure) = selected.ok_or_else(|| catalog("selected trusted bundle profile is missing"))?;
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
    Ok(SelectedTrustedBundleV1 { package_indices: order, profile: profile.clone() })
}

fn validate_native_bindings(bindings: &[NativeCodecBinding]) -> Result<BTreeMap<CodecKey, &NativeCodecBinding>, AuthorityError> {
    ensure_count(bindings.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
    let mut map = BTreeMap::new();
    for binding in bindings {
        if !valid_identity(&binding.plugin_id) || !valid_package_id(&binding.package_id) || !valid_identity(&binding.artifact_kind) || !valid_identity(&binding.codec.schema) {
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
    if descriptor.descriptor_version != 1
        || !valid_package_id(&descriptor.package_id)
        || descriptor.package_id != record.package_id
        || !record.role.matches(descriptor.role)
        || descriptor.manifest.plugin_id != record.plugin_id
        || descriptor.manifest.version != record.version
        || descriptor.hashes.wasm_sha256 != record.component.sha256
    {
        return Err(catalog("decoded package descriptor identity does not exactly match its trust record"));
    }
    if decode_digest(&descriptor.hashes.core_wasm_sha256, "descriptor core wasm sha256")? == [0; 32] || decode_digest(&descriptor.hashes.descriptor_sha256, "descriptor metadata sha256")? == [0; 32] {
        return Err(catalog("decoded package descriptor hash metadata is zero"));
    }
    if descriptor.manifest.dependencies.len() != record.dependencies.len() {
        return Err(catalog("decoded manifest dependency count does not match the trust record"));
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
    reject_duplicate_descriptor_fields(&value)?;
    let descriptor: PackageDescriptor = from_dsl_value(value.clone()).map_err(catalog_error)?;
    let projection = to_dsl_value(&descriptor).map_err(catalog_error)?;
    let canonical = directory::os_store::pack_rt::encode_wire_value(&value);
    if canonical != bytes || directory::os_store::pack_rt::encode_wire_value(&projection) != canonical {
        return Err(catalog("package descriptor is not its exact canonical schema projection"));
    }
    Ok(descriptor)
}

fn reject_duplicate_descriptor_fields(value: &DslValue) -> Result<(), AuthorityError> {
    match value {
        DslValue::Array(items) => {
            for item in items {
                reject_duplicate_descriptor_fields(item)?;
            }
        }
        DslValue::Object(entries) => {
            let mut keys = BTreeSet::new();
            for (key, item) in entries {
                if !keys.insert(key.as_str()) {
                    return Err(catalog("package descriptor contains a duplicate object field"));
                }
                reject_duplicate_descriptor_fields(item)?;
            }
        }
        _ => {}
    }
    Ok(())
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
    #[cfg(feature = "native-artifact-execution")]
    use crate::artifact_authority::native_openable_provider::NativeCodecProviderSetV1;
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

    struct FixtureProviderSource<'a> {
        bindings: Vec<NativeCodecBinding>,
        calls: Mutex<Vec<String>>,
        failure: Option<&'static str>,
        cancel_after_preview: Option<&'a TestControl>,
        hostile: Option<&'static str>,
        exact_pool: bool,
    }

    impl FixtureProviderSource<'_> {
        fn new(bindings: Vec<NativeCodecBinding>) -> Self {
            Self { bindings, calls: Mutex::new(Vec::new()), failure: None, cancel_after_preview: None, hostile: None, exact_pool: true }
        }
    }

    impl NativeCodecProviderSourceV1 for FixtureProviderSource<'_> {
        fn preflight_selection(&self, selected: &[NativeCodecProviderRequirementV1<'_>]) -> Result<(), AuthorityError> {
            if self.exact_pool {
                let bindings = validate_native_bindings(&self.bindings)?;
                if bindings.keys().any(|key| {
                    !selected.iter().any(|required| required.package.plugin_id == key.plugin_id && required.package.package_id == key.package_id && required.artifact_kind == key.artifact_kind && required.artifact_schema == key.artifact_schema)
                }) {
                    return Err(catalog("no explicit native codec binding matches the selected closure"));
                }
            }
            Ok(())
        }

        fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &PackageDescriptor, _context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError> {
            assert_eq!(descriptor.package_id, package.package_id);
            assert_eq!(descriptor.manifest.plugin_id, package.plugin_id);
            assert_eq!(descriptor.manifest.version, package.version);
            self.calls.lock().expect("provider calls").push(package.package_id.to_owned());
            if self.failure == Some(package.package_id) {
                return Err(catalog("selected fixture provider failed"));
            }
            let mut bindings = self.bindings.iter().filter(|binding| binding.plugin_id == package.plugin_id && binding.package_id == package.package_id).cloned().collect::<Vec<_>>();
            if package.plugin_id == "fixture.editor" {
                match self.hostile {
                    Some("foreign") => bindings[0].package_id = "semio:unselected".into(),
                    Some("missing") => bindings.clear(),
                    Some("duplicate") => bindings.push(bindings[0].clone()),
                    Some("zero") => bindings[0].codec.pack_schema_hash = [0; 32],
                    Some("hash") => bindings[0].codec.pack_schema_hash = [0x22; 32],
                    Some("extra") => {
                        let mut extra = bindings[0].clone();
                        extra.artifact_kind = "fixture.extra".into();
                        bindings.push(extra);
                    }
                    _ => {}
                }
            }
            if let Some(control) = self.cancel_after_preview {
                control.cancelled.store(true, Ordering::SeqCst);
            }
            Ok(bindings)
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

        fn refresh_profile_generation(&mut self) {
            let bundle: Bundle = serde_json::from_value(self.bundle.clone()).expect("fixture bundle");
            self.bundle["profiles"][0]["selectedClosureSha256"] = hex_lower(&selected_closure_digest(&bundle.profiles[0].selected_closure).expect("fixture closure digest")).into();
            self.bundle["profiles"][0]["generationId"] = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("fixture generation").into();
        }

        fn component_path(&self, package: usize) -> PathBuf {
            self.root.join(self.bundle["packages"][package]["component"]["path"].as_str().expect("component path"))
        }

        fn binding(&self) -> NativeCodecBinding {
            NativeCodecBinding::new("fixture.editor", "semio:fixture-editor", "s.fixture.document", fixture_codec(&self.schema, [0x11; 32]))
        }

        fn rewrite_descriptor(&mut self, index: usize, schema: Option<&str>, dependency: Option<(&str, &str)>) {
            let record = &mut self.bundle["packages"][index];
            let bytes = descriptor_bytes(
                record["pluginId"].as_str().expect("plugin"),
                record["packageId"].as_str().expect("package"),
                record["version"].as_str().expect("version"),
                record["component"]["sha256"].as_str().expect("component hash"),
                schema,
                dependency,
            );
            record["descriptor"]["byteLength"] = bytes.len().into();
            record["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
            std::fs::write(self.root.join(record["descriptor"]["path"].as_str().expect("descriptor path")), bytes).expect("replace descriptor");
            self.refresh_profile_generation();
            self.persist_bundle();
        }

        fn make_two_codec_bindings(&mut self) -> Vec<NativeCodecBinding> {
            let schema = format!("{}.base", self.schema);
            self.bundle["packages"][1]["nativeCodecs"] = serde_json::json!([{ "artifactKind": "s.fixture.document", "artifactSchema": schema, "packSchemaHash": "11".repeat(32) }]);
            self.rewrite_descriptor(1, Some(&schema), None);
            vec![NativeCodecBinding::new("fixture.base", "semio:fixture-base", "s.fixture.document", fixture_codec(&schema, [0x11; 32])), self.binding()]
        }
    }

    impl Drop for FixtureDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn fixture_json() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️fixtures/👥️two-package/🔣️.json")).expect("trusted-catalog fixture")
    }

    fn local_stdio_gis_profile_bundle() -> Bundle {
        let version = "0.1.0".to_owned();
        let map_hash = "a1".repeat(32);
        let target = BundleOpenTarget {
            artifact_kind: "s.gis.gismap".into(),
            artifact_schema: "gis.map".into(),
            pack_schema_hash: map_hash.clone(),
            surface_id: "s.gis.gismap@1/*#editor".into(),
            app_id: "s.gis.gismap@1/*#editor".into(),
            window_kind_id: "gis2d-main".into(),
            role: BundleOpenRole::Editor,
            renderer_target: BundleRendererTarget::Wasm,
            parent_dialect: semio_framework::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() },
            grant: BundleGrant { read: true, write: true, observe: true },
        };
        let gis_identity = BundleIdentity { plugin_id: "gis".into(), package_id: "semio:gis".into(), version: version.clone() };
        let stdio_identity = BundleIdentity { plugin_id: "stdio".into(), package_id: "semio:stdio".into(), version: version.clone() };
        let stdio_codecs = (0u8..26).map(|index| BundleCodec { artifact_kind: format!("s.stdio.fixture{index:02}"), artifact_schema: format!("stdio.fixture{index:02}"), pack_schema_hash: format!("{:02x}", index + 1).repeat(32) }).collect();
        let packages = vec![
            BundlePackage {
                plugin_id: "gis".into(),
                package_id: "semio:gis".into(),
                version: version.clone(),
                role: BundleRole::Plugin,
                dependencies: vec![],
                component: BundleComponent { path: "packages/gis/component.wasm".into(), byte_length: 1, sha256: "11".repeat(32), blake3: "12".repeat(32) },
                descriptor: BundleFile { path: "packages/gis/descriptor.semio".into(), byte_length: 1, sha256: "13".repeat(32) },
                native_codecs: vec![
                    BundleCodec { artifact_kind: "s.gis.gismap".into(), artifact_schema: "gis.map".into(), pack_schema_hash: map_hash },
                    BundleCodec { artifact_kind: "s.gis.gisterrain".into(), artifact_schema: "gis.terrain".into(), pack_schema_hash: "a2".repeat(32) },
                ],
                open_targets: vec![target.clone()],
            },
            BundlePackage {
                plugin_id: "stdio".into(),
                package_id: "semio:stdio".into(),
                version: version.clone(),
                role: BundleRole::Plugin,
                dependencies: vec![],
                component: BundleComponent { path: "packages/stdio/component.wasm".into(), byte_length: 1, sha256: "21".repeat(32), blake3: "22".repeat(32) },
                descriptor: BundleFile { path: "packages/stdio/descriptor.semio".into(), byte_length: 1, sha256: "23".repeat(32) },
                native_codecs: stdio_codecs,
                open_targets: vec![],
            },
        ];
        let selected_closure = vec![gis_identity.clone(), stdio_identity];
        let mut bundle = Bundle {
            schema_version: 2,
            profiles: vec![BundleProfile { id: "local-stdio-gis-open-v1".into(), selected_closure, selected_closure_sha256: "01".repeat(32), open_target: BundleProfileOpenTarget { package: gis_identity, target }, generation_id: "02".repeat(32) }],
            packages,
        };
        bundle.profiles[0].selected_closure_sha256 = hex_lower(&selected_closure_digest(&bundle.profiles[0].selected_closure).expect("closure digest"));
        bundle.profiles[0].generation_id = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("profile generation");
        bundle
    }

    fn descriptor_bytes(plugin_id: &str, package_id: &str, version: &str, component_sha256: &str, schema: Option<&str>, dependency: Option<(&str, &str)>) -> Vec<u8> {
        let artifact_kinds = schema.map_or_else(Vec::new, |schema| {
            vec![serde_json::json!({
                "id": "s.fixture.document",
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
        let apps = schema.map_or_else(Vec::new, |_| {
            let manifest = semio_s_plugin_stdio::plugin().expect("stdio fixture app source").manifest;
            [semio_framework::AppRole::Editor, semio_framework::AppRole::Viewer]
                .into_iter()
                .map(|role| {
                    let suffix = format!("#{}", role.as_str());
                    let mut app = manifest.apps.iter().find(|app| app.id == format!("s.stdio.json@rfc8259/*{suffix}")).unwrap_or_else(|| panic!("stdio json {suffix} app")).clone();
                    app.dialect.artifact_kind = "s.fixture.document".into();
                    app.dialect.standard = "1".into();
                    app.dialect.subset = "*".into();
                    app.id = semio_framework::surface_app_id(&app.dialect, app.role);
                    app
                })
                .collect()
        });
        let dependencies = dependency.map_or_else(Vec::new, |(plugin_id, version)| vec![serde_json::json!({ "pluginId": plugin_id, "version": format!("={version}") })]);
        let json = serde_json::json!({
            "descriptorVersion": 1,
            "packageId": package_id,
            "role": "plugin",
            "manifest": {
                "pluginId": plugin_id,
                "label": plugin_id,
                "version": version,
                "apps": apps,
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
        os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("project descriptor"))
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
        for target in bundle["packages"][0]["openTargets"].as_array_mut().expect("open targets") {
            target["artifactSchema"] = schema.clone().into();
        }
        bundle["profiles"][0]["openTarget"]["target"]["artifactSchema"] = schema.clone().into();
        let component = [b'a', b'b', b'c'];
        for index in 0..2 {
            let component_path = root.join(bundle["packages"][index]["component"]["path"].as_str().expect("component path"));
            std::fs::write(component_path, component).expect("write component");
        }
        let root_descriptor = descriptor_bytes("fixture.editor", "semio:fixture-editor", "1.2.3", fixture["componentSha256"].as_str().expect("sha256"), Some(&schema), Some(("fixture.base", "1.0.0")));
        let base_descriptor = descriptor_bytes("fixture.base", "semio:fixture-base", "1.0.0", fixture["componentSha256"].as_str().expect("sha256"), None, None);
        for (index, bytes) in [(0, root_descriptor), (1, base_descriptor)] {
            bundle["packages"][index]["descriptor"]["byteLength"] = bytes.len().into();
            bundle["packages"][index]["descriptor"]["sha256"] = hex_lower(&Sha256::digest(&bytes)).into();
            let path = root.join(bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));
            std::fs::write(path, bytes).expect("write descriptor");
        }
        let bundle_path = root.join("trusted-catalog.json");
        let mut fixture = FixtureDirectory { root, bundle_path, bundle, schema };
        fixture.refresh_profile_generation();
        fixture.persist_bundle();
        fixture
    }

    /// 🧪️ Loads real GIS assembly metadata and native receipts around synthetic component bytes; component execution is outside this fixture.
    async fn prepared_gis_binding_fixture(viewer: bool, foreign_service: bool) -> FixtureDirectory {
        let runtime = semio_framework_plugin::plugin_runtime::PluginRuntime::new();
        semio_framework_plugin::plugin_runtime::install_plugin_bundle(&runtime, semio_s_plugin_gis::plugin().expect("GIS assembly"));
        let emitted = semio_framework_plugin::describe::describe_plugin(&runtime).await;
        let mut descriptor = decode_package_descriptor(&emitted).expect("actual native GIS descriptor");
        assert!(descriptor.manifest.dependencies.is_empty());
        let component = b"synthetic-gis-component-for-catalog-binding-test";
        let component_sha256 = hex_lower(&Sha256::digest(component));
        let mut component_blake3 = Hasher::new();
        component_blake3.update(component);
        descriptor.hashes.wasm_sha256 = component_sha256.clone();
        descriptor.hashes.core_wasm_sha256 = component_sha256.clone();
        descriptor.hashes.descriptor_sha256.clear();
        if foreign_service {
            descriptor.contributions.inference_services.iter_mut().find(|service| service.inference_schema == "s.gis.gismap.inference").expect("actual GIS inference declaration").contributor = "foreign".into();
        }
        descriptor.hashes.descriptor_sha256 = hex_lower(&Sha256::digest(&os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("GIS descriptor self-hash projection"))));
        let bytes = os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).expect("project GIS descriptor"));
        let native_codecs: Vec<_> = semio_s_plugin_gis::native_codecs::native_codec_factory_receipts()
            .expect("actual GIS codec receipts")
            .into_iter()
            .map(|receipt| {
                let identity = receipt.identity();
                serde_json::json!({ "artifactKind": identity.artifact_kind, "artifactSchema": identity.schema, "packSchemaHash": hex_lower(&identity.pack_schema_hash) })
            })
            .collect();
        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🧊️gis-map-frozen-binding-v1/🔣️.json")).expect("neutral frozen binding corpus");
        let binding = &corpus["binding"];
        let package = serde_json::json!({ "pluginId": descriptor.manifest.plugin_id, "packageId": descriptor.package_id, "version": descriptor.manifest.version });
        let mut target = serde_json::json!({
            "artifactKind": binding["artifact"]["kind"], "artifactSchema": binding["artifact"]["schema"],
            "packSchemaHash": native_codecs.iter().find(|codec| codec["artifactKind"] == binding["artifact"]["kind"]).expect("Map native receipt")["packSchemaHash"],
            "surfaceId": binding["surface"]["surfaceId"], "appId": binding["surface"]["appId"], "windowKindId": binding["surface"]["windowKindId"],
            "role": binding["surface"]["role"], "rendererTarget": binding["surface"]["rendererTarget"],
            "parentDialect": binding["parentDialect"], "grant": binding["grant"]
        });
        if viewer {
            let app = descriptor.manifest.apps.iter().find(|app| app.role == semio_framework::AppRole::Viewer && app.dialect.artifact_kind == "s.gis.gismap").expect("actual Map viewer");
            target["surfaceId"] = app.id.clone().into();
            target["appId"] = app.id.clone().into();
            target["windowKindId"] = app.window_kinds.first().expect("actual viewer window").id.clone().into();
            target["role"] = "viewer".into();
            target["grant"]["write"] = false.into();
        }
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::SeqCst);
        let root = PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("ticket-owned exact-law artifact directory")).join(format!("gis-binding-catalog-{}-{sequence}", std::process::id()));
        std::fs::create_dir(&root).expect("exclusive GIS binding fixture directory");
        std::fs::write(root.join("component.wasm"), component).expect("write synthetic GIS component");
        std::fs::write(root.join("descriptor.semio"), &bytes).expect("write actual GIS descriptor");
        let bundle = serde_json::json!({
            "schemaVersion": 2,
            "profiles": [{ "id": "frozen-gis-test", "selectedClosure": [package.clone()], "selectedClosureSha256": "01".repeat(32),
                "openTarget": { "package": package.clone(), "target": target.clone() }, "generationId": "02".repeat(32) }],
            "packages": [{ "pluginId": package["pluginId"], "packageId": package["packageId"], "version": package["version"], "role": "plugin", "dependencies": [],
                "component": { "path": "component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
                "descriptor": { "path": "descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
                "nativeCodecs": native_codecs, "openTargets": [target] }]
        });
        let mut fixture = FixtureDirectory { bundle_path: root.join("trusted-catalog.json"), root, bundle, schema: "gis.map".into() };
        fixture.refresh_profile_generation();
        fixture.persist_bundle();
        fixture
    }

    #[tokio::test]
    async fn gis_map_binding_constructs_from_loaded_catalog_and_retains_verified_bytes() {
        for (viewer, foreign_service) in [(false, false), (true, false), (false, true)] {
            let fixture = prepared_gis_binding_fixture(viewer, foreign_service).await;
            let control = TestControl::new();
            let catalog = Arc::new(TrustedCatalogLoader::load(&fixture.bundle_path, "frozen-gis-test", &NativeCodecProviderSetV1::linked(), &control.context()).await.expect("catalog loaded with real GIS receipts"));
            let result = crate::inference::verified_gis_map_binding(catalog.clone());
            if foreign_service {
                assert!(matches!(result, Err(crate::inference::InferenceErrorV1::Denied)));
            } else if viewer {
                assert!(result.expect("viewer profile is admissible").is_none());
            } else {
                let binding = result.expect("verified editor binding").expect("GIS Map editor is bound");
                assert!(Arc::ptr_eq(binding.catalog(), &catalog));
                assert_eq!(binding.selection(), catalog.selected_document_open().expect("sole selection"));
                assert_eq!(binding.service().executable_identity(), semio_s_plugin_gis::artifacts::gismap::gis_map_inference_service().executable_identity());
                let retained = catalog.packages()[0].component_bytes().to_vec();
                let digest = binding.digest().to_owned();
                std::fs::write(fixture.component_path(0), b"tampered").expect("mutate fixture backing component");
                assert!(TrustedCatalogLoader::load(&fixture.bundle_path, "frozen-gis-test", &NativeCodecProviderSetV1::linked(), &control.context()).await.is_err());
                drop(catalog);
                assert_eq!(binding.catalog().packages()[0].component_bytes(), retained);
                assert_eq!(binding.digest(), digest);
            }
        }
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
        ArtifactCodec { schema: schema.to_string(), extension: "fixture", pack_schema_hash, compile_dsl: fixture_compile, print_mirror: fixture_print, edit_text_from_envelope: fixture_edit, apply_ops_binary: fixture_apply }
    }

    async fn expect_load_error(fixture: &FixtureDirectory, bindings: &[NativeCodecBinding], control: &TestControl) -> AuthorityError {
        match load_fixture(fixture, bindings, &control.context()).await {
            Ok(_) => panic!("trusted catalog load unexpectedly succeeded"),
            Err(error) => error,
        }
    }

    async fn load_fixture(fixture: &FixtureDirectory, bindings: &[NativeCodecBinding], context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        TrustedCatalogLoader::load_selected(&fixture.bundle_path, "fixture", &FixtureProviderSource::new(bindings.to_vec()), context).await
    }

    #[tokio::test]
    async fn selected_native_providers_are_descriptor_verified_dependency_first_and_only_selected() {
        let mut single = prepared_fixture();
        single.bundle["packages"][0]["dependencies"] = serde_json::json!([]);
        single.bundle["profiles"][0]["selectedClosure"] = serde_json::json!([
            { "pluginId": "fixture.editor", "packageId": "semio:fixture-editor", "version": "1.2.3" }
        ]);
        single.rewrite_descriptor(0, Some(&single.schema.clone()), None);
        let unselected = NativeCodecBinding::new("unselected", "semio:unselected", "fixture.unselected", fixture_codec(&format!("{}.unselected", single.schema), [0x11; 32]));
        let mut source = FixtureProviderSource::new(vec![single.binding(), unselected]);
        source.exact_pool = false;
        let catalog = TrustedCatalogLoader::load_selected(&single.bundle_path, "fixture", &source, &TestControl::new().context()).await.expect("selected-only catalog");
        assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-editor"]);
        assert_eq!(catalog.packages().len(), 1);
        assert_eq!(catalog.codec_count(), 1);
        assert!(document_codec(&format!("{}.unselected", single.schema)).await.expect("registry").is_none());

        let mut pair = prepared_fixture();
        let source = FixtureProviderSource::new(pair.make_two_codec_bindings());
        let catalog = TrustedCatalogLoader::load_selected(&pair.bundle_path, "fixture", &source, &TestControl::new().context()).await.expect("complete selected closure");
        assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"]);
        assert_eq!(catalog.packages().len(), 2);
        assert_eq!(catalog.codec_count(), 2);
        assert!(document_codec(&pair.schema).await.expect("registry").is_some());
        assert!(document_codec(&format!("{}.base", pair.schema)).await.expect("registry").is_some());
    }

    #[tokio::test]
    async fn gis_native_provider_selection_binds_literal_owner_version_and_cancellation_without_publication() {
        struct SelectionControl {
            cancelled: bool,
            now_ms: u64,
        }
        impl AuthorityOperationControl for SelectionControl {
            fn now_ms(&self) -> u64 {
                self.now_ms
            }
            fn is_cancelled(&self) -> bool {
                self.cancelled
            }
            fn report(&self, _progress: AuthorityProgress) {}
        }
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../📇️native-openable-provider/🧪️fixtures/🌍️gis-v1/🔣️.json")).unwrap();
        let expected: serde_json::Value = serde_json::from_str(include_str!("../../../✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json")).unwrap();
        assert_eq!(fixture["packageVersion"], expected["packageVersion"]);
        let providers = NativeCodecProviderSetV1::linked();
        for case in fixture["cases"].as_array().unwrap() {
            let control = SelectionControl { cancelled: case["cancelled"].as_bool().unwrap(), now_ms: case["nowMs"].as_u64().unwrap() };
            let context = OperationContext::new(case["deadlineMs"].as_u64().unwrap(), AuthorityLimits::maximum(), &control);
            let result = providers.preview(case["pluginId"].as_str().unwrap(), case["packageId"].as_str().unwrap(), case["version"].as_str().unwrap(), &context);
            assert_eq!(result.is_ok(), case["accepted"].as_bool().unwrap(), "{}", case["name"]);
            if let Ok(bindings) = result {
                assert_eq!(bindings.len(), fixture["codecCount"].as_u64().unwrap() as usize);
                for (binding, row) in bindings.iter().zip(expected["receipts"].as_array().unwrap()) {
                    assert_eq!(binding.plugin_id, expected["pluginId"]);
                    assert_eq!(binding.package_id, expected["packageId"]);
                    assert_eq!(binding.artifact_kind, row["kind"]);
                    assert_eq!(binding.codec.schema, row["schema"]);
                    assert_eq!(binding.codec.extension, row["extension"]);
                    assert_eq!(hex_lower(&binding.codec.pack_schema_hash), row["protocolSha256"]);
                }
            }
            for row in expected["receipts"].as_array().unwrap() {
                assert!(document_codec(row["schema"].as_str().unwrap()).await.unwrap().is_none(), "{} must not publish even a partial codec closure", case["name"]);
            }
        }
    }

    #[tokio::test]
    async fn selected_native_provider_failure_substitution_and_conflict_publish_no_partial_closure() {
        for hostile in ["foreign", "missing", "duplicate", "zero", "hash", "extra", "provider-failure", "registry-conflict"] {
            let mut fixture = prepared_fixture();
            let mut source = FixtureProviderSource::new(fixture.make_two_codec_bindings());
            source.hostile = Some(hostile);
            if hostile == "provider-failure" {
                source.failure = Some("semio:fixture-editor");
            }
            if hostile == "registry-conflict" {
                os_store::register_document_codec(fixture_codec(&fixture.schema, [0x22; 32])).await.expect("prior immutable owner");
            }
            assert!(TrustedCatalogLoader::load_selected(&fixture.bundle_path, "fixture", &source, &TestControl::new().context()).await.is_err(), "{hostile}");
            assert_eq!(*source.calls.lock().expect("calls"), ["semio:fixture-base", "semio:fixture-editor"], "{hostile}");
            assert!(document_codec(&format!("{}.base", fixture.schema)).await.expect("registry").is_none(), "{hostile} published first provider");
            let existing = document_codec(&fixture.schema).await.expect("registry");
            if hostile == "registry-conflict" {
                assert_eq!(existing.expect("prior owner retained").pack_schema_hash, [0x22; 32]);
            } else {
                assert!(existing.is_none(), "{hostile} published second provider");
            }
        }
        let fixture = prepared_fixture();
        assert!(TrustedCatalogLoader::load(&fixture.bundle_path, "fixture", &NativeCodecProviderSetV1::linked(), &TestControl::new().context()).await.is_err());
        assert!(document_codec(&fixture.schema).await.expect("registry").is_none());
    }

    #[tokio::test]
    async fn selected_native_provider_descriptor_and_cancellation_fences_precede_publication() {
        let mut invalid = prepared_fixture();
        invalid.bundle["packages"][0]["dependencies"] = serde_json::json!([]);
        invalid.rewrite_descriptor(0, Some(&invalid.schema.clone()), None);
        let descriptor_path = invalid.root.join(invalid.bundle["packages"][0]["descriptor"]["path"].as_str().expect("descriptor path"));
        std::fs::write(descriptor_path, b"invalid descriptor").expect("hostile descriptor bytes");
        let source = FixtureProviderSource::new(vec![invalid.binding()]);
        assert!(TrustedCatalogLoader::load_selected(&invalid.bundle_path, "fixture", &source, &TestControl::new().context()).await.is_err());
        assert!(source.calls.lock().expect("calls").is_empty());
        assert!(document_codec(&invalid.schema).await.expect("registry").is_none());

        for before in [true, false] {
            let mut fixture = prepared_fixture();
            let control = TestControl::new();
            control.cancelled.store(before, Ordering::SeqCst);
            let mut source = FixtureProviderSource::new(fixture.make_two_codec_bindings());
            source.cancel_after_preview = Some(&control);
            let result = TrustedCatalogLoader::load_selected(&fixture.bundle_path, "fixture", &source, &control.context()).await;
            assert!(matches!(result, Err(AuthorityError::Cancelled)));
            let calls = source.calls.lock().expect("calls").clone();
            assert_eq!(calls, if before { Vec::<String>::new() } else { vec!["semio:fixture-base".into()] });
            assert!(document_codec(&fixture.schema).await.expect("registry").is_none());
            assert!(document_codec(&format!("{}.base", fixture.schema)).await.expect("registry").is_none());
        }
    }

    #[tokio::test]
    async fn neutral_fixture_proves_dependency_order_hash_oracles_and_exact_limit_edges() {
        let fixture = fixture_json();
        let bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle shape");
        assert_eq!(validate_bundle(&bundle, "fixture").expect("valid closure").package_indices, vec![1, 0]);
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
        let catalog = load_fixture(&fixture, &[binding], &control.context()).await.expect("verified catalog");
        assert_eq!(catalog.packages().iter().map(VerifiedTrustedPackage::plugin_id).collect::<Vec<_>>(), vec!["fixture.base", "fixture.editor"]);
        let editor = &catalog.packages()[1];
        assert_eq!(editor.package_ref().package.0, "semio:fixture-editor");
        assert_eq!(hex_lower(&editor.package_ref().hash.0), fixture_json()["componentBlake3"]);
        assert_ne!(editor.plugin_id(), editor.package_ref().package.0);
        assert_eq!(editor.component_bytes(), b"abc");
        assert_eq!(hex_lower(editor.component_sha256()), fixture_json()["componentSha256"]);
        assert_eq!(editor.descriptor().manifest.plugin_id, editor.plugin_id());
        assert_eq!(editor.descriptor().manifest.version, editor.version());
        assert_eq!(Sha256::digest(editor.descriptor_bytes()), *editor.descriptor_sha256());
        assert_eq!(catalog.codec_count(), 1);
        assert_eq!(catalog.open_target_count(), 1);
        assert_eq!(catalog.generation_id().len(), 64);
        let descriptor = DocumentDescriptor {
            space_id: "space".into(),
            document_id: "document".into(),
            artifact_kind: "s.fixture.document".into(),
            artifact_schema: fixture.schema.clone(),
            owner: directory::os_directory::DocumentOwner {
                plugin_id: "fixture.editor".into(),
                package_id: "semio:fixture-editor".into(),
                version: "1.2.3".into(),
                package_hash: fixture_json()["componentSha256"].as_str().expect("component sha256").into(),
            },
            pack_schema_hash: "11".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash: "33".repeat(32),
        };
        assert!(catalog.resolve(&TrustedArtifactIdentity::from_descriptor(&descriptor)).await.is_ok(), "the codec and open plan use the same descriptor SHA-256 owner identity");
        let editor = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("exact editor target");
        assert_eq!(editor.surface.role, DocumentOpenSurfaceRoleV1::Editor);
        assert!(editor.grant.write);
        assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), false).is_none());
        assert!(catalog.resolve_document_open(&descriptor, None, true).is_some());
        assert!(document_codec(&fixture.schema).await.expect("codec registry").is_some());
        let progress = control.progress.lock().expect("progress lock");
        assert_eq!(progress.first().map(|entry| entry.stage), Some(AuthorityProgressStage::Preflight));
        assert_eq!(progress.last().map(|entry| entry.stage), Some(AuthorityProgressStage::CatalogResolved));
        assert!(progress.iter().all(|entry| entry.completed_units <= entry.total_units));
    }

    /// 🧱 The exact-selection asset accessor is bound to the current generation and to every
    /// selected digest: it answers only for the exact descriptor/role/surface the catalog itself
    /// resolves, only while the caller-observed generation is still this catalog's own, and the
    /// bytes it returns are the very bytes whose SHA-256/BLAKE3 the selection projects.
    #[tokio::test]
    async fn selected_execution_target_assets_are_generation_and_digest_bound() {
        let fixture = prepared_fixture();
        let catalog = load_fixture(&fixture, &[fixture.binding()], &TestControl::new().context()).await.expect("verified catalog");
        let descriptor = DocumentDescriptor {
            space_id: "space".into(),
            document_id: "document".into(),
            artifact_kind: "s.fixture.document".into(),
            artifact_schema: fixture.schema.clone(),
            owner: directory::os_directory::DocumentOwner {
                plugin_id: "fixture.editor".into(),
                package_id: "semio:fixture-editor".into(),
                version: "1.2.3".into(),
                package_hash: fixture_json()["componentSha256"].as_str().expect("component sha256").into(),
            },
            pack_schema_hash: "11".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash: "33".repeat(32),
        };
        let generation = catalog.generation_id().to_string();
        let assets = catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), true, &generation).expect("selected assets");
        let selection = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("selection");
        assert_eq!(assets.selection, selection);
        assert!(!assets.component.is_empty() && !assets.descriptor.is_empty());
        assert_eq!(hex_lower(&Sha256::digest(&assets.component)), assets.selection.package.component_sha256);
        assert_eq!(semio_framework_hash::hash_bytes(&assets.component), assets.selection.package.component_blake3);
        assert_eq!(hex_lower(&Sha256::digest(&assets.descriptor)), assets.selection.package.descriptor_byte_sha256);
        assert!(assets.component.len() as u64 <= TRUSTED_COMPONENT_MAX_BYTES && assets.descriptor.len() as u64 <= TRUSTED_DESCRIPTOR_MAX_BYTES);
        // 🔁 A rotated (or merely guessed) generation is never served, and no role, surface or
        // descriptor substitution reaches bytes.
        assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), true, &"ab".repeat(32)).is_none());
        assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#editor"), false, &generation).is_none());
        assert!(catalog.assets_for_current_selection(&descriptor, Some("s.fixture.document@1/*#viewer"), true, &generation).is_none());
        assert!(catalog.assets_for_current_selection(&descriptor, Some("foreign"), true, &generation).is_none());
        for change in ["plugin", "package", "version", "hash", "kind", "schema", "pack"] {
            let mut candidate = descriptor.clone();
            match change {
                "plugin" => candidate.owner.plugin_id.push_str(".foreign"),
                "package" => candidate.owner.package_id.push_str(".foreign"),
                "version" => candidate.owner.version.push_str("-foreign"),
                "hash" => candidate.owner.package_hash = "ab".repeat(32),
                "kind" => candidate.artifact_kind.push_str(".foreign"),
                "schema" => candidate.artifact_schema.push_str("-foreign"),
                "pack" => candidate.pack_schema_hash = "ab".repeat(32),
                _ => unreachable!(),
            }
            assert!(catalog.assets_for_current_selection(&candidate, Some("s.fixture.document@1/*#editor"), true, &generation).is_none(), "descriptor {change} reached selected bytes");
        }
    }

    #[tokio::test]
    async fn verified_trusted_catalog_document_open_generation_and_resolution_are_exact() {
        let fixture = prepared_fixture();
        let catalog = load_fixture(&fixture, &[fixture.binding()], &TestControl::new().context()).await.expect("verified catalog");
        let descriptor = DocumentDescriptor {
            space_id: "space".into(),
            document_id: "document".into(),
            artifact_kind: "s.fixture.document".into(),
            artifact_schema: fixture.schema.clone(),
            owner: directory::os_directory::DocumentOwner {
                plugin_id: "fixture.editor".into(),
                package_id: "semio:fixture-editor".into(),
                version: "1.2.3".into(),
                package_hash: fixture_json()["componentSha256"].as_str().expect("component sha256").into(),
            },
            pack_schema_hash: "11".repeat(32),
            bootstrap_version: 1,
            bootstrap_frontier: directory::os_directory::DocumentFrontier { head_seq: 0, commit_seq: 0, epoch: 0 },
            bootstrap_snapshot_hash: "33".repeat(32),
        };
        assert_eq!(catalog.open_target_count(), 1);
        assert_eq!(catalog.generation_id().len(), 64);
        assert!(catalog.generation_id().bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')));
        let editor = catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#editor"), true).expect("editor");
        assert_eq!(editor.surface.role, DocumentOpenSurfaceRoleV1::Editor);
        assert_eq!(editor.grant, DocumentOpenGrantV1 { read: true, write: true, observe: true });
        assert_eq!(editor.parent_dialect, semio_framework::ArtifactDialect { artifact_kind: "s.fixture.document".into(), standard: "1".into(), subset: "*".into() });
        assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), false).is_none());
        for field in ["artifactKind", "standard", "subset"] {
            let mut changed: Bundle = serde_json::from_value(fixture.bundle.clone()).expect("bundle");
            let dialect = &mut changed.profiles[0].open_target.target.parent_dialect;
            match field {
                "artifactKind" => dialect.artifact_kind.push_str(".foreign"),
                "standard" => dialect.standard.push_str("-foreign"),
                "subset" => dialect.subset.push_str("-foreign"),
                _ => unreachable!(),
            }
            let changed_generation = trusted_profile_generation(&changed, &changed.profiles[0]).unwrap();
            assert_ne!(changed_generation, catalog.generation_id(), "catalog binds parent {field}");
        }
        eprintln!("[DEBUG] trusted open catalog retained verified editor/viewer parent dialect;3 field changes alter generation");
        assert!(catalog.resolve_document_open(&descriptor, Some("s.fixture.document@1/*#viewer"), true).is_none());
        assert!(catalog.resolve_document_open(&descriptor, Some("foreign"), false).is_none());
        let roles: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🪪️identity-roles/🔣️.json")).unwrap();
        for case in roles["cases"].as_array().unwrap() {
            let mut candidate = descriptor.clone();
            let mut surface = "s.fixture.document@1/*#editor".to_owned();
            match case["change"].as_str().unwrap() {
                "none" => {}
                "blake3-owner" => candidate.owner.package_hash = fixture_json()["componentBlake3"].as_str().unwrap().to_owned(),
                "descriptor-owner" => candidate.owner.package_hash = hex_lower(catalog.packages()[1].descriptor_sha256()),
                "zero-owner" => candidate.owner.package_hash = "00".repeat(32),
                "bare-kind" => candidate.artifact_kind = "fixture.document".to_owned(),
                "bare-surface" => surface = "fixture.document@1/*#editor".to_owned(),
                _ => panic!("unknown catalog identity role vector"),
            }
            assert_eq!(catalog.resolve(&TrustedArtifactIdentity::from_descriptor(&candidate)).await.is_ok(), case["codec"].as_bool().unwrap(), "codec {}", case["change"]);
            assert_eq!(catalog.resolve_document_open(&candidate, Some(&surface), true).is_some(), case["open"].as_bool().unwrap(), "open {}", case["change"]);
        }
    }

    #[test]
    fn descriptor_projection_rejects_package_conflicts_unknown_fields_and_duplicate_fields() {
        let fixture = fixture_json();
        let bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        let component_sha256 = fixture["componentSha256"].as_str().expect("sha256");
        let canonical = descriptor_bytes("fixture.editor", "semio:fixture-editor", "1.2.3", component_sha256, Some("fixture.document@1"), Some(("fixture.base", "1.0.0")));
        decode_package_descriptor(&canonical).unwrap_or_else(|error| panic!("canonical descriptor must decode: {error}"));

        let conflicting =
            decode_package_descriptor(&descriptor_bytes("fixture.editor", "semio:other-package", "1.2.3", component_sha256, Some("fixture.document@1"), Some(("fixture.base", "1.0.0")))).expect("structurally valid conflicting descriptor");
        assert!(validate_descriptor(&bundle.packages[0], &conflicting, &bundle.packages).expect_err("package mismatch").to_string().contains("identity"));

        let mut unknown = os_store::pack_rt::decode_wire_value(&canonical).expect("canonical value");
        let DslValue::Object(fields) = &mut unknown else { panic!("descriptor object") };
        fields.push(("unexpected".to_owned(), DslValue::Null));
        let error = decode_package_descriptor(&os_store::pack_rt::encode_wire_value(&unknown)).expect_err("unknown descriptor field");
        assert!(error.to_string().contains("unknown field") || error.to_string().contains("canonical schema projection"));

        let mut duplicate = os_store::pack_rt::decode_wire_value(&canonical).expect("canonical value");
        let DslValue::Object(fields) = &mut duplicate else { panic!("descriptor object") };
        fields.push(("packageId".to_owned(), DslValue::String("semio:fixture-editor".to_owned())));
        assert!(decode_package_descriptor(&os_store::pack_rt::encode_wire_value(&duplicate)).expect_err("duplicate descriptor field").to_string().contains("duplicate object field"));
        assert!(valid_package_id("semio:fixture-editor"));
        assert!(!valid_package_id("fixture.editor.native"));
        assert!(!valid_package_id("semio:Fixture"));
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
        let lossy = NativeCodecBinding::new("fixture.editor", "semio:fixture-wrong", "s.fixture.document", fixture_codec(&wrong_package.schema, [0x11; 32]));
        let error = expect_load_error(&wrong_package, &[lossy], &control).await;
        assert!(error.to_string().contains("no explicit native codec"));
        assert!(document_codec(&wrong_package.schema).await.expect("codec registry").is_none());

        let mut zero = prepared_fixture();
        zero.bundle["packages"][0]["nativeCodecs"][0]["packSchemaHash"] = "00".repeat(32).into();
        zero.persist_bundle();
        let error = expect_load_error(&zero, &[zero.binding()], &control).await;
        assert!(error.to_string().contains("zero"));
        assert!(document_codec(&zero.schema).await.expect("codec registry").is_none());

        let mut detached_open_target = prepared_fixture();
        detached_open_target.bundle["packages"][0]["openTargets"][0]["packSchemaHash"] = "12".repeat(32).into();
        detached_open_target.persist_bundle();
        let error = expect_load_error(&detached_open_target, &[detached_open_target.binding()], &control).await;
        assert!(error.to_string().contains("open target"));
        assert!(document_codec(&detached_open_target.schema).await.expect("codec registry").is_none());

        let mismatch = prepared_fixture();
        let binding = NativeCodecBinding::new("fixture.editor", "semio:fixture-editor", "s.fixture.document", fixture_codec(&mismatch.schema, [0x12; 32]));
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

    #[tokio::test]
    async fn descriptor_owned_surface_is_required_before_any_catalog_or_codec_publication() {
        for (field, value) in [
            ("artifactKind", serde_json::json!("fixture.document")),
            ("surfaceId", serde_json::json!("s.fixture.document@1/*#foreign")),
            ("appId", serde_json::json!("s.fixture.document@1/*#foreign")),
            ("windowKindId", serde_json::json!("foreign.window")),
            ("role", serde_json::json!("viewer")),
            ("rendererTarget", serde_json::json!("wgpu")),
        ] {
            let mut fixture = prepared_fixture();
            fixture.bundle["packages"][0]["openTargets"][0][field] = value;
            fixture.persist_bundle();
            let error = expect_load_error(&fixture, &[fixture.binding()], &TestControl::new()).await;
            assert!(error.to_string().contains("document-open target"), "{field}: {error}");
            assert!(document_codec(&fixture.schema).await.expect("codec registry").is_none(), "{field} published a codec");
        }
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
        bundle.profiles[0].selected_closure[0].version = "9.9.9".to_string();
        assert!(validate_bundle(&bundle, "fixture").expect_err("conflicting closure identity").to_string().contains("conflicts"));

        let mut bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        bundle.packages[0].component.path = "../escape.wasm".to_string();
        let control = TestControl::new();
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        let error = runtime.block_on(contained_path(Path::new("."), &bundle.packages[0].component.path)).expect_err("escaping path");
        assert!(error.to_string().contains("relative path"));
    }

    #[test]
    fn trusted_profile_generation_binds_zero_target_package_and_every_codec_row() {
        let fixture = fixture_json();
        let bundle: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("bundle");
        let original = trusted_profile_generation(&bundle, &bundle.profiles[0]).expect("generation");

        let mut component: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("component bundle");
        component.packages[1].component.sha256 = "31".repeat(32);
        assert_ne!(trusted_profile_generation(&component, &component.profiles[0]).expect("component generation"), original, "zero-target component SHA-256 must rotate the profile");

        let mut descriptor: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("descriptor bundle");
        descriptor.packages[1].descriptor.sha256 = "32".repeat(32);
        assert_ne!(trusted_profile_generation(&descriptor, &descriptor.profiles[0]).expect("descriptor generation"), original, "zero-target descriptor SHA-256 must rotate the profile");

        let mut codec: Bundle = serde_json::from_value(fixture["bundle"].clone()).expect("codec bundle");
        codec.packages[0].native_codecs[0].pack_schema_hash = "33".repeat(32);
        assert_ne!(trusted_profile_generation(&codec, &codec.profiles[0]).expect("codec generation"), original, "every codec row must rotate the profile");
    }

    #[test]
    fn local_stdio_gis_profile_is_exact_two_packages_twenty_eight_codecs_and_one_map_target() {
        let bundle = local_stdio_gis_profile_bundle();
        let selected = validate_bundle(&bundle, "local-stdio-gis-open-v1").expect("closed stdio+GIS profile");
        assert_eq!(selected.package_indices.len(), 2);
        assert_eq!(bundle.packages.iter().map(|package| package.native_codecs.len()).sum::<usize>(), 28);
        assert_eq!(bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>(), 1);

        let mut missing_terrain = local_stdio_gis_profile_bundle();
        missing_terrain.packages[0].native_codecs.pop();
        assert!(validate_bundle(&missing_terrain, "local-stdio-gis-open-v1").expect_err("missing Terrain").to_string().contains("exact closed"));

        let mut terrain_target = local_stdio_gis_profile_bundle();
        let mut target = terrain_target.packages[0].open_targets[0].clone();
        target.artifact_kind = "s.gis.gisterrain".into();
        target.artifact_schema = "gis.terrain".into();
        target.pack_schema_hash = "a2".repeat(32);
        target.surface_id = "s.gis.gisterrain@1/*#editor".into();
        target.app_id = target.surface_id.clone();
        target.parent_dialect.artifact_kind = target.artifact_kind.clone();
        terrain_target.packages[0].open_targets.push(target);
        assert!(validate_bundle(&terrain_target, "local-stdio-gis-open-v1").expect_err("Terrain target").to_string().contains("exact closed"));

        let mut reordered = local_stdio_gis_profile_bundle();
        reordered.profiles[0].selected_closure.reverse();
        assert!(validate_bundle(&reordered, "local-stdio-gis-open-v1").expect_err("noncanonical closure").to_string().contains("canonical"));
    }
}
