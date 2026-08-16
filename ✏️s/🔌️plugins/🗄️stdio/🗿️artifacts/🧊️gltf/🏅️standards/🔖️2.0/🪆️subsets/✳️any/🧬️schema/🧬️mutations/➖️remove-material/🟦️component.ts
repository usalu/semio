//#region 🦠️Mutation
/** 🦠️ remove-material payload. */
export interface RemoveMaterial { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-material sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveMaterialDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveMaterial semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveMaterialInverse = GltfMutation;
//#endregion ↩️Inverse
