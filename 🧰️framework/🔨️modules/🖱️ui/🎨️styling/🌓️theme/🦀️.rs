    use super::generated::{BoardPalette, CanvasPalette, ChromePalette, MapPalette, BOARD_DARK, BOARD_LIGHT, CANVAS_DARK, CANVAS_LIGHT, CHROME_DARK, CHROME_LIGHT, MAP_DARK, MAP_LIGHT};

    /// @emoji 🎨️ Active appearance (light/dark) for canvas hosts.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AppearanceName {
        Light,
        Dark,
    }

    impl AppearanceName {
        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        pub fn board(self) -> &'static BoardPalette {
            match self {
                Self::Light => &BOARD_LIGHT,
                Self::Dark => &BOARD_DARK,
            }
        }

        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        pub fn map(self) -> &'static MapPalette {
            match self {
                Self::Light => &MAP_LIGHT,
                Self::Dark => &MAP_DARK,
            }
        }

        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        pub fn canvas(self) -> &'static CanvasPalette {
            match self {
                Self::Light => &CANVAS_LIGHT,
                Self::Dark => &CANVAS_DARK,
            }
        }

        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        pub fn chrome(self) -> &'static ChromePalette {
            match self {
                Self::Light => &CHROME_LIGHT,
                Self::Dark => &CHROME_DARK,
            }
        }

        // 🚫️async: E1 pure accessor consumed by external-trait impls (Default) and sync render/paint call sites — see R9
        pub fn parse(s: &str) -> Self {
            if s.eq_ignore_ascii_case("dark") {
                Self::Dark
            } else {
                Self::Light
            }
        }
    }
