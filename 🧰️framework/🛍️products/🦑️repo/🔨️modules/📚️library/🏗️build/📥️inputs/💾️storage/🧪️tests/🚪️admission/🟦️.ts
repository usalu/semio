import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import{requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {AsyncLocalStorage} from "node:async_hooks";
import {resolve} from "node:path";
import {withBuildTestInvocationV1} from "../../../../🧪️tests/📥️invocation/🟦️.ts";
import {checkScriptInvocation,withScriptInvocationSignalV1,acknowledgeScriptInvocationSignalStopV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import * as api from "../../🟨️.mjs";
import componentSchema from "../../🧬️schema/🔣️.json";
const quota=componentSchema.definitions.OptionalQuota;
const schema=componentSchema.definitions.OptionalLedger;
import corpus from "./🔣️.json";

test("graph writer admission reconciles exactly once before pressure-free work",async()=>withBuildTestInvocationV1(async parent=>{
 const file=ts.createSourceFile("writer.mjs",readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),declaration=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="withGraphFactWriterV1");if(!declaration)throw Error("Actual graph writer admission implementation required");
 const validate=new Ajv({strict:false}).compile(schema.definitions.Maintenance),db=new Database(":memory:");
 try{for(const row of corpus.cases){
  const abort=new AbortController(),cleanup=Error("ledger cleanup"),primary=["undefined","body-undefined"].includes(row.interrupt)?undefined:row.interrupt==="close"?cleanup:Object.freeze(Error("original admission "+row.name));
  await withScriptInvocationSignalV1(parent,abort.signal,async original=>{
   const events:string[]=[],writers=new AsyncLocalStorage<any>(),datasets=new AsyncLocalStorage<any>(),workspace=resolve("/graph-admission-fixture"),control=scriptCompilerAdmissionControlV1(original,"graph:admission-model",20000);let physical=row.availableBytes,censuses=0,maintenance=0,opens=0,storeShared=0,ledgerLive=0,bodyEntered=false;
   db.exec("drop table if exists usage;create table usage(used integer,dirty integer);insert into usage values("+row.usedBytes+","+Number(row.dirty)+")");
   const bindings={writers,datasets,graphDatasetWorkspaceV1:(root:string)=>resolve(root),resolve,repositoryLeaseDirectoryV1:()=>"fixture",randomBytes,requireIssuedResourceControlInvocationV1,requireIssuedResourceControlRefinementV1,checkResourceLeaseControl,buildGraphWriterResourceV1:api.buildGraphWriterResourceV1,requireRepositoryBuildGraphWriterV1:()=>{},checkScriptInvocation,quotaSchema:quota,buildGraphWriterMaintenanceDueV1:api.buildGraphWriterMaintenanceDueV1,buildGraphOptionalRetirementDueV1:api.buildGraphOptionalRetirementDueV1,
    statSync:()=>({dev:1,ino:1}),statfsSync:()=>({bsize:1,bavail:physical}),createBuildGraphOptionalQuotaV1:api.createBuildGraphOptionalQuotaV1,
    acquireQueuedResourceLease:async()=>({release:()=>events.push("writer-release")}),
    openRepositoryCachePartitionV1:async()=>({root:workspace,recheck:async()=>checkScriptInvocation(original),close:async()=>events.push("image-close")}),
    maintainBuildGraphOptionalCachesV1:async()=>{checkScriptInvocation(original);expect(storeShared).toBe(0);expect(ledgerLive).toBe(0);events.push("maintenance");maintenance++;physical=1140850688;db.exec("update usage set used=12,dirty=0");return true;},
    withRepositoryCacheStoreV1:async(_root:string,kind:string,invocation:unknown,phase:typeof control,body:(store:any,actualControl:typeof control)=>Promise<unknown>)=>{expect(invocation).toBe(original);requireIssuedResourceControlRefinementV1(control,phase);if(kind!=="graph")return body({control:phase},phase);opens++;storeShared++;events.push("store-open");try{return await body({root:workspace,control:phase,invocation:original,recheck:async()=>checkResourceLeaseControl(phase)},phase);}finally{storeShared--;events.push("store-close");}},
    openOriginalGraphLedgerV1:async()=>{censuses++;events.push("reconcile");await Promise.resolve();if(row.interrupt==="cancel")abort.abort(primary);checkScriptInvocation(original);if(row.interrupt==="undefined")throw primary;db.query("update usage set used=?1,dirty=0").run(maintenance?12:row.physicalBytes);physical=maintenance?1140850688:row.afterCensusAvailableBytes;ledgerLive++;if(row.interrupt==="cancel-after-census")abort.abort(primary);return {observe:()=>{const current=db.query("select used usedBytes,dirty from usage").get()as{usedBytes:number;dirty:number};return{...current,dirty:Boolean(current.dirty)};},close:()=>{ledgerLive--;events.push("ledger-close");if(["body","body-undefined","close"].includes(row.interrupt))throw cleanup;}};}
   };
   const run=new Function(...Object.keys(bindings),"return ("+declaration.getText(file)+")")(...Object.values(bindings))as(root:string,invocation:unknown,phase:typeof control,body:(store:any)=>Promise<unknown>)=>Promise<unknown>;
   let caught=false,failure:unknown,result:unknown;try{result=await run(workspace,original,control,async()=>{bodyEntered=true;events.push("body");if(["body","body-undefined"].includes(row.interrupt))throw primary;return row.name;});}catch(error){caught=true;failure=error;}
   if(row.interrupt.startsWith("cancel"))acknowledgeScriptInvocationSignalStopV1(original);
   expect(validate({kind:"full",usedBytes:row.physicalBytes,availableBytes:row.afterCensusAvailableBytes})).toBe(true);
   const oracle=db.query("select (?1>536870912 or ?2<1073741824) due").get(row.physicalBytes,row.afterCensusAvailableBytes)as{due:number};
   expect(Boolean(oracle.due)||row.availableBytes<quota.policy.minimumAvailableBytes).toBe(row.maintenance===1||row.interrupt==="close");
   expect(censuses).toBe(row.censuses);expect(maintenance).toBe(row.maintenance);expect(opens).toBe(row.opens);expect(storeShared).toBe(0);expect(ledgerLive).toBe(0);expect(caught).toBe(row.interrupt!=="none");
   if(caught)expect(failure).toBe(primary);else expect(result).toBe(row.name);
   expect(bodyEntered).toBe(!["cancel","cancel-after-census","undefined","close"].includes(row.interrupt));if(row.maintenance&&row.opens===2){expect(events.indexOf("ledger-close")).toBeLessThan(events.indexOf("maintenance"));expect(events.indexOf("store-close")).toBeLessThan(events.indexOf("maintenance"));}if(row.interrupt==="none")expect(db.query("select used,dirty from usage").get()).toEqual({used:row.maintenance?12:row.physicalBytes,dirty:0});
  });
 }}finally{db.close();}
}),60000);
