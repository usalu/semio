//! 🧪️ The one unit-test executable of the sourcing curation artifact: every test module of the domains below is a module of this single target, because a test executable's compile unit is dominated by linking the whole dependency closure (about the same memory whatever it tests), so one executable costs that once instead of once per domain.
#![allow(unused_imports)]
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as vcs;
extern crate semio_framework_schema as framework_schema;
pub use semio_s_artifact_sourcing_curation::*;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧰️harness/🦀️.rs"]
pub mod harness;
pub mod editor_unit_tests {
    pub(crate) use crate::harness as context;
}
semio_plugin::history_edit_acceptance_law!("sourcing", crate::editor::sourcing::SourcingCurationApp, crate::harness::sourcing_manifest_for_tests, "../..");
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_2;
#[path = "../🔬️unit/🦀️.rs"]
mod tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️curation-add/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_3;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️load-document-json/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_4;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📉️set-filter-min-availability/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_5;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_6;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🔢️grid/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_7;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_8;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_9;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧬️app/🦀️.rs"]
mod app_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🌉️actions/🦀️.rs"]
mod action_tests;
#[path = "../../🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📋️clipboard/🧪️tests/🔬️unit/🦀️.rs"]
mod editing_tests;
