//#region 🔣️JsonArtifactSerializerAssembly
/** 🫙️ Deliberately empty. `🦀️.rs` builds `artifacts::json` inline (`#[path = "."]`) and
 * `#[path]`s straight past this level into `🔖️rfc8259/✳️any/🦀️.rs`, which is where
 * the real glTF-embeds-JSON serializer lives — there's no TS leaf at that position at all, so
 * there's nothing for this file to re-export either. `📜️docx`/`🎞️pptx`/`📕️xlsx`/`🎨️svg` show
 * the same shape one format over (`🎒️zip`, `📰️xml`): no component file at the format-name
 * folder, only at `<format>/<version>/<subset>/`. Not imported by any TS module. */
export {};
//#endregion 🔣️JsonArtifactSerializerAssembly
