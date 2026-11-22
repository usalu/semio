import {expect,test} from "bun:test";
import {awaitPlannerQuietV1,plannerPathsV1} from "../🟦️.ts";

const TREE="/w/planner";
const policy={milliseconds:60,maximumMilliseconds:500};

/** 🔕️ Contract: only edits of the planner's own source delay a repeated owner round, and only until that source has been quiet. */
test("planner paths are those inside the planner tree, never a sibling that shares its prefix",()=>{
 expect(plannerPathsV1(["/w/planner/a.ts","/w/planner","/w/planner-x/b.ts","/w/other/c.ts","relative/d.ts","/w/planner/../other/e.ts"],TREE)).toEqual(["/w/planner/a.ts","/w/planner"]);
});

test("a round kept alive by tenant files alone never waits",async()=>{
 let looked=0;
 const waited=await awaitPlannerQuietV1({paths:["/w/tenant/a.ts"],tree:TREE,policy,stamps:()=>{looked++;return new Map();}});
 expect(waited).toBe(0);expect(looked).toBe(0);
});

test("a planner file that was quiet long enough lets the round repeat at once",async()=>{
 const clock=1_000_000;
 const waited=await awaitPlannerQuietV1({paths:["/w/planner/a.ts","/w/tenant/b.ts"],tree:TREE,policy,now:()=>clock,stamps:paths=>new Map([...paths].map(path=>[path,clock-policy.milliseconds]))});
 expect(waited).toBe(0);
});

test("a burst of edits is waited out once and the wait names the newest file",async()=>{
 let clock=1_000_000;const edits=new Map([["/w/planner/a.ts",clock-10],["/w/planner/b.ts",clock-30]]),said:(string|undefined)[]=[];
 const waited=await awaitPlannerQuietV1({paths:[...edits.keys()],tree:TREE,policy,heartbeatMilliseconds:5,now:()=>{clock+=20;return clock;},stamps:()=>edits,announce:newest=>said.push(newest)});
 expect(waited).toBeGreaterThan(0);expect(waited).toBeLessThanOrEqual(policy.maximumMilliseconds);
 expect(said.length).toBeGreaterThan(0);expect(new Set(said)).toEqual(new Set(["/w/planner/a.ts"]));
});

test("an endless burst is waited for at most the maximum, and an abort ends the wait early",async()=>{
 let clock=1_000_000;const waited=await awaitPlannerQuietV1({paths:["/w/planner/a.ts"],tree:TREE,policy:{milliseconds:60,maximumMilliseconds:100},heartbeatMilliseconds:5,now:()=>{clock+=40;return clock;},stamps:()=>new Map([["/w/planner/a.ts",clock]])});
 expect(waited).toBeGreaterThanOrEqual(100);
 const controller=new AbortController();controller.abort();
 expect(await awaitPlannerQuietV1({paths:["/w/planner/a.ts"],tree:TREE,policy,signal:controller.signal,now:()=>0,stamps:()=>new Map([["/w/planner/a.ts",0]])})).toBe(0);
});
