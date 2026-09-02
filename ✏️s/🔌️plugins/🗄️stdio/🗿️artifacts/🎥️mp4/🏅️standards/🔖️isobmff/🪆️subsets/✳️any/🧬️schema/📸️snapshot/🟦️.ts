/** 🧬️ Logical ISO-BMFF movie model. Container syntax is materialized only by native IO. */
export interface Mp4Ftyp {
  majorBrand: string;
  minorVersion: number;
  compatibleBrands: string[];
}
export interface Mp4AvcExtension { chromaFormat: number; bitDepthLumaMinus8: number; bitDepthChromaMinus8: number; spsExt: number[][]; }
export interface Mp4Codec { sps: number[][]; pps: number[][]; nalLengthSize: number; extension?: Mp4AvcExtension; }
export interface Mp4Sample {
  data: number[];
  duration: number;
  ctsOffset: number;
  sync: boolean;
}
export interface Mp4Movie {
  creationTime: number; modificationTime: number; timescale: number; duration: number; rate: number; volume: number;
  matrix: number[]; nextTrackId: number; title?: string; encoder?: string;
}
export interface Mp4Edit { segmentDuration: number; mediaTime: number; mediaRateInteger: number; mediaRateFraction: number; }
export interface Mp4VisualSampleEntry {
  dataReferenceIndex: number; version: number; revisionLevel: number; vendor: number; temporalQuality: number; spatialQuality: number;
  horizontalResolution: number; verticalResolution: number; frameCount: number; compressorName: string; depth: number; colorTableId: number;
}
export interface Mp4Color { colorType: string; primaries: number; transfer: number; matrix: number; fullRange?: boolean; }
export interface Mp4PixelAspectRatio { horizontalSpacing: number; verticalSpacing: number; }
export interface Mp4Bitrate { bufferSize: number; maximum: number; average: number; }
export interface Mp4TrackMetadata {
  creationTime: number; modificationTime: number; flags: number; duration: number; layer: number; alternateGroup: number; volume: number; matrix: number[];
  mediaDuration: number; mediaCreationTime: number; mediaModificationTime: number; language: string; quality: number; handlerName: string;
  edits: Mp4Edit[]; visual: Mp4VisualSampleEntry; color?: Mp4Color; pixelAspectRatio?: Mp4PixelAspectRatio; bitrate?: Mp4Bitrate;
}
export interface Mp4Track {
  trackId: number;
  timescale: number;
  codec: Mp4Codec;
  width: number;
  height: number;
  metadata: Mp4TrackMetadata;
  chunkSampleCounts: number[];
  samples: Mp4Sample[];
}
export interface Mp4Snapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ ftyp: Mp4Ftyp;
  /** @state artifact */ movie: Mp4Movie;
  /** @state artifact */ tracks: Mp4Track[];
}
