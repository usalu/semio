/** 🪦️ Contract: a cached build script stand-in whose sealed module was retired runs the real executable directly instead of failing with `Module not found` (m03, serde_core, 11/27). */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {shimSourceV1} from "../../🏗️buildscript/🟦️.ts";

test("a stand-in whose module is gone runs the real build script with the same arguments",()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-retired-module-"));
 try{
  const real=join(directory,"build_script_build.real"),shim=join(directory,"build_script_build");
  writeFileSync(real,'#!/bin/sh\necho "real:$1:$2"\n');chmodSync(real,0o755);
  const text=shimSourceV1(real,process.execPath),guard=text.split("\n").find(line=>line.startsWith("[ -f "))!,module=/^\[ -f (\S+) \]/u.exec(guard)![1]!;writeFileSync(shim,text.replace(module,"'"+join(directory,"retired","🟦️.ts")+"'").replace(module,"'"+join(directory,"retired","🟦️.ts")+"'"));chmodSync(shim,0o755);
  const run=spawnSync(shim,["a","b"],{encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);expect(run.stdout.trim()).toBe("real:a:b");
  expect(shimSourceV1(real,"/x").split("\n").some(line=>line.startsWith("[ -f "))).toBe(true);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
