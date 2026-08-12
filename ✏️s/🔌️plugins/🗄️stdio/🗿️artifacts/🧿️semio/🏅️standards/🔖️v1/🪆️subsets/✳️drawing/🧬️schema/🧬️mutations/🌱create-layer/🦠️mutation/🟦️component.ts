/** mutation payload — mirrors `CreateLayer`. */
export interface CreateLayer {
  index: number;
  layer: { id: string; name: string; visible: boolean; root: unknown };
}
