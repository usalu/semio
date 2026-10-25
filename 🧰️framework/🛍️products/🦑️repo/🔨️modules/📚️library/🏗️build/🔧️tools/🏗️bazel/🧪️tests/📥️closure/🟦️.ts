/** 🔗️ Contract: a closure marker in a hash-representation graph expands to the exact workspace files its content-addressed manifest names, so Bazel declares them and its action key moves with every imported module; a missing, tampered or unsafe manifest is refused instead of silently dropped. The scenario lives in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema.json";

const api=await import("../../📜️script.ts"),closure=await import("../../🔗️closure/🟦️.ts"),python=testPythonV1(),bazel=process.env.SEMIO_TEST_BAZEL_BINARY;
const sha=(text:string|Uint8Array)=>createHash("sha256").update(text).digest("hex"),text=(value:unknown)=>JSON.stringify(value);
const marker=(kind:"source"|"lock",value:unknown)=>({env:(kind==="source"?"SEMIO_SOURCE_CLOSURE_":"SEMIO_CARGO_LOCK_")+sha(text(value))});
const names=new Map<string,{kind:"source"|"lock";value:unknown}>();
for(const row of corpus.manifests.source)names.set("@source:"+row.name,{kind:"source",value:row.value});
for(const row of corpus.manifests.lock)names.set("@lock:"+row.name,{kind:"lock",value:row.value});
const resolve=(value:unknown):unknown=>Array.isArray(value)?value.map(resolve):value&&typeof value==="object"?("env" in value&&typeof (value as {env:unknown}).env==="string"&&names.has((value as {env:string}).env)?(()=>{const row=names.get((value as {env:string}).env)!;return marker(row.kind,row.value);})():Object.fromEntries(Object.entries(value).map(([key,child])=>[key,resolve(child)]))):value;
const graph=resolve(corpus.graph) as any;
const store=(root:string)=>{
 for(const[kind,directory]of [["source","build-input-closures"],["lock","build-lock-closures"]] as const)for(const row of corpus.manifests[kind]){const body=text(row.value);mkdirSync(join(root,"store",directory),{recursive:true});writeFileSync(join(root,"store",directory,sha(body)+".json"),body);}
 return join(root,"store");
};
const workspace=(root:string)=>{for(const[path,body]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),body);}return root;};
const plan=(rows:readonly {project:string;inputs:unknown[]}[])=>rows.map(row=>({project:row.project,definition:{inputs:resolve(row.inputs) as unknown[]}}));
const load=async(root:string,rows:readonly {project:string;inputs:unknown[]}[]=corpus.cases)=>(workspace(root),closure.loadBazelClosuresV1(closure.bazelClosureMarkersForPlanV1({...graph,dependencies:graph.dependencies},plan(rows)),closure.filesystemBazelClosureFetchV1(store(root))));
const task=(row:{project:string;inputs:unknown[]})=>({id:row.project+":t",project:row.project,target:"t",label:"t",dependencies:[],definition:{inputs:resolve(row.inputs)}}) as any;
const split=(inputs:unknown[])=>({strings:inputs.filter((value):value is string=>typeof value==="string"),objects:inputs.filter(value=>typeof value!=="string")});

test("closure fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test("markers expand to exactly the manifest files, in place, through own and inherited named groups",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bazel-closure-"));
 try{
  const loaded=await load(root);
  for(const row of corpus.cases){
   const inputs=api.bazelTaskInputsV1(graph,task(row),loaded),got=split(inputs);
   expect(got.strings,row.name).toEqual(row.expanded);
   expect(got.objects.filter(value=>!closure.bazelClosureMarkerV1(value)),row.name).toEqual(row.passthrough);
   expect(inputs.some(value=>closure.bazelClosureMarkerV1(value)?.kind==="source"),row.name).toBe(false);
   expect(inputs.filter(value=>closure.bazelClosureMarkerV1(value)?.kind==="lock").length,row.name).toBe(row.expanded.includes("crate/Cargo.toml")?1:0);
   expect(got.strings,row.name+": no lock file is declared").not.toContain("Cargo.lock");
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a plan loads only the manifests of the projects it can reach",()=>{
 const reached=(project:string)=>closure.bazelClosureMarkersForPlanV1(graph,plan([{project,inputs:["default","^default"]}])).map((row:{key:string})=>row.key);
 const unreached=names.get("@source:unreached")!,key=sha(text(unreached.value));
 expect(reached("q")).not.toContain(key);
 expect(reached(corpus.unreachedProject)).toContain(key);
 expect(reached("r")).toEqual([]);
 expect(closure.bazelClosureMarkersForPlanV1(graph,plan([{project:"r",inputs:[{input:"default",projects:["p"]}]}])).length).toBe(1);
});

test("an unexpanded marker is refused unless a plan only asks about task shape",async()=>{
 const row=corpus.cases[0]!;
 expect(()=>api.bazelTaskInputsV1(graph,task(row))).toThrow("requires its loaded manifest");
 expect(split(api.bazelTaskInputsV1(graph,task(row),"ignore")).objects).toEqual(row.passthrough);
});

test("manifests are content addressed and closed: tampered, missing and unsafe ones are refused",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bazel-closure-refusal-"));
 try{
  const directory=store(root),row=corpus.manifests.source[0]!,key=sha(text(row.value)),path=join(directory,"build-input-closures",key+".json");
  writeFileSync(path,text({version:1,files:["tools/other.ts"]}));
  await expect(closure.filesystemBazelClosureFetchV1(directory)("source",[key])).rejects.toThrow("Exact Source inventory identity changed");
  await expect(closure.loadBazelClosuresV1([{kind:"source",key:"0".repeat(64)}],closure.filesystemBazelClosureFetchV1(directory))).rejects.toThrow();
  await expect(closure.loadBazelClosuresV1([{kind:"source",key:"1".repeat(64)}],async()=>new Map())).rejects.toThrow("Bazel closure manifest missing");
  for(const refusal of corpus.refusals)expect(()=>closure.parseBazelClosureManifestV1(refusal.kind as "source"|"lock",refusal.value),refusal.name).toThrow(refusal.error);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("without expansion a changed imported module never reaches the declared sources, with expansion it does",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bazel-closure-hole-"));
 try{
  const loaded=await load(root),files=Object.keys(corpus.files).sort(),row=corpus.cases[0]!;
  const before=api.bazelTaskSourcePathsV1(api.bazelTaskInputsV1(graph,task(row),"ignore"),files),after=api.bazelTaskSourcePathsV1(api.bazelTaskInputsV1(graph,task(row),loaded),files);
  expect(before).not.toContain("tools/dep.ts");
  expect(after).toContain("tools/dep.ts");expect(after).toContain("tools/entry.ts");expect(after).not.toContain("Cargo.lock");
 }finally{rmSync(root,{recursive:true,force:true});}
});

const identity=async(root:string,paths:readonly string[])=>{const hash=createHash("sha256");for(const path of [...paths].sort())hash.update(path+"\0"+await api.bazelSourceIdentityV1(join(root,path),new AbortController().signal)+"\0");return hash.digest("hex");};

test("the declared-source identity moves exactly when an imported module changes",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bazel-closure-key-"));
 try{
  const loaded=await load(root),row=corpus.cases[0]!,selected=()=>api.bazelTaskSourcePathsV1(api.bazelTaskInputsV1(graph,task(row),loaded),Object.keys(corpus.files).sort()).filter(path=>!path.includes("*"));
  const before=await identity(root,selected());
  writeFileSync(join(root,corpus.mutation.unrelatedFile),corpus.mutation.unrelatedContent);
  expect(await identity(root,selected())).toBe(before);
  writeFileSync(join(root,corpus.mutation.file),corpus.mutation.content);
  expect(await identity(root,selected())).not.toBe(before);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test.skipIf(!python)("expanded closures agree with an independent Python reading of the manifests",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bazel-closure-python-"));
 try{
  const loaded=await load(root),directory=store(root),row=corpus.manifests.source[0]!,key=sha(text(row.value));
  const result=Bun.spawnSync([python!,"-I","-c","import hashlib,json,sys\nd=json.loads(sys.stdin.read())\nb=open(d['path'],'rb').read()\nassert hashlib.sha256(b).hexdigest()==d['key']\nprint(json.dumps(sorted(set(json.loads(b)['files']))))"],{stdin:new TextEncoder().encode(JSON.stringify({path:join(directory,"build-input-closures",key+".json"),key})),stdout:"pipe",stderr:"pipe"});
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  expect(loaded.files({kind:"source",key})).toEqual(JSON.parse(result.stdout.toString()));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test.skipIf(!bazel)("real Bazel re-executes an action exactly when a file of the expanded closure changes",async()=>{
 const root=mkdtempSync(join(tmpdir(),"bazel-closure-real-")),output=mkdtempSync(join(tmpdir(),"bzlclo-"));
 try{
  const loaded=await load(root),row=corpus.cases[0]!,srcs=api.bazelTaskSourcePathsV1(api.bazelTaskInputsV1(graph,task(row),loaded),Object.keys(corpus.files).sort()).filter(path=>!path.includes("*")&&existsSync(join(root,path)));
  const counter=join(output,"counter");writeFileSync(counter,"");
  writeFileSync(join(root,"MODULE.bazel"),'module(name = "closure_fixture", version = "0.0.0")\n');
  writeFileSync(join(root,"check.bzl"),`def _impl(ctx):\n    out = ctx.actions.declare_file(ctx.label.name + ".out")\n    ctx.actions.run_shell(inputs = ctx.files.srcs, outputs = [out], arguments = [out.path] + [f.path for f in ctx.files.srcs], command = "echo run >> ${counter} && out=$1 && shift && cat \\"$@\\" | shasum > $out", execution_requirements = {"local": "1"})\n    return [DefaultInfo(files = depset([out]))]\n\ncheck = rule(implementation = _impl, attrs = {"srcs": attr.label_list(allow_files = True)})\n`);
  writeFileSync(join(root,"BUILD.bazel"),`load(":check.bzl", "check")\n\ncheck(\n    name = "t",\n    srcs = [${srcs.map(path=>JSON.stringify(path)).join(", ")}],\n)\n`);
  const run=()=>{const result=spawnSync(bazel!,[`--output_user_root=${join(output,"user")}`,"--max_idle_secs=30","build","//:t","--spawn_strategy=local","--lockfile_mode=off",`--repository_cache=${join(output,"repository")}`,"--incompatible_autoload_externally=","--noshow_progress"],{cwd:root,encoding:"utf8",timeout:240000});expect(result.status,result.stderr).toBe(0);return readFileSync(counter,"utf8").split("\n").filter(Boolean).length;};
  expect(run()).toBe(1);
  expect(run()).toBe(1);
  writeFileSync(join(root,corpus.mutation.unrelatedFile),corpus.mutation.unrelatedContent);
  expect(run()).toBe(1);
  writeFileSync(join(root,corpus.mutation.file),corpus.mutation.content);
  expect(run()).toBe(2);
  expect(run()).toBe(2);
  spawnSync(bazel!,[`--output_user_root=${join(output,"user")}`,"shutdown"],{cwd:root});
 }finally{rmSync(root,{recursive:true,force:true});rmSync(output,{recursive:true,force:true});}
},600000);
