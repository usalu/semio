//! 📦️ Package glue — proc-macro crate root; implementation in owner `🦀️.rs`.

#![feature(proc_macro_tracked_path, proc_macro_tracked_env)]

#[path = "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️resources/🧮️compiler/🦀️.rs"]
mod compiler_resources;

fn observe_compiler_resources<R>(run: impl FnOnce() -> R) -> R {
    compiler_resources::with_compiler_resources_v1(env!("CARGO_MANIFEST_DIR"), concat!(env!("CARGO_MANIFEST_DIR"), "/🦀️.rs"), proc_macro::Span::call_site().local_file(), proc_macro::tracked::env_var("SEMIO_COMPILER_RESOURCE_ROOT").ok(), |path| proc_macro::tracked::path(path), run)
}

#[path = "../../🦀️.rs"]
mod component;

use proc_macro::TokenStream;

#[proc_macro_derive(MutationLeaf, attributes(mutation_leaf))]
pub fn derive_mutation_leaf(input: TokenStream) -> TokenStream {
    observe_compiler_resources(|| component::expand_mutation_leaf(input))
}



//#region 🔖️DslArtifact
#[proc_macro_derive(DslArtifact, attributes(artifact))]
// 🚫️async: E3 proc-macro entry
pub fn derive_dsl_document(input: TokenStream) -> TokenStream {
    observe_compiler_resources(|| component::expand_dsl_document(input))
}
//#endregion 🔖️DslArtifact

/// 📝️ Implements a diff's text representation at its I/O owner.
#[proc_macro]
pub fn diff_text(input: TokenStream) -> TokenStream {
    observe_compiler_resources(|| component::expand_diff_text(input))
}

/// 💾️ Implements a diff's binary representation at its I/O owner.
#[proc_macro]
pub fn diff_binary(input: TokenStream) -> TokenStream {
    observe_compiler_resources(|| component::expand_diff_binary(input))
}








/// 🧩️ Derives transparent delegation and full source-validated metadata from direct mutation leaves.
#[proc_macro_derive(Mutations, attributes(mutations))]
pub fn derive_mutations(input: TokenStream) -> TokenStream {
    observe_compiler_resources(|| component::expand_derive_mutations(input))
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
    observe_compiler_resources(|| component::expand_derive_composite_mutation(input))
}
