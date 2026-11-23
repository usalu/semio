import {afterAll,expect,test} from "bun:test";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawn} from "node:child_process";
import {acquireLightSlotV1,acquireSlotV1} from "../../../🔒️lock/🟦️.ts";
import {runBuildScriptV1,slotHeldEnvironmentV1} from "../../🟦️.ts";
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"semio-slot-held-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
test("a build script that probes the compiler runs with the compile slots switched off, so the probe does not wait for the slot its parent holds",()=>{
 const environment=Object.freeze({SEMIO_UNIT_CACHE_SLOTS:"1",SEMIO_UNIT_CACHE_LIGHT_SLOTS:"6",RUSTC_WRAPPER:"wrapper"});
 expect(slotHeldEnvironmentV1(environment)).toEqual({...environment,SEMIO_UNIT_CACHE_SLOTS:"0"});
 expect(environment.SEMIO_UNIT_CACHE_SLOTS).toBe("1");
});
test("a build script run under the only compile slot finishes although its child needs a compile slot too, and the slot is free afterwards",async()=>{
 const root=fresh(),out=join(root,"out"),script=join(root,"build_script_build.real"),probe=join(root,"probe.ts"),seen=join(root,"seen");
 mkdirSync(out);
 writeFileSync(probe,`import {acquireSlotV1} from ${JSON.stringify(join(import.meta.dir,"../../../🔒️lock/🟦️.ts"))};import {writeFileSync} from "node:fs";\nconst slots=Number(process.env.SEMIO_UNIT_CACHE_SLOTS??0);\nif(slots>0){const claim=await acquireSlotV1(process.env.SEMIO_UNIT_CACHE_ROOT!,slots);claim.release();}\nwriteFileSync(${JSON.stringify(seen)},String(slots));\n`);
 writeFileSync(script,`#!/bin/sh\n${JSON.stringify(process.execPath)} ${JSON.stringify(probe)}\n`);chmodSync(script,0o755);
 const environment={...process.env,SEMIO_UNIT_CACHE_ROOT:root,SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_SLOTS:"1",SEMIO_UNIT_CACHE_LIGHT_SLOTS:"0",OUT_DIR:out,SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),CARGO_PKG_NAME:"probe"};
 const status=await Promise.race([runBuildScriptV1(script,[],environment,root),new Promise<number>(done=>setTimeout(()=>done(-1),20000))]);
 expect(status,"stalled: the child waited for the slot of its parent").toBe(0);
 expect(readFileSync(seen,"utf8")).toBe("0");
 const free=await acquireSlotV1(root,1);free.release();
 expect(readdirSync(join(root,"s")).filter(name=>name.endsWith(".lock"))).toEqual([]);
},30000);
test("a build script run that waits for a slot answers SIGTERM at once and never runs the script",async()=>{
 const root=fresh(),out=join(root,"out"),script=join(root,"build_script_build.real"),driver=join(root,"driver.ts"),ran=join(root,"ran");
 mkdirSync(out);
 writeFileSync(script,`#!/bin/sh\ntouch ${JSON.stringify(ran)}\n`);chmodSync(script,0o755);
 writeFileSync(driver,`import {runBuildScriptV1} from ${JSON.stringify(join(import.meta.dir,"../../🟦️.ts"))};\nprocess.exitCode=await runBuildScriptV1(${JSON.stringify(script)},[],process.env as Record<string,string>,process.cwd());\n`);
 const slot=await acquireSlotV1(root,1),light=await acquireLightSlotV1(root,1);
 try{
  const environment={...process.env,SEMIO_UNIT_CACHE_ROOT:root,SEMIO_UNIT_CACHE_BUILD_SCRIPTS:"on",SEMIO_UNIT_CACHE_SLOTS:"1",SEMIO_UNIT_CACHE_LIGHT_SLOTS:"1",OUT_DIR:out,SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),CARGO_PKG_NAME:"probe"},child=spawn(process.execPath,[driver],{env:environment,cwd:root,stdio:"ignore"});
  const exited=new Promise<number|null>(done=>child.on("close",status=>done(status)));
  await new Promise(done=>setTimeout(done,3000));
  child.kill("SIGTERM");
  const status=await Promise.race([exited,new Promise<string>(done=>setTimeout(()=>done("stalled"),15000))]);
  if(status==="stalled")child.kill("SIGKILL");
  expect(status,"SIGTERM was swallowed while waiting for a slot").toBe(143);
  expect(readdirSync(root)).not.toContain("ran");
 }finally{slot.release();light.release();}
},40000);
