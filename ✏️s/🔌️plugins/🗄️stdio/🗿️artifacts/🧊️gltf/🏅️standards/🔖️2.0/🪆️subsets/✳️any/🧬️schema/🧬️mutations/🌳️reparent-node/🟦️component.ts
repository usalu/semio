//#region 🦠️Mutation
/** 🦠️ reparent-node payload. */
export interface ReparentNode { index: number; parent?: number; scene?: number; position: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ reparent-node sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type ReparentNodeDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ ReparentNode semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type ReparentNodeInverse = GltfMutation;
//#endregion ↩️Inverse
