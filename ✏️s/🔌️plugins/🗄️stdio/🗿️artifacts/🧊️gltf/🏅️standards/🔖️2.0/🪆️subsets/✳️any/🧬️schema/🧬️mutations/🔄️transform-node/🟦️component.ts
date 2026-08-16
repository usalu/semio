//#region 🦠️Mutation
/** 🦠️ transform-node payload. */
export interface TransformNode { index: number; matrix?: [number, number, number, number, number, number, number, number, number, number, number, number, number, number, number, number]; translation?: [number, number, number]; rotation?: [number, number, number, number]; scale?: [number, number, number] }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ transform-node sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type TransformNodeDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ TransformNode semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type TransformNodeInverse = GltfMutation;
//#endregion ↩️Inverse
