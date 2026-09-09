/** 🧬️ Block3dConfig */
export interface Block3dConfig {
  /** @state config */
  activeRepresentationId?: string;
  /** @state config */
  wantedTags: string[];
  /** @state config */
  /** @state config */
  windows: Block3dWindowView[];
  /** @state config */
  brushVortexKindId?: string;
  /** @state config */
  brushRadius: number;
  /** @state config */
  brushFlip: boolean;
  /** @state config */
  camera?: BlockCamera3d;
}

export interface Block3dWindowView { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }
