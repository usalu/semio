import{readFileSync,writeFileSync,renameSync,rmSync,statSync}from"node:fs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const policy=schema.definitions.Policy.const,rowPattern=new RegExp(schema.definitions.Memo.properties.rows.additionalProperties.pattern,"u");
/** 🧊️ Is the file identity a digest stays valid for: device, inode, size and nanosecond mtime. */
export type DigestStampV1=Readonly<{dev:bigint;ino:bigint;size:bigint;mtimeNs:bigint}>;
/** 🔑️ Renders a stamp as the colon-joined decimal fields named by the policy. */
export const digestStampKeyV1=(stamp:DigestStampV1):string=>policy.stampFields.map(field=>String(stamp[field as keyof DigestStampV1])).join(":");
/** 🧊️ Is a persisted path to digest memo shared by every process of one store: a row answers only for the exact stamp it was computed under. */
export interface DigestMemoV1{get(path:string,stamp:DigestStampV1):string|undefined;set(path:string,stamp:DigestStampV1,digest:string):void;flush():void}

/** 🧊️ Opens the memo file lazily; an absent, oversized, malformed or foreign file is an empty memo and a failed write never fails a caller.
 * @see ./🧬️schema/🔣️.json */
export function openDigestMemoV1(file:string):DigestMemoV1{
 let rows:Map<string,string>|undefined,dirty=false;const touched=new Set<string>();
 const load=():Map<string,string>=>{
  if(rows)return rows;rows=new Map();
  try{if(statSync(file).size>policy.maximumBytes)return rows;const value=JSON.parse(readFileSync(file,"utf8"))as{version?:unknown;rows?:Record<string,unknown>};
   if(value.version!==policy.version||value.rows===null||typeof value.rows!=="object")return rows;
   for(const[path,row]of Object.entries(value.rows))if(typeof row==="string"&&rowPattern.test(row))rows.set(path,row);
  }catch{rows=new Map();}
  return rows;
 };
 return{
  get(path,stamp){const row=load().get(path),key=digestStampKeyV1(stamp)+":";if(row!==undefined&&row.startsWith(key)){touched.add(path);return row.slice(key.length);}return undefined;},
  set(path,stamp,digest){load().set(path,digestStampKeyV1(stamp)+":"+digest);touched.add(path);dirty=true;},
  flush(){
   if(!dirty||!rows)return;dirty=false;
   const kept=rows.size>policy.maximumRows?new Map([...rows].filter(([path])=>touched.has(path))):rows,temporary=file+"."+process.pid+".tmp";
   try{writeFileSync(temporary,JSON.stringify({version:policy.version,rows:Object.fromEntries(kept)}),{mode:0o600});renameSync(temporary,file);rows=new Map(kept);}catch{try{rmSync(temporary,{force:true});}catch{}}
  },
 };
}
