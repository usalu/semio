// stdio.avi mutations 💾️binary facet — same shape as ../🟦️.ts.
/** 🧬️ AviMutation — named-variant vocabulary. Mirrors 🦀️.rs field-for-field. */
export type AviMutation =
  | { mutation: "setSnapshot"; snapshot: import("./🟦️").AviSnapshot }
  | { mutation: "setMainHeader"; mainHeader: import("./🟦️").AviMainHeader }
  | { mutation: "setIdx1Present"; idx1Present: boolean }
  | { mutation: "insertStream"; index: number; stream: import("./🟦️").AviStream }
  | { mutation: "removeStream"; index: number }
  | { mutation: "setStreamHeader"; streamIndex: number; strh: import("./🟦️").AviStreamHeader }
  | { mutation: "setStreamFormat"; streamIndex: number; strf: import("./🟦️").AviStreamFormat }
  | { mutation: "insertChunk"; streamIndex: number; index: number; chunk: import("./🟦️").AviChunk }
  | { mutation: "removeChunk"; streamIndex: number; index: number }
  | { mutation: "setChunkKeyframe"; streamIndex: number; index: number; keyframe: boolean }
  | { mutation: "addUnknownChunk"; index: number; item: import("./🟦️").RiffChunk }
  | { mutation: "removeUnknownChunk"; index: number };
