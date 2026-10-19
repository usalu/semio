//! 🧱️ The five part 2d field mutation leaves of `semio-s-artifact-puzzle-5d` with their diff and inverse, held by their own crate so the aggregate crate stays inside the source gate.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_value_derive as value_derive;

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
                        pub mod replace_part_2d_geometry {
                            #[path = "../🧊replace-part2d-geometry/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🧊replace-part2d-geometry/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🧊replace-part2d-geometry/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod edit_part_2d_text {
                            #[path = "../✏️edit-part2d-text/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../✏️edit-part2d-text/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../✏️edit-part2d-text/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_icon {
                            #[path = "../🎨change-part2d-icon/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🎨change-part2d-icon/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🎨change-part2d-icon/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_hidden {
                            #[path = "../🙈change-part2d-hidden/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🙈change-part2d-hidden/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🙈change-part2d-hidden/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                        #[path = "."]
                        pub mod change_part_2d_locked {
                            #[path = "../🔒change-part2d-locked/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "../🔒change-part2d-locked/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "../🔒change-part2d-locked/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                        }
                    }
                }
            }
        }
    }
}
