import {afterAll,describe,expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {auditIsolationV1,readRehearsalOptionsV1,rehearsalEnvironmentV1,rehearsalLimitsV1,rehearsalMachinePlansV1,rehearsalVerdictV1,markedProcessRowsV1,rehearsalHostV1,runRehearsalV1,REHEARSAL_POLICY_V1} from "../🟦️.ts";

/** 🏟️ Contract: the rehearsal gives every simulated machine its own identity, state and memory, derives the limits of a small worker through the build library's own seams, judges a run by duplicate work, readiness, footprint, throughput and orphans, and leaves neither a process nor a directory behind. */
const options=readRehearsalOptionsV1({machines:3,memoryMegabytes:16384,cores:8,owners:40,ownerSeconds:1,scenario:"steady",timeoutSeconds:120});

describe("plan",()=>{
 test("every machine has its own origin, directories, ports and marker",()=>{
  const plans=rehearsalMachinePlansV1(options,"abc","/w");
  expect(plans.map(plan=>plan.origin)).toEqual(["rehearsal-m1","rehearsal-m2","rehearsal-m3"]);
  for(const key of ["state","temporary","runtime","capacity"] as const)expect(new Set(plans.map(plan=>plan[key])).size).toBe(3);
  const environment=rehearsalEnvironmentV1(plans[1]!,{PATH:"/bin"});
  expect(environment.SEMIO_FLEET_ORIGIN).toBe("rehearsal-m2");
  expect(environment.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES).toBe("16384");
  expect(environment.TMPDIR).toBe(plans[1]!.temporary);
  expect(environment.SEMIO_FLEET_STORE_ROOT).toBe("/w/store");
  expect(environment.SEMIO_SERVICE_LAUNCHD).toBe("off");
  expect(environment.SEMIO_REMOTE_CACHE_TOKEN).toBeUndefined();
 });
 test("the declared machine reaches the build through its own seam",()=>{
  const plan=rehearsalMachinePlansV1(options,"abc","/w")[0]!,host=rehearsalHostV1(rehearsalEnvironmentV1(plan,{}));
  expect(host.totalMegabytes).toBe(16384);expect(host.cores).toBeLessThanOrEqual(8);expect(host.declared).toBe(true);
 });
 test("the limits of a 16 GB and an 8 GB worker come from the build library",()=>{
  expect(rehearsalLimitsV1(16384).planMegabytes).toBe(6553);
  expect(rehearsalLimitsV1(8192).planMegabytes).toBeLessThan(6553);
  expect(rehearsalLimitsV1(16384).child.hardMegabytes).toBeGreaterThan(rehearsalLimitsV1(8192).child.hardMegabytes-1);
 });
 test("options are closed",()=>{
  expect(()=>readRehearsalOptionsV1({...options,machines:99})).toThrow();
  expect(()=>readRehearsalOptionsV1({...options,extra:1})).toThrow();
  expect(()=>readRehearsalOptionsV1({...options,memoryMegabytes:12000})).toThrow();
 });
});

describe("process table",()=>{
 test("only rows that carry the marker of this run are ours",()=>{
  const text=[" 10 1 1000 1.0 bun x --rehearsal=abc --machine=rehearsal-m1"," 11 10 2000 2.5 bun y --rehearsal=abc --machine=rehearsal-m2"," 12 1 9 0.0 bun z --rehearsal=other --machine=rehearsal-m1"," 13 1 9 0.0 vim"].join("\n");
  expect(markedProcessRowsV1(text,"abc")).toEqual([{pid:10,ppid:1,rssMegabytes:1000/1024,cpu:1,machine:"rehearsal-m1"},{pid:11,ppid:10,rssMegabytes:2000/1024,cpu:2.5,machine:"rehearsal-m2"}]);
 });
});

describe("verdict",()=>{
 const base=(over:object={})=>({options,limits:rehearsalLimitsV1(16384),machines:["rehearsal-m1","rehearsal-m2"],totalOwners:10,samples:[{atSeconds:0,machine:"rehearsal-m1",ledger:{total:10,present:0,fetched:0,here:0,wanted:10,done:0},pool:{state:"pulling",reason:null,running:2,open:10},footprintMegabytes:100,cpu:50},{atSeconds:5,machine:"rehearsal-m1",ledger:{total:10,present:0,fetched:5,here:5,wanted:0,done:10},pool:{state:"idle",reason:"no-wave",running:0,open:0},footprintMegabytes:4000,cpu:10},{atSeconds:5,machine:"rehearsal-m2",ledger:{total:10,present:0,fetched:5,here:5,wanted:0,done:10},pool:{state:"idle",reason:null,running:0,open:0},footprintMegabytes:300,cpu:1}],readyAtSeconds:{"rehearsal-m1":5,"rehearsal-m2":5},puts:[{machine:"rehearsal-m1",key:"a",atSeconds:1},{machine:"rehearsal-m2",key:"b",atSeconds:2}],claims:[{machine:"rehearsal-m1",key:"a",state:"granted",atSeconds:1}],orphans:0,workRemoved:true,elapsedSeconds:5,...over});
 test("a clean run passes",()=>{
  const verdict=rehearsalVerdictV1(base());
  expect(verdict.pass).toBe(true);expect(verdict.allReady).toBe(true);expect(verdict.duplicateOwners).toEqual([]);expect(verdict.peakFootprintMegabytes["rehearsal-m1"]).toBe(4000);expect(verdict.ownersPerMinute).toBe(120);expect(verdict.orphans).toBe(0);
 });
 test("an owner stored twice, a machine that never got ready, a leftover process and a footprint over the limit each fail it",()=>{
  expect(rehearsalVerdictV1(base({puts:[{machine:"rehearsal-m1",key:"a",atSeconds:1},{machine:"rehearsal-m2",key:"a",atSeconds:2}]})).duplicateOwners).toEqual(["a"]);
  expect(rehearsalVerdictV1(base({readyAtSeconds:{"rehearsal-m1":5}})).pass).toBe(false);
  expect(rehearsalVerdictV1(base({orphans:2})).pass).toBe(false);
  const over=base();over.samples[1]!.footprintMegabytes=7000;
  const verdict=rehearsalVerdictV1(over);expect(verdict.withinLimit["rehearsal-m1"]).toBe(false);expect(verdict.pass).toBe(false);
 });
 test("a landing records the stored results it lost and the time the pools sat idle with owners open",()=>{
  const verdict=rehearsalVerdictV1(base({landing:{atSeconds:2,storedBefore:6,movedRevision:true},samples:[...base().samples,{atSeconds:10,machine:"rehearsal-m1",ledger:{total:10,present:0,fetched:0,here:0,wanted:10,done:0},pool:{state:"idle",reason:"no-current-wave",running:0,open:10},footprintMegabytes:90,cpu:0},{atSeconds:15,machine:"rehearsal-m1",ledger:{total:10,present:0,fetched:0,here:0,wanted:10,done:0},pool:{state:"idle",reason:"no-current-wave",running:0,open:10},footprintMegabytes:90,cpu:0}]}));
  expect(verdict.landing?.storedLost).toBe(6);expect(verdict.landing?.poolIdleSeconds).toBeGreaterThan(0);
 });
});

describe("audit",()=>{
 test("the audit names the state of the checkout that a machine cannot redirect",()=>{
  const audit=auditIsolationV1(process.cwd().includes("semio")?process.env.SEMIO_WORKSPACE_ROOT??"/Users/ueli/Documents/semio":"/Users/ueli/Documents/semio");
  expect(audit.shared.some(row=>row.path===".cache/b")).toBe(true);
  expect(audit.shared.every(row=>row.reason.length>0)).toBe(true);
 });
});

describe("run",()=>{
 const work:string[]=[];
 afterAll(()=>{});
 test("two machines infer 12 owners exactly once, both get ready, nothing is left behind",async()=>{
  const small=readRehearsalOptionsV1({machines:2,memoryMegabytes:16384,cores:4,owners:12,ownerSeconds:0.5,ownerMegabytes:32,scenario:"steady",timeoutSeconds:90});
  const result=await runRehearsalV1(small);work.push(result.workDirectory);
  expect(result.verdict.allReady).toBe(true);
  expect(result.verdict.duplicateOwners).toEqual([]);
  expect(result.verdict.orphans).toBe(0);
  expect(result.verdict.workRemoved).toBe(true);
  expect(existsSync(result.workDirectory)).toBe(false);
  expect(result.timeline.length).toBeGreaterThan(0);
  expect(REHEARSAL_POLICY_V1.markerFlag).toBe("--rehearsal=");
 },120000);
});
