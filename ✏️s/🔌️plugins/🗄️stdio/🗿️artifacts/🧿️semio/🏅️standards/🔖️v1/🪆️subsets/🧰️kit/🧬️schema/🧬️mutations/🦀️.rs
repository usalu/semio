//! 🧬️ SemioKitMutation — document mutation dispatch. Vocabulary derived from
//! `📸️snapshot/🦀️.rs`'s `SemioKitSnapshot` shape: two owned-CHILD collections
//! (`objects`/`models`, `create`/`delete` pairs), one optional owned-CHILD slot (`properties`,
//! `create`/`delete`), one LINK collection (`representations`, `bind`/`unbind` attach/detach plus
//! `change` to re-pin), and two id-keyed value collections (`types`: `add`/`remove`/`rename`;
//! `designs`: `add`/`remove`/`edit` — a design's pieces/connections are one authored unit, `edit`
//! replaces them wholesale per `📓️taxonomy.md`'s "replace an authored content body" rule, same
//! shape `🔤️text`'s `edit-run` uses one level down).
//!
//! `object` has no LINK slots (`📦️object`'s own doc comment), so this is the FIRST facet in the
//! ticket to exercise `bind`/`unbind`/`change-link-pin` for real.

use crate::standards::v1::subsets::kit::schema::diff::SemioKitDiff;
use crate::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Leaves
use super::add_design;
use super::add_type;
use super::bind_representation;
use super::change_representation_pin;
use super::create_model;
use super::create_object;
use super::create_properties;
use super::delete_model;
use super::delete_object;
use super::delete_properties;
use super::edit_design;
use super::remove_design;
use super::remove_type;
use super::rename_type;
use super::unbind_representation;
//#endregion 🔖️Leaves

//#region 🔖️Mutations

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::Mutations)]
#[mutations(snapshot = SemioKitSnapshot, diff = SemioKitDiff, schema = "s.stdio.semio.kit")]
pub enum SemioKitMutation {
    CreateObject(create_object::CreateObject),
    DeleteObject(delete_object::DeleteObject),
    CreateModel(create_model::CreateModel),
    DeleteModel(delete_model::DeleteModel),
    CreateProperties(create_properties::CreateProperties),
    DeleteProperties(delete_properties::DeleteProperties),
    BindRepresentation(bind_representation::BindRepresentation),
    UnbindRepresentation(unbind_representation::UnbindRepresentation),
    ChangeRepresentationPin(change_representation_pin::ChangeRepresentationPin),
    AddType(add_type::AddType),
    RemoveType(remove_type::RemoveType),
    RenameType(rename_type::RenameType),
    AddDesign(add_design::AddDesign),
    RemoveDesign(remove_design::RemoveDesign),
    EditDesign(edit_design::EditDesign),
}

/// 🏷️ Kebab-case spelling of every `SemioKitMutation` variant, in declaration order — the
/// vocabulary the `semio-v1-kit` mutation catalog (`../../🔣️oracle.json`) declares and
/// `🧰️mutate-semio-kit`'s exhaustive test case measures itself against. `kinds_match_the_enum_and_
/// the_catalog` below is what keeps this list honest against the enum, since the framework never
/// parses Rust.
pub const KINDS: &[&str] = &["create-object",
    "delete-object",
    "create-model",
    "delete-model",
    "create-properties",
    "delete-properties",
    "bind-representation",
    "unbind-representation",
    "change-representation-pin",
    "add-type",
    "remove-type",
    "rename-type",
    "add-design",
    "remove-design",
    "edit-design",
];
//#endregion 🔖️Mutations

/// 🧮️ Pure diff face of [`Mutation::diff`], named only in this subset's own reachable types (`protocol` is a private
/// `extern crate` alias, so an owner-root test adapter cannot bring the `Mutation` trait into scope).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_semio_kit_mutation(mutation: &SemioKitMutation, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    <SemioKitMutation as protocol::Mutation<SemioKitSnapshot>>::diff(mutation, base)
}


/// ↩️ Computes `mutation`'s own inverse against `base` — thin wrapper around `protocol::Mutation::
/// inverse` so external Rust callers that cannot name this crate's private `protocol` extern-crate
/// item (e.g. `🧰️mutate-semio-kit`'s test adapter, whose `@id-inverse` scenario needs a mutation's
/// own computed inverse and cannot `use protocol::Mutation;` itself) can still exercise the
/// inverse-law scenario `apply_semio_kit_mutation` alone can't reach.
pub fn inverse_semio_kit_mutation(mutation: &SemioKitMutation, base: &SemioKitSnapshot) -> Result<Vec<SemioKitMutation>, semio_framework_value::ValueError> {
    Ok({
    use protocol::Mutation;
    mutation.inverse(base)?

    })
}


//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️FixtureTests
/// 🧪️ Handcrafted mutation fixtures (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`)
/// — one case per triad leaf, self-wired here rather than in `🦀️.rs` so this subset owns its
/// own test surface. `#[path = "."]` re-roots the nested `#[path]`s at THIS file's directory (the
/// `🧬️mutations` root) instead of the implicit `🦀️component/` child directory.
#[cfg(test)]
#[path = "🧪️tests/🔬️fixture/🦀️.rs"]
mod fixture_tests;
//#endregion 🧪️FixtureTests
