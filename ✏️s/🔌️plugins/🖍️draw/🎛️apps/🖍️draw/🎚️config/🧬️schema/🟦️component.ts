/** 🧬️ DrawConfig */
export interface DrawConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  hoveredId?: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  camera: DrawCamera;
  /** @state local-ui */
  activeUtilityId: string;
  /** @state local-ui */
  locale: string;
}
export interface DrawCamera { x: number; y: number; zoom: number; }
