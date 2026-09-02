// stdio.mp4 mutations 💾️binary facet — same shape as ../🟦️.ts.
/** 🧬️ Mp4Mutation — named-variant vocabulary. Mirrors 🦀️.rs field-for-field. */
export type Mp4Mutation =
  | { mutation: "setSnapshot"; snapshot: import("./🟦️").Mp4Snapshot }
  | { mutation: "setFtyp"; ftyp: import("./🟦️").Mp4Ftyp }
  | { mutation: "insertTrack"; index: number; track: import("./🟦️").Mp4Track }
  | { mutation: "removeTrack"; index: number }
  | { mutation: "setTrackDimensions"; trackIndex: number; width: number; height: number }
  | { mutation: "setTrackCodec"; trackIndex: number; codec: import("./🟦️").Mp4Codec }
  | { mutation: "insertSample"; trackIndex: number; index: number; sample: import("./🟦️").Mp4Sample }
  | { mutation: "removeSample"; trackIndex: number; index: number }
  | { mutation: "setSampleSync"; trackIndex: number; index: number; sync: boolean };
