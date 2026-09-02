/** 👁️ Semio Brep viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const SEMIO_BREP_VIEWER_DIALECT = { artifactKind: "s.stdio.semio", standard: "v1", subset: "brep" } as const;

export const SEMIO_BREP_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
