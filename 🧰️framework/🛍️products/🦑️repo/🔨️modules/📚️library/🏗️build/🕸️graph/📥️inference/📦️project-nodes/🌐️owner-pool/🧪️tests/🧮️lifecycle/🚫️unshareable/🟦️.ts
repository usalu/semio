/** 🚫️ Contract: a machine whose records cannot be shared (a Windows checkout, 515 owners claimed and refused as `unshareable` on 2026-11-23) stops claiming after `unshareableLimit` refusals in a row, claims nothing for the pause, probes again afterwards (one more refusal pauses it again at once) and is never stopped by refusals that a published record interrupts. Oracle: the same walk written on the log of g1-m05 (3 refusals, a pause of 10 minutes). */
import{expect,test}from"bun:test";
import{OWNER_POOL_POLICY_V1 as P}from"../../../🟦️.ts";
import{ownerPoolSharelessStepV1}from"../../../🧮️lifecycle/🟦️.ts";

const run=(events:readonly("unshareable"|"published"|"tick")[],times:readonly number[]=[])=>{
 let state={count:0,until:0},tripped=0;
 events.forEach((event,at)=>{const step=ownerPoolSharelessStepV1(state,event,times[at]??at*1000);state=step.state;if(step.tripped)tripped++;});
 return{state,tripped};
};

test("the limit of refusals in a row starts one pause, further refusals do not extend it",()=>{
 const rows=run(["unshareable","unshareable"]);expect(rows.tripped).toBe(0);expect(rows.state.until).toBe(0);
 const third=run(["unshareable","unshareable","unshareable"]);expect(third.tripped).toBe(1);expect(third.state.until).toBe(2000+P.unshareablePauseMilliseconds);
 const many=run(Array.from({length:40},()=>"unshareable" as const));expect(many.tripped).toBe(1);expect(many.state.until).toBe(2000+P.unshareablePauseMilliseconds);
});

test("a published record in between resets the count",()=>{
 const rows=run(["unshareable","unshareable","published","unshareable","unshareable"]);
 expect(rows.tripped).toBe(0);expect(rows.state).toEqual({count:2,until:0});
});

test("after the pause one refusal pauses again at once and a published record frees the machine",()=>{
 const pause=P.unshareablePauseMilliseconds,rows=run(["unshareable","unshareable","unshareable","tick","tick","unshareable"],[0,1,2,3,pause+10,pause+11]);
 expect(rows.tripped).toBe(2);expect(rows.state.until).toBe(pause+11+pause);
 const freed=run(["unshareable","unshareable","unshareable","tick","published","unshareable","unshareable"],[0,1,2,pause+5,pause+6,pause+7,pause+8]);
 expect(freed.tripped).toBe(1);expect(freed.state).toEqual({count:2,until:0});
});

test("a tick before the end of the pause changes nothing",()=>{
 const rows=run(["unshareable","unshareable","unshareable","tick"],[0,1,2,P.unshareablePauseMilliseconds-1]);
 expect(rows.state.until).toBe(2+P.unshareablePauseMilliseconds);
});
