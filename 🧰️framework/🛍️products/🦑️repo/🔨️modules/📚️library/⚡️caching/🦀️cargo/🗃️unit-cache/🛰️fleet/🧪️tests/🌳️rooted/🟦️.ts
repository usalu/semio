import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {cargoScratchBaseV1} from "../../../🧪️tests/🏖️scratch/🟦️.ts";
import {drainUploadsV1} from "../../📤️uploader/🟦️.ts";
import {spoolLengthV1} from "../../📏️spool-size/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
import {readFallbacksV1} from "../../📊️counters/🟦️.ts";
import {leakedRootKindsV1,parseRootHintV1,rootHintBytesV1,rootHintKeyV1,rootPathsV1,ROOT_KINDS_V1} from "../../🟦️.ts";
process.env.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS="1500";

/** 🌳️ A unit whose compiled bytes hold a real directory that its source never names (a procedural macro that writes the caller's manifest directory into the program) is shared between machines whose directories are equal and between those only: the producer publishes a root hint beside the neutral key and the unit under a key that names the directories; a machine at another root builds it itself. The units without such a directory stay shared among all roots, and a unit built through the fleet equals a clean local build byte for byte. */
const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../../📜️script.ts"),counter=resolve(here,"../🔢️count/🟦️.ts"),fleet=FLEET_POLICY_V1,cargo=process.env.SEMIO_TEST_CARGO??"cargo";
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string,base=cargoScratchBaseV1())=>{const path=realpathSync(mkdtempSync(join(base,"rt-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("the root hint is an action result of one closed shape, keyed by the neutral key, and the root kinds name the machine's directories",()=>{
 const key=sha("neutral");
 expect(rootHintKeyV1(key)).toMatch(/^[a-f0-9]{64}$/u);expect(rootHintKeyV1(key)).not.toBe(key);expect(rootHintKeyV1(key)).toBe(rootHintKeyV1(key));
 expect(parseRootHintV1(rootHintBytesV1(["manifest","cwd","cwd"]))).toEqual(["cwd","manifest"]);
 expect(parseRootHintV1(rootHintBytesV1([]))).toBeUndefined();expect(parseRootHintV1(new Uint8Array([1,2,3]))).toBeUndefined();
 const named={cwd:"/w/a",manifest:"/w/a/m",workspace:undefined,"cargo-home":"/h/c"};
 expect(rootPathsV1(["manifest","cwd"],named)).toEqual(["cwd=/w/a","manifest=/w/a/m"]);
 expect(rootPathsV1(["workspace"],named),"a kind the machine lacks cannot be keyed").toBeUndefined();
 expect(ROOT_KINDS_V1).not.toContain("lane");
});

test("the leak scan finds split UTF-8 paths and refuses unreadable compiler outputs",()=>{
 const dir=directory("split",tmpdir()),path=join(dir,"unit.rlib"),root="/équipe/🧰️framework";
 writeFileSync(path,Buffer.concat([Buffer.alloc(64*1024-2),Buffer.from(root),Buffer.alloc(80*1024)]));
 expect(leakedRootKindsV1([path],{workspace:root})?.kinds).toEqual(["workspace"]);
 expect(leakedRootKindsV1([join(dir,"missing.rlib")],{workspace:root})?.kinds).toEqual(["unreadable"]);
});

test("the leak scan names which of the machine's directories a compiled output holds, in every spelling, and ignores dependency files",()=>{
 const dir=directory("scan",tmpdir()),real=join(dir,"real");mkdirSync(real);
 writeFileSync(join(dir,"a.rmeta"),Buffer.concat([Buffer.from("xx"),Buffer.from(real+"/src/lib.rs"),Buffer.from("yy")]));writeFileSync(join(dir,"b.rlib"),"clean");writeFileSync(join(dir,"c.d"),real);
 expect(leakedRootKindsV1([join(dir,"c.d"),join(dir,"b.rlib")],{manifest:real})).toBeUndefined();
 const found=leakedRootKindsV1([join(dir,"b.rlib"),join(dir,"a.rmeta")],{manifest:real,cwd:join(dir,"other"),lane:"/lane/x/long"});
 expect(found?.kinds).toEqual(["manifest"]);expect(found?.file).toContain("a.rmeta");
});

async function serve():Promise<RemoteCacheServerV1>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 servers.push(server);return server;
}
function crate(root:string,name:string,manifest:string,source:string,extra:Record<string,string>={}){const dir=join(root,name);mkdirSync(join(dir,"src"),{recursive:true});writeFileSync(join(dir,"Cargo.toml"),manifest);writeFileSync(join(dir,"src",source.includes("fn main()")?"main.rs":"lib.rs"),source);for(const [file,text] of Object.entries(extra))writeFileSync(join(dir,file),text);}
function workspace(root:string):void{
 mkdirSync(root,{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["core","mac","leafr","app"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 const manifest=(name:string,dependencies:string,extra="")=>`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n${extra}[dependencies]\n${dependencies}`;
 crate(root,"core",manifest("core",""),"pub fn id() -> u32 { 1 }\npub fn panics(x: Option<u32>) -> u32 { x.unwrap() }\n");
 crate(root,"mac",manifest("mac","","[lib]\nproc-macro = true\n"),"use proc_macro::TokenStream;\n#[proc_macro]\npub fn here(_: TokenStream) -> TokenStream { format!(\"{:?}\", std::env::var(\"CARGO_MANIFEST_DIR\").unwrap()).parse().unwrap() }\n");
 crate(root,"leafr",manifest("leafr","core = { path = \"../core\" }\nmac = { path = \"../mac\" }\n"),"pub fn id() -> u32 { core::id() }\n#[inline]\npub fn dir() -> &'static str { mac::here!() }\n");
 crate(root,"app",manifest("app","leafr = { path = \"../leafr\" }\ncore = { path = \"../core\" }\n"),"fn main() { println!(\"{} {}\", leafr::id() + core::id(), leafr::dir().len() > 0); }\n");
}
type Machine=Readonly<{name:string;checkout:string;cache:string;cas:string;log:string}>;
function machine(name:string,checkout:string,fresh:boolean):Machine{
 const root=directory(name),cache=join(root,"cache"),cas=join(root,"cas"),log=join(root,"compiles.jsonl");
 if(fresh)workspace(checkout);mkdirSync(cache,{recursive:true});writeFileSync(log,"");return {name,checkout,cache,cas,log};
}
function build(m:Machine,environment:Record<string,string|undefined>):Promise<{code:number;output:string}>{
 const lane="l1",laneRoot=join(m.cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const env={...process.env,...Object.fromEntries(Object.keys(process.env).filter(key=>key.startsWith("SEMIO_REMOTE_CACHE_")||key.startsWith("SEMIO_FLEET_")||key==="SEMIO_WORKSPACE_ROOT"||key==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT").map(key=>[key,undefined])),RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,counter,m.log,"-","0","--","rustc"]),SEMIO_CAS_ROOT:m.cas,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",...environment};
 for(const key of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete (env as Record<string,string|undefined>)[key];
 return new Promise(done=>{const child=spawn(cargo,["build","--manifest-path",join(m.checkout,"Cargo.toml"),"-p","app"],{cwd:m.checkout,env:env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});child.on("close",code=>done({code:code??1,output}));});
}
const remote=(server:{url:string})=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"});
const compiled=(m:Machine)=>readFileSync(m.log,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {crate:string}).map(row=>row.crate).filter(name=>["core","mac","leafr","app"].includes(name)).sort();
const events=(m:Machine)=>{const dir=join(m.cache,"x");return existsSync(dir)?readdirSync(dir).filter(name=>name.endsWith(".jsonl")).flatMap(name=>readFileSync(join(dir,name),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {kind:string;crate:string;reason?:string})):[];};
const hits=(m:Machine)=>events(m).filter(row=>row.kind==="remote-hit").map(row=>row.crate).sort();
async function settle(m:Machine,environment:Record<string,string|undefined>):Promise<void>{
 const before=process.env.SEMIO_CAS_ROOT;process.env.SEMIO_CAS_ROOT=m.cas;
 try{await settleIn(m,environment);}finally{if(before===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=before;}
}
async function settleIn(m:Machine,environment:Record<string,string|undefined>):Promise<void>{
 const deadline=Date.now()+60000;
 while(Date.now()<deadline){const result=await drainUploadsV1(m.cache,{...process.env,...environment,SEMIO_CAS_ROOT:m.cas},{idleMilliseconds:0,takeover:true});if(result!=="busy"&&spoolLengthV1(m.cache)===0&&!existsSync(join(m.cache,fleet.layout.state,"uploader.lock")))return;await new Promise(done=>setTimeout(done,150));}
 throw Error("uploads did not settle");
}
const rlibs=(m:Machine,name:string)=>{const dir=join(m.cache,"l","l1","t","debug","deps");return readdirSync(dir).filter(file=>file.startsWith("lib"+name+"-")&&file.endsWith(".rlib")).map(file=>sha(readFileSync(join(dir,file)).toString("latin1")));};

test("a unit that holds the root is shared among machines at the same root and built by a machine at another; every other unit is shared among all roots and equals a clean local build",async()=>{
 const server=await serve(),environment=remote(server),home=directory("home"),checkout=join(home,"x","ws"),other=join(directory("other"),"y","z","ws");
 const m1=machine("m1",checkout,true);
 const first=await build(m1,environment);expect(first.code,first.output).toBe(0);await settle(m1,environment);
 const cold=compiled(m1);expect(cold).toEqual(["app","core","leafr","mac"]);
 expect(readFallbacksV1(m1.cache)["root-keyed"],"the producer keyed the two units that hold the root (leafr, and app which inlines its string) by the root").toBe(2);
 const m2=machine("m2",checkout,false);
 const second=await build(m2,environment);expect(second.code,second.output).toBe(0);
 expect(compiled(m2),"same root: nothing is compiled").toEqual([]);expect(hits(m2).filter(name=>name==="leafr").length,"the root-keyed unit came from the fleet").toBe(1);
 const m3=machine("m3",other,true);
 const third=await build(m3,environment);expect(third.code,third.output).toBe(0);await settle(m3,environment);
 expect(compiled(m3),"other root: only the unit that holds the root and what depends on it").toEqual(["app","leafr"]);
 expect(hits(m3).filter(name=>name==="core"||name==="mac").length,"the path-independent units are shared among all roots").toBe(2);
 const m4=machine("m4",other,false);
 const fourth=await build(m4,environment);expect(fourth.code,fourth.output).toBe(0);expect(compiled(m4),"second machine at the other root builds nothing").toEqual([]);
 const clean=machine("clean",join(directory("clean"),"q","ws"),true);
 const local=await build(clean,{SEMIO_REMOTE_CACHE_ENDPOINT:undefined});expect(local.code,local.output).toBe(0);
 expect(rlibs(m3,"core"),"a unit installed from the fleet equals a clean local build").toEqual(rlibs(clean,"core"));expect(rlibs(m3,"core").length).toBe(1);
 expect(rlibs(m1,"core")).toEqual(rlibs(clean,"core"));
},600000);

test("a unit that holds the root and was compiled while the cache was unreachable is keyed locally, its hint and unit wait in the spool, and the other machine builds nothing once the cache answers",async()=>{
 const server=await serve(),dead=await serve(),closed=dead.url;await dead.stop();
 const home=directory("home-o"),checkout=join(home,"x","ws"),m1=machine("o1",checkout,true);
 const first=await build(m1,remote({url:closed}));expect(first.code,first.output).toBe(0);
 expect(readFallbacksV1(m1.cache)["root-keyed-deferred"],"the root key is computed without the cache").toBe(2);
 expect(readFallbacksV1(m1.cache)["root-key-failed"],"no unit is given up").toBeUndefined();
 const spool=join(m1.cache,fleet.layout.state,fleet.layout.spool),jobs=readdirSync(spool).filter(name=>name.endsWith(".json")).map(name=>JSON.parse(readFileSync(join(spool,name),"utf8")) as {hint?:{kinds:string[]}});
 expect(jobs.filter(job=>job.hint!==undefined).length,"the units that hold the root carry their hint").toBe(2);
 const live=remote(server);
 for(const name of readdirSync(spool))if(name.endsWith(".json")){const path=join(spool,name),job=JSON.parse(readFileSync(path,"utf8"));writeFileSync(path,JSON.stringify({...job,notBeforeMs:0}));}
 rmSync(join(m1.cache,fleet.layout.state,fleet.layout.down),{force:true});
 const casBefore=process.env.SEMIO_CAS_ROOT;process.env.SEMIO_CAS_ROOT=m1.cas;
 try{await settle(m1,live);}finally{if(casBefore===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=casBefore;}
 const m2=machine("o2",checkout,false),second=await build(m2,live);expect(second.code,second.output).toBe(0);
 expect(compiled(m2),"same root: nothing is compiled").toEqual([]);expect(hits(m2).filter(name=>name==="leafr").length).toBe(1);
},600000);
