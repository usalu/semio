/** 🔖️ Law: an inferrer that changes its output for the same inputs must declare a new stamp. The corpus in `./🔣️.json` is language-agnostic data (calls with JSON arguments and fixture files); the sha256 of the canonical JSON of every output is recorded per declared stamp. The law fails when a probe answers differently under a stamp that already has a recorded answer, and when a stamp has no recorded answer at all, so a bump and the refreshed record move together (`SEMIO_UPDATE_OUTPUT_STAMPS=1` rewrites the record of the current stamps). The canonical form and the digests are recomputed by an independent Python implementation. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import protocolSchema from "../../🔖️stamps/🧬️schema/🔣️.json";
import {observedSourceAnalysisV1} from "../../../../👁️observation/🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️.py"),fixturePath=join(here,"🔣️.json");
const protocol=protocolSchema.definitions.Protocol.const as {inferrers:Record<string,{stamp:string;scope:string}>};
const plugin=await import("../../🟨️.mjs");await plugin.libraryBootstrap;
const api=plugin.cacheInternals as Record<string,(...args:unknown[])=>unknown>,library=resolve(here,"../../../../..");

/** 🧾️ Sorted keys at every depth, no whitespace: the canonical form every language reproduces. */
export const canonical=(value:unknown):string=>JSON.stringify(sort(value));
const sort=(value:any):any=>Array.isArray(value)?value.map(sort):value&&typeof value==="object"?Object.fromEntries(Object.keys(value).sort().map(key=>[key,sort(value[key])])):value;
const digest=(value:unknown)=>createHash("sha256").update(canonical(value)).digest("hex");
const revive=(value:any,workspace:string):any=>typeof value==="string"?value.split("{ws}").join(workspace):Array.isArray(value)?value.map(item=>revive(item,workspace)):value&&typeof value==="object"?("$map" in value?new Map(value.$map.map((row:any)=>revive(row,workspace))):Object.fromEntries(Object.entries(value).map(([key,item])=>[key,revive(item,workspace)]))):value;

/** 🏃️ Runs one probe in its own fixture workspace and returns its output with the machine specific paths replaced. */
export async function runProbe(probe:any):Promise<unknown>{
 const workspace=mkdtempSync(join(realpathSync(tmpdir()),"output-stamps-"));
 try{
  for(const[path,text]of Object.entries(probe.files??{})){mkdirSync(dirname(join(workspace,path)),{recursive:true});writeFileSync(join(workspace,path),text as string);}
  let output:unknown;
  if(probe.analysis){const prepared=(plugin as any).prepareSourceAnalysesV1();output=observedSourceAnalysisV1(prepared[probe.analysis],join(workspace,probe.file));}
  else if(probe.call==="each"){const results=[];for(const args of probe.calls){try{results.push({value:await api[probe.fn]!(...revive(args,workspace))});}catch(error){results.push({error:String((error as Error).message)});}}output=results;}
  else{try{output={value:await api[probe.call]!(...revive(probe.args,workspace))};}catch(error){output={error:String((error as Error).message)};}}
  const text=JSON.stringify(output,(_key,value)=>value instanceof Map?[...value.entries()]:value);
  return JSON.parse(text.split(workspace).join("{ws}").split(library).join("{library}"));
 }finally{rmSync(workspace,{recursive:true,force:true});}
}

/** ⚖️ The violations of the law: a stamp without a record, an output that moved under a recorded stamp, a record of a stamp nobody declares, an inferrer without probes and without a reason. */
export function outputStampViolations({probes,unprobed,golden,digests,inferrers}:{probes:{id:string;inferrer:string}[];unprobed:Record<string,string>;golden:Record<string,Record<string,string>>;digests:Record<string,string>;inferrers:Record<string,{stamp:string}>}):string[]{
 const out:string[]=[],probed=new Set(probes.map(probe=>probe.inferrer));
 for(const id of Object.keys(inferrers)){
  if(!probed.has(id)&&!(id in unprobed))out.push(`inferrer ${id} has neither probes nor a reason in unprobed`);
  if(probed.has(id)&&id in unprobed)out.push(`inferrer ${id} is probed and listed as unprobed`);
 }
 for(const probe of probes)if(!(probe.inferrer in inferrers))out.push(`probe ${probe.id} names the undeclared inferrer ${probe.inferrer}`);
 for(const id of probed){
  const stamp=inferrers[id]?.stamp;if(stamp===undefined)continue;
  const recorded=golden[stamp];
  if(recorded===undefined){out.push(`stamp ${stamp} has no recorded output: refresh the record (SEMIO_UPDATE_OUTPUT_STAMPS=1) together with the bump`);continue;}
  for(const probe of probes.filter(probe=>probe.inferrer===id)){
   if(recorded[probe.id]===undefined)out.push(`stamp ${stamp} has no recorded output for ${probe.id}: refresh the record`);
   else if(recorded[probe.id]!==digests[probe.id])out.push(`inferrer ${id} changed the output of ${probe.id} under the same stamp ${stamp}: bump the stamp and refresh the record`);
  }
 }
 const declared=new Set(Object.values(inferrers).map(row=>row.stamp));
 for(const stamp of Object.keys(golden))if(!declared.has(stamp))out.push(`the record names the stamp ${stamp}, which no inferrer declares: refresh the record`);
 return out;
}

const digests:Record<string,string>={},outputs:Record<string,unknown>={};
for(const probe of fixture.probes){outputs[probe.id]=await runProbe(probe);digests[probe.id]=digest(outputs[probe.id]);}
const current=()=>({probes:fixture.probes as never,unprobed:fixture.unprobed as Record<string,string>,golden:fixture.golden as Record<string,Record<string,string>>,digests,inferrers:protocol.inferrers});

test("the corpus is closed under its schema and every probe is deterministic",async()=>{
  const ajv=new Ajv({strict:true});expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(new Set(fixture.probes.map(probe=>probe.id)).size).toBe(fixture.probes.length);
 for(const probe of fixture.probes)expect(digest(await runProbe(probe)),probe.id).toBe(digests[probe.id]);
},60000);

test("the canonical form and the digests are the ones an independent implementation computes",()=>{
 const directory=mkdtempSync(join(realpathSync(tmpdir()),"output-stamps-py-")),file=join(directory,"outputs.json");
 try{
  writeFileSync(file,JSON.stringify(outputs));
  const model=JSON.parse(Bun.spawnSync([python!,"-I",oracle,"digest",file]).stdout.toString());
  expect(model).toEqual(digests);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the output of every probed inferrer is the recorded one under its declared stamp",()=>{
 if(process.env.SEMIO_UPDATE_OUTPUT_STAMPS==="1"){
  const golden:Record<string,Record<string,string>>={};
  for(const probe of fixture.probes){const stamp=protocol.inferrers[probe.inferrer]!.stamp;(golden[stamp]??={})[probe.id]=digests[probe.id]!;}
  writeFileSync(fixturePath,JSON.stringify(sort({...fixture,golden}),null,1)+"\n");
  return;
 }
 expect(outputStampViolations(current())).toEqual([]);
});

test("the law fails when an inferrer changes an output without a new stamp, and passes when the stamp and the record move together",()=>{
 const base=current(),golden:Record<string,Record<string,string>>={};
 for(const probe of fixture.probes){const stamp=protocol.inferrers[probe.inferrer]!.stamp;(golden[stamp]??={})[probe.id]=digests[probe.id]!;}
 expect(outputStampViolations({...base,golden})).toEqual([]);
 const moved:Record<string,string>={...digests,"target-policy/policy-build":"0".repeat(64)};
 expect(outputStampViolations({...base,golden,digests:moved})).toEqual([`inferrer target-policy changed the output of target-policy/policy-build under the same stamp ${protocol.inferrers["target-policy"]!.stamp}: bump the stamp and refresh the record`]);
 const next=protocol.inferrers["target-policy"]!.stamp.replace(/\/(\d+)$/u,(_,n)=>`/${Number(n)+1}`),bumped={...protocol.inferrers,"target-policy":{...protocol.inferrers["target-policy"]!,stamp:next}};
 expect(outputStampViolations({...base,golden,digests:moved,inferrers:bumped}).sort()).toEqual([`stamp ${next} has no recorded output: refresh the record (SEMIO_UPDATE_OUTPUT_STAMPS=1) together with the bump`,`the record names the stamp ${protocol.inferrers["target-policy"]!.stamp}, which no inferrer declares: refresh the record`].sort());
 const refreshed={...golden};delete refreshed[protocol.inferrers["target-policy"]!.stamp];refreshed[next]=Object.fromEntries(fixture.probes.filter(probe=>probe.inferrer==="target-policy").map(probe=>[probe.id,moved[probe.id]??digests[probe.id]!]));
 expect(outputStampViolations({...base,golden:refreshed,digests:moved,inferrers:bumped})).toEqual([]);
 expect(outputStampViolations({...base,golden,unprobed:{}})).toContain("inferrer coordinates has neither probes nor a reason in unprobed");
});
