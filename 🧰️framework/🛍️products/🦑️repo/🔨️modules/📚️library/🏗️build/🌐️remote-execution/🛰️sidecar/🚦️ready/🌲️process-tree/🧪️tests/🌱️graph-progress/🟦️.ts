import {expect,test} from "bun:test";
import {READY_TREE_POLICY_V1 as P,readyGraphCounterTrackerV1,readyReadingLiveV1,readyStallCountV1,readyStalledV1,readyDeadlineV1} from "../../🟦️.ts";

/** 🌱️ Contract: a graph read whose server counter advances is progress even when its normalised text repeats; a live read in the reading phase is not stalled and runs to the hard cap; the restart budget counts only stalls that followed no progress. */
const record=(over:object={})=>({reason:"request" as const,runner:7,read:1,startedMs:1,status:null,endedMs:null,...over});

test("a counter that advances is progress, one that repeats or falls back is not",()=>{
 const tracker=readyGraphCounterTrackerV1();
 expect(tracker.observe(["Reading the project graph slice (10 s): taken 461 of 600."])).toBe(true);
 expect(tracker.observe(["Reading the project graph slice (900 s): taken 461 of 600."])).toBe(false);
 expect(tracker.observe(["Lese den Projektgraph-Ausschnitt (905 s): taken 462 von 600."])).toBe(true);
 expect(tracker.observe(["taken 300 of 600"])).toBe(false);
 expect(tracker.observe(["no counter here"])).toBe(false);
 const again=readyGraphCounterTrackerV1(P,tracker.snapshot());
 expect(again.observe(["taken 462 of 600"])).toBe(false);expect(again.observe(["taken 463 of 600"])).toBe(true);
});

test("a live read in the reading phase is exempt from the stall stop and bounded by the hard cap",()=>{
 const alive=(pid:number)=>pid===7;
 expect(readyReadingLiveV1({phase:"reading",record:record(),alive})).toBe(true);
 expect(readyReadingLiveV1({phase:"executing",record:record(),alive})).toBe(false);
 expect(readyReadingLiveV1({phase:"reading",record:record({endedMs:5}),alive})).toBe(false);
 expect(readyReadingLiveV1({phase:"reading",record:record({runner:8}),alive})).toBe(false);
 expect(readyReadingLiveV1({phase:"reading",record:undefined,alive})).toBe(false);
 expect(readyStalledV1(P.stallStopMilliseconds,P,true)).toBe(false);
 expect(readyStalledV1(P.stallStopMilliseconds,P,false)).toBe(true);
 expect(readyDeadlineV1({stage:"workspace",elapsedMs:3600000*7,softMs:3600000,sinceProgressMs:0,policy:P})).toBe("extended");
 expect(readyDeadlineV1({stage:"workspace",elapsedMs:3600000*9,softMs:3600000,sinceProgressMs:0,policy:P})).toBe("timeout");
});

test("the restart budget counts only stalls without progress",()=>{
 expect(readyStallCountV1(0,false)).toBe(1);
 expect(readyStallCountV1(3,false)).toBe(4);
 expect(readyStallCountV1(4,true)).toBe(1);
});
