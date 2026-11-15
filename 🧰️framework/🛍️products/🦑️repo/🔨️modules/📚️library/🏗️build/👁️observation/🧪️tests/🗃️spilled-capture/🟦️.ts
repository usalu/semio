import{afterAll,expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

/** 🗃️ Contract: a plan-wide capture that spills its rows into a row log reports exactly the distinct rows the in-memory capture reports, counted and iterable, while keeping only log ids. */
const api=await import(new URL("../../🟨️.mjs",import.meta.url).href) as any;
const{createRowLogV1}=await import(new URL("../../../🕸️graph/🕰️generations/🗃️row-log/🟨️.mjs",import.meta.url).href) as any;
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const key=(row:{kind:string;path:string;fact:unknown})=>JSON.stringify([row.kind,row.path,row.fact]);

test("the spilled capture holds the same distinct rows as the in-memory capture",async()=>{
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});
 const root=mkdtempSync(join(realpathSync(parent),"spilled-capture-"));roots.push(root);
 const files=Array.from({length:40},(_,at)=>join(root,"f"+at+".txt"));files.forEach((file,at)=>writeFileSync(file,"content "+at));
 const read=()=>api.captureObservedSourceInputsV1(async()=>{for(const file of [...files,...files])await api.observedFs.readFile(file,"utf8");},undefined,false,undefined);
 const memory=await read();
 const log=createRowLogV1({path:join(root,"log","rl.rows")});
 const spilled=await api.captureObservedSourceInputsV1(async()=>{for(const file of [...files,...files])await api.observedFs.readFile(file,"utf8");},undefined,false,log);
 try{
  expect(spilled.observations.length).toBe(memory.observations.length);
  expect(memory.observations.length).toBeGreaterThanOrEqual(files.length);
  expect([...spilled.observations].map(key).sort()).toEqual([...memory.observations].map(key).sort());
  expect(log.stats().rows).toBe(memory.observations.length);
 }finally{log.close();}
});
