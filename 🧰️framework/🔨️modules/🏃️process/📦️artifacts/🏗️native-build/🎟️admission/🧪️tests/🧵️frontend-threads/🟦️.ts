import {registeredToolExecutableV1} from "../../../../../🧰️tools/🏃️run/🟦️.ts";
/** 🧵️ Contract: the scheduler hands rustc the front-end thread count the run was admitted, not the cores of the host. Cargo's authored `-Z threads=0` becomes the admitted parallelism, a larger count is capped to it, a smaller one stays, both spellings are read and nothing changes without a grant. The real scheduler binary runs a stand-in rustc that prints its arguments; the verdict of every language-agnostic corpus case (`./🔣️.json`) must equal the independent Python oracle (`./🐍️.py`). Measured on one 2.7 MB crate (opt-level 0, debuginfo 1, 256 codegen units, fresh incremental cache, load 23 to 35): 1 thread 181 s wall / 129 s CPU / 2.9 GB peak, 4 threads 119 s / 132 s / 5.4 GB, 10 threads (`-Z threads=0`) 90 s / 143 s / 5.2 GB. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {chmodSync,closeSync,existsSync,mkdirSync,mkdtempSync,openSync,readdirSync,readFileSync,rmSync,writeFileSync,writeSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const manifest=resolve(fileURLToPath(new URL("../../📦️packages/🦀️rust/Cargo.toml",import.meta.url))),oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));

const scratchRoot=()=>process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();
let builtScheduler:string|undefined;
function schedulerBinary(scratch:string):string{
 if(process.env.SEMIO_TEST_SCHEDULER)return process.env.SEMIO_TEST_SCHEDULER;
 if(builtScheduler&&existsSync(builtScheduler))return builtScheduler;
 const built=spawnSync(registeredToolExecutableV1("cargo"),["build","--release","--manifest-path",manifest,"--bins"],{cwd:scratch,env:{...process.env,CARGO_TARGET_DIR:join(scratch,"target")},encoding:"utf8"});
 expect(built.status,built.stderr).toBe(0);
 return builtScheduler=join(scratch,"target","release","semio-build-scheduler");
}

test("the corpus is closed by its schema and agrees with the independent Python oracle",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 const run=spawnSync(requireTestToolV1("python"),["-I",oracle,fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 expect(JSON.parse(run.stdout)).toEqual(corpus.cases.map(item=>item.expected));
});

test("the scheduler binary hands the stand-in rustc exactly the corpus arguments",()=>{
 if(process.platform==="win32"||!existsSync(manifest))return;
 const scratch=mkdtempSync(join(scratchRoot(),"ft-"));
 try{
  const scheduler=schedulerBinary(scratch);
  const rustc=join(scratch,"rustc.sh");writeFileSync(rustc,'#!/bin/sh\nfor a in "$@"; do printf "%s\\n" "$a"; done\n');chmodSync(rustc,0o755);
  for(const item of corpus.cases){
   const env:Record<string,string|undefined>={...process.env,SEMIO_BUILD_BROKER:undefined,SEMIO_BUILD_PASS:undefined,SEMIO_COMPILER_PARALLELISM:item.tokens===null?undefined:String(item.tokens)};
   const run=spawnSync(scheduler,[rustc,...item.args],{env:env as NodeJS.ProcessEnv,encoding:"utf8"});
   expect(run.status,item.id+run.stderr).toBe(0);
   expect(run.stdout.split("\n").slice(0,-1),item.id).toEqual(item.expected);
  }
  const expanding=join(scratch,"rustc-argfile.sh");writeFileSync(expanding,'#!/bin/sh\nfor a in "$@"; do case "$a" in @*) printf "FILE:\\n"; cat "${a#@}"; printf "\\n";; *) printf "%s\\n" "$a";; esac; done\n');chmodSync(expanding,0o755);
  const argfile=join(scratch,"command.args");writeFileSync(argfile,"-C\nopt-level=0\n-Z\nthreads=0\nlib.rs");
  const bounded=spawnSync(scheduler,[expanding,"--edition=2021","@"+argfile],{env:{...process.env,SEMIO_BUILD_BROKER:undefined,SEMIO_BUILD_PASS:undefined,SEMIO_COMPILER_PARALLELISM:"3"} as NodeJS.ProcessEnv,encoding:"utf8"});
  expect(bounded.status,bounded.stderr).toBe(0);
  expect(bounded.stdout,"the thread count inside an argument file is bounded like one on the command line").toContain("-Z\nthreads=3\nlib.rs");
  expect(bounded.stdout).not.toContain("threads=0");
  expect(existsSync(argfile+"."+"semio-threads-"+String(bounded.pid))||readdirSync(scratch).some(name=>name.includes("semio-threads")),"the bounded copy is removed after rustc ended").toBe(false);
  const free=spawnSync(scheduler,[expanding,"@"+argfile],{env:{...process.env,SEMIO_BUILD_BROKER:undefined,SEMIO_BUILD_PASS:undefined,SEMIO_COMPILER_PARALLELISM:undefined} as NodeJS.ProcessEnv,encoding:"utf8"});
  expect(free.stdout,"without a grant nothing changes").toContain("threads=0");
 }finally{rmSync(scratch,{recursive:true,force:true});}
},180000);

test("Law: the compiled artifact does not depend on the front-end thread count: the same crate built with 1, 4 and the authored 0 threads gives byte-identical rlib and rmeta files, so bounding the threads can never change an output",()=>{
 const toolchain=/channel\s*=\s*"([^"]+)"/u.exec(readFileSync(resolve(fileURLToPath(new URL("../../../../../../../../rust-toolchain.toml",import.meta.url))),"utf8"))?.[1];
 expect(toolchain,"the repository pins its toolchain").toBeDefined();
 if(!toolchain||spawnSync("rustc",[`+${toolchain}`,"--version"],{encoding:"utf8"}).status!==0)return;
 const scratch=mkdtempSync(join(tmpdir(),"ft-det-"));
 try{
  writeFileSync(join(scratch,"lib.rs"),Array.from({length:40},(_,at)=>`mod m${at};`).join("\n")+"\n"+Array.from({length:40},(_,at)=>`pub use m${at}::*;`).join("\n")+"\n");
  for(let at=0;at<40;at++)writeFileSync(join(scratch,`m${at}.rs`),`pub struct S${at}<T>(pub T);\nimpl<T: Clone + std::fmt::Debug> S${at}<T> { pub fn go(&self) -> String { format!("{:?}${at}", self.0.clone()) } }\npub fn f${at}(x: u64) -> u64 { let v: Vec<u64> = (0..x).map(|n| n * ${at+1}).collect(); v.iter().sum() }\nmacro_rules! m${at} { ($e:expr) => { $e + ${at} } }\npub fn g${at}() -> u64 { m${at}!(1) }\n`);
  const digests=["1","4","0","4"].map((threads,index)=>{
   const out=join(scratch,`out${index}`);mkdirSync(out);
   const run=spawnSync("rustc",[`+${toolchain}`,"--edition","2021","--crate-type=rlib","--crate-name","det","--emit=link,metadata","-C","opt-level=1","-C","codegen-units=4","-C","metadata=abc",`-Zthreads=${threads}`,"--out-dir",out,"lib.rs"],{cwd:scratch,encoding:"utf8"});
   expect(run.status,run.stderr).toBe(0);
   return ["libdet.rlib","libdet.rmeta"].map(name=>createHash("sha256").update(readFileSync(join(out,name))).digest("hex"));
  });
  for(const row of digests.slice(1))expect(row).toEqual(digests[0]);
 }finally{rmSync(scratch,{recursive:true,force:true});}
},300000);

test("a cdylib final runs its ThinLTO back end serially whatever the granted jobserver, so its memory never grows with idle cores",()=>{
 const toolchain=/channel\s*=\s*"([^"]+)"/u.exec(readFileSync(resolve(fileURLToPath(new URL("../../../../../../../../rust-toolchain.toml",import.meta.url))),"utf8"))?.[1];
 if(process.platform==="win32"||!toolchain||spawnSync("rustc",[`+${toolchain}`,"--version"],{encoding:"utf8"}).status!==0||!existsSync("/usr/bin/time"))return;
 const scratch=mkdtempSync(join(scratchRoot(),"ft-par-"));let handle:number|undefined;
 try{
  const scheduler=schedulerBinary(scratch),modules=32;
  writeFileSync(join(scratch,"lib.rs"),Array.from({length:modules},(_,at)=>`mod m${at};`).join("\n")+"\n");
  for(let at=0;at<modules;at++)writeFileSync(join(scratch,`m${at}.rs`),`pub struct S<T>(pub T);\nimpl<T: Clone + std::fmt::Debug> S<T> { pub fn go(&self) -> String { format!("{:?}", self.0.clone()) } }\n`+Array.from({length:12},(_,k)=>`#[unsafe(no_mangle)] pub extern "C" fn f${at}_${k}(x: u64) -> u64 { let v: Vec<u64> = (0..x % 97).map(|n| n.wrapping_mul(${at+k+1}) ^ ${k}).collect(); v.iter().fold(0u64, |a, b| a.wrapping_mul(31).wrapping_add(*b)) + S(x).go().len() as u64 }\n`).join(""));
  const pool=join(scratch,"pool");expect(spawnSync("mkfifo",["-m","600",pool]).status).toBe(0);
  handle=openSync(pool,"r+");writeSync(handle,"+++");
  const grant=`-j --jobserver-auth=fifo:${pool}`,measure=(grantedJobs:number,name:string)=>{
   const out=join(scratch,name);mkdirSync(out);
   const run=spawnSync("/usr/bin/time",["-p",scheduler,"rustc",`+${toolchain}`,"--edition","2021","--crate-type","cdylib","--crate-name","par","-C","opt-level=2","-C","lto=thin","-C","codegen-units=16","-C","metadata=x","-Z","threads=0","--out-dir",out,"lib.rs"],{cwd:scratch,env:{...process.env,CARGO_MAKEFLAGS:grant,MAKEFLAGS:grant,SEMIO_COMPILER_PARALLELISM:String(grantedJobs),SEMIO_BUILD_BROKER:undefined,SEMIO_BUILD_PASS:undefined} as NodeJS.ProcessEnv,encoding:"utf8"});
   expect(run.status,run.stderr).toBe(0);
   const seconds=(label:string)=>Number(new RegExp(`^${label} ([0-9.]+)$`,"mu").exec(run.stderr)?.[1]);
   return{ratio:(seconds("user")+seconds("sys"))/seconds("real")};
  };
  const granted=measure(4,"granted"),single=measure(1,"single");
  expect(granted.ratio,"4 granted cores, one unit running: the codegen and LTO workers stay on one thread").toBeLessThan(1.25);
  expect(single.ratio).toBeLessThan(1.25);
 }finally{if(handle!==undefined)closeSync(handle);rmSync(scratch,{recursive:true,force:true});}
},600000);
