//! 🎞️ remodeling media mutations: stream, asset and durable content leaves.

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
                        pub mod create_stream {
                            #[path = "../../../🌱create-stream/🦀️.rs"]
                            mod component;
                            #[path = "../../../🌱create-stream/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🌱create-stream/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_stream {
                            #[path = "../../../🪓delete-stream/🦀️.rs"]
                            mod component;
                            #[path = "../../../🪓delete-stream/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🪓delete-stream/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod replace_stream_source {
                            #[path = "../../../🔁replace-stream/🦀️.rs"]
                            mod component;
                            #[path = "../../../🔁replace-stream/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../../../🔁replace-stream/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        pub use semio_s_artifact_remodel_remodeling_mut_media_2::standards::v1::subsets::any::schema::mutations::{change_stream_sync, create_asset, delete_asset, append_content, remove_content};
                        pub use semio_s_artifact_remodel_remodeling_mut_media_3::standards::v1::subsets::any::schema::mutations::{add_stream_frame, remove_stream_frame};
                    }
                }
            }
        }
    }
}
