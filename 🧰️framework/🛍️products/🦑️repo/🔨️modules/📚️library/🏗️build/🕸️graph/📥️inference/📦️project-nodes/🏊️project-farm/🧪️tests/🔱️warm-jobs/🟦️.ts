/** 🔱️ Contract: a warm job weighs almost nothing for the recycling of a farm child: a child that is replaced after four derivations serves hundreds of warm jobs on one process, while a derivation still counts one and the memory ceiling still replaces a child.
 * @see ../../🏊️pool/🟨️.mjs */
import{expect,test}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{EventEmitter}from"node:events";
import{readFileSync}from"node:fs";
import{openProjectFarmV1,closeProjectFarmsV1}from"../../🟨️.mjs";
import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";
import{graphFarmIdentityV1}from"../../🪆️captured/🟨️.mjs";
import{farmGrantEnvironmentV1}from"../🧫️fixtures/🎟️grant/🟨️.mjs";
import{appendWarmRowV1,readWarmIndexV1,warmDigestV1,warmWaveV1,WARM_POLICY_V1}from"../../../🔱️warm/🟨️.mjs";
import cancellationCases from"./🛑️cancel.json";
import cancellationSchema from"./🧬️schema/🛑️cancel.json";
import{dirname,join}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{GRAPH_FARM_POLICY_V1,createFarmV1}from"../../🏊️pool/🟨️.mjs";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";

const here=dirname(fileURLToPath(import.meta.url)),worker=join(here,"..","..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🧫️fixtures","🟨️.mjs")).href;
const farm=(options:Record<string,unknown>={})=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:1,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",policy:{...GRAPH_FARM_POLICY_V1,recycleAfterProjects:4},...options} as never);

test("120 warm jobs run on one child process while 8 derivations replace it twice",async()=>{
 const pool=farm(),pids=new Set<number>();
 for(let n=0;n<120;n++)pids.add(((await pool.run({warm:true,n})).reply as{pid:number}).pid);
 expect(pids.size).toBe(1);expect(pool.stats().recycled).toBe(0);
 for(let n=0;n<8;n++)await pool.run({n});
 expect(pool.stats().recycled).toBeGreaterThanOrEqual(2);
 await pool.close();
},120000);

test("the memory ceiling still replaces a child that serves warm jobs",async()=>{
 const pool=farm({megabytes:1,policy:{...GRAPH_FARM_POLICY_V1,recycleAfterProjects:4,recycleCeilingFactor:0}});
 for(let n=0;n<3;n++)await pool.run({warm:true,n});
 expect(pool.stats().recycled).toBe(3);
 await pool.close();
},60000);

test("real children that die now and then never disable the pool: the restart budget counts the deaths in a row, not the deaths of a day",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-die-")),pool=farm();let replies=0;
 for(let n=0;n<30;n++){const outcome=await pool.run({warm:true,n,...(n%3===1?{dieFile:join(directory,"die-"+n)}:{})});if(outcome.reply!==undefined)replies++;}
 expect(replies).toBe(30);expect(pool.stats().restarts).toBeLessThanOrEqual(1);expect(pool.usable).toBe(true);
 await pool.close();rmSync(directory,{recursive:true,force:true});
},120000);

test("stopping the warm lane retires its active worker and removes its queued job without interrupting required work",async()=>{
 expect(new Ajv({strict:true}).validate(cancellationSchema,cancellationCases)).toBe(true);
 using oracle=new Database(":memory:");
 const expected=oracle.query(`SELECT CAST($blocked='warm' AS INTEGER) AS startedWarm,
  CAST($blocked='warm' AS INTEGER) AS cancelledWorkers,1 AS startedObserve,1 AS startedInfer,1 AS durable`);
 for(const row of cancellationCases){
  const root=mkdtempSync(join(tmpdir(),"fw-")),directory=join(root,"w"),wave=new AbortController(),stop=new AbortController();
  const required=Promise.withResolvers<void>(),events:string[]=[],workers:ControlledWorker[]=[];
  const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:farmGrantEnvironmentV1(root,"warm_stop"),signal:wave.signal,cores:10,engage:true,write:()=>{},
   spawnProcess:(()=>{const worker=new ControlledWorker(row.blocked,events,required.promise);workers.push(worker);return worker;}) as never,
   request:(()=>({granted:Promise.resolve(undefined),release(){}})) as never,
   adopt:()=>({published:1,present:0,declined:false})})!;
  try{
   expect(await farm.whenReady(5000)).toBe(true);
   const observe=()=>farm.observe({name:"required-observe",observe:true});
   let observing=row.blocked==="observe"?observe():undefined;
   if(observing)await until(()=>events.includes("observe"));
   const names=["durable",...Array.from({length:WARM_POLICY_V1.minimumOwners},(_,i)=>"warm-"+i)];
   const key=warmDigestV1("durable"),stamp="1:1";
   appendWarmRowV1(directory,{v:1,n:"durable",k:key,d:stamp,o:"settled",t:Date.now()});
   const warming=warmWaveV1({farm,names,cores:4,directory,signal:stop.signal,environment:{},
    describe:(name:string)=>({key:warmDigestV1(name),stamp,cost:Number(name!=="durable"),job:{name,warm:true}})});
   if(row.blocked==="warm"){
    await until(()=>events.includes("warm"));
    observing=observe();
   }
   await until(()=>farm.describe().includes("queued jobs 1"));
   stop.abort();
   const counters=await Promise.race([warming,Bun.sleep(1500).then(()=>{throw Error("stopped warm work did not settle");})]);
   required.resolve();
   expect(await observing).toMatchObject({verdict:"present"});
   const name="required-infer";
   expect(await farm.prepare({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)})).toBe(true);
   expect(counters).toMatchObject({failed:0,done:1,durable:1});
   const count=(event:string)=>events.filter(value=>value===event).length;
   expect<unknown>({startedWarm:count("warm"),cancelledWorkers:workers.filter(worker=>worker.killed).length,startedObserve:count("observe"),startedInfer:count("infer"),durable:readWarmIndexV1(directory).size}).toEqual(expected.get({$blocked:row.blocked}));
   expect(wave.signal.aborted).toBe(false);
  }finally{
   required.resolve();
   wave.abort();
   await farm.close();
   await closeProjectFarmsV1();
   rmSync(root,{recursive:true,force:true});
  }
 }
},20000);

class ControlledWorker extends EventEmitter{
 connected=true;
 exitCode:number|null=null;
 signalCode:string|null=null;
 killed=false;
 constructor(private readonly blocked:string,private readonly events:string[],private readonly required:Promise<void>){
  super();
  setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));
 }
 send(message:{type:string;id?:number;jobFile?:string}){
  if(message.type==="shutdown"){
   this.exitCode=0;
   setImmediate(()=>this.emit("exit"));
  }else if(message.type==="run"){
   const job=JSON.parse(readFileSync(message.jobFile!,"utf8")),kind=job.warm?"warm":job.observe?"observe":"infer";
   this.events.push(kind);
   if(kind==="warm")return true;
   const result=()=>{if(this.exitCode===null)this.emit("message",{type:"result",id:message.id,reply:{verdict:"present",names:["inference/np/"+"0".repeat(24)+".json"],cpuMilliseconds:0},rssMegabytes:1});};
   if(this.blocked===kind)void this.required.then(result);else setImmediate(result);
  }
  return true;
 }
 kill(){
  this.killed=true;
  this.exitCode=0;
  setImmediate(()=>this.emit("exit"));
  return true;
 }
}

async function until(predicate:()=>boolean){
 const deadline=performance.now()+5000;
 while(!predicate()){
  if(performance.now()>deadline)throw Error("expected farm dispatch or queue state");
  await Bun.sleep(5);
 }
}

test("removing a warm queue head admits a required job while another owner is still active",async()=>{
 const stop=new AbortController(),held=Promise.withResolvers<void>(),events:string[]=[];
 const pool=farm({workers:2,limitMegabytes:5,accept:()=>true,spawnProcess:()=>new ControlledWorker("observe",events,held.promise)});
 try{
  expect(await pool.ready).toBe(true);
  await until(()=>pool.snapshot.idle===2);
  const observing=pool.run({name:"active-observation",observe:true,knownMegabytes:3});
  await until(()=>pool.snapshot.busy===1);
  const warming=pool.run({name:"queued-warm",warm:true,knownMegabytes:3},{signal:stop.signal});
  const inferred=pool.run({name:"required-inference",knownMegabytes:1});
  expect(pool.snapshot.queued).toBe(2);
  stop.abort();
  expect(await warming).toEqual({lost:"aborted"});
  await until(()=>events.includes("infer"));
  expect(await inferred).toHaveProperty("reply");
  expect(pool.snapshot.busy).toBe(1);
  expect(events).toEqual(["observe","infer"]);
  held.resolve();
  expect(await observing).toHaveProperty("reply");
 }finally{
  held.resolve();
  await pool.close();
 }
},15000);
