import { expect, spyOn, test } from "bun:test";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import ts from "typescript";
import { compileJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };

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
  for(const row of cases.compounds){const project=JSON.parse(read(row.file)),compound=project.metadata.semio.dashboard.compounds.find((compound:any)=>compound.id===row.id);expect(compound.stop).toBe("together");expect(compound.members.map((member:any)=>member.run)).toEqual(row.members);}
  const phases=manifest.targets["test-build-tool-bazel"].metadata.semio.dashboard.parameters.find((parameter:any)=>parameter.id==="phase");expect(phases.values.map((value:any)=>value.id)).toEqual(cases.phases);for(const value of phases.values)expect(value.args).toEqual([value.id]);expect(phases.default).toBeUndefined();
  const owners=JSON.parse(read("📋️build-owners.json")),ownerSchema=JSON.parse(read("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️owners.json")),ownerOracle=new Ajv2020({strict:false}).compile(ownerSchema);expect(ownerOracle(owners),JSON.stringify(ownerOracle.errors)).toBe(true);expect(compileJsonSchemaSubset(ownerSchema).validate(owners)).toEqual([]);
  for(const name of cases.nativeOwners){const owner=owners.owners.find((owner:any)=>owner.name===name);expect(owner,name).toBeDefined();expect(JSON.parse(read(owner.manifest)).name).toBe(name);expect(Bun.TOML.parse(read(owner.root+"/Cargo.toml")).package).toBeDefined();}
  const course=JSON.parse(read(cases.course.file));expect(validate(course),JSON.stringify(validate.errors)).toBe(true);expect(own.validate(course)).toEqual([]);for(const target of cases.course.releaseTargets){const mode=course.targets[target].metadata?.semio?.dashboard?.parameters?.find((parameter:any)=>parameter.id==="execution-mode");expect(mode,target).toMatchObject({kind:"choice",default:"release",values:[{id:"release",env:{SEMIO_BUILD_MODE:"release"}}]});expect(Object.keys(mode.title).sort()).toEqual(["de","en"]);}for(const target of cases.course.readyTargets)expect(course.targets[target].metadata.semio.dashboard.ready).toEqual({port:6061,portEnv:"TEACHING_ARCHITECTURE_COURSE_PORT"});
  console.log("[DEBUG] Native owner controls preserve backend selection, nine Bazel phases and four supervised compounds");
});

test("dashboard control regression is admitted by our schema compiler and Ajv", () => {
  const own = compileJsonSchemaSubset(schema);
  const oracle = new Ajv({ strict: false }).compile(schema);
  for (const value of [fixture, {}, { ...fixture, absentFiles: [] }, { ...fixture, targets: [{ file: "x", absent: [], present: [] }] }]) {
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
    if ("inputTarget" in row) {
      const inputs = manifest.targets[row.inputTarget].inputs;
      for (const input of row.absentInputs) expect(inputs).not.toContain(input);
      for (const input of row.presentInputs) expect(inputs).toContain(input);
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
