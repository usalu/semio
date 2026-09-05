/** 🧬️ change-header-fields direct payload. */
import type { BmpRowOrder } from '../../📸️snapshot/🟦️.ts';
export interface ChangeHeaderFieldsMutation {
  readonly headerSize?: number | null;
  readonly width?: number | null;
  readonly height?: number | null;
  readonly rowOrder?: BmpRowOrder | null;
  readonly planes?: number | null;
  readonly bitsPerPixel?: number | null;
  readonly compression?: number | null;
  readonly imageSize?: number | null;
  readonly xPixelsPerMeter?: number | null;
  readonly yPixelsPerMeter?: number | null;
  readonly colorsUsed?: number | null;
  readonly colorsImportant?: number | null;
}
