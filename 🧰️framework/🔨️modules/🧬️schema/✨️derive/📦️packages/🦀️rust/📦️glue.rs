//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️component.rs`.

#[path = "../../🦀️component.rs"]
mod component;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// ✨️ Derives [`ArtifactSchemaFields`] from `#[artifact_schema]` / `#[state]` / `#[derived]` annotations, plus a
/// sibling [`ArtifactCompositionFields`] impl from `ArtifactChild<T>` / `ArtifactLink` field types
/// (`#[child(kind = "…")]` / `#[link_slot(roles(…))]`) — one derive, since both traits describe the
/// same struct's fields and a struct with no composition fields still needs a (trivially empty) impl.
#[proc_macro_derive(ArtifactSchema, attributes(artifact_schema, state, derived, child, link_slot))]
// 🚫️async: E3 proc-macro entry
pub fn derive_artifact_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match component::expand_artifact_schema(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
