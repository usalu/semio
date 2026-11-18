//! 🧪️ Surface laws, `setActiveExample` and example-roster laws for the families this plugin owns (contract §2.5).

const ARTIFACTS_ROOT: &str = "../../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts";

include!("../../../../🧪️tests/🧰️laws/🔬️surface/🦀️.rs");

surface_law!(en1990_surface_laws_hold, semio_s_artifact_norm_en1990_editor::En1990PlayApp, semio_s_artifact_norm_en1990_viewer::En1990Viewer);
surface_law!(en1991_surface_laws_hold, semio_s_artifact_norm_en1991_editor::En1991PlayApp, semio_s_artifact_norm_en1991_viewer::En1991Viewer);
surface_law!(en1997_surface_laws_hold, semio_s_artifact_norm_en1997_editor::En1997PlayApp, semio_s_artifact_norm_en1997_viewer::En1997Viewer);
surface_law!(en1998_surface_laws_hold, semio_s_artifact_norm_en1998_editor::En1998PlayApp, semio_s_artifact_norm_en1998_viewer::En1998Viewer);

set_active_example_case!(
    en1990_set_active_example_loads_high_consequence_office,
    semio_s_artifact_norm_en1990_editor::create_en1990_app,
    semio_s_artifact_norm_en1990_editor::En1990Command,
    semio_s_artifact_norm_en1990::editor::en1990::commands::set_active_example,
    semio_s_artifact_norm_en1990_examples::high_consequence_office::ID,
    semio_s_artifact_norm_en1990_examples::high_consequence_office::PRIMARY_TEXT,
    semio_s_artifact_norm_en1990_model::En1990Snapshot
);
set_active_example_case!(
    en1991_set_active_example_loads_multi_fail_noncompliant,
    semio_s_artifact_norm_en1991_editor::create_en1991_app,
    semio_s_artifact_norm_en1991_editor::En1991Command,
    semio_s_artifact_norm_en1991::editor::en1991::commands::set_active_example,
    semio_s_artifact_norm_en1991_examples::multi_fail_noncompliant::ID,
    semio_s_artifact_norm_en1991_examples::multi_fail_noncompliant::PRIMARY_TEXT,
    semio_s_artifact_norm_en1991_model::En1991Snapshot
);
set_active_example_case!(
    en1997_set_active_example_loads_demo,
    semio_s_artifact_norm_en1997_editor::create_en1997_app,
    semio_s_artifact_norm_en1997_editor::En1997Command,
    semio_s_artifact_norm_en1997::editor::en1997::commands::set_active_example,
    semio_s_artifact_norm_en1997_examples::demo::ID,
    semio_s_artifact_norm_en1997_examples::demo::PRIMARY_TEXT,
    semio_s_artifact_norm_en1997_model::En1997Snapshot
);
set_active_example_case!(
    en1998_set_active_example_loads_seismic_rc_frame,
    semio_s_artifact_norm_en1998_editor::create_en1998_app,
    semio_s_artifact_norm_en1998_editor::En1998Command,
    semio_s_artifact_norm_en1998::editor::en1998::commands::set_active_example,
    semio_s_artifact_norm_en1998_examples::seismic_rc_frame::ID,
    semio_s_artifact_norm_en1998_examples::seismic_rc_frame::PRIMARY_TEXT,
    semio_s_artifact_norm_en1998_model::En1998Snapshot
);

roster_examples_case!(en1990_roster_examples_all_load, semio_s_artifact_norm_en1990_editor::En1990PlayApp, semio_s_artifact_norm_en1990::editor::en1990::commands::set_active_example, semio_s_artifact_norm_en1990_model::En1990Snapshot, "⚖️en1990");
roster_examples_case!(en1991_roster_examples_all_load, semio_s_artifact_norm_en1991_editor::En1991PlayApp, semio_s_artifact_norm_en1991::editor::en1991::commands::set_active_example, semio_s_artifact_norm_en1991_model::En1991Snapshot, "🏋️en1991");
roster_examples_case!(en1997_roster_examples_all_load, semio_s_artifact_norm_en1997_editor::En1997PlayApp, semio_s_artifact_norm_en1997::editor::en1997::commands::set_active_example, semio_s_artifact_norm_en1997_model::En1997Snapshot, "🌍️en1997");
roster_examples_case!(en1998_roster_examples_all_load, semio_s_artifact_norm_en1998_editor::En1998PlayApp, semio_s_artifact_norm_en1998::editor::en1998::commands::set_active_example, semio_s_artifact_norm_en1998_model::En1998Snapshot, "🫨️en1998");
