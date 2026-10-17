//! 💡️ modules of `semio-s-artifact-stdio-gltf-inferences` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v2_0 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "."]
                        pub mod size {
                            #[path = "."]
                            pub mod overall_size {
                                #[cfg(test)]
                                #[path = "../../../../🧬️schema/💡️inferences/📦️size/📏️overall-size/🧪️tests/🔬️canonical-vectors/🦀️.rs"]
                                pub mod canonical_vectors;
                            }
                            #[path = "."]
                            pub mod axis_aligned_bounds {
                                #[cfg(test)]
                                #[path = "../../../../🧬️schema/💡️inferences/📦️size/↔️axis-aligned-bounds/🧪️tests/🔬️canonical-vectors/🦀️.rs"]
                                pub mod canonical_vectors;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "../../🦀️.rs"]
                        pub mod inference_measures;
                        #[path = "../../../🧾️measurement-contracts/🦀️.rs"]
                        pub mod measurement_contracts;
                        #[path = "../../../🕸️mesh-topology/🦀️.rs"]
                        pub mod mesh_topology;
                        #[path = "../../../🧮️vector-operations/🦀️.rs"]
                        pub mod vector_operations;
                    }
                }
            }
        }
    }
}
