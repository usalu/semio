/** mutation payload — mirrors `AddNodePort`. */
export interface AddNodePort {
  nodeId: { value: string };
  index: number;
  port: { name: string; kind: "in" | "out" | "inOut" };
}
