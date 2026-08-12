/** ↩️ `create-mesh` inverse — restores the prior mesh handle, or clears the slot if it was empty. */
export interface CreateMeshInverse {
  priorMesh?: { childId: string; target: string };
}
