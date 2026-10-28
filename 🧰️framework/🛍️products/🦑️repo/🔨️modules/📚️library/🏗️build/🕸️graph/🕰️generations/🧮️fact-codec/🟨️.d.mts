/** 🧮️ The declared constants of the binary fact index. */
export interface FactCodecPolicyV1{readonly version:1;readonly magic:string;readonly maximumDepth:number;readonly maximumVarintBytes:number;readonly tags:Readonly<Record<"null"|"false"|"true"|"uint"|"negint"|"float"|"string"|"array"|"object"|"hex",number>>}
export const FACT_CODEC_POLICY_V1:FactCodecPolicyV1;
export interface FactCodecRowV1{kind:string;path:string;fact:unknown}
export interface FactCodecIndexV1{version:1;owners:string[];rows:FactCodecRowV1[];classes:{r:number[];o:number[]}[]}
export function compareCodePointsV1(a:string,b:string):-1|0|1;
export function canonicalJsonV1(value:unknown):string;
export function encodeFactIndexV1(index:FactCodecIndexV1,options:{maximumBytes:number}):Buffer;
export function decodeFactIndexV1(bytes:Uint8Array,options?:{maximumBytes?:number;share?:(text:string)=>string;build?:true}):FactCodecIndexV1;
export function decodeFactIndexV1(bytes:Uint8Array,options:{maximumBytes?:number;share?:(text:string)=>string;build:false}):{strings:string[]};
/** 🪶️ A fact index opened for verification: `rows` materialises a row when it is read, `rowKindOf` answers the kind of a row without reading it. */
export interface FactCodecOpenIndexV1 extends FactCodecIndexV1{readonly rowKindOf:(id:number)=>string;readonly rowsRead:()=>number}
export function openFactIndexV1(bytes:Uint8Array,options?:{maximumBytes?:number;share?:(text:string)=>string}):FactCodecOpenIndexV1;
