//#region 🦠️Mutation
/** 🦠️ bind-node-mesh payload. */
export interface BindNodeMesh { index: number; mesh?: number }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ bind-node-mesh sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type BindNodeMeshDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ BindNodeMesh semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type BindNodeMeshInverse = GltfMutation;
//#endregion ↩️Inverse
