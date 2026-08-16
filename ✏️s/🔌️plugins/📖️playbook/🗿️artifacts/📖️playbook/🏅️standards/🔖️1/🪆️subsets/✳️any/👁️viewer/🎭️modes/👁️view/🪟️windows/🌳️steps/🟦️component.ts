/** 🌳️ Playbook viewer — Steps window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror
 * of the same step/block nesting the editor's Builder window edits — no mutation-shaped fields (no
 * palette, no per-kind form fields), matching the viewer's `ViewEmit`-only contract. */

export interface PlaybookViewStepNode {
  id: string;
  label: string;
  children: PlaybookViewStepNode[];
}

/** 👁️ The Steps window's typed view-model — the TS mirror of the Rust `render()` boundary's inputs
 * (a bare document snapshot: a viewer has no runtime/config/utility state). */
export interface PlaybookViewStepsViewModel {
  windowKindId: "playbook-view-steps";
  bodyKey: "playbook.view.steps";
  roots: PlaybookViewStepNode[];
}

export const PLAYBOOK_VIEW_WINDOW_STEPS = "playbook-view-steps" as const;
export const PLAYBOOK_VIEW_BODY_STEPS = "playbook.view.steps" as const;
