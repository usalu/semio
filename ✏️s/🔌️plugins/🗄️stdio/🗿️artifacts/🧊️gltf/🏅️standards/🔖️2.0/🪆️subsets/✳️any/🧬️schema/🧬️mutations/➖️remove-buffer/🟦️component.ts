//#region 🦠️Mutation
/** 🦠️ remove-buffer payload. */
export interface RemoveBuffer { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-buffer sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveBufferDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveBuffer semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveBufferInverse = GltfMutation;
//#endregion ↩️Inverse
