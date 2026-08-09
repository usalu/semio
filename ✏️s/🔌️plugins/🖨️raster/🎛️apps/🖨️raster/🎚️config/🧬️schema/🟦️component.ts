/** 🧬️ RasterConfig */
export interface RasterCamera {
  /** @state local-ui */
  x: number;
  /** @state local-ui */
  y: number;
  /** @state local-ui */
  zoom: number;
}

export interface RasterConfigViewportSize {
  /** @state local-ui */
  width: number;
  /** @state local-ui */
  height: number;
}

export interface RasterConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  hoveredId?: string;
  /** @state local-ui */
  brushSize: number;
  /** @state local-ui */
  brushOpacity: number;
  /** @state local-ui */
  compositeViewport?: RasterConfigViewportSize;
  /** @state local-ui */
  camera: RasterCamera;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
}
