/** 🧬️ Puzzle2d diff schema — sparse field delta. */

export interface Puzzle2dDiff {
  /** @state persistent */
  artifact?: Puzzle2dArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  camera?: Puzzle2dCamera;
  /** @state persistent */
  nodes?: Puzzle2dNodesDelta;
  /** @state persistent */
  edges?: Puzzle2dEdgesDelta;
  /** @state persistent */
  meta?: Puzzle2dMeta;
  /** @state shared-ui */
  selectedIds?: Puzzle2dStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  selectionMethod?: string;
  /** @state local-ui */
  gridSnapEnabled?: boolean;
  /** @state local-ui */
  gridFactor?: number;
  /** @state local-ui */
  suggestionOffset?: number;
  /** @state local-ui */
  fillCount?: number;
  /** @state local-ui */
  brushCandidateIndex?: number;
  /** @state local-ui */
  brushCandidateSourceHandleId?: string;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  terminology?: string;
  /** @state local-ui */
  lodModeByPaneJson?: string;
  /** @state local-ui */
  engagementInputByPaneJson?: string;
  /** @state local-ui */
  brushCandidatesJson?: string;
  /** @state local-ui */
  nodeKindWeightsJson?: string;
  /** @state local-ui */
  handleKindWeightsJson?: string;
  /** @state local-ui */
  activeUtilityByWindowIdJson?: string;
  /** @state preview */
  hoveredNodeId?: string | null;
  /** @state preview */
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

