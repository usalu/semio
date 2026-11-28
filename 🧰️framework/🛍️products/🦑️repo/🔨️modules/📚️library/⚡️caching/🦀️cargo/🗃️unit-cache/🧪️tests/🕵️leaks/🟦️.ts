import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {wrapRustcV1} from "../../📜️script.ts";
import {leakedRootKindsV1} from "../../🛰️fleet/🟦️.ts";
import {drainUploadsV1} from "../../🛰️fleet/📤️uploader/🟦️.ts";
import {spoolLengthV1} from "../../🛰️fleet/📏️spool-size/🟦️.ts";
import {readLookupsV1} from "../../💾️store/🟦️.ts";
import {producerKeyFileV1} from "../../🪪️extern/🟦️.ts";
import {startRemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

/** 🕵️ A compiled output that holds a path of this machine can be reused only by a checkout with matching roots. */
const base=realpathSync(mkdtempSync(join(realpathSync(tmpdir()),"leaks-")));
afterAll(()=>{rmSync(base,{recursive:true,force:true});});
const write=(name:string,text:string)=>{const path=join(base,name);writeFileSync(path,text);return path;};

test("a root inside an output is found; a dependency file and an output without a root are not",()=>{
 const roots={"cargo-home":join(base,"cargo-home")};
 const clean=write("clean.rlib","no path here"),dirty=write("dirty.rlib",`prefix ${join(roots["cargo-home"],"registry","src","x")} suffix`),depinfo=write("dirty.d",roots["cargo-home"]);
 expect(leakedRootKindsV1([clean],roots)).toBeUndefined();
 expect(leakedRootKindsV1([dirty],roots)?.kinds).toEqual(["cargo-home"]);
 expect(leakedRootKindsV1([depinfo],roots)).toBeUndefined();
 expect(leakedRootKindsV1([dirty],{cwd:undefined,manifest:"/x",workspace:"",lane:join(base,"other")})).toBeUndefined();
});

test("a root is found under its resolved spelling too: the output holds the real path, the root is given through a link",()=>{
 const real=join(base,"real-rustup");mkdirSync(real);const alias=join(base,"alias-rustup");symlinkSync(real,alias,process.platform==="win32"?"junction":"dir");
 const output=write("tool.rlib",`path ${join(real,"toolchains","t")}`);
 expect(leakedRootKindsV1([output],{"rustup-home":alias})?.kinds).toEqual(["rustup-home"]);
 expect(leakedRootKindsV1([output],{"rustup-home":join(base,"missing-root-dir")})).toBeUndefined();
});

test("a split UTF-8 root and the native, slash and escaped spellings of a Windows root are found",()=>{
 const unicode="/équipe/🧰️framework",output=write("split.rlib"," ".repeat(64*1024-2)+unicode);
 expect(leakedRootKindsV1([output],{workspace:unicode})?.kinds).toEqual(["workspace"]);
 const windows="C:\\work\\repo";
 for(const spelling of[windows,windows.replaceAll("\\","/"),windows.replaceAll("\\","\\\\")])expect(leakedRootKindsV1([write("win.rlib",spelling)],{workspace:windows})?.kinds).toEqual(["workspace"]);
 expect(leakedRootKindsV1([join(base,"missing.rlib")],{workspace:unicode})?.kinds).toEqual(["unreadable"]);
});

const compiler=String.raw`
const {writeFileSync}=require("node:fs"),{join}=require("node:path"),args=process.argv.slice(1);
if(args.includes("--print"))process.exit(0);
if(args.includes("-vV")){console.log("rustc 1.0.0\nhost: test-host");process.exit(0);}
const name=args[args.indexOf("--crate-name")+1],out=args[args.indexOf("--out-dir")+1];
const dep=join(out,name+".d"),artifact=join(out,"lib"+name+".rlib");
writeFileSync(dep,name+".d: lib.rs\n");
writeFileSync(artifact,name==="rooted"?process.env.CARGO_MANIFEST_DIR:"portable");
for(const [emit,path] of [["dep-info",dep],["link",artifact]])console.log(JSON.stringify({emit,artifact:path}));
`;

async function compile(cwd:string,cache:string,lane:string,crate:string,remote:Readonly<Record<string,string>>={}){
 const out=join(cwd,"target"),artifact=join(out,"lib"+crate+".rlib");
 rmSync(out,{recursive:true,force:true});mkdirSync(out,{recursive:true});writeFileSync(join(cwd,"lib.rs"),"pub fn answer() -> u32 { 42 }\n");
 const environment={...process.env,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:undefined,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:cwd,SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,"-e",compiler,"--"]),SEMIO_UNIT_CACHE_TOOLCHAIN:"fake-compiler",SEMIO_OFFLOAD:"never",SEMIO_REMOTE_CACHE_ENDPOINT:undefined,SEMIO_UNIT_CACHE_SLOTS:"0",CARGO_MANIFEST_DIR:cwd,CARGO_PKG_NAME:crate,CARGO_PKG_VERSION:"0.1.0",PWD:cwd,...remote};
 const args=["--crate-name",crate,"--crate-type","lib","--emit=dep-info,link","--json=artifacts","--out-dir",out,"--remap-path-prefix="+cwd+"=.","lib.rs"];
 expect(await wrapRustcV1(args,environment,cwd)).toBe(0);
 const result=readLookupsV1(cache).findLast(entry=>entry.crate===crate&&entry.lane===lane);
 expect(result).toBeDefined();
 expect(JSON.parse(readFileSync(producerKeyFileV1(artifact),"utf8")).key).toBe(result!.key);
 return {result:result!,artifact,environment};
}

for(const row of fixture.cases)test(row.crate+" compiler outputs use the correct shared local cache scope",async()=>{
 const cache=join(base,row.crate,"cache"),cas=join(base,row.crate,"cas"),previous=process.env.SEMIO_CAS_ROOT;
 const results:string[]=[],keys:string[]=[];
 process.env.SEMIO_CAS_ROOT=cas;
 try{
  for(const name of row.checkoutOrder){
   const cwd=join(base,row.crate,name),{result,artifact}=await compile(cwd,cache,name,row.crate);
   expect(readFileSync(artifact,"utf8")).toBe(row.leaksRoot?cwd:"portable");
   results.push(result.outcome);keys.push(result.key);
  }
  expect(results).toEqual(row.expected);
  expect(new Set(keys).size).toBe(row.leaksRoot?2:1);
  const python=Bun.which("python3")??Bun.which("python");
  expect(python).toBeTruthy();
  const oracle=spawnSync(python!,["-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"🔣️.json"),base,row.crate],{encoding:"utf8"});
  expect(oracle.status,oracle.stderr).toBe(0);
  expect(JSON.parse(oracle.stdout)).toEqual(results);
 }finally{if(previous===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previous;}
});

for(const localFirst of [false,true])test("the rooted fleet result of a "+(localFirst?"local hit":"cold compile")+" installs into the scoped local key",async()=>{
 const previous=process.env.SEMIO_CAS_ROOT,root=join(base,localFirst?"fleet-offer":"fleet"),cwd=join(root,"checkout"),first=join(root,"cache-first"),second=join(root,"cache-second");
 const server=await startRemoteCacheServerV1({directory:join(root,"server"),host:"127.0.0.1",port:0,maximumBytes:2**26,maximumEntryBytes:2**23,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 process.env.SEMIO_CAS_ROOT=join(root,"cas");
 const remote={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS:"100"};
 try{
  const cold=await compile(cwd,first,"cold","rooted",localFirst?{}:remote);
  expect(cold.result.outcome).toBe("miss");
  const offered=localFirst?await compile(cwd,first,"offered","rooted",remote):cold;
  if(localFirst)expect(offered.result.outcome).toBe("local-hit");
  const deadline=Date.now()+5000;
  while(spoolLengthV1(first)>0&&Date.now()<deadline){
   await drainUploadsV1(first,offered.environment,{idleMilliseconds:0});
   if(spoolLengthV1(first)>0)await Bun.sleep(50);
  }
  expect(spoolLengthV1(first)).toBe(0);
  const fetched=await compile(cwd,second,"fetched","rooted",remote);
  expect(fetched.result.outcome).toBe("fleet-hit");
  expect(fetched.result.key).toBe(cold.result.key);
  expect(readFileSync(fetched.artifact,"utf8")).toBe(cwd);
  const warm=await compile(cwd,second,"warm","rooted");
  expect(warm.result.outcome).toBe("local-hit");
  expect(warm.result.key).toBe(cold.result.key);
 }finally{
  await server.stop();
  if(previous===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previous;
 }
},20000);
