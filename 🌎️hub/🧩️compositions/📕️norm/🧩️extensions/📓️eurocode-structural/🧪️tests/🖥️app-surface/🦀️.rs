//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.

use semio_hub_norm_eurocode_structural::plugin;

const VARIANTS: &[&str] = &["en1992", "en1993", "en1994"];

macro_rules! assert_family_pairs {
    () => {
        assert_norm_pair! {
            semio_s_artifact_norm_en1992 => (semio_s_artifact_norm_en1992_editor::En1992PlayApp, semio_s_artifact_norm_en1992_viewer::En1992Viewer),
            semio_s_artifact_norm_en1993 => (semio_s_artifact_norm_en1993_editor::En1993PlayApp, semio_s_artifact_norm_en1993_viewer::En1993Viewer),
            semio_s_artifact_norm_en1994 => (semio_s_artifact_norm_en1994_editor::En1994PlayApp, semio_s_artifact_norm_en1994_viewer::En1994Viewer),
        }
    };
}

include!("../../../../🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs");
