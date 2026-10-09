//! 🪟️ Private first-party Win32 ABI used by the native terminal and ConPTY targets.

#![allow(non_camel_case_types, non_snake_case, clippy::upper_case_acronyms)]

use core::ffi::c_void;
use core::mem::size_of;

pub(crate) type HANDLE = *mut c_void;
pub(crate) type HPCON = isize;

pub(crate) const CREATE_UNICODE_ENVIRONMENT: u32 = 0x0000_0400;
pub(crate) const DISABLE_NEWLINE_AUTO_RETURN: u32 = 0x0000_0008;
pub(crate) const ENABLE_ECHO_INPUT: u32 = 0x0000_0004;
pub(crate) const ENABLE_EXTENDED_FLAGS: u32 = 0x0000_0080;
pub(crate) const ENABLE_LINE_INPUT: u32 = 0x0000_0002;
pub(crate) const ENABLE_PROCESSED_INPUT: u32 = 0x0000_0001;
pub(crate) const ENABLE_QUICK_EDIT_MODE: u32 = 0x0000_0040;
pub(crate) const ENABLE_VIRTUAL_TERMINAL_INPUT: u32 = 0x0000_0200;
pub(crate) const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0000_0004;
pub(crate) const ENABLE_WINDOW_INPUT: u32 = 0x0000_0008;
pub(crate) const EXTENDED_STARTUPINFO_PRESENT: u32 = 0x0008_0000;
pub(crate) const HANDLE_FLAG_INHERIT: u32 = 0x0000_0001;
pub(crate) const INFINITE: u32 = 0xFFFF_FFFF;
pub(crate) const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
pub(crate) const KEY_EVENT: u16 = 0x0001;
pub(crate) const VK_MENU: u16 = 0x0012;
pub(crate) const VK_SPACE: u16 = 0x0020;
pub(crate) const VK_2: u16 = 0x0032;
pub(crate) const CONTROL_KEYS: u32 = 0x0004 | 0x0008;
pub(crate) const WINDOW_BUFFER_SIZE_EVENT: u16 = 0x0004;
pub(crate) const CTRL_BREAK_EVENT: u32 = 1;
pub(crate) const CTRL_CLOSE_EVENT: u32 = 2;
pub(crate) const CTRL_LOGOFF_EVENT: u32 = 5;
pub(crate) const CTRL_SHUTDOWN_EVENT: u32 = 6;
pub(crate) const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: usize = 0x0002_0016;
pub(crate) const STILL_ACTIVE: i32 = 259;
pub(crate) const WAIT_OBJECT_0: u32 = 0;
pub(crate) const WAIT_TIMEOUT: u32 = 258;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct COORD {
    pub(crate) X: i16,
    pub(crate) Y: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct SMALL_RECT {
    pub(crate) Left: i16,
    pub(crate) Top: i16,
    pub(crate) Right: i16,
    pub(crate) Bottom: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct CONSOLE_SCREEN_BUFFER_INFO {
    pub(crate) dwSize: COORD,
    pub(crate) dwCursorPosition: COORD,
    pub(crate) wAttributes: u16,
    pub(crate) srWindow: SMALL_RECT,
    pub(crate) dwMaximumWindowSize: COORD,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct KEY_EVENT_RECORD {
    pub(crate) bKeyDown: i32,
    pub(crate) wRepeatCount: u16,
    pub(crate) wVirtualKeyCode: u16,
    pub(crate) wVirtualScanCode: u16,
    pub(crate) uChar: u16,
    pub(crate) dwControlKeyState: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct MOUSE_EVENT_RECORD {
    pub(crate) dwMousePosition: COORD,
    pub(crate) dwButtonState: u32,
    pub(crate) dwControlKeyState: u32,
    pub(crate) dwEventFlags: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct WINDOW_BUFFER_SIZE_RECORD {
    pub(crate) dwSize: COORD,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union INPUT_RECORD_0 {
    pub(crate) KeyEvent: KEY_EVENT_RECORD,
    pub(crate) MouseEvent: MOUSE_EVENT_RECORD,
    pub(crate) WindowBufferSizeEvent: WINDOW_BUFFER_SIZE_RECORD,
    pub(crate) Raw: [u32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct INPUT_RECORD {
    pub(crate) EventType: u16,
    pub(crate) Event: INPUT_RECORD_0,
}

impl Default for INPUT_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct SECURITY_ATTRIBUTES {
    pub(crate) nLength: u32,
    pub(crate) lpSecurityDescriptor: *mut c_void,
    pub(crate) bInheritHandle: i32,
}

impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct PROCESS_INFORMATION {
    pub(crate) hProcess: HANDLE,
    pub(crate) hThread: HANDLE,
    pub(crate) dwProcessId: u32,
    pub(crate) dwThreadId: u32,
}

impl Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct STARTUPINFOW {
    pub(crate) cb: u32,
    pub(crate) lpReserved: *mut u16,
    pub(crate) lpDesktop: *mut u16,
    pub(crate) lpTitle: *mut u16,
    pub(crate) dwX: u32,
    pub(crate) dwY: u32,
    pub(crate) dwXSize: u32,
    pub(crate) dwYSize: u32,
    pub(crate) dwXCountChars: u32,
    pub(crate) dwYCountChars: u32,
    pub(crate) dwFillAttribute: u32,
    pub(crate) dwFlags: u32,
    pub(crate) wShowWindow: u16,
    pub(crate) cbReserved2: u16,
    pub(crate) lpReserved2: *mut u8,
    pub(crate) hStdInput: HANDLE,
    pub(crate) hStdOutput: HANDLE,
    pub(crate) hStdError: HANDLE,
}

impl Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct STARTUPINFOEXW {
    pub(crate) StartupInfo: STARTUPINFOW,
    pub(crate) lpAttributeList: *mut c_void,
}

impl Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct OVERLAPPED {
    pub(crate) Internal: usize,
    pub(crate) InternalHigh: usize,
    pub(crate) Anonymous: OVERLAPPED_0,
    pub(crate) hEvent: HANDLE,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) union OVERLAPPED_0 {
    pub(crate) Anonymous: OVERLAPPED_0_0,
    pub(crate) Pointer: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub(crate) struct OVERLAPPED_0_0 {
    pub(crate) Offset: u32,
    pub(crate) OffsetHigh: u32,
}

#[link(name = "kernel32")]
extern "system" {
    pub(crate) fn CreateJobObjectW(attributes: *const SECURITY_ATTRIBUTES, name: *const u16) -> HANDLE;
    pub(crate) fn SetInformationJobObject(job: HANDLE, class: i32, information: *const c_void, length: u32) -> i32;
    pub(crate) fn AssignProcessToJobObject(job: HANDLE, process: HANDLE) -> i32;
    pub(crate) fn TerminateJobObject(job: HANDLE, exit_code: u32) -> i32;
    pub(crate) fn ResumeThread(thread: HANDLE) -> u32;
    pub(crate) fn CloseHandle(handle: HANDLE) -> i32;
    pub(crate) fn SetHandleInformation(handle: HANDLE, mask: u32, flags: u32) -> i32;
    pub(crate) fn ReadFile(handle: HANDLE, buffer: *mut u8, bytes_to_read: u32, bytes_read: *mut u32, overlapped: *mut OVERLAPPED) -> i32;
    pub(crate) fn WriteFile(handle: HANDLE, buffer: *const u8, bytes_to_write: u32, bytes_written: *mut u32, overlapped: *mut OVERLAPPED) -> i32;
    pub(crate) fn ClosePseudoConsole(hpcon: HPCON);
    pub(crate) fn CreatePseudoConsole(size: COORD, input: HANDLE, output: HANDLE, flags: u32, hpcon: *mut HPCON) -> i32;
    pub(crate) fn GetConsoleMode(console: HANDLE, mode: *mut u32) -> i32;
    pub(crate) fn GetConsoleCP() -> u32;
    pub(crate) fn GetConsoleOutputCP() -> u32;
    pub(crate) fn SetConsoleCP(code_page: u32) -> i32;
    pub(crate) fn SetConsoleOutputCP(code_page: u32) -> i32;
    pub(crate) fn GetConsoleScreenBufferInfo(console: HANDLE, info: *mut CONSOLE_SCREEN_BUFFER_INFO) -> i32;
    pub(crate) fn ResizePseudoConsole(hpcon: HPCON, size: COORD) -> i32;
    pub(crate) fn SetConsoleMode(console: HANDLE, mode: u32) -> i32;
    pub(crate) fn CreatePipe(read_pipe: *mut HANDLE, write_pipe: *mut HANDLE, attributes: *const SECURITY_ATTRIBUTES, size: u32) -> i32;
    pub(crate) fn CreateProcessW(
        application_name: *const u16,
        command_line: *mut u16,
        process_attributes: *const SECURITY_ATTRIBUTES,
        thread_attributes: *const SECURITY_ATTRIBUTES,
        inherit_handles: i32,
        creation_flags: u32,
        environment: *const c_void,
        current_directory: *const u16,
        startup_info: *const STARTUPINFOW,
        process_information: *mut PROCESS_INFORMATION,
    ) -> i32;
    pub(crate) fn DeleteProcThreadAttributeList(attribute_list: *mut c_void);
    pub(crate) fn GetExitCodeProcess(process: HANDLE, exit_code: *mut u32) -> i32;
    pub(crate) fn GetProcessId(process: HANDLE) -> u32;
    pub(crate) fn InitializeProcThreadAttributeList(attribute_list: *mut c_void, attribute_count: u32, flags: u32, size: *mut usize) -> i32;
    pub(crate) fn TerminateProcess(process: HANDLE, exit_code: u32) -> i32;
    pub(crate) fn UpdateProcThreadAttribute(attribute_list: *mut c_void, flags: u32, attribute: usize, value: *const c_void, size: usize, previous_value: *mut c_void, return_size: *const usize) -> i32;
    pub(crate) fn WaitForSingleObject(handle: HANDLE, milliseconds: u32) -> u32;
    pub(crate) fn WaitForMultipleObjects(count: u32, handles: *const HANDLE, wait_all: i32, milliseconds: u32) -> u32;
    pub(crate) fn CreateEventW(attributes: *const SECURITY_ATTRIBUTES, manual_reset: i32, initial_state: i32, name: *const u16) -> HANDLE;
    pub(crate) fn SetEvent(event: HANDLE) -> i32;
    pub(crate) fn GetNumberOfConsoleInputEvents(console: HANDLE, count: *mut u32) -> i32;
    pub(crate) fn ReadConsoleInputW(console: HANDLE, buffer: *mut INPUT_RECORD, length: u32, read: *mut u32) -> i32;
    pub(crate) fn SetConsoleCtrlHandler(handler: Option<unsafe extern "system" fn(u32) -> i32>, add: i32) -> i32;
}

/// 🌳 The native job limit layout; closing the last job handle terminates its process tree.
#[repr(C)]
#[derive(Default)]
pub(crate) struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub(crate) PerProcessUserTimeLimit: i64,
    pub(crate) PerJobUserTimeLimit: i64,
    pub(crate) LimitFlags: u32,
    pub(crate) MinimumWorkingSetSize: usize,
    pub(crate) MaximumWorkingSetSize: usize,
    pub(crate) ActiveProcessLimit: u32,
    pub(crate) Affinity: usize,
    pub(crate) PriorityClass: u32,
    pub(crate) SchedulingClass: u32,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub(crate) BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub(crate) IoInfo: [u64; 6],
    pub(crate) ProcessMemoryLimit: usize,
    pub(crate) JobMemoryLimit: usize,
    pub(crate) PeakProcessMemoryUsed: usize,
    pub(crate) PeakJobMemoryUsed: usize,
}

/// 🔒 An owning Win32 handle that closes exactly once and rejects sentinel values.
#[repr(transparent)]
pub(crate) struct OwnedHandle(HANDLE);

impl OwnedHandle {
    pub(crate) unsafe fn from_raw(handle: HANDLE) -> Option<Self> {
        (!handle.is_null() && handle != INVALID_HANDLE_VALUE).then_some(Self(handle))
    }

    pub(crate) fn as_raw(&self) -> HANDLE {
        self.0
    }
}

impl Drop for OwnedHandle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

/// 🫧 An owning pseudo-console handle that closes exactly once.
#[repr(transparent)]
pub(crate) struct OwnedPseudoConsole(HPCON);

impl OwnedPseudoConsole {
    pub(crate) unsafe fn from_raw(hpcon: HPCON) -> Option<Self> {
        (hpcon != 0).then_some(Self(hpcon))
    }

    pub(crate) fn as_raw(&self) -> HPCON {
        self.0
    }
}

impl Drop for OwnedPseudoConsole {
    fn drop(&mut self) {
        let hpcon = self.0;
        if std::thread::Builder::new().name("ConPTY cleanup".into()).stack_size(64 * 1024).spawn(move || unsafe { ClosePseudoConsole(hpcon) }).is_err() {
            unsafe { ClosePseudoConsole(hpcon); }
        }
    }
}

/// 🧵 An aligned initialized process-thread attribute list with exact teardown ownership.
pub(crate) struct ProcThreadAttributeList {
    storage: Vec<usize>,
}

impl ProcThreadAttributeList {
    pub(crate) unsafe fn new(attribute_count: u32) -> std::io::Result<Self> {
        let mut bytes = 0usize;
        unsafe {
            InitializeProcThreadAttributeList(core::ptr::null_mut(), attribute_count, 0, &mut bytes);
        }
        if bytes == 0 {
            return Err(std::io::Error::last_os_error());
        }
        let words = bytes.div_ceil(size_of::<usize>());
        let mut storage = vec![0usize; words];
        if unsafe { InitializeProcThreadAttributeList(storage.as_mut_ptr().cast(), attribute_count, 0, &mut bytes) } == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Self { storage })
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut c_void {
        self.storage.as_mut_ptr().cast()
    }

    pub(crate) unsafe fn set_pseudo_console(&mut self, hpcon: HPCON) -> std::io::Result<()> {
        if unsafe { UpdateProcThreadAttribute(self.as_mut_ptr(), 0, PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE, hpcon as *const c_void, size_of::<HPCON>(), core::ptr::null_mut(), core::ptr::null()) } == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for ProcThreadAttributeList {
    fn drop(&mut self) {
        unsafe {
            DeleteProcThreadAttributeList(self.as_mut_ptr());
        }
    }
}

