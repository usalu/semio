/** 💾️ Binary-facet grammar mirror (descriptive) — see ../🟦️.ts for the canonical facet
 * schema; this file describes the PACK-BINARY ENCODING of the same shape. */
export interface SemioGraphSnapshotPackHeader {
  format: number;
  schemaLen: number;
  schemaBytes: string;
  payload: string;
}
