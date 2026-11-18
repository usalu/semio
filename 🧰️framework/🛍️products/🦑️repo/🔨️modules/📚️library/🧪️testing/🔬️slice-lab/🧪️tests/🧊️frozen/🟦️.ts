import {describe,expect,test} from "bun:test";
import {chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,readlinkSync,realpathSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {SLICE_LAB_POLICY_V1 as P,readSliceLabOptionsV1,sliceLabReportV1,sliceLabVerdictV1,treeChurnV1,type SliceLabSampleV1} from "../../🟦️.ts";
import {editFrozenFileV1,freezeSliceLabRootV1,frozenRootV1,linkAmbientV1,relinkFrozenRootV1} from "../../🧊️frozen/🟦️.ts";

/** 🧊️ The frozen view of the slice lab: a clone of the files a read sees, outside the tree, with a git index of its own, in which a later edit of the live tree is invisible and the churn reads 0. */
const git=(cwd:string,...args:string[])=>spawnSync("git",["-c","core.fsmonitor=false","-c","core.quotepath=off",...args],{cwd,encoding:"utf8"});
const fixture=():string=>{
 const root=mkdtempSync(join(tmpdir(),"frz-live-"));
 git(root,"init","-q");
 mkdirSync(join(root,"🧰️framework","lib"),{recursive:true});mkdirSync(join(root,".🧬semio","🦑️repo","🎫️tickets","t1"),{recursive:true});mkdirSync(join(root,"deps-store","pkg"),{recursive:true});mkdirSync(join(root,"deps-store","@semio-tech"),{recursive:true});symlinkSync("deps-store",join(root,"node_modules"));symlinkSync(join(root,"🧰️framework","lib"),join(root,"deps-store","workspace-pkg"));mkdirSync(join(root,".cache","b","d","m","j"),{recursive:true});
 writeFileSync(join(root,".gitignore"),"node_modules\ndeps-store\nignored.txt\nout\n.cache\n");
 writeFileSync(join(root,"🧰️framework","lib","a.ts"),"export const a=1;\n");writeFileSync(join(root,"🧰️framework","lib","tool.sh"),"#!/bin/sh\n");chmodSync(join(root,"🧰️framework","lib","tool.sh"),0o755);
 symlinkSync("a.ts",join(root,"🧰️framework","lib","link.ts"));symlinkSync("/nonexistent-store/deps",join(root,"🧰️framework","lib","node_modules"));
 writeFileSync(join(root,".🧬semio","🦑️repo","🎫️tickets","t1","note.md"),"ticket\n");writeFileSync(join(root,"ignored.txt"),"i");
 git(root,"add",".gitignore","🧰️framework");git(root,"add","-f",join(".🧬semio","🦑️repo","🎫️tickets","t1","note.md"));
 writeFileSync(join(root,"🧰️framework","lib","untracked.ts"),"export const u=1;\n");
 return root;
};
const tree=(root:string)=>spawnSync("find",[".","-path","./.git","-prune","-o","-print"],{cwd:root,encoding:"utf8"}).stdout.split("\n").filter(Boolean).sort();
describe("frozen slice lab root",()=>{
 const live=fixture(),work=mkdtempSync(join(tmpdir(),"frz-work-")),frozen=join(work,"r");
 const result=freezeSliceLabRootV1(live,frozen);
 test("the frozen root holds the tracked and the untracked files the build sees, not the ignored, ambient or excluded ones",()=>{
  const paths=tree(frozen);
  expect(paths).toContain("./🧰️framework/lib/a.ts");expect(paths).toContain("./🧰️framework/lib/untracked.ts");expect(paths).toContain("./.gitignore");
  expect(paths.filter(path=>path.includes("node_modules")||path.includes(".cache"))).toEqual(["./.cache","./.cache/b","./.cache/b/d","./.cache/b/d/m","./.cache/b/d/m/j","./node_modules","./node_modules/pkg","./🧰️framework/lib/node_modules"]);expect(lstatSync(join(frozen,"node_modules")).isDirectory()).toBe(true);expect(readlinkSync(join(frozen,"node_modules","pkg"))).toBe(join(realpathSync(live),"deps-store","pkg"));expect(existsSync(join(frozen,"node_modules","@semio-tech"))).toBe(false);expect(existsSync(join(frozen,"node_modules","workspace-pkg"))).toBe(false);expect(paths.some(path=>path.includes("ignored.txt"))).toBe(false);expect(paths.some(path=>path.includes("🎫️tickets"))).toBe(false);
  expect(result.files).toBe(5);expect(result.tracked).toBe(4);expect(result.excluded).toBe(1);expect(result.ambient).toBe(3);expect(lstatSync(join(frozen,".cache")).isDirectory()).toBe(true);expect(readlinkSync(join(frozen,".cache","b","d","m","j"))).toBe(join(live,".cache","b","d","m","j"));
  expect(readlinkSync(join(frozen,"🧰️framework","lib","node_modules"))).toBe("/nonexistent-store/deps");expect(linkAmbientV1(live,frozen)).toBe(0);
  expect(P.frozen.exclude).toContain(".🧬semio/🦑️repo/🎫️tickets");
 });
 test("contents, executable bit and symbolic links are exactly those of the live tree (diff -r as the independent check)",()=>{
  expect(readFileSync(join(frozen,"🧰️framework","lib","a.ts"),"utf8")).toBe("export const a=1;\n");
  expect(lstatSync(join(frozen,"🧰️framework","lib","tool.sh")).mode&0o111).not.toBe(0);
  expect(readlinkSync(join(frozen,"🧰️framework","lib","link.ts"))).toBe("a.ts");
  const diff=spawnSync("diff",["-r","--no-dereference","-x",".git","-x","node_modules","-x","ignored.txt","-x","deps-store","-x","🎫️tickets",live,frozen],{encoding:"utf8"});
  expect(diff.stdout.split("\n").filter(line=>line.startsWith("Only in")&&!line.includes(".🧬semio")&&!line.includes("🦑️repo"))).toEqual([]);
 });
 test("git of the frozen root lists the same tracked and untracked files as git of the live tree (minus the excluded ones), and has a HEAD",()=>{
  const names=(root:string,...args:string[])=>git(root,"ls-files",...args).stdout.split("\n").filter(Boolean).sort();
  expect(names(frozen,"--cached")).toEqual(names(live,"--cached").filter(name=>!name.includes("🎫️tickets")));
  expect(names(frozen,"--others","--exclude-standard")).toEqual(names(live,"--others","--exclude-standard"));
  expect(git(frozen,"rev-parse","--verify","HEAD").status).toBe(0);
 });
 test("an edit of the live tree after the freeze is invisible in the frozen root, and the frozen churn reads 0 while the live churn sees the edit",()=>{
  const marker=join(work,"marker"),future=new Date(Date.now()+60000);writeFileSync(marker,"");utimesSync(marker,new Date(),new Date());
  for(const [name,text] of [["a.ts","export const a=2;\n"],["b.ts","new\n"]] as const){writeFileSync(join(live,"🧰️framework","lib",name),text);utimesSync(join(live,"🧰️framework","lib",name),future,future);}
  expect(readFileSync(join(frozen,"🧰️framework","lib","a.ts"),"utf8")).toBe("export const a=1;\n");
  expect(existsSync(join(frozen,"🧰️framework","lib","b.ts"))).toBe(false);
  expect(treeChurnV1(frozen,marker).count).toBe(0);
  expect(treeChurnV1(live,marker).count).toBe(2);
 });
 test("the options accept frozen and the verdict of a frozen run fails when the frozen view changed",()=>{
  const options=readSliceLabOptionsV1({machines:1,memoryMegabytes:16384,cores:8,slice:["a"],timeoutSeconds:60,frozen:true});
  expect(options.frozen).toBe(true);
  expect(()=>readSliceLabOptionsV1({...options,frozen:"yes"})).toThrow();
  const two={...options,machines:2,stagger:-1,edit:"a/b.ts"};
  expect(readSliceLabOptionsV1(two).edit).toBe("a/b.ts");
  const {frozen:_unfrozen,...plain}=two as typeof two&{frozen?:boolean};expect(()=>readSliceLabOptionsV1(plain)).toThrow();expect(()=>readSliceLabOptionsV1({...two,machines:1})).toThrow();expect(()=>readSliceLabOptionsV1({...two,stagger:0})).toThrow();
  expect(readSliceLabOptionsV1({...plain,frozenDirectory:"/tmp/x"}).frozenDirectory).toBe("/tmp/x");
  const sample:SliceLabSampleV1={atSeconds:2,machine:"m1",stage:"workspace",state:"running",phase:null,done:null,total:null,owners:{},processes:1,busy:0,footprintMegabytes:1,cpu:0,loopLagMs:null,activity:"-"};
  const input={options,machines:["m1"],samples:[sample],results:{m1:{ready:true,failure:null,timings:[]}},duplicateOwners:0,leftoverProcesses:0,leftoverImages:0,elapsedSeconds:3,load:1,timedOut:false,frozen:{files:5,megabytes:1,seconds:1,drift:3}};
  expect(sliceLabVerdictV1({...input,churn:{count:0,sample:[]}}).pass).toBe(true);
  const moved=sliceLabVerdictV1({...input,churn:{count:2,sample:["x"]}});
  expect(moved.pass).toBe(false);
  expect(sliceLabReportV1("en",options,moved,[])).toContain("frozen view");
  expect(sliceLabReportV1("de",options,moved,[])).toContain("eingefroren");
 });
 test("a named frozen root is built once and reused, and one edited file is written and taken back",()=>{
  const named=join(work,"named"),first=frozenRootV1(live,named),after=join(work,"after");writeFileSync(after,"");utimesSync(after,new Date(Date.now()+1000),new Date(Date.now()+1000));
  expect(first.reused).toBe(false);expect(existsSync(`${named}.frozen.json`)).toBe(true);
  writeFileSync(join(live,"🧰️framework","lib","late.ts"),"late\n");
  const second=frozenRootV1(live,named);
  expect(second.reused).toBe(true);expect(second.files).toBe(first.files);expect(existsSync(join(named,"🧰️framework","lib","late.ts"))).toBe(false);
  const file=join(named,"🧰️framework","lib","a.ts"),original=readFileSync(file,"utf8"),restore=editFrozenFileV1(named,join("🧰️framework","lib","a.ts"));
  expect(readFileSync(file,"utf8")).toBe(original+"\n");expect(readFileSync(join(frozen,"🧰️framework","lib","a.ts"),"utf8")).toBe("export const a=1;\n");
  restore();expect(readFileSync(file,"utf8")).toBe(original);
  expect(treeChurnV1(named,after,[join("🧰️framework","lib","a.ts")]).count).toBe(0);
 });
 test("the churn of a frozen view counts changed source files anywhere in it and leaves out ignored build outputs",()=>{
  const after=join(work,"whole");writeFileSync(after,"");utimesSync(after,new Date(),new Date());const future=new Date(Date.now()+60000);
  mkdirSync(join(frozen,"out"),{recursive:true});writeFileSync(join(frozen,"out","x.o"),"1");writeFileSync(join(frozen,"ignored.txt"),"2");writeFileSync(join(frozen,"top.ts"),"3");
  for(const name of ["out/x.o","ignored.txt","top.ts"])utimesSync(join(frozen,name),future,future);
  expect(treeChurnV1(frozen,after,[],true).sample).toEqual(["top.ts"]);
  expect(treeChurnV1(frozen,after,[],false).count).toBe(0);
 });
 test("cleanup",()=>{rmSync(live,{recursive:true,force:true});rmSync(work,{recursive:true,force:true});});
});

describe("frozen slice lab root after a real dependencies stage",()=>{
 test("the links the stage pointed into the lab's state are made again as the live tree has them, the root node_modules is a directory of entry links again",()=>{
  const live=fixture(),work=realpathSync(mkdtempSync(join(tmpdir(),"frz-rl-"))),frozen=join(work,"r"),gen=join(work,"s","j","g1");
  try{
   freezeSliceLabRootV1(live,frozen);
   mkdirSync(join(gen,"node_modules"),{recursive:true});
   const member=join(frozen,"🧰️framework","lib","node_modules");rmSync(member,{force:true});symlinkSync(join(gen,"node_modules"),member);
   rmSync(join(frozen,"node_modules"),{recursive:true,force:true});symlinkSync(join(gen,"node_modules"),join(frozen,"node_modules"));
   expect(relinkFrozenRootV1(live,frozen,work)).toBe(2);
   expect(readlinkSync(member)).toBe("/nonexistent-store/deps");
   expect(lstatSync(join(frozen,"node_modules")).isDirectory()).toBe(true);expect(readdirSync(join(frozen,"node_modules"))).toContain("pkg");
   expect(relinkFrozenRootV1(live,frozen,work)).toBe(0);
  }finally{rmSync(work,{recursive:true,force:true});}
 });
});
