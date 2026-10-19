import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { lstatSync, mkdirSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import findUp from "find-up";
import Ajv from "ajv";
import { findWorkspaceRoot } from "../../🟦️.ts";
import corpus from "./🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";

const root = findWorkspaceRoot(import.meta.dir), host = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test");
const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
if (!output) throw Error("Workspace identity tests require explicit SEMIO_TEST_ARTIFACT_DIR");
const generated = resolve(output, `wi-${process.pid}`);
mkdirSync(generated, { recursive: true });

function run(command: string, args: string[], cwd = generated): string {
  const result = spawnSync(command, args, { cwd, encoding: "utf8", timeout: 120_000, env: { ...process.env, PYTHONDONTWRITEBYTECODE: "1", GOWORK: "off", DOTNET_NOLOGO: "1", DOTNET_SKIP_FIRST_TIME_EXPERIENCE: "1" } });
  if (result.error || result.status !== 0) throw Error(`${command}: ${result.error ?? result.status}\n${result.stdout}\n${result.stderr}`);
  return result.stdout.trim();
}

function oracle(start: string, boundary: string): string | null {
  try {
    return findUp.sync(directory => {
      const markers = ["📋️project.json", "package.json"].map(name => join(directory, name));
      if (markers.every(path => lstatSync(path, { throwIfNoEntry: false })?.isFile())) {
        const workspaces = JSON.parse(readFileSync(markers[1]!, "utf8")).workspaces;
        if (Array.isArray(workspaces) || workspaces !== null && typeof workspaces === "object") return directory;
      }
      return directory === boundary ? findUp.stop : undefined;
    }, { cwd: start, type: "directory" }) ?? null;
  } catch { return null; }
}

function discover(start: string, boundary: string): string | null {
  try { return findWorkspaceRoot(start, { stopAt: boundary }); } catch { return null; }
}

function materialize(index: number, rows = corpus.cases): string {
  const row = rows[index]!;
  const directory = join(generated, String(index), ...(row.expected === null ? Array(32).fill("a") : []));
  mkdirSync(join(directory, row.start), { recursive: true });
  for (const [name, content] of Object.entries(row.files)) {
    const path = join(directory, name);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, content);
  }
  return directory;
}

test("workspace identity schema and independent ancestry oracle agree", () => {
  expect(new Ajv({ strict: false }).validate(schema, corpus)).toBe(true);
  for (const [index, row] of corpus.cases.entries()) {
    const directory = materialize(index);
    const expected = row.expected === null ? null : resolve(directory, row.expected);
    expect(oracle(join(directory, row.start), directory), row.id).toBe(expected);
    expect(discover(join(directory, row.start), directory), row.id).toBe(expected);
  }
});

test("native Rust, Python, Go and C# hosts resolve the same neutral workspace vectors", () => {
  const rustLibrary = join(generated, "libworkspace_host.rlib"), rustMain = join(generated, "main.rs"), rustBinary = join(generated, process.platform === "win32" ? "rust.exe" : "rust");
  run("rustc", ["--edition=2021", "--crate-type=rlib", "--crate-name=workspace_host", join(host, "🦀️.rs"), "-o", rustLibrary]);
  writeFileSync(rustMain, 'fn main()->std::process::ExitCode {workspace_host::run_main(workspace_host::Adapter::new("rust").subject("workspace", |context| Ok(workspace_host::Outcome::projection(workspace_host::Json::String(context.repo_root.to_string_lossy().into_owned())))))}');
  run("rustc", ["--edition=2021", "--crate-name=workspace_probe", rustMain, "--extern", `workspace_host=${rustLibrary}`, "-o", rustBinary]);
  const goDirectory = join(generated, "go");
  mkdirSync(goDirectory, { recursive: true });
  writeFileSync(join(goDirectory, "go.mod"), `module workspace-probe\n\ngo 1.25\n\nrequire semio.tech/repo/test v0.0.0\nreplace semio.tech/repo/test => ${JSON.stringify(join(host, "📦️packages/🐹️go"))}\n`);
  writeFileSync(join(goDirectory, "main.go"), 'package main\nimport t "semio.tech/repo/test"\nfunc main(){t.RunMain(t.NewAdapter("go").Subject("workspace",func(c *t.Context)(t.Outcome,error){return t.Outcome{Projection:c.RepoRoot},nil}))}\n');
  const goBinary = join(goDirectory, process.platform === "win32" ? "probe.exe" : "probe");
  run("go", ["build", "-o", goBinary, "."], goDirectory);
  const csharp = join(generated, "cs");
  mkdirSync(csharp, { recursive: true });
  const csharpHost = join(host, "🖥️host/🔷️.cs").replaceAll("&", "&amp;").replaceAll('"', "&quot;");
  writeFileSync(join(csharp, "probe.csproj"), `<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings><Nullable>enable</Nullable></PropertyGroup><ItemGroup><Compile Include="${csharpHost}" Link="Host.cs" /></ItemGroup></Project>`);
  writeFileSync(join(csharp, "Program.cs"), 'using System.Reflection;using System.Text.Json;try{var method=typeof(Semio.Repo.Test.TestHost).GetMethod("RepoRootFrom",BindingFlags.Static|BindingFlags.NonPublic)!;Console.WriteLine(JsonSerializer.Serialize(method.Invoke(null,new object[]{args[0]})));}catch{Console.WriteLine("null");}');
  run("dotnet", ["build", join(csharp, "probe.csproj"), "--nologo", "--verbosity", "quiet"]);
  const python = 'import json,runpy,sys\nhost=runpy.run_path(sys.argv[1])\ntry: print(json.dumps(host["_repo_root_from"](sys.argv[2])))\nexcept Exception: print("null")';
  for (const [index, row] of corpus.cases.entries()) {
    const directory = materialize(index);
    const start = join(directory, row.start);
    expect(start.length).toBeLessThanOrEqual(256);
    mkdirSync(start, { recursive: true });
    const expected = row.expected === null ? null : resolve(directory, row.expected);
    expect(JSON.parse(run(process.platform === "win32" ? "python" : "python3", ["-c", python, join(host, "🖥️host/🐍️.py"), start])), `Python ${row.id}`).toBe(expected);
    expect(JSON.parse(run("dotnet", [join(csharp, "bin/Debug/net10.0/probe.dll"), start])), `C# ${row.id}`).toBe(expected);
    const plan = join(directory, "plan.json");
    writeFileSync(plan, JSON.stringify({ schemaVersion: 2, owner: "workspace", case: row.id, role: "subject", implementation: "workspace", workDir: start, outputDir: directory, artifactDir: directory, inputs: [], scenarios: [{ id: "workspace", name: "workspace", level: "fundamental", mode: "batch" }] }));
    for (const [language, executable] of [["Rust", rustBinary], ["Go", goBinary]]) {
      const resultPath = join(directory, `${language}.jsonl`);
      const result = spawnSync(executable!, ["--plan", plan, "--out", resultPath], { encoding: "utf8", timeout: 10_000 });
      expect(result.error, `${language} ${row.id}`).toBeUndefined();
      if (expected === null) expect(result.status, `${language} ${row.id}`).toBe(2);
      else {
        expect(result.status, `${language} ${row.id}: ${result.stderr}`).toBe(0);
        expect(JSON.parse(readFileSync(resultPath, "utf8")).output.projection, `${language} ${row.id}`).toBe(expected);
      }
    }
  }
}, 120_000);

test("workspace identity production consumers mount their actual neutral helper",()=>{
 const source=readFileSync(join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"),"utf8");
 expect(source).toContain("workspace_identity::is_workspace_root(path,");expect(source).toContain("mod workspace_identity;");expect(source).not.toMatch(/#\[cfg\(test\)\]\s*#\[path\s*=\s*"[^"\n]+workspace\/🦀️\.rs"\]\s*mod workspace_identity;/);
 console.log("[DEBUG] MCP production path admission mounts the defining tool-neutral workspace identity outside test-only cfg; native compilation remains separate");
});
