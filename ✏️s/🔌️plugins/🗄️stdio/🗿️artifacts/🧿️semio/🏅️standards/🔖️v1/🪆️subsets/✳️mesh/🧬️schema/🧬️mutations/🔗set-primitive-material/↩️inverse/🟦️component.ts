/** ↩️ inverse for `SetPrimitiveMaterial` — undoes to another `SetPrimitiveMaterial` restoring the prior reference. */
export interface SetPrimitiveMaterialInverseSetPrimitiveMaterial {
  meshId: string;
  primitiveId: string;
  materialId: string | null;
}
