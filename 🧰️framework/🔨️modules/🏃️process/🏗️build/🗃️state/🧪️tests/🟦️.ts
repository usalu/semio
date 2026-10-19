import {expect,test} from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {Database} from "bun:sqlite";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {withCompilerState} from "../🟦️.ts";
import {typecheckProject} from "../../🟦️typescript/🟦️.ts";
import {typecheckAdmittedProjectV1,type TypeScriptProjectCheckRequestV1} from "../../🟦️typescript/🔌️provider/🟦️.ts";
import {receiveScriptProcessInvocation,withIssuedResourceControlV1,withScriptInvocationSignalV1,createIssuedResourceControlV1} from "../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import cases from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

test("compiler state authenticates its actual storage owner before effect DATA",async()=>receiveScriptProcessInvocation(process.env,async original=>withIssuedResourceControlV1(original,"compiler-state:admission",0,async control=>{
 expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);
 const database=new Database(":memory:");database.exec("create table admission(receiver text,kind text, accepted integer check(accepted=0), dataReads integer check(dataReads=0), effects integer check(effects=0),primary key(receiver,kind))");
 const rows:Record<string,unknown>[]=[],root=join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"compiler-state-unauthorized-"+process.pid);
 try{for(const receiver of cases.receivers)for(const row of cases.cases){
  const invoke=async(storageControl:unknown)=>{let dataReads=0,effects=0,accepted=false,failure:unknown;
   const options={invocation:original,control,storageControl,storage:row.kind==="missing-storage"?undefined:Object.freeze({}),owner:"state-admission",compiler:"source-only",configuration:"admission",mode:"prototype",capacity:false,get root(){dataReads++;throw Error("Effect root DATA read before actual storage authority");},get workspace(){dataReads++;throw Error("Workspace DATA read before actual storage authority");},get cwd(){dataReads++;throw Error("Compiler cwd DATA read before actual storage authority");},get project(){dataReads++;throw Error("Compiler project DATA read before actual storage authority");},get timeoutMs(){dataReads++;throw Error("Compiler budget DATA read before actual storage authority");},get leaseDirectory(){dataReads++;throw Error("Caller lease DATA read before actual storage authority");},get files(){dataReads++;throw Error("Materialization DATA read before actual storage authority");},get environment(){dataReads++;throw Error("Environment DATA read before actual storage authority");}};
   try{if(receiver==="state")await withCompilerState(options as unknown as Parameters<typeof withCompilerState>[0],async()=>{effects++;return undefined;});else await typecheckProject(options as unknown as Parameters<typeof typecheckProject>[0]);accepted=true;}catch(error){failure=error;}
   const observed={receiver,kind:row.kind,accepted,dataReads,effects,failure:failure instanceof Error?failure.message:String(failure),created:existsSync(root)};rows.push(observed);
  };
  if(row.kind==="foreign-owner")await withScriptInvocationSignalV1(original,new AbortController().signal,async alias=>invoke(createIssuedResourceControlV1(alias,"compiler-state:foreign",0)));
  else await invoke(row.kind==="copied-owner"?{...control}:control);
 }
 for(const kind of cases.repositoryCases){let dataReads=0,failure:unknown;const options={invocation:kind==="copied-invocation"?{...original}:original,control:kind==="copied-control"?{...control}:control,get workspace(){dataReads++;throw Error("Repository workspace DATA read before actual control authority");}};try{await typecheckAdmittedProjectV1(options as unknown as TypeScriptProjectCheckRequestV1);}catch(error){failure=error;}expect(dataReads,kind).toBe(0);expect(String(failure),kind).toMatch(/Original|issuance/i);}
 for(const[index,observed]of rows.entries()){const row=cases.cases[index%cases.cases.length]!;expect(observed.accepted,row.kind).toBe(row.accepted);expect(observed.dataReads,row.kind).toBe(row.dataReads);expect(observed.effects,row.kind).toBe(row.effects);expect(observed.created,row.kind).toBe(false);expect(String(observed.failure),row.kind).toMatch(/Original|native storage|refinement|issuance/i);database.query("insert into admission values(?,?,?,?,?)").run(String(observed.receiver),row.kind,Number(observed.accepted),Number(observed.dataReads),Number(observed.effects));}
 expect(database.query("select count(*) as count from admission").get()).toEqual({count:cases.cases.length*cases.receivers.length});
 }finally{database.close();}
})),10000);


test("compiler state Source rechecks its actual effect phase after asynchronous admission barriers",()=>{
 const path=new URL("../🟦️.ts",import.meta.url),source=ts.createSourceFile(path.pathname,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);let body:ts.Block|undefined;const locate=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&ts.isIdentifier(node.name)&&node.name.text==="run"&&node.initializer&&ts.isArrowFunction(node.initializer)&&ts.isBlock(node.initializer.body))body=node.initializer.body;ts.forEachChild(node,locate);};locate(source);if(!body)throw Error("Actual compiler state materialization body required");const calls=body.statements.flatMap(statement=>{if(!ts.isExpressionStatement(statement))return[];const expression=ts.isAwaitExpression(statement.expression)?statement.expression.expression:statement.expression;return ts.isCallExpression(expression)?[{call:expression.expression.getText(source),argument:expression.arguments[0]?.getText(source)??""}]:[];}),database=new Database(":memory:");
 try{database.exec("create table step(ordinal integer primary key, call text, argument text)");calls.forEach((row,index)=>database.query("insert into step values(?,?,?)").run(index,row.call,row.argument));const observations=cases.effectGuards.map(row=>({row,next:database.query("select successor.call, successor.argument from step barrier join step successor on successor.ordinal=barrier.ordinal+1 where barrier.call=?").get(row.operation)}));for(const value of observations)expect(value.next,value.row.operation).toEqual({call:value.row.guard,argument:value.row.argument});}finally{database.close();}
});
