import {createHash} from "node:crypto";
import {mkdirSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {fetchGraphEntryV1,graphFleetClientV1,publishGraphEntryOutcomeV1,tryGraphEntryV1} from "../../../🏗️build/🪪️graph-state/🛰️exchange/🟦️.ts";
import {rehearsalHostV1,rehearsalOwnerStateV1} from "../🟦️.ts";
import type {GraphStateV1} from "../../../🏗️build/🪪️graph-state/🟦️.ts";
import {OWNER_POOL_POLICY_V1 as P,ownerPoolAcceptsRevisionV1,ownerPoolIdV1,ownerPoolTargetV1,publishOwnerPoolV1,pullOwnerPoolV1,readOwnerPoolPointerV1,readOwnerPoolV1,type OwnerPoolUnitV1} from "../../../🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import type {ProjectClaimV1} from "../../../🏗️build/🪪️graph-state/📦️project-records/🟦️.ts";

/** 👷️ One simulated machine of the rehearsal: a reader that looks at every owner in the fleet cache and publishes the wave of the missing ones (the real publisher of the owner pool), and a pool that pulls the open waves with the real puller: claims, heartbeats and results go through the real client and the real claim table of the cache server; an owner runs as a process of its own. It writes the ledger and the pool state in the shape the sampler reads, and `ready.json` when every owner is known. */
type Config={origin:string;proxy:string;instance:string;state:string;memoryMegabytes:number;cores:number;owners:readonly string[];ownerMilliseconds:number;ownerMegabytes:number;revision:string;work:string;sampleMilliseconds:number;id:string;slot:string};
const config=JSON.parse(readFileSync(process.argv.find(value=>value.startsWith("--config="))!.slice(9),"utf8")) as Config;
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const client=graphFleetClientV1({endpoint:config.proxy,instance:config.instance,token:undefined,upload:true,stateDirectory:join(config.state,"graph-state")});
const owner=`${config.origin}:${process.pid}`,child=join(dirname(fileURLToPath(import.meta.url)),"..","🐜️owner","🟦️.ts");
let revision=config.revision,generation=0,here=new Set<string>(),fetched=new Set<string>(),running=0,wave:string|null=null,reason:string|null="no-wave",claimed=0,held=0,stopped=false;
const stateOf=(name:string,at:string=revision):GraphStateV1=>rehearsalOwnerStateV1(name,at),total=config.owners.length;
const write=(name:string,value:unknown)=>{const file=join(config.state,name),temporary=`${file}.${process.pid}.tmp`;writeFileSync(temporary,JSON.stringify(value));renameSync(temporary,file);};
const done=()=>here.size+fetched.size;
const publish=()=>{
 const done_=done(),open=total-done_;
 write("slice-memory.json",{version:1,atMs:Date.now(),pid:process.pid,sources:{owners:{ledger:{total,present:0,fetched:fetched.size,here:here.size,wanted:open,done:done_}}}});
 write("pool.json",{state:running>0?"pulling":"idle",reason,wave,running,open,claimed,held,updatedMs:Date.now()});
 if(done_===total)write("ready.json",{ready:true,origin:config.origin,atMs:Date.now()});
};
const look=async()=>{for(const name of config.owners){if(here.has(name)||fetched.has(name))continue;const answer=await fetchGraphEntryV1(client,stateOf(name));if(answer!==undefined&&answer!=="fault")fetched.add(name);}};
const head=(units:readonly OwnerPoolUnitV1[])=>{const base={version:1 as const,revision,identity:"rehearsal:"+revision,globals:{projectAuthority:"rehearsal",registryDigest:"rehearsal",commandRevision:"rehearsal",late:false}};return{...base,id:ownerPoolIdV1(base,units),createdMs:Date.now(),publisher:config.origin};};
const announce=async()=>{
 const missing=config.owners.filter(name=>!here.has(name)&&!fetched.has(name));if(missing.length===0)return;
 const units:OwnerPoolUnitV1[]=missing.map(name=>({name,kind:"project",expectedMilliseconds:config.ownerMilliseconds,megabytes:config.ownerMegabytes,json:{name}}));
 await publishOwnerPoolV1({client,owner,manifest:{head:head(units),session:new Uint8Array(0),units}});
};
const claimOf=async(unit:OwnerPoolUnitV1):Promise<ProjectClaimV1>=>{
 const born=revision,state=stateOf(unit.name,born),tried=await tryGraphEntryV1({client,state,owner,expectedMilliseconds:unit.expectedMilliseconds,ttlMilliseconds:P.claimTtlMilliseconds});
 if(tried.outcome==="stored")return{state:"stored"};
 if(tried.outcome==="held")return{state:"held",by:tried.by};
 if(tried.outcome!=="granted")return{state:"fault"};
 claimed++;
 return{state:"granted",digest:state.digest,why:()=>"",code:()=>"",deterministic:()=>false,publish:async()=>{const put=await publishGraphEntryOutcomeV1({client,state,owner,hold:tried.hold,documents:[{name:"record",bytes:new TextEncoder().encode(JSON.stringify({name:unit.name,origin:config.origin}))}]});if(put.stored&&born===revision)here.add(unit.name);return put.stored;},abandon:async()=>{await tried.hold.release();}};
};
const run=async(unit:OwnerPoolUnitV1):Promise<boolean>=>{
 running++;publish();
 try{const process_=Bun.spawn([process.execPath,child,`--rehearsal=${config.id}`,`--machine=${config.origin}`,`--duration=${config.ownerMilliseconds}`,`--mb=${unit.megabytes??config.ownerMegabytes}`],{windowsHide:true,stdin:"ignore",stdout:"ignore",stderr:"ignore"});return await process_.exited===0;}
 finally{running--;}
};
const host=rehearsalHostV1(process.env);
const slots=()=>{const target=ownerPoolTargetV1({host:{cores:host.cores,totalMegabytes:host.totalMegabytes,availableMegabytes:Math.max(0,host.totalMegabytes-running*config.ownerMegabytes-2048),pressure:"normal",load:null,swapOutMegabytesPerSecond:null,foreground:{cpu:0,unrealizedMegabytes:0,waiters:0}},children:running,childMegabytes:config.ownerMegabytes});return Math.max(1,Math.min(host.cores,target.children));};
const landings=()=>{try{const row=JSON.parse(readFileSync(join(config.work,"landing.json"),"utf8")) as {revision:string};if(row.revision!==revision){revision=row.revision;generation++;here=new Set();fetched=new Set();wave=null;reason="no-current-wave";rmSync(join(config.state,"ready.json"),{force:true});}}catch{}};
mkdirSync(config.state,{recursive:true});
process.on("SIGTERM",()=>{stopped=true;process.exit(0);});
const ticker=setInterval(()=>{landings();publish();},config.sampleMilliseconds);
await pause(Math.floor(Math.random()*500));
let announced=-1;
while(!stopped){
 landings();
 await look();
 if(done()===total){publish();await pause(config.sampleMilliseconds);continue;}
 if(announced!==generation){await announce();announced=generation;}
 const pointer=await readOwnerPoolPointerV1(client);
 const open=pointer===undefined||pointer==="fault"?[]:pointer.waves;
 let pulled=false;
 for(const entry of open){
  const manifest=await readOwnerPoolV1(client,entry.id);
  if(manifest===undefined)continue;
  if(!ownerPoolAcceptsRevisionV1(manifest.head.revision,revision)){reason="no-current-wave";continue;}
  wave=entry.id;reason=null;pulled=true;const generationAtStart=generation;
  await pullOwnerPoolV1<OwnerPoolUnitV1>({units:()=>manifest.units.filter(unit=>!here.has(unit.name)&&!fetched.has(unit.name)),claim:claimOf,run,slots,puller:config.origin,stop:()=>stopped||generation!==generationAtStart||done()===total,progress:(stats)=>{held=stats.held;publish();},known:new Set<string>()});
  await look();
 }
 if(!pulled){running=0;publish();}
 await pause(1000);
}
clearInterval(ticker);
