import type { BinarySnapshot } from '../📸️snapshot/🟦️component.ts';

/** 🧬️ BinaryMutation union. */
export type BinaryMutation =
  | { mutation: 'setSnapshot'; snapshot: BinarySnapshot }
  | { mutation: 'splice'; offset: number; removeLen: number; insert: number[] }
  | { mutation: 'appendBytes'; data: number[] }
  | { mutation: 'truncateAt'; offset: number };
