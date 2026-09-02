/** 👁️ Block 3D viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no config schema beyond the framework's own empty config). */

export const BLOCK3D_VIEWER_DIALECT = { artifactKind: "s.block.block3d", standard: "1", subset: "*" } as const;

export const BLOCK3D_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🌐️world/🟦️component";
