/** 🚀️ Build bootstrap and the dependency CLI enter online preparation through their defining modules. */
import assert from "node:assert/strict";
import {dirname,join,resolve} from "node:path";
import {expect,test} from "bun:test";
import {createRequire} from "node:module";
import {readFileSync} from "node:fs";
import {build} from "esbuild";
import fixture from "../../🧫️fixtures/🕸️graph/🔣️.json";
import dependencyInvocationSchema from "../../../🧬️schema/📥️dependencies.json";
import {createNativeBuildCallerCapabilitiesV1} from "../../../📥️caller/🟦️.ts";
import {readBuildDependencyInvocationV1} from "../../../🧭️invocation/🟦️.ts";
import {createScriptProcessEnvelope} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import "../🧰️support/🟦️.ts";

const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
test("build bootstrap receives the defining dependency module while its executable preserves the original header",async()=>{
  const row=fixture.bootstrap.moduleReceiving,library=resolve(import.meta.dir,"../../../.."),owner=join(library,row.owner),router=join(library,row.router),commandModule=join(library,row.commandModule),parser=require("typescript"),body=readFileSync(owner,"utf8"),commands=readFileSync(router,"utf8"),syntax=parser.createSourceFile(owner,body,parser.ScriptTarget.Latest,true),commandSyntax=parser.createSourceFile(router,commands,parser.ScriptTarget.Latest,true),moduleSyntax=parser.createSourceFile(commandModule,readFileSync(commandModule,"utf8"),parser.ScriptTarget.Latest,true),functions=(source:any)=>source.statements.filter((node:any)=>parser.isFunctionDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text);
  expect(commands.split("\n")[0]).toBe(row.scriptHeader);expect(body.startsWith("#!")).toBe(false);expect(syntax.parseDiagnostics).toEqual([]);expect(commandSyntax.parseDiagnostics).toEqual([]);for(const name of row.operationExports)expect(functions(syntax).filter((value:string)=>value===name)).toHaveLength(1);expect(functions(commandSyntax)).toEqual([]);expect(commandSyntax.statements.filter((node:any)=>parser.isClassDeclaration(node)).map((node:any)=>node.name.text)).toEqual([]);expect(moduleSyntax.statements.filter((node:any)=>parser.isClassDeclaration(node)&&node.modifiers?.some((modifier:any)=>modifier.kind===parser.SyntaxKind.ExportKeyword)).map((node:any)=>node.name.text)).toEqual(row.commands);for(const name of row.commands)expect(commands).toContain(name);
  const bootstrap=readFileSync(resolve(import.meta.dir,"../../../🚀️bootstrap/🟦️.ts"),"utf8"),imports=parser.preProcessFile(bootstrap,true,true).importedFiles.map((value:any)=>value.fileName);expect(imports).toContain("../../"+row.owner);expect(imports).not.toContain("../../"+row.router);
  const oracle=await build({stdin:{contents:body,sourcefile:owner,resolveDir:dirname(owner),loader:"ts"},bundle:false,write:false,format:"esm",platform:"node",logLevel:"silent"});expect(oracle.errors).toEqual([]);expect(oracle.outputFiles![0]!.text.startsWith("#!")).toBe(false);expect(new Bun.Transpiler({loader:"ts"}).transformSync(body).startsWith("#!")).toBe(false);
  const received=await import(owner);for(const name of row.operationExports)expect(typeof received[name]).toBe("function");for(const operation of ["sync","lock"] as const)expect(received.javascriptDependencyProducerArgumentsV1(operation,"/cache")).toEqual(["install",operation==="sync"?"--frozen-lockfile":"--lockfile-only","--ignore-scripts","--linker=isolated","--backend="+({darwin:"clonefile",linux:"copyfile",win32:"copyfile"}as Record<string,string>)[process.platform],"--cache-dir","/cache"]);
});
test("original direct dependency CLI admits explicit whole or scoped selections and refuses installer overrides",()=>{
  const oracle=new(require("ajv"))({strict:true}).compile(dependencyInvocationSchema);
  for(const row of fixture.dependencyInvocations.cases){const selected=readBuildDependencyInvocationV1(row.args);assert.deepEqual(selected,row.value);expect(oracle(selected),row.name).toBe(true);expect(selected.selection.kind==="all"?["--all"]:["--scope",selected.selection.owner]).toEqual(row.forwarded);}
  for(const args of fixture.dependencyInvocations.refused)expect(()=>readBuildDependencyInvocationV1(args)).toThrow("Expected repo dependencies sync|lock --all|--scope <owner>");
  for(const value of [{mode:"unknown",selection:{kind:"all"}},{mode:"sync",selection:{kind:"all",owner:"."}},{mode:"lock",selection:{kind:"scope"}},{mode:"sync",selection:{kind:"scope",owner:""}},{mode:"sync",selection:{kind:"scope",owner:"\0"}}])expect(oracle(value)).toBe(false);
});
test("registered JavaScript dependency initial callers enter direct finite online preparation before graph bootstrap",async()=>{
  const parser=require("typescript"),workspace=process.cwd(),source=readFileSync(resolve(import.meta.dir,"../../../📜️script.ts"),"utf8"),syntax=parser.createSourceFile("build.ts",source,parser.ScriptTarget.Latest,true),definition=syntax.statements.find((node:any)=>parser.isFunctionDeclaration(node)&&node.name?.text==="runOwnedBuildInLeasesV1"),body=definition.getText(syntax);expect(body.indexOf('command==="dependencies"')).toBeLessThan(body.indexOf("await ensureBuildEnvironmentV1"));expect(body).toContain("readBuildDependencyInvocationV1(resolved.args)");
  
  const dashboard=JSON.parse(readFileSync(join(workspace,"📋️project.json"),"utf8")).metadata.semio.dashboard;expect(dashboard.tools.find((tool:any)=>tool.id==="run-script").command.slice(0,3)).toEqual(["bun","repo","exec"]);
  for(const expected of fixture.dependencyInvocations.launches){const args=expected.command.split(" ").slice(2),capabilities=createNativeBuildCallerCapabilitiesV1(workspace,args,"nx",{});expect(capabilities.native.network.offline).toBe(false);expect(capabilities.arguments).toEqual(["repo",...args]);const envelope=createScriptProcessEnvelope(expected.policy as Parameters<typeof createScriptProcessEnvelope>[0],capabilities,Date.now());expect(envelope.deadlineEpochMilliseconds).not.toBeNull();expect(envelope.capabilities).toEqual(capabilities);}
});
