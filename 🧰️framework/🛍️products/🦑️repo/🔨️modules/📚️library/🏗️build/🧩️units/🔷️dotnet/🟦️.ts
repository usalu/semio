import {mkdirSync, readFileSync, readdirSync, rmSync} from "node:fs";
import {join} from "node:path";
import {sha256V1} from "../🔏️key/🟦️.ts";
import {compareCodePointsV1, sortedCodePointsV1, type UnitDependencyV1, type UnitGraphV1, type UnitsPolicyV1, type UnitV1} from "../🕸️graph/🟦️.ts";
import type {ExecuteOutcomeV1, ExecuteRequestV1} from "../🏃️engine/🟦️.ts";

/** 🔷️ What the planner reads from one SDK-style project file. */
export type CsprojFactsV1 = {
  readonly sdk: string | null; readonly properties: Readonly<Record<string, string>>; readonly projectReferences: readonly string[]; readonly packageReferences: readonly {readonly id: string; readonly version: string}[];
  readonly compileIncludes: readonly string[]; readonly compileRemoves: readonly string[]; readonly defaultCompile: boolean;
};
const attribute = (tag: string, name: string): string | undefined => new RegExp(`\\b${name}\\s*=\\s*("([^"]*)"|'([^']*)')`, "i").exec(tag)?.slice(2).find(value => value !== undefined);
/** 📄️ Reads a project file: SDK, unconditional properties (a property group with a `Condition` is ignored), project and package references, `Compile` includes and removes, and whether the default `**\/*.cs` glob is on. */
export function parseCsprojV1(text: string): CsprojFactsV1 {
  const clean = text.replace(/<!--[\s\S]*?-->/g, ""), properties: Record<string, string> = {};
  const sdk = attribute(/<Project\b[^>]*>/i.exec(clean)?.[0] ?? "", "Sdk") ?? null;
  for (const group of clean.matchAll(/<PropertyGroup\b([^>]*)>([\s\S]*?)<\/PropertyGroup>/gi)) {
    if (/\bCondition\s*=/i.test(group[1]!)) continue;
    for (const property of group[2]!.matchAll(/<([A-Za-z_][\w.]*)\b[^>]*>([\s\S]*?)<\/\1>/g)) properties[property[1]!] = property[2]!.trim();
  }
  const projectReferences: string[] = [], packageReferences: {id: string; version: string}[] = [], compileIncludes: string[] = [], compileRemoves: string[] = [];
  for (const group of clean.matchAll(/<ItemGroup\b([^>]*)>([\s\S]*?)<\/ItemGroup>/gi)) {
    if (/\bCondition\s*=/i.test(group[1]!)) continue;
    for (const item of group[2]!.matchAll(/<(ProjectReference|PackageReference|Compile)\b([^>]*?)(\/>|>([\s\S]*?)<\/\1>)/gi)) {
      const kind = item[1]!.toLowerCase(), tag = item[2]!, body = item[4] ?? "", include = attribute(tag, "Include"), remove = attribute(tag, "Remove");
      if (kind === "projectreference" && include) projectReferences.push(include);
      else if (kind === "packagereference" && include) packageReferences.push({id: include, version: attribute(tag, "Version") ?? /<Version>([^<]*)<\/Version>/i.exec(body)?.[1]?.trim() ?? ""});
      else if (kind === "compile") {if (include) compileIncludes.push(include); if (remove) compileRemoves.push(remove);}
    }
  }
  return {sdk, properties, projectReferences, packageReferences, compileIncludes, compileRemoves, defaultCompile: (properties.EnableDefaultCompileItems ?? "true").toLowerCase() !== "false"};
}

const normalize = (path: string): string => {const out: string[] = []; for (const part of path.split("\\").join("/").split("/")) {if (part === "" || part === ".") continue; if (part === "..") out.pop(); else out.push(part);} return out.join("/");};
const joinPath = (...parts: readonly string[]): string => normalize(parts.filter(part => part !== "").join("/"));
/** 🔨️ A MSBuild glob (`*`, `?`, `**`) as an expression over normalized workspace paths. */
export function msbuildGlobV1(glob: string): RegExp {
  const parts = normalize(glob).split("/"), pieces = parts.map((part, index) => part === "**" ? (index === parts.length - 1 ? ".*" : "(?:[^/]+/)*") : part.replace(/[.+^${}()|[\]\\]/g, "\\$&").replace(/\*/g, "[^/]*").replace(/\?/g, "[^/]") + (index === parts.length - 1 ? "" : "/"));
  return new RegExp("^" + pieces.join("") + "$", "u");
}

/** ➡️ One planned project. */
export type DotnetProjectV1 = {readonly csproj: string; readonly directory: string; readonly assemblyName: string; readonly frameworks: readonly string[]; readonly outputType: string; readonly compile: readonly string[]; readonly projectReferences: readonly string[]; readonly packageReferences: readonly string[]; readonly test: boolean};
export type DotnetWorkspaceSnapshotV1 = {readonly files: readonly string[]; readonly read: (path: string) => string};

/** 🕸️ Plans the unit graph of a .NET workspace: one unit per project file with `interface` edges (reference assembly) between library projects, `implementation` edges from executables and test projects, NuGet packages as exact externals, the SDK in the toolchain identity. Compile items follow MSBuild: the default glob (below the project directory, outside `bin`/`obj` and nested projects) unless disabled, plus explicit includes, minus removes. */
export function planDotnetUnitsV1(snapshot: DotnetWorkspaceSnapshotV1, options: {readonly policy: UnitsPolicyV1; readonly toolchain: string; readonly configuration?: string}): {readonly graph: UnitGraphV1; readonly projects: readonly DotnetProjectV1[]} {
  const configuration = options.configuration ?? "Release", csprojs = sortedCodePointsV1(snapshot.files.filter(path => path.endsWith(".csproj"))), directories = csprojs.map(path => path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : "");
  const fileSet = new Set(snapshot.files), projects: (DotnetProjectV1 & {readonly facts: CsprojFactsV1})[] = [];
  for (const [index, csproj] of csprojs.entries()) {
    const facts = parseCsprojV1(snapshot.read(csproj)), directory = directories[index]!, base = csproj.slice(csproj.lastIndexOf("/") + 1, -".csproj".length);
    const frameworks = (facts.properties.TargetFrameworks ?? facts.properties.TargetFramework ?? "").split(";").map(value => value.trim()).filter(Boolean);
    const compile = new Set<string>();
    if (facts.defaultCompile) for (const path of snapshot.files) {
      if (!path.endsWith(".cs") || !(directory === "" || path.startsWith(directory + "/"))) continue;
      const rest = directory === "" ? path : path.slice(directory.length + 1);
      if (rest.split("/").slice(0, -1).some(segment => segment === "bin" || segment === "obj")) continue;
      if (directories.some(other => other !== directory && other.length > directory.length && (other === "" ? false : path.startsWith(other + "/")))) continue;
      compile.add(path);
    }
    for (const include of facts.compileIncludes) {
      const absolute = joinPath(directory, include);
      if (/[*?]/.test(include)) {const expression = msbuildGlobV1(absolute); for (const path of snapshot.files) if (expression.test(path)) compile.add(path);} else if (fileSet.has(absolute)) compile.add(absolute);
    }
    for (const remove of facts.compileRemoves) {const expression = msbuildGlobV1(joinPath(directory, remove)); for (const path of [...compile]) if (expression.test(path)) compile.delete(path);}
    const test = facts.packageReferences.some(row => row.id.toLowerCase() === "microsoft.net.test.sdk") || (facts.properties.IsTestProject ?? "").toLowerCase() === "true";
    projects.push({csproj, directory, assemblyName: facts.properties.AssemblyName ?? base, frameworks, outputType: facts.properties.OutputType ?? "Library", compile: sortedCodePointsV1(compile), projectReferences: facts.projectReferences.map(reference => joinPath(directory, reference)), packageReferences: facts.packageReferences.map(row => `${row.id}@${row.version}`), test, facts});
  }
  const unitId = (csproj: string): string => "dotnet:" + csproj.slice(0, -".csproj".length), byCsproj = new Map(projects.map(project => [project.csproj, project] as const));
  const ancestors = (directory: string, names: readonly string[]): string[] => {
    const found: string[] = [];
    for (let at = directory; ; at = at.includes("/") ? at.slice(0, at.lastIndexOf("/")) : "") {
      for (const name of names) {const candidate = at === "" ? name : at + "/" + name; if (fileSet.has(candidate)) found.push(candidate);}
      if (at === "") break;
    }
    return found;
  };
  const units: UnitV1[] = projects.map(project => {
    const library = project.outputType.toLowerCase() === "library" && !project.test, dependencies = new Map<string, UnitDependencyV1>();
    for (const reference of project.projectReferences) {
      const target = byCsproj.get(reference);
      if (!target || target.outputType.toLowerCase() !== "library" || target.test) continue;
      dependencies.set(unitId(reference), {unit: unitId(reference), use: library ? "interface" : "implementation"});
    }
    const inputs = sortedCodePointsV1(new Set([project.csproj, ...ancestors(project.directory, ["Directory.Build.props", "Directory.Build.targets", "Directory.Packages.props", "global.json", "nuget.config", "NuGet.Config"]), ...(fileSet.has(joinPath(project.directory, "packages.lock.json")) ? [joinPath(project.directory, "packages.lock.json")] : [])])).filter(path => !project.compile.includes(path));
    return {
      id: unitId(project.csproj), language: "dotnet", kind: project.test ? "test" : library ? "library" : "binary", label: project.assemblyName, sources: [...project.compile], inputs,
      dependencies: [...dependencies.values()].sort((a, b) => compareCodePointsV1(a.unit, b.unit)), externals: sortedCodePointsV1(project.packageReferences.map(row => "nuget:" + row)),
      interface: library ? {kind: "dotnet-reference-assembly"} : null, outputs: [`${project.assemblyName}.dll`], toolchain: {id: options.toolchain, parts: [`frameworks=${project.frameworks.join(";")}`, `configuration=${configuration}`, `nullable=${project.facts.properties.Nullable ?? ""}`, `lang=${project.facts.properties.LangVersion ?? ""}`]},
      action: {
        argv: ["dotnet", project.test ? "test" : "build", project.csproj, "-c", configuration, "--nologo", "-p:ProduceReferenceAssembly=true", "-p:Deterministic=true", "-p:DeterministicSourcePaths=false", "-p:DebugType=none", "-p:UseSharedCompilation=false", "-p:BaseIntermediateOutputPath={out}/obj/", "-p:OutDir={out}/bin/"],
        env: {DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1", MSBUILDDISABLENODEREUSE: "1", SEMIO_DOTNET_ASSEMBLY: project.assemblyName, SEMIO_BAZEL_INPUTS: "sources"},
      },
    };
  });
  return {graph: {version: 1, units}, projects: projects.map(({facts, ...project}) => (void facts, project))};
}

const findFiles = (root: string, pattern: RegExp): string[] => {
  const found: string[] = [];
  const walk = (directory: string): void => {try {for (const entry of readdirSync(directory, {withFileTypes: true})) {const path = join(directory, entry.name); if (entry.isDirectory()) walk(path); else if (pattern.test(path)) found.push(path);}} catch {/* absent */}};
  walk(root);
  return found.sort();
};
/** 🏭️ The process effect of the .NET executor. */
export type DotnetRunnerV1 = (argv: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string>>; readonly signal: AbortSignal}) => Promise<{readonly code: number; readonly output: string}>;
/** 🏗️ Executes a .NET unit with `dotnet build` into private obj and bin directories (`ProduceReferenceAssembly`, deterministic): the interface digest is the SHA-256 of the reference assembly (metadata of the public surface only, so a private edit leaves it unchanged), the output digest that of the implementation assembly. A test project runs `dotnet test` and its output digest is the verdict. */
export function dotnetUnitExecutorV1(options: {readonly workspace: string; readonly outputRoot: string; readonly run: DotnetRunnerV1}): (request: ExecuteRequestV1) => Promise<ExecuteOutcomeV1> {
  return async request => {
    const {unit} = request, out = join(options.outputRoot, sha256V1(unit.id).slice(0, 24));
    rmSync(out, {recursive: true, force: true}); mkdirSync(out, {recursive: true});
    const started = performance.now(), argv = [...unit.action.argv.map(part => part.split("{out}").join(out)), `-p:PathMap=${options.workspace}=/src`], result = await options.run(argv, {cwd: options.workspace, env: {...unit.action.env, DOTNET_CLI_HOME: join(out, "home")}, signal: request.signal}), milliseconds = Math.round(performance.now() - started);
    if (unit.kind === "test") return {ok: result.code === 0, diagnostics: result.code === 0 ? "" : result.output.trim(), outputs: sha256V1(result.code === 0 ? "ok" : "fail"), interface: null, milliseconds};
    if (result.code !== 0) return {ok: false, diagnostics: result.output.trim(), outputs: sha256V1("failed:" + result.output), interface: null, milliseconds};
    const assembly = unit.action.env.SEMIO_DOTNET_ASSEMBLY ?? "", implementation = findFiles(join(out, "bin"), new RegExp(`/${assembly.replace(/[.+^${}()|[\]\\]/g, "\\$&")}\\.dll$`, "u"))[0], reference = findFiles(join(out, "obj"), new RegExp(`/ref/${assembly.replace(/[.+^${}()|[\]\\]/g, "\\$&")}\\.dll$`, "u"))[0];
    if (!implementation) return {ok: false, diagnostics: `no ${assembly}.dll below ${join(out, "bin")}`, outputs: sha256V1("missing"), interface: null, milliseconds};
    if (unit.interface && !reference) return {ok: false, diagnostics: `no reference assembly for ${assembly}`, outputs: sha256V1("missing"), interface: null, milliseconds};
    return {ok: true, diagnostics: "", outputs: sha256V1(readFileSync(implementation)), interface: unit.interface ? sha256V1(readFileSync(reference!)) : null, milliseconds};
  };
}
