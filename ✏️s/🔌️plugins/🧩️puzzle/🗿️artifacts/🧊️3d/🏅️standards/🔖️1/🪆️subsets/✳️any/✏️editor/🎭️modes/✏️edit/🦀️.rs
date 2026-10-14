//! ✏️ Puzzle 3d play app — the `edit` mode: the dual-pane default layout (an orthographic Top pane
//! on the left third, a three-point Perspective pane on the right two thirds — two INSTANCES of the
//! one `🪟️windows/🧊️main` window kind) plus the mode-level Fill tool. The per-window chrome options
//! every instance shares live in `☑️options/*`.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use crate::windows::main;
use semio_framework_plugin::create_window_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutAxisNode;
use semio_framework_plugin::WindowLayoutChild;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;

pub const PUZZLE3D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::puzzle3d::create_puzzle3d_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: PUZZLE3D_PLAY_MODE_EDIT.into(),
        label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(),
        tools: vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_FILL_TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ Top (left ⅓) + Perspective (right ⅔) — the default dual-pane workbench for Puzzle 3D and the Aggregator.
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(100.0 / 3.0),
                    active_window_kind_id: None,
                    children: vec![create_window_layout(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, Some("Top".into()), Some(main::WINDOW_INSTANCE_TOP.into()), Some(main::TEMPLATE_TOP.into()))],
                }),
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(200.0 / 3.0),
                    active_window_kind_id: None,
                    children: vec![create_window_layout(semio_s_artifact_puzzle_3d_editor_config::WINDOW_KIND_ID, Some("Perspective".into()), Some(main::WINDOW_INSTANCE_PERSPECTIVE.into()), Some(main::TEMPLATE_PERSPECTIVE.into()))],
                }),
            ],
        }),
    }
}
//#endregion 🔖️Definition


//#region 🌳️Modules
#[path = "."]
pub mod options {
    #[path = "☑️options/🌐️grid/🦀️.rs"]
    pub mod grid;
    #[path = "☑️options/🔭️lod/🦀️.rs"]
    pub mod lod;
    #[path = "☑️options/🎥️projection/🦀️.rs"]
    pub mod projection;
    #[path = "☑️options/🎯️select/🦀️.rs"]
    pub mod select;
    #[path = "☑️options/☀️sun/🦀️.rs"]
    pub mod sun;
    #[path = "☑️options/🌀️vortex/🦀️.rs"]
    pub mod vortex;
}
#[path = "."]
pub mod tools {
    #[path = "🛠️tools/🪣️fill/🦀️.rs"]
    pub mod fill;
}
#[path = "."]
pub mod windows {
    #[path = "."]
    pub mod main {
        #[path = "🪟️windows/🧊️main/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod utilities {
            #[path = "🪟️windows/🧊️main/🪛️utilities/🖌️brush/🦀️.rs"]
            pub mod brush;
            #[path = "🪟️windows/🧊️main/🪛️utilities/🧊️volume-brush/🦀️.rs"]
            pub mod volume_brush;
            #[path = "🪟️windows/🧊️main/🪛️utilities/🚚️world-relocate/🦀️.rs"]
            pub mod world_relocate;
        }
    }
}
//#endregion 🌳️Modules
