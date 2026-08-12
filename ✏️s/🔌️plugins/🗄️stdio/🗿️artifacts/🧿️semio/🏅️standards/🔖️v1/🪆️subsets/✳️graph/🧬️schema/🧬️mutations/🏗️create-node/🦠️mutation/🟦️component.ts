/** mutation payload — mirrors `CreateNode`. */
export interface CreateNode {
  id: { value: string };
  kind: string;
  label: string;
  position: { x: number; y: number };
  ports: { name: string; kind: "in" | "out" | "inOut" }[];
  properties: { key: string; value: unknown }[];
}
