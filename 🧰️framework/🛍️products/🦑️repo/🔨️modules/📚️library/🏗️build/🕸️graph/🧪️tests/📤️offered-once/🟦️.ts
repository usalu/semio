/** 📤️ Contract: a wave of the read never offers again what a wave of the same read looked up itself, a restarted read does not offer what the earlier read offered, and the first wave offers at most one wave of names; the cost of the offer pass is a stage of the wave. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createOfferedStateV1,noteOfferedV1,offerFreshPriorityV1}from"../../🤝️prefetch/🟦️.ts";
import{writePendingV1,writePriorityV1,readPriorityV1}from"../../🕰️generations/🧭️steering/🟨️.mjs";

const made:string[]=[],temp=()=>{const path=mkdtempSync(join(tmpdir(),"offered-once-"));made.push(path);return path;};
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});
const stubsOf=(names:string[])=>Object.fromEntries(names.map(name=>[name,{root:"libs/"+name}]));

test("the first wave of a read offers at most one wave of the fresh priority projects, the next waves the rest, and the projects a wave looked up itself never",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},names=Array.from({length:130},(_,index)=>"p"+String(index).padStart(3,"0")),calls:string[][]=[],state=createOfferedStateV1();
 writePriorityV1(directory,names,5);writePendingV1(environment,1,[],readPriorityV1(environment));
 const run=()=>offerFreshPriorityV1({stubs:stubsOf(names),environment,state,offer:async roots=>{calls.push([...roots].sort());}});
 await noteOfferedV1(names.slice(0,40),environment,state);
 expect(await run()).toBe(48);expect(await run()).toBe(42);expect(await run()).toBe(0);
 expect(calls.flat().length).toBe(90);expect(new Set(calls.flat()).size).toBe(90);
 expect(calls.flat().some(root=>names.slice(0,40).map(name=>"libs/"+name).includes(root))).toBe(false);
});

test("a restarted read loads the names the earlier one offered and offers none of them",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},names=Array.from({length:60},(_,index)=>"q"+index),calls:string[][]=[];
 writePriorityV1(directory,names,5);writePendingV1(environment,1,[],readPriorityV1(environment));
 expect(await offerFreshPriorityV1({stubs:stubsOf(names),environment,state:createOfferedStateV1(),offer:async()=>{}})).toBe(48);
 expect(await offerFreshPriorityV1({stubs:stubsOf(names),environment,state:createOfferedStateV1(),offer:async roots=>{calls.push([...roots]);}})).toBe(12);
 expect(calls[0].length).toBe(12);
});

test("a pass that fails keeps its names for the next wave and notes nothing as offered",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},names=["r0","r1"];
 writePriorityV1(directory,names,5);writePendingV1(environment,1,[],readPriorityV1(environment));
 expect(await offerFreshPriorityV1({stubs:stubsOf(names),environment,state:createOfferedStateV1(),say:()=>{},offer:async()=>{throw new Error("busy");}})).toBe(0);
 expect(await offerFreshPriorityV1({stubs:stubsOf(names),environment,state:createOfferedStateV1(),offer:async()=>{}})).toBe(2);
});

test("the limit follows the size of the wave: a wave of 120 owners offers up to 120 fresh projects, never less than the policy wave",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},names=Array.from({length:300},(_,index)=>"w"+String(index).padStart(3,"0")),sizes:number[]=[];
 writePriorityV1(directory,names,5);writePendingV1(environment,1,[],readPriorityV1(environment));
 const run=(limit?:number)=>offerFreshPriorityV1({stubs:stubsOf(names),environment,state,limit,offer:async roots=>{sizes.push(roots.size);}}),state=createOfferedStateV1();
 expect(await run(120)).toBe(120);expect(await run(undefined)).toBe(48);expect(await run(500)).toBe(132);expect(sizes).toEqual([120,48,132]);
});
