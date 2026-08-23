//! 🔮️ `semio-s-plugin-stdio-test-oracle` — the reference implementations for the artifacts this
//! plugin owns.
//!
//! This crate exists so the FRAMEWORK test platform never has to know that PDF, PNG, GIF, ZIP,
//! zlib, WAVE or CSV exist. It is contributed to the platform by
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️component.json`, which the platform discovers by convention;
//! adding a new artifact family here requires no framework edit at all.
//!
//! Every third-party reference library is linked ONLY here, behind an owned interface — no external
//! type appears in this crate's public API — and only under the `oracles` feature, which no
//! production target enables.

//#region 🔖️Modules
#[path = "../../📄️document/🦀️component.rs"]
pub mod document;

#[path = "../../🖼️raster/🦀️component.rs"]
pub mod raster;

#[path = "../../🎒️archive/🦀️component.rs"]
pub mod archive;

#[path = "../../🔊️audio/🦀️component.rs"]
pub mod audio;

#[path = "../../📊️tabular/🦀️component.rs"]
pub mod tabular;

#[path = "../../🧊️mesh/🦀️component.rs"]
pub mod mesh;
//#endregion 🔖️Modules
