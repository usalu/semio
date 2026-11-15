/** 🚪️ Contract: a task that hands its owner slot back while it waits for a worker process does not count against the admission of the process, so the parent holds at most the bound of what it infers itself and the workers are fed by every project in flight. */
import{expect,test}from"bun:test";
import{createMemoryGovernorV1}from"../../🟨️.mjs";

test("tasks that wait away from their slot run beyond the bound, tasks that work in it never do",async()=>{
 const{GRAPH_MEMORY_POLICY_V1}=await import("../../🟨️.mjs"),make=()=>createMemoryGovernorV1({workers:2,headroom:()=>1e15,policy:{...GRAPH_MEMORY_POLICY_V1,admitMinimumMilliseconds:0,pollMilliseconds:1},sleep:()=>new Promise(resolve=>setTimeout(resolve,1))});
 for(const away of[false,true]){
  const g=make();let running=0,peak=0,awayPeak=0,waiting=0;
  await Promise.all(Array.from({length:8},()=>g.run(async handle=>{
   running++;peak=Math.max(peak,running);
   if(away)await handle.away(async()=>{running--;waiting++;awayPeak=Math.max(awayPeak,waiting);await Bun.sleep(40);waiting--;running++;});else await Bun.sleep(40);
   running--;
  },{kind:"language:cargo"})));
  if(away)expect(awayPeak,"six or more projects wait for workers at once").toBeGreaterThan(2);else expect(peak).toBeLessThanOrEqual(2);
  g.dispose();
 }
});
