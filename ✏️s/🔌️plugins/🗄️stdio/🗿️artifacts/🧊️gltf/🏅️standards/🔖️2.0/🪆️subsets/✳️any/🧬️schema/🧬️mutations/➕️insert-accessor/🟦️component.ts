//#region 🦠️Mutation
/** 🦠️ insert-accessor payload. */
import type { GltfAccessor } from '../../📸️snapshot/🟦️component.ts';
export interface InsertAccessor { index: number; accessor: GltfAccessor }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-accessor sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertAccessorDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertAccessor semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertAccessorInverse = GltfMutation;
//#endregion ↩️Inverse
