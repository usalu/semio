/** ↩️ inverse for `RemoveNodePort`. */
export interface RemoveNodePortInverseAddNodePort {
  nodeId: { value: string };
  index: number;
  port: { name: string; kind: "in" | "out" | "inOut" };
}
