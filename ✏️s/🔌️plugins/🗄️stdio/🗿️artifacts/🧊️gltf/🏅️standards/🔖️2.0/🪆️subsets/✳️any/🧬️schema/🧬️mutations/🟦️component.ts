/** 🧬 Complete semantic glTF mutation vocabulary; binary ordinals are frozen at 0–27. */
import type { NoMutation } from './🚫️no-mutation/🦠️mutation/🟦️component.ts';
import type { SetSnapshot } from './📄set-snapshot/🦠️mutation/🟦️component.ts';
import type { SetAsset } from './🏷️set-asset/🦠️mutation/🟦️component.ts';
import type { InsertScene } from './➕️insert-scene/🦠️mutation/🟦️component.ts';
import type { RemoveScene } from './➖️remove-scene/🦠️mutation/🟦️component.ts';
import type { SetScene } from './✏️set-scene/🦠️mutation/🟦️component.ts';
import type { InsertNode } from './➕️insert-node/🦠️mutation/🟦️component.ts';
import type { RemoveNode } from './➖️remove-node/🦠️mutation/🟦️component.ts';
import type { SetNode } from './✏️set-node/🦠️mutation/🟦️component.ts';
import type { InsertMesh } from './➕️insert-mesh/🦠️mutation/🟦️component.ts';
import type { RemoveMesh } from './➖️remove-mesh/🦠️mutation/🟦️component.ts';
import type { SetMesh } from './✏️set-mesh/🦠️mutation/🟦️component.ts';
import type { InsertAccessor } from './➕️insert-accessor/🦠️mutation/🟦️component.ts';
import type { RemoveAccessor } from './➖️remove-accessor/🦠️mutation/🟦️component.ts';
import type { SetAccessor } from './✏️set-accessor/🦠️mutation/🟦️component.ts';
import type { InsertMaterial } from './➕️insert-material/🦠️mutation/🟦️component.ts';
import type { RemoveMaterial } from './➖️remove-material/🦠️mutation/🟦️component.ts';
import type { SetMaterial } from './✏️set-material/🦠️mutation/🟦️component.ts';
import type { InsertBuffer } from './➕️insert-buffer/🦠️mutation/🟦️component.ts';
import type { RemoveBuffer } from './➖️remove-buffer/🦠️mutation/🟦️component.ts';
import type { SetBuffer } from './✏️set-buffer/🦠️mutation/🟦️component.ts';
import type { InsertAnimation } from './➕️insert-animation/🦠️mutation/🟦️component.ts';
import type { RemoveAnimation } from './➖️remove-animation/🦠️mutation/🟦️component.ts';
import type { SetAnimation } from './✏️set-animation/🦠️mutation/🟦️component.ts';
import type { TransformNode } from './🔄️transform-node/🦠️mutation/🟦️component.ts';
import type { ReparentNode } from './🌳️reparent-node/🦠️mutation/🟦️component.ts';
import type { BindNodeMesh } from './🔗️bind-node-mesh/🦠️mutation/🟦️component.ts';
import type { BindPrimitiveMaterial } from './🔗️bind-primitive-material/🦠️mutation/🟦️component.ts';
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
  | ({ mutation: 'noMutation' } & NoMutation)
  | ({ mutation: 'setSnapshot' } & SetSnapshot)
  | ({ mutation: 'setAsset' } & SetAsset)
  | ({ mutation: 'insertScene' } & InsertScene)
  | ({ mutation: 'removeScene' } & RemoveScene)
  | ({ mutation: 'setScene' } & SetScene)
  | ({ mutation: 'insertNode' } & InsertNode)
  | ({ mutation: 'removeNode' } & RemoveNode)
  | ({ mutation: 'setNode' } & SetNode)
  | ({ mutation: 'insertMesh' } & InsertMesh)
  | ({ mutation: 'removeMesh' } & RemoveMesh)
  | ({ mutation: 'setMesh' } & SetMesh)
  | ({ mutation: 'insertAccessor' } & InsertAccessor)
  | ({ mutation: 'removeAccessor' } & RemoveAccessor)
  | ({ mutation: 'setAccessor' } & SetAccessor)
  | ({ mutation: 'insertMaterial' } & InsertMaterial)
  | ({ mutation: 'removeMaterial' } & RemoveMaterial)
  | ({ mutation: 'setMaterial' } & SetMaterial)
  | ({ mutation: 'insertBuffer' } & InsertBuffer)
  | ({ mutation: 'removeBuffer' } & RemoveBuffer)
  | ({ mutation: 'setBuffer' } & SetBuffer)
  | ({ mutation: 'insertAnimation' } & InsertAnimation)
  | ({ mutation: 'removeAnimation' } & RemoveAnimation)
  | ({ mutation: 'setAnimation' } & SetAnimation)
  | ({ mutation: 'transformNode' } & TransformNode)
  | ({ mutation: 'reparentNode' } & ReparentNode)
  | ({ mutation: 'bindNodeMesh' } & BindNodeMesh)
  | ({ mutation: 'bindPrimitiveMaterial' } & BindPrimitiveMaterial);
