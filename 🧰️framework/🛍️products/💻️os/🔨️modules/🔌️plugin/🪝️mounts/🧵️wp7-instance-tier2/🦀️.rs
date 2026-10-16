// 🧵️ HUB WP7: mount hub of WP7 instance tier 2 and 3, builder, testing: instance-commands, instance-typed-ops, instance-render, instance-ladders, time-travel-turns, tool-run-turns, instance-app, builder, testing-laws, testing-acceptance and the root test fixtures.
// This file is spliced into the plugin crate root by `include!` from the root, so every item below stays a crate-root item;
// its `#[path]` literals resolve against this folder (`🪝️mounts/<hub>/`), hence the `../../` prefix of the plugin folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-PLUGIN-CRATES.md, section 12.5).

//#region 🏛️OwnershipFacet
#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../../🧪️tests/🧰️ui-action-collection/🦀️.rs"]
pub mod ui_action_laws;
//#endregion 🏛️OwnershipFacet
//#region 📄️DeclarationFixtureMutationMount
#[cfg(test)]
#[path = "../../🧪️tests/🛰️declaration-channels/🦀️.rs"]
pub(crate) mod declaration_fixture_mutations;
//#endregion 📄️DeclarationFixtureMutationMount
//#region 🏛️OwnershipFacet
#[cfg(test)]
#[path = "../../🧪️tests/📢️publication-fixtures/🦀️.rs"]
pub(crate) mod publication_fixture;
#[cfg(test)]
#[path = "../../🧪️tests/📢️publication-owners/🦀️.rs"]
pub(crate) mod window_transient_owners;
#[cfg(test)]
#[path = "../../🧪️tests/🖥️test-app-mutations/🦀️.rs"]
pub(crate) mod test_app_mutation_fixture;
#[cfg(test)]
#[path = "../../🧪️tests/📡️contributed-mutation-wire/🦀️.rs"]
pub(crate) mod contributed_mutation_wire;
//#endregion 🏛️OwnershipFacet
//#region 🧵️InstanceFacet
#[path = "../../🧵️instance/🦀️.rs"]
pub(crate) mod artifact_instance;
#[path = "../../🏗️builder/🦀️.rs"]
pub(crate) mod builder;
//#endregion 🧵️InstanceFacet
//#region ⚖️AppLaws
#[cfg(any(test, feature = "artifact-app-testing"))]
#[path = "../../🧪️testing/🦀️.rs"]
pub(crate) mod app_testing;
//#endregion ⚖️AppLaws
