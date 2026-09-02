/** 👁️ Semio Value viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const SEMIO_VALUE_VIEWER_DIALECT = { artifactKind: "s.stdio.semio", standard: "v1", subset: "value" } as const;

export const SEMIO_VALUE_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
