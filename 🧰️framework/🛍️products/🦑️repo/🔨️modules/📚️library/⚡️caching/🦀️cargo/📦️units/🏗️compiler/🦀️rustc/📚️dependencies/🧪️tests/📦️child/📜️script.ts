#!/usr/bin/env repo
import{Database}from"bun:sqlite";
import{writeFile,mkdir}from"node:fs/promises";
import{join,resolve}from"node:path";
import{planRustUnitsV1}from"../../../../../🟦️.ts";
import{captureRustUnitPlanV1}from"../../../../../📋️plan/🟦️.ts";
import*as receipts from"../../../../../📤️receipt/🟦️.ts";
import*as api from"../../🟦️.ts";
import fixture from"../../../../../🧪️tests/🧫️fixtures/🔣️.json";
import type corpus from"../🔣️.json";
import{receiveScriptProcessInvocation,withIssuedResourceControlV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withResourceLeases,resourceLeasePathV1,type ResourceLeaseControl}from"@semio-tech/process/leases";
const root=process.argv[2],encoded=process.argv[3];if(!root||resolve(root)!==root||!encoded||process.argv.length!==4)throw new Error("Exact owned dependency fixture arguments required");
const row=JSON.parse(encoded)as typeof corpus.current[number],directory=join(root,"a"),ledger=join(root,"l"),resource="rust-unit:"+directory,primary=row.body==="undefined"?undefined:Object.freeze(Error("Exact current dependency body")),result={entered:0,readers:0,released:0,failed:false,exact:false,error:"",wire:"",boundDigest:"",blockedInBody:null as boolean|null,blockedAfter:null as boolean|null,resourceRefinement:false,value:null as string|null};
function probe():boolean{const db=new Database(resourceLeasePathV1(ledger,resource));try{db.exec("pragma busy_timeout=0;begin exclusive");db.exec("rollback");return false;}catch(error){if(/busy|locked/iu.test(String(error)))return true;throw error;}finally{db.close();}}
try{await receiveScriptProcessInvocation(process.env,original=>withIssuedResourceControlV1(original,"rust:current-dependency-child",0,async control=>{
 const plans=planRustUnitsV1(fixture.units.map(unit=>({...unit,dependencies:unit.dependencies.map(edge=>({...edge,artifactDigest:null}))})),"owned/units"),base=plans.find(plan=>!plan.unit.dependencies.length)!,selected=plans.find(plan=>plan.unit.dependencies.length)!;await mkdir(directory);await writeFile(join(directory,base.unit.outputs[0]!),"actual ordinary current dependency DATA");const receipt=await receipts.captureRustUnitReceiptV1(directory,base,control);result.wire=JSON.stringify(receipt);await writeFile(join(directory,"receipt.json"),result.wire);
 const withReader=async<T>(actual:ResourceLeaseControl,body:()=>Promise<T>):Promise<T>=>{result.readers++;try{return await withResourceLeases({directory:ledger,control:actual,signal:actual.signal,resources:[{resource,mode:"shared"}]},body);}finally{result.released++;}},dependencies=new Map([[base.unit.id,{plan:base,directory,receipt,withReader}]]),bound=receipts.bindRustUnitDependencyArtifactsV1(selected,dependencies);result.boundDigest=bound.dependencyDigests[0]!.artifactDigest!;
 const plan=row.binding==="unbound"?selected:row.binding==="wrong"?captureRustUnitPlanV1({...bound,dependencyDigests:bound.dependencyDigests.map(edge=>({...edge,artifactDigest:"f".repeat(64)}))}):bound,captured=api.captureRustUnitDependencyArtifactsV1(plan,dependencies);if(row.changedArtifact)await writeFile(join(directory,base.unit.outputs[0]!),"changed ordinary current dependency DATA");if(row.changedReceipt)await writeFile(join(directory,"receipt.json"),JSON.stringify({...receipt,semanticDigest:"f".repeat(64)}));
 result.value=await api.withCurrentRustUnitDependencyArtifactsV1(captured,control,async actual=>{requireIssuedResourceControlRefinementV1(control,actual);result.resourceRefinement=actual!==control;result.entered++;result.blockedInBody=probe();if(row.body!=="success")throw primary;return"reused";});
}));}catch(error){result.failed=true;result.exact=Object.is(error,primary);result.error=String(error);}
if(result.readers)result.blockedAfter=probe();console.log(JSON.stringify(result));process.exitCode=result.failed?1:0;
