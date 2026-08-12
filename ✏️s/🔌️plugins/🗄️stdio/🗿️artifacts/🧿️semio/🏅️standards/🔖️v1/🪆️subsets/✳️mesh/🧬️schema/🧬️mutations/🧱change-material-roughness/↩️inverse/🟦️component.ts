/** ↩️ inverse for `ChangeMaterialRoughness` — undoes to another `ChangeMaterialRoughness` restoring the prior factor. */
export interface ChangeMaterialRoughnessInverseChangeMaterialRoughness {
  id: string;
  newRoughness: number;
}
