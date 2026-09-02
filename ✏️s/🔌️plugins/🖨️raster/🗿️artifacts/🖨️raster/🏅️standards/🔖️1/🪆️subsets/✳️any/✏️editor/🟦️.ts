/** ✏️ Raster editor — subset-level typed twin. Re-exports every window's typed view-model binding so
 * a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_raster_app()` stitching every window/mode module together. */

export const RASTER_EDITOR_DIALECT = { artifactKind: "s.raster.raster", standard: "1", subset: "*" } as const;

export const RASTER_PLAY_MODE_EDIT = "edit" as const;

export * as compositeWindow from "./🎭️modes/✏️edit/🪟️windows/🖼️composite/🟦️component";
export * as navigatorWindow from "./🎭️modes/✏️edit/🪟️windows/🧭️navigator/🟦️component";
