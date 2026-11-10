/** 💾️ The durable node store. */
export interface NodeStoreV1{get(id:string):Promise<any|undefined>;put(node:unknown,params:unknown):Promise<void>;flush():Promise<void>;prune(options?:{now?:number}):Promise<{removed:number;kept:number}>}
export function createNodeStoreV1(options:{root:string;fs?:unknown;blobs?:{put(bytes:Uint8Array):Promise<{hash:string}>;read(hash:string):Promise<Uint8Array|undefined>;pin?(holder:string,hashes:readonly string[],ttl?:number):Promise<void>};limits?:unknown}):NodeStoreV1;
