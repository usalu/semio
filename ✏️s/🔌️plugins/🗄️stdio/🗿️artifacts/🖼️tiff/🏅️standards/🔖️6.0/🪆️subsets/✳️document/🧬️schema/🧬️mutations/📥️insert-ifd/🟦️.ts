/** 🧬️ insert-ifd direct payload. */
import type { TiffIfd } from '../../📸️snapshot/🟦️.ts';
export interface InsertIfdMutation {
  readonly index: number;
  readonly ifd: TiffIfd;
}
