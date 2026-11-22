import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv/dist/2020";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {closeBuildOwnerAdmissionV1,createSourceOwnerResolverV1,readBuildGraphSelectionV1,type BuildOwnerRegistryV1} from "../../../🚪️admission/🟦️.ts";
import {observeOwnerFactsMemoizedV1,selectedAdmissionFactsV1} from "../../🟦️.ts";
import {loadedGraphAdmissionFormatV1} from "../../../🔐️authority/🟦️.ts";
import {canonicalTestAggregatePolicyV1 as aggregate,deriveCanonicalTestAggregateV1,readCanonicalTestRoutesV1,requireCanonicalTestAggregateV1} from "../../../../../⚡️caching/🗂️suites/🧮️aggregate/🟦️.ts";
import {isOwnerFailureV1,joinOwnerFailuresV1,observeOwnerWithRetryV1,ownerFailureDecisionV1,ownerFailureV1,ownerIsolationPolicyV1,registryPendingFailureV1,stableOwnerFailureHoldsV1} from "../🟦️.ts";
import {isClosedOwnerRegistryInvalidationV1,registryFaultTextV1,withOwnerRegistryCustodyV1} from "../../🧭️registry-follow/🟦️.ts";
import {revalidatedWithheldOwnersV1,withheldRevalidationV1} from "../🔁️revalidation/🟨️.mjs";

const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const placeholders=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
const workspace=(()=>{for(let directory=import.meta.dirname;directory!==dirname(directory);directory=dirname(directory))if(existsSync(join(directory,aggregate.manifest)))return directory;throw Error("workspace not found");})();

interface Attempt {fails:boolean;rowsHold?:boolean;young?:boolean;foreign?:string}
const run=async(attempts:readonly Attempt[])=>{
 let index=0;const pauses:number[]=[];
 const clock=10_000,outcome=await observeOwnerWithRetryV1({
  owner:"o",racyMilliseconds:fixture.racyMilliseconds,now:()=>clock,pause:async milliseconds=>{pauses.push(milliseconds);},
  hold:async()=>attempts[index-1]!.rowsHold===true,
  attempt:async onFailure=>{
   const script=attempts[index++];if(script===undefined)throw Error("scripted attempts ended");
   if(!script.fails)return "value";
   onFailure([{kind:"file",path:"/w/a",fact:{mtimeMs:script.young?clock-1:0,ctimeMs:0}}]);
   throw Object.assign(Error("boom"),script.foreign?{code:script.foreign}:{});
  },
 }).then(()=>({outcome:"value",attempts:index}),(error:unknown)=>({outcome:isOwnerFailureV1(error)?(error.stable?"stable":"unsettled"):"foreign",attempts:index}));
 return {...outcome,pauses};
};

test("schema: the policy, the failure shape and both languages of every text agree",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/Failure"});
 expect(validate({code:"BUILD_OWNER_FAILED",owners:[{name:"a",code:"Error",reason:"x"}],rounds:1,stable:true,message:"x"})).toBe(true);
 expect(validate({code:"BUILD_OWNER_FAILED",owners:[],rounds:1,stable:true,message:"x"})).toBe(false);
 expect(validate({code:"BUILD_OWNER_FAILED",owners:["a"],rounds:1,stable:true,message:"x"})).toBe(false);
 expect(validate({code:"BUILD_OWNER_FAILED",owners:[{name:"a",code:"Error",reason:"x",causedBy:"b",extra:1}],rounds:1,stable:true,message:"x"})).toBe(false);
 expect(validate({code:"BUILD_OWNER_FAILED",owners:[{name:"a",code:"Error",reason:"x"}],rounds:1,stable:true,message:"x",extra:1})).toBe(false);
 for(const [key,text] of Object.entries(schema.$defs.Texts.const)){expect(placeholders(text.en),key).toEqual(placeholders(text.de));expect(text.en.length>0&&text.de.length>0).toBe(true);}
 expect(Object.keys(schema.$defs.PolicyDescriptions.const.en).sort()).toEqual(Object.keys(schema.$defs.PolicyDescriptions.const.de).sort());
 expect(Object.keys(schema.$defs.PolicyDescriptions.const.en).sort()).toEqual(Object.keys(ownerIsolationPolicyV1).filter(key=>!["version","code"].includes(key)).sort());
 expect(fixture.retryRounds).toBe(ownerIsolationPolicyV1.retryRounds);
});

const pythonOracle=()=>{
 const python=spawnSync(process.platform==="win32"?"python":"python3",["-I",fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url))],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 return JSON.parse(python.stdout) as {retry:unknown;decisions:Record<string,string>;revalidation:Record<string,unknown>};
};

test("retry decisions equal the hand-stated corpus and an independent Python reading of it",async()=>{
 const oracle=pythonOracle().retry as Record<string,{outcome:string;attempts:number}>;
 for(const testCase of fixture.cases){
  const seen=await run(testCase.attempts as Attempt[]);
  expect({outcome:seen.outcome,attempts:seen.attempts},testCase.name).toEqual(testCase.expect);
  expect(oracle[testCase.name],testCase.name).toEqual(testCase.expect);
 }
});

test("a pause grows with the round, never past the declared ceiling, and waits out young rows",async()=>{
 const seen=await run(fixture.cases[5]!.attempts as Attempt[]),policy=ownerIsolationPolicyV1;
 expect(seen.pauses).toEqual([1,2,3].map(round=>round*policy.settlePauseMilliseconds));
 const young=await run(fixture.cases[3]!.attempts as Attempt[]);
 expect(young.pauses.length).toBe(1);
 expect(young.pauses[0]).toBeGreaterThanOrEqual(policy.settlePauseMilliseconds);
 expect(young.pauses[0]).toBeLessThanOrEqual(policy.maximumPauseRounds*policy.settlePauseMilliseconds);
});

test("the named failure carries the owner and both languages",async()=>{
 const failure=await observeOwnerWithRetryV1({owner:"@semio-tech/repo",racyMilliseconds:0,hold:async()=>true,attempt:async onFailure=>{onFailure([]);throw Error("route-undeclared:test-x");}}).catch(error=>error);
 expect(isOwnerFailureV1(failure)).toBe(true);
 expect(failure.owners.map(owner=>owner.name)).toEqual(["@semio-tech/repo"]);
 expect(failure.owners[0]).toEqual({name:"@semio-tech/repo",code:"Error",reason:"route-undeclared:test-x"});
 expect(failure.stable).toBe(true);
 expect(failure.text.en).toContain("@semio-tech/repo");
 expect(failure.text.en).toContain("route-undeclared:test-x");
 expect(failure.text.de).toContain("kann nicht abgeleitet werden");
 expect(failure.message).toBe(failure.text.en+" / "+failure.text.de);
 expect(failure.cause.message).toBe("route-undeclared:test-x");
});

test("a table read through the observed reader follows an edit inside one process, the route table is never a module imported once",()=>{
 const directory=mkdtempSync(join(tmpdir(),"owner-isolation-tables-"));
 try{
  const put=(path:string,text:string)=>{mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),text);};
  const build=JSON.parse(readFileSync(join(workspace,aggregate.buildTable),"utf8"));
  put(aggregate.cacheTable,readFileSync(join(workspace,aggregate.cacheTable),"utf8"));put(aggregate.buildTable,JSON.stringify(build));
  const read=(path:string)=>readFileSync(join(directory,path),"utf8"),before=readCanonicalTestRoutesV1(read).map(route=>route.id);
  put(aggregate.buildTable,JSON.stringify({...build,suites:[...build.suites,{...build.suites[0],id:"zz-edited-in-process"}]}));
  const after=readCanonicalTestRoutesV1(read).map(route=>route.id);
  expect(after.filter(id=>!before.includes(id))).toEqual(["zz-edited-in-process"]);expect(after.length).toBe(before.length+1);
  expect(readCanonicalTestRoutesV1().map(route=>route.id)).toEqual(readCanonicalTestRoutesV1(path=>readFileSync(join(workspace,path),"utf8")).map(route=>route.id));
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const pair=()=>{
 const directory=mkdtempSync(join(tmpdir(),"owner-isolation-pair-")),put=(path:string,text:string)=>{mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),text);};
 for(const path of [aggregate.cacheTable,aggregate.buildTable,aggregate.manifest])put(path,readFileSync(join(workspace,path),"utf8"));
 const id="zz-mid-read-pair",build=JSON.parse(readFileSync(join(directory,aggregate.buildTable),"utf8")),manifest=JSON.parse(readFileSync(join(directory,aggregate.manifest),"utf8"));
 const donor=build.suites.find((row:{id:string})=>row.id==="build-test-file")??build.suites[0],donorTarget="test-"+donor.id;
 const first=()=>put(aggregate.buildTable,JSON.stringify({...build,suites:[...build.suites,{...donor,id}]}));
 const second=()=>{
  const target=structuredClone(manifest.targets[donorTarget]);target.options.command=String(target.options.command).replace(donor.id,id);
  const next={...manifest,targets:{...manifest.targets,["test-"+id]:target}};
  next.targets[aggregate.target]=deriveCanonicalTestAggregateV1({...next,root:aggregate.root},path=>readFileSync(join(directory,path),"utf8"));
  put(aggregate.manifest,JSON.stringify(next,null,2)+"\n");
 };
 return {directory,first,second,cleanup:()=>rmSync(directory,{recursive:true,force:true})};
};
const observedRows=(seen:Map<string,string>)=>[...seen].map(([path,digest])=>({kind:"file",path,fact:{sha256:digest,mtimeMs:0,ctimeMs:0}}));

test("a cross-file pair edited mid-read (first file, then the second) completes with the final state",async()=>{
 const state=pair();
 try{
  const seen=new Map<string,string>(),text=(path:string)=>{const value=readFileSync(join(state.directory,path),"utf8");seen.set(path,sha(value));return value;};
  requireCanonicalTestAggregateV1(JSON.parse(text(aggregate.manifest)),aggregate.root,text);
  let attempts=0,peerLanded=false;const retries:number[]=[];
  const result=await observeOwnerWithRetryV1({
   owner:"repo",racyMilliseconds:fixture.racyMilliseconds,pause:async()=>{},onRetry:event=>retries.push(event.round),
   hold:async()=>{if(!peerLanded){peerLanded=true;state.second();}return [...seen].every(([path,digest])=>sha(readFileSync(join(state.directory,path),"utf8"))===digest);},
   attempt:async onFailure=>{
    attempts++;seen.clear();
    try{const manifest=JSON.parse(text(aggregate.manifest));if(attempts===1)state.first();requireCanonicalTestAggregateV1(manifest,aggregate.root,text);return "sealed";}
    catch(error){onFailure(observedRows(seen));throw error;}
   },
  });
  expect(result).toBe("sealed");
  expect(attempts).toBe(2);
  expect(retries).toEqual([1]);
 }finally{state.cleanup();}
});

test("the same pair left half edited is the named stable failure of that owner, reproduced on unchanged inputs",async()=>{
 const state=pair();
 try{
  const seen=new Map<string,string>(),text=(path:string)=>{const value=readFileSync(join(state.directory,path),"utf8");seen.set(path,sha(value));return value;};
  state.first();
  const failure=await observeOwnerWithRetryV1({
   owner:"repo",racyMilliseconds:fixture.racyMilliseconds,pause:async()=>{},
   hold:async()=>[...seen].every(([path,digest])=>sha(readFileSync(join(state.directory,path),"utf8"))===digest),
   attempt:async onFailure=>{seen.clear();try{requireCanonicalTestAggregateV1(JSON.parse(text(aggregate.manifest)),aggregate.root,text);return "sealed";}catch(error){onFailure(observedRows(seen));throw error;}},
  }).catch(error=>error);
  expect(isOwnerFailureV1(failure)).toBe(true);
  expect(failure.stable).toBe(true);
  expect(failure.rounds).toBe(1);
  expect(failure.owners.map(owner=>owner.name)).toEqual(["repo"]);
  expect(typeof failure.owners[0].path).toBe("string");
  expect(String(failure.cause.message)).toContain("route-targets");
 }finally{state.cleanup();}
});

const observation=(rows:()=>unknown[])=>({captureObservedSourceInputsV1:async(observe:()=>Promise<unknown>,onFailure?:(rows:unknown[])=>void)=>{try{return {value:await observe(),observations:rows()};}catch(error){onFailure?.(rows());throw error;}},retainObservedRowsV1:()=>{}});
interface Row {dependencies?:string[];files?:string[];targets?:Record<string,string[]>}
const owners:Record<string,Row>={app:{dependencies:["lib"],files:["app/src/main.ts","side/shared.ts"],targets:{build:["^build"]}},lib:{dependencies:["core"],targets:{build:["^build"]}},core:{targets:{build:[]}},bad:{targets:{build:[]}},side:{targets:{build:[]}}};
const registry=(names:string[]):BuildOwnerRegistryV1=>({version:1,owners:names.map(name=>({name,root:name,manifest:name+"/📋️project.json",packages:[]})),catalogProviders:[]});
const factsOf=(name:string,table:Record<string,Row>)=>({project:{name,targets:Object.fromEntries(Object.entries(table[name]!.targets??{}).map(([target,dependsOn])=>[target,{dependsOn}]))},files:table[name]!.files??[],dependencies:table[name]!.dependencies??[]});
const walk=(table:Record<string,Row>,failing:string,scope:string,report?:Parameters<typeof closeBuildOwnerAdmissionV1>[3],declares?:Parameters<typeof closeBuildOwnerAdmissionV1>[4])=>{
 const observed:string[]=[];
 return {observed,run:()=>closeBuildOwnerAdmissionV1(readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:"app",target:"build"}]}),registry(Object.keys(table)),async owner=>observeOwnerFactsMemoizedV1({
  key:scope+"\0"+loadedGraphAdmissionFormatV1+"\0"+owner.name,owner:owner.name,hold:async()=>true,observation:observation(()=>[]) as never,
  observe:async()=>{observed.push(owner.name);if(owner.name===failing)throw Error("deterministic inference failure of "+owner.name);return factsOf(owner.name,table);},
 }) as never,report,declares)};
};

test("a needed owner that fails deterministically ends the closure with its named failure and every other owner keeps its record",async()=>{
 const table:Record<string,Row>={...owners,app:{...owners.app,dependencies:["lib","bad"],files:["app/src/main.ts"]}};delete table.side;
 const scope="owner-isolation-needed",reports:{owner:string;needed:boolean}[]=[],state=walk(table,"bad",scope,failure=>reports.push({owner:failure.owner,needed:failure.needed}));
 const failure=await state.run().catch(error=>error);
 expect(isOwnerFailureV1(failure)).toBe(true);
 expect(failure.owners.map(owner=>owner.name)).toEqual(["bad"]);
 expect(failure.text.de).toContain("bad");
 expect(reports).toEqual([{owner:"bad",needed:true}]);
 expect(selectedAdmissionFactsV1.entries(scope).map(entry=>entry.suffix).sort()).toEqual(["app","core","lib"]);
 expect(state.observed).toContain("lib");
 expect(state.observed).toContain("core");
});

test("an owner reached only through a file edge is admitted ownership-only: it is never observed, so it cannot fail the read, and it keeps its manifest and its imported file as members",async()=>{
 const table:Record<string,Row>={...owners};delete table.bad;
 const scope="owner-isolation-withheld",reports:{owner:string;needed:boolean}[]=[],state=walk(table,"side",scope,failure=>reports.push({owner:failure.owner,needed:failure.needed}));
 const admission=await state.run();
 expect(admission.owners).toEqual(["app","core","lib","side"]);
 expect(Object.keys(admission.targets).sort()).toEqual(["app","core","lib"]);
 expect(admission.files).toContain("side/shared.ts");expect(admission.files).toContain("side/📋️project.json");
 expect(state.observed).not.toContain("side");
 expect(reports).toEqual([]);
 expect(selectedAdmissionFactsV1.entries(scope).map(entry=>entry.suffix).sort()).toEqual(["app","core","lib"]);
});

test("an owner no selected task needs fails alone when a dependency edge reaches it: the read still closes over every other owner and names it",async()=>{
 const table:Record<string,Row>={...owners,app:{...owners.app,dependencies:["lib","side"],files:["app/src/main.ts"]}};delete table.bad;
 const scope="owner-isolation-withheld-dependency",reports:{owner:string;needed:boolean}[]=[],state=walk(table,"side",scope,failure=>reports.push({owner:failure.owner,needed:failure.needed}),()=>false);
 const admission=await state.run();
 expect(admission.owners).toEqual(["app","core","lib"]);
 expect(reports).toEqual([{owner:"side",needed:false}]);
 expect(selectedAdmissionFactsV1.entries(scope).map(entry=>entry.suffix).sort()).toEqual(["app","core","lib"]);
});

test("a failure that is not an owner's own passes through the walk unchanged",async()=>{
 const table:Record<string,Row>={...owners};delete table.bad;delete table.side;
 const failure=await closeBuildOwnerAdmissionV1(readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:"app",target:"build"}]}),registry(Object.keys(table)),async()=>{throw Object.assign(Error("restart"),{code:"BUILD_AUTHORITY_CHANGED"});}).catch(error=>error);
 expect(failure.code).toBe("BUILD_AUTHORITY_CHANGED");
 expect(isOwnerFailureV1(failure)).toBe(false);
});

test("the memoized observer observes a failed owner again when the rows it read moved, and answers once the inputs settle",async()=>{
 let calls=0,holds=0;const events:number[]=[];
 const value=await observeOwnerFactsMemoizedV1({
  key:"owner-isolation-memo\0"+loadedGraphAdmissionFormatV1+"\0retry",owner:"retry",
  hold:async()=>++holds>1,onRetry:event=>events.push(event.round),
  observation:observation(()=>[{kind:"file",path:"/w/a",fact:{mtimeMs:0,ctimeMs:0}}]) as never,
  observe:async()=>{if(++calls===1)throw Error("half edited pair");return {project:{name:"retry",targets:{}},files:[],dependencies:[]};},
 }) as {project:{name:string}};
 expect(value.project.name).toBe("retry");
 expect(calls).toBe(2);
 expect(events).toEqual([1]);
 expect(selectedAdmissionFactsV1.entries("owner-isolation-memo").length).toBe(1);
});

test("the decision table shared by every inference path equals the hand-stated cases and the Python oracle over its whole domain",()=>{
 expect(fixture.revalidation.attempts).toBe(ownerIsolationPolicyV1.revalidateAttempts);
 expect(fixture.revalidation.cooldownMilliseconds).toBe(ownerIsolationPolicyV1.revalidateCooldownMilliseconds);
 for(const row of fixture.decisionCases)expect(ownerFailureDecisionV1(row as never),row.name).toBe(row.expect);
 const oracle=pythonOracle().decisions;
 for(const foreign of [0,1])for(const moved of [0,1])for(const young of [0,1])for(let round=1;round<=ownerIsolationPolicyV1.retryRounds;round++)
  expect(ownerFailureDecisionV1({foreign:foreign===1,moved:moved===1,young:young===1,round}),`${foreign}${moved}${young}${round}`).toBe(oracle[`${foreign}${moved}${young}${round}`]!);
});

test("the re-validation of a withheld owner equals the hand-stated cases and the Python oracle",()=>{
 const oracle=pythonOracle().revalidation;
 for(const row of fixture.revalidationCases){
  const seen=withheldRevalidationV1({entry:(row as {entry?:{fragment:string;attempts:number;at:number}}).entry,fragment:row.fragment,now:row.now});
  expect(seen,row.name).toEqual(row.expect);
  expect(oracle[row.name],row.name).toEqual(row.expect);
 }
});

test("a read infers a withheld owner again once per cool-down, bounded, and never judges tagged, healed or out-of-scope owners",async()=>{
 let book:unknown,writes=0,clock=1_000_000;
 const owners={a:{kind:"unavailable",config:"a/project.json",fragment:"f1"},b:{kind:"project",fragment:"f2"},c:{kind:"unavailable",tag:"group",config:"c/project.json",fragment:"f3"},d:{kind:"unavailable",config:"d/project.json",fragment:"f4"}};
 const lines:string[]=[],port=(include?:(name:string)=>boolean)=>({ledger:{read:async()=>book,write:async(next:unknown)=>{book=next;writes++;}},now:()=>clock,...include?{include}:{},say:(text:{en:string;de:string})=>lines.push(text.en+" / "+text.de)});
 expect(await revalidatedWithheldOwnersV1(owners,port())).toEqual([]);
 expect(Object.keys(book as object).sort()).toEqual(["a","d"]);
 clock+=ownerIsolationPolicyV1.revalidateCooldownMilliseconds-1;
 expect(await revalidatedWithheldOwnersV1(owners,port())).toEqual([]);
 clock+=1;
 expect(await revalidatedWithheldOwnersV1(owners,port(name=>name==="a"))).toEqual(["a"]);
 expect((book as {d:{attempts:number}}).d.attempts).toBe(0);
 expect(lines.length).toBe(1);
 expect(lines[0]).toContain("Eigentümer a");
 expect(lines[0]).toContain("Owner a");
 for(let attempt=2;attempt<=ownerIsolationPolicyV1.revalidateAttempts;attempt++){clock+=ownerIsolationPolicyV1.revalidateCooldownMilliseconds;expect(await revalidatedWithheldOwnersV1(owners,port(name=>name==="a"))).toEqual(["a"]);}
 clock+=100*ownerIsolationPolicyV1.revalidateCooldownMilliseconds;
 expect(await revalidatedWithheldOwnersV1(owners,port(name=>name==="a"))).toEqual([]);
 const healed={...owners,a:{kind:"project",fragment:"f9"}};
 await revalidatedWithheldOwnersV1(healed,port());
 expect("a" in (book as object)).toBe(false);
 const broken=await revalidatedWithheldOwnersV1(owners,{ledger:{read:async()=>{throw Error("no ledger");},write:async()=>undefined},now:()=>clock});
 expect(broken).toEqual([]);
 const unwritable=await revalidatedWithheldOwnersV1(owners,{ledger:{read:async()=>({}),write:async()=>{throw Error("read-only");}},now:()=>clock});
 expect(unwritable).toEqual([]);
});


test("a stable owner failure whose rows hold ends the repair walk at once, any other failure keeps repairing",()=>{
 const stable=ownerFailureV1("a",Error("boom"),1,true),unsettled=ownerFailureV1("a",Error("boom"),3,false);
 expect(stableOwnerFailureHoldsV1(stable,true)).toBe(true);
 expect(stableOwnerFailureHoldsV1(stable,false)).toBe(false);
 expect(stableOwnerFailureHoldsV1(unsettled,true)).toBe(false);
 expect(stableOwnerFailureHoldsV1(Error("boom"),true)).toBe(false);
});

const fileRow=(path:string)=>({kind:"file",path,fact:{mtimeMs:0,ctimeMs:0}});
const coded=(message:string,code:string)=>Object.assign(Error(message),{code});
const ownerOfFirstSegment=(path:string)=>path.split("/")[0];

test("a joined failure lists every owner with its own code, first reason line and observed path; a cascade is grouped under the root cause it rests on",()=>{
 const joined=joinOwnerFailuresV1([
  ownerFailureV1("a",Error("boom-a"),1,true,[fileRow("a/x.ts")]),
  ownerFailureV1("b",coded("boom-b\nsecond line","ROUTE"),2,false,[fileRow("c/y.ts")]),
  ownerFailureV1("c",Error("boom-c"),1,true,[fileRow("c/z.ts")]),
 ],ownerOfFirstSegment);
 expect(joined.code).toBe("BUILD_OWNER_FAILED");
 expect(joined.owners).toEqual([
  {name:"a",code:"Error",reason:"boom-a",path:"a/x.ts"},
  {name:"b",code:"ROUTE",reason:"boom-b",path:"c/y.ts",causedBy:"c"},
  {name:"c",code:"Error",reason:"boom-c",path:"c/z.ts"},
 ]);
 expect(joined.rounds).toBe(2);expect(joined.stable).toBe(false);
 expect(joined.text.en).toContain("3 build owners cannot be inferred, 2 of them root causes");
 expect(joined.text.en).toContain("a [Error]: boom-a (rests on a/x.ts)");
 expect(joined.text.en).toContain("c [Error]: boom-c (rests on c/z.ts) [cascaded: b [ROUTE]: boom-b (rests on c/y.ts)]");
 expect(joined.text.de).toContain("davon 2 Ursachen");
 expect(joined.text.de).toContain("c [Error]: boom-c (beruht auf c/z.ts) [kaskadiert: b [ROUTE]: boom-b (beruht auf c/y.ts)]");
 expect(joined.message).toBe(joined.text.en+" / "+joined.text.de);
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$defs:schema.$defs,$ref:"#/$defs/Failure"});
 expect(validate({code:joined.code,owners:joined.owners,rounds:joined.rounds,stable:joined.stable,message:joined.message})).toBe(true);
});

test("a chain of cascades groups every owner under the root cause at its end, and a cycle has no root: both keep their own failure",()=>{
 const chain=joinOwnerFailuresV1([
  ownerFailureV1("x",Error("x-fails"),1,true,[fileRow("y/a.ts")]),
  ownerFailureV1("y",Error("y-fails"),1,true,[fileRow("z/b.ts")]),
  ownerFailureV1("z",Error("z-fails"),1,true,[fileRow("z/c.ts")]),
 ],ownerOfFirstSegment);
 expect(chain.owners.map(owner=>[owner.name,owner.causedBy])).toEqual([["x","z"],["y","z"],["z",undefined]]);
 const cycle=joinOwnerFailuresV1([
  ownerFailureV1("p",Error("p-fails"),1,true,[fileRow("q/a.ts")]),
  ownerFailureV1("q",Error("q-fails"),1,true,[fileRow("p/b.ts")]),
 ],ownerOfFirstSegment);
 expect(cycle.owners.map(owner=>owner.causedBy)).toEqual([undefined,undefined]);
 expect(cycle.text.en).toContain("2 of them root causes");
});

test("without an owner resolver every owner is its own root cause, and an owner without an observed path says so",()=>{
 const joined=joinOwnerFailuresV1([
  ownerFailureV1("a",Error("boom-a"),1,true,[fileRow("c/y.ts")]),
  ownerFailureV1("c",Error("boom-c"),1,true),
 ]);
 expect(joined.owners.map(owner=>owner.causedBy)).toEqual([undefined,undefined]);
 expect(joined.owners[1]).toEqual({name:"c",code:"Error",reason:"boom-c"});
 expect(joined.text.en).toContain("c [Error]: boom-c (rests on no observed path)");
 expect(joined.text.de).toContain("c [Error]: boom-c (beruht auf keinem beobachteten Pfad)");
});

test("the owner resolver of the admission names cascades from real owner roots, the deepest root wins",()=>{
 const resolver=createSourceOwnerResolverV1({version:1,owners:[
  {name:"lib",root:"lib",manifest:"lib/📋️project.json",packages:[]},
  {name:"app",root:"app",manifest:"app/📋️project.json",packages:[]},
  {name:"app-tools",root:"app/tools",manifest:"app/tools/📋️project.json",packages:[]},
 ],catalogProviders:[]} satisfies BuildOwnerRegistryV1);
 const joined=joinOwnerFailuresV1([
  ownerFailureV1("app",Error("app-fails"),1,true,[fileRow("app/tools/run.ts")]),
  ownerFailureV1("lib",Error("lib-fails"),1,true,[fileRow("lib/src/core.ts")]),
  ownerFailureV1("app-tools",Error("tools-fail"),1,true,[fileRow("app/tools/run.ts")]),
 ],resolver);
 expect(joined.owners.map(owner=>[owner.name,owner.causedBy])).toEqual([["app","app-tools"],["app-tools",undefined],["lib",undefined]]);
});

test("a pending or vanished native owner root is never an owner's own failure: it passes the isolation unchanged for the registry follow, whatever its rows do",async()=>{
 const root="🧰️framework/🛍️products/💻️os/🦀️rust",faults=[coded("Canonical native dependency owner required: "+root,"BUILD_SOURCE_AUTHORITY"),coded("Native dependency has no build project owner: "+root+" (required by x)","BUILD_SOURCE_AUTHORITY"),coded("Build owner manifest absent: "+root+"/Cargo.toml (required by x)","BUILD_OWNER_MANIFEST_ABSENT")];
 for(const fault of faults){
  let attempts=0;
  const thrown=await observeOwnerWithRetryV1({owner:"native",pause:async()=>{},hold:async()=>true,attempt:async onFailure=>{attempts++;onFailure([fileRow("a")]);throw fault;}}).catch(error=>error);
  expect(thrown).toBe(fault);expect(attempts).toBe(1);expect(isOwnerFailureV1(thrown)).toBe(false);expect(registryPendingFailureV1(fault)).toBe(true);
  const named=ownerFailureV1("native",fault,1,true);
  expect(registryPendingFailureV1(named)).toBe(true);expect(stableOwnerFailureHoldsV1(named,true)).toBe(false);
 }
 expect(registryPendingFailureV1(coded("Canonical native dependency owner required: x","BUILD_OTHER"))).toBe(false);
 expect(registryPendingFailureV1(ownerFailureV1("a",Error("boom"),1,true))).toBe(false);
});

test("a joined failure whose owners all failed on a pending native root is followed by the registry custody, never surfaced as an owner failure",async()=>{
 const root="🧰️framework/🔨️modules/🖱️ui/🧬️contract/🚚️window-resource/📦️packages/🦀️rust",pending=(name:string)=>ownerFailureV1(name,coded("Canonical native dependency owner required: "+root,"BUILD_SOURCE_AUTHORITY"),1,true);
 const joined=joinOwnerFailuresV1([pending("a"),pending("b")]);
 expect(registryFaultTextV1(joined)).toContain(root);
 const run=(error:Error)=>withOwnerRegistryCustodyV1<string>(new AbortController().signal,async contain=>contain(async()=>{throw error;}));
 expect(isClosedOwnerRegistryInvalidationV1(await run(joined).catch(thrown=>thrown))).toBe(true);
 const mixed=joinOwnerFailuresV1([pending("a"),ownerFailureV1("b",Error("boom"),1,true)]);
 expect(registryFaultTextV1(mixed)).toBeUndefined();
 expect(await run(mixed).catch(thrown=>thrown)).toBe(mixed);
});
