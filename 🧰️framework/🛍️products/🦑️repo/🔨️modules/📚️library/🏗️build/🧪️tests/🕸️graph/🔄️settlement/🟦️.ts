import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import{AsyncLocalStorage}from"node:async_hooks";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{withBuildTestInvocationV1}from"../../📥️invocation/🟦️.ts";
import*as routing from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import schema from"./🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

test("Observed settlement preserves thrown undefined and exact arbitrary failures",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const source=await readFile(new URL("../../../👁️observation/🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("observation.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="settleObservedComputationV1");if(!node)throw Error("Actual observed settlement owner required");const code=ts.transpileModule(node.getText(tree).replace(/^export\s/u,""),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,db=new Database(":memory:"),phase=routing.createIssuedResourceControlV1(original,"observation:settlement-law",0);
 try{for(const row of corpus.cases){let calls=0,roundFailures=0,accepted=false,value:unknown,failure:{error:unknown}|undefined;const primary=row.kind==="frozen"?Object.freeze({primary:true}):undefined,conflict=Object.freeze({path:"modeled Source conflict"}),bindings={scopes:new AsyncLocalStorage(),sourceCaptures:new AsyncLocalStorage(),inferenceValidationScopes:new AsyncLocalStorage(),retainObservationsV1:()=>{},consistentObservationsV1:()=>row.kind!=="undefined-limit"&&(row.kind!=="recovery"||calls>1),observationsHold:async()=>true,conflictingInferenceErrorV1:()=>conflict,RACY_MS:0},run=new Function(...Object.keys(bindings),code+";return settleObservedComputationV1;")(...Object.values(bindings));try{value=await run(async()=>{routing.requireIssuedResourceControlInvocationV1(phase,original);checkResourceLeaseControl(phase);calls++;await Promise.resolve();if(row.kind==="value"||row.kind==="recovery"&&calls>1)return 73;if(row.kind==="undefined-return")return;throw primary;},{signal:phase.signal,limit:row.kind==="undefined-limit"?1:2,onRound:(event:object)=>{if(Object.hasOwn(event,"failure"))roundFailures++;}});accepted=true;}catch(error){failure={error};}const expected=db.query("select ? in ('value','undefined-return','recovery') accepted,case when ?='recovery' then 2 else 1 end calls,? in ('undefined-limit','recovery') failures").get(row.kind,row.kind,row.kind)as{accepted:number;calls:number;failures:number};expect(row.accepted).toBe(expected.accepted===1);expect(row.calls).toBe(expected.calls);expect(row.roundFailures).toBe(expected.failures);expect(accepted).toBe(row.accepted);expect(calls).toBe(row.calls);expect(roundFailures).toBe(row.roundFailures);if(row.accepted)expect(value).toBe(row.kind==="undefined-return"?undefined:73);else{expect(failure).toBeDefined();expect(failure!.error).toBe(primary);}}
 }finally{db.close();}
}));
