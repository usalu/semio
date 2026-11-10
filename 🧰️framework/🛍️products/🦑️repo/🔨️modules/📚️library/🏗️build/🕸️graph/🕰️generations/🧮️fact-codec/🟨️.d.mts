/** 🧮️ The declared constants of the binary fact index. */
export interface FactCodecPolicyV1{readonly version:1;readonly magic:string;readonly maximumDepth:number;readonly maximumVarintBytes:number;readonly view:Readonly<Record<"rowEntries"|"rowBytes"|"factEntries"|"factBytes"|"factBytesPerStored"|"textEntries"|"textBytes"|"classEntries"|"classBytes",number>>;readonly tags:Readonly<Record<"null"|"false"|"true"|"uint"|"negint"|"float"|"string"|"array"|"object"|"hex",number>>}
export const FACT_CODEC_POLICY_V1:FactCodecPolicyV1;
export interface FactCodecRowV1{kind:string;path:string;fact:unknown}
export interface FactCodecIndexV1{version:1;owners:string[];rows:FactCodecRowV1[];classes:{r:number[];o:number[]}[]}
export function compareCodePointsV1(a:string,b:string):-1|0|1;
export function canonicalJsonV1(value:unknown):string;
export function encodeFactIndexV1(index:FactCodecIndexV1,options:{maximumBytes:number}):Buffer;
export function decodeFactIndexV1(bytes:Uint8Array,options?:{maximumBytes?:number;share?:(text:string)=>string;build?:true}):FactCodecIndexV1;
export function decodeFactIndexV1(bytes:Uint8Array,options:{maximumBytes?:number;share?:(text:string)=>string;build:false}):{strings:string[]};
/** 📊️ One cache of an opened index (see `🧊️bounded`). */
export interface FactCodecCacheStatsV1{readonly name:string;readonly entries:number;readonly bytes:number;readonly maxEntries:number;readonly maxBytes:number;readonly hits:number;readonly misses:number;readonly evictions:number;readonly rejected:number;readonly loads:number}
/** 🪶️ A fact index opened as a bounded view: `rows` and `classes` are read only views that decode on demand into bounded caches, `rowKindOf` answers the kind of a row without reading it, `stats` lists the document size and the caches, `drop` empties the caches. */
export interface FactCodecOpenIndexV1 extends FactCodecIndexV1{readonly rowKindOf:(id:number)=>string;readonly rowsRead:()=>number;readonly stats:()=>{readonly bytes:number;readonly caches:readonly FactCodecCacheStatsV1[]};readonly drop:()=>void}
export function openFactIndexV1(bytes:Uint8Array,options?:{maximumBytes?:number;share?:(text:string)=>string;name?:string}):FactCodecOpenIndexV1;
