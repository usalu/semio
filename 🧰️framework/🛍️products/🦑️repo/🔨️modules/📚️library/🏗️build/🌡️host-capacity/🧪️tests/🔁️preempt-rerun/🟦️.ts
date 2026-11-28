/** 🔁️ Contract: a build invocation that the gate preempted to break a memory hold-and-wait runs again at once (its finished units are cached), a bounded number of times; every other error and every success passes through untouched.
 * @see ../../🟦️.ts (`rerunPreemptedV1`, `withBuildCapacityV1`) */
import {expect,test} from "bun:test";
import {PREEMPTED_RERUNS_V1,rerunPreemptedV1} from "../../🟦️.ts";

const preempted=()=>Object.assign(Error("preempted"),{code:"capacity-preempted"});

test("a preempted attempt runs again until it completes and says so in the chosen language",async()=>{
 for(const [language,text] of [["en_US.UTF-8","running again after preemption (2/5)"],["de_DE.UTF-8","erneut gestartet (2/5)"]] as const){
  const lines:string[]=[],seen:number[]=[];
  const result=await rerunPreemptedV1(async number=>{seen.push(number);if(number<2)throw preempted();return "built";},{write:line=>lines.push(line),environment:{LANG:language}});
  expect(result).toBe("built");expect(seen).toEqual([0,1,2]);expect(lines).toHaveLength(2);expect(lines[1]).toContain(text);
 }
});

test("the reruns are bounded: the preemption is raised after the last one",async()=>{
 let attempts=0;
 await expect(rerunPreemptedV1(async()=>{attempts++;throw preempted();},{write:()=>undefined,environment:{LANG:"en_US.UTF-8"}})).rejects.toThrow("preempted");
 expect(attempts).toBe(PREEMPTED_RERUNS_V1+1);
 attempts=0;
 await expect(rerunPreemptedV1(async()=>{attempts++;throw preempted();},{write:()=>undefined,reruns:0})).rejects.toThrow("preempted");
 expect(attempts).toBe(1);
});

test("any other error and a success are never repeated",async()=>{
 let attempts=0;
 await expect(rerunPreemptedV1(async()=>{attempts++;throw Error("compile failed");},{write:()=>undefined})).rejects.toThrow("compile failed");
 expect(attempts).toBe(1);
 expect(await rerunPreemptedV1(async()=>{attempts++;return 7;},{write:()=>undefined})).toBe(7);
 expect(attempts).toBe(2);
});
