//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).

const ARTIFACTS_ROOT: &str = "../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts";

include!("../../../../🧪️tests/🧰️laws/🔬️surface/🦀️.rs");

surface_law!(din18599_surface_laws_hold, semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599_viewer::Din18599Viewer);

set_active_example_case!(
    din18599_set_active_example_loads_demo,
    semio_s_artifact_norm_din18599_editor::create_din18599_app,
    semio_s_artifact_norm_din18599_editor::Din18599Command,
    semio_s_artifact_norm_din18599::editor::din18599::commands::set_active_example,
    semio_s_artifact_norm_din18599_examples::demo::ID,
    semio_s_artifact_norm_din18599_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din18599_model::Din18599Snapshot
);

roster_examples_case!(din18599_roster_examples_all_load, semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599::editor::din18599::commands::set_active_example, semio_s_artifact_norm_din18599_model::Din18599Snapshot, "⚡️din18599");
