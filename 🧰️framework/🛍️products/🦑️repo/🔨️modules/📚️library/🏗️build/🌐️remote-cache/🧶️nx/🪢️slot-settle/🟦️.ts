import {readdirSync,readFileSync} from "node:fs";
import {isAbsolute,join,resolve} from "node:path";
import {readNxTaskRowsV1,supersedeNxCacheV1,type NxSupersessionV1,type NxTaskReaderV1} from "../../🪢️nx-slots/🟦️.ts";
import type {NxFlightV1} from "../🚦️single-flight/🟦️.ts";

/** 📋️ What the end of a run did for the slots: tasks it claimed as current at the fleet cache and the local supersession. */
export type NxSlotSettlementV1=Readonly<{registered:Readonly<{claimed:number;missing:number}>|undefined;local:NxSupersessionV1|undefined}>;

/** 🗄️ The Nx cache directory of a workspace: the `cacheDirectory` of its nx.json, relative to the root, `.nx/cache` by default. */
export function nxCacheDirectoryV1(root:string):string{
 let configured:unknown;try{configured=JSON.parse(readFileSync(join(root,"nx.json"),"utf8")).cacheDirectory;}catch{}
 return typeof configured==="string"&&configured!==""?(isAbsolute(configured)?configured:resolve(root,configured)):join(root,".nx","cache");
}

/** 🪢️ Settles the slots after an Nx run: every task the run touched (stored or hit since `sinceMs`) is claimed as current at the fleet cache through the front (the server cannot see a local hit or a header-less upload), and the local Nx cache drops the entries a newer entry of the same task slot superseded beyond the grace, forgetting their rows in the Nx database so Nx never believes in a removed entry. Fails soft: nothing here can fail a build. */
export async function settleNxSlotsV1(options:Readonly<{root:string;environment:Readonly<Record<string,string|undefined>>;flight?:NxFlightV1|undefined;sinceMs:number;graceMilliseconds?:number;reader?:NxTaskReaderV1;platform?:string;nowMs?:number}>):Promise<NxSlotSettlementV1>{
 try{
  const directory=options.environment.NX_WORKSPACE_DATA_DIRECTORY;if(directory===undefined||directory==="")return {registered:undefined,local:undefined};
  const database=readdirSync(directory).filter(name=>name.endsWith(".db")).map(name=>join(directory,name))[0];if(database===undefined)return {registered:undefined,local:undefined};
  const rows=await(options.reader??readNxTaskRowsV1)(database),touched=rows.filter(row=>Math.max(row.createdAt??0,row.accessedAt??0)>=options.sinceMs-2000);
  const registered=options.flight&&touched.length?await options.flight.registerSlots(touched):undefined;
  const local=await supersedeNxCacheV1({cacheDirectory:nxCacheDirectoryV1(options.root),rows,...(options.graceMilliseconds===undefined?{}:{graceMilliseconds:options.graceMilliseconds}),...(options.platform===undefined?{}:{platform:options.platform}),...(options.nowMs===undefined?{}:{nowMs:options.nowMs}),forget:async hashes=>{
   const {Database}=await import("bun:sqlite"),writable=new Database(database,{readwrite:true});
   try{const mark=hashes.map(()=>"?").join(",");writable.query(`DELETE FROM cache_outputs WHERE hash IN (${mark})`).run(...hashes);}finally{writable.close();}
  }});
  return {registered,local};
 }catch{return {registered:undefined,local:undefined};}
}
