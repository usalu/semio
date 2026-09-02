export interface DwgStructure { layerCount: number; entityCount: number; geometryValueCount: number; geometryIndexCount: number; textCharacterCount: number; codepage: number; version: string; }
export interface DwgInference { structure: DwgStructure; }
