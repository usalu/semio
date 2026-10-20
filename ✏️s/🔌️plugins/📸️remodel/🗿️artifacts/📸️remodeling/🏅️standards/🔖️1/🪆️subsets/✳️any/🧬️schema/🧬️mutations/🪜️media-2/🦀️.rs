//! 🪜️ remodeling media mutations: stream, asset and durable content leaves (part 2).

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
                        pub mod change_stream_sync {
                            #[path = "../⏱️change-stream-sync/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../⏱️change-stream-sync/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../⏱️change-stream-sync/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod create_asset {
                            #[path = "../🧷create-asset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🧷create-asset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧷create-asset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod delete_asset {
                            #[path = "../🗞️delete-asset/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🗞️delete-asset/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🗞️delete-asset/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod append_content {
                            #[path = "../📦append-content/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../📦append-content/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../📦append-content/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_content {
                            #[path = "../🔪remove-content/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🔪remove-content/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔪remove-content/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
