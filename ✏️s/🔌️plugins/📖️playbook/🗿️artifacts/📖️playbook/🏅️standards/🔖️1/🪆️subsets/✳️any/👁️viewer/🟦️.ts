/** 👁️ Playbook viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no config schema beyond the framework's own empty config). */

export const PLAYBOOK_VIEWER_DIALECT = { artifactKind: "s.playbook.playbook", standard: "1", subset: "*" } as const;

export const PLAYBOOK_VIEW_MODE_VIEW = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🌳️steps/🟦️component";
