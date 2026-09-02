/** ✏️ Zip editor (2.0/✳️iso21320) — surface root: namespaced re-export of the one window's typed
 * twin. Not a blanket `export *` — every migrated surface in this ticket follows the same
 * convention even with a single window, so a second window added later never becomes an ambiguous
 * re-export. */

export const ZIP_ISO21320_EDITOR_DIALECT = { artifactKind: "s.stdio.zip", standard: "2.0", subset: "iso21320" } as const;

export const ZIP_ISO21320_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
