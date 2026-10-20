//! 🪝️ remodeling media mutations: stream, asset and durable content leaves (part 3).

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
                        pub mod add_stream_frame {
                            #[path = "../➕add-stream-frame/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../➕add-stream-frame/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../➕add-stream-frame/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod remove_stream_frame {
                            #[path = "../➖remove-stream-frame/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../➖remove-stream-frame/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../➖remove-stream-frame/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
