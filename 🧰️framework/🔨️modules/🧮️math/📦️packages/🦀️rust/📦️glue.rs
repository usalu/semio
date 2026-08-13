//! 🧮️ The semio math framework: one crate for every mathematical domain the OS kernel, the s-modules and the plugins compute with.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

extern crate semio_framework_geometry as geometry;
// 🃏️ wave MATHEND: `dsl_core`/`dsl_schema`/`dsl`/`graph_core` aliases and the `os_dsl` re-export
// removed — they existed only for `graph::dsl` (Jack), which this same wave relocated to the
// framework `🕸️graph` module (see below); `sampling`, the only content left in this crate, only
// ever needed `geometry::random`.
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d moved
// `algebra`/`optimize`/`lie`/`signal`/`spatial` out of this crate — `📸️remodel` was their sole
// consumer (verified symbol-by-symbol); they now live as `crate::algebra`/`crate::optimize`/
// `crate::lie`/`crate::signal`/`crate::spatial` under that plugin's own artifact schema (`algebra`
// duplicated in part into `🏗️fem`'s own `➕️algebra` too — see that crate's own module for why).
// 🎲️ `entropy` dissolved into the `✳️table` subset (`🧬️schema/🎲️entropy-internals` + a real
// `💡️inferences/🎲entropy` `InferredField`) — information-theoretic measures are derivations over
// tabular data, not a persisted content shape of their own. `fuzzy` likewise moved to
// `✳️value/🧬️schema/🌫️fuzzy-internals`, parked without an inference pending a domain owner.
// 🔢️ wave MATHEND: `number` relocated to its own framework module (`semio-framework-number`) —
// domain-neutral exact-numeric vocabulary with framework-tier consumers (`🧊️3d/📐️brep/⚖️predicates`)
// that cannot depend on a plugin, exactly the exemption category `📐️geometry` already occupies.
// See that wave's report for the full exemption justification and symbol-parity proof.
// 🕸️ wave MATHEND: `traversal`/`operators`/`normal`/`ports` (~4,993 LOC) had ZERO consumers
// anywhere in the repo (verified: not even inside `🧮️math` itself) — migrated under the
// "nothing deleted" rule to `✏️s/🔌️plugins/🗄️stdio`'s `✳️graph` subset as
// `🧬️schema/{🚶️traversal-internals,🔧️operators-internals,➕️normal-internals,🔌️ports-internals}`,
// with a genuine `💡️inferences/🔗connectivity` `InferredField` (per-node degree +
// weakly-connected-component id) proving they are a real artifact facet, not a relocated library.
// `dsl` (Jack) relocated to `🧰️framework/🔨️modules/🕸️graph/🗣️dsl` — see that wave's report for
// the framework/plugin split hypothesis, measured and rejected (real internal coupling), and the
// domain-neutral reasoning for keeping it whole in the framework `🕸️graph` module.
#[path = "../../🎯️sampling/🦀️component.rs"]
pub mod sampling;

// 🧩️ `wfc` dissolved into the Assembly artifact
// (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/…/🧬️schema/💡️inferences/🧩️wfc-engine/`),
// where the solve is reached only as an `InferredField` over a snapshot authored by mutations —
// slots/rules/weights/seed as persisted content, `AssemblySolve`/`AssemblyContradiction`/
// `AssemblyEntropy` as its derivations. Verified 626/626 symbol parity before this removal.
