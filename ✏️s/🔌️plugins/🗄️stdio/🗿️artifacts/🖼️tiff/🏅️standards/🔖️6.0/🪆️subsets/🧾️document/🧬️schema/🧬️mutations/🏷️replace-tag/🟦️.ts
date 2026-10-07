/** 🧬️ replace-tag direct payload. */
import type { TiffValues } from '../../📸️snapshot/🟦️.ts';
export interface ReplaceTagMutation {
  readonly ifdIndex: number;
  readonly tag: number;
  readonly values: TiffValues;
}
