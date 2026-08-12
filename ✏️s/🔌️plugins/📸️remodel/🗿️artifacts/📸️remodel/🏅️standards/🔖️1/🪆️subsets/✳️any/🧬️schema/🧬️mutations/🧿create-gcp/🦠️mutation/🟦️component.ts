/** 🧿 create-gcp mutation payload — brings a new ground control point into existence. */
export interface CreateGcp {
  gcp: GroundControlPoint;
}

export interface GroundControlPoint {
  id: string;
  name: string;
  worldPosition: [number, number, number];
  observations: GcpObservation[];
}

export interface GcpObservation {
  streamId: string;
  frameIndex: number;
  pixel: [number, number];
}
