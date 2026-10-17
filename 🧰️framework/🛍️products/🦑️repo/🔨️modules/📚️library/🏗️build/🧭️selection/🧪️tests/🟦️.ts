/** 🧪️ Selection: 4 tests covering neutral selection corpus has closed schema and independent boolean parity; watch callback literals never enable global source events; affected booleans and backend overrides preserve selected tasks a… */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {createRequire} from "node:module";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import ts from "typescript";
import {parseArgsStringToArgv} from "string-argv";
import {buildOptionV1,buildPatternV1,selectBuildProjectsV1,validateBuildTaskArgumentsV1} from "../🟦️.ts";
import {parseBuildInvocationV1,nxBuildArgumentsV1} from "../../🧭️invocation/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,checkScriptInvocation,requireIssuedResourceControlInvocationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./📋️schema.json";

const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("selection.ts",source,ts.ScriptTarget.Latest,true),require=createRequire(import.meta.url),parse=require("yargs-parser"),compilers=[(text:string)=>new Bun.Transpiler({loader:"ts"}).transformSync(text),(text:string)=>ts.transpileModule(text,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText];
const select=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="selectBuildTasksV1");
if(!select)throw Error("Actual task selection body required");
const functions=tree.statements.filter(node=>ts.isFunctionDeclaration(node)&&["shellArgument","buildBooleanOptionV1"].includes(node.name?.text??"")).map(node=>node.getText(tree).replace(/^export /u,"")),predicates:ts.VariableStatement[]=[],watchOptions:ts.VariableStatement[]=[];
ts.forEachChild(tree,function visit(node:ts.Node):void{if(ts.isVariableStatement(node)&&node.declarationList.declarations.some(row=>row.name.getText(tree)==="global"))predicates.push(node);if(ts.isVariableStatement(node)&&node.declarationList.declarations.some(row=>row.name.getText(tree)==="includeGlobal"))watchOptions.push(node);ts.forEachChild(node,visit);});
if(predicates.length!==1)throw Error("Unique actual watch event predicate required");

/** 🧬️ The independent grammar and yargs parser agree on all accepted boolean forms. */
test("neutral selection corpus has closed schema and independent boolean parity",()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:");try{for(const row of [...corpus.tasks,...corpus.watch]){
  expect(()=>validateBuildTaskArgumentsV1(row.args)).not.toThrow();
  const boundary=row.args.indexOf("--"),head=row.args.slice(1,boundary<0?undefined:boundary),nx=nxBuildArgumentsV1(row.args).slice(1,boundary<0?undefined:boundary),flags=["all","untracked","uncommitted","include-global-files"],parsed=parse(head,{boolean:flags}),projected=parse(nx,{boolean:["all","untracked","uncommitted","includeGlobalWorkspaceFiles"]});
  for(const flag of flags){const result=db.query("select exists(select 1 from json_each(?1) where value=?2 or value=?3) enabled").get(JSON.stringify(head),"--"+flag,"--"+flag+"=true") as {enabled:number};expect(Boolean(result.enabled)).toBe(Boolean(parsed[flag]));expect(Boolean(projected[flag==="include-global-files"?"includeGlobalWorkspaceFiles":flag])).toBe(Boolean(parsed[flag]));}
 }}finally{db.close();}
});

/** 👁️ Executes the exact production event predicate while SQLite independently interprets the callback boundary. */
test("watch callback literals never enable global source events",()=>{
 const db=new Database(":memory:");try{for(const compile of compilers)for(const row of corpus.watch){
  const run=new Function("args","boundary","path","names","graph",compile([...watchOptions,...predicates].map(node=>node.getText(tree)).join("\n"))+";return {selected,global};"),actual=run(row.args,row.args.indexOf("--"),row.path,["app"],corpus.graph),oracle=db.query("select exists(select 1 from json_each(?1) where cast(key as integer)<(select min(cast(key as integer)) from json_each(?1) where value='--') and value in ('--include-global-files','--include-global-files=true')) and not exists(select 1 from json_each(?2,'$.projects') where ?3 like json_extract(value,'$.root')||'/%') global").get(JSON.stringify(row.args),JSON.stringify(corpus.graph),row.path) as {global:number};
  expect(Boolean(oracle.global)).toBe(row.global);expect(actual).toEqual({selected:row.selected,global:row.global});
 }}finally{db.close();}
});

/** 🎯️ Models Git results through an explicit port while retaining actual issued selection controls and literal argv. */
test("affected booleans and backend overrides preserve selected tasks and callback arguments",async()=>{
 const db=new Database(":memory:");try{for(const compile of compilers)for(const tool of ["nx","bazel"] as const)for(const row of corpus.tasks){
  const captured:string[][]=[],graph=structuredClone(corpus.graph),parsed=parseBuildInvocationV1(["--tool="+tool,...row.args],{SEMIO_BUILD_TOOL:tool==="nx"?"bazel":"nx"});expect(parsed.tool).toBe(tool);expect(parsed.args).toEqual(row.args);
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"selection-source-model",maximumElapsedMilliseconds:30000},{},Date.now()),async original=>{
   const capture=async(_program:string,args:string[],_root:string,options:{control:Parameters<typeof requireIssuedResourceControlInvocationV1>[0]})=>{requireIssuedResourceControlInvocationV1(options.control,original);captured.push([...args]);return "packages/lib/main.ts\0";},bindings={checkScriptInvocation,buildOptionV1,buildPatternV1,selectBuildProjectsV1,scriptCompilerAdmissionControlV1,captureOwnedCommandV1:capture},run=new Function(...Object.keys(bindings),compile(functions.join("\n")+"\n"+select.getText(tree).replace(/^export /u,""))+";return selectBuildTasksV1;")(...Object.values(bindings)),actual=await run(process.cwd(),graph,parsed.args,original);
   expect(captured).toEqual(row.git);expect(actual.tasks).toEqual(row.tasks);expect(actual.args).toEqual(row.forwarded);
   const tail=db.query("select json_group_array(value) value from json_each(?1) where cast(key as integer)>(select min(cast(key as integer)) from json_each(?1) where value='--')").get(JSON.stringify(row.args)) as {value:string};
   if(row.args[0]==="exec"){const execution=graph.projects.app.targets as unknown as Record<string,{execution:{program:string;args:string[]}}> ;expect([execution["repo-exec"]!.execution.program,...execution["repo-exec"]!.execution.args]).toEqual(JSON.parse(tail.value));}else expect(actual.args).toEqual(JSON.parse(tail.value));
   checkScriptInvocation(original);
  });
 }}finally{db.close();}
});

/** 🛑️ The portable default and every admitted bail form agree with the adapter's independent yargs grammar. */
test("neutral bail defaults false and respects explicit boolean values",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(corpus)).toBe(true);
 for(const compile of compilers)for(const row of corpus.bail){
  expect(()=>validateBuildTaskArgumentsV1(row.args)).not.toThrow();
  const boundary=row.args.indexOf("--"),head=row.args.slice(1,boundary<0?undefined:boundary),oracle=parse(nxBuildArgumentsV1(head),{boolean:["nx-bail"]});expect(oracle["nx-bail"]??false).toBe(row.expected);
  const bindings={checkScriptInvocation,buildOptionV1,buildPatternV1,selectBuildProjectsV1,scriptCompilerAdmissionControlV1},run=new Function(...Object.keys(bindings),compile([...functions,select.getText(tree).replace(/^export /u,"")].join("\n"))+";return selectBuildTasksV1;")(...Object.values(bindings));
  await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"selection-bail-model",maximumElapsedMilliseconds:30000},{},Date.now()),async original=>{expect((await run(process.cwd(),structuredClone(corpus.graph),row.args,original)).bail).toBe(row.expected);});
 }
});
