//#region 📤️ArtifactSerializersAssembly
/** 🫙️ Deliberately empty. `🦀️.rs` builds `export::serializers::artifacts` inline
 * (`#[path = "."]`) and `#[path]`s straight past this level into the leaf
 * `🔣️json/🔖️rfc8259/✳️any/🦀️.rs`. Every stdio sibling with embedded artifacts —
 * `🔺️stl`/`🎒️zip`/`📄️pdf` (`💾️binary`, `🔤️txt`/`🗜️deflate`), `📜️docx`/`🎞️pptx`/`📕️xlsx`/
 * `🎨️svg` (`🎒️zip`, `📰️xml`) — mounts nothing at this wrapper either; content always lives at
 * `🗿️artifacts/<format>/<version>/<subset>/component.*`. Not imported by any TS module. */
export {};
//#endregion 📤️ArtifactSerializersAssembly
