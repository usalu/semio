//! 🎨️♿️ Laws for ticket 26/09/17/WGPU-RENDERER-REACT-PARITY packet W15d — the settings panel's DEPTH
//! (the theme editor's eight sections, General/Theme completeness) and control ACCESSIBILITY
//! (accessible names, inline hotkey badges, keybinding-registry drift). Audit `📓️audit-w14-shell-
//! residual.md` items §B8, §B2, §B3, §C4, §B14 and the §E keybindings table.
//!
//! Each law names the React source it mirrors. The React sources are
//! `📌️ChromePanels/🟦️.tsx`'s `buildSettingsThemeTree`/`buildSettingsGeneralTree`,
//! `🏛️ShellHost/🟦️.tsx`'s theme mutators/`exportTheme`/`importTheme`, and
//! `🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx`'s `SHELL_KEYBINDINGS`.

use semio_framework_os_renderer_wgpu_fixtures::shell::split_pane_shell;
use super::*;

include!("🧰️theme-editor-fixtures/🦀️.rs");

include!("🎚️theme-publication-and-sections/🦀️.rs");

include!("🔁️theme-tokens-and-leaves/🦀️.rs");

include!("♿️accessible-names-and-activation/🦀️.rs");

include!("🪞️mirror-address-liveness/🦀️.rs");

include!("🧭️settings-strip-and-keybindings/🦀️.rs");

include!("📐️passive-regions-and-geometry/🦀️.rs");
