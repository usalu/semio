//! 🧮️ The semio math framework: one crate for every mathematical domain the OS kernel, the s-modules and the plugins compute with.
//!
//! Each domain is a `🦀️component.rs` in the owner tree; this entry file is pure wiring.

extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_os_kernel as dsl_schema;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_geometry as geometry;
extern crate semio_framework_graph as graph_core;
pub use dsl_core::os_dsl;
// 🧮️ 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3d moved
// `algebra`/`optimize`/`lie`/`signal`/`spatial` out of this crate — `📸️remodel` was their sole
// consumer (verified symbol-by-symbol); they now live as `crate::algebra`/`crate::optimize`/
// `crate::lie`/`crate::signal`/`crate::spatial` under that plugin's own artifact schema (`algebra`
// duplicated in part into `🏗️fem`'s own `➕️algebra` too — see that crate's own module for why).
// 🎲️ `entropy` dissolved into the `✳️table` subset (`🧬️schema/🎲️entropy-internals` + a real
// `💡️inferences/🎲entropy` `InferredField`) — information-theoretic measures are derivations over
// tabular data, not a persisted content shape of their own. `fuzzy` likewise moved to
// `✳️value/🧬️schema/🌫️fuzzy-internals`, parked without an inference pending a domain owner.
#[path = "../../🔢️number/🦀️component.rs"]
pub mod number;

#[path = "../../🎯️sampling/🦀️component.rs"]
pub mod sampling;

#[path = "."]
pub mod graph {
    #[path = "../../🕸️graph/🚶️traversal/🦀️component.rs"]
    pub mod traversal;

    #[path = "../../🕸️graph/🔧️operators/🦀️component.rs"]
    pub mod operators;

    #[path = "../../🕸️graph/🗣️dsl/🦀️component.rs"]
    pub mod dsl;

    #[path = "."]
    pub mod normal {
        #[path = "../../🕸️graph/➕️normal/↔️undirected/🦀️component.rs"]
        pub mod undirected;

        #[path = "../../🕸️graph/➕️normal/➡️directed/🦀️component.rs"]
        pub mod directed;
    }

    #[path = "."]
    pub mod ports {
        #[path = "../../🕸️graph/🔌️ports/↔️undirected/🦀️component.rs"]
        pub mod undirected;

        #[path = "."]
        pub mod directed {
            #[path = "../../🕸️graph/🔌️ports/➡️directed/➕️normal/🦀️component.rs"]
            pub mod normal;
        }
    }
}

// 🧩️ `wfc` dissolved into the Assembly artifact
// (`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/…/🧬️schema/💡️inferences/🧩️wfc-engine/`),
// where the solve is reached only as an `InferredField` over a snapshot authored by mutations —
// slots/rules/weights/seed as persisted content, `AssemblySolve`/`AssemblyContradiction`/
// `AssemblyEntropy` as its derivations. Verified 626/626 symbol parity before this removal.
