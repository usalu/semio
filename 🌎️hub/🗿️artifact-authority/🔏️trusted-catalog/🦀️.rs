//! 🗂️ Immutable trusted-catalog bundle verification for headless hub authority startup.

use super::adapters::{bounded_message, AUTHORITY_MAX_CODEC_TEXT_BYTES, TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_PACKAGES};
use super::{AcceptedArtifactOperation, ArtifactPair, ArtifactValidationStage, AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext, TrustedArtifactCatalog, TrustedArtifactCodec, TrustedArtifactGenesisCodec, TrustedArtifactIdentity, TrustedArtifactReplayCodec};
use directory::os_directory::{hex_lower, DocumentDescriptor, DocumentExecutionProtocolV1, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use directory::os_store::{self, ArtifactCodec};
use semio_framework::{from_dsl_value, to_dsl_value, DslValue, PackageDescriptor, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::{PackageHash, PackageId, PackageRef};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
use schema::{publication_revision, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1, TrustedPluginModuleIndexEntryV1, TrustedPluginModuleIndexV1, TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA, TrustedCatalogCurrentPointerV1, TrustedCatalogPublicationCommandV1, TrustedCatalogPublicationReceiptV1, TRUSTED_CATALOG_PUBLICATION_MAX_BYTES, TRUSTED_CATALOG_PUBLICATION_OUTCOME_DURABLE, TRUSTED_CATALOG_PUBLICATION_OUTCOME_UNCONFIRMED, TRUSTED_CATALOG_PUBLICATION_RECEIPT_SCHEMA, TRUSTED_CATALOG_PUBLICATION_SCHEMA, TRUSTED_CATALOG_GUEST_RESIDENCY, TrustedCatalogGuestResidencyStateV1, TrustedCatalogGuestResidencyV1, TrustedCatalogLoadProgressV1, TrustedCatalogPackagePhaseV1, TrustedCatalogPackageProgressV1, GuestCodecVerificationV1, GUEST_CODEC_VERIFICATION_SCHEMA};

#[path = "🌐️browser-actor/🦀️.rs"]
mod browser_actor;
#[path = "🧩️plugin-module/🦀️.rs"]
pub mod plugin_module;
use plugin_module::{decode_plugin_module_bundle, plugin_module_blob_path, verify_plugin_module_file, TrustedPluginModuleSourceV1, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES};
#[path = "🛡️opened-root/🦀️.rs"]
mod opened_root;
use opened_root::{TrustedCatalogDataRoot, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, TRUSTED_READ_CHUNK_BYTES};
#[cfg(all(test, unix))]
use opened_root::create_fifo_fixture;
use directory::os_directory::schema::{DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1, DOCUMENT_BROWSER_ACTOR_MAX_BYTES};

/// 🧯️ Maximum accepted serialized bundle bytes.
pub const TRUSTED_BUNDLE_MAX_BYTES: u64 = 4 * 1024 * 1024;
/// 🧯️ Maximum accepted committed package-descriptor bytes: the schema-declared execution-target
/// descriptor bound the browser enforces on the same bytes.
pub const TRUSTED_DESCRIPTOR_MAX_BYTES: u64 = directory::os_directory::DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES;
/// 🧮️ Maximum logical owned storage admitted before descriptor schema/canonical projection.
pub const TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION: u64 = 32 * 1024 * 1024;
/// 🧯️ Maximum accepted bytes for one component: the schema-declared execution-target component bound
/// the browser enforces on the same bytes. A closure has no byte total: components and actors are
/// retained by identity on disk ([`TrustedCatalogAsset`]), so the catalog is bounded by storage and the
/// hub's memory by the assets in use.
pub const TRUSTED_COMPONENT_MAX_BYTES: u64 = directory::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES;
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
        let generation = Arc::new(owner.open_generation(&command.generation_id)?);
        let relative = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation.read_regular(&relative, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != bundle_digest { return Err(catalog("trusted publication candidate bundle differs from its digest")); }
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let (verified, _) = TrustedCatalogLoader::verify_selected(&generation, relative, bundle_bytes, &command.profile_id, providers, &GuestCodecVerificationCacheV1::beside(data_path), context).await?;
        if verified.generation_id() != command.generation_id { return Err(catalog("trusted publication candidate generation differs from the verified profile")); }
        let mut published_module_files = BTreeSet::new();
        for package in &verified.packages {
            let record = bundle.packages.iter().find(|record| record.plugin_id == package.plugin_id && record.package_id == package.package.package.0 && record.version == package.version).ok_or_else(|| catalog("verified publication package lost its bundle record"))?;
            let mut files = vec![TrustedBundleFileV1 { path: record.component.path.clone(), byte_length: record.component.byte_length, sha256: record.component.sha256.clone() }, record.descriptor.clone()];
            if let Some(actor) = record.browser_actor.file() { files.push(actor); }
            files.push(TrustedBundleFileV1 { path: record.plugin_module.path.clone(), byte_length: record.plugin_module.byte_length, sha256: record.plugin_module.sha256.clone() });
            for file in &package.plugin_module.bundle.files {
                if published_module_files.insert(file.sha256.clone()) { files.push(TrustedBundleFileV1 { path: plugin_module_blob_path(&file.sha256), byte_length: file.byte_length, sha256: file.sha256.clone() }); }
            }
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
}

impl NativeCodecBinding {
    /// 🪢️ Binds a native executable without deriving package identity from plugin identity.
    pub fn new(plugin_id: impl Into<String>, package_id: impl Into<String>, artifact_kind: impl Into<String>, codec: ArtifactCodec) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package_id.into(), artifact_kind: artifact_kind.into(), codec }
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

/// 💾️ One hash-verified generation file retained by IDENTITY, not by bytes: the opened private
/// generation root, the bundle-relative path and the digests the load verified. A read reopens the
/// path beneath that root and re-verifies length and digests, and the bytes stay resident only while
/// some reader holds them, so the hub's memory is the set of assets in use, never the catalog closure.
struct TrustedRetainedFile {
    root: Arc<TrustedCatalogGenerationRoot>,
    path: TrustedCatalogRelativePathV1,
    byte_length: u64,
    sha256: [u8; 32],
    blake3: Option<[u8; 32]>,
    resident: std::sync::Mutex<std::sync::Weak<[u8]>>,
}

impl TrustedRetainedFile {
    fn new(root: Arc<TrustedCatalogGenerationRoot>, path: TrustedCatalogRelativePathV1, byte_length: u64, sha256: [u8; 32], blake3: Option<[u8; 32]>) -> Self {
        let resident: std::sync::Weak<[u8]> = std::sync::Weak::<[u8; 0]>::new();
        Self { root, path, byte_length, sha256, blake3, resident: std::sync::Mutex::new(resident) }
    }

    async fn read(&self, context: &OperationContext<'_>) -> Result<Arc<[u8]>, AuthorityError> {
        if let Some(bytes) = self.resident.lock().unwrap_or_else(std::sync::PoisonError::into_inner).upgrade() {
            return Ok(bytes);
        }
        let bytes = self.root.read_regular(&self.path, self.byte_length, context).await?;
        verify_length(self.byte_length, bytes.len())?;
        let verified = match self.blake3 {
            Some(blake3) => dual_hash(&bytes, context).await? == (self.sha256, blake3),
            None => sha256(&bytes, context).await? == self.sha256,
        };
        if !verified {
            return Err(catalog("retained trusted file differs from the digests its catalog verified"));
        }
        let bytes: Arc<[u8]> = bytes.into();
        *self.resident.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Arc::downgrade(&bytes);
        Ok(bytes)
    }
}

/// 🧱️ One verified execution-target asset (a component or a closed browser actor), read on demand.
/// Its length is known without reading; [`Self::read`] yields exactly the bytes the catalog verified.
#[derive(Clone)]
pub struct TrustedCatalogAsset {
    source: TrustedCatalogAssetSource,
}

#[derive(Clone)]
enum TrustedCatalogAssetSource {
    Retained(Arc<TrustedRetainedFile>),
    Resident(Arc<[u8]>),
}

impl TrustedCatalogAsset {
    /// 🧷️ An asset whose bytes a caller already holds in memory (a fixture catalog built in-process).
    pub fn resident(bytes: Arc<[u8]>) -> Self {
        Self { source: TrustedCatalogAssetSource::Resident(bytes) }
    }

    fn retained(file: TrustedRetainedFile) -> Self {
        Self { source: TrustedCatalogAssetSource::Retained(Arc::new(file)) }
    }

    /// 📏️ The verified byte length, known without reading.
    pub fn byte_length(&self) -> u64 {
        match &self.source {
            TrustedCatalogAssetSource::Retained(file) => file.byte_length,
            TrustedCatalogAssetSource::Resident(bytes) => bytes.len() as u64,
        }
    }

    /// 📖️ The verified bytes; a retained asset is reread and re-verified unless a reader still holds it.
    pub async fn read(&self, context: &OperationContext<'_>) -> Result<Arc<[u8]>, AuthorityError> {
        match &self.source {
            TrustedCatalogAssetSource::Retained(file) => file.read(context).await,
            TrustedCatalogAssetSource::Resident(bytes) => Ok(Arc::clone(bytes)),
        }
    }

    /// 🚰️ The verified bytes as a chunked stream with no request deadline: a retained asset is reopened beneath its
    /// generation root and hashed while it streams (see [`TrustedCatalogAssetStream`]).
    pub fn stream(&self) -> Result<TrustedCatalogAssetStream, AuthorityError> {
        match &self.source {
            TrustedCatalogAssetSource::Retained(file) => {
                let (reader, length) = file.root.open_regular(&file.path)?.into_reader();
                if length != file.byte_length {
                    return Err(catalog("retained trusted file differs from the length its catalog verified"));
                }
                Ok(TrustedCatalogAssetStream {
                    byte_length: file.byte_length,
                    source: TrustedCatalogAssetStreamSource::Retained { reader, streamed: 0, sha256: Sha256::new(), blake3: Hasher::new(), expected: (file.sha256, file.blake3), held: None },
                })
            }
            TrustedCatalogAssetSource::Resident(bytes) => Ok(TrustedCatalogAssetStream { byte_length: bytes.len() as u64, source: TrustedCatalogAssetStreamSource::Resident { bytes: Arc::clone(bytes), offset: 0 } }),
        }
    }
}

/// 🚰️ One verified asset served in bounded chunks. Serving has no absolute deadline — bytes flow while the file is
/// hashed, so a large module on a loaded host is slow, never refused — and the final chunk is released only once the
/// streamed length, SHA-256 and BLAKE3 equal what the catalog verified: a file changed on disk after verification ends
/// the stream in an error before its last bytes, so a reader holding the declared length never completes it. A
/// resident asset streams the bytes the catalog already verified. Dropping the stream stops reading.
pub struct TrustedCatalogAssetStream {
    byte_length: u64,
    source: TrustedCatalogAssetStreamSource,
}

enum TrustedCatalogAssetStreamSource {
    Retained { reader: tokio::fs::File, streamed: u64, sha256: Sha256, blake3: Hasher, expected: ([u8; 32], Option<[u8; 32]>), held: Option<Vec<u8>> },
    Resident { bytes: Arc<[u8]>, offset: usize },
    Finished,
}

impl TrustedCatalogAssetStream {
    /// 📏️ The verified byte length the stream delivers in full or not at all.
    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    /// 📦️ The next chunk, `None` after the last one; an error ends the stream.
    pub async fn next_chunk(&mut self) -> Option<Result<Vec<u8>, AuthorityError>> {
        let outcome = match &mut self.source {
            TrustedCatalogAssetStreamSource::Finished => return None,
            TrustedCatalogAssetStreamSource::Resident { bytes, offset } => {
                let end = offset.saturating_add(TRUSTED_READ_CHUNK_BYTES).min(bytes.len());
                let chunk = (*offset < end).then(|| bytes[*offset..end].to_vec());
                *offset = end;
                chunk.map(Ok)
            }
            TrustedCatalogAssetStreamSource::Retained { reader, streamed, sha256, blake3, expected, held } => match Self::next_retained(self.byte_length, reader, streamed, sha256, blake3, *expected, held).await {
                Some(Ok((chunk, verified_last))) => {
                    if verified_last {
                        self.source = TrustedCatalogAssetStreamSource::Finished;
                    }
                    return Some(Ok(chunk));
                }
                refused => refused.map(|chunk| chunk.map(|(chunk, _)| chunk)),
            },
        };
        if !matches!(outcome, Some(Ok(_))) {
            self.source = TrustedCatalogAssetStreamSource::Finished;
        }
        outcome
    }

    /// 🔐️ The next held-back chunk of a retained file, `true` beside the last one, which is released only after the
    /// streamed length and digests equal the verified ones.
    async fn next_retained(byte_length: u64, reader: &mut tokio::fs::File, streamed: &mut u64, sha256: &mut Sha256, blake3: &mut Hasher, expected: ([u8; 32], Option<[u8; 32]>), held: &mut Option<Vec<u8>>) -> Option<Result<(Vec<u8>, bool), AuthorityError>> {
        use tokio::io::AsyncReadExt;
        loop {
            let remaining = byte_length.saturating_sub(*streamed);
            let mut chunk = vec![0u8; usize::try_from(remaining.saturating_add(1)).map_or(TRUSTED_READ_CHUNK_BYTES, |limit| limit.min(TRUSTED_READ_CHUNK_BYTES))];
            let read = match reader.read(&mut chunk).await {
                Ok(read) => read,
                Err(error) => return Some(Err(catalog_error(error))),
            };
            if read == 0 {
                if *streamed != byte_length {
                    return Some(Err(catalog("retained trusted file length changed while streaming")));
                }
                let digests = (std::mem::replace(sha256, Sha256::new()).finalize(), *blake3.finalize().as_bytes());
                if digests.0 != expected.0 || expected.1.is_some_and(|verified| verified != digests.1) {
                    return Some(Err(catalog("retained trusted file differs from the digests its catalog verified")));
                }
                return held.take().map(|chunk| Ok((chunk, true)));
            }
            *streamed = streamed.saturating_add(read as u64);
            if *streamed > byte_length {
                return Some(Err(catalog("retained trusted file grew beyond its verified length while streaming")));
            }
            chunk.truncate(read);
            sha256.update(&chunk);
            blake3.update(&chunk);
            if let Some(previous) = held.replace(chunk) {
                return Some(Ok((previous, false)));
            }
        }
    }
}

/// 🧩️ One package's verified plugin module: its canonical manifest, addressed by the manifest's SHA-256,
/// and every file it lists, retained by identity and reread and re-verified on use.
pub struct VerifiedPluginModule {
    bundle_sha256: String,
    manifest_bytes: Arc<[u8]>,
    bundle: TrustedPluginModuleBundleV1,
    files: BTreeMap<String, TrustedCatalogAsset>,
}

impl VerifiedPluginModule {
    /// 🔐️ The content address: the SHA-256 of the canonical manifest bytes.
    pub fn bundle_sha256(&self) -> &str {
        &self.bundle_sha256
    }

    /// 📜️ The exact canonical manifest bytes the catalog verified.
    pub fn manifest_bytes(&self) -> &[u8] {
        &self.manifest_bytes
    }

    /// 🧩️ The decoded manifest.
    pub fn bundle(&self) -> &TrustedPluginModuleBundleV1 {
        &self.bundle
    }

    /// 📖️ The verified file at one module-relative path, read on demand.
    pub fn file(&self, path: &str) -> Option<&TrustedCatalogAsset> {
        self.files.get(path)
    }
}

/// 🧬️ One fully verified package retained in dependency-first order.
pub struct VerifiedTrustedPackage {
    plugin_id: String,
    package: PackageRef,
    version: String,
    dependencies: Vec<String>,
    plugin_module: VerifiedPluginModule,
    component_sha256: [u8; 32],
    descriptor_sha256: [u8; 32],
    component: TrustedCatalogAsset,
    descriptor_bytes: Arc<[u8]>,
    browser_actor: DocumentOpenBrowserActorV1,
    browser_actor_asset: Option<TrustedCatalogAsset>,
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

    /// 🧱️ Returns the verified component, read on demand.
    pub fn component(&self) -> &TrustedCatalogAsset {
        &self.component
    }

    /// 📜️ Returns the exact bytes decoded into `descriptor()`.
    pub fn descriptor_bytes(&self) -> &[u8] {
        &self.descriptor_bytes
    }

    /// 🗂️ Returns the decoded existing `PackageDescriptor`.
    pub fn descriptor(&self) -> &PackageDescriptor {
        &self.descriptor
    }

    /// 🧩️ Returns the package's verified plugin module.
    pub fn plugin_module(&self) -> &VerifiedPluginModule {
        &self.plugin_module
    }
}

/// ⛽️ Fuel and wall-clock ceiling for ONE guest codec call. A `codec` function is pure and bounded
/// (encode a snapshot, print a history log, reduce an op batch), so a call that needs more than this
/// is refused rather than allowed to hold a request thread. The fuel figure is the same order the
/// build-time `describe()` cap sits at, measured on a real debug-built component.
const GUEST_CODEC_BUDGET: semio_framework::kernel::Budget =
    semio_framework::kernel::Budget { fuel: 4_000_000_000, deadline_ms: 30_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };

/// 🧊️ Compiled values held within an operator-configured memory budget (`TrustedCatalogGuestResidencyV1`):
/// every value is charged its source's byte length, and every operation ([`OperationContext`]) that uses a
/// value counts as one use of it however many calls it makes. A newly compiled value that fits stays resident.
/// One that does not fit stays only when its uses before this operation outnumber those of every value it
/// would release — the least recently used values no call holds, taken in that order — and otherwise serves
/// the operation that compiled it (and any other that reaches it meanwhile) and is dropped with it. Use counts
/// are capped and halve together after a declared number of uses per registered value, so a round-robin over
/// more values than fit keeps a stable resident set instead of recompiling every value on every use, while a
/// workload that moved on displaces the values it left. Nothing is released for idleness, a value a running
/// call holds is never released, and a released value compiles again on its next use.
pub(crate) struct GuestResidencyLedgerV1<T> {
    budget_bytes: std::sync::atomic::AtomicU64,
    access_count_ceiling: u32,
    access_count_aging_per_guest: u64,
    clock: std::sync::atomic::AtomicU64,
    accesses_since_aging: std::sync::atomic::AtomicU64,
    hits: std::sync::atomic::AtomicU64,
    compiles: std::sync::atomic::AtomicU64,
    admitted: std::sync::atomic::AtomicU64,
    bypassed: std::sync::atomic::AtomicU64,
    released: std::sync::atomic::AtomicU64,
    compile_micros: std::sync::atomic::AtomicU64,
    slots: std::sync::Mutex<Vec<Arc<GuestResidencySlotV1<T>>>>,
}

/// 🧩️ What one slot holds: nothing, a resident value, or a value that serves only the calls holding it.
enum GuestResidentValueV1<T> {
    Absent,
    Resident(Arc<T>),
    Held(std::sync::Weak<T>),
}

struct GuestResidencySlotV1<T> {
    value: tokio::sync::Mutex<GuestResidentValueV1<T>>,
    charge_bytes: u64,
    last_used: std::sync::atomic::AtomicU64,
    access_count: std::sync::atomic::AtomicU32,
    last_operation: std::sync::atomic::AtomicU64,
    prior_uses: std::sync::atomic::AtomicU32,
}

impl<T> GuestResidencySlotV1<T> {
    fn counts_as_resident(&self) -> bool {
        self.value.try_lock().map_or(true, |value| match &*value {
            GuestResidentValueV1::Resident(_) => true,
            GuestResidentValueV1::Held(held) => held.strong_count() > 0,
            GuestResidentValueV1::Absent => false,
        })
    }

    fn charged(&self) -> bool {
        self.value.try_lock().map_or(true, |value| matches!(&*value, GuestResidentValueV1::Resident(_)))
    }
}

impl<T> GuestResidencyLedgerV1<T> {
    pub(crate) fn new(residency: TrustedCatalogGuestResidencyV1) -> Arc<Self> {
        let counter = || std::sync::atomic::AtomicU64::new(0);
        Arc::new(Self {
            budget_bytes: std::sync::atomic::AtomicU64::new(residency.resident_component_bytes),
            access_count_ceiling: residency.access_count_ceiling,
            access_count_aging_per_guest: residency.access_count_aging_per_guest.max(1),
            clock: counter(),
            accesses_since_aging: counter(),
            hits: counter(),
            compiles: counter(),
            admitted: counter(),
            bypassed: counter(),
            released: counter(),
            compile_micros: counter(),
            slots: std::sync::Mutex::new(Vec::new()),
        })
    }

    /// 🪪️ Registers one value charged `charge_bytes` once resident; answers its residency handle.
    pub(crate) fn register(self: &Arc<Self>, charge_bytes: u64) -> GuestResidencyV1<T> {
        let slot = Arc::new(GuestResidencySlotV1 {
            value: tokio::sync::Mutex::new(GuestResidentValueV1::Absent),
            charge_bytes,
            last_used: std::sync::atomic::AtomicU64::new(0),
            access_count: std::sync::atomic::AtomicU32::new(0),
            last_operation: std::sync::atomic::AtomicU64::new(0),
            prior_uses: std::sync::atomic::AtomicU32::new(0),
        });
        self.slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).push(Arc::clone(&slot));
        GuestResidencyV1 { ledger: Arc::clone(self), slot }
    }

    fn snapshot(&self) -> Vec<Arc<GuestResidencySlotV1<T>>> {
        self.slots.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    /// 🎚️ Applies a new budget and releases least recently used values no call holds until the resident
    /// charge fits it.
    pub(crate) fn configure(&self, budget_bytes: u64) {
        self.budget_bytes.store(budget_bytes, std::sync::atomic::Ordering::Release);
        let mut slots = self.snapshot();
        slots.sort_by_key(|slot| slot.last_used.load(std::sync::atomic::Ordering::Acquire));
        let mut charged: u64 = slots.iter().filter(|slot| slot.charged()).map(|slot| slot.charge_bytes).sum();
        for slot in slots {
            if charged <= budget_bytes {
                break;
            }
            let Ok(mut value) = slot.value.try_lock() else { continue };
            if matches!(&*value, GuestResidentValueV1::Resident(resident) if Arc::strong_count(resident) == 1) {
                *value = GuestResidentValueV1::Absent;
                charged = charged.saturating_sub(slot.charge_bytes);
                self.released.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            }
        }
    }

    /// 🕰️ Records one call of `operation`: the slot becomes the most recently used, and the operation's first
    /// call counts one use (capped); every `accessCountAgingPerGuest × registered` uses all counts halve.
    /// Answers the slot's uses before this operation and whether this call was the operation's first.
    fn touch(&self, slot: &GuestResidencySlotV1<T>, operation: u64) -> (u32, bool) {
        let tick = self.clock.fetch_add(1, std::sync::atomic::Ordering::AcqRel) + 1;
        slot.last_used.store(tick, std::sync::atomic::Ordering::Release);
        if slot.last_operation.swap(operation, std::sync::atomic::Ordering::AcqRel) == operation {
            return (slot.prior_uses.load(std::sync::atomic::Ordering::Acquire), false);
        }
        let slots = self.snapshot();
        let window = self.access_count_aging_per_guest.saturating_mul(u64::try_from(slots.len()).unwrap_or(u64::MAX).max(1));
        let aged = self.accesses_since_aging.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |uses| Some(if uses + 1 >= window { 0 } else { uses + 1 })).is_ok_and(|uses| uses + 1 >= window);
        if aged {
            for slot in slots {
                let _ = slot.access_count.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| Some(count / 2));
            }
        }
        let ceiling = self.access_count_ceiling;
        let prior = slot.access_count.fetch_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |count| (count < ceiling).then_some(count + 1)).unwrap_or_else(|count| count);
        slot.prior_uses.store(prior, std::sync::atomic::Ordering::Release);
        (prior, true)
    }

    /// ⚖️ Whether `keep`'s compiled value stays resident: it fits, or its `prior` uses outnumber those of every
    /// least recently used unheld value it would release, which are then released. Never admits a value larger
    /// than the whole budget. A slot another call is compiling or acquiring right now counts as charged.
    fn admit(&self, keep: &Arc<GuestResidencySlotV1<T>>, prior: u32) -> bool {
        let budget = self.budget_bytes.load(std::sync::atomic::Ordering::Acquire);
        if keep.charge_bytes > budget {
            return false;
        }
        let slots = self.snapshot();
        let others: Vec<&Arc<GuestResidencySlotV1<T>>> = slots.iter().filter(|slot| !Arc::ptr_eq(slot, keep)).collect();
        let charged: u64 = others.iter().filter(|slot| slot.charged()).map(|slot| slot.charge_bytes).sum();
        let excess = charged.saturating_add(keep.charge_bytes).saturating_sub(budget);
        if excess == 0 {
            return true;
        }
        let mut candidates: Vec<(&Arc<GuestResidencySlotV1<T>>, tokio::sync::MutexGuard<'_, GuestResidentValueV1<T>>)> = others
            .iter()
            .filter_map(|slot| slot.value.try_lock().ok().filter(|value| matches!(&**value, GuestResidentValueV1::Resident(resident) if Arc::strong_count(resident) == 1)).map(|value| (*slot, value)))
            .collect();
        candidates.sort_by_key(|(slot, _)| slot.last_used.load(std::sync::atomic::Ordering::Acquire));
        let (mut freed, mut victim_count, mut hottest_victim) = (0u64, 0usize, 0u32);
        for (slot, _) in &candidates {
            if freed >= excess {
                break;
            }
            freed = freed.saturating_add(slot.charge_bytes);
            victim_count += 1;
            hottest_victim = hottest_victim.max(slot.access_count.load(std::sync::atomic::Ordering::Acquire));
        }
        if freed < excess || prior <= hottest_victim {
            return false;
        }
        for (_, value) in candidates.iter_mut().take(victim_count) {
            **value = GuestResidentValueV1::Absent;
        }
        self.released.fetch_add(u64::try_from(victim_count).unwrap_or(u64::MAX), std::sync::atomic::Ordering::AcqRel);
        true
    }

    /// 📏️ The ledger's state now.
    pub(crate) fn state(&self) -> TrustedCatalogGuestResidencyStateV1 {
        let slots = self.snapshot();
        let resident: Vec<_> = slots.iter().filter(|slot| slot.counts_as_resident()).collect();
        let load = |counter: &std::sync::atomic::AtomicU64| counter.load(std::sync::atomic::Ordering::Acquire);
        TrustedCatalogGuestResidencyStateV1 {
            budget_bytes: load(&self.budget_bytes),
            registered_guests: u64::try_from(slots.len()).unwrap_or(u64::MAX),
            resident_guests: u64::try_from(resident.len()).unwrap_or(u64::MAX),
            resident_bytes: resident.iter().map(|slot| slot.charge_bytes).sum(),
            hits: load(&self.hits),
            compiles: load(&self.compiles),
            admitted: load(&self.admitted),
            bypassed: load(&self.bypassed),
            released: load(&self.released),
            compile_micros: load(&self.compile_micros),
        }
    }
}

/// 🔑️ One value's place in a [`GuestResidencyLedgerV1`].
pub(crate) struct GuestResidencyV1<T> {
    ledger: Arc<GuestResidencyLedgerV1<T>>,
    slot: Arc<GuestResidencySlotV1<T>>,
}

impl<T: Send + Sync + 'static> GuestResidencyV1<T> {
    /// 🔑️ The resident or held value for one call of `context`'s operation, compiled by `compile` when there is
    /// none (one compile at a time). A value that is not admitted stays with the operation that compiled it — and
    /// with any other operation that reaches it meanwhile — and is dropped with the last of them; a held value
    /// is admitted as soon as its uses outnumber the values it would release.
    pub(crate) async fn acquire<F, Fut>(&self, context: &OperationContext<'_>, compile: F) -> Result<Arc<T>, AuthorityError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AuthorityError>>,
    {
        let mut value = self.slot.value.lock().await;
        let (prior, first_call) = self.ledger.touch(&self.slot, context.serial());
        let present = match &*value {
            GuestResidentValueV1::Resident(resident) => Some((Arc::clone(resident), true)),
            GuestResidentValueV1::Held(held) => held.upgrade().map(|held| (held, false)),
            GuestResidentValueV1::Absent => None,
        };
        if let Some((present, resident)) = present {
            self.ledger.hits.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            if !resident {
                if self.ledger.admit(&self.slot, prior) {
                    self.ledger.admitted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    *value = GuestResidentValueV1::Resident(Arc::clone(&present));
                } else if first_call {
                    context.retain(Arc::clone(&present) as Arc<dyn std::any::Any + Send + Sync>);
                }
            }
            return Ok(present);
        }
        let started = std::time::Instant::now();
        let compiled = Arc::new(compile().await?);
        self.ledger.compile_micros.fetch_add(u64::try_from(started.elapsed().as_micros()).unwrap_or(u64::MAX), std::sync::atomic::Ordering::AcqRel);
        self.ledger.compiles.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        *value = if self.ledger.admit(&self.slot, prior) {
            self.ledger.admitted.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            GuestResidentValueV1::Resident(Arc::clone(&compiled))
        } else {
            self.ledger.bypassed.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            context.retain(Arc::clone(&compiled) as Arc<dyn std::any::Any + Send + Sync>);
            GuestResidentValueV1::Held(Arc::downgrade(&compiled))
        };
        Ok(compiled)
    }
}

impl<T> GuestResidencyV1<T> {
    /// 📏️ Whether a compiled value stays resident now (not merely held by a running operation).
    #[cfg(test)]
    pub(crate) fn is_resident(&self) -> bool {
        self.slot.charged()
    }
}

/// 📈️ The live [`TrustedCatalogLoadProgressV1`] of one catalog: its load and its background verification
/// report into it, and the hub reads it for `/readyz` while it starts and for its admin observability route.
#[derive(Default)]
pub struct TrustedCatalogLoadProgressCellV1 {
    progress: std::sync::Mutex<TrustedCatalogLoadProgressV1>,
}

impl TrustedCatalogLoadProgressCellV1 {
    /// 📸️ The progress now.
    pub fn snapshot(&self) -> TrustedCatalogLoadProgressV1 {
        self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
    }

    fn update(&self, change: impl FnOnce(&mut TrustedCatalogLoadProgressV1)) {
        let mut progress = self.progress.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        change(&mut progress);
        progress.packages_total = u64::try_from(progress.packages.len()).unwrap_or(u64::MAX);
        progress.packages_ready = u64::try_from(progress.packages.iter().filter(|package| package.phase == TrustedCatalogPackagePhaseV1::Ready).count()).unwrap_or(u64::MAX);
        progress.packages_refused = u64::try_from(progress.packages.iter().filter(|package| package.phase == TrustedCatalogPackagePhaseV1::Refused).count()).unwrap_or(u64::MAX);
        progress.component_bytes_total = progress.packages.iter().map(|package| package.component_bytes).sum();
        progress.rows_total = progress.packages.iter().map(|package| package.rows).sum();
        progress.rows_pinned = progress.packages.iter().map(|package| package.rows_pinned).sum();
        progress.rows_verified = progress.packages.iter().map(|package| package.rows_verified).sum();
    }

    fn select(&self, packages: Vec<TrustedCatalogPackageProgressV1>) {
        self.update(|progress| *progress = TrustedCatalogLoadProgressV1 { packages, ..TrustedCatalogLoadProgressV1::default() });
    }

    fn package(&self, position: usize, change: impl FnOnce(&mut TrustedCatalogPackageProgressV1)) {
        self.update(|progress| {
            if let Some(package) = progress.packages.get_mut(position) {
                change(package);
            }
        });
    }

    fn phase(&self, position: usize, phase: TrustedCatalogPackagePhaseV1) {
        self.package(position, |package| package.phase = phase);
    }

    fn component_read(&self, position: usize) {
        self.update(|progress| {
            if let Some(bytes) = progress.packages.get(position).map(|package| package.component_bytes) {
                progress.component_bytes_read = progress.component_bytes_read.saturating_add(bytes);
            }
        });
    }
}

/// 🗜️ One verified package's actor, compiled when a document operation needs it and kept within the
/// catalog's residency budget ([`GuestResidencyLedgerV1`]). Its codec rows that neither a linked native
/// codec nor this engine's verification memory pinned at load (`rows`) are pinned against the component's
/// own `pack-schema-hash` once, before its first codec call — by the catalog's background verification or
/// by that call, whichever comes first — so a hub serves without interpreting its whole catalog first and
/// no codec call ever runs on a row its component did not answer as the trust record says.
struct GuestArtifactComponent {
    runtime: Arc<semio_framework_plugin_host::OwnedRuntime>,
    package: PackageRef,
    component: TrustedCatalogAsset,
    compiled: GuestResidencyV1<semio_framework_plugin_host::CompiledHandle>,
    position: usize,
    component_sha256: [u8; 32],
    rows: Vec<(String, [u8; 32])>,
    verified: tokio::sync::OnceCell<Result<(), String>>,
    verifications: Arc<GuestCodecVerificationCacheV1>,
    progress: Arc<TrustedCatalogLoadProgressCellV1>,
}

impl GuestArtifactComponent {
    async fn compiled(&self, context: &OperationContext<'_>) -> Result<Arc<semio_framework_plugin_host::CompiledHandle>, AuthorityError> {
        self.compiled
            .acquire(context, || async {
                let bytes = self.component.read(context).await?;
                let (runtime, package) = (Arc::clone(&self.runtime), self.package.clone());
                interpret_off_worker(context, move |_handle, _progress| runtime.compile_component(&package, &bytes).map_err(semio_framework_plugin_host::TurnFault::Host)).await?.map_err(|error| catalog_error(format!("{}: {error}", self.package.package.0)))
            })
            .await
    }

    /// 🔐️ Every row of `rows` pinned against this component's own answer, once for the catalog's lifetime. A
    /// caller that cancels or stalls leaves the rows for the next caller; a component that answers a row
    /// differently from its trust record, or does not compile, refuses every codec call of its package.
    async fn verified(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        if self.rows.is_empty() {
            return Ok(());
        }
        let outcome = self
            .verified
            .get_or_try_init(|| async {
                self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Verifying);
                match self.verify_rows(context).await {
                    Ok(()) => {
                        self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Ready);
                        Ok(Ok(()))
                    }
                    Err(error @ (AuthorityError::Cancelled | AuthorityError::Stalled | AuthorityError::DeadlineExceeded)) => {
                        self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Staged);
                        Err(error)
                    }
                    Err(AuthorityError::Catalog(refusal)) => {
                        self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Refused);
                        Ok(Err(refusal))
                    }
                    Err(error) => {
                        self.progress.phase(self.position, TrustedCatalogPackagePhaseV1::Refused);
                        Ok(Err(error.to_string()))
                    }
                }
            })
            .await?;
        outcome.clone().map_err(AuthorityError::Catalog)
    }

    /// 🔐️ Interprets every pending row on one compile, several at once ([`guest_verification_concurrency`]),
    /// compares each answer with its trust record and remembers it for this engine.
    async fn verify_rows(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        use futures::StreamExt;
        let schemas = self.rows.iter().map(|(schema, _)| schema.as_str()).collect::<Vec<_>>().join(", ");
        let compiled = self.compiled(context).await.map_err(|error| catalog_error(format!("{schemas}: {error}")))?;
        let rows = self.rows.iter().map(|(schema, expected)| {
            let compiled = Arc::clone(&compiled);
            async move {
                context.checkpoint()?;
                let (runtime, row_schema) = (Arc::clone(&self.runtime), schema.clone());
                let observed = interpret_off_worker(context, move |handle, progress| handle.block_on(runtime.codec_pack_schema_hash_observed(&compiled, &row_schema, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel))))
                    .await?
                    .map_err(|error| catalog_error(format!("{schema}: {error}")))?;
                context.checkpoint()?;
                if observed != *expected {
                    return Err(catalog(&format!("{schema}: guest artifact codec schema hash differs from its trust record")));
                }
                self.verifications.remember(&self.component_sha256, schema, &observed).await;
                self.progress.package(self.position, |package| package.rows_verified = package.rows_verified.saturating_add(1));
                Ok::<_, AuthorityError>(())
            }
        });
        let mut running = futures::stream::iter(rows).buffer_unordered(guest_verification_concurrency());
        while let Some(row) = running.next().await {
            row?;
        }
        Ok(())
    }
}

/// 🧩️ The component half of one artifact identity: the package's own actor plus the document
/// schema that selects the owning app inside it. Every verified codec carries one, because `genesis`
/// now comes from the component for EVERY package (ticket 26/09/18 slice TC3b) — the compiled-in
/// stdio/gis/vcs genesis table it replaced could only ever answer for three packages.
pub struct GuestArtifactCodecBinding {
    component: Arc<GuestArtifactComponent>,
    artifact_schema: String,
}

impl GuestArtifactCodecBinding {
    /// 🌱️ The guest's canonical empty document. The call is the longest single thing a creation
    /// does — the interpreter walks a ≈ 48 MB component's whole app bundle — so the guest's own
    /// fuel progress is reported into the caller's context as it happens: under a stall bound a
    /// checkpoint is what says "still moving", and without these an honest interpreter would look
    /// exactly like a wedged one. Cancellation is not read inside the interpretation: a caller that
    /// stops waiting is released at once and the call ends on its own fuel bound.
    async fn genesis(&self, document_id: &str, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, document_id) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), document_id.to_string());
        let pair = interpret_off_worker(context, move |handle, progress| handle.block_on(runtime.codec_genesis_observed(&compiled, &schema, &document_id, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel))))
            .await?
            .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: pair.pack, spr: pair.spr })
    }

    async fn print_mirror(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        let (runtime, schema, pack, spr) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), pair.pack.clone(), pair.spr.clone());
        let mirror = interpret_off_worker(context, move |handle, progress| handle.block_on(runtime.codec_print_mirror_observed(&compiled, &schema, &pack, &spr, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel))))
            .await?
            .map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        Ok(())
    }

    async fn apply_ops(&self, pair: ArtifactPair, encoded: Vec<u8>, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone());
        let next = interpret_off_worker(context, move |handle, progress| handle.block_on(runtime.codec_apply_ops_observed(&compiled, &schema, &pair.pack, &pair.spr, &encoded, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel))))
            .await?
            .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: next.pack, spr: next.spr })
    }

    /// 📜️ The guest's own replica fold of a ledger stream; fuel progress reaches the caller's stall
    /// bound exactly as [`Self::genesis`]'s does.
    async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        self.component.verified(context).await?;
        let compiled = self.component.compiled(context).await?;
        let (runtime, schema, envelopes) = (Arc::clone(&self.component.runtime), self.artifact_schema.clone(), envelopes.to_vec());
        let next = interpret_off_worker(context, move |handle, progress| {
            handle.block_on(runtime.codec_replay_envelopes_observed(&compiled, &schema, &pair.pack, &pair.spr, &envelopes, GUEST_CODEC_BUDGET, |fuel, _elapsed| progress(fuel)))
        })
        .await?
        .map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        Ok(ArtifactPair { pack: next.pack, spr: next.spr })
    }
}

/// 🧵️ Interprets one guest codec call on the hub runtime's blocking pool, never on the async worker
/// that awaits it: a call interprets for seconds to minutes, and on a worker it would stall every
/// request, socket and timer queued behind it (a sign-in, a presence heartbeat, an edit's ack). The
/// call's fuel observations reach the caller's context while it runs — each one a checkpoint of its
/// stall bound — and the first observation after the caller cancelled (or stalled) releases the caller
/// with that outcome while the call ends on its own fuel bound; dropping the caller releases it at once.
/// The outer result is the caller's own outcome, the inner one the guest's answer. Guest codec calls
/// belong to the hub's runtime; outside one they are refused.
async fn interpret_off_worker<T, F>(context: &OperationContext<'_>, call: F) -> Result<Result<T, semio_framework_plugin_host::TurnFault>, AuthorityError>
where
    T: Send + 'static,
    F: FnOnce(&tokio::runtime::Handle, &mut dyn FnMut(u64)) -> Result<T, semio_framework_plugin_host::TurnFault> + Send + 'static,
{
    let handle = tokio::runtime::Handle::try_current().map_err(|_| catalog("guest codec calls run on the hub runtime"))?;
    let (sender, mut observations) = tokio::sync::mpsc::unbounded_channel::<u64>();
    let blocking = handle.clone();
    let joined = handle.spawn_blocking(move || {
        call(&blocking, &mut |fuel| {
            let _ = sender.send(fuel);
        })
    });
    while let Some(fuel) = observations.recv().await {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::GuestCodecExecuting, completed_units: fuel.min(GUEST_CODEC_BUDGET.fuel), total_units: GUEST_CODEC_BUDGET.fuel })?;
    }
    Ok(joined.await.unwrap_or_else(|error| Err(semio_framework_plugin_host::TurnFault::Trapped(format!("guest codec call ended abnormally: {error}")))))
}

/// 🧪️ One immutable authority identity bound to its executable. `codec` is `Some` only for a package
/// this binary links a Rust codec for (stdio import/export and GIS inference still run natively, TC2
/// §9); every other package validates and applies through its own component. `guest` is never
/// optional: it is the only creation authority there is.
pub struct VerifiedNativeArtifactCodec {
    identity: TrustedArtifactIdentity,
    codec: Option<ArtifactCodec>,
    guest: GuestArtifactCodecBinding,
}

impl TrustedArtifactCodec for VerifiedNativeArtifactCodec {
    fn identity(&self) -> &TrustedArtifactIdentity {
        &self.identity
    }

    async fn validate_pair(&self, pair: &ArtifactPair, stage: ArtifactValidationStage, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        let Some(codec) = &self.codec else {
            self.guest.print_mirror(pair, stage, context).await?;
            return context.checkpoint();
        };
        let mirror = (codec.print_mirror)(&pair.pack, &pair.spr).await.map_err(|error| AuthorityError::Codec { stage, message: bounded_message(error) })?;
        if mirror.dsl.len().checked_add(mirror.ops.len()).is_none_or(|length| length > AUTHORITY_MAX_CODEC_TEXT_BYTES) {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()
    }

    async fn apply_operation(&self, pair: ArtifactPair, operation: &AcceptedArtifactOperation, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let encoded = directory::os_spr::encode_ops_vec(std::slice::from_ref(&operation.encoded));
        let Some(codec) = &self.codec else {
            let next = self.guest.apply_ops(pair, encoded, context).await?;
            context.checkpoint()?;
            return Ok(next);
        };
        let (pack, spr, ops) = (codec.apply_ops_binary)(&pair.pack, &pair.spr, &encoded).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

impl TrustedArtifactReplayCodec for VerifiedNativeArtifactCodec {
    async fn replay_envelopes(&self, pair: ArtifactPair, envelopes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let Some(codec) = &self.codec else {
            let next = self.guest.replay_envelopes(pair, envelopes, context).await?;
            context.checkpoint()?;
            return Ok(next);
        };
        let (pack, spr, ops) = (codec.replay_envelopes)(&pair.pack, &pair.spr, envelopes).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Output, message: bounded_message(error) })?;
        if ops.len() > AUTHORITY_MAX_CODEC_TEXT_BYTES {
            return Err(AuthorityError::ResourceLimit("codec text byte"));
        }
        context.checkpoint()?;
        Ok(ArtifactPair { pack, spr })
    }
}

impl TrustedArtifactGenesisCodec for VerifiedNativeArtifactCodec {
    /// 🌱️ Creation authority is the COMPONENT's, always. The `dialect` the caller carries is not
    /// passed to the guest: the guest stamps its own app's dialect, and a genesis whose dialect
    /// differs from the catalog's open target is refused here rather than silently accepted.
    async fn initial_pair(&self, document_id: &str, dialect: &directory::os_io::ArtifactDialect, context: &OperationContext<'_>) -> Result<ArtifactPair, AuthorityError> {
        context.checkpoint()?;
        let pair = self.guest.genesis(document_id, context).await?;
        context.checkpoint()?;
        let parsed = directory::os_spr::decode_history(&pair.spr, &directory::os_spr::DecodeOptions::default()).await.map_err(|error| AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: bounded_message(error) })?;
        if parsed.doc_id != document_id || !parsed.edits.is_empty() || !parsed.transitions.is_empty() || !parsed.conflicts.is_empty() {
            return Err(AuthorityError::Codec { stage: ArtifactValidationStage::Input, message: "guest genesis is not a zero-history document of the requested identity".into() });
        }
        let _ = dialect;
        context.checkpoint()?;
        Ok(pair)
    }
}

/// 🗂️ Process-lifetime snapshot produced only after complete bundle verification and codec activation.
pub struct VerifiedTrustedCatalog {
    residency: Arc<GuestResidencyLedgerV1<semio_framework_plugin_host::CompiledHandle>>,
    guests: Box<[Arc<GuestArtifactComponent>]>,
    progress: Arc<TrustedCatalogLoadProgressCellV1>,
    packages: Box<[VerifiedTrustedPackage]>,
    codecs: Box<[VerifiedNativeArtifactCodec]>,
    open_targets: Box<[VerifiedDocumentOpenSelectionV1]>,
    generation_id: String,
}

/// 🧱 The exact verified component, raw descriptor bytes and closed actor bound to one current
/// selection. It is produced only by [`VerifiedTrustedCatalog::assets_for_current_selection`] and
/// carries no path, origin or catalog handle; the component and actor are read on demand.
pub struct VerifiedExecutionTargetAssets {
    pub selection: VerifiedDocumentOpenSelectionV1,
    pub component: TrustedCatalogAsset,
    pub descriptor: Arc<[u8]>,
    pub browser_actor: Option<TrustedCatalogAsset>,
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

    /// 📏️ What the compiled-guest residency holds now and has done since the catalog loaded.
    pub fn guest_residency(&self) -> TrustedCatalogGuestResidencyStateV1 {
        self.residency.state()
    }

    /// 🎚️ Applies the operator's residency budget (`OS_HUB_GUEST_RESIDENCY_BYTES`), releasing least recently
    /// used unheld guests until the resident charge fits it.
    pub fn configure_guest_residency(&self, residency: TrustedCatalogGuestResidencyV1) {
        self.residency.configure(residency.resident_component_bytes);
    }

    /// 📈️ How far this catalog's load and codec-row verification have come.
    pub fn load_progress(&self) -> TrustedCatalogLoadProgressV1 {
        self.progress.snapshot()
    }

    /// 🔐️ Pins every package's rows that still wait for their component's answer, smallest component first, so the
    /// most packages are ready soonest; a codec call that reaches a package first verifies it itself and this
    /// pass waits for it. A refused package is recorded in [`Self::load_progress`] and the pass moves on; the
    /// pass stops at cancellation or a stall of `context`.
    pub async fn verify_pending_guests(&self, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        let mut pending: Vec<&Arc<GuestArtifactComponent>> = self.guests.iter().filter(|guest| !guest.rows.is_empty() && guest.verified.get().is_none()).collect();
        pending.sort_by_key(|guest| guest.component.byte_length());
        for guest in pending {
            context.checkpoint()?;
            match guest.verified(context).await {
                Ok(()) | Err(AuthorityError::Catalog(_)) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
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

    /// 📇️ Every plugin module of this generation in ascending plugin order: what
    /// `GET /trusted-catalog/plugin-modules` answers.
    pub fn plugin_module_index(&self) -> TrustedPluginModuleIndexV1 {
        let mut modules = self
            .packages
            .iter()
            .map(|package| TrustedPluginModuleIndexEntryV1 {
                plugin_id: package.plugin_id.clone(),
                package_id: package.package.package.0.clone(),
                version: package.version.clone(),
                component_sha256: hex_lower(&package.component_sha256),
                descriptor_byte_sha256: hex_lower(&package.descriptor_sha256),
                dependencies: package.dependencies.clone(),
                dialect_artifact_kinds: package.descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).collect::<BTreeSet<_>>().into_iter().collect(),
                extends_plugin_id: (package.descriptor.role == semio_framework::PackageRole::Extension).then(|| package.descriptor.manifest.dependencies.first().map(|dependency| dependency.plugin_id.clone())).flatten(),
                bundle_sha256: package.plugin_module.bundle_sha256.clone(),
                bundle_byte_length: package.plugin_module.manifest_bytes.len() as u64,
                entry: package.plugin_module.bundle.entry.clone(),
            })
            .collect::<Vec<_>>();
        modules.sort_by(|left, right| left.plugin_id.as_bytes().cmp(right.plugin_id.as_bytes()));
        TrustedPluginModuleIndexV1 { schema: TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA.into(), generation_id: self.generation_id.clone(), modules }
    }

    /// 🧩️ The verified plugin module whose manifest has this SHA-256, when this generation carries it.
    pub fn plugin_module(&self, bundle_sha256: &str) -> Option<&VerifiedPluginModule> {
        self.packages.iter().map(|package| &package.plugin_module).find(|module| module.bundle_sha256 == bundle_sha256)
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
                    codec.identity.plugin_id == selection.package.plugin_id
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
        if package.component.byte_length() == 0 || package.descriptor_bytes.is_empty() || package.component.byte_length() > TRUSTED_COMPONENT_MAX_BYTES || package.descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
            return None;
        }
        let browser_actor = match (&selection.browser_actor, &package.browser_actor, &package.browser_actor_asset) {
            (DocumentOpenBrowserActorV1::None, _, _) if !matches!(selection.surface.renderer_target, DocumentOpenRendererTargetV1::Wasm) => None,
            (selected, retained, Some(asset)) if selected == retained && matches!(selected, DocumentOpenBrowserActorV1::ClosedBrowserActor { .. }) && asset.byte_length() != 0 && asset.byte_length() <= DOCUMENT_BROWSER_ACTOR_MAX_BYTES => {
                Some(asset.clone())
            }
            _ => return None,
        };
        Some(VerifiedExecutionTargetAssets { selection, component: package.component.clone(), descriptor: Arc::clone(&package.descriptor_bytes), browser_actor })
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

/// 🪪️ What a remembered verification was computed by: the owned engine's semantic identity and the
/// fuel ceiling every guest codec call runs under.
pub(crate) fn guest_codec_engine_identity() -> String {
    format!("{}:fuel-{}", semio_framework_plugin_host::owned_engine_identity(), GUEST_CODEC_BUDGET.fuel)
}

/// 🗃️ The hub's content-addressed memory of guest codec verifications (`GuestCodecVerificationV1`),
/// one file per verification under `<hub data>/trusted-catalog/guest-codec-verifications/`, named by
/// the SHA-256 of its canonical key. A record is used only when its component SHA-256 — taken from the
/// bytes this very load re-read and re-hashed — its artifact schema and its engine all match. The
/// engine is the owned interpreter's semantic identity plus the codec budget, never the executable's
/// path, inode or signature, so a copied, re-signed or container build of the same engine boots warm
/// on a catalog another build verified or published, and a changed engine verifies afresh. The records
/// live inside `trusted-catalog/` so they travel with a copied or restored catalog. Reading or writing a
/// record never decides a verification: an unreadable record is a miss, and a record that cannot be
/// written is recomputed on the next boot.
#[derive(Clone)]
pub(crate) struct GuestCodecVerificationCacheV1 {
    root: Option<std::path::PathBuf>,
    engine: String,
}

impl GuestCodecVerificationCacheV1 {
    /// 📂️ The cache inside a hub's trusted catalog, keyed to the owned engine that interprets its guests.
    pub(crate) fn beside(data_path: &Path) -> Self {
        Self { root: Some(data_path.join("trusted-catalog").join("guest-codec-verifications")), engine: guest_codec_engine_identity() }
    }

    /// 🚫️ No memory at all: every verification interprets its component.
    pub(crate) fn disabled() -> Self {
        Self { root: None, engine: String::new() }
    }

    #[cfg(test)]
    pub(crate) fn at(root: std::path::PathBuf, engine: &str) -> Self {
        Self { root: Some(root), engine: engine.into() }
    }

    fn record(&self, component_sha256: &[u8; 32], artifact_schema: &str, pack_schema_hash: &[u8; 32]) -> GuestCodecVerificationV1 {
        GuestCodecVerificationV1 { schema: GUEST_CODEC_VERIFICATION_SCHEMA.into(), component_sha256: hex_lower(component_sha256), artifact_schema: artifact_schema.into(), engine: self.engine.clone(), pack_schema_hash: hex_lower(pack_schema_hash) }
    }

    fn path(&self, component_sha256: &[u8; 32], artifact_schema: &str) -> Option<std::path::PathBuf> {
        let root = self.root.as_ref()?;
        let key = serde_json::to_vec(&(GUEST_CODEC_VERIFICATION_SCHEMA, hex_lower(component_sha256), artifact_schema, &self.engine)).ok()?;
        let mut hash = Sha256::new();
        hash.update(&key);
        Some(root.join(format!("{}.json", hex_lower(&hash.finalize()))))
    }

    /// 🔎️ The pack-schema hash remembered for exactly this component, schema and engine.
    pub(crate) async fn recall(&self, component_sha256: &[u8; 32], artifact_schema: &str) -> Option<[u8; 32]> {
        let bytes = tokio::fs::read(self.path(component_sha256, artifact_schema)?).await.ok()?;
        if bytes.len() > 4096 {
            return None;
        }
        let record: GuestCodecVerificationV1 = serde_json::from_slice(&bytes).ok()?;
        let hash = decode_digest(&record.pack_schema_hash, "remembered pack schema hash").ok()?;
        (record == self.record(component_sha256, artifact_schema, &hash) && hash != [0; 32]).then_some(hash)
    }

    /// 💾️ Remembers one verification: written to a sibling temporary file, then renamed into place.
    pub(crate) async fn remember(&self, component_sha256: &[u8; 32], artifact_schema: &str, pack_schema_hash: &[u8; 32]) -> bool {
        let Some(path) = self.path(component_sha256, artifact_schema) else { return false };
        let Ok(bytes) = serde_json::to_vec(&self.record(component_sha256, artifact_schema, pack_schema_hash)) else { return false };
        let staged = path.with_extension(format!("{}.tmp", std::process::id()));
        let written = async {
            tokio::fs::create_dir_all(path.parent().ok_or_else(|| std::io::Error::other("cache path has no parent"))?).await?;
            tokio::fs::write(&staged, &bytes).await?;
            tokio::fs::rename(&staged, &path).await
        }
        .await;
        if written.is_err() {
            let _ = tokio::fs::remove_file(&staged).await;
        }
        written.is_ok()
    }
}

/// 🧵️ How many codec rows of one package are interpreted at once: the declared residency bound, never
/// more than half the cores this hub may use, never fewer than one.
pub(crate) fn guest_verification_concurrency() -> usize {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    TRUSTED_CATALOG_GUEST_RESIDENCY.concurrent_verifications.min(cores / 2).max(1)
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
        let generation_root = Arc::new(data_root.open_generation(&current.generation_id)?);
        let bundle_path = TrustedCatalogRelativePathV1::parse("trusted-catalog.json")?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        if sha256(&bundle_bytes, context).await? != expected_bundle_sha256 {
            return Err(catalog("trusted bundle differs from the current pointer digest"));
        }
        let verifications = GuestCodecVerificationCacheV1::beside(data_path);
        let verified = Self::load_selected(&generation_root, bundle_path, bundle_bytes, &current.profile_id, providers, &verifications, context).await?;
        if verified.generation_id() != current.generation_id {
            return Err(catalog("trusted current pointer generation differs from the selected profile"));
        }
        Ok(Some(verified))
    }

    #[cfg(any(test, feature = "integration-fixtures"))]
    pub(crate) async fn load_fixture(bundle_path: &Path, profile_id: &str, providers: &dyn NativeCodecProviderSourceV1, context: &OperationContext<'_>) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let path = std::fs::canonicalize(bundle_path).map_err(catalog_error)?;
        let fixture_root = path.parent().ok_or_else(|| catalog("bundle has no containing directory"))?;
        let generation_root = Arc::new(TrustedCatalogGenerationRoot::open_fixture_owned(fixture_root)?);
        let bundle_path = TrustedCatalogRelativePathV1::parse(path.file_name().and_then(|name| name.to_str()).ok_or_else(|| catalog("fixture bundle name is not UTF-8"))?)?;
        let bundle_bytes = generation_root.read_regular(&bundle_path, TRUSTED_BUNDLE_MAX_BYTES, context).await?;
        Self::load_selected(&generation_root, bundle_path, bundle_bytes, profile_id, providers, &GuestCodecVerificationCacheV1::disabled(), context).await
    }

    async fn load_selected(
        root: &Arc<TrustedCatalogGenerationRoot>,
        bundle_path: TrustedCatalogRelativePathV1,
        bundle_bytes: Vec<u8>,
        profile_id: &str,
        providers: &dyn NativeCodecProviderSourceV1,
        verifications: &GuestCodecVerificationCacheV1,
        context: &OperationContext<'_>,
    ) -> Result<VerifiedTrustedCatalog, AuthorityError> {
        let (catalog, registration_codecs) = Self::verify_selected(root, bundle_path, bundle_bytes, profile_id, providers, verifications, context).await?;
        let assembly = os_store::begin_artifact_assembly().map_err(catalog_error)?;
        os_store::preflight_document_codecs_in_assembly(&assembly, &registration_codecs).map_err(catalog_error)?;
        context.checkpoint()?;
        os_store::register_document_codecs_in_assembly(&assembly, registration_codecs).map_err(catalog_error)?;
        Ok(catalog)
    }

    /// 📦️ One selected package whose files, digests, descriptor and browser actor are already
    /// verified, held until the whole closure is proven so no provider sees a package belonging to a
    /// closure that is still able to be refused. Component and actor bytes are not held: each was
    /// read, hashed and dropped, and is retained by identity.
    async fn verify_selected(
        root: &Arc<TrustedCatalogGenerationRoot>,
        bundle_path: TrustedCatalogRelativePathV1,
        bundle_bytes: Vec<u8>,
        profile_id: &str,
        providers: &dyn NativeCodecProviderSourceV1,
        verifications: &GuestCodecVerificationCacheV1,
        context: &OperationContext<'_>,
    ) -> Result<(VerifiedTrustedCatalog, Vec<ArtifactCodec>), AuthorityError> {
        context.report(AuthorityProgress { stage: AuthorityProgressStage::Preflight, completed_units: 0, total_units: 1 })?;
        let guest_runtime = Arc::new(semio_framework_plugin_host::OwnedRuntime::new());
        let residency = GuestResidencyLedgerV1::new(TRUSTED_CATALOG_GUEST_RESIDENCY);
        let progress = context.catalog_progress().unwrap_or_default();
        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let SelectedTrustedBundleV1 { package_indices: order, profile } = validate_bundle(&bundle, profile_id)?;
        progress.select(
            order
                .iter()
                .map(|index| {
                    let record = &bundle.packages[*index];
                    TrustedCatalogPackageProgressV1 { plugin_id: record.plugin_id.clone(), component_bytes: record.component.byte_length, phase: TrustedCatalogPackagePhaseV1::Pending, rows: u64::try_from(record.native_codecs.len()).unwrap_or(u64::MAX), rows_pinned: 0, rows_verified: 0 }
                })
                .collect(),
        );
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
        let mut packages = Vec::with_capacity(order.len());
        let mut codecs = Vec::new();
        let mut open_targets = Vec::new();
        let mut registration_codecs = Vec::new();
        let mut resolved_paths = BTreeSet::from([bundle_path]);

        struct StagedTrustedPackage<'a> {
            position: usize,
            record: &'a TrustedBundlePackageV1,
            component: TrustedCatalogAsset,
            component_sha256: [u8; 32],
            component_blake3: [u8; 32],
            descriptor_bytes: Vec<u8>,
            descriptor_sha256: [u8; 32],
            descriptor: PackageDescriptor,
            browser_actor: DocumentOpenBrowserActorV1,
            browser_actor_asset: Option<TrustedCatalogAsset>,
            plugin_module: VerifiedPluginModule,
        }

        let mut staged = Vec::with_capacity(order.len());
        let mut plugin_module_files = BTreeMap::new();
        for (position, index) in order.into_iter().enumerate() {
            context.checkpoint()?;
            progress.phase(position, TrustedCatalogPackagePhaseV1::Reading);
            let record = &bundle.packages[index];
            let component_path = TrustedCatalogRelativePathV1::parse(&record.component.path)?;
            if !resolved_paths.insert(component_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let component_bytes = root.read_regular(&component_path, TRUSTED_COMPONENT_MAX_BYTES, context).await?;
            verify_length(record.component.byte_length, component_bytes.len())?;
            let (component_sha256, component_blake3) = dual_hash(&component_bytes, context).await?;
            drop(component_bytes);
            verify_digest(&record.component.sha256, component_sha256, "component sha256")?;
            verify_digest(&record.component.blake3, component_blake3, "component blake3")?;
            let component = TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), component_path, record.component.byte_length, component_sha256, Some(component_blake3)));
            progress.component_read(position);
            report_package_progress(context, position, 1, total_units)?;

            let descriptor_path = TrustedCatalogRelativePathV1::parse(&record.descriptor.path)?;
            if !resolved_paths.insert(descriptor_path.clone()) {
                return Err(catalog("trusted file path is already used by the selected closure"));
            }
            let descriptor_bytes = root.read_regular(&descriptor_path, TRUSTED_DESCRIPTOR_MAX_BYTES, context).await?;
            verify_length(record.descriptor.byte_length, descriptor_bytes.len())?;
            let descriptor_sha256 = sha256(&descriptor_bytes, context).await?;
            verify_digest(&record.descriptor.sha256, descriptor_sha256, "descriptor sha256")?;
            let descriptor = decode_package_descriptor(&descriptor_bytes)?;
            validate_descriptor(record, &descriptor, &bundle.packages)?;
            report_package_progress(context, position, 2, total_units)?;

            let browser_actor = record.browser_actor.identity();
            record.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &hex_lower(&component_sha256), descriptor_byte_sha256: &hex_lower(&descriptor_sha256) }, package_actor_renderer(record))?;
            let browser_actor_asset = if let Some(file) = record.browser_actor.file() {
                let actor_path = TrustedCatalogRelativePathV1::parse(&file.path)?;
                if !resolved_paths.insert(actor_path.clone()) {
                    return Err(catalog("trusted browser actor path is already used by the selected closure"));
                }
                let bytes = root.read_regular(&actor_path, file.byte_length, context).await?;
                verify_length(file.byte_length, bytes.len())?;
                let actor_sha256 = sha256(&bytes, context).await?;
                drop(bytes);
                verify_digest(&file.sha256, actor_sha256, "browser actor sha256")?;
                Some(TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), actor_path, file.byte_length, actor_sha256, None)))
            } else {
                None
            };
            let plugin_module = verify_plugin_module(root, record, &hex_lower(&component_sha256), &hex_lower(&descriptor_sha256), &mut resolved_paths, &mut plugin_module_files, context).await?;
            report_package_progress(context, position, 3, total_units)?;
            progress.phase(position, TrustedCatalogPackagePhaseV1::Staged);
            staged.push(StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module });
        }

        let mut previews = Vec::with_capacity(staged.len());
        let mut pending_rows = BTreeMap::new();
        for stage in &staged {
            context.checkpoint()?;
            let native_bindings = providers.preview(NativeCodecProviderPackageV1 { plugin_id: &stage.record.plugin_id, package_id: &stage.record.package_id, version: &stage.record.version }, &stage.descriptor, context)?;
            context.checkpoint()?;
            let bound = validate_native_bindings(&native_bindings)?.into_keys().collect::<BTreeSet<_>>();
            let mut rows = Vec::new();
            for expected in &stage.record.native_codecs {
                if bound.contains(&CodecKey::from_parts(&stage.record.plugin_id, &stage.record.package_id, &expected.artifact_kind, &expected.artifact_schema)) {
                    continue;
                }
                let expected_hash = decode_digest(&expected.pack_schema_hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                if verifications.recall(&stage.component_sha256, &expected.artifact_schema).await == Some(expected_hash) {
                    continue;
                }
                rows.push((expected.artifact_schema.clone(), expected_hash));
            }
            let pinned = u64::try_from(stage.record.native_codecs.len() - rows.len()).unwrap_or(u64::MAX);
            progress.package(stage.position, |package| {
                package.rows_pinned = pinned;
                if rows.is_empty() {
                    package.phase = TrustedCatalogPackagePhaseV1::Ready;
                }
            });
            pending_rows.insert(stage.position, rows);
            previews.push(native_bindings);
        }
        let verifications = Arc::new(verifications.clone());
        let mut guests = Vec::with_capacity(staged.len());

        for (stage, native_bindings) in staged.into_iter().zip(previews) {
            let StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module } = stage;
            context.checkpoint()?;
            let binding_map = validate_native_bindings(&native_bindings)?;
            let mut consumed_bindings = BTreeSet::new();
            let package_ref = PackageRef { package: PackageId(record.package_id.clone()), hash: PackageHash(component_blake3) };
            let guest = Arc::new(GuestArtifactComponent {
                runtime: Arc::clone(&guest_runtime),
                package: package_ref.clone(),
                component: component.clone(),
                compiled: residency.register(record.component.byte_length),
                position,
                component_sha256,
                rows: pending_rows.remove(&position).unwrap_or_default(),
                verified: tokio::sync::OnceCell::new(),
                verifications: Arc::clone(&verifications),
                progress: Arc::clone(&progress),
            });
            guests.push(Arc::clone(&guest));

            for expected in &record.native_codecs {
                if codecs.len() >= TRUSTED_CATALOG_MAX_CODECS {
                    return Err(AuthorityError::ResourceLimit("trusted codec count"));
                }
                let key = CodecKey::from_parts(&record.plugin_id, &record.package_id, &expected.artifact_kind, &expected.artifact_schema);
                let expected_hash = decode_digest(&expected.pack_schema_hash, "pack schema hash")?;
                if expected_hash == [0; 32] {
                    return Err(catalog("artifact codec schema hash is zero"));
                }
                let binding = binding_map.get(&key);
                if let Some(binding) = binding {
                    if binding.codec.pack_schema_hash == [0; 32] || binding.codec.pack_schema_hash != expected_hash || binding.codec.schema != expected.artifact_schema {
                        return Err(catalog("native codec schema hash is zero or mismatched"));
                    }
                    consumed_bindings.insert(key);
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
                if let Some(binding) = binding {
                    registration_codecs.push(binding.codec.clone());
                }
                codecs.push(VerifiedNativeArtifactCodec {
                    identity,
                    codec: binding.map(|binding| binding.codec.clone()),
                    guest: GuestArtifactCodecBinding { component: Arc::clone(&guest), artifact_schema: expected.artifact_schema.clone() },
                });
            }
            if consumed_bindings.len() != binding_map.len() {
                return Err(catalog("selected provider returned a binding outside its exact declared package closure"));
            }
            for target in &record.open_targets {
                if open_targets.len() >= TRUSTED_CATALOG_MAX_OPEN_TARGETS {
                    return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
                }
                let parent_dialect = validate_descriptor_open_target(&descriptor, target)?;
                if !profile.open_targets.iter().any(|selected| {
                    selected.package.plugin_id == record.plugin_id && selected.package.package_id == record.package_id && selected.package.version == record.version && selected.target == *target
                }) {
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
                package: package_ref,
                version: record.version.clone(),
                dependencies: record.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect(),
                plugin_module,
                component_sha256,
                descriptor_sha256,
                component,
                descriptor_bytes: descriptor_bytes.into(),
                browser_actor,
                browser_actor_asset,
                descriptor: Arc::new(descriptor),
            });
        }
        if codecs.is_empty() {
            return Err(catalog("selected profile exposes no executable artifact codec"));
        }
        sort_open_targets(&mut open_targets);
        if open_targets.len() != profile.open_targets.len() {
            return Err(catalog("selected profile resolved a different number of document-open targets than it declares"));
        }
        if open_targets.is_empty() {
            return Err(catalog("selected profile must resolve at least one document-open target"));
        }
        let generation_id = trusted_profile_generation(&bundle, &profile)?;
        if generation_id != profile.generation_id {
            return Err(catalog("trusted profile generation differs from the completely verified package, codec, and target closure"));
        }
        let catalog = VerifiedTrustedCatalog { residency, guests: guests.into_boxed_slice(), progress, packages: packages.into_boxed_slice(), codecs: codecs.into_boxed_slice(), open_targets: open_targets.into_boxed_slice(), generation_id };
        context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogResolved, completed_units: total_units, total_units })?;
        Ok((catalog, registration_codecs))
    }
}

/// 🧩️ Reads, verifies and retains one package's plugin module: the manifest against its record and the
/// package's own verified digests, then every file against the manifest. A file several modules share
/// (the vendored imports, the fonts) is one content-addressed file, verified once per load.
async fn verify_plugin_module(
    root: &Arc<TrustedCatalogGenerationRoot>,
    record: &TrustedBundlePackageV1,
    component_sha256: &str,
    descriptor_sha256: &str,
    resolved_paths: &mut BTreeSet<TrustedCatalogRelativePathV1>,
    verified_files: &mut BTreeMap<String, (TrustedPluginModuleFileV1, TrustedCatalogAsset)>,
    context: &OperationContext<'_>,
) -> Result<VerifiedPluginModule, AuthorityError> {
    let manifest_path = TrustedCatalogRelativePathV1::parse(&record.plugin_module.path)?;
    if !resolved_paths.insert(manifest_path.clone()) {
        return Err(catalog("trusted plugin module path is already used by the selected closure"));
    }
    let manifest_bytes = root.read_regular(&manifest_path, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, context).await?;
    verify_length(record.plugin_module.byte_length, manifest_bytes.len())?;
    let (manifest_sha256, manifest_blake3) = dual_hash(&manifest_bytes, context).await?;
    verify_digest(&record.plugin_module.sha256, manifest_sha256, "plugin module sha256")?;
    verify_digest(&record.plugin_module.blake3, manifest_blake3, "plugin module blake3")?;
    let source = TrustedPluginModuleSourceV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version, component_sha256, descriptor_byte_sha256: descriptor_sha256 };
    let bundle = decode_plugin_module_bundle(&manifest_bytes, source)?;
    let mut files = BTreeMap::new();
    for file in &bundle.files {
        context.checkpoint()?;
        let asset = match verified_files.get(&file.sha256) {
            Some((known, asset)) if known.byte_length == file.byte_length && known.blake3 == file.blake3 => asset.clone(),
            Some(_) => return Err(catalog("trusted plugin module files disagree about one content address")),
            None => {
                let path = TrustedCatalogRelativePathV1::parse(&plugin_module_blob_path(&file.sha256))?;
                if !resolved_paths.insert(path.clone()) {
                    return Err(catalog("trusted plugin module file path is already used by the selected closure"));
                }
                let bytes = root.read_regular(&path, file.byte_length, context).await?;
                let (sha256, blake3) = dual_hash(&bytes, context).await?;
                verify_plugin_module_file(file, bytes.len(), sha256, blake3)?;
                drop(bytes);
                let asset = TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), path, file.byte_length, sha256, Some(blake3)));
                verified_files.insert(file.sha256.clone(), (file.clone(), asset.clone()));
                asset
            }
        };
        files.insert(file.path.clone(), asset);
    }
    Ok(VerifiedPluginModule { bundle_sha256: hex_lower(&manifest_sha256), manifest_bytes: manifest_bytes.into(), bundle, files })
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

/// 🗂️ The one pairing rule between a verified descriptor's artifact kinds and its surfaces, shared by
/// publication ([`descriptor_open_targets`]) and verification ([`validate_descriptor_open_target`]). A
/// plugin-level kind (`PluginBuilder::artifact_kind`, the channel GIS still uses) is opened only by the
/// surfaces whose own dialect names it, even when a sibling app lists it as an input (GIS's terrain editor
/// lists the map). Any other kind is opened by the editor that declares it itself (where a plugin migrated
/// onto the declaration tree, ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, stitches its spec)
/// and by the viewers of that editor's dialect, the read-only surface of the same documents.
fn app_opens_kind(descriptor: &PackageDescriptor, app: &semio_framework::AppDefinition, artifact_kind: &str, artifact_schema: &str) -> bool {
    let declares = |kinds: &[semio_framework::ArtifactKindSpec]| kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);
    if declares(&descriptor.manifest.artifact_kinds) {
        return app.dialect.artifact_kind == artifact_kind;
    }
    match app.role {
        semio_framework::AppRole::Editor => declares(&app.artifact_kinds),
        semio_framework::AppRole::Viewer => declares(&app.artifact_kinds) || descriptor.manifest.apps.iter().any(|editor| editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect && declares(&editor.artifact_kinds)),
    }
}

/// 🎯️ Every document-open surface one verified descriptor declares, in app then kind declaration order:
/// each (editor or viewer, kind) pair [`app_opens_kind`] admits, in the app's first window kind, rendered by
/// the catalog's closed wasm actor, with the grant its role fixes (a viewer reads and observes, never writes),
/// so a read-only member opens the same document through its viewer. `os-hub trusted-catalog open-targets`
/// answers exactly this list, so a publisher never re-derives the rule.
pub fn descriptor_open_targets(descriptor: &PackageDescriptor) -> Vec<schema::TrustedDescriptorOpenTargetV1> {
    if descriptor.execution != semio_framework::ExecutionMode::Isolated {
        return Vec::new();
    }
    let mut targets = Vec::new();
    for app in &descriptor.manifest.apps {
        if app.id != semio_framework::surface_app_id(&app.dialect, app.role) {
            continue;
        }
        let role = match app.role {
            semio_framework::AppRole::Viewer => TrustedBundleOpenRole::Viewer,
            semio_framework::AppRole::Editor => TrustedBundleOpenRole::Editor,
        };
        let mut seen = BTreeSet::new();
        let editors = descriptor.manifest.apps.iter().filter(|editor| app.role == semio_framework::AppRole::Viewer && editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect);
        for kind in descriptor.manifest.artifact_kinds.iter().chain(app.artifact_kinds.iter()).chain(editors.flat_map(|editor| editor.artifact_kinds.iter())) {
            if !seen.insert((kind.id.as_str(), kind.schema.as_str())) || !app_opens_kind(descriptor, app, &kind.id, &kind.schema) {
                continue;
            }
            targets.push(schema::TrustedDescriptorOpenTargetV1 {
                artifact_kind: kind.id.clone(),
                artifact_schema: kind.schema.clone(),
                surface_id: app.id.clone(),
                app_id: app.id.clone(),
                window_kind_id: app.window_kinds.first().id.clone(),
                role,
                renderer_target: TrustedBundleRendererTarget::Wasm,
                parent_dialect: app.dialect.clone(),
                grant: TrustedBundleGrantV1 { read: true, write: matches!(role, TrustedBundleOpenRole::Editor), observe: true },
            });
        }
    }
    targets
}

/// 📤️ Answers `os-hub trusted-catalog open-targets`: decodes one bounded descriptor exactly as the loader
/// does and serializes [`descriptor_open_targets`] as `TrustedCatalogDescriptorOpenTargetsV1`.
pub fn descriptor_open_targets_answer(descriptor_bytes: &[u8]) -> Result<Vec<u8>, AuthorityError> {
    if descriptor_bytes.is_empty() || descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
        return Err(catalog("descriptor bytes are empty or exceed the trusted descriptor bound"));
    }
    let descriptor = decode_package_descriptor(descriptor_bytes)?;
    let targets = descriptor_open_targets(&descriptor);
    if targets.len() > TRUSTED_CATALOG_MAX_OPEN_TARGETS {
        return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
    }
    let mut bytes = serde_json::to_vec(&schema::TrustedCatalogDescriptorOpenTargetsV1 { schema: schema::TRUSTED_CATALOG_DESCRIPTOR_OPEN_TARGETS_SCHEMA, targets }).map_err(catalog_error)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn validate_descriptor_open_target(descriptor: &PackageDescriptor, target: &TrustedBundleOpenTargetV1) -> Result<semio_framework::ArtifactDialect, AuthorityError> {
    let expected_role = match target.role {
        TrustedBundleOpenRole::Viewer => semio_framework::AppRole::Viewer,
        TrustedBundleOpenRole::Editor => semio_framework::AppRole::Editor,
    };
    let app = descriptor.manifest.apps.iter().find(|app| app.id == target.app_id).ok_or_else(|| catalog("document-open target app is absent from the verified descriptor"))?;
    if !app_opens_kind(descriptor, app, &target.artifact_kind, &target.artifact_schema)
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

/// 🎯️ The whole SET, in one canonical order, so a generation id names every creatable kind it
/// admits — not merely the first (ticket 26/09/18 slice TC3b). The count is framed ahead of the
/// rows exactly as the selected closure's is, so adding a target can never collide with a
/// different bundle whose rows happen to concatenate identically.
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
        append_document_open_catalog_field(&mut encoded, package.plugin_module.path.as_bytes())?;
        encoded.extend_from_slice(&package.plugin_module.byte_length.to_be_bytes());
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.sha256, "profile plugin module sha256")?.as_slice())?;
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.blake3, "profile plugin module blake3")?.as_slice())?;
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
    let mut selected = profile.open_targets.iter().collect::<Vec<_>>();
    selected.sort_by(|left, right| {
        (&left.package.plugin_id, &left.package.package_id, &left.package.version, &left.target.artifact_kind, &left.target.artifact_schema, &left.target.surface_id, left.target.role as u8)
            .cmp(&(&right.package.plugin_id, &right.package.package_id, &right.package.version, &right.target.artifact_kind, &right.target.artifact_schema, &right.target.surface_id, right.target.role as u8))
    });
    encoded.extend_from_slice(&u32::try_from(selected.len()).map_err(catalog_error)?.to_be_bytes());
    for selection in selected {
        let package = bundle
            .packages
            .iter()
            .find(|package| package.plugin_id == selection.package.plugin_id && package.package_id == selection.package.package_id && package.version == selection.package.version)
            .ok_or_else(|| catalog("profile generation open-target package is absent"))?;
        let target = &selection.target;
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

/// 🎯️ An open target carries TWO artifact-kind ids from two deliberately distinct spaces, and they
/// are never equal in a real bundle: `artifact_kind` is the manifest `ArtifactKindSpec::id`
/// (`stdio.json` — `validate_descriptor_open_target` requires a manifest kind with exactly that id
/// and schema), while `parent_dialect` is the owning app's `Dialect`, whose kind is the descriptor
/// id (`s.stdio.json` — the same function requires `app.dialect == target.parent_dialect`). Every
/// stdio artifact ships both spellings (`📜️native-codec-factories.json`: `artifact_kind` is
/// `stdio.<x>` for all of them; every `Viewer::builder(…)` dialect is `s.stdio.<x>`). This loop
/// used to refuse a target whose two spellings differed, which made every real stdio bundle
/// unloadable and left `artifactAuthority` permanently not-ready. Binding is enforced where it is
/// meaningful: to a native codec of the same package below, and to the descriptor's own app and
/// artifact kind in `validate_descriptor_open_target`.
fn validate_bundle(bundle: &TrustedBundleV1, profile_id: &str) -> Result<SelectedTrustedBundleV1, AuthorityError> {
    if bundle.schema_version != 3 || bundle.packages.is_empty() || bundle.packages.len() > TRUSTED_CATALOG_MAX_PACKAGES || bundle.profiles.is_empty() || bundle.profiles.len() > TRUSTED_BUNDLE_MAX_PROFILES || !valid_identity(profile_id) {
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
        let module = &package.plugin_module;
        if module.path.is_empty() || module.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES || !(1..=TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES).contains(&module.byte_length) {
            return Err(catalog("trusted plugin module record is empty or exceeds its fixed boundary"));
        }
        decode_digest(&module.sha256, "plugin module sha256")?;
        decode_digest(&module.blake3, "plugin module blake3")?;
        if !paths.insert(module.path.clone()) {
            return Err(catalog("trusted plugin module path is reused across package records"));
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
            {
                return Err(catalog("trusted document-open target names an empty, padded or control-bearing identity"));
            }
            if target.grant != expected_grant {
                return Err(catalog("trusted document-open target grant differs from the one its role fixes"));
            }
            if decode_digest(&target.pack_schema_hash, "open target pack schema hash")? == [0; 32] {
                return Err(catalog("trusted document-open target pack schema hash is zero"));
            }
            if !package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash) {
                return Err(catalog("trusted document-open target is bound to no native codec of its own package"));
            }
            if !open_target_keys.insert((target.artifact_kind.as_str(), target.artifact_schema.as_str(), target.surface_id.as_str(), target.role as u8)) {
                return Err(catalog("trusted document-open target key is duplicated within one package"));
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
        if profile.open_targets.is_empty() || profile.open_targets.len() > TRUSTED_CATALOG_MAX_OPEN_TARGETS {
            return Err(catalog("trusted profile declares no document-open target or exceeds the generation ceiling"));
        }
        let mut profile_target_keys = BTreeSet::new();
        for selection in &profile.open_targets {
            validate_identity(&selection.package)?;
            let target_index = *plugins.get(selection.package.plugin_id.as_str()).ok_or_else(|| catalog("trusted profile open target package is absent"))?;
            let target_package = &bundle.packages[target_index];
            if !closure.contains(&target_index) || target_package.package_id != selection.package.package_id || target_package.version != selection.package.version || !target_package.open_targets.contains(&selection.target) {
                return Err(catalog("trusted profile open target is outside its selected closure"));
            }
            if !profile_target_keys.insert((
                selection.package.plugin_id.as_str(),
                selection.package.package_id.as_str(),
                selection.target.artifact_kind.as_str(),
                selection.target.artifact_schema.as_str(),
                selection.target.surface_id.as_str(),
                selection.target.role as u8,
            )) {
                return Err(catalog("trusted profile declares the same document-open target twice"));
            }
        }
        if profile.id == "local-stdio-gis-open-v1" {
            let identities = profile.selected_closure.iter().map(|identity| (identity.plugin_id.as_str(), identity.package_id.as_str())).collect::<Vec<_>>();
            let gis = bundle.packages.iter().find(|package| package.plugin_id == "gis");
            let stdio = bundle.packages.iter().find(|package| package.plugin_id == "stdio");
            if identities != [("gis", "semio:gis"), ("stdio", "semio:stdio")]
                || bundle.packages.len() != 2
                || gis.is_none_or(|package| {
                    package.native_codecs.len() != 2
                        || package.dependencies.as_slice() != std::slice::from_ref(&profile.selected_closure[1])
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gismap" && codec.artifact_schema == "gis.map")
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gisterrain" && codec.artifact_schema == "gis.terrain")
                })
                || stdio.is_none_or(|package| package.native_codecs.len() != 26 || !package.dependencies.is_empty())
                || profile.open_targets.len() != bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>()
            {
                return Err(catalog("local stdio plus GIS profile is not its exact closed two-package native-codec closure opening every package target"));
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
    let total_units = u64::try_from(bytes.len().max(1)).map_err(catalog_error)?;
    let mut completed_units = 0u64;
    let mut last_report_ms = 0u64;
    for chunk in bytes.chunks(64 * 1024) {
        context.checkpoint()?;
        sha256.update(chunk);
        blake3.update(chunk);
        completed_units = completed_units.saturating_add(u64::try_from(chunk.len()).map_err(catalog_error)?);
        let now_ms = context.now_ms();
        if now_ms.saturating_sub(last_report_ms) >= 1_000 {
            last_report_ms = now_ms;
            context.report(AuthorityProgress {
                stage: AuthorityProgressStage::CatalogLoading,
                completed_units: completed_units.min(total_units),
                total_units,
            })?;
        }
        semio_framework_async::yield_once().await;
    }
    context.checkpoint()?;
    Ok((sha256.finalize(), *blake3.finalize().as_bytes()))
}

async fn sha256(bytes: &[u8], context: &OperationContext<'_>) -> Result<[u8; 32], AuthorityError> {
    let mut hash = Sha256::new();
    let total_units = u64::try_from(bytes.len().max(1)).map_err(catalog_error)?;
    let mut completed_units = 0u64;
    let mut last_report_ms = 0u64;
    for chunk in bytes.chunks(64 * 1024) {
        context.checkpoint()?;
        hash.update(chunk);
        completed_units = completed_units.saturating_add(u64::try_from(chunk.len()).map_err(catalog_error)?);
        let now_ms = context.now_ms();
        if now_ms.saturating_sub(last_report_ms) >= 1_000 {
            last_report_ms = now_ms;
            context.report(AuthorityProgress {
                stage: AuthorityProgressStage::CatalogLoading,
                completed_units: completed_units.min(total_units),
                total_units,
            })?;
        }
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

/// 🧫️ The files of a never-executed fixture plugin module: its entry, both descriptor forms (the packed one
/// the package's own descriptor) and one vendored import shared by every fixture module.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn fixture_plugin_module_files(plugin_id: &str, descriptor_bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
    vec![
        (format!("{plugin_id}/🌉️bridge.js"), b"export async function createActorApi() {}\n".to_vec()),
        (format!("{plugin_id}/🔣️.json"), serde_json::json!({ "manifest": { "pluginId": plugin_id } }).to_string().into_bytes()),
        (format!("{plugin_id}/🛂️.descriptor.semio"), descriptor_bytes.to_vec()),
        ("🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js".to_owned(), b"export const streams = {};\n".to_vec()),
    ]
}

/// 🧫️ Writes one package's fixture plugin module beneath `root` and answers the `pluginModule` record naming it.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn write_fixture_plugin_module(root: &Path, plugin_id: &str, package_id: &str, version: &str, component_sha256: &str, descriptor_bytes: &[u8]) -> Result<serde_json::Value, AuthorityError> {
    let descriptor_byte_sha256 = hex_lower(&Sha256::digest(descriptor_bytes));
    let source = TrustedPluginModuleSourceV1 { plugin_id, package_id, version, component_sha256, descriptor_byte_sha256: &descriptor_byte_sha256 };
    let record = plugin_module::write_plugin_module_bundle(root, &format!("plugin-module-{plugin_id}.json"), source, plugin_id, &fixture_plugin_module_files(plugin_id, descriptor_bytes))?;
    serde_json::to_value(record).map_err(catalog_error)
}

/// 🧫️ Shares headless Stdio metadata between native GIS fixtures; synthetic bytes are never executed.
#[cfg(all(feature = "native-artifact-execution", any(test, feature = "integration-fixtures")))]
fn headless_stdio_fixture_package(root: &Path) -> Result<(serde_json::Value, serde_json::Value), AuthorityError> {
    let dependency = semio_s_plugin_stdio::registry::native_artifact_catalog_dependency().map_err(catalog_error)?;
    let version = dependency.version.0.to_string();
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
    let plugin_module = write_fixture_plugin_module(root, "stdio", "semio:stdio", &version, &component_sha256, &bytes)?;
    let record = serde_json::json!({
        "pluginId": "stdio", "packageId": "semio:stdio", "version": version, "role": "plugin", "dependencies": [],
        "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
        "component": { "path": "stdio-component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
        "descriptor": { "path": "stdio-descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
        "browserActor": { "kind": "none" }, "pluginModule": plugin_module, "nativeCodecs": codecs, "openTargets": [],
    });
    serde_json::from_value::<TrustedBundlePackageV1>(record.clone()).map_err(catalog_error)?;
    Ok((identity, record))
}

/// 🏗️ Feature-gated real GIS Map profile builder, reachable from every crate target (see its module doc).
#[cfg(all(feature = "integration-fixtures", feature = "native-artifact-execution"))]
#[path = "../../🧪️tests/🔏️trusted-catalog-profile/🦀️.rs"]
pub mod trusted_catalog_fixture;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
