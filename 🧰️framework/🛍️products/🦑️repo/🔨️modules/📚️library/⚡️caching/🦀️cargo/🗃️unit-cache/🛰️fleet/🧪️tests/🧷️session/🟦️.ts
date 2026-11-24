import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {spawn} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {cargoScratchBaseV1} from "../../../🧪️tests/🏖️scratch/🟦️.ts";
import {drainUploadsV1,spoolLengthV1} from "../../📤️uploader/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
process.env.SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS="1500";

/** 🧷️ The determinism law of the shared units: a unit that is published to the fleet is always compiled without rustc's incremental session (whose object names differ on every run), so two machines that compile the same workspace under Cargo's incremental default publish byte-identical rlibs and rmetas; a focused unit keeps its session, its local entry is marked `session`, and it is never queued for the fleet, neither after its compile nor after a later local hit. */
const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../../📜️script.ts"),counter=resolve(here,"../🔢️count/🟦️.ts"),fleet=FLEET_POLICY_V1,base=cargoScratchBaseV1(),cargo=process.env.SEMIO_TEST_CARGO??"cargo";
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"ss-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(bytes:Buffer)=>createHash("sha256").update(bytes).digest("hex");
async function serve():Promise<RemoteCacheServerV1>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 servers.push(server);return server;
}
function workspace(root:string):void{
 mkdirSync(root,{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["core","leaf"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 const crate=(name:string,dependencies:string,source:string)=>{mkdirSync(join(root,name,"src"),{recursive:true});writeFileSync(join(root,name,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\n${dependencies}`);writeFileSync(join(root,name,"src","lib.rs"),source);};
 crate("core","",Array.from({length:40},(_,index)=>`pub struct S${index}{pub a:u32}\nimpl S${index}{pub fn f(&self)->u32{let c=|y:u32|y+self.a;c(${index})}}\n`).join(""));
 crate("leaf",'core = { path = "../core" }\n',"pub fn f() -> u32 { core::S0 { a: 1 }.f() }\n");
}
type Machine=Readonly<{checkout:string;cache:string;cas:string;log:string}>;
function machine(name:string,fresh:boolean,checkout?:string):Machine{
 const root=directory(name),cache=join(root,"cache"),cas=join(root,"cas"),log=join(root,"compiles.jsonl"),where=checkout??join(root,"ws");
 if(fresh)workspace(where);mkdirSync(cache,{recursive:true});writeFileSync(log,"");return {checkout:where,cache,cas,log};
}
function build(m:Machine,environment:Record<string,string|undefined>):Promise<{code:number;output:string}>{
 const lane="l1",laneRoot=join(m.cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const env={...process.env,...Object.fromEntries(Object.keys(process.env).filter(key=>key.startsWith("SEMIO_REMOTE_CACHE_")||key.startsWith("SEMIO_FLEET_")||key==="SEMIO_WORKSPACE_ROOT"||key==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT"||key==="SEMIO_UNIT_CACHE_FOCUS").map(key=>[key,undefined])),RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,counter,m.log,"-","0","--","rustc"]),SEMIO_CAS_ROOT:m.cas,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"1",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",...environment};
 for(const key of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete (env as Record<string,string|undefined>)[key];
 return new Promise(done=>{const child=spawn(cargo,["build","--manifest-path",join(m.checkout,"Cargo.toml"),"-p","leaf"],{cwd:m.checkout,env:env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});child.on("close",code=>done({code:code??1,output}));});
}
const remote=(server:{url:string})=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"});
const events=(m:Machine)=>{const dir=join(m.cache,"x");return existsSync(dir)?readdirSync(dir).filter(name=>name.endsWith(".jsonl")).flatMap(name=>readFileSync(join(dir,name),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as {kind:string;crate:string;reason?:string})):[];};
async function settle(m:Machine,environment:Record<string,string|undefined>):Promise<void>{
 const deadline=Date.now()+60000;
 while(Date.now()<deadline){const result=await drainUploadsV1(m.cache,{...process.env,...environment,SEMIO_CAS_ROOT:m.cas},{idleMilliseconds:0});if(result!=="busy"&&spoolLengthV1(m.cache)===0&&!existsSync(join(m.cache,fleet.layout.state,"uploader.lock")))return;await new Promise(done=>setTimeout(done,150));}
 throw Error("uploads did not settle");
}
const deps=(m:Machine)=>join(m.cache,"l","l1","t","debug","deps");
const outputs=(m:Machine,name:string,extension:string)=>readdirSync(deps(m)).filter(file=>file.startsWith("lib"+name+"-")&&file.endsWith("."+extension)).map(file=>sha(readFileSync(join(deps(m),file))));
const sessions=(m:Machine)=>{const root=join(m.cache,"l","l1","t","debug","incremental");return existsSync(root)?readdirSync(root).map(name=>name.replace(/-[^-]+$/u,"")):[];};
const spooled=(m:Machine)=>{const spool=join(m.cache,fleet.layout.state,fleet.layout.spool);return existsSync(spool)?readdirSync(spool).filter(name=>name.endsWith(".json")).map(name=>(JSON.parse(readFileSync(join(spool,name),"utf8")) as {crate:string}).crate):[];};
const entries=(m:Machine,crate:string)=>{const out:{session?:true;crate:string}[]=[];const walk=(directory:string):void=>{for(const item of readdirSync(directory,{withFileTypes:true})){const path=join(directory,item.name);if(item.isDirectory())walk(path);else if(item.name.endsWith(".json")){try{const row=JSON.parse(readFileSync(path,"utf8")) as {crate?:string;outputs?:unknown};if(row.crate===crate&&row.outputs)out.push(row as {crate:string});}catch{}}}};const root=join(m.cache,"e");if(existsSync(root))walk(root);return out;};

test("two machines that compile the same workspace under Cargo's incremental default publish byte-identical rlibs and rmetas, and no shared unit keeps a session",async()=>{
 const a=machine("a",true),b=machine("b",true),one=await serve(),two=await serve();
 const first=await build(a,remote(one));expect(first.code,first.output).toBe(0);await settle(a,remote(one));
 const second=await build(b,remote(two));expect(second.code,second.output).toBe(0);await settle(b,remote(two));
 for(const name of["core","leaf"]){
  expect(outputs(a,name,"rlib").length,name).toBe(1);expect(outputs(b,name,"rlib")).toEqual(outputs(a,name,"rlib"));
  expect(outputs(a,name,"rmeta").length,name).toBe(1);expect(outputs(b,name,"rmeta")).toEqual(outputs(a,name,"rmeta"));
 }
 expect(sessions(a)).toEqual([]);expect(sessions(b)).toEqual([]);
 expect(events(a).filter(row=>row.reason?.startsWith("incremental")).length).toBe(0);
 expect(entries(a,"core").every(row=>row.session===undefined)).toBe(true);
},600000);

test("a focused unit keeps its incremental session, its entry is marked and it never reaches the fleet, not after the compile and not after a later local hit; the unit it depends on is shared",async()=>{
 const server=await serve(),environment={...remote(server),SEMIO_UNIT_CACHE_FOCUS:"leaf"},m=machine("c",true);
 const first=await build(m,environment);expect(first.code,first.output).toBe(0);
 expect(sessions(m).sort(),"only the focused unit has a session").toEqual(["leaf"]);
 expect(events(m).filter(row=>row.kind==="remote-skip"&&row.crate==="leaf"&&row.reason?.startsWith("incremental")).length).toBe(1);
 expect(entries(m,"leaf").every(row=>row.session===true)&&entries(m,"leaf").length>0).toBe(true);
 expect(entries(m,"core").every(row=>row.session===undefined)).toBe(true);
 await settle(m,environment);
 rmSync(join(m.cache,"l","l1","t"),{recursive:true,force:true});
 const second=await build(m,{...remote(server)});expect(second.code,second.output).toBe(0);await settle(m,remote(server));
 expect(spooled(m),"the offer after the local hit leaves the focused unit out").not.toContain("leaf");
 expect(events(m).filter(row=>row.kind==="hit"&&row.crate==="leaf").length).toBe(1);
 const probe=machine("d",false,m.checkout),third=await build(probe,remote(server));expect(third.code,third.output).toBe(0);
 expect(events(probe).filter(row=>row.crate==="leaf"&&row.kind==="remote-hit").length,"the fleet never received the focused unit").toBe(0);
 expect(events(probe).filter(row=>row.crate==="core"&&row.kind==="remote-hit").length,"the dependency was shared").toBe(1);
},600000);
