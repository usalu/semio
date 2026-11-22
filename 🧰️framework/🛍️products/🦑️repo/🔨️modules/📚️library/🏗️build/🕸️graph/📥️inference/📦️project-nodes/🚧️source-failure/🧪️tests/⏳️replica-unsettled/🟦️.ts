/** ⏳️ Contract: a module that an owner's source imports and that is not there yet, in a source a peer wrote a moment ago, is a replica that has not settled: the process ends with the restart code and says so, the restart is waited for and never counted as one deterministic failure, and the service answers a typed refusal instead of an unexpected failure; the same failure in a source that is stable withholds its owner and names it.
 * @see ../../🟨️ */
import{afterAll,expect,test}from"bun:test";
import{mkdirSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync}from"node:fs";
import{dirname,join}from"node:path";
import{exitIfStaleImplementationV1,formatStampProtocolV1,staleImplementationOfV1}from"../../../../../../🧷️format-stamps/🟨️.mjs";
import{createSlicePoolV1}from"../../../../../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import{HOST_POLICY_V1,renderBothV1}from"../../../../../../🏡️resident/📨️protocol/🎚️policy/🟨️.mjs";
import{failureRefusalV1}from"../../../../../../🏡️resident/🏛️service/🧯️failure/🟨️.mjs";
import{privateRepositoryWorkspaceV1}from"../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{ownerBoundaryFailureV1,ownerSourceFailureV1,replicaUnsettledV1}from"../../🟨️.mjs";
import{workspacePackageModuleV1}from"../../../../🧭️script-inputs/🟨️.mjs";
import{createRequire}from"node:module";
const repository=await privateRepositoryWorkspaceV1("graph");
declareLibraryBuildTestEnvelopeV1({suite:"build-inference-projects",scratch:"checkout"});
const roots:string[]=[],protocol=formatStampProtocolV1() as any;
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const[path,text]of Object.entries(files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const loadFailure=async(file:string)=>{try{await import(file+"?"+Math.random());}catch(error){return error as any;}throw Error("the import must fail");};
const age=(path:string,seconds:number)=>{const then=new Date(Date.now()-seconds*1000);utimesSync(path,then,then);};
const files={"proj/project.json":"{}","proj/src/🟦️.ts":'import "./🧊️stamps/🟦️.ts";\n'};
const failureOf=async(seconds:number)=>{
 const root=await repository.scratch("replica-unsettled-");roots.push(root);write(root,files);
 const importer=join(root,"proj/src/🟦️.ts"),raw=await loadFailure(importer);age(importer,seconds);
 return{root,importer,failure:ownerSourceFailureV1(raw,{workspaceRoot:root,fallback:join(root,"proj/project.json"),implementationRoot:join(root,"tooling")}) as any};
};
const sink=()=>{const lines:string[]=[];let code:number|undefined;return{lines,exited:()=>code,write:(text:string)=>{lines.push(text);},exit:(value:number)=>{code=value;}};};

test("a missing import of a source written seconds ago waits for the replica: the real loader error ends the process with the restart code and the unsettled text",async()=>repository.invoke(async()=>{
 const{failure}=await failureOf(1);
 expect(failure.code).toBe("BUILD_SOURCE_UNPARSEABLE");expect(failure.unresolvedImport).toBe(true);
 const marked=replicaUnsettledV1(failure);
 expect(marked?.replicaUnsettled).toBe(true);
 const found=staleImplementationOfV1(marked);
 expect(found?.origin).toBe("error");expect(found?.message.startsWith(protocol.loadFailure.unsettledPrefix)).toBe(true);expect(found?.message).toContain("stamps");
 const out=sink();exitIfStaleImplementationV1(marked,{write:out.write,exit:out.exit,sleepSync:()=>undefined});
 expect(out.exited()).toBe(protocol.exitCode);expect(out.lines.join("")).toContain(protocol.loadFailure.unsettledPrefix);
 const wrapped=Object.assign(Error("outer",{cause:marked}),{});expect(staleImplementationOfV1(wrapped)?.origin).toBe("error");
}),60_000);

test("the same missing import in a stable source withholds its owner and never ends a process; the window of a missing import is longer than the racy window of any other failure",async()=>repository.invoke(async()=>{
 const stable=await failureOf(protocol.progressMilliseconds/1000);
 expect(replicaUnsettledV1(stable.failure),"stable importer").toBeUndefined();
 expect(staleImplementationOfV1(stable.failure)).toBeUndefined();
 const out=sink();exitIfStaleImplementationV1(stable.failure,{write:out.write,exit:out.exit,sleepSync:()=>undefined});expect(out.exited()).toBeUndefined();
 const aged=await failureOf(10);
 expect(replicaUnsettledV1(aged.failure),"10 s old, a missing import still waits").toBeDefined();
 expect(replicaUnsettledV1({...aged.failure,unresolvedImport:false,source:aged.importer}),"10 s old, any other failure is withheld").toBeUndefined();
 expect(replicaUnsettledV1({source:join(stable.root,"absent.ts"),unresolvedImport:true}),"a source that is gone is a peer in the middle of a move").toBeDefined();
}),60_000);

test("the service answers the typed refusals, never an unexpected failure: stale and retried for a source of a replica that has not settled, one named owner for a stable source",async()=>repository.invoke(async()=>{
 const fresh=replicaUnsettledV1((await failureOf(1)).failure),stable=(await failureOf(600)).failure,logged:string[]=[];
 const options={log:(event:string)=>logged.push(event),logPath:"/log",root:"/r"};
 const waiting=failureRefusalV1(fresh,options),owner=failureRefusalV1(Object.assign(Error("wrapper",{cause:stable}),{}),options);
 expect(waiting).toMatchObject({code:"stale",key:"host-graph-replica-unsettled",retry:"later"});
 expect(owner).toMatchObject({code:"unserved",key:"host-graph-owner-source",retry:"later"});
 expect((owner.fields as any).source).toBe(stable.source);expect((waiting.fields as any).message).toContain("stamps");
 expect(logged).toEqual(["graph-replica-unsettled","graph-owner-source"]);
 for(const refusal of[waiting,owner]){const text=renderBothV1(refusal.key,refusal.fields as never);for(const language of["en","de"] as const){expect(text[language]).toContain(String((refusal.fields as any).source));expect(text[language]).not.toMatch(/\{[a-z]+\}/u);}}
 expect(failureRefusalV1(new Error("boom"),options).code).toBe("internal");
}),60_000);

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const harness=(tails:readonly(string|"done")[],sameCauseMaximum=3)=>{
 let index=0;const started:number[]=[];
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,restartPolicy:{backoffMilliseconds:[1],settleMilliseconds:60000,progressMilliseconds:60,totalMaximum:100,sameCauseMaximum},hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
  createWorker:()=>{const step=tails[index++]!,listeners:Record<string,(value:unknown)=>void>={};started.push(index);queueMicrotask(()=>step==="done"?listeners.message?.({type:"done",graph:{}}):listeners.exit?.(75));return{pid:undefined,stderrTail:()=>step==="done"?"":step,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
 return{pool,started,run:()=>pool.run({root:"/tmp/unsettled"},{onProgress:()=>undefined})};
};

test("a read waits through any number of restarts of an unsettled replica, while the same other text still stops the plan after the declared number",async()=>{
 const cause=protocol.loadFailure.unsettledPrefix+"Cannot find module './🧊️stamps/🟦️.ts' (imported by 🔏️index/🟦️.ts)";
 const waiting=harness([cause,cause,cause,cause,cause,"done"]);await waiting.run();await waiting.pool.close();expect(waiting.started.length).toBe(6);
 const other=harness(["error: Cannot find module './x.ts'","error: Cannot find module './x.ts'","error: Cannot find module './x.ts'","done"]);
 const failure:any=await other.run().then(()=>undefined,reason=>reason);await other.pool.close();expect(failure.code).toBe("slice-worker-failing");expect(other.started.length).toBe(3);
});

const scopeOf=(root:string)=>({workspaceRoot:root,fallback:join(root,"proj/project.json"),implementationRoot:join(root,"tooling")});
const workspaceFiles={"proj/project.json":"{}","proj/src/🟦️.ts":'import "@x/pkg/missing";\n',"pkg/package.json":JSON.stringify({name:"@x/pkg",exports:{".":"./a.ts"}}),"pkg/a.ts":"export const a=1;\n","package.json":JSON.stringify({name:"w",private:true,workspaces:["pkg","proj"]})};
const metadataFailure=async(seconds:number)=>{
 const root=await repository.scratch("replica-metadata-");roots.push(root);write(root,workspaceFiles);mkdirSync(join(root,"node_modules/@x"),{recursive:true});symlinkSync(join(root,"pkg"),join(root,"node_modules/@x/pkg"));
 const importer=join(root,"proj/src/🟦️.ts");let raised:any;
 try{workspacePackageModuleV1(importer,"@x/pkg/missing",root);}catch(error){raised=error;}
 age(importer,seconds);age(join(root,"pkg/package.json"),seconds);
 return{root,importer,raised};
};

test("an unresolved workspace package import is classified at the owner boundary: fresh package metadata waits for the replica, stable metadata withholds the owner, and the rows record what it rests on",async()=>repository.invoke(async()=>{
 const fresh=await metadataFailure(1);
 expect(fresh.raised?.code,"the real resolver raises MODULE_NOT_FOUND").toBe("MODULE_NOT_FOUND");expect(fresh.raised.unresolved).toBe(true);expect(fresh.raised.watched?.length).toBeGreaterThan(0);
 const waiting=ownerBoundaryFailureV1(fresh.raised,scopeOf(fresh.root)) as any;
 expect(waiting.code).toBe("BUILD_SOURCE_UNPARSEABLE");expect(waiting.replicaUnsettled).toBe(true);expect(waiting.source).toBe(fresh.importer);
 expect(staleImplementationOfV1(waiting)?.message.startsWith(protocol.loadFailure.unsettledPrefix)).toBe(true);
 const stable=await metadataFailure(600),withheld=ownerBoundaryFailureV1(stable.raised,scopeOf(stable.root)) as any;
 expect(withheld.code).toBe("BUILD_SOURCE_UNPARSEABLE");expect(withheld.replicaUnsettled).toBeUndefined();expect(staleImplementationOfV1(withheld)).toBeUndefined();
 const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,cachePath=join(stable.root,"np","project.json");
 await observation.observeInferenceSnapshotV1(()=>observation.observeProjectInferenceV1({cachePath,key:"project",owner:"project"},async()=>{ownerBoundaryFailureV1(stable.raised,scopeOf(stable.root));return 1;}));
 const rows=(observation.projectInferenceObservationsV1(cachePath)??[]) as{kind:string;path:string;fact:{mtimeMs?:number;size?:number}}[],metadata=join(stable.root,"pkg/package.json"),row=rows.find(entry=>entry.path===metadata);
 expect(row,"the package metadata is an observed row").toBeDefined();expect(rows.some(entry=>entry.path===stable.importer),"and so is the importing source").toBe(true);
}),60_000);

test("any other failure attributable to a source of the owner (a missing file, a real require miss) is classified at the boundary, a failure that names no workspace source stays fatal",async()=>repository.invoke(async()=>{
 const root=await repository.scratch("replica-generic-");roots.push(root);write(root,files);
 const importer=join(root,"proj/src/🟦️.ts");let miss:any,enoent:any;
 try{createRequire(importer).resolve("./gone.ts");}catch(error){miss=error;}
 try{await Bun.file(join(root,"proj/absent.json")).text();}catch(error){enoent=error;}
 try{(await import("node:fs")).readFileSync(join(root,"proj/absent.json"));}catch(error){enoent=error;}
 age(importer,600);
 const requireMiss=ownerBoundaryFailureV1(miss,scopeOf(root)) as any;
 expect(miss.code).toBe("MODULE_NOT_FOUND");expect(requireMiss?.code).toBe("BUILD_SOURCE_UNPARSEABLE");expect(requireMiss.source).toBe(importer);expect(requireMiss.replicaUnsettled).toBeUndefined();
 const missingFile=ownerBoundaryFailureV1(enoent,scopeOf(root)) as any;
 expect(enoent.code).toBe("ENOENT");expect(missingFile?.code,"a file of the owner that is not there").toBe("BUILD_SOURCE_UNPARSEABLE");
 write(root,{"proj/fresh.ts":"x"});const freshMiss=ownerBoundaryFailureV1(Object.assign(Error("Cannot find module './g.ts' from '"+join(root,"proj/fresh.ts")+"'"),{code:"MODULE_NOT_FOUND"}),scopeOf(root)) as any;
 expect(freshMiss.replicaUnsettled,"written just now").toBe(true);
 expect(ownerBoundaryFailureV1(Error("boom"),scopeOf(root))).toBeUndefined();
 expect(ownerBoundaryFailureV1(Object.assign(Error("ENOENT: no such file"),{code:"ENOENT",path:"/elsewhere/x"}),scopeOf(root)),"outside the workspace").toBeUndefined();
 expect(ownerBoundaryFailureV1(Object.assign(Error("ENOENT: no such file"),{code:"ENOENT",path:join(root,".cache/x")}),scopeOf(root)),"infrastructure").toBeUndefined();
 expect(ownerBoundaryFailureV1(Object.assign(Error("x"),{code:"MODULE_NOT_FOUND",path:join(root,"tooling/a.mjs")}),{...scopeOf(root),implementationRoot:join(root,"tooling")})).toBeUndefined();
}),60_000);

test("the owner observation boundary classifies a source failure of the owner and leaves every other failure fatal, in the real memoizer",async()=>repository.invoke(async()=>{
 const{observeOwnerFactsMemoizedV1}=await import("../../../../../📥️load/🟦️.ts") as any,observation=await import("../../../../../../👁️observation/🟨️.mjs") as any;
 const fresh=await metadataFailure(1),stable=await metadataFailure(600);
 const ask=(key:string,error:unknown,scope:{root:string})=>observeOwnerFactsMemoizedV1({key,hold:async()=>false,observation,observe:async()=>{throw error;},boundary:(raised:unknown)=>ownerBoundaryFailureV1(raised,scopeOf(scope.root))}).then(()=>undefined,(raised:any)=>raised);
 const waiting=await ask("a",fresh.raised,fresh),withheld=await ask("b",stable.raised,stable),unknown=Error("unknown failure");
 expect(waiting.replicaUnsettled).toBe(true);expect(staleImplementationOfV1(waiting)?.origin).toBe("error");
 expect(withheld.code).toBe("BUILD_SOURCE_UNPARSEABLE");expect(withheld.replicaUnsettled).toBeUndefined();expect(withheld.source).toBe(stable.importer);
 const named=await ask("c",unknown,stable);expect(named).toMatchObject({code:"BUILD_OWNER_FAILED",owners:["c"],cause:unknown});
}),60_000);
