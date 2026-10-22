import compilerSchema from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧬️schema/🔣️.json";
/** 🪝️ Contract: task runs queued behind one dependency synchronization install at most once (each re-checks readiness under the exclusive hold it finally obtains) and a shared walk's evidence is adopted only when the exclusive writer proved it. The scenarios live in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {adoptedCandidateV1,installUnlessReadyV1} from "../🟦️.ts";

test("bootstrap install fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

test("a shared walk's evidence is adopted only after the exclusive writer published or found the receipt",()=>{
 for(const row of fixture.adoption)expect(adoptedCandidateV1(row.status as never),row.status).toBe(row.adopted);
});

test("runs queued for the exclusive hold install once and repeat only an installation that failed to ready the environment",async()=>{
 for(const scenario of fixture.installs){
  let ready=scenario.initiallyReady,prepares=0,tail=Promise.resolve();
  const store={} as never,control={} as never,recheck=scenario.recheck?async()=>ready:undefined,prepare=async()=>{prepares++;await new Promise(accept=>setTimeout(accept,1));ready=scenario.readyAfterInstall;};
  const runs=Array.from({length:scenario.runs},()=>{const turn=tail.then(()=>installUnlessReadyV1(store,control,recheck,prepare));tail=turn.then(()=>undefined);return turn;});
  const results=await Promise.all(runs);
  expect(prepares,scenario.id).toBe(scenario.expectedPrepares);
  expect(results.filter(Boolean).length,scenario.id).toBe(scenario.expectedPrepares);
 }
});

import ts from "typescript";
import {Database} from "bun:sqlite";
import {relative} from "node:path";
import {fileURLToPath} from "node:url";
import declarationSchema from "../🌱️sources/🧬️schema/🔣️.json";
import {physicalSourceOriginV1} from "../../📥️inputs/🔏️revision/🟨️.mjs";
import {captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1} from "../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

/** 🌱️ Checks real compiler edges and callable dispatch against closed publisher DATA and SQLite. */
test("bootstrap publishers remain executable captured Source with closed DATA",()=>withBuildTestInvocationV1(async original=>{
 const root=process.cwd(),origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL),entry=relative(root,fileURLToPath(new URL("../🟦️.ts",origin))).replaceAll("\\","/"),control=createIssuedResourceControlV1(original,"test:bootstrap-source-admission",50000),captured=await captureNxAuthorizerSourcesV1(root,entry,control.signal),bytes=nxAuthorizerCapturedBytesV1(captured),imports=nxAuthorizerCapturedImportsV1(root,captured).get(entry)!;
 const declarationPath=imports["./🌱️sources/🔣️.json"];expect(declarationPath).toBeDefined();expect(bytes.has(declarationPath!)).toBe(true);const declaration=JSON.parse(Buffer.from(bytes.get(declarationPath!)!).toString("utf8")),ajv=new Ajv({strict:true}),tree=ts.createSourceFile("bootstrap.ts",Buffer.from(bytes.get(entry)!).toString("utf8"),ts.ScriptTarget.Latest,true),imported=tree.statements.filter(ts.isImportDeclaration),db=new Database(":memory:");
 try{expect(ajv.validate(schema,fixture)).toBe(true);expect(ajv.validate(declarationSchema,declaration)).toBe(true);for(const row of declaration.sources)expect(ajv.validate(declarationSchema,{sources:[row,{...row,requiredBy:row.requiredBy+" duplicate"}]}),row.id).toBe(false);const projected=db.query("select json_extract(value,'$.id') id,json_extract(value,'$.module') module,json_extract(value,'$.export') export from json_each(?1,'$.sources') order by id").all(JSON.stringify(declaration));expect(projected).toEqual([...fixture.publishers].sort((a,b)=>a.id.localeCompare(b.id)));for(const row of fixture.publishers){const edge=imported.find(node=>ts.isStringLiteral(node.moduleSpecifier)&&imports[node.moduleSpecifier.text]===row.module),bindings=edge?.importClause?.namedBindings;expect(bindings&&ts.isNamedImports(bindings)&&bindings.elements.some(node=>(node.propertyName??node.name).text===row.export&&node.name.text===row.export),row.id).toBe(true);expect(captured.manifest.sources.some(node=>node.path===row.module),row.id).toBe(true);}
 const catalog=fixture.catalog;expect(compilerSchema.$defs.RuntimeDataCompanions.const.find(row=>row.owner===catalog.owner)?.paths).toContain(catalog.path);expect(captured.manifest.sources.some(node=>node.path===catalog.owner)).toBe(true);expect(captured.manifest.sources.some(node=>node.path===catalog.path)).toBe(true);expect(bytes.has(catalog.path)).toBe(true);const reader=ts.createSourceFile("ui-source.ts",Buffer.from(bytes.get(catalog.owner)!).toString("utf8"),ts.ScriptTarget.Latest,true),read=reader.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="readUiAxes");if(!read)throw Error("Actual captured UI catalog reader required");const catalogReads:ts.CallExpression[]=[];const inspect=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="readFileSync")catalogReads.push(node);ts.forEachChild(node,inspect);};inspect(read);expect(catalogReads).toHaveLength(1);const catalogPath=catalogReads[0]!.arguments[0];expect(catalogPath&&ts.isCallExpression(catalogPath)&&catalogPath.expression.getText(reader)==="join"&&catalogPath.arguments[0]?.getText(reader)==="repoRoot"&&catalogPath.arguments[1]&&ts.isStringLiteral(catalogPath.arguments[1])&&catalogPath.arguments[1].text===catalog.path).toBe(true);expect(catalogReads[0]!.arguments[1]?.getText(reader)).toBe('"utf8"');
 const program=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="publishBuildBootstrapSourcesV1");expect(program).toBeDefined();const variables:ts.VariableDeclaration[]=[],calls:ts.CallExpression[]=[];const visit=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node))variables.push(node);if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);};visit(program!);const registry=variables.find(node=>ts.isIdentifier(node.name)&&node.name.text==="publishers")?.initializer;expect(registry&&ts.isObjectLiteralExpression(registry)).toBe(true);if(!registry||!ts.isObjectLiteralExpression(registry))throw Error("Actual callable bootstrap publisher registry required");expect(registry.properties.map(node=>{if(!ts.isShorthandPropertyAssignment(node))throw Error("Imported bootstrap callable required");return node.name.text;}).sort()).toEqual(fixture.publishers.map(row=>row.export).sort());expect(calls.some(node=>node.expression.getText(tree)==="Object.hasOwn"&&node.arguments.map(argument=>argument.getText(tree)).join("|")==="publishers|source.export")).toBe(true);const dispatch=calls.filter(node=>ts.isElementAccessExpression(node.expression)&&node.expression.expression.getText(tree)==="publishers");expect(dispatch).toHaveLength(1);expect(dispatch[0]!.expression.getText(tree)).toBe("publishers[source.export as keyof typeof publishers]");expect(dispatch[0]!.arguments.map(node=>node.getText(tree))).toEqual(["root"]);expect(calls.some(node=>ts.isIdentifier(node.expression)&&node.expression.text==="createRequire")).toBe(false);expect(program!.getText(tree)).toContain("declaration.sources");
 }finally{db.close();}
}),60000);
