/** 🧬️ Mp4Mutation — named-variant vocabulary. Mirrors 🦀️.rs field-for-field. */
export type Mp4Mutation =
  | { mutation: "setSnapshot"; snapshot: import("../📸️snapshot/🟦️").Mp4Snapshot }
  | { mutation: "setFtyp"; ftyp: import("../📸️snapshot/🟦️").Mp4Ftyp }
  | { mutation: "insertTrack"; index: number; track: import("../📸️snapshot/🟦️").Mp4Track }
  | { mutation: "removeTrack"; index: number }
  | { mutation: "setTrackDimensions"; trackIndex: number; width: number; height: number }
  | { mutation: "setTrackCodec"; trackIndex: number; codec: import("../📸️snapshot/🟦️").Mp4Codec }
  | { mutation: "insertSample"; trackIndex: number; index: number; sample: import("../📸️snapshot/🟦️").Mp4Sample }
  | { mutation: "removeSample"; trackIndex: number; index: number }
  | { mutation: "setSampleSync"; trackIndex: number; index: number; sync: boolean };
