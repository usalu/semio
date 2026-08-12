/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️component.ts for the canonical facet
 * schema; this file describes the DIFF PACK-BINARY ENCODING header of the same shape. */
export interface SemioTextDiffFrameHeader {
  format: number;
  presence: number;
  payload: string;
}
