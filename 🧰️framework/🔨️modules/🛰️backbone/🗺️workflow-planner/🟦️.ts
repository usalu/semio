/** 🗺️ Workflow planner: the media flow topology types, topological node order and the workflow plan. */

//#region 🔖️WorkflowPlanner
/**
 * 🎬️ TS mirror of `workflow::{OsMediaPort,OsWorkflowNode,OsWorkflowEdge,OsWorkflow}` (Rust,
 * `framework/os/core/rs/lib.rs`) — camelCase-field-identical (Rust: `#[serde(rename_all =
 * "camelCase")]`). See this file's header for why only this pure-planner slice is hand-mirrored.
 */
export type OsMediaPort = {
  readonly id: string;
  readonly artifactKind: string;
  readonly direction: string;
};

export type OsWorkflowNode = {
  readonly id: string;
  readonly instanceId: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly inputs: readonly OsMediaPort[];
  readonly outputs: readonly OsMediaPort[];
};

/** 🤝️ TS twin of Rust `MediaContract` (`workflow::MediaContract`, hand-written `dsl::DslField`). */
export type MediaContract = {
  readonly kindId: string;
  readonly mediaType: { readonly class: string; readonly form: string };
  readonly wire: { readonly kind: "binary"; readonly format: string } | { readonly kind: "document"; readonly schema: string };
  readonly conversion?: readonly [string, string] | null;
};

export type OsWorkflowEdge = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
  readonly contract: MediaContract;
};

export type OsWorkflow = {
  readonly schema: string;
  readonly nodes: readonly OsWorkflowNode[];
  readonly edges: readonly OsWorkflowEdge[];
};

/** 🚚️ TS twin of Rust `WorkflowDelivery`. */
export type WorkflowDelivery = {
  readonly edgeId: string;
  readonly producerInstanceId: string;
  readonly producerPortId: string;
  readonly consumerInstanceId: string;
  readonly consumerPortId: string;
};

/**
 * 🧭️ TS twin of Rust `workflow_topological_node_order` — DFS post-order reversed into a
 * topological node order (source before target); deterministic purely from `graph.nodes`/
 * `graph.edges` insertion order, so it matches the Rust side edge-for-edge.
 */
function mediaFlowTopologicalNodeOrder(graph: OsWorkflow): readonly string[] {
  const adjacency = new Map<string, string[]>();
  for (const edge of graph.edges) {
    const targets = adjacency.get(edge.sourceNodeId) ?? [];
    targets.push(edge.targetNodeId);
    adjacency.set(edge.sourceNodeId, targets);
  }
  const visited = new Set<string>();
  const order: string[] = [];
  const dfs = (nodeId: string): void => {
    if (visited.has(nodeId)) return;
    visited.add(nodeId);
    for (const next of adjacency.get(nodeId) ?? []) dfs(next);
    order.push(nodeId);
  };
  for (const node of graph.nodes) dfs(node.id);
  order.reverse();
  return order;
}

/**
 * 🚚️ TS twin of Rust `plan_workflow` — plans one {@link WorkflowDelivery} per edge in the
 * downstream closure of `dirtyInstanceIds`, propagating dirtiness onto each edge's consumer instance
 * so multi-hop chains (A→B→C) resolve in a single topological pass. Pure/side-effect-free.
 */
export function planWorkflow(graph: OsWorkflow, dirtyInstanceIds: ReadonlySet<string>): readonly WorkflowDelivery[] {
  const nodeById = new Map<string, OsWorkflowNode>(graph.nodes.map((node) => [node.id, node]));
  const edgesBySource = new Map<string, OsWorkflowEdge[]>();
  for (const edge of graph.edges) {
    const edges = edgesBySource.get(edge.sourceNodeId) ?? [];
    edges.push(edge);
    edgesBySource.set(edge.sourceNodeId, edges);
  }
  const order = mediaFlowTopologicalNodeOrder(graph);
  const dirty = new Set(dirtyInstanceIds);
  const deliveries: WorkflowDelivery[] = [];
  for (const nodeId of order) {
    const node = nodeById.get(nodeId);
    if (!node || !dirty.has(node.instanceId)) continue;
    for (const edge of edgesBySource.get(nodeId) ?? []) {
      const targetNode = nodeById.get(edge.targetNodeId);
      if (!targetNode) continue;
      deliveries.push({
        edgeId: edge.id,
        producerInstanceId: node.instanceId,
        producerPortId: edge.sourcePortId,
        consumerInstanceId: targetNode.instanceId,
        consumerPortId: edge.targetPortId,
      });
      dirty.add(targetNode.instanceId);
    }
  }
  return deliveries;
}
//#endregion 🔖️WorkflowPlanner
