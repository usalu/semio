import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import {copyFileSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {tmpdir} from "node:os";
import fixture from "./🧫️fixtures/🔣️.json";
import {BAZEL_HERMETIC_RUNNER} from "../../🏭️runner/🟦️.ts";

const line=BAZEL_HERMETIC_RUNNER.split("\n").find(row=>row.startsWith("const shim="))!;
const build=(io:{symlinkSync:unknown;copyFileSync:unknown;readFileSync:unknown;writeFileSync:unknown},executable:string)=>new Function("symlinkSync","copyFileSync","readFileSync","writeFileSync","resolve","dirname","executable","fail",line+"\nreturn shim;")(io.symlinkSync,io.copyFileSync,io.readFileSync,io.writeFileSync,resolve,dirname,executable,(message:string)=>{throw Error(message);}) as (target:string,path:string,platform?:string)=>void;
const check=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["name","platform","path","target","kind"],properties:{name:{type:"string"},platform:{enum:["win32","linux","darwin"]},path:{type:"string"},target:{type:"string"},kind:{enum:["cmd","exe","refuse","missing","link"]},real:{type:"string"},exists:{type:"boolean"}}});
const python=testPythonV1()!==null;

test("a Windows shim is a .cmd or .exe file, never a symbolic link or a hard link, and a POSIX shim stays a symbolic link",()=>{
 expect(line,"the runner declares one shim function").toContain("const shim=");
 for(const row of fixture.rows){
  expect(check(row),row.name+JSON.stringify(check.errors)).toBe(true);
  const calls:string[][]=[],io={symlinkSync:(...a:string[])=>calls.push(["symlink",...a]),copyFileSync:(...a:string[])=>calls.push(["copy",...a]),readFileSync:(path:string)=>{if(row.exists===false)throw Object.assign(Error("ENOENT"),{code:"ENOENT"});calls.push(["read",path]);return "";},writeFileSync:(...a:string[])=>calls.push(["write",...a])};
  const shim=build(io,fixture.executable),run=()=>shim(row.target,row.path,row.platform);
  if(row.kind==="link"){run();expect(calls,row.name).toEqual([["symlink",row.target,row.path]]);continue;}
  if(row.kind==="refuse"){expect(run,row.name).toThrow("cannot be shimmed");expect(calls,row.name).toEqual([]);continue;}
  if(row.kind==="missing"){expect(run,row.name).toThrow("ENOENT");expect(calls.some(call=>call[0]==="write"||call[0]==="symlink"),row.name).toBe(false);continue;}
  run();
  if(hasReal(row)&&python){const walked=Bun.spawnSync([testPythonV1()??"python3","-c","import os,sys;print(os.path.normpath(os.path.join(os.path.dirname(sys.argv[1]),sys.argv[2])))",row.path,row.target],{stdout:"pipe"}).stdout.toString().trim();expect(walked,row.name).toBe(row.real);}
  if(row.kind==="cmd")expect(calls.filter(call=>call[0]==="write"),row.name).toEqual([["write",row.path+".cmd",'@"'+fixture.executable+'" "'+row.real+'" %*\r\n']]);
  else expect<unknown>(calls.filter(call=>call[0]==="copy"),row.name).toEqual([["copy",row.real,row.path+".exe"]]);
  expect(calls.some(call=>call[0]==="symlink"),row.name+": no symbolic link on Windows").toBe(false);
 }
});
const hasReal=(row:{real?:string}):row is {real:string}=>row.real!==undefined;

test("on the real file system the Windows branch writes a shim file and copies an executable without sharing its inode",()=>{
 const base=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"shim-"));try{
  const directory=join(base,"node_modules"),bin=join(directory,".bin");mkdirSync(join(directory,"tool","bin"),{recursive:true});mkdirSync(bin,{recursive:true});
  writeFileSync(join(directory,"tool","bin","tool.js"),"console.log(1)\n");writeFileSync(join(directory,"tool","bin","tool.exe"),"MZ");
  const shim=build({symlinkSync,copyFileSync,readFileSync,writeFileSync},"/opt/bun");
  shim("../tool/bin/tool.js",join(bin,"tool"),"win32");shim("../tool/bin/tool.exe",join(bin,"native"),"win32");
  expect(readFileSync(join(bin,"tool.cmd"),"utf8")).toBe('@"/opt/bun" "'+join(directory,"tool","bin","tool.js")+'" %*\r\n');
  expect(lstatSync(join(bin,"tool.cmd")).isSymbolicLink()).toBe(false);
  const source=lstatSync(join(directory,"tool","bin","tool.exe")),copy=lstatSync(join(bin,"native.exe"));
  expect(copy.isSymbolicLink()).toBe(false);expect(copy.nlink).toBe(1);expect(source.nlink).toBe(1);expect(copy.ino).not.toBe(source.ino);
  shim("../tool/bin/tool.js",join(bin,"linked"),"linux");expect(lstatSync(join(bin,"linked")).isSymbolicLink()).toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
});
