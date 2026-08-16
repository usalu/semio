/** 📝 Canonical UTF-8 text envelope for one independently executable glTF inference leaf. */
export const GLTF_INFERENCE_LEAF_TEXT_VERSION = 1 as const;

export interface GltfInferenceLeafEnvelope {
  id: string;
  algorithmVersion: number;
  policyHash: string;
  dependencyHashes: readonly string[];
  cacheKey: string;
  validity: string;
  quality: string;
  diagnosticIds: readonly string[];
  provenance: readonly string[];
  value: unknown;
}

export interface GltfInferenceLeafTextDocument {
  schema: string;
  version: typeof GLTF_INFERENCE_LEAF_TEXT_VERSION;
  payloadLength: number;
  checksum: string;
  value: GltfInferenceLeafEnvelope;
}

export type GltfInferenceLeafText = string;
