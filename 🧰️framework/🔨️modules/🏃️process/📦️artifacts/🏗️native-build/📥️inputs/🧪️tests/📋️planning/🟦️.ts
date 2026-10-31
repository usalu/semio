import type {ScriptInvocation} from "../../../../../🧭️routing/📥️invocation/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {withCargoCompilerInputsV1,withCargoArtifactPlanInputsV1,type CargoCompilerInputsPortV1,type CargoArtifactPlanInputsActualV1} from "../../🟦️.ts";
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

const withModel=(body:(original:ScriptInvocation)=>Promise<void>)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"test:cargo-plannable-inputs",maximumElapsedMilliseconds:30000},{workspaceRoot:process.cwd()},Date.now()),body);
for(const row of fixture.cases)test(row.id,async()=>{
 const database=new Database(":memory:");let entries=0,refused=false;
 try{
  try{await withModel(async original=>{
   const control=createIssuedResourceControlV1(original,"test:plannable-inputs",0),port:CargoCompilerInputsPortV1=(request,body)=>body({...request,...(row.writer?{writer:<T>(actual:ResourceLeaseControl,run:(actual:ResourceLeaseControl)=>Promise<T>):Promise<T>=>{entries++;return run(actual);}}:{})}),run=row.scope==="compiler"?withCargoCompilerInputsV1:withCargoArtifactPlanInputsV1;
   await run({manifest:process.cwd()+"/Cargo.toml",cwd:process.cwd(),arguments:["build"],environment:{},control},port,async actual=>{if(row.production)(actual as CargoArtifactPlanInputsActualV1).requireProduction();if(row.enter)await actual.withWriter(async()=>{});});
  });}catch(error){refused=true;expect(String(error)).toContain("writer-custody-not-entered");}
  const expected=(database.query("select ? and (? = 'compiler' or ?) and not ? as refused").get(Number(row.writer),row.scope,Number(row.production),Number(row.enter)) as {refused:number}).refused===1;
  expect(expected).toBe(row.refused);expect(refused).toBe(expected);expect(entries).toBe(row.writer&&row.enter?1:0);
 }finally{database.close();}
});

test("late production activation refuses an expired genuine input scope",()=>withModel(async original=>{
 const control=createIssuedResourceControlV1(original,"test:expired-plan-inputs",0);let escaped:CargoArtifactPlanInputsActualV1|undefined;
 await withCargoArtifactPlanInputsV1({manifest:process.cwd()+"/Cargo.toml",cwd:process.cwd(),arguments:["build"],environment:{},control},undefined,async actual=>{escaped=actual;});
 expect(()=>escaped!.requireProduction()).toThrow();
}));
