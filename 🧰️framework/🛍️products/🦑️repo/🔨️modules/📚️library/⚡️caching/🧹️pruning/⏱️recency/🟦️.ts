/** 🧾️ Carries bounded metadata observations without granting file or retirement custody. */
export type RecencyFileV1=Readonly<{path:string;device:string;inode:string;bytes:string;atimeNs:string;mtimeNs:string;ctimeNs:string}>;
/** 🔎️ Identifies the exact file and clock field supplying a sampled maximum. */
export function summarizeRecencyFilesV1(rows:readonly RecencyFileV1[]){
 if(rows.length>4096)throw Error("Bounded recency metadata required");let bytes=0n,maximum:bigint|null=null;const names=new Set<string>(),ordered=[...rows].sort((a,b)=>a.path<b.path?-1:a.path>b.path?1:0);
 for(const row of ordered){if(!row.path||new TextEncoder().encode(row.path).length>256||names.has(row.path)||[row.device,row.inode,row.bytes,row.atimeNs,row.mtimeNs,row.ctimeNs].some(value=>!/^\d{1,32}$/u.test(value)))throw Error("Exact bounded regular file facts required");names.add(row.path);bytes+=BigInt(row.bytes);for(const field of ["atimeNs","mtimeNs"]as const){const value=BigInt(row[field]);if(maximum===null||value>maximum)maximum=value;}}
 const sources:Readonly<RecencyFileV1&{field:"atimeNs"|"mtimeNs"}>[]=[];for(const row of ordered)for(const field of ["atimeNs","mtimeNs"]as const)if(BigInt(row[field])===maximum&&sources.length<8)sources.push({...row,field});return{files:rows.length,bytes:String(bytes),maximumNs:maximum===null?null:String(maximum),sources};
}
