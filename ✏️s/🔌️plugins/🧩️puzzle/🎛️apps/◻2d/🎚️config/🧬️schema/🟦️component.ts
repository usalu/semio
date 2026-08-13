/** 🧬️ Puzzle2dConfig */
export interface Puzzle2dConfig {
  /** @state config */
  selectedIds: string[];
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
  selectionMethod: string;
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
}
