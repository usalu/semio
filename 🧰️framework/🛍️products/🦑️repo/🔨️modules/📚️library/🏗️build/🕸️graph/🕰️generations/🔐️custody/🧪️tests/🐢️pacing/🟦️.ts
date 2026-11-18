/** 🐢️ Contract: an attached reader never spends more than a fixed share of the wall time looking at the custody, however slow one look is on a busy host. */
import {expect,test} from "bun:test";
import schema from "../../🧬️schema/🔣️.json";
import {CUSTODY_POLICY_V1,attachToBuilderV1} from "../../🟨️.mjs";

const policy=CUSTODY_POLICY_V1 as typeof CUSTODY_POLICY_V1&{attachSpendRatio:number;attachMaximumPollMilliseconds:number};
const attach=async(looks:number[],used=policy)=>{
 let clock=0,at=0;const sleeps:number[]=[];
 await attachToBuilderV1({look:async()=>{clock+=looks[at++];return{held:at<looks.length,generation:undefined};},recheck:async()=>undefined,now:()=>clock,sleep:async(milliseconds:number)=>{sleeps.push(milliseconds);clock+=milliseconds;},policy:used});
 return sleeps;
};

test("the declared pacing is part of the schema document and bounded",()=>{
 expect(policy).toEqual(schema.definitions.Policy.const as never);
 expect(policy.attachSpendRatio).toBeGreaterThanOrEqual(1);
 expect(policy.attachMaximumPollMilliseconds).toBeGreaterThanOrEqual(policy.attachPollMilliseconds);
});

test("a quick look waits the plain poll interval, a slow look waits a multiple of its own duration, a very slow look waits the maximum",async()=>{
 expect(await attach([1,2,1,1])).toEqual([policy.attachPollMilliseconds,policy.attachPollMilliseconds,policy.attachPollMilliseconds]);
 expect(await attach([18000,18000,1])).toEqual([18000*policy.attachSpendRatio,18000*policy.attachSpendRatio]);
 expect(await attach([600000,1])).toEqual([policy.attachMaximumPollMilliseconds]);
});

test("whatever the duration of the looks, they take at most 1/(1+ratio) of the wall time unless the maximum applies",async()=>{
 let seed=20261115;const random=()=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 for(let round=0;round<200;round++){
  const took=Math.floor(random()*60000)+1,sleeps=await attach([took,1]),expected=Math.min(policy.attachMaximumPollMilliseconds,Math.max(policy.attachPollMilliseconds,policy.attachSpendRatio*took));
  expect(sleeps).toEqual([expected]);
  if(policy.attachSpendRatio*took>=policy.attachPollMilliseconds&&expected===policy.attachSpendRatio*took)expect(took/(took+expected)).toBeLessThanOrEqual(1/(1+policy.attachSpendRatio)+1e-9);
 }
});
