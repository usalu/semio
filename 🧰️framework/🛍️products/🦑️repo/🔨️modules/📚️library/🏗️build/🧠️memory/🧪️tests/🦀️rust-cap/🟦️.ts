/** 🦀️ Contract: the policy bounds the owners of one kind that run in the process at once (`kindLimits`, two for `language:rust`), the others of the machine run beside them, and a kind without a bound is not limited. Independent model: a counter around every task body.
 * @see ../../🟨️.mjs */
import{expect,test}from"bun:test";
import{createMemoryGovernorV1,GRAPH_MEMORY_POLICY_V1}from"../../🟨️.mjs";

const make=(workers:number)=>createMemoryGovernorV1({workers,headroom:()=>1e15,policy:{...GRAPH_MEMORY_POLICY_V1,admitMinimumMilliseconds:0,pollMilliseconds:1},sleep:()=>new Promise(done=>setTimeout(done,1))});

test("policy declares two concurrent rust owners",()=>{
 expect((GRAPH_MEMORY_POLICY_V1 as {kindLimits:Record<string,number>}).kindLimits["language:rust"]).toBe(2);
});

test("eight rust owners run two at a time while cargo owners run beside them",async()=>{
 const g=make(8),running={rust:0,cargo:0},peak={rust:0,cargo:0,all:0};
 const task=(kind:"rust"|"cargo")=>g.run(async()=>{running[kind]++;peak[kind]=Math.max(peak[kind],running[kind]);peak.all=Math.max(peak.all,running.rust+running.cargo);await Bun.sleep(30);running[kind]--;},{kind:"language:"+kind});
 await Promise.all([...Array.from({length:8},()=>task("rust")),...Array.from({length:4},()=>task("cargo"))]);
 expect(peak.rust).toBe(2);
 expect(peak.cargo).toBeGreaterThan(2);
 expect(peak.all).toBeGreaterThan(2);
 g.dispose();
});

test("a queue of only rust owners still finishes",async()=>{
 const g=make(8);let done=0;
 await Promise.all(Array.from({length:6},()=>g.run(async()=>{await Bun.sleep(5);done++;},{kind:"language:rust"})));
 expect(done).toBe(6);g.dispose();
});
