/** 🔖️ Contract: the inference protocol stamps. Every inferrer carries one declared stamp, a record or generation fact names the stamps it took part in instead of the bytes of the code that ran, the declared sources classify every file of the loaded implementation, and a stamp row holds exactly while the process carries the stamp it was recorded with. The classification and the admission rules are answered a second time by an independent Python oracle. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {existsSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve,sep} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import roster from "../../🗂️components/🔣️.json";
import {admitInferenceProtocolV1,classifyImplementationRowsV1,inferenceFragmentsV1,inferenceStampEntriesV1,matchesFragmentsV1,unclassifiedCodeV1} from "../🟨️.mjs";
import {captureObservedSourceInputsV1,observationsHold,observedInferenceStampsV1,registerInferenceStampsV1} from "../../../../👁️observation/🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️.py"),schemaPath=join(here,"..","🧬️schema","🔣️.json");
const library=resolve(here,"../../../../.."),workspace=resolve(library,"../../../../..");
const protocol=schema.definitions.Protocol.const as never;
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-stamps-"));
const ask=(...args:string[])=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,...args]).stdout.toString());
const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value));
const at=(root:any,path:readonly (string|number)[])=>path.reduce((value,key)=>value[key],root);
const rowOf=(path:string)=>({kind:"digest",path,fact:{sha256:"0".repeat(64),size:1,mtimeMs:1}});

test("the fixture and the declared protocol are closed under their schemas, validated by an independent implementation",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 const document=new Ajv({strict:false}).compile({definitions:schema.definitions,$ref:"#/definitions/Document"});
 expect(document(protocol),JSON.stringify(document.errors)).toBe(true);
 expect(admitInferenceProtocolV1(clone(protocol))).toEqual(protocol);
});

for(const row of fixture.refusals)test("protocol admission: "+row.id,()=>{
 const value=clone(protocol) as any;
 for(const step of row.mutate as any[]){
  const parent=step.path.length>1?at(value,step.path.slice(0,-1)):value,key=step.path.at(-1);
  if("fromPath" in step)parent[key]=at(value,step.fromPath);else parent[key]=step.value;
 }
 const file=join(scratch(),"protocol.json");writeFileSync(file,JSON.stringify(value));
 try{
  const model=ask("admit",file);
  if(row.expected==="ok"){expect(model).toEqual({ok:true});expect(admitInferenceProtocolV1(value)).toBeTruthy();}
  else{expect(model.error,"the oracle refuses").toBeTruthy();expect(()=>admitInferenceProtocolV1(value)).toThrow();}
 }finally{rmSync(dirname(file),{recursive:true,force:true});}
});

test("classification keeps the data nobody covers and drops covered code, infrastructure and stamped files, like the independent oracle",()=>{
 const rows=fixture.rows.map(row=>rowOf(row.path)),file=join(scratch(),"rows.json");writeFileSync(file,JSON.stringify(rows));
 try{
  const model=ask("classify",schemaPath,file),actual=classifyImplementationRowsV1(rows as never,protocol);
  expect(JSON.parse(JSON.stringify(actual))).toEqual(model);
  const kept=new Set(actual.filter(row=>row.kind!=="stamp").map(row=>row.path));
  for(const row of fixture.rows)expect(kept.has(row.path),row.path).toBe(row.cls==="keep");
  const stamps=actual.filter(row=>row.kind==="stamp");
  expect(stamps.map(row=>row.path)).toEqual(Object.keys((protocol as any).inferrers).map(id=>"inferrer:"+id));
  expect(stamps.every(row=>(protocol as any).inferrers[row.path.slice(9)].stamp===row.fact&&(protocol as any).inferrers[row.path.slice(9)].scope===row.scope)).toBe(true);
 }finally{rmSync(dirname(file),{recursive:true,force:true});}
});

const walk=(directory:string,out:string[]=[]):string[]=>{
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  if(entry.name==="node_modules"||entry.name==="target"||entry.name.startsWith(".git"))continue;
  const path=join(directory,entry.name);if(entry.isDirectory())walk(path,out);else out.push(relative(workspace,path).split(sep).join("/"));
 }
 return out;
};

test("every executable file of the loaded implementation and of the graph host is claimed by an inferrer or by infrastructure, and no fragment is dead",async()=>{
 const plugin=await import("../../🟨️.mjs");await plugin.libraryBootstrap;
 const loaded=plugin.loadedImplementationRowsV1.map((row:{path:string})=>({kind:"digest",path:relative(workspace,row.path).split(sep).join("/")}));
 const source=readFileSync(join(library,"🏗️build","🕸️graph","🟦️.ts"),"utf8"),host=[...source.split("\n").find(line=>line.startsWith("const graphImplementationUrlsV1"))!.matchAll(/new URL\("([^"]+)"/gu)].map(match=>relative(workspace,resolve(library,"🏗️build","🕸️graph",match[1]!)).split(sep).join("/")).map(path=>({kind:"digest",path}));
 expect(loaded.length).toBeGreaterThan(50);expect(host.length).toBeGreaterThan(20);
 expect(unclassifiedCodeV1([...loaded,...host] as never,protocol)).toEqual([]);
 const files=[...walk(library),...walk(join(workspace,"🧰️framework","🔨️modules")),...loaded.map((row:{path:string})=>row.path)];
 for(const fragment of inferenceFragmentsV1(protocol))expect(files.some(path=>path.includes(fragment)),"dead fragment "+fragment).toBe(true);
},60000);

test("every component of the inference roster is stamped or declared infrastructure",()=>{
 for(const entry of roster.components){
  const directory=entry.path.slice(2,entry.path.lastIndexOf("/")),path=join("🏗️build","🕸️graph","📥️inference",directory,"🟨️.mjs").split(sep).join("/");
  const classified=classifyImplementationRowsV1([rowOf(path)] as never,protocol);
  expect(classified.some(row=>row.kind!=="stamp"),entry.name).toBe(false);
 }
 const stamped=new Set(Object.keys((protocol as any).inferrers));
 for(const name of["project-nodes","project-assembly","target-policy","native-manifests","script-inputs","source-analyses","bun-lock","generator-outputs","component-targets","playground-targets","owner-discovery","dependency-edges","coordinates","bootstrap"])expect(stamped.has(name),name).toBe(true);
});

test("a stamp row holds exactly while the process carries the stamp it was recorded with",async()=>{
 registerInferenceStampsV1(inferenceStampEntriesV1(protocol),inferenceFragmentsV1(protocol));
 const recorded=await captureObservedSourceInputsV1(async()=>{observedInferenceStampsV1(["inferrer:target-policy","inferrer:native-manifests"]);},undefined,false);
 expect(recorded.observations.map(row=>[row.kind,row.path,row.fact])).toEqual([["stamp","inferrer:target-policy",schema.definitions.Protocol.const.inferrers["target-policy"].stamp],["stamp","inferrer:native-manifests",schema.definitions.Protocol.const.inferrers["native-manifests"].stamp]]);
 expect(await observationsHold(recorded.observations as never)).toBe(true);
 registerInferenceStampsV1([["inferrer:native-manifests","semio.graph-inferrer.native-manifests/2"]]);
 expect(await observationsHold([recorded.observations[0]!] as never)).toBe(true);
 expect(await observationsHold([recorded.observations[1]!] as never)).toBe(false);
 expect(await observationsHold([{kind:"stamp",path:"inferrer:never-declared",fact:"semio.graph-inferrer.never-declared/1"}] as never)).toBe(false);
 registerInferenceStampsV1(inferenceStampEntriesV1(protocol),inferenceFragmentsV1(protocol));
 expect(await observationsHold(recorded.observations as never)).toBe(true);
});

test("no record, parser identity or generation fact is keyed by the bytes of the implementation",()=>{
 const inference=join(library,"🏗️build","🕸️graph","📥️inference"),read=(...path:string[])=>readFileSync(join(inference,...path),"utf8");
 for(const component of readdirSync(inference,{withFileTypes:true}).filter(entry=>entry.isDirectory()).map(entry=>entry.name)){
  const file=join(inference,component,"🟨️.mjs");if(!existsSync(file))continue;
  if(["🚦️invocation","🪢️revision-graph","♻️revision","🧰️internals"].includes(component))continue;
  expect(readFileSync(file,"utf8").includes("revisionIdentity.revision"),component).toBe(false);
 }
 expect(read("📦️project-nodes","🟨️.mjs")).toContain("projectStampPathsV1({native})");
 expect(read("🧬️source-analyses","🟨️.mjs")).toContain("inferrerStampV1(\"source-analyses\")");
 const observation=readFileSync(join(library,"🏗️build","👁️observation","🟨️.mjs"),"utf8");
 expect(observation).toContain("[recordStampV1(),inferenceAuthorityDigestV1(),key]");
 expect(observation).toContain("nestedProject?recordStampV1():OBSERVATION_REVISION");
 expect(observation).toContain("stampedFragments.some(fragment=>row.path.includes(fragment))");
 const host=readFileSync(join(library,"🏗️build","🕸️graph","🟦️.ts"),"utf8");
 expect(host).toContain("producer.classifyImplementationRowsV1(rows)");expect(host).toContain("generationSchemaV1.definitions.Identity.const");expect(readFileSync(join(library,"🏗️build","🕸️graph","🕰️generations","🧬️schema","🔣️.json"),"utf8")).toContain("graph-generations-v4");
 const generations=readFileSync(join(library,"🏗️build","🕸️graph","🕰️generations","🟨️.mjs"),"utf8");
 expect(generations.includes("implementation:Object.freeze({none:")).toBe(false);
});

test("the matcher is substring containment of workspace relative paths",()=>{
 expect(matchesFragmentsV1("a/b/c.mjs",["b/"])).toBe(true);expect(matchesFragmentsV1("a/b/c.mjs",["x/"])).toBe(false);
});
