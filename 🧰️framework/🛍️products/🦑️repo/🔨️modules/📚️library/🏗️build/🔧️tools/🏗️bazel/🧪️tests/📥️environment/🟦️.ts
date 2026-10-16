import{test,expect}from"bun:test";
import{readFile}from"node:fs/promises";
import{resolve}from"node:path";
import ts from"typescript";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import corpus from"./🔣️.json";
import schema from"./🧬️schema/🔣️.json";
import{withBuildTestInvocationV1}from"../../../../🧪️tests/📥️invocation/🟦️.ts";
import*as routing from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

test("Bazel permanent dispatcher publishes its canonical workspace before purpose birth",async()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const source=await readFile(new URL("../../../../📜️script.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("build.ts",source,ts.ScriptTarget.Latest,true),owner=tree.statements.find((node):node is ts.ClassDeclaration=>ts.isClassDeclaration(node)&&node.name?.text==="TestScript");expect(owner).toBeDefined();
 const transformed=ts.transform(owner!,[context=>{const visit:ts.Visitor=node=>ts.isPropertyAccessExpression(node)&&ts.isMetaProperty(node.expression)&&node.name.text==="dir"?ts.factory.createStringLiteral("/owned/Source/build"):ts.isCallExpression(node)&&node.expression.kind===ts.SyntaxKind.ImportKeyword?ts.factory.createCallExpression(ts.factory.createIdentifier("imports"),undefined,node.arguments):ts.visitEachChild(node,visit,context);return node=>ts.visitNode(node,visit)as ts.ClassDeclaration;}]),database=new Database(":memory:");
 try{const printed=ts.createPrinter().printNode(ts.EmitHint.Unspecified,transformed.transformed[0]!,tree),code=ts.transpileModule(printed,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText,build=await import("../../../../📜️script.ts"),wire=routing.scriptInvocationEnvironment(original,{}).SEMIO_SCRIPT_PROCESS_INVOCATION;
 database.exec("CREATE TABLE roots(name TEXT PRIMARY KEY,root TEXT)");for(const row of corpus.cases)database.query("INSERT INTO roots VALUES(?,?)").run(row.name,row.root);
 for(const row of corpus.cases){let captured:NodeJS.ProcessEnv|undefined,budget:number|undefined,signal:AbortSignal|undefined;class Script{constructor(readonly repoRoot:string,readonly invocation:routing.ScriptInvocation){}}
 const imports=async(name:string)=>{expect(name).toContain("test-output");return{repoTestArtifactEnvironment:()=>({SEMIO_TEST_ARTIFACT_DIR:"/owned/artifacts"})};},bindings={Script,imports,bazelTestCommandsV1:build.bazelTestCommandsV1,resolve,process:{execPath:process.execPath,env:row.inheritedRoot===null?{}:{SEMIO_WORKSPACE_ROOT:row.inheritedRoot}},advanceScriptInvocation:async(invocation:routing.ScriptInvocation)=>{expect(invocation).toBe(original);routing.checkScriptInvocation(original);},scriptInvocationBudget:routing.scriptInvocationBudget,scriptInvocationEnvironment:routing.scriptInvocationEnvironment,runOwnedCommand:async(_program:string,_args:string[],root:string,_label:string,maximum:number,options:{env:NodeJS.ProcessEnv;signal:AbortSignal;onProgress:()=>Promise<void>})=>{expect(root).toBe(row.root);captured=options.env;budget=maximum;signal=options.signal;await options.onProgress();}};
 const Constructor=Function(...Object.keys(bindings),code+";return TestScript;")(...Object.values(bindings));await new Constructor(row.root,original).run(["bazel","cache"]);const expected=database.query("SELECT root FROM roots WHERE name=?").get(row.name)as{root:string};expect(captured?.SEMIO_WORKSPACE_ROOT).toBe(expected.root);expect(captured?.SEMIO_TEST_ARTIFACT_DIR).toBe("/owned/artifacts");expect(captured?.SEMIO_SCRIPT_PROCESS_INVOCATION).toBe(wire);expect(signal).toBe(original.control.signal);expect(budget).toBeLessThanOrEqual(corpus.maximumChildMilliseconds);expect(budget).toBeGreaterThan(0);routing.checkScriptInvocation(original);
 }}finally{transformed.dispose();database.close();}
 console.log("[DEBUG] Actual permanent dispatcher body preserves same wire/signal/bound and canonical workspace; Ajv and SQLite environment oracle agree");
}));
