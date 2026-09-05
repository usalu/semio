/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the OP PACK-BINARY ENCODING header of the mutation dispatch shape. */
export interface SemioTextMutationOpFrameHeader {
  format: number;
  tag: number;
  payload: string;
}
