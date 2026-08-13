//! 🕸️ The semio graph framework module: storage and view vocabulary, index-based algorithms, drawing layouts, the compile-time manifest registry, and the Jack graph query language.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

// 🃏️ Renamed `dsl` → `dsl_core` (wave MATHEND) to free the crate-root name `dsl` for
// `pub mod dsl` (Jack) below — both alias the identical crate. `🛂️manifest` updated to match.
extern crate semio_framework_os_kernel as dsl_core;

#[path = "../../⚙️engine/🦀️component.rs"]
mod engine;
pub use engine::*;

#[path = "../../🧮️algorithms/🦀️component.rs"]
pub mod algorithms;

#[path = "../../🖊️drawing/🦀️component.rs"]
pub mod drawing;

#[path = "../../🛂️manifest/🦀️component.rs"]
pub mod manifest;

// 🃏️ wave MATHEND (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS):
// Jack relocated verbatim from `🧰️framework/🔨️modules/🧮️math/🕸️graph/🗣️dsl` — measured NOT
// cleanly splittable into a framework "core" + plugin "language-service" half as first hypothesized:
// `complete`/`format` are needed by BOTH the framework-tier `DslIdiom` self-registration seam
// (`idiom_hooks`) AND `🔱️trinity`'s explicit LSP-style calls, and `complete`/`hover` share private
// helpers (`collect_bound_vars`, `lex_spanned`) that would need to become new public API or be
// duplicated (forbidden) to cross a crate boundary. Independently, Jack (a generic pattern-matching
// query language over graphs, plus its own editor tooling) passes the domain-neutral test — it names
// no domain, structurally analogous to `💻️os/🔨️modules/🗣️dsl`'s already-framework-tier
// diagnostic/completion machinery. See that wave's report for the full evidence and reasoning.
#[path = "../../🗣️dsl/🦀️component.rs"]
pub mod dsl;
