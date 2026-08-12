/** ↩️ `create-properties` inverse — restores the prior properties handle, or clears the slot. */
export interface CreatePropertiesInverse {
  priorProperties?: { childId: string; target: string };
}
