/** 🧬️ insert-ifd direct payload. */
import type { TiffIfd } from '../../📸️snapshot/🟦️component.ts';
export interface InsertIfdMutation {
  readonly index: number;
  readonly ifd: TiffIfd;
}
