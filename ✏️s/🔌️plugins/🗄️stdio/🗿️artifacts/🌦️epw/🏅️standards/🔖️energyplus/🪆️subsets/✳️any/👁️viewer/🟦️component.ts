/** 👁️ EPW viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`: no
 * mutation-shaped exports, no command payload types. */

export const EPW_VIEWER_DIALECT = { artifactKind: "s.stdio.epw", standard: "energyplus", subset: "*" } as const;

export const EPW_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
