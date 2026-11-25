/** 🎫️ Contract: an inventory name a session found present is answered later by one stat of its leaf while the leaf keeps its stamp; a leaf replaced, removed or linked twice, or one whose admitted ancestry changed, is forgotten and must be asked in a session again; the memory is bounded, oldest first, and counts each newly remembered name once. The stamp equals the one CPython's `os.lstat` gives for the same file.
 * @see ../../📥️reuse/🎫️verified/🟨️.mjs */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{linkSync,lstatSync,mkdtempSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createVerifiedPresenceV1,inventoryLeafStampV1}from"../../📥️reuse/🎫️verified/🟨️.mjs";

const name=(at:number)=>"SEMIO_SOURCE_CLOSURE_"+at.toString(16).padStart(64,"0");
const stampOf=(path:string)=>inventoryLeafStampV1(lstatSync(path,{bigint:true}));

test("the stamp of a leaf equals CPython's lstat of the same file",()=>{
 const directory=mkdtempSync(join(tmpdir(),"verified-presence-"));
 try{
  const path=join(directory,"a.json");writeFileSync(path,"{}");
  const run=spawnSync("python3",["-I","-c","import os,sys;s=os.lstat(sys.argv[1]);print(':'.join(str(v) for v in (s.st_dev,s.st_ino,s.st_mode,s.st_nlink,s.st_size,s.st_mtime_ns,s.st_ctime_ns)))",path],{encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);expect(stampOf(path)).toBe(run.stdout.trim());
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a remembered leaf holds while unchanged; a replaced, removed or doubly linked leaf is forgotten",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"verified-presence-"));
 try{
  const memory=createVerifiedPresenceV1({maximum:16}),paths=[0,1,2,3].map(at=>join(directory,at+".json"));
  for(const[at,path]of paths.entries()){writeFileSync(path,"{\"at\":"+at+"}");memory.remember(name(at),path,stampOf(path));}
  expect(paths.map((_,at)=>memory.holds(name(at)))).toEqual([true,true,true,true]);
  await Bun.sleep(5);
  writeFileSync(paths[0]+".next","{\"at\":0}");renameSync(paths[0]+".next",paths[0]!);
  rmSync(paths[1]!);
  linkSync(paths[2]!,paths[2]+".twin");
  expect(paths.map((_,at)=>memory.holds(name(at)))).toEqual([false,false,false,true]);
  expect(memory.has(name(0))||memory.has(name(1))||memory.has(name(2))).toBe(false);
  expect(memory.holds(name(9))).toBe(false);
  expect(memory.stats()).toEqual({names:1,hits:5,misses:3,remembered:4});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the memory keeps at most its maximum, the oldest leaves leave first, a name seen again counts once",()=>{
 const directory=mkdtempSync(join(tmpdir(),"verified-presence-"));
 try{
  const memory=createVerifiedPresenceV1({maximum:3}),path=join(directory,"leaf.json");writeFileSync(path,"{}");const stamp=stampOf(path);
  for(let at=0;at<5;at++)memory.remember(name(at),path,stamp);
  memory.remember(name(4),path,stamp);
  expect([0,1,2,3,4].map(at=>memory.has(name(at)))).toEqual([false,false,true,true,true]);
  expect(memory.stats().remembered).toBe(5);expect(memory.stats().names).toBe(3);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a leaf whose admitted ancestry changed is forgotten although its own stamp holds",()=>{
 const directory=mkdtempSync(join(tmpdir(),"verified-presence-"));
 try{
  const memory=createVerifiedPresenceV1({maximum:4}),path=join(directory,"leaf.json"),ancestry={rows:[{path:directory}]};writeFileSync(path,"{}");
  memory.remember(name(1),path,stampOf(path),ancestry);
  const seen:unknown[]=[];
  expect(memory.holds(name(1),row=>{seen.push(row);})).toBe(true);expect(seen).toEqual([ancestry]);
  expect(memory.holds(name(1),()=>{throw Object.assign(Error("Inference inventory ancestry changed"),{code:"BUILD_STORE_CHANGED"});})).toBe(false);
  expect(memory.has(name(1))).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
