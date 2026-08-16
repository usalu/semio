//#region 🦠️Mutation
/** 🦠️ remove-node payload. */
export interface RemoveNode { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-node sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveNodeDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveNode semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveNodeInverse = GltfMutation;
//#endregion ↩️Inverse
