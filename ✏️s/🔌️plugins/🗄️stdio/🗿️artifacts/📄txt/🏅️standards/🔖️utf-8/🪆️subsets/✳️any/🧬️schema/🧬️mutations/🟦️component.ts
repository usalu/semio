import type { LineEnding, TxtSnapshot } from '../📸️snapshot/🟦️component.ts';

/** 🧬️ TxtMutation union. */
export type TxtMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: TxtSnapshot }
  | { mutation: 'setTrailingNewline'; value: boolean }
  | { mutation: 'setLineEnding'; value: LineEnding }
  | { mutation: 'insertLine'; index: number; text: string }
  | { mutation: 'removeLine'; index: number }
  | { mutation: 'setLine'; index: number; text: string };
