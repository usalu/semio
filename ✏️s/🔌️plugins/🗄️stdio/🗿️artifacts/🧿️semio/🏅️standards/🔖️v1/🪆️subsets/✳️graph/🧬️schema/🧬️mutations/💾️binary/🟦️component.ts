/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️component.ts for the canonical facet
 * schema; this file describes the OP PACK-BINARY ENCODING of the same shape. */
export interface SemioGraphOpFrameHeader {
  format: number;
  tag: number;
  payload: string;
}
