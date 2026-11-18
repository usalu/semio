//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.

use semio_hub_norm::plugin;

const VARIANTS: &[&str] = &["din4108", "din16798", "iso16757", "vdi3805"];

macro_rules! assert_family_pairs {
    () => {
        assert_norm_pair! {
            semio_s_artifact_norm_din4108 => (semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108_viewer::Din4108Viewer),
            semio_s_artifact_norm_din16798 => (semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798_viewer::Din16798Viewer),
            semio_s_artifact_norm_iso16757 => (semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer),
            semio_s_artifact_norm_vdi3805 => (semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer),
        }
    };
}

include!("../🧰️shared/🦀️.rs");
