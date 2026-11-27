/** 🫧️ Contract: the release runs only when the resident memory is above 1.5 times heap plus external, above the resident minimum, at most once per interval and never when switched off; one release is a full collection and a shrink and reports its pause.
 * @see ../🟨️.mjs */
import{expect,test}from"bun:test";
import{MEMORY_RELEASE_POLICY_V1,createMemoryReleaseV1,memoryReleaseDecisionV1,releaseResidentMemoryV1}from"../🟨️.mjs";

const MiB=1048576,policy=MEMORY_RELEASE_POLICY_V1,base={residentBytes:3000*MiB,heapBytes:800*MiB,externalBytes:500*MiB,nowMilliseconds:100000};

test("the decision needs the resident memory above the declared share of live memory, above the minimum, outside the interval and not switched off",()=>{
 expect(memoryReleaseDecisionV1(base)).toBe(true);
 expect(memoryReleaseDecisionV1({...base,residentBytes:1900*MiB})).toBe(false);
 expect(memoryReleaseDecisionV1({...base,residentBytes:1951*MiB})).toBe(true);
 expect(memoryReleaseDecisionV1({...base,residentBytes:(policy.minimumResidentMegabytes-1)*MiB,heapBytes:10*MiB,externalBytes:10*MiB})).toBe(false);
 expect(memoryReleaseDecisionV1({...base,lastAtMilliseconds:base.nowMilliseconds-policy.minimumIntervalMilliseconds+1})).toBe(false);
 expect(memoryReleaseDecisionV1({...base,lastAtMilliseconds:base.nowMilliseconds-policy.minimumIntervalMilliseconds})).toBe(true);
 expect(memoryReleaseDecisionV1({...base,environment:{[policy.disableEnvironment]:policy.disabledValue}})).toBe(false);
 expect(memoryReleaseDecisionV1({...base,heapBytes:Number.NaN})).toBe(false);
});

test("one release collects, shrinks and reports its pause",()=>{
 const calls:string[]=[];let clock=0;
 const done=releaseResidentMemoryV1({collect:()=>{calls.push("collect");clock+=40;},shrink:()=>{calls.push("shrink");clock+=10;},clock:()=>clock});
 expect(calls).toEqual(["collect","shrink"]);expect(done.pauseMilliseconds).toBe(50);
});

test("between owners the release runs once per interval while the process stays bloated, and not at all once the resident memory is back",()=>{
 let clock=0,rss=3000*MiB;const rows:unknown[]=[],ran:number[]=[];
 const release=createMemoryReleaseV1({usage:()=>({rss,heapUsed:800*MiB,external:500*MiB}),now:()=>clock,release:()=>{ran.push(clock);rss=1500*MiB;return{pauseMilliseconds:120};},report:row=>rows.push(row)});
 expect(release.between()).toEqual({residentMegabytes:3000,heapMegabytes:800,externalMegabytes:500,pauseMilliseconds:120});
 clock+=1000;rss=3000*MiB;expect(release.between()).toBeUndefined();
 clock=policy.minimumIntervalMilliseconds;expect(release.between()).toBeDefined();
 clock+=policy.minimumIntervalMilliseconds;expect(release.between()).toBeUndefined();
 expect(ran.length).toBe(2);expect(rows.length).toBe(2);expect(release.stats()).toEqual({releases:2,pausedMilliseconds:240});
});
