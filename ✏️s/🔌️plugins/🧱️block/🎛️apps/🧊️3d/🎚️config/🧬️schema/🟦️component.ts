/** 🧬️ Block3dConfig */
export interface Block3dConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  activeRepresentationId?: string;
  /** @state local-ui */
  wantedTags: string[];
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  windows: Block3dWindowView[];
  /** @state local-ui */
  brushVortexKindId?: string;
  /** @state local-ui */
  brushRadius: number;
  /** @state local-ui */
  brushFlip: boolean;
  /** @state local-ui */
  brushPreview?: Block3dBrushPreview;
  /** @state local-ui */
  camera?: BlockCamera3d;
  /** @state local-ui */
  hoveredVortexFullId?: string;
}

export interface Block3dWindowView { [key: string]: unknown; }

export interface Block3dBrushPreview { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }
