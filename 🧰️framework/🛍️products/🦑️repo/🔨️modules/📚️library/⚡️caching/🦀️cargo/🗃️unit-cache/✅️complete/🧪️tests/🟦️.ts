import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {putCasObjectSyncV1,readCasObjectSyncV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🔁️sync/🟦️.ts";
import {casRootV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {cargoScratchBaseV1} from "../../🧪️tests/🏖️scratch/🟦️.ts";
import {parseRustcUnitV1} from "../../🔏️key/🟦️.ts";
import {lookupUnitV1,publishUnitV1,restoreUnitV1,writeEntryV1,type UnitEntryV1} from "../../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../🛰️fleet/📡️client/🟦️.ts";
import {installFleetUnitV1} from "../../🛰️fleet/📥️adopt/🟦️.ts";
import {uploadUnitV1,type SpoolJobV1} from "../../🛰️fleet/📤️uploader/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import unitSchema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {announcedPathsV1,expectedOutputsV1,missingOutputsV1,uncoveredOutputsV1} from "../🟦️.ts";

/** ✅️ A unit is reported done only when every file a dependant names is in place. Cargo starts the dependants of a pipelined unit as soon as it reads the `.rmeta` announcement and gives them `--extern name=<out>/lib<name>-<hash>.rmeta`; a unit that was served, installed or offloaded without that file ends the build with `extern location for <name> does not exist`. These tests show the rule against the compiler itself, against real Cargo with `embed-metadata = "DoNotEmbed"` (the repository's configuration) and against the fleet tier. */
const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../📜️script.ts"),base=cargoScratchBaseV1(),cargo=process.env.SEMIO_TEST_CARGO??"cargo";
const pinned=/^\s*channel\s*=\s*"([^"]+)"/mu.exec(readFileSync(resolve(here,"../../../../../../../../../../rust-toolchain.toml"),"utf8"))![1]!;
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"co-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const ajv=new Ajv({strict:false});ajv.addSchema(schema);
const python=testPythonV1()??"python3";

test("the schema is closed and every vector and announcement validates against it",()=>{
 const validate=ajv.getSchema(`${schema.$id}#/definitions/Fixture`)!;
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...fixture,extra:1})).toBe(false);
 expect(ajv.getSchema(`${schema.$id}#/definitions/Vector`)!({name:"x",platform:"plan9",arguments:[],expected:[]})).toBe(false);
 const events=new Ajv({strict:false});events.addSchema(unitSchema);
 expect((unitSchema.definitions.Event.properties.kind.enum as string[])).toContain("incomplete");
});

const unitOf=(args:readonly string[])=>parseRustcUnitV1(args);
test("the promised output names equal the vectors, an independent Python model and, where the platform allows, the files the compiler itself writes",()=>{
 const rows=fixture.vectors;
 const model=spawnSync(python,["-I",resolve(here,"🐍️.py")],{input:JSON.stringify(rows),encoding:"utf8"});expect(model.status,model.stderr).toBe(0);
 const modelled=JSON.parse(model.stdout) as string[][];
 rows.forEach((row,index)=>{
  const names=[...expectedOutputsV1(unitOf(row.arguments),row.arguments,row.platform)];
  expect(names,row.name).toEqual(row.expected);expect(modelled[index],row.name).toEqual(row.expected);
 });
 for(const row of rows.filter(entry=>entry.runnable===true&&entry.platform===process.platform)){
  const work=directory("rustc"),out=join(work,"o");mkdirSync(out);
  const types=row.arguments.flatMap((argument,index)=>argument==="--crate-type"?[row.arguments[index+1]!]:argument.startsWith("--crate-type=")?[argument.slice(13)]:[]);
  const source=types.includes("proc-macro")?"extern crate proc_macro;\n":types.includes("bin")||types.length===0?"fn main() {}\n":"pub fn f() {}\n";
  writeFileSync(join(work,"lib.rs"),source);
  const args=row.arguments.map(argument=>argument==="/o"?out:argument==="--out-dir=/o"?"--out-dir="+out:argument);
  const run=spawnSync("rustc",["--edition=2021",...args,join(work,"lib.rs")],{cwd:work,encoding:"utf8",env:{...process.env,RUSTUP_TOOLCHAIN:pinned}});
  expect(run.status,row.name+"\n"+run.stderr).toBe(0);
  expect(readdirSync(out).sort(),row.name).toEqual([...row.expected].sort());
 }
});

test("announcements are read as the vectors say and the missing files are found by name",()=>{
 for(const row of fixture.announcements)expect(announcedPathsV1(row.text),row.name).toEqual(row.paths);
 const dir=directory("missing");writeFileSync(join(dir,"a.d"),"x");mkdirSync(join(dir,"folder"));
 const text=`{"artifact":"${join(dir,"liba.rmeta")}","emit":"metadata"}\n{"artifact":"${join(dir,"a.d")}","emit":"dep-info"}\n`;
 expect(missingOutputsV1({directory:dir,cwd:dir,expected:["a.d","liba.rlib","folder"],text})).toEqual(["liba.rlib","folder","liba.rmeta"]);
 expect(missingOutputsV1({directory:dir,cwd:dir,expected:["a.d"],text:""})).toEqual([]);
 expect(missingOutputsV1({directory:dir,cwd:dir,expected:[],text:`{"artifact":"a.d","emit":"dep-info"}\n`})).toEqual([]);
 expect(uncoveredOutputsV1(["a","b"],["b","c","c","d"])).toEqual(["c","d"]);
});

const useCas=()=>{const cas=join(directory("cas"));process.env.SEMIO_CAS_ROOT=cas;return cas;};
function entryFor(root:string,key:string,names:readonly string[],content="x"):UnitEntryV1{
 return{version:1,key,crate:"x",dependencies:[],environment:[],outputs:names.map(name=>({name,sha256:putCasObjectSyncV1(casRootV1(),name+content),bytes:(name+content).length,executable:false})),stderr:null,stdout:null,createdMs:Date.now()};
}
test("an entry that lacks a promised output is never served, never stored and never installed",()=>{
 useCas();const root=directory("store"),key="a".repeat(64),context={cwd:root,laneRoot:undefined,environment:{},stampDirectory:undefined};
 const complete=["x-1.d","libx-1.rmeta","libx-1.rlib"];
 writeEntryV1(root,{...entryFor(root,key,["x-1.d","libx-1.rlib"]),createdMs:1000});
 expect(lookupUnitV1(root,key,context),"without a promise the old behaviour holds").toBeDefined();
 expect(lookupUnitV1(root,key,{...context,expected:complete}),"the variant that lacks the rmeta is a miss").toBeUndefined();
 writeEntryV1(root,{...entryFor(root,key,complete),createdMs:2000});
 expect(lookupUnitV1(root,key,{...context,expected:complete})?.outputs.length).toBe(3);
 writeEntryV1(root,{...entryFor(root,key,["x-1.d","libx-1.rlib"]),createdMs:3000});
 expect(lookupUnitV1(root,key,{...context,expected:complete})?.createdMs,"an older complete variant is served behind a newer incomplete one").toBe(2000);
 const out=directory("out");for(const name of ["x-1.d","libx-1.rlib"])writeFileSync(join(out,name),"1");
 const input={key:"b".repeat(64),crate:"x",cwd:out,outDir:out,laneRoot:undefined,outputs:["x-1.d","libx-1.rlib"].map(name=>join(out,name)),stdout:"",stderr:"",stampDirectory:undefined};
 writeFileSync(join(out,"x-1.d"),"libx-1.rlib: src/lib.rs\n");
 expect(publishUnitV1(root,{...input,expected:complete}),"an incomplete unit is not published").toBeUndefined();
 expect(publishUnitV1(root,input)).toBeDefined();
});

test("a unit is restored by placing each file in one step: a reader never finds a name absent while the unit is restored again",async()=>{
 useCas();const root=directory("atomic"),out=directory("atomic-out"),big=Buffer.alloc(48*1024*1024,7);
 const sha=putCasObjectSyncV1(casRootV1(),big),entry:UnitEntryV1={...entryFor(root,"c".repeat(64),[]),outputs:[{name:"libbig-1.rmeta",sha256:sha,bytes:big.length,executable:false}]};
 restoreUnitV1(root,entry,{outDir:out,laneRoot:undefined,stampDirectory:undefined});
 const reader=`const {existsSync}=require("node:fs");const path=process.argv[1];let misses=0,reads=0;process.stdout.write("ready\\n");const stop=Date.now()+Number(process.argv[2]);while(Date.now()<stop){reads++;if(!existsSync(path))misses++;}process.stdout.write(JSON.stringify({misses,reads})+"\\n");`;
 const child=spawn(process.execPath,["-e",reader,join(out,"libbig-1.rmeta"),"2500"],{stdio:["ignore","pipe","inherit"]});let text="";child.stdout.on("data",chunk=>{text+=chunk;});
 await new Promise<void>(done=>{const timer=setInterval(()=>{if(text.includes("ready")){clearInterval(timer);done();}},5);});
 const until=Date.now()+2000;let rounds=0;while(Date.now()<until){restoreUnitV1(root,entry,{outDir:out,laneRoot:undefined,stampDirectory:undefined});rounds++;}
 await new Promise<void>(done=>child.on("close",()=>done()));
 const seen=JSON.parse(text.split("\n").filter(line=>line.startsWith("{"))[0]!) as {misses:number;reads:number};
 expect(rounds).toBeGreaterThan(5);expect(seen.reads).toBeGreaterThan(1000);expect(seen.misses).toBe(0);
 expect(readdirSync(out)).toEqual(["libbig-1.rmeta"]);
},60000);

function crate(root:string,name:string,manifest:string,source:string,file="lib.rs"){const dir=join(root,name);mkdirSync(join(dir,"src"),{recursive:true});writeFileSync(join(dir,"Cargo.toml"),manifest);writeFileSync(join(dir,"src",file),source);}
function workspace(root:string):void{
 mkdirSync(join(root,".cargo"),{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),`cargo-features = ["trim-paths"]\n[workspace]\nresolver = "2"\nmembers = ["pa","pb","mac","app"]\n[profile.dev]\ndebug = 0\ntrim-paths = "object"\n`);
 writeFileSync(join(root,".cargo","config.toml"),`[build]\nrustflags = ["-Z", "threads=0"]\n[unstable]\nembed-metadata = "DoNotEmbed"\nchecksum-freshness = true\n`);
 const manifest=(name:string,dependencies:string,extra="")=>`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n${extra}[dependencies]\n${dependencies}`;
 crate(root,"pa",manifest("pa",""),"pub fn id() -> u32 { 1 }\n");
 crate(root,"pb",manifest("pb","pa = { path = \"../pa\" }\n"),"pub fn id() -> u32 { pa::id() + 1 }\n");
 crate(root,"mac",manifest("mac","","[lib]\nproc-macro = true\n"),"use proc_macro::TokenStream;\n#[proc_macro]\npub fn five(_: TokenStream) -> TokenStream { \"5\".parse().unwrap() }\n");
 crate(root,"app",manifest("app","pb = { path = \"../pb\" }\nmac = { path = \"../mac\" }\n"),"fn main() { println!(\"{}\", pb::id() + mac::five!()); }\n","main.rs");
}
type Machine=Readonly<{checkout:string;cache:string;cas:string}>;
function machine(label:string):Machine{const root=directory(label),checkout=join(root,"w","ws"),cache=join(root,"cache");workspace(checkout);mkdirSync(cache,{recursive:true});return{checkout,cache,cas:join(root,"cas")};}
function build(m:Machine,lane:string,environment:Record<string,string|undefined>={}):Promise<{code:number;output:string}>{
 const laneRoot=join(m.cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const env:Record<string,string|undefined>={...process.env,RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:m.cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_TOOLCHAIN:"test-toolchain",SEMIO_UNIT_CACHE_INNER:JSON.stringify(["rustc"]),SEMIO_CAS_ROOT:m.cas,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_BUILD_BUILD_DIR:join(laneRoot,"b"),CARGO_TERM_COLOR:"never",CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:pinned,...environment};
 for(const key of Object.keys(process.env))if(key.startsWith("SEMIO_FLEET_")||key==="SEMIO_WORKSPACE_ROOT"||key==="SEMIO_UNIT_CACHE_WORKSPACE_ROOT"||(key.startsWith("SEMIO_REMOTE_CACHE_")&&!(key in environment)))delete env[key];
 for(const key of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_MAKEFLAGS","MAKEFLAGS","MFLAGS"])delete env[key];
 return new Promise(done=>{const child=spawn(cargo,["build","--manifest-path",join(m.checkout,"Cargo.toml"),"-p","app","-j","3"],{cwd:m.checkout,env:env as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});child.on("close",code=>done({code:code??1,output}));});
}
type Event={kind:string;crate:string;reason?:string};
const events=(m:Machine):Event[]=>{const dir=join(m.cache,"x");return existsSync(dir)?readdirSync(dir).filter(name=>name.endsWith(".jsonl")).flatMap(name=>readFileSync(join(dir,name),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as Event)):[];};
const count=(rows:readonly Event[],kind:string,name?:string)=>rows.filter(row=>row.kind===kind&&(name===undefined||row.crate===name)).length;
function entryFiles(m:Machine,name:string):string[]{
 const rows:string[]=[],root=join(m.cache,"e");
 for(const shard of readdirSync(root))for(const key of readdirSync(join(root,shard)))for(const file of readdirSync(join(root,shard,key))){const path=join(root,shard,key,file);if((JSON.parse(readFileSync(path,"utf8")) as {crate:string}).crate===name)rows.push(path);}
 return rows;
}
function tamper(m:Machine,name:string,change:(entry:UnitEntryV1)=>UnitEntryV1):void{for(const path of entryFiles(m,name))writeFileSync(path,JSON.stringify(change(JSON.parse(readFileSync(path,"utf8")) as UnitEntryV1)));}
function filesBelow(dir:string):string[]{return readdirSync(dir,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?filesBelow(join(dir,entry.name)):[entry.name]);}
const program=(m:Machine,lane:string)=>spawnSync(join(m.cache,"l",lane,"t","debug","app"),{encoding:"utf8"}).stdout.trim();

test("the fault, reproduced: an entry without the .rmeta no longer ends a pipelined build with 'extern location does not exist'; the unit is compiled again and republished complete",async()=>{
 const m=machine("rmeta"),first=await build(m,"l1");expect(first.code,first.output).toBe(0);expect(program(m,"l1")).toBe("7");
 const cold=events(m);expect(count(cold,"miss")).toBeGreaterThanOrEqual(4);expect(count(cold,"incomplete")).toBe(0);
 for(const name of["pa","pb"])tamper(m,name,entry=>({...entry,outputs:entry.outputs.filter(row=>!row.name.endsWith(".rmeta"))}));
 const second=await build(m,"l2");expect(second.code,second.output).toBe(0);expect(second.output).not.toContain("extern location");expect(program(m,"l2")).toBe("7");
 const after=events(m);expect(count(after,"miss","pa")).toBe(2);expect(count(after,"miss","pb")).toBe(2);expect(count(after,"miss","mac")).toBe(1);
 for(const name of["pa","pb"]){const newest=entryFiles(m,name).map(path=>JSON.parse(readFileSync(path,"utf8")) as UnitEntryV1).sort((left,right)=>right.createdMs-left.createdMs)[0]!;expect(newest.outputs.map(row=>row.name).some(file=>file.endsWith(".rmeta")),name).toBe(true);}
 const third=await build(m,"l3");expect(third.code,third.output).toBe(0);expect(count(events(m),"miss")).toBe(count(after,"miss"));
},300000);

test("an entry whose stored messages announce a file that its outputs do not hold is detected after the restore and compiled again",async()=>{
 const m=machine("ghost"),first=await build(m,"l1");expect(first.code,first.output).toBe(0);process.env.SEMIO_CAS_ROOT=m.cas;
 tamper(m,"pa",entry=>{const text=readCasObjectSyncV1(casRootV1(),entry.stderr!)!.toString("utf8");return{...entry,stderr:putCasObjectSyncV1(casRootV1(),text+`{"$message_type":"artifact","artifact":"@@semio-out@@/libpa-ghost.rmeta","emit":"metadata"}\n`)};});
 const second=await build(m,"l2");expect(second.code,second.output).toBe(0);
 const rows=events(m);expect(count(rows,"incomplete","pa")).toBeGreaterThanOrEqual(1);expect(rows.find(row=>row.kind==="incomplete")?.reason).toContain("libpa-ghost.rmeta");expect(count(rows,"miss","pa")).toBe(2);
},300000);

test("a proc macro entry without its dynamic library is a miss too",async()=>{
 const m=machine("macro"),first=await build(m,"l1");expect(first.code,first.output).toBe(0);
 tamper(m,"mac",entry=>({...entry,outputs:entry.outputs.filter(row=>!/\.(dylib|so|dll)$/u.test(row.name))}));
 const second=await build(m,"l2");expect(second.code,second.output).toBe(0);expect(program(m,"l2")).toBe("7");expect(count(events(m),"miss","mac")).toBe(2);
},300000);

test("entries made under a lane of the previous identifier length are served in a lane of the new length: outputs are placed by the current output directory from content",async()=>{
 const m=machine("lanes"),old=await build(m,"qa1b2c3d4e5f");expect(old.code,old.output).toBe(0);
 const misses=count(events(m),"miss");expect(misses).toBeGreaterThanOrEqual(4);
 const current=await build(m,"q0c9bae");expect(current.code,current.output).toBe(0);expect(program(m,"q0c9bae")).toBe("7");
 expect(count(events(m),"miss"),"nothing is compiled again").toBe(misses);expect(count(events(m),"hit")).toBeGreaterThanOrEqual(misses);
 expect(filesBelow(join(m.cache,"l","q0c9bae","b")).filter(file=>/^libpa-.*\.rmeta$/u.test(file)),"the unit sits in the new lane").toHaveLength(1);
},300000);

test("the fleet tier refuses a manifest that lacks a promised output and installs the complete one",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const m=machine("fleet"),built=await build(m,"l1");expect(built.code,built.output).toBe(0);
 process.env.SEMIO_CAS_ROOT=m.cas;
 const names=(path:string)=>(JSON.parse(readFileSync(path,"utf8")) as UnitEntryV1).outputs.map(row=>row.name);
 const path=entryFiles(m,"pa")[0]!,promised=names(path),platform={version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""} as const;
 expect(promised.some(file=>file.endsWith(".rmeta"))).toBe(true);
 const settings=readFleetSettingsV1(directory("fleet-state"),{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"})!,client=createFleetClientV1(settings);
 const entry=JSON.parse(readFileSync(path,"utf8")) as UnitEntryV1,put=async(key:string,keep:(row:{name:string})=>boolean)=>{
  const local=join(directory("poison"),"x"),shard=join(local,"e",entry.key.slice(0,2),entry.key);mkdirSync(shard,{recursive:true});
  writeFileSync(join(shard,"000000000000001-aaaaaa.json"),JSON.stringify({...entry,outputs:entry.outputs.filter(keep)}));
  const job:SpoolJobV1={version:1,key,localKey:entry.key,slot:"2".repeat(64),crate:"pa",cwd:"/w",workspace:null,laneRoot:null,owner:"t:1:aa",claimed:false,costMs:1,platform,queuedMs:Date.now()};
  expect(await uploadUnitV1(client,local,job)).toBe(true);return job;
 };
 const partial=await put("4".repeat(64),row=>!row.name.endsWith(".rmeta")),whole=await put("5".repeat(64),()=>true);
 const target=(job:SpoolJobV1,expected?:readonly string[])=>({client,root:directory("install"),localKey:"6".repeat(64),key:job.key,slot:job.slot,platform,roots:[],...(expected?{expected}:{})});
 expect(await installFleetUnitV1(target(partial)),"without the promise the hole is open").toBe("hit");
 expect(await installFleetUnitV1(target(partial,promised))).toBe("miss");
 expect(await installFleetUnitV1(target(whole,promised))).toBe("hit");
},300000);
