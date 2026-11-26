/** 💾️ The durable node store. */
export interface NodeStoreV1{get(id:string):Promise<any|undefined>;put(node:unknown,params:unknown):Promise<void>;flush():Promise<void>;prune(options?:{now?:number}):Promise<{removed:number;kept:number}>}
export function createNodeStoreV1(options:{root:string;fs?:unknown;blobs?:{put(bytes:Uint8Array):Promise<{hash:string}>;read(hash:string):Promise<Uint8Array|undefined>;pin?(holder:string,hashes:readonly string[],ttl?:number):Promise<void>};limits?:unknown}):NodeStoreV1;

/** 📁️ The file port of the node store: read, atomic write, list, remove and make directory. */
export interface NodeFilePortV1{read(path:string):Promise<Uint8Array|undefined>;write(path:string,bytes:Uint8Array):Promise<void>;list(directory:string):Promise<{name:string;path:string;mtimeMs:number}[]>;remove(path:string):Promise<void>;mkdir(directory:string):Promise<unknown>}
/** 📁 The node file port of the durable stores. */
export const NODE_FILE_PORT_V1: Readonly<NodeFilePortV1>;
