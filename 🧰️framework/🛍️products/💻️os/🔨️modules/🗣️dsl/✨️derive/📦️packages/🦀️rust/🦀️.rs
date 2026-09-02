//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

#[path = "../../🦀️.rs"]
mod component;

use proc_macro::TokenStream;

#[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))]
pub fn derive_mutation_leaf(input: TokenStream) -> TokenStream {
    component::expand_mutation_leaf(input)
}

//#region 🔖️DslRecord
#[proc_macro_derive(DslRecord, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_record(input: TokenStream) -> TokenStream {
    component::expand_dsl_record(input)
}
//#endregion 🔖️DslRecord

//#region 🔖️DslArtifact
#[proc_macro_derive(DslArtifact, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_document(input: TokenStream) -> TokenStream {
    component::expand_dsl_document(input)
}
//#endregion 🔖️DslArtifact

//#region 🔖️DslDiff
/// @emoji 🧬️ W1 foundation of the `handcrafted-grammar-for-every-artifact` diff track (design ruling
/// B-R4): emits a `protocol::DiffCodec` impl from the SAME `RecordSpec`-generation machinery
/// `#[derive(DslRecord)]`/`#[derive(DslArtifact)]` already use — a diff is structurally just another
/// record, so this reuses `record_codegen` verbatim rather than reinventing field lowering. Unlike
/// `DslArtifact` there is no `EXTENSION`/file-extension concept (a diff is never opened as its own
/// file) and no `ArtifactPack` (the pack/binary side is `DiffCodec::encode_diff`/`decode_diff`
/// instead, routed through the same `store::pack_rt` the `ArtifactPack` impl above uses — every
/// crate that already derives an operation/document alongside its diff already depends on `store`).
#[proc_macro_derive(DslDiff, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_diff(input: TokenStream) -> TokenStream {
    component::expand_dsl_diff(input)
}
//#endregion 🔖️DslDiff

//#region 🔖️DslScalar
#[proc_macro_derive(DslScalar, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_scalar(input: TokenStream) -> TokenStream {
    component::expand_dsl_scalar(input)
}
//#endregion 🔖️DslScalar

#[proc_macro_derive(DslOps, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_ops(input: TokenStream) -> TokenStream {
    component::expand_dsl_ops(input)
}

//#region 🔖️DslEnum
/// @emoji 🌳️ Tagged-record enum whose variants are plain data (a recursive block tree, a wire
/// node kind, ...) rather than a `Mutation` — implements `::dsl::DslVariants` only, so it can be
/// used inside `#[dsl(statements)]`/`#[dsl(statements, block)]` collection fields without also
/// gaining (and having to satisfy the bounds of) `store::OpText`.
#[proc_macro_derive(DslEnum, attributes(dsl))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_enum(input: TokenStream) -> TokenStream {
    component::expand_dsl_enum(input)
}
//#endregion 🔖️DslEnum

/// 🧩️ Derives transparent delegation and full source-validated metadata from direct mutation leaves.
#[proc_macro_derive(Mutations, attributes(mutations))]
pub fn derive_mutations(input: TokenStream) -> TokenStream {
    component::expand_derive_mutations(input)
}

/// @emoji 🌉️ Wires a composite mutation kind's delegating `::semio_framework_os_kernel::MutationKind` impl from its
/// handcrafted `::semio_framework_os_kernel::CompositeMutationKind` impl — `#[composite(snapshot = YourSnapshot, op =
/// YourOpEnum)]` on the payload struct that already `impl CompositeMutationKind<YourSnapshot,
/// YourOpEnum> for` itself. `diff`/`inverse`/`foreign_steps` delegate to the free
/// `::semio_framework_os_kernel::fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps` helpers — deliberately NOT
/// a blanket `impl<T: CompositeMutationKind> MutationKind for T`, which coherence rejects against
/// the ~200 concrete `impl MutationKind` in the tree (see
/// `.🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
/// §1). Emits the same kind/verb `const _: () = assert!(..)` checks `#[derive(Mutations)]` emits,
/// checked against the struct's OWN kebab name (a composite kind is never wrapped in an enum
/// variant the way a handcrafted `MutationKind` payload is).
#[proc_macro_derive(CompositeMutation, attributes(composite))]
// 🚫️async: E3 proc-macro entry
pub fn derive_composite_mutation(input: TokenStream) -> TokenStream {
    component::expand_derive_composite_mutation(input)
}
