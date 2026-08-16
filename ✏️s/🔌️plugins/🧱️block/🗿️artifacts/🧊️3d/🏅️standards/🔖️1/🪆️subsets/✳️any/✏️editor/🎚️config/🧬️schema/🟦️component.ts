/** 🧬️ Block3dConfig */
export interface Block3dConfig {
  /** @state config */
  selectedIds: string[];
  /** @state config */
  activeRepresentationId?: string;
  /** @state config */
  wantedTags: string[];
  /** @state config */
  locale: string;
  /** @state config */
  windows: Block3dWindowView[];
  /** @state config */
  brushVortexKindId?: string;
  /** @state config */
  brushRadius: number;
  /** @state config */
  brushFlip: boolean;
  /** @state config */
  brushPreview?: Block3dBrushPreview;
  /** @state config */
  camera?: BlockCamera3d;
  /** @state config */
  hoveredVortexFullId?: string;
}

export interface Block3dWindowView { [key: string]: unknown; }

export interface Block3dBrushPreview { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }
