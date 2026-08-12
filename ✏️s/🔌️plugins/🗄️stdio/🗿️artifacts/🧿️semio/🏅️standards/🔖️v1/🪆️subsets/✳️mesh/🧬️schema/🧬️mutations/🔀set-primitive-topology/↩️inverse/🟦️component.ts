/** ↩️ inverse for `SetPrimitiveTopology` — undoes to another `SetPrimitiveTopology` restoring the prior topology. */
export interface SetPrimitiveTopologyInverseSetPrimitiveTopology {
  meshId: string;
  primitiveId: string;
  topology: import("../../../📸️snapshot/🟦️component.ts").SemioTopology;
}
