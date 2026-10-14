//! 🎭️ remodeling editor modes: capture, model and analyze modes with their windows and tools.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod remodeling {
        #[path = "../../../🫧️transient/🦀️.rs"]
        pub mod transient;
        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod model {
                #[path = "../../🧊️model/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod model {
                        #[path = "../../🧊️model/🪟️windows/🧊️model/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../🧊️model/🪟️windows/🧊️model/🎚️config/🦀️.rs"]
                        pub mod config;
                        #[path = "."]
                        pub mod options {
                            #[path = "../../🧊️model/🪟️windows/🧊️model/☑️options/👁️layers/🦀️.rs"]
                            pub mod layers;
                        }
                    }
                }
                #[path = "."]
                pub mod tools {
                    #[path = "../../🧊️model/🛠️tools/🏗️reconstruction/🦀️.rs"]
                    pub mod reconstruction;
                }
            }
            #[path = "."]
            pub mod capture {
                #[path = "../../📷️capture/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod frames {
                        #[path = "../../📷️capture/🪟️windows/🖼️frames/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../📷️capture/🪟️windows/🖼️frames/🎚️config/🦀️.rs"]
                        pub mod config;
                    }
                }
            }
            #[path = "."]
            pub mod analyze {
                #[path = "../../🔍️analyze/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod report {
                        #[path = "../../🔍️analyze/🪟️windows/📊️report/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "../../🔍️analyze/🪟️windows/📊️report/🎚️config/🦀️.rs"]
                        pub mod config;
                    }
                }
            }
        }
    }
}
