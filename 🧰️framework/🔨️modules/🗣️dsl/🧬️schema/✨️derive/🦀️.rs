//! 🧬️ Generic record, scalar and tagged-variant compilation under explicit owned composition.
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};
#[path="📥️owned/🦀️.rs"]
mod owned_receiving;

/// 🧬️ Derives the complete generic record construction contract.
#[proc_macro_derive(DslRecord, attributes(dsl))]
pub fn derive_record(input: TokenStream) -> TokenStream { emit_record(parse_macro_input!(input as DeriveInput)).into() }

/// 🏷️ Derives a generic unit-variant scalar field.
#[proc_macro_derive(DslScalar, attributes(dsl))]
pub fn derive_scalar(input: TokenStream) -> TokenStream { emit_scalar(parse_macro_input!(input as DeriveInput)).into() }

/// 🪆️ Derives generic tagged variants without product operation traits.
#[proc_macro_derive(DslEnum, attributes(dsl))]
pub fn derive_enum(input: TokenStream) -> TokenStream { emit_enum(parse_macro_input!(input as DeriveInput)).into() }

/// 🫴️ Derives typed receiving with every partial authored field retained by the original owner.
#[proc_macro_derive(DslReceiving, attributes(dsl))]
pub fn derive_receiving(input:TokenStream)->TokenStream{emit_receiving(parse_macro_input!(input as DeriveInput)).into()}

#[path = "🏷️attribute-parsing/🦀️.rs"]
mod attribute_parsing;
use attribute_parsing::*;

#[path = "🧩️type-shape/🦀️.rs"]
mod type_shape;
use type_shape::*;

#[path = "🏗️record-field-codegen/🦀️.rs"]
mod record_field_codegen;
use record_field_codegen::*;

#[path = "🧹️lifecycle-codegen/🦀️.rs"]
mod lifecycle_codegen;
use lifecycle_codegen::*;

#[path = "🚦️emitters/🦀️.rs"]
mod emitters;
use emitters::*;

#[path = "🌿️variant-helpers/🦀️.rs"]
mod variant_helpers;
use variant_helpers::*;

#[path = "🪢️receiving-and-projection/🦀️.rs"]
mod receiving_and_projection;
use receiving_and_projection::*;

#[path = "📚️borrowed-owner-codegen/🦀️.rs"]
mod borrowed_owner_codegen;
use borrowed_owner_codegen::*;

/// 🧩️ Composes complete generic record bindings into a higher declaration.
#[proc_macro]
pub fn record_binding(input: TokenStream) -> TokenStream { emit_record(parse_macro_input!(input as DeriveInput)).into() }

/// 📑️ Composes only the caller-named ordinary record projection members.
#[proc_macro]
pub fn record_projection(input: TokenStream) -> TokenStream { let input = parse_macro_input!(input as ProjectionInput); emit_projection(input.declaration, input.spec, input.to, input.from).into() }

/// 🌿️ Composes generic tagged variant bindings into a higher declaration.
#[proc_macro]
pub fn variant_binding(input: TokenStream) -> TokenStream { emit_enum(parse_macro_input!(input as DeriveInput)).into() }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
