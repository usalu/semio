/** ↩️ inverse for `ReplaceSurface` — undoes to another `ReplaceSurface` restoring the prior surface. */
export interface ReplaceSurfaceInverseReplaceSurface {
  faceId: string;
  newSurface: unknown;
}
