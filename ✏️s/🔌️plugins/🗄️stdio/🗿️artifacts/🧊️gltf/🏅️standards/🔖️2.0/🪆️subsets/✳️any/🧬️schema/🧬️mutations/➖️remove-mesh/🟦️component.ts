//#region 🦠️Mutation
/** 🦠️ remove-mesh payload. */
export interface RemoveMesh { index: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ remove-mesh sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type RemoveMeshDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ RemoveMesh semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type RemoveMeshInverse = GltfMutation;
//#endregion ↩️Inverse
