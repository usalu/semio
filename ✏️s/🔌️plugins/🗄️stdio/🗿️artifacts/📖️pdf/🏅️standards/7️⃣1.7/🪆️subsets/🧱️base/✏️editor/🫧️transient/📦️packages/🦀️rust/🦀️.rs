//! 🫧️ View lane of the PDF editors: the typed ephemeral local state of the page view (zoom, anchored page, search) and the retained work that publishes it.

#![allow(clippy::result_large_err)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod pdf_view {
        #[path = "."]
        pub mod transient {
            #[path = "../../🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "."]
            pub mod io {
                #[path = "."]
                pub mod text {
                    #[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
                #[path = "."]
                pub mod binary {
                    #[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
                    pub mod mutations;
                }
            }
        }
    }
}

#[path = "../../🧵️work/🦀️.rs"]
mod work;
pub use work::*;
pub use editor::pdf_view::transient::*;
