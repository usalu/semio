//! 🌉 Native declared file Save over the established program exchange.
#[path = "🧮️declaration.rs"] mod declaration;
#[path = "📥️driver.rs"] mod driver;
#[path = "📤️sink.rs"] mod sink;
#[path = "📎️reference.rs"] mod reference;
#[path = "📂️open.rs"] mod open;
pub(crate) use open::import_media;
#[path = "🔁️peer.rs"] mod peer;
pub use declaration::NativeMediaDeclaration;
pub use driver::NativeSaveOutcome;
pub(crate) use peer::save_media;
