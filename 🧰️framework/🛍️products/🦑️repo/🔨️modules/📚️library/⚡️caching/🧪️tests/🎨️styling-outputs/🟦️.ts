import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🎨️ Executes actual styling assemblies after native Nx restoration and compares them with direct MSBuild output. */
export async function testStylingOutputs(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const packagePath = join(fixture.owner, fixture.package), project = JSON.parse(readFileSync(join(workspace, packagePath, "📋️project.json"), "utf8"));
  assert.equal(project.targets.build.cache, true);
  assert.deepEqual(project.targets.build.outputs, [`{projectRoot}/${fixture.output}`], "The assembly must be restored separately from native compiler state");
  assert.equal(project.targets.deps.cache, false);
  assert.ok(project.targets.build.dependsOn.includes("deps"));
  assert.ok(project.targets.build.dependsOn.includes("@semio-tech/ui-styling-tokens:generate"));
  assert.ok(project.targets.build.inputs.some((input: any) => input.dependentTasksOutputFiles === "**/*"));
  const bundle = await require("esbuild").build({ entryPoints: [join(workspace, packagePath, "📜️script.ts")], absWorkingDir: workspace, bundle: true, packages: "external", platform: "node", format: "esm", write: false, metafile: true });
  assert.ok(!Object.keys(bundle.metafile.inputs).some(path => path.endsWith("🎨️styling/🏗️builder/🟦️.ts")), "The .NET command must not import the Python command and test implementation");
  const root = mkdtempSync(join(output, "styling-dotnet-"));
  const put = (path: string, text: string) => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
  for (const path of [join(packagePath, "🔷️.csproj"), join(fixture.owner, fixture.source)]) put(path, readFileSync(join(workspace, path), "utf8"));
  put("🧫️palette.cs", readFileSync(join(workspace, fixture.owner, fixture.source), "utf8"));
  put("package.json", JSON.stringify({ name: "styling-output-fixture", private: true }));
  put("nx.json", JSON.stringify({ useDaemonProcess: false, cacheDirectory: ".nx/cache" }));
  put("Directory.Build.props", "<Project />");
  put("Directory.Build.targets", "<Project />");
  put(".gitignore", "node_modules\n.nx\ndist\nstate\noracle\nconsumer\n.runs\n.deps\n.🧬semio\n");
  put(join(packagePath, "project.json"), JSON.stringify({ name: "styling", targets: {
    deps: { executor: "nx:run-commands", cache: false, outputs: [], options: { command: "bun ./📜️script.ts deps", cwd: "." } },
    generate: { executor: "nx:run-commands", cache: true, inputs: ["{workspaceRoot}/🧫️palette.cs", "{workspaceRoot}/📜️script.ts"], outputs: [`{workspaceRoot}/${fixture.owner}/${fixture.source}`], options: { command: "bun ./📜️script.ts generate", cwd: "." } },
    build: { executor: "nx:run-commands", cache: project.targets.build.cache, dependsOn: ["deps", "generate"], outputs: project.targets.build.outputs,
      inputs: ["{projectRoot}/🔷️.csproj", { dependentTasksOutputFiles: "**/*" }, "{workspaceRoot}/📜️script.ts"], options: { command: "bun ./📜️script.ts build", cwd: "." } }
  } }));
  put("📜️script.ts", `import { appendFileSync, copyFileSync } from "node:fs";
import { join } from "node:path";
import { StylingDotnetBuildScript, StylingDotnetDepsScript } from ${JSON.stringify(join(workspace, fixture.owner, "🏗️builder/🔷️dotnet/📜️script.ts"))};
const root = process.cwd(), command = process.argv[2], Command = command === "deps" ? StylingDotnetDepsScript : StylingDotnetBuildScript;
if (command === "generate") { copyFileSync(join(root, "🧫️palette.cs"), join(root, ${JSON.stringify(join(fixture.owner, fixture.source))})); process.exit(0); }
await new Command(join(root, ${JSON.stringify(packagePath)}), root).run([]);
appendFileSync(join(root, command === "deps" ? ".deps" : ".runs"), command + "\\n");
`);
  put("consumer/consumer.csproj", `<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><TargetFramework>net8.0</TargetFramework><OutputType>Exe</OutputType><ImplicitUsings>enable</ImplicitUsings></PropertyGroup></Project>`);
  put("consumer/Program.cs", `var assembly = System.Reflection.Assembly.LoadFrom(args[0]); Console.Write(assembly.GetType(${JSON.stringify(fixture.type)})!.GetField(${JSON.stringify(fixture.field)})!.GetRawConstantValue());`);
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  const env = { ...process.env, NX_DAEMON: "false", NX_WORKSPACE_ROOT: root, NX_WORKSPACE_ROOT_PATH: root, NX_WORKSPACE_DATA_DIRECTORY: join(root, ".nx/data"), NX_CACHE_DIRECTORY: join(root, ".nx/cache"), SEMIO_REPO_ROOT: root, SEMIO_STYLING_DOTNET_ARTIFACTS_ROOT: join(root, "state"), MSBUILDDISABLENODEREUSE: "1", DOTNET_CLI_TELEMETRY_OPTOUT: "1" };
  const execute = async (args: string[]) => {
    const child = Bun.spawn(args, { cwd: root, env, stdout: "pipe", stderr: "pipe" });
    const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
    assert.equal(code, 0, stdout + stderr); return stdout;
  };
  const cli = join(dirname(require.resolve("nx/package.json")), "dist/bin/nx.js");
  const run = () => execute(["node", cli, "run", "styling:build", "--outputStyle=static"]);
  const runs = () => readFileSync(join(root, ".runs"), "utf8").trim().split("\n").length;
  const published = join(root, packagePath, fixture.output), assembly = join(published, fixture.assembly);
  console.log("[DEBUG] Styling assembly fixture: compiling its independent reflection consumer");
  await execute(["dotnet", "build", "consumer/consumer.csproj", "--output", "consumer/bin", "--nologo"]);
  const value = (path: string) => execute(["dotnet", join(root, "consumer/bin/consumer.dll"), path]);
  const check = async (expected: string) => {
    await execute(["dotnet", "build", join(root, packagePath, "🔷️.csproj"), "--configuration", "Release", "--artifacts-path", join(root, "oracle/state"), "--output", join(root, "oracle/bin"), `-p:PathMap=${join(root, "oracle/state")}=/_/native%2C${root}=/_/`, "-p:ContinuousIntegrationBuild=true", "--nologo"]);
    assert.equal(await value(join(root, "oracle/bin", fixture.assembly)), expected);
    assert.equal(await value(assembly), expected);
    assert.equal(readFileSync(assembly).equals(readFileSync(join(root, "oracle/bin", fixture.assembly))), true, "Separate compiler stores must produce identical styling assembly bytes");
  };
  await run(); assert.equal(runs(), 1); await check(fixture.values[0]);
  const bytes = new Map(readdirSync(published).map(name => [name, readFileSync(join(published, name))]));
  assert.ok(bytes.has(".nx-artifact.json"));
  assert.ok(!bytes.has("project.assets.json"));
  await run(); assert.equal(runs(), 1);
  rmSync(published, { recursive: true });
  await run(); assert.equal(runs(), 1);
  assert.deepEqual(new Map(readdirSync(published).map(name => [name, readFileSync(join(published, name))])), bytes);
  await check(fixture.values[0]);
  const sourcePath = "🧫️palette.cs";
  put(sourcePath, readFileSync(join(root, sourcePath), "utf8").replace(fixture.values[0], fixture.values[1]));
  await run(); assert.equal(runs(), 2); await check(fixture.values[1]);
  assert.equal(readFileSync(join(root, ".deps"), "utf8").trim().split("\n").length, 4);
  console.log("[DEBUG] Styling .NET native Nx cold/warm/restoration/source-change outputs execute and match direct MSBuild bytes; preparation remains uncached PASS");
  rmSync(root, { recursive: true, force: true });
}
