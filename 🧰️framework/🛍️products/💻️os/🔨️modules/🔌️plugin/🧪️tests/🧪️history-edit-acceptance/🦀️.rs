//! ⏪️ The cross-plugin history-edit acceptance law (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING design §16.3, gap G12):
//! for a representative editable leaf of any app, read from the app's own committed mutation fixtures (on their own documents,
//! else on the documents the app ships: its initial document and its examples), the real verbs run
//! `historyEditWithdraw` → `historyEditAccept` → `historyEditRestore` (a withdrawal taken back leaves no trace), then
//! `historyEditBegin` (the preview is the document as of the edited mutation: downstream is not applied) → `historyEditInput` (a
//! schema-valid change derived from the leaf's input descriptors) → `historyEditAccept` → the Report replay →
//! `historyEditFinalize` → `historyEditCommit` as an overwrite and, on a second instance, as a new alternative; each head must
//! equal a fresh fold of the edited log, and the edited mutation's history row must carry its leaf label in every locale. A leaf
//! whose descriptor declares `"editable": false` is withdraw-only (design §22.20) and never a case. One macro call, [`history_edit_acceptance_law!`](crate::history_edit_acceptance_law), wires it into a
//! plugin crate's test target; every failure names the plugin, the leaf and the fixture case.

extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as pack;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::{ActionMeta, Emit};
use semio_framework::kernel::{ActorId, MutationId};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_app_builder::App;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_app::ArtifactApp;
use store::os_store::store_schema::ArtifactCommand;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, ArtifactStoreInitializationAuthority, MemberFactory, Mutation, SpaceMember};
use std::collections::BTreeSet;
use semio_framework_plugin_command::CATALOGUE_EXAMPLE_ACTION_ID;
use semio_framework_value::{DslValue, ValueError};
use semio_framework_plugin_app_builder::ExampleSource;
use semio_framework::{Fault, ViewModel};
use std::future::Future;
use protocol::{LOCAL_ACTOR_ID, OpBinary, OpText};
use semio_framework_ui_locale::{Locale, LocalizedLabel, Terminology};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use serde_json::Value;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_ui_layout::layout::WindowMeasure;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::{artifact_app_laws, with_authoring_identity};
use store::os_store::document_materialize::create_document_envelope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate as history_edit_acceptance;
use semio_framework_plugin_time_travel as time_travel;
use semio_framework_ui_contract::*;

#[path = "🚧️acceptance-error-limits-and-verdicts/🦀️.rs"]
mod acceptance_error_limits_and_verdicts;
use acceptance_error_limits_and_verdicts::{
    AcceptanceError, AcceptanceSeedFault, AcceptanceVerdict, ACCEPTANCE_ACTOR,
    ACCEPTANCE_ALTERNATIVE, ACCEPTANCE_CASCADE_BLOCKERS, ACCEPTANCE_CASCADE_CASES,
    ACCEPTANCE_CASCADE_UNBUILT, ACCEPTANCE_CASES_PER_LEAF, ACCEPTANCE_CHANGES_PER_LEAF,
    ACCEPTANCE_CONTROL_KINDS, ACCEPTANCE_DOWNSTREAM_PER_LEAF, ACCEPTANCE_KIND_ATTEMPTS,
};
pub use acceptance_error_limits_and_verdicts::AcceptanceCase;

#[path = "📚️acceptance-fixtures-cases-and-change-buckets/🦀️.rs"]
mod acceptance_fixtures_cases_and_change_buckets;
use acceptance_fixtures_cases_and_change_buckets::{acceptance_json, acceptance_text_difference};
pub use acceptance_fixtures_cases_and_change_buckets::{
    acceptance_cases, acceptance_change_buckets, acceptance_changes, acceptance_example_cases,
};

#[path = "🎛️acceptance-policy-pump-and-drive/🦀️.rs"]
mod acceptance_policy_pump_and_drive;
use acceptance_policy_pump_and_drive::{
    acceptance_close, acceptance_drive, acceptance_head, acceptance_meta, acceptance_policy,
    acceptance_pump, acceptance_seeded, acceptance_verb,
};

#[path = "🔁️withdraw-restore-and-alternative-scenarios/🦀️.rs"]
mod withdraw_restore_and_alternative_scenarios;
use withdraw_restore_and_alternative_scenarios::{acceptance_first_pass, acceptance_scenario};
pub use withdraw_restore_and_alternative_scenarios::acceptance_withdraw_only_kinds;

#[path = "🔢️end-to-end-assertion-census-and-search/🦀️.rs"]
mod end_to_end_assertion_census_and_search;
pub use end_to_end_assertion_census_and_search::assert_history_edits_end_to_end;
use end_to_end_assertion_census_and_search::{acceptance_search, AcceptanceSearch};

#[path = "💾️reload-and-child-history-scenarios/🦀️.rs"]
mod reload_and_child_history_scenarios;
use reload_and_child_history_scenarios::{acceptance_reloaded, acceptance_reloaded_head};
pub use reload_and_child_history_scenarios::{
    assert_child_history_edits_end_to_end, assert_documents_reload_identically,
};

#[path = "📐️input-schema-resolution-and-conflict-classification/🦀️.rs"]
mod input_schema_resolution_and_conflict_classification;
pub use input_schema_resolution_and_conflict_classification::{
    assert_input_schemas_resolve, input_schema_resolution_failures,
};
use input_schema_resolution_and_conflict_classification::{
    acceptance_applied_ids, acceptance_blocks, acceptance_classified, acceptance_overwrite_folds,
    acceptance_resolve, acceptance_withdraw_accept,
};

#[path = "⚔️cascade-withdraw-edit-and-conflict-resolution/🦀️.rs"]
mod cascade_withdraw_edit_and_conflict_resolution;
pub use cascade_withdraw_edit_and_conflict_resolution::{
    assert_history_conflicts_resolve, block_on_acceptance,
};

use semio_framework::kernel::{HistoryTimeTravelReview, HistoryTimeTravelStage};
use semio_framework::{ActionArgDef, ArgSchema};
use semio_framework::history_edit::{HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};

/// 🔌️ Wires [`assert_history_edits_end_to_end`] into a plugin crate's test target for one editor: `plugin` names the failure,
/// `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$fixtures` the fixture root relative to the crate's
/// manifest directory (the artifact tree: `"../.."`).
#[macro_export]
macro_rules! history_edit_acceptance_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $fixtures:literal) => {
        /// ⏮️ LAW (design §16.3): a representative editable leaf of this editor is edited in history end to end.
        #[test]
        fn history_edits_end_to_end() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[history-edit-acceptance] {summary}");
        }

        /// ⛓️ LAW (design §16.3): every document leaf payload schema of this editor resolves through the runtime resolver.
        #[test]
        fn history_edit_inputs_resolve() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let (leaves, withdraw_only) =
                $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_input_schemas_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>($plugin, $manifest, &fixtures));
            ::std::println!("[history-edit-inputs] {}: {leaves} leaf payload schema(s) resolve, {withdraw_only} withdraw-only leaf(s) not judged", $plugin);
        }

        /// ⚔️ LAW (design §23.2): a conflict a history edit of this editor causes downstream is classified, named and resolved end to end.
        #[test]
        fn conflict_history_edits_end_to_end() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_history_conflicts_resolve::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[history-edit-cascade] {summary}");
        }
    };
}

/// 🛠️ Wires [`assert_documents_reload_identically`] into a composed plugin's test target for one editor (design §20.15), the same
/// arguments as [`history_edit_acceptance_law!`](crate::history_edit_acceptance_law): `plugin` names the failure, `$editor` is the
/// `ArtifactEditor` (its `Members` and `examples()` are read from it), `$manifest` its `fn() -> App`, `$fixtures` the fixture root
/// relative to the crate's manifest directory.
#[macro_export]
macro_rules! composed_reload_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $fixtures:literal) => {
        /// 💾️ LAW (design §20.15): every document this editor ships, and a history edit on it, survives save → fresh load.
        #[test]
        fn documents_reload_identically() {
            let fixtures = ::std::path::Path::new(::core::env!("CARGO_MANIFEST_DIR")).join($fixtures);
            let examples = <$editor as $crate::ArtifactEditor>::examples();
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_documents_reload_identically::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &fixtures, &examples,
            ));
            ::std::println!("[documents-reload] {}: {summary}", $plugin);
        }
    };
}

/// 🧩️ Wires [`assert_child_history_edits_end_to_end`] into a composed plugin's test target (design §12, §20.15): `plugin` names the
/// failure, `$editor` is the `ArtifactEditor`, `$manifest` its `fn() -> App`, `$seed` the `[(action id, JSON object args)]` gestures
/// that land a child-lane edit in an owned member of the editor's initial document.
#[macro_export]
macro_rules! composed_child_history_law {
    ($plugin:literal, $editor:ty, $manifest:expr, $seed:expr) => {
        /// 🍼️ LAW (design §12, §20.15): a child-lane mutation of this editor is edited in history end to end on its member store.
        #[test]
        fn child_history_edits_end_to_end() {
            let summary = $crate::app::history_edit_acceptance::block_on_acceptance($crate::app::history_edit_acceptance::assert_child_history_edits_end_to_end::<$crate::EditorApp<$editor>, <$editor as $crate::ArtifactEditor>::Members>(
                $plugin, $manifest, &$seed,
            ));
            ::std::println!("[child-history-edit] {summary}");
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🔎️array-controls/🦀️.rs"]
mod array_controls;

#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../🧰️ui-action-collection/🦀️.rs"]
pub mod ui_action_laws;
#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../🧪️node-drag-history/🦀️.rs"]
pub mod node_drag_history;
