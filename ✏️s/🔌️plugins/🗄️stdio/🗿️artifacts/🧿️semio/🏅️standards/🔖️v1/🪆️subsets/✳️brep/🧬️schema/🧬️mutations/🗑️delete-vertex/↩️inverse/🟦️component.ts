/** ↩️ inverse for `DeleteVertex` — a real multi-mutation cascade: one `CreateVertex` followed by
 * one `CreateEdge` per severed edge. */
export type DeleteVertexInverse =
  | { mutation: "createVertex"; payload: unknown }
  | { mutation: "createEdge"; payload: unknown };
