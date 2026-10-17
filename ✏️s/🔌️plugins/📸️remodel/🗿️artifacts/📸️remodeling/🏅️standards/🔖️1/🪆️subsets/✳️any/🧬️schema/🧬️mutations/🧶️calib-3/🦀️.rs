//! 🧶️ remodeling calibration mutations: camera calibration, rig extrinsic and ground control point leaves (part 3).

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
                        pub mod create_gcp {
                            #[path = "../🧿create-gcp/🦀️.rs"]
                            mod component;
                            #[path = "../🧿create-gcp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧿create-gcp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_gcp {
                            #[path = "../🚮delete-gcp/🦀️.rs"]
                            mod component;
                            #[path = "../🚮delete-gcp/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🚮delete-gcp/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
