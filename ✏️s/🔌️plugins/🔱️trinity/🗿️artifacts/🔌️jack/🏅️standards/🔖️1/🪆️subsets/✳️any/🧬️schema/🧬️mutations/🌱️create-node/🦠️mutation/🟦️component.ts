/** 🌱️ jack create-node/🦠️mutation — payload mirror of `CreateNode`. */
export interface JackPort {
  id: string;
  kind: string;
  direction: "in" | "out";
  properties: Record<string, unknown>;
}

export interface JackNode {
  id: string;
  kind: string;
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  properties: Record<string, unknown>;
  ports: JackPort[];
}

export interface CreateNode {
  node: JackNode;
}
