import type { Viewport2d } from "@semio-tech/framework-ui-viewport-ts/2d/schema";
import { type SceneLane, type SceneLaneRef, sceneFromLanes } from "../🛤️lanes/🟦️.ts";

/** 🔌️ One port on a node-graph node: identity + display label (direction is implied by whether the
 * record lives in the owning node's `inputs` or `outputs` array). `code`/`abbreviation`/`fullName`/
 * `resourceKind` are set only for OS-workflow app-instance nodes (the wire key stays `resourceKind` —
 * the rename to `artifactKind` is W4/`OsWorkflowNodeGraphPayload` scope, not this ticket's). */
export type NodeGraphPortRecord = {
  readonly id: string;
  readonly label?: string;
  readonly code?: string;
  readonly abbreviation?: string;
  readonly fullName?: string;
  readonly resourceKind?: string;
  readonly valueType?: string;
};

/** 🕸️ One node-graph node: identity, label, layout rect, typed ports. `instanceId`/`pluginId`/`appId`/
 * `icon` are set only for OS-workflow app-instance nodes (the space canvas's node-graph). */
export type NodeGraphNodeRecord = {
  readonly id: string;
  readonly label?: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly inputs: readonly NodeGraphPortRecord[];
  readonly outputs: readonly NodeGraphPortRecord[];
  readonly instanceId?: string;
  readonly pluginId?: string;
  readonly appId?: string;
  readonly icon?: string;
};

/** 🔗️ One node-graph edge between two node/port endpoints. */
export type NodeGraphEdgeRecord = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
  readonly label?: string;
};

/** 🔎️ One spotlight/find result row for a node-graph surface. */
export type NodeGraphFindItem = {
  readonly id: string;
  readonly label: string;
  readonly category: string;
};

/** 🖱️ Hovered node id, if any. */
export type NodeGraphHover = {
  readonly nodeId?: string;
  /** 🔆️ Hovered channel (port) id on `nodeId`, matching the `"{nodeId}@{portId}"` pick id. */
  readonly portId?: string;
};

/** ➕️ Variadic input/output slot on an operator catalogue entry. */
export type NodeGraphOperatorVariadicRecord = {
  readonly slotKey: string;
  readonly min: number;
  readonly max?: number;
};

/** 📡️ Declared operator channel (input or output); `cardinality` rides as its serialized symbol
 * string (`"!"`/`"?"`/`"*"`/`"+"`/digits). */
export type NodeGraphOperatorChannelRecord = {
  readonly code: string;
  readonly abbreviation: string;
  readonly name: string;
  readonly fullName: string;
  readonly operators?: readonly string[];
  readonly valueTypes?: readonly string[];
  readonly itemTypes?: readonly string[];
  readonly default?: unknown;
  readonly label?: string;
  readonly cardinality: string;
};

/** 🧠️ One operator catalogue entry offered to a flow-backed node-graph's spotlight/palette. */
export type NodeGraphOperatorRecord = {
  readonly id: string;
  readonly extension: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
  readonly inputs: readonly NodeGraphOperatorChannelRecord[];
  readonly outputs: readonly NodeGraphOperatorChannelRecord[];
  readonly variadicInput?: NodeGraphOperatorVariadicRecord;
  readonly variadicOutput?: NodeGraphOperatorVariadicRecord;
  readonly group?: readonly string[];
};

/** 🛍️ One drag-and-drop palette entry — mirrors the Rust `CatalogueItem`. */
export type AppCatalogueItem = {
  readonly kind: string;
  readonly neuronKind?: string;
  readonly action?: string;
  readonly format?: string;
  readonly name: string;
  readonly abbreviation: string;
  readonly icon: string;
  readonly summary: string;
};

/** 📑️ One palette section — mirrors the Rust `CatalogueSection`/`CatalogueGroup` pair. */
export type AppCatalogueSection = {
  readonly id: string;
  readonly title: string;
  readonly items?: readonly AppCatalogueItem[];
  readonly groups?: readonly AppCatalogueSection[];
};

/**
 * 📚️ The APP-STATIC catalogue an app publishes ONCE per instance on the reserved
 * `framework.section.catalogue` retained surface — mirrors the Rust `FlowAppCatalogue`
 * (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗂️catalogue/🦀️.rs`).
 *
 * 🚨️ It is deliberately NOT part of {@link NodeGraphScene}. With the real `brep`/`math` operator sets
 * installed it is ~100 KB, and a surface payload is admitted against the fixed 32 KiB
 * `UI_FIXED_BYTES` capacity, so embedding it per scene made every node-graph window fail admission at
 * 111 031 B and render nothing (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). A scene now references
 * an operator by KIND ID only, and the renderer caches this payload for the app's whole lifetime.
 */
export type AppCatalogue = {
  readonly operators?: readonly NodeGraphOperatorRecord[];
  readonly sections?: readonly AppCatalogueSection[];
};

/** 🎯️ A node graph's complete framework interaction target address. */
export type NodeGraphInteractionDomain = {
  readonly id: string;
  readonly nodeTargetPrefix: string;
  readonly edgeTargetPrefix: string;
  readonly handleTargetPrefix: string;
};

/** 🎬️ A node-graph surface scene payload — mirrors the wasm `componentScene` node's `nodeGraph` field. */
export type NodeGraphScene = {
  readonly lanes?: readonly SceneLaneRef[];
  readonly nodes: readonly NodeGraphNodeRecord[];
  readonly edges: readonly NodeGraphEdgeRecord[];
  readonly viewport?: Viewport2d;
  readonly editable?: boolean;
  /** ✅️ The complete interaction address; absent surfaces publish no selection or hover. */
  readonly interactionDomain?: NodeGraphInteractionDomain;
  /** ⚙️ DOCUMENT-DERIVED operator records only — one per node this graph holds (the OS workflow window
   * derives one per workflow node so the canvas can lay its ports out). The app's REGISTERED operator
   * catalogue rides {@link AppCatalogue} on the reserved `framework.section.catalogue` surface instead;
   * a flow-backed scene leaves this empty. */
  readonly operators?: readonly NodeGraphOperatorRecord[];
  readonly findItems?: readonly NodeGraphFindItem[];
  readonly selection?: readonly string[];
  readonly hover?: NodeGraphHover;
  /** ✨️ Ids — nodes, edges, or `"{nodeId}@{portId}"` ports — the plugin wants highlighted, e.g.
   * because they are transitively hovered from another window. */
  readonly highlighted?: readonly string[];
  readonly previewOffJson?: string;
  readonly lodJson?: string;
  readonly controlsJson?: string;
  readonly clustersJson?: string;
  readonly computingJson?: string;
  readonly statusJson?: string;
  readonly capabilitiesJson?: string;
  readonly hostSnapshotJson?: string;
  readonly presencePeersJson?: string;
  /** 🧵️ Channel-structured eval outputs from an off-main-thread `flowEvalTick` chain, applied via
   * `FlowWasmSession.applyEvalOutputsJson` — lets the canvas session pick up results without ever
   * evaluating itself. */
  readonly evalJson?: string;
};

/** 👥️ A live-collaboration cursor/selection peer shown on a shared surface. */
export type PresencePeer = {
  readonly clientId: string;
  readonly name: string;
  readonly selectionCount: number;
};

/** 📝️ A text-editor surface scene payload — mirrors the wasm `componentScene` node's `textEditor` field. */
export type TextEditorScene = {
  readonly buffer: string;
  readonly lanes?: readonly SceneLaneRef[];
  readonly language?: string;
  readonly selectionJson?: string;
  readonly tokensJson?: string;
  readonly diagnosticsJson?: string;
  readonly completionsJson?: string;
  readonly overlaysJson?: string;
  readonly occurrencesJson?: string;
  readonly placeholdersJson?: string;
  readonly extraCaretsJson?: string;
  readonly selectableSpansJson?: string;
  readonly settingsJson?: string;
  readonly cameraJson?: string;
  readonly hoverJson?: string;
  readonly newlineGatesJson?: string;
  readonly renameJson?: string;
};

/** 🚚️ The complete editable buffer travels outside the bounded scene header. */
export const TEXT_EDITOR_SCENE_LANES: readonly SceneLane<TextEditorScene>[] = [
  { lane: "buffer", field: "buffer", bodyKey: "framework.scene.text.buffer", optional: false },
];

/** ✏️ Restores a text editor's complete source from its paged buffer carrier. */
export function textEditorSceneFromLanes(spine: TextEditorScene, laneTexts: ReadonlyMap<string, string>): TextEditorScene {
  return sceneFromLanes(spine, laneTexts, TEXT_EDITOR_SCENE_LANES);
}

export const nodeGraphActions = {
  select: "interactionSelect",
  hover: "interactionHover",
  clearSelection: "clearSelection",
  edit: "nodeGraphEdit",
  viewport: "nodeGraphViewport",
  spotlightCommit: "spotlightCommit",
} as const;

/** 🛤️ Node graph carriers declared by the language-neutral scene lane schema. */
export const NODE_GRAPH_SCENE_LANES: readonly SceneLane<NodeGraphScene>[] = [
  {"lane": "nodes", "field": "nodes", "bodyKey": "framework.scene.nodeGraph.nodes", "optional": false, "encoding": "json"},
  {"lane": "edges", "field": "edges", "bodyKey": "framework.scene.nodeGraph.edges", "optional": false, "encoding": "json"},
  {"lane": "operators", "field": "operators", "bodyKey": "framework.scene.nodeGraph.operators", "optional": false, "encoding": "json"},
  {"lane": "findItems", "field": "findItems", "bodyKey": "framework.scene.nodeGraph.findItems", "optional": false, "encoding": "json"},
  {"lane": "selection", "field": "selection", "bodyKey": "framework.scene.nodeGraph.selection", "optional": false, "encoding": "json"},
  {"lane": "highlighted", "field": "highlighted", "bodyKey": "framework.scene.nodeGraph.highlighted", "optional": false, "encoding": "json"},
  {"lane": "previewOff", "field": "previewOffJson", "bodyKey": "framework.scene.nodeGraph.previewOff", "optional": true},
  {"lane": "lod", "field": "lodJson", "bodyKey": "framework.scene.nodeGraph.lod", "optional": true},
  {"lane": "controls", "field": "controlsJson", "bodyKey": "framework.scene.nodeGraph.controls", "optional": true},
  {"lane": "clusters", "field": "clustersJson", "bodyKey": "framework.scene.nodeGraph.clusters", "optional": true},
  {"lane": "computing", "field": "computingJson", "bodyKey": "framework.scene.nodeGraph.computing", "optional": true},
  {"lane": "status", "field": "statusJson", "bodyKey": "framework.scene.nodeGraph.status", "optional": true},
  {"lane": "capabilities", "field": "capabilitiesJson", "bodyKey": "framework.scene.nodeGraph.capabilities", "optional": true},
  {"lane": "hostSnapshot", "field": "hostSnapshotJson", "bodyKey": "framework.scene.nodeGraph.hostSnapshot", "optional": true},
  {"lane": "presencePeers", "field": "presencePeersJson", "bodyKey": "framework.scene.nodeGraph.presencePeers", "optional": true},
  {"lane": "eval", "field": "evalJson", "bodyKey": "framework.scene.nodeGraph.eval", "optional": true},
];

/** ♻️ Restores typed node graph records and opaque JSON fields from complete carriers. */
export function nodeGraphSceneFromLanes(spine: NodeGraphScene, laneTexts: ReadonlyMap<string, string>): NodeGraphScene {
  return sceneFromLanes(spine, laneTexts, NODE_GRAPH_SCENE_LANES);
}
