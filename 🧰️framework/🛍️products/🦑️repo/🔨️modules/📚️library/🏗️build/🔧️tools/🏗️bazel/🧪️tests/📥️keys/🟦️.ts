/** 🔑️ Contract: no per-invocation value (caller envelope, deadline, cwd, ambient environment) reaches a generated Bazel action input, and generator snapshots restore byte-identically, repair edits and refuse a missing declared output. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import type {ScriptInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";

const repo=process.cwd(),library=join(repo,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"),routing=join(repo,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),python=testPythonV1();
const api=await import(join(library,"🏗️build/🔧️tools/🏗️bazel/📜️script.ts")),executorApi=await import(join(library,"🏗️build/🔧️tools/🏗️bazel/🎬️executor/🟦️.ts")),{createNativeBuildCallerCapabilitiesV1}=await import(join(library,"🏗️build/📥️caller/🟦️.ts")),{createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1,scriptInvocationEnvironment}=await import(routing);
const sha=(text:string)=>createHash("sha256").update(text).digest("hex"),scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"bazel-keys-"));
const write=(root:string,files:Record<string,string>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};

/** 🧮️ Independent oracle: Python hashes both trees and lists the paths whose bytes or link targets differ. */
const oracleDifferences=(left:string,right:string):string[]=>{
 const program="import hashlib,json,os,sys\nleft,right=json.loads(sys.stdin.read())\ndef walk(base):\n out={}\n for d,_,fs in os.walk(base):\n  for f in fs:\n   p=os.path.join(d,f);r=os.path.relpath(p,base)\n   out[r]=('link:'+os.readlink(p)) if os.path.islink(p) else hashlib.sha256(open(p,'rb').read()).hexdigest()\n  for n in _:\n   p=os.path.join(d,n)\n   if os.path.islink(p): out[os.path.relpath(p,base)]='link:'+os.readlink(p)\n return out\na,b=walk(left),walk(right)\nprint(json.dumps(sorted(k for k in set(a)|set(b) if a.get(k)!=b.get(k) and not os.path.islink(os.path.join(left,k)))))";
 const result=Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify([left,right])),stdout:"pipe",stderr:"pipe"});expect(result.exitCode,result.stderr.toString()).toBe(0);return JSON.parse(result.stdout.toString());
};

async function generate(variant:typeof corpus.variants[number],cwd:string){
 const base=scratch(),root=join(base,"checkout"),workspace=join(base,"workspace");mkdirSync(root);mkdirSync(workspace);write(root,corpus.files);
 const implementation={version:1,snapshot:join(workspace,"implementation.json"),runner:join(workspace,"📜️script.ts"),sha256:"a".repeat(64),loaderSha256:"b".repeat(64)};writeFileSync(implementation.snapshot,"{}");writeFileSync(implementation.runner,"//");
 const previousCwd=process.cwd(),previousEnv={...process.env};Object.assign(process.env,variant.ambient);process.chdir(cwd);
 try{
  const graph=corpus.graph as any,request=corpus.request as any,plan=api.planBazelTasksV1(graph,request),capabilities=createNativeBuildCallerCapabilitiesV1(root,["run",request.tasks[0]],"bazel",{});
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:keys-contract",maximumElapsedMilliseconds:variant.maximumElapsedMilliseconds},capabilities,Date.now()),async (original:ScriptInvocation)=>{
   process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=scriptInvocationEnvironment(original,{}).SEMIO_SCRIPT_PROCESS_INVOCATION;
   await api.generateBazelWorkspaceV1(root,graph,request,plan,workspace,original,createIssuedResourceControlV1(original,"bazel:keys-contract",0),implementation,()=>{});
  });
  return {base,workspace};
 }finally{process.chdir(previousCwd);process.env={...previousEnv};}
}

test("Bazel key fixtures are closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test.skipIf(!python)("the caller envelope, deadline, cwd and ambient environment never reach a generated Bazel action input; only runtime.json carries them and snapshot.json records host file identities that no action reads",async()=>{
 const first=await generate(corpus.variants[0]!,repo),second=await generate(corpus.variants[1]!,realpathSync(tmpdir()));
 try{
  const differing=oracleDifferences(first.workspace,second.workspace);expect(differing).toEqual(corpus.onlyRuntimeMayDiffer);
  writeFileSync(join(first.workspace,"BUILD.bazel"),readFileSync(join(first.workspace,"BUILD.bazel"),"utf8")+"# mutated\n");expect(oracleDifferences(first.workspace,second.workspace)).toEqual(["BUILD.bazel",...corpus.onlyRuntimeMayDiffer]);writeFileSync(join(first.workspace,"BUILD.bazel"),readFileSync(join(second.workspace,"BUILD.bazel")));
  const runtime=[first,second].map(row=>JSON.parse(readFileSync(join(row.workspace,"runtime.json"),"utf8")).environment.SEMIO_SCRIPT_PROCESS_INVOCATION as string);
  expect(typeof runtime[0]).toBe("string");expect(runtime[0]).not.toEqual(runtime[1]);
  for(const name of["BUILD.bazel","tasks.bzl"]){const text=readFileSync(join(first.workspace,name),"utf8");expect(text).not.toMatch(/SEMIO_SCRIPT_PROCESS_INVOCATION|deadlineEpochMilliseconds/u);}
  const attributeKeys=[...readFileSync(join(first.workspace,"BUILD.bazel"),"utf8").matchAll(/^    ([^\s=]+) = /gmu)].map(row=>row[1]!);expect(attributeKeys.length).toBeGreaterThan(0);expect(attributeKeys.filter(key=>!/^[A-Za-z_][A-Za-z0-9_]*$/u.test(key)),"every generated rule attribute is a Starlark identifier").toEqual([]);
  const environments=[...readFileSync(join(first.workspace,"tasks.bzl"),"utf8").matchAll(/env = (\{[^}]*\})/gu)].map(row=>row[1]);expect(new Set(environments)).toEqual(new Set(['{"BUN_RUNTIME_TRANSPILER_CACHE_PATH": "0"}']));
 }finally{rmSync(first.base,{recursive:true,force:true});rmSync(second.base,{recursive:true,force:true});}
},120000);

test.skipIf(!python)("a generator snapshot restore leaves identical files untouched, repairs edited ones and refuses a missing declared output",async()=>{
 const base=scratch(),root=join(base,"checkout"),snapshot=join(base,"snapshot");mkdirSync(join(root,".cache/b/l/m/data"),{recursive:true});write(root,corpus.files);write(root,corpus.snapshot.files);
 const task=(outputs:string[],cache=true,metadata?:unknown)=>({id:"gen:generate",project:"gen",target:"generate",label:"t_"+"a".repeat(16),root:"gen",args:[],dependencies:[],definition:{execution:{kind:"command",command:"true"},outputs,cache,...(metadata?{metadata}:{})}}) as any;
 const facts=()=>Object.fromEntries(Object.keys(corpus.snapshot.files).map(path=>{const stat=statSync(join(root,path),{bigint:true});return[path,[String(stat.ino),String(stat.mtimeNs)]];})),bytes=(path:string)=>readFileSync(join(root,path),"utf8");
 try{
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel:snapshot-contract",maximumElapsedMilliseconds:120000},createNativeBuildCallerCapabilitiesV1(root,["run","gen:generate"],"bazel",{}),Date.now()),async (original:ScriptInvocation)=>{
   const control=()=>createIssuedResourceControlV1(original,"bazel:snapshot-contract",0);
   await executorApi.captureBazelSnapshotV1({workspace:root,task:task(corpus.snapshot.outputs)},snapshot,original);
   const before=facts();await new Promise(accept=>setTimeout(accept,25));await executorApi.restoreBazelSnapshotV1(root,snapshot,original,control());
   expect(facts()).toEqual(before);
   const oracle=Bun.spawnSync([python!,"-I","-c","import hashlib,json,sys\nroot,files=json.loads(sys.stdin.read())\nprint(json.dumps({p:hashlib.sha256(open(root+'/'+p,'rb').read()).hexdigest() for p in files}))"],{stdin:new TextEncoder().encode(JSON.stringify([root,Object.keys(corpus.snapshot.files)])),stdout:"pipe"});
   expect(JSON.parse(oracle.stdout.toString())).toEqual(Object.fromEntries(Object.entries(corpus.snapshot.files).map(([path,text])=>[path,sha(text)])));
   write(root,corpus.snapshot.edited);await executorApi.restoreBazelSnapshotV1(root,snapshot,original,control());
   for(const[path,text]of Object.entries(corpus.snapshot.files))expect(bytes(path)).toBe(text);
   await expect(executorApi.captureBazelSnapshotV1({workspace:root,task:task([corpus.snapshot.missing])},join(base,"missing"),original)).rejects.toThrow("was not produced");
   await executorApi.captureBazelSnapshotV1({workspace:root,task:task([corpus.snapshot.missing],true,{semio:{optionalOutputs:true}})},join(base,"optional"),original);
   await executorApi.captureBazelSnapshotV1({workspace:root,task:task([corpus.snapshot.missing],false)},join(base,"uncached"),original);
  });
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);
