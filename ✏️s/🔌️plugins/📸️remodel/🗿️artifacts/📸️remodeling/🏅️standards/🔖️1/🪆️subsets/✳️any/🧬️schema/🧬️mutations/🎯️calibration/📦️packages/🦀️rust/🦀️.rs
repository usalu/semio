//! 🎯️ remodeling calibration mutations: camera calibration, rig extrinsic and ground control point leaves.

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
                        pub mod create_camera_calibration {
                            #[path = "../../../🔭create-camera/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🔭create-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔭create-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod update_camera_calibration {
                            #[path = "../../../🛠️update-camera-calibration/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🛠️update-camera-calibration/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🛠️update-camera-calibration/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_camera_calibration {
                            #[path = "../../../🚫delete-camera/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../../../🚫delete-camera/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🚫delete-camera/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        pub use semio_s_artifact_remodel_remodeling_mut_calib_2::standards::v1::subsets::any::schema::mutations::{create_rig_extrinsic, delete_rig_extrinsic, update_rig_extrinsic, add_gcp_observation, remove_gcp_observation};
                        pub use semio_s_artifact_remodel_remodeling_mut_calib_3::standards::v1::subsets::any::schema::mutations::{create_gcp, delete_gcp};
                    }
                }
            }
        }
    }
}
