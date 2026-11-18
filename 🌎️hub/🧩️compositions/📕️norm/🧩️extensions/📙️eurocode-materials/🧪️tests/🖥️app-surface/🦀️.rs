//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.

use semio_hub_norm_eurocode_materials::plugin;

const VARIANTS: &[&str] = &["en1995", "en1996", "en1999"];

macro_rules! assert_family_pairs {
    () => {
        assert_norm_pair! {
            semio_s_artifact_norm_en1995 => (semio_s_artifact_norm_en1995_editor::En1995PlayApp, semio_s_artifact_norm_en1995_viewer::En1995Viewer),
            semio_s_artifact_norm_en1996 => (semio_s_artifact_norm_en1996_editor::En1996PlayApp, semio_s_artifact_norm_en1996_viewer::En1996Viewer),
            semio_s_artifact_norm_en1999 => (semio_s_artifact_norm_en1999_editor::En1999PlayApp, semio_s_artifact_norm_en1999_viewer::En1999Viewer),
        }
    };
}

include!("../../../../🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs");
