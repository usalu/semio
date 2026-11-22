//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).

const ARTIFACTS_ROOT: &str = "../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts";

include!("../🧰️laws/🔬️surface/🦀️.rs");

surface_law!(din4108_surface_laws_hold, semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108_viewer::Din4108Viewer);
surface_law!(din16798_surface_laws_hold, semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798_viewer::Din16798Viewer);
surface_law!(iso16757_surface_laws_hold, semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer);
surface_law!(vdi3805_surface_laws_hold, semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer);

roster_examples_case!(din4108_roster_examples_all_load, semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108_model::Din4108Snapshot, "🧱️din4108");
roster_examples_case!(din16798_roster_examples_all_load, semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798_model::Din16798Snapshot, "🌬️din16798");
roster_examples_case!(iso16757_roster_examples_all_load, semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757_model::Iso16757Snapshot, "📇️iso16757");
roster_examples_case!(vdi3805_roster_examples_all_load, semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot, "🏭️vdi3805");
