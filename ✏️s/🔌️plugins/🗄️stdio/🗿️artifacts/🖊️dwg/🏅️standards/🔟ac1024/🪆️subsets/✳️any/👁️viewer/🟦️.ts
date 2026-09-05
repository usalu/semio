/** 👁️ DWG AC1024 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const DWG_AC1024_VIEWER_DIALECT = { artifactKind: "s.stdio.dwg", standard: "ac1024", subset: "*" } as const;

export const DWG_AC1024_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
