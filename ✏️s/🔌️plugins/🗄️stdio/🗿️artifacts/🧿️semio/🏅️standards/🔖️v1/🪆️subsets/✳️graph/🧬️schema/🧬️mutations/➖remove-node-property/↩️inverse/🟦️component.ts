/** ↩️ inverse for `RemoveNodeProperty`. */
export interface RemoveNodePropertyInverseAddNodeProperty {
  nodeId: { value: string };
  index: number;
  property: { key: string; value: unknown };
}
