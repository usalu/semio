/** 🎚️ Contract: a grant is charged what it uses, not what it feared. On the live 16 GB gate (5304 MB available, two explicit-memory commands and a graph plan reserving 10.1 GB while they held 5.2 GB) seven requests of 1.2 to 3.6 GB waited 7 to 22 minutes under "-2558 MB spare". After a warm-up of 60 s and three heartbeats the charge of a retained grant converges within 120 s to its observed peak plus a margin (a floor, three standard deviations of its own measurements, 15 % of the slack its class estimate holds), never under what it holds and never over its reservation; the freed headroom is admittable at once; a grant that grows back is charged in full again at its next heartbeat; the 17:30 counters of 2026-11-27 (swap 95 % used, 705 MB/s out, pressure warn) still block every request whatever the charges are. The charges, the headroom and the verdicts of every case equal those of the independent Python oracle.
 * @see ./🐍️.py
 * @see ./🔣️.json
 * @see ../../📏️decision/🟦️.ts `elasticHostMegabytesV1` */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json";
import {elasticHostMegabytesV1,evaluateHostCapacityV1,readHostCapacityPolicyV1,withUnmanagedHostV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../📏️decision/🟦️.ts";
import {hostBootAtV1,readGrants} from "../../🎫️gate/📖️status/🟦️.ts";
import fixture from "./🔣️.json";

type Oracle={name:string;charges:Record<string,number>;unrealizedMegabytes:number;headroomMegabytes:number;probe:"admit"|"wait"}[];
const python=testPythonV1()??Bun.which("python3")??"python3",script=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",script]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as Oracle;})();
const now=1_000_000_000_000,policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),{memory:{elastic:_,...rigid}}=buildPolicy.hostCapacity,plain=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:rigid} as never);
type Case=(typeof fixture)["cases"][number]&{ages?:Record<string,number>;observations?:number;overrides?:Record<string,Partial<HostGrantV1>>;elastic?:boolean};
const ledgerOf=(row:Case):HostGrantV1[]=>fixture.grants.map(each=>{
 const grant=each as (typeof fixture)["grants"][number]&{peakMegabytes?:number;deviationMegabytes?:number;observations?:number;retain?:boolean;quiet?:boolean},age=(row.ages?.[each.id]??each.ageSeconds)*1000,over=row.overrides?.[each.id]??{};
 return {id:each.id.padEnd(8,"_"),operation:each.operation,jobs:1,cpu:1,reservedMegabytes:each.reservedMegabytes,observedMegabytes:each.observedMegabytes,admittedAtMs:now-age,pid:100,bootAtMs:0,label:each.operation,priority:"batch",session:`a3fba9d52db0.${each.session}`,...grant.retain?{retainReservation:true}:{},...grant.quiet?{quiet:true,rampMilliseconds:300000}:{},...grant.peakMegabytes===undefined?{}:{peakMegabytes:grant.peakMegabytes,deviationMegabytes:grant.deviationMegabytes,observations:row.observations??grant.observations},...over} as never;
});
const hostOf=(row:Case,pressure:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:fixture.cores,totalMegabytes:fixture.totalMegabytes,availableMegabytes:row.availableMegabytes,pressure:"normal",swapUsedMegabytes:fixture.swapUsedMegabytes,swapOutMegabytesPerSecond:fixture.swapOutMegabytesPerSecond,load:fixture.load,openFiles:6000,openFilesMaximum:122880,attachedImages:21,compressedMegabytes:fixture.compressedMegabytes,...pressure}) as HostSampleV1;
const probe=(megabytes:number,id="probe",waited=600):HostWaiterV1=>({arrivedAtMs:now-waited*1000,pid:1,session:"a3fba9d52db0.z",request:{id:id.padEnd(8,"_"),operation:"command",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,label:"command",retainReservation:true}}) as never;
const decide=(row:Case,megabytes:number,host:Partial<HostSampleV1>={},use=row.elastic===false?plain:policy)=>{const ledger=ledgerOf(row);return evaluateHostCapacityV1(use,withUnmanagedHostV1(hostOf(row,host),ledger),ledger,[probe(megabytes)],now);};

test("every case gives the charges, the headroom and the verdict of the oracle",()=>{
 expect(expected.length).toBe(fixture.cases.length);
 for(const [at,row] of (fixture.cases as Case[]).entries()){
  const want=expected[at]!,ledger=ledgerOf(row),use=row.elastic===false?plain:policy,evaluation=decide(row,row.probe);
  expect(want.name).toBe(row.name);
  for(const grant of ledger)if(grant.retainReservation===true)expect(Math.min(elasticHostMegabytesV1(use,grant,now)??grant.reservedMegabytes,grant.reservedMegabytes),`${row.name}: ${grant.id}`).toBeLessThanOrEqual(grant.reservedMegabytes);
  expect({unrealized:evaluation.unrealizedMegabytes,headroom:evaluation.headroomMegabytes,verdict:evaluation.decisions[0]!.verdict},row.name).toEqual({unrealized:want.unrealizedMegabytes,headroom:want.headroomMegabytes,verdict:want.probe});
 }
});

test("the live snapshot: the 7 minute old requests of 1200 and 1900 MB are admitted, 3600 MB waits for real room, and without elastic reservations nothing fits",()=>{
 const live=fixture.cases[2] as Case;
 expect(decide(live,1200).decisions[0]).toMatchObject({verdict:"admit"});
 expect(decide(live,1900).decisions[0]).toMatchObject({verdict:"admit"});
 expect(decide(live,3600).decisions[0]).toMatchObject({verdict:"wait",reason:"memory"});
 const rigid=fixture.cases[7] as Case;
 expect(decide(rigid,1200).decisions[0]).toMatchObject({verdict:"wait"});
 expect(decide(rigid,1200).headroomMegabytes).toBeLessThan(0);
});

test("the charge: whole reservation while warming up, never under the peak or what the grant holds, never over the reservation, nothing for a quiet grant, a holder or a farm child",()=>{
 const grant=(over:Partial<HostGrantV1>):HostGrantV1=>({id:"g".repeat(8),operation:"command",jobs:1,cpu:1,reservedMegabytes:3600,observedMegabytes:1557,admittedAtMs:now-600000,pid:1,bootAtMs:0,retainReservation:true,peakMegabytes:1600,deviationMegabytes:120,observations:26,...over} as never);
 expect(elasticHostMegabytesV1(policy,grant({}),now)).toBe(1960);
 expect(elasticHostMegabytesV1(policy,grant({admittedAtMs:now-59000}),now),"warm-up").toBeUndefined();
 expect(elasticHostMegabytesV1(policy,grant({observations:2}),now),"observations").toBeUndefined();
 expect(elasticHostMegabytesV1(policy,grant({peakMegabytes:undefined}),now),"no peak yet").toBeUndefined();
 expect(elasticHostMegabytesV1(policy,grant({admittedAtMs:now-60000}),now),"converging starts at the reservation").toBe(3600);
 expect(elasticHostMegabytesV1(policy,grant({admittedAtMs:now-120000}),now)).toBe(3600-Math.floor((3600-1960)*60000/120000));
 expect(elasticHostMegabytesV1(policy,grant({resumedAtMs:now-1000}),now),"a resumed grant warms up again").toBeUndefined();
 expect(elasticHostMegabytesV1(policy,grant({observedMegabytes:3000,peakMegabytes:1600}),now),"holds more than its peak: the margin follows what it holds").toBe(3360);
 expect(elasticHostMegabytesV1(policy,grant({peakMegabytes:1000,observedMegabytes:200,deviationMegabytes:0}),now),"floor margin and class share").toBe(1000+Math.max(256,Math.ceil(2600*0.15)));
 expect(elasticHostMegabytesV1(policy,grant({peakMegabytes:5000,observedMegabytes:5000}),now),"above the reservation").toBe(3600);
 for(const over of [{quiet:true},{holder:"h".repeat(8)},{label:"graph-farm"}])expect(elasticHostMegabytesV1(policy,grant(over as never),now)).toBeUndefined();
 expect(elasticHostMegabytesV1(plain,grant({}),now)).toBeUndefined();
});

test("the policy carries the elastic rule and every field is bounded",()=>{
 expect(buildPolicy.hostCapacity.memory.elastic).toEqual({warmupMilliseconds:60000,convergeMilliseconds:120000,minimumObservations:3,marginMinimumMegabytes:256,deviationFactor:3,classKeepBasisPoints:1500});
 expect(()=>readHostCapacityPolicyV1({...buildPolicy.hostCapacity,memory:{...buildPolicy.hostCapacity.memory,elastic:{...buildPolicy.hostCapacity.memory.elastic,deviationFactor:99}}} as never)).toThrow();
});

test("the 17:30 incident counters block every request whatever the charges: swap 95 % used at 705 MB/s with pressure warn",()=>{
 const incident:Partial<HostSampleV1>={availableMegabytes:2980,swapUsedMegabytes:3060,swapTotalMegabytes:3220,swapOutMegabytesPerSecond:705,pressure:"warn",compressedMegabytes:8460} as never,live=fixture.cases[2] as Case;
 for(const megabytes of [512,1200,1900]){
  const evaluation=decide({...live,availableMegabytes:2980},megabytes,incident);
  expect(evaluation.decisions[0]!.verdict,`${megabytes} MB at 17:30`).toBe("wait");
  expect(["swap","swap-activity","pressure-warn","memory"]).toContain(evaluation.decisions[0]!.reason);
 }
});

test("a grant that grows back is charged what it holds at once, so nothing more is admitted into its room",()=>{
 const grown=fixture.cases[5] as Case,evaluation=decide(grown,1200);
 expect(evaluation.decisions[0]).toMatchObject({verdict:"wait"});
 expect(evaluation.headroomMegabytes).toBeLessThan(1200);
});

const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
test("the reader merges the sidecar of a grant into its row, and a row without a valid sidecar charges its whole reservation",()=>{
 const directory=mkdtempSync(join(tmpdir(),"hcel-"));roots.push(directory);mkdirSync(join(directory,"grants"),{recursive:true});
 const at=Date.now(),boot=hostBootAtV1(directory,at),row=(id:string)=>({id,operation:"command",jobs:1,cpu:1,reservedMegabytes:3600,admittedAtMs:at-600000,observedMegabytes:1557,pid:process.pid,bootAtMs:boot,retainReservation:true});
 for(const [id,stats] of [["withstats",'{"peakMegabytes":1600,"observations":26,"deviationMegabytes":120}'],["badstats_",'{"peakMegabytes":"x"}'],["nostats__",undefined]] as const){
  writeFileSync(join(directory,"grants",`${id}.format`),'{"format":1}');writeFileSync(join(directory,"grants",`${id}.json`),JSON.stringify(row(id)));
  if(stats!==undefined)writeFileSync(join(directory,"grants",`${id}.stats`),stats);
 }
 const rows=readGrants(directory,at,policy,undefined,undefined,false),byId=new Map(rows.map(grant=>[grant.id,grant]));
 expect(byId.get("withstats")).toMatchObject({peakMegabytes:1600,observations:26,deviationMegabytes:120});
 expect(byId.get("badstats_")?.peakMegabytes).toBeUndefined();expect(byId.get("nostats__")?.peakMegabytes).toBeUndefined();
 expect(elasticHostMegabytesV1(policy,byId.get("withstats")!,at)).toBe(1960);
 expect(elasticHostMegabytesV1(policy,byId.get("badstats_")!,at)).toBeUndefined();
 expect(Object.keys(JSON.parse(require("node:fs").readFileSync(join(directory,"grants","withstats.json"),"utf8")))).not.toContain("peakMegabytes");
});
