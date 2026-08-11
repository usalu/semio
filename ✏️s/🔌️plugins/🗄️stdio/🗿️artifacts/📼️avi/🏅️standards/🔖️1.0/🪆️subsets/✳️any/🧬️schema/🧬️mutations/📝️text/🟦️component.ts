// stdio.avi mutations 📝️text facet — same shape as ../🟦️component.ts.
/** 🧬️ AviMutation — named-variant vocabulary. Mirrors 🦀️component.rs field-for-field. */
export type AviMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: import("../📸️snapshot/🟦️component").AviSnapshot }
  | { mutation: "setMainHeader"; mainHeader: import("../📸️snapshot/🟦️component").AviMainHeader }
  | { mutation: "setIdx1Present"; idx1Present: boolean }
  | { mutation: "insertStream"; index: number; stream: import("../📸️snapshot/🟦️component").AviStream }
  | { mutation: "removeStream"; index: number }
  | { mutation: "setStreamHeader"; streamIndex: number; strh: import("../📸️snapshot/🟦️component").AviStreamHeader }
  | { mutation: "setStreamFormat"; streamIndex: number; strf: import("../📸️snapshot/🟦️component").AviStreamFormat }
  | { mutation: "insertChunk"; streamIndex: number; index: number; chunk: import("../📸️snapshot/🟦️component").AviChunk }
  | { mutation: "removeChunk"; streamIndex: number; index: number }
  | { mutation: "setChunkKeyframe"; streamIndex: number; index: number; keyframe: boolean }
  | { mutation: "addUnknownChunk"; index: number; item: import("../📸️snapshot/🟦️component").RiffChunk }
  | { mutation: "removeUnknownChunk"; index: number };
