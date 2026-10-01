/** 🚚️ dag `move-nodes` payload mirror of `MoveNodes` (internally tagged `mutation: "moveNodes"`), with its closed-schema parser. */
export interface MoveNodes {
  mutation: "moveNodes";
  ids: string[];
  dx: number;
  dy: number;
}

/** 🚪️ Parses one `move-nodes` payload the way its JSON Schema admits it, or throws. */
export function parseMoveNodes(value: unknown): MoveNodes {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("move-nodes: payload is not an object");
  const row = value as Record<string, unknown>;
  const unknownKey = Object.keys(row).find((key) => !["mutation", "ids", "dx", "dy"].includes(key));
  if (unknownKey !== undefined) throw new TypeError(`move-nodes: unknown field ${unknownKey}`);
  if (row.mutation !== "moveNodes") throw new TypeError("move-nodes: mutation must be moveNodes");
  if (!Array.isArray(row.ids) || row.ids.length === 0 || row.ids.some((entry) => typeof entry !== "string" || entry.length === 0) || new Set(row.ids).size !== row.ids.length) throw new TypeError("move-nodes: ids must be a nonempty list of unique ids");
  if (typeof row.dx !== "number" || !Number.isFinite(row.dx)) throw new TypeError("move-nodes: dx must be a finite number");
  if (typeof row.dy !== "number" || !Number.isFinite(row.dy)) throw new TypeError("move-nodes: dy must be a finite number");
  return { mutation: "moveNodes", ids: row.ids as string[], dx: row.dx, dy: row.dy };
}
