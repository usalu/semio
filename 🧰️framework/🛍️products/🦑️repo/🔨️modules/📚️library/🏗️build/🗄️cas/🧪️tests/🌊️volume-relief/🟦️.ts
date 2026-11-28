/** 🧪️ Contract: the service tick of the cache host defends the free space of the volume before the hard reserve of the store is breached: below `volumeReliefReserveFactor` times the reserve it first asks the volume relief (the derivable classes no store accounts for) for the bytes up to that level, then judges and sweeps what is left; an unreachable or failing relief never stops the tick. */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {casEnforceTickV1,resetCasGuardsV1} from "../../🚦️guard/🟦️.ts";
import {casPartitionPolicyForV1} from "../../🌡️partition/🟦️.ts";
import {casReserveBytesV1} from "../../🌡️partition/🟦️.ts";
import {CAS_POLICY_V1} from "../../🧬️schema/🟦️.ts";
import {casUsageHintPathV1,readCasUsageHintV1,type CasCollectionV1} from "../../🧹️collector/🟦️.ts";

const roots:string[]=[],GIB=1024**3;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const hinted=():string=>{const root=mkdtempSync(join(tmpdir(),"cas-relief-"));roots.push(root);mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:0,atMs:Date.now()}));return root;};
const stub:CasCollectionV1={status:"collected",partial:false,dryRun:false,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:{totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,capShortfallBytes:0,partitionShortfallBytes:0,needBytes:0},byReason:{age:0,area:0,host:0}};
const volume={partition:"/volume",totalBytes:926*GIB,role:"cache-host" as const};
const reserve=casReserveBytesV1(CAS_POLICY_V1,volume);

test("the relief level is a declared factor above the hard reserve",()=>{
 expect(CAS_POLICY_V1.volumeReliefReserveFactor).toBeGreaterThan(1);
 expect(reserve).toBe(Math.floor(Math.max(0.08*volume.totalBytes,Math.min(77309411328,volume.totalBytes*CAS_POLICY_V1.sizing.reserveFloorMaxPercent/100))));
 expect(casPartitionPolicyForV1(CAS_POLICY_V1,{...volume,freeBytes:reserve}).partition.criticalUsedFraction).toBeGreaterThan(0);
});

test("below the relief level the tick asks the volume relief first, with the bytes up to that level, then re-reads the volume",async()=>{
 resetCasGuardsV1();
 const root=hinted(),calls:Array<{target:number;free:number}>=[],order:string[]=[];let free=reserve*1.2;
 await casEnforceTickV1({root,policy:CAS_POLICY_V1,partition:()=>({...volume,freeBytes:free}),usage:async()=>0,collect:async()=>{order.push("collect");return stub;},relieve:async request=>{order.push("relieve");calls.push({target:request.targetFreeBytes,free:request.freeBytes});free=reserve*3;}});
 expect(calls).toEqual([{target:Math.ceil(reserve*CAS_POLICY_V1.volumeReliefReserveFactor),free:reserve*1.2}]);
 expect(order[0]).toBe("relieve");
});

test("above the relief level the volume relief is not asked",async()=>{
 resetCasGuardsV1();
 let asked=0;
 await casEnforceTickV1({root:hinted(),policy:CAS_POLICY_V1,partition:()=>({...volume,freeBytes:reserve*(CAS_POLICY_V1.volumeReliefReserveFactor+1)}),usage:async()=>0,collect:async()=>stub,relieve:async()=>{asked++;}});
 expect(asked).toBe(0);
});

test("a failing relief never stops the tick, a cancelled one does, and a tick without a relief port is unchanged",async()=>{
 resetCasGuardsV1();
 let measured=0;
 const ports={root:hinted(),policy:CAS_POLICY_V1,partition:()=>{measured++;return{...volume,freeBytes:reserve*0.5};},usage:async()=>0,collect:async()=>stub};
 await casEnforceTickV1({...ports,relieve:async()=>{throw Error("boom");}});
 expect(measured).toBeGreaterThanOrEqual(2);
 resetCasGuardsV1();
 const abort=new AbortController();abort.abort();
 await expect(casEnforceTickV1({...ports,signal:abort.signal,relieve:async request=>{request.signal?.throwIfAborted();}})).rejects.toBeDefined();
 resetCasGuardsV1();
 await casEnforceTickV1(ports);
});

test("a stale hint is replaced by a measurement before the relief and the collection run, and the measurement is recorded for every reader",async()=>{
 resetCasGuardsV1();
 const root=hinted();writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:1,atMs:Date.now()-3600000}));
 const order:string[]=[];
 await casEnforceTickV1({root,policy:CAS_POLICY_V1,partition:()=>({...volume,freeBytes:reserve*1.2}),usage:async()=>{order.push("measure");return 7*GIB;},collect:async()=>{order.push("collect");return stub;},relieve:async()=>{order.push("relieve");}});
 expect(order.slice(0,2)).toEqual(["measure","relieve"]);
 const hint=await readCasUsageHintV1(root);
 expect(hint?.bytes).toBe(7*GIB);
 expect(hint?.atMs).toBeGreaterThan(Date.now()-60000);
});
