// stdio.avi snapshot binary facet — same shape as ../🟦️component.ts.
/** 🧬️ AviSnapshot — RIFF/AVI 1.0. Mirrors 🦀️component.rs field-for-field. */
export interface AviMainHeader {
  microSecPerFrame: number; maxBytesPerSec: number; paddingGranularity: number; flags: number;
  totalFrames: number; initialFrames: number; streams: number; suggestedBufferSize: number;
  width: number; height: number; reserved: number[];
}
export interface AviStreamHeader {
  fccType: string; fccHandler: string; flags: number; priority: number; language: number;
  initialFrames: number; scale: number; rate: number; start: number; length: number;
  suggestedBufferSize: number; quality: number; sampleSize: number;
  rcFrameLeft: number; rcFrameTop: number; rcFrameRight: number; rcFrameBottom: number;
}
export type AviStreamFormat =
  | { format: "bitmapInfo"; size: number; width: number; height: number; planes: number; bitCount: number; compression: string; sizeImage: number; xPelsPerMeter: number; yPelsPerMeter: number; colorsUsed: number; colorsImportant: number }
  | { format: "waveFormat"; formatTag: number; channels: number; samplesPerSec: number; avgBytesPerSec: number; blockAlign: number; bitsPerSample: number; extra: number[] }
  | { format: "raw"; data: number[] };
export interface AviChunk { fourcc: string; data: number[]; keyframe: boolean; }
export interface AviStream { strh: AviStreamHeader; strf: AviStreamFormat; chunks: AviChunk[]; }
export interface RiffChunk { fourcc: string; data: number[]; }
export interface AviSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ mainHeader: AviMainHeader;
  /** @state artifact */ streams: AviStream[];
  /** @state artifact */ idx1Present: boolean;
  /** @state artifact */ unknownChunks: RiffChunk[];
}
