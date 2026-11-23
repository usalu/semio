/** ⏱️ Contract: the wait of a read for the fleet to infer heavy owners is bound by the arrival of these owners, not by the news of the fleet: machines that start to deliver other records, or producers that keep their status fresh, never restart the stall clock, so one owner that nobody infers ends the wait after the declared stall time, and the read runs it here once (`🔁️retry-alone`) instead of waiting for hours.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{heavyProgressSignV1}from"../../🟨️.mjs";
import{awaitWantedV1,wantedPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),wait=source.slice(source.indexOf("const waitHeavy=async()=>{"),source.indexOf("const largestOf="));

test("only the arrived owners among the wanted count as progress",()=>{
 const taken=new Set(["a","x"]);
 expect(heavyProgressSignV1(["a","b"],taken)).toBe(1);
 expect(heavyProgressSignV1(["b","c"],taken)).toBe(0);
 taken.add("b");
 expect(heavyProgressSignV1(["a","b"],taken)).toBe(2);
 expect(heavyProgressSignV1([],taken)).toBe(0);
});

test("the wait passes the arrival sign, never the machine sign of producers, as its progress",()=>{
 expect(wait).toContain("progressSign:()=>heavyProgressSignV1(names,taken)");
 expect(wait).not.toContain("progressSign:sign");
});

test("new machines and fresh producer status keep the wait alive but do not extend the stall: it ends `stalled` after the declared stall time",async()=>{
 let at=0,machines=0;const causes:string[]=[],taken=new Set<string>();
 const left=await awaitWantedV1({projects:["heavy"],take:async()=>false,now:()=>at,sleep:async milliseconds=>{at+=milliseconds;if(at%600000===0)machines++;},
  alive:()=>machines*1000+at,progressSign:()=>heavyProgressSignV1(["heavy"],taken),end:cause=>{causes.push(cause);}});
 expect(left).toEqual(["heavy"]);
 expect(causes).toEqual(["stalled"]);
 expect(at).toBeGreaterThanOrEqual(wantedPolicyV1.waitMilliseconds);
 expect(at).toBeLessThan(wantedPolicyV1.waitMilliseconds+2*wantedPolicyV1.pollMilliseconds);
 expect(machines).toBeGreaterThan(1);
});

test("the old wiring with the machine sign never ends before the safety net, which is the bug this contract closes",async()=>{
 let at=0,machines=0;const causes:string[]=[];
 await awaitWantedV1({projects:["heavy"],take:async()=>false,now:()=>at,sleep:async milliseconds=>{at+=milliseconds;if(at%600000===0)machines++;},
  alive:()=>machines*1000+at,progressSign:()=>machines,end:cause=>{causes.push(cause);}});
 expect(causes).toEqual(["limit"]);
 expect(at).toBeGreaterThanOrEqual(wantedPolicyV1.waitMaximumMilliseconds);
});

test("a fleet that delivers none of the heavy owners within the declared heavy bound is left at once: the wait is wired with that bound and ends `stalled` after it, so the read runs the owners here",async()=>{
 expect(wait).toContain("policy:{...wantedModule.wantedPolicyV1,waitMilliseconds:wantedModule.wantedPolicyV1.heavyWaitMilliseconds}");
 expect(wantedPolicyV1.heavyWaitMilliseconds).toBeLessThanOrEqual(300000);
 expect(wantedPolicyV1.heavyWaitMilliseconds).toBeLessThan(wantedPolicyV1.waitMilliseconds);
 let at=0;const causes:string[]=[];
 const left=await awaitWantedV1({projects:["heavy","other"],take:async()=>false,now:()=>at,sleep:async milliseconds=>{at+=milliseconds;},alive:()=>at,progressSign:()=>0,end:cause=>{causes.push(cause);},policy:{...wantedPolicyV1,waitMilliseconds:wantedPolicyV1.heavyWaitMilliseconds}});
 expect(left).toEqual(["heavy","other"]);expect(causes).toEqual(["stalled"]);
 expect(at).toBeGreaterThanOrEqual(wantedPolicyV1.heavyWaitMilliseconds);expect(at).toBeLessThan(wantedPolicyV1.heavyWaitMilliseconds+2*wantedPolicyV1.pollMilliseconds);
});

test("an arrival restarts the heavy bound: a fleet that delivers keeps being waited for, only silence ends the wait",async()=>{
 let at=0;const causes:string[]=[],taken=new Set<string>(),names=["a","b"];
 const left=await awaitWantedV1({projects:names,take:async name=>taken.has(name),now:()=>at,sleep:async milliseconds=>{at+=milliseconds;if(at===wantedPolicyV1.heavyWaitMilliseconds-wantedPolicyV1.pollMilliseconds)taken.add("a");},alive:()=>at,progressSign:()=>heavyProgressSignV1(names,taken),end:cause=>{causes.push(cause);},policy:{...wantedPolicyV1,waitMilliseconds:wantedPolicyV1.heavyWaitMilliseconds}});
 expect(left).toEqual(["b"]);expect(causes).toEqual(["stalled"]);
 expect(at).toBeGreaterThan(wantedPolicyV1.heavyWaitMilliseconds);expect(at).toBeLessThan(2*wantedPolicyV1.heavyWaitMilliseconds+2*wantedPolicyV1.pollMilliseconds);
});
