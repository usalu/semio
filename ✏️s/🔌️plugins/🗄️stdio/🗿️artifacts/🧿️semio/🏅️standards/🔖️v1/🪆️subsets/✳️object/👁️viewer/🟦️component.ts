/** 👁️ Semio Object viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const SEMIO_OBJECT_VIEWER_DIALECT = { artifactKind: "s.stdio.semio", standard: "v1", subset: "object" } as const;

export const SEMIO_OBJECT_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
