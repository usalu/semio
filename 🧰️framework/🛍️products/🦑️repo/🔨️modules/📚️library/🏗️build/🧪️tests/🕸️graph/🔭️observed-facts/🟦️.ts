/** 🔭️ Observed graph facts retain exact reads, retries and replay under original custody. */
import {mkdir,mkdtemp,writeFile,readFile,rm,access} from "node:fs/promises";
import * as rawPromises from "node:fs/promises";
import {join,resolve} from "node:path";
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {build} from "esbuild";
import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";
import {buildGraphMembershipV1,buildGraphSourceFactsHoldV1,workspaceBuildFilesV1} from "../../../🕸️graph/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {independentGlobV1,withCommandSourceControlV1} from "../🧰️support/🟦️.ts";

const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
test("test owner inference names only the current declared fact namespace without storage admission",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"ti-")),plugin=await import("../../../../../🧪️test/📦️packages/🟦️typescript/🟨️.mjs"),declaration=JSON.parse(await readFile(new URL("../../../📥️inputs/💾️storage/🧬️schema/🔣️.json",import.meta.url),"utf8")),admitted=new Ajv({strict:true}).compile(declaration);
 try{expect(admitted(fixture.testInferenceLocation.namespace)).toBe(true);expect(plugin.testBuildInferenceDirectoryV1(root)).toBe(join(root,fixture.testInferenceLocation.relative));const prior=process.env.SEMIO_BUILD_INFERENCE_DIRECTORY;try{process.env.SEMIO_BUILD_INFERENCE_DIRECTORY=join(root,fixture.testInferenceLocation.override);expect(plugin.testBuildInferenceDirectoryV1(root)).toBe(join(root,fixture.testInferenceLocation.relative));}finally{if(prior===undefined)delete process.env.SEMIO_BUILD_INFERENCE_DIRECTORY;else process.env.SEMIO_BUILD_INFERENCE_DIRECTORY=prior;}expect(await access(join(root,".cache")).then(()=>true,()=>false)).toBe(false);}finally{await rm(root,{recursive:true,force:true});}
});
test("the defining graph fact module exposes its declared writer port under original custody",async()=>{
 const source=new URL("../../../📥️inputs/💾️storage/🟨️.mjs",import.meta.url),storage=await import(source.href),parser=require("typescript"),tree=parser.createSourceFile(fileURLToPath(source),readFileSync(source,"utf8"),parser.ScriptTarget.Latest,true),exports=tree.statements.filter((node:any)=>parser.isFunctionDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text);
 expect(exports).toEqual(fixture.initialAuthoring.storagePorts);
 for(const name of fixture.initialAuthoring.storagePorts)expect(typeof storage[name],name).toBe("function");
 expect(()=>storage.repositoryBuildGraphWriterDirectoryV1(process.cwd(),"inference")).toThrow("Original bounded graph writer custody required");
});
test("observed authority ignores unread body edits and identical rewrites while retaining consumed bytes and physical membership",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"facts-")),observation=require("../../../👁️observation/🟨️.mjs");
  try{
    for(const[path,source]of Object.entries(fixture.authority.files))await writeFile(join(root,path),source);
    const captured=await observation.captureObservedSourceInputsV1(async()=>observation.observedFs.readFileSync(join(root,fixture.authority.consumed),"utf8")),facts=captured.observations.filter((row:any)=>row.path.startsWith(root)),before=await buildGraphMembershipV1(root,await workspaceBuildFilesV1(root));
    const contract={version:3,module:{implementation:"a".repeat(64),publication:"b".repeat(64)},membership:before,graph:"a".repeat(64),observations:facts};expect(new Ajv({strict:false}).compile((await import("../../../🧬️schema/📥️authority.json")).default)(contract)).toBe(true);
    for(const row of fixture.authority.cases){if(row.kind==="remove")await rm(join(root,row.path));else await writeFile(join(root,row.path),row.content!);expect(await buildGraphMembershipV1(root,await workspaceBuildFilesV1(root))===before,row.name).toBe(row.membership);expect(await buildGraphSourceFactsHoldV1(facts),row.name).toBe(row.facts);}
  }finally{await rm(root,{recursive:true,force:true});}
});
test("failed reads retain precise facts for transient retry while stable malformed sources remain stable",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"failed-facts-")),observation=require("../../../👁️observation/🟨️.mjs"),source=join(root,fixture.authority.consumed);let facts:unknown[]=[];
  try{
    await writeFile(source,"{");await expect(observation.captureObservedSourceInputsV1(()=>JSON.parse(observation.observedFs.readFileSync(source,"utf8")),(rows:any[])=>{facts=rows.filter(row=>row.path===source);})).rejects.toThrow();
    expect(facts.length).toBe(1);expect(await buildGraphSourceFactsHoldV1(facts)).toBe(true);
    await writeFile(source,fixture.authority.files[fixture.authority.consumed as keyof typeof fixture.authority.files]);expect(await buildGraphSourceFactsHoldV1(facts)).toBe(false);
  }finally{await rm(root,{recursive:true,force:true});}
});
test("a captured descendant whose ancestor becomes a file provides membership retry evidence",async()=>{
  const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"replaced-ancestor-")),row=fixture.authority.replacedAncestor,oracle=independentGlobV1;
  try{
    await mkdir(join(root,row.directory));await writeFile(join(root,row.file),row.content);const files=await workspaceBuildFilesV1(root);
    expect(files).toEqual(oracle.sync("**/*",{cwd:root,onlyFiles:true}));await rm(join(root,row.directory),{recursive:true});await writeFile(join(root,row.directory),row.content);
    expect(oracle.sync(row.file,{cwd:root,onlyFiles:true})).toEqual([]);await expect(rawPromises.lstat(join(root,row.file))).rejects.toMatchObject({code:"ENOTDIR"});await expect(buildGraphMembershipV1(root,files)).rejects.toMatchObject({code:"BUILD_MEMBERSHIP_CHANGED"});
  }finally{await rm(root,{recursive:true,force:true});}
});
test("same receiver observed replay retains exact facts and original control across validation frames",async()=>{
 const row=fixture.sameReceiverReplay,root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"sr-")),observation=require("../../../👁️observation/🟨️.mjs"),{Database}=await import("bun:sqlite"),database=new Database(":memory:"),memo=new Map(),providerId=row.provider.id+":"+createHash("sha256").update(root).digest("hex"),entry=join(root,"entry.json"),leaf=join(root,"leaf.json");let checkpoints=0,computations=0,provider:any;
 database.exec("CREATE TABLE facts(receiver TEXT NOT NULL,row TEXT NOT NULL,PRIMARY KEY(receiver,row)) WITHOUT ROWID");const insert=database.prepare("INSERT OR IGNORE INTO facts VALUES(?,?)"),equal=(name:string,actual:any[],expected:any[])=>{for(const value of actual)insert.run(name,JSON.stringify(value));for(const value of expected)insert.run(name+":expected",JSON.stringify(value));expect(database.query("SELECT row FROM facts WHERE receiver=? EXCEPT SELECT row FROM facts WHERE receiver=?").all(name,name+":expected")).toEqual([]);expect(database.query("SELECT row FROM facts WHERE receiver=? EXCEPT SELECT row FROM facts WHERE receiver=?").all(name+":expected",name)).toEqual([]);};
 const register=(revision:string,bias:number)=>observation.registerSourceAnalysisV1({id:providerId,revision,analyze:(_path:string,bytes:Buffer)=>{const value=JSON.parse(bytes.toString()),dependency=JSON.parse(observation.observedFs.readFileSync(join(root,value.imports[0]),"utf8"));return {entry:value.value,leaf:dependency.value+bias};}}),compute=()=>{computations++;observation.observedFs.readFileSync(entry);return observation.observedSourceAnalysisV1(provider,entry);},capture=(body:()=>unknown)=>observation.captureObservedSourceInputsV1(body,undefined,false);
 try{for(const[path,value]of Object.entries(row.files))await writeFile(join(root,path),value);provider=register(row.provider.firstRevision,0);const baseline=await capture(compute);expect(baseline.value).toEqual({entry:1,leaf:2});expect(baseline.observations.length).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"same-receiver-replay",maximumElapsedMilliseconds:20000},{workspaceRoot:root},Date.now()),async original=>{
   let narrowed=original.control.signal,abortNext:AbortController|undefined;const checkpoint=async()=>{original.control.signal.throwIfAborted();narrowed.throwIfAborted();const remaining=original.control.remainingMilliseconds();if(remaining!==null&&remaining<=0)throw Error("Original replay deadline expired");checkpoints++;await original.control.publish({owner:original.policy.owner,command:"source:replay",stage:"running"});await original.control.yieldContinuation();abortNext?.abort(Error(row.cancelReason));narrowed.throwIfAborted();},read=()=>observation.memoizedObservedAsyncComputationV1(memo,"value",compute,checkpoint),frame=(body:()=>Promise<unknown>)=>capture(()=>observation.settleObservedComputationV1(()=>observation.memoizedObservedAsyncComputationV1(new Map(),"parent",body,checkpoint),{signal:original.control.signal}));
   const first=await frame(async()=>{await read();const coldEntry=memo.get("value"),before=checkpoints;await read();expect(checkpoints-before).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);const hit=checkpoints;expect(await read()).toEqual(baseline.value);expect(checkpoints-hit).toBe(row.expectedFastCheckpoints);
    const receiver=new Map(),prior=checkpoints;await observation.memoizedObservedAsyncComputationV1(receiver,"new-parent",read,checkpoint);expect(checkpoints-prior).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);equal("new-parent",receiver.get("new-parent").observations,baseline.observations);
    const freshCapture=await capture(async()=>{const before=checkpoints;await read();expect(checkpoints-before).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);const again=checkpoints;await read();expect(checkpoints-again).toBe(row.expectedFastCheckpoints);});equal("new-capture",freshCapture.observations,baseline.observations);
    const nested=await frame(async()=>{const before=checkpoints;const value=await read();expect(checkpoints-before).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);return value;});equal("new-validation",nested.observations,baseline.observations);
    memo.delete("value");await read();expect(memo.get("value")).not.toBe(coldEntry);const replaced=checkpoints;await read();expect(checkpoints-replaced).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);return baseline.value;});equal("first-frame",first.observations,baseline.observations);
   const changedBefore=createHash("sha256").update(await readFile(leaf)).digest("hex");await writeFile(leaf,row.mutation.after);const changedFresh=await capture(compute),changed=await frame(read);expect(changed.value).toEqual({entry:1,leaf:3});equal("changed-source",changed.observations,changedFresh.observations);expect(changed.observations.some((value:any)=>value.kind==="file"&&value.path===leaf&&value.fact.sha256===changedBefore)).toBe(false);
   const obsolete=provider;provider=register(row.provider.secondRevision,1);expect(()=>observation.observedSourceAnalysisV1(obsolete,entry)).toThrow("Current registered source analysis provider required");const revisedFresh=await capture(compute),revised=await frame(read);expect(revised.value).toEqual({entry:1,leaf:4});equal("new-provider",revised.observations,revisedFresh.observations);
   const cancelled=await frame(async()=>{const controller=new AbortController();narrowed=AbortSignal.any([original.control.signal,controller.signal]);abortNext=controller;await expect(read()).rejects.toThrow(row.cancelReason);abortNext=undefined;narrowed=original.control.signal;const retry=checkpoints;await read();expect(checkpoints-retry).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);const fast=checkpoints;await read();expect(checkpoints-fast).toBe(row.expectedFastCheckpoints);const finalCancel=new AbortController();narrowed=AbortSignal.any([original.control.signal,finalCancel.signal]);abortNext=finalCancel;const before=checkpoints;await expect(read()).rejects.toThrow(row.cancelReason);expect(checkpoints-before).toBe(1);abortNext=undefined;narrowed=original.control.signal;return revised.value;});equal("cancelled-receiver",cancelled.observations,revisedFresh.observations);expect(original.control.signal.aborted).toBe(false);
   const failed=new Map(),failure=Error("original observed computation failed");await frame(async()=>{await expect(observation.memoizedObservedAsyncComputationV1(failed,"failure",async()=>{observation.observedFs.readFileSync(leaf);throw failure;},checkpoint)).rejects.toThrow(failure);expect(failed.has("failure")).toBe(false);const next=await observation.memoizedObservedAsyncComputationV1(failed,"failure",compute,checkpoint);expect(next).toEqual(revised.value);const hit=checkpoints;await observation.memoizedObservedAsyncComputationV1(failed,"failure",compute,checkpoint);expect(checkpoints-hit).toBeGreaterThanOrEqual(row.minimumFullCheckpoints);return next;});
   expect(computations).toBeGreaterThan(1);
  });
 }finally{database.close();await rm(root,{recursive:true,force:true});}
},25000);
test("same receiver observed replay awaits actual command closure cancellation in production frames",async()=>{
 const row=fixture.commandSourceCancellation,root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!),"sc-")),port=await import("../../../🕸️graph/📥️inference/🟨️.mjs"),observation=require("../../../👁️observation/🟨️.mjs"),expected=[row.entry,...row.files].sort(),reason=Error("original same-frame command closure cancelled");
 try{await mkdir(join(root,"owner"));for(const path of row.files)await writeFile(join(root,path),row.leaf);await writeFile(join(root,row.entry),row.source);const oracle=await build({entryPoints:[row.entry],absWorkingDir:root,bundle:true,write:false,metafile:true,platform:"node",logLevel:"silent"});expect(Object.keys(oracle.metafile!.inputs).sort()).toEqual(expected);
  await withCommandSourceControlV1(root,"same-frame-command-source",20000,reason,async control=>{
   const {parent,invocation:original,phaseControl}=control,cache=port.createScriptInputCache({workspaceRoot:root,invocation:original,phaseControl});
   const captured=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(()=>observation.memoizedObservedAsyncComputationV1(new Map(),"actual-command-receiver",async()=>{const first=await port.cacheInternals.relativeScriptInputs([join(root,row.entry)],root,cache);expect(first.map((path:string)=>path.slice("{workspaceRoot}/".length))).toEqual(expected);expect(await port.cacheInternals.relativeScriptInputs([join(root,row.entry)],root,cache)).toBe(first);control.arm("build:command-source-imports");await expect(port.cacheInternals.relativeScriptInputs([join(root,row.entry)],root,cache)).rejects.toThrow(reason);return first;},()=>parent.control.yieldContinuation()),{signal:parent.control.signal,limit:1}),undefined,false);
   expect(control.events.filter(event=>event==="build:command-source-imports").length).toBeGreaterThan(0);expect(control.continuations()).toBe(row.expectedContinuationsAfterAbort);expect(original.control.signal.aborted).toBe(true);expect(parent.control.signal.aborted).toBe(false);expect(captured.observations.some((value:any)=>value.kind==="analysis"&&value.path===join(root,row.entry))).toBe(true);
  });
 }finally{await rm(root,{recursive:true,force:true});}
},25000);
