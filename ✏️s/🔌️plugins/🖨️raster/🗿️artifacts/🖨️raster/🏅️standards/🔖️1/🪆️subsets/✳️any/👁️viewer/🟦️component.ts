/** 👁️ Raster viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no options interfaces beyond the framework's own empty config). */

export const RASTER_VIEWER_DIALECT = { artifactKind: "s.raster.raster", standard: "1", subset: "*" } as const;

export const RASTER_VIEW_MODE_VIEW = "view" as const;

export * as compositeWindow from "./🎭️modes/👁️view/🪟️windows/🖼️composite/🟦️component";
export * as navigatorWindow from "./🎭️modes/👁️view/🪟️windows/🧭️navigator/🟦️component";
