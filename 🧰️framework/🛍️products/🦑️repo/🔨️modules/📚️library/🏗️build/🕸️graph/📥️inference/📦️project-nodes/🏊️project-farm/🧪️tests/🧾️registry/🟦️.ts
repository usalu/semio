/** 🧾️ Contract: an edit of the owner registry inside one process keeps the worker pools of the read (the key of a hub never names the registry), every job names the digest of the registry it was asked under, and a worker that holds another registry reads the current one once, not once per job. Oracle: the independent registry digest of `../../../../../🧾️registry-reads/🧪️tests/🐍️.py`. */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {EventEmitter} from "node:events";
import {mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import fixture from "../../../../../🧾️registry-reads/🧪️tests/🔣️.json";
import {closeProjectFarmsV1,openProjectFarmV1} from "../../🟨️.mjs";
import {adoptProjectInferenceRegistryV1} from "../../../🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";
import {ownerRegistryDigestOnceV1} from "../../../../../🧾️registry-reads/🟨️.mjs";
import {graphFarmIdentityV1} from "../../🪆️captured/🟨️.mjs";
import {farmGrantEnvironmentV1} from "../🧫️fixtures/🎟️grant/🟨️.mjs";

const roots:string[]=[],scratch=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"graph-farm-registry-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const oracle=()=>{const run=spawnSync("python3",["-I",join(import.meta.dir,"..","..","..","..","..","🧾️registry-reads","🧪️tests","🐍️.py"),join(import.meta.dir,"..","..","..","..","..","🧾️registry-reads","🧪️tests","🔣️.json")],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const edited=()=>{const changed=structuredClone(fixture.registry) as any,target=changed.owners.find((owner:any)=>owner.name==="@semio-tech/epsilon");target.packages=["epsilon","epsilon-extra"];return changed;};

class Worker extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly jobs:{registryDigest?:string}[]){super();setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string;id?:number;jobFile?:string}){
  if(message.type==="run"){this.jobs.push(JSON.parse(readFileSync(message.jobFile!,"utf8")));setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:{names:["inference/np/000000000000000000000000.json"],cpuMilliseconds:1},rssMegabytes:1}));}
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});

test("a registry edit between two constructions keeps the workers of the read and every job names the digest of the registry of its construction",async()=>{
 const root=scratch(),environment=farmGrantEnvironmentV1(root,"farm_registry_01"),children:Worker[]=[],jobs:{registryDigest?:string}[]=[];
 const make=(registry:unknown)=>openProjectFarmV1({workspaceRoot:root,context:{registry},contracts:{},launch:null,projects:48,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{...environment,SEMIO_GRAPH_WORKERS:"7"},processed:()=>40,cores:10,write:()=>undefined,engage:true,
  spawnProcess:(()=>{const child=new Worker(jobs);children.push(child);return child;}) as never,
  request:(()=>({granted:Promise.resolve(undefined),release:()=>undefined})) as never,
  adopt:(()=>({published:1,present:0,declined:false})) as never} as never)!;
 const first=make(fixture.registry);await new Promise<number>(done=>first.onReady(done));
 expect(await first.prepare(job("a"))).toBe(true);
 const started=children.length;
 const second=make(edited());
 expect(await second.prepare(job("b"))).toBe(true);
 expect(children.length,"the pools of the read serve the edited registry without a start").toBe(started);
 expect(jobs.map(each=>each.registryDigest)).toEqual([oracle().digest,ownerRegistryDigestOnceV1(edited())]);
 expect(jobs[1]!.registryDigest).not.toBe(jobs[0]!.registryDigest);
 await first.close();await second.close();await closeProjectFarmsV1();
},30000);

test("a worker keeps the registry it holds while its parent names that digest and reads the current registry once when it differs",()=>{
 const root=scratch(),file=join(root,"📋️build-owners.json"),admitted=(value:unknown)=>JSON.parse(JSON.stringify(value));
 const base=admitted(fixture.registry),next=admitted(edited()),environment={workspaceRoot:root,context:{registry:base},registryAskedFor:undefined as string|undefined};
 writeFileSync(file,JSON.stringify(next));
 adoptProjectInferenceRegistryV1(environment,ownerRegistryDigestOnceV1(base) as string);
 expect(environment.context.registry,"the digest it holds: no read").toBe(base);
 adoptProjectInferenceRegistryV1(environment,undefined as never);
 expect(environment.context.registry).toBe(base);
 const wanted=ownerRegistryDigestOnceV1(next) as string;
 adoptProjectInferenceRegistryV1(environment,wanted);
 expect(ownerRegistryDigestOnceV1(environment.context.registry)).toBe(wanted);
 expect(environment.registryAskedFor).toBe(wanted);
 const held=environment.context.registry;rmSync(file);
 adoptProjectInferenceRegistryV1(environment,wanted);
 expect(environment.context.registry,"asked twice for the same digest: no second read").toBe(held);
 adoptProjectInferenceRegistryV1(environment,"0".repeat(64));
 expect(environment.context.registry,"an unreadable registry keeps the one held").toBe(held);
});
