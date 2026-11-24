import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{join}from"node:path";
import{OWNER_POOL_POLICY_V1 as P,ownerPoolTargetV1,ownerPoolWaveOrderV1,pullOwnerPoolV1}from"../../🟦️.ts";
import type{ProjectClaimV1}from"../../../../../../🪪️graph-state/📦️project-records/🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};

/** 🧭️ Law: the fleet infers owners like it compiles crates. Machines with given spare memory serve as many owners at once as their spare memory holds, every owner is inferred and published exactly once, the owners a read waits for (tier 0: selection and priority) go before the owners it only looks ahead to (tier 1), and the claim of a machine that died with owners in flight lapses and the others infer them. The oracle is an independent Python implementation of the admission by spare memory.
 * @see ./🐍️oracle.py */
type Machine=(typeof fixture.scenarios)[number]["machines"][number];
type Unit={name:string;tier:number};
const policy={...P,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
const pause=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const hostOf=(machine:Machine)=>({cores:machine.cores,totalMegabytes:machine.totalMegabytes,availableMegabytes:machine.availableMegabytes,pressure:"normal" as const,load:null,swapOutMegabytesPerSecond:null,foreground:{cpu:machine.busy?2:0,unrealizedMegabytes:0,waiters:0}});

const oracle=(()=>{
 const result=spawnSync("python3",["-I",join(import.meta.dir,"🐍️oracle.py"),join(import.meta.dir,"🔣️.json"),join(import.meta.dir,"..","..","🧬️schema","🔣️.json")],{encoding:"utf8"});
 expect(result.status,result.stderr).toBe(0);
 return JSON.parse(result.stdout) as {slots:number[];totalSlots:number;owners:number;tier0:number;names:string[];served:string[]}[];
})();

test("the admission by spare memory of every machine equals the independent Python implementation",()=>{
 const ours=fixture.scenarios.map(scenario=>scenario.machines.map(machine=>ownerPoolTargetV1({host:hostOf(machine),children:0,childMegabytes:machine.childMegabytes}).children));
 expect(ours).toEqual(oracle.map(row=>row.slots));
});

type Claim={by:string;at:number};
async function simulate(scenario:(typeof fixture.scenarios)[number],expected:(typeof oracle)[number]){
 const units:Unit[]=[...Array.from({length:scenario.owners.tier0},(_all,index)=>({name:`t0-${String(index).padStart(3,"0")}`,tier:0})),...Array.from({length:scenario.owners.tier1},(_all,index)=>({name:`t1-${String(index).padStart(3,"0")}`,tier:1}))];
 const table=new Map<string,"stored"|Claim>(),published:string[]=[],runs=new Map<string,number>(),starts:{name:string;machine:string}[]=[],dead=new Set<string>(),inFlight=new Map<string,Set<string>>(),open:string[]=[];
 const invariant:string[]=[];
 const kill=(scenario as {kill?:{machine:string;afterStarts:number}}).kill;
 const controllers=new Map(scenario.machines.map(machine=>[machine.name,new AbortController()]));
 const claimOf=(machine:string)=>async(unit:Unit):Promise<ProjectClaimV1>=>{
  await pause(2);
  const row=table.get(unit.name);
  if(row==="stored")return{state:"stored"};
  if(row!==undefined&&Date.now()-row.at<fixture.claimTtlMilliseconds)return{state:"held",by:row.by};
  table.set(unit.name,{by:machine,at:Date.now()});
  return{state:"granted",digest:unit.name,why:()=>"",code:()=>"",deterministic:()=>false,publish:async()=>{if(dead.has(machine))return false;if(table.get(unit.name)==="stored"){invariant.push("published twice "+unit.name);return false;}table.set(unit.name,"stored");published.push(unit.name);return true;},abandon:async()=>{if(!dead.has(machine)&&table.get(unit.name)!=="stored")table.delete(unit.name);}};
 };
 const fleet=scenario.machines.map((machine,index)=>{
  const slots=expected.slots[index]!;
  if(slots===0)return Promise.resolve();
  inFlight.set(machine.name,new Set());
  return pullOwnerPoolV1<Unit>({units:()=>units,puller:`${machine.name}:1`,policy,slots:()=>slots,signal:controllers.get(machine.name)!.signal,claim:claimOf(machine.name),
   run:async unit=>{
    runs.set(unit.name,(runs.get(unit.name)??0)+1);starts.push({name:unit.name,machine:machine.name});inFlight.get(machine.name)!.add(unit.name);
    if(unit.tier===1&&scenario.owners.tier0>0&&!(kill!==undefined))for(const first of units)if(first.tier===0&&table.get(first.name)===undefined)invariant.push(`tier 1 ${unit.name} started while ${first.name} is unclaimed`);
    if(kill!==undefined&&!dead.has(kill.machine)&&starts.length>=kill.afterStarts&&machine.name===kill.machine){dead.add(kill.machine);open.push(...inFlight.get(kill.machine)!);controllers.get(kill.machine)!.abort();}
    await pause(fixture.runMilliseconds);
    inFlight.get(machine.name)!.delete(unit.name);
    return!dead.has(machine.name);
   }});
 });
 await Promise.race([Promise.all(fleet),pause(20000)]);
 return{units,table,published,runs,starts,dead,lost:open,invariant};
}

for(const [index,scenario] of fixture.scenarios.entries()){
 test(`schedule: ${scenario.name}`,async()=>{
  const expected=oracle[index]!,result=await simulate(scenario,expected);
  expect(result.invariant).toEqual([]);
  expect(expected.totalSlots,"the machines have spare capacity").toBeGreaterThan(0);
  expect([...result.published].sort(),"every owner is published").toEqual(expected.names);
  expect(new Set(result.published).size,"and published exactly once").toBe(result.published.length);
  for(const [name,count] of result.runs)expect(count,`${name} runs once, twice only after a machine died with it in flight`).toBeLessThanOrEqual(result.lost.includes(name)?2:1);
  const idle=scenario.machines.filter((_machine,at)=>expected.slots[at]===0).map(machine=>machine.name);
  expect(result.starts.filter(row=>idle.includes(row.machine)),"a machine without spare memory infers nothing").toEqual([]);
  if(result.dead.size>0){expect(result.lost.length,"the killed machine had owners in flight").toBeGreaterThan(0);for(const name of result.lost)expect(result.published,`${name} is published by another machine`).toContain(name);}
  if(scenario.owners.tier0>0&&scenario.owners.tier1>0&&result.dead.size===0)expect(result.starts.some(row=>row.name.startsWith("t1-")),"the lookahead owners are inferred too").toBe(true);
 },60000);
}

test("a machine serves the wave a command waits for before the background read of the whole graph, then the wave with fewest children, then the newest",()=>{
 const waves=[{id:"whole",priority:1,children:0,createdMs:30},{id:"selection-busy",priority:0,children:3,createdMs:10},{id:"selection-idle",priority:0,children:1,createdMs:5},{id:"selection-idle-new",priority:0,children:1,createdMs:9},{id:"legacy",children:2,createdMs:40}];
 expect([...waves].sort(ownerPoolWaveOrderV1).map(wave=>wave.id)).toEqual(["selection-idle-new","selection-idle","legacy","selection-busy","whole"]);
 expect([...waves].reverse().sort(ownerPoolWaveOrderV1).map(wave=>wave.id)).toEqual(["selection-idle-new","selection-idle","legacy","selection-busy","whole"]);
});

test("a single machine claims every selection owner before any lookahead owner",async()=>{
 const units:Unit[]=[...Array.from({length:10},(_all,index)=>({name:`t1-${index}`,tier:1})),...Array.from({length:6},(_all,index)=>({name:`t0-${index}`,tier:0}))],order:string[]=[];
 await pullOwnerPoolV1<Unit>({units:()=>units,puller:"solo:1",policy,slots:()=>1,claim:async unit=>({state:"granted",digest:unit.name,why:()=>"",code:()=>"",deterministic:()=>false,publish:async()=>true,abandon:async()=>undefined}),run:async unit=>{order.push(unit.name);return true;}});
 expect(order.length).toBe(16);
 expect(order.slice(0,6).every(name=>name.startsWith("t0-"))).toBe(true);
 expect(order.slice(6).every(name=>name.startsWith("t1-"))).toBe(true);
});
