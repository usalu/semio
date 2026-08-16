//#region 🦠️Mutation
/** 🦠️ insert-mesh payload. */
import type { GltfMesh } from '../../📸️snapshot/🟦️component.ts';
export interface InsertMesh { index: number; mesh: GltfMesh }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-mesh sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertMeshDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertMesh semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertMeshInverse = GltfMutation;
//#endregion ↩️Inverse
