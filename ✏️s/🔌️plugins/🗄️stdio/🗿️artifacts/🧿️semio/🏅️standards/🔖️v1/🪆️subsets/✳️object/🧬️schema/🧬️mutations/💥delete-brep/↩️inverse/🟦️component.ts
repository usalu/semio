/** ↩️ `delete-brep` inverse — CreateBrep with the escrowed handle, or empty if already absent. */
export interface DeleteBrepInverse {
  restoredBrep?: { childId: string; target: string };
}
