/** 👁️ IFC 2x3 Cobie viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const IFC2X3_COBIE_VIEWER_DIALECT = { artifactKind: "s.stdio.ifc", standard: "2x3", subset: "cobie" } as const;

export const IFC2X3_COBIE_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
