/** 🧭️ Contract: a graph read served by the graph server takes the projects machines asked for in its first wave and tells which of them are still dirty; files are replaced atomically and a damaged file means no steering. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{clearSteeringV1,freshPriorityV1,orderOwnersV1,readPendingV1,readPriorityV1,steeringPathsV1,writePendingV1,writePriorityV1}from"../🟨️.mjs";

const made:string[]=[],temp=()=>{const path=mkdtempSync(join(tmpdir(),"steer-"));made.push(path);return path;};
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});
const environment=(directory:string)=>({SEMIO_GRAPH_STEER_DIRECTORY:directory});

test("without a named directory there is no steering and the order of a wave is the given one",()=>{
 expect(steeringPathsV1({})).toBeUndefined();expect(readPriorityV1({})).toBeUndefined();
 expect(orderOwnersV1(["a","b","c"],undefined)).toEqual(["a","b","c"]);
 expect(orderOwnersV1(["a","b","c"],{atMs:1,projects:[]})).toEqual(["a","b","c"]);
});

test("the priority projects come first in the order of the server, the others keep their order, unknown and repeated names are ignored; the wave of 48 takes them",()=>{
 const dirty=Array.from({length:200},(_,index)=>"p"+String(index).padStart(3,"0")),priority={atMs:1,projects:["p150","zzz","p007","p150","p199"]};
 const ordered=orderOwnersV1(dirty,priority);
 expect(ordered.slice(0,3)).toEqual(["p150","p007","p199"]);
 expect(ordered.slice(3)).toEqual(dirty.filter(name=>!["p150","p007","p199"].includes(name)));
 expect(new Set(ordered).size).toBe(200);
 const independent=[...dirty].sort((a,b)=>{const rank=(name:string)=>{const at=["p150","p007","p199"].indexOf(name);return at<0?3:at;};return rank(a)-rank(b)||dirty.indexOf(a)-dirty.indexOf(b);});
 expect(ordered).toEqual(independent);
 expect(orderOwnersV1(dirty,priority).slice(0,48)).toContain("p199");
});

test("priority and pending files round trip, validate against the schema and are replaced as a whole; clearing removes both",()=>{
 const directory=temp(),ajv=new Ajv({strict:true});
 writePriorityV1(directory,["x","y","x"],5);
 const priority=readPriorityV1(environment(directory))!;expect(priority).toEqual({atMs:5,projects:["x","y"]});
 expect(ajv.validate({...schema.definitions.Priority,definitions:{}},JSON.parse(readFileSync(join(directory,"priority.json"),"utf8"))),JSON.stringify(ajv.errors)).toBe(true);
 writePendingV1(environment(directory),44,["a","y","b"],priority);
 const pending=readPendingV1(directory)!;expect(pending).toMatchObject({generation:44,priorityAtMs:5,projects:["y"]});
 expect(ajv.validate({...schema.definitions.Pending,definitions:{}},pending),JSON.stringify(ajv.errors)).toBe(true);
 writePendingV1(environment(directory),45,["a","b"],priority);expect(readPendingV1(directory)!.projects).toEqual([]);
 clearSteeringV1(directory);expect(readPriorityV1(environment(directory))).toBeUndefined();expect(readPendingV1(directory)).toBeUndefined();
});

test("a damaged or foreign file means no steering and writing the pending file never throws",()=>{
 const directory=temp();
 writeFileSync(join(directory,"priority.json"),"{oops");expect(readPriorityV1(environment(directory))).toBeUndefined();
 writeFileSync(join(directory,"priority.json"),JSON.stringify({version:2,atMs:1,projects:[]}));expect(readPriorityV1(environment(directory))).toBeUndefined();
 writeFileSync(join(directory,"pending.json"),JSON.stringify({version:1,generation:1,atMs:1,priorityAtMs:1,projects:[1]}));expect(readPendingV1(directory)).toBeUndefined();
 expect(()=>writePendingV1(environment("/nonexistent/steer"),1,["a"],{atMs:1,projects:["a"]})).not.toThrow();
 expect(()=>writePendingV1({},1,["a"],{atMs:1,projects:["a"]})).not.toThrow();
});

test("the priority projects the plan does not list as dirty are the fresh ones to offer, once, and only against the current priority",()=>{
 const directory=temp(),env=environment(directory),offered=new Set<string>();
 writePriorityV1(directory,["a","b","c","d"],10);
 expect(freshPriorityV1(env,offered)).toEqual([]);
 writePendingV1(env,3,["b","z"],{atMs:9,projects:["b","z"]});
 expect(freshPriorityV1(env,offered),"a plan made against an older priority says nothing").toEqual([]);
 writePendingV1(env,3,["a","b","c","d","x"],readPriorityV1(env));
 expect(freshPriorityV1(env,offered)).toEqual([]);
 writePendingV1(env,4,["b"],readPriorityV1(env));
 expect(freshPriorityV1(env,offered)).toEqual(["a","c","d"]);
 offered.add("a");offered.add("c");
 expect(freshPriorityV1(env,offered)).toEqual(["d"]);
 expect(freshPriorityV1({},offered)).toEqual([]);
});
