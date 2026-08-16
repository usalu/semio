//#region 🦠️Mutation
/** 🦠️ insert-buffer payload. */
import type { GltfBuffer } from '../../📸️snapshot/🟦️component.ts';
export interface InsertBuffer { index: number; buffer: GltfBuffer; bytes: number[] }
//#endregion 🦠️Mutation

//#region 🔺️Diff
/** 🔺️ insert-buffer sparse diff. */
import type { GltfDiff } from '../../🔺️diff/🟦️component.ts';
export type InsertBufferDiff = GltfDiff;
//#endregion 🔺️Diff

//#region ↩️Inverse
/** ↩️ InsertBuffer semantic inverse. */
import type { GltfMutation } from '../../../🔨️modules/🧭️mutation-dispatch/🟦️component.ts';
export type InsertBufferInverse = GltfMutation;
//#endregion ↩️Inverse
