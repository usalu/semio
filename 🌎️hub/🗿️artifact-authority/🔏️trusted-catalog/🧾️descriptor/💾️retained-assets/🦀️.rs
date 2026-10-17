//! 💾️ Retained assets of a verified package: hash-verified files read on demand, asset streams, plugin modules and the verified package and selection records.

use super::*;

/// 💾️ One hash-verified generation file retained by IDENTITY, not by bytes: the opened private
/// generation root, the bundle-relative path and the digests the load verified. A read reopens the
/// path beneath that root and re-verifies length and digests, and the bytes stay resident only while
/// some reader holds them, so the hub's memory is the set of assets in use, never the catalog closure.
pub struct TrustedRetainedFile {
    pub root: Arc<TrustedCatalogGenerationRoot>,
    pub path: TrustedCatalogRelativePathV1,
    pub byte_length: u64,
    pub sha256: [u8; 32],
    pub blake3: Option<[u8; 32]>,
    pub resident: std::sync::Mutex<std::sync::Weak<[u8]>>,
}

impl TrustedRetainedFile {
    pub fn new(root: Arc<TrustedCatalogGenerationRoot>, path: TrustedCatalogRelativePathV1, byte_length: u64, sha256: [u8; 32], blake3: Option<[u8; 32]>) -> Self {
        let resident: std::sync::Weak<[u8]> = std::sync::Weak::<[u8; 0]>::new();
        Self { root, path, byte_length, sha256, blake3, resident: std::sync::Mutex::new(resident) }
    }

    pub async fn read(&self, context: &OperationContext<'_>) -> Result<Arc<[u8]>, AuthorityError> {
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
    pub source: TrustedCatalogAssetSource,
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

    pub fn retained(file: TrustedRetainedFile) -> Self {
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
    pub(super) byte_length: u64,
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
    pub bundle_sha256: String,
    pub manifest_bytes: Arc<[u8]>,
    pub bundle: TrustedPluginModuleBundleV1,
    pub files: BTreeMap<String, TrustedCatalogAsset>,
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
    pub plugin_id: String,
    pub package: PackageRef,
    pub version: String,
    pub dependencies: Vec<String>,
    pub plugin_module: VerifiedPluginModule,
    pub component_sha256: [u8; 32],
    pub descriptor_sha256: [u8; 32],
    pub component: TrustedCatalogAsset,
    pub descriptor_bytes: Arc<[u8]>,
    pub receipt: ComponentReceiptV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
    pub browser_actor_asset: Option<TrustedCatalogAsset>,
    pub descriptor: Arc<PackageDescriptor>,
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

    /// 🧾️ Returns the shipped receipt binding the semantic descriptor to the component bytes.
    pub fn receipt(&self) -> &ComponentReceiptV1 {
        &self.receipt
    }

    /// 🧩️ Returns the package's verified plugin module.
    pub fn plugin_module(&self) -> &VerifiedPluginModule {
        &self.plugin_module
    }
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

/// 🧩️ Reads, verifies and retains one package's plugin module: the manifest against its record and the
/// package's own verified digests, then every file against the manifest. A file several modules share
/// (the vendored imports, the fonts) is one content-addressed file, verified once per load.
pub async fn verify_plugin_module(
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
    let receipt_sha256 = hex_lower(&Sha256::digest(&record.receipt.encode_canonical()));
    let source = TrustedPluginModuleSourceV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version, component_sha256, descriptor_byte_sha256: descriptor_sha256, receipt_sha256: &receipt_sha256 };
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
