import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import{join}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import*as routing from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl}from"@semio-tech/process/leases";
import schema from"./🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

test("Framework Cargo helper carries the caller actual phase through completion",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const capabilities=routing.readIssuedScriptProcessEnvelopeV1(original).capabilities;if(!("workspaceRoot"in capabilities)||typeof capabilities.workspaceRoot!=="string")throw Error("Actual workspace capability required");const root=capabilities.workspaceRoot,source=await readFile(join(root,"🧰️framework/📦️packages/🦀️rust/📜️script.ts"),"utf8"),tree=ts.createSourceFile("script.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="testCargo");if(!node)throw Error("Actual Framework Cargo helper required");const code=ts.transpileModule(node.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,db=new Database(":memory:");
 try{for(const row of corpus.cases){const stop=new AbortController(),base=routing.createIssuedResourceControlV1(original,"framework:caller-law",row.phase==="expired"?1:0),issued=routing.createRequestedIssuedResourceControlV1(base,0,stop.signal),phase=row.phase==="copied"?{...issued}:issued,primary=row.phase==="undefined"?undefined:Object.freeze({primary:true});if(row.phase==="cancelled")stop.abort(Error("Framework phase cancelled"));if(row.phase==="expired")await new Promise(done=>setTimeout(done,4));let calls=0,accepted=false,failure:{error:unknown}|undefined;const bindings={cargoPolicy:()=>({manifestPath:"/modeled/Cargo.toml",targetDirectory:"/modeled/target"}),runCargoTestsV1:async(_request:unknown,_policy:unknown,_root:unknown,actual:typeof original,actualPhase:typeof phase)=>{routing.requireIssuedResourceControlInvocationV1(actualPhase,actual);expect(actual).toBe(original);expect(actualPhase).toBe(phase);calls++;await Promise.resolve();if(row.phase==="after-await")stop.abort(Error("Framework phase cancelled after await"));if(row.phase==="undefined"||row.phase==="frozen")throw primary;},requireIssuedResourceControlInvocationV1:routing.requireIssuedResourceControlInvocationV1,checkResourceLeaseControl,process:{env:{}}},run=new Function(...Object.keys(bindings),code+";return testCargo;")(...Object.values(bindings));try{await run(original,phase,"/modeled","Cargo.toml",["fixture"],[]);accepted=true;}catch(error){failure={error};}const expected=db.query("select ?='issued' accepted,? in ('issued','after-await','undefined','frozen') calls").get(row.phase,row.phase)as{accepted:number;calls:number};expect(row.accepted).toBe(expected.accepted===1);expect(row.calls).toBe(expected.calls);expect(accepted).toBe(row.accepted);expect(calls).toBe(row.calls);if(row.phase==="undefined"||row.phase==="frozen"){expect(failure).toBeDefined();expect(failure!.error).toBe(primary);}}
 }finally{db.close();}
}));
