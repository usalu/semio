/** mutation payload — mirrors `AddNodeProperty`. */
export interface AddNodeProperty {
  nodeId: { value: string };
  index: number;
  property: { key: string; value: unknown };
}
