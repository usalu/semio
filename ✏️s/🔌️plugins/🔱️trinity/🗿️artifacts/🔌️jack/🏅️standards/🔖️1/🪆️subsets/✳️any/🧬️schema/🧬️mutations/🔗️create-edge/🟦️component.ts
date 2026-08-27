/** 🔗️ jack direct `create-edge` payload mirror of `CreateEdge`. */
export interface JackEdge {
  id: string;
  kind: string;
  source: string;
  target: string;
  properties: Record<string, unknown>;
}

export interface CreateEdge {
  edge: JackEdge;
}
