/** 🧬️ Owned JPEG image content and metadata schema. */


export type JfifDensityUnits = 'aspect' | 'pixelsPerInch' | 'pixelsPerCm';

export interface JfifThumbnail {
  width: number;
  height: number;
  rgbData: number[];
}

export interface JpgSegment {
  marker: number;
  data: number[];
}

export interface JpgImage {
  width: number;
  height: number;
  pixels: number[];
  jfifVersion: [number, number];
  jfifDensityUnits: JfifDensityUnits;
  jfifXDensity: number;
  jfifYDensity: number;
  jfifThumbnail?: JfifThumbnail;
  otherSegments: JpgSegment[];
}
export interface JpgSnapshot {schema:string;image:JpgImage}
