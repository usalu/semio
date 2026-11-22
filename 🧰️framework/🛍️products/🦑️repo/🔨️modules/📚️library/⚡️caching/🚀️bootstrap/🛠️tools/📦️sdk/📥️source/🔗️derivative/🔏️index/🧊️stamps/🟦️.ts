import{createHash,randomBytes}from"node:crypto";
import{existsSync,type BigIntStats}from"node:fs";
import{mkdir,readFile,rename,rm,writeFile}from"node:fs/promises";
import{dirname,isAbsolute,join}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{nxAuthorizerMemoBaseV1}from"../../../../../🔏️authorizers/🧊️memo/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
export const nxSdkStampPolicyV1=schema.definitions.Policy.const;
const policy=nxSdkStampPolicyV1;
/** 🏷️ The identity facts of one immutable base file; ctime and link count are left out because every derivative generation hardlinks the file and changes them without changing a byte. */
export type NxSdkStampFactsV1=Pick<BigIntStats,"dev"|"ino"|"size"|"mtimeNs"|"mode">;
/** 🗒️ Remembers which base files already hashed to their sealed digest while their stamp stays the same; a miss means hash again. */
export interface NxSdkStampMemoV1{has(path:string,facts:NxSdkStampFactsV1):boolean;add(path:string,facts:NxSdkStampFactsV1):void;commit():Promise<void>}
/** 🔑️ Spells the stamp of one file the same way the independent oracle does. */
export const nxSdkStampKeyV1=(facts:NxSdkStampFactsV1):string=>[facts.dev,facts.ino,facts.size,facts.mtimeNs,facts.mode].join(":");
/** 📂️ Names the memo file of one base directory and one sealed index digest below the machine cache base. */
export function nxSdkStampMemoFileV1(base:string,directory:string,sourceIndex:string):string{return join(base,policy.directory,createHash("sha256").update(directory+"\0"+sourceIndex).digest("hex").slice(0,32)+".json");}
/** 🧭️ The memo base: the machine cache base or an absolute override from the environment; nothing when the environment says off or when the file `sdk-verified/off` exists below the base, which reaches every process of the machine whatever environment the script process was given. */
export function nxSdkStampBaseV1(environment:Readonly<Record<string,string|undefined>>=process.env):string|undefined{
 const value=environment[policy.environment],base=value===undefined||value===""?nxAuthorizerMemoBaseV1(environment):value!==policy.disabled&&isAbsolute(value)?value:undefined;
 return base===undefined||existsSync(join(base,policy.directory,policy.disabledMarker))?undefined:base;
}
type State={rows:Map<string,string>;verifiedAt:number;dirty:boolean};
const states=new Map<string,State>();
/** 🚫️ The memo that remembers nothing: verification hashes every file. */
export const disabledNxSdkStampMemoV1:NxSdkStampMemoV1=Object.freeze({has:()=>false,add:()=>undefined,commit:async()=>undefined});
/** 🧹️ Forgets the in-process copies so a test can observe a cold process. */
export function resetNxSdkStampMemosV1():void{states.clear();}
const fresh=(now:number):State=>({rows:new Map(),verifiedAt:now,dirty:true});
async function load(file:string,sourceIndex:string,now:number):Promise<State>{
 try{
  const text=await readFile(file,"utf8");
  if(text.length>policy.maximumBytes)return fresh(now);
  const value=JSON.parse(text);
  if(validateJsonSchemaSubset(schema.definitions.Memo,value,schema).length||value.sourceIndex!==sourceIndex||value.verifiedAt>now||now-value.verifiedAt>policy.maximumAgeMilliseconds||Object.keys(value.stamps).length>policy.maximumRows)return fresh(now);
  return{rows:new Map(Object.entries(value.stamps as Record<string,string>)),verifiedAt:value.verifiedAt,dirty:false};
 }catch{return fresh(now);}
}
async function store(file:string,sourceIndex:string,state:State):Promise<void>{
 const temporary=file+"."+process.pid+"."+randomBytes(6).toString("hex")+".tmp";
 try{
  await mkdir(dirname(file),{recursive:true});
  await writeFile(temporary,JSON.stringify({version:1,sourceIndex,verifiedAt:state.verifiedAt,stamps:Object.fromEntries([...state.rows].sort(([left],[right])=>left<right?-1:left>right?1:0))})+"\n",{flag:"wx",mode:0o600});
  await rename(temporary,file);
 }catch(error){await rm(temporary,{force:true}).catch(()=>undefined);throw error;}
}
/** 📖️ Opens the memo of one sealed base: an unreadable, foreign, malformed, oversized or older than a day memo is a cold memo, so the caller hashes everything again and the next commit replaces it. */
export async function openNxSdkStampMemoV1(directory:string,sourceIndex:string,options:Readonly<{environment?:Readonly<Record<string,string|undefined>>;now?:()=>number}>={}):Promise<NxSdkStampMemoV1>{
 const base=nxSdkStampBaseV1(options.environment);
 if(base===undefined)return disabledNxSdkStampMemoV1;
 const file=nxSdkStampMemoFileV1(base,directory,sourceIndex),now=(options.now??Date.now)();
 let state=states.get(file);
 if(state===undefined||now-state.verifiedAt>policy.maximumAgeMilliseconds){state=await load(file,sourceIndex,now);states.set(file,state);}
 const current=state;
 return Object.freeze({
  has:(path:string,facts:NxSdkStampFactsV1)=>current.rows.get(path)===nxSdkStampKeyV1(facts),
  add(path:string,facts:NxSdkStampFactsV1){const key=nxSdkStampKeyV1(facts);if(current.rows.get(path)!==key){current.rows.set(path,key);current.dirty=true;}},
  async commit(){
   if(!current.dirty||current.rows.size>policy.maximumRows)return;
   current.dirty=false;
   try{await store(file,sourceIndex,current);}catch{current.dirty=true;}
  },
 });
}
