// stdio.mp4 snapshot binary facet — same shape as ../🟦️component.ts.
/** 🧬️ Mp4Snapshot — ISO-BMFF: ftyp typed, decoded per-track sample tables, everything else
 * typed-raw retained. Mirrors 🦀️component.rs field-for-field. */
export interface Mp4Ftyp {
  majorBrand: string;
  minorVersion: number;
  compatibleBrands: string[];
}
export type Mp4Codec =
  | { codec: "avc"; sps: number[][]; pps: number[][]; nalLengthSize: number }
  | { codec: "other"; fourcc: string; raw: number[] };
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
  samples: Mp4Sample[];
}
export interface Mp4Box {
  fourcc: string;
  data: number[];
}
export interface Mp4Snapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ ftyp: Mp4Ftyp;
  /** @state persistent */ tracks: Mp4Track[];
  /** @state persistent */ unknownBoxes: Mp4Box[];
}
