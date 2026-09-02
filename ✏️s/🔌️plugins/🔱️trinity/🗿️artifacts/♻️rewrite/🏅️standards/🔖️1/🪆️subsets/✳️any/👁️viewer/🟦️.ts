/** 👁️ Trinity Rewrite viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports (no command payload types, no config schema beyond the framework's own
 * empty config). */

export const TRINITY_REWRITE_VIEWER_DIALECT = { artifactKind: "s.trinity.rewrite", standard: "1", subset: "*" } as const;

export const TRINITY_REWRITE_VIEW_MODE_ID = "view" as const;

export * as ruleWindow from "./🎭️modes/👁️view/🪟️windows/📜️rule/🟦️component";
