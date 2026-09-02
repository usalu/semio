/** 👁️ IFC 2x3 Cv20 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const IFC2X3_CV20_VIEWER_DIALECT = { artifactKind: "s.stdio.ifc", standard: "2x3", subset: "cv20" } as const;

export const IFC2X3_CV20_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
