import type { MdBlock, MdInline, MdSnapshot } from '../📸️snapshot/🟦️.ts';
import type { MdPathStep } from '../🔺️diff/🟦️.ts';

/** 🧬️ MdMutation union. Every `path`-carrying variant addresses the CONTAINER (the block
 * sequence -- top level, a block-quote's content, or a list item's content) `index` lives in;
 * `path: []` addresses the top-level document blocks. */
export type MdMutation =
  | { mutation: 'setSnapshot'; snapshot: MdSnapshot }
  | { mutation: 'insertBlock'; path: MdPathStep[]; index: number; block: MdBlock }
  | { mutation: 'removeBlock'; path: MdPathStep[]; index: number }
  | { mutation: 'replaceBlock'; path: MdPathStep[]; index: number; block: MdBlock }
  | { mutation: 'setInlines'; path: MdPathStep[]; index: number; inlines: MdInline[] };
