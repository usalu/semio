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
export interface FactCodecOpenIndexV1 extends FactCodecIndexV1{readonly rowKindOf:(id:number)=>string;readonly rowsRead:()=>number;readonly stats:()=>{readonly bytes:number;readonly caches:readonly FactCodecCacheStatsV1[]};readonly drop:()=>void;readonly document:FactCodecDocumentV1}
/** 🧭️ The offset tables of the packed bytes of an opened index, for a streaming reader of the same layout: the document, the number of its strings, kinds and owners with the string rank of each, the byte span of every fact, the kind, path and fact of every row, and the front coding of the string table (`lcps`, `suffixAt`, `suffixLength`; `textAt` decodes one string; `classTyped` reads the two lists of one class straight into typed arrays, bypassing the caches). */
export interface FactCodecDocumentV1{readonly buffer:Buffer;readonly stringCount:number;readonly kinds:readonly string[];readonly kindString:Uint32Array;readonly ownerString:Uint32Array;readonly factCount:number;readonly factAt:Uint32Array;readonly factsEnd:number;readonly rowCount:number;readonly rowKind:Uint16Array;readonly rowPath:Uint32Array;readonly rowFact:Uint32Array;readonly classCount:number;readonly classTyped:(id:number)=>{r:Uint32Array;o:Uint32Array};readonly suffixAt:Uint32Array;readonly suffixLength:Uint32Array;readonly lcps:Uint32Array;readonly textAt:(id:number,remember?:boolean)=>string}
/** ✍️ A growable byte sink that refuses to grow beyond the finite document limit. */
export interface FactSinkV1{byte(value:number):void;varint(value:number):void;bytes(source:Uint8Array,from?:number):void;size():number;truncate(size:number):void;result():Buffer}
export function collectFactStringsV1(value:unknown,strings:Set<string>):void;
export function writeFactValueV1(sink:FactSinkV1,value:unknown,textId:(text:string)=>number,depth:number):void;
export function writeFactListV1(sink:FactSinkV1,list:ArrayLike<number>):void;
export function createFactSinkV1(maximumBytes:number):FactSinkV1;
export function openFactIndexV1(bytes:Uint8Array,options?:{maximumBytes?:number;share?:(text:string)=>string;name?:string}):FactCodecOpenIndexV1;
