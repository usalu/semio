use semio_framework_test_fixtures::fixture_path;
use semio_plugin::artifact_app_laws::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

macro_rules! surface_law {
    ($name:ident, $editor:ty, $viewer:ty) => {
        #[semio_framework_async_macros::async_test]
        async fn $name() {
            assert_viewer_never_mutates::<$viewer>(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
            assert_editor_and_viewer_share_dialect::<$editor, $viewer>().await;
        }
    };
}

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
            let config = semio_plugin::NoConfig::default();
            let emit = {
                use $set_active as set_active_example;
                set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: $example_id.to_string() },
                    &semio_plugin::ArtifactView::new(&boot, &semio_plugin::HistoryView::empty()),
                    &semio_plugin::ConfigView { snapshot: &config, window: None },
                )
                .expect("setActiveExample handle")
            };
            let unknown = {
                use $set_active as set_active_example;
                set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: "__unknown_example__".into() },
                    &semio_plugin::ArtifactView::new(&boot, &semio_plugin::HistoryView::empty()),
                    &semio_plugin::ConfigView { snapshot: &config, window: None },
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


//#region 🗂️RosterExamples
/// 🗂️ Asserts every example one family editor offers in its navbar picker loads through `setActiveExample`,
/// and that the picker roster is exactly the family's `📚️examples/*` example modules — an example on disk
/// the picker hides, or a picker entry `setActiveExample` cannot load, is the drift this law exists for
/// (en1992 offered five examples and loaded two).
macro_rules! roster_examples_case {
    ($name:ident, $editor:ty, $set_active:path, $snapshot:ty, $family_dir:literal) => {
        #[test]
        fn $name() {
            use semio_plugin::ArtifactEditor;
            use $set_active as set_active_example;
            let roster = <$editor as ArtifactEditor>::examples();
            let boot = <$snapshot>::default();
            let config = semio_plugin::NoConfig::default();
            for example in &roster {
                let emit = set_active_example::handle(
                    &set_active_example::SetActiveExample { example_id: example.id().to_string() },
                    &semio_plugin::ArtifactView::new(&boot, &semio_plugin::HistoryView::empty()),
                    &semio_plugin::ConfigView { snapshot: &config, window: None },
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
    let root = fixture_path!(ARTIFACTS_ROOT).join(family_dir).join("🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples");
    std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("{}: {error}", root.display()))
        .filter_map(|entry| std::fs::read_to_string(entry.ok()?.path().join("🦀️.rs")).ok())
        .filter_map(|source| source.lines().find_map(|line| line.trim().strip_prefix("pub const ID: &str = \"")?.strip_suffix("\";").map(str::to_string)))
        .collect()
}


//#endregion 🗂️RosterExamples
