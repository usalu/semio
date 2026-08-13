/** 🧬️ RasterPresence */
export interface RasterPresenceCamera {
  /** @state presence */
  x: number;
  /** @state presence */
  y: number;
  /** @state presence */
  zoom: number;
}

export interface RasterPresence {
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  hoveredId?: string;
  /** @state presence */
  brushSize: number;
  /** @state presence */
  brushOpacity: number;
  /** @state presence */
  camera: RasterPresenceCamera;
  /** @state presence */
  activeUtilityId: string;
}
