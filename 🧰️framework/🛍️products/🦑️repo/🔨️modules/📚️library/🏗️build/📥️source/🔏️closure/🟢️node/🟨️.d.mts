export declare const buildSourceClosureNodeLoaderV1:string;
export declare function buildSourceClosureNodeArgumentsV1(snapshot:unknown,policy:unknown,heapMiB:number,executable:string):Readonly<{argv:readonly string[];bootstrap:string;digest:string;commandUnits:number}>;

/** 📥️ Selects the closed Native stdin variant of the same immutable Node kernel. */
export function buildSourceClosureNodeStdinLoaderV1(policy:Readonly<Record<string,number>>):string;
