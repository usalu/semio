/** 💡️ Dxf inference schema — entity-derived 3D bounding box over top-level and block-nested
 * entities. */

export interface DxfBounds {
  min: [number, number, number];
  max: [number, number, number];
  entityCount: number;
}

export interface DxfInference {
  /** @derived */
  bounds: DxfBounds;
}
