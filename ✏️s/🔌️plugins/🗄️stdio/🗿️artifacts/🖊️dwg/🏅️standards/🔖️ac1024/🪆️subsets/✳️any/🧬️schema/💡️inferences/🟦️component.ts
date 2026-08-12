/** 💡️ Dwg (ac1024) inference schema — structural byte/section/page statistics over the real D1/D2
 * section-page decode, richer than ac1018's (no geometric entities are decoded at this
 * standard). */

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

export interface DwgInference {
  /** @state inferred */
  structure: DwgStructure;
}
