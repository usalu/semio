/** ↩️ inverse for `ChangeMaterialBaseColor` — undoes to another `ChangeMaterialBaseColor` restoring the prior color. */
export interface ChangeMaterialBaseColorInverseChangeMaterialBaseColor {
  id: string;
  newBaseColor: import("../../../📸️snapshot/🟦️component.ts").SemioRgba;
}
