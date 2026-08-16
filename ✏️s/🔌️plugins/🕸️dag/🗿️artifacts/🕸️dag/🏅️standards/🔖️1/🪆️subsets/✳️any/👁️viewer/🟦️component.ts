/** 👁️ DAG viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no config schema beyond the framework's own empty config). */

export const DAG_VIEWER_DIALECT = { artifactKind: "s.dag.dag", standard: "1", subset: "*" } as const;

export const DAG_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🕸️main/🟦️component";
