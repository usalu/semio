/** 📤️ Contract: the records of asked-for projects that are fresh in the server's tree reach the fleet cache (one pass over exactly those projects), once, also when only some of the asked-for projects are fresh; a failing pass is told and never fails the read. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{offerFreshPriorityV1}from"../../🤝️prefetch/🟦️.ts";
import{writePendingV1,writePriorityV1,readPriorityV1}from"../../🕰️generations/🧭️steering/🟨️.mjs";

const made:string[]=[],temp=()=>{const path=mkdtempSync(join(tmpdir(),"offer-fresh-"));made.push(path);return path;};
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});

test("arrived but partial: of 8 asked-for projects 5 are fresh and are offered by root, the 3 dirty ones are left to the inference, nothing is offered twice",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},names=Array.from({length:8},(_,index)=>"p"+index);
 const stubs=Object.fromEntries(names.map(name=>[name,{root:"libs/"+name}])),calls:string[][]=[];
 writePriorityV1(directory,names,5);
 const offer=async(roots:ReadonlySet<string>)=>{calls.push([...roots].sort());};
 expect(await offerFreshPriorityV1({stubs,offer,environment})).toBe(0);
 writePendingV1(environment,7,["p1","p4","p6"],readPriorityV1(environment));
 expect(await offerFreshPriorityV1({stubs,offer,environment})).toBe(5);
 expect(calls).toEqual([["libs/p0","libs/p2","libs/p3","libs/p5","libs/p7"]]);
 expect(await offerFreshPriorityV1({stubs,offer,environment})).toBe(0);
 writePendingV1(environment,8,["p6"],readPriorityV1(environment));
 expect(await offerFreshPriorityV1({stubs,offer,environment})).toBe(2);
 expect(calls.at(-1)).toEqual(["libs/p1","libs/p4"]);
});

test("a pass that fails is told and answers nothing offered, so the next round tries again",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},lines:string[]=[];
 writePriorityV1(directory,["q0"],5);writePendingV1(environment,1,[],readPriorityV1(environment));
 let fail=true;
 const run=()=>offerFreshPriorityV1({stubs:{q0:{root:"libs/q0"}},environment,say:line=>lines.push(line),offer:async()=>{if(fail)throw new Error("store busy");}});
 expect(await run()).toBe(0);expect(lines[0]).toContain("store busy");
 fail=false;expect(await run()).toBe(1);
});
