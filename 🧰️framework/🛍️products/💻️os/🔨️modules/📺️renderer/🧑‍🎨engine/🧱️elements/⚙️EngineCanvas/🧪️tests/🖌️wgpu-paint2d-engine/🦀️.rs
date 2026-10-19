//! 🖌️ Wgpu paint-2d and text-editor attach — the production path that constructs a `RasterHost` /
//! `EditorHost` behind a `SurfaceKind::Paint2d` / `SurfaceKind::TextEditor` window, feeds it the
//! scene, paints it through the host's own vector renderer, and routes pointer/wheel/keyboard input.
//!
//! The action payloads are pinned against React's own hosts — the other implementation of these
//! surfaces (`🧱️elements/🖌️Paint2dHost/🟦️.tsx` — `world3dHoverActionArgs`/`world3dSelectionActionArgs`
//! under the `"layers"` domain, `dispatch("setCamera", { camera })`; `🧱️elements/✏️TextEditor/🟦️.tsx` —
//! the `textEdit`/`textSelect` pair every keystroke commits).

use super::node_graph_attach_tests::action_fields;
use semio_framework_os_renderer_wgpu_engine_attach::drop_engine_surface;
use super::*;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_ui_declaration::ui::SurfaceKind;
use semio_framework_ui_layout::layout::UiPresence;
use semio_framework_ui_scene_plane::Paint2dScene;
use semio_framework_ui_scene_views::TextEditorScene;
use semio_framework_ui_wgpu_input::InputState;

include!("🧰️scene-fixtures-and-drains/🦀️.rs");

include!("🎨️paint2d-attach-and-selection-publication/🦀️.rs");

include!("⌨️text-editor-attach-draft-and-key-laws/🦀️.rs");

include!("📤️text-editor-outbox-receipts-and-delivery/🦀️.rs");

include!("🧲️marquee-and-navigator-laws/🦀️.rs");

include!("🪣️host-retirement-fill-and-stroke-laws/🦀️.rs");

include!("📜️editor-scene-document-laws/🦀️.rs");
