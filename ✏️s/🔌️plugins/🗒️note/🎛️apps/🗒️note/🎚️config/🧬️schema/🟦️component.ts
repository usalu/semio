/** 🧬️ NoteConfig */
export interface NoteConfig {
  /** @state config */
  selectedBlockIds: string[];
  /** @state config */
  hoveredBlockId?: string;
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: NoteCamera;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}

export interface NoteCamera {
  x: number;
  y: number;
  zoom: number;
}
