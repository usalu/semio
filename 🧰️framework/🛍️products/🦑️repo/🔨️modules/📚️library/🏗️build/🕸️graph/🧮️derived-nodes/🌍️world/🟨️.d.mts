import type{WorldPortV1}from"../🟨️.mjs";
/** 🌍️ The world port over a directory. */
export function createDiskWorldV1(workspaceRoot:string,options?:{skip?:readonly string[];remember?:boolean;index?:import("../🗂️stat-index/🟨️.mjs").StatIndexV1;racyMilliseconds?:number;maximumDirectories?:number;now?:()=>number}):WorldPortV1&{verify(root:string,signal?:AbortSignal):Promise<string|undefined>;stamp(root:string):string|undefined;stat(path:string):Promise<{size:number;mtimeMs:number;ino:number}|undefined>};
