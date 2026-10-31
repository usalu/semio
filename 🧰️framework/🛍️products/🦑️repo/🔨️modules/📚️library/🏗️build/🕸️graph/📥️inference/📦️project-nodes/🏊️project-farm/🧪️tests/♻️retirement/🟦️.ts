/** ♻️ Proves a paid worker slot is retained until every retiring process actually exits. */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import Ajv from"ajv";
import fixture from"./🔣️.json";
import contract from"../../🏊️pool/🧬️schema/🔣️.json";
import{createFarmV1,GRAPH_FARM_POLICY_V1}from"../../🏊️pool/🟨️.mjs";

const validate=new Ajv({strict:true}).compile(contract);
class DelayedChild extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;shutdown=false;
 constructor(readonly rss:number,readonly outcome="result"){super();}
 answer(message:unknown){expect(validate(message),JSON.stringify(validate.errors)).toBe(true);this.emit("message",message);}
 send(message:{type:string;id?:number}){
  expect(validate(message),JSON.stringify(validate.errors)).toBe(true);
  if(message.type==="init")queueMicrotask(()=>this.answer({type:"ready",pid:1,identity:"law"}));
  if(message.type==="run"){const job=JSON.parse(readFileSync((message as unknown as{jobFile:string}).jobFile,"utf8"));expect(job.value).toBe(7);queueMicrotask(()=>this.answer(this.outcome==="result"?{type:"result",id:message.id,reply:7,rssMegabytes:this.rss}:{type:"failure",id:message.id,name:"Error",message:"controlled failure",code:null,rssMegabytes:this.rss}));}
  if(message.type==="shutdown")this.shutdown=true;
  return true;
 }
 kill(){this.shutdown=true;return true;}
 finish(){this.exitCode=0;this.connected=false;this.emit("exit",0,null);}
}

test("worker replacement and pool close retain retiring processes until actual exit",async()=>{

 const db=new Database(":memory:");
 try{for(const row of fixture.cases){
  const children:DelayedChild[]=[],retired:number[]=[],policy={...GRAPH_FARM_POLICY_V1,recycleAfterProjects:row.reason==="projects"?row.jobs:100,shutdownGraceMilliseconds:1000},pool=createFarmV1({entry:"law-worker",service:"file:///law-service",session:{},workers:1,megabytes:10,policy,accept:identity=>identity==="law",spawnProcess:(()=>{const child=new DelayedChild(row.rssMegabytes,row.outcome);children.push(child);return child;})as never,retired:(_slot:number,generation:number)=>retired.push(generation)});
  try{
   const assertOutcome=(answer:any)=>row.outcome==="result"?expect(answer.reply).toBe(7):expect(answer.failure.message).toBe("controlled failure");
   assertOutcome(await pool.run({value:7}));
   const pending=pool.run({value:7});await Bun.sleep(0);
   const beforeExit=children.length;
   children[0]!.finish();assertOutcome(await pending);
   const afterExit=children.length;let closed=false;
   const closing=pool.close().then(()=>{closed=true;});await Bun.sleep(0);
   const closedBeforeExit=closed;children[1]?.finish();await closing;
   const actual={beforeExit,afterExit,closedBeforeExit},oracle=db.query("select 1 beforeExit,2 afterExit,0 closedBeforeExit").get()as{beforeExit:number;afterExit:number;closedBeforeExit:number};
   expect(actual,row.reason).toEqual(fixture.expected);expect({...oracle,closedBeforeExit:Boolean(oracle.closedBeforeExit)}).toEqual(fixture.expected);expect(retired).toEqual([1,2]);expect(pool.pending).toBe(0);
  }finally{for(const child of children)if(child.exitCode===null)child.finish();await pool.close();}
 }}finally{db.close();}
});

import{openProjectFarmV1}from"../../🟨️.mjs";
import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";

test("a refused worker retains its host capacity grant until its process exits",async()=>{
 const children:DelayedChild[]=[],db=new Database(":memory:");let released=0;
 const farm=openProjectFarmV1({workspaceRoot:process.cwd(),context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{SEMIO_LANGUAGE:"en"},processed:()=>24,cores:10,write:()=>undefined,request:(()=>({granted:Promise.resolve({jobs:1,megabytes:3584}),release:()=>{released++;}}))as never,spawnProcess:(()=>{const child=new DelayedChild(1);child.answer=(message:unknown)=>{const row=message as{type:string;identity?:unknown};if(row.type==="ready")row.identity={implementation:"different",records:inferenceRecordIdentityV1()};expect(validate(row),JSON.stringify(validate.errors)).toBe(true);child.emit("message",row);};children.push(child);return child;})as never})!;
 const job={name:"law",root:"law",json:{},native:false,pipeline:false,nativeRoot:"law",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
 try{
  for(let i=0;i<24;i++)await farm.prepare(job);await Bun.sleep(0);
  expect(children.length).toBe(1);expect(children[0]!.shutdown).toBe(true);
  const beforeExit=released;children[0]!.finish();await Bun.sleep(0);
  const afterExit=released,oracle=db.query("select 0 beforeExit,1 afterExit").get();
  expect({beforeExit,afterExit}).toEqual(fixture.release);expect(oracle).toEqual(fixture.release);
 }finally{for(const child of children)if(child.exitCode===null)child.finish();await farm.close();db.close();}
});

import{resolve}from"node:path";
import{pathToFileURL}from"node:url";
test("real worker failures carry measured memory and obey the same recycling policy",async()=>{
 const retired:number[]=[],pool=createFarmV1({entry:resolve(import.meta.dir,"../../👷️worker/🟨️.mjs"),service:pathToFileURL(resolve(import.meta.dir,"../🧫️fixtures/🛰️service/🟨️.mjs")).href,session:{identity:"law"},workers:1,megabytes:1,accept:identity=>identity==="law",retired:(_slot:number,generation:number)=>retired.push(generation)}),db=new Database(":memory:");
 try{
  for(const n of fixture.failures.jobs)expect(await pool.run({kind:"throw",n})).toEqual({failure:{name:"Error",message:"boom "+n,code:"BOOM"}});
  await pool.close();const stats=pool.stats(),oracle=db.query("select count(*) failures,count(*) recycled,count(*) retired from json_each(?)").get(JSON.stringify(fixture.failures.jobs));
  expect({failures:stats.failures,recycled:stats.recycled,retired:retired.length}).toEqual(fixture.failures.expected);expect(oracle).toEqual(fixture.failures.expected);expect(stats.peakRssMegabytes).toBeGreaterThan(0);
 }finally{await pool.close();db.close();}
});
