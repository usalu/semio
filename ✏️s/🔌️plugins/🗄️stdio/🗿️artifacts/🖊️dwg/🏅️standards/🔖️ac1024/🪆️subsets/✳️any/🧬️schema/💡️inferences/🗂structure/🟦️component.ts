/** 🗂 `structure` — the dwg (ac1024) snapshot's honest structural byte/section/page statistics
 * over the real D1/D2 section-page decode (no geometric entities are decoded at this standard). */

export interface DwgStructure {
  byteCount: number;
  sectionCount: number;
  pageCount: number;
  decodedPageCount: number;
  errorPageCount: number;
  declaredTotalSize: number;
  codepage: number;
  version: string;
}
