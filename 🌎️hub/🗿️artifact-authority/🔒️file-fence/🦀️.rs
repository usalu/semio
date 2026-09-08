//! 🔒️ OS-released exclusive file ownership shared by Hub publication domains.

/// 🛡️ One lock held until this file handle is explicitly unlocked and closed.
pub(crate) struct FileFence(std::fs::File);

#[cfg(unix)]
pub(crate) fn try_acquire(file: std::fs::File) -> std::io::Result<Option<FileFence>> {
    use std::os::fd::AsRawFd as _;
    unsafe extern "C" {
        fn flock(fd: i32, operation: i32) -> i32;
    }
    if unsafe { flock(file.as_raw_fd(), 2 | 4) } == 0 {
        return Ok(Some(FileFence(file)));
    }
    let error = std::io::Error::last_os_error();
    if matches!(error.kind(), std::io::ErrorKind::WouldBlock) {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(unix)]
impl Drop for FileFence {
    fn drop(&mut self) {
        use std::os::fd::AsRawFd as _;
        unsafe extern "C" {
            fn flock(fd: i32, operation: i32) -> i32;
        }
        let _ = unsafe { flock(self.0.as_raw_fd(), 8) };
    }
}

#[cfg(windows)]
pub(crate) fn try_acquire(file: std::fs::File) -> std::io::Result<Option<FileFence>> {
    use std::os::windows::io::AsRawHandle as _;
    #[repr(C)]
    struct Overlapped {
        internal: usize,
        internal_high: usize,
        offset: u32,
        offset_high: u32,
        event: *mut core::ffi::c_void,
    }
    unsafe extern "system" {
        fn LockFileEx(file: *mut core::ffi::c_void, flags: u32, reserved: u32, low: u32, high: u32, overlapped: *mut Overlapped) -> i32;
    }
    let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: std::ptr::null_mut() };
    if unsafe { LockFileEx(file.as_raw_handle(), 2 | 1, 0, 1, 0, &mut overlapped) } != 0 {
        return Ok(Some(FileFence(file)));
    }
    let error = std::io::Error::last_os_error();
    if matches!(error.raw_os_error(), Some(33 | 158)) {
        Ok(None)
    } else {
        Err(error)
    }
}

#[cfg(windows)]
impl Drop for FileFence {
    fn drop(&mut self) {
        use std::os::windows::io::AsRawHandle as _;
        #[repr(C)]
        struct Overlapped {
            internal: usize,
            internal_high: usize,
            offset: u32,
            offset_high: u32,
            event: *mut core::ffi::c_void,
        }
        unsafe extern "system" {
            fn UnlockFileEx(file: *mut core::ffi::c_void, reserved: u32, low: u32, high: u32, overlapped: *mut Overlapped) -> i32;
        }
        let mut overlapped = Overlapped { internal: 0, internal_high: 0, offset: 0, offset_high: 0, event: std::ptr::null_mut() };
        let _ = unsafe { UnlockFileEx(self.0.as_raw_handle(), 0, 1, 0, &mut overlapped) };
    }
}
