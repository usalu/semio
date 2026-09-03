//#region 📥️ArtifactDeserializersAssembly
//! 🫙️ Deliberately empty. `🦀️.rs` builds
//! `pub mod import { pub mod deserializers { pub mod artifacts { … } } }` inline
//! (`#[path = "."]`) and `#[path]`s straight past this level into the leaf
//! `🔣️json/🔖️rfc8259/✳️any/🦀️.rs`. Every stdio sibling with embedded artifacts —
//! `🟪️stl`/`🎒️zip`/`📄️pdf` (`💾️binary`, `📄️txt`/`🗜️deflate`), `📜️docx`/`🎞️pptx`/`📕️xlsx`/
//! `🎨️svg` (`🎒️zip`, `📰️xml`) — mounts nothing at this wrapper either; content always lives at
//! `🗿️artifacts/<format>/<version>/<subset>/component.rs`. Not part of any `mod` tree.
//#endregion 📥️ArtifactDeserializersAssembly
