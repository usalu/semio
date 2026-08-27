//! 🔧 imperative artifact — OpText/OpBinary bridge for `ImperativeMutation`.

pub use crate::artifacts::imperative::schema::mutations::ImperativeMutation;

pub const TEXT_OPCODE_REGISTRY: &[(&str, &str)] =
    &[("create-step", super::create_step::text::TEXT_OPCODE), ("delete-step", super::delete_step::text::TEXT_OPCODE), ("reorder-steps", super::reorder_steps::text::TEXT_OPCODE), ("edit-step-params", super::edit_step_params::text::TEXT_OPCODE)];

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
