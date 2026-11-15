import {expect,test} from "bun:test";
import {decideHostCapacityV1,evaluateHostCapacityV1,lendHostHoldersV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {sqliteHostOracleV1} from "../🔮️sqlite/🟦️.ts";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,maximumQuiet:fixture.policy.maximumQuiet}});
type World=(typeof fixture.worlds)[number];
const plain=<T>(value:T):T=>JSON.parse(JSON.stringify(value));
const view=(world:World)=>{const lent=lendHostHoldersV1(policy,world.ledger as HostGrantV1[],world.queue as HostWaiterV1[],world.nowMs);return plain({ledger:lent.ledger,waiters:lent.waiters,lentWaiters:lent.lentWaiters,lent:Object.fromEntries(lent.lent)});};
const named=(prefix:string)=>fixture.worlds.find(world=>world.name.startsWith(prefix))!;
const evaluate=(world:World)=>plain(evaluateHostCapacityV1(policy,world.host as HostSampleV1,world.ledger as HostGrantV1[],world.queue as HostWaiterV1[],world.nowMs));

test("The lent view and the decision on every stored world equal the Python oracle, and the vectors reach every rule",()=>{
 for(const world of fixture.worlds){expect(view(world),world.name).toEqual(world.view);expect(evaluate(world),world.name).toEqual(world.expect);}
 expect(fixture.protocol.lendKeepBasisPoints).toBe(schema.definitions.Protocol.const.lendKeepBasisPoints);
 expect(fixture.coverage.lent,"worlds with lent workers").toBeGreaterThan(30);
 expect(fixture.coverage.withChildren,"worlds with workers already running").toBeGreaterThan(30);
 const reasons=new Set(fixture.worlds.flatMap(world=>world.expect.decisions.map(row=>`${row.verdict}:${row.reason}`)));
 for(const needed of ["admit:lent","wait:memory","wait:pressure-critical"])expect(reasons.has(needed),`vectors reach ${needed}`).toBe(true);
});

test("The decision on the view equals the recursive SQLite oracle: the lent requests (priced at nothing) and the others are two plain decisions",()=>{
 let compared=0;
 for(const world of fixture.worlds){
  const lent=view(world);
  if(lent.lentWaiters.length===0)continue;
  const free={...world.host,availableMegabytes:world.host.totalMegabytes},unpriced=lent.ledger.map(grant=>({...grant,reservedMegabytes:0,observedMegabytes:0}));
  const first=sqliteHostOracleV1(policy,{name:world.name,host:free as HostSampleV1,ledger:unpriced as HostGrantV1[],queue:lent.lentWaiters.map(waiter=>({...waiter,request:Object.fromEntries(Object.entries(waiter.request).filter(([key])=>key!=="elastic"))})) as HostWaiterV1[],nowMs:world.nowMs});
  expect(first.decisions,world.name).toEqual(plain(decideHostCapacityV1(policy,free as HostSampleV1,unpriced as HostGrantV1[],lent.lentWaiters as HostWaiterV1[],world.nowMs).decisions));
  compared++;
 }
 expect(compared).toBeGreaterThan(30);
});

test("Law: on a 16 GB machine the workers of a graph build run inside its reservation and the parent keeps below 3 GB",()=>{
 const six=named("16 GB machine, show admitted with 6553 MB"),four=named("16 GB machine, show admitted with 4096 MB");
 expect(six.expect.decisions).toEqual([{id:"farmreq0001",verdict:"admit",jobs:2,reservedMegabytes:3072,reason:"lent"}]);
 expect(four.expect.decisions).toEqual([{id:"farmreq0001",verdict:"admit",jobs:1,reservedMegabytes:1536,reason:"lent"}]);
 for(const world of [six,four]){
  const lent=world.view.lent.farmreq0001!,holder=world.ledger[0]!;
  expect(lent.keptMegabytes,"the parent share").toBeLessThan(3000);
  expect(lent.keptMegabytes+world.expect.decisions[0]!.reservedMegabytes,"the parent and its workers stay inside the reservation").toBeLessThanOrEqual(holder.reservedMegabytes);
 }
 const refused=named("16 GB machine, the two workers run");
 expect(refused.expect.decisions[0]!.reason,"a third request finds nothing left to lend and takes the ordinary path").toBe("memory");
 const heavy=named("16 GB machine, the parent holds 5500 MB");
 expect(heavy.expect.decisions[0]!.verdict,"a parent that holds its reservation lends nothing").toBe("wait");
});

test("Law: lending moves the charge and never adds one: once the lent workers are grants of the ledger, the holder plus its children charge no more than the holder's reservation did",()=>{
 let checked=0;
 for(const world of fixture.worlds){
  const lent=evaluate(world).decisions.filter(row=>row.reason==="lent");
  if(lent.length===0)continue;
  const active=(ledger:readonly {lentSinceMs?:number;reservedMegabytes:number}[])=>ledger.filter(grant=>grant.lentSinceMs===undefined).reduce((sum,grant)=>sum+grant.reservedMegabytes,0);
  const started=lent.map(row=>({...world.queue.find(waiter=>waiter.request.id===row.id)!.request,row})).map(({row,...request})=>({id:request.id,operation:request.operation,jobs:row.jobs,cpu:row.jobs*request.cpuPerJob,reservedMegabytes:row.reservedMegabytes,admittedAtMs:world.nowMs,observedMegabytes:null,pid:1,bootAtMs:0,holder:(request as {holder:string}).holder}));
  const after=lendHostHoldersV1(policy,[...world.ledger,...started] as HostGrantV1[],[],world.nowMs);
  expect(active(after.ledger),world.name).toBeLessThanOrEqual(active(world.ledger));
  checked++;
 }
 expect(checked).toBeGreaterThan(30);
});

test("A holder that does not ramp, waits in a queue, is not in the ledger, is quiet or is a child lends nothing; a request that is not elastic or whose minimum does not fit takes the ordinary path",()=>{
 for(const prefix of ["the holder is past its ramp","the holder only waits","the named holder is not in the ledger","a quiet grant holds nothing","a child cannot be a holder","a request that is not elastic","a request below its minimum jobs"]){
  const world=named(prefix);
  expect(Object.keys(world.view.lent),prefix).toEqual([]);
  expect(world.expect.decisions.every(row=>row.reason!=="lent"),prefix).toBe(true);
 }
});

test("Law: a graph rebuild keeps its reservation for the whole rebuild: forty minutes in, its farm workers are still lent from the unused part, and a second rebuild of the same size waits instead of being admitted beside it",()=>{
 const nowMs=1800000000000,host={version:1,atMs:nowMs,platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:27000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:2,openFiles:8000,openFilesMaximum:245760,attachedImages:2} as HostSampleV1;
 const holder=(rampMilliseconds:number)=>({id:"showholder1",operation:"show",jobs:1,cpu:2,reservedMegabytes:14889,admittedAtMs:nowMs-2_400_000,resumedAtMs:nowMs-2_400_000,observedMegabytes:4000,pid:4242,bootAtMs:1700000000000,label:"projects",rampMilliseconds}) as HostGrantV1;
 const farm={request:{id:"farmreq0001",operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:6,fixedMegabytes:0,perJobMegabytes:1536,cpuPerJob:1,elastic:true,label:"graph-farm",holder:"showholder1"},arrivedAtMs:nowMs-1000,pid:4243} as HostWaiterV1;
 const second={request:{id:"showsecond1",operation:"show",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:14889,perJobMegabytes:0,cpuPerJob:2,label:"projects"},arrivedAtMs:nowMs-2000,pid:4244} as HostWaiterV1;
 const verdicts=(rampMilliseconds:number)=>Object.fromEntries(evaluateHostCapacityV1(policy,host,[holder(rampMilliseconds)],[second,farm],nowMs).decisions.map(row=>[row.id,`${row.verdict}:${row.reason}`]));
 expect(verdicts(21_600_000),"protected for the whole rebuild").toEqual({farmreq0001:"admit:lent",showsecond1:"wait:memory"});
 const lapsed=verdicts(1_800_000);
 expect(lapsed.showsecond1,"counted only by what it holds once its learned time to peak is over, a second rebuild of 14889 MB lands beside the first on a host with 28672 MB above the floor").toBe("admit:fits");
 expect(lapsed.farmreq0001,"and the workers of the first are not lent anything").not.toBe("admit:lent");
});

test("The vocabulary agrees: the holder is part of the request and the grant, lent is a reason with an English and a German text",()=>{
 expect(Object.keys(schema.definitions.Request.properties)).toContain("holder");
 expect(Object.keys(schema.definitions.Grant.properties)).toContain("holder");
 expect(schema.definitions.Reason.enum).toContain("lent");
 for(const language of ["en","de"] as const){const text=schema.definitions.Messages.const[language].lent;for(const placeholder of ["{operation}","{holder}","{jobs}","{reserved}"])expect(text,language).toContain(placeholder);}
 expect(schema.definitions.Messages.const.en.lent).not.toBe(schema.definitions.Messages.const.de.lent);
});
