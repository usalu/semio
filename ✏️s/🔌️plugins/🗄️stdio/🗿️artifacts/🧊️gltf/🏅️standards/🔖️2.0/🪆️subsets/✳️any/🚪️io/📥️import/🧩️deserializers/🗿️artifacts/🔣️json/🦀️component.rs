//#region 🔣️JsonArtifactDeserializerAssembly
//! 🫙️ Deliberately empty. `📦️glue.rs` builds `pub mod json { pub mod v_rfc8259 { … } }` inline
//! (`#[path = "."]`) and `#[path]`s straight past this level into the real leaf,
//! `🔖️rfc8259/✳️any/🦀️component.rs`. `📜️docx`/`🎞️pptx`/`📕️xlsx`/`🎨️svg` show the same shape
//! one format over (`🎒️zip`, `📰xml`): no component file at the format-name folder, only at
//! `<format>/<version>/<subset>/`. Not part of any `mod` tree.
//#endregion 🔣️JsonArtifactDeserializerAssembly
