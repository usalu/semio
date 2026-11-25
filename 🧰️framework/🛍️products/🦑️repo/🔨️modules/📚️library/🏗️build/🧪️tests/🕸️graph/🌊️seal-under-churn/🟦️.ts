/** 🌊️ Contract: a read of a tree that changes while it is read seals in bounded time, with work proportional to the changes, and an owner whose file changed after its observation is stale in the NEXT read only (200 seeded worlds, the real `loadBuildGraphAttemptV1` and the real owner facts memo over a model tree whose files change on a logical clock). */
import{decodeAuthorityObservationsV1,distinctObservationRowsV1,encodeAuthorityObservationsV1}from"../../../👁️observation/🔢️row-table/🟨️.mjs";
import sliceReportSchema from "../../../🧠️memory/📋️slice-report/🧬️schema/🔣️.json" with {type:"json"};
import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{posix}from"node:path";
import ts from"typescript";
import{graphInventoryTextV1,graphRetryTextV1,graphSettleTextV1,graphDirtyTextV1,graphVolatileTextV1,membershipChangePathsV1,followedPathsV1,untoleratedFollowedPathsV1}from"../../../🕸️graph/📥️load/🧲️membership-follow/🟦️.ts";
import{settleInventoryRoundsV1}from"../../../🕸️graph/📥️load/🔁️inventory-rounds/🟦️.ts";
import{observeOwnerWithRetryV1,ownerWithheldTextV1}from"../../../🕸️graph/📥️load/🛟️owner-isolation/🟦️.ts";
import Ajv from"ajv";
import{createHash}from"node:crypto";
import authoritySchema from"../../../🧬️schema/📥️authority.json";
import retrySchema from"../../../🕸️graph/🔁retry/🧬️schema/🔣️.json";
const pagesV1={admit:()=>undefined,facts:(entry:{facts:unknown})=>entry.facts,release:()=>undefined};

const LOAD=process.env.R151_LOAD_SOURCE??fileURLToPath(new URL("../../../🕸️graph/📥️load/🟦️.ts",import.meta.url));
const RETRY=process.env.R156_RETRY??new URL("../../../🕸️graph/🔁retry/🟨️.mjs",import.meta.url).href,{sealVerdictV1,reportSealV1}=await import(RETRY);
const limits=JSON.parse(readFileSync(process.env.R151_SCHEMA??fileURLToPath(new URL("../../../🕸️graph/🧬️schema/🔣️.json",import.meta.url)),"utf8")) as {definitions:{Limits:{const:Record<string,number>}}};
const ROUNDS=limits.definitions.Limits.const.repairRounds!;
const source=readFileSync(LOAD,"utf8"),tree=ts.createSourceFile("load.ts",source,ts.ScriptTarget.Latest,true);

const extract=(name:string):string=>{
 const node=tree.statements.find((each):each is ts.FunctionDeclaration=>ts.isFunctionDeclaration(each)&&each.name?.text===name);if(!node)throw Error("Actual function required: "+name);
 const moved=ts.transform(node,[context=>{const visit:ts.Visitor=each=>ts.isPropertyAccessExpression(each)&&ts.isMetaProperty(each.expression)?ts.factory.createStringLiteral("file:///owned/graph.ts"):ts.isMetaProperty(each)?ts.factory.createIdentifier("importMetaV1"):ts.isCallExpression(each)&&each.expression.kind===ts.SyntaxKind.ImportKeyword?ts.factory.createCallExpression(ts.factory.createIdentifier("importModuleV1"),undefined,ts.visitNodes(each.arguments,visit,ts.isExpression)):ts.visitEachChild(each,visit,context);return each=>ts.visitNode(each,visit)as ts.FunctionDeclaration;}]);
 const text=ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,moved.transformed[0]!,tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;moved.dispose();return text;
};

const generator=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let t=Math.imul(seed^seed>>>15,1|seed);t=t+Math.imul(t^t>>>7,61|t)^t;return((t^t>>>14)>>>0)/4294967296;};
const PERIODS=[2,5,11,25,60,150,600],LOOK=1,INFER=12,OBSERVE=3,HOLD=1,INVENTORY=2;

type Row={kind:"file";path:string;fact:{version:number}};

const world=(seed:number)=>{
 const pick=generator(seed),churn=generator(seed*7919+1),owners=3+Math.floor(pick()*8),period=PERIODS[Math.floor(pick()*PERIODS.length)]!;
 const names=Array.from({length:owners},(_,index)=>"owner-"+index),files=[...names.flatMap(name=>[name+"/manifest",name+"/src"]),"tooling/shared.ts","elsewhere/unrelated.ts"];
 const tree=new Map<string,number>(files.map(file=>[file,1])),changes:string[]=[],records=new Map<string,Row[]>(),store=new Map<string,string>();
 let clock=0,nextChange=period,churning=true;
 const advance=(milliseconds:number)=>{clock+=milliseconds;while(churning&&nextChange<=clock){const file=files[Math.floor(churn()*files.length)]!;tree.set(file,tree.get(file)!+1);changes.push(file);nextChange+=period;}};
 const lines:string[]=[],stack:Map<string,Row>[]=[],inferred:Record<string,number>={},observed:Record<string,number>={},counts={discovered:0};
 for(const name of names){inferred[name]=0;observed[name]=0;}
 const record=(path:string):Row=>{const row:Row={kind:"file",path,fact:{version:tree.get(path)??0}};for(const each of stack)each.set(path,row);return row;};
 const observation={requirePinnedImplementationFactsV1:()=>{},
  observedFs:{readFileSync:(path:string|URL)=>{record(typeof path==="string"?path:path.pathname);return"";}},
  captureObservedSourceInputsV1:async(body:()=>Promise<unknown>,_failure?:unknown,authority=true)=>{const mine=new Map<string,Row>(),parent=stack.at(-1);stack.push(mine);try{const value=await body();return{value,observations:[...mine.values()],authorityObservations:authority?[]:[]};}finally{stack.pop();if(parent)for(const[path,row]of mine)parent.set(path,row);}},
  settleObservedComputationV1:async(body:()=>Promise<unknown>)=>body(),
  staleSourceFactsErrorV1:(rows:readonly Row[])=>Object.assign(new Error("stale "+rows.map(row=>row.path).join(",")),{code:"BUILD_AUTHORITY_CHANGED"}),
  retainObservedRowsV1:(rows:readonly Row[])=>{for(const row of rows)for(const each of stack)each.set(row.path,row);},
 };
 const registry={version:1,owners:names.map(name=>({name}))},admitted=()=>({owners:names,files:[],targets:{},catalogProviders:[],inputs:{}});
 const producer={
  readBuildOwnerRegistrySourceV1:()=>JSON.stringify(registry),libraryBootstrap:Promise.resolve(),prepareSourceAnalysesV1:()=>{},
  discoverBuildOwnerAdmissionFactsV1:async(owner:{name:string})=>{advance(OBSERVE);observed[owner.name]!++;observation.observedFs.readFileSync(owner.name+"/manifest");observation.observedFs.readFileSync("tooling/shared.ts");return{project:{name:owner.name,targets:{}},files:[],dependencies:[]};},
 };
 const hold=async(rows:readonly Row[],_invocation?:unknown,onStale?:(row:Row)=>void)=>{advance(HOLD);const stale=rows.filter(row=>(tree.get(row.path)??0)!==row.fact.version);for(const row of stale)onStale?.(row);return stale.length===0;};
 const bindings={distinctObservationRowsV1,decodeAuthorityObservationsV1,encodeAuthorityObservationsV1,sourceAuthoritySchema:authoritySchema,sliceReportSchema,
  importMetaV1:{url:"file:///owned/graph.ts"},graphSelfUrlsV1:()=>[],physicalSourceOriginV1:(url:string)=>new URL(url),observationV1:observation,
  importModuleV1:async(specifier:string)=>specifier==="../📥️inference/🟨️.mjs"?producer:{},
  requireIssuedResourceControlInvocationV1:()=>{},checkResourceLeaseControl:()=>{},checkScriptInvocation:()=>{},
  process:{env:{},stderr:{write:(text:string)=>{lines.push(text);return true;}}},performance,JSON,Buffer,URL,join:posix.join,dirname:posix.dirname,
  buildGraphPathV1:()=>"/owned/cache/canonical.json",graphDeclarationV1:()=>({properties:{authority:{const:"authority.json"}}}),
  createRequire:()=>(()=>observation),readBuildOwnerRegistryV1:(value:unknown)=>value,buildGraphScopeKeyV1:()=>"scope",ownerRegistryDigestV1:()=>"registry",askOwnerRegistryV1:()=>"registry",
  ownerRegistryInvalidationV1:{kind:"invalidated",reason:"owner-registry"},isOwnerRegistryInvalidationV1:(value:unknown)=>value===undefined?false:(value as{kind?:string})?.kind==="invalidated",
  buildGraphErrorCodeV1:(error:unknown)=>(error as{code?:string})?.code,
  readForgottenPathsV1:()=>({paths:new Map(),listings:new Map()}),rowsForgottenV1:()=>false,forgetPersistedPathsV1:async()=>true,readAdmissionMemoV1:async()=>undefined,writeAdmissionMemoV1:async()=>{},
  buildGraphProducerAuthorityV1:()=>true,loadedGraphModuleAuthorityV1:{implementation:"same",publication:"same"},loadedGraphAdmissionFormatV1:"same",selectedAdmissionFactsV1:{entries:()=>[],suffixes:()=>[],seed:()=>{}},packAdmissionMemoV1:(value:unknown)=>value,ownersOfAdmissionMemoV1:()=>[],
  closeBuildOwnerAdmissionV1:async(_selection:unknown,_registry:unknown,inspect:(owner:{name:string},targets:string[],inputs:string[])=>Promise<unknown>)=>{for(const name of names)await inspect({name},[],[]);return admitted();},
  buildGraphMembershipV1:async()=>{advance(INVENTORY);return"membership";},buildGraphMembersV1:(list:string[])=>list,workspaceBuildFilesV1:async()=>[],inventoryDirectoriesV1:new WeakMap(),
  readFile:async(path:string)=>{const text=store.get(path);if(text===undefined)throw Object.assign(Error("absent"),{code:"ENOENT"});return text;},
  createPassReuseV1:()=>({memo:undefined,carried:{recycles:0,round:0,passes:0,owners:0,heapMegabytes:0},boundary:()=>{}}),discoverBuildGraphV1:async()=>{
   counts.discovered++;
   for(const name of names){
    advance(LOOK);const held=records.get(name);
    if(held!==undefined&&held.every(row=>(tree.get(row.path)??0)===row.fact.version)){observation.retainObservedRowsV1(held);continue;}
    inferred[name]!++;records.set(name,[record(name+"/src")]);advance(INFER);
   }
   return{version:1,projects:Object.fromEntries(names.map(name=>[name,{root:name}])),dependencies:[],admission:{selection:{kind:"tasks",tasks:[{project:"owner-0",target:"build"}]}}};
  },
  buildGraphSourceFactsHoldV1:hold,
  mkdir:async()=>{},writeFile:async(path:string,text:string)=>{store.set(path,text);},rename:async(from:string,to:string)=>{store.set(to,store.get(from)!);store.delete(from);},rm:async(path:string)=>{store.delete(path);},
  createHash:()=>({update:()=>({digest:()=>"digest"})}),randomUUID:()=>"nonce",sealVerdictV1,reportSealV1,sliceReadStepV1:(_id:string,run:()=>unknown)=>run(),
  graphSnapshotPolicyV1:{maximumSnapshotBytes:1<<30},portableInputs:async(_root:string,value:unknown)=>value,readBuildGraphV1:(value:unknown)=>value,
  GRAPH_LIMITS_V1:limits.definitions.Limits.const,observeOwnerFactsMemoizedV1:undefined as unknown,evictOwnerFactsReferencingV1:()=>0,forgetOwnerFactsReferencingV1:()=>[],graphInventoryTextV1,graphRetryTextV1,graphSettleTextV1,graphDirtyTextV1,graphVolatileTextV1,membershipChangePathsV1,followedPathsV1,untoleratedFollowedPathsV1,settleInventoryRoundsV1,awaitPlannerQuietV1:async()=>{},ownerWithheldTextV1,
 };
 const owned=new Map();bindings.observeOwnerFactsMemoizedV1=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1","observeOwnerWithRetryV1","ownerFactsPagesV1",extract("rememberOwnerFactsV1")+";"+extract("forgetOwnerFactV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(owned,limits.definitions.Limits.const,observeOwnerWithRetryV1,pagesV1);const forget=Function("ownerFactsMemoV1","loadedGraphAdmissionFormatV1","ownerFactsPagesV1",extract("forgetOwnerFactV1")+";"+extract("forgetOwnerFactsReferencingV1")+";return forgetOwnerFactsReferencingV1;")(owned,"same",pagesV1) as()=>string[];bindings.forgetOwnerFactsReferencingV1=forget;bindings.evictOwnerFactsReferencingV1=((...args:[])=>forget(...args).length)as()=>number;
 const run=async()=>{const execute=Function(...Object.keys(bindings),extract("createGraphInventoryAwaitReportV1")+";"+extract("loadBuildGraphAttemptV1")+";return loadBuildGraphAttemptV1;")(...Object.values(bindings));return execute("/owned",{control:{signal:new AbortController().signal}},{},{selection:{kind:"tasks",tasks:[{project:"owner-0",target:"build"}]}},registry,"scope");};
 const receipt=()=>{const packed=JSON.parse(store.get("/owned/cache/selected/scope.authority.json")!);return {...packed,observations:decodeAuthorityObservationsV1(packed.observations,authoritySchema.definitions.ObservationKind.enum) as Row[],seal:packed.seal as ReturnType<typeof sealVerdictV1>};};
 return{run,tree,names,counts,inferred,observed,changes,lines,receipt,quiesce:()=>{churning=false;},owners,period,records,clock:()=>clock};
};

const staleRows=(state:ReturnType<typeof world>)=>state.receipt().observations.filter(row=>(state.tree.get(row.path)??0)!==row.fact.version);
const holderOf=(state:ReturnType<typeof world>,path:string)=>state.names.find(name=>path.startsWith(name+"/"))??"(workspace)";
const WORLDS=200;

test("a tree that changes during the read is sealed in bounded rounds, with work proportional to the changes (200 seeded worlds)",async()=>{
 let sealedLater=0,quiet=0;
 for(let seed=1;seed<=WORLDS;seed++){
  const state=world(seed),graph=await state.run();
  expect(graph,"world "+seed+" sealed").toBeDefined();
  expect(state.counts.discovered,"world "+seed+" rounds").toBeLessThanOrEqual(1+ROUNDS);
  expect(state.lines.filter(line=>line.startsWith("[graph-retry] ")),"world "+seed+" no restart").toHaveLength(0);
  const readers=(file:string)=>state.changes.filter(each=>each===file).length;
  for(const name of state.names){
   expect(state.inferred[name]!,"world "+seed+" inferences of "+name).toBeLessThanOrEqual(1+readers(name+"/src"));
   expect(state.observed[name]!,"world "+seed+" observations of "+name).toBeLessThanOrEqual(1+readers(name+"/manifest")+readers("tooling/shared.ts"));
  }
  const seal=state.receipt().seal,stale=staleRows(state);
  expect(seal.sealedWithLaterChanges,"world "+seed+" later changes").toBe(stale.length);
  expect(seal.laterChangedOwners,"world "+seed+" owners").toEqual([...new Set(stale.map(row=>holderOf(state,row.path)))].sort());
  expect(seal.mode===(stale.length===0?(state.counts.discovered===1?"quiet":"repaired"):"sealed-with-later-changes"),"world "+seed+" mode "+seal.mode).toBe(true);
  expect(Object.keys(seal.owners).length).toBeGreaterThan(0);
  if(stale.length>0)sealedLater++;else quiet++;
 }
 expect(sealedLater).toBeGreaterThan(20);expect(quiet).toBeGreaterThan(5);
},120000);

test("an owner whose file changed after its observation is stale in the next read only (200 seeded worlds)",async()=>{
 for(let seed=1;seed<=WORLDS;seed++){
  const state=world(seed);await state.run();
  const staleOwners=state.names.filter(name=>state.records.get(name)!.some(row=>(state.tree.get(row.path)??0)!==row.fact.version)),firstDiscoveries=state.counts.discovered,hadStale=staleRows(state).length>0,before={...state.inferred};
  state.quiesce();
  const second=await state.run();
  expect(second,"world "+seed+" second read").toBeDefined();
  const again=state.names.filter(name=>state.inferred[name]!>before[name]!);
  expect(again,"world "+seed+" only the stale owners are inferred again").toEqual(staleOwners);
  for(const name of again)expect(state.inferred[name]!-before[name]!).toBe(1);
  if(!hadStale)expect(state.counts.discovered,"world "+seed+" a valid seal is served as it is").toBe(firstDiscoveries);
  else{expect(state.counts.discovered).toBe(firstDiscoveries+1);const seal=state.receipt().seal;expect(seal.sealedWithLaterChanges).toBe(0);expect(seal.mode).toBe("quiet");expect(staleRows(state)).toHaveLength(0);}
 }
},120000);

test("the seal verdict and the receipt that carries it satisfy their schemas and name the digest of the rows each owner was observed with",async()=>{
 const ajv=new Ajv({strict:false,allErrors:true}),seal=ajv.compile({...retrySchema.$defs.Seal}),receipt=ajv.compile(authoritySchema);
 for(const seed of[3,17,42,99,150]){
  const state=world(seed);await state.run();
  const written=state.receipt(),rows=written.observations,groups=new Map<string,string[]>();
  expect(seal(written.seal),JSON.stringify(seal.errors)).toBe(true);
  for(const row of rows){const owner=holderOf(state,row.path);groups.set(owner,[...(groups.get(owner)??[]),JSON.stringify([row.kind,row.path,row.fact])]);}
  const independent=Object.fromEntries([...groups].sort(([left],[right])=>left<right?-1:1).map(([owner,list])=>[owner,createHash("sha256").update(list.sort().join("\n")).digest("hex")]));
  expect(written.seal.owners).toEqual(independent);
  const full={version:authoritySchema.properties.version.const,module:{implementation:"a".repeat(64),publication:"b".repeat(64)},membership:"c".repeat(64),graph:"d".repeat(64),seal:written.seal,observations:JSON.parse(encodeAuthorityObservationsV1(rows,1<<20))};
  expect(receipt(full),JSON.stringify(receipt.errors)).toBe(true);
  expect(receipt({...full,seal:{...written.seal,mode:"later"}})).toBe(false);
 }
});
