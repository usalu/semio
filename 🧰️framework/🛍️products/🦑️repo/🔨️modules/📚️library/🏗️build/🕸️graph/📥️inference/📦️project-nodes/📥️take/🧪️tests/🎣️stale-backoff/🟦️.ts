/** 🎣️ Law: the adopt loop does not look again and again at an owner whose record from the fleet was taken and found stale for this tree: after a "stale" answer the owner waits `adoptStaleMilliseconds` plus the retry time before its next look, a plain miss only the retry time, and a fetched owner is never looked at again.
 * @see ../../🟨️.mjs `takePollV1` */
import{expect,test}from"bun:test";
import{TAKE_POLICY_V1,takePollV1}from"../../🟨️.mjs";

const POLICY={...TAKE_POLICY_V1,pollFirstMilliseconds:1000,hotRetryMilliseconds:5000,adoptStaleMilliseconds:60000};
const run=async(answer:string)=>{
 let clock=0,looks=0;const hot=new Set(["a"]),taken=new Set<string>();
 await takePollV1({hotOnly:true,hot:()=>hot,queue:()=>taken.has("a")?[]:["a"],attempt:async()=>{looks++;if(answer==="fetched")taken.add("a");return answer as never;},arrived:()=>{},stop:()=>clock>=200000,now:()=>clock,sleep:async()=>{clock+=1000;},policy:POLICY});
 return looks;
};

test("a stale owner is looked at once per 65 s, a missing owner once per 5 s, a fetched owner once",async()=>{
 expect(await run("stale")).toBeLessThanOrEqual(4);
 expect(await run("stale")).toBeGreaterThanOrEqual(3);
 expect(await run("miss")).toBeGreaterThan(30);
 expect(await run("fetched")).toBe(1);
});
