/** 🌿️ Contract: advancing a frontier lists only the directories whose stamp moved and answers exactly what a walk of the current tree answers, for seeded random edit sequences (files and directories created, removed, renamed, replaced, chmod-ed, excluded names, symbolic links) on a real directory; the independent Python walker agrees with both. */
import {expect,test} from "bun:test";
import {chmodSync,mkdirSync,mkdtempSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {lstat,readdir} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {advanceFrontierV1,walkFrontierV1} from "../🟨️.mjs";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle","🐍️.py"),windows=process.platform==="win32";
const ports=(root:string)=>({
 list:async(directory:string)=>{try{return(await readdir(join(root,directory),{withFileTypes:true})).map(entry=>({name:entry.name,kind:entry.isSymbolicLink()?"other" as const:entry.isDirectory()?"directory" as const:entry.isFile()?"file" as const:"other" as const}));}catch{return undefined;}},
 stat:async(path:string)=>{try{const status=await lstat(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
 descend:(name:string)=>name!=="node_modules"&&name!=="dist"&&!name.startsWith("ext-"),
 member:(path:string)=>{const name=path.slice(path.lastIndexOf("/")+1);return name==="project.json"||name==="Cargo.toml"||name.endsWith(".feature");},
});
const lcg=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
const shape=(walk:{directories:readonly (readonly [string,number,number])[];members:readonly (readonly [string,number])[]})=>({directories:walk.directories.map(row=>row[0]).sort(),members:[...walk.members].map(row=>[row[0],row[1]] as [string,number]).sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0)});
const model=(root:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,root]).stdout.toString());

test("a walk equals the independent Python walker on a seeded tree",async()=>{
 if(!python||windows)return;
 const root=mkdtempSync(join(realpathSync(tmpdir()),"frontier-walk-"));
 try{
  const random=lcg(3);
  for(let at=0;at<60;at++){const parts=Array.from({length:1+Math.floor(random()*4)},()=>["a","b","c","node_modules","dist","ext-x","d"][Math.floor(random()*7)]!);mkdirSync(join(root,...parts),{recursive:true});writeFileSync(join(root,...parts,["project.json","Cargo.toml","x.feature","other.txt"][Math.floor(random()*4)]!),"x");}
  symlinkSync(root,join(root,"a","loop"));
  const walk=await walkFrontierV1(ports(root)),expected=model(root);
  expect(shape(walk)).toEqual(expected);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("advancing after random edits lists only the moved directories and equals a walk and the Python walker",async()=>{
 if(windows)return;
 const root=mkdtempSync(join(realpathSync(tmpdir()),"frontier-walk-"));
 try{
  const random=lcg(11),names=["a","b","c","d","node_modules","dist","ext-x"],leafs=["project.json","Cargo.toml","x.feature","other.txt"];
  const pick=<T,>(list:T[])=>list[Math.floor(random()*list.length)]!;
  const dirsNow=async()=>{const out=["."];const visit=async(directory:string)=>{for(const entry of await readdir(join(root,directory),{withFileTypes:true}))if(entry.isDirectory()&&!entry.isSymbolicLink()){const path=directory==="."?entry.name:directory+"/"+entry.name;out.push(path);await visit(path);}};await visit(".");return out;};
  for(let at=0;at<40;at++){const parts=Array.from({length:1+Math.floor(random()*3)},()=>pick(names));mkdirSync(join(root,...parts),{recursive:true});writeFileSync(join(root,...parts,pick(leafs)),"x");}
  const port=ports(root);let walk=await walkFrontierV1(port),capturedAt=Date.now()+60000,listedTotal=0,steps=0;
  for(let step=0;step<120;step++){
   const dirs=await dirsNow(),here=pick(dirs),base=here==="."?root:join(root,here),kind=random();
   try{
    if(kind<0.25)writeFileSync(join(base,pick(leafs)),"y"+step);
    else if(kind<0.4)mkdirSync(join(base,pick(names),pick(names)),{recursive:true});
    else if(kind<0.5)writeFileSync(join(base,pick(names)+"-new",pick(leafs)),"z"),mkdirSync(join(base,pick(names)+"-new"),{recursive:true});
    else if(kind<0.65){const entries=(await readdir(base)).filter(name=>name!=="loop");if(entries.length)rmSync(join(base,pick(entries)),{recursive:true,force:true});}
    else if(kind<0.75){const entries=(await readdir(base)).filter(name=>name!=="loop");if(entries.length){const name=pick(entries);renameSync(join(base,name),join(base,name+"-moved"+step));}}
    else if(kind<0.82){const entries=(await readdir(base,{withFileTypes:true})).filter(entry=>entry.isFile());if(entries.length)chmodSync(join(base,pick(entries).name),random()<0.5?0o755:0o644);}
    else if(kind<0.9){const entries=(await readdir(base,{withFileTypes:true})).filter(entry=>entry.isFile());if(entries.length){const name=pick(entries).name;rmSync(join(base,name));mkdirSync(join(base,name),{recursive:true});writeFileSync(join(base,name,pick(leafs)),"q");}}
    else writeFileSync(join(base,"content-only.txt"),"w"+step);
   }catch{}
   const advanced=await advanceFrontierV1({...port,frontier:{capturedAt,directories:walk.directories,members:walk.members}});
   const fresh=await walkFrontierV1(port);
   expect(advanced,"step "+step).not.toBeUndefined();
   if(advanced&&!("unchanged"in advanced)){expect(shape(advanced),"step "+step).toEqual(shape(fresh));listedTotal+=advanced.listed;steps++;expect(advanced.listed).toBeLessThanOrEqual(Math.max(8,fresh.directories.length/2));}
   else expect(shape(walk),"step "+step).toEqual(shape(fresh));
   walk=fresh;
   if(python&&step%20===0)expect(shape(fresh)).toEqual(model(root));
  }
  expect(steps).toBeGreaterThan(20);
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("an untouched tree is unchanged after one stat pass and lists nothing",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"frontier-walk-"));
 try{
  for(const dir of["a/b","a/c","d"]){mkdirSync(join(root,dir),{recursive:true});writeFileSync(join(root,dir,"project.json"),"{}");}
  const port=ports(root),walk=await walkFrontierV1(port);let listed=0;
  const counted={...port,list:async(directory:string)=>{listed++;return port.list(directory);}};
  expect(await advanceFrontierV1({...counted,frontier:{capturedAt:Date.now()+60000,directories:walk.directories,members:walk.members}})).toEqual({unchanged:true});
  expect(listed).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a directory that vanished without its parent moving is not explained, so the caller walks again",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"frontier-walk-"));
 try{
  mkdirSync(join(root,"a/b"),{recursive:true});writeFileSync(join(root,"a/b/project.json"),"{}");
  const port=ports(root),walk=await walkFrontierV1(port),forged={...port,stat:async(path:string)=>path==="a/b"?undefined:port.stat(path)};
  expect(await advanceFrontierV1({...forged,frontier:{capturedAt:Date.now()+60000,directories:walk.directories,members:walk.members}})).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});
