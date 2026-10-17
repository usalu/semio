//! 📨️ Framed pipe I/O: deadlines, bounded frame reads and writes and the inherited descriptor handle.

use super::*;

pub(super) fn checked_deadline(now: i64) -> DirectoryResult<i64> {
    now.checked_add(LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS).ok_or_else(resource_limit)
}

fn remaining(context: &IdentityVerificationContext<'_>) -> DirectoryResult<std::time::Duration> {
    let remaining = context.deadline_ms.checked_sub(context.control.now_ms()).ok_or_else(|| DirectoryError::Conflict("local bootstrap deadline exceeded".into()))?;
    if remaining <= 0 || context.control.is_cancelled() {
        return Err(DirectoryError::Conflict("local bootstrap deadline or cancellation reached".into()));
    }
    Ok(std::time::Duration::from_millis(remaining as u64))
}

pub(super) async fn read_frame(reader: &mut (impl tokio::io::AsyncRead + Unpin), context: &IdentityVerificationContext<'_>) -> DirectoryResult<Option<Box<[u8]>>> {
    let mut prefix = [0u8; 4];
    match tokio::time::timeout(remaining(context)?, reader.read_exact(&mut prefix)).await {
        Ok(Ok(_)) => {}
        Ok(Err(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
        Ok(Err(_)) => return Err(unavailable()),
        Err(_) => return Err(DirectoryError::Conflict("local bootstrap read deadline exceeded".into())),
    }
    let length = u32::from_be_bytes(prefix) as usize;
    if length == 0 || length.checked_add(4).is_none_or(|complete| complete > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
        return Err(resource_limit());
    }
    let mut bytes = vec![0; length].into_boxed_slice();
    match tokio::time::timeout(remaining(context)?, reader.read_exact(&mut bytes)).await {
        Ok(Ok(_)) => Ok(Some(bytes)),
        Ok(Err(_)) => Err(denied()),
        Err(_) => Err(DirectoryError::Conflict("local bootstrap frame deadline exceeded".into())),
    }
}

pub(super) async fn read_admitted_frame(reader: &mut (impl tokio::io::AsyncRead + Unpin), control: &dyn IdentityVerificationControl) -> DirectoryResult<Option<(Box<[u8]>, i64)>> {
    let mut prefix = [0u8; 4];
    loop {
        if control.is_cancelled() {
            return Err(DirectoryError::Conflict("local bootstrap cancellation reached while idle".into()));
        }
        match tokio::time::timeout(std::time::Duration::from_millis(LOCAL_BOOTSTRAP_IDLE_POLL_MS), reader.read_exact(&mut prefix[..1])).await {
            Ok(Ok(_)) => break,
            Ok(Err(error)) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(None),
            Ok(Err(_)) => return Err(unavailable()),
            Err(_) => continue,
        }
    }
    let deadline_ms = checked_deadline(control.now_ms())?;
    let context = IdentityVerificationContext { deadline_ms, control };
    control.report(semio_hub_directory_contract::IdentityVerificationProgress { completed_units: 1, total_units: 2 });
    match tokio::time::timeout(remaining(&context)?, reader.read_exact(&mut prefix[1..])).await {
        Ok(Ok(_)) => {}
        Ok(Err(_)) => return Err(denied()),
        Err(_) => return Err(DirectoryError::Conflict("local bootstrap frame deadline exceeded".into())),
    }
    let length = u32::from_be_bytes(prefix) as usize;
    if length == 0 || length.checked_add(4).is_none_or(|complete| complete > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
        return Err(resource_limit());
    }
    let mut bytes = vec![0; length].into_boxed_slice();
    match tokio::time::timeout(remaining(&context)?, reader.read_exact(&mut bytes)).await {
        Ok(Ok(_)) => {
            control.report(semio_hub_directory_contract::IdentityVerificationProgress { completed_units: 2, total_units: 2 });
            Ok(Some((bytes, deadline_ms)))
        }
        Ok(Err(_)) => Err(denied()),
        Err(_) => Err(DirectoryError::Conflict("local bootstrap frame deadline exceeded".into())),
    }
}

pub(super) async fn write_frame(writer: &mut (impl tokio::io::AsyncWrite + Unpin), bytes: &[u8], context: &IdentityVerificationContext<'_>) -> DirectoryResult<()> {
    if bytes.is_empty() || bytes.len().checked_add(4).is_none_or(|complete| complete > LOCAL_BOOTSTRAP_FRAME_BYTES_MAX) {
        return Err(resource_limit());
    }
    let length = u32::try_from(bytes.len()).map_err(|_| resource_limit())?.to_be_bytes();
    tokio::time::timeout(remaining(context)?, async {
        writer.write_all(&length).await?;
        writer.write_all(bytes).await?;
        writer.flush().await
    })
    .await
    .map_err(|_| DirectoryError::Conflict("local bootstrap write deadline exceeded".into()))?
    .map_err(|_| unavailable())
}

#[cfg(unix)]
pub(super) fn inherited_bootstrap_file() -> DirectoryResult<std::fs::File> {
    use std::os::fd::FromRawFd;
    unsafe extern "C" {
        fn fcntl(fd: i32, command: i32, ...) -> i32;
    }
    if unsafe { fcntl(INHERITED_BOOTSTRAP_DESCRIPTOR, 1) } < 0 {
        return Err(unavailable());
    }
    Ok(unsafe { std::fs::File::from_raw_fd(INHERITED_BOOTSTRAP_DESCRIPTOR) })
}

#[cfg(unix)]
pub(super) fn into_async_bootstrap_io(file: std::fs::File) -> DirectoryResult<LocalBootstrapIo> {
    use std::os::fd::{FromRawFd, IntoRawFd};
    let stream = unsafe { std::os::unix::net::UnixStream::from_raw_fd(file.into_raw_fd()) };
    stream.set_nonblocking(true).map_err(|_| unavailable())?;
    tokio::net::UnixStream::from_std(stream).map_err(|_| unavailable())
}

#[cfg(windows)]
pub(super) fn inherited_bootstrap_file() -> DirectoryResult<std::fs::File> {
    use std::os::windows::io::FromRawHandle;
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
        fn DuplicateHandle(source_process: *mut std::ffi::c_void, source_handle: *mut std::ffi::c_void, target_process: *mut std::ffi::c_void, target_handle: *mut *mut std::ffi::c_void, desired_access: u32, inherit_handle: i32, options: u32) -> i32;
    }
    let handle = unsafe { _get_osfhandle(INHERITED_BOOTSTRAP_DESCRIPTOR) };
    if handle == -1 {
        return Err(unavailable());
    }
    let process = unsafe { GetCurrentProcess() };
    let mut duplicate = std::ptr::null_mut();
    let duplicated = unsafe { DuplicateHandle(process, handle as *mut std::ffi::c_void, process, &mut duplicate, 0, 0, 0x0000_0002) };
    if duplicated == 0 || duplicate.is_null() {
        return Err(unavailable());
    }
    Ok(unsafe { std::fs::File::from_raw_handle(duplicate) })
}

#[cfg(windows)]
pub(super) fn into_async_bootstrap_io(file: std::fs::File) -> DirectoryResult<LocalBootstrapIo> {
    Ok(tokio::fs::File::from_std(file))
}
