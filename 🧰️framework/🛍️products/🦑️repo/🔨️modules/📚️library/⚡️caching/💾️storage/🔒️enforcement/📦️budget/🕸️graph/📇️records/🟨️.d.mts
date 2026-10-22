/** 🧾️ The finite limits of the graph record format. */
export interface GraphRecordPolicyV1{readonly version:2;readonly projectsPerYield:number;readonly edgesPerRecord:number;readonly externalNodesPerRecord:number;readonly maximumRecordBytes:number;readonly compressionLevel:number;readonly internMinimumItems:number}
/** 📇️ A pinned graph as content-addressed gzip records and a small manifest. */
export interface EncodedGraphRecordsV1{readonly manifestBytes:Buffer;readonly digest:string;readonly records:ReadonlyMap<string,Buffer>;readonly recordBytes:number;readonly bytes:number}
/** 📇️ Encodes a build graph without ever holding it as one string; refuses beyond `maximumBytes` compressed bytes. */
export function encodeGraphRecordsV1(graph:unknown,options:{policy:GraphRecordPolicyV1;maximumBytes:number}):Promise<EncodedGraphRecordsV1>;
/** 🧷️ Collapses long identical string arrays to one retained array. */
export function createArrayInternerV1(minimumItems:number):<T>(value:T)=>T;
/** 📖️ Decodes a pinned graph from its manifest and a record reader. */
export function decodeGraphRecordsV1(manifestBytes:Uint8Array,readRecord:(digest:string)=>Promise<Uint8Array|undefined>,options:{policy:GraphRecordPolicyV1}):Promise<any>;
/** 🔗️ Names the record digests one manifest holds. */
export function graphRecordDigestsV1(manifestBytes:Uint8Array):Set<string>;
