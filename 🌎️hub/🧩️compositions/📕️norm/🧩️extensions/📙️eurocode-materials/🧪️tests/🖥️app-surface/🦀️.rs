//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.

use semio_hub_norm_eurocode_materials::plugin;

const VARIANTS: &[&str] = &["en1992", "en1993", "en1994", "en1995", "en1996", "en1999"];

macro_rules! assert_family_pairs {
    () => {
        assert_norm_pair! {
            semio_s_artifact_norm_en1992 => (semio_s_artifact_norm_en1992_editor::En1992PlayApp, semio_s_artifact_norm_en1992_viewer::En1992Viewer),
            semio_s_artifact_norm_en1993 => (semio_s_artifact_norm_en1993_editor::En1993PlayApp, semio_s_artifact_norm_en1993_viewer::En1993Viewer),
            semio_s_artifact_norm_en1994 => (semio_s_artifact_norm_en1994_editor::En1994PlayApp, semio_s_artifact_norm_en1994_viewer::En1994Viewer),
            semio_s_artifact_norm_en1995 => (semio_s_artifact_norm_en1995_editor::En1995PlayApp, semio_s_artifact_norm_en1995_viewer::En1995Viewer),
            semio_s_artifact_norm_en1996 => (semio_s_artifact_norm_en1996_editor::En1996PlayApp, semio_s_artifact_norm_en1996_viewer::En1996Viewer),
            semio_s_artifact_norm_en1999 => (semio_s_artifact_norm_en1999_editor::En1999PlayApp, semio_s_artifact_norm_en1999_viewer::En1999Viewer),
        }
    };
}

include!("../../../../🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs");
