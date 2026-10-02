/** ⚡️ The label hot path of the browser canvas bundles: usvg text layout (fontdb, ttf-parser face tables, rustybuzz GSUB/GPOS
 * shaping, tiny-skia-path outlines, roxmltree/svgtypes/simplecss markup) and vello's scene encoding (kurbo, peniko). At
 * `opt-level = 0` one shaping of a ~100-character line cost ~25 ms in wasm, so every typed character of the trinity query
 * editor painted in ~85 ms (ticket 26/09/23 F1). Compiled at `opt-level = 3` in the dev profile too — the browser twin of
 * the root manifest's dev overrides for the wasmtime crates — scoped to the bundles that pass it, so no other build unit
 * changes. */
export const BROWSER_CANVAS_HOT_CRATES = ["ttf-parser", "rustybuzz", "usvg", "fontdb", "roxmltree", "svgtypes", "simplecss", "strict-num", "tiny-skia-path", "kurbo", "peniko", "vello_encoding"] as const;

