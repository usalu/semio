/** 🧬️ NoteConfig */
export interface NoteConfig {
  /** @state local-ui */
  selectedBlockIds: string[];
  /** @state local-ui */
  hoveredBlockId?: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  camera: NoteCamera;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
}

export interface NoteCamera {
  x: number;
  y: number;
  zoom: number;
}
