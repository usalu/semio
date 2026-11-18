import{expect,test}from"bun:test";
import{OWNER_POOL_POLICY_V1 as P,pullOwnerPoolV1}from"../../🟦️.ts";
import type{ProjectClaimV1}from"../../../../../../🪪️graph-state/📦️project-records/🟦️.ts";

const fast={...P,sweepPollMilliseconds:5,heldRetryMilliseconds:60,faultRetryMilliseconds:60};
type Unit={name:string;megabytes?:number};
const granted=(publishes:boolean[],log:string[]):ProjectClaimV1=>{let last="";return{state:"granted",digest:"d",why:()=>last,code:()=>last,deterministic:()=>false,async publish(){const ok=publishes.shift()??false;last=ok?"":"no-record";log.push(ok?"published":"no-record");return ok;},async abandon(){log.push("abandon");}} as unknown as ProjectClaimV1;};

/** 🔁️ An owner whose record was not exported because its inputs changed while it ran is inferred once more at once, with a new claim; a second miss ends as a failure. */
test("an owner with no record to export is retried once immediately and published, a second miss is a failure",async()=>{
 const runs:string[]=[],claims:string[]=[],log:string[]=[];
 const stats=await pullOwnerPoolV1<Unit>({units:()=>[{name:"a"}],puller:"r101:1",policy:fast,slots:()=>1,claim:async()=>{claims.push("claim");return granted([false],log);},run:async unit=>{runs.push(unit.name);return true;}});
 expect(runs.length).toBe(2);expect(stats.failed).toBe(1);expect(stats.published).toBe(0);expect(claims.length).toBe(2);
expect(log.filter(row=>row==="no-record").length).toBe(2);
});

test("a retried owner is published when the second inference stores its record",async()=>{
 const runs:string[]=[],log:string[]=[];let asked=0;
 const stats=await pullOwnerPoolV1<Unit>({units:()=>[{name:"a"}],puller:"r101:2",policy:fast,slots:()=>1,claim:async()=>{asked++;return granted(asked===1?[false]:[true],log);},run:async unit=>{runs.push(unit.name);return true;}});
 expect(runs.length).toBe(2);expect(stats.published).toBe(1);expect(stats.failed).toBe(0);expect(log).toEqual(["no-record","published"]);
});

test("a failure that is not a missing record is not retried",async()=>{
 const runs:string[]=[];let asked=0;
 const stats=await pullOwnerPoolV1<Unit>({units:()=>[{name:"a"}],puller:"r101:3",policy:fast,slots:()=>1,claim:async()=>{asked++;return{state:"granted",digest:"d",why:()=>"refused",code:()=>"claim-lost",deterministic:()=>false,publish:async()=>false,abandon:async()=>undefined} as unknown as ProjectClaimV1;},run:async unit=>{runs.push(unit.name);return true;}});
 expect(runs.length).toBe(1);expect(stats.failed).toBe(1);expect(asked).toBe(1);
});
