/** 💾 Fixed-header binary envelope carrying canonical `GltfInference` JSON. */
export const GLTF_INFERENCE_BINARY_MAGIC = new Uint8Array([0x89, 0x53, 0xf8, 0x3f, 0x7d, 0x34, 0x0d, 0x0b]);
export const GLTF_INFERENCE_BINARY_SCHEMA = 's.stdio.gltf.inference' as const;
export const GLTF_INFERENCE_BINARY_FORMAT_MAJOR = 1 as const;
export const GLTF_INFERENCE_BINARY_FORMAT_MINOR = 0 as const;
export const GLTF_INFERENCE_BINARY_SCHEMA_VERSION = 2 as const;
export const GLTF_INFERENCE_BINARY_SCHEMA_CRC32 = 0x6b257ae0 as const;
export const GLTF_INFERENCE_BINARY_HEADER_LENGTH = 40 as const;
export const GLTF_INFERENCE_BINARY_CANONICAL_JSON_FLAG = 1 as const;
export interface GltfInferenceBinaryHeader { formatMajor: 1; formatMinor: 0; schemaVersion: 2; flags: number; schemaCrc32: number; payloadLength: bigint; payloadCrc32: number; headerCrc32: number }
export type GltfInferenceBinary = Uint8Array;
