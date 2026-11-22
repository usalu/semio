//! 👥️ Presence lane of the PDF editors: the typed ephemeral shared state of one collaborator, the fold of every collaborator's presence events and the overlays projected from it.

#![allow(clippy::result_large_err)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod pdf_presence {
        #[path = "."]
        pub mod lane {
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
pub use editor::pdf_presence::lane::*;

#[path = "../../🧮️board/🦀️.rs"]
mod board;
pub use board::*;

#[path = "../../🖍️layers/🦀️.rs"]
mod layers;
pub use layers::*;

#[path = "../../🧭️intent/🦀️.rs"]
mod intent;
pub use intent::*;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
