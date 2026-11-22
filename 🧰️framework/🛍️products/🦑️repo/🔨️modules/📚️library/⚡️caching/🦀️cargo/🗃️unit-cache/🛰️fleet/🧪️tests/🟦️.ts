import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {createHash} from "node:crypto";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {parseRemoteCacheActionResultV1} from "../../../../../🏗️build/🌐️remote-cache/🔬️action-result/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {cargoScratchBaseV1} from "../../🧪️tests/🏖️scratch/🟦️.ts";
import {drainUploadsV1,spoolLengthV1} from "../📤️uploader/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../📡️client/🟦️.ts";
import {obtainFleetUnitV1,type FleetUnitV1} from "../🟦️.ts";
import {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";
import {decodeActionResultV1,encodeActionResultV1,parseRemoteUnitV1,readUnitSlotV1,wireDigestOfV1} from "../📦️wire/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import unitSchema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 🛰️ The fleet tier is proven end to end on one machine: two scratch checkouts at different absolute paths act as two machines against one real cache server over real sockets. Machine 1 builds cold and uploads, machine 2 builds cold and compiles nothing, two machines racing compile every unit exactly once in total, and a server that is down, denies, stalls or serves a corrupt blob never fails or hangs a build. Real compiles are counted by a shim in front of rustc. */
const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../📜️script.ts"),counter=resolve(here,"🔢️count/🟦️.ts"),fleet=FLEET_POLICY_V1,base=cargoScratchBaseV1(),cargo=process.env.SEMIO_TEST_CARGO??"cargo";
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"fl-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});

async function serve(options:{tokens?:{read?:string;write?:string}}={}):Promise<RemoteCacheServerV1>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:options.tokens??{write:"w-token"}});
 servers.push(server);return server;
}

function crate(root:string,name:string,manifest:string,source:string,extra:Record<string,string>={}){const dir=join(root,name);mkdirSync(join(dir,"src"),{recursive:true});writeFileSync(join(dir,"Cargo.toml"),manifest);writeFileSync(join(dir,"src",source.includes("fn main()")?"main.rs":"lib.rs"),source);for(const [file,text] of Object.entries(extra))writeFileSync(join(dir,file),text);}
function workspace(root:string):void{
 mkdirSync(root,{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["core","mac","leaf1","leaf2","app"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 const manifest=(name:string,dependencies:string,extra="")=>`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n${extra}[dependencies]\n${dependencies}`;
 crate(root,"core",manifest("core",""),"pub fn id() -> u32 { 1 }\n");
 crate(root,"mac",manifest("mac","","[lib]\nproc-macro = true\n"),"use proc_macro::TokenStream;\n#[proc_macro]\npub fn five(_: TokenStream) -> TokenStream { \"5\".parse().unwrap() }\n");
 for(const [index,name] of["leaf1","leaf2"].entries())crate(root,name,manifest(name,"core = { path = \"../core\" }\nmac = { path = \"../mac\" }\n","build = \"build.rs\"\n"),"pub fn f() -> u32 { core::id() + mac::five!() + env!(\"LEAF_ID\").parse::<u32>().unwrap() }\n",{"build.rs":`fn main() { println!("cargo:rustc-env=LEAF_ID=${index+1}"); }\n`});
 crate(root,"app",manifest("app","leaf1 = { path = \"../leaf1\" }\nleaf2 = { path = \"../leaf2\" }\n"),"fn main() { println!(\"{}\", leaf1::f() + leaf2::f()); }\n");
}

type Machine=Readonly<{name:string;checkout:string;cache:string;cas:string;log:string}>;
function machine(name:string,depth:string):Machine{
 const root=directory(name),checkout=join(root,depth,"ws"),cache=join(root,"cache"),cas=join(root,"cas"),log=join(root,"compiles.jsonl");
 workspace(checkout);mkdirSync(cache,{recursive:true});writeFileSync(log,"");return {name,checkout,cache,cas,log};
}

type Session=Readonly<{code:number;output:string;milliseconds:number}>;
function build(m:Machine,environment:Record<string,string|undefined>,args:string[]=[],options:{delayCrate?:string;delayMs?:number;lane?:string}={}):Promise<Session>{
 const lane=options.lane??"l1",laneRoot=join(m.cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const env={...process.env,...Object.fromEntries(Object.keys(process.env).filter(key=>key.startsWith("SEMIO_REMOTE_CACHE_")||key.startsWith("SEMIO_FLEET_")||key==="SEMIO_WORKSPACE_ROOT"||key==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT").map(key=>[key,undefined])),RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify([process.execPath,counter,m.log,options.delayCrate??"-",String(options.delayMs??0),"--","rustc"]),SEMIO_CAS_ROOT:m.cas,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:"nightly-2026-07-20",...environment};
 for(const key of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete (env as Record<string,string|undefined>)[key];
 return new Promise(done=>{const started=Date.now(),child=spawn(cargo,["build","--manifest-path",join(m.checkout,"Cargo.toml"),...args],{cwd:m.checkout,env:env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});child.on("close",code=>done({code:code??1,output,milliseconds:Date.now()-started}));});
}
const remote=(server:{url:string},token="w-token",access="read-write")=>({SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:access,SEMIO_REMOTE_CACHE_TOKEN:token});

type Compile={crate:string;unit:string;pid:number;at:number};
const compiles=(m:Machine):Compile[]=>readFileSync(m.log,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as Compile);
type Event={kind:string;crate:string;key:string|null;reason?:string;waitedMs:number};
const events=(m:Machine,lane?:string):Event[]=>{const dir=join(m.cache,"x");return existsSync(dir)?readdirSync(dir).filter(name=>name.endsWith(".jsonl")&&(!lane||name===lane+".jsonl")).flatMap(name=>readFileSync(join(dir,name),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as Event)):[];};
const count=(rows:readonly {kind:string}[],kind:string)=>rows.filter(row=>row.kind===kind).length;
const workspaceCompiles=(m:Machine)=>compiles(m).filter(row=>["core","mac","leaf1","leaf2","app"].includes(row.crate)||row.crate.startsWith("build_script_"));

async function settled(m:Machine,environment:Record<string,string|undefined>):Promise<void>{
 const deadline=Date.now()+60000;
 while(Date.now()<deadline){
  const result=await drainUploadsV1(m.cache,{...process.env,...environment,SEMIO_CAS_ROOT:m.cas},{idleMilliseconds:0});
  if(result!=="busy"&&spoolLengthV1(m.cache)===0)return;
  await new Promise(done=>setTimeout(done,150));
 }
 throw Error("uploads did not settle");
}
const waitForUploader=async(m:Machine)=>{const deadline=Date.now()+60000;while(Date.now()<deadline){if(spoolLengthV1(m.cache)===0&&!existsSync(join(m.cache,fleet.layout.state,"uploader.lock")))return;await new Promise(done=>setTimeout(done,200));}throw Error("the upload worker did not finish");};

test("the schema is closed and the manifest, platform, spool job and counters validate against it",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);
 const validator=(name:string)=>{const validate=ajv.getSchema(`${schema.$id}#/definitions/${name}`);if(!validate)throw Error(name);return validate;};
 const platform={version:1,os:"darwin",arch:"arm64",host:"aarch64-apple-darwin",rustc:"a".repeat(64),toolchain:"t",linker:"darwin-27",cpu:""};
 expect(validator("Platform")(platform)).toBe(true);expect(validator("Platform")({...platform,extra:1})).toBe(false);
 const unit={version:1,key:"b".repeat(64),localKey:"c".repeat(64),slot:"d".repeat(64),crate:"core",platform,createdMs:1,costMs:2,dependencies:[{path:"src/lib.rs",sha256:"e".repeat(64)}],environment:[{name:"CARGO_PKG_NAME",value:"core"}],outputs:[{name:"libcore.rlib",sha256:"f".repeat(64),bytes:3,executable:false}],stdout:null,stderr:null};
 expect(validator("RemoteUnit")(unit)).toBe(true);expect(validator("RemoteUnit")({...unit,outputs:[]})).toBe(false);
 expect(parseRemoteUnitV1(new TextEncoder().encode(JSON.stringify(unit)),{key:unit.key}).crate).toBe("core");
 expect(()=>parseRemoteUnitV1(new TextEncoder().encode(JSON.stringify(unit)),{key:"0".repeat(64)})).toThrow();
 expect(()=>parseRemoteUnitV1(new TextEncoder().encode(JSON.stringify({...unit,outputs:[{...unit.outputs[0],name:"../x"}]})),{key:unit.key})).toThrow();
 const job={version:1,key:unit.key,localKey:unit.localKey,slot:unit.slot,crate:"core",cwd:"/w",workspace:null,laneRoot:null,owner:"m:1",claimed:false,costMs:1,platform,queuedMs:1};
 expect(validator("SpoolJob")(job)).toBe(true);expect(validator("SpoolJob")({...job,owner:undefined})).toBe(false);
 const counters={version:1,wallMs:1,unit:{localHit:1,remoteHit:2,miss:3,stored:4},offload:{out:0,refused:0,fellBack:0}};
 expect(validator("Counters")(counters)).toBe(true);expect(validator("Counters")({...counters,unit:{localHit:1}})).toBe(false);
 expect(validator("Policy")(fleet)).toBe(true);
 const entryEvent=new Ajv({strict:false});entryEvent.addSchema(unitSchema);
 const kinds=(unitSchema.definitions.Event.properties.kind.enum as string[]);for(const kind of["remote-hit","remote-miss","remote-claim","remote-wait","remote-publish","remote-skip"])expect(kinds).toContain(kind);
});

const python=testPythonV1(),oracle=resolve(here,"../../../../../🏗️build/🌐️remote-cache/🔬️action-result/🧪️tests/🐍️oracle/🐍️.py");
test("an encoded action result is read back, accepted by the server's strict decoder and by an independent Python decoder, and carries its slot",()=>{
 const files=[{path:"unit.json",digest:wireDigestOfV1(new TextEncoder().encode("{}")),executable:false},{path:"out/libx.rlib",digest:wireDigestOfV1(new Uint8Array([1,2,3])),executable:true},{path:"out/empty",digest:wireDigestOfV1(new Uint8Array(0)),executable:false}],slot="9".repeat(64);
 const stdout=wireDigestOfV1(new TextEncoder().encode("out")),bytes=encodeActionResultV1({files,stdout,slot});
 const decoded=decodeActionResultV1(bytes);
 expect(decoded.files).toEqual(files);expect(decoded.stdout).toEqual(stdout);expect(decoded.slot).toBe(slot);expect(decoded.worker).toBe(fleet.worker);expect(readUnitSlotV1(bytes)).toBe(slot);
 const parsed=parseRemoteCacheActionResultV1(bytes);expect(parsed.references.map(row=>row.hash).sort()).toEqual([files[0]!.digest.hash,files[1]!.digest.hash,stdout.hash].sort());
 if(python){
  const run=Bun.spawnSync([python,"-I",oracle,"decode"],{stdin:new TextEncoder().encode(JSON.stringify([{kind:"result",hex:Buffer.from(bytes).toString("hex")},{kind:"result",hex:Buffer.from(bytes.subarray(0,bytes.length-3)).toString("hex")}])),stdout:"pipe",stderr:"pipe"});
  expect(run.exitCode,run.stderr.toString()).toBe(0);const verdicts=JSON.parse(run.stdout.toString()) as {valid:boolean}[];expect(verdicts[0]!.valid).toBe(true);expect(verdicts[1]!.valid).toBe(false);
 }
 expect(readUnitSlotV1(bytes.subarray(0,bytes.length-3))).toBeUndefined();
});

test("two machines at different paths: the first builds cold and uploads, the second builds cold and compiles nothing and runs the same program",async()=>{
 const server=await serve(),m1=machine("m1","a"),m2=machine("m2","b/c/d/e"),environment=remote(server);
 expect(m1.checkout).not.toBe(m2.checkout);
 const first=await build(m1,environment,["-p","app"]);expect(first.code,first.output).toBe(0);
 await waitForUploader(m1);
 const cold=events(m1);expect(count(cold,"miss")).toBeGreaterThanOrEqual(7);expect(count(cold,"remote-miss")).toBe(count(cold,"miss"));expect(count(cold,"remote-publish")).toBe(count(cold,"miss"));
 const compiledFirst=workspaceCompiles(m1).length;expect(compiledFirst).toBeGreaterThanOrEqual(7);
 const second=await build(m2,environment,["-p","app"]);expect(second.code,second.output).toBe(0);
 const warm=events(m2);expect(count(warm,"miss"),JSON.stringify(warm.map(row=>[row.kind,row.crate,row.reason]))).toBe(0);expect(count(warm,"remote-hit")).toBe(count(cold,"miss"));
 expect(workspaceCompiles(m2),"machine 2 spawned the compiler for a unit").toEqual([]);
 const program=(m:Machine)=>spawnSync(join(m.cache,"l","l1","t","debug","app"),{encoding:"utf8"}).stdout.trim();
 expect(program(m1)).toBe("15");expect(program(m2)).toBe("15");
 const crates=(rows:Event[],kind:string)=>rows.filter(row=>row.kind===kind).map(row=>row.crate).sort();
 expect(crates(warm,"remote-hit")).toEqual(crates(cold,"miss"));
 const localKeys=(rows:Event[],kind:string)=>new Set(rows.filter(row=>row.kind===kind).map(row=>row.key));
 expect([...localKeys(warm,"remote-hit")].filter(key=>localKeys(cold,"miss").has(key)).length,"a unit without a real path has one local key in every checkout").toBeGreaterThan(0);
 const status=await (await fetch(server.url+"/status",{headers:{authorization:"Bearer w-token"}})).json() as {counters:{ac:{puts:number;hits:number}};claims:{active:number}};
 expect(status.counters.ac.puts).toBe(count(cold,"miss"));expect(status.counters.ac.hits).toBeGreaterThanOrEqual(count(cold,"miss"));expect(status.claims.active).toBe(0);
 const entries=readdirSync(join(m2.cache,"e")).flatMap(shard=>readdirSync(join(m2.cache,"e",shard)).flatMap(key=>readdirSync(join(m2.cache,"e",shard,key)).map(file=>JSON.parse(readFileSync(join(m2.cache,"e",shard,key,file),"utf8")) as {slot?:string})));
 expect(entries.length).toBe(count(cold,"miss"));for(const entry of entries)expect(entry.slot).toMatch(/^[a-f0-9]{64}$/u);
 const edited=join(m2.checkout,"leaf2","src","lib.rs");writeFileSync(edited,readFileSync(edited,"utf8")+"pub fn extra() -> u32 { 2 }\n");
 const third=await build(m2,environment,["-p","app"]);expect(third.code,third.output).toBe(0);
 expect(workspaceCompiles(m2).map(row=>row.crate).sort()).toEqual(["app","leaf2"]);
},300000);

test("two machines start the same cold build at the same time: every unit is compiled exactly once in total",async()=>{
 const server=await serve(),m1=machine("r1","a"),m2=machine("r2","b/deeper"),environment=remote(server);
 const [first,second]=await Promise.all([build(m1,environment,["-p","app"],{delayCrate:"core",delayMs:2500}),build(m2,environment,["-p","app"],{delayCrate:"core",delayMs:2500})]);
 expect(first.code,first.output).toBe(0);expect(second.code,second.output).toBe(0);
 await Promise.all([waitForUploader(m1),waitForUploader(m2)]);
 const all=[...workspaceCompiles(m1),...workspaceCompiles(m2)],byCrate=new Map<string,number>();
 for(const row of all)byCrate.set(row.unit,(byCrate.get(row.unit)??0)+1);
 for(const [name,times] of byCrate)expect(times,`${name} compiled ${times} times in total`).toBe(1);
 expect([...byCrate.keys()].map(unit=>unit.split("/")[0]!).sort()).toEqual(["app","build_script_build","build_script_build","core","leaf1","leaf2","mac"]);
 const waited=[...events(m1),...events(m2)];expect(count(waited,"remote-hit")).toBeGreaterThanOrEqual(4);
 expect(waited.some(row=>row.kind==="remote-hit"&&row.waitedMs>0)).toBe(true);
 const program=(m:Machine)=>spawnSync(join(m.cache,"l","l1","t","debug","app"),{encoding:"utf8"}).stdout.trim();
 expect(program(m1)).toBe("15");expect(program(m2)).toBe("15");
},300000);

test("a server that is down, denies the credential or only reads never fails the build and costs a bounded time",async()=>{
 const server=await serve({tokens:{read:"r-token",write:"w-token"}}),dead=await serve();const closed=dead.url;await dead.stop();
 const plain=machine("p0","a");const base0=await build(plain,{},["-p","app"]);expect(base0.code,base0.output).toBe(0);
 const down=machine("p1","a"),stopped=await build(down,remote({url:closed}),["-p","app"]);expect(stopped.code,stopped.output).toBe(0);
 expect(workspaceCompiles(down).length).toBe(workspaceCompiles(plain).length);expect(statSync(join(down.cache,fleet.layout.state,fleet.layout.down)).isFile()).toBe(true);
 expect(stopped.milliseconds).toBeLessThan(base0.milliseconds+8000);
 const denied=machine("p2","a"),refused=await build(denied,remote(server,"wrong-token"),["-p","app"]);expect(refused.code,refused.output).toBe(0);
 expect(statSync(join(denied.cache,fleet.layout.state,fleet.layout.denied)).isFile()).toBe(true);expect(count(events(denied),"remote-publish")).toBe(0);
 const writer=machine("p3","a"),wrote=await build(writer,remote(server),["-p","app"]);expect(wrote.code,wrote.output).toBe(0);await waitForUploader(writer);
 const reader=machine("p4","b/c"),readOnly=await build(reader,remote(server,"r-token","read-only"),["-p","app"]);expect(readOnly.code,readOnly.output).toBe(0);
 expect(workspaceCompiles(reader),"a read-only machine still downloads").toEqual([]);expect(spoolLengthV1(reader.cache)).toBe(0);
 const another=machine("p5","a"),readerCompiled=await build(another,remote(server,"r-token","read-only"),["-p","app"],{lane:"l2"});expect(readerCompiled.code).toBe(0);
},300000);

test("a blob that is corrupt or missing on the server turns that unit into a miss; everything else is still served",async()=>{
 const server=await serve(),m1=machine("c1","a"),m2=machine("c2","b/c"),environment=remote(server);
 expect((await build(m1,environment,["-p","app"])).code).toBe(0);await waitForUploader(m1);
 const stored=await (async()=>{const rows:string[]=[];const walk=(dir:string)=>{for(const entry of readdirSync(dir,{withFileTypes:true})){const path=join(dir,entry.name);if(entry.isDirectory())walk(path);else rows.push(path);}};walk(resolve(scratch.filter(path=>path.includes("fl-srv-")).at(-1)!));return rows;})();
 const victims=stored.filter(path=>path.includes(`${"/"}cas${"/"}`)&&!path.includes(".tmp"));expect(victims.length).toBeGreaterThan(3);
 const victim=[...victims].sort((left,right)=>statSync(right).size-statSync(left).size)[0]!,original=readFileSync(victim);writeFileSync(victim,Buffer.concat([original.subarray(0,original.length-1),Buffer.from([(original[original.length-1]!+1)&255])]));
 const second=await build(m2,environment,["-p","app"]);expect(second.code,second.output).toBe(0);
 const compiled=workspaceCompiles(m2);expect(compiled.length,JSON.stringify([victim,original.length,events(m2).map(row=>[row.kind,row.crate,row.reason])])).toBeGreaterThanOrEqual(1);expect(compiled.length).toBeLessThan(workspaceCompiles(m1).length);
 const program=spawnSync(join(m2.cache,"l","l1","t","debug","app"),{encoding:"utf8"}).stdout.trim();expect(program).toBe("15");
},300000);

function fakeUnit(server:{url:string},key:string,owner:string):FleetUnitV1{
 const settings=readFleetSettingsV1(directory("fake"),{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"})!;
 return {client:createFleetClientV1(settings),root:settings.stateDirectory,environment:{},localKey:"1".repeat(64),key,slot:"2".repeat(64),slotParts:[],crate:"fake",platform:{version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""},roots:[],cwd:"/w",workspace:undefined,laneRoot:undefined,owner,expected:[]};
}
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");

test("claims: a holder that dies is replaced after its TTL, a holder that is too slow is overtaken, a waiter wakes when the entry arrives",async()=>{
 const server=await serve(),noise=()=>{};
 const key=sha("dead holder"),ghost=fakeUnit(server,key,"ghost:1:aa");
 expect((await ghost.client.claim(key,"ghost:1:aa",1200,60)).state).toBe("granted");
 const waiter=fakeUnit(server,key,"waiter:2:bb"),started=Date.now(),decision=await obtainFleetUnitV1(waiter,noise,{minimumWaitMilliseconds:20000});
 expect(decision.outcome).toBe("compile");expect(Date.now()-started).toBeGreaterThanOrEqual(900);expect(Date.now()-started).toBeLessThan(10000);
 if(decision.outcome==="compile"){expect(decision.hold).toBeDefined();await decision.hold!.release();}
 const slowKey=sha("slow holder"),slow=fakeUnit(server,slowKey,"slow:1:aa");
 expect((await slow.client.claim(slowKey,"slow:1:aa",60000,50)).state).toBe("granted");
 const impatient=fakeUnit(server,slowKey,"impatient:3:cc"),began=Date.now(),overtaken=await obtainFleetUnitV1(impatient,noise,{minimumWaitMilliseconds:400,waitFactor:1,defaultExpectedMilliseconds:100});
 expect(overtaken.outcome).toBe("compile");expect(overtaken.outcome==="compile"&&overtaken.hold).toBeUndefined();expect(Date.now()-began).toBeGreaterThanOrEqual(350);expect(Date.now()-began).toBeLessThan(6000);
 const lateKey=sha("late"),late=fakeUnit(server,lateKey,"late:1:aa");
 expect((await late.client.claim(lateKey,"late:1:aa",30000)).state).toBe("granted");
 const watcher=obtainFleetUnitV1(fakeUnit(server,lateKey,"watch:4:dd"),noise,{minimumWaitMilliseconds:20000});
 await new Promise(done=>setTimeout(done,150));
 const status=(await (await fetch(server.url+"/status",{headers:{authorization:"Bearer w-token"}})).json() as {claims:{waiting:number}}).claims;expect(status.waiting).toBe(1);
 await late.client.release(lateKey,"late:1:aa");
 const answer=await watcher;expect(answer.outcome).toBe("compile");if(answer.outcome==="compile")await answer.hold?.release();
},60000);

test("the counters are one byte per decision, safe across processes, and read as package D's load reporter wants them",async()=>{
 const {countUnitV1,readBuildCountersV1,writeBuildCountersV1}=await import("../📊️counters/🟦️.ts");
 const workspace=directory("counters"),ajv=new Ajv({strict:false});ajv.addSchema(schema);
 const children=Array.from({length:8},()=>Bun.spawn([process.execPath,"-e",`const m=await import(${JSON.stringify(resolve(here,"../📊️counters/🟦️.ts"))});for(let i=0;i<50;i++){m.countUnitV1(${JSON.stringify(workspace)},"localHit");if(i%5===0)m.countUnitV1(${JSON.stringify(workspace)},"miss");}`]));
 for(const child of children)expect(await child.exited).toBe(0);
 countUnitV1(workspace,"remoteHit");countUnitV1(undefined,"stored");
 const counters=readBuildCountersV1(workspace);expect(counters.unit).toEqual({localHit:400,remoteHit:1,miss:80,stored:0,skipped:0});
 expect(ajv.getSchema(`${schema.$id}#/definitions/Counters`)!(counters)).toBe(true);
 writeBuildCountersV1(workspace);expect(JSON.parse(readFileSync(join(workspace,".cache","f","build-counters.json"),"utf8")).unit.localHit).toBe(400);
});

test("a fleet that fails once while it was reachable is ridden through in a process that has nothing else to wait for, never ending it silently",async()=>{
 const state=directory("ridethrough"),client=resolve(here,"../📡️client/🟦️.ts");
 writeFileSync(join(state,fleet.layout.reachable),"");
 const program=`const m=await import(${JSON.stringify(client)});let calls=0;const settings={endpoint:"http://127.0.0.1:1",instance:"main",token:undefined,upload:false,stateDirectory:${JSON.stringify(state)},signedRoot:undefined,identity:""};const client=m.createFleetClientV1(settings,{fetch:async()=>{if(++calls===1)throw new Error("down");return new Response("",{status:404});}});process.exitCode=70;client.getResult("a".repeat(64)).then(answer=>{console.log(JSON.stringify({answer,calls}));process.exitCode=0;});`;
 const child=spawnSync(process.execPath,["-e",program],{encoding:"utf8",timeout:60000});
 expect(child.stdout.trim(),child.stderr).toBe(JSON.stringify({answer:"miss",calls:2}));expect(child.status,child.stderr).toBe(0);
});

test("a wrapper process that ends before its work settled fails instead of reporting success",()=>{
 const program=`import {UNFINISHED_EXIT_STATUS_V1} from ${JSON.stringify(script)};process.exitCode=UNFINISHED_EXIT_STATUS_V1;new Promise(()=>{}).then(()=>{process.exitCode=0;});`;
 const child=spawnSync(process.execPath,["-e",program],{encoding:"utf8",timeout:60000});
 expect(child.status,child.stderr).toBe(70);
});

test("a remote hit claims its key as current for its slot at the server (POST /v1/slots)",async()=>{
 const seen:string[]=[],stub=Bun.serve({port:0,hostname:"127.0.0.1",fetch:async request=>{seen.push(request.method+" "+new URL(request.url).pathname+" "+(request.method==="POST"?await request.text():""));return Response.json({claimed:1,missing:0});}});
 try{
  const settings=readFleetSettingsV1(directory("slotclaim"),{SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${stub.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"t"})!;
  expect(await createFleetClientV1(settings).claimSlot("a".repeat(64),"b".repeat(64))).toBe(true);
  expect(seen).toEqual([`POST /main/v1/slots {"entries":[{"kind":"ac","key":"${"a".repeat(64)}","slot":"${"b".repeat(64)}"}]}`]);
 }finally{stub.stop(true);}
});

test("a machine that built before seeds the fleet cache from its local hits, once, and a second machine then compiles nothing",async()=>{
 const server=await serve(),m1=machine("o1","a"),m2=machine("o2","b/c"),off={SEMIO_REMOTE_CACHE_ENDPOINT:undefined},environment=remote(server);
 expect((await build(m1,off,["-p","app"])).code).toBe(0);
 expect(spoolLengthV1(m1.cache)).toBe(0);
 expect((await build(m1,environment,["-p","app"],{lane:"l2"})).code).toBe(0);
 await waitForUploader(m1);
 const status=await (await fetch(server.url+"/status",{headers:{authorization:"Bearer w-token"}})).json() as {counters:{ac:{puts:number}}};expect(status.counters.ac.puts).toBe(7);
 expect((await build(m1,environment,["-p","app"],{lane:"l3"})).code).toBe(0);expect(spoolLengthV1(m1.cache)).toBe(0);
 expect((await build(m2,environment,["-p","app"])).code).toBe(0);expect(workspaceCompiles(m2)).toEqual([]);
},300000);

test("a requester whose compiler build differs from the fleet's gets no hit and uploads under its own keys; a machine with the fleet's build still compiles nothing, and the claim table is empty afterwards",async()=>{
 const server=await serve(),m1=machine("t1","a"),m2=machine("t2","b/c"),m3=machine("t3","d"),environment=remote(server);
 const first=await build(m1,environment,["-p","app"]);expect(first.code,first.output).toBe(0);await waitForUploader(m1);
 const built=count(events(m1),"miss");expect(built).toBeGreaterThanOrEqual(7);
 const other=await build(m2,{...environment,SEMIO_UNIT_CACHE_TOOLCHAIN:"another-rustc-build"},["-p","app"]);expect(other.code,other.output).toBe(0);await waitForUploader(m2);
 const foreign=events(m2);expect(count(foreign,"remote-hit"),"no unit of another compiler build is taken").toBe(0);expect(count(foreign,"miss")).toBe(built);expect(count(foreign,"remote-publish")).toBe(built);
 expect(workspaceCompiles(m2).length,"it compiled every unit itself").toBeGreaterThanOrEqual(7);
 const same=await build(m3,environment,["-p","app"]);expect(same.code,same.output).toBe(0);
 const warm=events(m3);expect(count(warm,"miss")).toBe(0);expect(count(warm,"remote-hit")).toBe(built);expect(workspaceCompiles(m3)).toEqual([]);
 const status=await (await fetch(server.url+"/status",{headers:{authorization:"Bearer w-token"}})).json() as {counters:{ac:{puts:number}};claims:{active:number}};
 expect(status.counters.ac.puts,"two builds, two key spaces").toBe(2*built);expect(status.claims.active).toBe(0);
},600000);

test("with signed access the client takes its fetch from the remote cache module and a request without a usable signer still reaches an open server",async()=>{
 const seen:string[]=[],stub=Bun.serve({port:0,hostname:"127.0.0.1",fetch:request=>{seen.push(request.method+" "+new URL(request.url).pathname+" "+(request.headers.get("authorization")?"auth":"plain"));return new Response("Stored");}});
 try{
  const root=directory("signed"),settings=readFleetSettingsV1(root,{SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${stub.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_SIGNED:"1",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:root})!;
  expect(settings.signedRoot).toBe(root);
  expect(await createFleetClientV1(settings).putBlob("a".repeat(64),new Uint8Array([1]))).toBe(true);
  expect(seen).toEqual(["PUT /main/cas/"+"a".repeat(64)+" plain"]);
 }finally{stub.stop(true);}
});

test("a refusal is re-checked as soon as the credential changes, a unit that skips the lookup is counted and logged, and a storage fault while publishing is logged with its reason and does not fail the build",async()=>{
 const server=await serve({tokens:{write:"w-token"}}),m=machine("d1","a"),workspaceRoot=directory("dwork");
 const first=await build(m,{...remote(server,"wrong-token"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspaceRoot},["-p","app"]);expect(first.code,first.output).toBe(0);
 expect(statSync(join(m.cache,fleet.layout.state,fleet.layout.denied)).isFile()).toBe(true);
 const skipped=events(m).filter(row=>row.kind==="remote-skip");expect(skipped.some(row=>(row.reason??"").startsWith("breaker: denied"))).toBe(true);
 expect(spoolLengthV1(m.cache),"the compiled units wait in the spool, none is dropped").toBeGreaterThan(0);expect(existsSync(join(workspaceRoot,".cache","f","unit-counters","skipped")),"waiting is no skip").toBe(false);
 const before=count(events(m),"remote-publish");
 const second=await build(m,{...remote(server,"w-token"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspaceRoot},["-p","app"],{lane:"l2"});expect(second.code,second.output).toBe(0);
 const casBefore=process.env.SEMIO_CAS_ROOT;process.env.SEMIO_CAS_ROOT=m.cas;
 try{await settled(m,{...remote(server,"w-token"),SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspaceRoot});}finally{if(casBefore===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=casBefore;}expect(count(events(m),"remote-publish"),"the right credential is used at once, not after the window").toBeGreaterThan(before);
 const fault=join(directory("blocked"),"file");writeFileSync(fault,"x");const faulty=machine("d3","a");
 const casFault=await build(faulty,{SEMIO_CAS_ROOT:fault},["-p","app"]);expect(casFault.code,casFault.output).toBe(0);
 const skips=events(faulty).filter(row=>row.kind==="publish-skip");expect(skips.length).toBeGreaterThanOrEqual(7);expect(skips.every(row=>(row.reason??"").startsWith("error"))).toBe(true);
},300000);

function withPathCrate(m:Machine):void{
 writeFileSync(join(m.checkout,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["core","mac","leaf1","leaf2","app","pathy"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 crate(m.checkout,"pathy",`[package]\nname = "pathy"\nversion = "0.1.0"\nedition = "2021"\n`,"pub fn dir() -> &'static str { env!(\"CARGO_MANIFEST_DIR\") }\n");
 crate(m.checkout,"app",`[package]\nname = "app"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\nleaf1 = { path = "../leaf1" }\nleaf2 = { path = "../leaf2" }\npathy = { path = "../pathy" }\n`,"fn main() { println!(\"{} {}\", leaf1::f() + leaf2::f(), pathy::dir()); }\n");
}
test("a unit that embeds its manifest directory is shared only between machines with the same path, every other unit still crosses paths, and no upload diverges",async()=>{
 const server=await serve(),m1=machine("s1","a"),m2=machine("s2","b/c/d"),environment=remote(server);
 withPathCrate(m1);withPathCrate(m2);
 const first=await build(m1,environment,["-p","app"]);expect(first.code,first.output).toBe(0);await waitForUploader(m1);
 const second=await build(m2,environment,["-p","app"]);expect(second.code,second.output).toBe(0);await waitForUploader(m2);
 const missed=new Set(events(m2).filter(row=>row.kind==="miss").map(row=>row.crate)),hit=new Set(events(m2).filter(row=>row.kind==="remote-hit").map(row=>row.crate));
 expect(missed.has("pathy"),"the path is part of its key").toBe(true);expect(missed.has("app"),"its dependant holds the other bytes").toBe(true);
 for(const crate of["core","mac","leaf1","leaf2"])expect(hit.has(crate),`${crate} crosses paths`).toBe(true);
 const program=(m:Machine)=>spawnSync(join(m.cache,"l","l1","t","debug","app"),{encoding:"utf8"}).stdout.trim();
 expect(program(m1)).toContain(m1.checkout);expect(program(m2)).toContain(m2.checkout);expect(program(m2)).not.toContain(m1.checkout);
 const status=await (await fetch(server.url+"/status",{headers:{authorization:"Bearer w-token"}})).json() as {divergences?:{count:number}};
 expect(status.divergences?.count??0).toBe(0);
},300000);

test("a second upload that the server refuses as a divergence is reported as such, never as a plain failure",async()=>{
 const seen:string[]=[];
 const stub=Bun.serve({port:0,fetch(request){seen.push(request.method+" "+new URL(request.url).pathname.split("/")[2]);return new Response("{}",{status:request.method==="PUT"?409:200});}});
 try{
  const settings=readFleetSettingsV1(directory("stub"),{SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${stub.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"})!;
  const client=createFleetClientV1(settings);
  expect(await client.putResult("a".repeat(64),new Uint8Array([1]),"b".repeat(64))).toBe("diverged");
  expect(seen).toEqual(["PUT ac"]);
 }finally{stub.stop(true);}
});

test("the contract's timeoutSeconds and retries bound every call and the ride-through of the fleet client, and a slow or failing cache falls back at once",async()=>{
 let hits=0,slow=true;
 const stub=Bun.serve({port:0,async fetch(){hits++;if(slow)await Bun.sleep(4000);return new Response("{}",{status:404});}});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${stub.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"};
 try{
  expect(readFleetSettingsV1(directory("absent"),environment)).toMatchObject({timeoutMilliseconds:undefined,retries:undefined});
  expect(readFleetSettingsV1(directory("garbage"),{...environment,SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"soon",SEMIO_REMOTE_CACHE_RETRIES:"-1"})).toMatchObject({timeoutMilliseconds:undefined,retries:undefined});
  const settings=readFleetSettingsV1(directory("bounded"),{...environment,SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"1",SEMIO_REMOTE_CACHE_RETRIES:"0"})!;
  expect(settings).toMatchObject({timeoutMilliseconds:1000,retries:0});
  const client=createFleetClientV1(settings),started=Date.now();
  expect(await client.getResult("a".repeat(64)),"the 3 s lookup budget is cut to the contract's one second").toBe("down");
  expect(Date.now()-started).toBeLessThan(2500);
  const unbounded=createFleetClientV1(readFleetSettingsV1(directory("policy"),environment)!),policyStarted=Date.now();
  expect(await unbounded.getResult("b".repeat(64))).toBe("down");expect(Date.now()-policyStarted,"without the contract the policy's own 3 s apply").toBeGreaterThan(2500);
  slow=false;
  const flaky=Bun.serve({port:0,fetch(){hits++;return new Response("{}",{status:404});}});
  const reach=readFleetSettingsV1(directory("retry"),{...environment,SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${flaky.port}`,SEMIO_REMOTE_CACHE_RETRIES:"0"})!;
  expect(await createFleetClientV1(reach).getResult("c".repeat(64))).toBe("miss");flaky.stop(true);
  let attempts=0;const failing=(async()=>{attempts++;throw Error("connection reset");}) as unknown as typeof fetch;
  const ride=directory("ride");mkdirSync(join(ride,fleet.layout.state),{recursive:true});writeFileSync(join(ride,fleet.layout.state,fleet.layout.reachable),"");
  for(const [retries,expected] of [[0,1],[2,3]] as const){
   attempts=0;rmSync(join(ride,fleet.layout.state,fleet.layout.down),{force:true});writeFileSync(join(ride,fleet.layout.state,fleet.layout.reachable),"");
   const counted=createFleetClientV1(readFleetSettingsV1(ride,{...environment,SEMIO_REMOTE_CACHE_RETRIES:String(retries)})!,{fetch:failing});
   expect(await counted.getResult("d".repeat(64))).toBe("down");expect(attempts,`retries=${retries}`).toBe(expected);
  }
 }finally{stub.stop(true);}
},30000);
