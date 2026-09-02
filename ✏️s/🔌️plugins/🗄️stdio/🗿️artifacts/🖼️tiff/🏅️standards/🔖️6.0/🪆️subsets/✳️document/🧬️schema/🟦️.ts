/** 🧬️ TiffArtifact schema facet — full artifact state, mirrors TiffSnapshot field-for-field
 * (see ./📸️snapshot/🟦️.ts for the supporting types). */
import type { TiffByteOrder, TiffIfd } from './📸️snapshot/🟦️.ts';

export interface TiffArtifact {
  schema: string;
  byteOrder: TiffByteOrder;
  ifds: TiffIfd[];
  pixels: number[];
}
