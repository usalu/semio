/** 💡️ Dwg (ac1024) inference schema over logical drawing concepts. */

export interface DwgStructure {
  layerCount: number;
  entityCount: number;
  geometryValueCount: number;
  geometryIndexCount: number;
  textCharacterCount: number;
  codepage: number;
  version: string;
}

export interface DwgInference {
  /** @derived */
  structure: DwgStructure;
}
