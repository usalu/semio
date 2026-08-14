/** 🧬️ DwgDiff schema. */
export interface DwgLogicalGeometry { kind: string; values: number[]; indices: number[]; text: string; closed: boolean; }
export interface DwgLogicalLayer { name: string; color: number; }
export interface DwgLogicalEntity { layer: number; color: number; geometry: DwgLogicalGeometry; }
export interface DwgLogicalDrawing { layers: DwgLogicalLayer[]; entities: DwgLogicalEntity[]; extmin: number[]; extmax: number[]; }
export interface DwgDiff {
  version?: string;
  maintenanceVersion?: number;
  codepage?: number;
  drawing?: DwgLogicalDrawing;
}
