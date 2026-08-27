/** 🧬️ replace-tag direct payload. */
import type { TiffFieldType, TiffValues } from '../../📸️snapshot/🟦️component.ts';
export interface ReplaceTagMutation {
  readonly ifdIndex: number;
  readonly tag: number;
  readonly kind: TiffFieldType;
  readonly values: TiffValues;
}
