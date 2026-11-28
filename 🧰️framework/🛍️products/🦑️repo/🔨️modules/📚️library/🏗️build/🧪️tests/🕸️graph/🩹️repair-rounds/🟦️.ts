import{decodeAuthorityObservationsV1,distinctObservationRowsV1,encodeAuthorityObservationsV1}from"../../../👁️observation/🔢️row-table/🟨️.mjs";
import authoritySchema from"../../../🧬️schema/📥️authority.json";
import sliceReportSchema from "../../../🧠️memory/📋️slice-report/🧬️schema/🔣️.json" with {type:"json"};
import{test,expect}from"bun:test";
import{mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{createHash}from"node:crypto";
import pinnedFixture from"./🔣️pinned-source.json" with {type:"json"};
import{tmpdir}from"node:os";
import*as originalObservation from"../../../👁️observation/🟨️.mjs";
import{withImplementationPinV1}from"../../../📥️inputs/🔏️revision/🟨️.mjs";
import{fileURLToPath}from"node:url";
import{join,dirname}from"node:path";
import ts from"typescript";
import{graphInventoryTextV1,graphRetryTextV1,graphSettleTextV1,graphDirtyTextV1,graphVolatileTextV1,membershipChangePathsV1,followedPathsV1,untoleratedFollowedPathsV1}from"../../../🕸️graph/📥️load/🧲️membership-follow/🟦️.ts";
import{settleInventoryRoundsV1}from"../../../🕸️graph/📥️load/🔁️inventory-rounds/🟦️.ts";
import{observeOwnerWithRetryV1,ownerWithheldTextV1}from"../../../🕸️graph/📥️load/🛟️owner-isolation/🟦️.ts";
import{createPassReuseV1}from"../../../🕸️graph/📥️load/♻️pass-reuse/🟦️.ts";
import{createStringSharingV1}from"../../../🕸️graph/📥️load/🪢️string-sharing/🟦️.ts";
import{SLICE_RECYCLE_POLICY_V1,sliceRecycleEnvironmentV1,sliceRecycleOfV1}from"../../../🧠️memory/♻️recycle/🟨️.mjs";
const pagesV1={admit:()=>undefined,facts:(entry:{facts:unknown})=>entry.facts,release:()=>undefined};

const LOAD=process.env.R151_LOAD_SOURCE??fileURLToPath(new URL("../../../🕸️graph/📥️load/🟦️.ts",import.meta.url));
const limits=JSON.parse(readFileSync(process.env.R151_SCHEMA??fileURLToPath(new URL("../../../🕸️graph/🧬️schema/🔣️.json",import.meta.url)),"utf8")) as {definitions:{Limits:{const:Record<string,number>};LimitsDescriptions:{const:{en:Record<string,string>;de:Record<string,string>}}}};
const RETRY=process.env.R156_RETRY??new URL("../../../🕸️graph/🔁retry/🟨️.mjs",import.meta.url).href,{sealVerdictV1,reportSealV1}=await import(RETRY);
const source=readFileSync(LOAD,"utf8"),tree=ts.createSourceFile("load.ts",source,ts.ScriptTarget.Latest,true);

const extract=(name:string):string=>{
 const node=tree.statements.find((each):each is ts.FunctionDeclaration=>ts.isFunctionDeclaration(each)&&each.name?.text===name);if(!node)throw Error("Actual function required: "+name);
 const moved=ts.transform(node,[context=>{const visit:ts.Visitor=each=>ts.isPropertyAccessExpression(each)&&ts.isMetaProperty(each.expression)?ts.factory.createStringLiteral("file:///owned/graph.ts"):ts.isMetaProperty(each)?ts.factory.createIdentifier("importMetaV1"):ts.isCallExpression(each)&&each.expression.kind===ts.SyntaxKind.ImportKeyword?ts.factory.createCallExpression(ts.factory.createIdentifier("importModuleV1"),undefined,ts.visitNodes(each.arguments,visit,ts.isExpression)):ts.visitEachChild(each,visit,context);return each=>ts.visitNode(each,visit)as ts.FunctionDeclaration;}]);
 const text=ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,moved.transformed[0]!,tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;moved.dispose();return text;
};

type Row={kind:"file"|"digest";path:string;fact:{version?:number;sha256?:string;size?:number;mtimeMs?:number;ctimeMs?:number;ino?:number}};
const asRow=(row:originalObservation.SourceObservationV1):Row|undefined=>typeof row.fact==="object"&&"sha256" in row.fact?{kind:row.kind==="file"?"file":"digest",path:row.path,fact:row.fact}:undefined;

const world=(owners:number,scenario:{discoveryFailure?:(owner:string,call:number,tree:Map<string,number>)=>Error|undefined;membershipFailures?:(call:number,tree:Map<string,number>)=>string[]|undefined;onDiscover?:(call:number,tree:Map<string,number>)=>void;digestAt?:(call:number)=>string;admissionAt?:(call:number)=>string[];sharedInferenceRead?:boolean;settleRounds?:number;passReuse?:()=>unknown; farm?:{ready:()=>boolean;size:number;discover:(request:Record<string,unknown>,signal?:AbortSignal)=>Promise<unknown>}; pinned?:{rows:Row[];fail?:boolean;before?:()=>void}}={})=>{
 const tree=new Map<string,number>(),names=Array.from({length:owners},(_,index)=>"owner-"+index);
 for(const name of names)for(const file of["manifest","src"])tree.set(name+"/"+file,1);
 tree.set("tooling/shared.ts",1);tree.set("elsewhere/unrelated.ts",1);
 const lines:string[]=[],writes:string[]=[],stack:Map<string,Row>[]=[],observed:Record<string,number>={},counts={closes:0,discovered:0,holds:0,memoWrites:0,memberships:0,memoReads:0},memo=new Map<string,{facts:unknown;rows:readonly unknown[]}>();
 const record=(path:string)=>{const row:Row={kind:"file",path,fact:{version:tree.get(path)??0}};for(const each of stack)each.set(path,row);};
  const observation={
  observedFs:{readFileSync:(path:string|URL)=>{record(typeof path==="string"?path:path.pathname);return"";}},
  captureObservedSourceInputsV1:async(body:()=>Promise<unknown>,_failure?:unknown,authority=true)=>{const mine=new Map<string,Row>(),parent=stack.at(-1);stack.push(mine);try{const value=await body();return{value,observations:[...mine.values()],authorityObservations:authority?[]:[]};}catch(error){if(typeof _failure==="function")_failure([...mine.values()],[]);throw error;}finally{stack.pop();if(parent)for(const[path,row]of mine)parent.set(path,row);}},
  settleObservedComputationV1:async(body:()=>Promise<unknown>,options?:{onRound?:(round:{round:number})=>void})=>{for(let round=0;round<(scenario.settleRounds??0);round++){await body();options?.onRound?.({round});}return body();},
  staleSourceFactsErrorV1:(rows:readonly Row[])=>Object.assign(new Error("stale "+rows.map(row=>row.path).join(",")),{code:"BUILD_AUTHORITY_CHANGED"}),
  requirePinnedImplementationFactsV1:originalObservation.requirePinnedImplementationFactsV1,
  retainObservedRowsV1:(rows:readonly Row[])=>{for(const row of rows)for(const each of stack)each.set(row.path,row);},
  shareObservationRowsV1:<Item>(rows:Item[])=>rows,
 };
 const registry={version:1,owners:names.map(name=>({name}))},admitted=()=>({owners:names,files:[],targets:{},catalogProviders:[],inputs:{}});
 const producer={
  readBuildOwnerRegistrySourceV1:()=>JSON.stringify(registry),libraryBootstrap:Promise.resolve(),prepareSourceAnalysesV1:()=>{},
  ownerBoundaryFailureV1:()=>undefined,
  ownerFactsFarmV1:()=>scenario.farm,
  discoverBuildOwnerAdmissionFactsV1:async(owner:{name:string})=>{observed[owner.name]=(observed[owner.name]??0)+1;const failure=scenario.discoveryFailure?.(owner.name,observed[owner.name]!,tree);if(failure)throw failure;observation.observedFs.readFileSync(owner.name+"/manifest");observation.observedFs.readFileSync("tooling/shared.ts");return{project:{name:owner.name,targets:{}},files:[],dependencies:[]};},
 };
 const bindings={distinctObservationRowsV1,decodeAuthorityObservationsV1,encodeAuthorityObservationsV1,sourceAuthoritySchema:authoritySchema,sliceReportSchema,
  importMetaV1:{url:"file:///owned/graph.ts"},graphSelfUrlsV1:()=>[],physicalSourceOriginV1:(url:string)=>new URL(url),observationV1:observation,
  importModuleV1:async(specifier:string)=>specifier==="../📥️inference/🟨️.mjs"?producer:{},
  requireIssuedResourceControlInvocationV1:()=>{},checkResourceLeaseControl:()=>{},checkScriptInvocation:()=>{},
  process:{env:{},stderr:{write:(text:string)=>{lines.push(text);return true;}}},performance,JSON,Buffer,URL,join,dirname,
  buildGraphPathV1:()=>"/owned/cache/canonical.json",graphDeclarationV1:()=>({properties:{authority:{const:"authority.json"}}}),
  createRequire:()=>(()=>observation),readBuildOwnerRegistryV1:(value:unknown)=>value,buildGraphScopeKeyV1:()=>"scope",ownerRegistryDigestV1:()=>"registry",askOwnerRegistryV1:()=>"registry",
  ownerRegistryInvalidationV1:{kind:"invalidated",reason:"owner-registry"},isOwnerRegistryInvalidationV1:(value:unknown)=>value===undefined?false:(value as{kind?:string})?.kind==="invalidated",
  buildGraphErrorCodeV1:(error:unknown)=>(error as{code?:string})?.code,
  openOwnerVouchV1:async()=>undefined,vouchedOwnerV1:(_vouch:unknown,_owner:string,probe:(rows:readonly unknown[],clean:undefined)=>Promise<boolean>)=>({hold:(rows:readonly unknown[])=>probe(rows,undefined),outcome:()=>undefined}),
  readForgottenPathsV1:()=>({paths:new Map(),listings:new Map()}),rowsForgottenV1:()=>false,forgetPersistedPathsV1:async()=>true,readAdmissionMemoV1:async()=>{counts.memoReads++;return undefined;},writeAdmissionMemoV1:async()=>{counts.memoWrites++;},
  buildGraphProducerAuthorityV1:()=>true,loadedGraphModuleAuthorityV1:{implementation:"same",publication:"same"},loadedGraphAdmissionFormatV1:"same",selectedAdmissionFactsV1:{entries:()=>[],suffixes:()=>[],seed:()=>{}},packAdmissionMemoV1:(value:unknown)=>value,ownersOfAdmissionMemoV1:()=>[],
  closeBuildOwnerAdmissionV1:async(_selection:unknown,_registry:unknown,inspect:(owner:{name:string},targets:string[],inputs:string[])=>Promise<unknown>)=>{counts.closes++;for(const name of names)await inspect({name},[],[]);return admitted();},
  buildGraphMembershipV1:async(_root:string,_files:unknown,_invocation:unknown,_volatile:unknown,vanished?:(path:string)=>void)=>{const failing=scenario.membershipFailures?.(++counts.memberships,tree);if(failing&&vanished&&failing.length>0)for(const path of failing)vanished(path.replace("/owned/",""));else if(failing)throw Object.assign(new Error("Build membership changed during discovery: "+failing.join(", ")),{code:"BUILD_MEMBERSHIP_CHANGED",paths:failing});return scenario.digestAt?.(counts.discovered)??"membership";},buildGraphMembersV1:(files:string[])=>files,workspaceBuildFilesV1:async()=>[],inventoryDirectoriesV1:new WeakMap(),
  readFile:async()=>{throw Object.assign(Error("absent"),{code:"ENOENT"});},
  createStringSharingV1,memoryReleaseV1:{between:()=>undefined},createPassReuseV1:scenario.passReuse??(()=>({memo:undefined,carried:{recycles:0,round:0,passes:0,owners:0,heapMegabytes:0},boundary:()=>{}})),discoverBuildGraphV1:async()=>{counts.discovered++;for(const name of names)observation.observedFs.readFileSync(name+"/src");if(scenario.sharedInferenceRead)observation.observedFs.readFileSync("tooling/shared.ts");scenario.onDiscover?.(counts.discovered,tree);if(scenario.pinned){scenario.pinned.before?.();observation.retainObservedRowsV1(scenario.pinned.rows);if(scenario.pinned.fail)throw Error("failed inference kept its original observations");}return{version:1,projects:Object.fromEntries(names.map(name=>[name,{root:name}])),dependencies:[]};},
  buildGraphSourceFactsHoldV1:async(rows:readonly Row[],_invocation?:unknown,onStale?:(row:Row)=>void)=>{counts.holds++;const stale=rows.filter(row=>row.fact.version!==undefined&&(tree.get(row.path)??0)!==row.fact.version);for(const row of stale)onStale?.(row);return stale.length===0&&await originalObservation.observationsHold(rows.filter(row=>row.fact.sha256!==undefined),64,new Set(),new Set(),undefined,onStale===undefined?undefined:(row:originalObservation.SourceObservationV1)=>{const mapped=asRow(row);if(mapped!==undefined)onStale(mapped);});},
  mkdir:async()=>{},writeFile:async(_path:string,text:string)=>{writes.push(text);},sealVerdictV1,reportSealV1,sliceReadStepV1:(_id:string,run:()=>unknown)=>run(),rename:async()=>{},rm:async()=>{},createHash:()=>({update:()=>({digest:()=>"digest"})}),randomUUID:()=>"nonce",
  graphSnapshotPolicyV1:{maximumSnapshotBytes:1<<30},portableInputs:async(_root:string,value:unknown)=>value,readBuildGraphV1:(value:unknown)=>value,
  GRAPH_LIMITS_V1:limits.definitions.Limits.const,observeOwnerFactsMemoizedV1:undefined as unknown,evictOwnerFactsReferencingV1:undefined as unknown,forgetOwnerFactsReferencingV1:undefined as unknown,graphInventoryTextV1,graphRetryTextV1,graphSettleTextV1,graphDirtyTextV1,graphVolatileTextV1,membershipChangePathsV1,followedPathsV1,untoleratedFollowedPathsV1,settleInventoryRoundsV1,awaitPlannerQuietV1:async()=>{},ownerWithheldTextV1,
 };
 if(source.includes("function observeOwnerFactsMemoizedV1"))bindings.observeOwnerFactsMemoizedV1=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1","observeOwnerWithRetryV1","ownerFactsPagesV1",extract("rememberOwnerFactsV1")+";"+extract("forgetOwnerFactV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(memo,limits.definitions.Limits.const,observeOwnerWithRetryV1,pagesV1);
 if(source.includes("function forgetOwnerFactsReferencingV1")){const forget=Function("ownerFactsMemoV1","loadedGraphAdmissionFormatV1","ownerFactsPagesV1",extract("forgetOwnerFactV1")+";"+extract("forgetOwnerFactsReferencingV1")+";return forgetOwnerFactsReferencingV1;")(memo,"same",pagesV1) as(scope:string,paths:readonly string[])=>string[],local=(paths:readonly string[])=>paths.map(path=>path.replace("/owned/",""));bindings.forgetOwnerFactsReferencingV1=(scope:string,paths:readonly string[])=>forget(scope,local(paths));bindings.evictOwnerFactsReferencingV1=(scope:string,paths:readonly string[])=>forget(scope,local(paths)).length;}
 const run=async()=>{const execute=Function(...Object.keys(bindings),extract("createGraphInventoryAwaitReportV1")+";"+extract("loadBuildGraphAttemptV1")+";return loadBuildGraphAttemptV1;")(...Object.values(bindings));return execute("/owned",{control:{signal:new AbortController().signal}},{},{selection:{kind:"tasks",tasks:[{project:"owner-0",target:"build"}]}},registry,"scope");};
 const seal=()=>JSON.parse(writes.at(-1)!).seal as{mode:string;rounds:number;sealedWithLaterChanges:number;laterChangedOwners:string[];membershipMoved:boolean;admissionMoved:boolean;owners:Record<string,string>};
 return{run,tree,observed,counts,names,lines,writes,seal};
};

const observedOnce=(state:ReturnType<typeof world>)=>state.names.every(name=>state.observed[name]===1);

for(const fail of [false,true]){
 test("a proven pinned implementation mismatch stops before rebuilding inventory after "+(fail?"failed":"successful")+" inference",async()=>{
  const directory=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"graph-pinned-repair-"));
  const path=join(directory,"implementation.mjs"),sha=(text:string)=>createHash("sha256").update(text).digest("hex");
  const {pinned:loaded,current:live,changedAgain}=pinnedFixture.values;
  expect(sha(loaded)).toBe(pinnedFixture.sha256.pinned);
  expect(sha(live)).toBe(pinnedFixture.sha256.current);
  const row=async()=>(await originalObservation.captureObservedSourceInputsV1(()=>originalObservation.observedFs.readFileSync(path))).observations.find(row=>row.path===path&&row.kind==="file") as Row;
  try{
   writeFileSync(path,loaded);
   const pinned=await row();
   if(pinned.fact.sha256===undefined)throw Error("Actual file row with sha256 required");
   originalObservation.registerLoadedImplementationRowsV1([{...pinned,kind:"digest",fact:{...pinned.fact,sha256:pinned.fact.sha256}}]);
   writeFileSync(path,live);
   const current=await row();
   const state=world(6,{pinned:{rows:[current],fail}});
   await expect(withImplementationPinV1(state.run)).rejects.toMatchObject({
    code:"BUILD_AUTHORITY_CHANGED",
    pinnedDrift:{path,expectedSha256:sha(live),currentSha256:sha(live),pinnedSha256:sha(loaded)},
   });
   expect(state.counts).toMatchObject({closes:1,discovered:1,memoWrites:1});
   expect(state.writes).toEqual([]);
   const compatible=world(6,{pinned:{rows:[pinned]}});
   expect(await withImplementationPinV1(compatible.run)).toBeDefined();
   expect(compatible.counts.closes).toBe(2);
   expect(compatible.seal().mode).toBe("quiet");
   const unpinned=world(6,{pinned:{rows:[current]}});
   expect(await unpinned.run()).toBeDefined();
   expect(unpinned.seal().mode).toBe("quiet");

   writeFileSync(path,changedAgain);
   const rows=[current];
   let attempts=0;
   const ordinaryRepair=world(6,{pinned:{rows,before:()=>{if(++attempts===2)rows[0]=pinned;}}});
   expect(await withImplementationPinV1(ordinaryRepair.run)).toBeDefined();
   expect(ordinaryRepair.counts.closes).toBe(3);
   expect(ordinaryRepair.seal().mode).toBe("repaired");
  }finally{rmSync(directory,{recursive:true,force:true});}
 });
}


test("a fact consumed during both admission and inference is published once",async()=>{
 const state=world(4,{sharedInferenceRead:true});
 await state.run();
 const receipt=JSON.parse(state.writes.at(-1)!) as {observations:Row[]};
 expect(decodeAuthorityObservationsV1(receipt.observations,authoritySchema.definitions.ObservationKind.enum).filter(row=>row.path==="tooling/shared.ts")).toEqual([{kind:"file",path:"tooling/shared.ts",fact:{version:1}}]);
 expect(state.seal().mode).toBe("quiet");
});

test("an edit of a file nobody read changes nothing",async()=>{
 const state=world(6,{onDiscover:(_call,tree)=>{tree.set("elsewhere/unrelated.ts",2);}});
 const graph=await state.run();
 expect(graph).toBeDefined();expect(state.counts.discovered).toBe(1);expect(observedOnce(state)).toBe(true);
});

test("an edit of a source one owner's inference read repeats the inference alone and observes no owner again",async()=>{
 const state=world(6,{onDiscover:(call,tree)=>{if(call===1)tree.set("owner-3/src",2);}});
 const graph=await state.run();
 expect(graph).toBeDefined();expect((graph as{kind?:string}).kind).not.toBe("invalidated");
 expect(state.counts.discovered).toBe(2);
 expect(observedOnce(state)).toBe(true);
});

test("every repair round and the seal are said with the sources that caused them",async()=>{
 const repaired=world(4,{onDiscover:(call,tree)=>{if(call===1)tree.set("owner-2/src",2);}});
 await repaired.run();
 const repair=repaired.lines.filter(line=>line.startsWith("[graph-repair] "));expect(repair).toHaveLength(1);expect(JSON.parse(repair[0]!.slice(15)).stale).toEqual(["owner-2/src"]);
 expect(repaired.seal().mode).toBe("repaired");expect(repaired.seal().sealedWithLaterChanges).toBe(0);
 const endless=world(4,{onDiscover:(_call,tree)=>{tree.set("owner-1/src",(tree.get("owner-1/src")??0)+1);}});
 expect(await endless.run()).toBeDefined();
 expect(endless.lines.filter(line=>line.startsWith("[graph-retry] "))).toHaveLength(0);
 const sealed=endless.lines.filter(line=>line.startsWith("[graph-seal] "));expect(sealed).toHaveLength(1);
 expect(JSON.parse(sealed[0]!.slice(13))).toMatchObject({version:1,mode:"sealed-with-later-changes",rounds:limits.definitions.Limits.const.repairRounds,sealedWithLaterChanges:1,laterChangedOwners:["owner-1"]});
});

test("an edit of a file one owner's admission read observes that owner again and no other",async()=>{
 const state=world(6,{onDiscover:(call,tree)=>{if(call===1)tree.set("owner-2/manifest",2);}});
 const graph=await state.run();
 expect(graph).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([1,1,2,1,1,1]);
 expect(state.counts.discovered).toBe(1);
});

test("an edit of a tooling file every owner's admission read is paid once per owner and not per attempt",async()=>{
 const state=world(6,{onDiscover:(call,tree)=>{if(call===1)tree.set("tooling/shared.ts",2);}});
 const graph=await state.run();
 expect(graph).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([2,2,2,2,2,2]);
 expect(state.counts.discovered).toBe(1);
});

test("repair rounds are bounded and the read then seals with the records it has",async()=>{
 const state=world(4,{onDiscover:(_call,tree)=>{tree.set("owner-1/src",(tree.get("owner-1/src")??0)+1);}});
 const result=await state.run();
 expect(result).toBeDefined();
 expect(state.counts.discovered).toBe(1+limits.definitions.Limits.const.repairRounds);
 expect(state.seal().sealedWithLaterChanges).toBe(1);
 expect(observedOnce(state)).toBe(true);
});

test("a changed membership is followed in place, the owners that stay valid are not observed again",async()=>{
 const state=world(4,{onDiscover:(call,tree)=>{if(call===1)tree.set("owner-1/src",2);},digestAt:call=>call>=1?"other":"membership"});
 const result=await state.run();
 expect(result).toBeDefined();expect(state.counts.discovered).toBe(2);expect(observedOnce(state)).toBe(true);
 expect(state.seal()).toMatchObject({mode:"repaired",membershipMoved:false,sealedWithLaterChanges:0});
 expect(JSON.parse(state.writes.at(-1)!).membership).toBe("other");
});

test("a membership that moves at every round is sealed with the count of the moved membership",async()=>{
 const state=world(4,{digestAt:call=>"m"+call+"x"});
 expect(await state.run()).toBeDefined();expect(state.seal()).toMatchObject({mode:"sealed-with-later-changes",membershipMoved:true});
 expect(state.counts.discovered).toBe(1+limits.definitions.Limits.const.repairRounds);
});

test("the owner facts memo keeps the newest entries within its limit",()=>{
 const text=extract("rememberOwnerFactsV1"),memo=new Map<string,unknown>(),run=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1","ownerFactsPagesV1",extract("forgetOwnerFactV1")+";"+text+";return rememberOwnerFactsV1;")(memo,{ownerFactsMemoEntries:3},pagesV1);
 for(const key of["a","b","c"])run(key,{facts:key,rows:[]});run("a",{facts:"a2",rows:[]});run("d",{facts:"d",rows:[]});
 expect([...memo.keys()]).toEqual(["c","a","d"]);
});

test("the limits carry the repair numbers in both languages",()=>{
 const schema=limits.definitions;
 for(const key of["repairRounds","ownerFactsMemoEntries","sealStalePaths"]){expect(typeof schema.Limits.const[key]).toBe("number");expect(schema.LimitsDescriptions.const.en[key].length).toBeGreaterThan(20);expect(schema.LimitsDescriptions.const.de[key].length).toBeGreaterThan(20);}
});

test("the slice report of a worker counts how often its process entered the owner loop",()=>{
 const text=readFileSync(process.env.R151_NODES??fileURLToPath(new URL("../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs",import.meta.url)),"utf8");
 const declared=text.indexOf("let ownerPassesV1=0;"),entered=text.indexOf("ownerPassesV1++;"),reported=text.indexOf("pass:ownerPassesV1,");
 expect(declared).toBeGreaterThan(-1);expect(entered).toBeGreaterThan(declared);expect(reported).toBeGreaterThan(entered);
 expect(text.split("ownerPassesV1++;").length).toBe(2);
});


const retryLines=(state:ReturnType<typeof world>)=>state.lines.filter(line=>line.startsWith("[graph-retry] ")).map(line=>JSON.parse(line.slice(14)));
const inventoryLines=(state:ReturnType<typeof world>)=>state.lines.filter(line=>line.startsWith("[graph-inventory] ")).map(line=>JSON.parse(line.slice(18)));

test("a membership change in the tail follows in place: only the owner that read the changed member is observed again",async()=>{
 const state=world(6,{membershipFailures:(call,tree)=>{if(call!==1)return undefined;tree.set("owner-2/manifest",2);return["/owned/owner-2/manifest"];}});
 expect(await state.run()).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([1,1,2,1,1,1]);
 expect(state.counts).toMatchObject({closes:3,memberships:3,memoReads:2,memoWrites:2});
 const [retry,...more]=retryLines(state);expect(more).toEqual([]);
 expect(retry).toMatchObject({version:1,round:1,first:["owner-2/manifest"],paths:1,dirtyOwners:["owner-2"],dirtyCount:1,streak:1,memo:0,checkpoint:0,discovery:6});
 expect(retry.text.en.length).toBeGreaterThan(20);expect(retry.text.de.length).toBeGreaterThan(20);expect(retry.text.en).not.toBe(retry.text.de);
 expect(inventoryLines(state)).toMatchObject([{version:1,round:2,memo:5,checkpoint:0,discovery:1}]);
 expect(state.seal().mode).toBe("quiet");
});

test("a changed member whose rows still hold is forgotten by name and no other owner is observed again",async()=>{
 const state=world(6,{membershipFailures:call=>call===1?["/owned/owner-4/manifest"]:undefined});
 expect(await state.run()).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([1,1,1,1,2,1]);
 expect(retryLines(state)[0]).toMatchObject({dirtyOwners:["owner-4"],dirtyCount:1});
});

test("a tree that keeps changing is followed with a fresh path each round until the inventory seals it lenient with the moved paths named volatile",async()=>{
 const rounds=limits.definitions.Limits.const.repairRounds*3;
 const state=world(4,{membershipFailures:(call,tree)=>{if(call>rounds)return undefined;const owner="owner-"+call%4;tree.set(owner+"/file"+call,call+1);return["/owned/"+owner+"/file"+call];}});
 expect(await state.run()).toBeDefined();
 const bound=limits.definitions.Limits.const.inventoryRounds*2,rest=rounds-bound;
 expect(state.counts.memberships).toBe(rounds+1);expect(state.counts.closes).toBe(rounds+1);
 expect(retryLines(state)).toHaveLength(rounds-1);
 expect(inventoryLines(state).map(line=>line.round)).toEqual([bound+1,rest]);
 const volatile=state.lines.filter(line=>line.startsWith("[graph-volatile] ")).map(line=>JSON.parse(line.slice(17)));
 expect(volatile).toMatchObject([{version:1,rounds:bound+1}]);expect(volatile[0].pathCount).toBeGreaterThan(0);
 expect(state.counts.memoReads).toBe(2);expect(state.counts.memoWrites).toBe(2);
 expect(Object.values(state.observed).reduce((sum,count)=>sum+count,0)).toBe(4);
});

test("the same members refused again after they were forgotten and observed again are a standing fault, not churn",async()=>{
 const state=world(4,{membershipFailures:()=>["/owned/owner-1/manifest"]});
 await expect(state.run()).rejects.toMatchObject({code:"BUILD_MEMBERSHIP_CHANGED",paths:["/owned/owner-1/manifest"]});
 expect(state.counts.memberships).toBe(limits.definitions.Limits.const.inventoryVolatileRounds+1);expect(state.writes).toEqual([]);
});

test("a refusal naming no member keeps the bounded repeat and fails when it stays",async()=>{
 const state=world(4,{membershipFailures:()=>[]});
 await expect(state.run()).rejects.toMatchObject({code:"BUILD_MEMBERSHIP_CHANGED"});
 expect(state.counts.memberships).toBeLessThanOrEqual(limits.definitions.Limits.const.repairRounds+1);
});


const vanished=(path:string)=>Object.assign(new Error("ENOENT: no such file or directory, scandir '"+path+"'"),{code:"ENOENT",syscall:"scandir",errno:-2,path});

test("a directory that vanishes inside one owner follows in place: only that owner is observed again and nothing restarts",async()=>{
 const state=world(6,{discoveryFailure:(owner,call)=>owner==="owner-3"&&call===1?vanished("/owned/owner-3/canvas/corners"):undefined});
 expect(await state.run()).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([1,1,1,2,1,1]);
 expect(state.counts).toMatchObject({closes:3,memoReads:2,memoWrites:1});
 const [retry,...more]=retryLines(state);expect(more).toEqual([]);
 expect(retry).toMatchObject({version:1,round:1,first:["owner-3/canvas/corners"],paths:1,dirtyCount:0,streak:1,memo:0,discovery:3});
 expect(retry.text.en).not.toBe(retry.text.de);
 expect(inventoryLines(state)).toMatchObject([{round:2,memo:3,discovery:3}]);
 expect(state.seal().mode).toBe("quiet");
});

test("a vanished directory that stays gone is a standing fault: refused after the volatile rounds with the owner failure that names the original error",async()=>{
 const raised=vanished("/owned/owner-1/canvas/corners"),state=world(4,{discoveryFailure:owner=>owner==="owner-1"?raised:undefined});
 await expect(state.run()).rejects.toMatchObject({code:"BUILD_OWNER_FAILED",owners:[{name:"owner-1"}],stable:true,cause:raised});
 expect(state.observed["owner-1"]).toBe(limits.definitions.Limits.const.inventoryVolatileRounds+1);expect(state.observed["owner-0"]).toBe(1);expect(state.writes).toEqual([]);
});

test("an error that names no path of the tree is not followed",async()=>{
 const outside=vanished("/elsewhere/corners"),plain=new Error("owner broke"),state=world(4,{discoveryFailure:owner=>owner==="owner-1"?outside:undefined});
 await expect(state.run()).rejects.toMatchObject({code:"BUILD_OWNER_FAILED",owners:[{name:"owner-1"}],stable:true,cause:outside});expect(state.observed["owner-1"]).toBe(1);
 const second=world(4,{discoveryFailure:owner=>owner==="owner-1"?plain:undefined});
 await expect(second.run()).rejects.toMatchObject({code:"BUILD_OWNER_FAILED",owners:[{name:"owner-1"}],stable:true,cause:plain});expect(second.observed["owner-1"]).toBe(1);
});

const heldOwners=(state:ReturnType<typeof world>)=>state.counts.holds;

test("a membership follow-up verifies no owner it already verified: only the forgotten owner is observed and checked again",async()=>{
 const quiet=world(6,{});await quiet.run();
 const state=world(6,{membershipFailures:(call,tree)=>{if(call!==1)return undefined;tree.set("owner-2/manifest",2);return["/owned/owner-2/manifest"];}});
 expect(await state.run()).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([1,1,2,1,1,1]);
 expect(heldOwners(state)).toBe(heldOwners(quiet));
 expect(inventoryLines(state)).toMatchObject([{round:2,memo:5,discovery:1}]);
 expect(state.seal().mode).toBe("quiet");
});

test("a change to an owner the follow-up trusted is still found by the next inventory and sealed as one consistent snapshot",async()=>{
 const state=world(6,{membershipFailures:(call,tree)=>{if(call!==1)return undefined;tree.set("owner-3/manifest",2);return["/owned/owner-2/manifest"];}});
 expect(await state.run()).toBeDefined();
 expect(state.names.map(name=>state.observed[name])).toEqual([1,1,2,2,1,1]);
 expect(state.seal()).toMatchObject({sealedWithLaterChanges:0,membershipMoved:false,admissionMoved:false});
 const receipt=JSON.parse(state.writes.at(-1)!) as {observations:Row[]};
 const rows=decodeAuthorityObservationsV1(receipt.observations,authoritySchema.definitions.ObservationKind.enum) as Row[];
 expect(rows.filter(row=>row.path==="owner-3/manifest").map(row=>row.fact.version)).toEqual([2]);
 expect(rows.filter(row=>row.path==="owner-1/manifest").map(row=>row.fact.version)).toEqual([1]);
});

// Two settled inventory passes run in this world (the first load and the post-discovery re-check); each settle round that repeated the whole admission resets trust, so each pass re-holds all four owners: 2 x 4 = 8 extra holds.
test("a settle round that repeated the whole admission in each of the two settled inventory passes forgets the trust and verifies every owner again",async()=>{
 const quiet=world(4,{});await quiet.run();
 const state=world(4,{settleRounds:1});
 expect(await state.run()).toBeDefined();
 expect(heldOwners(state)-heldOwners(quiet)).toBe(8);
});

test("an owner memo entry the caller trusts is not verified again and every answered entry becomes trusted",async()=>{
 const memo=new Map<string,{facts:unknown;rows:readonly unknown[]}>(),trusted=new WeakSet<object>();
 const run=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1","observeOwnerWithRetryV1","ownerFactsPagesV1",extract("rememberOwnerFactsV1")+";"+extract("forgetOwnerFactV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(memo,limits.definitions.Limits.const,observeOwnerWithRetryV1,pagesV1) as (input:Record<string,unknown>)=>Promise<unknown>;
 let holds=0,discovered=0;const outcomes:string[]=[];
 const ask=(key:string,holding=true)=>run({key,hold:async()=>{holds++;return holding;},observation:{captureObservedSourceInputsV1:async(body:()=>Promise<unknown>)=>({value:await body(),observations:[{kind:"file",path:key,fact:{version:1}}]}),retainObservedRowsV1:()=>{}},observe:async()=>{discovered++;return key;},outcome:(kind:string)=>outcomes.push(kind),trust:{has:(entry:object)=>trusted.has(entry),add:(entry:object)=>trusted.add(entry)}});
 expect(await ask("a")).toBe("a");expect([holds,discovered]).toEqual([0,1]);
 expect(await ask("a",false)).toBe("a");expect([holds,discovered]).toEqual([0,1]);
 trusted.delete(memo.get("a")!);
 expect(await ask("a",false)).toBe("a");expect([holds,discovered]).toEqual([1,2]);
 expect(await ask("a")).toBe("a");expect([holds,discovered]).toEqual([1,2]);
 expect(outcomes).toEqual(["discovery","memo","discovery","memo"]);
});


/** ♻️ Recycling at a pass boundary: a worker whose live heap is above the declared threshold hands the read over between two passes; the fresh worker continues from the carried counters and the read ends as an uninterrupted one does. */
const recycledRead=async(options:{heapMegabytes:number;edits?:boolean})=>{
 const lines:string[]=[],states:unknown[]=[];let environment:Record<string,string>={...sliceRecycleEnvironmentV1(undefined)};
 const reuse=()=>createPassReuseV1({environment,heapBytes:()=>options.heapMegabytes*1048576,say:line=>{lines.push(line);}});
 const state=world(4,{passReuse:reuse,onDiscover:(_call,tree)=>{tree.set("owner-1/src",(tree.get("owner-1/src")??0)+1);}});
 let graph:unknown,workers=0;
 for(;;){
  workers++;
  try{graph=await state.run();break;}catch(error){
   const carried=sliceRecycleOfV1(error);if(carried===undefined)throw error;
   states.push(carried);environment={...sliceRecycleEnvironmentV1(carried as never)};
  }
 }
 return{state,graph,states,workers,lines};
};

test("a worker above the heap threshold hands the read over at each pass boundary until the declared maximum, and the read ends as an uninterrupted one",async()=>{
 const plain=world(4,{onDiscover:(_call,tree)=>{tree.set("owner-1/src",(tree.get("owner-1/src")??0)+1);}});
 const plainGraph=await plain.run(),recycled=await recycledRead({heapMegabytes:SLICE_RECYCLE_POLICY_V1.heapMegabytes+1});
 expect(plainGraph).toBeDefined();expect(recycled.graph).toEqual(plainGraph);
 expect(recycled.states).toEqual([1,2,3].map(recycles=>({recycles,round:recycles,passes:recycles,owners:0,heapMegabytes:SLICE_RECYCLE_POLICY_V1.heapMegabytes+1})));
 expect(recycled.workers).toBe(SLICE_RECYCLE_POLICY_V1.maximumRecycles+1);
 expect(recycled.state.counts.discovered).toBe(plain.counts.discovered);
 expect(recycled.state.counts.discovered).toBe(1+limits.definitions.Limits.const.repairRounds);
 expect(recycled.state.seal()).toEqual(plain.seal());
 const said=recycled.lines.filter(line=>line.startsWith("[graph-reuse] ")).map(line=>JSON.parse(line.slice(14)));
 expect(said.length).toBeGreaterThanOrEqual(recycled.states.length);expect(said.every(row=>row.text.en.includes("Pass ")&&row.text.de.includes("Durchgang "))).toBe(true);
});

test("a worker below the threshold, or without one, never recycles",async()=>{
 const below=await recycledRead({heapMegabytes:SLICE_RECYCLE_POLICY_V1.heapMegabytes-1});
 expect(below.workers).toBe(1);expect(below.states).toEqual([]);expect(below.state.counts.discovered).toBe(1+limits.definitions.Limits.const.repairRounds);
 const lines:string[]=[],state=world(4,{passReuse:()=>createPassReuseV1({environment:{},heapBytes:()=>1e15,say:line=>{lines.push(line);}}),onDiscover:(_call,tree)=>{tree.set("owner-1/src",(tree.get("owner-1/src")??0)+1);}});
 expect(await state.run()).toBeDefined();expect(state.counts.discovered).toBe(1+limits.definitions.Limits.const.repairRounds);
});

const farmed=(failing:boolean)=>{
 const requests:Record<string,unknown>[]=[],farm={size:3,ready:()=>true,discover:async(request:Record<string,unknown>)=>{requests.push(request);if(failing)throw Object.assign(Error("child refused"),{childCode:"OWNER_FARM_FORMAT"});return{facts:{project:{name:request.owner,targets:{}},files:[],dependencies:[]},rows:[{kind:"file",path:request.owner+"/manifest",fact:{version:1}}]};}};
 return{farm,requests};
};

test("a ready discovery farm answers the owner facts and the parent discovers no owner itself",async()=>{
 const {farm,requests}=farmed(false),state=world(4,{farm});
 await state.run();
 expect(requests.map(request=>request.owner)).toEqual(state.names);
 expect(requests.every(request=>request.format==="same"&&request.registryDigest==="registry"&&typeof request.epoch==="string")).toBe(true);
 expect(new Set(requests.map(request=>request.epoch)).size).toBe(1);
 expect(state.observed).toEqual({});
});

test("a farm that refuses an owner leaves it to the parent, which discovers it as without a farm, and says so",async()=>{
 const {farm,requests}=farmed(true),state=world(4,{farm});
 await state.run();
 expect(requests).toHaveLength(4);
 expect(observedOnce(state)).toBe(true);
 expect(state.lines.filter(line=>line.startsWith("[graph-owner-farm]"))).toHaveLength(3);
});
