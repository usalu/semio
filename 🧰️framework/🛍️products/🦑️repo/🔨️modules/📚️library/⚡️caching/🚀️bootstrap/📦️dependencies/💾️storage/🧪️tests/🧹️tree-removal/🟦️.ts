import{expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

/** 🧹️ Contract: removing the Bun package cache is idempotent against a concurrent remover: an entry that vanishes between the listing and its removal is already removed, never an error (a second installer or purge of the same cache used to fail the whole stage with ENOENT), while a missing root stays refused. */
test("a sibling removed by someone else during the walk does not fail the removal",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"tree-removal-"));
 try{
  mkdirSync(join(root,"a"));mkdirSync(join(root,"z"));mkdirSync(join(root,"z","deep"),{recursive:true});writeFileSync(join(root,"z","deep","file"),"x");
  for(let index=0;index<300;index++)writeFileSync(join(root,"a","f"+index),"x");
  let ticks=0;
  const removed=await remove(root,async()=>{ticks++;rmSync(join(root,"z"),{recursive:true,force:true});},false);
  expect(ticks).toBeGreaterThan(0);
  expect(removed).toBeGreaterThan(299);
  expect(existsSync(root)).toBe(true);
  expect(existsSync(join(root,"a"))).toBe(false);
  expect(existsSync(join(root,"z"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a missing root is still refused",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts");
 await expect(remove(join(tmpdir(),"tree-removal-absent-"+process.pid),async()=>{},true)).rejects.toThrow();
});

test("a file a concurrent writer adds while a directory is emptied is removed by the next pass",async()=>{
 const{removeJavascriptDependencyTreeV1:remove}=await import("../../🟦️.ts"),root=mkdtempSync(join(tmpdir(),"tree-removal-"));
 try{
  mkdirSync(join(root,"a"));
  for(let index=0;index<300;index++)writeFileSync(join(root,"a","f"+index),"x");
  let written=false;
  await remove(root,async()=>{if(!written){written=true;mkdirSync(join(root,"a","late"));writeFileSync(join(root,"a","late","g"),"x");}},false);
  expect(written).toBe(true);
  expect(existsSync(join(root,"a"))).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
