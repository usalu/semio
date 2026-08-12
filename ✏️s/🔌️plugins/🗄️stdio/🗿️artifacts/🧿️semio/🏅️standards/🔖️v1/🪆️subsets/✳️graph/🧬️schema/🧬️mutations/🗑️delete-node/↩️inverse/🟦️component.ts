/** ↩️ inverse for `DeleteNode` — a real multi-mutation cascade: one `CreateNode` followed by one
 * `CreateEdge` per severed edge, in original order. */
export type DeleteNodeInverse =
  | { mutation: "createNode"; payload: import("../../🏗️create-node/🦠️mutation/🟦️component.ts").CreateNode }
  | { mutation: "createEdge"; payload: import("../../🔗create-edge/🦠️mutation/🟦️component.ts").CreateEdge };
