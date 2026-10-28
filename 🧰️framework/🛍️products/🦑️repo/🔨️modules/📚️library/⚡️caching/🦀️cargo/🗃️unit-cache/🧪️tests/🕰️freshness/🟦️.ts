import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const here=fileURLToPath(new URL(".",import.meta.url)),script=resolve(here,"../../📜️script.ts"),cargo=process.env.SEMIO_TEST_CARGO??"cargo",base=resolve(tmpdir());
type Row={kind:string;crate:string};
const events=(cache:string,lane:string):Row[]=>{const path=join(cache,"x",lane+".jsonl");return existsSync(path)?readFileSync(path,"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line) as Row):[];};
const settle=()=>new Promise(done=>setTimeout(done,fixture.settleMilliseconds));
/** 🕰️ One verbose Cargo build of one package through the unit-cache wrapper in one lane (scratch workspace in the system temporary directory, outside every checkout, so no repository `.cargo/config.toml` reaches Cargo). */
function build(workspace:string,cache:string,lane:string,target:string,mode:Record<string,string>):Promise<{code:number;output:string}>{
 const laneRoot=join(cache,"l",lane);mkdirSync(laneRoot,{recursive:true});
 const environment:Record<string,string|undefined>={...process.env,RUSTC_WRAPPER:process.execPath,RUSTC:script,SEMIO_UNIT_CACHE_ROOT:cache,SEMIO_UNIT_CACHE_LANE:lane,SEMIO_UNIT_CACHE_LANE_ROOT:laneRoot,SEMIO_UNIT_CACHE_SLOTS:"4",SEMIO_UNIT_CACHE_FOCUS:fixture.focus,CARGO_TARGET_DIR:join(laneRoot,"t"),CARGO_TERM_COLOR:"never",...mode};
 for(const name of["RUSTC_WORKSPACE_WRAPPER","CARGO_BUILD_RUSTC_WRAPPER","CARGO_INCREMENTAL","RUSTFLAGS","CARGO_ENCODED_RUSTFLAGS"])delete environment[name];
 return new Promise(done=>{const child=spawn(cargo,["build","-v","--manifest-path",join(workspace,"Cargo.toml"),"-p",target],{cwd:workspace,env:environment as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"]});let output="";child.stdout.on("data",chunk=>{output+=chunk;});child.stderr.on("data",chunk=>{output+=chunk;});child.on("close",code=>done({code:code??1,output}));});
}
const verdicts=(output:string,word:string)=>[...new Set(output.split("\n").flatMap(line=>{const match=/^\s+(Fresh|Dirty|Compiling) (\S+) /u.exec(line);return match&&match[1]===word?[match[2]!]:[];}))].sort();

for(const[label,mode]of Object.entries(fixture.modes))test.skipIf(process.platform==="win32")(`${label} freshness: a unit served from the cache gets outputs as new as a compile would write them (outputs above the clone threshold, where a reflink copy keeps the cache object's last-use time), so the next unchanged Cargo build in that lane runs no compiler even when another lane used a dependency's cache object later; Cargo's own verdict and the independent mtime oracle agree`,async()=>{
 const root=mkdtempSync(join(base,`uf-${label}-`)),workspace=join(root,"ws"),cache=join(root,"cache");
 mkdirSync(workspace,{recursive:true});
 writeFileSync(join(workspace,"Cargo.toml"),`[workspace]\nresolver = "2"\nmembers = [${Object.keys(fixture.crates).map(name=>`"${name}"`).join(",")}]\n[profile.dev]\ndebug = 0\n`);
 for(const[name,dependencies]of Object.entries(fixture.crates)){
  mkdirSync(join(workspace,name,"src"),{recursive:true});
  writeFileSync(join(workspace,name,"Cargo.toml"),`[package]\nname = "${name}"\nversion = "0.1.0"\nedition = "2021"\n[dependencies]\n${dependencies.map(dependency=>`${dependency} = { path = "../${dependency}" }\n`).join("")}`);
  writeFileSync(join(workspace,name,"src","lib.rs"),`pub fn ${name}() -> u32 { ${dependencies.map(dependency=>`${dependency}::${dependency}() + `).join("")}1 }\n`+Array.from({length:fixture.functions},(_,index)=>`#[inline(never)] pub fn ${name}_${index}(x: u64) -> u64 { x.wrapping_mul(${index * 2 + 1}).rotate_left(${index % 63}) }\n`).join(""));
 }
 for(const[lane,target]of[[fixture.seedLane,fixture.target],[fixture.touchLane,fixture.touchTarget],[fixture.servedLane,fixture.target]] as const){await settle();const run=await build(workspace,cache,lane,target,mode);expect(run.code,run.output).toBe(0);}
 const first=events(cache,fixture.servedLane),deps=join(cache,"l",fixture.servedLane,"t","debug","deps");
 expect(readdirSync(deps).filter(name=>/\.(rlib|rmeta)$/u.test(name)).every(name=>statSync(join(deps,name)).size>fixture.cloneBytes)).toBe(true);
 expect(first.filter(row=>row.kind==="hit").map(row=>row.crate).sort()).toEqual(fixture.expect.firstServed);
 const oracle=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),join(cache,"l",fixture.servedLane,"t","debug")],{encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);
 await settle();
 const second=await build(workspace,cache,fixture.servedLane,fixture.target,mode);expect(second.code,second.output).toBe(0);
 const invoked=events(cache,fixture.servedLane).slice(first.length).map(row=>row.crate).sort();
 expect({invoked,cargo:verdicts(second.output,"Dirty"),oracle:JSON.parse(oracle.stdout)}).toEqual({invoked:fixture.expect.secondInvoked,cargo:fixture.expect.secondInvoked,oracle:fixture.expect.secondInvoked});
 expect(verdicts(second.output,"Fresh")).toEqual(fixture.expect.firstServed);
},180000);
