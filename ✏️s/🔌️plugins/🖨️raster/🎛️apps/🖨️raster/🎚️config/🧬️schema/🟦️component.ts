/** 🧬️ RasterConfig */
export interface RasterCamera {
  /** @state config */
  x: number;
  /** @state config */
  y: number;
  /** @state config */
  zoom: number;
}

export interface RasterConfigViewportSize {
  /** @state config */
  width: number;
  /** @state config */
  height: number;
}

export interface RasterConfig {
  /** @state config */
  selectedIds: string[];
  /** @state config */
  hoveredId?: string;
  /** @state config */
  brushSize: number;
  /** @state config */
  brushOpacity: number;
  /** @state config */
  compositeViewport?: RasterConfigViewportSize;
  /** @state config */
  camera: RasterCamera;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}
