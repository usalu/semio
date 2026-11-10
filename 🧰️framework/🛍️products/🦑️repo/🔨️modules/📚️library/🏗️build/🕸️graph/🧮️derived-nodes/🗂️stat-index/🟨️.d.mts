export interface StatV1{size:number;mtimeMs:number;ino:number}
export interface StatWorldV1{stat(path:string):Promise<StatV1|undefined>;read(path:string):Promise<Uint8Array|undefined>}
/** 🗂️ The per-directory durable stat index. */
export interface StatIndexV1{digest(path:string,world:StatWorldV1,signal?:AbortSignal):Promise<{digest:string;read:boolean;bytes?:Uint8Array}>;flush():Promise<void>;prune(options?:{at?:number}):Promise<{removed:number;kept:number}>;stats():{directories:number;entries:number;dirty:number};directory(dir:string):Promise<{mtimeMs:number;at:number;entries:[string,number][]}|undefined>;directoryNow(dir:string):{mtimeMs:number;at:number;entries:[string,number][]}|undefined;remember(dir:string,record:{mtimeMs:number;at:number;entries:[string,number][]}):Promise<void>}
export function encodeShardV1(shard:{files:Map<string,Map<string,unknown[]>>;dirs:Map<string,unknown>}):Buffer;
export function decodeShardV1(bytes:Uint8Array,maximumBytes:number):{files:Map<string,Map<string,unknown[]>>;dirs:Map<string,any>}|undefined
export function createStatIndexV1(options:{root:string;fs?:unknown;limits?:unknown;now?:()=>number}):StatIndexV1;
