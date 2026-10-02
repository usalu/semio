/** 📍️ One absolute workflow node position of a `set-node-positions` payload. */
export interface WorkflowNodePosition {
  nodeId: string;
  x: number;
  y: number;
}

/** 📌️ workflow `set-node-positions` payload mirror of `SetNodePositions` (tagged `operation: "setNodePositions"`). */
export interface SetNodePositions {
  operation: "setNodePositions";
  positions: WorkflowNodePosition[];
}

/** 🛃️ Parses one `set-node-positions` payload the way its JSON Schema and `unique-node-ids` invariant admit it, or throws. */
export function parseSetNodePositions(value: unknown): SetNodePositions {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("set-node-positions: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["operation", "positions"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`set-node-positions: unknown field ${unknownKey}`);
  if (row.operation !== "setNodePositions") throw new TypeError("set-node-positions: operation must be setNodePositions");
  if (!Array.isArray(row.positions) || row.positions.length === 0) throw new TypeError("set-node-positions: positions must be a nonempty list");
  const positions = row.positions.map((entry, index): WorkflowNodePosition => {
    if (entry === null || typeof entry !== "object" || Array.isArray(entry)) throw new TypeError(`set-node-positions: position ${index} is not an object`);
    const position = entry as Record<string, unknown>;
    const unknownField = Object.keys(position).find((key) => !["nodeId", "x", "y"].includes(key));
    if (unknownField !== undefined) throw new TypeError(`set-node-positions: position ${index} has unknown field ${unknownField}`);
    if (typeof position.nodeId !== "string" || position.nodeId.length === 0) throw new TypeError(`set-node-positions: position ${index} needs a nonempty nodeId`);
    if (typeof position.x !== "number" || !Number.isFinite(position.x)) throw new TypeError(`set-node-positions: position ${index} x must be a finite number`);
    if (typeof position.y !== "number" || !Number.isFinite(position.y)) throw new TypeError(`set-node-positions: position ${index} y must be a finite number`);
    return { nodeId: position.nodeId, x: position.x, y: position.y };
  });
  if (new Set(positions.map((position) => position.nodeId)).size !== positions.length) throw new TypeError("set-node-positions: node ids must be unique");
  return { operation: "setNodePositions", positions };
}
