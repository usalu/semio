/** 🧬️ change-jfif-header direct payload. */
import type { JfifDensityUnits, JfifThumbnail } from '../../📸️snapshot/🟦️component.ts';
export interface ChangeJfifHeaderMutation {
  readonly version: [number, number];
  readonly densityUnits: JfifDensityUnits;
  readonly xDensity: number;
  readonly yDensity: number;
  readonly thumbnail?: JfifThumbnail | null;
}
