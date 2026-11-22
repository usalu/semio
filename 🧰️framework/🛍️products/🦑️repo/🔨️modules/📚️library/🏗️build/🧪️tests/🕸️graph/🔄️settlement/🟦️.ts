import{test,expect}from"bun:test";
import{AsyncLocalStorage}from"node:async_hooks";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import*as routing from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"@semio-tech/process/leases";
import schema from"./🧬️schema/🔣️.json";
import corpus from"./🔣️.json";
import{instantiateOwnerClosureV1,prepareOwnerClosureV1}from"../🧰️support/🔗️owner-closure/🟦️.ts";
declareBuildTestEnvelopeV1({owner:"test:build-workspace-graph-laws",maximumElapsedMilliseconds:300000,capabilities:{}});
const SETTLE_STUBS_V1=["scopes","sourceCaptures","inferenceValidationScopes","retainObservationsV1","firstObservationConflictV1","observationsHold","conflictingInferenceErrorV1","RACY_MS"];

test("Observed settlement preserves thrown undefined and exact arbitrary failures",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const closure=await prepareOwnerClosureV1({moduleUrl:new URL("../../../👁️observation/🟨️.mjs",import.meta.url),owner:"settleObservedComputationV1",provided:SETTLE_STUBS_V1}),db=new Database(":memory:"),phase=routing.createIssuedResourceControlV1(original,"observation:settlement-law",0);
 try{for(const row of corpus.cases){let calls=0,roundFailures=0,accepted=false,value:unknown,failure:{error:unknown}|undefined;const primary=row.kind==="frozen"?Object.freeze({primary:true}):undefined,conflict=Object.freeze({path:"modeled Source conflict"}),bindings={scopes:new AsyncLocalStorage(),sourceCaptures:new AsyncLocalStorage(),inferenceValidationScopes:new AsyncLocalStorage(),retainObservationsV1:()=>{},firstObservationConflictV1:()=>row.kind!=="undefined-limit"&&(row.kind!=="recovery"||calls>1)?undefined:{row:conflict},observationsHold:async()=>true,conflictingInferenceErrorV1:()=>conflict,RACY_MS:0},run=instantiateOwnerClosureV1(closure,bindings);try{value=await run(async()=>{routing.requireIssuedResourceControlInvocationV1(phase,original);checkResourceLeaseControl(phase);calls++;await Promise.resolve();if(row.kind==="value"||row.kind==="recovery"&&calls>1)return 73;if(row.kind==="undefined-return")return;throw primary;},{signal:phase.signal,limit:row.kind==="undefined-limit"?1:2,onRound:(event:object)=>{if(Object.hasOwn(event,"failure"))roundFailures++;}});accepted=true;}catch(error){failure={error};}const expected=db.query("select ? in ('value','undefined-return','recovery') accepted,case when ?='recovery' then 2 else 1 end calls,? in ('undefined-limit','recovery') failures").get(row.kind,row.kind,row.kind)as{accepted:number;calls:number;failures:number};expect(row.accepted).toBe(expected.accepted===1);expect(row.calls).toBe(expected.calls);expect(row.roundFailures).toBe(expected.failures);expect(accepted).toBe(row.accepted);expect(calls).toBe(row.calls);expect(roundFailures).toBe(row.roundFailures);if(row.accepted)expect(value).toBe(row.kind==="undefined-return"?undefined:73);else{expect(failure).toBeDefined();expect(failure!.error).toBe(primary);}}
 }finally{db.close();}
}));
