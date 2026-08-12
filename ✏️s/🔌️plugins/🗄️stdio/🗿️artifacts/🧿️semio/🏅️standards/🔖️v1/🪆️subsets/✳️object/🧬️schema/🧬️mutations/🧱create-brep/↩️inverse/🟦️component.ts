/** ↩️ `create-brep` inverse — restores the prior brep handle, or clears the slot if it was empty. */
export interface CreateBrepInverse {
  priorBrep?: { childId: string; target: string };
}
