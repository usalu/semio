mod typed_command_full_operation_tests {
    use semio_framework_test_fixtures::fixture_path;

    use super::*;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
    use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::change_publication_presence::ChangePublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation;

    /// 📖️ One module tree of the plugin SDK, read at run time from the manifest folder of the hosting crate instead of embedded: the root file first, then every file it mounts in declaration order.
    fn module_source(relative: &str) -> String {
        semio_framework_plugin_test_support::read_module_tree_source(&fixture_path!("../../../..").join(relative)).expect("the module tree reads")
    }

    /// 🧵️ Source text of the mounted artifact instance: its module tree, the parent file followed by every impl band in declaration order.
    #[doc(hidden)]
pub fn instance_source() -> String {
        module_source("🧵️instance/🦀️.rs")
    }

    #[doc(hidden)]
pub const FIXTURE: &str = include_str!("../../🧵️retained-command/🔄️full-operation/🧪️tests/🧫️fixtures/🔣️.json");

    include!("🔎️census-oracles-and-fixture-machine/🦀️.rs");
    include!("🔐️two-turn-presence-and-root-retirement/🦀️.rs");
    include!("🚫️cancellation-publication-boundaries/🦀️.rs");
    include!("📌️document-cancellation-and-latest-wins-keys/🦀️.rs");
    include!("⚖️slot-and-publication-fairness/🦀️.rs");
    include!("🌐️neutral-hostile-rows/🦀️.rs");
    include!("👪️child-genesis-wire-and-group-owner/🦀️.rs");
    include!("🧾️source-rejection-anchors-and-host-lane/🦀️.rs");
}
