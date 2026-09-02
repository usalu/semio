/** 🕳️ Deliberately empty — mirrors this directory's Rust twin `🦀️.rs`, which declares no
 * `io() -> IoDeclaration` for this dialect (its own doc comment: registration flows through the
 * `s.stdio.gltf` `ArtifactDeclaration`, not a per-leaf `io()`/`register()`). Repo-wide, only 2 of the
 * 76 stdio `🚪️io/🟦️.ts` mounts carry a real `IoEntryDescriptorMirror[]` — `📄txt/🔖️utf-8`
 * and `💾️binary/🔖️raw` — and both are exactly the 2 stdio artifacts whose Rust twin defines `fn io()`;
 * every other mount, including this one, its own `📥️import`/`📤️export` serializer/deserializer
 * subdirectories carrying the real per-format codec logic instead. The same holds for every complete
 * sibling checked at this identical position — `🟪️stl/…/✳️any/🚪️io`, `📄️pdf/…/✳️base/🚪️io` (1.4 and
 * 1.7), `🌐️html/…/✳️any/🚪️io`, `📊️csv/…/✳️any/🚪️io` and `🖼️tiff/…/✳️baseline/🚪️io` — none of their
 * Rust twins declare `fn io()` either, so none carry a mirror. */
export {};
