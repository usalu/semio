/** 🚧️ Contract: a job that a farm child could take never goes to the parent while the farm exists. `immediate` starts the children at the creation of the farm; a pool that stays silent keeps the job in the queue with the reason shown (no hold time ends it); a child that fails the job three times ends it with a refusal that names the owner and the reason, it is not inferred in the parent in its place; `observe` (the verdict and the closure references) waits for a child without a limit and answers undefined only when the farm is gone; `exists` says whether the farm is still there.
 * @see ../../🟨️.mjs */
import {afterEach,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {closeProjectFarmsV1,openProjectFarmV1} from "../../🟨️.mjs";
import {GRAPH_FARM_POLICY_V1} from "../../🏊️pool/🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";
import {graphFarmIdentityV1} from "../../🪆️captured/🟨️.mjs";

const roots:string[]=[];
afterEach(async()=>{await closeProjectFarmsV1();for(const root of roots.splice(0))rmSync(root,{recursive:true,force:true});});
type Mode="ok"|"fail"|"silent"|"verdict";
class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly mode:Mode,readonly log:string[]){super();if(mode!=="silent")setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string;id?:number;jobFile?:string}){
  if(message.type==="run"){
   const observing=JSON.parse(readFileSync(message.jobFile!,"utf8")).observe===true;
   this.log.push(observing?"observe":"run");
   if(this.mode==="fail")setImmediate(()=>this.emit("message",{type:"failure",id:message.id,name:"Error",message:"the child broke on it"}));
   else setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:observing?{verdict:"holds",headerDigest:"a".repeat(64),names:[],cpuMilliseconds:1}:{names:["np/x.json"],cpuMilliseconds:1},rssMegabytes:1}));
  }else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
const open=(mode:Mode,log:string[],lines:string[],asked:{essential?:boolean}[]=[],extra:Record<string,unknown>={})=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"r122-farm-")));roots.push(root);
 return openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off"},processed:()=>0,cores:10,write:(line:string)=>{lines.push(line);},immediate:true,
  policy:{...GRAPH_FARM_POLICY_V1,grantWaitMilliseconds:150,holdMilliseconds:300},
  adopt:(()=>({published:1,present:0,declined:false})) as never,
  spawnProcess:(()=>new Scripted(mode,log)) as never,
  request:(((input:{essential?:boolean})=>{asked.push({essential:input.essential});return{granted:asked.length===1?Promise.resolve({jobs:1,megabytes:1536}):new Promise<{jobs:number;megabytes:number}|undefined>(()=>undefined),release:()=>undefined};}) as never),
  ...extra} as never)!;
};

test("immediate asks the host at the creation of the farm and starts the own pool, before any owner misses",async()=>{
 const asked:{essential?:boolean}[]=[],log:string[]=[],lines:string[]=[];
 const farm=open("ok",log,lines,asked);
 expect(asked.length,"the host is asked for the children at once").toBe(1);
 expect(farm.exists()).toBe(true);
 await farm.close();
});

test("a pool that stays silent keeps the owner in the queue for longer than the hold time, with the reason shown, and the parent infers nothing",async()=>{
 const log:string[]=[],lines:string[]=[];
 const farm=open("silent",log,lines);
 let answer:boolean|undefined;const waiting=farm.prepare(job).then(value=>{answer=value;});
 await Bun.sleep(1200);
 expect(answer,"4 times the hold time: still queued").toBeUndefined();
 expect(farm.exists(),"the farm is still there, so the parent computes nothing").toBe(true);
 expect(lines.some(line=>line.includes("waits in the queue")&&line.includes("Warteschlange")),lines.join("\n")).toBe(true);
 expect(log).toEqual([]);
 await farm.close();void waiting;
},20000);

test("a child that fails an owner three times ends it with a refusal naming the owner and the reason; the owner is never inferred in the parent",async()=>{
 const log:string[]=[],lines:string[]=[];
 const farm=open("fail",log,lines);
 let caught:any;try{await farm.prepare(job);}catch(error){caught=error;}
 expect(caught?.code).toBe("graph.farm-job-failed");expect(caught.semioProject).toBe("p");
 expect(caught.message).toContain("the child broke on it");expect(caught.message).toContain("Elternprozess");
 expect(log,"three tries").toEqual(["run","run","run"]);
 await farm.close();
},20000);

test("observe waits for a child without a limit and answers the verdict; it answers undefined only when the farm is gone",async()=>{
 const log:string[]=[],lines:string[]=[];
 const farm=open("verdict",log,lines);
 const reply=await farm.observe({name:"p",observe:true} as never);
 expect(reply?.verdict).toBe("holds");expect(log).toEqual(["observe"]);
 const silent=open("silent",[],[],[],{projects:567});
 let answer:unknown="pending";const waiting=silent.observe({name:"q",observe:true} as never).then(value=>{answer=value;});
 await Bun.sleep(700);
 expect(answer,"queued, not answered undefined after the warm deadline of a short wait").toBe("pending");
 await silent.close();await closeProjectFarmsV1();
 await waiting;
 expect(answer,"the farm is gone: the parent may verify itself").toBeUndefined();
 expect(farm.exists()).toBe(false);
},20000);

test("a child that fails an observe job three times refuses it instead of returning undefined",async()=>{
 const farm=open("fail",[],[]);
 let caught:any;try{await farm.observe({name:"p",observe:true} as never);}catch(error){caught=error;}
 expect(caught?.code).toBe("graph.farm-job-failed");expect(caught.message).toContain("farm children could not infer p");
 await farm.close();
},20000);
