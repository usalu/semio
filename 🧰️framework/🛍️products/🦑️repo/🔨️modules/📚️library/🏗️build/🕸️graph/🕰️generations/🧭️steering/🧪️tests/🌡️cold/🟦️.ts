/** 🌡️ Contract: a read in the cold state (no generation to publish before all owners are inferred) tells the server which of the asked-for projects are done while it goes on, so the requesters get their fresh projects as each owner completes and not at the end of the generation; the file is rate limited, trailing, bounded by the priority and never lies about a project that is not done. */
import Ajv from"ajv";
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync,readFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{createColdPendingV1,freshPriorityV1,readPendingV1,readPriorityV1,writePriorityV1}from"../../🟨️.mjs";

const made:string[]=[],temp=()=>{const path=mkdtempSync(join(tmpdir(),"cold-"));made.push(path);return path;};
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});
const sleep=(milliseconds:number)=>new Promise<void>(done=>{setTimeout(done,milliseconds);});

test("the first file lists every priority project as dirty; each finished owner leaves the list; an owner nobody asked for changes nothing; the file answers the current priority",()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory};writePriorityV1(directory,["a","b","c","d"],5000);
 let clock=0;const cold=createColdPendingV1(environment,1000,()=>clock);
 cold.begin();expect(readPendingV1(directory)).toMatchObject({generation:-1,priorityAtMs:5000,projects:["a","b","c","d"]});
 clock=5000;cold.done("b");expect(readPendingV1(directory)?.projects,"written at once: the interval has passed").toEqual(["a","c","d"]);
 cold.done("zzz");cold.flush();expect(readPendingV1(directory)?.projects).toEqual(["a","c","d"]);
 for(const name of["a","c","d"])cold.done(name);cold.flush();expect(readPendingV1(directory)?.projects).toEqual([]);
 const pending=readPendingV1(directory)!,priority=readPriorityV1(environment)!;
 expect(pending.priorityAtMs,"the server accepts it for its priority").toBe(priority.atMs);
 expect(freshPriorityV1(environment,new Set(),48,()=>true),"all four are fresh now and can be offered to the fleet").toEqual(["a","b","c","d"]);
});

test("writes are rate limited with a trailing write, so 700 finished owners cost a handful of files, and the last owner is always written",async()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},names=Array.from({length:700},(_,index)=>"p"+index);writePriorityV1(directory,names,1);
 const cold=createColdPendingV1(environment,60);cold.begin();
 let writes=0,last="";const watcher=setInterval(()=>{const text=readFileSync(join(directory,"pending.json"),"utf8");if(text!==last){last=text;writes++;}},5);
 for(const name of names){cold.done(name);if(Number(name.slice(1))%100===99)await sleep(30);}
 await sleep(200);clearInterval(watcher);
 expect(readPendingV1(directory)?.projects,"the trailing write carried the last owner").toEqual([]);
 expect(writes,"far fewer files than owners").toBeLessThan(60);
});

test("without a priority nothing is written (a read nobody serves has nobody to tell); the file has the shape of the schema",()=>{
 const directory=temp(),environment={SEMIO_GRAPH_STEER_DIRECTORY:directory},cold=createColdPendingV1(environment,0);
 cold.begin();cold.done("a");cold.flush();expect(readPendingV1(directory)).toBeUndefined();
 writePriorityV1(directory,["a","b"],7);cold.done("a");cold.flush();
 const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Pending",definitions:schema.definitions});
 expect(validate(JSON.parse(readFileSync(join(directory,"pending.json"),"utf8"))),JSON.stringify(validate.errors)).toBe(true);
 expect(createColdPendingV1({},0).flush(),"no directory named: no steering, no throw").toBeUndefined();
});

test("the part discovery of a cold read creates the tracker once, inside the capture, announces the first file and notes every owner that is done; a warm part creates none",()=>{
 const source=readFileSync(new URL("../../../../🧭️discovery/🟦️.ts",import.meta.url),"utf8"),part=source.slice(source.indexOf("export async function discoverBuildGraphPartV1"));
 expect(part.match(/createColdPendingV1\(\)/gu)?.length).toBe(1);
 expect(part).toContain("const coldPending=part.cold?(await import(");
 expect(part).toContain("coldPending?.begin();");
 expect(part.match(/coldPending\?\.done\(name\)/gu)?.length).toBe(1);
 expect(part.indexOf("stages.mark(\"inference\");")).toBeLessThan(part.indexOf("const coldPending="));
 expect(part.indexOf("const coldPending=")).toBeLessThan(part.indexOf("onOwnerObservations:"));
});
