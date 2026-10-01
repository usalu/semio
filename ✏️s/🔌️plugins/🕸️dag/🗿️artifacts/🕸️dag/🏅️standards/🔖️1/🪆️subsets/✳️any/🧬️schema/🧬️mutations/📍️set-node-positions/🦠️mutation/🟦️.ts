/** 📍️ dag `set-node-positions` payload mirror of `SetNodePositions` (internally tagged `mutation: "setNodePositions"`), with its closed-schema parser. */
export interface DagNodePosition {
  id: string;
  x: number;
  y: number;
}

export interface SetNodePositions {
  mutation: "setNodePositions";
  positions: DagNodePosition[];
}

/** 🚪️ Parses one `set-node-positions` payload the way its JSON Schema (and its `unique-node-ids` invariant) admits it, or throws. */
export function parseSetNodePositions(value: unknown): SetNodePositions {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("set-node-positions: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["mutation", "positions"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`set-node-positions: unknown field ${unknownKey}`);
  if (row.mutation !== "setNodePositions") throw new TypeError("set-node-positions: mutation must be setNodePositions");
  if (!Array.isArray(row.positions) || row.positions.length === 0) throw new TypeError("set-node-positions: positions must be a nonempty list");
  const positions = row.positions.map((entry): DagNodePosition => {
    if (entry === null || typeof entry !== "object" || Array.isArray(entry)) throw new TypeError("set-node-positions: a position is not an object");
    const position = entry as Record<string, unknown>;
    const extra = Object.keys(position).find((key) => !["id", "x", "y"].includes(key));
    if (extra !== undefined) throw new TypeError(`set-node-positions: unknown position field ${extra}`);
    if (typeof position.id !== "string" || position.id.length === 0) throw new TypeError("set-node-positions: a position names a node id");
    if (typeof position.x !== "number" || !Number.isFinite(position.x) || typeof position.y !== "number" || !Number.isFinite(position.y)) throw new TypeError("set-node-positions: a position is two finite numbers");
    return { id: position.id, x: position.x, y: position.y };
  });
  if (new Set(positions.map((position) => position.id)).size !== positions.length) throw new TypeError("set-node-positions: each node at most once");
  return { mutation: "setNodePositions", positions };
}
