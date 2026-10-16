//! 🖍 `shell-view-paint` staging of `storage-prefs-and-i18n`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region ShellChrome
//#region 🗣️ChromeI18n
pub(crate) fn scene_chrome_labels(is_de: bool) -> SceneChromeLabels {
    SceneChromeLabels {
        german: is_de,
        virtual_file_system: VirtualFileSystemChromeLabels {
            name: shell_chrome_string("common.name", is_de),
            no_file_system_nodes: shell_chrome_string("common.noFileSystemNodes", is_de),
            expand: shell_chrome_string("common.expand", is_de),
            collapse: shell_chrome_string("common.collapse", is_de),
        },
        block_list: BlockListChromeLabels { steps: shell_chrome_string("blockList.steps", is_de), add_step: shell_chrome_string("blockList.addStep", is_de), delete: shell_chrome_string("common.delete", is_de) },
        icon_render: crate::scenes::IconRenderChromeLabels { empty_scene: shell_chrome_string("host.emptyScene", is_de), rendering: shell_chrome_string("host.rendering", is_de), failed: shell_chrome_string("host.iconRenderFailed", is_de) },
    }
}
//#endregion 🗣️ChromeI18n
//#endregion ShellChrome
