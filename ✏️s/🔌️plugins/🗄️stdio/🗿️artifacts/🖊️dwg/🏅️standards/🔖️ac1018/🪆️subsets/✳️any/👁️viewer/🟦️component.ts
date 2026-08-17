/** 👁️ DWG AC1018 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const DWG_AC1018_VIEWER_DIALECT = { artifactKind: "s.stdio.dwg", standard: "ac1018", subset: "*" } as const;

export const DWG_AC1018_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
