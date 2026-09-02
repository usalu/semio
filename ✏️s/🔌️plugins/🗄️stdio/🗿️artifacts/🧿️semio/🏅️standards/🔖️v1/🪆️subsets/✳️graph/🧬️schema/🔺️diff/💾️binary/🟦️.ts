/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the DIFF PACK-BINARY ENCODING of the same shape. */
export interface SemioGraphDiffFrameHeader {
  format: number;
  presence: number;
  payload: string;
}
