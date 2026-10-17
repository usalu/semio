use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_apps, host_test_shell, test_app};
use super::*;
// 🩹️ `PanelTabKind` stopped being reachable through the shell module's own `use` list, and the
// `📌️panel-state` fixture moved from `🧑‍🎨engine/🧫️fixtures/` into this element's own `🧫️fixtures/`
// (panel-tab lane, 2026-09-13) — both are re-pointed here so the crate's test target builds again.
use semio_framework::PanelTabKind;

include!("🪟️spawned-documents-and-host-panel/🦀️.rs");

include!("📌️anchor-ids-resize-and-default-dock/🦀️.rs");

include!("🧱️anchor-geometry-and-state/🦀️.rs");

include!("🗄️dock-override-round-trip/🦀️.rs");

include!("⚙️settings-keybindings-hug-and-mobile/🦀️.rs");

include!("💬️command-form-and-agent-chat/🦀️.rs");

include!("🌳️tree-units-and-chrome-bands/🦀️.rs");
