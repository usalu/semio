/** 📐 Logical DWG geometry independent of page, section, and compression encodings. */
export interface DwgLogicalGeometry { kind: string; values: number[]; indices: number[]; text: string; closed: boolean; }
export interface DwgLogicalLayer { name: string; color: number; }
export interface DwgLogicalEntity { layer: number; color: number; geometry: DwgLogicalGeometry; }
export interface DwgLogicalDrawing { layers: DwgLogicalLayer[]; entities: DwgLogicalEntity[]; extmin: number[]; extmax: number[]; }
/** 🧬️ Source-free logical DWG snapshot state. */
export interface DwgSnapshot {
  schema: string;
  version: string;
  maintenanceVersion: number;
  codepage: number;
  drawing: DwgLogicalDrawing;
  sectionNames: string[];
  decodeStatus: string;
}
