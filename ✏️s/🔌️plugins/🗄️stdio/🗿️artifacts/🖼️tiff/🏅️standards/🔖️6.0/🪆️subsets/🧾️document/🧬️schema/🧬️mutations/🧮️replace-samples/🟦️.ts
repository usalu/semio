import type { TiffWord64 } from '../../📸️snapshot/🟦️.ts';

/** 🧮️ Contiguous run of owned TIFF sample words inside one block of one image page. */
export interface ReplaceSamplesMutation {
  readonly ifdIndex: number;
  readonly block: number;
  readonly offset: number;
  readonly samples: readonly TiffWord64[];
}
