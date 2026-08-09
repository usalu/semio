/** 🧬️ NotePresence */
export interface NotePresence {
  /** @state shared-ui */
  selectedBlockIds: string[];
  /** @state shared-ui */
  cameraX: number;
  /** @state shared-ui */
  cameraY: number;
  /** @state shared-ui */
  cameraZoom: number;
  /** @state shared-ui */
  hoveredBlockId?: string;
  /** @state shared-ui */
  activeUtilityId: string;
}
