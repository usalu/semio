//! 🗂️ Immutable trusted-catalog bundle verification for headless hub authority startup.

use super::adapters::{bounded_message, AUTHORITY_MAX_CODEC_TEXT_BYTES, TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_PACKAGES};
use super::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactGenesisCodec, TrustedArtifactIdentity};
use directory::os_directory::{hex_lower, DocumentDescriptor, DocumentExecutionProtocolV1, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use directory::os_store::{self, ArtifactCodec};
use semio_framework::{from_dsl_value, to_dsl_value, DslValue, PackageDescriptor, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
use schema::{publication_revision, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedCatalogCurrentPointerV1, TrustedCatalogPublicationCommandV1, TrustedCatalogPublicationReceiptV1, TRUSTED_CATALOG_PUBLICATION_MAX_BYTES, TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED, TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, TRUSTED_CATALOG_PUBLICATION_SCHEMA};

#[path = "🌐️browser-actor/🦀️.rs"]
mod browser_actor;
#[path = "🛡️opened-root/🦀️.rs"]
mod opened_root;
use opened_root::{TrustedCatalogDataRoot, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1};
#[cfg(all(test, unix))]
use opened_root::create_fifo_fixture;
use directory::os_directory::schema::{DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES};

/// 🧯️ Maximum accepted serialized bundle bytes.
pub const TRUSTED_BUNDLE_MAX_BYTES: u64 = 4 * 1024 * 1024;
/// 🧯️ Maximum accepted committed package-descriptor bytes.
pub const TRUSTED_DESCRIPTOR_MAX_BYTES: u64 = 4 * 1024 * 1024;
/// 🧮️ Maximum logical owned storage admitted before descriptor schema/canonical projection.
pub const TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION: u64 = 32 * 1024 * 1024;
/// 🧯️ Maximum accepted bytes for one retained component.
pub const TRUSTED_COMPONENT_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// 🧯️ Maximum retained component bytes across one selected closure.
pub const TRUSTED_COMPONENT_CLOSURE_MAX_BYTES: u64 = 512 * 1024 * 1024;
/// 🧯️ Maximum retained descriptor bytes across one selected closure.
pub const TRUSTED_DESCRIPTOR_CLOSURE_MAX_BYTES: u64 = 64 * 1024 * 1024;
/// 🌐️ Maximum retained actor bodies across the selected package closure.
pub const TRUSTED_BROWSER_ACTOR_CLOSURE_MAX_BYTES: u64 = 128 * 1024 * 1024;
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

/// 🔐️ Publishes only a fully verified selection while holding the cooperating-writer OS fence.
pub struct TrustedCatalogPublisher;

/// 🚦️ Requires callers to distinguish durable selection from reconciliation-only visibility.
pub enum TrustedCatalogPublicationOutcome {
    Durable(Vec<u8>),
    Unconfirmed(Vec<u8>),
}

impl TrustedCatalogPublisher {
    /// 📤️ Consumes the closed command schema under a private server-owned catalog namespace.
    pub async fn publish_current(data_path: &Path, command_bytes: &[u8], providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<TrustedCatalogPublicationOutcome, AuthorityError> {
        context.checkpoint()?;
        if command_bytes.is_empty() || command_bytes.len() > TRUSTED_CATALOG_PUBLICATION_MAX_BYTES { return Err(catalog("trusted publication command exceeds its bound")); }
        let fields: serde_json::Value = serde_json::from_slice(command_bytes).map_err(catalog_error)?;
        if !fields.as_object().is_some_and(|object| object.contains_key("expectedCurrentSha256")) { return Err(catalog("trusted publication expected current token is required")); }
        let command: TrustedCatalogPublicationCommandV1 = serde_json::from_slice(command_bytes).map_err(catalog_error)?;
        if command.schema != TRUSTED_CATALOG_PUBLICATION_SCHEMA || command.request_id.len() != 32 || !command.request_id.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) || command.profile_id.is_empty() || command.profile_id.len() > TRUSTED_IDENTITY_MAX_BYTES || command.profile_id.chars().any(char::is_control) {
            return Err(catalog("trusted publication command has invalid identity"));
        }
        decode_digest(&command.generation_id, "trusted generation id")?;
        let bundle_digest = decode_digest(&command.bundle_sha256, "trusted bundle sha256")?;
        if let Some(expected) = &command.expected_current_sha256 { decode_digest(expected, "trusted expected current sha256")?; }
        let data = TrustedCatalogDataRoot::open_server_owned(data_path)?;
        let owner = data.acquire_publication(context).await?;
        let (observed, revision) = match owner.open_current()? {
            Some(file) => {
                let bytes = file.read_bounded(65_536, context).await?;
                let current = TrustedCatalogCurrentPointerV1::decode(&bytes)?;
                (Some(hex_lower(&sha256(&bytes, context).await?)), publication_revision(&current.publication_revision)?)
            }
            None => (None, 0),
        };
        if observed != command.expected_current_sha256 { return Err(catalog("trusted publication current token is stale")); }
        let revision = revision.checked_add(1).ok_or_else(|| catalog("trusted publication revision exhausted"))?;
        let generation = owner.open_generation(&command.generation_id)?;
        let relative = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation.read_regular(&relative, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != bundle_digest { return Err(catalog("trusted publication candidate bundle differs from its digest")); }
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let (verified, _) = TrustedCatalogLoader::verify_selected(&generation, relative, bundle_bytes, &command.profile_id, providers, context).await?;
        if verified.generation_id() != command.generation_id { return Err(catalog("trusted publication candidate generation differs from the verified profile")); }
        for package in &verified.packages {
            let record = bundle.packages.iter().find(|record| record.plugin_id == package.plugin_id && record.package_id == package.package.package.0 && record.version == package.version).ok_or_else(|| catalog("verified publication package lost its bundle record"))?;
            let mut files = vec![TrustedBundleFileV1 { path: record.component.path.clone(), byte_length: record.component.byte_length, sha256: record.component.sha256.clone() }, record.descriptor.clone()];
            if let Some(actor) = record.browser_actor.file() { files.push(actor); }
            for file in files {
                let bytes = generation.read_regular(&TrustedCatalogRelativePathV1::parse(&file.path)?, file.byte_length, context).await?;
                if bytes.len() as u64 != file.byte_length || sha256(&bytes, context).await? != decode_digest(&file.sha256, "trusted publication leaf sha256")? { return Err(catalog("trusted publication leaf changed after candidate verification")); }
            }
        }
        let final_bundle = generation.read_regular(&TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&final_bundle, context).await? != bundle_digest { return Err(catalog("trusted publication bundle changed after candidate verification")); }
        let pointer = TrustedCatalogCurrentPointerV1 { profile_id: command.profile_id, generation_id: command.generation_id, bundle_sha256: command.bundle_sha256, publication_revision: revision.to_string() };
        let current_bytes = pointer.encode()?;
        let current_sha256 = hex_lower(&sha256(&current_bytes, context).await?);
        let sync = owner.replace_current(&command.request_id, &current_bytes, context)?;
        let receipt = TrustedCatalogPublicationReceiptV1 { schema: TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, request_id: command.request_id, profile_id: pointer.profile_id, generation_id: pointer.generation_id, bundle_sha256: pointer.bundle_sha256, publication_revision: pointer.publication_revision, current_sha256, outcome: match sync { opened_root::TrustedPublicationSync::Durable => TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, opened_root::TrustedPublicationSync::Unconfirmed => TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED } };
        let bytes = serde_json::to_vec(&receipt).map_err(catalog_error)?;
        Ok(match sync { opened_root::TrustedPublicationSync::Durable => TrustedCatalogPublicationOutcome::Durable(bytes), opened_root::TrustedPublicationSync::Unconfirmed => TrustedCatalogPublicationOutcome::Unconfirmed(bytes) })
    }
}

#[derive(Debug)]
struct SelectedTrustedBundleV1 {
    package_indices: Vec<usize>,
    profile: TrustedBundleProfileV1,
}

/// 🔗️ One explicitly linked native executable for an exact bundle artifact identity.
#[derive(Clone)]
pub struct NativeCodecBinding {
    plugin_id: String,
    package_id: String,
    artifact_kind: String,
    codec: ArtifactCodec,
    genesis: Option<semio_framework_plugin::NativeArtifactGenesisFactoryV1>,
}

impl NativeCodecBinding {
    /// 🪢️ Binds a native executable without deriving package identity from plugin identity.
    pub fn new(plugin_id: impl Into<String>, package_id: impl Into<String>, artifact_kind: impl Into<String>, codec: ArtifactCodec) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package_id.into(), artifact_kind: artifact_kind.into(), codec, genesis: None }
    }

    /// 🌱️ Binds an exact package-owned editor genesis factory to the same native codec identity.
    pub fn with_genesis(
        plugin_id: impl Into<String>,
        package_id: impl Into<String>,
        artifact_kind: impl Into<String>,
        codec: ArtifactCodec,
        genesis: semio_framework_plugin::NativeArtifactGenesisFactoryV1,
    ) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package_id.into(), artifact_kind: artifact_kind.into(), codec, genesis: Some(genesis) }
    }

    pub(super) fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    pub(super) fn package_id(&self) -> &str {
        &self.package_id
    }

    pub(super) fn artifact_kind(&self) -> &str {
        &self.artifact_kind
    }

    pub(super) fn codec(&self) -> &ArtifactCodec {
        &self.codec
    }

    /// 🌱️ Reports whether this exact native binding also carries package-owned creation authority.
    pub(super) fn has_genesis(&self) -> bool {
        self.genesis.is_some()
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
    browser_actor: DocumentOpenBrowserActorV1,
    browser_actor_bytes: Option<Arc<[u8]>>,
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
    genesis: Option<semio_framework_plugin::NativeArtifactGenesisFactoryV1>,
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

impl TrustedArtifactGenesisCodec for VerifiedNativeArtifactCodec {
    async fn initial_pair(&self, document_id: &str, dialect: &directory::os_io::ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let genesis = self.genesis.ok_or_else(|| AuthorityError::Catalog("selected native artifact codec has no package-owned genesis factory".into()))?;
        let files = genesis(document_id, dialect).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        context.checkpoint()?;
        Ok(ArtifactPair { pack: files.pack, spr: files.spr })
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
    pub browser_actor: Option<Arc<[u8]>>,
}

/// 🧬 One exact document-open choice retained only after the complete catalog verifies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedDocumentOpenSelectionV1 {
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: semio_framework::ArtifactDialect,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
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

    /// 🌱️ Creation resolves one unambiguous writable target in this exact admitted generation.
    pub(crate) fn artifact_creation_selection(&self, kind_id: &str) -> Option<&VerifiedDocumentOpenSelectionV1> {
        let mut matches = self.open_targets.iter().filter(|selection| {
            selection.artifact.kind == kind_id
                && selection.surface.role == DocumentOpenSurfaceRoleV1::Editor
                && self.codecs.iter().any(|codec| {
                    codec.genesis.is_some()
                        && codec.identity.plugin_id == selection.package.plugin_id
                        && codec.identity.package_id == selection.package.package_id
                        && codec.identity.version == selection.package.version
                        && codec.identity.package_hash == selection.package.component_sha256
                        && codec.identity.artifact_kind == selection.artifact.kind
                        && codec.identity.artifact_schema == selection.artifact.schema
                        && codec.identity.pack_schema_hash == selection.artifact.pack_schema_hash
                })
        });
        let selected = matches.next()?;
        matches.next().is_none().then_some(selected)
    }

    /// 🗣️ Projects only unambiguous factory-backed choices from retained compiled descriptors.
    pub(crate) fn artifact_creation_catalog(&self, space_id: &str) -> Option<directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationCatalogV1> {
        use directory::os_directory::schema::space_artifact_creation::{
            SpaceArtifactCreationCatalogV1, SpaceArtifactCreationDialectV1, SpaceArtifactCreationKindV1, SpaceArtifactCreationLabelV1,
        };
        let kind_ids = self.open_targets.iter().map(|selection| selection.artifact.kind.as_str()).collect::<BTreeSet<_>>();
        let mut kinds = Vec::new();
        for kind_id in kind_ids {
            let Some(selection) = self.artifact_creation_selection(kind_id) else { continue };
            let retained = self.packages.iter().find(|package| {
                package.plugin_id == selection.package.plugin_id
                    && package.package.package.0 == selection.package.package_id
                    && package.version == selection.package.version
                    && hex_lower(&package.component_sha256) == selection.package.component_sha256
            })?;
            let app = retained.descriptor.manifest.apps.iter().find(|app| app.id == selection.surface.app_id && app.dialect == selection.parent_dialect)?;
            kinds.push(SpaceArtifactCreationKindV1 {
                kind_id: selection.artifact.kind.clone(),
                schema: selection.artifact.schema.clone(),
                dialect: SpaceArtifactCreationDialectV1 {
                    artifact_kind: selection.parent_dialect.artifact_kind.clone(),
                    standard: selection.parent_dialect.standard.clone(),
                    subset: selection.parent_dialect.subset.clone(),
                },
                label: SpaceArtifactCreationLabelV1 {
                    en: app.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::En).to_string(),
                    de: app.label.resolve(semio_framework::Terminology::Native, semio_framework::Locale::De).to_string(),
                },
            });
        }
        kinds.sort_by(|left, right| left.kind_id.cmp(&right.kind_id));
        let catalog = SpaceArtifactCreationCatalogV1 { schema: "semio.hub.space-artifact-creation-catalog/v1".into(), space_id: space_id.into(), catalog_generation_id: self.generation_id.clone(), kinds };
        catalog.validate().then_some(catalog)
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
        let browser_actor = match (&selection.browser_actor, &package.browser_actor, &package.browser_actor_bytes) {
            (DocumentOpenBrowserActorV1::None, _, _) if !matches!(selection.surface.renderer_target, DocumentOpenRendererTargetV1::Wasm) => None,
            (selected, retained, Some(bytes)) if selected == retained && matches!(selected, DocumentOpenBrowserActorV1::ClosedBrowserActor { .. }) && !bytes.is_empty() && bytes.len() as u64 <= DOCUMENT_BROWSER_ACTOR_MAX_BYTES => {
                Some(Arc::clone(bytes))
            }
            _ => return None,
        };
        Some(VerifiedExecutionTargetAssets { selection, component: Arc::clone(&package.component_bytes), descriptor: Arc::clone(&package.descriptor_bytes), browser_actor })
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
    /// 🛡️ Opens the current immutable generation beneath one server-owned Hub data root.
    pub async fn load_current(data_path: &Path, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<Option<VerifiedTrustedCatalog>, AuthorityError> {
        if !data_path.try_exists().map_err(catalog_error)? {
            return Ok(None);
        }
        let data_root = TrustedCatalogDataRoot::open_server_owned(data_path)?;
        let Some(current_file) = data_root.open_current()? else {
            return Ok(None);
        };
        let current_bytes = current_file.read_bounded(64 * 1024, context).await?;
        let current = TrustedCatalogCurrentPointerV1::decode(&current_bytes)?;
        let expected_bundle_sha256 = decode_digest(&current.bundle_sha256, "trusted bundle sha256")?;
        let generation_root = data_root.open_generation(&current.generation_id)?;
        let bundle_path = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != expected_bundle_sha256 {
            return Err(catalog("trusted bundle differs from the current pointer digest"));
        }
        let verified = Self::load_selected(&generation_root, bundle_path, bundle_bytes, &current.profile_id, providers, context).await?;
        if verified.generation_id() != current.generation_id {
            return Err(catalog("trusted current pointer generation differs from the selected profile"));
        }
        Ok(Some(verified))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) async fn load_fixture(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let path = std::fs::canonicalize(bundle_path).map_err(catalog_error)?;
        let fixture_root = path.parent().ok_or_else(|| catalog("bundle has no containing directory"))?;
        let generation_root = TrustedCatalogGenerationRoot::open_fixture_owned(fixture_root)?;
        let bundle_path = TrustedCatalogRelativePathV1::parse(path.file_name().and_then(|name| name.to_str()).ok_or_else(|| catalog("fixture bundle name is not UTF-8"))?)?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        Self::load_selected(&generation_root, bundle_path, bundle_bytes, profile_id, providers, context).await
    }

    async fn load_selected(root: &TrustedCatalogGenerationRoot, bundle_path: TrustedCatalogRelativePathV1, bundle_bytes: Vec<u8>, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let (catalog, registration_codecs) = Self::verify_selected(root, bundle_path, bundle_bytes, profile_id, providers, context).await?;
        let assembly = os_store::begin_artifact_assembly().map_err(catalog_error)?;
        os_store::preflight_document_codecs_in_assembly(&assembly, &registration_codecs).map_err(catalog_error)?;
        context.checkpoint()?;
        os_store::register_document_codecs_in_assembly(&assembly, registration_codecs).map_err(catalog_error)?;
        Ok(catalog)
    }

    async fn verify_selected(root: &TrustedCatalogGenerationRoot, bundle_path: TrustedCatalogRelativePathV1, bundle_bytes: Vec<u8>, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<(VerifiedTrustedCatalog, Vec<ArtifactCodec>), AuthorityError> {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 0, total_units: 1 })?;
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let SelectedTrustedBundleV1 { package_indices: order, profile } = validate_bundle(&bundle, profile_id)?;
        let order_len = u64::try_from(order.len()).map_err(|error| catalog_error(error))?;
        let total_units = order_len.checked_mul(4).and_then(|units| units.checked_add(1)).ok_or_else(|| catalog("catalog progress total overflow"))?;
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
        let mut retained_browser_actor_bytes = 0u64;
        let mut packages = Vec::with_capacity(order.len());
        let mut codecs = Vec::new();
        let mut open_targets = Vec::new();
        let mut registration_codecs = Vec::new();
        let mut resolved_paths = BTreeSet::from([bundle_path]);

        for (position, index) in order.into_iter().enumerate() {
            context.checkpoint()?;
            let record = &bundle.packages[index];
            let component_path = TrustedCatalogRelativePathV1::parse(&record.component.path)?;
            if !resolved_paths.insert(component_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let component_bytes = root.read_regular(&component_path, TRUSTED_COMPONENT_MAX_BYTES, context).await?;
            retained_component_bytes = retained_component_bytes
                .checked_add(u64::try_from(component_bytes.len()).map_err(catalog_error)?)
                .filter(|bytes| *bytes <= TRUSTED_COMPONENT_CLOSURE_MAX_BYTES)
                .ok_or_else(|| AuthorityError::ResourceLimit("trusted component closure byte"))?;
            verify_length(record.component.byte_length, component_bytes.len())?;
            let (component_sha256, component_blake3) = dual_hash(&component_bytes, context).await?;
            verify_digest(&record.component.sha256, component_sha256, "component sha256")?;
            verify_digest(&record.component.blake3, component_blake3, "component blake3")?;
            report_package_progress(context, position, 1, total_units)?;

            let descriptor_path = TrustedCatalogRelativePathV1::parse(&record.descriptor.path)?;
            if !resolved_paths.insert(descriptor_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let descriptor_bytes = root.read_regular(&descriptor_path, TRUSTED_DESCRIPTOR_MAX_BYTES, context).await?;
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

            let browser_actor = record.browser_actor.identity();
            record.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &hex_lower(&component_sha256), descriptor_byte_sha256: &hex_lower(&descriptor_sha256) }, package_actor_renderer(record))?;
            let browser_actor_bytes = if let Some(file) = record.browser_actor.file() {
                retained_browser_actor_bytes = retained_browser_actor_bytes.checked_add(file.byte_length).filter(|bytes| *bytes <= TRUSTED_BROWSER_ACTOR_CLOSURE_MAX_BYTES).ok_or(AuthorityError::ResourceLimit("trusted browser actor closure byte"))?;
                let actor_path = TrustedCatalogRelativePathV1::parse(&file.path)?;
                if !resolved_paths.insert(actor_path.clone()) {
                    return Err(catalog("trusted browser actor path is already used by the selected closure"));
                }
                let bytes = root.read_regular(&actor_path, file.byte_length, context).await?;
                verify_length(file.byte_length, bytes.len())?;
                verify_digest(&file.sha256, sha256(&bytes, context).await?, "browser actor sha256")?;
                Some(Arc::<[u8]>::from(bytes))
            } else {
                None
            };
            report_package_progress(context, position, 3, total_units)?;

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
                codecs.push(VerifiedNativeArtifactCodec { identity, codec: binding.codec.clone(), genesis: binding.genesis });
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
                    TrustedBundleOpenRole::Viewer => DocumentOpenSurfaceRoleV1::Viewer,
                    TrustedBundleOpenRole::Editor => DocumentOpenSurfaceRoleV1::Editor,
                };
                let renderer_target = match target.renderer_target {
                    TrustedBundleRendererTarget::React => DocumentOpenRendererTargetV1::React,
                    TrustedBundleRendererTarget::Wgpu => DocumentOpenRendererTargetV1::Wgpu,
                    TrustedBundleRendererTarget::Wasm => DocumentOpenRendererTargetV1::Wasm,
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
                        execution_protocol: DocumentExecutionProtocolV1 { app_channel_version: descriptor.execution_protocol.app_channel_version },
                    },
                    artifact: DocumentOpenArtifactV1 { kind: target.artifact_kind.clone(), schema: target.artifact_schema.clone(), pack_schema_hash: target.pack_schema_hash.clone() },
                    surface: DocumentOpenSurfaceV1 { surface_id: target.surface_id.clone(), app_id: target.app_id.clone(), window_kind_id: target.window_kind_id.clone(), role, renderer_target },
                    grant: DocumentOpenGrantV1 { read: target.grant.read, write: target.grant.write, observe: target.grant.observe },
                    browser_actor: if matches!(renderer_target, DocumentOpenRendererTargetV1::Wasm) { browser_actor.clone() } else { DocumentOpenBrowserActorV1::None },
                };
                if open_targets.iter().any(|existing| document_open_target_sort_key(existing) == document_open_target_sort_key(&selection)) {
                    return Err(catalog("document-open target identity is duplicated"));
                }
                open_targets.push(selection);
            }
            report_package_progress(context, position, 4, total_units)?;
            packages.push(VerifiedTrustedPackage {
                plugin_id: record.plugin_id.clone(),
                package: PackageRef { package: PackageId(record.package_id.clone()), hash: PackageHash(component_blake3) },
                version: record.version.clone(),
                component_sha256,
                descriptor_sha256,
                component_bytes: component_bytes.into(),
                descriptor_bytes: descriptor_bytes.into(),
                browser_actor,
                browser_actor_bytes,
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
        Ok((catalog, registration_codecs))
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

fn validate_descriptor_open_target(descriptor: &PackageDescriptor, target: &TrustedBundleOpenTargetV1) -> Result<semio_framework::ArtifactDialect, AuthorityError> {
    let expected_role = match target.role {
        TrustedBundleOpenRole::Viewer => semio_framework::AppRole::Viewer,
        TrustedBundleOpenRole::Editor => semio_framework::AppRole::Editor,
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
        || target.renderer_target != TrustedBundleRendererTarget::Wasm
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

fn selected_closure_digest(identities: &[TrustedBundleIdentityV1]) -> Result<[u8; 32], AuthorityError> {
    let mut encoded = b"semio/hub/trusted-profile-selected-closure/v1\0".to_vec();
    encoded.extend_from_slice(&u32::try_from(identities.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in identities {
        for value in [identity.plugin_id.as_bytes(), identity.package_id.as_bytes(), identity.version.as_bytes()] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
    }
    Ok(Sha256::digest(&encoded))
}

fn package_actor_renderer(package: &TrustedBundlePackageV1) -> &'static str {
    if package.open_targets.iter().any(|target| matches!(target.renderer_target, TrustedBundleRendererTarget::Wasm)) {
        "wasm"
    } else {
        "react"
    }
}

/// 🧮️ Frames resolved dependency identities in native tuple order for every profile generation.
fn append_trusted_profile_dependencies(encoded: &mut Vec<u8>, dependencies: &[TrustedBundleIdentityV1]) -> Result<(), AuthorityError> {
    let mut dependencies = dependencies.iter().collect::<Vec<_>>();
    dependencies.sort();
    encoded.extend_from_slice(&u32::try_from(dependencies.len()).map_err(catalog_error)?.to_be_bytes());
    for dependency in dependencies {
        for value in [dependency.plugin_id.as_bytes(), dependency.package_id.as_bytes(), dependency.version.as_bytes()] {
            append_document_open_catalog_field(encoded, value)?;
        }
    }
    Ok(())
}

fn trusted_profile_generation(bundle: &TrustedBundleV1, profile: &TrustedBundleProfileV1) -> Result<String, AuthorityError> {
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
            TrustedBundlePackageRole::Plugin => b"plugin".as_slice(),
            TrustedBundlePackageRole::Extension => b"extension".as_slice(),
        };
        for value in [
            package.plugin_id.as_bytes(),
            package.package_id.as_bytes(),
            package.version.as_bytes(),
            role,
            decode_digest(&package.component.sha256, "profile component sha256")?.as_slice(),
            decode_digest(&package.component.blake3, "profile component blake3")?.as_slice(),
            decode_digest(&package.descriptor.sha256, "profile descriptor sha256")?.as_slice(),
            package.execution_protocol.app_channel_version.to_be_bytes().as_slice(),
        ] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
        package.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &package.component.sha256, descriptor_byte_sha256: &package.descriptor.sha256 }, package_actor_renderer(package))?;
        package.browser_actor.append_generation(&mut encoded)?;
        append_trusted_profile_dependencies(&mut encoded, &package.dependencies)?;
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
        TrustedBundleOpenRole::Viewer => b"viewer".as_slice(),
        TrustedBundleOpenRole::Editor => b"editor".as_slice(),
    };
    let renderer = match target.renderer_target {
        TrustedBundleRendererTarget::React => b"react".as_slice(),
        TrustedBundleRendererTarget::Wgpu => b"wgpu".as_slice(),
        TrustedBundleRendererTarget::Wasm => b"wasm".as_slice(),
    };
    for value in [
        package.plugin_id.as_bytes(),
        package.package_id.as_bytes(),
        package.version.as_bytes(),
        decode_digest(&package.component.sha256, "open target component sha256")?.as_slice(),
        decode_digest(&package.component.blake3, "open target component blake3")?.as_slice(),
        decode_digest(&package.descriptor.sha256, "open target descriptor sha256")?.as_slice(),
        package.execution_protocol.app_channel_version.to_be_bytes().as_slice(),
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
    let bundle: TrustedBundleV1 = serde_json::from_slice(bundle_bytes).map_err(catalog_error)?;
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

fn validate_identity(identity: &TrustedBundleIdentityV1) -> Result<(), AuthorityError> {
    if !valid_identity(&identity.plugin_id) || !valid_package_id(&identity.package_id) || !valid_identity(&identity.version) {
        return Err(catalog("trusted package identity is empty, padded, or oversized"));
    }
    Ok(())
}

fn validate_file(file: &TrustedBundleFileV1, maximum: u64) -> Result<(), AuthorityError> {
    if file.path.is_empty() || file.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES || file.byte_length == 0 || file.byte_length > maximum {
        return Err(catalog("trusted file record is empty or exceeds its fixed boundary"));
    }
    decode_digest(&file.sha256, "sha256")?;
    Ok(())
}

fn validate_bundle(bundle: &TrustedBundleV1, profile_id: &str) -> Result<SelectedTrustedBundleV1, AuthorityError> {
    if bundle.schema_version != 2 || bundle.packages.is_empty() || bundle.packages.len() > TRUSTED_CATALOG_MAX_PACKAGES || bundle.profiles.is_empty() || bundle.profiles.len() > TRUSTED_BUNDLE_MAX_PROFILES || !valid_identity(profile_id) {
        return Err(catalog("trusted bundle shape or version is invalid"));
    }
    let mut plugins = BTreeMap::new();
    let mut package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for (index, package) in bundle.packages.iter().enumerate() {
        let identity = TrustedBundleIdentityV1 { plugin_id: package.plugin_id.clone(), package_id: package.package_id.clone(), version: package.version.clone() };
        validate_identity(&identity)?;
        if plugins.insert(package.plugin_id.as_str(), index).is_some() || !package_ids.insert(package.package_id.as_str()) {
            return Err(catalog("trusted plugin or package identity is duplicated"));
        }
        Version::parse(&package.version).map_err(catalog_error)?;
        ensure_count(package.dependencies.len(), TRUSTED_PACKAGE_MAX_DEPENDENCIES, "trusted dependency count")?;
        ensure_count(package.native_codecs.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
        ensure_count(package.open_targets.len(), TRUSTED_CATALOG_MAX_OPEN_TARGETS, "trusted document-open target count")?;
        validate_file(&TrustedBundleFileV1 { path: package.component.path.clone(), byte_length: package.component.byte_length, sha256: package.component.sha256.clone() }, TRUSTED_COMPONENT_MAX_BYTES)?;
        decode_digest(&package.component.blake3, "component blake3")?;
        validate_file(&package.descriptor, TRUSTED_DESCRIPTOR_MAX_BYTES)?;
        if !paths.insert(package.component.path.clone()) || !paths.insert(package.descriptor.path.clone()) {
            return Err(catalog("trusted file path is reused across package records"));
        }
        package.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &package.component.sha256, descriptor_byte_sha256: &package.descriptor.sha256 }, package_actor_renderer(package))?;
        if let Some(file) = package.browser_actor.file() {
            if !paths.insert(file.path) {
                return Err(catalog("trusted browser actor path is reused across package records"));
            }
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
            let expected_grant = TrustedBundleGrantV1 { read: true, write: matches!(target.role, TrustedBundleOpenRole::Editor), observe: true };
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
                        || package.dependencies.as_slice() != std::slice::from_ref(&profile.selected_closure[1])
                        || package.open_targets.len() != 1
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gismap" && codec.artifact_schema == "gis.map")
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gisterrain" && codec.artifact_schema == "gis.terrain")
                })
                || stdio.is_none_or(|package| package.native_codecs.len() != 26 || !package.open_targets.is_empty() || !package.dependencies.is_empty())
                || profile.open_target.package.plugin_id != "gis"
                || target.artifact_kind != "s.gis.gismap"
                || target.artifact_schema != "gis.map"
                || target.surface_id != "s.gis.gismap@1/*#editor"
                || target.app_id != "s.gis.gismap@1/*#editor"
                || target.window_kind_id != "gis2d-main"
                || target.parent_dialect != (semio_framework::ArtifactDialect { artifact_kind: "s.gis.gismap".into(), standard: "1".into(), subset: "*".into() })
                || target.role != TrustedBundleOpenRole::Editor
                || target.renderer_target != TrustedBundleRendererTarget::Wasm
                || target.grant != (TrustedBundleGrantV1 { read: true, write: true, observe: true })
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

fn validate_descriptor(record: &TrustedBundlePackageV1, descriptor: &PackageDescriptor, packages: &[TrustedBundlePackageV1]) -> Result<(), AuthorityError> {
    if descriptor.descriptor_version != 1
        || !valid_package_id(&descriptor.package_id)
        || descriptor.package_id != record.package_id
        || !record.role.matches(descriptor.role)
        || descriptor.manifest.plugin_id != record.plugin_id
        || descriptor.manifest.version != record.version
        || descriptor.hashes.wasm_sha256 != record.component.sha256
        || descriptor.execution_protocol != record.execution_protocol
        || descriptor.execution_protocol.app_channel_version != directory::os_spr::CHANNEL_VERSION
    {
        return Err(catalog("decoded package descriptor identity does not exactly match its trust record"));
    }
    if decode_digest(&descriptor.hashes.core_wasm_sha256, "descriptor core wasm sha256")? == [0; 32] || decode_digest(&descriptor.hashes.descriptor_sha256, "descriptor metadata sha256")? == [0; 32] {
        return Err(catalog("decoded package descriptor hash metadata is zero"));
    }
    if descriptor.manifest.dependencies.len() != record.dependencies.len() {
        return Err(catalog("decoded manifest dependency count does not match the trust record"));
    }
    let records: BTreeMap<&str, &TrustedBundlePackageV1> = packages.iter().map(|package| (package.plugin_id.as_str(), package)).collect();
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
    let mut options = os_store::PackDecodeOptions::default();
    options.limits.max_file_len = TRUSTED_DESCRIPTOR_MAX_BYTES;
    options.limits.max_segment_len = TRUSTED_DESCRIPTOR_MAX_BYTES;
    options.limits.max_symbols = 131_072;
    options.limits.max_items = 262_144;
    options.limits.max_depth = 64;
    options.limits.max_total_alloc = TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION;
    let value = os_store::pack_rt::decode_wire_value_with_options(bytes, &options).map_err(catalog_error)?;
    reject_duplicate_descriptor_fields(&value)?;
    let canonical = os_store::pack_rt::encode_wire_value(&value);
    if canonical != bytes {
        return Err(catalog("package descriptor is not its exact canonical schema projection"));
    }
    let descriptor: PackageDescriptor = from_dsl_value(value).map_err(catalog_error)?;
    let projection = to_dsl_value(&descriptor).map_err(catalog_error)?;
    if os_store::pack_rt::encode_wire_value(&projection) != bytes { return Err(catalog("package descriptor is not its exact canonical schema projection")); }
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
    let completed_units = position.checked_mul(4).and_then(|units| units.checked_add(package_phase)).ok_or_else(|| catalog("catalog progress overflow"))?;
    context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogLoading, completed_units, total_units })
}

/// 🧫️ Shares headless Stdio metadata between native GIS fixtures; synthetic bytes are never executed.
#[cfg(all(feature = "native-artifact-execution", any(test, feature = "test-support")))]
fn headless_stdio_fixture_package(root: &Path) -> Result<(serde_json::Value, serde_json::Value), AuthorityError> {
    let dependency = semio_s_plugin_stdio::registry::native_artifact_catalog_dependency().map_err(catalog_error)?;
    let semio_framework::VersionReq::Exact(version) = dependency.version else { return Err(catalog("compiled Stdio fixture dependency is not exact")); };
    let version = version.to_string();
    let receipts = semio_s_plugin_stdio::registry::native_codec_factory_receipts().map_err(catalog_error)?;
    let mut builder = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio").label("Stdio Fixture").version(version.clone()).package_id("semio:stdio");
    for kind in semio_s_plugin_stdio::registry::native_codec_artifact_kinds() {
        builder = builder.artifact_kind(kind);
    }
    let plugin = builder.contributes_topic(semio_s_plugin_stdio::registry::native_artifact_catalog_contribution().map_err(catalog_error)?).try_library().map_err(catalog_error)?;
    let component = b"synthetic-stdio-component-for-linked-catalog-test";
    let component_sha256 = hex_lower(&Sha256::digest(component));
    let mut component_blake3 = Hasher::new();
    component_blake3.update(component);
    let mut descriptor = semio_framework::PackageDescriptor {
        descriptor_version: 1, package_id: "semio:stdio".into(), role: semio_framework::PackageRole::Plugin,
        manifest: plugin.manifest, activation_events: Vec::new(), capability_requests: Vec::new(), extension_points: Vec::new(),
        execution: semio_framework::ExecutionMode::Isolated,
        execution_protocol: semio_framework::ExecutionProtocol { app_channel_version: directory::os_spr::CHANNEL_VERSION },
        quotas: semio_framework::kernel::QuotaSchema::default(), contributions: semio_framework::ContributionSet::default(), assets: Vec::new(),
        hashes: semio_framework::PackageHashes { wasm_sha256: component_sha256.clone(), core_wasm_sha256: component_sha256.clone(), descriptor_sha256: String::new() },
    };
    descriptor.hashes.descriptor_sha256 = hex_lower(&Sha256::digest(&os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).map_err(catalog_error)?)));
    let bytes = os_store::pack_rt::encode_wire_value(&to_dsl_value(&descriptor).map_err(catalog_error)?);
    decode_package_descriptor(&bytes)?;
    std::fs::write(root.join("stdio-component.wasm"), component).map_err(catalog_error)?;
    std::fs::write(root.join("stdio-descriptor.semio"), &bytes).map_err(catalog_error)?;
    let identity = serde_json::json!({ "pluginId": "stdio", "packageId": "semio:stdio", "version": version });
    let codecs: Vec<_> = receipts.into_iter().map(|receipt| serde_json::json!({ "artifactKind": receipt.artifact_kind, "artifactSchema": receipt.schema, "packSchemaHash": hex_lower(&receipt.pack_schema_hash) })).collect();
    let record = serde_json::json!({
        "pluginId": "stdio", "packageId": "semio:stdio", "version": version, "role": "plugin", "dependencies": [],
        "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
        "component": { "path": "stdio-component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
        "descriptor": { "path": "stdio-descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
        "browserActor": { "kind": "none" }, "nativeCodecs": codecs, "openTargets": [],
    });
    serde_json::from_value::<TrustedBundlePackageV1>(record.clone()).map_err(catalog_error)?;
    Ok((identity, record))
}

/// 🏗️ Feature-gated real GIS Map profile builder, reachable from every crate target (see its module doc).
#[cfg(all(feature = "test-support", feature = "native-artifact-execution"))]
#[path = "🏗️test-support/🦀️.rs"]
pub mod test_support;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
