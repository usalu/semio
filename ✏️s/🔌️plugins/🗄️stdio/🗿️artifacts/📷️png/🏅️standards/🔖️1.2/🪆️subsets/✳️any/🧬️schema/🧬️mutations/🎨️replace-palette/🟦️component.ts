/** 🧬️ replace-palette direct payload. */
import type { PngRgb } from '../../📸️snapshot/🟦️component.ts';
export interface ReplacePaletteMutation {
  readonly plte?: ReadonlyArray<PngRgb> | null;
}
