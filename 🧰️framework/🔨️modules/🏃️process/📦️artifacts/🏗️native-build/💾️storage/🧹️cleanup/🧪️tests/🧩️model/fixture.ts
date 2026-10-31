import cleanupSchema from"../../🎛️command/🧬️schema/🔣️.json"with{type:"json"};
import{expect}from"bun:test";
import ts from"typescript";
import{AsyncLocalStorage}from"node:async_hooks";
import{lstatSync,readFileSync}from"node:fs";
import{dirname,join}from"node:path";
import*as leases from"../../../../../../🔒️leases/🟦️.ts";
import*as issuer from"../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import*as production from"../../🎛️command/🟦️.ts";
import{nativeImageOperationAdmissionV1}from"../../📋️state/🟦️.ts";
import{requireArtifactDescriptorClosureV1}from"../../../../../../../📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";
import{evaluateExtractedFunctionV1}from"../../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
type Input=Parameters<typeof production.acceptNativeImageCleanupOwnerV1>[0];
/** 🔬️ Exercises actual lifecycle Source with real kernel loans and an explicitly separate model map. */
export async function nativeOperationModelV1(){
 const origin=((import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL??import.meta.url),tree=ts.createSourceFile("cleanup.ts",readFileSync(new URL("../../🎛️command/🟦️.ts",origin),"utf8"),ts.ScriptTarget.Latest,true),find=(name:string)=>{const node=tree.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text===name);expect(node,name).toBeDefined();return node!;};
 expect(find("acceptOwner").getText(tree)).toContain('process.platform!=="darwin"');expect(find("action").getText(tree)).toContain("platform:process.platform");
 const operation=find("withNativeImageBodyOperationV1").getText(tree);expect(operation.indexOf("scopeOf(owner)")).toBeLessThan(operation.indexOf("nativeImageOperationAdmissionV1"));expect(operation.indexOf("check(scope,false)")).toBeLessThan(operation.indexOf("nativeImageOperationAdmissionV1"));
 const settlement=find("settleNativeImageCleanupV1").getText(tree);expect(settlement).toContain("nextNativeImageCleanupCommandV1(owner)");expect(settlement).toContain("removeSettledNativeImageV1(owner)");
 const names=["scopeOf","exact","check","refinement","withNativeImageBodyOperationV1","closeNativeImageBodyOperationsV1","stopNativeImageBodyV1","settleNativeImageCleanupV1"],body=names.map(name=>find(name).getText(tree).replace(/^export\s+/u,"")).join("\n"),source=ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;
 const owners=new WeakMap<object,any>(),acceptedOwners=new Set<object>(),counts={next:0,removed:0};
 const ports={owners,acceptedOwners,lstatSync,resourceLeasePathV1:leases.resourceLeasePathV1,requireAcceptedPhysicalCleanupDescendantV1:issuer.requireAcceptedPhysicalCleanupDescendantV1,requireAcceptedPhysicalCleanupWriterProofV1:leases.requireAcceptedPhysicalCleanupWriterProofV1,requireIssuedResourceControlRefinementV1:issuer.requireIssuedResourceControlRefinementV1,requireResourceLeaseWriterProofV1:leases.requireResourceLeaseWriterProofV1,withRequestedIssuedResourceControlV1:issuer.withRequestedIssuedResourceControlV1,operationContext:new AsyncLocalStorage(),nativeImageOperationAdmissionV1,requireArtifactDescriptorClosureV1,requirePersistentChildParent:()=>{throw Error("No persistent parent in lifecycle model");},verifyPersistentFacts:()=>{throw Error("No persistent receiver in lifecycle model");},limits:cleanupSchema.definitions.Protocol.const,captureAcceptedNativeImageCleanupCommandV1:async()=>{throw Error("No physical command in lifecycle model");},nextNativeImageCleanupCommandV1:(owner:object)=>{if(!owners.has(owner))throw Error("Actual model owner required");counts.next++;return null;},removeSettledNativeImageV1:(owner:object)=>{const scope=owners.get(owner);if(!scope)throw Error("Actual model owner required");scope.settled=true;acceptedOwners.delete(scope);counts.removed++;}};
 const fn=(name:string)=>evaluateExtractedFunctionV1(source,name,ports),check=fn("check");
 return Object.freeze({
  createModelOwnerV1(input:Input):object{const options=Object.freeze({...input}),parents=new Map();for(let current=dirname(options.directory);;current=dirname(current)){const row=lstatSync(current);expect(row.isDirectory()&&!row.isSymbolicLink()).toBe(true);parents.set(current,{dev:row.dev,ino:row.ino,size:0,mode:row.mode});if(dirname(current)===current)break;}const owner=Object.freeze({}),scope={options,directory:options.directory,image:join(options.directory,"fixed.dmg"),root:join(options.directory,"m"),resource:"native-fixed-image:"+options.directory,parents,created:new Map(),bodyOpen:true,createAttempt:false,activationAttempt:false,pending:new Set(),failures:[],commands:0,deactivations:0,observations:0,mapping:"unknown",settled:false,operationOpen:true,operations:new Set(),operationFailures:[]};check(scope,false);owners.set(owner,scope);acceptedOwners.add(scope);expect(()=>production.stopNativeImageBodyV1(owner as never)).toThrow("Live privately accepted Native creator required");if(process.platform!=="darwin")expect(()=>production.acceptNativeImageCleanupOwnerV1(options)).toThrow("Exact declared macOS Native creator required");return owner;},
  withNativeImageBodyOperationV1:fn("withNativeImageBodyOperationV1") as <T>(owner:object,control:leases.ResourceLeaseControl,run:(actual:leases.ResourceLeaseControl)=>Promise<T>)=>Promise<T>,
  closeNativeImageBodyOperationsV1:fn("closeNativeImageBodyOperationsV1") as (owner:object)=>Promise<void>,
  stopNativeImageBodyV1:fn("stopNativeImageBodyV1") as (owner:object)=>Promise<void>,
  settleModelV1:fn("settleNativeImageCleanupV1") as (owner:object)=>Promise<void>,
  observedModelEffects:()=>Object.freeze({...counts}),
 });
}
