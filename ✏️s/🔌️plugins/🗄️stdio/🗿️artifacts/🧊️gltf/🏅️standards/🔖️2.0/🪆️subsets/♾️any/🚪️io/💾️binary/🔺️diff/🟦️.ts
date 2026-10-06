/** 💾 Ordered 21-slot sparse diff wire with explicit absent/null/value states. */
export const GLTF_DIFF_BINARY_FIELDS = ['asset', 'scene', 'scenes', 'nodes', 'meshes', 'accessors', 'bufferViews', 'buffers', 'bufferBytes', 'materials', 'textures', 'images', 'samplers', 'skins', 'animations', 'cameras', 'extensionsUsed', 'extensionsRequired', 'extensions', 'extras', 'sourceForm'] as const;
export type GltfDiffBinaryField = typeof GLTF_DIFF_BINARY_FIELDS[number];
export type GltfDiffBinaryPresence = 0 | 1 | 2;
export type GltfDiffBinary = Uint8Array;
