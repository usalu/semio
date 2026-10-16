// 🧪️ HUB WP8: mount hub of WP8 root-mounted tests of the facade.
// This file is spliced into the crate root by `include!` from the kernel root, so every item below stays a crate-root item;
// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the kernel package folder.
// Carving a crate removes its unit here and puts the facade `pub use` lines of the crate in its place (PLAN-OS-KERNEL-CRATES.md, section 10.2).

//#region 🧪️Tests
#[cfg(test)]
include!("../../../../🧪️tests/🔬️standalone/🦀️.rs");
//#endregion 🧪️Tests

#[cfg(test)]
#[path="../../../../../../🔨️modules/🌱️value/🔁️codec/🧪️tests/♻️retirement/🦀️.rs"]
mod canonical_intrinsic_retirement_tests;

#[cfg(test)]
#[path = "../../../../🔨️modules/📡️spr/🧵️channel/🧪️tests/📤️return-content/🦀️.rs"]
mod channel_return_content_tests;

#[cfg(test)]
#[path = "../../../../🔨️modules/🌿️vcs/🧪️tests/🧪️branch-provenance/🦀️.rs"]
mod vcs_branch_provenance_tests;
