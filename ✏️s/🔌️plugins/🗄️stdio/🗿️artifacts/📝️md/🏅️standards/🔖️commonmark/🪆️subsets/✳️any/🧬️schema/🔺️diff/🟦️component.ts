import type { MdBlock, MdInline } from '../📸️snapshot/🟦️component.ts';

/** 🔺️ Diff for `stdio.md`. `blocks` is an index-keyed recursive triple over the top-level block
 * sequence -- no `snapshot` full-replace slot. */
export interface MdDiff {
  blocks?: MdBlocksDiff;
}

/** 🌳 Index-keyed, recursive block-sequence triple. Reused verbatim for `list`'s item content
 * and `blockQuote`'s content -- both are `MdBlock[]`. */
export interface MdBlocksDiff {
  removed: number[];
  modified: MdBlockModified[];
  added: MdBlockAdded[];
}

export interface MdBlockModified {
  index: number;
  diff: MdBlockDiff;
}

export interface MdBlockAdded {
  index: number;
  item: MdBlock;
}

/** 🌳 Per-block diff, shaped like the `MdBlock` it targets. `replace` is the kind-change
 * fallback. `MdInline` fields are always whole-value replaced (weak entity), never sub-diffed. */
export type MdBlockDiff =
  | { kind: 'heading'; level?: number; inlines?: MdInline[] }
  | { kind: 'paragraph'; inlines?: MdInline[] }
  | { kind: 'list'; ordered?: boolean; start?: number | null; tight?: boolean; items?: MdListItemsDiff }
  | { kind: 'codeBlock'; info?: string | null; literal?: string }
  | { kind: 'blockQuote'; blocks?: MdBlocksDiff }
  | { kind: 'thematicBreak' }
  | { kind: 'htmlBlock'; raw?: string }
  | { kind: 'replace'; block: MdBlock };

/** 🌳 Index-keyed triple over a `list`'s `items: MdBlock[][]` -- each item's content is diffed
 * with the same recursive `MdBlocksDiff` used everywhere else. */
export interface MdListItemsDiff {
  removed: number[];
  modified: MdListItemModified[];
  added: MdListItemAdded[];
}

export interface MdListItemModified {
  index: number;
  diff: MdBlocksDiff;
}

export interface MdListItemAdded {
  index: number;
  item: MdBlock[];
}

/** 🧭️ One descent step from a block container down into a nested one -- mirrors the Rust
 * `MdPathStep` used by path-carrying mutations (`../🧬️mutations/🟦️component.ts`). */
export type MdPathStep =
  | { step: 'blockQuote'; index: number }
  | { step: 'listItem'; index: number; item: number };
