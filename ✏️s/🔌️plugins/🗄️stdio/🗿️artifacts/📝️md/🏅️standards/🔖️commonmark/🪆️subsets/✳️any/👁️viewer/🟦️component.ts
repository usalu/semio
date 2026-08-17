/** 👁️ `md` viewer (any) — read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const MD_VIEWER_DIALECT = { artifactKind: "s.stdio.md", standard: "commonmark", subset: "*" } as const;

export const MD_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
