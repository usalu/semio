/** 🗂 `structure` — the dwg (ac1018) snapshot's honest structural byte/section statistics (no
 * geometric entities are decoded at this standard). */

export interface DwgStructure {
  byteCount: number;
  sectionCount: number;
  codepage: number;
  version: string;
}
