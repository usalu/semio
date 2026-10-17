//! 📁️ The dedicated filesystem CAS: leaf opening, fences, symlink and parent validation and the storage port over files.

use super::*;

/// 📁️ Dedicated filesystem CAS rooted below `artifact-cas/v1`.
pub struct FsArtifactChunkCasStorage {
    root: PathBuf,
    nonce: std::sync::atomic::AtomicU64,
}

#[cfg(unix)]
pub(super) fn open_artifact_cas_leaf(path: &Path, write: bool, create: bool) -> std::io::Result<std::fs::File> {
    use std::os::unix::ffi::OsStrExt as _;
    use std::os::unix::io::FromRawFd as _;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_CREATE: i32 = 0x40;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const O_CREATE: i32 = 0x0200;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_NOFOLLOW: i32 = 0x20_000;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const O_NOFOLLOW: i32 = 0x0100;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_CLOEXEC: i32 = 0x8_0000;
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    const O_CLOEXEC: i32 = 0x0100_0000;
    unsafe extern "C" {
        fn open(path: *const core::ffi::c_char, flags: i32, ...) -> i32;
    }
    let path = std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS path contains NUL"))?;
    let flags = (if write { 2 } else { 0 }) | O_NOFOLLOW | O_CLOEXEC | (if create { O_CREATE } else { 0 });
    let descriptor = unsafe { open(path.as_ptr(), flags, 0o600 as core::ffi::c_int) };
    if descriptor < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { std::fs::File::from_raw_fd(descriptor) })
    }
}

#[cfg(windows)]
pub(super) fn open_artifact_cas_leaf(path: &Path, write: bool, create: bool) -> std::io::Result<std::fs::File> {
    use std::os::windows::fs::MetadataExt as _;
    use std::os::windows::fs::OpenOptionsExt as _;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    let mut options = std::fs::OpenOptions::new();
    options.read(true).write(write).create(create).truncate(false).custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    let file = options.open(path)?;
    if file.metadata()?.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS leaf is a reparse point"))
    } else {
        Ok(file)
    }
}

impl FsArtifactChunkCasStorage {
    /// 🏗️ Creates and validates one non-symlink dedicated storage root.
    pub async fn open(root: &Path) -> Result<Self, AuthorityError> {
        tokio::fs::create_dir_all(root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem root create failed".to_string()))?;
        let metadata = tokio::fs::symlink_metadata(root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem root metadata failed".to_string()))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(AuthorityError::Store("artifact CAS filesystem root must be a non-symlink directory".to_string()));
        }
        let root = tokio::fs::canonicalize(root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem root canonicalization failed".to_string()))?;
        Ok(Self { root, nonce: std::sync::atomic::AtomicU64::new(1) })
    }

    pub(super) fn object_path(&self, key: &ArtifactCasObjectKey) -> Result<PathBuf, AuthorityError> {
        Ok(self.root.join(hex_lower(&space_digest(&key.space_id)?.0)).join(key.kind.name()).join(hex_lower(&key.digest.0)))
    }

    fn validate_directory(path: &Path) -> Result<(), AuthorityError> {
        let metadata = std::fs::symlink_metadata(path).map_err(|_| AuthorityError::Store("artifact CAS filesystem directory metadata failed".into()))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(AuthorityError::Store("artifact CAS filesystem path must be a non-symlink directory".into()));
        }
        Ok(())
    }

    fn reject_symlink(path: &Path) -> Result<(), AuthorityError> {
        match std::fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_symlink() => Err(AuthorityError::Store("artifact CAS filesystem leaf must not be a symlink".into())),
            Ok(_) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(AuthorityError::Store("artifact CAS filesystem leaf metadata failed".into())),
        }
    }

    fn validate_opened_leaf(path: &Path, file: &std::fs::File) -> Result<(), AuthorityError> {
        let path_metadata = std::fs::symlink_metadata(path).map_err(|_| AuthorityError::Store("artifact CAS filesystem leaf metadata failed".into()))?;
        if path_metadata.file_type().is_symlink() {
            return Err(AuthorityError::Store("artifact CAS filesystem leaf must not be a symlink".into()));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            let opened = file.metadata().map_err(|_| AuthorityError::Store("artifact CAS filesystem opened leaf metadata failed".into()))?;
            if opened.dev() != path_metadata.dev() || opened.ino() != path_metadata.ino() {
                return Err(AuthorityError::Store("artifact CAS filesystem leaf changed during open".into()));
            }
        }
        Ok(())
    }

    fn read_fence(path: &Path) -> Result<Vec<u8>, AuthorityError> {
        use std::io::Read as _;
        Self::reject_symlink(path)?;
        let mut file = open_artifact_cas_leaf(path, false, false).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence read failed".into()))?;
        Self::validate_opened_leaf(path, &file)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence read failed".into()))?;
        Ok(bytes)
    }

    fn validate_object_parents(&self, key: &ArtifactCasObjectKey) -> Result<(), AuthorityError> {
        let target = self.object_path(key)?;
        let kind = target.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem object parent missing".into()))?;
        let space = kind.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem space parent missing".into()))?;
        Self::validate_directory(space)?;
        Self::validate_directory(kind)
    }

    async fn acquire_file_fence(&self, path: &Path, context: &OperationContext<'_>) -> Result<FileFence, AuthorityError> {
        loop {
            context.checkpoint()?;
            let path = path.to_path_buf();
            let attempt = tokio::task::spawn_blocking(move || -> std::io::Result<Option<FileFence>> {
                if std::fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS lock is a symlink"));
                }
                let file = open_artifact_cas_leaf(&path, true, true)?;
                Self::validate_opened_leaf(&path, &file).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "artifact CAS lock changed during open"))?;
                try_acquire(file)
            })
            .await
            .map_err(|_| AuthorityError::Store("artifact CAS filesystem fence worker failed".into()))?
            .map_err(|error| AuthorityError::Store(format!("artifact CAS filesystem fence lock failed: {error}")))?;
            if let Some(fence) = attempt {
                return Ok(fence);
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }

    fn encode_fence(domain: &[u8], coordinator_id: [u8; 32], epoch: Option<u64>) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(domain.len() + 72);
        bytes.extend_from_slice(domain);
        bytes.extend_from_slice(&coordinator_id);
        if let Some(epoch) = epoch {
            bytes.extend_from_slice(&epoch.to_be_bytes());
        }
        let checksum = Sha256::digest(&bytes);
        bytes.extend_from_slice(&checksum);
        bytes
    }

    fn decode_fence(bytes: &[u8], domain: &[u8], has_epoch: bool) -> Result<([u8; 32], u64), AuthorityError> {
        let payload = domain.len() + 32 + if has_epoch { 8 } else { 0 };
        if bytes.len() != payload + 32 || &bytes[..domain.len()] != domain || Sha256::digest(&bytes[..payload]) != bytes[payload..] {
            return Err(AuthorityError::Store("artifact CAS filesystem fence metadata is invalid".into()));
        }
        let mut coordinator_id = [0; 32];
        coordinator_id.copy_from_slice(&bytes[domain.len()..domain.len() + 32]);
        let epoch = if has_epoch { u64::from_be_bytes(bytes[domain.len() + 32..payload].try_into().map_err(|_| AuthorityError::Store("artifact CAS filesystem fence metadata is invalid".into()))?) } else { 0 };
        Ok((coordinator_id, epoch))
    }

    #[cfg(unix)]
    fn replace_fence(from: &Path, to: &Path) -> std::io::Result<()> {
        std::fs::rename(from, to)
    }

    #[cfg(windows)]
    fn replace_fence(from: &Path, to: &Path) -> std::io::Result<()> {
        use std::os::windows::ffi::OsStrExt as _;
        unsafe extern "system" {
            fn MoveFileExW(existing: *const u16, replacement: *const u16, flags: u32) -> i32;
        }
        let from: Vec<u16> = from.as_os_str().encode_wide().chain([0]).collect();
        let to: Vec<u16> = to.as_os_str().encode_wide().chain([0]).collect();
        if unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), 1 | 8) } != 0 {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }

    #[cfg(unix)]
    fn sync_fence_parent(path: &Path) -> std::io::Result<()> {
        std::fs::File::open(path)?.sync_all()
    }

    #[cfg(windows)]
    fn sync_fence_parent(_: &Path) -> std::io::Result<()> {
        Ok(())
    }

    fn write_fence(path: &Path, bytes: &[u8]) -> Result<(), AuthorityError> {
        use std::io::Write as _;
        Self::reject_symlink(path)?;
        Self::validate_directory(path.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem fence parent missing".into()))?)?;
        let nonce = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_nanos());
        let temporary = path.with_extension(format!("tmp.{}.{nonce}", std::process::id()));
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence temporary create failed".into()))?;
        file.write_all(bytes).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence write failed".into()))?;
        file.sync_all().map_err(|_| AuthorityError::Store("artifact CAS filesystem fence sync failed".into()))?;
        drop(file);
        Self::replace_fence(&temporary, path).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence replace failed".into()))?;
        let parent = path.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem fence parent missing".into()))?;
        Self::sync_fence_parent(parent).map_err(|_| AuthorityError::Store("artifact CAS filesystem fence directory sync failed".into()))
    }

    async fn read_path(&self, key: &ArtifactCasObjectKey, path: &Path, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        use tokio::io::AsyncReadExt as _;
        context.checkpoint()?;
        self.validate_object_parents(key)?;
        Self::reject_symlink(path)?;
        let path = path.to_path_buf();
        let std_file = tokio::task::spawn_blocking(move || {
            let file = open_artifact_cas_leaf(&path, false, false).map_err(|_| AuthorityError::Store("artifact CAS filesystem object not found".to_string()))?;
            Self::validate_opened_leaf(&path, &file)?;
            Ok::<_, AuthorityError>(file)
        })
        .await
        .map_err(|_| AuthorityError::Store("artifact CAS filesystem object open worker failed".into()))??;
        let file = tokio::fs::File::from_std(std_file);
        let metadata = file.metadata().await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object metadata failed".to_string()))?;
        if !metadata.is_file() || metadata.len() > key.kind.maximum_bytes() as u64 {
            return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take(key.kind.maximum_bytes() as u64 + 1).read_to_end(&mut bytes).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object read failed".to_string()))?;
        validate_object(key, &bytes)?;
        context.checkpoint()?;
        Ok(bytes)
    }
}

impl ArtifactChunkCasStorage for FsArtifactChunkCasStorage {
    async fn configure_coordinator(&self, coordinator_id: [u8; 32], context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if coordinator_id == [0; 32] {
            return Err(AuthorityError::Store("artifact CAS coordinator identity is invalid".into()));
        }
        let _fence = self.acquire_file_fence(&self.root.join("coordinator.lock"), context).await?;
        let path = self.root.join("coordinator-v1");
        match std::fs::symlink_metadata(&path) {
            Ok(_) => {
                if Self::decode_fence(&Self::read_fence(&path)?, ARTIFACT_CAS_COORDINATOR_DOMAIN_V1, false)?.0 != coordinator_id {
                    return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Self::write_fence(&path, &Self::encode_fence(ARTIFACT_CAS_COORDINATOR_DOMAIN_V1, coordinator_id, None))?,
            Err(_) => return Err(AuthorityError::Store("artifact CAS filesystem coordinator read failed".into())),
        }
        context.checkpoint()
    }

    async fn advance_physical_epoch(&self, coordinator_id: [u8; 32], space_id: &str, epoch: u64, context: &OperationContext<'_>) -> Result<(), AuthorityError> {
        context.checkpoint()?;
        if epoch == 0 {
            return Err(AuthorityError::Store("artifact CAS physical epoch is invalid".into()));
        }
        let coordinator = Self::read_fence(&self.root.join("coordinator-v1")).map_err(|_| AuthorityError::Store("artifact CAS coordinator is not configured".into()))?;
        if Self::decode_fence(&coordinator, ARTIFACT_CAS_COORDINATOR_DOMAIN_V1, false)?.0 != coordinator_id {
            return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
        }
        let space_root = self.root.join(hex_lower(&space_digest(space_id)?.0));
        tokio::fs::create_dir_all(&space_root).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem space create failed".into()))?;
        Self::validate_directory(&space_root)?;
        let _fence = self.acquire_file_fence(&space_root.join("fence.lock"), context).await?;
        let path = space_root.join("fence-v1");
        let current = match std::fs::symlink_metadata(&path) {
            Ok(_) => {
                let (current_id, current_epoch) = Self::decode_fence(&Self::read_fence(&path)?, ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1, true)?;
                if current_id != coordinator_id {
                    return Err(AuthorityError::Store("artifact CAS coordinator identity mismatch".into()));
                }
                current_epoch
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
            Err(_) => return Err(AuthorityError::Store("artifact CAS filesystem physical epoch read failed".into())),
        };
        if epoch > current {
            Self::write_fence(&path, &Self::encode_fence(ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1, coordinator_id, Some(epoch)))?;
        }
        context.checkpoint()
    }

    async fn put_if_absent(&self, key: &ArtifactCasObjectKey, bytes: &[u8], context: &OperationContext<'_>) -> Result<ArtifactCasPutOutcome, AuthorityError> {
        use tokio::io::AsyncWriteExt as _;
        context.checkpoint()?;
        validate_object(key, bytes)?;
        let target = self.object_path(key)?;
        let parent = target.parent().ok_or_else(|| AuthorityError::Store("artifact CAS filesystem object parent missing".to_string()))?;
        tokio::fs::create_dir_all(parent).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object directory create failed".to_string()))?;
        self.validate_object_parents(key)?;
        Self::reject_symlink(&target)?;
        let nonce = self.nonce.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let temporary = parent.join(format!(".{}.{}.tmp", std::process::id(), nonce));
        let mut options = tokio::fs::OpenOptions::new();
        options.write(true).create_new(true);
        let mut file = options.open(&temporary).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem temporary create failed".to_string()))?;
        file.write_all(bytes).await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object write failed".to_string()))?;
        file.sync_all().await.map_err(|_| AuthorityError::Store("artifact CAS filesystem object sync failed".to_string()))?;
        drop(file);
        context.checkpoint()?;
        let inserted = match tokio::fs::hard_link(&temporary, &target).await {
            Ok(()) => true,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => false,
            Err(_) => {
                let _ = tokio::fs::remove_file(&temporary).await;
                return Err(AuthorityError::Store("artifact CAS filesystem atomic install failed".to_string()));
            }
        };
        let _ = tokio::fs::remove_file(&temporary).await;
        let exact = self.read_path(key, &target, context).await?;
        if exact != bytes {
            return Err(AuthorityError::Store("artifact CAS immutable key collision".to_string()));
        }
        Ok(if inserted { ArtifactCasPutOutcome::Inserted } else { ArtifactCasPutOutcome::AlreadyPresent })
    }

    async fn get(&self, key: &ArtifactCasObjectKey, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        self.read_path(key, &self.object_path(key)?, context).await
    }

    async fn delete_if_unreferenced(&self, key: &ArtifactCasObjectKey, fence: &ArtifactCasDeleteFence, context: &OperationContext<'_>) -> Result<ArtifactCasDeleteOutcome, AuthorityError> {
        context.checkpoint()?;
        if !fence.permits(key) {
            return Err(AuthorityError::Store("artifact CAS deletion fence mismatch".to_string()));
        }
        let space_root = self.root.join(hex_lower(&space_digest(&key.space_id)?.0));
        Self::validate_directory(&space_root)?;
        let _physical_fence = self.acquire_file_fence(&space_root.join("fence.lock"), context).await?;
        let metadata = Self::read_fence(&space_root.join("fence-v1"))?;
        let (coordinator_id, physical_epoch) = Self::decode_fence(&metadata, ARTIFACT_CAS_PHYSICAL_FENCE_DOMAIN_V1, true)?;
        if &coordinator_id != fence.coordinator_id() || physical_epoch != fence.physical_epoch() {
            return Err(AuthorityError::Store("artifact CAS deletion fence is stale".into()));
        }
        self.validate_object_parents(key)?;
        let object_path = self.object_path(key)?;
        Self::reject_symlink(&object_path)?;
        match tokio::fs::remove_file(object_path).await {
            Ok(()) => Ok(ArtifactCasDeleteOutcome::Deleted),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(ArtifactCasDeleteOutcome::Missing),
            Err(_) => Err(AuthorityError::Store("artifact CAS filesystem object delete failed".to_string())),
        }
    }
}
