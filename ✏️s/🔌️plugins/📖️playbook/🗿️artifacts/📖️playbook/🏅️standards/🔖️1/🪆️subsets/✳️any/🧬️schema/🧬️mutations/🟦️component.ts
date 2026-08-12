/** 🧬️ PlaybookMutation union — one variant per 🧬️mutations/<slug> triad leaf. */
export interface PlaybookStepShape {
  id: string;
  title: string;
  description?: string;
  blocks: PlaybookBlockShape[];
}

/** 🧱 Mirrors the framework kernel's `PlaybookBlock` — kind-dependent optional fields collapsed to
 * an index signature here since this facet only needs the addressing shape, not full validation. */
export interface PlaybookBlockShape {
  id: string;
  label: string;
  kind: string;
  [field: string]: unknown;
}

export type PlaybookMutation =
  | { mutation: 'addStep'; step: PlaybookStepShape; index?: number }
  | { mutation: 'removeStep'; stepId: string }
  | { mutation: 'moveStep'; stepId: string; index: number }
  | { mutation: 'addBlock'; stepId: string; block: PlaybookBlockShape; index?: number }
  | { mutation: 'removeBlock'; stepId: string; blockId: string }
  | { mutation: 'moveBlock'; blockId: string; fromStepId: string; toStepId: string; index: number }
  | { mutation: 'replaceBlock'; stepId: string; block: PlaybookBlockShape }
  | { mutation: 'updateStep'; stepId: string; title: string; description?: string }
  | { mutation: 'changeTitle'; newTitle?: string };
