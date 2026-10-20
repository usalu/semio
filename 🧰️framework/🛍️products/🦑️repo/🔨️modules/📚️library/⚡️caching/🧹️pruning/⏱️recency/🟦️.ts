import schema from"./🧬️schema/🔣️.json" with { type: "json" };
/** 🧾️ Carries bounded metadata observations without granting file or retirement custody. */
export type RecencyFileV1=Readonly<{path:string;device:string;inode:string;bytes:string;atimeNs:string;mtimeNs:string;ctimeNs:string}>;
/** 🔎️ Identifies the exact file and clock field supplying a sampled maximum. */
export function summarizeRecencyFilesV1(rows:readonly RecencyFileV1[]){
 if(rows.length>4096)throw Error("Bounded recency metadata required");let bytes=0n,maximum:bigint|null=null;const names=new Set<string>(),ordered=[...rows].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
 for(const row of ordered){if(!row.path||new TextEncoder().encode(row.path).length>256||names.has(row.path)||[row.device,row.inode,row.bytes,row.atimeNs,row.mtimeNs,row.ctimeNs].some(value=>!/^\d{1,32}$/u.test(value)))throw Error("Exact bounded regular file facts required");names.add(row.path);bytes+=BigInt(row.bytes);for(const field of ["atimeNs","mtimeNs"]as const){const value=BigInt(row[field]);if(maximum===null||value>maximum)maximum=value;}}
 const sources:Readonly<RecencyFileV1&{field:"atimeNs"|"mtimeNs"}>[]=[];for(const row of ordered)for(const field of ["atimeNs","mtimeNs"]as const)if(BigInt(row[field])===maximum&&sources.length<8)sources.push({...row,field});return{files:rows.length,bytes:String(bytes),maximumNs:maximum===null?null:String(maximum),sources};
}

/** 📐️ Bounds diagnostic metadata only; this decision grants no storage or retirement authority. */
export function fitsRecencySourceBytesV1(usedBytes:number,sourceBytes:number):boolean{const policy=schema.definitions.ProvenancePolicy.const;return Number.isSafeInteger(usedBytes)&&usedBytes>=0&&usedBytes<=policy.maximumScanSourceBytes&&Number.isSafeInteger(sourceBytes)&&sourceBytes>0&&sourceBytes<=policy.maximumSourceBytes&&sourceBytes<=policy.maximumScanSourceBytes-usedBytes;}

/** 🧩️ Carries measured diagnostic data without proving storage or writer ownership. */
type RecencyComparisonUnitV1=Readonly<{path:string;bytes:number;recencyMs:number;lockHeld:boolean;recencySource?:Readonly<RecencyFileV1&{field:"atimeNs"|"mtimeNs"}>}>;
/** 📣️ Formats original and recensus facts after the unchanged retirement guard has refused. */
export function describeCacheRecencyChangeV1(root:string,expected:RecencyComparisonUnitV1,current:RecencyComparisonUnitV1|undefined){
 const describe=(unit:RecencyComparisonUnitV1,capture:boolean)=>({bytes:unit.bytes,recencyMs:unit.recencyMs,lockHeld:unit.lockHeld,recencySource:capture?unit.recencySource??null:null,sourceStatus:capture&&unit.recencySource?"captured":"not-retained"});
 const build=(capture:boolean)=>({root:root.slice(0,256),unit:expected.path.slice(0,256),expected:describe(expected,capture),current:current?describe(current,capture):null,changed:{missing:!current,bytes:Boolean(current&&current.bytes!==expected.bytes),recency:Boolean(current&&current.recencyMs!==expected.recencyMs),lockHeld:Boolean(current?.lockHeld)}}),value=build(true);return Buffer.byteLength(JSON.stringify(value))<=schema.definitions.ProvenancePolicy.const.maximumMismatchBytes?value:build(false);
}
