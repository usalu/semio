//! ✏️ The Din4108 editor app of the `norm` plugin: the one instantiation of the instance runtime for [`semio_s_artifact_norm_din4108_editor::Din4108PlayApp`], compiled here once so the plugin crate only delegates to it.

#![allow(async_fn_in_trait)]

use semio_plugin::__semio_dispatch_PluginApp;
use semio_plugin::plugin_app_close_prelude::*;
use semio_plugin::PluginApp;

semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app of the Din4108 editor.
    pub enum Din4108EditorApp: PluginApp {
        App(VcsArtifactApp<EditorApp<semio_s_artifact_norm_din4108_editor::Din4108PlayApp>>),
    }
}
