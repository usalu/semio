/** mutation payload — mirrors `CreateEdge`. Edges are id-keyed ENTITIES; source/target are
 * ordinary data fields, not an attach handle. */
export interface CreateEdge {
  id: { value: string };
  source: { value: string };
  target: { value: string };
  kind: string;
  label: string;
}
