/** 👁️ IFC 2x3 Sav viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const IFC2X3_SAV_VIEWER_DIALECT = { artifactKind: "s.stdio.ifc", standard: "2x3", subset: "sav" } as const;

export const IFC2X3_SAV_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
