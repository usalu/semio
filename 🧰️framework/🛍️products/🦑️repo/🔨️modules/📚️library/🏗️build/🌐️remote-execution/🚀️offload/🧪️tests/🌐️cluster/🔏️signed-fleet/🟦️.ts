import {afterAll,beforeAll,expect,test} from "bun:test";
import {chmodSync,copyFileSync,cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {createServer} from "node:net";
import {tmpdir} from "node:os";
import {statfsSync} from "node:fs";
import {CAS_POLICY_V1} from "../../../../../🗄️cas/🧬️schema/🟦️.ts";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {registeredToolExecutableV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {cacheAreaDirectoryV1,cacheLedgerV1} from "../../../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {wrapRustcV1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {drainUploadsV1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts";
import {fleetRemoteEnvironmentV1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🚪️launch/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {publishFleetCacheFactsV1} from "../../../../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {syncFleetIdentityV1} from "../../../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {refreshFleetCacheFactsV1,remoteCacheFetchV1} from "../../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {eventuallyV1,writeHostViewV1} from "../../../../🧪️tests/🧰️cluster/🟦️.ts";
import {createTestCheckoutV1,externHash,testHostProbeV1,testRustEnvironmentV1,type TestCheckoutV1} from "../../🧰️workspace/🟦️.ts";

/** 🔏️ The whole fleet on one machine with real processes, scratch ports and an explicit scratch store: the signed cache server is the real server module on a real socket (in this process: the sidecar entry builds a CAS host store whose disk guard refuses writes on a nearly full disk), the scheduler and two agents are the replica daemon's sidecar entries as real processes, the requester and a third machine are unit-cache wrapper calls. A unit is offloaded to an agent (signed gRPC and worker API), uploaded under the requester's key with its slot through a claim, and taken by the third machine without any compiler; no secret exists anywhere in the scratch store. */
const bun=registeredToolExecutableV1("bun"),sidecar=fileURLToPath(new URL("../../../../🛰️sidecar/🟦️.ts",import.meta.url)),scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"signed-fleet-")));
const store=join(scratch,"store"),children:ReturnType<typeof Bun.spawn>[]=[],roots=new Map<string,string>();
const freePort=async():Promise<number>=>{for(;;){const port=21000+Math.floor(Math.random()*20000);if(await new Promise<boolean>(done=>{const server=createServer();server.once("error",()=>{done(false);});server.listen(port,"127.0.0.1",()=>{server.close(()=>{done(true);});});}))return port;}};
const NETWORK="127.0.0.0/8";

async function join_(root:string,origin:string):Promise<string>{
 mkdirSync(join(root,".cache","f"),{recursive:true});mkdirSync(join(store,"v1","origins",origin),{recursive:true});
 writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"0".repeat(32)}));
 const synced=await syncFleetIdentityV1({root});if(synced.error!==undefined)throw Error(synced.error);
 roots.set(origin,root);return root;
}
const environmentOf=(root:string,origin:string,extra:Record<string,string>={}):Record<string,string>=>({...(process.env as Record<string,string>),...testRustEnvironmentV1(),SEMIO_FLEET_ROOT:root,SEMIO_WORKSPACE_ROOT:root,SEMIO_FLEET_ORIGIN:origin,SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_NETWORK:NETWORK,SEMIO_CAS_ROOT:join(root,"cas"),...extra});
const start=(role:"scheduler"|"agent",environment:Record<string,string>)=>{const log=join(scratch,"logs");mkdirSync(log,{recursive:true});const child=Bun.spawn([bun,sidecar,role],{stdout:"ignore",stderr:Bun.file(join(log,`${role}-${children.length}.log`)),cwd:scratch,env:{...environment}});children.push(child);return child;};
const factsPath=join(store,"v1","fleet","cache.json"),facts=():{execution?:{port:number}}|undefined=>{try{return JSON.parse(readFileSync(factsPath,"utf8"));}catch{return undefined;}};
const remember=(root:string,service:"cache"|"execution",endpoint:string):void=>{const path=join(root,".cache","f","last-good.json");let known:Record<string,string>={};try{known=JSON.parse(readFileSync(path,"utf8"));}catch{}mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(path,JSON.stringify({...known,[`g1-server/${service}`]:endpoint}));};
const detachers:(()=>void)[]=[];
const attachPartition=(worktree:string,id:string):void=>{
 if(process.platform!=="darwin")return;
 const ledger=cacheLedgerV1(),area=ledger.areas.find(row=>row.id===id)!,found=cacheAreaDirectoryV1(worktree,ledger,area)!;
 let mount=found;for(let step=0;step<(area.directory===undefined?0:area.directory.split("/").filter(Boolean).length);step++)mount=dirname(mount);
 mkdirSync(mount,{recursive:true});
 const image=`${mount}.test-partition.sparseimage`;
 if(spawnSync("hdiutil",["create","-type","SPARSE","-size","8g","-fs","APFS","-volname","semio-test","-ov",image]).status!==0||spawnSync("hdiutil",["attach",image,"-mountpoint",mount,"-nobrowse","-noverify"]).status!==0)throw Error("the test partition could not be attached");
 detachers.push(()=>{spawnSync("hdiutil",["detach",mount,"-force"]);});
};
const idle=join(scratch,"idle-host.json");
let ports:number[]=[],cache:RemoteCacheServerV1|undefined,cacheUrl="";

beforeAll(async()=>{
 const server=await join_(join(scratch,"server"),"g1-server");
 ports=[await freePort(),await freePort(),await freePort()];
 writeHostViewV1(idle,{atMs:Date.now()+86400000});
 const base=(root:string,origin:string)=>environmentOf(root,origin,{SEMIO_GRAPH_SERVER:"off",SEMIO_REMOTE_EXECUTION_PORTS:`${ports[0]},${ports[1]}`,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idle,SEMIO_REMOTE_CACHE_FLEET_PORT:String(ports[2])});
 cache=await startRemoteCacheServerV1({directory:join(scratch,"cache"),host:"127.0.0.1",port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:30,defaultInstance:"main",tokens:{},fleet:{storeRoot:store,group:"g1",networks:[NETWORK],registryTtlMilliseconds:200}});
 await publishFleetCacheFactsV1({admin:{role:"admin",originId:"g1-server"},storeRoot:store,directory:join(scratch,"cache-data"),group:"g1",instance:"main",port:Number(new URL(cache.url).port)});
 cacheUrl=cache.url;remember(server,"cache",cache.url);
 await refreshFleetCacheFactsV1({root:server,force:true});
 start("scheduler",base(server,"g1-server"));
 await eventuallyV1(()=>facts()?.execution!==undefined,60000,"the scheduler publishes its endpoint\n"+(()=>{try{return readdirSync(join(scratch,"logs")).map(name=>name+": "+readFileSync(join(scratch,"logs",name),"utf8").slice(-600)).join("\n");}catch{return"";}})());
 for(const origin of ["g1-m09","g1-m10"]){const root=await join_(join(scratch,origin),origin);attachPartition(root,"remote-execution-worker");remember(root,"cache",cache.url);remember(root,"execution",`http://127.0.0.1:${ports[0]}`);await refreshFleetCacheFactsV1({root,force:true});start("agent",base(root,origin));}
},180000);
afterAll(async()=>{await cache?.stop().catch(()=>{});for(const child of children)child.kill("SIGKILL");await Bun.sleep(300);for(const detach of detachers.splice(0))detach();if(process.env.SEMIO_TEST_KEEP_LOGS)try{cpSync(join(scratch,"logs"),process.env.SEMIO_TEST_KEEP_LOGS,{recursive:true});}catch{}rmSync(scratch,{recursive:true,force:true});});

const countingRustc=(directory:string):{path:string;log:string}=>{
 mkdirSync(directory,{recursive:true});
 const path=join(directory,"counting-rustc.sh"),log=join(directory,"rustc-calls.log");
 writeFileSync(path,`#!/bin/sh\ncase " $* " in *" -vV "*|*" -Z ls="*|*" --print "*) ;; *) echo call >> '${log}';; esac\nexec rustc "$@"\n`);chmodSync(path,0o755);
 return{path,log};
};
const callsOf=(log:string):number=>existsSync(log)?readFileSync(log,"utf8").split("\n").filter(Boolean).length:0;
async function machine(label:string,origin:string):Promise<{checkout:TestCheckoutV1;cache:string;environment:Record<string,string>}>{
 const checkout=createTestCheckoutV1(label,scratch);
 checkout.compile("a","pub fn a()->u32{1}\n");checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);checkout.crate("c","pub fn c()->u32{b::b()+13}\n");
 await join_(checkout.root,origin);remember(checkout.root,"cache",cacheUrl);remember(checkout.root,"execution",`http://127.0.0.1:${ports[0]}`);await refreshFleetCacheFactsV1({root:checkout.root,force:true});
 const cache=join(scratch,"unit-"+label),counter=countingRustc(cache);
 const environment={...testRustEnvironmentV1(checkout.environment("c")),...fleetRemoteEnvironmentV1({},checkout.root),SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:"t",SEMIO_UNIT_CACHE_INNER:JSON.stringify([counter.path]),SEMIO_UNIT_CACHE_SLOTS:"0",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:checkout.root,SEMIO_WORKSPACE_ROOT:checkout.root,SEMIO_CAS_ROOT:join(scratch,"cas-"+label)};
 return{checkout,cache,environment};
}
const run=async(m:{checkout:TestCheckoutV1},environment:Record<string,string>,admission?:Parameters<typeof wrapRustcV1>[3]):Promise<number>=>{
 const previous=process.cwd();process.chdir(m.checkout.root);
 try{return await wrapRustcV1(["rustc",...m.checkout.arguments("c",["b"])],environment,m.checkout.root,admission);}finally{process.chdir(previous);}
};
const events=(root:string):{kind:string;reason?:string}[]=>existsSync(join(root,"x"))?readdirSync(join(root,"x")).filter(name=>name.endsWith(".jsonl")).flatMap(name=>readFileSync(join(root,"x",name),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line))):[];
const status=async(root:string):Promise<any>=>{const signed=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:root},{root,audience:"cache",roleOf:()=>"cache-read"});return (await signed(cache!.url+"/status")).json();};

/** 💽️ The local store refuses new objects once the disk passes the CAS critical fraction (a unit would then not even be published); on such a disk this proof cannot run, and it says so instead of failing. */
const diskFull=(():boolean=>{try{const row=statfsSync(tmpdir());return 1-Number(row.bavail)/Number(row.blocks)>=CAS_POLICY_V1.partition.criticalUsedFraction-0.005;}catch{return false;}})();
test.skipIf(diskFull)("signed end to end: an unsigned caller is refused, a unit is offloaded to an agent, uploaded once under the requester's key with its slot through a claim, and a third machine takes it with no compiler",async()=>{
 const endpoint=cache!.url;
 expect((await fetch(endpoint+"/status")).status,"an unsigned caller is refused").toBeGreaterThanOrEqual(401);
 const requester=await machine("requester","g1-m11"),third=await machine("third","g1-m12");
 let lastTable="";
 try{
 await eventuallyV1(async()=>{const signed=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:requester.checkout.root},{root:requester.checkout.root,audience:"execution",roleOf:()=>"submit"});const api=`http://127.0.0.1:${ports[1]}/v1/fleet/load`;const table=await (await signed(api)).json() as {machines?:{origin:string;headroom?:boolean}[]};lastTable=JSON.stringify(table);return["g1-m09","g1-m10"].every(origin=>table.machines?.some(row=>row.origin===origin&&row.headroom===true));},90000,"both agents report headroom");
 }catch(error){throw Error(String((error as Error).message)+" "+lastTable.slice(0,4000));}
 expect(requester.environment.SEMIO_REMOTE_CACHE_ENDPOINT,"the fleet environment names the signed cache").toBe(endpoint);
 const code=await run(requester,{...requester.environment,SEMIO_OFFLOAD:"distribute"},{admission:{probe:testHostProbeV1("critical-pressure-gigabyte-swap")}});
 const trace=JSON.stringify(events(requester.cache))+(existsSync(join(requester.cache,"x","offload.progress"))?readFileSync(join(requester.cache,"x","offload.progress"),"utf8"):"");
 expect(code,trace).toBe(0);
 expect(callsOf(join(requester.cache,"rustc-calls.log")),"the requester compiled nothing itself: "+trace).toBe(0);
 expect(trace,"the unit went to an agent").toMatch(/offloaded:\S+/u);
 const kinds=events(requester.cache).map(row=>row.kind);
 expect(kinds).toContain("remote-claim");
 const drained=await drainUploadsV1(requester.cache,requester.environment,{idleMilliseconds:200});
 await eventuallyV1(async()=>((await status(requester.checkout.root)).claims?.active??-1)===0,30000,"the claim is released");
 const after=await status(requester.checkout.root);
 expect(after.counters.ac.puts,"two action results (the scheduler's own for the action and the one unit result) and the input manifest of the slot").toBe(2+events(requester.cache).filter(row=>row.kind==="manifest-publish"&&(row.reason??"").startsWith("stored")).length);expect(after.counters.cas.puts).toBeGreaterThan(0);expect(after.divergences.count).toBe(0);expect(after.fleet.denied["no-signature"],"the unsigned probe was counted and refused").toBeGreaterThanOrEqual(1);await eventuallyV1(()=>cache!.store.usage().slotted>=1,10000,"stored with its slot: "+JSON.stringify(cache!.store.usage())+" entriesDir="+JSON.stringify(existsSync(join(requester.cache,"e"))?readdirSync(join(requester.cache,"e")):null)+" outputs="+JSON.stringify(readdirSync(requester.checkout.deps).filter(n=>n.includes(externHash("c"))))+" drain="+JSON.stringify(drained)+" events="+JSON.stringify(events(requester.cache).slice(-6)));expect(cache!.store.usage().slotted,"the unit and the input manifest of its slot carry the slot").toBe(1+events(requester.cache).filter(row=>row.kind==="manifest-publish"&&(row.reason??"").startsWith("stored")).length);
 for(const name of readdirSync(requester.checkout.deps).filter(name=>!name.includes("-"+externHash("c"))))copyFileSync(join(requester.checkout.deps,name),join(third.checkout.deps,name));
 const codeThird=await run(third,{...third.environment,SEMIO_OFFLOAD:"never"});
 expect(codeThird).toBe(0);
 expect(callsOf(join(third.cache,"rustc-calls.log")),"the third machine compiled nothing: "+JSON.stringify(events(third.cache))).toBe(0);
 expect(events(third.cache).map(row=>row.kind)).toContain("remote-hit");
 expect(readdirSync(third.checkout.deps).some(name=>name.includes(`-${externHash("c")}`)&&name.endsWith(".rlib"))).toBe(true);
 const secrets=readdirSync(store,{recursive:true,withFileTypes:true}).filter(entry=>entry.isFile()&&/fleet-auth|cache\.json/u.test(entry.name)).map(entry=>readFileSync(join(entry.parentPath,entry.name),"utf8")).join("\n");
 expect(secrets,"the store holds public keys only").not.toMatch(/"(private|secret|token)"/iu);
},300000);
