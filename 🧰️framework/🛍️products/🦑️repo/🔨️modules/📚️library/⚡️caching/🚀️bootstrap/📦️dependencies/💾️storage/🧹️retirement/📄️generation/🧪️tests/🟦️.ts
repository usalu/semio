import {expect,test} from "bun:test";
import Ajv from "ajv";
import {parse as parseThirdParty} from "jsonc-parser";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const clone=()=>JSON.parse(JSON.stringify(fixture.receipt));
test("retained generation metadata stays inert across obsolete recipes and removed owners",async()=>{
 const {readJavascriptDependencyRetainedGenerationDataV1:read}=await import("../🟦️.ts"),validate=new Ajv({strict:true}).compile(schema);
 for(const mode of fixture.accepted){
  const receipt=clone();let workspace=fixture.workspace,root=fixture.root;
  if(mode==="current-recipe")receipt.recipeSha256="e".repeat(64);
  if(mode==="windows"){workspace="C:\\workspace";root="C:\\store";receipt.bindings[0]={sourceDirectory:workspace+"\\node_modules",targetDirectory:root+"\\j\\"+receipt.generation.slice(0,20)+"\\node_modules"};}
  const result=read(workspace,root,receipt),reference=parseThirdParty(JSON.stringify(receipt));
  expect(validate(result)).toBe(true);expect(validateJsonSchemaSubset(schema as never,result)).toEqual([]);
  expect(result).toMatchObject({version:1,purpose:"retained-dependency-generation-data",authority:false,currentReady:false,generation:reference.generation,prefix:reference.generation.slice(0,20),scope:reference.scope,recipeSha256:reference.recipeSha256,members:reference.members.length,sourceBindings:reference.bindings,ownerReferences:reference.ownerReferences});
  const {readJavascriptDependencySourceOwnerReadyV1}=await import("../../../🟦️.ts");expect(()=>readJavascriptDependencySourceOwnerReadyV1(result)).toThrow();
  expect(Object.isFrozen(result)).toBe(true);expect(Object.isFrozen(result.sourceBindings)).toBe(true);
  if(mode!=="windows"&&mode!=="current-recipe")expect(result.receiptSha256).toBe(fixture.receiptSha256);
 }
});
test("retained metadata refuses foreign roots and malformed receipts without evaluating accessors",async()=>{
 const {readJavascriptDependencyRetainedGenerationDataV1:read}=await import("../🟦️.ts");let evaluated=0;
 for(const mode of fixture.rejected){
  let receipt=clone(),workspace=fixture.workspace,root=fixture.root;
  if(mode==="foreign-workspace")workspace="/foreign";
  if(mode==="foreign-store")root="/foreign";
  if(mode==="extra")receipt.extra=true;
  if(mode==="proxy")receipt=new Proxy(receipt,{ownKeys(){evaluated++;throw Error("accessed");}});
  if(mode==="accessor")Object.defineProperty(receipt,"generation",{get(){evaluated++;return "a".repeat(64);}});
  if(mode==="noncanonical")receipt.scope="../former";
  if(mode==="wrong-generation")receipt.generation="e".repeat(64);
  if(mode==="binding-outside")receipt.bindings[0].sourceDirectory="/foreign/former/node_modules";
  if(mode==="reference-self")receipt.ownerReferences=[{scope:".",generation:"a".repeat(64)}];
  expect(()=>read(workspace,root,receipt),mode).toThrow();
 }
 expect(evaluated).toBe(0);
});

 test("retirement reference DATA refuses historical aliases and incomplete live graphs",async()=>{
 const api=await import("../🟦️.ts"),cases=(await import("./🔗️references.json")).default,{Database}=await import("bun:sqlite"),db=new Database(":memory:");
 try{for(const row of cases){
  const old=clone(),fresh=clone();fresh.generation="e".repeat(64);fresh.bindings[0].targetDirectory="/store/j/"+fresh.generation.slice(0,20)+"/node_modules";
  const previous=api.readJavascriptDependencyRetainedGenerationDataV1(fixture.workspace,fixture.root,old),active=api.readJavascriptDependencyRetainedGenerationDataV1(fixture.workspace,fixture.root,fresh),input:any={generations:[previous,active],current:[active.generation],aliases:[{kind:"link",resolvedTarget:active.sourceBindings[0]!.targetDirectory}],selected:previous.generation};
  if(row.mode==="historical-alias")input.aliases.push({kind:"link",resolvedTarget:previous.sourceBindings[0]!.targetDirectory});
  if(row.mode==="unknown-alias")input.aliases[0].resolvedTarget="/unknown/node_modules";
  if(row.mode==="physical-alias")input.aliases[0]={kind:"directory",resolvedTarget:null};
  if(["unknown-graph","inactive-absent","current-absent","duplicate-absent","known-absent","present-prefix-absent","absent-current"].includes(row.mode))input.generations[row.mode==="current-absent"?1:0]={...(row.mode==="current-absent"?active:previous),ownerReferences:[{scope:"gone",generation:"f".repeat(64)}]};
  if(row.mode!=="unknown-graph"&&["inactive-absent","current-absent","duplicate-absent","known-absent","present-prefix-absent","unused-absent","absent-current"].includes(row.mode))input.absent=["f".repeat(64)];
  if(row.mode==="duplicate-absent")input.absent.push("f".repeat(64));if(row.mode==="known-absent")input.absent=[previous.generation];if(row.mode==="present-prefix-absent")input.absent=[previous.prefix+"f".repeat(44)];if(row.mode==="absent-current")input.current=["f".repeat(64)];
  if(row.mode==="referenced-obsolete")input.generations[1]={...active,ownerReferences:[{scope:"former",generation:previous.generation}]};
  if(row.mode==="obsolete-artifact-reference")input.generations.push({...previous,generation:"f".repeat(64),prefix:"f".repeat(20),ownerReferences:[{scope:"former",generation:previous.generation}]});
  if(row.mode==="selected-current")input.selected=active.generation;
  if(row.mode==="missing-current")input.current=["f".repeat(64)];
  if(row.mode==="duplicate-generation")input.generations.push(previous);
  let accepted=true;try{const result=api.inspectJavascriptDependencyRetirementReferencesV1(input);expect(result.authority).toBe(false);expect(result.selected).toBe(previous.generation);}catch{accepted=false;}
  expect(accepted,row.mode).toBe(row.accepted);const referenceSchema=(await import("../🧬️schema/🔗️references.json")).default,valid=new Ajv({strict:true}).compile(referenceSchema)(input),reference=db.query("WITH docs AS (SELECT value d FROM json_each(?1,'$.generations')), known AS (SELECT json_extract(d,'$.generation') g,json_extract(d,'$.prefix') p FROM docs), current AS (SELECT value g FROM json_each(?1,'$.current')), absent AS (SELECT value g FROM json_each(?1,'$.absent')), edges AS (SELECT json_extract(d,'$.generation') source,json_extract(r.value,'$.generation') target FROM docs,json_each(d,'$.ownerReferences') r), targets AS (SELECT json_extract(b.value,'$.targetDirectory') target FROM docs,json_each(d,'$.sourceBindings') b WHERE json_extract(d,'$.generation') IN (SELECT g FROM current))\n SELECT NOT EXISTS(SELECT g FROM known GROUP BY g HAVING count(*)>1) AND NOT EXISTS(SELECT p FROM known GROUP BY p HAVING count(*)>1) AND NOT EXISTS(SELECT g FROM current GROUP BY g HAVING count(*)>1) AND NOT EXISTS(SELECT g FROM absent GROUP BY g HAVING count(*)>1) AND NOT EXISTS(SELECT 1 FROM known WHERE p<>substr(g,1,20)) AND NOT EXISTS(SELECT 1 FROM absent a JOIN known k ON a.g=k.g OR substr(a.g,1,20)=k.p) AND NOT EXISTS(SELECT 1 FROM absent a JOIN current c ON a.g=c.g) AND NOT EXISTS(SELECT 1 FROM current WHERE g NOT IN (SELECT g FROM known)) AND NOT EXISTS(SELECT 1 FROM edges WHERE target NOT IN (SELECT g FROM known) AND (source IN (SELECT g FROM current) OR target NOT IN (SELECT g FROM absent))) AND NOT EXISTS(SELECT 1 FROM edges WHERE source IN (SELECT g FROM current) AND target NOT IN (SELECT g FROM current)) AND (json_extract(?1,'$.selected') IS NULL OR (json_extract(?1,'$.selected') IN (SELECT g FROM known) AND json_extract(?1,'$.selected') NOT IN (SELECT g FROM current))) AND NOT EXISTS(SELECT 1 FROM edges WHERE target=json_extract(?1,'$.selected') AND source<>json_extract(?1,'$.selected')) AND NOT EXISTS(SELECT 1 FROM json_each(?1,'$.aliases') a WHERE json_extract(a.value,'$.kind')<>'absent' AND (json_extract(a.value,'$.kind')<>'link' OR json_extract(a.value,'$.resolvedTarget') IS NULL OR json_extract(a.value,'$.resolvedTarget') NOT IN (SELECT target FROM targets))) accepted").get(JSON.stringify(input))as{accepted:number};expect(Boolean(valid&&reference.accepted),row.mode).toBe(accepted);
 }}finally{db.close();}
});

test("private selected reference owner holds roster pins and refuses incomplete proofs before its body",async()=>{
 const api=await import("../🟦️.ts"),ts=await import("typescript"),{readFileSync}=await import("node:fs"),path=await import("node:path"),cases=(await import("./👮️owner.json")).default,source=readFileSync(new URL("../../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("owner.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find((row):row is import("typescript").FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text==="withRepositoryDependencyRetirementReferencesV1");
 const owner=await import("../../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/📤️publication/🟦️.ts");expect(typeof owner.maintainSelectedRepositoryDependencyGenerationV1).toBe("function");expect(ts.transpileModule(source,{reportDiagnostics:true,compilerOptions:{target:ts.ScriptTarget.ES2022}}).diagnostics?.filter(row=>row.category===ts.DiagnosticCategory.Error)??[]).toEqual([]);
 expect(node).toBeDefined();const compiled=ts.transpileModule(node!.getText(tree)+";return withRepositoryDependencyRetirementReferencesV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const ownerSchema=(await import("./👮️owner.schema.json")).default,validateCase=new Ajv({strict:true}).compile(ownerSchema);
 for(const vector of cases){
  expect(validateCase(vector)).toBe(true);expect(validateJsonSchemaSubset(ownerSchema as never,vector)).toEqual([]);
  const old=clone(),fresh=clone();fresh.generation="e".repeat(64);fresh.bindings[0]={sourceDirectory:"/workspace/node_modules",targetDirectory:"/store/j/"+fresh.generation.slice(0,20)+"/node_modules"};
  if(["unknown-graph","present-prefix-target","absent-reference-pin","absent-target-drift"].includes(vector.mode))old.ownerReferences=[{scope:"gone",generation:"f".repeat(64)}];
  if(vector.mode==="referenced-obsolete")fresh.ownerReferences=[{scope:"former",generation:old.generation}];
  const docs=[old,fresh],candidates=docs.map((doc,index)=>({prefix:doc.generation.slice(0,20),generation:doc.generation,status:index?"active":"inactive",receiptFingerprint:"fp-"+index,roles:["bindings.json"],ownerReferences:doc.ownerReferences}));
  if(vector.mode==="unqualified")candidates[0]!.status="unqualified";
  let selected:any=candidates[0];if(vector.mode==="empty"){selected={prefix:"f".repeat(20),generation:null,status:"unqualified",receiptFingerprint:null,roles:[]};candidates.push(selected);}
  const held:Array<{name:string;active:boolean;release:()=>void}>=[],released:string[]=[],controller=new AbortController(),control={signal:controller.signal,publish:async()=>{},yieldContinuation:async()=>{}},context={root:fixture.root,check:(actual:unknown)=>{expect(actual).toBe(control);controller.signal.throwIfAborted();},recheck:async()=>{}},current={scopes:[{generation:fresh.generation,bindings:fresh.bindings}]},plan={candidates,selected,generations:{names:[...candidates.map(row=>row.prefix),...(vector.mode==="present-prefix-target"?["f".repeat(20)]:[])]}};let after=false,bodies=0;
  const target=fresh.bindings[0].targetDirectory,aliasRows=[{kind:"link",resolvedTarget:vector.mode==="unknown-alias"?"/unknown/node_modules":target},...(vector.mode==="historical-alias"?[{kind:"link",resolvedTarget:old.bindings[0].targetDirectory}]:[])],ports={repositoryDependencySelectedNamesV1:(_root:string,maximum:number,check:()=>void)=>{check();expect(maximum).toBe(4096);return{names:[...plan.generations.names,...(after&&vector.mode==="absent-target-drift"?["f".repeat(20)]:[])]};},dependencySelectedPlanProtocol:{maximumGenerations:4096},readJavascriptDependencyUnreadyMetadataV1:()=>{throw Error("Ordinary generation fixture must not read opaque capsule metadata");},dependencyInstallSchema:{$defs:{UnreadyRetirementProtocol:{const:{maximumMetadataBytes:4096}}}},join:path.join,dirname:path.dirname,relative:path.relative,sep:path.sep,readJavascriptDependencyRetainedGenerationDataV1:api.readJavascriptDependencyRetainedGenerationDataV1,inspectJavascriptDependencyRetirementReferencesV1:api.inspectJavascriptDependencyRetirementReferencesV1,concurrentPinResourceV1:(_workspace:string,name:string)=>name,tryConcurrentPinExclusiveV1:async(_directory:string,_workspace:string,name:string)=>{if(vector.mode==="absent-pin"||vector.mode==="absent-reference-pin"&&name==="f".repeat(20))return null;const lease={name,active:true,release:()=>{expect(lease.active).toBe(true);lease.active=false;released.push(name);if(vector.mode==="release-error")throw Error("release failure");}};held.push(lease);return lease;},requireSelectedToolingLeaseCustodyV1:(lease:typeof held[number],_directory:string,name:string,actual:unknown)=>{expect(lease.active).toBe(true);expect(lease.name).toBe(name);expect(actual).toBe(control);},readJavascriptDependencyBindingReceiptReadV1:async(_context:unknown,filename:string)=>{const index=docs.findIndex(doc=>filename.includes(doc.generation.slice(0,20)));if(index<0)throw Error("unknown receipt");return{value:docs[index],fingerprint:"fp-"+index+(after&&index===1&&vector.mode==="metadata-drift"?"changed":"")};},readJavascriptDependencySourceAliasesV1:async(_workspace:string,sources:string[])=>{expect(sources.sort()).toEqual(["current","former"]);return{rows:aliasRows,sha256:after&&vector.mode==="alias-drift"?"changed":"stable"};},retainNativeStorageFailureV1:(failures:unknown[],error:unknown)=>{failures.push(error);},retainedRepositoryProtectedParents:new Set()};
  const run=new Function(...Object.keys(ports),compiled)(...Object.values(ports)),primary=Object.freeze({source:"retained-owner-primary"});let accepted=true,failure:unknown;
  try{await run(fixture.workspace,"/leases",context,control,current,plan,async(recheck:()=>Promise<void>)=>{bodies++;expect(held).toHaveLength(candidates.length+(["unknown-graph","absent-target-drift"].includes(vector.mode)?1:0));expect(held.every(lease=>lease.active)).toBe(true);await recheck();if(vector.mode==="frozen-primary")throw primary;after=true;});}catch(error){accepted=false;failure=error;}
  expect(accepted,vector.mode).toBe(vector.accepted);expect(bodies,vector.mode).toBe(vector.bodies);expect(held.every(lease=>!lease.active)).toBe(true);expect(released).toEqual(held.map(row=>row.name).reverse());if(vector.mode==="frozen-primary")expect(failure).toBe(primary);
 }
});

test("selected maintenance retains actual funding under a larger declaration ceiling",async()=>{
 const api=await import("../🟦️.ts"),fixture=(await import("./💰️funding.json")).default,wireSchema=(await import("../🧬️schema/💰️funding.json")).default,validate=new Ajv({strict:true}).compile(wireSchema),{Database}=await import("bun:sqlite"),db=new Database(":memory:");
 try{for(const row of fixture.cases){
  const value:any={ceiling:structuredClone(fixture.ceiling),retained:structuredClone(fixture.retained),allocationBytes:fixture.allocationBytes,backingMaximumPhysicalBytes:fixture.backingMaximumPhysicalBytes};
  if(row.mode==="equal"){value.retained=structuredClone(value.ceiling);value.allocationBytes=value.ceiling.maximumArtifactBytes;value.backingMaximumPhysicalBytes=value.ceiling.capacityBytes;}
  if(row.mode==="over-ceiling"){value.retained.capacityBytes=value.ceiling.capacityBytes+1;value.backingMaximumPhysicalBytes=value.retained.capacityBytes;}
  if(row.mode==="wrong-root")value.retained.root="/foreign";
  if(row.mode==="wrong-lease")value.retained.leaseDirectory="/foreign";
  if(row.mode==="wrong-capacity")value.retained.capacityBytes++;
  if(row.mode==="unfunded-workspace")value.retained.maximumWorkspaceBytes++;
  if(row.mode==="unfunded-artifact")value.retained.maximumArtifactBytes++;
  if(row.mode==="over-backing"){value.retained.capacityBytes=value.allocationBytes-1;value.backingMaximumPhysicalBytes=value.retained.capacityBytes;}
  if(row.mode==="no-allocation")value.allocationBytes=0;
  if(row.mode==="fraction")value.retained.capacityBytes+=0.5;
  if(row.mode==="extra")value.callback=true;
  let accepted=true;try{const actual=api.readJavascriptDependencyRetirementFundingPolicyV1(value);expect(actual.authority).toBe(false);expect(actual.policy).toEqual(value.retained);expect(Object.isFrozen(actual.policy)).toBe(true);}catch{accepted=false;}
  const relation=db.query("SELECT json_extract(?1,?2)=json_extract(?1,?3) AND json_extract(?1,?4)=json_extract(?1,?5) AND json_extract(?1,?6)=json_extract(?1,?7) AND json_extract(?1,?8)=json_extract(?1,?9) AND json_extract(?1,?10)=json_extract(?1,?9) AND json_extract(?1,?6)>=json_extract(?1,?9) AND json_extract(?1,?6)<=json_extract(?1,?11) AND json_extract(?1,?8)<=json_extract(?1,?12) AND json_extract(?1,?10)<=json_extract(?1,?13) accepted").get(JSON.stringify(value),"$.retained.root","$.ceiling.root","$.retained.leaseDirectory","$.ceiling.leaseDirectory","$.retained.capacityBytes","$.backingMaximumPhysicalBytes","$.retained.maximumWorkspaceBytes","$.allocationBytes","$.retained.maximumArtifactBytes","$.ceiling.capacityBytes","$.ceiling.maximumWorkspaceBytes","$.ceiling.maximumArtifactBytes")as{accepted:number};
  expect(accepted,row.mode).toBe(row.accepted);expect(accepted,row.mode).toBe(Boolean(validate(value)&&relation.accepted));expect(validateJsonSchemaSubset(wireSchema as never,value).length===0).toBe(Boolean(validate(value)));
 }}finally{db.close();}
});
