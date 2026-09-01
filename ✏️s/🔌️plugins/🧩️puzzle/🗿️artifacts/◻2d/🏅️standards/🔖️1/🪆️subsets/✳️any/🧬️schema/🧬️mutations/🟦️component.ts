/** 🧬️ Puzzle2d semantic mutation dispatch union — field-for-field mirror of `Puzzle2dMutation`. */
import type {
  Puzzle2dCompatSpecificity,
  Puzzle2dHandle,
  Puzzle2dKindCatalogs,
  Puzzle2dNode,
  Puzzle2dNodeAnchor,
} from "../📸️snapshot/🟦️component.ts";

/** 🌱 `create-node` payload. */
export interface CreateNode {
  node: Puzzle2dNode;
  index: number | null;
}

/** 🗑 `delete-node` payload. */
export interface DeleteNode {
  id: string;
}

/** 📍 `move-node` payload. */
export interface MoveNode {
  id: string;
  newX: number;
  newY: number;
}

/** 🧊 `replace-node-geometry` payload. */
export interface ReplaceNodeGeometry {
  id: string;
  newShape: string | null;
  newRadius: number | null;
  newWidth: number | null;
  newHeight: number | null;
}

/** 🏗️ `change-node-kind` payload. */
export interface ChangeNodeKind {
  id: string;
  newNodeKind: string | null;
}

/** ✏️️ `edit-node-text` payload. */
export interface EditNodeText {
  id: string;
  newText: string | null;
}

/** 🎨️ `change-node-icon` payload. */
export interface ChangeNodeIcon {
  id: string;
  newIconKind: string | null;
}

/** 📏️ `scale-node` payload. */
export interface ScaleNode {
  id: string;
  newScale: number | null;
}

/** 👁️ `change-node-visible` payload. */
export interface ChangeNodeVisible {
  id: string;
  newVisible: boolean | null;
}

/** 🔒️ `change-node-locked` payload. */
export interface ChangeNodeLocked {
  id: string;
  newLocked: boolean | null;
}

/** 🌟️ `change-node-root` payload. */
export interface ChangeNodeRoot {
  id: string;
  newRoot: boolean | null;
}

/** ⚓️ `change-node-anchor` payload. */
export interface ChangeNodeAnchor {
  id: string;
  newAnchor: Puzzle2dNodeAnchor;
}

/** ➕ `add-node-handle` payload. */
export interface AddNodeHandle {
  nodeId: string;
  handle: Puzzle2dHandle;
  index: number | null;
}

/** ➖ `remove-node-handle` payload. */
export interface RemoveNodeHandle {
  nodeId: string;
  handleId: string;
}

/** 🔌 `replace-node-handle` payload. */
export interface ReplaceNodeHandle {
  nodeId: string;
  handleId: string;
  newHandle: Puzzle2dHandle;
}

/** 🔗 `connect-handles` payload. */
export interface ConnectHandles {
  id: string;
  source: string;
  target: string;
  edgeKind: string | null;
  gap: number;
  shift: number;
  rise: number;
  rotation: number;
  turn: number;
  tilt: number;
  x: number;
  y: number;
  sourceTip: string | null;
  targetTip: string | null;
}

/** ✂️ `disconnect-handles` payload. */
export interface DisconnectHandles {
  id: string;
}

/** 🧮 `replace-edge-geometry` payload. */
export interface ReplaceEdgeGeometry {
  id: string;
  newGap: number;
  newShift: number;
  newRise: number;
  newRotation: number;
  newTurn: number;
  newTilt: number;
  newX: number;
  newY: number;
}

/** 🏷️ `change-edge-kind` payload. */
export interface ChangeEdgeKind {
  id: string;
  newEdgeKind: string | null;
}

/** 🖇️ `change-edge-tips` payload. */
export interface ChangeEdgeTips {
  id: string;
  newSourceTip: string | null;
  newTargetTip: string | null;
}

/** 👀 `change-edge-visible` payload. */
export interface ChangeEdgeVisible {
  id: string;
  newVisible: boolean | null;
}

/** 🔐 `change-edge-locked` payload. */
export interface ChangeEdgeLocked {
  id: string;
  newLocked: boolean | null;
}

/** 🆔 `change-manifest-id` payload. */
export interface ChangeManifestId {
  newManifestId: string | null;
}

/** 🤝 `connect-kind-compatibility` payload. */
export interface ConnectKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle2dCompatSpecificity;
}

/** 💔 `disconnect-kind-compatibility` payload. */
export interface DisconnectKindCompatibility {
  source: string;
  target: string;
}

/** 📚 `replace-kind-catalogs` payload. */
export interface ReplaceKindCatalogs {
  newCatalogs: Puzzle2dKindCatalogs | null;
}

export type Puzzle2dMutation =
  | ({ mutation: "createNode" } & CreateNode)
  | ({ mutation: "deleteNode" } & DeleteNode)
  | ({ mutation: "moveNode" } & MoveNode)
  | ({ mutation: "replaceNodeGeometry" } & ReplaceNodeGeometry)
  | ({ mutation: "changeNodeKind" } & ChangeNodeKind)
  | ({ mutation: "editNodeText" } & EditNodeText)
  | ({ mutation: "changeNodeIcon" } & ChangeNodeIcon)
  | ({ mutation: "scaleNode" } & ScaleNode)
  | ({ mutation: "changeNodeVisible" } & ChangeNodeVisible)
  | ({ mutation: "changeNodeLocked" } & ChangeNodeLocked)
  | ({ mutation: "changeNodeRoot" } & ChangeNodeRoot)
  | ({ mutation: "changeNodeAnchor" } & ChangeNodeAnchor)
  | ({ mutation: "addNodeHandle" } & AddNodeHandle)
  | ({ mutation: "removeNodeHandle" } & RemoveNodeHandle)
  | ({ mutation: "replaceNodeHandle" } & ReplaceNodeHandle)
  | ({ mutation: "connectHandles" } & ConnectHandles)
  | ({ mutation: "disconnectHandles" } & DisconnectHandles)
  | ({ mutation: "replaceEdgeGeometry" } & ReplaceEdgeGeometry)
  | ({ mutation: "changeEdgeKind" } & ChangeEdgeKind)
  | ({ mutation: "changeEdgeTips" } & ChangeEdgeTips)
  | ({ mutation: "changeEdgeVisible" } & ChangeEdgeVisible)
  | ({ mutation: "changeEdgeLocked" } & ChangeEdgeLocked)
  | ({ mutation: "changeManifestId" } & ChangeManifestId)
  | ({ mutation: "connectKindCompatibility" } & ConnectKindCompatibility)
  | ({ mutation: "disconnectKindCompatibility" } & DisconnectKindCompatibility)
  | ({ mutation: "replaceKindCatalogs" } & ReplaceKindCatalogs);
