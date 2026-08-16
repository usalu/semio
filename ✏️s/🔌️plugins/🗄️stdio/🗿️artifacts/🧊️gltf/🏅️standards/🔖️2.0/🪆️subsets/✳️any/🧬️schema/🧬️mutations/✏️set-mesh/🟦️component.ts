//#region 🦠️Mutation
/** 🦠️ set-mesh payload. */
import type { GltfMesh } from '../../📸️snapshot/🟦️component.ts';
export interface SetMesh { index: number; mesh: GltfMesh }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ set-mesh sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type SetMeshDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ SetMesh semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type SetMeshInverse = GltfMutation;
//#endregion ↩️Inverse
