//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️component.rs`.

#[path = "../../🦀️.rs"]
mod component;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// 🗃️ Compiles a `machine <name> { .. }` declaration into a `pub mod <name> { .. }`.
#[proc_macro]
pub fn statechart(input: TokenStream) -> TokenStream {
    component::expand_statechart(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🗃️ Implements `machine::StatechartEvent` for a consumer-authored enum.
#[proc_macro_derive(StatechartEvent)]
pub fn derive_statechart_event(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    component::expand_statechart_event(&derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🗃️ Implements `machine::StatechartSchema` for a consumer-authored context struct.
#[proc_macro_derive(StatechartSchema)]
pub fn derive_statechart_schema(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    component::expand_statechart_schema(&derive_input).unwrap_or_else(|e| e.to_compile_error()).into()
}

/// 🗃️ Emits a `#[wasm_bindgen]`-exported class wrapping one machine instance.
#[proc_macro]
pub fn export_wasm_machine(input: TokenStream) -> TokenStream {
    component::expand_export_wasm_machine(input.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}
