//! 🐧️ Unix native terminal module: the wake pipe and signals and the native terminal, with the shared imports of the former inline module. See [`super`].

use super::native_common::*;
use super::*;
use crate::ansi::{setup_sequence, teardown_sequence};
use std::cell::UnsafeCell;
use std::io::Write;
use std::mem::MaybeUninit;
use std::os::unix::io::RawFd;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[path = "🌬️wake-pipe-and-signals/🦀️.rs"]
mod wake_pipe_and_signals;
use wake_pipe_and_signals::{
    arm_restore, disarm_restore, err, restore_for_panic, retry_mode_restoration, write_byte,
    SignalGuard, WakePipe, WakeState, PENDING_RESIZE, READ_BURST, SIGNAL_PIPE,
};
#[cfg(test)]
use wake_pipe_and_signals::restore_now;
#[cfg(not(target_os="macos"))]
pub(super) use wake_pipe_and_signals::wait_readable;
#[cfg(target_os="macos")]
pub(super) use wake_pipe_and_signals::wait_readable;

#[path = "🖥️unix-native-terminal/🦀️.rs"]
mod unix_native_terminal;
pub use unix_native_terminal::NativeTerminal;
