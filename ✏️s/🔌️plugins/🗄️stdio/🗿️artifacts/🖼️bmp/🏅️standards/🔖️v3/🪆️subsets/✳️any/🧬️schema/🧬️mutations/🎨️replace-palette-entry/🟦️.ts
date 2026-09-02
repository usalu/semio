/** 🧬️ replace-palette-entry direct payload. */
import type { BmpPaletteEntry } from '../../📸️snapshot/🟦️.ts';
export interface ReplacePaletteEntryMutation {
  readonly index: number;
  readonly entry: BmpPaletteEntry;
}
