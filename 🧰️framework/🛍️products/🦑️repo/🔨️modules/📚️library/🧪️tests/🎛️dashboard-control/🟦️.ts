import { expect, spyOn, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import Ajv2020 from "ajv/dist/2020";
import { closeSync, constants, existsSync, fstatSync, lstatSync, openSync, readdirSync, readFileSync, readSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import ts from "typescript";
import { compileJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import censusSchema from "./🧬️schema/🔍️ide-census.json" with { type: "json" };
import censusFixture from "./🧫️fixtures/🔍️ide-census.json" with { type: "json" };
import { advanceScriptInvocation, checkScriptInvocation, receiveScriptProcessInvocation } from "@semio-tech/process/routing";

const root = process.cwd();
const read = (file: string): string => readFileSync(join(root, file), "utf8");

test("the authored compiler project form uses the closed current diagnostic contract",async()=>{
  const owner="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck",cases=JSON.parse(read(owner+"/🧫️fixtures/🎯️project.json")),selectionSchema=JSON.parse(read(owner+"/🧬️schema/🎯️project.json")),oracle=new Ajv2020({strict:true}).compile(selectionSchema),own=compileJsonSchemaSubset(selectionSchema);
  const {readTypecheckProjectSelectionV1}=await import("../../🎛️commands/🪁️typecheck/🟦️.ts");
  const compiler=await import("../../🏗️build/🟦️typescript/🟦️.ts"),libraries=await import("../../🏗️build/🟦️typescript/📚️libraries/🟦️.ts");expect(typeof compiler.typecheckRepositoryProjectV1).toBe("function");expect(libraries.typeScriptLibraryArgumentsV1()).toEqual(["--lib",JSON.parse(read("tsconfig.json")).compilerOptions.lib.join(",")]);
  for(const row of cases.accepted){expect(oracle(row.selection)).toBe(true);expect(own.validate(row.selection)).toEqual([]);expect(readTypecheckProjectSelectionV1(row.words)).toEqual(row.selection);}
  for(const words of cases.rejected){const selection={project:words[0],args:words.slice(1)};expect(oracle(selection)).toBe(false);expect(own.validate(selection).length).toBeGreaterThan(0);expect(()=>readTypecheckProjectSelectionV1(words)).toThrow();}
  const current=JSON.parse(read("🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🧬️schema/🔣️.json"));expect(selectionSchema.properties.args).toEqual(current.definitions.Arguments);
  const {resolveTypecheckProjectSelectionV1}=await import("../../🎛️commands/🪁️typecheck/🟦️.ts"),resolved=await resolveTypecheckProjectSelectionV1(root,["tsconfig.json","--diagnostics"]);expect(resolved.project).toBe(join(root,"tsconfig.json"));expect(resolved.args).toEqual(["--diagnostics"]);
  for(const row of cases.accepted){const parsed=ts.parseCommandLine(["--noEmit","-p",row.selection.project,...row.selection.args,"--incremental","true","--tsBuildInfoFile","owned/state.tsbuildinfo"]);expect(parsed.errors).toEqual([]);expect(parsed.options.noEmit).toBe(true);expect(parsed.options.project).toBe(row.selection.project);expect(parsed.options.tsBuildInfoFile).toBe("owned/state.tsbuildinfo");}
  const project=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json")),target=project.targets["typecheck-project"],parameters=target.metadata.semio.dashboard.parameters;expect(target.options.command).toBe("bun ./📜️script.ts typecheck project");expect(parameters[0].id).toBe("config");expect(parameters[0].valuePositional).toBe(true);expect(parameters[1].values.flatMap((row:any)=>row.args??[]).sort()).toEqual([...current.definitions.Arguments.items.enum].sort());
  const registrySchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json")),projectSchema={...registrySchema,$ref:"#/$defs/ProjectManifest"},metadataOracle=new Ajv2020({strict:false}).compile(projectSchema);expect(metadataOracle(project),JSON.stringify(metadataOracle.errors)).toBe(true);expect(compileJsonSchemaSubset(projectSchema).validate(project)).toEqual([]);expect(parameters[0].suggestions).toEqual({source:"configs"});
  console.log("[DEBUG] Authored compiler form initializes its current owner and selects a physical workspace project with closed diagnostic arguments; compiler execution is separate");
});

test("native owner controls conserve build selections and all four supervised compounds", () => {
  const cases = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🎮️owners.json"), "utf8")), manifest = JSON.parse(read("📋️project.json")), declaration = manifest.metadata.semio.dashboard;
  const registrySchema = JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json")), projectSchema = { ...registrySchema, $ref: "#/$defs/ProjectManifest" };
  const validate = new Ajv2020({strict:false}).compile(projectSchema), own = compileJsonSchemaSubset(projectSchema);
  expect(validate(manifest), JSON.stringify(validate.errors)).toBe(true);expect(own.validate(manifest)).toEqual([]);
  for(const row of cases.tools){
    const tool=declaration.tools.find((tool:any)=>tool.id===row.id);expect(tool,row.id).toBeDefined();
    const command=tool.command.map((word:string)=>word.replace(/\{([^}]+)\}/gu,(_,name:string)=>row.values[name]));for(const parameter of tool.parameters??[]){const value=row.values[parameter.id];if(value===undefined)continue;if(parameter.valuePositional)command.push(value);if(parameter.valueFlag)command.push(...parameter.valueFlag.endsWith("=")?[parameter.valueFlag+value]:[parameter.valueFlag,value]);if(parameter.kind==="choice")command.push(...(parameter.values.find((choice:any)=>choice.id===value)?.args??[]));if(parameter.kind==="flag"&&value===true)command.push(...parameter.args??[]);}expect(command,row.id).toEqual(row.command);
    for(const parameter of tool.parameters??[])expect(Object.keys(parameter.title).sort()).toEqual(["de","en"]);
  }
  const backend=declaration.parameters.find((parameter:any)=>parameter.id==="build-tool");
  for(const target of ["dev-mcp","dev-mcp-repo"])expect(manifest.targets[target].metadata.semio.dashboard.ready).toEqual({port:6274,portEnv:"CLIENT_PORT",printed:true});
  for(const [id,env] of Object.entries(cases.buildTools))expect(backend.values.find((value:any)=>value.id===id).env).toEqual(env);
  const axes=cases.axisVerbs,known=[...read(axes.verbSource).match(/pub const VERBS: &\[&str\] = &\[([^\]]*)\]/u)![1]!.matchAll(/"([a-z]+)"/gu)].map(match=>match[1]!),cacheAxis=declaration.parameters.find((parameter:any)=>parameter.id==="cache"),sorted=(values:string[])=>[...values].sort();
  expect(known.length).toBeGreaterThan(20);
  expect(sorted(backend.appliesTo.verbs)).toEqual(sorted([...known,axes.fallback,...axes.toolVerbs]));
  expect(sorted(cacheAxis.appliesTo.verbs)).toEqual(sorted([...known.filter(verb=>!axes.uncachedVerbs.includes(verb)),axes.fallback]));
  expect(new Set(backend.appliesTo.verbs).size).toBe(backend.appliesTo.verbs.length);
  for(const id of ["build-cache-status","build-cache-clear"]){const tool=declaration.tools.find((tool:any)=>tool.id===id);expect(backend.appliesTo.verbs,id).toContain(tool.verb);}
  const exec=declaration.tools.filter((tool:any)=>tool.command.slice(0,3).join(" ")==="bun repo exec");expect(exec.map((tool:any)=>tool.id).sort()).toEqual(["build-exec","bun-test","native-cargo","run-script"]);
  for(const tool of exec){const planning=tool.parameters.find((parameter:any)=>parameter.id==="dry-run");expect(planning.kind,tool.id).toBe("choice");expect(planning.default,tool.id).toBe("false");expect(planning.values.map((value:any)=>value.id),tool.id).toEqual(["false","true"]);expect(tool.command.indexOf("--dry-run={dry-run}"),tool.id).toBeGreaterThan(-1);expect(tool.command.indexOf("--dry-run={dry-run}"),tool.id).toBeLessThan(tool.command.indexOf("--"));}
  for(const id of ["run-targets","build-affected","build-task"]){const planning=declaration.tools.find((tool:any)=>tool.id===id).parameters.find((parameter:any)=>parameter.id==="dry-run");expect(planning.kind,id).toBe("flag");expect(planning.args,id).toEqual(["--dry-run"]);}
  for(const row of cases.compounds){const project=JSON.parse(read(row.file)),compound=project.metadata.semio.dashboard.compounds.find((compound:any)=>compound.id===row.id);expect(compound.stop).toBe("together");expect(compound.members.map((member:any)=>member.run)).toEqual(row.members);}
  const phases=manifest.targets["test-build-tool-bazel"].metadata.semio.dashboard.parameters.find((parameter:any)=>parameter.id==="phase");expect(phases.values.map((value:any)=>value.id)).toEqual(cases.phases);for(const value of phases.values)expect(value.args).toEqual([value.id]);expect(phases.default).toBeUndefined();
  const owners=JSON.parse(read("📋️build-owners.json")),ownerSchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️owners.json")),ownerOracle=new Ajv2020({strict:false}).compile(ownerSchema);expect(ownerOracle(owners),JSON.stringify(ownerOracle.errors)).toBe(true);expect(compileJsonSchemaSubset(ownerSchema).validate(owners)).toEqual([]);
  for(const name of cases.nativeOwners){const owner=owners.owners.find((owner:any)=>owner.name===name);expect(owner,name).toBeDefined();expect(JSON.parse(read(owner.manifest)).name).toBe(name);expect((Bun.TOML.parse(read(owner.root+"/Cargo.toml")) as {package?:unknown}).package).toBeDefined();}
  const course=JSON.parse(read(cases.course.file));expect(validate(course),JSON.stringify(validate.errors)).toBe(true);expect(own.validate(course)).toEqual([]);for(const target of cases.course.releaseTargets){const mode=course.targets[target].metadata?.semio?.dashboard?.parameters?.find((parameter:any)=>parameter.id==="execution-mode");expect(mode,target).toMatchObject({kind:"choice",default:"release",values:[{id:"release",env:{SEMIO_BUILD_MODE:"release"}}]});expect(Object.keys(mode.title).sort()).toEqual(["de","en"]);}for(const target of cases.course.readyTargets)expect(course.targets[target].metadata.semio.dashboard.ready).toEqual({port:6061,portEnv:"TEACHING_ARCHITECTURE_COURSE_PORT"});
  console.log("[DEBUG] Native owner controls preserve backend selection, nine Bazel phases and four supervised compounds");
});

test("exec preview words and cache words declared by the registry are accepted by the build command line and the Nx translation", async () => {
  const cases = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🎮️owners.json"), "utf8")), { validateBuildTaskArgumentsV1 } = await import("../../🏗️build/🧭️selection/🟦️.ts"), { nxBuildArgumentsV1 } = await import("../../🏗️build/🧭️invocation/🟦️.ts");
  for (const row of cases.tools.filter((row: any) => row.command[2] === "exec" || row.command[2] === "run-many" || row.command[2] === "affected")) {
    const args: string[] = row.command.slice(2);
    expect(() => validateBuildTaskArgumentsV1(args), row.id).not.toThrow();
    const translated = nxBuildArgumentsV1(args), planning = args.slice(0, args.includes("--") ? args.indexOf("--") : undefined).some(word => word === "--dry-run" || word === "--dry-run=true");
    expect(translated.includes("--graph=stdout"), row.id).toBe(planning);
    expect(translated.some(word => word.startsWith("--dry-run")), row.id).toBe(false);
  }
  for (const words of [["cache", "status"], ["cache", "clear"]]) expect(cases.tools.some((row: any) => row.command.slice(2).join(" ") === words.join(" ")), words.join(" ")).toBe(true);
});

test("dashboard control regression is admitted by our schema compiler and Ajv", () => {
  const own = compileJsonSchemaSubset(schema);
  const oracle = new Ajv({ strict: false }).compile(schema);
  for (const value of [fixture, {}, { ...fixture, absentFiles: [] }, { ...fixture, namedInputs: [] }, { ...fixture, targets: [{ file: "x", absent: [], present: [] }] }]) {
    expect(own.validate(value).length === 0).toBe(oracle(value));
  }
  expect(oracle(fixture)).toBe(true);
});

test("generation and authoring expose neutral controls without IDE projections", () => {
  for (const file of fixture.absentFiles) expect(existsSync(join(root, file)), file).toBe(false);
  for (const row of fixture.targets) {
    const source = read(row.file), manifest = JSON.parse(source);
    const parsed = ts.parseConfigFileTextToJson(row.file, source);
    expect(parsed.error).toBeUndefined();
    expect(parsed.config).toEqual(manifest);
    for (const target of row.absent) expect(manifest.targets[target], `${row.file}: ${target}`).toBeUndefined();
    for (const target of row.present) expect(manifest.targets[target], `${row.file}: ${target}`).toBeDefined();
    if (row.inputTarget !== undefined) {
      const inputs = manifest.targets[row.inputTarget].inputs;
      for (const input of row.absentInputs ?? []) expect(inputs).not.toContain(input);
      for (const input of row.presentInputs ?? []) expect(inputs).toContain(input);
    }
  }
  for (const row of fixture.sources) {
    const source = read(row.file);
    for (const symbol of row.absent) expect(source, `${row.file}: ${symbol}`).not.toContain(symbol);
  }
  console.log("[DEBUG] Dashboard control retirement admits native owners and rejects IDE generation paths");
});


test("retired generate launch invocations fail before producing editor configuration", async () => {
  const { GenerateScript } = await import("../../🎛️commands/🏭️generate/🟦️.ts");
  const usage: string[] = [], codes: number[] = [];
  const output = spyOn(console, "error").mockImplementation(message => { usage.push(String(message)); });
  const exit = spyOn(process, "exit").mockImplementation(code => { codes.push(Number(code)); throw new Error("retired-command-rejected"); });
  try {
    for (const args of [["launch"], ["launch", "--check"]]) {
      await expect(GenerateScript.prototype.run.call(Object.create(null), args)).rejects.toThrow("retired-command-rejected");
    }
  } finally {
    output.mockRestore();
    exit.mockRestore();
  }
  expect(codes).toEqual([1, 1]);
  expect(usage).toHaveLength(2);
  for (const line of usage) {
    expect(line).toContain("generate <config [--check]");
    expect(line).not.toContain("launch");
  }
  console.log("[DEBUG] generate launch and generate launch --check reject with exit 1");
});


test("source and registration regressions read neutral declarations without IDE documents", () => {
  for (const path of fixture.readerFiles) {
    const text = read(path), source = ts.createSourceFile(path, text, ts.ScriptTarget.Latest, true);
    expect(ts.transpileModule(text, { fileName: path, reportDiagnostics: true }).diagnostics, path).toEqual([]);
    const readers: string[] = [];
    const visit = (node: ts.Node): void => {
      if (ts.isCallExpression(node) && /(?:^|\.)(?:readFileSync|readFile|read)$/.test(node.expression.getText(source)) && node.arguments[0] && /(?:\.vscode|launch\.json|launch\.seed\.jsonc)/.test(node.arguments[0].getText(source))) readers.push(node.getText(source));
      ts.forEachChild(node, visit);
    };
    visit(source);
    expect(readers, path).toEqual([]);
  }
  console.log("[DEBUG] Source and registration suites retain neutral metadata readers");
}, 15_000);


test("setup instruction aliases retain only the current headless client contract", () => {
  const path = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts", source = ts.createSourceFile(path, read(path), ts.ScriptTarget.Latest, true);
  const aliases: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isVariableDeclaration(node) && node.name.getText(source) === "AGENT_INSTRUCTION_ALIASES" && node.initializer) {
      const collect = (child: ts.Node): void => { if (ts.isStringLiteral(child)) aliases.push(child.text); ts.forEachChild(child, collect); };
      collect(node.initializer);
    }
    ts.forEachChild(node, visit);
  };
  visit(source);
  expect(aliases).toEqual(fixture.instructionAliases);
  const rootReaders = read("🧪️tests/🧱️checked-in-asset-contracts/🟦️.ts");
  for (const path of [".cursor/mcp.json", ".vscode/mcp.json"]) expect(rootReaders).not.toContain(path);
  console.log("[DEBUG] Setup aliases and root asset readers retain current headless controls only");
});


test("build source inputs depend on neutral policies without retired IDE assets", () => {
  for (const row of fixture.namedInputs) {
    const text = read(row.file), manifest = JSON.parse(text), parsed = ts.parseConfigFileTextToJson(row.file, text);
    expect(parsed.error).toBeUndefined();
    expect(parsed.config).toEqual(manifest);
    const inputs = manifest.namedInputs[row.name];
    for (const input of row.absent) expect(inputs, row.file).not.toContain(input);
    for (const input of row.present) {
      expect(inputs, row.file).toContain(input);
      if (input.startsWith("{workspaceRoot}/") && !input.includes("*")) expect(existsSync(join(root, input.slice("{workspaceRoot}/".length))), input).toBe(true);
    }
    expect(inputs.filter((input: unknown) => typeof input === "string" && /(?:\.vscode(?:-insiders)?[/\\]|\.(?:sln|slnx|code-workspace)$)/u.test(input)), row.file).toEqual([]);
  }
  console.log("[DEBUG] .NET and course named inputs retain existing neutral policies and reject retired IDE assets");
});


test("ordinary build entry launches native dashboard commands before install and backend validation", async () => {
  const text = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts"), tree = ts.createSourceFile("build-entry.ts", text, ts.ScriptTarget.Latest, true);
  const inner = tree.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "runOwnedBuildInLeasesV1");
  if (!inner || !ts.isFunctionDeclaration(inner) || !inner.body) throw Error("Actual dashboard build dispatch required");
  const retained = inner.body.statements.find(ts.isTryStatement), branch = retained?.tryBlock.statements.find(node => ts.isIfStatement(node) && node.getText(tree).includes("native.dashboardInvocation"));
  if (!branch) throw Error("Actual native dashboard dispatch branch required");
  const compile = (body: string): string => { const compiled = ts.transpileModule(body, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext }, reportDiagnostics: true });expect(compiled.diagnostics).toEqual([]);return compiled.outputText.replace(/import\(([^)]+)\)/gu, "load($1)"); };
  const code = compile("async function run(){" + branch.getText(tree) + ";fallback();}");
  const routing = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"), leases = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts"), capacity = await import("../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts"), native = await import("../../⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation/🟦️.ts");
  const oracle = new Ajv({ strict: false }).compile(schema), own = compileJsonSchemaSubset(schema), database = new Database(":memory:");
  try {
    expect(oracle(fixture)).toBe(true);expect(own.validate(fixture)).toEqual([]);
    for (const row of fixture.dashboardRoutes) await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({ version: 1, owner: "dashboard:build-route-test", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => {
      const network = Object.freeze({ offline: true }), events: string[] = [], invoked = native.dashboardInvocation(row.args), bindings = {
        root, resolved: { args: row.args, env: {} }, capabilities: { buildMode: row.mode }, invocation,
        readBuildMode: (value: string) => { expect(value).toBe(row.mode);return value; },
        scriptCompilerAdmissionControlV1: capacity.scriptCompilerAdmissionControlV1,
        fallback: () => { events.push("build"); },
        load: async (path: string) => {
          if (path.endsWith("/📦️installation/🟦️.ts")) return { dashboardInvocation: native.dashboardInvocation, dashboardInstalled: () => row.installed, launchDashboard: async (workspace: string, args: readonly string[], options: any) => {
            expect(workspace).toBe(root);expect(options.workspace).toBe(root);expect(options.invocation).toBe(invocation);expect(options.mode).toBe(row.mode);expect(options.network).toBe(network);routing.requireIssuedResourceControlInvocationV1(options.control, invocation);expect(options.control.signal).toBe(invocation.control.signal);leases.checkResourceLeaseControl(options.control);events.push("native");if(invoked===undefined)throw Error("A dashboard launch needs a dashboard invocation: "+row.args.join(" "));expect(args).toEqual(invoked);await Promise.resolve();leases.checkResourceLeaseControl(options.control);return row.status;
          } };
          if (path.endsWith("/📥️invocation/🟦️.ts")) return { readNativeCallerAuthorityV1: (value: unknown) => { expect(value).toBe(invocation.capabilities);return { network }; } };
          throw Error("Undeclared dashboard route port: " + path);
        }
      };
      let failed = false;
      try { await new Function(...Object.keys(bindings), code + ";return run;")(...Object.values(bindings))(); } catch (error) { failed = true;expect(String(error)).toContain("Dashboard invocation failed (" + row.status + ")"); }
      const args = JSON.stringify(row.args), actual = failed ? "failed" : events[0], expected = database.query("WITH arg AS (SELECT key,value FROM json_each(?)), boundary AS (SELECT coalesce(min(key),999) n FROM arg WHERE value='--') SELECT CASE WHEN json_extract(?,'$[0]')='run' AND json_extract(?,'$[1]') IN ('@semio-tech/repo-dashboard-rs:run','@semio-tech/repo-dashboard-rs:daemon','@semio-tech/repo-dashboard-rs:workflow','@semio-tech/repo-dashboard-rs:preferences') AND NOT EXISTS(SELECT 1 FROM arg,boundary WHERE key>1 AND key<n AND (value GLOB '--graph*' OR value GLOB '--batch*' OR value GLOB '--parallel*' OR value GLOB '--exclude-dependencies*')) THEN CASE WHEN ?=0 THEN 'native' ELSE 'failed' END ELSE 'build' END route").get(args, args, args, row.status) as { route: string };
      expect(expected.route).toBe(row.expected);expect(actual, JSON.stringify(row)).toBe(row.expected);expect(events).toEqual([row.expected === "build" ? "build" : "native"]);
    });
    const entry = tree.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "runOwnedBuildInvocationV1");
    if (!entry || !ts.isFunctionDeclaration(entry)) throw Error("Actual dashboard help entry required");
    const entryCode = compile(entry.getText(tree).replace("export async function", "async function")), parser = await import("../../🏗️build/🧭️invocation/🟦️.ts");
    for (const row of fixture.dashboardHelp) await routing.withScriptProcessEnvelope(routing.createScriptProcessEnvelope({ version: 1, owner: "dashboard:help-route-test", maximumElapsedMilliseconds: 0 }, { tool: "nx" }, Date.now()), async invocation => {
      const events: string[] = [], opened = Object.freeze({}), phase = routing.createIssuedResourceControlV1(invocation, "dashboard:help-route", 0), bindings = {
        requireIssuedResourceControlInvocationV1: routing.requireIssuedResourceControlInvocationV1, checkResourceLeaseControl: leases.checkResourceLeaseControl, console: { log: () => { events.push("help"); } },
        load: async (path: string) => {
          if (path === "./📥️caller/🟦️.ts") return { readOwnedBuildInvocationV1: (_root: string, original: unknown, args: unknown) => { expect(original).toBe(invocation);expect(args).toBe(row.args); } };
          if (path === "./🧭️invocation/🟦️.ts") return parser;
          if (path.endsWith("/📦️installation/🟦️.ts")) return native;
          if (path.endsWith("/📥️opening/🟦️.ts")) return { openRepositoryCachePartitionV1: async () => { events.push("open");throw opened; } };
          throw Error("Undeclared dashboard help port: " + path);
        }
      };
      try { await new Function(...Object.keys(bindings), entryCode + ";return runOwnedBuildInvocationV1;")(...Object.values(bindings))(root, row.args, invocation, phase); } catch (error) { expect(error).toBe(opened); }
      expect(events, JSON.stringify(row)).toEqual([row.expected]);
    });
    console.log("[DEBUG] Actual build dispatch routes missing/installed dashboard commands and native help with Original control; authoring and backend selections retain build routing");
  } finally { database.close(); }
});


test("shared developer tooling declarations and all lock scopes retain canonical providers", () => {
  const row=fixture.sharedProviders,canonical=JSON.parse(read(row.canonical)),consumer=JSON.parse(read(row.consumer)),database=new Database(":memory:"),provider=(packages:Readonly<Record<string,readonly unknown[]|undefined>>,name:string)=>packages[consumer.name+"/"+name]?.[0]??packages[name]?.[0];
  try{
    database.exec("create table provider(scope text,name text,version text)");
    for(const vector of row.vectors){const actual=provider(vector.packages,"postcss"),selected=database.query("select json_extract(value,'$[0]') provider from json_each(?) where key in (?,?) order by case when key=? then 0 else 1 end limit 1").get(JSON.stringify(vector.packages),consumer.name+"/postcss","postcss",consumer.name+"/postcss")as{provider:string}|null;expect(actual==="postcss@8.5.30").toBe(vector.expected);expect(selected?.provider==="postcss@8.5.30").toBe(vector.expected);}
    for(const dependency of row.dependencies){const expected=canonical.dependencies?.[dependency.name]??canonical.devDependencies?.[dependency.name];expect(expected).toBe(dependency.version);expect(consumer[dependency.group][dependency.name]).toBe(expected);database.query("insert into provider values(?,?,?)").run("canonical",dependency.name,expected);database.query("insert into provider values(?,?,?)").run("consumer",dependency.name,consumer[dependency.group][dependency.name]);}
    for(const path of row.locks){const text=read(path),parsed=ts.parseConfigFileTextToJson(path,text);expect(parsed.error,path).toBeUndefined();const lock=JSON.parse(text);expect(parsed.config).toEqual(lock);const memberships=Object.entries(lock.workspaces).filter(([,value])=>(value as any).name===consumer.name);expect(memberships,path).toHaveLength(1);expect(resolve(root,dirname(path),memberships[0]![0]),path).toBe(resolve(root,dirname(row.consumer)));for(const dependency of row.dependencies){expect((memberships[0]![1]as any)[dependency.group][dependency.name]).toBe(dependency.version);const actual=provider(lock.packages,dependency.name);expect(actual,path+": "+dependency.name).toBe(dependency.name+"@"+dependency.version);database.query("insert into provider values(?,?,?)").run(path,dependency.name,String(actual).slice(dependency.name.length+1));}}
    expect(database.query("select p.scope,p.name,p.version from provider p join provider canonical on canonical.scope='canonical' and canonical.name=p.name where p.version<>canonical.version").all()).toEqual([]);
    console.log("[DEBUG] Shared UI Source memberships resolve exact canonical provider versions in all six locks, including nested provider precedence");
  }finally{database.close();}
});

test("all-command axes match neutral vectors and SQLite without enumerating the verb vocabulary", async () => {
  const dashboard="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard",cases=JSON.parse(read(dashboard+"/🧪️tests/🧫️fixtures/🎮️registry/🎯️axes.json")),schema=JSON.parse(read(dashboard+"/🧬️schema/🎮️registry/🔣️.json")),contract={...schema,$ref:"#/$defs/AppliesTo"},own=compileJsonSchemaSubset(contract),oracle=new Ajv2020({strict:false}).compile(contract),database=new Database(":memory:"),{axisApplies}=await import("@semio-tech/repo-product/modules/dashboard/testing/coverage/oracle");
  try{
    for(const [rows,accepted]of [[cases.accepted,true],[cases.rejected,false]]as const)for(const value of rows){expect(oracle(value),JSON.stringify(value)).toBe(accepted);expect(own.validate(value).length===0,JSON.stringify(value)).toBe(accepted);}
    for(const row of cases.cases){const parameter=row.parameter,scope=parameter.appliesTo,effect=Boolean(parameter.valueEnv||parameter.args?.length||parameter.env&&Object.keys(parameter.env).length),selected=database.query("select case when ?1='playground' then coalesce(json_extract(?2,'$.playground'),0)=1 when ?1='tool' then ?4=1 and ?2 is not null and (coalesce(json_extract(?2,'$.all'),0)=1 or exists(select 1 from json_each(?2,'$.verbs') where value=?3)) else ?2 is null or coalesce(json_extract(?2,'$.all'),0)=1 or exists(select 1 from json_each(?2,'$.verbs') where value=?3) end selected").get(row.command,scope===undefined?null:JSON.stringify(scope),row.verb,effect?1:0)as{selected:number};expect(Boolean(selected.selected),JSON.stringify(row)).toBe(row.expected);expect(axisApplies(parameter,row.verb,row.command),JSON.stringify(row)).toBe(row.expected);}
    const budget=JSON.parse(read("📋️project.json")).metadata.semio.dashboard.parameters.find((row:any)=>row.id==="build-budget");expect(budget.appliesTo).toEqual({all:true,playground:true});expect(budget.title).toEqual({en:"Process time limit",de:"Prozesszeitlimit"});expect(budget.valueEnv).toBe("SEMIO_BUILD_BUDGET_MS");
  }finally{database.close();}
  console.log("[DEBUG] Process time limit covers current and future authored verbs, explicit playgrounds and effectful tools through independent schema/SQLite vectors");
});

test("selected process deadlines share numeric grammar and immutable elapsed-time vectors", () => {
  const dashboard="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard",schema=JSON.parse(read(dashboard+"/🧬️schema/🌀️daemon/🔣️.json")),protocol=schema.definitions.ProcessDeadlineProtocol.const,vectors=JSON.parse(read(dashboard+"/🧪️tests/🧫️fixtures/🌀️control-plane/⏱️deadline.json")),shared=JSON.parse(read("🧰️framework/🔨️modules/🏃️process/⏱️budget/🧬️schema/🔣️.json")),own=compileJsonSchemaSubset(shared),oracle=new Ajv({strict:false}).compile(shared),database=new Database(":memory:");
  try{
    expect(protocol.maximumMilliseconds).toBe(shared.$defs.Milliseconds.maximum);expect(protocol.unboundedMilliseconds).toBe(0);expect(protocol.status).toBe("timed_out");expect(protocol.code).toBe(124);expect(protocol.capture).toBe("before-spawn");expect(protocol.renewal).toBe("explicit-restart-only");
    for(const row of vectors.parseCases){const actual=Number(row.raw??0),accepted=Number.isSafeInteger(actual)&&actual>=0&&actual<=protocol.maximumMilliseconds;expect(oracle(actual)).toBe(accepted);expect(own.validate(actual).length===0).toBe(accepted);expect(accepted?(actual===0?0:actual):null,JSON.stringify(row)).toBe(row.expected);}
    for(const row of vectors.expirationCases){const selected=database.query("select ?1<>0 and ?2>=?1 expired").get(row.milliseconds,row.elapsed)as{expired:number};expect(Boolean(selected.expired)).toBe(row.expired);}
    for(const row of vectors.continuations){const selected=database.query("select ?1>=100 expired").get(row.elapsed)as{expired:number};expect(Boolean(selected.expired),row.kind).toBe(row.expired);}
  }finally{database.close();}
  console.log("[DEBUG] Central process deadline vectors match shared Number grammar, schema bounds and SQLite elapsed-time decisions without continuation renewal");
});


type CensusKind = "file" | "directory" | "symlink";
type ManifestRole="active"|"ticket"|"retained"|"neutral";
type IdeDisposition="excluded"|"retired"|"accepted";
/** 🧷️ Census fixture rows whose authored expectations are read as the closed unions the implementation returns. */
type PathCaseRow=Omit<(typeof censusFixture.pathCases)[number],"expected">&{readonly expected:IdeDisposition};
type RoleCaseRow=Omit<(typeof censusFixture.roleCases)[number],"expectedRole">&{readonly expectedRole:ManifestRole};

function censusXmlRoot(text:string):string|null{
  let remaining=text.replace(/^\uFEFF/u,"").replace(/^[ \t\r\n]+/u,"");
  for(let index=0;index<=censusFixture.maximumXmlPrologSections;index++){
    const end=remaining.startsWith("<?")?"?>":remaining.startsWith("<!--")?"-->":null;
    if(end){const offset=remaining.indexOf(end);if(offset<0)return null;remaining=remaining.slice(offset+end.length).replace(/^[ \t\r\n]+/u,"");continue;}
    return remaining.match(/^(?:<!DOCTYPE[ \t\r\n]+|<)((?:[A-Za-z_][A-Za-z0-9_.-]*:)?[A-Za-z_][A-Za-z0-9_.-]*)(?=[ \t\r\n />\[])/u)?.[1]?.split(":").at(-1)??null;
  }
  throw Error("XML launch root exceeds the bounded prolog census");
}

function censusManifestRole(path:string,ticketStatus:string|null):ManifestRole{
  const normalized=path.replaceAll("\\","/").toLowerCase(),parts=normalized.split("/"),declaration=censusFixture.semanticRoles,name=parts.at(-1)??"";
  if(declaration.ticketRoots.some(root=>normalized.startsWith(root+"/"))&&parts.some(part=>declaration.retainedInputDirectories.includes(part)))return "retained";
  if(name===declaration.ticketCommandName)return ticketStatus==="open"?"ticket":"neutral";
  return censusFixture.manifestNames.includes(name)?"active":"neutral";
}

function censusManifestDocument(text:string,role:ManifestRole):Record<string,any>|null{
  if(role==="retained"||role==="neutral")return null;
  const value:unknown=JSON.parse(text);if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Active control manifest object required");return value;
}

function censusReadText(file:string,maximum=2000000,prefix=false):{text:string;truncated:boolean}{
  const before=lstatSync(file);if(!before.isFile()||before.isSymbolicLink())throw Error("Authored census file changed kind: "+file);if(!prefix&&before.size>maximum)throw Error("Authored census file exceeds read bound: "+file);
  const descriptor=openSync(file,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let text:string;
  try{const opened=fstatSync(descriptor);if(before.dev!==opened.dev||before.ino!==opened.ino||!opened.isFile())throw Error("Authored census file changed identity before read: "+file);if(prefix){const bytes=Buffer.alloc(Math.min(maximum,opened.size));let count=0;while(count<bytes.length){const received=readSync(descriptor,bytes,count,bytes.length-count,count);if(received===0)throw Error("Authored census prefix ended before its observed bound: "+file);count+=received;}text=bytes.toString("utf8");}else text=readFileSync(descriptor,"utf8");}finally{closeSync(descriptor);}
  const after=lstatSync(file);if(before.dev!==after.dev||before.ino!==after.ino||after.isSymbolicLink())throw Error("Authored census file changed identity during read: "+file);return{text,truncated:before.size>maximum};
}

const ideDirectories=new Set(censusFixture.directories),ideFiles=new Set(censusFixture.files),ideExclusions=new Set(censusFixture.excludedDirectories);

function retiredIdePath(path:string,kind:CensusKind,reference=false,content?:string):IdeDisposition{
  const parts=path.toLowerCase().split(reference?/[\\/\s"'=;:,()\[\]{}]+/u:/[\\/]+/u).filter(Boolean),last=parts.at(-1)??"";
  if(!reference&&parts.some(part=>ideExclusions.has(part)))return "excluded";
  const file=(part:string)=>ideFiles.has(part)||censusFixture.suffixes.some(suffix=>part.endsWith("."+suffix));
  return parts.some(part=>ideDirectories.has(part))||(reference?parts.some(file):kind!=="directory"&&(file(last)||last.endsWith(".launch")&&content!==undefined&&censusXmlRoot(content)==="launchConfiguration"))?"retired":"accepted";
}

function manifestControlStrings(manifest:Record<string,any>,embedded=false,ticket=false):{value:string;input:boolean}[]{
  const selected:{value:string;input:boolean}[]=[],collect=(value:unknown,input=false):void=>{if(typeof value==="string")selected.push({value,input});else if(Array.isArray(value))for(const item of value)collect(item,input);else if(value&&typeof value==="object")for(const item of Object.values(value))collect(item,input);};
  const effect=(value:unknown):void=>{if(Array.isArray(value)){for(const item of value)effect(item);}else if(value&&typeof value==="object")for(const[key,item]of Object.entries(value))if(["command","commands","args","env"].includes(key))collect(item);else effect(item);};
  const target=(value:any):void=>{if(!value||typeof value!=="object")return;for(const key of ["inputs","outputs","dependsOn"])collect(value[key],true);for(const key of ["command","commands","options","configurations"])collect(value[key]);effect(value.metadata?.semio?.dashboard);};
  if(ticket){effect(manifest.tools);effect(manifest.compounds);return selected;}collect(manifest.scripts);collect(manifest.namedInputs,true);target(manifest);for(const value of Object.values(manifest.targets??{}))target(value);for(const value of Object.values(manifest.targetDefaults??{}))target(value);for(const value of Object.values(manifest.tasks??{}))target(value);effect(manifest.metadata?.semio?.dashboard);if(!embedded&&manifest.nx&&typeof manifest.nx==="object"&&!Array.isArray(manifest.nx))selected.push(...manifestControlStrings(manifest.nx,true));
  return selected;
}

function retiredManifestReferences(manifest:Record<string,any>,ticket=false):string[]{
  return [...new Set(manifestControlStrings(manifest,false,ticket).filter(row=>!(row.input&&row.value.trimStart().startsWith("!"))&&retiredIdePath(row.value,"file",true)==="retired").map(row=>row.value))].sort();
}


const guiRegistrySchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"));
const guiExecutable=new RegExp(guiRegistrySchema.$defs.SecondaryDeveloperGuiExecutable.pattern),guiLauncher=new RegExp(guiRegistrySchema.$defs.SecondaryDeveloperGuiLauncher.pattern);
function acceptedDashboardToolCommand(command:readonly string[]):boolean{return !guiExecutable.test(command[0]??"")&&!(guiLauncher.test(command[0]??"")&&command.slice(1).some(word=>guiExecutable.test(word)));}
function declaredDashboardToolCommands(manifest:Record<string,any>,ticket=false):string[][]{
  const commands:string[][]=[],add=(declaration:any):void=>{for(const tool of declaration?.tools??[])if(Array.isArray(tool.command)&&tool.command.every((word:unknown)=>typeof word==="string"))commands.push(tool.command);};
  if(ticket)add(manifest);else{add(manifest.metadata?.semio?.dashboard);for(const collection of [manifest.targets,manifest.targetDefaults,manifest.tasks])for(const target of Object.values(collection??{})as any[])add(target?.metadata?.semio?.dashboard);if(manifest.nx&&typeof manifest.nx==="object")commands.push(...declaredDashboardToolCommands(manifest.nx));}return commands;
}

test("dashboard tool commands refuse secondary developer GUI handoffs while retaining domain tools",()=>{
  const fixtureOracle=new Ajv({strict:false}).compile(censusSchema);expect(fixtureOracle(censusFixture),JSON.stringify(fixtureOracle.errors)).toBe(true);
  const commandSchema={...guiRegistrySchema,$ref:"#/$defs/DashboardToolCommand"},oracle=new Ajv2020({strict:false}).compile(commandSchema),database=new Database(":memory:");
  try{
    const sql=(command:readonly string[]):boolean=>(database.query("WITH RECURSIVE paths(ordinal,rest,part) AS (SELECT cast(key AS integer),lower(replace(value,char(92),'/'))||'/','' FROM json_each(?1) UNION ALL SELECT ordinal,substr(rest,instr(rest,'/')+1),substr(rest,1,instr(rest,'/')-1) FROM paths WHERE rest<>''),names AS (SELECT ordinal,CASE WHEN substr(part,-4) IN ('.exe','.cmd','.app') THEN substr(part,1,length(part)-4) ELSE part END name FROM paths WHERE rest='') SELECT NOT EXISTS(SELECT 1 FROM names WHERE ordinal=0 AND name IN (SELECT value FROM json_each(?2))) AND NOT (EXISTS(SELECT 1 FROM names WHERE ordinal=0 AND name IN ('open','start','cmd','powershell','pwsh')) AND EXISTS(SELECT 1 FROM names WHERE ordinal>0 AND name IN (SELECT value FROM json_each(?2)))) accepted").get(JSON.stringify(command),JSON.stringify(guiRegistrySchema.$defs.SecondaryDeveloperGuiName.enum))as{accepted:number}).accepted===1;
    for(const row of censusFixture.developerGuiCases){expect(acceptedDashboardToolCommand(row.command),row.name).toBe(row.accepted);expect(oracle(row.command),row.name).toBe(row.accepted);expect(sql(row.command),row.name).toBe(row.accepted);}for(const name of guiRegistrySchema.$defs.SecondaryDeveloperGuiName.enum){for(const command of [[name],["C:\\Tools\\"+name.toUpperCase()+".EXE"],["open","-a",name]]){expect(acceptedDashboardToolCommand(command),name).toBe(false);expect(oracle(command),name).toBe(false);expect(sql(command),name).toBe(false);}}
    const project=JSON.parse(read("📋️project.json")),commands=declaredDashboardToolCommands(project);expect(commands.length).toBeGreaterThan(0);for(const command of commands){expect(acceptedDashboardToolCommand(command),command.join(" ")).toBe(true);expect(oracle(command),command.join(" ")).toBe(true);expect(sql(command),command.join(" ")).toBe(true);}expect(project.metadata.semio.dashboard.tools.some((tool:any)=>tool.id==="gitkraken")).toBe(false);
  }finally{database.close();}
});

test("physical IDE census and manifest references refuse unknown ignored owner controls",async()=>{
  const own=compileJsonSchemaSubset(censusSchema),schemaOracle=new Ajv({strict:false}).compile(censusSchema),database=new Database(":memory:");
  expect(schemaOracle(censusFixture),JSON.stringify(schemaOracle.errors)).toBe(true);expect(own.validate(censusFixture)).toEqual([]);
  try{
    let normalized="replace(?1,char(92),'/')";for(const separator of [" ","\t","\r","\n","\v","\f",'"',"'","=",";",":",",","(",")","[","]","{","}"])normalized="replace("+normalized+","+"char("+separator.charCodeAt(0)+")"+",'/')";
    const xmlRootOracle=(text:string):string|null=>{
      let normalized="raw";for(const separator of [9,10,13,47,62,91])normalized="replace("+normalized+",char("+separator+"),' ')";
      const query="WITH RECURSIVE prolog(rest,n) AS (SELECT ltrim(?1,?2),0 UNION ALL SELECT ltrim(CASE WHEN substr(rest,1,2)='<?' THEN substr(rest,instr(rest,'?>')+2) ELSE substr(rest,instr(rest,'-->')+3) END,?2),n+1 FROM prolog WHERE n<?3 AND ((substr(rest,1,2)='<?' AND instr(rest,'?>')>0) OR (substr(rest,1,4)='<!--' AND instr(rest,'-->')>0))),root AS (SELECT CASE WHEN substr(rest,1,9)='<!DOCTYPE' AND substr(rest,10,1) IN (' ',char(9),char(10),char(13)) THEN ltrim(substr(rest,10),?2) WHEN substr(rest,1,1)='<' THEN substr(rest,2) ELSE '' END raw FROM prolog ORDER BY n DESC LIMIT 1),normalized AS (SELECT "+normalized+" raw FROM root),token AS (SELECT substr(raw,1,instr(raw||' ',' ')-1) name FROM normalized) SELECT nullif(CASE WHEN instr(name,':')>0 THEN substr(name,instr(name,':')+1) ELSE name END,'') name FROM token";
      return(database.query(query).get(text," \t\r\n\uFEFF",censusFixture.maximumXmlPrologSections)as{name:string|null}).name;
    };
    const classify=(value:string,kind:CensusKind,reference=false,content?:string):IdeDisposition=>{
      const query="WITH RECURSIVE parts(rest,part,n) AS (SELECT lower("+(reference?normalized:"replace(?1,char(92),'/')")+")||'/',null,0 UNION ALL SELECT substr(rest,instr(rest,'/')+1),substr(rest,1,instr(rest,'/')-1),n+1 FROM parts WHERE rest<>'') SELECT CASE WHEN ?2=0 AND EXISTS(SELECT 1 FROM parts JOIN json_each(?3) names ON part=names.value) THEN 'excluded' WHEN EXISTS(SELECT 1 FROM parts JOIN json_each(?4) names ON part=names.value) OR EXISTS(SELECT 1 FROM parts WHERE (?2=1 OR (?5<>'directory' AND n=(SELECT max(n) FROM parts))) AND (part IN (SELECT value FROM json_each(?6)) OR EXISTS(SELECT 1 FROM json_each(?7) suffix WHERE substr(part,-length(suffix.value)-1)='.'||suffix.value))) THEN 'retired' ELSE 'accepted' END disposition";
      const disposition=(database.query(query).get(value,reference?1:0,JSON.stringify(censusFixture.excludedDirectories),JSON.stringify(censusFixture.directories),kind,JSON.stringify(censusFixture.files),JSON.stringify(censusFixture.suffixes))as{disposition:IdeDisposition}).disposition;return disposition==="accepted"&&!reference&&kind!=="directory"&&value.toLowerCase().endsWith(".launch")&&content!==undefined&&xmlRootOracle(content)==="launchConfiguration"?"retired":disposition;
    };
    for(const row of censusFixture.pathCases as readonly PathCaseRow[]){expect(retiredIdePath(row.path,row.kind as CensusKind,false,"content"in row?row.content:undefined),row.path).toBe(row.expected);expect(classify(row.path,row.kind as CensusKind,false,"content"in row?row.content:undefined),row.path).toBe(row.expected);}
    const manifestOracle=(document:Record<string,any>,ticket=false):string[]=>{
      const rows=database.query("WITH entries AS (SELECT value,CASE WHEN ?2=0 AND fullkey GLOB '$.nx.*' THEN '$.'||substr(fullkey,6) ELSE fullkey END fullkey,type FROM json_tree(?1)) SELECT value,fullkey FROM entries WHERE type='text' AND ((?2=1 AND (fullkey GLOB '$.tools*' OR fullkey GLOB '$.compounds*') AND (fullkey GLOB '*.command*' OR fullkey GLOB '*.args*' OR fullkey GLOB '*.env*')) OR (?2=0 AND (fullkey GLOB '$.scripts.*' OR fullkey GLOB '$.namedInputs.*' OR fullkey GLOB '$.inputs*' OR fullkey GLOB '$.outputs*' OR fullkey GLOB '$.dependsOn*' OR fullkey GLOB '$.command*' OR fullkey GLOB '$.options*' OR fullkey GLOB '$.configurations*' OR ((fullkey GLOB '$.targets.*' OR fullkey GLOB '$.targetDefaults.*' OR fullkey GLOB '$.tasks*') AND (fullkey GLOB '*.inputs*' OR fullkey GLOB '*.outputs*' OR fullkey GLOB '*.dependsOn*' OR fullkey GLOB '*.options*' OR fullkey GLOB '*.configurations*' OR fullkey GLOB '*.command*')) OR ((fullkey GLOB '$.metadata.semio.dashboard.*' OR fullkey GLOB '$.targets.*.metadata.semio.dashboard.*' OR fullkey GLOB '$.targetDefaults.*.metadata.semio.dashboard.*' OR fullkey GLOB '$.tasks*.metadata.semio.dashboard.*') AND (fullkey GLOB '*.command*' OR fullkey GLOB '*.args*' OR fullkey GLOB '*.env*')))))").all(JSON.stringify(document),ticket?1:0)as{value:string;fullkey:string}[];
      return [...new Set(rows.filter(row=>!((row.fullkey.includes(".inputs")||row.fullkey.includes(".outputs")||row.fullkey.includes(".dependsOn")||row.fullkey.startsWith("$.namedInputs."))&&row.value.trimStart().startsWith("!"))&&classify(row.value,"file",true)==="retired").map(row=>row.value))].sort();
    };
    for(const row of censusFixture.manifestCases){expect(retiredManifestReferences(row.document),row.name).toEqual([...row.expected].sort());expect(manifestOracle(row.document),row.name).toEqual([...row.expected].sort());}
    const roleOracle=(path:string,status:string|null):ManifestRole=>{
      const normalized=path.replaceAll("\\","/").toLowerCase(),declaration=censusFixture.semanticRoles,parts=JSON.stringify(normalized.split("/"));
      return(database.query("SELECT CASE WHEN EXISTS(SELECT 1 FROM json_each(?2) roots WHERE substr(?1,1,length(roots.value)+1)=roots.value||'/') AND EXISTS(SELECT 1 FROM json_each(?3) parts JOIN json_each(?4) names ON parts.value=names.value) THEN 'retained' WHEN (SELECT value FROM json_each(?3) ORDER BY cast(key AS integer) DESC LIMIT 1)=?5 THEN CASE WHEN ?6='open' THEN 'ticket' ELSE 'neutral' END WHEN (SELECT value FROM json_each(?3) ORDER BY cast(key AS integer) DESC LIMIT 1) IN (SELECT value FROM json_each(?7)) THEN 'active' ELSE 'neutral' END role").get(normalized,JSON.stringify(declaration.ticketRoots),parts,JSON.stringify(declaration.retainedInputDirectories),declaration.ticketCommandName,status,JSON.stringify(censusFixture.manifestNames))as{role:ManifestRole}).role;
    };
    for(const row of censusFixture.roleCases as readonly RoleCaseRow[]){
      const role=censusManifestRole(row.path,row.ticketStatus);expect(role,row.path).toBe(row.expectedRole);expect(roleOracle(row.path,row.ticketStatus),row.path).toBe(role);
      if(row.expectedParse==="not-read"){expect(censusManifestDocument(row.text,role)).toBeNull();continue;}
      const independent=ts.parseConfigFileTextToJson(row.path,row.text);expect(independent.error!==undefined,row.path).toBe(row.expectedParse==="invalid");
      if(row.expectedParse==="invalid"){expect(()=>censusManifestDocument(row.text,role),row.path).toThrow();continue;}
      const document=censusManifestDocument(row.text,role)!;expect(independent.config).toEqual(document);expect(retiredManifestReferences(document,role==="ticket")).toEqual(row.expectedReferences);expect(manifestOracle(document,role==="ticket")).toEqual(row.expectedReferences);
    }
    await receiveScriptProcessInvocation(process.env,async original=>{
    const queue=[""],retired:string[]=[],references:{file:string;values:string[]}[]=[],deadline=performance.now()+censusFixture.maximumMilliseconds;let entries=0,directories=0,manifests=0,nextContinuation=0,nextProgress=0;
    const checkpoint=async():Promise<void>=>{checkScriptInvocation(original);const now=performance.now();if(now>deadline)throw Error(`Physical IDE census exceeded ${censusFixture.maximumMilliseconds} ms at ${entries} entries/${directories} directories`);if(now>=nextContinuation){await advanceScriptInvocation(original,"dashboard:ide-census","running");nextContinuation=performance.now()+250;}if(now>=nextProgress){console.log(`[DEBUG] IDE census active: ${entries} entries, ${directories} directories, ${manifests} manifests`);nextProgress=performance.now()+5000;}checkScriptInvocation(original);};
    while(queue.length){
      const relative=queue.pop()!,absolute=join(root,relative),before=lstatSync(absolute);expect(before.isSymbolicLink(),relative).toBe(false);expect(before.isDirectory(),relative).toBe(true);if(++directories>censusFixture.maximumDirectories)throw Error("Physical IDE census exceeded its directory bound: "+directories);await checkpoint();
      for(const entry of readdirSync(absolute,{withFileTypes:true})){
        if(++entries>censusFixture.maximumEntries)throw Error("Physical IDE census exceeded its entry bound: "+entries);if(entries%256===0)await checkpoint();
        const path=relative?relative+"/"+entry.name:entry.name,kind:CensusKind=entry.isSymbolicLink()?"symlink":entry.isDirectory()?"directory":"file",disposition=retiredIdePath(entry.name,kind);
        if(disposition==="excluded")continue;if(disposition==="retired"){retired.push(path);continue;}if(entry.isSymbolicLink())continue;
        if(entry.isDirectory()){queue.push(path);continue;}if(!entry.isFile())continue;const file=join(root,path),name=entry.name.toLowerCase();
        if(name.endsWith(".launch")){const probe=censusReadText(file,censusFixture.contentProbeBytes,true),sourceRoot=censusXmlRoot(probe.text);expect(xmlRootOracle(probe.text)==="launchConfiguration",path).toBe(sourceRoot==="launchConfiguration");if(sourceRoot==="launchConfiguration")retired.push(path);else if(probe.truncated&&sourceRoot===null&&probe.text.trimStart().startsWith("<"))throw Error("XML launch root exceeds content probe bound: "+path);}
        if(!censusFixture.manifestNames.includes(name)||censusManifestRole(path,null)==="retained")continue;
        let status:string|null=null;if(name===censusFixture.semanticRoles.ticketCommandName){const ticket=join(dirname(file),censusFixture.semanticRoles.ticketDocumentName);if(existsSync(ticket)){const metadata=censusManifestDocument(censusReadText(ticket).text,"active")!;status=typeof metadata.status==="string"?metadata.status:null;}}
        const role=censusManifestRole(path,status);expect(roleOracle(path,status),path).toBe(role);if(role==="neutral")continue;
        const document=censusManifestDocument(censusReadText(file).text,role)!;manifests++;const values=retiredManifestReferences(document,role==="ticket");expect(manifestOracle(document,role==="ticket"),path).toEqual(values);for(const command of declaredDashboardToolCommands(document,role==="ticket"))if(!acceptedDashboardToolCommand(command))values.push(command.join(" "));if(values.length)references.push({file:path,values});
      }
      const after=lstatSync(absolute);if(before.dev!==after.dev||before.ino!==after.ino||after.isSymbolicLink())throw Error("Authored directory changed identity during IDE census: "+relative);
    }
    expect(retired.sort()).toEqual([]);expect(references).toEqual([]);console.log(`[DEBUG] No-follow IDE census checked ${entries} physical entries, ${directories} authored directories and ${manifests} manifests; external/generated cache roots were pruned`);
    });
  }finally{database.close();}
},330000);
