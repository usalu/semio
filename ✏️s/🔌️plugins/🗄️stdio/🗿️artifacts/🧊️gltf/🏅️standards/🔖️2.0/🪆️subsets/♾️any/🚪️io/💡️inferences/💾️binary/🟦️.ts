/** 💾 Fixed-header binary envelope carrying canonical JSON for one glTF inference leaf. */
export const GLTF_INFERENCE_LEAF_BINARY_MAGIC = new Uint8Array([0x89, 0x53, 0xf8, 0x3f, 0x7d, 0x34, 0x0d, 0x0c]);
export const GLTF_INFERENCE_LEAF_BINARY_FORMAT_MAJOR = 1 as const;
export const GLTF_INFERENCE_LEAF_BINARY_FORMAT_MINOR = 0 as const;
export const GLTF_INFERENCE_LEAF_BINARY_SCHEMA_VERSION = 1 as const;
export const GLTF_INFERENCE_LEAF_BINARY_SCHEMA_CRC32 = 0xcbd108c3 as const;
export const GLTF_INFERENCE_LEAF_BINARY_HEADER_LENGTH = 40 as const;
export const GLTF_INFERENCE_LEAF_BINARY_CANONICAL_JSON_FLAG = 1 as const;

export interface GltfInferenceLeafBinaryHeader {
  formatMajor: 1;
  formatMinor: 0;
  schemaVersion: 1;
  flags: number;
  schemaCrc32: number;
  payloadLength: bigint;
  payloadCrc32: number;
  headerCrc32: number;
}

export type GltfInferenceLeafBinary = Uint8Array;
