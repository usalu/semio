//! 🧪️ Every public norm editor/viewer wrapper of this plugin renders its declared language-neutral surface inventory.

use semio_hub_norm_energy::plugin;

const VARIANTS: &[&str] = &["din18599"];

macro_rules! assert_family_pairs {
    () => {
        assert_norm_pair! {
            semio_s_artifact_norm_din18599 => (semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599_viewer::Din18599Viewer),
        }
    };
}

include!("../../../../🖥️app-surface/🧪️tests/🧰️shared/🦀️.rs");
