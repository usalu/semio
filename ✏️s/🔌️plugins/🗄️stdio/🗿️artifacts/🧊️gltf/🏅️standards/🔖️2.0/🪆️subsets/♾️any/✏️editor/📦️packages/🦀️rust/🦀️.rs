//! ✏️ `stdio.gltf` editor, viewer and examples.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        #[path = "../../../📚️examples/🎬️demo/🦀️.rs"]
        mod component;
        pub use component::*;
    }
    #[path = "."]
    pub mod metabolism {
        #[path = "../../../📚️examples/🌱️metabolism/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod gltf {
        #[path = "../../🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "../../👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "../../🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod gltf {
        #[path = "../../../👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;
        #[path = "."]
        pub mod presence {
            #[path = "../../../👁️viewer/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod config {
            #[path = "../../../👁️viewer/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "../../../👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "../../../👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
