/** 🚪️ IO facet barrel — the remodeling document's format hops, TypeScript side.
 *
 *  Mirrors the Rust `IoDeclaration` this subset publishes: `json` and `txt` are the two `Exact`
 *  hops and are real here; `ply`/`las`/`obj`/`stl`/`gltf`/`png` are `Lossy` geometry/raster hops
 *  whose Rust leaves refuse with a reason, and they carry no TypeScript implementation at all —
 *  each of those twelve leaves is an honest `export {}` rather than a codec that lies. There is no
 *  `dwg` hop.
 *
 *  Not implemented, and why:
 *  - **txt export** — needs a DSL *printer*. `dsl::print`'s layout rules (block/table selection,
 *    column-header synthesis, unit re-suffixing, per-type number lexemes, the `_` placeholder
 *    policy) live in `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/` and have no TypeScript twin
 *    anywhere in the repo. Porting `dsl::print` is its own packet.
 *  - **ply / las / obj / stl / gltf / png, both directions** — a remodeling document's geometry is
 *    a content-addressed `ArtifactChild` handle plus packed base64 buffers, never inline vertex
 *    data reachable from a leaf; and the schema has no raster field at all. Writing any of these
 *    would silently claim an export that did not happen.
 */

export { REMODELING_DSL_ENVELOPE, RemodelingDslError, lexDsl, remodelingSnapshotFromDslText } from "./📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🟦️.ts";
export { RemodelingCodecError, RemodelingDecodeError, decodeRemodelingDiff, decodeRemodelingMutation, decodeRemodelingSnapshot, remodelingSnapshotFromJsonText } from "./📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts";
export { encodeRemodelingDiff, encodeRemodelingSnapshot, floatLexeme, remodelingDiffToJsonText, remodelingSnapshotToJsonText } from "./📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🟦️.ts";

/** 🚧 The foreign dialects this subset can only refuse, with the reason each refusal carries. */
export const REMODELING_UNIMPLEMENTED_IO: readonly { dialect: string; direction: "import" | "export"; reason: string }[] = [
  { dialect: "s.stdio.txt@utf-8", direction: "export", reason: "no TypeScript DSL printer exists; dsl::print has no TS twin" },
  ...(["s.stdio.ply@1.0", "s.stdio.las@1.0", "s.stdio.obj@3.0", "s.stdio.stl@ascii", "s.stdio.gltf@2.0"] as const).flatMap((dialect) =>
    (["import", "export"] as const).map((direction) => ({ dialect, direction, reason: "geometry lives behind a content-addressed ArtifactChild handle, never as inline vertex data a leaf can reach" })),
  ),
  ...(["import", "export"] as const).map((direction) => ({ dialect: "s.stdio.png@1.2", direction, reason: "the schema has no raster field and the plugin ships no rasterizer" })),
];
