import {fileURLToPath as physicalSchemaSourcePathV1} from "node:url";
import {compileSchemaScopeExport as physicalSchemaScopeV1} from "./../../../../../../🔨️modules/🧬️schema/📦️packages/🟦️typescript/🔮️oracles/✅️validator/🟦️.ts";
import {findWorkspaceRoot as physicalSchemaWorkspaceRootV1} from "./../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
const physicalProjectOracleV1=()=>physicalSchemaScopeV1(physicalSchemaWorkspaceRootV1(import.meta.dir),physicalSchemaSourcePathV1(new URL("./../../🏗️build/🧬️schema/📋️project/🔣️.json",import.meta.url)),null,[physicalSchemaSourcePathV1(new URL("./../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🧬️schema/🔣️.json",import.meta.url))]);
import { expect, test } from "bun:test";
import {testRipgrepV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Ajv from "ajv";
import Ajv2020 from "ajv";
import ts from "typescript";
import {parseArgsStringToArgv} from "string-argv";
import {createRequire} from "node:module";
import { parseBuildInvocationV1 } from "../../🏗️build/🧭️invocation/🟦️.ts";
import { validateBuildTaskArgumentsV1 } from "../../🏗️build/🧭️selection/🟦️.ts";
import { readFileSync,existsSync } from "node:fs";
import {Database} from "bun:sqlite";
import {readBuildDependencyInvocationV1} from "../../🏗️build/🧭️invocation/🟦️.ts";
import {javascriptDependencyProducerArgumentsV1} from "../../⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts";
import { dirname, join, resolve, posix, win32 } from "node:path";
import { spawnSync } from "node:child_process";
import { literalCommandTokensV1 } from "../../🏗️build/📤️arguments/🟨️.mjs";
import { findWorkspaceRoot } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import fixture from "../🧫️fixtures/🔧️build-surfaces/🔣️.json";
import schema from "../../🧬️schema/🔧️build-surfaces/🔣️.json";
import projectSchema from "../../🏗️build/🧬️schema/📋️project/🔣️.json";
import graphSchema from "../../🏗️build/🧬️schema/🔣️.json";
import {neutralBuildTargetV1,resolveBuildTargetV1} from "../../🏗️build/🕸️graph/🟦️.ts";
import {planBazelTasksV1} from "../../🏗️build/🔧️tools/🏗️bazel/📜️script.ts";
import { nativeHostContractLaws } from "../../🎮️playground/🖥️native-host/🧪️tests/🟦️.ts";
const root = findWorkspaceRoot(import.meta.dir);
const read = (path: string): string => readFileSync(join(root, path), "utf8");

test("package wrappers route to the repository root with independent argv and path oracles", () => {
  for (const row of fixture.packageRouting.cases) {
    const argv=literalCommandTokensV1(row.command), paths=row.platform==="win32"?win32:posix;
    expect(argv).toEqual(parseArgsStringToArgv(row.command));
    expect(argv.slice(0,2)).toEqual(["repo","--cwd"]);
    expect(paths.resolve(row.root,row.package,argv[2]!)).toBe(paths.resolve(row.root));
    expect(argv[3]).toBe("repo");
    expect(argv.slice(4)).toEqual(row.arguments);
  }
  const inventory=spawnSync("git",["-c","core.fsmonitor=false","ls-files","-z","--cached","--","**/package.json","package.json"],{cwd:root,encoding:"utf8",timeout:10000,maxBuffer:16*1024*1024});
  expect(inventory.status,inventory.stderr).toBe(0);
  let wrappers=0;
  for(const path of new Set(inventory.stdout.split("\0").filter((entry)=>entry!==""&&!entry.startsWith(".🧬semio/")&&existsSync(join(root,entry))))) {
    const manifest=JSON.parse(read(path));
    for(const [name,command] of Object.entries(manifest.scripts??{})) {
      if(typeof command!=="string"||!/^repo (?:repo |--cwd |run )/u.test(command))continue;
      const argv=literalCommandTokensV1(command);
      expect(argv[1]==="nx"||argv[3]==="nx",`${path}:${name}`).toBe(false);
      if(argv[1]==="run"){expect(argv,`${path}:${name}`).toEqual(parseArgsStringToArgv(command));wrappers++;continue;}
      if(argv[1]!=="repo"&&argv[3]!=="repo")continue;
      expect(argv,`${path}:${name}`).toEqual(parseArgsStringToArgv(command));
      if(path==="package.json"){expect(argv[1]).toBe("repo");continue;}
      expect(argv.slice(0,2),`${path}:${name}`).toEqual(["repo","--cwd"]);
      expect(resolve(root,dirname(path),argv[2]!),`${path}:${name}`).toBe(root);
      expect(argv[2]).not.toContain("\\");
      expect(argv[3]).toBe("repo");
      wrappers++;
    }
  }
  expect(wrappers).toBeGreaterThan(600);
  const source=ts.createSourceFile("caller.ts",read(fixture.packageRouting.sourceCaller),ts.ScriptTarget.Latest,true),calls:ts.CallExpression[]=[];
  const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&node.expression.getText(source)==="runCmd"&&node.arguments[0]?.getText(source)==='"repo"')calls.push(node);ts.forEachChild(node,visit);};visit(source);
  expect(calls.length).toBeGreaterThan(0);
  for(const call of calls){const options=call.arguments[2];expect(options&&ts.isObjectLiteralExpression(options)).toBe(true);const cwd=(options as ts.ObjectLiteralExpression).properties.find(property=>ts.isPropertyAssignment(property)&&property.name.getText(source)==="cwd") as ts.PropertyAssignment;expect(cwd.initializer.getText(source)).toBe("this.repoRoot");}
});

test("portable developer entry points agree with the independent schema oracle", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  const manifest = JSON.parse(read("package.json"));
  for (const [name, expected] of Object.entries(fixture.scripts)) expect(manifest.scripts[name], name).toBe(expected);
  expect(manifest.scripts.nx).toBeUndefined();
  expect(manifest.nx).toBeUndefined();
  const config = ts.parseConfigFileTextToJson("devcontainer.json", read(".devcontainer/devcontainer.json"));
  expect(config.error).toBeUndefined();
  expect(config.config.postCreateCommand).toEqual(fixture.postCreateCommand);
});

test("workspace context uses the neutral execution identity", () => {
  for (const row of fixture.contexts) expect(getWorkspaceRoot(row.environment)).toBe(resolve(row.expected));
});

test("root orchestration, editor settings and CI use neutral build commands", () => {
  const source = read("📜️script.ts");
  expect(/\["nx",\s*"run(?:-many)?"/u.test(source)).toBe(false);
  expect(source.includes("NX_TASK_TARGET_PROJECT")).toBe(false);
  expect(JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🗂️registry/🔣️.json")).commands.some((row: { id: string }) => row.id === "repo")).toBe(true);
  for (const path of [".devcontainer/devcontainer.json", ".devcontainer/Dockerfile"]) expect(read(path), path).not.toMatch(/\bNX_[A-Z_]+/u);
  expect(read(".github/workflows/architecture-course.yml").includes(["bun", "nx"].join(" "))).toBe(false);
});

test("project declarations expose neutral executors through the owned schema", () => {
  const validate = physicalProjectOracleV1();
  expect(validate({ name: "sample", targets: { build: { executor: "repo:script", options: { command: "repo ./📜️script.ts build" } }, noop: { executor: "repo:noop" } } })).toBe(true);
  expect(validate({ name: "sample", targets: { build: { executor: "nx:run-commands" } } })).toBe(false);
  const project = JSON.parse(read("📋️project.json"));
  expect(validate(project), JSON.stringify(validate.errors)).toBe(true);
  expect(project.$schema).toContain("🏗️build/🧬️schema/📋️project/🔣️.json");
});

test("native executable producers admit portable project declarations", () => nativeHostContractLaws(root));

/** 📜️ Journal DATA owns its one immutable contract without a redundant runtime facade. */
test("native journal DATA owner remains complete within neutral Source closures",async()=>{const policy=fixture.journalDataOwner,owner=await import(join(root,policy.owner))as Record<string,unknown>,db=new Database(":memory:");try{for(const name of policy.exports)expect(owner[name],name).toBeDefined();expect(existsSync(join(root,policy.retired))).toBe(false);const contract=owner.nativeStorageSchemaV1 as any,admit=owner.readNativeStorageDataV1 as((definition:string,value:unknown)=>unknown),event={type:"committed",id:"f".repeat(32)};expect(Object.isFrozen(contract)).toBe(true);expect(Object.isFrozen(contract.definitions.Event)).toBe(true);expect(admit("Event",event)).toBe(event);expect(new Ajv({strict:false}).validate({...contract.definitions.Event,definitions:contract.definitions},event)).toBe(true);expect(()=>admit("Event",{type:"unknown"})).toThrow();const tree=ts.createSourceFile(policy.owner,read(policy.owner),ts.ScriptTarget.Latest,true),exports=tree.statements.flatMap(node=>ts.isFunctionDeclaration(node)&&node.modifiers?.some(token=>token.kind===ts.SyntaxKind.ExportKeyword)?[node.name!.text]:ts.isVariableStatement(node)&&node.modifiers?.some(token=>token.kind===ts.SyntaxKind.ExportKeyword)?node.declarationList.declarations.map(row=>row.name.getText(tree)):[]);for(const name of policy.exports)expect(db.query("select exists(select 1 from json_each(?1) where value=?2) present").get(JSON.stringify(exports),name)).toEqual({present:1});expect(read(policy.owner)).not.toContain(policy.retired.split("/").slice(-2).join("/"));}finally{db.close();}});

/** 🧾️ Every authored target normalizes through the first-party graph owner before backend planning. */
test("all authored executor omissions preserve neutral execution through the actual Bazel planner",()=>{
 const db=new Database(":memory:"),admit=new Ajv2020({strict:false}).compile(graphSchema),check=(raw:Record<string,any>,identity:string)=>{const target=neutralBuildTargetV1(raw,identity),graph={version:1 as const,projects:{audit:{name:"audit",root:".",targets:{task:{...target,dependsOn:[]}}}},dependencies:[]};expect(admit(graph),identity+":"+JSON.stringify(admit.errors)).toBe(true);const oracle=db.query("select json_patch(json_object('kind',case when json_extract(?1,'$.executor')='repo:noop' then 'noop' else 'command' end),coalesce(json_extract(?1,'$.options'),'{}')) value").get(JSON.stringify(raw))as{value:string},expected=JSON.parse(oracle.value);if(raw.command!==undefined)expected.command=raw.command;expect(target.execution,identity).toEqual(expected);const plan=planBazelTasksV1(graph,{tasks:["audit:task"],args:[]}),actual=plan.tasks[plan.selected[0]!]!;expect(plan.selected).toHaveLength(1);const mode=actual.configuration,modeOnly=mode!==undefined&&(mode==="prototype"||mode==="release")&&Object.keys(target.configurations??{}).length===0;expect(actual.definition.execution,identity).toEqual(modeOnly?{...target.execution,env:{...target.execution.env,SEMIO_BUILD_MODE:mode}}:resolveBuildTargetV1(target,mode).execution);expect("executor"in actual.definition,identity).toBe(false);return target;};let projects=0,tasks=0,implicit=0;try{for(const row of fixture.implicitExecutors){if(row.target.executor===undefined)implicit++;if(row.accepted)check(row.target,row.name);else expect(()=>check(row.target,row.name)).toThrow();}const inventory=spawnSync(testRipgrepV1()??"rg",["--files","--hidden","--null","-g","📋️project.json","-g","!node_modules","-g","!.git","-g","!.cache","-g","!🗑️generated"],{cwd:root,encoding:"utf8",timeout:10000,maxBuffer:16*1024*1024});expect(inventory.status,inventory.stderr).toBe(0);for(const path of new Set(inventory.stdout.split("\0").filter(path=>path&&!path.includes("/🎫️tickets/")))){const value=JSON.parse(read(path));projects++;for(const[name,raw]of Object.entries(value.targets??{})){const target=raw as Record<string,any>;check(target,path+":"+name);tasks++;}}expect(implicit).toBeGreaterThan(0);}finally{db.close();}
});

test("dashboard entry points retain localized build and source controls",()=>{const dashboard=JSON.parse(read("📋️project.json")).metadata.semio.dashboard;for(const id of ["run-targets","run-script"]){const tool=dashboard.tools.find((tool:any)=>tool.id===id);expect(tool,id).toBeDefined();for(const language of ["en","de"]){expect(tool.title[language].trim().length).toBeGreaterThan(0);expect(tool.description[language].trim().length).toBeGreaterThan(0);}expect(tool.command.slice(0,1)).toEqual(["repo"]);}});

test("application task environments use neutral execution controls", () => {
  for (const path of fixture.environmentSources) expect(read(path), path).not.toMatch(/\bNX_[A-Z_]+/u);
});

test("dashboard task arguments cross the neutral argument boundary",()=>{const dashboard=JSON.parse(read("📋️project.json")).metadata.semio.dashboard;expect(dashboard.tools.find((tool:any)=>tool.id==="run-targets").command.slice(0,2)).toEqual(["repo","run-many"]);for(const row of fixture.editorCommands)for(const answers of fixture.editorAnswers){const template=createRequire(import.meta.url)("lodash/template"),command=("repo "+row.args).replace(/\$\{input:(repoBuild[^}]+)\}/gu,(_,id:string)=>{expect(Object.hasOwn(answers,id),id).toBe(true);return answers[id as keyof typeof answers];});expect(command).toBe(template("repo "+row.args,{interpolate:/\$\{input:(repoBuild[^}]+)\}/g})(answers));const parsed=parseBuildInvocationV1(parseArgsStringToArgv(command).slice(2),{}).args;if(["run","run-many","affected","exec","watch"].includes(parsed[0]!))expect(()=>validateBuildTaskArgumentsV1(parsed),row.args).not.toThrow();}}, { timeout: 120_000 });

test("native orchestration and rebuild declarations use neutral build commands", () => {
  const manifest = JSON.parse(read("package.json"));
  for (const command of Object.values(manifest.scripts)) expect(command).not.toMatch(/(?:^|\s)(?:bun\s+(?:(?:run|x)\s+)?)?nx\s/u);
  for (const path of fixture.commandSources) expect(read(path), path).not.toMatch(/\bbun\s+nx\s|--skip-nx-cache|--excludeTaskDependencies|--outputStyle|"nx"\s*,|node_modules\/nx\/(?:dist\/)?bin\/nx\.js|\[cpp\] Nx completed/u);
});

test("MCP native staging uses one neutral command with independent argv", () => {
  const path=fixture.commandSources.find(path=>path.endsWith("/🌉️mcp/🟦️.ts"))!;
  expect(path).toBeDefined();
  const source=ts.createSourceFile(path,read(path),ts.ScriptTarget.Latest,true),calls:ts.CallExpression[]=[];
  const visit=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&node.name.getText(source)==="nativeMcpBinaryStagingFor"){const collect=(child:ts.Node):void=>{if(ts.isCallExpression(child)&&child.expression.getText(source)==="runToolSyncV1")calls.push(child);ts.forEachChild(child,collect);};collect(node);}else ts.forEachChild(node,visit);};visit(source);
  expect(calls.length).toBe(1);
  const project=JSON.parse(read(join(dirname(path),"📦️packages/🦀️rust/📋️project.json"))),target=project.name+":build",call=calls[0]!,argumentsNode=call.arguments[1];
  expect(ts.isStringLiteral(call.arguments[0]!)&&call.arguments[0]!.text).toBe("repo");
  expect(argumentsNode&&ts.isArrayLiteralExpression(argumentsNode)).toBe(true);
  const argv=(argumentsNode as ts.ArrayLiteralExpression).elements.map(node=>ts.isStringLiteral(node)?node.text:node.getText(source)==="MCP_BINARY_TARGET"?target:undefined);
  const expected=["run",target,"--output=stream"],command=["repo",...argv].map(value=>JSON.stringify(value)).join(" ");
  expect(argv).toEqual(expected);
  expect(literalCommandTokensV1(command)).toEqual(parseArgsStringToArgv(command));
  expect(parseArgsStringToArgv(command)).toEqual(["repo",...expected]);
  const options=call.arguments[2] as ts.ObjectLiteralExpression,property=(name:string)=>options.properties.find(node=>ts.isPropertyAssignment(node)&&node.name.getText(source)===name) as ts.PropertyAssignment;
  expect(property("cwd").initializer.getText(source)).toBe("repoRoot");
  expect(property("shell").initializer.getText(source)).toBe("false");
  expect(property("env").initializer.getText(source)).toBe("mcpStagingEnvironment()");
  const environment=source.statements.flatMap(node=>ts.isVariableStatement(node)?node.declarationList.declarations:[]).find(node=>node.name.getText(source)==="mcpStagingEnvironment")!.initializer!.getText(source);
  expect(environment).toContain("...process.env");
  expect(environment).toContain("[CARGO_BUILD_OWNER_PID_ENV]: String(process.pid)");
});

test("Bazel configuration files have explicit repository filename authority", () => {
  const taxonomy = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"));
  for (const row of fixture.fixedBuildFiles) {
    expect(taxonomy.fixedFilenameContracts[row.id], row.path).toMatchObject({ pathPattern: row.path, authority: "Bazel", scope: { kind: "repository-root" } });
    expect(read(row.path).length).toBeGreaterThan(0);
  }
});


test("CI dependency synchronization initializes complete Source ownership through the neutral owner", () => {
  expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
  const row=fixture.ciDependencies,source=read(row.path),yaml=createRequire(import.meta.url)("yaml"),parsed=Bun.YAML.parse(source) as {jobs:Record<string,{steps:{name?:string;run?:string;uses?:string}[]}>};
  expect(parsed).toEqual(yaml.parse(source));
  const accepts=(command:string):boolean=>{try{const argv=literalCommandTokensV1(command);expect(argv).toEqual(parseArgsStringToArgv(command));if(argv[0]!=="repo")return false;const selected=readBuildDependencyInvocationV1(argv.slice(1));return selected.mode==="sync"&&selected.selection.kind==="all";}catch{return false;}};
  const db=new Database(":memory:");
  try{db.exec("CREATE TABLE commands (argv TEXT NOT NULL)");for(const vector of row.cases){const argv=parseArgsStringToArgv(vector.command);db.exec("DELETE FROM commands");db.query("INSERT INTO commands VALUES (?)").run(JSON.stringify(argv));const actual=db.query("SELECT CASE WHEN json_array_length(argv)=4 AND json_extract(argv,'$[0]')='repo' AND json_extract(argv,'$[1]')='dependencies' AND json_extract(argv,'$[2]')='sync' AND json_extract(argv,'$[3]')='--all' THEN 1 ELSE 0 END AS accepted FROM commands").get() as {accepted:number};expect(accepts(vector.command),vector.name).toBe(vector.accepted);expect(Boolean(actual.accepted),vector.name).toBe(vector.accepted);}
    for(const job of row.jobs){const steps=parsed.jobs[job]!.steps,install=steps.filter(step=>step.run?.includes("repo dependencies "));expect(install).toHaveLength(1);expect(install[0]!.run!.endsWith(" "+row.command)||install[0]!.run===row.command).toBe(true);expect(accepts(row.command)).toBe(true);expect(steps.findIndex(step=>step.uses?.startsWith("oven-sh/setup-bun@"))).toBeLessThan(steps.indexOf(install[0]!));for(const step of steps)if(step.run)expect(step.run).not.toMatch(/\bbun\s+(?:run\s+)?install\b|\bbun\s+nx\b/u);}
    for(const cache of ["/workspace/cache/bun-cache","C:\\workspace\\cache\\bun-cache"]){const args=javascriptDependencyProducerArgumentsV1("sync",cache);expect(args.slice(0,3)).toEqual(["install","--frozen-lockfile","--ignore-scripts"]);expect(args.slice(-2)).toEqual(["--cache-dir",cache]);}
  }finally{db.close();}
});
