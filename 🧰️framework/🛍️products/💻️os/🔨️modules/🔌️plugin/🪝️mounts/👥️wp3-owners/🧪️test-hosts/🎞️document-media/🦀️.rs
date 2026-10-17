//! 🧪️ Facade test host of `document-media`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[doc(hidden)]
#[cfg(any(test, feature = "media-turn-testing"))]
#[path = "🧱️unit-1/🦀️.rs"]
pub mod document_media_unit_1;
