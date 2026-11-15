import type {DirectoryStampsV1} from "../🗂️directory-stamps/🟨️.mjs";
/** 🔎️ Decides which fact rows need the precise check from the directory stamps and the file stamps of one verify epoch. */
export interface EpochDoubtV1{
 classify(rows:readonly unknown[],options?:{signal?:AbortSignal;progress?:(event:unknown)=>Promise<void>}):Promise<Uint8Array>;
 settled(input:{rows:readonly unknown[];ids:readonly number[];stale:ReadonlySet<number>}):Promise<void>;
 published(input:{rows:readonly unknown[];observedAt:number}):Promise<void>;
 stats():{directories:number;files:number};
}
export function createEpochDoubtV1(options:{stamps:DirectoryStampsV1;statFile?:(path:string)=>Promise<unknown>;concurrency?:number}):EpochDoubtV1;
