/** mutation payload — mirrors `CreateEdge`. */
export interface CreateEdge {
  id: string;
  startVertex: string;
  endVertex: string;
  curve: unknown;
}
