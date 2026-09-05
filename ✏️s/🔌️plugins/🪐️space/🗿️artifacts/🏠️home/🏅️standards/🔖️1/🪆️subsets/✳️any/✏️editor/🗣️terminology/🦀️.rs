//! 🗣️ S Home launcher app — locale × terminology label set (constitutional: ui/Terminology).
//!
//! 🔁️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the table-column/origin/
//! empty-message strings the main window renders moved to the plugin-root `crate::HomeTableLabels`
//! (shared with the read-only viewer, which can never import through `::editor::`). This file keeps
//! only editor-exclusive strings: the window title and the actions-summary words (the viewer never
//! renders row actions, contract §2.2).

use semio_framework_plugin::app_labels;

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Home launcher; one field per label makes every locale×terminology combination compile-checked.
    pub struct SHomeLabels {
        window_main: native_en "Studios", native_de "Studios", reuse_en "Studios", reuse_de "Studios";
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: the
        // `#s-home-create-space` toolbar button's own label (contract §C0 id grammar).
        action_create: native_en "Create Space", native_de "Space erstellen", reuse_en "Create Space", reuse_de "Space erstellen";
        action_open: native_en "open", native_de "öffnen", reuse_en "open", reuse_de "öffnen";
        action_rename: native_en "rename", native_de "umbenennen", reuse_en "rename", reuse_de "umbenennen";
        action_share: native_en "share", native_de "teilen", reuse_en "share", reuse_de "teilen";
        action_delete: native_en "delete", native_de "löschen", reuse_en "delete", reuse_de "löschen";
        // 🏛️ Author-only administration pane (members, roles, invites) — see `row_actions`.
        action_manage: native_en "manage", native_de "verwalten", reuse_en "manage", reuse_de "verwalten";
    }
}
//#endregion 🔖️Terminology
