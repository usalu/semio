/** 🧬️ change-header direct payload. */
import type { PngColorType } from '../../📸️snapshot/🟦️component.ts';
export interface ChangeHeaderMutation {
  readonly width: number;
  readonly height: number;
  readonly bitDepth: number;
  readonly colorType: PngColorType;
  readonly interlace: boolean;
}
