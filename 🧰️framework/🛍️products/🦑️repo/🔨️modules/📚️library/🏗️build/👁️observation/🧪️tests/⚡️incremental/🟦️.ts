import {withRepositoryBuildInferenceDatasetV1,buildGraphOptionalRetirementDueV1} from "../../../📥️inputs/💾️storage/🟨️.mjs";
import {buildGraphMembersV1} from "../../../🕸️graph/🟦️.ts";
import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,writeFileSync,rmSync,existsSync} from "node:fs";
import {join,dirname,resolve,relative} from "node:path";
import {createHash} from "node:crypto";
import {createRequire} from "node:module";
import {pathToFileURL} from "node:url";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import picomatch from "picomatch";
import fixture from "../🧫️fixtures/⚡️incremental/🔣️.json";
import schema from "../../🧬️schema/⚡️incremental/🔣️.json";
import graphSchema from "../../../🧬️schema/🔣️.json";
import policy from "../../../🔣️policy.json";
import {scriptInvocationEnvironment} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";

type Snapshot=Record<string,string>;
type Outcome={result:string;counts:Record<string,number>;raced?:boolean;persisted?:{records:number;foreignLibraryFacts:number;authorityRows:number}};

/** 🧪️ Loads actual TypeScript Source in Node with the admitted immutable compiler. */
function nodeTypeScriptLoaderV1(compiler:string):string{
 return `require('node:module').registerHooks({load(url,context,next){if(/\\.tsx?(?:\\?|$)/u.test(url)){const ts=require(${JSON.stringify(compiler)}),source=require('node:fs').readFileSync(new URL(url),'utf8');return{format:'module',source:ts.transpileModule(source,{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText,shortCircuit:true};}return next(url,context)}});`;
}

/** 🎬️ One Node process per pass family: `oracle` is a cold serial full inference of one snapshot; `scenario` replays the concurrent-edit timeline in parallel. */
const childSourceV1=`const fs=require('node:fs'),{join,dirname}=require('node:path'),crypto=require('node:crypto');(async()=>{const routing=await import(process.env.SEMIO_G1_ROUTING),storage=await import(process.env.SEMIO_G1_STORAGE);await routing.receiveScriptProcessInvocation(process.env,original=>storage.withRepositoryBuildInferenceDatasetV1(process.env.SEMIO_G1_WORKSPACE,original,routing.createIssuedResourceControlV1(original,"graph:node-dataset",0),JSON.parse(process.env.SEMIO_G1_DATASET),async(scope,phaseControl)=>{
const fixture=JSON.parse(fs.readFileSync(process.env.SEMIO_G1_FIXTURE,'utf8')),cwd=process.cwd(),originalRead=fs.readFileSync,originalWrite=fs.writeFileSync,write=(path,content)=>{fs.mkdirSync(dirname(join(cwd,path)),{recursive:true});originalWrite(join(cwd,path),content);},age=(path,milliseconds)=>{const at=new Date(Date.now()-milliseconds);fs.utimesSync(join(cwd,path),at,at);};
await new Promise(resolve=>setTimeout(resolve,2200));const plugin=await import(process.env.SEMIO_G1_PLUGIN),observation=await import(process.env.SEMIO_G1_OBSERVER);await plugin.libraryBootstrap;
const files=fixture.owners.map(root=>root+'/📋️project.json'),run=async concurrency=>JSON.stringify(await plugin.default.createNodesV2[1](files,{projectConcurrency:concurrency},{workspaceRoot:cwd,invocation:original,phaseControl})),counts=()=>({...observation.projectInferenceCountsV1()}),hook=(path,after)=>{let fired=false;fs.readFileSync=function(file,...args){const bytes=originalRead.call(this,file,...args);if(!fired&&String(file)===join(cwd,path)){fired=true;after();}return bytes;};require('node:module').syncBuiltinESMExports();return()=>{fs.readFileSync=originalRead;require('node:module').syncBuiltinESMExports();return fired;};};
if(process.env.SEMIO_G1_MODE==='oracle'){console.log(JSON.stringify({result:await run(fixture.concurrency.serial),counts:counts()}));return;}
const out={};write(fixture.race.extra.path,fixture.race.extra.content);const release=hook(fixture.race.path,()=>write(fixture.race.path,fixture.race.content));
out.race={result:await run(fixture.concurrency.parallel),counts:counts(),raced:release()};
out.warm={result:await run(fixture.concurrency.parallel),counts:counts()};
write(fixture.stale.extra.path,fixture.stale.extra.content);write(fixture.stale.path,fixture.stale.content);out.stale={result:await run(fixture.concurrency.parallel),counts:counts()};
const declaration=originalRead(join(cwd,fixture.unparseable.source),'utf8');write(fixture.unparseable.source,fixture.unparseable.content);age(fixture.unparseable.source,fixture.unparseable.ageMilliseconds);out.unparseable={result:await run(fixture.concurrency.parallel),counts:counts()};
write(fixture.unparseable.source,declaration);out.restored={result:await run(fixture.concurrency.parallel),counts:counts()};
write(fixture.halfWritten.source,fixture.halfWritten.partial);setTimeout(()=>write(fixture.halfWritten.source,declaration),fixture.halfWritten.completeAfterMilliseconds);out.halfWritten={result:await run(fixture.concurrency.parallel),counts:counts()};
const library=process.env.SEMIO_G1_LIBRARY,directory=join(scope.root,'np'),records=fs.existsSync(directory)?fs.readdirSync(directory).filter(name=>name.endsWith('.json')):[];let foreignLibraryFacts=0,authorityRows=0;for(const name of records){const text=originalRead(join(directory,name),'utf8'),header=JSON.parse(text.slice(0,text.indexOf('\\n')));authorityRows+=header.authorityObservations.length;foreignLibraryFacts+=header.observations.filter(row=>row.kind==='file'&&row.path.startsWith(library)).length;}
out.halfWritten.persisted={records:records.length,foreignLibraryFacts,authorityRows};console.log(JSON.stringify(out));}));})().catch(error=>{console.error(error);process.exit(1)});`;

/** 🧫️ Materializes one source snapshot in its own artifact root. */
function materializeV1(root:string,files:Snapshot):void{for(const[path,content]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}}

test("parallel incremental inference settles concurrent edits onto serial full inference and withholds only unparseable projects",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR,identity=process.env.SEMIO_NX_TOOLING_IDENTITY;if(!artifacts||!identity)throw Error("Original artifact namespace and immutable SDK required");
 const sdk=createRequire(join(process.cwd(),".cache/b/t/m/nx",identity,"package.json")),compiler=sdk.resolve("typescript"),library=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),input=join(mkdtempSync(join(artifacts,"g1-")),"fixture.json");writeFileSync(input,JSON.stringify(fixture));
 const initial:Snapshot={...fixture.files},raced:Snapshot={...initial,[fixture.race.path]:fixture.race.content,[fixture.race.extra.path]:fixture.race.extra.content},stale:Snapshot={...raced,[fixture.stale.path]:fixture.stale.content,[fixture.stale.extra.path]:fixture.stale.extra.content},broken:Snapshot={...stale,[fixture.unparseable.source]:fixture.unparseable.content};
 const runs=[{name:"scenario",files:initial},{name:"raced",files:raced},{name:"stale",files:stale},{name:"broken",files:broken}].map(run=>{const root=mkdtempSync(join(artifacts,"g1-"+run.name+"-"));materializeV1(root,run.files);return {...run,root,dataset:{version:1,identity:createHash("sha256").update(root).digest("hex"),sourceRoot:relative(process.cwd(),root).replaceAll("\\","/")}};});
 try{
  const outputs=await withBuildTestInvocationV1(original=>Promise.all(runs.map(async run=>{
   const brokenAge=run.name==="broken"?"const{utimesSync}=require('node:fs'),at=new Date(Date.now()-"+fixture.unparseable.ageMilliseconds+");utimesSync("+JSON.stringify(join(run.root,fixture.unparseable.source))+",at,at);":"";
   const output=await captureOwnedCommandV1("node",["--conditions","development","-e",nodeTypeScriptLoaderV1(compiler)+brokenAge+childSourceV1],run.root,{control:scriptCompilerAdmissionControlV1(original,"g1-incremental:"+run.name,240000),env:{...scriptInvocationEnvironment(original,process.env),NODE_PATH:join(dirname(dirname(sdk.resolve("nx/package.json")))),NX_DAEMON:"false",SEMIO_G1_MODE:run.name==="scenario"?"scenario":"oracle",SEMIO_G1_WORKSPACE:process.cwd(),SEMIO_G1_DATASET:JSON.stringify(run.dataset),SEMIO_G1_FIXTURE:input,SEMIO_G1_LIBRARY:library,SEMIO_G1_STORAGE:pathToFileURL(join(library,"🏗️build/📥️inputs/💾️storage/🟨️.mjs")).href,SEMIO_G1_ROUTING:pathToFileURL(resolve("🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")).href,SEMIO_G1_PLUGIN:pathToFileURL(join(library,"🏗️build/🕸️graph/📥️inference/🟨️.mjs")).href,SEMIO_G1_OBSERVER:pathToFileURL(join(library,"🏗️build/👁️observation/🟨️.mjs")).href}});
   return [run.name,JSON.parse(output.trim().split("\n").findLast(row=>row.startsWith("{"))!)] as const;
  })));
  const result=Object.fromEntries(outputs) as {scenario:Record<string,Outcome>;raced:Outcome;stale:Outcome;broken:Outcome},scenario=result.scenario,owners=Object.fromEntries(fixture.owners.map(owner=>[owner,owner]));
  expect(scenario.race!.raced).toBe(true);
  expect(scenario.race!.result).toBe(result.raced.result);
  for(const root of fixture.race.untouched)expect(scenario.race!.counts[owners[root]!]).toBe(1);
  expect(scenario.warm!.counts).toEqual(scenario.race!.counts);
  expect(scenario.warm!.result).toBe(scenario.race!.result);
  expect(scenario.stale!.result).toBe(result.stale.result);
  for(const root of fixture.stale.recomputed)expect(scenario.stale!.counts[root]).toBeGreaterThan(scenario.warm!.counts[root]!);
  for(const root of fixture.stale.untouched)expect(scenario.stale!.counts[root]).toBe(scenario.warm!.counts[root]);
  expect(scenario.unparseable!.result).toBe(result.broken.result);
  for(const root of fixture.unparseable.untouched)expect(scenario.unparseable!.counts[root]).toBe(scenario.stale!.counts[root]);
  const ts=sdk(compiler),withheld=JSON.parse(scenario.unparseable!.result) as [string,{projects:Record<string,unknown>;unavailable?:unknown[]}][],rows=withheld.flatMap(([,row])=>row.unavailable??[]) as {root:string;source:string;sha256:string}[];
  expect(ts.parseConfigFileTextToJson("project.json",fixture.unparseable.content).error).toBeDefined();
  expect(rows.map(row=>[row.root,row.source,row.sha256])).toEqual([[fixture.unparseable.root,fixture.unparseable.source,createHash("sha256").update(fixture.unparseable.content).digest("hex")]]);
  const unavailable=new Ajv2020({strict:false}).addSchema(graphSchema,"graph").compile({$ref:"graph#/$defs/UnavailableProject"});for(const row of rows)expect(unavailable(row)).toBe(true);
  expect(withheld.flatMap(([,row])=>Object.keys(row.projects)).sort()).toEqual(fixture.owners.filter(root=>root!==fixture.unparseable.root).sort());
  expect(scenario.restored!.result).toBe(result.stale.result);
  expect(scenario.restored!.counts).toEqual(scenario.unparseable!.counts);
  expect(scenario.halfWritten!.result).toBe(result.stale.result);
  expect(scenario.halfWritten!.persisted!.records).toBeGreaterThan(0);
  expect(scenario.halfWritten!.persisted!.foreignLibraryFacts).toBe(fixture.foreignLibraryFacts);
  expect(scenario.halfWritten!.persisted!.authorityRows).toBe(0);
 }finally{
  try{await withBuildTestInvocationV1(original=>Promise.all(runs.map(run=>withRepositoryBuildInferenceDatasetV1(process.cwd(),original,scriptCompilerAdmissionControlV1(original,"graph:dataset-retirement",0),run.dataset,async scope=>{await scope.retire();expect(existsSync(scope.root)).toBe(false);}))));}
  finally{for(const run of runs)rmSync(run.root,{recursive:true,force:true});rmSync(dirname(input),{recursive:true,force:true});}
 }
},300_000);

test("optional graph cache retirement opens every writer session below the low-water mark",()=>{
 for(const row of fixture.retirement.cases)expect(buildGraphOptionalRetirementDueV1(row.usedBytes,row.availableBytes),row.id).toBe(row.due);
});

test("graph membership keeps exactly the declared consumed files",()=>{
 const members=(policy as {graphMembers:{basenames:string[];suffixes:string[]}}).graphMembers,oracle=picomatch([`**/{${members.basenames.join(",")}}`,...members.suffixes.map(suffix=>`**/*${suffix}`)],{dot:true}),files=fixture.members.files;
 expect(files.filter(file=>oracle(file))).toEqual(fixture.members.expected);
 expect(buildGraphMembersV1(files)).toEqual(fixture.members.expected);
});
