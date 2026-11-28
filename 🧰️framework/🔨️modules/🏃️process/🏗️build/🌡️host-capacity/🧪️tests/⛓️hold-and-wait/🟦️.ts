/** ⛓️ Contract: admitted jobs can never all block on their own growth for ever, and no request queues for ever that this host cannot serve. Deterministic one-second simulations drive the real decision, hold-and-wait and unreachable laws through JSON scenarios (a four-job hold-and-wait, the same without resolution, a reduced growth, an unsatisfiable request, a large request aging past a stream); the independent Python oracle names the timeline.
 * @see ./🐍️oracle/🐍️.py
 * @see ./🧫️fixtures/🔣️.json
 * @see ./🧬️schema/🔣️.json */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {evaluateHostCapacityV1,hostGrowthLabelV1,hostGrowthParentV1,hostGrowthOfV1,hostLeastMegabytesV1,hostPossibleMegabytesV1,hostRequestUnreachableV1,hostUnreachableSinceV1,readHostCapacityPolicyV1,requireHostCapacityV1,resolveHostHoldAndWaitV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import base from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import requireSchema from "../../🧬️schema/🔣️.json";

type Scenario=typeof fixture.scenarios[number];
type Event=[number,string,string,number];
const python=testPythonV1()??Bun.which("python3")??"python3",oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",oracle]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as Record<string,Event[]>;})();
const ORDER=["finish","requeue","admit","grow","preempt","refuse"],origin=1_000_000;

type Entry={id:string;kind:"job"|"plain";priority:"batch"|"interactive";arrive:number;grant:number;minimum:boolean;work:number;extra:number;floor?:number;ask:number;state:"pending"|"queued"|"running"|"asked"|"grown"|"done"|"refused";admitAt:number;askedAt:number;grown:number;end:number;kill?:number;marked:boolean;resumed:boolean};

const simulate=(scenario:Scenario):Event[]=>{
 const policy=readHostCapacityPolicyV1({...base.policy,memory:{...base.policy.memory,...fixture.memory},wait:{...base.policy.wait,...scenario.wait}} as never);
 const entries=new Map<string,Entry>(),blank={state:"pending" as const,admitAt:0,askedAt:0,grown:0,end:0,marked:false,resumed:false,extra:0,ask:0,minimum:false};
 for(const job of scenario.jobs)entries.set(job.id,{...blank,id:job.id,kind:"job",priority:job.priority as "batch",arrive:job.arriveSecond,grant:job.grantMegabytes,work:job.workSeconds,extra:job.growthMegabytes,ask:job.askAfterSeconds,..."floorMegabytes" in job?{floor:job.floorMegabytes as number}:{}});
 for(const request of scenario.requests)entries.set(request.id,{...blank,id:request.id,kind:"plain",priority:request.priority as "batch",arrive:request.arriveSecond,grant:request.megabytes,work:request.workSeconds,minimum:"minimum" in request&&request.minimum===true});
 for(const stream of scenario.streams)for(let at=0;at<stream.count;at++)entries.set(`${stream.prefix}-${at}`,{...blank,id:`${stream.prefix}-${at}`,kind:"plain",priority:stream.priority as "batch",arrive:stream.firstSecond+at*stream.everySeconds,grant:stream.megabytes,work:stream.workSeconds});
 const events:Event[]=[],since=new Map<string,number|undefined>(),live=(entry:Entry)=>entry.state==="running"||entry.state==="asked"||entry.state==="grown";
 for(let second=0;second<scenario.horizonSeconds;second++){
  const nowMs=origin+second*1000;
  for(const entry of entries.values()){
   if(entry.kind==="job"&&entry.state==="grown"&&entry.end<=second){entry.state="done";events.push([second,"finish",entry.id,entry.grant+entry.grown]);}
   if(entry.kind==="plain"&&entry.state==="running"&&entry.end<=second){entry.state="done";events.push([second,"finish",entry.id,entry.grant]);}
   if(entry.kill!==undefined&&entry.kill<=second){Object.assign(entry,{state:"queued",marked:false,resumed:true});delete entry.kill;events.push([second,"requeue",entry.id,0]);}
   if(entry.state==="pending"&&entry.arrive===second)entry.state="queued";
   if(entry.kind==="job"&&entry.state==="running"&&second>=entry.admitAt+(entry.resumed?0:entry.ask)){entry.state="asked";entry.askedAt=second;}
  }
  const rows=[...entries.values()].filter(live);
  const ledger:HostGrantV1[]=rows.flatMap(entry=>[{id:entry.id,operation:entry.kind==="job"?"cargo":"command",jobs:1,cpu:1,reservedMegabytes:entry.grant,admittedAtMs:origin+entry.admitAt*1000,observedMegabytes:entry.grant,pid:1,bootAtMs:0,priority:entry.priority,label:entry.id} as HostGrantV1,...entry.state==="grown"?[{id:`${entry.id}-grow`,operation:"cargo-growth",jobs:1,cpu:1,reservedMegabytes:entry.grown,admittedAtMs:origin+(entry.end-entry.work)*1000,observedMegabytes:entry.grown,pid:1,bootAtMs:0,priority:"batch",label:hostGrowthLabelV1(entry.id)} as HostGrantV1]:[]]);
  const request=(id:string,operation:string,priority:"batch"|"interactive",megabytes:number,more:object={})=>({id,operation,priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,...more});
  const waiters:HostWaiterV1[]=[...entries.values()].flatMap(entry=>entry.state==="asked"?[{request:request(`${entry.id}-ask`,"cargo-growth","batch",entry.extra,{label:hostGrowthLabelV1(entry.id),...entry.floor===undefined?{}:{minimumMegabytes:entry.extra,floorMegabytes:entry.floor}}),arrivedAtMs:origin+entry.askedAt*1000,pid:1}]:entry.state==="queued"?[{request:request(entry.id,entry.kind==="job"?"cargo":"command",entry.priority,entry.grant,entry.minimum?{minimumMegabytes:entry.grant}:{}),arrivedAtMs:origin+entry.arrive*1000,pid:1}]:[]);
  const used=ledger.reduce((sum,row)=>sum+row.reservedMegabytes,0),host:HostSampleV1={version:1,atMs:nowMs,platform:"darwin",cores:16,totalMegabytes:scenario.totalMegabytes,availableMegabytes:scenario.totalMegabytes-scenario.foreignMegabytes-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never;
  const evaluation=evaluateHostCapacityV1(policy,host,ledger,waiters,nowMs),verdict=new Map(evaluation.decisions.map(row=>[row.id,row] as const));
  for(const row of evaluation.decisions)if(row.verdict==="admit"){
   const growth=row.id.endsWith("-ask"),entry=entries.get(growth?row.id.slice(0,-4):row.id)!;
   if(growth){Object.assign(entry,{state:"grown",grown:row.reservedMegabytes,end:second+entry.work});events.push([second,"grow",entry.id,row.reservedMegabytes]);}
   else if(entry.kind==="job"){Object.assign(entry,{state:"running",admitAt:second});events.push([second,"admit",entry.id,row.reservedMegabytes]);}
   else{Object.assign(entry,{state:"running",admitAt:second,end:second+entry.work});events.push([second,"admit",entry.id,row.reservedMegabytes]);}
  }
  const stall=resolveHostHoldAndWaitV1(policy,{ledger,waiters,decisions:evaluation.decisions,spareMegabytes:evaluation.headroomMegabytes,nowMs});
  if(stall.kind==="preempt")for(const victim of stall.victims){const entry=entries.get(victim)!;if(!entry.marked){entry.marked=true;entry.kill=second+scenario.preemptLatencySeconds;events.push([second,"preempt",victim,0]);}}
  const reach=hostPossibleMegabytesV1(policy,host,ledger);
  for(const waiter of waiters){
   if(verdict.get(waiter.request.id)?.verdict!=="wait")continue;
   const next=hostUnreachableSinceV1(since.get(waiter.request.id),{leastMegabytes:hostLeastMegabytesV1(policy,waiter.request),possibleMegabytes:reach.possibleMegabytes,nowMs});
   since.set(waiter.request.id,next);
   if(hostRequestUnreachableV1(policy,next,nowMs)){events.push([second,"refuse",waiter.request.id.replace(/-ask$/u,""),0]);entries.get(waiter.request.id.replace(/-ask$/u,""))!.state="refused";}
  }
 }
 return events.sort((left,right)=>left[0]-right[0]||ORDER.indexOf(left[1])-ORDER.indexOf(right[1])||(left[2]<right[2]?-1:left[2]>right[2]?1:0));
};

test("the scenario fixture satisfies its schema",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Fixture,fixture,schema)).toEqual([]);
});

for(const scenario of fixture.scenarios)test(`scenario ${scenario.name}: the simulated timeline equals the oracle`,()=>{
 expect(simulate(scenario)).toEqual(expected[scenario.name]!);
});

const find=(name:string)=>simulate(fixture.scenarios.find(row=>row.name===name)!);

test("four jobs that all wait for their growth are resolved by preempting the youngest batch grant once; every job finishes",()=>{
 const events=find("four-job-hold-and-wait");
 expect(events.filter(row=>row[1]==="preempt")).toEqual([[133,"preempt","job-3-hold",0]]);
 expect(events.filter(row=>row[1]==="finish").map(row=>row[2]).sort()).toEqual(["job-1-hold","job-2-hold","job-3-hold","job-4-hold","publication-1"]);
});

test("without the deadlock time the same jobs never finish: that is the failure being fixed",()=>{
 expect(find("four-job-without-resolution").filter(row=>row[1]==="finish")).toEqual([]);
});

test("a growth that accepts a lower memory budget starts reduced and nothing is preempted",()=>{
 const events=find("four-job-reduced-growth");
 expect(events.some(row=>row[1]==="preempt")).toBe(false);
 expect(events.find(row=>row[1]==="grow"&&row[2]==="job-2-hold")![3]).toBeLessThan(2200);
});

test("a request that can never fit beside other applications is refused after the unreachable time while smaller ones run",()=>{
 const events=find("unsatisfiable-request");
 expect(events.filter(row=>row[1]==="refuse")).toEqual([[180,"refuse","command-big-1",0]]);
 expect(events.filter(row=>row[1]==="admit").map(row=>row[2])).toEqual(["request-small-2","request-small-3"]);
});

test("a large request is admitted past a saturating stream once it aged, and starves without aging",()=>{
 expect(find("large-request-aging").some(row=>row[1]==="admit"&&row[2]==="large-batch-1")).toBe(true);
 expect(find("large-request-without-aging").some(row=>row[1]==="admit"&&row[2]==="large-batch-1")).toBe(false);
});

const policy=readHostCapacityPolicyV1({...base.policy,wait:{...base.policy.wait,deadlockMilliseconds:1000}} as never);
const grant=(id:string,at:number,priority:"batch"|"interactive"="batch",more:object={}):HostGrantV1=>({id,operation:"cargo",jobs:1,cpu:1,reservedMegabytes:1000,admittedAtMs:at,observedMegabytes:1000,pid:1,bootAtMs:0,priority,...more}) as never;
const growth=(parent:string,arrivedAtMs:number,megabytes=3000):HostWaiterV1=>({request:{id:`${parent}-ask`,operation:"cargo-growth",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,label:hostGrowthLabelV1(parent)},arrivedAtMs,pid:1}) as never;
const wait=(...ids:string[])=>ids.map(id=>({id,verdict:"wait" as const}));

test("a hold-and-wait needs every running grant stalled and nothing admitted this round",()=>{
 const ledger=[grant("grant-aaaa1",1),grant("grant-bbbb2",2)],waiters=[growth("grant-aaaa1",0),growth("grant-bbbb2",0)],facts={ledger,waiters,decisions:wait("grant-aaaa1-ask","grant-bbbb2-ask"),spareMegabytes:2100,nowMs:5000};
 expect(resolveHostHoldAndWaitV1(policy,facts)).toMatchObject({kind:"preempt",victims:["grant-bbbb2"]});
 expect(resolveHostHoldAndWaitV1(policy,{...facts,waiters:[waiters[0]!]}),"a grant that does not wait for growth still progresses").toEqual({kind:"none"});
 expect(resolveHostHoldAndWaitV1(policy,{...facts,nowMs:500}),"not yet waited long enough").toEqual({kind:"none"});
 expect(resolveHostHoldAndWaitV1(policy,{...facts,decisions:[{id:"grant-aaaa1-ask",verdict:"admit"}]}),"something was admitted").toEqual({kind:"none"});
 expect(resolveHostHoldAndWaitV1(readHostCapacityPolicyV1(base.policy as never),facts),"no deadlock time: no resolution").toEqual({kind:"none"});
});

test("the victim is the lowest priority, then the youngest, and only as many as it takes",()=>{
 const ledger=[grant("grant-aaaa1",1,"batch"),grant("grant-bbbb2",5,"batch"),grant("grant-cccc3",9,"interactive")],waiters=ledger.map(row=>growth(row.id,0)),facts={ledger,waiters,decisions:wait(...waiters.map(row=>row.request.id)),spareMegabytes:2100,nowMs:5000};
 expect(resolveHostHoldAndWaitV1(policy,facts)).toMatchObject({kind:"preempt",victims:["grant-bbbb2"],releasedMegabytes:1000});
 const big=waiters.map(row=>growth(hostGrowthParentV1(row.request.label)!,0,3500));
 expect(resolveHostHoldAndWaitV1(policy,{...facts,waiters:big})).toMatchObject({victims:["grant-bbbb2","grant-aaaa1"],releasedMegabytes:2000});
 expect(resolveHostHoldAndWaitV1(policy,{...facts,waiters:waiters.map(row=>growth(hostGrowthParentV1(row.request.label)!,0,90000))}),"nothing would ever fit: the unreachable rule refuses instead").toEqual({kind:"none"});
});

test("a growth belongs to a live grant that is not lent",()=>{
 const row=growth("grant-aaaa1",0);
 expect(hostGrowthOfV1([grant("grant-aaaa1",1)],row)).toBe("grant-aaaa1");
 expect(hostGrowthOfV1([grant("grant-aaaa1",1,"batch",{lentSinceMs:2})],row)).toBeUndefined();
 expect(hostGrowthOfV1([],row)).toBeUndefined();
});

test("the unreachable timer restarts when the host could give enough for a moment",()=>{
 const limit=readHostCapacityPolicyV1({...base.policy,wait:{...base.policy.wait,unreachableMilliseconds:1000}} as never);
 let since=hostUnreachableSinceV1(undefined,{leastMegabytes:5000,possibleMegabytes:3000,nowMs:100});
 expect(since).toBe(100);
 since=hostUnreachableSinceV1(since,{leastMegabytes:5000,possibleMegabytes:3000,nowMs:900});
 expect(hostRequestUnreachableV1(limit,since,1099)).toBe(false);
 expect(hostRequestUnreachableV1(limit,since,1100)).toBe(true);
 expect(hostUnreachableSinceV1(since,{leastMegabytes:5000,possibleMegabytes:5000,nowMs:1000})).toBeUndefined();
 expect(hostRequestUnreachableV1(readHostCapacityPolicyV1(base.policy as never),0,1e9)).toBe(false);
});

test("the on-disk record formats are unchanged: the parent of a growth travels in the label, never in a new field",()=>{
 const defs=(requireSchema as unknown as {definitions:Record<string,{properties:Record<string,unknown>}>}).definitions;
 for(const name of ["Request","Grant","Waiter"])expect(Object.keys(defs[name]!.properties),name).not.toContain("parentGrant");
 const asked=growth("grant-aaaa1",0).request;
 expect(requireHostCapacityV1("Request",asked)).toBe(asked);
 expect(hostGrowthParentV1(asked.label)).toBe("grant-aaaa1");
 expect(hostGrowthParentV1("unit-growth")).toBeUndefined();
});
