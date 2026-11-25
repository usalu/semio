/** 🌳️ The declared policy of the module closure graph. */
export const CLOSURE_GRAPH_POLICY_V1:Readonly<{version:1;fact:"closure-graph";directory:string;buckets:number;compressionQuality:number;chunkGraceMilliseconds:number;saveDelayMilliseconds:number;fleet:Readonly<{domain:string;retentionManifest:string}>}>;
/** 🏷️ The name of the `global` row a command walk is recorded as. */
export const CLOSURE_GRAPH_FACT_V1:"closure-graph";
/** 🧩️ One node of the graph: a module or resource with the digest of what a walk reads of it and the keys of the nodes it imports. */
export type ClosureGraphNodeV1=Readonly<{key:string;name:string;kind:string;digest:string;edges:readonly string[];extra?:unknown}>;
/** 🌳️ One module closure graph. */
export type ClosureGraphV1=Readonly<{
 update(put?:Iterable<ClosureGraphNodeV1>,removed?:Iterable<string>):{forgotten:number};
 get(key:string):ClosureGraphNodeV1|undefined;
 has(key:string):boolean;
 records():IterableIterator<ClosureGraphNodeV1>;
 closureDigest(roots:readonly string[],options?:{stops?:ReadonlyMap<string,string>;redirect?:ReadonlyMap<string,string>}):string|undefined;
 reach(roots:readonly string[],options?:{stops?:ReadonlyMap<string,string>;redirect?:ReadonlyMap<string,string>}):{names:string[];tools:string[];keys:string[]};
 missing(roots:readonly string[]):string[];
 stats():{nodes:number;edges:number;families:number;digests:number};
}>;
/** 🧾️ The digest of what a walk reads of one module. */
export function moduleNodeDigestV1(content:unknown):string;
/** 🏷️ The entry digest of one reached node. */
export const closureEntryDigestV1:(name:string,digest:string)=>string;
/** 🛑️ The entry digest of one tool a closure is cut at. */
export const closureToolDigestV1:(name:string)=>string;
/** 🌳️ Creates a module closure graph. */
export function createClosureGraphV1():ClosureGraphV1;
/** 💾️ Stores a graph digest addressed under a directory. */
export function saveClosureGraphV1(graph:ClosureGraphV1,directory:string,options?:{signal?:AbortSignal;progress?:(step:{done:number;total:number})=>unknown;policy?:typeof CLOSURE_GRAPH_POLICY_V1}):Promise<{chunks:number;written:number;bytes:number;nodes:number;edges:number;head:string}>;
/** 📂️ Loads the graph a directory holds. */
export function loadClosureGraphV1(directory:string,options?:{signal?:AbortSignal;progress?:(step:{done:number;total:number})=>unknown}):Promise<(ClosureGraphV1&{readonly head:string})|undefined>;
/** 📂️ Loads the graph a directory holds without turns. */
export function loadClosureGraphSyncV1(directory:string):(ClosureGraphV1&{readonly head:string})|undefined;
