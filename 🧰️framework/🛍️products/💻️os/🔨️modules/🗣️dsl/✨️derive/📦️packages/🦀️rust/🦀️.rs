//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

#[path = "../../🦀️.rs"]
mod component;

use proc_macro::TokenStream;

#[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))]
pub fn derive_mutation_leaf(input: TokenStream) -> TokenStream {
    component::expand_mutation_leaf(input)
}



//#region 🔖️DslArtifact
#[proc_macro_derive(DslArtifact, attributes(artifact))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_document(input: TokenStream) -> TokenStream {
    component::expand_dsl_document(input)
}
//#endregion 🔖️DslArtifact

//#region 🔖️DslDiff
/// 🧩️ Derives OS diff transport over the canonical record contract.
#[proc_macro_derive(DslDiff)]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_diff(input: TokenStream) -> TokenStream {
    component::expand_dsl_diff(input)
}
//#endregion 🔖️DslDiff







/// 🧩️ Derives transparent delegation and full source-validated metadata from direct mutation leaves.
#[proc_macro_derive(Mutations, attributes(mutations))]
pub fn derive_mutations(input: TokenStream) -> TokenStream {
    component::expand_derive_mutations(input)
}

/// 🌉️ Wires a composite mutation kind's delegating `::semio_framework_os_kernel::MutationKind` impl from its
/// handcrafted `::semio_framework_os_kernel::CompositeMutationKind` impl — `#[composite(snapshot = YourSnapshot, op =
/// YourOpEnum)]` on the payload struct that already `impl CompositeMutationKind<YourSnapshot,
/// YourOpEnum> for` itself. `diff`/`inverse`/`foreign_steps` delegate to the free
/// `::semio_framework_os_kernel::fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps` helpers — deliberately NOT
/// a blanket `impl<T: CompositeMutationKind> MutationKind for T`, which coherence rejects against
/// the ~200 concrete `impl MutationKind` in the tree (see
/// `.🧬semio/🦑️repo/🎫️tickets/26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS/📋️contract-freeze.md`
/// §1). Emits the same kind/verb `const _: () = assert!(..)` checks `#[derive(Mutations)]` emits,
/// checked against the struct's OWN kebab name (a composite kind is never wrapped in an enum
/// variant the way a handcrafted `MutationKind` payload is).
#[proc_macro_derive(CompositeMutation, attributes(composite))]
// 🚫️async: E3 proc-macro entry
pub fn derive_composite_mutation(input: TokenStream) -> TokenStream {
    component::expand_derive_composite_mutation(input)
}
