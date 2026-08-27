/** 🧬️ insert-palette-entry direct payload. */
import type { BmpPaletteEntry } from '../../📸️snapshot/🟦️component.ts';
export interface InsertPaletteEntryMutation {
  readonly index: number;
  readonly entry: BmpPaletteEntry;
}
