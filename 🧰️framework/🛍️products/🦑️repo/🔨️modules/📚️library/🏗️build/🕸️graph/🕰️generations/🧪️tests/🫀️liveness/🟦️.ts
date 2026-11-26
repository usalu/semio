/** 🫀️ Contract: every stage of a whole read reports a counter or a stage change, to the stall watch of the sidecar (the phase rows) and to the claim of the build (the door board), so a read that works is never called stalled; the shared closures a first wave primes count one by one. */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{advanceProgressV1,idleMillisecondsV1}from"../../🪧️door-board/🟨️.mjs";
import{custodyProgressV1}from"../../🔌️load/🟨️.mjs";
import{createWorld,ringFiles,seed}from"../🏃️world/🟦️.ts";

const STAGES=["facts","fragments","patching","packing","publishing"];

test("custody progress advances on every stage change and every counted closure, and not on a repeat",async()=>{
 const forwarded:{phase:string;done:number;total:number}[]=[];
 const emitted:unknown[]=[];
 const forward=custodyProgressV1({progress:async(event:never)=>{forwarded.push(event);}},async event=>{emitted.push(event);});
 const plain=custodyProgressV1(true,async event=>{emitted.push(event);});
 await plain({phase:"facts",done:1,total:2});
 expect(forwarded).toEqual([]);
 const events=[{phase:"facts",done:5,total:9},{phase:"verification",done:1,total:3},{phase:"decisions",done:10,total:10},...Array.from({length:49},(_u,at)=>({phase:"prime",done:at+1,total:49})),{phase:"patching",done:1,total:1},{phase:"packing",done:1,total:1},{phase:"publishing",done:1,total:1}];
 for(const event of events)await forward(event);
 await forward({phase:"decisions",done:3,total:10,running:[]} as never);
 expect(forwarded.length).toBe(events.length);
 let state=undefined as ReturnType<typeof advanceProgressV1>|undefined,at=0;
 for(const event of forwarded){const next=advanceProgressV1(state,event,at+=60_000);expect(state===undefined||next.advancedMs===at,JSON.stringify(event)).toBe(true);state=next;}
 const repeated=advanceProgressV1(state,forwarded.at(-1)!,at+600_000);
 expect(idleMillisecondsV1({progress:repeated},at+600_000)).toBe(600_000);
});

test("a real build reports its sealing stages after the facts and fragments, in order",async()=>{
 const dir=mkdtempSync(join(realpathSync(tmpdir()),"graph-liveness-"));
 try{
  await seed(dir,ringFiles(12));
  const world=createWorld(dir),phases:string[]=[];
  await world.refresh({progress:(event:unknown)=>{const phase=(event as{phase?:string}).phase;if(phase&&phases.at(-1)!==phase)phases.push(phase);}} as never);
  const seen=phases.filter(phase=>STAGES.includes(phase));
  expect(seen.filter((phase,at)=>seen.indexOf(phase)===at)).toEqual(STAGES.filter(stage=>seen.includes(stage)));
  for(const stage of["fragments","patching","packing","publishing"])expect(phases,"stage "+stage).toContain(stage);
 }finally{rmSync(dir,{recursive:true,force:true});}
},60_000);
