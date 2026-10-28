import {expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {ensureUnitCacheShimV1,unitCacheEnvironmentV1,unitCacheShimPathV1,UNIT_CACHE_NATIVE_BUILD_V1,type UnitCacheNativeBuildV1} from "../../🚪️launch/🟦️.ts";
import {removeClaimFileV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {churnProtocolV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

/** 🪟️ Contract: Cargo executes `RUSTC_WRAPPER` directly with arguments that carry quotes; on Windows that has to be a native executable (a batch file cannot take them), and a claim release in a `finally` never throws on a sharing violation. The native wrapper is built and run for real on every host (the `.exe` name is only a name there); the Windows-only facts are injected. */
const here=fileURLToPath(new URL(".",import.meta.url)),base=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());mkdirSync(base,{recursive:true});
const scratch=()=>mkdtempSync(join(base,"ucw-"));
const echoScript=resolve(here,"🧫️fixtures/🟦️echo.ts"),echoPython=resolve(here,"🧫️fixtures/🐍️echo.py");

test("the Windows stand-in is a native executable that hands Cargo's quoted arguments to the compiler unchanged, like the independent Python echo", () => {
 const root=scratch();
 try{
  const wrapper=ensureUnitCacheShimV1(join(root,"u"),process.execPath,undefined,"win32");
  expect(wrapper.endsWith(".exe")).toBe(true);expect(readdirSync(join(root,"u","bin")).some(name=>name.endsWith(".cmd")||name.endsWith(".bat")),"never a batch file").toBe(false);
  const python=testPythonV1();
  for(const row of fixture.rustcArguments){
   const run=spawnSync(wrapper,["C:\\rust\\bin\\rustc.exe",...row],{encoding:"utf8",env:{PATH:process.env.PATH??"",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,echoScript])}});
   expect(run.status,run.stderr).toBe(0);expect(JSON.parse(run.stdout)).toEqual(row);
   if(python){const oracle=spawnSync(python,["-I",echoPython,...row],{encoding:"utf8"});expect(JSON.parse(oracle.stdout)).toEqual(JSON.parse(run.stdout));}
  }
  for(const status of fixture.exitStatuses){
   const run=spawnSync(wrapper,["rustc","--crate-name","x"],{encoding:"utf8",env:{PATH:process.env.PATH??"",XPLAT_EXIT:String(status),SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,echoScript])}});
   expect(run.status).toBe(status);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("the executable is named by the code it embeds: unchanged code is never recompiled, changed code replaces the older executable", () => {
 const root=join(scratch(),"u"),calls={bundle:0,compile:0};let code="one";
 const build:UnitCacheNativeBuildV1={bundle:()=>{calls.bundle++;return code;},compile:(_runtime,_entry,outfile)=>{calls.compile++;writeFileSync(outfile,"exe:"+code);}};
 try{
  const first=ensureUnitCacheShimV1(root,"/bin/bun","/s.ts","win32",build,"/entry.ts");
  expect(ensureUnitCacheShimV1(root,"/bin/bun","/s.ts","win32",build,"/entry.ts")).toBe(first);expect(calls.compile).toBe(1);
  code="two";const second=ensureUnitCacheShimV1(root,"/bin/bun","/s.ts","win32",build,"/entry.ts");
  expect(second).not.toBe(first);expect(calls.compile).toBe(2);expect(existsSync(first),"the older executable is removed").toBe(false);expect(existsSync(second)).toBe(true);
  expect(readdirSync(join(root,"bin")).filter(name=>name.endsWith(".tmp"))).toEqual([]);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("a wrapper that cannot be built switches the unit cache off for that Cargo process and says so in both languages", () => {
 const root=join(scratch(),"u"),failing:UnitCacheNativeBuildV1={bundle:()=>{throw Error("bundler missing");},compile:()=>{throw Error("unused");}};
 try{
  const block=unitCacheEnvironmentV1({cacheRoot:root,lane:"l",laneRoot:join(root,"l"),compiler:"rustc",toolchain:"tc",focus:[],platform:"win32",build:failing});
  expect(block.RUSTC_WRAPPER,"an empty value overrides the configured wrapper").toBe("");expect(block.SEMIO_UNIT_CACHE).toBe("off");
  const lines=block.SEMIO_UNIT_CACHE_REFUSAL!.split("\n");expect(lines.length).toBe(2);expect(lines[0]).toContain("bundler missing");expect(lines[1]).toContain("Unit-Cache");expect(lines[1]).toContain("bundler missing");
  expect(existsSync(join(root,"bin"))).toBe(false);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("the POSIX stand-in stays a script and the Windows path names an executable", () => {
 expect(unitCacheShimPathV1("/u","linux")).toBe(join("/u","bin","rustc-wrapper"));expect(unitCacheShimPathV1("/u","win32","0123").endsWith("rustc-wrapper-0123.exe")).toBe(true);
 expect(typeof UNIT_CACHE_NATIVE_BUILD_V1.compile).toBe("function");
});

test.skipIf(process.platform==="win32")("releasing a claim never throws, whatever the file system says; a sharing violation is retried on the churn pauses first and announced when it stays", () => {
 const root=scratch(),locks=join(root,"locks");mkdirSync(locks);
 try{
  const claim=tryClaimV1(locks,"unit")!;expect(claim).toBeDefined();
  chmodSync(locks,0o500);
  expect(()=>claim.release(),"a refusal of the host is final and never thrown").not.toThrow();
  const pauses:number[]=[],reports:string[]=[];
  expect(removeClaimFileV1(join(locks,"unit.lock"),{platform:"win32",sleepSync:ms=>{pauses.push(ms);},report:text=>{reports.push(text);}})).toBe(false);
  expect(pauses).toEqual([...churnProtocolV1().profiles.read.backoffMs]);expect(reports.length).toBe(1);expect(reports[0]).toContain("unit.lock");
  chmodSync(locks,0o700);
  expect(removeClaimFileV1(join(locks,"unit.lock"))).toBe(true);expect(existsSync(join(locks,"unit.lock"))).toBe(false);
  expect(removeClaimFileV1(join(locks,"absent.lock"))).toBe(true);
 }finally{try{chmodSync(locks,0o700);}catch{}rmSync(root,{recursive:true,force:true});}
});
