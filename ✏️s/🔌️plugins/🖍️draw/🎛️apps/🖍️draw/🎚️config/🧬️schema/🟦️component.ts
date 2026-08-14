/** 🧬️ DrawConfig */
export interface DrawConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: DrawCamera;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  locale: string;
}
export interface DrawCamera { x: number; y: number; zoom: number; }
