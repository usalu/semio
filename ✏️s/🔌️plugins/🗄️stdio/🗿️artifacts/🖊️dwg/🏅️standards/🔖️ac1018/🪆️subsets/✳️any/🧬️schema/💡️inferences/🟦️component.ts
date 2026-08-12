/** 💡️ Dwg (ac1018) inference schema — structural byte/section statistics over the undecoded raw
 * payload (no geometric entities are decoded at this standard). */

export interface DwgStructure {
  byteCount: number;
  sectionCount: number;
  codepage: number;
  version: string;
}

export interface DwgInference {
  /** @state inferred */
  structure: DwgStructure;
}
