/** ↩️ remove-gcp-observation inverse — an `add-gcp-observation` restoring the captured BASE observation. */
export interface RemoveGcpObservationInverse {
  id: string;
  observation: { streamId: string; frameIndex: number; pixel: [number, number] };
}
