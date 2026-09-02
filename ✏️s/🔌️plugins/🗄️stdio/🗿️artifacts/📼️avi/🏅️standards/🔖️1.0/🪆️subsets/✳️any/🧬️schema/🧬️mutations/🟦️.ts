/** 🧬️ AviMutation — named-variant vocabulary. Mirrors 🦀️.rs field-for-field. */
export type AviMutation =
  | { mutation: "setSnapshot"; snapshot: import("../📸️snapshot/🟦️").AviSnapshot }
  | { mutation: "setMainHeader"; mainHeader: import("../📸️snapshot/🟦️").AviMainHeader }
  | { mutation: "setIdx1Present"; idx1Present: boolean }
  | { mutation: "insertStream"; index: number; stream: import("../📸️snapshot/🟦️").AviStream }
  | { mutation: "removeStream"; index: number }
  | { mutation: "setStreamHeader"; streamIndex: number; strh: import("../📸️snapshot/🟦️").AviStreamHeader }
  | { mutation: "setStreamFormat"; streamIndex: number; strf: import("../📸️snapshot/🟦️").AviStreamFormat }
  | { mutation: "insertChunk"; streamIndex: number; index: number; chunk: import("../📸️snapshot/🟦️").AviChunk }
  | { mutation: "removeChunk"; streamIndex: number; index: number }
  | { mutation: "setChunkKeyframe"; streamIndex: number; index: number; keyframe: boolean }
  | { mutation: "addUnknownChunk"; index: number; item: import("../📸️snapshot/🟦️").RiffChunk }
  | { mutation: "removeUnknownChunk"; index: number };
