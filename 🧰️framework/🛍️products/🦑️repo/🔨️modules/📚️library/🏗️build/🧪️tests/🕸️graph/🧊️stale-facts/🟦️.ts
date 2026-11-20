/** 🧊️ Contract: a source fact that a repeated inference leaves exactly as it was (a process that reads a pinned implementation file whose bytes changed on disk) can never be refreshed by this process, so the attempt ends as a stale implementation after the first repair that changed nothing, instead of repairing and restarting for ever. */
import{decodeAuthorityObservationsV1,distinctObservationRowsV1,encodeAuthorityObservationsV1}from"../../../👁️observation/🔢️row-table/🟨️.mjs";
import authoritySchema from"../../../🧬️schema/📥️authority.json";
import sliceReportSchema from "../../../🧠️memory/📋️slice-report/🧬️schema/🔣️.json" with {type:"json"};
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{join,dirname}from"node:path";
import ts from"typescript";

const LOAD=process.env.R151_LOAD_SOURCE??fileURLToPath(new URL("../../../🕸️graph/📥️load/🟦️.ts",import.meta.url));
const limits=JSON.parse(readFileSync(process.env.R151_SCHEMA??fileURLToPath(new URL("../../../🕸️graph/🧬️schema/🔣️.json",import.meta.url)),"utf8")) as {definitions:{Limits:{const:Record<string,number>};LimitsDescriptions:{const:{en:Record<string,string>;de:Record<string,string>}}}};
const RETRY=process.env.R156_RETRY??new URL("../../../🕸️graph/🔁retry/🟨️.mjs",import.meta.url).href,{sealVerdictV1,reportSealV1}=await import(RETRY);
const source=readFileSync(LOAD,"utf8"),tree=ts.createSourceFile("load.ts",source,ts.ScriptTarget.Latest,true);

const extract=(name:string):string=>{
 const node=tree.statements.find((each):each is ts.FunctionDeclaration=>ts.isFunctionDeclaration(each)&&each.name?.text===name);if(!node)throw Error("Actual function required: "+name);
 const moved=ts.transform(node,[context=>{const visit:ts.Visitor=each=>ts.isPropertyAccessExpression(each)&&ts.isMetaProperty(each.expression)?ts.factory.createStringLiteral("file:///owned/graph.ts"):ts.isMetaProperty(each)?ts.factory.createIdentifier("importMetaV1"):ts.isCallExpression(each)&&each.expression.kind===ts.SyntaxKind.ImportKeyword?ts.factory.createCallExpression(ts.factory.createIdentifier("importModuleV1"),undefined,ts.visitNodes(each.arguments,visit,ts.isExpression)):ts.visitEachChild(each,visit,context);return each=>ts.visitNode(each,visit)as ts.FunctionDeclaration;}]);
 const text=ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,moved.transformed[0]!,tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;moved.dispose();return text;
};

type Row={kind:"file";path:string;fact:{version:number}};

const world=(owners:number,scenario:{onDiscover?:(call:number,tree:Map<string,number>)=>void;digestAt?:(call:number)=>string;pinned?:ReadonlySet<string>}={})=>{
 const tree=new Map<string,number>(),names=Array.from({length:owners},(_,index)=>"owner-"+index);
 for(const name of names)for(const file of["manifest","src"])tree.set(name+"/"+file,1);
 tree.set("tooling/shared.ts",1);tree.set("elsewhere/unrelated.ts",1);
 const lines:string[]=[],stack:Map<string,Row>[]=[],observed:Record<string,number>={},counts={closes:0,discovered:0,holds:0,memoWrites:0};
 const record=(path:string)=>{const row:Row={kind:"file",path,fact:{version:scenario.pinned?.has(path)?1:tree.get(path)??0}};for(const each of stack)each.set(path,row);};
  const observation={
  observedFs:{readFileSync:(path:string|URL)=>{record(typeof path==="string"?path:path.pathname);return"";}},
  captureObservedSourceInputsV1:async(body:()=>Promise<unknown>,_failure?:unknown,authority=true)=>{const mine=new Map<string,Row>(),parent=stack.at(-1);stack.push(mine);try{const value=await body();return{value,observations:[...mine.values()],authorityObservations:authority?[]:[]};}finally{stack.pop();if(parent)for(const[path,row]of mine)parent.set(path,row);}},
  settleObservedComputationV1:async(body:()=>Promise<unknown>)=>body(),
  staleSourceFactsErrorV1:(rows:readonly Row[])=>Object.assign(new Error("stale "+rows.map(row=>row.path).join(",")),{code:"BUILD_AUTHORITY_CHANGED"}),
  retainObservedRowsV1:(rows:readonly Row[])=>{for(const row of rows)for(const each of stack)each.set(row.path,row);},
 };
 const registry={version:1,owners:names.map(name=>({name}))},admitted=()=>({owners:names,files:[],targets:{},catalogProviders:[],inputs:{}});
 const producer={
  readBuildOwnerRegistrySourceV1:()=>JSON.stringify(registry),libraryBootstrap:Promise.resolve(),prepareSourceAnalysesV1:()=>{},
  discoverBuildOwnerAdmissionFactsV1:async(owner:{name:string})=>{observed[owner.name]=(observed[owner.name]??0)+1;observation.observedFs.readFileSync(owner.name+"/manifest");observation.observedFs.readFileSync("tooling/shared.ts");return{project:{name:owner.name,targets:{}},files:[],dependencies:[]};},
 };
 const bindings={distinctObservationRowsV1,decodeAuthorityObservationsV1,encodeAuthorityObservationsV1,sourceAuthoritySchema:authoritySchema,sliceReportSchema,
  importMetaV1:{url:"file:///owned/graph.ts"},graphSelfUrlsV1:()=>[],physicalSourceOriginV1:(url:string)=>new URL(url),observationV1:observation,
  importModuleV1:async(specifier:string)=>specifier==="../📥️inference/🟨️.mjs"?producer:{},
  requireIssuedResourceControlInvocationV1:()=>{},checkResourceLeaseControl:()=>{},checkScriptInvocation:()=>{},
  process:{env:{},stderr:{write:(text:string)=>{lines.push(text);return true;}}},performance,JSON,Buffer,URL,join,dirname,
  buildGraphPathV1:()=>"/owned/cache/canonical.json",graphDeclarationV1:()=>({properties:{authority:{const:"authority.json"}}}),
  createRequire:()=>(()=>observation),readBuildOwnerRegistryV1:(value:unknown)=>value,buildGraphScopeKeyV1:()=>"scope",
  ownerRegistryInvalidationV1:{kind:"invalidated",reason:"owner-registry"},isOwnerRegistryInvalidationV1:(value:unknown)=>value===undefined?false:(value as{kind?:string})?.kind==="invalidated",
  buildGraphErrorCodeV1:(error:unknown)=>(error as{code?:string})?.code,
  readAdmissionMemoV1:async()=>undefined,writeAdmissionMemoV1:async()=>{counts.memoWrites++;},
  buildGraphProducerAuthorityV1:()=>true,loadedGraphModuleAuthorityV1:{implementation:"same",publication:"same"},loadedGraphAdmissionFormatV1:"same",selectedAdmissionFactsV1:{entries:()=>[],seed:()=>{}},packAdmissionMemoV1:(value:unknown)=>value,ownersOfAdmissionMemoV1:()=>[],
  closeBuildOwnerAdmissionV1:async(_selection:unknown,_registry:unknown,inspect:(owner:{name:string},targets:string[],inputs:string[])=>Promise<unknown>)=>{counts.closes++;for(const name of names)await inspect({name},[],[]);return admitted();},
  buildGraphMembershipV1:async()=>scenario.digestAt?.(counts.discovered)??"membership",buildGraphMembersV1:(files:string[])=>files,workspaceBuildFilesV1:async()=>[],inventoryDirectoriesV1:new WeakMap(),
  readFile:async()=>{throw Object.assign(Error("absent"),{code:"ENOENT"});},
  discoverBuildGraphV1:async()=>{counts.discovered++;for(const name of names)observation.observedFs.readFileSync(name+"/src");scenario.onDiscover?.(counts.discovered,tree);return{version:1,projects:{},dependencies:[]};},
  buildGraphSourceFactsHoldV1:async(rows:readonly Row[],_invocation?:unknown,onStale?:(row:Row)=>void)=>{counts.holds++;const stale=rows.filter(row=>(tree.get(row.path)??0)!==row.fact.version);for(const row of stale)onStale?.(row);return stale.length===0;},
  mkdir:async()=>{},writeFile:async()=>{},sealVerdictV1,reportSealV1,sliceReadStepV1:(_id:string,run:()=>unknown)=>run(),rename:async()=>{},rm:async()=>{},createHash:()=>({update:()=>({digest:()=>"digest"})}),randomUUID:()=>"nonce",
  graphSnapshotPolicyV1:{maximumSnapshotBytes:1<<30},portableInputs:async(_root:string,value:unknown)=>value,readBuildGraphV1:(value:unknown)=>value,
  GRAPH_LIMITS_V1:limits.definitions.Limits.const,observeOwnerFactsMemoizedV1:undefined as unknown,
 };
 if(source.includes("function observeOwnerFactsMemoizedV1"))bindings.observeOwnerFactsMemoizedV1=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1",extract("rememberOwnerFactsV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(new Map(),limits.definitions.Limits.const);
 const run=async()=>{const execute=Function(...Object.keys(bindings),extract("createGraphInventoryAwaitReportV1")+";"+extract("loadBuildGraphAttemptV1")+";return loadBuildGraphAttemptV1;")(...Object.values(bindings));return execute("/owned",{control:{signal:new AbortController().signal}},{},{selection:{kind:"tasks",tasks:[{project:"owner-0",target:"build"}]}},registry,"scope");};
 return{run,tree,observed,counts,names,lines};
};


test("a repair that leaves the same stale source fact ends the attempt as a stale implementation after one round",async()=>{
 const state=world(4,{pinned:new Set(["owner-1/src"]),onDiscover:(call,tree)=>{if(call===1)tree.set("owner-1/src",2);}});
 const error=await state.run().then(()=>undefined,failure=>failure as{code?:string;message?:string});
 expect(error?.code).toBe("BUILD_AUTHORITY_CHANGED");expect(error?.message).toContain("owner-1/src");
 expect(state.counts.discovered).toBe(2);
 expect(state.lines.filter(line=>line.startsWith("[graph-repair] "))).toHaveLength(1);
});

test("a repair that refreshes the fact, or a fact that moves again at every round, is not mistaken for a stuck process",async()=>{
 const fresh=world(4,{onDiscover:(call,tree)=>{if(call===1)tree.set("owner-1/src",2);}});
 expect(await fresh.run()).toBeDefined();expect(fresh.counts.discovered).toBe(2);
 const moving=world(4,{onDiscover:(_call,tree)=>{tree.set("owner-1/src",(tree.get("owner-1/src")??0)+1);}});
 expect(await moving.run()).toBeDefined();expect(moving.counts.discovered).toBe(1+limits.definitions.Limits.const.repairRounds);
});
