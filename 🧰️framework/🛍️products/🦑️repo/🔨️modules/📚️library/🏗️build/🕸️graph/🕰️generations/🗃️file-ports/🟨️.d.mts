/** 🗂️ The filesystem port on real files. */
export function createNodeFilePortV1():Readonly<{read(path:string):Promise<Buffer|undefined>;write(path:string,bytes:Uint8Array,options?:{ifAbsent?:boolean}):Promise<boolean>;names(directory:string):Promise<string[]>;list(directory:string):Promise<{name:string;modifiedMs:number}[]>;remove(path:string,options?:{singleLink?:boolean;signal?:AbortSignal}):Promise<void>}>;
/** ⏱️ Waits some milliseconds, honouring an abort signal. */
export const defaultSleep:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;
/** 🔒️ A reference exclusive lease over lock files. */
export function createFileLeasePortV1(directory:string,options?:{staleMilliseconds?:number;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;onHold?:(row:{resource:string;heldMs:number;waitedMs:number})=>void}):Readonly<{exclusive<T>(resource:string,body:()=>Promise<T>,options?:{signal?:AbortSignal;timeoutMs?:number}):Promise<T>}>;
