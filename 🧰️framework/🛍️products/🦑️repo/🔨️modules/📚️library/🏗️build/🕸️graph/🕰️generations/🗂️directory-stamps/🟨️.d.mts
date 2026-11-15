/** 🗂️ The directory stamp table of the fact verification, kept in one file beside the generations. */
export interface DirectoryStampsV1{
 classify(directories:readonly string[],options?:{signal?:AbortSignal;progress?:(event:unknown)=>Promise<void>;observedAt?:number}):Promise<{unchanged:Set<string>;measured:Map<string,unknown>;links:Map<string,unknown>;missing:Set<string>}>;
 save(measured:Map<string,unknown>,exclude?:ReadonlySet<string>):Promise<void>;
}
export function createDirectoryStampsV1(options:{file:string;racyMilliseconds?:number;concurrency?:number;now?:()=>number;statDirectory?:(path:string)=>Promise<unknown>;listLinks?:(path:string)=>Promise<readonly string[]|undefined>}):DirectoryStampsV1;
