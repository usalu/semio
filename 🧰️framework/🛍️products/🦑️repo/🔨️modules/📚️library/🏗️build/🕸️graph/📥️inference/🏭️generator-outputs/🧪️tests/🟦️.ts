/** 🏭 Contract: the coupling of generator contracts to Build producers (contract inputs, output ownership, declared output paths, check/generate coupling and the content digest of an output root that Nx cannot see) equals the authored cases in `./🔣️.json` and an independent Python derivation; the digest command the graph hands to Nx, run for real, prints the digest of a tree built from the corpus. */
import {afterAll,beforeAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {generatorOutputOwners} from "../🟨️.mjs";
import policy from "../../../../../⚡️caching/🔣️policy.json";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const inference=await import("../../🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py"),posix=(path:string)=>process.platform==="win32"?path.replace(/^[A-Za-z]:/u,"").replaceAll("\\","/"):path;
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({policy:policy.generatorInputs,workspaceRoot:corpus.workspaceRoot,requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const outcome=(action:()=>unknown):{result?:unknown;error?:string}=>{try{return{result:action()};}catch(error){return{error:(error as Error).message};}};
const runtimeOf=(absolute:string):string=>cacheInternals.outputRootInputs({path:absolute},corpus.workspaceRoot).runtime;
let workspace="";
beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"gen-outputs-")));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,workspaceRoot:"ws"})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,digests:[{...corpus.digests[0],expected:"abc"}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,owners:[{...corpus.owners[0],expected:[],error:"x"}]})).toBe(false);
});

test("a contract authors its patterns, the fingerprint output of its discovery kind and the producer of that fingerprint",()=>{
 const answers=ask(corpus.inputs.map(row=>({kind:"inputs",contract:row.contract})));
 for(const[index,row]of corpus.inputs.entries()){
  const actual=outcome(()=>cacheInternals.generatorContractInputs(structuredClone(row.contract)));
  let expected=(row as any).expected;
  if("fingerprint" in row){const entry=(policy.generatorInputs as any)[row.fingerprint as string];expected={inputs:[...row.contract.inputPatterns.map(path=>"{workspaceRoot}/"+path),{dependentTasksOutputFiles:entry.output}],dependsOn:[entry.target]};}
  if("error" in row){expect(actual.error,row.name).toBe(row.error);expect(answers[index].error,row.name).toBe(row.error);}
  else{expect(actual.error,row.name).toBeUndefined();expect(actual.result,row.name).toEqual(expected);expect(answers[index].result,row.name).toEqual(expected);}
 }
});

test("outputs go to the Build producer of their root and an output without a producer in the project is refused",()=>{
 const answers=ask(corpus.owners.map(row=>({kind:"owners",contracts:row.contracts,root:row.root,project:row.project,targets:row.targets})));
 for(const[index,row]of corpus.owners.entries()){
  const targets=Object.fromEntries(row.targets.map(name=>[name,{}]));
  const actual=outcome(()=>[...generatorOutputOwners(structuredClone(row.contracts),row.root,row.project,targets)]);
  if("error" in row){expect(actual.error,row.name).toBe(row.error);expect(answers[index].error,row.name).toBe(row.error);}
  else{expect(actual.error,row.name).toBeUndefined();expect(actual.result,row.name).toEqual(row.expected);expect(answers[index].result,row.name).toEqual(row.expected);}
 }
});

test("a declared output string resolves against the project or workspace root, and only literally",()=>{
 const answers=ask(corpus.paths.map(row=>({kind:"path",output:row.output,root:row.root})));
 for(const[index,row]of corpus.paths.entries()){
  const actual=cacheInternals.resolveOutputPath(row.output,row.root,corpus.workspaceRoot),shown=actual===undefined?null:posix(actual);
  expect(shown,row.output+" in "+row.root).toBe(row.expected);
  expect(answers[index].result,row.output+" in "+row.root).toBe(row.expected);
 }
});

test("a check target digests the current bytes of its generate siblings' outputs exactly once each",()=>{
 const answers=ask(corpus.couplings.map(row=>({kind:"coupling",name:row.target,authored:(row as any).authored??[],targets:row.targets,root:row.root})));
 for(const[index,row]of corpus.couplings.entries()){
  const authored=[...((row as any).authored??[]).map(runtimeOf).map((runtime:string)=>({runtime})),...((row as any).authoredOther??[])];
  const actual=cacheInternals.generatorOutputCouplingInputs(row.target,{inputs:authored},structuredClone(row.targets),row.root,corpus.workspaceRoot);
  expect(actual.map((entry:any)=>entry.runtime),row.name).toEqual(row.expected.map(runtimeOf));
  expect(answers[index].result,row.name).toEqual(row.expected);
  for(const entry of actual)expect(Object.keys(entry)).toEqual(["runtime"]);
 }
});

const node=Bun.which("node"),exec=node?test:test.skip;
exec("the digest command printed for an output root equals the content digest of the tree, in sorted order and without links",()=>{
 const base=join(workspace,"digests");
 for(const[index,row]of corpus.digests.entries()){
  const root=join(base,String(index));mkdirSync(root,{recursive:true});
  for(const directory of (row as any).directories??[])mkdirSync(join(root,directory),{recursive:true});
  for(const[path,text]of Object.entries(row.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
  if(process.platform!=="win32")for(const[path,target]of Object.entries<string>((row as any).links??{}))symlinkSync(target,join(root,path));
  const absolute=join(root,row.path),command=cacheInternals.outputRootInputs({path:row.path},root).runtime as string;
  expect(command.startsWith("node -e ")).toBe(true);
  const run=spawnSync(command,{shell:true,encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);
  expect(run.stdout,row.name).toBe(row.expected);
  expect(ask([{kind:"digest",path:absolute}])[0].result,row.name).toBe(row.expected);
 }
});
