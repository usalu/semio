import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import ts from "typescript";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import * as routing from "../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withCargoCompilerInputsV1,type CargoCompilerInputsRequestV1} from "../../🟦️.ts";
import {CargoInputContinuationTransactionV1,withNestedInputOwnersRetiredV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));

async function observe(row:typeof corpus.cases[number]):Promise<boolean>{
 let settled=false;
 try{await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({version:1,owner:"component:continuation-law",maximumElapsedMilliseconds:30000},{fixture:"component-continuations"},Date.now()),async original=>{
  const admission=routing.createIssuedResourceControlV1(original,"component:admission",0),plan=new CargoInputContinuationTransactionV1(admission),passes=new CargoInputContinuationTransactionV1(admission),request=(control:CargoCompilerInputsRequestV1["control"]):CargoCompilerInputsRequestV1=>({manifest:"/workspace/Cargo.toml",cwd:"/workspace",arguments:[],environment:{},control});
  const failure=Error(row.name),started=plan.start(consume=>withCargoCompilerInputsV1(request(admission),undefined,consume),async view=>{
   const body=async()=>{for(let index=0;index<row.nested;index++)await passes.start(consume=>withCargoCompilerInputsV1(request(view.control),undefined,consume),async()=>undefined);if(row.body==="throw")throw failure;};
   if(row.retire==="before-exit")await withNestedInputOwnersRetiredV1(passes,body);else await body();
  });
  const outcome=await Promise.race([started.then(()=>"resolved",error=>error===failure?"failed":"other"),Bun.sleep(corpus.settleMilliseconds).then(()=>"parked")]);
  settled=outcome!=="parked";
  await passes.close().catch(()=>undefined);await plan.close().catch(()=>undefined);
 });}catch(error){if(!(error instanceof Error)||error.message!==row.name)throw error;}
 return settled;
}

test("pass owners are retired before a failing plan body leaves, so a failed native build settles instead of holding its leases forever",async()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const expected=spawnSync(testPythonV1()??"python3",["-I",oracle],{input:JSON.stringify(corpus),encoding:"utf8"});expect(expected.status,expected.stderr).toBe(0);
 const predicted=JSON.parse(expected.stdout) as boolean[];
 expect(predicted).toEqual(corpus.cases.map(row=>row.settles));
 for(const [index,row] of corpus.cases.entries())expect(await observe(row),row.name).toBe(predicted[index]!);
},60000);

test("the native build driver runs its plan body through the pass-owner retirement",()=>{
 const source=readFileSync(fileURLToPath(new URL("../../../🧩️component-pass/🟦️.ts",import.meta.url)),"utf8"),tree=ts.createSourceFile("driver.ts",source,ts.ScriptTarget.Latest,true),wrapped:string[]=[];
 const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.getText(tree)==="planContinuations.start"){const body=node.arguments[1]!;const inspect=(inner:ts.Node)=>{if(ts.isCallExpression(inner)&&inner.expression.getText(tree)==="withNestedInputOwnersRetiredV1")wrapped.push(inner.arguments[0]!.getText(tree)+":"+inner.arguments[1]!.getText(tree));ts.forEachChild(inner,inspect);};inspect(body);}ts.forEachChild(node,visit);};
 visit(tree);expect(wrapped).toEqual(["inputContinuations:()=>executePasses(actual.control)"]);
});
