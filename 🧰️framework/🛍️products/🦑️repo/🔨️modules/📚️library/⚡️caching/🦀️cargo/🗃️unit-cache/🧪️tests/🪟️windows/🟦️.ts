import {expect,test} from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import publicationSchema from "./🧫️fixtures/publication-interleaving.schema.json" with {type:"json"};
import publicationInterleaving from "./🧫️fixtures/publication-interleaving.json" with {type:"json"};
import {chmodSync,readFileSync,existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import responseFixture from "./🧫️fixtures/response-file.json" with {type:"json"};
import responseSchema from "./🧫️fixtures/response-file.schema.json" with {type:"json"};
import {expandRustcArgumentsV1} from "../../🔏️key/🟦️.ts";
import {ensureUnitCacheShimV1,unitCacheEnvironmentV1,unitCacheShimPathV1,UNIT_CACHE_NATIVE_BUILD_V1,type UnitCacheNativeBuildV1} from "../../🚪️launch/🟦️.ts";
import {removeClaimFileV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {churnProtocolV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";

test.skipIf(process.platform!=="win32")("nested compiler launch preserves a near-limit Cargo command through a response file", () => {
 const root=scratch();
 try {
  const data=JSON.parse(readFileSync(resolve(here,"🧫️fixtures/nested-argv.json"),"utf8")),schema=JSON.parse(readFileSync(resolve(here,"🧫️fixtures/nested-argv.schema.json"),"utf8"));
  expect(new Ajv({strict:true}).validate(schema,data)).toBe(true);
  const wrapper=ensureUnitCacheShimV1(join(root,"u"),process.execPath,undefined,"win32"),echo=resolve(here,"🧫️fixtures/🟦️response-echo.ts");
  const args=[...data.arguments,data.prefix+"x".repeat(data.payloadLength-data.prefix.length)];
  const run=spawnSync(wrapper,["rustc",...args],{encoding:"utf8",env:{PATH:process.env.PATH??"",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,echo])},maxBuffer:100000});
  expect(run.status,run.stderr).toBe(0);expect(JSON.parse(run.stdout)).toEqual(args);
 } finally {rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(process.platform!=="win32")("long compiler arguments agree with rustc including interleaved response files", () => {
 const root=scratch();
 try {
  const data=JSON.parse(readFileSync(resolve(here,"🧫️fixtures/nested-argv.json"),"utf8"));
  const tool=spawnSync("rustup",["which","rustc"],{encoding:"utf8"});expect(tool.status,tool.stderr).toBe(0);const rustc=tool.stdout.trim();
  const first=join(root,"first response.rsp"),second=join(root,"second response.rsp");writeFileSync(first,"--cfg=first\n");writeFileSync(second,"--cfg=second\n");
  const args=["--print=cfg","@"+first,data.compilerQuoted,...Array(data.compilerRepetitions).fill(data.compilerArgument),"@"+second];
  const expected=spawnSync(rustc,args,{encoding:"utf8",maxBuffer:1000000});expect(expected.status,expected.stderr).toBe(0);
  const wrapper=ensureUnitCacheShimV1(join(root,"u"),process.execPath,undefined,"win32");
  const actual=spawnSync(wrapper,["rustc",...args],{encoding:"utf8",env:{...process.env,SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc])},maxBuffer:1000000});
  expect(actual.status,actual.stderr).toBe(expected.status);expect(actual.stdout).toBe(expected.stdout);
  for(const value of ["","line\nbreak"]){const direct=spawnSync(rustc,[...args,value],{encoding:"utf8",maxBuffer:1000000}),wrapped=spawnSync(wrapper,["rustc",...args,value],{encoding:"utf8",env:{...process.env,SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc])},maxBuffer:1000000});expect(wrapped.status).toBe(direct.status);expect(wrapped.stderr).toBe(direct.stderr);}
 } finally {rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(process.platform!=="win32")("response files outlive the child and are retired after its actual failure", () => {
 const root=scratch();
 try {
  const data=JSON.parse(readFileSync(resolve(here,"🧫️fixtures/nested-argv.json"),"utf8")),wrapper=ensureUnitCacheShimV1(join(root,"u"),process.execPath,undefined,"win32"),probe=resolve(here,"🧫️fixtures/🟦️response-exit.ts");
  const actual=spawnSync(wrapper,["rustc",data.prefix+"x".repeat(data.payloadLength-data.prefix.length)],{encoding:"utf8",env:{PATH:process.env.PATH??"",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,probe])}});
  expect(actual.status,actual.stderr).toBe(7);const files=JSON.parse(actual.stdout) as {path:string;exists:boolean}[];expect(files.length).toBeGreaterThan(0);for(const file of files){expect(file.exists).toBe(true);expect(existsSync(file.path)).toBe(false);}
 } finally {rmSync(root,{recursive:true,force:true});}
},120000);

test.skipIf(process.platform!=="win32")("a failed asynchronous compiler spawn retires its own response file", () => {
 const root=scratch();
 try {
  const data=JSON.parse(readFileSync(resolve(here,"🧫️fixtures/nested-argv.json"),"utf8")),wrapper=ensureUnitCacheShimV1(join(root,"u"),process.execPath,undefined,"win32"),temporary=join(root,"temp");mkdirSync(temporary);
  const actual=spawnSync(wrapper,["rustc",data.prefix+"x".repeat(data.payloadLength-data.prefix.length)],{encoding:"utf8",env:{PATH:process.env.PATH??"",TMP:temporary,TEMP:temporary,TMPDIR:temporary,SEMIO_UNIT_CACHE_INNER:JSON.stringify([join(root,"absent.exe")])}});
  expect(actual.status).toBe(1);expect(actual.stderr).toContain("ENOENT");expect(readdirSync(temporary)).toEqual([]);
 } finally {rmSync(root,{recursive:true,force:true});}
},120000);

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
   if(python){const oracle=spawnSync(python,["-I","-X","utf8",echoPython,...row],{encoding:"utf8"});expect(JSON.parse(oracle.stdout)).toEqual(JSON.parse(run.stdout));}
  }
  for(const status of fixture.exitStatuses){
   const run=spawnSync(wrapper,["rustc","--crate-name","x"],{encoding:"utf8",env:{PATH:process.env.PATH??"",XPLAT_EXIT:String(status),SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,echoScript])}});
   expect(run.status).toBe(status);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
},120000);

test("Cargo response files remain response files across a cache miss above the Windows command limit", () => {
 expect(new Ajv({strict:true}).validate(responseSchema,responseFixture)).toBe(true);
 const root=scratch();
 try {
  const args=[...responseFixture.arguments,...Array.from({length:responseFixture.repetitions},()=>responseFixture.repeatedArgument)],file=join(root,"rustc-arguments.rsp"),reference="@"+file;
  expect(args.join(" ").length).toBeGreaterThan(responseFixture.minimumCommandCharacters);
  writeFileSync(file,args.join("\n"));
  const wrapper=ensureUnitCacheShimV1(join(root,"u"),process.execPath,undefined,"win32"),run=spawnSync(wrapper,["rustc",reference],{encoding:"utf8",env:{PATH:process.env.PATH??"",SEMIO_UNIT_CACHE_ROOT:join(root,"u"),SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,echoScript])}});
  expect(run.status,run.stderr).toBe(0);
  expect(JSON.parse(run.stdout)).toEqual([reference]);
  expect(expandRustcArgumentsV1([reference],root)).toEqual(args);
  const python=testPythonV1();
  expect(python).toBeDefined();
  const oracle=spawnSync(python!,["-I","-X","utf8","-c","import argparse,json,sys; print(json.dumps(argparse.ArgumentParser(fromfile_prefix_chars=chr(64),add_help=False).parse_known_args(sys.argv[1:])[1],ensure_ascii=False))",reference],{encoding:"utf8"});
  expect(oracle.status,oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(expandRustcArgumentsV1([reference],root));
 } finally {rmSync(root,{recursive:true,force:true});}
},120000);

test("the executable is named by the code it embeds: unchanged code is never recompiled, changed code retains prior immutable executables", () => {
 const root=join(scratch(),"u"),calls={bundle:0,compile:0};let code="one";
 const build:UnitCacheNativeBuildV1={bundle:()=>{calls.bundle++;return code;},compile:(_runtime,_entry,outfile)=>{calls.compile++;writeFileSync(outfile,"exe:"+code);}};
 try{
  const first=ensureUnitCacheShimV1(root,"/bin/bun","/s.ts","win32",build,"/entry.ts");
  expect(ensureUnitCacheShimV1(root,"/bin/bun","/s.ts","win32",build,"/entry.ts")).toBe(first);expect(calls.compile).toBe(1);
  code="two";const second=ensureUnitCacheShimV1(root,"/bin/bun","/s.ts","win32",build,"/entry.ts");
  expect(second).not.toBe(first);expect(calls.compile).toBe(2);expect(existsSync(first),"a peer may still need its immutable executable").toBe(true);expect(existsSync(second)).toBe(true);
  expect(readdirSync(join(root,"bin")).filter(name=>name.includes(".tmp"))).toEqual([]);
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

test("native wrapper publication preserves another producer staged executable", () => {
 expect(new Ajv({strict:true}).validate(publicationSchema,publicationInterleaving)).toBe(true);
 const root=join(scratch(),"u"),database=new Database(":memory:");
 database.exec("create table files(owner text primary key, phase text)");
 try {
  for(const step of publicationInterleaving.steps){
   const [owner,operation]=step.split(".");
   if(operation==="stage")database.query("insert into files values(?,?)").run(owner!,"staged");
   else if(operation==="publish")database.query("update files set phase=? where owner=?").run("published",owner!);
   else database.query("delete from files where phase=?").run("retired");
  }
  const oracle=(database.query("select owner from files where phase=? order by owner").all("published") as {owner:string}[]).map(row=>row.owner);
  expect(oracle).toEqual(publicationInterleaving.expectedPublished);
  const winner:UnitCacheNativeBuildV1={bundle:()=>"winner",compile:(_runtime,_entry,outfile)=>writeFileSync(outfile,"exe:winner")};
  const peer:UnitCacheNativeBuildV1={bundle:()=>"peer",compile:(_runtime,_entry,outfile)=>{expect(outfile.endsWith(publicationInterleaving.temporarySuffix)).toBe(true);writeFileSync(outfile,"exe:peer");ensureUnitCacheShimV1(root,"/bin/bun",undefined,"win32",winner,"/winner.ts");expect(existsSync(outfile),"the winner must retain its peer original stage").toBe(true);}};
  const published=ensureUnitCacheShimV1(root,"/bin/bun",undefined,"win32",peer,"/peer.ts");
  expect(readFileSync(published,"utf8")).toBe("exe:peer");
  expect(readdirSync(join(root,"bin")).map(name=>readFileSync(join(root,"bin",name),"utf8").slice(4)).sort()).toEqual(oracle);
  expect(readdirSync(join(root,"bin")).filter(name=>name.includes(".tmp"))).toEqual([]);
 } finally { database.close();rmSync(join(root,".."),{recursive:true,force:true}); }
});
