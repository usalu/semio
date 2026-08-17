/** 👁️ IFC 2x3 Any viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const IFC2X3_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.ifc", standard: "2x3", subset: "*" } as const;

export const IFC2X3_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
