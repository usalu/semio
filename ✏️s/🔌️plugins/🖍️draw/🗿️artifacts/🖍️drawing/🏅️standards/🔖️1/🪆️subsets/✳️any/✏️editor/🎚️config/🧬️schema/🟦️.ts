/** 🧬️ DrawingConfig */
export interface DrawingConfig {
  /** @state config */
  engagementInput: string;
  /** @state config */
  camera: DrawingCamera;
  /** @state config */
  activeUtilityId: string;
  /** @state config */
  tracePointerGeneration: number;
  /** @state config */
  tracePointerCompletedWork: number;
  /** @state config */
  tracePointerPendingWork: number;
  /** @state config */
  /** @state config */
  locale: string;
}
export interface DrawingCamera { x: number; y: number; zoom: number; }
