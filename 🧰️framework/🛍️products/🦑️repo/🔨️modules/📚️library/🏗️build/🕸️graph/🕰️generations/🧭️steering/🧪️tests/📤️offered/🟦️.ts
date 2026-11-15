/** 📤️ Contract: what the graph read has offered to the fleet cache is kept in one small file next to the steering files, survives the restart of a read and the clearing of the steering files, expires after its time to live, never exceeds its maximum, and a fresh priority list hands out at most one wave of names per call. */
import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import schema from"../../🧬️schema/🔣️.json"with{type:"json"};
import{clearSteeringV1,freshPriorityV1,readOfferedV1,writeOfferedV1,writePendingV1,writePriorityV1,readPriorityV1}from"../../🟨️.mjs";

const made:string[]=[],temp=()=>{const path=mkdtempSync(join(tmpdir(),"offered-"));made.push(path);return path;};
afterAll(()=>{for(const path of made)rmSync(path,{recursive:true,force:true});});
const environment=(directory:string)=>({SEMIO_GRAPH_STEER_DIRECTORY:directory}),rule=schema.definitions.Offered.const;

test("the offered names round trip through the file, validate against the schema and merge with the earlier ones",()=>{
 const directory=temp(),ajv=new Ajv({strict:true});
 expect(readOfferedV1(environment(directory),10).size).toBe(0);
 writeOfferedV1(environment(directory),["b","a","b"],10);writeOfferedV1(environment(directory),["c"],20);
 expect([...readOfferedV1(environment(directory),30)].sort()).toEqual(["a","b","c"]);
 const file=JSON.parse(readFileSync(join(directory,rule.file),"utf8"));
 expect(ajv.validate({...schema.definitions.OfferedFile,definitions:{}},file),JSON.stringify(ajv.errors)).toBe(true);
 expect(file.offered).toEqual({a:10,b:10,c:20});
});

test("without a steering directory nothing is read or written; a damaged file means nothing offered",()=>{
 expect(readOfferedV1({},0).size).toBe(0);expect(()=>writeOfferedV1({},["a"],0)).not.toThrow();
 const directory=temp();writeFileSync(join(directory,rule.file),"{not json");
 expect(readOfferedV1(environment(directory),0).size).toBe(0);
 writeFileSync(join(directory,rule.file),JSON.stringify({version:1,offered:{a:"yesterday"}}));
 expect(readOfferedV1(environment(directory),0).size).toBe(0);
});

test("a name older than the time to live is offered again, and the file never holds more than its maximum, the oldest dropped first",()=>{
 const directory=temp();
 writeOfferedV1(environment(directory),["old"],0);writeOfferedV1(environment(directory),["new"],rule.ttlMilliseconds-1);
 expect([...readOfferedV1(environment(directory),rule.ttlMilliseconds)].sort()).toEqual(["new"]);
 const many=Array.from({length:rule.maximumNames+5},(_,index)=>"n"+index);
 writeOfferedV1(environment(directory),many.slice(0,rule.maximumNames),1000);writeOfferedV1(environment(directory),many.slice(rule.maximumNames),2000);
 const kept=readOfferedV1(environment(directory),2001);
 expect(kept.size).toBe(rule.maximumNames);expect(many.slice(rule.maximumNames).every(name=>kept.has(name))).toBe(true);
});

test("clearing the steering files at the start of a read keeps the offered names",()=>{
 const directory=temp();
 writePriorityV1(directory,["x"],1);writeOfferedV1(environment(directory),["x"],5);
 clearSteeringV1(directory);
 expect(existsSync(join(directory,rule.file))).toBe(true);expect(readPriorityV1(environment(directory))).toBeUndefined();expect(readOfferedV1(environment(directory),6).has("x")).toBe(true);
});

test("a fresh priority list hands out at most the limit per call, never an offered name, and only names the caller knows",()=>{
 const directory=temp(),names=Array.from({length:200},(_,index)=>"p"+String(index).padStart(3,"0"));
 writePriorityV1(directory,names,5);writePendingV1(environment(directory),3,["p000","p001"],readPriorityV1(environment(directory)));
 const offered=new Set<string>(["p002"]);
 const first=freshPriorityV1(environment(directory),offered);
 expect(first.length).toBe(rule.maximumPerWave);expect(first).not.toContain("p000");expect(first).not.toContain("p002");expect(first[0]).toBe("p003");
 for(const name of first)offered.add(name);
 const second=freshPriorityV1(environment(directory),offered);
 expect(second.length).toBe(rule.maximumPerWave);expect(second.filter(name=>first.includes(name))).toEqual([]);
 const known=freshPriorityV1(environment(directory),new Set(),5,name=>name.endsWith("7"));
 expect(known).toEqual(names.filter(name=>name.endsWith("7")&&name!=="p000"&&name!=="p001").slice(0,5));
 expect(freshPriorityV1(environment(directory),new Set(),Infinity).length).toBe(198);
});
