//! 🧪️ `assert_viewer_never_mutates`/`assert_editor_and_viewer_share_dialect` (contract §2.5) —
//! local stand-ins per the pilot's `📓️w2-cad-report.md` "SDK gaps" #2: as of this packet's W0-F
//! handoff, the canonical `semio_framework_plugin::artifact_app_laws` versions exist
//! (`👁️✏️SurfaceUnitTests` region) and are used directly here rather than re-invented.
use semio_framework_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};
use semio_framework_test_fixtures::fixture_path;

macro_rules! surface_law {
    ($name:ident, $editor:ty, $viewer:ty) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            assert_viewer_never_mutates::<$viewer>(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
            assert_editor_and_viewer_share_dialect::<$editor, $viewer>().await;
        }
    };
}

surface_law!(din4108_surface_laws_hold, semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108_viewer::Din4108Viewer);
surface_law!(din16798_surface_laws_hold, semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798_viewer::Din16798Viewer);
surface_law!(din18599_surface_laws_hold, semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599_viewer::Din18599Viewer);
surface_law!(en1990_surface_laws_hold, semio_s_artifact_norm_en1990_editor::En1990PlayApp, semio_s_artifact_norm_en1990_viewer::En1990Viewer);
surface_law!(en1991_surface_laws_hold, semio_s_artifact_norm_en1991_editor::En1991PlayApp, semio_s_artifact_norm_en1991_viewer::En1991Viewer);
surface_law!(en1992_surface_laws_hold, semio_s_artifact_norm_en1992_editor::En1992PlayApp, semio_s_artifact_norm_en1992_viewer::En1992Viewer);
surface_law!(en1993_surface_laws_hold, semio_s_artifact_norm_en1993_editor::En1993PlayApp, semio_s_artifact_norm_en1993_viewer::En1993Viewer);
surface_law!(en1994_surface_laws_hold, semio_s_artifact_norm_en1994_editor::En1994PlayApp, semio_s_artifact_norm_en1994_viewer::En1994Viewer);
surface_law!(en1995_surface_laws_hold, semio_s_artifact_norm_en1995_editor::En1995PlayApp, semio_s_artifact_norm_en1995_viewer::En1995Viewer);
surface_law!(en1996_surface_laws_hold, semio_s_artifact_norm_en1996_editor::En1996PlayApp, semio_s_artifact_norm_en1996_viewer::En1996Viewer);
surface_law!(en1997_surface_laws_hold, semio_s_artifact_norm_en1997_editor::En1997PlayApp, semio_s_artifact_norm_en1997_viewer::En1997Viewer);
surface_law!(en1998_surface_laws_hold, semio_s_artifact_norm_en1998_editor::En1998PlayApp, semio_s_artifact_norm_en1998_viewer::En1998Viewer);
surface_law!(en1999_surface_laws_hold, semio_s_artifact_norm_en1999_editor::En1999PlayApp, semio_s_artifact_norm_en1999_viewer::En1999Viewer);
surface_law!(iso16757_surface_laws_hold, semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757_viewer::Iso16757Viewer);
surface_law!(vdi3805_surface_laws_hold, semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805_viewer::Vdi3805Viewer);

//#region 🎨️SetActiveExample
/// 🎨️ Asserts one family editor declares `setActiveExample` and loading its example mutates away from boot when the texts differ.
macro_rules! set_active_example_case {
    ($name:ident, $create:path, $command:ty, $set_active:path, $example_id:expr, $primary:expr, $snapshot:ty) => {
        #[test]
        fn $name() {
            let definition = $create();
            assert!(
                definition.actions.iter().any(|action| action.id == "setActiveExample"),
                "{} must declare setActiveExample on the editor action list",
                stringify!($name)
            );
            assert!(
                <$command>::TOOL_JOB_IDS.contains(&"setActiveExample"),
                "{} command catalog must include setActiveExample",
                stringify!($name)
            );
            let boot = <$snapshot>::default();
            let config = semio_framework_plugin::NoConfig::default();
            let emit = {
                use $set_active as set_active_example;
                set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: $example_id.to_string() },
                    &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()),
                    &semio_framework_plugin::ConfigView { snapshot: &config, window: None },
                )
                .expect("setActiveExample handle")
            };
            let unknown = {
                use $set_active as set_active_example;
                set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: "__unknown_example__".into() },
                    &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()),
                    &semio_framework_plugin::ConfigView { snapshot: &config, window: None },
                )
                .expect("unknown example is a no-op")
            };
            assert!(unknown.artifact_mutations.is_empty(), "{} unknown example must be a no-op", stringify!($name));
            assert!(
                !emit.artifact_mutations.is_empty() || emit.description.as_deref() == Some("setSnapshot"),
                "{} must load the example through setSnapshot (mutations when the example differs from boot)",
                stringify!($name)
            );
            let _ = $primary;
        }
    };
}

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
    en1992_set_active_example_loads_liquid_retaining_fem_anchor,
    semio_s_artifact_norm_en1992_editor::create_en1992_app,
    semio_s_artifact_norm_en1992_editor::En1992Command,
    semio_s_artifact_norm_en1992::editor::en1992::commands::set_active_example,
    semio_s_artifact_norm_en1992_examples::liquid_retaining_fem_anchor::ID,
    semio_s_artifact_norm_en1992_examples::liquid_retaining_fem_anchor::PRIMARY_TEXT,
    semio_s_artifact_norm_en1992_model::En1992Snapshot
);
set_active_example_case!(
    en1993_set_active_example_loads_high_strength_connection,
    semio_s_artifact_norm_en1993_editor::create_en1993_app,
    semio_s_artifact_norm_en1993_editor::En1993Command,
    semio_s_artifact_norm_en1993::editor::en1993::commands::set_active_example,
    semio_s_artifact_norm_en1993_examples::high_strength_connection::ID,
    semio_s_artifact_norm_en1993_examples::high_strength_connection::PRIMARY_TEXT,
    semio_s_artifact_norm_en1993_model::En1993Snapshot
);
set_active_example_case!(
    en1994_set_active_example_loads_composite_bridge_girder,
    semio_s_artifact_norm_en1994_editor::create_en1994_app,
    semio_s_artifact_norm_en1994_editor::En1994Command,
    semio_s_artifact_norm_en1994::editor::en1994::commands::set_active_example,
    semio_s_artifact_norm_en1994_examples::composite_bridge_girder::ID,
    semio_s_artifact_norm_en1994_examples::composite_bridge_girder::PRIMARY_TEXT,
    semio_s_artifact_norm_en1994_model::En1994Snapshot
);
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
set_active_example_case!(
    en1999_set_active_example_loads_aluminium_roof_purlin,
    semio_s_artifact_norm_en1999_editor::create_en1999_app,
    semio_s_artifact_norm_en1999_editor::En1999Command,
    semio_s_artifact_norm_en1999::editor::en1999::commands::set_active_example,
    semio_s_artifact_norm_en1999_examples::aluminium_roof_purlin::ID,
    semio_s_artifact_norm_en1999_examples::aluminium_roof_purlin::PRIMARY_TEXT,
    semio_s_artifact_norm_en1999_model::En1999Snapshot
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
//#endregion 🎨️SetActiveExample

//#region 🗂️RosterExamples
/// 🗂️ Asserts every example one family editor offers in its navbar picker loads through `setActiveExample`,
/// and that the picker roster is exactly the family's `📚️examples/*` example modules — an example on disk
/// the picker hides, or a picker entry `setActiveExample` cannot load, is the drift this law exists for
/// (en1992 offered five examples and loaded two).
macro_rules! roster_examples_case {
    ($name:ident, $editor:ty, $set_active:path, $snapshot:ty, $family_dir:literal) => {
        #[test]
        fn $name() {
            use semio_framework_plugin::ArtifactEditor;
            use $set_active as set_active_example;
            let roster = <$editor as ArtifactEditor>::examples();
            let boot = <$snapshot>::default();
            let config = semio_framework_plugin::NoConfig::default();
            for example in &roster {
                let emit = set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: example.id().to_string() },
                    &semio_framework_plugin::ArtifactView::new(&boot, &semio_framework_plugin::HistoryView::empty()),
                    &semio_framework_plugin::ConfigView { snapshot: &config, window: None },
                )
                .unwrap_or_else(|fault| panic!("{}: example '{}' must load: {fault:?}", stringify!($name), example.id()));
                assert!(
                    !emit.artifact_mutations.is_empty() || emit.description.as_deref() == Some("setSnapshot"),
                    "{}: example '{}' loaded nothing",
                    stringify!($name),
                    example.id()
                );
            }
            let mut rostered: Vec<String> = roster.iter().map(|example| example.id().to_string()).collect();
            rostered.sort();
            let mut on_disk = example_ids_on_disk($family_dir);
            on_disk.sort();
            assert_eq!(rostered, on_disk, "{}: the picker roster must be exactly the family's 📚️examples/* modules", stringify!($name));
        }
    };
}

/// 📚️ The `pub const ID` of every `📚️examples/*/🦀️.rs` example module of one norm family.
fn example_ids_on_disk(family_dir: &str) -> Vec<String> {
    let root = fixture_path!("../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts").join(family_dir).join("🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples");
    std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("{}: {error}", root.display()))
        .filter_map(|entry| std::fs::read_to_string(entry.ok()?.path().join("🦀️.rs")).ok())
        .filter_map(|source| source.lines().find_map(|line| line.trim().strip_prefix("pub const ID: &str = \"")?.strip_suffix("\";").map(str::to_string)))
        .collect()
}

roster_examples_case!(din4108_roster_examples_all_load, semio_s_artifact_norm_din4108_editor::Din4108PlayApp, semio_s_artifact_norm_din4108::editor::din4108::commands::set_active_example, semio_s_artifact_norm_din4108_model::Din4108Snapshot, "🧱️din4108");
roster_examples_case!(din16798_roster_examples_all_load, semio_s_artifact_norm_din16798_editor::Din16798PlayApp, semio_s_artifact_norm_din16798::editor::din16798::commands::set_active_example, semio_s_artifact_norm_din16798_model::Din16798Snapshot, "🌬️din16798");
roster_examples_case!(din18599_roster_examples_all_load, semio_s_artifact_norm_din18599_editor::Din18599PlayApp, semio_s_artifact_norm_din18599::editor::din18599::commands::set_active_example, semio_s_artifact_norm_din18599_model::Din18599Snapshot, "⚡️din18599");
roster_examples_case!(en1990_roster_examples_all_load, semio_s_artifact_norm_en1990_editor::En1990PlayApp, semio_s_artifact_norm_en1990::editor::en1990::commands::set_active_example, semio_s_artifact_norm_en1990_model::En1990Snapshot, "⚖️en1990");
roster_examples_case!(en1991_roster_examples_all_load, semio_s_artifact_norm_en1991_editor::En1991PlayApp, semio_s_artifact_norm_en1991::editor::en1991::commands::set_active_example, semio_s_artifact_norm_en1991_model::En1991Snapshot, "🏋️en1991");
roster_examples_case!(en1992_roster_examples_all_load, semio_s_artifact_norm_en1992_editor::En1992PlayApp, semio_s_artifact_norm_en1992::editor::en1992::commands::set_active_example, semio_s_artifact_norm_en1992_model::En1992Snapshot, "🏛️en1992");
roster_examples_case!(en1993_roster_examples_all_load, semio_s_artifact_norm_en1993_editor::En1993PlayApp, semio_s_artifact_norm_en1993::editor::en1993::commands::set_active_example, semio_s_artifact_norm_en1993_model::En1993Snapshot, "🔩️en1993");
roster_examples_case!(en1994_roster_examples_all_load, semio_s_artifact_norm_en1994_editor::En1994PlayApp, semio_s_artifact_norm_en1994::editor::en1994::commands::set_active_example, semio_s_artifact_norm_en1994_model::En1994Snapshot, "🧩️en1994");
roster_examples_case!(en1995_roster_examples_all_load, semio_s_artifact_norm_en1995_editor::En1995PlayApp, semio_s_artifact_norm_en1995::editor::en1995::commands::set_active_example, semio_s_artifact_norm_en1995_model::En1995Snapshot, "🪵️en1995");
roster_examples_case!(en1996_roster_examples_all_load, semio_s_artifact_norm_en1996_editor::En1996PlayApp, semio_s_artifact_norm_en1996::editor::en1996::commands::set_active_example, semio_s_artifact_norm_en1996_model::En1996Snapshot, "🪨️en1996");
roster_examples_case!(en1997_roster_examples_all_load, semio_s_artifact_norm_en1997_editor::En1997PlayApp, semio_s_artifact_norm_en1997::editor::en1997::commands::set_active_example, semio_s_artifact_norm_en1997_model::En1997Snapshot, "🌍️en1997");
roster_examples_case!(en1998_roster_examples_all_load, semio_s_artifact_norm_en1998_editor::En1998PlayApp, semio_s_artifact_norm_en1998::editor::en1998::commands::set_active_example, semio_s_artifact_norm_en1998_model::En1998Snapshot, "🫨️en1998");
roster_examples_case!(en1999_roster_examples_all_load, semio_s_artifact_norm_en1999_editor::En1999PlayApp, semio_s_artifact_norm_en1999::editor::en1999::commands::set_active_example, semio_s_artifact_norm_en1999_model::En1999Snapshot, "🪶️en1999");
roster_examples_case!(iso16757_roster_examples_all_load, semio_s_artifact_norm_iso16757_editor::Iso16757PlayApp, semio_s_artifact_norm_iso16757::editor::iso16757::commands::set_active_example, semio_s_artifact_norm_iso16757_model::Iso16757Snapshot, "📇️iso16757");
roster_examples_case!(vdi3805_roster_examples_all_load, semio_s_artifact_norm_vdi3805_editor::Vdi3805PlayApp, semio_s_artifact_norm_vdi3805::editor::vdi3805::commands::set_active_example, semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot, "🏭️vdi3805");
//#endregion 🗂️RosterExamples
