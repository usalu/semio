/** 💾 Structured mutation wire: format byte, frozen tag byte, then variant payload. */
export const GLTF_MUTATION_BINARY_FORMAT = 1 as const;
export const GLTF_MUTATION_BINARY_TAGS = {
  noMutation: 0, setSnapshot: 1, setAsset: 2, insertScene: 3, removeScene: 4, setScene: 5,
  insertNode: 6, removeNode: 7, setNode: 8, insertMesh: 9, removeMesh: 10, setMesh: 11,
  insertAccessor: 12, removeAccessor: 13, setAccessor: 14, insertMaterial: 15, removeMaterial: 16,
  setMaterial: 17, insertBuffer: 18, removeBuffer: 19, setBuffer: 20, insertAnimation: 21,
  removeAnimation: 22, setAnimation: 23, transformNode: 24, reparentNode: 25, bindNodeMesh: 26,
  bindPrimitiveMaterial: 27,
} as const;
export interface GltfMutationBinaryHeader { format: 1; tag: typeof GLTF_MUTATION_BINARY_TAGS[keyof typeof GLTF_MUTATION_BINARY_TAGS] }
export type GltfMutationsBinary = Uint8Array;
