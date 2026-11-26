/** 🚦️ Contract: a waiting request is never overtaken for ever. A heavy agent session asks for 3500 MB beside the 4000 MB it holds (past its session cap) while a light session streams small requests that keep the room saturated; once the request has waited `wait.agingMilliseconds` it heads the line, the cap no longer holds it and the stream is drained behind it, so it is admitted at the second the independent Python oracle names. A session cap that binds only while a competitor waits keeps an idle competitor from holding memory idle.
 * @see ./🐍️.py
 * @see ./🧫️fixtures/🔣️.json */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {evaluateHostCapacityV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import base from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const python=testPythonV1()??Bun.which("python3")??"python3",oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",oracle]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as {admittedAtSecond:number|null;foreignAdmittedAtSecond:number|null};})();
const policy=readHostCapacityPolicyV1({...base.policy,memory:{...base.policy.memory,...fixture.memory}} as never),origin=1000000;
const request=(id:string,megabytes:number)=>({id,operation:"command",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1});
const grant=(id:string,session:string,megabytes:number):HostGrantV1=>({id,operation:"command",jobs:1,cpu:1,reservedMegabytes:megabytes,admittedAtMs:origin-3600000,observedMegabytes:megabytes,pid:1,bootAtMs:0,session}) as never;

const scenario=(wait:Partial<typeof base.policy.wait>)=>{
 const tuned=readHostCapacityPolicyV1({...base.policy,wait:{...base.policy.wait,...wait},memory:{...base.policy.memory,...fixture.memory}} as never),{heavy,light}=fixture;
 let ledger:HostGrantV1[]=[grant("held",heavy.session,heavy.heldMegabytes),...light.initialEndSeconds.map((end,at)=>grant(`seed${at}`,light.session,light.requestMegabytes))];
 const ends=new Map<string,number>(light.initialEndSeconds.map((end,at)=>[`seed${at}`,end]));
 let queue:HostWaiterV1[]=[{request:request("big",heavy.bigMegabytes),arrivedAtMs:origin,pid:1,session:heavy.session}];
 let admittedAt:number|null=null,arrivals=0;
 for(let second=0;second<fixture.horizonSeconds&&admittedAt===null;second+=fixture.stepSeconds){
  const now=origin+second*1000;
  ledger=ledger.filter(row=>!(ends.has(row.id)&&ends.get(row.id)!<=second));
  if(second>=light.firstArrivalSecond&&(second-light.firstArrivalSecond)%light.arrivalEverySeconds===0)queue.push({request:request(`light${arrivals++}`,light.requestMegabytes),arrivedAtMs:now,pid:1,session:light.session});
  const used=ledger.reduce((sum,row)=>sum+row.reservedMegabytes,0),host:HostSampleV1={version:1,atMs:now,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-fixture.foreignMegabytes-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never;
  const evaluation=evaluateHostCapacityV1(tuned,host,ledger,queue,now);
  for(const row of evaluation.decisions.filter(decision=>decision.verdict==="admit")){
   const waiter=queue.find(candidate=>candidate.request.id===row.id)!;
   if(row.id==="big")admittedAt=second;
   ledger=[...ledger,grant(row.id,waiter.session!,row.reservedMegabytes)];ends.set(row.id,second+light.runSeconds);
  }
  queue=queue.filter(waiter=>!evaluation.decisions.some(decision=>decision.id===waiter.request.id&&decision.verdict==="admit"));
 }
 return admittedAt;
};

test("a large request of a heavy session behind a saturating stream of a light session is admitted when the oracle says, within the aging bound; without aging the same stream starves it",()=>{
 const {light}=fixture,admittedAt=scenario({});
 expect(expected.admittedAtSecond).not.toBeNull();
 expect(admittedAt).toBe(expected.admittedAtSecond);
 expect(admittedAt!).toBeLessThanOrEqual(fixture.agingSeconds+light.runSeconds+fixture.stepSeconds);
 expect(scenario({agingMilliseconds:fixture.horizonSeconds*10000}),"the stream and the cap starve the request for the whole horizon").toBeNull();
});

const verdict=(rows:readonly HostGrantV1[],queue:readonly HostWaiterV1[],id:string)=>{
 const used=rows.reduce((sum,row)=>sum+(row.observedMegabytes??row.reservedMegabytes),0),host:HostSampleV1={version:1,atMs:origin,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-fixture.foreignMegabytes-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never;
 return evaluateHostCapacityV1(policy,host,rows,queue,origin).decisions.find(row=>row.id===id)!;
};

test("the session cap counts what the grants of a session are charged, not what they reserved, and binds only while another session waits",()=>{
 const idle=[1,2,3,4].map(at=>({...grant(`idle${at}`,"heavy",2000),observedMegabytes:300})) as HostGrantV1[],full=idle.map(row=>({...row,observedMegabytes:2000})) as HostGrantV1[];
 const mine:HostWaiterV1={request:request("mine",1000),arrivedAtMs:origin,pid:1,session:"heavy"},other:HostWaiterV1={request:request("other",9000),arrivedAtMs:origin,pid:1,session:"light"};
 expect(verdict(idle,[mine,other],"mine"),"8000 MB reserved but 1200 MB charged: under the cap").toMatchObject({verdict:"admit"});
 expect(verdict(full,[mine,other],"mine"),"8000 MB charged while a second session waits: the cap holds").toMatchObject({verdict:"wait",reason:"session-cap"});
 expect(verdict(full,[mine],"mine"),"nobody else waits: judged by memory alone").not.toMatchObject({reason:"session-cap"});
 expect(verdict(full,[mine,{...other,session:"heavy"}],"mine")).not.toMatchObject({reason:"session-cap"});
});

test("two sessions that are both over the cap do not hold each other back: a competitor that the cap itself would refuse is no competitor",()=>{
 const rows=[...[1,2,3,4].map(at=>({...grant(`a${at}`,"heavy",1700),observedMegabytes:1700})),...[1,2,3,4].map(at=>({...grant(`b${at}`,"light",1700),observedMegabytes:200}))] as HostGrantV1[];
 const a:HostWaiterV1={request:request("a",300),arrivedAtMs:origin,pid:1,session:"heavy"},b:HostWaiterV1={request:request("b",3000),arrivedAtMs:origin,pid:1,session:"light"};
 expect(verdict(rows,[a,b],"a"),"light is under the cap and waits: heavy (6800 MB charged) is held").toMatchObject({verdict:"wait",reason:"session-cap"});
 const over=rows.map(row=>row.session==="light"?{...row,observedMegabytes:1700}:row) as HostGrantV1[];
 expect(verdict(over,[a,b],"a"),"light (6800 MB charged) would be refused by the cap itself: heavy is not held for it").not.toMatchObject({reason:"session-cap"});
});

const foreignScenario=(tuned:typeof policy,withBig:boolean)=>{
 const {foreign}=fixture,small=(id:string)=>({request:request(id,foreign.smallMegabytes),arrivedAtMs:0,pid:1}) as HostWaiterV1;
 let ledger:HostGrantV1[]=foreign.initialEndSeconds.map((end,at)=>({...grant(`seed${at}`,"light",foreign.smallMegabytes),session:undefined,admittedAtMs:origin-3600000}) as HostGrantV1);
 const ends=new Map<string,number>(foreign.initialEndSeconds.map((end,at)=>[`seed${at}`,end]));
 let queue:HostWaiterV1[]=withBig?[{request:request("big",foreign.bigMegabytes),arrivedAtMs:origin,pid:1}]:[],arrivals=0,admittedBig:number|null=null,admittedSmall=0;
 for(let second=0;second<fixture.horizonSeconds&&admittedBig===null;second+=fixture.stepSeconds){
  const now=origin+second*1000;
  ledger=ledger.filter(row=>!(ends.has(row.id)&&ends.get(row.id)!<=second));
  if(second%foreign.arrivalEverySeconds===0)queue.push({...small(`light${arrivals++}`),arrivedAtMs:now});
  const used=ledger.reduce((sum,row)=>sum+row.reservedMegabytes,0),held=second%foreign.periodSeconds===foreign.lowAtSecond?foreign.lowMegabytes:foreign.highMegabytes;
  const host:HostSampleV1={version:1,atMs:now,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-held-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never;
  const evaluation=evaluateHostCapacityV1(tuned,host,ledger,queue,now);
  for(const row of evaluation.decisions.filter(decision=>decision.verdict==="admit")){
   const waiter=queue.find(candidate=>candidate.request.id===row.id)!;
   if(row.id==="big"){admittedBig=second;continue;}
   admittedSmall++;ledger=[...ledger,{...grant(row.id,"light",row.reservedMegabytes),session:undefined,admittedAtMs:now} as HostGrantV1];ends.set(row.id,second+foreign.runSeconds);
  }
  queue=queue.filter(waiter=>!evaluation.decisions.some(decision=>decision.id===waiter.request.id&&decision.verdict==="admit"));
 }
 return {admittedBig,admittedSmall,waiting:queue.length};
};

test("a large request held back by foreign memory that dips for one step a period is reserved for once it has waited bypassMilliseconds: the stream cannot take the room, and the admission second equals the oracle",()=>{
 const run=foreignScenario(policy,true);
 expect(expected.foreignAdmittedAtSecond).not.toBeNull();
 expect(run.admittedBig).toBe(expected.foreignAdmittedAtSecond);
 expect(run.admittedBig!).toBeLessThanOrEqual(fixture.foreign.periodSeconds+fixture.foreign.runSeconds);
 const patient=readHostCapacityPolicyV1({...base.policy,wait:{...base.policy.wait,bypassMilliseconds:3600000,maximumBypasses:1000},memory:{...base.policy.memory,...fixture.memory}} as never);
 expect(foreignScenario(patient,true).admittedBig,"without the reservation the stream keeps the dip from ever fitting").toBeNull();
});

test("small requests still flow while no large request waits",()=>{
 const run=foreignScenario(policy,false);
 expect(run.admittedSmall).toBeGreaterThan(fixture.horizonSeconds/fixture.foreign.arrivalEverySeconds/2);
 expect(run.waiting).toBeLessThan(fixture.foreign.initialEndSeconds.length+3);
});

test("a head that is feasible on this host is never refused as starved, however long it waits, while a request that can never fit still is",()=>{
 const bounded=readHostCapacityPolicyV1({...base.policy,wait:{...base.policy.wait,starvedMilliseconds:1800000},memory:{...base.policy.memory,...fixture.memory}} as never),late=origin+bounded.wait.starvedMilliseconds!*3,held=Array.from({length:6},(_,at)=>({...grant(`h${at}`,"light",1500),session:undefined,admittedAtMs:late-1000}) as HostGrantV1);
 const sample=(used:number)=>({version:1,atMs:late,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0}) as never;
 const feasible:HostWaiterV1={request:request("big",7680),arrivedAtMs:origin,pid:1},never:HostWaiterV1={request:request("huge",fixture.totalMegabytes),arrivedAtMs:origin,pid:1};
 expect(evaluateHostCapacityV1(bounded,sample(9000),held,[feasible],late).decisions[0]).toMatchObject({verdict:"wait",reason:"memory"});
 expect(evaluateHostCapacityV1(bounded,sample(9000),held,[never],late).decisions[0]).toMatchObject({verdict:"wait",reason:"starved"});
});

test("the session share never blocks the head of a session behind its own small grants: past agingMilliseconds the cap no longer applies and the head is reserved for",()=>{
 const own=[1,2,3,4].map(at=>grant(`own${at}`,"heavy",1000)),big:HostWaiterV1={request:request("big",7680),arrivedAtMs:origin,pid:1,session:"heavy"},other:HostWaiterV1={request:request("other",9000),arrivedAtMs:origin,pid:1,session:"light"};
 const small:HostWaiterV1={request:request("small",300),arrivedAtMs:origin+1000,pid:1,session:"heavy"};
 expect(verdict(own,[big,other],"big"),"young: the cap holds the heavy session").toMatchObject({verdict:"wait",reason:"session-cap"});
 const later=origin+policy.wait.agingMilliseconds+1000,at=(rows:readonly HostGrantV1[],queue:readonly HostWaiterV1[])=>{const used=rows.reduce((sum,row)=>sum+row.reservedMegabytes,0);return evaluateHostCapacityV1(policy,{version:1,atMs:later,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-fixture.foreignMegabytes-used,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never,rows,queue,later).decisions;};
 expect(at(own,[big,other]).find(row=>row.id==="big")).not.toMatchObject({reason:"session-cap"});
 const rows=at(own,[big,{...small,arrivedAtMs:later-1000}]);
 expect(rows.find(row=>row.id==="big")).toMatchObject({verdict:"wait",reason:"memory"});
 expect(rows.find(row=>row.id==="small"),"its own later small request does not overtake it").toMatchObject({verdict:"wait",reason:"queue-order"});
 expect(at([],[big])[0]).toMatchObject({verdict:"admit"});
});
