/** 🧬️ Puzzle2d diff schema — sparse field delta. */

export interface Puzzle2dDiff {
  /** @state artifact */
  artifact?: Puzzle2dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  camera?: Puzzle2dCamera;
  /** @state artifact */
  nodes?: Puzzle2dNodesDelta;
  /** @state artifact */
  edges?: Puzzle2dEdgesDelta;
  /** @state artifact */
  meta?: Puzzle2dMeta;
  /** @state presence */
  selectedIds?: Puzzle2dStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  selectionMethod?: string;
  /** @state config */
  gridSnapEnabled?: boolean;
  /** @state config */
  gridFactor?: number;
  /** @state config */
  suggestionOffset?: number;
  /** @state config */
  fillCount?: number;
  /** @state config */
  brushCandidateIndex?: number;
  /** @state config */
  brushCandidateSourceHandleId?: string;
  /** @state config */
  locale?: string;
  /** @state config */
  terminology?: string;
  /** @state config */
  lodModeByPaneJson?: string;
  /** @state config */
  engagementInputByPaneJson?: string;
  /** @state config */
  brushCandidatesJson?: string;
  /** @state config */
  nodeKindWeightsJson?: string;
  /** @state config */
  handleKindWeightsJson?: string;
  /** @state config */
  activeUtilityByWindowIdJson?: string;
  /** @state artifact */
  hoveredNodeId?: string | null;
  /** @state artifact */
  previewSeq?: number;
}

export interface Puzzle2dStringList { values: string[]; }
export interface Puzzle2dNodesDelta { added: Puzzle2dNode[]; removed: string[]; patched: Puzzle2dNodePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dNodePatchEntry { id: string; patch: Puzzle2dNodePatch; }
export interface Puzzle2dNodePatch { replacement?: Puzzle2dNode; }
export interface Puzzle2dEdgesDelta { added: Puzzle2dEdge[]; removed: string[]; patched: Puzzle2dEdgePatchEntry[]; reordered?: string[]; }
export interface Puzzle2dEdgePatchEntry { id: string; patch: Puzzle2dEdgePatch; }
export interface Puzzle2dEdgePatch { replacement?: Puzzle2dEdge; }
export interface Puzzle2dArtifact { schema: string; [key: string]: unknown; }

export type Puzzle2dNodeAnchor = "fixed" | "derived";

export interface Puzzle2dCamera {
  x: number;
  y: number;
  zoom: number;
}

export interface Puzzle2dHandle {
  id: string;
  handleKind?: string;
  angle: number;
  radius?: number;
  color?: string;
  iconKind?: string;
  scale?: number;
  visible?: boolean;
  locked?: boolean;
}

export interface Puzzle2dNode {
  id: string;
  nodeKind?: string;
  shape?: string;
  x: number;
  y: number;
  radius?: number;
  width?: number;
  height?: number;
  text?: string;
  iconKind?: string;
  root?: boolean;
  scale?: number;
  visible?: boolean;
  locked?: boolean;
  anchor: Puzzle2dNodeAnchor;
  handles: Puzzle2dHandle[];
}

export interface Puzzle2dEdge {
  id: string;
  source: string;
  target: string;
  edgeKind?: string;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
  sourceTip?: string;
  targetTip?: string;
  visible?: boolean;
  locked?: boolean;
}

export type Puzzle2dCompatSpecificity = "general" | "node" | "edge" | "handle" | "wire" | "vortex";

export interface Puzzle2dKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle2dCompatSpecificity;
}

export interface Puzzle2dAttribute {
  id: string;
  key: string;
  value: string;
  definition?: string;
}

export interface Puzzle2dAuthor {
  id: string;
  name: string;
  email: string;
  role?: string;
  rank?: number;
}

export interface Puzzle2dRepresentation {
  id: string;
  name: string;
  url: string;
  mime: string;
  tags: string[];
  lod?: string;
  description: string;
}

export interface Puzzle2dHandleTemplate {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  handleKind?: string;
  angle: number;
  t?: number;
  mandatory?: boolean;
  radius?: number;
}

export interface Puzzle2dCatalogNodeKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  image: string;
  unit: string;
  abstract: boolean;
  baseKinds: string[];
  representations: Puzzle2dRepresentation[];
  handles: Puzzle2dHandleTemplate[];
  attributes: Puzzle2dAttribute[];
  authors: Puzzle2dAuthor[];
}

export interface Puzzle2dCatalogHandleKind {
  id: string;
  code?: string;
  label?: string;
  order?: number;
  compatibleWith: string[];
  description: string;
  icon: string;
  color: string;
  defaultWireKind: string;
}

export interface Puzzle2dCatalogEdgeKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  color: string;
}

export interface Puzzle2dCatalogWireKind {
  id: string;
  name: string;
  label: string;
  description: string;
  icon: string;
  color: string;
  defaultEdgeKind: string;
}

export interface Puzzle2dKindCatalogs {
  nodes: Puzzle2dCatalogNodeKind[];
  handles: Puzzle2dCatalogHandleKind[];
  edges: Puzzle2dCatalogEdgeKind[];
  wires: Puzzle2dCatalogWireKind[];
}

export interface Puzzle2dMeta {
  manifestId?: string;
  kindCompatibility: Puzzle2dKindCompatibility[];
  kindCatalogs?: Puzzle2dKindCatalogs;
}

