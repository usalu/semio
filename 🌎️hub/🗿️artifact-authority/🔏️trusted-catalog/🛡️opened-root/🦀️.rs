//! 🛡️ Descriptor-rooted, no-link trusted catalog file ownership.

use super::{catalog, catalog_error, AuthorityError, OperationContext, TRUSTED_RELATIVE_PATH_MAX_BYTES};
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Component, Path};
use tokio::io::AsyncReadExt;

const TRUSTED_RELATIVE_PATH_MAX_SEGMENTS: usize = 64;
const TRUSTED_READ_CHUNK_BYTES: usize = 64 * 1024;

/// 🧭 One parsed bundle-relative path whose segments can be opened without reparsing.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrustedCatalogRelativePathV1 {
    segments: Box<[String]>,
}

impl TrustedCatalogRelativePathV1 {
    pub(super) fn parse(value: &str) -> Result<Self, AuthorityError> {
        let segments = value.split('/').collect::<Vec<_>>();
        if value.is_empty()
            || value.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES
            || value.contains('\\')
            || value.contains('\0')
            || value.starts_with('/')
            || segments.len() > TRUSTED_RELATIVE_PATH_MAX_SEGMENTS
            || segments.iter().any(|segment| segment.is_empty() || *segment == "." || *segment == "..")
        {
            return Err(catalog("trusted file path is not a bounded relative path"));
        }
        Ok(Self { segments: segments.into_iter().map(str::to_owned).collect() })
    }

    fn os_segments(&self) -> impl Iterator<Item = &OsStr> {
        self.segments.iter().map(AsRef::as_ref)
    }
}

/// 🏠️ Opened server-owned data root used as the sole catalog startup authority.
pub(super) struct TrustedCatalogDataRoot {
    directory: File,
}

impl TrustedCatalogDataRoot {
    pub(super) fn open_server_owned(path: &Path) -> Result<Self, AuthorityError> {
        Ok(Self { directory: platform::open_server_owned(path).map_err(catalog_error)? })
    }

    pub(super) fn open_current(&self) -> Result<Option<TrustedCatalogOpenedFile>, AuthorityError> {
        let path = TrustedCatalogRelativePathV1::parse("trusted-catalog/current.json")?;
        match platform::open_regular_at(&self.directory, path.os_segments()) {
            Ok(file) => TrustedCatalogOpenedFile::new(file).map(Some),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(catalog_error(error)),
        }
    }

    pub(super) fn open_generation(&self, generation_id: &str) -> Result<TrustedCatalogGenerationRoot, AuthorityError> {
        let path = TrustedCatalogRelativePathV1::parse(&format!("trusted-catalog/generations/{generation_id}"))?;
        Ok(TrustedCatalogGenerationRoot { directory: platform::open_directory_at(&self.directory, path.os_segments()).map_err(catalog_error)? })
    }

    pub(super) async fn acquire_publication(&self, context: &OperationContext<'_>) -> Result<TrustedCatalogPublicationOwner, AuthorityError> {
        context.checkpoint()?;
        let directory = platform::open_directory_at(&self.directory, std::iter::once(OsStr::new("trusted-catalog"))).map_err(catalog_error)?;
        loop {
            context.checkpoint()?;
            let attempt_root = directory.try_clone().map_err(catalog_error)?;
            let acquired = tokio::task::spawn_blocking(move || super::super::file_fence::try_acquire(platform::open_lock(&attempt_root)?)).await.map_err(catalog_error)?.map_err(catalog_error)?;
            if let Some(fence) = acquired {
                context.checkpoint()?;
                return Ok(TrustedCatalogPublicationOwner { directory, _fence: fence });
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }
}

/// 📤️ One exclusive publication window bound to the already-opened catalog directory.
pub(super) struct TrustedCatalogPublicationOwner {
    directory: File,
    _fence: super::super::file_fence::FileFence,
}

/// 💾️ Separates visible replacement from confirmed directory synchronization.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum TrustedPublicationSync {
    Durable,
    Unconfirmed,
}

impl TrustedCatalogPublicationOwner {
    pub(super) fn open_current(&self) -> Result<Option<TrustedCatalogOpenedFile>, AuthorityError> {
        match platform::open_regular_at(&self.directory, std::iter::once(OsStr::new("current.json"))) {
            Ok(file) => TrustedCatalogOpenedFile::new(file).map(Some),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(catalog_error(error)),
        }
    }

    pub(super) fn open_generation(&self, generation_id: &str) -> Result<TrustedCatalogGenerationRoot, AuthorityError> {
        let path = TrustedCatalogRelativePathV1::parse(&format!("generations/{generation_id}"))?;
        Ok(TrustedCatalogGenerationRoot { directory: platform::open_directory_at(&self.directory, path.os_segments()).map_err(catalog_error)? })
    }

    pub(super) fn replace_current(&self, nonce: &str, bytes: &[u8], context: &OperationContext<'_>) -> Result<TrustedPublicationSync, AuthorityError> {
        use std::io::Write;
        context.checkpoint()?;
        if nonce.len() != 32 || !nonce.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) || bytes.is_empty() || bytes.len() > 65_536 {
            return Err(catalog("trusted publication pointer or nonce exceeds its exact boundary"));
        }
        let name = format!(".current-{nonce}.json");
        let mut file = platform::create_new(&self.directory, OsStr::new(&name)).map_err(catalog_error)?;
        let result = (|| {
            file.write_all(bytes).map_err(catalog_error)?;
            file.sync_all().map_err(catalog_error)?;
            context.checkpoint()?;
            platform::replace_current(&self.directory, &file, OsStr::new(&name)).map_err(catalog_error)?;
            Ok(platform::sync_publication(&self.directory))
        })();
        if result.is_err() {
            let _ = platform::remove_owned(&self.directory, &file, OsStr::new(&name));
        }
        result
    }
}

/// 🗂️ Retained immutable generation directory descriptor.
pub(super) struct TrustedCatalogGenerationRoot {
    directory: File,
}

impl TrustedCatalogGenerationRoot {
    #[cfg(any(test, feature = "test-support"))]
    pub(super) fn open_fixture_owned(path: &Path) -> Result<Self, AuthorityError> {
        Ok(Self { directory: platform::open_server_owned(path).map_err(catalog_error)? })
    }

    pub(super) fn open_regular(&self, path: &TrustedCatalogRelativePathV1) -> Result<TrustedCatalogOpenedFile, AuthorityError> {
        TrustedCatalogOpenedFile::new(platform::open_regular_at(&self.directory, path.os_segments()).map_err(catalog_error)?)
    }

    pub(super) async fn read_regular(&self, path: &TrustedCatalogRelativePathV1, maximum: u64, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        self.open_regular(path)?.read_bounded(maximum, context).await
    }
}

/// 📄 One already-opened regular file; later pathname replacement cannot redirect its read.
pub(super) struct TrustedCatalogOpenedFile {
    file: File,
    length: u64,
}

impl TrustedCatalogOpenedFile {
    fn new(file: File) -> Result<Self, AuthorityError> {
        let length = platform::fstat_regular_length(&file).map_err(catalog_error)?;
        Ok(Self { file, length })
    }

    pub(super) async fn read_bounded(self, maximum: u64, context: &OperationContext<'_>) -> Result<Vec<u8>, AuthorityError> {
        context.checkpoint()?;
        if self.length == 0 || self.length > maximum {
            return Err(catalog("trusted file is empty, non-regular, or exceeds its fixed byte boundary"));
        }
        let capacity = usize::try_from(self.length).map_err(catalog_error)?;
        let mut file = tokio::fs::File::from_std(self.file);
        let mut bytes = Vec::with_capacity(capacity);
        let mut chunk = [0u8; TRUSTED_READ_CHUNK_BYTES];
        loop {
            context.checkpoint()?;
            let remaining = capacity.checked_sub(bytes.len()).ok_or(AuthorityError::ResourceLimit("trusted file byte"))?;
            let read_limit = remaining.saturating_add(1).min(chunk.len());
            let read = file.read(&mut chunk[..read_limit]).await.map_err(catalog_error)?;
            if read == 0 {
                break;
            }
            let next = bytes.len().checked_add(read).ok_or(AuthorityError::ResourceLimit("trusted file byte"))?;
            if next > capacity {
                return Err(catalog("trusted file grew beyond its opened-handle length while reading"));
            }
            bytes.extend_from_slice(&chunk[..read]);
            semio_framework_async::yield_once().await;
        }
        context.checkpoint()?;
        if bytes.len() != capacity {
            return Err(catalog("trusted file length changed while reading its opened handle"));
        }
        Ok(bytes)
    }
}

#[cfg(all(test, unix))]
/// 🧪️ Creates a native FIFO used to prove final-leaf admission cannot block before type rejection.
pub(super) fn create_fifo_fixture(path: &Path) -> Result<(), AuthorityError> {
    platform::create_fifo(path).map_err(catalog_error)
}

#[cfg(unix)]
mod platform {
    use super::*;
    use std::ffi::CString;
    use std::os::fd::{AsRawFd, FromRawFd};
    use std::os::unix::ffi::OsStrExt;

    const O_RDONLY: i32 = 0;
    const O_RDWR: i32 = 2;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const O_CREATE: i32 = 0x0200;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const O_EXCL: i32 = 0x0800;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_CREATE: i32 = 0x0040;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_EXCL: i32 = 0x0080;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const O_NOFOLLOW: i32 = 0x0000_0100;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const O_NONBLOCK: i32 = 0x0000_0004;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const O_DIRECTORY: i32 = 0x0010_0000;
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    const O_CLOEXEC: i32 = 0x0100_0000;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_NOFOLLOW: i32 = 0x0002_0000;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_NONBLOCK: i32 = 0x0000_0800;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_DIRECTORY: i32 = 0x0001_0000;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    const O_CLOEXEC: i32 = 0x0008_0000;

    unsafe extern "C" {
        fn openat(directory: i32, path: *const std::ffi::c_char, flags: i32, ...) -> i32;
        fn renameat(source: i32, source_path: *const std::ffi::c_char, destination: i32, destination_path: *const std::ffi::c_char) -> i32;
        fn unlinkat(directory: i32, path: *const std::ffi::c_char, flags: i32) -> i32;
        #[cfg(test)]
        fn mkfifo(path: *const std::ffi::c_char, mode: ModeT) -> i32;
    }

    #[cfg(all(test, any(target_os = "macos", target_os = "ios")))]
    type ModeT = u16;
    #[cfg(all(test, any(target_os = "linux", target_os = "android")))]
    type ModeT = u32;

    pub(super) fn open_server_owned(path: &Path) -> std::io::Result<File> {
        let mut components = path.components();
        let mut current = if path.is_absolute() { File::open("/")? } else { File::open(".")? };
        for component in &mut components {
            match component {
                Component::RootDir | Component::CurDir => {}
                Component::Normal(segment) => current = open_one(&current, segment, true)?,
                Component::ParentDir | Component::Prefix(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "server-owned root is not a normalized path")),
            }
        }
        Ok(current)
    }

    pub(super) fn open_directory_at<'a>(root: &File, segments: impl Iterator<Item = &'a OsStr>) -> std::io::Result<File> {
        let mut current = root.try_clone()?;
        for segment in segments {
            current = open_one(&current, segment, true)?;
        }
        Ok(current)
    }

    pub(super) fn open_regular_at<'a>(root: &File, mut segments: impl Iterator<Item = &'a OsStr>) -> std::io::Result<File> {
        let mut current = root.try_clone()?;
        let first = segments.next().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty relative file path"))?;
        let mut pending = first;
        for segment in segments {
            current = open_one(&current, pending, true)?;
            pending = segment;
        }
        open_one(&current, pending, false)
    }

    fn open_one(parent: &File, segment: &OsStr, directory: bool) -> std::io::Result<File> {
        let path = CString::new(segment.as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "path segment contains NUL"))?;
        let flags = O_RDONLY | O_NOFOLLOW | O_CLOEXEC | if directory { O_DIRECTORY } else { O_NONBLOCK };
        let descriptor = unsafe { openat(parent.as_raw_fd(), path.as_ptr(), flags, 0) };
        if descriptor < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(unsafe { File::from_raw_fd(descriptor) })
    }

    pub(super) fn fstat_regular_length(file: &File) -> std::io::Result<u64> {
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "opened object is not regular"));
        }
        Ok(metadata.len())
    }

    pub(super) fn open_lock(parent: &File) -> std::io::Result<File> {
        let file = open_writable(parent, OsStr::new(".publication.lock"), false)?;
        if fstat_regular_length(&file)? != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "publication lock is not empty"));
        }
        Ok(file)
    }

    pub(super) fn create_new(parent: &File, name: &OsStr) -> std::io::Result<File> {
        open_writable(parent, name, true)
    }

    fn open_writable(parent: &File, name: &OsStr, exclusive: bool) -> std::io::Result<File> {
        let name = CString::new(name.as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "publication leaf contains NUL"))?;
        let flags = O_RDWR | O_CREATE | O_NOFOLLOW | O_CLOEXEC | O_NONBLOCK | if exclusive { O_EXCL } else { 0 };
        let descriptor = unsafe { openat(parent.as_raw_fd(), name.as_ptr(), flags, 0o600 as std::ffi::c_uint) };
        if descriptor < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let file = unsafe { File::from_raw_fd(descriptor) };
        fstat_regular_length(&file)?;
        Ok(file)
    }

    fn same_file(left: &File, right: &File) -> std::io::Result<bool> {
        use std::os::unix::fs::MetadataExt;
        let left = left.metadata()?;
        let right = right.metadata()?;
        Ok(left.dev() == right.dev() && left.ino() == right.ino())
    }

    pub(super) fn replace_current(parent: &File, file: &File, name: &OsStr) -> std::io::Result<()> {
        if !same_file(file, &open_one(parent, name, false)?)? {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "publication temporary leaf was replaced"));
        }
        let name = CString::new(name.as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "publication leaf contains NUL"))?;
        if unsafe { renameat(parent.as_raw_fd(), name.as_ptr(), parent.as_raw_fd(), c"current.json".as_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    pub(super) fn remove_owned(parent: &File, file: &File, name: &OsStr) -> std::io::Result<()> {
        if !same_file(file, &open_one(parent, name, false)?)? {
            return Ok(());
        }
        let name = CString::new(name.as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "publication leaf contains NUL"))?;
        if unsafe { unlinkat(parent.as_raw_fd(), name.as_ptr(), 0) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    pub(super) fn sync_publication(parent: &File) -> TrustedPublicationSync {
        if parent.sync_all().is_ok() {
            TrustedPublicationSync::Durable
        } else {
            TrustedPublicationSync::Unconfirmed
        }
    }

    #[cfg(test)]
    pub(super) fn create_fifo(path: &Path) -> std::io::Result<()> {
        let path = CString::new(path.as_os_str().as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "FIFO path contains NUL"))?;
        if unsafe { mkfifo(path.as_ptr(), 0o600) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(windows)]
mod platform {
    use super::*;
    use std::ffi::c_void;
    use std::mem::{size_of, zeroed};
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    use std::path::PathBuf;
    use std::ptr::null_mut;

    type Handle = isize;
    const INVALID_HANDLE_VALUE: Handle = -1;
    const GENERIC_READ: u32 = 0x8000_0000;
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const DELETE: u32 = 0x0001_0000;
    const SYNCHRONIZE: u32 = 0x0010_0000;
    const FILE_READ_ATTRIBUTES: u32 = 0x0000_0080;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_SHARE_DELETE: u32 = 0x0000_0004;
    const OPEN_EXISTING: u32 = 3;
    const FILE_ATTRIBUTE_NORMAL: u32 = 0x0000_0080;
    const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x0000_0010;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_OPEN: u32 = 1;
    const FILE_CREATE: u32 = 2;
    const FILE_OPEN_IF: u32 = 3;
    const FILE_DIRECTORY_FILE: u32 = 0x0000_0001;
    const FILE_NON_DIRECTORY_FILE: u32 = 0x0000_0040;
    const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;
    const FILE_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const OBJ_CASE_INSENSITIVE: u32 = 0x0000_0040;
    const OBJ_DONT_REPARSE: u32 = 0x0000_1000;
    const FILE_ATTRIBUTE_TAG_INFO_CLASS: i32 = 9;
    const STATUS_SUCCESS: i32 = 0;
    const STATUS_OBJECT_NAME_NOT_FOUND: i32 = 0xc000_0034u32 as i32;
    const STATUS_OBJECT_PATH_NOT_FOUND: i32 = 0xc000_003au32 as i32;

    #[repr(C)]
    struct UnicodeString {
        length: u16,
        maximum_length: u16,
        buffer: *mut u16,
    }

    #[repr(C)]
    struct ObjectAttributes {
        length: u32,
        root_directory: Handle,
        object_name: *mut UnicodeString,
        attributes: u32,
        security_descriptor: *mut c_void,
        security_quality_of_service: *mut c_void,
    }

    #[repr(C)]
    union IoStatus {
        status: i32,
        pointer: *mut c_void,
    }

    #[repr(C)]
    struct IoStatusBlock {
        status: IoStatus,
        information: usize,
    }

    #[repr(C)]
    struct FileAttributeTagInfo {
        file_attributes: u32,
        reparse_tag: u32,
    }

    #[link(name = "ntdll")]
    unsafe extern "system" {
        fn NtCreateFile(
            handle: *mut Handle,
            desired_access: u32,
            object_attributes: *mut ObjectAttributes,
            io_status: *mut IoStatusBlock,
            allocation_size: *mut i64,
            file_attributes: u32,
            share_access: u32,
            create_disposition: u32,
            create_options: u32,
            ea_buffer: *mut c_void,
            ea_length: u32,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetFileInformationByHandleEx(handle: Handle, class: i32, information: *mut c_void, size: u32) -> i32;
        fn SetFileInformationByHandle(handle: Handle, class: i32, information: *const c_void, size: u32) -> i32;
    }

    pub(super) fn open_server_owned(path: &Path) -> std::io::Result<File> {
        let mut components = path.components().peekable();
        let mut anchor = PathBuf::from(".");
        if let Some(Component::Prefix(prefix)) = components.peek().copied() {
            anchor = PathBuf::from(prefix.as_os_str());
            components.next();
            if !matches!(components.next(), Some(Component::RootDir)) {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "drive-relative server root is not allowed"));
            }
            anchor.push("\\");
        } else if matches!(components.peek(), Some(Component::RootDir)) {
            anchor = PathBuf::from("\\");
            components.next();
        }
        let mut current = std::fs::OpenOptions::new().read(true).custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT).open(anchor)?;
        require_attributes(&current, true)?;
        for component in components {
            match component {
                Component::CurDir => {}
                Component::Normal(segment) => current = open_one(&current, segment, true)?,
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "server-owned root is not a normalized path")),
            }
        }
        Ok(current)
    }

    pub(super) fn open_directory_at<'a>(root: &File, segments: impl Iterator<Item = &'a OsStr>) -> std::io::Result<File> {
        let mut current = root.try_clone()?;
        for segment in segments {
            current = open_one(&current, segment, true)?;
        }
        Ok(current)
    }

    pub(super) fn open_regular_at<'a>(root: &File, mut segments: impl Iterator<Item = &'a OsStr>) -> std::io::Result<File> {
        let mut current = root.try_clone()?;
        let first = segments.next().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty relative file path"))?;
        let mut pending = first;
        for segment in segments {
            current = open_one(&current, pending, true)?;
            pending = segment;
        }
        open_one(&current, pending, false)
    }

    fn open_one(parent: &File, segment: &OsStr, directory: bool) -> std::io::Result<File> {
        open_one_access(parent, segment, directory, false, FILE_OPEN)
    }

    fn open_one_access(parent: &File, segment: &OsStr, directory: bool, write: bool, disposition: u32) -> std::io::Result<File> {
        let mut name = segment.encode_wide().collect::<Vec<_>>();
        if name.is_empty() || name.iter().any(|unit| *unit == 0) || name.len().checked_mul(2).and_then(|bytes| u16::try_from(bytes).ok()).is_none() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid rooted Windows path segment"));
        }
        let length = u16::try_from(name.len() * 2).expect("checked Windows path byte length");
        let mut unicode = UnicodeString { length, maximum_length: length, buffer: name.as_mut_ptr() };
        let mut attributes = ObjectAttributes {
            length: size_of::<ObjectAttributes>() as u32,
            root_directory: parent.as_raw_handle() as Handle,
            object_name: &mut unicode,
            attributes: OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
            security_descriptor: null_mut(),
            security_quality_of_service: null_mut(),
        };
        let mut io_status = IoStatusBlock { status: IoStatus { status: 0 }, information: 0 };
        let mut handle = INVALID_HANDLE_VALUE;
        let options = FILE_SYNCHRONOUS_IO_NONALERT | FILE_OPEN_REPARSE_POINT | if directory { FILE_DIRECTORY_FILE } else { FILE_NON_DIRECTORY_FILE };
        let status = unsafe {
            NtCreateFile(
                &mut handle,
                GENERIC_READ | SYNCHRONIZE | FILE_READ_ATTRIBUTES | if write { GENERIC_WRITE | DELETE } else { 0 },
                &mut attributes,
                &mut io_status,
                null_mut(),
                FILE_ATTRIBUTE_NORMAL,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                disposition,
                options,
                null_mut(),
                0,
            )
        };
        if status != STATUS_SUCCESS {
            let kind = if status == STATUS_OBJECT_NAME_NOT_FOUND || status == STATUS_OBJECT_PATH_NOT_FOUND { std::io::ErrorKind::NotFound } else { std::io::ErrorKind::PermissionDenied };
            return Err(std::io::Error::new(kind, format!("rooted NtCreateFile failed with status {status:#x}")));
        }
        let file = unsafe { File::from_raw_handle(handle as _) };
        require_attributes(&file, directory)?;
        Ok(file)
    }

    fn require_attributes(file: &File, directory: bool) -> std::io::Result<FileAttributeTagInfo> {
        let mut info = unsafe { zeroed::<FileAttributeTagInfo>() };
        let result = unsafe { GetFileInformationByHandleEx(file.as_raw_handle() as Handle, FILE_ATTRIBUTE_TAG_INFO_CLASS, (&mut info as *mut FileAttributeTagInfo).cast(), size_of::<FileAttributeTagInfo>() as u32) };
        if result == 0 {
            return Err(std::io::Error::last_os_error());
        }
        if info.file_attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 || (info.file_attributes & FILE_ATTRIBUTE_DIRECTORY != 0) != directory {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "opened object has a reparse or type mismatch"));
        }
        Ok(info)
    }

    pub(super) fn open_lock(parent: &File) -> std::io::Result<File> {
        let file = open_one_access(parent, OsStr::new(".publication.lock"), false, true, FILE_OPEN_IF)?;
        if fstat_regular_length(&file)? != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "publication lock is not empty"));
        }
        Ok(file)
    }

    pub(super) fn create_new(parent: &File, name: &OsStr) -> std::io::Result<File> {
        open_one_access(parent, name, false, true, FILE_CREATE)
    }

    pub(super) fn replace_current(parent: &File, file: &File, _: &OsStr) -> std::io::Result<()> {
        #[repr(C)]
        struct RenameInfo {
            flags: u32,
            root_directory: Handle,
            name_length: u32,
            name: [u16; 12],
        }
        let name: [u16; 12] = OsStr::new("current.json").encode_wide().collect::<Vec<_>>().try_into().expect("fixed current leaf");
        let info = RenameInfo { flags: 1, root_directory: parent.as_raw_handle() as Handle, name_length: 24, name };
        let length = std::mem::offset_of!(RenameInfo, name) + 24;
        if unsafe { SetFileInformationByHandle(file.as_raw_handle() as Handle, 3, (&info as *const RenameInfo).cast(), length as u32) } == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    pub(super) fn remove_owned(_: &File, file: &File, _: &OsStr) -> std::io::Result<()> {
        let delete: u8 = 1;
        if unsafe { SetFileInformationByHandle(file.as_raw_handle() as Handle, 4, (&delete as *const u8).cast(), 1) } == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    pub(super) fn sync_publication(_: &File) -> TrustedPublicationSync {
        TrustedPublicationSync::Unconfirmed
    }

    pub(super) fn fstat_regular_length(file: &File) -> std::io::Result<u64> {
        require_attributes(file, false)?;
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "opened object is not regular"));
        }
        Ok(metadata.len())
    }
}

#[cfg(not(any(unix, windows)))]
compile_error!("trusted catalog opened-root owner requires Unix or Windows descriptor support");

#[cfg(test)]
#[path = "🧪️tests/🔬️publication/🦀️.rs"]
mod publication_tests;
