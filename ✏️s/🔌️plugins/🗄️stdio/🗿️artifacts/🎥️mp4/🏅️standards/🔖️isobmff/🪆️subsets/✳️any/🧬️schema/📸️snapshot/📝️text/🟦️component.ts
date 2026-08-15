// stdio.mp4 snapshot text facet — structured DSL records for the logical movie model.
/** 🧬️ Logical ISO-BMFF movie model. Container syntax is materialized only by native IO. */
export interface Mp4Ftyp {
  majorBrand: string;
  minorVersion: number;
  compatibleBrands: string[];
}
export interface Mp4Codec { sps: number[][]; pps: number[][]; nalLengthSize: number; }
export interface Mp4Sample {
  data: number[];
  duration: number;
  ctsOffset: number;
  sync: boolean;
}
export interface Mp4Track {
  trackId: number;
  timescale: number;
  codec: Mp4Codec;
  width: number;
  height: number;
  chunkSampleCounts: number[];
  samples: Mp4Sample[];
}
export interface Mp4Snapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ ftyp: Mp4Ftyp;
  /** @state artifact */ tracks: Mp4Track[];
}
