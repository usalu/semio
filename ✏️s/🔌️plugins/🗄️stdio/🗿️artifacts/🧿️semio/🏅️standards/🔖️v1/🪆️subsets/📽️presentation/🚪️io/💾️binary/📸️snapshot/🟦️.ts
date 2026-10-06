/** Binary envelope shape descriptor (payload is opaque JSON bytes at this level). */
export interface SnapshotBinaryEnvelope { envelopeId: string; componentTag: number; version: number; payload: Uint8Array; }
