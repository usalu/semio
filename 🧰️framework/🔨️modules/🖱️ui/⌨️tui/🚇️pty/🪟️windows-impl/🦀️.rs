//! 🪟️ Windows pty module: the console events and spawn, the handle with its command line builders and the process implementation, with the shared imports of the former inline module. See [`super`].

use super::*;
use crate::windows_abi::{AssignProcessToJobObject, CreateJobObjectW, JOBOBJECT_EXTENDED_LIMIT_INFORMATION, ResumeThread, SetInformationJobObject, TerminateJobObject, CreatePipe, CreateProcessW, CreatePseudoConsole, GetExitCodeProcess, GetProcessId, OwnedHandle, OwnedPseudoConsole, ProcThreadAttributeList, ReadFile, ResizePseudoConsole, SetHandleInformation, TerminateProcess, WaitForSingleObject, WriteFile, COORD, CREATE_UNICODE_ENVIRONMENT, EXTENDED_STARTUPINFO_PRESENT, HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE, PROCESS_INFORMATION, SECURITY_ATTRIBUTES, STARTUPINFOEXW, STILL_ACTIVE, WAIT_OBJECT_0, WAIT_TIMEOUT};
use std::collections::VecDeque;
use std::ffi::OsStr;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::sync::{Arc, Condvar, Mutex};

#[path = "🔔️console-events-and-spawn/🦀️.rs"]
mod console_events_and_spawn;
use console_events_and_spawn::{
    console_event, Pipe, Shared, CONTROL_C_EXIT, CTRL_BREAK_EVENT,
    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, PIPE_BACKLOG_BYTES, WAIT_FOREVER,
};
pub use console_events_and_spawn::{interrupts, spawn_detached, Notifier};

#[path = "🧾️pty-handle-and-command-line/🦀️.rs"]
mod pty_handle_and_command_line;
pub use pty_handle_and_command_line::{resolve_program, Pty};
use pty_handle_and_command_line::{
    build_cmdline, build_env_block, build_exact_env_block, runs_batch, to_wide,
};

#[path = "🏃️pty-process-impl/🦀️.rs"]
mod pty_process_impl;
