/** ↩️ inverse for `DeleteEdge`. */
export interface DeleteEdgeInverseCreateEdge {
  id: { value: string };
  source: { value: string };
  target: { value: string };
  kind: string;
  label: string;
}
