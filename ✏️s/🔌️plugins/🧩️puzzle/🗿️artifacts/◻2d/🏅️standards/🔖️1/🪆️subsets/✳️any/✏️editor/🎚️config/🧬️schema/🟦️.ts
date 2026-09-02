/** 🧬️ Puzzle2dConfig */
export type Puzzle2dFillLifecycle =
  | "idle"
  | "capturing"
  | "queued"
  | "running"
  | "checkpointReady"
  | "applying"
  | "awaitingAdoption"
  | "closing"
  | "completed"
  | "cancelled"
  | "faulted"
  | "discarded";

export interface Puzzle2dConfig {
  /** @state config */
  cameraX: number;
  /** @state config */
  cameraY: number;
  /** @state config */
  cameraZoom: number;
  /** @state config */
  lodModeByPane: Record<string, string>;
  /** @state config */
  engagementInputByPane: Record<string, string>;
  /** @state config */
  brushCandidateIndex: number;
  /** @state config */
  brushCandidates: unknown[];
  /** @state config */
  brushCandidateSourceHandleId: string;
  /** @state config */
  fillCount: number;
  /** @state config */
  fillJobOperation: number;
  /** @state config */
  fillJobGeneration: number;
  /** @state config */
  fillJobSeed: number;
  /** @state config */
  fillJobBaseRevision: number;
  /** @state config */
  fillJobCheckpointSequence: number;
  /** @state config */
  fillJobAcceptedCount: number;
  /** @state config */
  fillJobSearchCount: number;
  /** @state config */
  fillJobStage: string;
  /** @state config */
  fillJobLifecycle: Puzzle2dFillLifecycle;
  /** @state config */
  fillJobFaultCode?: string;
  /** @state config */
  gridSnapEnabled: boolean;
  /** @state config */
  gridFactor: number;
  /** @state config */
  suggestionOffset: number;
  /** @state config */
  nodeKindWeights: Record<string, number>;
  /** @state config */
  handleKindWeights: Record<string, number>;
  /** @state config */
  activeUtilityByWindowId: Record<string, string>;
  /** @state config */
  locale: string;
  /** @state config */
  terminology: string;
  /** @state config */
  exampleLoadGeneration: number;
  /** @state config */
  exampleLoadId?: string;
}
