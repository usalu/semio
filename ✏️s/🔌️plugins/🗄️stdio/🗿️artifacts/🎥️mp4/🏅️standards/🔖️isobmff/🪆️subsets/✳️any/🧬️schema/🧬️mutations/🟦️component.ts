/** 🧬️ Mp4Mutation — named-variant vocabulary. Mirrors 🦀️component.rs field-for-field. */
export type Mp4Mutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: import("../📸️snapshot/🟦️component").Mp4Snapshot }
  | { mutation: "setFtyp"; ftyp: import("../📸️snapshot/🟦️component").Mp4Ftyp }
  | { mutation: "insertTrack"; index: number; track: import("../📸️snapshot/🟦️component").Mp4Track }
  | { mutation: "removeTrack"; index: number }
  | { mutation: "setTrackDimensions"; trackIndex: number; width: number; height: number }
  | { mutation: "setTrackCodec"; trackIndex: number; codec: import("../📸️snapshot/🟦️component").Mp4Codec }
  | { mutation: "insertSample"; trackIndex: number; index: number; sample: import("../📸️snapshot/🟦️component").Mp4Sample }
  | { mutation: "removeSample"; trackIndex: number; index: number }
  | { mutation: "setSampleSync"; trackIndex: number; index: number; sync: boolean };
