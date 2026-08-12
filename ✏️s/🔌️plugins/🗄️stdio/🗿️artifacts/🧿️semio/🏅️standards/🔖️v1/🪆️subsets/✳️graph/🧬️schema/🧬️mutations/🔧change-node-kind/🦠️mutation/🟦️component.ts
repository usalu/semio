/** mutation payload — mirrors `ChangeNodeKind`. */
export interface ChangeNodeKind {
  id: { value: string };
  newKind: string;
}
