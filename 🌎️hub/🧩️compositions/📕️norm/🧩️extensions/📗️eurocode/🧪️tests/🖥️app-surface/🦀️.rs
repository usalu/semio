//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.

use semio_hub_norm_eurocode::plugin;

const VARIANTS: &[&str] = &["en1990", "en1991", "en1997", "en1998"];

macro_rules! assert_family_pairs {
    () => {
        assert_norm_pair! {
            semio_s_artifact_norm_en1990 => (semio_s_artifact_norm_en1990_editor::En1990PlayApp, semio_s_artifact_norm_en1990_viewer::En1990Viewer),
            semio_s_artifact_norm_en1991 => (semio_s_artifact_norm_en1991_editor::En1991PlayApp, semio_s_artifact_norm_en1991_viewer::En1991Viewer),
            semio_s_artifact_norm_en1997 => (semio_s_artifact_norm_en1997_editor::En1997PlayApp, semio_s_artifact_norm_en1997_viewer::En1997Viewer),
            semio_s_artifact_norm_en1998 => (semio_s_artifact_norm_en1998_editor::En1998PlayApp, semio_s_artifact_norm_en1998_viewer::En1998Viewer),
        }
    };
}

include!("../../../../🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs");
