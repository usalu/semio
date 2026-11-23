import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {declareV1,judgeStalenessV1,openTreeDigestsV1,stalenessOnArrivalV1,workspaceRowsV1,type StalenessEntryV1,type StalenessVerdictV1} from "../🟦️.ts";
import fixtures from "./🧫️fixtures/🔣️.json" with {type:"json"};

const python=testPythonV1(),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`staleness-${label}-`));scratch.push(path);return path;};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
type Scenario=typeof fixtures.scenarios[number];
const label=(verdict:StalenessVerdictV1):string=>verdict.state==="stale"?`stale:${verdict.reason}`:verdict.state;
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const [path,text] of Object.entries(files)){const full=join(root,path);mkdirSync(dirname(full),{recursive:true});writeFileSync(full,text);}};
const entriesOf=(scenario:Scenario):StalenessEntryV1[]=>scenario.entries.map(row=>({key:row.key,storedAt:row.storedAt,refs:row.refs.map(sha),declaration:row.declared?{...declareV1({kind:(row.kind??"rust-unit") as "rust-unit",toolchain:sha(row.toolchain!),sources:Object.entries(row.sources!).sort(([a],[b])=>a<b?-1:1).map(([path,text])=>({path,sha256:sha(text)})),externs:row.externs!.map(sha),self:row.self===undefined?undefined:sha(row.self),units:(row.units??[]).map(sha)}),...row.version===undefined?{}:{version:row.version as 2}}:undefined}));

test("the verdicts equal the expectation of every scenario of the shared fixture and the independent Python model",async()=>{
 for(const scenario of fixtures.scenarios){
  const root=directory("judge"),tree=await openTreeDigestsV1(root);write(root,scenario.tree);
  const verdicts=await judgeStalenessV1(entriesOf(scenario),{now:scenario.now,graceMilliseconds:scenario.grace,digest:tree.digest,toolchains:scenario.toolchains===null?undefined:new Set(scenario.toolchains.map(sha)),retired:new Set(scenario.retired.map(sha))});
  expect(Object.fromEntries([...verdicts].map(([key,verdict])=>[key,label(verdict)])),scenario.name).toEqual(scenario.expected);
 }
});

test.skipIf(!python)("the independent Python model reaches the same verdicts",()=>{
 const result=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify(fixtures)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString()) as Record<string,string>[];
 fixtures.scenarios.forEach((scenario,index)=>expect(model[index],scenario.name).toEqual(scenario.expected));
});

test("tree digests equal the SHA-256 of the file, are remembered by size, time and inode, survive a restart and notice an edit",async()=>{
 const root=directory("tree"),memo=join(directory("memo"),"digests.jsonl");write(root,{"a/x.rs":"one","b.rs":"two"});
 const first=await openTreeDigestsV1(root,memo);
 expect(await first.digest("a/x.rs")).toBe(sha("one"));expect(await first.digest("b.rs")).toBe(sha("two"));expect(await first.digest("missing.rs")).toBeNull();expect(await first.digest("a")).toBeNull();expect(await first.digest("../escape")).toBeNull();
 expect(first.hashed).toBe(2);await first.digest("a/x.rs");expect(first.hashed,"a second ask costs a stat").toBe(2);await first.flush();
 const second=await openTreeDigestsV1(root,memo);expect(await second.digest("a/x.rs")).toBe(sha("one"));expect(second.hashed,"the memory survived the restart").toBe(0);
 writeFileSync(join(root,"a/x.rs"),"changed");utimesSync(join(root,"a/x.rs"),new Date(Date.now()+5000),new Date(Date.now()+5000));
 expect(await second.digest("a/x.rs")).toBe(sha("changed"));expect(second.hashed).toBe(1);
});

test("workspace rows keep sources below the workspace, drop generated roots, lane tokens, registry files and unreadable files, and sort",()=>{
 const workspace="/work/tree",rows=workspaceRowsV1({workspace,cwd:workspace,generatedRoots:[".cache","target"],dependencies:[
  {path:"zeta/src/lib.rs",sha256:sha("z")},{path:"/work/tree/alpha/src/lib.rs",sha256:sha("a")},{path:"/work/tree/.cache/b/out/gen.rs",sha256:sha("g")},{path:"target/debug/build/x/out/y.rs",sha256:sha("t")},
  {path:"/home/u/.cargo/registry/src/serde/lib.rs",sha256:sha("s")},{path:"@@semio-lane@@/x.rs",sha256:sha("l")},{path:"broken/lib.rs",sha256:null},{path:"zeta/src/lib.rs",sha256:sha("z")}]});
 expect(rows).toEqual([{path:"alpha/src/lib.rs",sha256:sha("a")},{path:"zeta/src/lib.rs",sha256:sha("z")}]);
});

test("the publish gate refuses a declaration whose sources differ, looks again after the recheck so a lagging replica passes, and refuses a toolchain that is not the fleet's",async()=>{
 const root=directory("gate"),tree=await openTreeDigestsV1(root);write(root,{"c/lib.rs":"v2"});
 const declaration=(text:string,toolchain="t1")=>declareV1({kind:"rust-unit",toolchain:sha(toolchain),sources:[{path:"c/lib.rs",sha256:sha(text)}],externs:[]});
 const waits:number[]=[],input={digest:tree.digest,recheckMilliseconds:7,wait:async(milliseconds:number)=>{waits.push(milliseconds);}};
 expect(await stalenessOnArrivalV1(declaration("v2"),input)).toBeUndefined();expect(waits).toEqual([]);
 expect(await stalenessOnArrivalV1(declaration("v1"),input)).toEqual({reason:"edited",detail:"c/lib.rs"});expect(waits).toEqual([7]);
 const lagging={...input,wait:async()=>{writeFileSync(join(root,"c/lib.rs"),"v1-replicated-late");utimesSync(join(root,"c/lib.rs"),new Date(Date.now()+9000),new Date(Date.now()+9000));}};
 expect(await stalenessOnArrivalV1(declaration("v1-replicated-late"),lagging),"the tree caught up during the recheck").toBeUndefined();
 expect((await stalenessOnArrivalV1(declareV1({kind:"rust-unit",toolchain:sha("t1"),sources:[{path:"gone.rs",sha256:sha("x")}],externs:[]}),input))?.reason).toBe("deleted");
 expect((await stalenessOnArrivalV1(declaration("v1-replicated-late","t1"),{...input,toolchains:new Set([sha("t2")])}))?.reason).toBe("toolchain");
});
