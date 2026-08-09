/** 🧬️ DrawPresence */
export interface DrawPresence {
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  hoveredId?: string;
  /** @state shared-ui */
  engagementInput: string;
  /** @state shared-ui */
  camera: DrawCamera;
  /** @state shared-ui */
  activeUtilityId: string;
}
export interface DrawCamera { x: number; y: number; zoom: number; }
