//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).

const ARTIFACTS_ROOT: &str = "../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts";

include!("../🧰️laws/🔬️surface/🦀️.rs");

surface_law!(din4108_surface_laws_hold, semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108_viewer::Din4108Viewer);
surface_law!(din16798_surface_laws_hold, semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798_viewer::Din16798Viewer);
surface_law!(din18599_surface_laws_hold, semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599_viewer::Din18599Viewer);
surface_law!(iso16757_surface_laws_hold, semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer);
surface_law!(vdi3805_surface_laws_hold, semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer);

set_active_example_case!(
    din4108_set_active_example_loads_demo,
    semio_s_artifact_norm_din4108_editor::create_din4108_app,
    semio_s_artifact_norm_din4108_editor::Din4108Command,
    semio_s_artifact_norm_din4108::editor::din4108::commands::set_active_example,
    semio_s_artifact_norm_din4108_examples::demo::ID,
    semio_s_artifact_norm_din4108_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din4108_model::Din4108Snapshot
);
set_active_example_case!(
    din16798_set_active_example_loads_demo,
    semio_s_artifact_norm_din16798_editor::create_din16798_app,
    semio_s_artifact_norm_din16798_editor::Din16798Command,
    semio_s_artifact_norm_din16798::editor::din16798::commands::set_active_example,
    semio_s_artifact_norm_din16798_examples::demo::ID,
    semio_s_artifact_norm_din16798_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din16798_model::Din16798Snapshot
);
set_active_example_case!(
    din18599_set_active_example_loads_demo,
    semio_s_artifact_norm_din18599_editor::create_din18599_app,
    semio_s_artifact_norm_din18599_editor::Din18599Command,
    semio_s_artifact_norm_din18599::editor::din18599::commands::set_active_example,
    semio_s_artifact_norm_din18599_examples::demo::ID,
    semio_s_artifact_norm_din18599_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_din18599_model::Din18599Snapshot
);
set_active_example_case!(
    iso16757_set_active_example_loads_demo,
    semio_s_artifact_norm_iso16757_editor::create_iso16757_app,
    semio_s_artifact_norm_iso16757_editor::Iso16757Command,
    semio_s_artifact_norm_iso16757::editor::iso16757::commands::set_active_example,
    semio_s_artifact_norm_iso16757_examples::demo::ID,
    semio_s_artifact_norm_iso16757_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_iso16757_model::Iso16757Snapshot
);
set_active_example_case!(
    vdi3805_set_active_example_loads_demo,
    semio_s_artifact_norm_vdi3805_editor::create_vdi3805_app,
    semio_s_artifact_norm_vdi3805_editor::Vdi3805Command,
    semio_s_artifact_norm_vdi3805::editor::vdi3805::commands::set_active_example,
    semio_s_artifact_norm_vdi3805_examples::demo::ID,
    semio_s_artifact_norm_vdi3805_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot
);

roster_examples_case!(din4108_roster_examples_all_load, semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108::editor::din4108::commands::set_active_example, semio_s_artifact_norm_din4108_model::Din4108Snapshot, "🧱️din4108");
roster_examples_case!(din16798_roster_examples_all_load, semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798::editor::din16798::commands::set_active_example, semio_s_artifact_norm_din16798_model::Din16798Snapshot, "🌬️din16798");
roster_examples_case!(din18599_roster_examples_all_load, semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599::editor::din18599::commands::set_active_example, semio_s_artifact_norm_din18599_model::Din18599Snapshot, "⚡️din18599");
roster_examples_case!(iso16757_roster_examples_all_load, semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757::editor::iso16757::commands::set_active_example, semio_s_artifact_norm_iso16757_model::Iso16757Snapshot, "📇️iso16757");
roster_examples_case!(vdi3805_roster_examples_all_load, semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805::editor::vdi3805::commands::set_active_example, semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot, "🏭️vdi3805");
