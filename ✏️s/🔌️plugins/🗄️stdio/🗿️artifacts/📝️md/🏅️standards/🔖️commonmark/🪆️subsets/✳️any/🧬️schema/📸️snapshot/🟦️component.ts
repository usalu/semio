/** 🧩 A real CommonMark inline node. Weak entity (recipe): whole-value replaced in diffs. */
export type MdInline =
  | { kind: 'text'; text: string }
  | { kind: 'emphasis'; inlines: MdInline[] }
  | { kind: 'strong'; inlines: MdInline[] }
  | { kind: 'code'; literal: string }
  | { kind: 'link'; text: MdInline[]; url: string; title?: string }
  | { kind: 'image'; alt: string; url: string; title?: string }
  | { kind: 'softBreak' }
  | { kind: 'hardBreak' }
  | { kind: 'htmlInline'; raw: string };

/** 🧱 A real CommonMark block. Strong-like entity: block collections are index-keyed and
 * per-field diffed (see `../🔺️diff/🟦️component.ts`). */
export type MdBlock =
  | { kind: 'heading'; level: number; inlines: MdInline[] }
  | { kind: 'paragraph'; inlines: MdInline[] }
  | { kind: 'list'; ordered: boolean; start?: number; tight: boolean; items: MdBlock[][] }
  | { kind: 'codeBlock'; info?: string; literal: string }
  | { kind: 'blockQuote'; blocks: MdBlock[] }
  | { kind: 'thematicBreak' }
  | { kind: 'htmlBlock'; raw: string };

/** 📸️ Persisted `stdio.md` snapshot: the complete top-level block sequence. */
export interface MdSnapshot {
  schema: string;
  blocks: MdBlock[];
}
