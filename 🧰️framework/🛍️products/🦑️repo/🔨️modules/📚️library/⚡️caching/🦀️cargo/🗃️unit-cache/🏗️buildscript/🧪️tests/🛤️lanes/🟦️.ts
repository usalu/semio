import {afterAll,beforeAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {lookupBuildRunV1,publishBuildRunV1,restoreBuildRunV1} from "../../../💾️store/🟦️.ts";
import {buildKeyEnvironmentV1,sha256V1,UNIT_CACHE_POLICY_V1} from "../../../🔏️key/🟦️.ts";
import {localKeyV1,portableRootsV1,portableTextV1,remoteKeyV1,toolRootsV1,type PlatformV1} from "../../../🛰️fleet/🪪️identity/🟦️.ts";
import {buildRunKeyInputV1} from "../../🔑️run-key/🟦️.ts";

/** 🛤️ The key of one build script run is a function of what the script was given, never of where: two lanes of one machine and a second machine with other roots compute one local key and one fleet key, so the second lane serves the first lane's run (byte-equal OUT_DIR, streams naming its own directories).
 * @see ./🔣️.json */
const fixture=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8")) as {package:{name:string;version:string;rust:string;identity:string};targets:{name:string;home:string;workspace:string;lane:string;dependencyHash:string;unitHash:string}[]};
type Target=(typeof fixture.targets)[number];
const scratch=realpathSync(mkdtempSync(join(tmpdir(),"lanes-"))),previous=process.env.SEMIO_CAS_ROOT;
const platform={version:1,os:"darwin",arch:"arm64",host:"aarch64-apple-darwin",rustc:"a".repeat(64),toolchain:"t",linker:"",cpu:""} as PlatformV1;
beforeAll(()=>{process.env.SEMIO_CAS_ROOT=join(scratch,"cas");});
afterAll(()=>{if(previous===undefined)delete process.env.SEMIO_CAS_ROOT;else process.env.SEMIO_CAS_ROOT=previous;rmSync(scratch,{recursive:true,force:true});});

const dependencyOut=(target:Target)=>target.lane+"/b/release/build/zlib-"+target.dependencyHash+"/out";
const environmentOf=(target:Target)=>({
 HOME:target.home,CARGO_HOME:target.home+"/.cargo",CARGO_MANIFEST_DIR:target.home+"/.cargo/registry/src/idx/"+fixture.package.name+"-"+fixture.package.version,CARGO_PKG_NAME:fixture.package.name,CARGO_PKG_VERSION:fixture.package.version,
 TARGET:"aarch64-apple-darwin",HOST:"aarch64-apple-darwin",PROFILE:"release",OPT_LEVEL:"0",DEBUG:"false",RUSTC:"rustc",RUSTC_WRAPPER:target.lane+"/bin/semio-unit-cache",
 CARGO_ENCODED_RUSTFLAGS:["-Z","threads=0","--remap-path-prefix",target.lane+"/t=/target","--remap-path-prefix="+target.workspace+"=/workspace"].join("\u001f"),
 DEP_ZLIB_ROOT:dependencyOut(target),DEP_ZLIB_INCLUDE:target.lane+"/b/release/build/zlib/"+target.dependencyHash+"/out/include",
});
const rootsOf=(target:Target)=>portableRootsV1({cwd:environmentOf(target).CARGO_MANIFEST_DIR,workspace:target.workspace,hints:[environmentOf(target).CARGO_MANIFEST_DIR],...toolRootsV1(environmentOf(target))});
const inputOf=(target:Target)=>buildRunKeyInputV1({toolchain:fixture.package.rust,identity:fixture.package.identity,args:[],environment:environmentOf(target),laneRoot:target.lane});
const keys=(target:Target)=>({local:localKeyV1(inputOf(target),rootsOf(target)),remote:remoteKeyV1(platform,inputOf(target),rootsOf(target))});

test("every lane of a machine and a second machine compute the same local and fleet key of one run",()=>{
 const rows=fixture.targets.map(keys);
 expect(new Set(rows.map(row=>row.local)).size).toBe(1);expect(new Set(rows.map(row=>row.remote)).size).toBe(1);
});

test("the dependency directories enter the key as the directory they are, not by lane or unit hash",()=>{
 const rows=buildKeyEnvironmentV1(environmentOf(fixture.targets[1]!),fixture.targets[1]!.lane);
 expect(rows.filter(row=>row[0].startsWith("DEP_")).map(row=>row[1])).toEqual(["@@semio-lane@@⟨build⟩zlib/out/include","@@semio-lane@@⟨build⟩zlib/out"]);
 const first=fixture.targets[0]!,moved={...environmentOf(first),DEP_ZLIB_ROOT:"/elsewhere/other"};
 expect(localKeyV1(buildRunKeyInputV1({toolchain:fixture.package.rust,identity:fixture.package.identity,args:[],environment:moved,laneRoot:first.lane}),rootsOf(first))).not.toBe(keys(first).local);
});

test("a run recorded in one lane is a hit in another lane, its OUT_DIR is byte-equal up to the lane's own paths and its streams name the replaying lane's directories",()=>{
 const [first,second]=fixture.targets as [Target,Target],cache=join(scratch,"cache"),key=keys(first).local;
 const outOf=(target:Target)=>target.lane.replace(/^\/(Users|Volumes)/u,scratch+"/$1")+"/b/release/build/"+fixture.package.name+"-"+target.unitHash+"/out";
 const placed=(target:Target):Target=>({...target,lane:target.lane.replace(/^\/(Users|Volumes)/u,scratch+"/$1"),workspace:target.workspace.replace(/^\/Users/u,scratch+"/Users")}),a=placed(first),b=placed(second);
 const outA=outOf(first),outB=outOf(second);mkdirSync(join(outA,"gen"),{recursive:true});mkdirSync(a.workspace,{recursive:true});mkdirSync(b.workspace,{recursive:true});
 const text=(out:string,target:Target)=>"pub const OUT:&str=\""+out+"\";pub const DEP:&str=\""+dependencyOut(target)+"\";\n";
 writeFileSync(join(outA,"gen","table.rs"),text(outA,a));writeFileSync(join(outA,"blob.bin"),new Uint8Array([0,255,1,254]));
 const stdout=(out:string,target:Target)=>"cargo:rustc-link-search=native="+dependencyOut(target)+"/lib\ncargo:rustc-env=GEN="+out+"/gen\ncargo:rerun-if-changed=build.rs\n";
 mkdirSync(join(a.workspace,"pkg"),{recursive:true});writeFileSync(join(a.workspace,"pkg","build.rs"),"fn main(){}\n");
 mkdirSync(join(b.workspace,"pkg"),{recursive:true});writeFileSync(join(b.workspace,"pkg","build.rs"),"fn main(){}\n");
 const entry=publishBuildRunV1(cache,{key,package:fixture.package.name,cwd:join(a.workspace,"pkg"),laneRoot:a.lane,directives:{files:["build.rs"],variables:[]},environment:environmentOf(a),outDir:outA,stdout:stdout(outA,a),stderr:"",stampDirectory:undefined});
 expect(entry).toBeDefined();
 const found=lookupBuildRunV1(cache,keys(b).local,{cwd:join(b.workspace,"pkg"),laneRoot:b.lane,environment:environmentOf(b),stampDirectory:undefined});
 expect(found?.key).toBe(key);
 mkdirSync(join(b.lane,"b","release","build","zlib-"+b.dependencyHash,"out","lib"),{recursive:true});
 const streams=restoreBuildRunV1(cache,found!,{outDir:outB,laneRoot:b.lane,environment:environmentOf(b)});
 expect(readFileSync(join(outB,"gen","table.rs"),"utf8")).toBe(text(outB,b));
 expect(readFileSync(join(outB,"blob.bin"))).toEqual(readFileSync(join(outA,"blob.bin")));
 expect(streams.stdout).toBe(stdout(outB,b));
 expect(streams.stdout.includes(a.lane)||streams.stdout.includes(a.dependencyHash)).toBe(false);
});

test("the key equals an independent recomputation by python's hashlib over the same portable inputs",()=>{
 const target=fixture.targets[2]!,roots=rootsOf(target),input=inputOf(target);
 const rows=input.environment.map(([name,value])=>[name,portableTextV1(value,roots)]);
 const run=spawnSync("python3",["-I","-c","import hashlib,json,sys;d=json.load(sys.stdin);print(hashlib.sha256(json.dumps([d['namespace'],d['toolchain'],d['normalized'],d['source'],[],d['environment']],separators=(',',':'),ensure_ascii=False).encode()).hexdigest())"],{input:JSON.stringify({namespace:UNIT_CACHE_POLICY_V1.namespace,toolchain:fixture.package.rust,normalized:input.normalized,source:input.sourceSha256,environment:rows}),encoding:"utf8"});
 expect(run.status).toBe(0);expect(run.stdout.trim()).toBe(keys(target).local);expect(sha256V1("x")).toHaveLength(64);
});
