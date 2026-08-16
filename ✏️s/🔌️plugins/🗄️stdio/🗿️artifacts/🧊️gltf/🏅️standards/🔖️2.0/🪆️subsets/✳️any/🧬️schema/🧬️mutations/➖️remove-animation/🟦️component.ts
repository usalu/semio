//#region 🦠️Mutation
/** 🦠️ remove-animation payload. */
export interface RemoveAnimation { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-animation sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveAnimationDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveAnimation semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveAnimationInverse = GltfMutation;
//#endregion ↩️Inverse
