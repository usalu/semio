/** 🧬️ RasterPresence */
export interface RasterPresenceCamera {
  /** @state shared-ui */
  x: number;
  /** @state shared-ui */
  y: number;
  /** @state shared-ui */
  zoom: number;
}

export interface RasterPresence {
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  hoveredId?: string;
  /** @state shared-ui */
  brushSize: number;
  /** @state shared-ui */
  brushOpacity: number;
  /** @state shared-ui */
  camera: RasterPresenceCamera;
  /** @state shared-ui */
  activeUtilityId: string;
}
