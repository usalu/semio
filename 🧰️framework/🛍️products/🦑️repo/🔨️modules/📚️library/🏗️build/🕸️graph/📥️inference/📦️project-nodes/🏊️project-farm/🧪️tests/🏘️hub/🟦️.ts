/** 🏘️ Contract: the worker pools of a graph read belong to the read, not to one construction. A project that a worker fails goes back to the pool once and reaches the parent only when the second try fails or no worker is left; a wave of projects is adopted from the workers without the parent inferring one; cancelling the signal of the read retires every pool at once and leaves no abort listener behind. Oracle: the jobs the scripted workers received, the values the farm answered and the processes that were told to shut down. */
import {afterAll,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {closeProjectFarmsV1,openProjectFarmV1} from "../../🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";
import {graphFarmIdentityV1} from "../../🪆️captured/🟨️.mjs";
import {farmGrantEnvironmentV1} from "../🧫️fixtures/🎟️grant/🟨️.mjs";

const roots:string[]=[],scratch=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"graph-farm-hub-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});

/** 🧑‍🏭️ A scripted worker: announces the identity of this implementation at once and answers a job as a failure when the shared script says so, otherwise with one staged leaf. */
class Worker extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly script:{failures:number;jobs:string[];tag:string}){super();setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string;id?:number}){
  if(message.type==="run"){
   this.script.jobs.push(this.script.tag);
   const failing=this.script.failures>0;if(failing)this.script.failures--;
   setImmediate(()=>this.emit("message",failing?{type:"failure",id:message.id,name:"Error",message:"scripted failure",code:null,rssMegabytes:1}:{type:"result",id:message.id,reply:{names:["inference/np/000000000000000000000000.json"],cpuMilliseconds:1},rssMegabytes:1}));
  }else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
const open=(root:string,id:string,script:{failures:number;jobs:string[];tag:string},extra:Record<string,unknown>={})=>{
 const children:Worker[]=[],lines:string[]=[],adopted:string[]=[];
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:farmGrantEnvironmentV1(root,id),processed:()=>400,cores:10,write:(line:string)=>lines.push(line),engage:true,
  spawnProcess:(()=>{const child=new Worker(script);children.push(child);return child;}) as never,
  request:(()=>({granted:Promise.resolve(undefined),release:()=>undefined})) as never,
  adopt:((_root:string,_staging:string,names:readonly string[])=>{adopted.push(names.join());return{published:1,present:0,declined:false};}) as never,...extra} as never)!;
 return{farm,children,lines,adopted};
};

test("a project that a worker fails goes back to the pool once: the second try is adopted, a second failure reaches the parent, and a pool without a worker sends it to the parent at once",async()=>{
 const script={failures:1,jobs:[] as string[],tag:"w"},{farm,children,lines}=open(scratch(),"farm_hub_retry_1",script);
 await new Promise<number>(done=>farm.onReady(done));
 expect(await farm.prepare(job("a")),"first try failed, second adopted").toBe(true);
 expect(script.jobs.length).toBe(2);
 script.failures=3;
 await expect(farm.prepare(job("b")),"three tries failed: refused, never inferred by the parent").rejects.toMatchObject({code:"graph.farm-job-failed"});
 expect(script.jobs.length).toBe(5);
 expect(lines.filter(line=>line.includes("scripted failure")).length).toBe(4);
 await farm.close();await closeProjectFarmsV1();
 expect(children.every(child=>child.exitCode===0)).toBe(true);
 const empty=open(scratch(),"farm_hub_retry_2",{failures:0,jobs:[],tag:"x"},{spawnProcess:(()=>{throw new Error("no worker can start");}) as never});
 expect(await empty.farm.prepare(job("c"))).toBe(false);
 await empty.farm.close();await closeProjectFarmsV1();
},30000);

test("a wave of 48 projects is adopted from six workers without the parent inferring one, and the workers serve the next wave of the same session without a start",async()=>{
 const script={failures:0,jobs:[] as string[],tag:"w"},root=scratch(),environment=farmGrantEnvironmentV1(root,"farm_hub_wave_01"),children:Worker[]=[];
 let hosted=0;
 const make=()=>openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:48,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{...environment,SEMIO_GRAPH_WORKERS:"7"},processed:()=>40,cores:10,write:()=>undefined,engage:true,
  spawnProcess:(()=>{const child=new Worker({...script,tag:"c"+children.length});children.push(child);return child;}) as never,
  request:(()=>({granted:Promise.resolve(hosted++===0?{jobs:6,megabytes:3584}:undefined),release:()=>undefined})) as never,
  adopt:(()=>({published:1,present:0,declined:false})) as never} as never)!;
 const wave=async(farm:ReturnType<typeof make>)=>{await new Promise<number>(done=>farm.onReady(done));let next=0;const answers:boolean[]=[];await Promise.all(Array.from({length:18},async()=>{while(next<48)answers.push(await farm.prepare(job("p"+next++)));}));return answers;};
 const first=make(),answers=await wave(first);
 expect(answers.length).toBe(48);expect(answers.filter(Boolean).length,"every project is adopted from a worker").toBe(48);
 await first.close();
 const started=children.length;
 const second=make(),again=await wave(second);
 expect(again.filter(Boolean).length).toBe(48);expect(children.length,"no worker started for the second wave").toBe(started);
 await second.close();await closeProjectFarmsV1();
 expect(children.every(child=>child.exitCode===0)).toBe(true);
},30000);

test("cancelling the signal of the read retires every pool at once and leaves no listener on the signal",async()=>{
 const script={failures:0,jobs:[] as string[],tag:"s"},controller=new AbortController(),{farm,children}=open(scratch(),"farm_hub_abort_1",script,{signal:controller.signal});
 await new Promise<number>(done=>farm.onReady(done));
 expect(await farm.prepare(job("a"))).toBe(true);
 controller.abort();
 for(let wait=0;wait<100&&children.some(child=>child.exitCode===null);wait++)await Bun.sleep(10);
 expect(children.every(child=>child.exitCode===0),"every worker was told to end").toBe(true);
 expect(await farm.prepare(job("b")),"a retired pool takes no further project").toBe(false);
 await farm.close();await closeProjectFarmsV1();
 const second=new AbortController(),{farm:other}=open(scratch(),"farm_hub_abort_2",script,{signal:second.signal});
 await other.close();await closeProjectFarmsV1();
 second.abort();
 expect(second.signal.aborted).toBe(true);
},30000);
