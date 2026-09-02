/** 🧬️ DrawPresence */
export interface DrawPresence {
  /** @state presence */
  engagementInput: string;
  /** @state presence */
  camera: DrawCamera;
  /** @state presence */
  activeUtilityId: string;
}
export interface DrawCamera { x: number; y: number; zoom: number; }
