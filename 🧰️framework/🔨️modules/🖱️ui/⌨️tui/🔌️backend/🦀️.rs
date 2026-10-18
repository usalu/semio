use crate::ansi::AnsiPatch;
use semio_framework_ui_tui_core::event::Event;
use semio_framework_ui_tui_core::geometry::Size;
use semio_framework_ui_tui_core::signal::CursorSpec;
use std::io;

#[path = "📋️clipboard-and-osc52/🦀️.rs"]
mod clipboard_and_osc52;
pub use clipboard_and_osc52::{
    osc52_copy_sequence, BackendError, Clipboard, ClipboardResult, HostClipboard, MemoryClipboard,
    OSC52_LIMIT,
};
#[cfg(any(test, all(feature = "native", not(target_arch = "wasm32"), any(unix, windows))))]
use clipboard_and_osc52::osc52_payload_check;

#[path = "🔎️capabilities-and-frame-presenter/🦀️.rs"]
mod capabilities_and_frame_presenter;
pub use capabilities_and_frame_presenter::{
    detect_capabilities, Capabilities, ColorDepth, FramePresenter, TerminalBackend, UnicodeLevel,
    Waker,
};
#[cfg(all(feature = "native", not(target_arch = "wasm32"), any(unix, windows)))]
use capabilities_and_frame_presenter::process_environment;

//#region 📋️Clipboard

#[cfg(all(test, not(target_arch = "wasm32")))]
include!("../🧪️tests/🔬️backend-clipboard-mailbox/🦀️.rs");
//#endregion 📋️Clipboard

#[cfg(test)]
include!("../🧪️tests/🔬️backend-capabilities/🦀️.rs");

#[cfg(all(feature = "native", not(target_arch = "wasm32"), any(unix, windows)))]
mod native_common {
    use super::*;
    use crate::ansi::{AnsiParser, ClickCounter};
    use std::time::{Duration, Instant};

    pub fn release_owned_terminal_cleanup(owned: &mut bool, succeeded: bool) -> bool {
        if *owned && succeeded {
            *owned = false;
        }
        !*owned
    }

    pub fn terminal_entry_is_available(entered: bool, ansi_setup_owned: bool, platform_cleanup_empty: bool) -> bool {
        !entered && !ansi_setup_owned && platform_cleanup_empty
    }

    pub fn ceil_millis(duration: Duration) -> u64 {
        (duration.as_micros() as u64).div_ceil(1000)
    }

    /// 📥️ Raw terminal bytes in, stamped events out, with the partial-input clock the parser needs.
    pub struct InputPipeline {
        parser: AnsiParser,
        clicks: ClickCounter,
        epoch: Instant,
        last_input: Instant,
    }

    impl InputPipeline {
        pub fn new() -> Self {
            let now = Instant::now();
            Self { parser: AnsiParser::new(), clicks: ClickCounter::default(), epoch: now, last_input: now }
        }

        pub fn feed(&mut self, bytes: &[u8], events: &mut Vec<Event>) {
            let now = Instant::now();
            self.expire_due(now, events);
            let start = events.len();
            self.parser.feed(bytes, events);
            self.last_input = now;
            let now_ms = now.saturating_duration_since(self.epoch).as_millis() as u64;
            self.clicks.stamp_all(&mut events[start..], now_ms);
        }

        pub fn remaining(&self, now: Instant) -> Option<Duration> {
            self.parser.pending_timeout().map(|timeout| (self.last_input + timeout).saturating_duration_since(now))
        }

        pub fn expire_due(&mut self, now: Instant, events: &mut Vec<Event>) {
            if self.remaining(now).is_some_and(|remaining| remaining.is_zero()) {
                self.parser.expire(events);
            }
        }
    }

    /// ⏲️ How long one platform wait may block: the nearer of the caller's deadline and the parser's partial-input deadline.
    pub fn next_timeout(deadline: Option<Instant>, input: &InputPipeline, now: Instant) -> Option<Duration> {
        let until_deadline = deadline.map(|deadline| deadline.saturating_duration_since(now));
        match (until_deadline, input.remaining(now)) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }

    /// 📐️ Remembers the last reported size so spurious resize signals produce no event.
    #[derive(Default)]
    pub struct ResizeTracker {
        last: Option<Size>,
    }

    impl ResizeTracker {
        pub fn baseline(&mut self, size: Size) {
            self.last = Some(size);
        }

        pub fn observe(&mut self, size: Size) -> Option<Event> {
            if size.width == 0 || size.height == 0 || self.last == Some(size) {
                return None;
            }
            self.last = Some(size);
            Some(Event::Resize(size))
        }
    }

    /// 🛟️ Restores the terminal before a panic message prints, only when the panicking thread entered it.
    pub mod panic_restore {
        use std::sync::{Mutex, Once};
        use std::thread::ThreadId;

        static OWNER: Mutex<Option<ThreadId>> = Mutex::new(None);
        static HOOK: Once = Once::new();

        pub fn claim() {
            *OWNER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(std::thread::current().id());
        }

        pub fn release() {
            *OWNER.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
        }

        pub fn install(fire: fn()) {
            HOOK.call_once(move || {
                let previous = std::panic::take_hook();
                std::panic::set_hook(Box::new(move |info| {
                    let owned = OWNER.try_lock().is_ok_and(|owner| *owner == Some(std::thread::current().id()));
                    if owned {
                        fire();
                    }
                    previous(info);
                }));
            });
        }
    }

    #[cfg(test)]
    include!("../🧪️tests/🔬️backend-native-common/🦀️.rs");
}

#[cfg(all(feature = "native", unix, not(target_arch = "wasm32")))]
#[path = "🐧️native-unix/🦀️.rs"]
mod native_unix;
#[cfg(all(feature = "native", unix, not(target_arch = "wasm32")))]
pub use native_unix::NativeTerminal;

#[cfg(all(feature = "native", windows))]
#[path = "🪟️native-windows/🦀️.rs"]
mod native_windows;
#[cfg(all(feature = "native", windows))]
pub use native_windows::NativeTerminal;
