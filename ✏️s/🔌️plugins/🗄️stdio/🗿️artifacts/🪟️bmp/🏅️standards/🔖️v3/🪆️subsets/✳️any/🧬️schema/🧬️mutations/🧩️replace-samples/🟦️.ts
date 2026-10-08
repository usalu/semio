import type { BmpNativeSample, BmpRegion } from '../../📸️snapshot/🟦️.ts';

export interface ReplaceSamples {
  readonly region: BmpRegion;
  readonly indices?: readonly number[];
  readonly samples?: readonly BmpNativeSample[];
}
