/** ↩️ inverse for `DeleteLayer` — always `CreateLayer`. */
export interface DeleteLayerInverseCreateLayer {
  index: number;
  layer: { id: string; name: string; visible: boolean; root: unknown };
}
