use semio_framework_test_fixtures::fixture_path;
use semio_plugin::artifact_app_laws::assert_editor_and_viewer_share_dialect;

macro_rules! surface_law {
    ($name:ident, $editor:ty, $viewer:ty) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            assert_editor_and_viewer_share_dialect::<$editor, $viewer>().await;
        }
    };
}

//#region 🗂️RosterExamples
/// 🗂️ Asserts every example one family editor offers in its navbar picker loads through `setActiveExample`,
/// and that the picker roster is exactly the family's `📚️examples/*` example modules — an example on disk
/// the picker hides, or a picker entry `setActiveExample` cannot load, is the drift this law exists for
/// (en1992 offered five examples and loaded two).
macro_rules! roster_examples_case {
    ($name:ident, $editor:ty, $snapshot:ty, $family_dir:literal) => {
        #[test]
        fn $name() {
            use semio_plugin::ArtifactEditor;
            let roster = <$editor as ArtifactEditor>::examples();
            assert!(semio_s_plugin_norm_app_surface::roster_example_snapshot::<$snapshot>(roster.clone(), "").expect("the empty id is the empty document").is_some());
            assert!(semio_s_plugin_norm_app_surface::roster_example_snapshot::<$snapshot>(roster.clone(), "__unknown_example__").expect("an unknown id is a no-op").is_none());
            for example in &roster {
                let loaded = semio_s_plugin_norm_app_surface::roster_example_snapshot::<$snapshot>(roster.clone(), example.id())
                    .unwrap_or_else(|fault| panic!("{}: example '{}' must load: {fault:?}", stringify!($name), example.id()));
                assert!(loaded.is_some(), "{}: example '{}' loaded nothing", stringify!($name), example.id());
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
    let root = fixture_path!(ARTIFACTS_ROOT).join(family_dir).join("🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples");
    std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("{}: {error}", root.display()))
        .filter_map(|entry| std::fs::read_to_string(entry.ok()?.path().join("🦀️.rs")).ok())
        .filter_map(|source| source.lines().find_map(|line| line.trim().strip_prefix("pub const ID: &str = \"")?.strip_suffix("\";").map(str::to_string)))
        .collect()
}


//#endregion 🗂️RosterExamples
