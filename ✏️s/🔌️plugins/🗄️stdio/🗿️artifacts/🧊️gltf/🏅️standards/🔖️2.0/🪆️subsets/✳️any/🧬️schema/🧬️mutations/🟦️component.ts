/** 🧬 Complete semantic glTF mutation vocabulary; binary ordinals are frozen at 0–27. */
import type { GltfSnapshot, GltfAsset, GltfScene, GltfNode, GltfMesh, GltfAccessor, GltfMaterial, GltfBuffer, GltfAnimation } from '../📸️snapshot/🟦️component.ts';
import type { GltfDiff } from '../🔺️diff/🟦️component.ts';

export interface GltfMutationRejection { code: string; path: string; detail: string }
/** 🛂 Application is total and typed: rejection never degrades to an empty diff. */
export type GltfMutationApplication = { accepted: true; diff: GltfDiff } | { accepted: false; rejection: GltfMutationRejection };
export type GltfTouchedRegion = 'asset' | 'scene' | 'scenes' | 'nodes' | 'meshes' | 'accessors' | 'bufferViews' | 'buffers' | 'bufferBytes' | 'materials' | 'textures' | 'images' | 'samplers' | 'skins' | 'animations' | 'cameras' | 'extensionsUsed' | 'extensionsRequired' | 'extensions' | 'extras' | 'sourceForm';
/** ↩️ `inverse` restores the exact base and `diff` is the accepted sparse state transition. */
export interface GltfMutationDerivation { mutation: GltfMutation; diff: GltfDiff; inverse: GltfMutation; touchedPaths: string[]; touchedRegions: GltfTouchedRegion[]; referenceRules: GltfReferenceRule[] }
export interface GltfReferenceRule { family: 'scene' | 'node' | 'mesh' | 'accessor' | 'material' | 'buffer'; pathPattern: string; validate: boolean; remapOnInsert: boolean; remapOnRemove: boolean }
/** 🔗 Includes primitive attributes, indices, and every morph-target semantic accessor reference. */
export const GLTF_ACCESSOR_REFERENCE_RULES: readonly GltfReferenceRule[] = [
  { family: 'accessor', pathPattern: 'document/meshes/{mesh}/primitives/{primitive}/attributes/{semantic}', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/meshes/{mesh}/primitives/{primitive}/targets/{target}/{semantic}', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/meshes/{mesh}/primitives/{primitive}/indices', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/skins/{skin}/inverseBindMatrices', validate: true, remapOnInsert: true, remapOnRemove: true },
  { family: 'accessor', pathPattern: 'document/animations/{animation}/samplers/{sampler}/{input|output}', validate: true, remapOnInsert: true, remapOnRemove: true },
];
export const GLTF_STRUCTURAL_REFERENCE_RULES: readonly GltfReferenceRule[] = [
  { family: 'node', pathPattern: 'incomingInsertNode/children/{slot}', validate: true, remapOnInsert: true, remapOnRemove: false },
  { family: 'buffer', pathPattern: 'document/buffers|snapshot/buffers', validate: true, remapOnInsert: true, remapOnRemove: true },
];

export type GltfMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: GltfSnapshot }
  | { mutation: 'setAsset'; asset: GltfAsset }
  | { mutation: 'insertScene'; index: number; scene: GltfScene }
  | { mutation: 'removeScene'; index: number }
  | { mutation: 'setScene'; index: number; scene: GltfScene }
  | { mutation: 'insertNode'; index: number; node: GltfNode }
  | { mutation: 'removeNode'; index: number }
  | { mutation: 'setNode'; index: number; node: GltfNode }
  | { mutation: 'transformNode'; index: number; matrix?: [number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number]; translation?: [number, number, number]; rotation?: [number, number, number, number]; scale?: [number, number, number] }
  | { mutation: 'reparentNode'; index: number; parent?: number; scene?: number; position: number }
  | { mutation: 'bindNodeMesh'; index: number; mesh?: number }
  | { mutation: 'insertMesh'; index: number; mesh: GltfMesh }
  | { mutation: 'removeMesh'; index: number }
  | { mutation: 'setMesh'; index: number; mesh: GltfMesh }
  | { mutation: 'insertAccessor'; index: number; accessor: GltfAccessor }
  | { mutation: 'removeAccessor'; index: number }
  | { mutation: 'setAccessor'; index: number; accessor: GltfAccessor }
  | { mutation: 'insertMaterial'; index: number; material: GltfMaterial }
  | { mutation: 'removeMaterial'; index: number }
  | { mutation: 'setMaterial'; index: number; material: GltfMaterial }
  | { mutation: 'bindPrimitiveMaterial'; mesh: number; primitive: number; material?: number }
  | { mutation: 'insertBuffer'; index: number; buffer: GltfBuffer; bytes: number[] }
  | { mutation: 'removeBuffer'; index: number }
  | { mutation: 'setBuffer'; index: number; buffer: GltfBuffer; bytes: number[] }
  | { mutation: 'insertAnimation'; index: number; animation: GltfAnimation }
  | { mutation: 'removeAnimation'; index: number }
  | { mutation: 'setAnimation'; index: number; animation: GltfAnimation };
