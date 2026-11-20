/** 🗑️ Contract: a `selected/<scope>.admission.json` without any publication of its scope is garbage once no writer holds it; it is retired only under the per-scope writer lease, only while the publication stays absent, and only when its leaf stamp is unchanged right before the unlink. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync,existsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{retireOrphanSelectedAdmissionsV1}from"../../🟦️.ts";

const hex=(digit:string)=>digit.repeat(64),control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>{},yieldContinuation:async()=>{}} as never;
const fixture=(files:Record<string,string>)=>{
 const root=mkdtempSync(join(tmpdir(),"orphan-admission-")),directory=join(root,"selected");mkdirSync(directory);
 for(const[name,text]of Object.entries(files))writeFileSync(join(directory,name),text);
 const image={root,recheck:async()=>{}} as never;
 return{root,directory,image,names:Object.keys(files),done:()=>rmSync(root,{recursive:true,force:true})};
};
const free=async<T>(_scope:string,body:()=>Promise<T>)=>body(),never=()=>false;

test("an orphan admission is retired and reports its bytes",async()=>{
 const x=fixture({[hex("a")+".admission.json"]:"x".repeat(5000)});
 try{const result=await retireOrphanSelectedAdmissionsV1(x.image,x.directory,x.names,control,free,never);expect(result.files).toBe(1);expect(result.bytes).toBeGreaterThan(0);expect(existsSync(join(x.directory,hex("a")+".admission.json"))).toBe(false);}finally{x.done();}
});

test("an admission with any publication of its scope is kept, and enough headroom stops the retirement",async()=>{
 const x=fixture({[hex("a")+".admission.json"]:"a",[hex("a")+".json"]:"{}",[hex("b")+".admission.json"]:"b",[hex("b")+".authority.json"]:"{}",[hex("c")+".admission.json"]:"c"});
 try{
  const stopped=await retireOrphanSelectedAdmissionsV1(x.image,x.directory,x.names,control,free,()=>true);expect(stopped.files).toBe(0);
  const result=await retireOrphanSelectedAdmissionsV1(x.image,x.directory,x.names,control,free,never);expect(result.files).toBe(1);
  expect(existsSync(join(x.directory,hex("a")+".admission.json"))).toBe(true);expect(existsSync(join(x.directory,hex("b")+".admission.json"))).toBe(true);expect(existsSync(join(x.directory,hex("c")+".admission.json"))).toBe(false);
 }finally{x.done();}
});

test("an admission whose scope a live writer holds is kept, a publication appearing under the lease keeps it too",async()=>{
 const x=fixture({[hex("a")+".admission.json"]:"a",[hex("b")+".admission.json"]:"b"});
 try{
  const held=async<T>(scope:string,body:()=>Promise<T>)=>{if(scope===hex("a"))return undefined;if(scope===hex("b"))writeFileSync(join(x.directory,hex("b")+".json"),"{}");return body();};
  const result=await retireOrphanSelectedAdmissionsV1(x.image,x.directory,x.names,control,held,never);
  expect(result.files).toBe(0);expect(existsSync(join(x.directory,hex("a")+".admission.json"))).toBe(true);expect(existsSync(join(x.directory,hex("b")+".admission.json"))).toBe(true);
 }finally{x.done();}
});

test("a leaf whose stamp changed after the observation and before the unlink is refused and kept",async()=>{
 const x=fixture({[hex("a")+".admission.json"]:"a"}),path=join(x.directory,hex("a")+".admission.json");
 try{
  const raced=async<T>(_scope:string,body:()=>Promise<T>)=>{writeFileSync(path,"rewritten by a writer");return body();};
  await expect(retireOrphanSelectedAdmissionsV1(x.image,x.directory,x.names,control,raced,never)).rejects.toThrow("Original orphan selected admission changed");
  expect(existsSync(path)).toBe(true);
 }finally{x.done();}
});
