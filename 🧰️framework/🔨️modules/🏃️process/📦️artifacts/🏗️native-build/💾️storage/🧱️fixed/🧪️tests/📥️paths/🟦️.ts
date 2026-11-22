import {test,expect} from "bun:test";
import {loadProductFunctionsV1} from "../../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {join,resolve} from "node:path";
import {requireMaterializedPathV1} from "../../../../../../🔒️leases/🟦️.ts";
import {nativeImageRootV1} from "../../../🌐️platform/📋️contract/🟦️.ts";
import schemaComponent from"../../🧬️schema/🔣️.json";const schema=schemaComponent.definitions.MaterializedPaths;
import corpus from "./🔣️.json";
test("persistent Native generic entry refuses every noncanonical or overlong materialized path before its first lease",async()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{for(const row of corpus.cases){expect(validate(row)).toBe(true);const directory="/"+"d".repeat(row.directoryLength-1)+(row.canonicalDirectory?"":"/../owned"),leaseDirectory="/"+"l".repeat(row.leaseLength-1)+(row.canonicalLease?"":"/../leases"),sentinel=Error("first lease boundary");let acquisitions=0;const control={},bindings={Object,join,requireNativeImageEnvelopeV1:()=>{},requireArtifactDescriptorClosureV1:()=>{},requireIssuedResourceControlRefinementV1:()=>{},checkResourceLeaseControl:()=>{},withIssuedResourceOperationV1:async(_control:unknown,body:()=>Promise<unknown>)=>body(),closeNativeImageBorrowerV1:async()=>{},recordFixedReleaseFailure:()=>{},requireMaterializedPathV1,nativeImageRootV1,fixedPlatformV1:()=>({filename:row.filename}),fixedReceivers:new Set(),fixedReceiverGuard:async()=>{acquisitions++;throw sentinel;},unresolvedFixedPreparations:new Set()};const open=(await loadProductFunctionsV1(new URL("../../🟦️.ts",import.meta.url),["openNativeFixedPlatformBodyV1"],bindings)).openNativeFixedPlatformBodyV1;let refused=false;try{await open(Object.freeze({directory,leaseDirectory,control}),control,undefined,()=>async()=>{});}catch{refused=true;}expect(refused).toBe(true);const lengths=[directory,leaseDirectory,join(directory,row.filename),join(directory,"image.json"),join(directory,"m")].map(path=>path.length),oracle=db.query("SELECT (?1<=256 AND ?2<=256 AND ?3<=256 AND ?4<=256 AND ?5<=256 AND ?6 AND ?7) AS admitted").get(...lengths,Number(directory===resolve(directory)),Number(leaseDirectory===resolve(leaseDirectory)))as {admitted:number};expect(oracle.admitted).toBe(row.expectedAcquisitions);expect(acquisitions).toBe(oracle.admitted);}}
 finally{db.close();}
});
