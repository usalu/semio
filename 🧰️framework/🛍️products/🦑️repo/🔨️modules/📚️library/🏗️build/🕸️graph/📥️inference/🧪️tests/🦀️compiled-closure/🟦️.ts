/** 🦀️ Contract: the Rust source closure of a crate compiled without `cfg(test)` is exactly what `rustc --emit=dep-info` reads, the conservative closure covers what `rustc --test` reads, and a cfg(test) mount, item or `include!` never reaches the compiled closure. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,libraryBootstrap}=inference as any;
await libraryBootstrap;
const slash=(path:string)=>path.split("\\").join("/");
const closure=(root:string,compiled:boolean)=>cacheInternals.rustSourceFiles([join(root,corpus.entry)],root,undefined,new Map(),compiled).map((path:string)=>slash(relative(root,path))).sort();
const rustc=(root:string,flags:string[])=>{
 const out=join(root,"oracle.d"),run=spawnSync("rustc",["--edition=2021","--emit=dep-info","-o",out,...flags,corpus.entry],{cwd:root,encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 return [...new Set(readFileSync(out,"utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/u).map(path=>slash(relative(root,resolve(root,path)))))].sort();
};
const build=(root:string)=>{for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

test("the fixture is closed by its schema",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test("a crate compiled without cfg(test) reads exactly what rustc reads, and no cfg(test) mount or include",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-compiled-"));
 try{
  build(root);
  const compiled=closure(root,true);
  expect(compiled).toEqual([...corpus.compiled].sort());
  expect(rustc(root,["--crate-type=lib"])).toEqual(compiled);
  for(const path of corpus.testOnly)expect(compiled,path).not.toContain(path);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the conservative closure keeps every mount and include and covers what rustc --test reads",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-everything-"));
 try{
  build(root);
  const everything=closure(root,false);
  expect(everything).toEqual([...corpus.everything].sort());
  for(const path of rustc(root,["--test"]))expect(everything,`rustc --test reads ${path}`).toContain(path);
  for(const path of closure(root,true))expect(everything,path).toContain(path);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a compiled closure and a conservative closure of the same crate do not share one memoized answer",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-memo-"));
 try{
  build(root);
  const snapshots=new Map(),entries=[join(root,corpus.entry)],first=cacheInternals.rustSourceFiles(entries,root,undefined,snapshots,true),second=cacheInternals.rustSourceFiles(entries,root,undefined,snapshots,false),third=cacheInternals.rustSourceFiles(entries,root,undefined,snapshots,true);
  expect(first.length).toBeLessThan(second.length);
  expect(third).toEqual(first);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("an inner cfg(test) attribute of an inline module never marks the whole file: the file's other mounts stay, the module's own mounts are kept conservatively and rustc reads a subset",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"rust-inner-"));
 try{
  for(const[path,text]of Object.entries(corpus.innerAttribute.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  const entry=corpus.innerAttribute.entry,compiled=cacheInternals.rustSourceFiles([join(root,entry)],root,undefined,new Map(),true).map((path:string)=>slash(relative(root,path))).sort();
  expect(compiled).toEqual([...corpus.innerAttribute.compiled].sort());
  const out=join(root,"oracle.d"),run=spawnSync("rustc",["--edition=2021","--crate-type=lib","--emit=dep-info","-o",out,entry],{cwd:root,encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);
  for(const path of readFileSync(out,"utf8").split("\n")[0]!.split(": ")[1]!.trim().split(/\s+/u))expect(compiled,`rustc reads ${path}`).toContain(slash(relative(root,resolve(root,path))));
 }finally{rmSync(root,{recursive:true,force:true});}
});
