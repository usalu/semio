//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).

const ARTIFACTS_ROOT: &str = "../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts";

include!("../../../../🧪️tests/🧰️laws/🔬️surface/🦀️.rs");

surface_law!(en1995_surface_laws_hold, semio_s_artifact_norm_en1995_editor::En1995PlayApp, semio_s_artifact_norm_en1995_viewer::En1995Viewer);
surface_law!(en1996_surface_laws_hold, semio_s_artifact_norm_en1996_editor::En1996PlayApp, semio_s_artifact_norm_en1996_viewer::En1996Viewer);
surface_law!(en1999_surface_laws_hold, semio_s_artifact_norm_en1999_editor::En1999PlayApp, semio_s_artifact_norm_en1999_viewer::En1999Viewer);

set_active_example_case!(
    en1995_set_active_example_loads_glulam_footbridge,
    semio_s_artifact_norm_en1995_editor::create_en1995_app,
    semio_s_artifact_norm_en1995_editor::En1995Command,
    semio_s_artifact_norm_en1995::editor::en1995::commands::set_active_example,
    semio_s_artifact_norm_en1995_examples::glulam_footbridge::ID,
    semio_s_artifact_norm_en1995_examples::glulam_footbridge::PRIMARY_TEXT,
    semio_s_artifact_norm_en1995_model::En1995Snapshot
);
set_active_example_case!(
    en1996_set_active_example_loads_loadbearing_wall,
    semio_s_artifact_norm_en1996_editor::create_en1996_app,
    semio_s_artifact_norm_en1996_editor::En1996Command,
    semio_s_artifact_norm_en1996::editor::en1996::commands::set_active_example,
    semio_s_artifact_norm_en1996_examples::loadbearing_wall::ID,
    semio_s_artifact_norm_en1996_examples::loadbearing_wall::PRIMARY_TEXT,
    semio_s_artifact_norm_en1996_model::En1996Snapshot
);
set_active_example_case!(
    en1999_set_active_example_loads_aluminium_roof_purlin,
    semio_s_artifact_norm_en1999_editor::create_en1999_app,
    semio_s_artifact_norm_en1999_editor::En1999Command,
    semio_s_artifact_norm_en1999::editor::en1999::commands::set_active_example,
    semio_s_artifact_norm_en1999_examples::aluminium_roof_purlin::ID,
    semio_s_artifact_norm_en1999_examples::aluminium_roof_purlin::PRIMARY_TEXT,
    semio_s_artifact_norm_en1999_model::En1999Snapshot
);

roster_examples_case!(en1995_roster_examples_all_load, semio_s_artifact_norm_en1995_editor::En1995PlayApp, semio_s_artifact_norm_en1995::editor::en1995::commands::set_active_example, semio_s_artifact_norm_en1995_model::En1995Snapshot, "🪵️en1995");
roster_examples_case!(en1996_roster_examples_all_load, semio_s_artifact_norm_en1996_editor::En1996PlayApp, semio_s_artifact_norm_en1996::editor::en1996::commands::set_active_example, semio_s_artifact_norm_en1996_model::En1996Snapshot, "🪨️en1996");
roster_examples_case!(en1999_roster_examples_all_load, semio_s_artifact_norm_en1999_editor::En1999PlayApp, semio_s_artifact_norm_en1999::editor::en1999::commands::set_active_example, semio_s_artifact_norm_en1999_model::En1999Snapshot, "🪶️en1999");
