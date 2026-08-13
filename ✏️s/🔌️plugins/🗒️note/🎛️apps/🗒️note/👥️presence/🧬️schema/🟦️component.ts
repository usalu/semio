/** 🧬️ NotePresence */
export interface NotePresence {
  /** @state presence */
  selectedBlockIds: string[];
  /** @state presence */
  cameraX: number;
  /** @state presence */
  cameraY: number;
  /** @state presence */
  cameraZoom: number;
  /** @state presence */
  hoveredBlockId?: string;
  /** @state presence */
  activeUtilityId: string;
}
