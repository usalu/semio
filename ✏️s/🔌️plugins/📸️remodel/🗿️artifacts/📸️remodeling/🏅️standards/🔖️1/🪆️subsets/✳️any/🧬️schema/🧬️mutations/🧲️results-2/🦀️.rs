//! 🧲️ remodeling result mutations: the replace-* result leaves and commit-reconstruction (part 2).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod replace_sparse {
                            #[path = "../⭐replace-sparse/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../⭐replace-sparse/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../⭐replace-sparse/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_mesh_result {
                            #[path = "../🧱replace-mesh-result/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🧱replace-mesh-result/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧱replace-mesh-result/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_tracks {
                            #[path = "../🚂replace-tracks/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🚂replace-tracks/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🚂replace-tracks/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_qc {
                            #[path = "../🧾replace-qc/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🧾replace-qc/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧾replace-qc/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
