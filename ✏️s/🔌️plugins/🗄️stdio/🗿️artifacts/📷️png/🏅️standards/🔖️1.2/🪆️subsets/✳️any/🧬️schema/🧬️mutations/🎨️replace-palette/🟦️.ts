/** 🧬️ replace-palette direct payload. */
import type { PngRgb } from '../../📸️snapshot/🟦️.ts';
export interface ReplacePaletteMutation {
  readonly plte?: ReadonlyArray<PngRgb> | null;
}
