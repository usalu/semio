import { expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { parseFeature } from "../../🥒️gherkin/🟦️.ts";
import type { AdapterContext, TestAdapter,TestCasePlan } from "../🟦️.ts";
import fixture from "../../🧪️tests/🧫️fixtures/🔌️adapter-ownership/🔣️.json" with { type: "json" };
import protocol from "../../🧬️schema/🔣️.json" with { type: "json" };
const root = resolve(import.meta.dir, "../../../../..");
const scenarioSchema = { $defs: protocol.$defs, $ref: "#/$defs/Scenario" };
const ajv = semioSchemaAjvV1({ strictRequired: false, allowUnionTypes: true });
const validScenario = ajv.compile(scenarioSchema);

test("neutral adapter invocation protocol requires explicit issued ports and owned host drainage",()=>{
 const contract=protocol.$defs.AdapterInvocationProtocol.const;expect(ajv.compile({const:contract})(JSON.parse(JSON.stringify(contract)))).toBe(true);
 const path=resolve(import.meta.dir,"../🟦️.ts"),source=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),definition=source.statements.find(row=>ts.isTypeAliasDeclaration(row)&&row.name.text==="AdapterContext");expect(definition).toBeDefined();const names:string[]=[];function visit(node:ts.Node):void{if(ts.isPropertySignature(node)&&node.name&&contract.ports.includes(node.name.getText(source))){expect(node.questionToken).toBeUndefined();names.push(node.name.getText(source));}ts.forEachChild(node,visit);}visit(definition!);expect(names).toEqual(contract.ports);
 const hostPath=resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🟦️.ts"),host=readFileSync(hostPath,"utf8"),execution=readFileSync(resolve(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts"),"utf8");const hostSyntax=ts.createSourceFile(hostPath,host,ts.ScriptTarget.Latest,true);let retainedResults=0;function inspectHost(node:ts.Node):void{if(ts.isCallExpression(node)&&node.expression.getText(hostSyntax)==="resultFor"&&node.arguments[4]?.getText(hostSyntax)==='"passed"'){let parent:ts.Node|undefined=node.parent;while(parent&&!ts.isFunctionDeclaration(parent)){if(ts.isCallExpression(parent)&&parent.expression.getText(hostSyntax)==="withRequestedIssuedResourceControlV1"){retainedResults++;break;}parent=parent.parent;}}ts.forEachChild(node,inspectHost);}inspectHost(hostSyntax);expect(retainedResults).toBe(1);expect(host).toContain("receiveScriptProcessInvocation(process.env");expect(host).toContain("withRequestedIssuedResourceControlV1(");expect(host).not.toContain("createScriptProcessEnvelope(");expect(execution).toContain("captureOwnedCommandResultV1(");expect(execution).toContain("scriptInvocationEnvironment(invocation,");expect(execution).not.toContain("runProbe(");
 console.log("[DEBUG] Neutral adapter protocol and actual host Source require carried issued ports, parent envelope and accepted drainage");
});

test("neutral adapter invocation retains genuine callback children before hashing results and refuses copied expiry before effects",async()=>{
 const{withBuildTestInvocationV1}=await import("../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧪️tests/📥️invocation/🟦️.ts"),issued=await import("../../../🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),{Database}=await import("bun:sqlite"),{mkdir,mkdtemp,rm,writeFile}=await import("node:fs/promises"),{join}=await import("node:path"),{createHash}=await import("node:crypto"),{makeAdapterContext}=await import("../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/🔌️adapter-context/🟦️.ts"),{runAdapter}=await import("../../../../🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🟦️.ts");
 await withBuildTestInvocationV1(async invocation=>{const phase=issued.createIssuedResourceControlV1(invocation,"test:adapter-custody",60000),generated=join(root,".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️13/MODULAR-MONOREPO-BUILDS-AND-SHARED-CACHES/🗑️generated");await mkdir(generated,{recursive:true});const area=await mkdtemp(join(generated,"adapter-custody-")),db=new Database(join(area,"order.sqlite"));try{db.exec("create table events(position integer primary key,name text unique)");const scenario={id:"retained",name:"Retained",level:"quick"as const,mode:"conformance"as const,tags:[],steps:[],line:1},plan:TestCasePlan={schemaVersion:2,owner:"fixture",case:"adapter-custody",featurePath:"fixture.feature",featureHash:"0".repeat(64),featureName:"Fixture",description:"",capability:"fixture",target:null,mutationManifestDigest:null,oracle:null,noOracleDecision:"fixture",comparison:"exact",oracleInput:null,comparisonPipeline:null,toleranceProfile:null,background:[],scenarios:[scenario],adapters:{typescript:"fixture.ts"},inputs:[],testEvidence:[],workDir:join(area,"work"),resultsPath:join(area,"results.jsonl"),outputDir:join(area,"output"),artifactDir:join(area,"artifacts"),level:"quick",role:"subject",implementation:"typescript",platform:"darwin-arm64"};
 const refused={...plan,workDir:join(area,"refused"),artifactDir:join(area,"refused-artifacts")};expect(()=>makeAdapterContext(root,refused,scenario,"subject",invocation,{...phase})).toThrow("issuance");const expired=issued.createRequestedIssuedResourceControlV1(phase,1);await new Promise<void>(done=>setTimeout(done,5));expect(()=>makeAdapterContext(root,refused,scenario,"subject",invocation,expired)).toThrow("deadline");expect(existsSync(refused.workDir)).toBe(false);expect(existsSync(refused.artifactDir)).toBe(false);
 let accepted:AdapterContext|undefined;const adapter:TestAdapter={implementation:"typescript",scenarios:{retained:{subject:async ctx=>{accepted=ctx;expect(ctx.invocation).toBe(invocation);issued.requireIssuedResourceControlInvocationV1(ctx.phaseControl,invocation);issued.requireIssuedResourceControlRefinementV1(phase,ctx.phaseControl);db.query("insert into events values(?,?)").run(1,"callback");const path=ctx.artifact("proof","child.txt");issued.withIssuedResourceOperationV1(ctx.phaseControl,async()=>{await new Promise<void>(done=>setTimeout(done,15));issued.requireIssuedResourceControlInvocationV1(ctx.phaseControl,invocation);await writeFile(path,"accepted child",{flag:"wx"});db.query("insert into events values(?,?)").run(2,"child");});return{raw:"body",projection:{retained:true},artifacts:[{role:"proof",path,mediaType:"text/plain"}]};}}}};const results=await runAdapter(root,plan,adapter,invocation,phase);db.query("insert into events values(?,?)").run(3,"result");expect(results).toHaveLength(1);expect(results[0]?.status).toBe("passed");expect(results[0]?.artifacts?.[0]?.sha256).toBe("sha256:"+createHash("sha256").update("accepted child").digest("hex"));expect(results[0]?.artifacts?.[0]?.bytes).toBe(Buffer.byteLength("accepted child"));expect(db.query("select name from events order by position").all()).toEqual([{name:"callback"},{name:"child"},{name:"result"}]);expect(accepted).toBeDefined();expect(()=>issued.requireIssuedResourceControlInvocationV1(accepted!.phaseControl,invocation)).toThrow("Live Original");expect(()=>accepted!.artifact("late","late.txt")).toThrow("Live Original");expect(existsSync(join(plan.artifactDir,scenario.id,"late"))).toBe(false);expect(Object.isFrozen(accepted)).toBe(true);issued.requireIssuedResourceControlInvocationV1(phase,invocation);
 const childRoot=join(area,"host"),adapterPath=join(area,"📜️script.ts"),planPath=join(area,"plan.json"),outPath=join(area,"host-results.jsonl"),childPlan={...plan,workDir:join(childRoot,"work"),outputDir:join(childRoot,"output"),artifactDir:join(childRoot,"artifacts"),resultsPath:outPath},routing=join(root,"🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 await writeFile(planPath,JSON.stringify(childPlan),{flag:"wx"});await writeFile(adapterPath,[`import{withIssuedResourceOperationV1,requireIssuedResourceControlInvocationV1}from${JSON.stringify(routing)};`,`import{writeFile}from"node:fs/promises";`,`export default{implementation:"typescript",scenarios:{retained:{subject:async ctx=>{requireIssuedResourceControlInvocationV1(ctx.phaseControl,ctx.invocation);const path=ctx.artifact("proof","child.txt");withIssuedResourceOperationV1(ctx.phaseControl,async()=>{await new Promise(done=>setTimeout(done,15));requireIssuedResourceControlInvocationV1(ctx.phaseControl,ctx.invocation);await writeFile(path,"parent envelope child",{flag:"wx"});console.log("[DEBUG] Actual parent-envelope adapter child settled under received Original");});return{projection:{retained:true},artifacts:[{role:"proof",path,mediaType:"text/plain"}]};}}}};`].join("\n"),{flag:"wx"});
 const{captureOwnedCommandResultV1}=await import("@semio-tech/process/owned-execution"),capture=await issued.withIssuedResourceOperationV1(phase,()=>captureOwnedCommandResultV1(process.execPath,[join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🖥️host/🟦️.ts"),"--plan",planPath,"--out",outPath,"--adapter",adapterPath],root,{control:phase,env:issued.scriptInvocationEnvironment(invocation,{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0",NODE_DISABLE_COMPILE_CACHE:"1"}),output:{maximumBytes:65536,maximumLines:256}}));expect(capture.status,capture.stderr).toBe(0);expect(capture.signal).toBe(null);expect(capture.stdout).toContain("[DEBUG] Actual parent-envelope adapter child settled");const childResult=JSON.parse(readFileSync(outPath,"utf8").trim())as{status:string;artifacts:{sha256:string}[]};expect(childResult.status).toBe("passed");expect(childResult.artifacts[0]?.sha256).toBe("sha256:"+createHash("sha256").update("parent envelope child").digest("hex"));
 console.log("[DEBUG] Actual adapter callback retains detached Core child through result hashing; SQLite orders callback/child/result and Node hashes actual bytes; copied/expired construction makes no directory");
 }finally{db.close();await rm(area,{recursive:true,force:true});}});
},15_000);

test("test evidence config refers to plain examples without a fixture schema", () => {
  for (const name of fixture.evidence.forbiddenDefinitions) expect(Object.hasOwn(protocol.$defs, name), name).toBe(false);
  const schema = { $defs: protocol.$defs, $ref: "#/$defs/TestEvidence" }, independent = ajv.compile(schema);
  for (const row of fixture.evidence.cases) {
    expect(independent(row.data), JSON.stringify(independent.errors)).toBe(row.valid);
    expect(validateJsonSchemaSubset(schema, row.data).length === 0).toBe(row.valid);
  }
  console.log("[DEBUG] testing evidence: plain examples agree with the independent schema oracle");
});

test("portable neutral ownership corpus agrees with the independent schema oracle", () => {
  expect(protocol.$id).toBe("https://json.schemas.assets.semio-tech.com/framework/test/schema.json");
  expect(protocol["x-semio"].owner).toBe("🧰️framework/🔨️modules/🧪️test");
  for (const file of fixture.removedFiles) expect(existsSync(resolve(root, file)), file).toBe(false);
  for (const row of fixture.removedExports) {
    const source = readFileSync(resolve(root, row.source), "utf8"), ast = ts.createSourceFile(row.source, source, ts.ScriptTarget.Latest, true);
    const exports = new Set<string>();
    for (const node of ast.statements) {
      if (ts.isExportDeclaration(node) && node.exportClause && ts.isNamedExports(node.exportClause)) for (const item of node.exportClause.elements) exports.add(item.name.text);
      if (!(ts.canHaveModifiers(node) && ts.getModifiers(node)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword))) continue;
      if (ts.isVariableStatement(node)) {
        for (const declaration of node.declarationList.declarations) if (ts.isIdentifier(declaration.name)) exports.add(declaration.name.text);
      } else if ("name" in node && node.name && ts.isIdentifier(node.name as ts.Node)) exports.add((node.name as ts.Identifier).text);
    }
    for (const name of row.names) expect(exports.has(name), name).toBe(false);
  }
});

for (const row of fixture.cases) test(`real feature vectors preserve neutral adapter behavior: ${row.directory}`, async () => {
  const source = readFileSync(resolve(root, row.directory, "🥒️.feature"), "utf8"), feature = parseFeature(source);
  expect(feature.errors).toEqual([]);
  expect(feature.scenarios.map((scenario) => scenario.id)).toEqual(row.scenarios);
  const adapter = (await import(pathToFileURL(resolve(root, row.directory, "🟦️.ts")).href)).default as TestAdapter;
  expect(adapter.implementation).toBe("typescript");
  expect(Object.keys(adapter.scenarios)).toEqual(row.scenarios);
  for (const scenario of feature.scenarios) {
    const value = JSON.parse(JSON.stringify(scenario));
    expect(validScenario(value), JSON.stringify(ajv.errors)).toBe(true);
    expect(validateJsonSchemaSubset(scenarioSchema, value)).toEqual([]);
    for (const invalid of [{ ...value, level: "unbounded" }, { ...value, mode: "unspecified" }]) {
      expect(validScenario(invalid)).toBe(false);
      expect(validateJsonSchemaSubset(scenarioSchema, invalid).length).toBeGreaterThan(0);
    }
    const unsupported = (): never => { throw Error("unexpected fixture or coordinator access by vector-only adapter"); };
    const context = { scenario, repoRoot: root, workDir: root, artifactDir: root, seed: "0", role: "subject", input: unsupported, inputBytes: unsupported, copyInput: unsupported, subjectRawBytes: unsupported, artifact: unsupported, row: unsupported, get plan(): never { return unsupported(); },get invocation():never{return unsupported();},get phaseControl():never{return unsupported();} } satisfies AdapterContext;
    const handler = adapter.scenarios[scenario.id]!;
    expect(typeof handler.subject).toBe("function");
    const subject = await handler.subject!(context);
    expect(subject.projection).toBeDefined();
    if ("oracle" in row) {
      expect(typeof handler.oracle).toBe("function");
      const oracle = await handler.oracle!(Object.assign(Object.create(context), { role: "oracle" }));
      expect(subject.projection).toEqual(oracle.projection);
    } else expect(handler.oracle).toBeUndefined();
  }
});

test("runtime entrypoints load example-backed factories only through explicit test calls", () => {
  for (const row of fixture.runtimeBoundaries) {
    const source = readFileSync(resolve(root, row.source), "utf8"), ast = ts.createSourceFile(row.source, source, ts.ScriptTarget.Latest, true);
    const ownsTest = (path: string) => path.replaceAll("\uFE0F", "").includes("/🧪tests/");
    const native = [...source.matchAll(/^import\s+(?!type\b)[^;\n]+from\s+["']([^"']+)["'];/gm)].filter((match) => ownsTest(match[1]!)).length;
    const independent = ast.statements.filter(ts.isImportDeclaration).filter((node) => !node.importClause?.isTypeOnly && ownsTest((node.moduleSpecifier as ts.StringLiteral).text)).length;
    expect(native, row.source).toBe(row.staticTestImports);
    expect(independent, row.source).toBe(native);
  }
  console.log("[DEBUG] runtime entrypoints: example factories stay outside module initialization");
});
