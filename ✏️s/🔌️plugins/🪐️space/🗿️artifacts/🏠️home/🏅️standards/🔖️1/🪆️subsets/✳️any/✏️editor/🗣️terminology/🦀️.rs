//! 🗣️ S Home launcher app — locale × terminology label set (constitutional: ui/Terminology).
//!
//! 🔁️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS: the table-column/origin/
//! empty-message strings the main window renders moved to the plugin-root `semio_s_space_core::HomeTableLabels`
//! (shared with the read-only viewer, which can never import through `::editor::`). This file keeps
//! only editor-exclusive strings: the window title and the actions-summary words (the viewer never
//! renders row actions, contract §2.2).

use semio_framework_ui_locale::app_labels;

//#region 🔖️Terminology
app_labels! {
    /// 🗣️ Complete UI label set for the Home launcher; one field per label makes every locale×terminology combination compile-checked.
    pub struct SHomeLabels {
        window_main: native_en "Studios", native_de "Studios", reuse_en "Studios", reuse_de "Studios";
        // 🐙️ Ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS lane 4-F: the
        // `#s-home-create-space` toolbar button's own label (contract §C0 id grammar).
        action_create: native_en "Create Space", native_de "Space erstellen", reuse_en "Create Space", reuse_de "Space erstellen";
        action_open: native_en "Open", native_de "Öffnen", reuse_en "Open", reuse_de "Öffnen";
        action_rename: native_en "Rename", native_de "Umbenennen", reuse_en "Rename", reuse_de "Umbenennen";
        action_share: native_en "Share", native_de "Teilen", reuse_en "Share", reuse_de "Teilen";
        action_delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
        // 🏛️ Author-only administration pane (members, roles, invites) — see `row_actions`.
        action_manage: native_en "Manage", native_de "Verwalten", reuse_en "Manage", reuse_de "Verwalten";
        action_promote: native_en "Promote to hub", native_de "Zum Hub hochstufen", reuse_en "Promote to hub", reuse_de "Zum Hub hochstufen";
        action_persist: native_en "Persist locally", native_de "Lokal speichern", reuse_en "Persist locally", reuse_de "Lokal speichern";
        action_import: native_en "Import Studio", native_de "Studio importieren", reuse_en "Import Studio", reuse_de "Studio importieren";
        action_remove: native_en "Remove from Home", native_de "Aus Home entfernen", reuse_en "Remove from Home", reuse_de "Aus Home entfernen";
        ephemeral_share_blocked: native_en "This studio is ephemeral and local-only. Share and collaboration require promoting it to a hub space or persisting it locally first.", native_de "Dieses Studio ist flüchtig und nur lokal. Teilen und Zusammenarbeit erfordern zuerst die Hochstufung zum Hub oder lokales Speichern.", reuse_en "This studio is ephemeral and local-only. Share and collaboration require promoting it to a hub space or persisting it locally first.", reuse_de "Dieses Studio ist flüchtig und nur lokal. Teilen und Zusammenarbeit erfordern zuerst die Hochstufung zum Hub oder lokales Speichern.";
    }
}
//#endregion 🔖️Terminology
