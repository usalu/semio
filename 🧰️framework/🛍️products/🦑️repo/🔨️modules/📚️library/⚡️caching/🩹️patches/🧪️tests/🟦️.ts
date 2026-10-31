import{createRepositoryFilesystemJournalV1}from"../../🕸️graph/🔏️seal/📥️membership/📥️journal/🟦️.ts";
import workspaceNativePin from"../../../🏗️build/🏡️resident/🔭️watch/🦀️rust/🔣️.json"with{type:"json"};
import{SOURCE_OPACITY_PROTOCOL_V1,isOpaqueSourcePathV1}from"../../🔍️discovery/📂️source/🟦️.ts";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync,existsSync} from "node:fs";
import {dirname,join,parse} from "node:path";
import {createRequire} from "node:module";
import {parsePatch,applyPatch,reversePatch} from "diff";
import ts from "typescript";
import schema from "./🧬️schema/🔣️.json";
import rawCorpus from "./🔣️.json";

type Case={name:string;elapsed:number;change:boolean;matches:boolean};
const validate=new Ajv({strict:false}).compile<Case>(schema.items),corpus=rawCorpus.map(row=>{if(!validate(row))throw Error(JSON.stringify(validate.errors));return row as Case;}),require=createRequire(import.meta.url);
const ownPatch=parsePatch(readFileSync(join(import.meta.dir,"..","nx@23.2.0.patch"),"utf8"));
function vendorSource(file:string):string{
 const nx=dirname(require.resolve("nx/package.json"));let owner=nx,prior:string|undefined;
 while(owner!==parse(owner).root){const manifest=join(owner,"package.json");if(existsSync(manifest)){const row=JSON.parse(readFileSync(manifest,"utf8")),patch=row.patchedDependencies?.["nx@23.2.0"];if(typeof patch==="string"&&existsSync(join(owner,patch))){prior=readFileSync(join(owner,patch),"utf8");break;}}owner=dirname(owner);}
 if(!prior)throw Error("Actual immutable dependency patch receipt required");
 const input=readFileSync(join(nx,file),"utf8"),old=parsePatch(prior).find(row=>row.oldFileName==="a/"+file),original=old?applyPatch(input,reversePatch(old)):input;
 if(original===false)throw Error("Exact installed Nx baseline required");
 const patch=ownPatch.find(row=>row.oldFileName==="a/"+file),result=patch?applyPatch(original,patch):original;
 if(result===false)throw Error("Current authored Nx patch must apply exactly");return result;
}
function declaration(source:string,name:string):string{
 const ast=ts.createSourceFile("bridge.js",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),node=ast.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);
 if(!node)throw Error("Actual Nx declaration required: "+name);return node.getText(ast);
}
function outputModule(){
 const order:string[]=[],native={getFilesForOutputsBatch:(_root:string,rows:string[][])=>{order.push("scan");return rows;}},watcher={registerSemioOutputExpressionsV1:async(rows:string[])=>{order.push("register:"+rows.join(","));},flushPendingWorkspaceChanges:async()=>{order.push("flush");}},exports:Record<string,any>={};
 new Function("require","exports",vendorSource("dist/src/daemon/server/outputs-tracking.js"))((name:string)=>name==="path"?require("node:path"):name==="../../native"?native:name==="./watcher"?watcher:name==="../../utils/workspace-root"?{workspaceRoot:"/repo"}:name==="../../utils/collapse-expanded-outputs"?{collapseExpandedOutputs:(rows:string[])=>rows}:(()=>{throw Error(name);})(),exports);
 return{exports,order,watcher};
}

test("closed immediate-output vectors agree with SQLite and actual patched invalidation",async()=>{
 const db=new Database(":memory:");try{for(const row of corpus){expect(validate(row),row.name).toBe(true);const oracle=db.query("SELECT NOT ?1 matches").get(row.change?1:0) as {matches:number};expect(Boolean(oracle.matches)).toBe(row.matches);
 const model=outputModule(),entry=[{outputs:["dist/result"],hash:"h"}];await model.exports.recordOutputsHashBatch(entry);expect(model.order).toEqual(["register:dist/result","flush","scan"]);
 if(row.change)model.exports.processFileChangesInOutputs([{path:"dist/result",type:"update"}],Date.now()+row.elapsed);
 expect(await model.exports.outputsHashesMatchBatch(entry),row.name).toEqual([row.matches]);expect(model.order.at(-2)).toBe("flush");expect(model.order.at(-1)).toBe("scan");
 }}finally{db.close();}
});

test("output baseline and RPC replies await the actual observation barriers",async()=>{
 const model=outputModule();let release!:()=>void;model.watcher.registerSemioOutputExpressionsV1=async()=>{model.order.push("register");await new Promise<void>(resolve=>{release=resolve;});model.order.push("ready");};
 const pending=model.exports.recordOutputsHashBatch([{outputs:["receipt"],hash:"h"}]);expect(model.order).toEqual(["register"]);release();await pending;expect(model.order).toEqual(["register","ready","flush","scan"]);
 const exports:Record<string,any>={};new Function("require","exports",vendorSource("dist/src/daemon/server/handle-outputs-tracking.js"))(()=>({recordOutputsHashBatch:async()=>{throw Error("registration failed");},outputsHashesMatchBatch:async()=>[true,false]}),exports);
 expect((await exports.handleRecordOutputsHashBatch({data:[]})).error.message).toContain("registration failed");expect((await exports.handleOutputsHashesMatchBatch({data:[]})).response).toEqual([true,false]);
});

test("zero Source events still drain output and marker delivery before graph reuse",async()=>{
 const source=vendorSource("dist/src/daemon/server/watcher.js"),order:string[]=[];let release!:()=>void;
 const run=new Function("shutdown_utils_1","flushSemioOutputChangesV1","dispatchWorkspaceChanges",declaration(source,"flushPendingWorkspaceChanges")+";return flushPendingWorkspaceChanges;");
 const flush=run({getWatcherInstance:()=>({forceFlushPending:()=>[]})},async()=>{order.push("output");await new Promise<void>(resolve=>{release=resolve;});order.push("drained");},()=>{throw Error("Empty Source dispatch forbidden");});
 let done=false;const pending=flush().then(()=>{done=true;});await Promise.resolve();expect(order).toEqual(["output"]);expect(done).toBe(false);release();await pending;expect(order).toEqual(["output","drained"]);expect(done).toBe(true);
 const raw=declaration(source,"dispatchWorkspaceChanges"),dispatch=new Function("semioOutputObserver","semioExcluded","semioReceive","semioPolicy","semioNativeChanges","workspaceChangesCallback","path_1","workspace_root_1","server_logger_1","activeServer","require","let semioChangedFileHashes;"+raw+";return dispatchWorkspaceChanges;")({receiveSource:(events:unknown)=>order.push("raw:"+JSON.stringify(events))},()=>true,()=>{throw Error("Excluded event must not reach Source journal");},null,()=>[],()=>{throw Error("Excluded event must not reach graph");},require("node:path"),{workspaceRoot:"/repo"},{},undefined,()=>{throw Error("No module load");});
 const events=[{path:".cache/receipt",type:"update"}];dispatch(events);expect(order.at(-1)).toBe("raw:"+JSON.stringify(events));
});

test("Native startup failure settles its allocated watcher and retains both errors",async()=>{
 const source=vendorSource("dist/src/daemon/server/watcher.js");let adapter:any,stops=0,failStop=false;
 class Watcher{watch(){throw Error("quota");}async stop(){stops++;if(failStop)throw Error("stop");}}
 const observer={register:async()=>{},flush:async()=>{},stop:async()=>{}},loaded={authorizers:{createNxDaemonOutputObservationV1:(watch:any)=>{adapter=watch;return observer;}}};
 const load=new Function("require","handle_import_1","path_1","path_2","workspace_root_1","__dirname","let semioOutputReady,semioOutputObserver;"+declaration(source,"semioOutputObservationV1")+";return semioOutputObservationV1;")((name:string)=>name==="../../utils/workspace-context"?{loadSemioGraphAuthorizersV1:async()=>loaded}:{handleOutputsChanges:()=>{}},{handleImport:async()=>({Watcher})},require("node:path"),{normalizePath:(s:string)=>s.replaceAll("\\","/")},{workspaceRoot:"/repo"},".");
 await load();await expect(adapter({path:"/repo/dist"},()=>{})).rejects.toThrow("quota");expect(stops).toBe(1);failStop=true;try{await adapter({path:"/repo/dist"},()=>{});throw Error("Expected failure");}catch(error){expect(error).toBeInstanceOf(AggregateError);expect((error as AggregateError).errors.map(row=>row.message)).toEqual(["quota","stop"]);}expect(stops).toBe(2);
});

test("private marker delivery signals shutdown without awaiting its own stop",async()=>{
 const source=vendorSource("dist/src/daemon/server/watcher.js"),order:string[]=[];let callback:any,stopFailure=true,signals=0;
 const observer={stop:async()=>{order.push("output-stop");if(stopFailure)throw Error("output stop");}},marker={stop:async()=>{order.push("marker-stop");if(stopFailure)throw Error("marker stop");}};
 const loaded={authorizers:{createNxDaemonProcessMarkerObservationV1:async(path:string,receive:any)=>{expect(path).toBe("/owned/daemon/p.json");callback=receive;return marker;},requestNxDaemonShutdownV1:()=>{signals++;order.push("shutdown-signal");}}};
 const watch=new Function("require","semioOutputObservationV1","cache_1","process","let semioOutputObserver,semioOutputReady,semioMarkerObserver;"+declaration(source,"watchOutputFiles")+";return watchOutputFiles;")(()=>({loadSemioGraphAuthorizersV1:async()=>loaded}),async()=>observer,{serverProcessJsonPath:"/owned/daemon/p.json",getDaemonProcessIdSync:()=>2},{pid:1});
 const handle=await watch({},async(error:unknown)=>{expect(error).toBeInstanceOf(Error);order.push("terminal-delivery");});
 await callback(null,[{type:"update",path:"p.json"}]);expect(order).toEqual(["shutdown-signal","terminal-delivery"]);expect(signals).toBe(1);
 try{await handle.stop();throw Error("Expected stop failure");}catch(error){expect(error).toBeInstanceOf(AggregateError);expect((error as AggregateError).errors.map(row=>row.message)).toEqual(["output stop","marker stop"]);}expect(order.slice(-2)).toEqual(["output-stop","marker-stop"]);
});

test("fatal shutdown reports primary and independent observer failure before exiting",async()=>{
 const source=vendorSource("dist/src/daemon/server/shutdown-utils.js"),primary=Error("source stop"),secondary=Error("output stop"),logged:unknown[]=[],exits:number[]=[];
 const frozen={authorizers:{beginNxDaemonShutdownV1:()=>{},markNxDaemonServerClosedV1:()=>{},completeNxDaemonShutdownV1:async()=>{throw secondary;}}},exit=Error("owned process exit");
 const stop=new Function("require","watcherInstance","outputWatcherInstance","process","console","let semioShutdownPromise;"+declaration(source,"performShutdown")+";return performShutdown;")(()=>({loadSemioGraphAuthorizersV1:async()=>frozen}),{stop:async()=>{throw primary;}},undefined,{exit:(status:number)=>{exits.push(status);throw exit;}},{error:(error:unknown)=>logged.push(error)});
 await expect(stop({close:(done:()=>void)=>done()},"test",[])).rejects.toBe(exit);expect(exits).toEqual([1]);expect(logged.length).toBe(1);expect(logged[0]).toBeInstanceOf(AggregateError);expect((logged[0] as AggregateError).errors).toEqual([primary,secondary]);
});

test("service settlement retains custody after an observer stop failure",async()=>{
 const service=readFileSync(join(import.meta.dir,"..","..","🕸️graph/🏁️startup/🚦️service/🟦️.ts"),"utf8"),ast=ts.createSourceFile("service.ts",service,ts.ScriptTarget.Latest,true),methods:ts.MethodDeclaration[]=[];
 const visit=(node:ts.Node)=>{if(ts.isMethodDeclaration(node)&&node.name.getText(ast)==="resourcesStopped")methods.push(node);ts.forEachChild(node,visit);};visit(ast);expect(methods.length).toBe(1);
 const method=ts.transpile(`const owner={${methods[0]!.getText(ast)}};`,{target:ts.ScriptTarget.ESNext});let finished=0,released=0,calls=0,fail=true;
 const observer={stop:async()=>{calls++;if(fail)throw Error("still live");}},others={stop:async()=>{calls++;}},observations=new Set([observer,others]);
 const owner=new Function("observations","finish","released",`let stopped=false;${method};return {stop:()=>owner.resourcesStopped(),settled:()=>stopped};`)(observations,()=>{finished++;},{then:(resolve:()=>void)=>{released++;resolve();}});
 await expect(owner.stop()).rejects.toThrow("still live");expect(calls).toBe(2);expect(owner.settled()).toBe(false);expect(finished).toBe(0);expect(released).toBe(0);
 fail=false;await owner.stop();expect(calls).toBe(4);expect(owner.settled()).toBe(true);expect(finished).toBe(1);expect(released).toBe(1);
});


test("workspace startup registers exactly the closed directory policy before ready",async()=>{
 const fs=await import("node:fs"),path=await import("node:path"),schema=JSON.parse(readFileSync(join(import.meta.dir,"🧬️schema/📥️watch.json"),"utf8")),row=JSON.parse(readFileSync(join(import.meta.dir,"📥️watch.json"),"utf8"));
 expect(new Ajv({strict:true}).validate(schema,row)).toBe(true);
 const db=new Database(":memory:");
 try{
  const dirs=new Set<string>([""]);for(const dir of row.directories){const parts=dir.split("/");for(let end=1;end<=parts.length;end++)dirs.add(parts.slice(0,end).join("/"));}
  const expected=[...dirs].filter(dir=>!(db.query("SELECT EXISTS(SELECT 1 FROM json_each(?1) WHERE ?3=value OR substr(?3,1,length(value)+1)=value||'/') OR EXISTS(SELECT 1 FROM json_each(?2) WHERE instr('/'||?3||'/','/'||value||'/')>0) OR EXISTS(SELECT 1 FROM json_each(?4) prefix,json_each(?5) part WHERE substr(part.value,1,length(prefix.value))=prefix.value AND length(part.value)=length(prefix.value)+?6 AND substr(part.value,length(prefix.value)+1) NOT GLOB '*[^0-9a-f]*') excluded").get(JSON.stringify(row.roots),JSON.stringify(row.names),dir,JSON.stringify(SOURCE_OPACITY_PROTOCOL_V1.prefixes),JSON.stringify(dir.split('/')),SOURCE_OPACITY_PROTOCOL_V1.digestHex)as{excluded:number}).excluded).sort();
  expect(expected).toEqual(row.expected);
  for(const entry of row.changes){const oracle=db.query("SELECT EXISTS(SELECT 1 FROM json_each(?1) part,json_each(?2) prefix WHERE substr(part.value,1,length(prefix.value))=prefix.value AND length(part.value)=length(prefix.value)+?3 AND substr(part.value,length(prefix.value)+1) NOT GLOB '*[^0-9a-f]*') excluded").get(JSON.stringify(entry.path.split("/")),JSON.stringify(SOURCE_OPACITY_PROTOCOL_V1.prefixes),SOURCE_OPACITY_PROTOCOL_V1.digestHex)as{excluded:number};expect(Boolean(oracle.excluded),entry.path).toBe(entry.excluded);}

  for(const entry of row.moves)for(const [endpoint,excluded]of[[entry.from,entry.fromExcluded],[entry.to,entry.excluded]]as const){const oracle=db.query("SELECT EXISTS(SELECT 1 FROM json_each(?1) part,json_each(?2) prefix WHERE substr(part.value,1,length(prefix.value))=prefix.value AND length(part.value)=length(prefix.value)+?3 AND substr(part.value,length(prefix.value)+1) NOT GLOB '*[^0-9a-f]*') excluded").get(JSON.stringify(endpoint.split("/")),JSON.stringify(SOURCE_OPACITY_PROTOCOL_V1.prefixes),SOURCE_OPACITY_PROTOCOL_V1.digestHex)as{excluded:number};expect(Boolean(oracle.excluded),endpoint).toBe(excluded);}
  if(process.platform!=="linux")return;
  const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifact)throw Error("Caller-owned watcher artifacts required");fs.mkdirSync(artifact,{recursive:true});const root=fs.mkdtempSync(join(artifact,"wp-"));
  let watcher:any,journal:Awaited<ReturnType<typeof createRepositoryFilesystemJournalV1>>|undefined;try{
   for(const dir of row.directories)fs.mkdirSync(join(root,dir),{recursive:true});for(const[name,text]of Object.entries(row.files))fs.writeFileSync(join(root,name),text as string);
   const source=vendorSource("dist/src/daemon/server/watcher.js"),policy={roots:row.roots,names:row.names,opaqueBasenames:SOURCE_OPACITY_PROTOCOL_V1},adapter=await import("../../🕸️graph/🏁️startup/🚦️service/👁️workspace/🟦️.ts"),{createWatchSupervisorV1}=await import("../../../🏗️build/🏡️resident/🔭️watch/🟨️.mjs"),{spawn}=await import("node:child_process"),pin=workspaceNativePin,binary=join(process.cwd(),".cache/b/t/m/repo-watch",pin.sources[0]!.sha256.slice(0,16),"semio-repo-watch");let child:any;const backend={createNxDaemonWorkspaceWatcherV1:async(signal:AbortSignal)=>({policy,watcher:await adapter.createRepositoryWorkspaceWatcherV1(root,policy,join(root,row.privateFence),binary,{signal,remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}},{createSupervisor:options=>createWatchSupervisorV1({...options,spawnProcess:(...args:any[])=>{child=(spawn as any)(...args);return child;}})})})};
   const received:{path:string;type:string}[]=[],errors:unknown[]=[],subscribers=new Set<(error:unknown,events:readonly {path:string;type:"create"|"update"|"delete"}[])=>void>(),reconciled:string[]=[];
   const start=new Function("handle_import_1","workspace_root_1","loadSemioFilesystemJournalOwners","dispatchWorkspaceChanges","semioReceive","stopSemioJournalThreadV1","__dirname","let activeServer,workspaceChangesCallback,semioGeneration=0,semioLifetime,semioJournal,semioJournalPromise,semioError,semioPending,semioPolicy,semioWatcher,semioThread;const semioSubscribers=new Set();let semioToolingIdentity='actual-test-policy';"+declaration(source,"watchWorkspace")+";return watchWorkspace;")({handleImport:async()=>{throw Error("Whole workspace Native registration forbidden");}},{workspaceRoot:root},async()=>[backend],(events:any[])=>{received.push(...events);for(const receive of subscribers)receive(null,events);},()=>{},()=>{},".");
   watcher=await start({},(error:unknown)=>{if(error)errors.push(error);});
   const subscriptions=()=>{const inodes=new Set<string>();for(const file of fs.readdirSync("/proc/"+child.pid+"/fdinfo")){let text="";try{text=fs.readFileSync("/proc/"+child.pid+"/fdinfo/"+file,"utf8");}catch{}for(const line of text.split("\n")){const match=line.match(/^inotify wd:.* ino:([0-9a-f]+)/u);if(match)inodes.add(BigInt("0x"+match[1]).toString());}}return inodes;},inodes=subscriptions();
   const actual=[...dirs].filter(dir=>inodes.has(fs.statSync(join(root,dir),{bigint:true}).ino.toString())).sort();
   expect(actual).toEqual([...expected,row.privateFence].sort());
   const control={signal:AbortSignal.timeout(30000),remainingMilliseconds:()=>30000,publish:async()=>{},yieldContinuation:async()=>{}},allFiles=()=>{const files:string[]=[],pending=[""];while(pending.length){const directory=pending.pop()!;for(const entry of fs.readdirSync(join(root,directory),{withFileTypes:true})){const file=directory?directory+"/"+entry.name:entry.name;if(entry.isDirectory())pending.push(file);else if(entry.isFile())files.push(file);}}return files;};
   journal=await createRepositoryFilesystemJournalV1(root,{watch(receive){subscribers.add(receive);return()=>{subscribers.delete(receive);};},async flush(){await watcher.forceFlushPending();return[];},async census(){return allFiles();},async admittedPaths(paths){reconciled.push(...paths);return paths;},isGlobalIdentity:()=>false},control);
   const scope={roots:expected.filter(path=>path!==""),files:row.changes.filter((entry:any)=>!entry.excluded).map((entry:any)=>entry.path)};
   await journal.capture(scope,control);
   fs.mkdirSync(join(root,row.opaqueDirectory),{recursive:true});fs.mkdirSync(join(root,row.visibleDirectory),{recursive:true});await watcher.forceFlushPending();
   expect(received.some(event=>event.path===row.visibleDirectory.split("/")[0]||event.path===row.visibleDirectory)).toBe(true);
   const subscriptionsAfterDirectory=subscriptions();expect(subscriptionsAfterDirectory.has(fs.statSync(join(root,row.opaqueDirectory),{bigint:true}).ino.toString())).toBe(false);expect(subscriptionsAfterDirectory.has(fs.statSync(join(root,row.visibleDirectory),{bigint:true}).ino.toString())).toBe(true);
   for(const entry of row.changes){fs.writeFileSync(join(root,entry.path),"changed");}
   await watcher.forceFlushPending();const witness=await journal.capture(scope,control);
   for(const entry of row.changes){expect(received.some(event=>event.path===entry.path),entry.path).toBe(!entry.excluded);expect(witness.selectedMembership.paths.includes(entry.path),entry.path).toBe(!entry.excluded);expect(reconciled.includes(entry.path),entry.path).toBe(!entry.excluded);}
   expect(received.some(event=>isOpaqueSourcePathV1(event.path))).toBe(false);expect(witness.changes.some(event=>isOpaqueSourcePathV1(event.path))).toBe(false);expect(witness.selectedMembership.paths.some(isOpaqueSourcePathV1)).toBe(false);expect(reconciled.some(isOpaqueSourcePathV1)).toBe(false);


   for(const move of row.moves){
    fs.mkdirSync(path.dirname(join(root,move.to)),{recursive:true});
    fs.mkdirSync(path.dirname(join(root,move.from,move.file)),{recursive:true});fs.writeFileSync(join(root,move.from,move.file),"before");await watcher.forceFlushPending();await journal.capture(scope,control);
    const start=received.length;fs.renameSync(join(root,move.from),join(root,move.to));fs.writeFileSync(join(root,move.to,move.file),"after");await watcher.forceFlushPending();const moved=received.slice(start),membership=await journal.capture(scope,control);
    expect(moved.some(event=>event.path===move.to+"/"+move.file),move.to).toBe(!move.excluded);
    if(move.excluded)expect(moved.some(event=>event.path===move.from+"/"+move.file),move.from).toBe(false);
    expect(subscriptions().has(fs.statSync(join(root,move.to,path.dirname(move.file)),{bigint:true}).ino.toString()),move.to).toBe(!move.excluded);
    expect(membership.selectedMembership.paths.includes(move.to+"/"+move.file),move.to).toBe(!move.excluded);expect(membership.changes.some(event=>isOpaqueSourcePathV1(event.path))).toBe(false);
   }

   fs.writeFileSync(join(root,"source/ignored/new.json"),"{}");fs.mkdirSync(join(root,"source/generated/later"),{recursive:true});await watcher.forceFlushPending();
   expect(errors).toEqual([]);expect(received.some(event=>event.path==="source/ignored/new.json")).toBe(true);expect(received.some(event=>event.path.includes("generated")||event.path.startsWith(".cache")||event.path.startsWith(".tickets"))).toBe(false);
   fs.mkdirSync(join(root,"source/ignored/new/deep"),{recursive:true});fs.writeFileSync(join(root,"source/ignored/new/deep/file"),"new");await watcher.forceFlushPending();expect(received.some(event=>event.path==="source/ignored/new")).toBe(true);
   const later=subscriptions();expect(later.has(fs.statSync(join(root,"source/ignored/new/deep"),{bigint:true}).ino.toString())).toBe(true);expect(later.has(fs.statSync(join(root,"source/generated/later"),{bigint:true}).ino.toString())).toBe(false);
  }finally{const failures:unknown[]=[];try{await journal?.close();}catch(error){failures.push(error);}try{await watcher?.stop();}catch(error){failures.push(error);}fs.rmSync(root,{recursive:true,force:true});if(failures.length)throw new AggregateError(failures,"Physical watcher cleanup failed");}
 }finally{db.close();}
});
