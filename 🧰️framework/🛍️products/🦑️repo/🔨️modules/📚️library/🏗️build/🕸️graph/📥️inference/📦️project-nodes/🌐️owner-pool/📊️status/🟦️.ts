import{mkdirSync,readFileSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{hostname}from"node:os";
import{join}from"node:path";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{OWNER_POOL_POLICY_V1 as P}from"../🟦️.ts";
import{ORIGIN_FILES_POLICY_V1 as R,retireOriginFilesV1}from"../🧹️origin-files/🟦️.ts";

/** 📊️ What a machine says about its part in the pool (see the schema definition `Status`). */
export type OwnerPoolStatusV1={version:1;role:"puller"|"publisher";origin:string;host:string;pid:number;state:"idle"|"pulling"|"declined"|"too-small"|"off";wave:string|null;units:number;claimed:number;published:number;stored:number;held:number;failed:number;running:number;slots:number;lastError:string|null;failures?:Readonly<Record<string,number>>;kept?:number;resent?:number;diverged?:number;shed?:number;rssMegabytes:number;totalMegabytes?:number;grantMegabytes?:number;siblings:number;children?:number;target?:number;claimAttempts?:number;recycled?:number;ownersPerMinute?:number;childMegabytes?:number;waves?:number;exits?:Readonly<Record<string,number>>;lastExits?:readonly Readonly<{atMs:number;pid:number;reason:string;code:number|null;ageSeconds:number;owners:number;text:{en:string;de:string}}>[];reason:string|null;reasonText:{en:string;de:string}|null;startedMs:number;updatedMs:number;consumedByHost?:Record<string,number>;waveSeconds?:number};
export const OWNER_POOL_STATUS_SCHEMA_V1=schema.definitions.Status;

/** 📊️ The status file of one machine, `<store>/v1/origins/<origin>/pool.json`, replaced atomically at most every few seconds (and at once when the state changes); never throws, a store that cannot be written costs nothing. `store` is `SEMIO_FLEET_STORE_ROOT`. */
export function createOwnerPoolStatusV1(input:Readonly<{store:string|undefined;origin:string;role:"puller"|"publisher";now?:()=>number}>):Readonly<{update(patch:Partial<OwnerPoolStatusV1>):void;flush():void;current():OwnerPoolStatusV1}>{
 const now=input.now??Date.now,startedMs=now();
 let doc:OwnerPoolStatusV1={version:1,role:input.role,origin:input.origin,host:hostname(),pid:process.pid,state:"idle",wave:null,units:0,claimed:0,published:0,stored:0,held:0,failed:0,running:0,slots:0,lastError:null,rssMegabytes:0,siblings:0,reason:null,reasonText:null,startedMs,updatedMs:startedMs},written=0,swept=-Infinity,timer:ReturnType<typeof setTimeout>|undefined;
 const retire=(directory:string):void=>{const at=now();if(at-swept<R.sweepIntervalMilliseconds)return;swept=at;void retireOriginFilesV1({directory,now:Date.now()}).catch(()=>undefined);};
 const write=()=>{
  timer=undefined;written=now();
  if(input.store===undefined||input.store==="")return;
  let temporary:string|undefined;
  try{const directory=join(input.store,"v1","origins",input.origin),file=join(directory,P.statusName);temporary=file+"."+process.pid+".tmp";mkdirSync(directory,{recursive:true});retire(directory);writeFileSync(temporary,JSON.stringify({...doc,updatedMs:now()}));renameSync(temporary,file);}catch{if(temporary!==undefined)try{rmSync(temporary,{force:true});}catch{}}
 };
 return Object.freeze({
  update(patch){
   const changed=patch.state!==undefined&&patch.state!==doc.state||patch.wave!==undefined&&patch.wave!==doc.wave||patch.reason!==undefined&&patch.reason!==doc.reason;
   doc={...doc,...patch};
   if(changed||now()-written>=P.statusIntervalMilliseconds){if(timer!==undefined)clearTimeout(timer);write();}
   else if(timer===undefined){timer=setTimeout(write,P.statusIntervalMilliseconds);timer.unref?.();}
  },
  flush(){if(timer!==undefined)clearTimeout(timer);write();},
  current:()=>doc,
 });
}

/** 🩺️ What the pool of this machine says about itself, read back from its status file: the state, the children and the running owners, the owners per minute and the reason it runs fewer children than it could in both languages (`text`, `null` at full width). `undefined` where there is no store, no file, a broken file or a status of another version: a read that asks never fails by it.
 * @see ./🧪️tests/🩺️pool-status-read/🟦️.ts */
export function readOwnerPoolStatusV1(input:Readonly<{store:string|undefined;origin:string|undefined}>):Readonly<{state:string;children:number;running:number;ownersPerMinute:number;reason:string|null;text:Readonly<{en:string;de:string}>|null}>|undefined{
 if(input.store===undefined||input.store===""||input.origin===undefined||input.origin==="")return undefined;
 try{
  const doc=JSON.parse(readFileSync(join(input.store,"v1","origins",input.origin,P.statusName),"utf8")) as Record<string,unknown>;
  if(doc?.version!==1||typeof doc.state!=="string")return undefined;
  const text=doc.reasonText as {en?:unknown;de?:unknown}|null|undefined;
  return Object.freeze({state:doc.state,children:Number(doc.children??0),running:Number(doc.running??0),ownersPerMinute:Number(doc.ownersPerMinute??0),reason:typeof doc.reason==="string"?doc.reason:null,text:text!==null&&text!==undefined&&typeof text.en==="string"&&typeof text.de==="string"?Object.freeze({en:text.en,de:text.de}):null});
 }catch{return undefined;}
}
