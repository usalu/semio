import {afterAll,expect,test} from "bun:test";
import {chmodSync,cpSync,createReadStream,mkdirSync,readFileSync,readdirSync,statSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {digestTaskViewInputsV1,openTaskViewV1,taskViewCargoEnvironmentV1} from "../../🟦️.ts";
import {removeV1,scratchV1,sleepV1,snapshotV1} from "../🧰️world/🟦️.ts";

const cargo=Bun.which("cargo"),pinned="nightly-2026-07-20",usable=cargo!==null&&Bun.spawnSync([cargo,"--version"],{env:{...process.env,RUSTUP_TOOLCHAIN:pinned}}).exitCode===0;
const scratch:string[]=[];
const world=(tag:string)=>{const root=scratchV1(tag);scratch.push(root);return root;};
afterAll(()=>{for(const root of scratch)removeV1(root);});
const CRATES:Record<string,{deps?:string;lib:string;build?:string}>={
 a:{lib:"pub fn a()->u32{1}\n"},
 b:{deps:'ta={path="../a"}\n',lib:"pub fn b()->u32{ta::a()+1}\n"},
 c:{lib:'pub fn c()->u32{panic!("c at {}",file!())}\n'},
 d:{lib:"pub fn d()->u32{2}\n",build:'fn main(){std::thread::sleep(std::time::Duration::from_millis(2500));println!("cargo:rerun-if-changed=build.rs");}\n'},
};
function growWorkspace(root:string):string[]{
 const files:Record<string,string>={"Cargo.toml":'cargo-features=["trim-paths"]\n[workspace]\nresolver="2"\nmembers=["crates/a","crates/b","crates/c","crates/d"]\n[profile.dev]\ntrim-paths="all"\n'};
 for(const[name,crate]of Object.entries(CRATES)){
  files[`crates/${name}/Cargo.toml`]=`[package]\nname="t${name}"\nversion="0.1.0"\nedition="2021"\n${crate.build?'build="build.rs"\n':""}[dependencies]\n${crate.deps??""}`;
  files[`crates/${name}/src/lib.rs`]=crate.lib;
  if(crate.build)files[`crates/${name}/build.rs`]=crate.build;
 }
 for(const[path,text]of Object.entries(files)){mkdirSync(join(root,path,".."),{recursive:true});writeFileSync(join(root,path),text);}
 const run=Bun.spawnSync([cargo!,"generate-lockfile","--offline"],{cwd:root,env:{...process.env,RUSTUP_TOOLCHAIN:pinned},stdout:"pipe",stderr:"pipe"});
 expect(run.exitCode,run.stderr.toString()).toBe(0);
 return[...Object.keys(files),"Cargo.lock"];
}
type Build=Readonly<{compiled:readonly string[];fresh:readonly string[];code:number;stderr:string}>;
async function build(root:string,target:string,extra:Record<string,string>={}):Promise<Build>{
 const environment=Object.fromEntries(Object.entries(process.env).filter(([name])=>!name.startsWith("CARGO_")&&!name.startsWith("RUSTFLAGS")&&!name.startsWith("RUSTUP_")&&!name.startsWith("SEMIO_")).map(([k,v])=>[k,String(v)]));
 const child=Bun.spawn(["nice","-n","5",cargo!,"build","--offline","--locked","-j","2","-v"],{cwd:root,env:{...environment,CARGO_INCREMENTAL:"0",RUSTUP_TOOLCHAIN:pinned,...taskViewCargoEnvironmentV1({root,targetDirectory:target}),...extra},stdout:"pipe",stderr:"pipe"});
 const stderr=await new Response(child.stderr).text(),code=await child.exited,names=(word:string)=>[...new Set([...stderr.matchAll(new RegExp(word+" (t[a-z]) v","gu"))].map(match=>match[1]!))].sort();
 return{compiled:names("Compiling"),fresh:names("Fresh"),code,stderr};
}
const rlib=(target:string,name:string):string=>createHash("sha256").update(readFileSync(join(target,"debug","lib"+name+".rlib"))).digest("hex");
const tree=(root:string,paths:readonly string[])=>digestTaskViewInputsV1(root,paths);
const ready=<T extends {kind:string}>(outcome:T):Extract<T,{kind:"ready"}>=>{expect(outcome.kind,JSON.stringify(outcome)).toBe("ready");return outcome as Extract<T,{kind:"ready"}>;};

test.skipIf(!usable)("unit law: a second view of the same content shares every unit, the view path is invisible to the fingerprints, a path-bearing rustflag is not",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),target=world("tv-target"),paths=growWorkspace(live),inputs=await tree(live,paths);
 const one=ready(await openTaskViewV1({source:live,directory,hash:"cargo0001",inputs})).view;
 const cold=await build(one.root,target);
 expect(cold.code,cold.stderr).toBe(0);expect(cold.compiled).toEqual(["ta","tb","tc","td"]);
 const same=await build(one.root,target);
 expect(same.compiled,"the same hash is the same view: nothing compiles").toEqual([]);expect(same.fresh).toEqual(["ta","tb","tc","td"]);
 const two=ready(await openTaskViewV1({source:live,directory,hash:"cargo0002",inputs})).view;
 expect(two.root).not.toBe(one.root);
 const other=await build(two.root,target);
 expect(other.code,other.stderr).toBe(0);expect(other.compiled,"another view of the same content: zero rebuilt units").toEqual([]);expect(other.fresh).toEqual(["ta","tb","tc","td"]);
 const flagged=world("tv-target2");
 expect((await build(one.root,flagged,{CARGO_ENCODED_RUSTFLAGS:"--remap-path-prefix="+one.root+"=/x"})).compiled).toEqual(["ta","tb","tc","td"]);
 expect((await build(two.root,flagged,{CARGO_ENCODED_RUSTFLAGS:"--remap-path-prefix="+two.root+"=/x"})).compiled,"control: a flag that names the view gives every view its own units").toEqual(["ta","tb","tc","td"]);
},180000);

test.skipIf(!usable)("unit law: one changed source file rebuilds only its crate and its dependents",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),target=world("tv-target"),paths=growWorkspace(live);
 const base=ready(await openTaskViewV1({source:live,directory,hash:"cargo0003",inputs:await tree(live,paths)})).view;
 expect((await build(base.root,target)).code).toBe(0);
 await sleepV1(1100);
 writeFileSync(join(live,"crates/a/src/lib.rs"),"pub fn a()->u32{1}\npub fn more()->u32{7}\n");
 const changed=ready(await openTaskViewV1({source:live,directory,hash:"cargo0004",inputs:await tree(live,paths)})).view;
 const result=await build(changed.root,target);
 expect(result.code,result.stderr).toBe(0);
 expect(result.compiled).toEqual(["ta","tb"]);expect(result.fresh).toEqual(["tc","td"]);
},180000);

test.skipIf(!usable)("churn law: every file of the closure is rewritten every 250 ms in the live tree during a real build in the view: one run, byte-identical to a build of the start state",async()=>{
 const live=world("tv-live"),pristine=world("tv-pristine"),directory=world("tv-views"),target=world("tv-target"),target2=world("tv-target"),paths=growWorkspace(live);
 cpSync(live,pristine,{recursive:true,preserveTimestamps:true});
 const start=snapshotV1(live),inputs=await tree(live,paths);
 const view=ready(await openTaskViewV1({source:live,directory,hash:"cargo0005",inputs})).view;
 let round=0,running=true;
 const writer=(async()=>{while(running){round++;for(const path of paths)writeFileSync(join(live,path),path.endsWith(".rs")||path.endsWith(".toml")?readFileSync(join(live,path),"utf8")+`# churn ${round}\n`.replace("# ",path.endsWith(".rs")?"// ":"# "):readFileSync(join(live,path)));await sleepV1(250);}})();
 const result=await build(view.root,target);
 running=false;await writer;
 expect(result.code,result.stderr).toBe(0);
 expect(round,"the live tree changed during the build").toBeGreaterThan(3);
 expect(snapshotV1(live)).not.toEqual(start);
 expect(result.compiled,"one run, every unit once").toEqual(["ta","tb","tc","td"]);
 expect(result.stderr.match(/Compiling td/gu)?.length).toBe(1);
 expect((await view.verify()).ok).toBe(true);
 const model=ready(await openTaskViewV1({source:pristine,directory,hash:"cargo0006",inputs:await tree(pristine,paths)})).view;
 expect((await build(model.root,target2)).code).toBe(0);
 for(const name of["ta","tb","tc","td"])expect(rlib(target,name),"unit "+name+" equals the build of the start state").toBe(rlib(target2,name));
 expect(readFileSync(join(target,"debug","libtc.rlib")).includes(Buffer.from(view.root)),"the view path is in no output").toBe(false);
},240000);

test.skipIf(!usable)("control: the same churn on the live root leaves a result that is not of the start state (the next build rebuilds units), which is what the view removes",async()=>{
 const live=world("tv-live"),target=world("tv-target"),paths=growWorkspace(live);
 let round=0,running=true;
 const writer=(async()=>{while(running){round++;for(const path of paths.filter(path=>path.endsWith(".rs")))writeFileSync(join(live,path),readFileSync(join(live,path),"utf8")+`// churn ${round}\n`);await sleepV1(250);}})();
 const first=await build(live,target,{});
 running=false;await writer;
 expect(first.code,first.stderr).toBe(0);
 const second=await build(live,target,{});
 expect(second.compiled.length,"the live build is stale against the tree it ran in").toBeGreaterThan(0);
},240000);

test.skipIf(!usable)("unit law: files rewritten with equal bytes (a new modification time) do not rebuild a unit in the next view, a real edit rebuilds only its crate",async()=>{
 const live=world("tv-live"),directory=world("tv-views"),target=world("tv-target"),paths=growWorkspace(live);
 const base=ready(await openTaskViewV1({source:live,directory,hash:"cargo0007",inputs:await tree(live,paths)})).view;
 expect((await build(base.root,target)).code).toBe(0);
 await sleepV1(1100);
 for(const path of paths)writeFileSync(join(live,path),readFileSync(join(live,path)));
 writeFileSync(join(live,"crates/c/src/lib.rs"),'pub fn c()->u32{panic!("c at {}",file!())}\npub fn more()->u32{1}\n');
 const next=ready(await openTaskViewV1({source:live,directory,hash:"cargo0008",inputs:await tree(live,paths)})).view;
 const result=await build(next.root,target);
 expect(result.code,result.stderr).toBe(0);
 expect(result.compiled,"only the crate whose bytes changed").toEqual(["tc"]);expect(result.fresh).toEqual(["ta","tb","td"]);
},180000);
