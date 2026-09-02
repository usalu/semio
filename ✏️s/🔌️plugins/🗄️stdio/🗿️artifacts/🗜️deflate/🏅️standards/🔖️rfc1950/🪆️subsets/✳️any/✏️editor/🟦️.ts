/** ✏️ Deflate editor — surface root: namespaced re-export of the one window's typed twin. Not a
 * blanket `export *` — every migrated surface in this ticket follows the same convention even with
 * a single window, so a second window added later never becomes an ambiguous re-export. */

export const DEFLATE_EDITOR_DIALECT = { artifactKind: "s.stdio.deflate", standard: "rfc1950", subset: "*" } as const;

export const DEFLATE_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
