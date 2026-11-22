import {afterAll,beforeAll,expect,test} from "bun:test";
import {spawn,type ChildProcess} from "node:child_process";
import {chmodSync,copyFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {statfsSync} from "node:fs";
import {CAS_POLICY_V1} from "../../../../../🗄️cas/🧬️schema/🟦️.ts";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {wrapRustcV1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/📜️script.ts";
import {drainUploadsV1} from "../../../../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts";
import {startClusterV1,eventuallyV1,writeHostViewV1,type ClusterV1} from "../../../../🧪️tests/🧰️cluster/🟦️.ts";
import {createTestCheckoutV1,externHash,testHostProbeV1,testRustEnvironmentV1,type TestCheckoutV1} from "../../🧰️workspace/🟦️.ts";

/** ☁️ A unit compiled by a worker reaches the fleet cache exactly like one compiled locally: stored once under the requester's key with its slot, served to a third checkout without any compiler, and its claim is released. Scheduler, cache server (an explicit scratch store, never the real one) and two worker processes run here. */
const entry=fileURLToPath(new URL("../../../../🏃️entry/🟦️.ts",import.meta.url)),scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"offload-store-")));
const TOKEN="scratch-fleet-token",AUTH={authorization:"Bearer scratch-fleet-token"},children=new Set<ChildProcess>();
let cluster:ClusterV1;
const idleHost=join(scratch,"idle-host.json");
writeHostViewV1(idleHost,{atMs:Date.now()+86400000});

beforeAll(async()=>{
 process.env.SEMIO_CAS_ROOT=join(scratch,"cas");
 cluster=await startClusterV1({workers:0,root:scratch,tokens:{write:TOKEN,worker:TOKEN},scheduler:{leaseSeconds:5,lingerSeconds:1,unmatchedSeconds:30}});
 for(const name of ["wA","wB"]){
  const directory=join(scratch,"worker-"+name);mkdirSync(directory,{recursive:true});
  const child=spawn(process.execPath,[entry,"--join",cluster.server.workerUrl,"--name",name,"--origin",name,"--work-directory",join(directory,"work"),"--concurrency","2"],{cwd:directory,env:testRustEnvironmentV1({SEMIO_CAS_ROOT:undefined,SEMIO_REMOTE_EXECUTION_HOST_VIEW:idleHost,SEMIO_REMOTE_CACHE_TOKEN:TOKEN,SEMIO_REMOTE_EXECUTION_WORKER_TOKEN:TOKEN}),stdio:["ignore","ignore","ignore"]});
  children.add(child);child.on("close",()=>children.delete(child));
 }
 await eventuallyV1(async()=>{const status=await (await fetch(cluster.server.workerUrl+"/status",{headers:AUTH})).json() as {workers?:unknown[]};return(status.workers?.length??0)>=2;},60000,"both worker processes join");
 await eventuallyV1(async()=>{const response=await fetch(cluster.server.workerUrl+"/v1/fleet/load",{headers:AUTH});if(!response.ok)return false;const table=await response.json() as {machines?:{origin:string;headroom?:boolean}[]};return["wA","wB"].every(name=>table.machines?.find(row=>row.origin===name)?.headroom===true);},60000,"both workers report headroom");
},120000);
afterAll(async()=>{
 for(const child of children)child.kill("SIGKILL");
 await cluster?.stop().catch(()=>{});
 rmSync(scratch,{recursive:true,force:true});
});

const countingRustc=(directory:string):{path:string;log:string}=>{
 const path=join(directory,"counting-rustc.sh"),log=join(directory,"rustc-calls.log");
 writeFileSync(path,`#!/bin/sh\ncase " $* " in *" -vV "*|*" -Z ls="*) ;; *) echo call >> '${log}';; esac\nexec rustc "$@"\n`);chmodSync(path,0o755);
 return{path,log};
};
const callsOf=(log:string):number=>existsSync(log)?readFileSync(log,"utf8").split("\n").filter(Boolean).length:0;
const environmentOf=(checkout:TestCheckoutV1,cacheRoot:string,extra:Record<string,string>):Record<string,string>=>{
 mkdirSync(cacheRoot,{recursive:true});
 return{...testRustEnvironmentV1(checkout.environment("c")),SEMIO_UNIT_CACHE_ROOT:cacheRoot,SEMIO_UNIT_CACHE_LANE:"t",SEMIO_UNIT_CACHE_INNER:JSON.stringify([countingRustc(cacheRoot).path]),SEMIO_UNIT_CACHE_SLOTS:"0",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:checkout.root,
  SEMIO_REMOTE_EXECUTION_ENDPOINT:cluster.server.grpcUrl,SEMIO_REMOTE_CACHE_ENDPOINT:cluster.cache.url.replace("//0.0.0.0","//127.0.0.1"),SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:TOKEN,SEMIO_CAS_ROOT:join(scratch,"cas-"+cacheRoot.split("/").at(-1)),...extra};
};
const checkoutOf=(label:string):TestCheckoutV1=>{
 const checkout=createTestCheckoutV1(label,scratch);
 checkout.compile("a","pub fn a()->u32{1}\n");checkout.compile("b","pub fn b()->u32{a::a()+1}\n",["a"]);checkout.crate("c","pub fn c()->u32{b::b()+11}\n");
 return checkout;
};
const run=async(checkout:TestCheckoutV1,environment:Record<string,string>,admission?:Parameters<typeof wrapRustcV1>[3]):Promise<number>=>{
 const previous=process.cwd();process.chdir(checkout.root);
 try{return await wrapRustcV1(["rustc",...checkout.arguments("c",["b"])],environment,checkout.root,admission);}finally{process.chdir(previous);}
};
const unitEntries=(root:string):{slot?:string;key:string}[]=>existsSync(join(root,"e"))?readdirSync(join(root,"e")).flatMap(shard=>readdirSync(join(root,"e",shard)).flatMap(key=>readdirSync(join(root,"e",shard,key)).filter(name=>name.endsWith(".json")).sort().slice(-1).map(name=>JSON.parse(readFileSync(join(root,"e",shard,key,name),"utf8")) as {slot?:string;key:string}))):[];
const claimsActive=async():Promise<number>=>((await (await fetch(cluster.cache.url.replace("//0.0.0.0","//127.0.0.1")+"/status",{headers:{authorization:"Bearer "+TOKEN}})).json()) as {claims?:{active:number}}).claims?.active??-1;

/** 💽️ The local store refuses new objects once the disk passes the CAS critical fraction (a unit would then not even be published); on such a disk this proof cannot run, and it says so instead of failing. */
const diskFull=(():boolean=>{try{const row=statfsSync(tmpdir());return 1-Number(row.bavail)/Number(row.blocks)>=CAS_POLICY_V1.partition.criticalUsedFraction-0.005;}catch{return false;}})();
test.skipIf(diskFull)("a unit a worker compiled is stored once in the fleet cache under the requester's key with its slot, a third checkout takes it without any compiler, and no claim stays held",async()=>{
 const requester=checkoutOf("req"),rootA=join(scratch,"unit-req"),slottedBefore=cluster.cache.store.usage().slotted;
 const environmentA=environmentOf(requester,rootA,{SEMIO_OFFLOAD:"distribute"});
 const status=await run(requester,environmentA,{admission:{probe:testHostProbeV1("critical-pressure-gigabyte-swap")}});
 expect(status).toBe(0);
 const trace=(root:string):string=>existsSync(join(root,"x"))?readdirSync(join(root,"x")).map(name=>name+": "+readFileSync(join(root,"x",name),"utf8")).join("\n"):"";
 expect(callsOf(join(rootA,"rustc-calls.log")),"the requester compiled nothing itself\n"+trace(rootA)).toBe(0);
 const local=unitEntries(rootA);
 expect(local.length,"one local entry").toBe(1);
 expect(local[0]!.slot,"the local entry carries its slot").toMatch(/^[a-f0-9]{64}$/u);
 const drained=await drainUploadsV1(rootA,environmentA,{idleMilliseconds:200});
 expect(drained).not.toBe("busy");
 await eventuallyV1(()=>cluster.cache.store.usage().slotted===slottedBefore+1,20000,"the unit entry is stored with a slot");
 expect(cluster.cache.store.usage().slotted,"stored exactly once").toBe(slottedBefore+1);
 await eventuallyV1(async()=>(await claimsActive())===0,20000,"the claim is released");

 const third=checkoutOf("third");for(const name of readdirSync(requester.deps).filter(name=>!name.includes("-"+externHash("c"))))copyFileSync(join(requester.deps,name),join(third.deps,name));
 const rootC=join(scratch,"unit-third"),assignedBefore=cluster.events.filter(event=>event.startsWith("assigned")).length;
 const statusC=await run(third,environmentOf(third,rootC,{SEMIO_OFFLOAD:"never"}));
 expect(statusC).toBe(0);
 expect(callsOf(join(rootC,"rustc-calls.log")),"the third checkout compiled nothing\n"+trace(rootC)+"\nA:\n"+trace(rootA)).toBe(0);
 expect(cluster.events.filter(event=>event.startsWith("assigned")).length,"no worker was asked again").toBe(assignedBefore);
 expect(cluster.cache.store.usage().slotted,"nothing was stored a second time").toBe(slottedBefore+1);
 const events=readdirSync(join(rootC,"x")).flatMap(name=>readFileSync(join(rootC,"x",name),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {kind:string}));
 expect(events.map(row=>row.kind)).toContain("remote-hit");
 expect(unitEntries(rootC)[0]?.slot).toBe(local[0]!.slot);
 expect(await claimsActive()).toBe(0);
},240000);
