//! 🧰️ remodeling calibration mutations: camera calibration, rig extrinsic and ground control point leaves (part 2).

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
                        pub mod create_rig_extrinsic {
                            #[path = "../⛓️create-rig-extrinsic/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../⛓️create-rig-extrinsic/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../⛓️create-rig-extrinsic/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_rig_extrinsic {
                            #[path = "../✂️delete-rig-extrinsic/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../✂️delete-rig-extrinsic/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../✂️delete-rig-extrinsic/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_rig_extrinsic {
                            #[path = "../🔩update-rig-extrinsic/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🔩update-rig-extrinsic/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔩update-rig-extrinsic/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod add_gcp_observation {
                            #[path = "../🔎add-gcp-observation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🔎add-gcp-observation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔎add-gcp-observation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_gcp_observation {
                            #[path = "../🚷remove-gcp-observation/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🚷remove-gcp-observation/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🚷remove-gcp-observation/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
