// stdio.avi mutations 💾️binary facet — same shape as ../🟦️.ts.
/** 🧬️ AviMutation — named-variant vocabulary. Mirrors 🦀️.rs field-for-field. */
export type AviMutation =
  | { mutation: "setMainHeader"; mainHeader: import("../📸️snapshot/🟦️.ts").AviMainHeader }
  | { mutation: "setIdx1Present"; idx1Present: boolean }
  | { mutation: "insertStream"; index: number; stream: import("../📸️snapshot/🟦️.ts").AviStream }
  | { mutation: "removeStream"; index: number }
  | { mutation: "setStreamHeader"; streamIndex: number; strh: import("../📸️snapshot/🟦️.ts").AviStreamHeader }
  | { mutation: "setStreamFormat"; streamIndex: number; strf: import("../📸️snapshot/🟦️.ts").AviStreamFormat }
  | { mutation: "insertChunk"; streamIndex: number; index: number; chunk: import("../📸️snapshot/🟦️.ts").AviChunk }
  | { mutation: "removeChunk"; streamIndex: number; index: number }
  | { mutation: "setChunkKeyframe"; streamIndex: number; index: number; keyframe: boolean }
  | { mutation: "addUnknownChunk"; index: number; item: import("../📸️snapshot/🟦️.ts").RiffChunk }
  | { mutation: "removeUnknownChunk"; index: number };
