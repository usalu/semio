/** 🥇️ Contract: the first worker of a farm is admitted whatever the free memory says; the floor applies to the additional workers. A farm without a child makes its parent infer every owner itself (the largest process doing the slowest work), so the gate keeps one child admitted: only a critical pressure, a sibling that already runs and a request that is not essential keep the old rules.
 * @see ../../📏️decision/🟦️.ts */
import {expect,test} from "bun:test";
import {decideHostCapacityV1,evaluateHostCapacityV1,readHostCapacityPolicyV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import lendFixture from "../🤲️lend/🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,memory:{...decisionFixture.policy.memory,maximumQuiet:lendFixture.policy.maximumQuiet}});
const world=lendFixture.worlds.find(row=>row.name.startsWith("16 GB machine, the parent holds 5500"))!;
const host=(change:Partial<HostSampleV1>={}):HostSampleV1=>({...world.host,...change}) as HostSampleV1;
const waiter=(request:Record<string,unknown>={}):HostWaiterV1=>({...world.queue[0]!,request:{...world.queue[0]!.request,...request}}) as HostWaiterV1;
const ledger=world.ledger as HostGrantV1[];
const sibling={id:"child000001",operation:"exec",jobs:1,cpu:1,reservedMegabytes:1536,admittedAtMs:world.nowMs-1000,observedMegabytes:900,pid:4300,bootAtMs:1700000000000,label:"graph-farm",holder:"showholder1"} as HostGrantV1;
const verdict=(waiters:HostWaiterV1[],grants:HostGrantV1[]=ledger,sample:HostSampleV1=host())=>evaluateHostCapacityV1(policy,sample,grants,waiters,world.nowMs).decisions.find(row=>row.id==="farmreq0001")!;

test("the baseline: an elastic farm request that is not essential waits for memory when the parent holds its reservation",()=>{
 expect(verdict([waiter()])).toMatchObject({verdict:"wait",reason:"memory"});
});

test("the first worker of a farm is admitted with its minimum jobs although the memory above the floor is short, beyond the reservation of the parent",()=>{
 const row=verdict([waiter({essential:true})]);
 expect(row.verdict).toBe("admit");expect(row.jobs).toBe(1);expect(row.reservedMegabytes).toBe(1536);
 const over=verdict([waiter({essential:true})],ledger,host({availableMegabytes:4096-233}));
 expect(over.verdict,"-233 MB spare above the floor").toBe("admit");
});

test("the floor applies to the additional workers: a sibling of the same label, holder and session that already runs makes the next request an ordinary one",()=>{
 expect(verdict([waiter({essential:true})],[...ledger,sibling]).verdict).toBe("wait");
 expect(verdict([waiter({essential:true,holder:"otherholder1"})],[...ledger,sibling]).verdict,"another holder has no worker yet").toBe("admit");
});

test("a critical pressure and a request that is not elastic keep the old rules",()=>{
 expect(verdict([waiter({essential:true})],ledger,host({pressure:"critical"})).verdict).toBe("wait");
 expect(verdict([waiter({essential:true,elastic:false})]).verdict).toBe("wait");
});

test("the admission is accounted: what the first worker reserves is charged, so a later request of someone else sees the room it took",()=>{
 const other:HostWaiterV1={request:{id:"otherreq001",operation:"exec",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:0,perJobMegabytes:1536,cpuPerJob:1},arrivedAtMs:world.nowMs-10,pid:99} as HostWaiterV1;
 const rows=evaluateHostCapacityV1(policy,host(),ledger,[waiter({essential:true}),other],world.nowMs).decisions;
 expect(rows.find(row=>row.id==="farmreq0001")!.verdict).toBe("admit");
 expect(rows.find(row=>row.id==="otherreq001")!.verdict).toBe("wait");
 expect(decideHostCapacityV1(policy,host(),ledger,[waiter({essential:true})],world.nowMs).decisions[0]!.reason).toBeDefined();
});

test("when the memory carries more than one worker the essential request gets them through the ordinary path, the rule only catches what the floor would refuse",()=>{
 const roomy=verdict([waiter({essential:true})],[],host({availableMegabytes:4096+20000}));
 expect(roomy.verdict).toBe("admit");expect(roomy.jobs,"the preferred jobs fit: no reduction to the minimum").toBeGreaterThan(1);
 expect(roomy.reason).not.toBe("override");
});
