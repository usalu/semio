/** 🧬️ PngArtifact schema facet — full artifact state, mirrors PngSnapshot field-for-field
 * (see ./📸️snapshot/🟦️.ts for the supporting types). */
import type {
  PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims,
  PngRgb, PngSrgbIntent, PngTextChunk, PngTimestamp, PngTransparency,
} from './📸️snapshot/🟦️.ts';

export interface PngArtifact {
  schema: string;
  width: number;
  height: number;
  bitDepth: number;
  colorType: PngColorType;
  interlace: boolean;
  plte?: PngRgb[];
  trns?: PngTransparency;
  gama?: number;
  chrm?: PngChromaticities;
  srgb?: PngSrgbIntent;
  phys?: PngPhysicalDims;
  time?: PngTimestamp;
  bkgd?: PngBackground;
  textChunks: PngTextChunk[];
  pixels: number[];
  chunkOrder: PngChunkMarker[];
  unknownChunks: PngChunk[];
}
