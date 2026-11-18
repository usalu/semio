/** 🥇️ Contract: an owner a farm child could run never falls back to the parent. While the capacity grant of the first worker is pending, `prepare` keeps waiting (the old rule gave up after `grantWaitMilliseconds` and sent every later owner of the read to the parent, the largest process, for good); the first request of a farm is essential and the later ones are not; when the grant comes the owner runs on the child.
 * @see ../../🟨️.mjs */
import {afterEach,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {closeProjectFarmsV1,openProjectFarmV1 as openFarmV1} from "../../🟨️.mjs";
const openProjectFarmV1=(options:Parameters<typeof openFarmV1>[0])=>openFarmV1({adopt:(()=>({published:1,present:0,declined:false})) as never,...options});
import {GRAPH_FARM_POLICY_V1} from "../../🏊️pool/🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";
import {graphFarmIdentityV1} from "../../🪆️captured/🟨️.mjs";
import {FOOTPRINT_POLICY_V1} from "../../../../../../🧠️memory/🦶️footprint/🟨️.mjs";

const roots:string[]=[];
afterEach(async()=>{await closeProjectFarmsV1();for(const root of roots.splice(0))rmSync(root,{recursive:true,force:true});});
class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly log:string[]){super();setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string;id?:number}){
  if(message.type==="run"){this.log.push("run");setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:{names:[],cpuMilliseconds:1},rssMegabytes:1}));}
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
const open=(asked:{essential?:boolean;preferredJobs:number}[],grants:((value:{jobs:number;megabytes:number}|undefined)=>void)[],log:string[],lines:string[])=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r117-farm-")));roots.push(root);
 return openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off"},processed:()=>400,cores:10,write:line=>lines.push(line),engage:true,
  policy:{...GRAPH_FARM_POLICY_V1,grantWaitMilliseconds:150,holdMilliseconds:300},
  spawnProcess:(()=>new Scripted(log)) as never,
  request:(((input:{essential?:boolean;preferredJobs:number})=>{asked.push({essential:input.essential,preferredJobs:input.preferredJobs});return{granted:new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grants.push(done);}),release:()=>undefined};}) as never)})!;
};

test("the first request of a farm is essential, the request for the other workers is not",async()=>{
 const asked:{essential?:boolean;preferredJobs:number}[]=[],grants:((value:{jobs:number;megabytes:number}|undefined)=>void)[]=[],log:string[]=[],lines:string[]=[];
 const farm=open(asked,grants,log,lines);
 expect(asked.length).toBe(1);expect(asked[0]!.essential).toBe(true);
 grants[0]!({jobs:1,megabytes:1536});
 await Bun.sleep(80);
 expect(asked.length,"one worker came, more are wanted").toBe(2);expect(asked[1]!.essential).toBe(false);
 grants[1]?.(undefined);
 await farm.close();
});

test("an owner waits for the pending grant of the first worker for much longer than the grant wait and then runs on the child, never in the parent",async()=>{
 const asked:{essential?:boolean;preferredJobs:number}[]=[],grants:((value:{jobs:number;megabytes:number}|undefined)=>void)[]=[],log:string[]=[],lines:string[]=[];
 const farm=open(asked,grants,log,lines);
 let answer:boolean|undefined;const waiting=farm.prepare(job).then(value=>{answer=value;});
 await Bun.sleep(900);
 expect(answer,"6 times the grant wait and 3 times the hold time: still waiting, not sent to the parent").toBeUndefined();
 expect(log).toEqual([]);
 grants[0]!({jobs:1,megabytes:1536});
 await waiting;
 expect(log,"the owner ran on the child").toEqual(["run"]);
 grants[1]?.(undefined);
 await farm.close();
},20000);

test("a cancelled wait for the grant ends the wait and the owner falls back, so a read that stops is never held",async()=>{
 const asked:{essential?:boolean;preferredJobs:number}[]=[],grants:((value:{jobs:number;megabytes:number}|undefined)=>void)[]=[],log:string[]=[],lines:string[]=[];
 const farm=open(asked,grants,log,lines);
 const waiting=farm.prepare(job);
 await Bun.sleep(100);
 grants[0]!(undefined);
 expect(await waiting).toBe(false);
 await farm.close();
},20000);

test("the parent releases its caches before it holds a child back, and only then: no release when the tree fits, a hold only when the release did not free enough",async()=>{
 const {farmHeldV1}=await import("../../🟨️.mjs");
 const MiB=1048576,grant=6553*MiB,gate={grantBytes:()=>grant},allowance=1500,limit=grant*FOOTPRINT_POLICY_V1.hardPercent/100;
 const calls:number[]=[];
 const run=(treeMegabytes:number,freedMegabytes:number)=>farmHeldV1(gate as never,allowance,{bytes:()=>treeMegabytes*MiB} as never,(()=>{calls.push(freedMegabytes);return freedMegabytes*MiB;}) as never);
 expect(run(1000,500),"fits: nothing to release").toBe(false);expect(calls).toEqual([]);
 const over=Math.ceil(limit/MiB)-allowance+200;
 expect(run(over,100),"the release frees too little").toBe(true);expect(calls.length).toBe(1);
 expect(run(over,1000),"the release frees enough").toBe(false);expect(calls.length).toBe(2);
 expect(farmHeldV1(gate as never,allowance,{bytes:()=>over*MiB} as never,(()=>{throw Error("cache broke");}) as never),"a failing release holds as before").toBe(true);
});

test("a graph read asks for what it needs now, the parent and one worker, and starts with it: the least a show or graph is admitted with covers one worker beside a lean parent of half a worker's size, whatever the learned size of the full request",async()=>{
 const {default:buildPolicy}=await import("../../../../../../🔣️policy.json");
 const operations=(buildPolicy as unknown as {hostCapacity:{operations:{name:string;minimumMegabytes?:number}[]}}).hostCapacity.operations;
 for(const name of ["show","graph"]){
  const row=operations.find(each=>each.name===name)!;
  expect(row.minimumMegabytes,name).toBeGreaterThanOrEqual(Math.floor(1.5*GRAPH_FARM_POLICY_V1.workerMegabytes));
 }
});
