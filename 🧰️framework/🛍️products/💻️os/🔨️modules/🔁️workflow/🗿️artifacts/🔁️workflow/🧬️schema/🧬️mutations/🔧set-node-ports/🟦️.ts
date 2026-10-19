/** 🔌️ One workflow node's input ports and height of a `set-node-ports` payload. */
export interface WorkflowNodePorts {
  nodeId: string;
  inputs: unknown[];
  height: number;
}

/** 🔧️ workflow `set-node-ports` payload mirror of `SetNodePorts` (tagged `operation: "setNodePorts"`). */
export interface SetNodePorts {
  operation: "setNodePorts";
  nodes: WorkflowNodePorts[];
}

/** 🛃️ Parses one `set-node-ports` payload the way its JSON Schema and `unique-node-ids` invariant admit it, or throws. */
export function parseSetNodePorts(value: unknown): SetNodePorts {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("set-node-ports: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["operation", "nodes"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`set-node-ports: unknown field ${unknownKey}`);
  if (row.operation !== "setNodePorts") throw new TypeError("set-node-ports: operation must be setNodePorts");
  if (!Array.isArray(row.nodes) || row.nodes.length === 0) throw new TypeError("set-node-ports: nodes must be a nonempty list");
  const nodes = row.nodes.map((entry, index): WorkflowNodePorts => {
    if (entry === null || typeof entry !== "object" || Array.isArray(entry)) throw new TypeError(`set-node-ports: node ${index} is not an object`);
    const node = entry as Record<string, unknown>;
    const unknownField = Object.keys(node).find((key) => !["nodeId", "inputs", "height"].includes(key));
    if (unknownField !== undefined) throw new TypeError(`set-node-ports: node ${index} has unknown field ${unknownField}`);
    if (typeof node.nodeId !== "string" || node.nodeId.length === 0) throw new TypeError(`set-node-ports: node ${index} needs a nonempty nodeId`);
    if (!Array.isArray(node.inputs)) throw new TypeError(`set-node-ports: node ${index} inputs must be a list`);
    if (typeof node.height !== "number" || !Number.isFinite(node.height)) throw new TypeError(`set-node-ports: node ${index} height must be a finite number`);
    return { nodeId: node.nodeId, inputs: node.inputs, height: node.height };
  });
  if (new Set(nodes.map((node) => node.nodeId)).size !== nodes.length) throw new TypeError("set-node-ports: node ids must be unique");
  return { operation: "setNodePorts", nodes };
}
