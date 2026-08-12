/** ↩️ `delete-mesh` inverse — CreateMesh with the escrowed handle, or empty if already absent. */
export interface DeleteMeshInverse {
  restoredMesh?: { childId: string; target: string };
}
