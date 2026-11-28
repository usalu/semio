//! ✏️ The Iso16757 editor app of the `norm` plugin: the one instantiation of the instance runtime for [`semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp`], compiled here once so the plugin crate only delegates to it.

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::PluginApp;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app of the Iso16757 editor.
    pub enum Iso16757EditorApp: PluginApp {
        App(VcsArtifactApp<EditorApp<semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp>>),
    }
}
