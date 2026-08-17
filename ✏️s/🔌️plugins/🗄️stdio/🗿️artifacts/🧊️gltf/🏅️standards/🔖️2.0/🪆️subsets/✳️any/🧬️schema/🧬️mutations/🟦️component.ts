/** 🧬 Open glTF mutation descriptor assembly. */
import type { GltfSnapshot } from '../📸️snapshot/🟦️component.ts';
import { GltfCreateSceneLeafDescriptor } from './create-scene/🟦️component.ts';

//#region 🔖️DescriptorContract
export interface GltfMutationLeafError { readonly code: string; readonly path: string; readonly detail: string }
export interface GltfMutationLeafPlan { readonly diffPayload: string; readonly inversePayload: string; readonly touchedPaths: readonly string[] }
export interface GltfMutationLeafApplication { readonly snapshot: GltfSnapshot; readonly touchedPaths: readonly string[] }
export type GltfMutationLeafResult<T> = { readonly accepted: true; readonly value: T } | { readonly accepted: false; readonly rejection: GltfMutationLeafError };
export interface GltfMutationLeafDescriptor {
  readonly commandId: string;
  readonly version: number;
  readonly plan: (payload: string, base: GltfSnapshot) => GltfMutationLeafResult<GltfMutationLeafPlan>;
  readonly planInverse: (payload: string, base: GltfSnapshot) => GltfMutationLeafResult<GltfMutationLeafPlan>;
  readonly applyDiff: (payload: string, base: GltfSnapshot) => GltfMutationLeafResult<GltfMutationLeafApplication>;
  readonly applyInverse: (payload: string, base: GltfSnapshot) => GltfMutationLeafResult<GltfMutationLeafApplication>;
}
//#endregion 🔖️DescriptorContract

//#region 🔖️Assembly
export const gltfMutationLeafDescriptors: readonly GltfMutationLeafDescriptor[] = [GltfCreateSceneLeafDescriptor];
//#endregion 🔖️Assembly
