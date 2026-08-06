//! 🌊️ OS flow family glue — wires core and extensions.

extern crate self as flow_core;
extern crate self as flow_extension_brep;
extern crate self as flow_extension_draw;
extern crate self as flow_extension_wasm;
extern crate self as flow_extension_sdk;

#[path = "../../🫀️core/🦀️component.rs"]
pub mod core;
pub use core::*;

#[path = "."]
pub mod extensions {
  #[path = "../../../🧩️extensions/📃️list/🦀️component.rs"]
  pub mod list;

  #[path = "../../../🧩️extensions/📐️brep/🦀️component.rs"]
  pub mod brep;

  #[path = "../../../🧩️extensions/📖️dictionary/🦀️component.rs"]
  pub mod dictionary;

  #[path = "../../../🧩️extensions/📝️text/🦀️component.rs"]
  pub mod text;

  #[path = "../../../🧩️extensions/🕸️wasm/🦀️component.rs"]
  pub mod wasm;

  #[path = "../../../🧩️extensions/🖍️draw/🦀️component.rs"]
  pub mod draw;

  #[path = "../../../🧩️extensions/🧠️logic/🦀️component.rs"]
  pub mod logic;

  #[path = "../../../🧩️extensions/🧮️math/🦀️component.rs"]
  pub mod math;

  #[path = "../../../🧩️extensions/🫀️core/🦀️component.rs"]
  pub mod ext_core;

}

pub use extensions::brep::*;
pub use extensions::draw::*;
pub use extensions::wasm::*;
