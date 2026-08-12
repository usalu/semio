/** 🔎 add-gcp-observation mutation payload — appends one observation to an existing gcp. */
export interface AddGcpObservation {
  id: string;
  observation: { streamId: string; frameIndex: number; pixel: [number, number] };
}
