/** 🧬️ DrawingPresence */
export interface DrawingPresence {
  /** @state presence */
  engagementInput: string;
  /** @state presence */
  camera: DrawingCamera;
  /** @state presence */
  activeUtilityId: string;
}
export interface DrawingCamera { x: number; y: number; zoom: number; }
