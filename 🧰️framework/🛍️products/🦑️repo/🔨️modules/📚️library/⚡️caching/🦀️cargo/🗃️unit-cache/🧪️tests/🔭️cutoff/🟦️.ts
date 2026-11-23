import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {homedir,tmpdir} from "node:os";
import {join} from "node:path";
import {wrapRustcV1} from "../../📜️script.ts";
import {readLookupsV1,digestFileV1} from "../../💾️store/🟦️.ts";
import {dependenciesFileV1,identityFileV1,listRootV1} from "../../🔭️interface/🟦️.ts";

/** 🧥️ The cutoff through the real wrapper and the real compiler: after a body-only edit of a library the dependants of a check chain are cache hits (their metadata rebased onto the rebuilt library), after a signature edit they are compiled. */
const scratch:string[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"r39-cut-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function findRustc():string|undefined{
 if(process.env.SEMIO_TEST_RUSTC)return process.env.SEMIO_TEST_RUSTC;
 for(const root of[process.env.SEMIO_TEST_CHECKOUT,"/Users/ueli/Documents/semio"]){
  if(root===undefined)continue;
  try{
   const channel=/^channel\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(root,"rust-toolchain.toml"),"utf8"))?.[1],base=join(process.env.RUSTUP_HOME??join(homedir(),".rustup"),"toolchains"),name=readdirSync(base).find(entry=>entry.startsWith(channel+"-"));
   if(name!==undefined&&existsSync(join(base,name,"bin","rustc")))return join(base,name,"bin","rustc");
  }catch{}
 }
 return undefined;
}
const rustc=findRustc(),real=rustc===undefined?test.skip:test;

type World={root:string;cache:string;work:string};
function world(label:string):World{
 const root=directory(label),cache=join(root,"cache"),work=join(root,"work");
 mkdirSync(join(work,"target","deps"),{recursive:true});mkdirSync(cache,{recursive:true});
 process.env.SEMIO_CAS_ROOT=join(root,"cas");
 return{root,cache,work};
}
const sources:Record<string,string>={
 a:"pub struct S { pub f: u32 }\npub fn answer(x: u32) -> u32 { x + 1 }\n",
 b:"pub fn twice(x: u32) -> u32 { a::answer(x) * 2 }\npub fn make() -> a::S { a::S { f: 1 } }\n",
 c:"pub fn thrice(x: u32) -> u32 { b::twice(x) + a::answer(x) }\n",
};
const externsOf:Record<string,readonly string[]>={a:[],b:["a"],c:["a","b"]};
async function check(w:World,crate:string,source:string,opts:{noCache?:boolean}={}):Promise<number>{
 const file=join(w.work,crate,"lib.rs");mkdirSync(join(w.work,crate),{recursive:true});writeFileSync(file,source);
 const argv=["--crate-name",crate,"--edition=2021",crate+"/lib.rs","--error-format=json","--json=diagnostic-rendered-ansi,artifacts,future-incompat","--crate-type","lib","--emit=dep-info,metadata","-C","embed-bitcode=no","-C","metadata="+crate,"-C","extra-filename=-"+crate,"--out-dir","target/deps","-L","dependency=target/deps",...externsOf[crate]!.flatMap(dependency=>["--extern",dependency+"=target/deps/lib"+dependency+"-"+dependency+".rmeta"]),"-Z","threads=0"];
 const environment={...process.env,SEMIO_UNIT_CACHE_ROOT:w.cache,SEMIO_UNIT_CACHE_LANE:"d",SEMIO_UNIT_CACHE_INNER:JSON.stringify([rustc!]),SEMIO_UNIT_CACHE_TOOLCHAIN:"r39-test",CARGO_PKG_NAME:crate,CARGO_MANIFEST_DIR:join(w.work,crate),RUSTC_BOOTSTRAP:"1",...(opts.noCache?{SEMIO_UNIT_CACHE:"off"}:{})} as Record<string,string|undefined>;
 delete environment.SEMIO_UNIT_CACHE_LANE_ROOT;
 const write=process.stdout.write.bind(process.stdout),err=process.stderr.write.bind(process.stderr);
 process.stdout.write=(()=>true) as typeof process.stdout.write;process.stderr.write=(()=>true) as typeof process.stderr.write;
 const origin=process.cwd();process.chdir(w.work);
 try{return await wrapRustcV1(argv,environment,w.work);}finally{process.chdir(origin);process.stdout.write=write;process.stderr.write=err;}
}
const round=async(w:World,edits:Partial<Record<string,string>>={}):Promise<Record<string,string>>=>{
 const before=readLookupsV1(w.cache).length,sourcesNow={...sources,...edits};
 for(const crate of["a","b","c"])expect(await check(w,crate,sourcesNow[crate]!),crate).toBe(0);
 const rows=readLookupsV1(w.cache).slice(before),outcomes:Record<string,string>={};
 for(const row of rows)outcomes[row.crate]=row.outcome;
 return outcomes;
};
const rmetaOf=(w:World,crate:string)=>join(w.work,"target","deps","lib"+crate+"-"+crate+".rmeta");
const rustcCheck=(w:World,name:string,source:string,externs:readonly string[])=>{
 const file=join(w.work,name+".rs");writeFileSync(file,source);
 return spawnSync(rustc!,["--edition","2021","--crate-name",name,"--crate-type","lib","--emit=metadata","--out-dir",join(w.root,"scratch-"+name),"-L","dependency="+join(w.work,"target","deps"),...externs.flatMap(dependency=>["--extern",dependency+"="+rmetaOf(w,dependency)]),file],{encoding:"utf8"});
};

real("a body-only edit of a library: it is compiled once, its dependants are hits whose metadata is rebased, the chain stays consistent",async()=>{
 const w=world("body");
 expect(await round(w)).toEqual({a:"miss",b:"miss",c:"miss"});
 expect(await round(w)).toEqual({a:"local-hit",b:"local-hit",c:"local-hit"});
 const hashBefore=listRootV1([rustc!],rmetaOf(w,"a"),{...process.env,RUSTC_BOOTSTRAP:"1"})!.svh;
 const outcomes=await round(w,{a:sources.a!.replace("x + 1","x + 100")});
 expect(outcomes,"a is compiled, b and c are served").toEqual({a:"miss",b:"local-hit",c:"local-hit"});
 const hashAfter=listRootV1([rustc!],rmetaOf(w,"a"),{...process.env,RUSTC_BOOTSTRAP:"1"})!.svh;
 expect(hashAfter,"the library really is another crate").not.toBe(hashBefore);
 const table=JSON.parse(readFileSync(dependenciesFileV1(rmetaOf(w,"c")),"utf8")).dependencies as [string,string][];
 expect(table.find(([id])=>id==="a-a")?.[1],"the served dependant records the hash of the rebuilt library").toBe(hashAfter);
 expect(table.find(([id])=>id==="b-b")?.[1]).toBe(JSON.parse(readFileSync(dependenciesFileV1(rmetaOf(w,"b")),"utf8")).svh);
 const consumer=rustcCheck(w,"d","pub fn run() -> u32 { c::thrice(1) + b::twice(1) + a::answer(1) + b::make().f }\n",["a","b","c"]);
 expect(consumer.status,consumer.stderr).toBe(0);
 expect(existsSync(identityFileV1(rmetaOf(w,"b")))).toBe(true);
 expect(await round(w,{a:sources.a!.replace("x + 1","x + 100")})).toEqual({a:"local-hit",b:"local-hit",c:"local-hit"});
},900000);

real("an edit that changes the interface compiles every dependant",async()=>{
 const w=world("signature");
 expect(await round(w)).toEqual({a:"miss",b:"miss",c:"miss"});
 const outcomes=await round(w,{a:sources.a+"pub fn extra() -> u8 { 0 }\n"});
 expect(outcomes).toEqual({a:"miss",b:"miss",c:"miss"});
},900000);

real("a body edit that a dependant can observe (a const function) compiles the dependants",async()=>{
 const w=world("const");
 const base={a:"pub const fn k(x: u32) -> u32 { x + 1 }\npub fn answer(x: u32) -> u32 { x }\n",b:"pub fn twice(x: u32) -> u32 { a::k(x) }\n",c:"pub fn thrice(x: u32) -> u32 { b::twice(x) + a::answer(x) }\n"};
 const run=async(edits:Partial<Record<string,string>>)=>{const before=readLookupsV1(w.cache).length,now={...base,...edits};for(const crate of["a","b","c"])expect(await check(w,crate,now[crate]!),crate).toBe(0);const out:Record<string,string>={};for(const row of readLookupsV1(w.cache).slice(before))out[row.crate]=row.outcome;return out;};
 expect(await run({})).toEqual({a:"miss",b:"miss",c:"miss"});
 expect(await run({a:base.a.replace("x + 1","x + 2")})).toEqual({a:"miss",b:"miss",c:"miss"});
},900000);

real("with the cache off nothing changes: the wrapper compiles what it is given",async()=>{
 const w=world("off");
 expect(await check(w,"a",sources.a!,{noCache:true})).toBe(0);
 expect(existsSync(dependenciesFileV1(rmetaOf(w,"a")))).toBe(false);
 expect(digestFileV1(rmetaOf(w,"a"),undefined)).not.toBeNull();
});
