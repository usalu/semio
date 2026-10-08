use crate::tui::ansi::{setup_sequence, teardown_sequence, AnsiParser};
use crate::tui::engine::Tui;
use crate::tui::event::Event;
use crate::tui::geometry::Size;
use crate::tui::theme::Theme;
use ui_styling::appearance::AppearanceName;

/// ??? A pure bytes-in/string-out host: feed terminal input, get an ANSI patch back.
pub struct WasmHost {
    pub tui: Tui,
    parser: AnsiParser,
}

impl WasmHost {
    pub fn new(width: u16, height: u16, dark: bool) -> Self {
        let appearance = if dark { AppearanceName::Dark } else { AppearanceName::Light };
        Self { tui: Tui::new(Size { width, height }, Theme::new(appearance)), parser: AnsiParser::new() }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Vec<Event> {
        let mut events = Vec::new();
        self.parser.feed(bytes, &mut events);
        for event in &events {
            self.tui.dispatch(event);
        }
        events
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.tui.dispatch(&Event::Resize(Size { width, height }));
    }

    pub fn render(&mut self) -> String {
        self.tui.render().0
    }

    pub fn setup(&self) -> String {
        setup_sequence().to_string()
    }

    pub fn teardown(&self) -> String {
        teardown_sequence().to_string()
    }
}

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this is a browser-only
// xterm.js bridge around the pure `WasmHost` renderer, so it is narrowed to exclude the
// WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "tui-bindgen"))]
mod bindgen_host {
    use super::WasmHost;
    use wasm_bindgen::prelude::*;

    /// ??? The `wasm-bindgen` surface for browser hosts (e.g. an xterm.js terminal).
    #[wasm_bindgen]
    pub struct TuiHost(WasmHost);

    #[wasm_bindgen]
    impl TuiHost {
        #[wasm_bindgen(constructor)]
        pub fn new(width: u16, height: u16, dark: bool) -> TuiHost {
            TuiHost(WasmHost::new(width, height, dark))
        }

        pub fn feed(&mut self, bytes: &[u8]) {
            self.0.feed(bytes);
        }

        pub fn resize(&mut self, width: u16, height: u16) {
            self.0.resize(width, height);
        }

        pub fn render(&mut self) -> String {
            self.0.render()
        }

        pub fn setup(&self) -> String {
            self.0.setup()
        }

        pub fn teardown(&self) -> String {
            self.0.teardown()
        }
    }
}
#[cfg(all(target_arch = "wasm32", not(target_env = "p2"), feature = "tui-bindgen"))]
pub use bindgen_host::TuiHost;
