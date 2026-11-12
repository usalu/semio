import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import ts from "typescript";

import schema from "../🧬️schema/🔣️.json";
import cases from "./🔣️.json";
import {storybookEnginePackagesV1} from "../🟦️.ts";

test("scope package requirements agree with independent SQLite selection",()=>{
 const validate=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");
 try{for(const row of cases){
  expect(validate(row),JSON.stringify(validate.errors)).toBe(true);
  const selected=db.query("SELECT DISTINCT p.value package FROM json_each(?1) s JOIN json_each(s.value,'$.wasmPackages') p ORDER BY s.key,p.key").all(JSON.stringify(row.scopes)) as {package:string}[];
  const admitted=selected.every(item=>row.catalog.includes(item.package)),expected=admitted?selected.map(item=>item.package):null;
  expect(expected).toEqual(row.expected);
  if(expected===null)expect(()=>storybookEnginePackagesV1(row.scopes,row.catalog)).toThrow();else expect(storybookEnginePackagesV1(row.scopes,row.catalog)).toEqual(expected);
 }}finally{db.close();}
});

const declaration=(name:string)=>{
 const source=readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("source.ts",source,ts.ScriptTarget.Latest,true),node=ast.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text===name);
 if(!node)throw Error("Actual prerequisite owner required");
 return ts.transpile(node.getText(ast).replace(/^export /u,""),{target:ts.ScriptTarget.ESNext});
};
test("actual preparation executes current owned producers serially under the same Original and phase",async()=>{
 const requests=[["run","surface:wasm"],["run","puzzle-plugin:wasm"]],original={identity:Symbol("Original")},phase={identity:Symbol("phase")},events:string[]=[],calls:unknown[][]=[];
 const source=declaration("prepareStorybookEnginesV1").replace(/import\("[^"]*🎟️capacity[^"]*"\)/u,"Promise.resolve(ports.capacity)").replace(/import\("[^"]*📤️child[^"]*"\)/u,"Promise.resolve(ports.child)");
 const run=new Function("storybookEngineBuildArgumentsV1","schema","ports",source+";return prepareStorybookEnginesV1;")(async(root:string,mode:string,scope:string)=>{expect([root,mode,scope]).toEqual(["/repo","prototype","puzzle/2d"]);return requests;},schema,{capacity:{scriptCompilerAdmissionControlV1:(received:unknown,label:string,milliseconds:number)=>{expect(received).toBe(original);expect(label).toBe("storybook:engines");expect(milliseconds).toBe(0);return phase;}},child:{runOwnedBuildChildInvocationV1:async(...args:unknown[])=>{calls.push(args);events.push("start:"+String((args[1] as string[])[1]));await Promise.resolve();events.push("done:"+String((args[1] as string[])[1]));}}});
 await run("/repo","prototype","puzzle/2d",original);
 expect(events).toEqual(["start:surface:wasm","done:surface:wasm","start:puzzle-plugin:wasm","done:puzzle-plugin:wasm"]);
 expect(calls).toEqual(requests.map(args=>["/repo",args,original,phase]));
});

test("actual birth descendant resolver selects the same closed scoped requests as the owning route",async()=>{
 const calls:unknown[][]=[],requests=[["run","actual-owner:wasm"]],root={name:"workspace",targets:{"build-storybook":{options:{command:"repo ./build-script.ts build storybook"}},"dev-storybook":{options:{command:"repo ./root-script.ts dev storybook",forwardAllArgs:true}},"dev-storybook-puzzle-2d":{options:{command:"repo ./root-script.ts dev storybook puzzle 2d"}}}};
 const run=new Function("schema","join","readFileSync","storybookEngineBuildArgumentsV1",declaration("storybookBuildDescendantsV1")+";return storybookBuildDescendantsV1;" )(schema,(...parts:string[])=>parts.join("/"),()=>JSON.stringify(root),async(...args:unknown[])=>{calls.push(args);return requests;});
 const env={SEMIO_BUILD_MODE:"prototype",STORYBOOK_SCOPE:"ui"};
 expect(await run("/repo",["build","storybook"],env)).toBe(requests);
 expect(await run("/repo",["test","storybook","--grep","puzzle"],env)).toBe(requests);
 expect(await run("/repo",["dev","storybook","puzzle","2d","--ci"],env)).toBe(requests);
 expect(await run("/repo",["dev","storybook"],env)).toBe(requests);
 expect(await run("/repo",["run","repo-storybook:build-storybook"],env)).toBe(requests);
 expect(await run("/repo",["run","repo-storybook:dev-storybook-puzzle-2d"],env)).toBe(requests);
 expect(await run("/repo",["run","repo-storybook:dev-storybook","--","puzzle","2d"],env)).toBe(requests);
 expect(await run("/repo",["run","actual-owner:wasm"],env)).toEqual([]);
 expect(await run("/repo",["build","storybook","--help"],env)).toEqual([]);
 expect(await run("/repo",["dev","storybook-static"],env)).toEqual([]);
 expect(calls).toEqual([["/repo","prototype","ui"],["/repo","prototype","ui"],["/repo","prototype","puzzle/2d"],["/repo","prototype",""],["/repo","prototype","ui"],["/repo","prototype","puzzle/2d"],["/repo","prototype","puzzle/2d"]]);
});

test("actual scope declarations exclude unrelated engines and resolve canonical producer project names",async()=>{
 const scopeSource=readFileSync(new URL("../../../../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts",import.meta.url),"utf8"),scopeAst=ts.createSourceFile("scopes.ts",scopeSource,ts.ScriptTarget.Latest,true),scopeNode=scopeAst.statements.find(row=>ts.isVariableStatement(row)&&row.declarationList.declarations.some(node=>ts.isIdentifier(node.name)&&node.name.text==="HAND_CURATED_SCOPES"));
 expect(scopeNode).toBeDefined();
 const catalog=JSON.parse(readFileSync(new URL("../../../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/📇️catalog.json",import.meta.url),"utf8")) as {packages:{packageName:string;id:string;sourceOwner:string}[];sourcePackages:{packageName:string;sourceOwner:string}[]};
 const common=catalog.packages.filter(row=>row.id!=="puzzle").map(row=>row.packageName),source=ts.transpile(scopeNode!.getText(scopeAst).replace(/^export /u,""),{target:ts.ScriptTarget.ESNext}),scopes=new Function("repoRelative","RENDERER_WASM_PACKAGES","runtimeAssetDeliveries",source+";return HAND_CURATED_SCOPES;")((path:string)=>path,common,()=>[]) as {id:string;wasmPackages?:string[]}[];
 const resolve=(id:string)=>scopes.filter(row=>row.id===id);
 expect(storybookEnginePackagesV1(resolve("ui"))).toEqual([]);
 expect(storybookEnginePackagesV1(resolve("infinite"))).toEqual([]);
 const puzzle=storybookEnginePackagesV1(resolve("puzzle/2d"));
 expect(puzzle).toEqual([...common,"@semio-tech/puzzle-wasm"]);
 expect(puzzle.some(name=>/cad|layout|flow/u.test(name))).toBe(false);
 const code=declaration("storybookEngineBuildArgumentsV1").replace(/import\("[^"]*coordination[^"]*"\)/u,"Promise.resolve(ports.scopes)");
 const run=new Function("catalog","readFileSync","join","storybookEnginePackagesV1","ports",code+";return storybookEngineBuildArgumentsV1;")(catalog,readFileSync,join,storybookEnginePackagesV1,{scopes:{resolveActiveScopes:resolve}});
 expect(await run(process.cwd(),"prototype","ui")).toEqual([]);
 const requests=await run(process.cwd(),"release","puzzle/2d");
 expect(requests).toEqual(puzzle.map(name=>{const owner=catalog.sourcePackages.find(row=>row.packageName===name)!;return ["run",JSON.parse(readFileSync(join(process.cwd(),owner.sourceOwner,"📋️project.json"),"utf8")).name+":wasm"];}));
 expect(requests).toContainEqual(["run","@semio-tech/puzzle-plugin:wasm"]);
 await expect(run(process.cwd(),"debug","ui")).rejects.toThrow();
});
