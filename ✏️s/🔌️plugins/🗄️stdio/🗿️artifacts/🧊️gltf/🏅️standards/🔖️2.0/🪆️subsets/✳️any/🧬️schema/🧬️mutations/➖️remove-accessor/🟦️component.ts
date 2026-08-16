//#region 🦠️Mutation
/** 🦠️ remove-accessor payload. */
export interface RemoveAccessor { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-accessor sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveAccessorDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveAccessor semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveAccessorInverse = GltfMutation;
//#endregion ↩️Inverse
