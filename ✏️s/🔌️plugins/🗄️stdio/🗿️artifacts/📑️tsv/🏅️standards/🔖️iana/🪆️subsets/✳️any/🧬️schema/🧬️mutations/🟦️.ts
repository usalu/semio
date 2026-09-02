/** 🧬️ TsvMutation union — mirrors 🦀️.rs's `#[serde(tag = "mutation")]` enum. */
import type { TsvSnapshot, TsvLineEnding } from '../📸️snapshot/🟦️.ts';

export type TsvMutation =
  | { mutation: 'setSnapshot'; snapshot: TsvSnapshot }
  | { mutation: 'setTrailingNewline'; trailingNewline: boolean }
  | { mutation: 'setLineEnding'; lineEnding: TsvLineEnding }
  | { mutation: 'insertRow'; index: number; row: string[] }
  | { mutation: 'removeRow'; index: number }
  | { mutation: 'setCell'; rowIndex: number; fieldIndex: number; value: string };
