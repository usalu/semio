/** 🍎️ The service entry shim on macOS: a worktree below Documents, Desktop or Downloads is always run by the launcher at its install location (the one identity that holds the file access consent) with the worktree as root, even when the launcher ends quickly, and never by a cached runtime of another path (a runtime update would silently lose the consent); every other machine keeps the former order of runtimes. The shim is run for real with a fake `uname`, launcher and runtime. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {extname,join} from "node:path";

const sourceDirectory=join(import.meta.dirname,"..","..");

const shimDirectory=(name:string):{home:string;entry:string;worktree:string;launcherLog:string;bunLog:string;run:(launcherBody:string|null,milliseconds?:number,marker?:"good"|"stale"|"none")=>Promise<{code:number|null;launcherCalls:string[];bunCalls:string[]}>}=>{
 const home=mkdtempSync(join(tmpdir(),`shim-${name}-`)),entry=join(home,"entry"),worktree=join(home,"Documents","semio"),bin=join(home,".local","bin"),launcherPath=join(home,".local","share","semio","launcher","bin","repo"),cache=join(home,"Library","Caches","semio","runtimes","bun","aa");
 for(const directory of [entry,worktree,bin,join(home,".local","share","semio","launcher","bin"),cache])mkdirSync(directory,{recursive:true});
 const launcherLog=join(home,"launcher.log"),bunLog=join(home,"bun.log");
 writeFileSync(join(bin,"uname"),"#!/bin/sh\necho Darwin\n",{mode:0o755});
 writeFileSync(join(cache,"bun"),`#!/bin/sh\necho "$@" >> ${bunLog}\nexit 1\n`,{mode:0o755});
 writeFileSync(join(entry,"boot.mjs"),"// boot\n");writeFileSync(join(entry,"worktree"),worktree+"\n");
 copyFileSync(join(sourceDirectory,readdirSync(sourceDirectory).find(file=>extname(file)===".sh")!),join(entry,"boot.sh"));
 const run=async(launcherBody:string|null,milliseconds=3500,marker:"good"|"stale"|"none"="good")=>{
  rmSync(launcherLog,{force:true});rmSync(bunLog,{force:true});
  if(launcherBody===null)rmSync(launcherPath,{force:true});else writeFileSync(launcherPath,`#!/bin/sh\necho "root=$SEMIO_WORKSPACE_ROOT $@" >> ${launcherLog}\n${launcherBody}\n`,{mode:0o755});
  const access=join(home,"Library","Application Support","semio","access",createHash("sha256").update(readFileSync(join(entry,"worktree"),"utf8").trim()).digest("hex").slice(0,12));
  rmSync(access,{recursive:true,force:true});
  if(marker!=="none"&&launcherBody!==null){mkdirSync(access,{recursive:true});writeFileSync(join(access,"launcher.json"),JSON.stringify({version:1,sha256:marker==="good"?createHash("sha256").update(readFileSync(launcherPath)).digest("hex"):"0".repeat(64)}));}
  const proc=Bun.spawn(["sh",join(entry,"boot.sh"),"run"],{stdout:"pipe",stderr:"pipe",cwd:"/",env:{PATH:"/usr/bin:/bin",HOME:home}});
  await Bun.sleep(milliseconds);
  proc.kill("SIGTERM");const code=await Promise.race([proc.exited,Bun.sleep(8000).then(()=>null)]);
  const read=(file:string):string[]=>existsSync(file)?readFileSync(file,"utf8").split("\n").filter(Boolean):[];
  return{code,launcherCalls:read(launcherLog),bunCalls:read(bunLog)};
 };
 return{home,entry,worktree,launcherLog,bunLog,run};
};

test("on macOS a worktree below Documents is run by the launcher at its install location with the worktree as root once the marker proves that the launcher holds the consent, even when the launcher fails quickly, and never by a runtime of another path",async()=>{
 const box=shimDirectory("protected");
 try{
  const result=await box.run("exit 1");
  expect(result.launcherCalls.length).toBeGreaterThanOrEqual(2);
  for(const call of result.launcherCalls){expect(call).toContain(`root=${box.worktree}`);expect(call).toContain(join(box.entry,"boot.mjs"));}
  expect(result.bunCalls).toEqual([]);
  const clean=await box.run("exit 0",1500);
  expect(clean.code).toBe(0);
 }finally{rmSync(box.home,{recursive:true,force:true});}
});

test("without the marker, or with the marker of a launcher that was rebuilt to other bytes, the former order of runtimes stays: the launcher is not started, so no consent dialog can stop the replica",async()=>{
 const box=shimDirectory("unproved");
 try{
  for(const marker of ["none","stale"] as const){
   const result=await box.run("exit 1",3500,marker);
   expect(result.launcherCalls,marker).toEqual([]);expect(result.bunCalls.length,marker).toBeGreaterThanOrEqual(1);
  }
 }finally{rmSync(box.home,{recursive:true,force:true});}
});

test("a worktree outside the protected folders, or a machine without the launcher at its install location, keeps the former order of runtimes",async()=>{
 const box=shimDirectory("free");
 try{
  writeFileSync(join(box.entry,"worktree"),join(box.home,"semio")+"\n");mkdirSync(join(box.home,"semio"));
  const outside=await box.run("exit 1");
  expect(outside.bunCalls.length).toBeGreaterThanOrEqual(1);
  writeFileSync(join(box.entry,"worktree"),box.worktree+"\n");
  const missing=await box.run(null);
  expect(missing.launcherCalls).toEqual([]);expect(missing.bunCalls.length).toBeGreaterThanOrEqual(1);
 }finally{rmSync(box.home,{recursive:true,force:true});}
});
