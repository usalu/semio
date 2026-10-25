mod typed_command_full_operation_tests {
    use semio_framework_actor::typed_operation_page::{TypedOperationResultPage,TypedOperationResultToken,TYPED_OPERATION_RESULT_PAGE_BYTES};
    use semio_framework_plugin_decl_app::tool_contract::TYPED_OPERATION_FAULT_BYTES;
    use semio_framework_plugin_instance_app::typed_operation_document_is_fresh;
    use semio_framework_plugin_operation::TOOL_CANCELLATION_SLOTS;
    use semio_framework_plugin_job::reserved_jobs::{plugin_job_yield_once,QualifiedToolProof};
    use semio_framework_os_kernel::one_item_publication;
use semio_framework_os_kernel::one_item::{ArtifactStoreOneItemCheckpoint,ArtifactStoreOneItemPreparationStep};
use semio_framework_os_kernel::one_item;
use semio_framework_plugin_decl_emission::ChildEmitGenesis;
use semio_framework::ArgFormat::ArtifactRef;
use semio_framework_plugin_operation::ArtifactDocumentAuthority;
use semio_framework_plugin_operation::ToolOperationKey;
use semio_framework::ArtifactDialect;
use semio_framework_os_kernel::SchemaId;
use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
use semio_framework_os_kernel::LocalizedLabel;
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

    /// 🧵️ Source text of the mounted artifact instance family: the composer and use-case crates of the instance, the maintenance ladder, the cancellation owners, the retained command shell and the lifted full-operation job, each crate family in path order, read at run time from the manifest folder of the hosting crate.
    #[doc(hidden)]
pub fn instance_source() -> String {
        ["🧵️instance", "🪜️maintenance-ladder", "♻️cancellation", "🧵️retained-command", "👷️job"]
            .iter()
            .map(|folder| semio_framework_plugin_test_support::read_source_family_source(&fixture_path!("../../../..").join(folder)).expect("the instance family reads"))
            .collect::<Vec<_>>()
            .join("\n")
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
