/** 🚑️ Contract: the readiness stage commands of a worker that is also a dev workstation run at interactive priority: they are not held behind a batch request at the head of the line and are lent the idle part of retained batch reservations (never more than their minimum), a batch request never spends the loan, and the whole run equals the independent Python oracle.
 * @see ./🐍️.py
 * @see ./🧫️fixtures/🔣️.json */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {evaluateHostCapacityV1,hostEnvironmentPriorityV1,hostStageAncestryV1,lendIdleReservationsV1,promoteStageWaitersV1,readHostCapacityPolicyV1,sizeHostRequestV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import base from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

type Row={verdict:"admit"|"wait";reserved:number;loan?:number;small?:string|null;spare?:number;head?:boolean};
const python=testPythonV1()??Bun.which("python3")??"python3",oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",oracle]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as {cases:Record<string,Row>;environments:("batch"|"interactive"|null)[]};})();
const policy=readHostCapacityPolicyV1({...base.policy,memory:{...base.policy.memory,...fixture.memory},wait:{...base.policy.wait,...fixture.wait}} as never),now=fixture.nowMs;
type Case=(typeof fixture.cases)[number];
const request=(id:string,priority:"batch"|"interactive",megabytes:number,minimum?:number)=>({id,operation:"command",priority,minimumJobs:1,preferredJobs:1,fixedMegabytes:megabytes,perJobMegabytes:0,cpuPerJob:1,...minimum===undefined?{}:{minimumMegabytes:minimum}});
const world=(item:Case)=>{
 const ledger=item.lenders.map(([reserved,observed,age],at)=>({id:`lender${at}xx`,operation:"command",jobs:1,cpu:1,reservedMegabytes:reserved,admittedAtMs:now-age,arrivedAtMs:now-age,observedMegabytes:observed,pid:1,bootAtMs:0,retainReservation:true,priority:"batch"}) as HostGrantV1);
 const queue:HostWaiterV1[]=[{request:request("headxxxx","batch",fixture.headMegabytes),arrivedAtMs:now-3600000,pid:1},{request:request("stagexxx",item.stage as "batch"|"interactive",fixture.stage.fixedMegabytes,fixture.stage.minimumMegabytes),arrivedAtMs:now,pid:1},...item.small?[{request:request("smallxxx","batch",fixture.smallBatchMegabytes),arrivedAtMs:now,pid:1}]:[]];
 const resident=item.lenders.reduce((sum,row)=>sum+row[1],0),host:HostSampleV1={version:1,atMs:now,platform:"darwin",cores:16,totalMegabytes:fixture.totalMegabytes,availableMegabytes:fixture.totalMegabytes-fixture.foreignMegabytes-resident,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0,openFiles:1000,openFilesMaximum:245760,attachedImages:0} as never;
 return {ledger,queue,host};
};
const decide=(item:Case)=>{const {ledger,queue,host}=world(item);return evaluateHostCapacityV1(policy,host,ledger,queue,now).decisions;};

for(const item of fixture.cases)test(`stage command: ${item.name} equals the oracle`,()=>{
 const rows=decide(item),stage=rows.find(row=>row.id==="stagexxx")!,want=expected.cases[item.name]!;
 expect(stage.verdict).toBe(want.verdict);
 expect(stage.verdict==="admit"?stage.reservedMegabytes:0).toBe(want.reserved);
 expect(rows.find(row=>row.id==="headxxxx")!.verdict).toBe("wait");
 if(want.small!==undefined&&want.small!==null)expect(rows.find(row=>row.id==="smallxxx")!.verdict).toBe(want.small as "wait");
});

test("no overcommit: what the stage is given never exceeds the room plus the loan, and the loan is never more than its minimum",()=>{
 const item=fixture.cases.find(row=>row.name==="interactive-borrows-idle-reservation")!,{ledger,queue}=world(item),loaned=lendIdleReservationsV1(policy,ledger,queue,now)!;
 const lent=ledger.reduce((sum,grant)=>sum+grant.reservedMegabytes,0)-loaned.reduce((sum,grant)=>sum+grant.reservedMegabytes,0);
 expect(lent).toBe(expected.cases[item.name]!.loan!);
 expect(lent).toBeLessThanOrEqual(fixture.stage.minimumMegabytes);
 for(const [at,grant] of loaned.entries())expect(grant.reservedMegabytes).toBeGreaterThanOrEqual(Math.max(512,Math.ceil(ledger[at]!.observedMegabytes!*1.5)));
});

test("nothing is lent while no interactive request waits",()=>{
 const item=fixture.cases.find(row=>row.name==="batch-behind-head")!,{ledger,queue}=world(item);
 expect(lendIdleReservationsV1(policy,ledger,queue,now)).toBeUndefined();
});

test("the environment names the class of the requests of a process, and sizing honours it",()=>{
 for(const row of fixture.environments)expect(hostEnvironmentPriorityV1(row.value===null?{}:{SEMIO_HOST_CAPACITY_PRIORITY:row.value})??null).toBe(row.class as "batch"|"interactive"|null);
 const sized=(environment?:Record<string,string>)=>sizeHostRequestV1(policy,{id:"sizedxxx",operation:"command",...environment===undefined?{}:{environment}})!;
 expect(sized().priority).toBe("batch");
 expect(sized({SEMIO_HOST_CAPACITY_PRIORITY:"interactive"}).priority).toBe("interactive");
 expect(sized({SEMIO_HOST_CAPACITY_PRIORITY:"urgent"}).priority).toBe("batch");
});

const tree=new Map<number,readonly string[]>([[100,["bun 🚦️ready/🏃️wrapper/🟦️.ts tools","bash setup","repo tools setup all"]],[200,["zsh","claude","repo capacity run --memory=8000"]]]);

test("a request whose process descends from a ready-supervisor stage is read as interactive without the environment, any other batch request is not, and the result equals the interactive case of the oracle",()=>{
 expect(hostStageAncestryV1(tree.get(100)!)).toBe(true);
 expect(hostStageAncestryV1(tree.get(200)!)).toBe(false);
 const item=fixture.cases.find(row=>row.name==="interactive-borrows-idle-reservation")!,{ledger,queue,host}=world({...item,stage:"batch",small:false}),pids=new Map<string,number>([["headxxxx",200],["stagexxx",100]]);
 const queued=queue.map(waiter=>({...waiter,pid:pids.get(waiter.request.id)!}));
 const promoted=promoteStageWaitersV1(queued,pid=>hostStageAncestryV1(tree.get(pid)!));
 expect(promoted.map(waiter=>[waiter.request.id,waiter.request.priority])).toEqual([["headxxxx","batch"],["stagexxx","interactive"]]);
 expect(queued[1]!.request.priority).toBe("batch");
 const stage=evaluateHostCapacityV1(policy,host,ledger,promoted,now).decisions.find(row=>row.id==="stagexxx")!,want=expected.cases[item.name]!;
 expect([stage.verdict,stage.reservedMegabytes]).toEqual([want.verdict,want.reserved]);
 expect(evaluateHostCapacityV1(policy,host,ledger,queued,now).decisions.find(row=>row.id==="stagexxx")!.verdict).toBe("wait");
});
