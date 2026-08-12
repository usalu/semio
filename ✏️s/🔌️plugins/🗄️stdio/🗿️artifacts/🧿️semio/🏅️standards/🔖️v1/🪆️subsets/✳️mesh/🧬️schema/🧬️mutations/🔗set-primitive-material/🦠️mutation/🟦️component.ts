/** mutation payload — mirrors `SetPrimitiveMaterial`. */
export interface SetPrimitiveMaterial {
  meshId: string;
  primitiveId: string;
  materialId: string | null;
}
