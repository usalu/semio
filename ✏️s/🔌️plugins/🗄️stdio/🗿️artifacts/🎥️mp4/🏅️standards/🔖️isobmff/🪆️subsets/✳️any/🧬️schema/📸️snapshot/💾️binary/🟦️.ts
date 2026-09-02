// stdio.mp4 snapshot binary facet — same shape as ../🟦️.ts.
/** 🧬️ Logical ISO-BMFF movie model encoded by the shared record protocol. */
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
