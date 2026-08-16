//#region 🦠️Mutation
/** 🦠️ insert-node payload. */
import type { GltfNode } from '../../📸️snapshot/🟦️component.ts';
export interface InsertNode { index: number; node: GltfNode }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-node sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertNodeDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertNode semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertNodeInverse = GltfMutation;
//#endregion ↩️Inverse
