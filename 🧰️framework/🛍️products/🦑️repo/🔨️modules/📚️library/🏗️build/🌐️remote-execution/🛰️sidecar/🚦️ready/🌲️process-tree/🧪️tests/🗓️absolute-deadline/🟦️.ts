import {expect,test} from "bun:test";
import {READY_TREE_POLICY_V1 as P,readyAbsoluteDeadlineV1} from "../../🟦️.ts";

/** 🗓️ Contract: the absolute deadline is the start plus the soft cap times (the larger of the hard-cap multiplier of the stage and the progress ceiling, plus the grace interval). */
test("absolute deadline per stage",()=>{
 expect(readyAbsoluteDeadlineV1({stage:"tools",startedMs:1000,softMs:3600000})).toBe(1000+3600000*25);
 expect(readyAbsoluteDeadlineV1({stage:"workspace",startedMs:0,softMs:100})).toBe(2500);
 expect(readyAbsoluteDeadlineV1({stage:"x",startedMs:0,softMs:10,policy:{...P,hardCapMultiplier:2,progressCeilingMultiplier:2,absoluteDeadlineExtraIntervals:0}})).toBe(20);
});
