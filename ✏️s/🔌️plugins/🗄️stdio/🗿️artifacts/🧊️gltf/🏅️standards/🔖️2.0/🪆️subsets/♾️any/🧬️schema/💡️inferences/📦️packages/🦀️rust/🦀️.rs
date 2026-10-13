//! 💡️ `stdio.gltf` inferences: the measurement and topology modules, the inference schema and codecs and the decoded-accessor projection.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_os_kernel as protocol;
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
                pub mod io {
                    #[path = "."]
                    pub mod binary {
                        #[path = "../../../../🚪️io/💾️binary/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "."]
                    pub mod text {
                        #[path = "../../../../🚪️io/📝️text/💡️inferences/🦀️.rs"]
                        pub mod inferences;
                    }
                    #[path = "../../../../🚪️io/💾️binary/📸️snapshot/🧮️decoded-accessors/🦀️.rs"]
                    pub mod inference_projection;
                }
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod inferences {
                        #[path = "../../🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../🤝️adjacency/🦀️.rs"]
                        pub mod adjacency;
                        #[path = "../../🧱️area-volume/🦀️.rs"]
                        pub mod area_volume;
                        #[path = "../../↔️clearance/🦀️.rs"]
                        pub mod clearance;
                        #[path = "../../🗜️compactness/🦀️.rs"]
                        pub mod compactness;
                        #[path = "../../🕳️concavity/🦀️.rs"]
                        pub mod concavity;
                        #[path = "../../🌀️curvature/🦀️.rs"]
                        pub mod curvature;
                        #[path = "../../🏗️dag-assembly/🦀️.rs"]
                        pub mod dag_assembly;
                        #[path = "../../🔨️geometry-core/🦀️.rs"]
                        pub mod geometry_core;
                        #[path = "../../⚖️mass-distribution/🦀️.rs"]
                        pub mod mass_distribution;
                        #[path = "../../🧭️orientation/🦀️.rs"]
                        pub mod orientation;
                        #[path = "../../📏️proportion/🦀️.rs"]
                        pub mod proportion;
                        #[path = "../../🌊️roughness/🦀️.rs"]
                        pub mod roughness;
                        #[path = "../../📦️size/🦀️.rs"]
                        pub mod size;
                        #[path = "../../🪞️symmetry/🦀️.rs"]
                        pub mod symmetry;
                        #[path = "../../↕️thickness/🦀️.rs"]
                        pub mod thickness;
                        #[path = "../../🕸️topology/🦀️.rs"]
                        pub mod topology;
                    }
                    #[path = "."]
                    pub mod modules {
                        #[path = "../../../../🔨️modules/💡️inference-measures/🦀️.rs"]
                        pub mod inference_measures;
                        #[path = "../../../../🔨️modules/🧾️measurement-contracts/🦀️.rs"]
                        pub mod measurement_contracts;
                        #[path = "../../../../🔨️modules/🕸️mesh-topology/🦀️.rs"]
                        pub mod mesh_topology;
                        #[path = "../../../../🔨️modules/🧮️vector-operations/🦀️.rs"]
                        pub mod vector_operations;
                    }
                }
            }
        }
    }
}
