import { REPO_TEST_DOMAIN_REL, REPO_TEST_RUST_PACKAGE_REL, REPO_TEST_GO_PACKAGE_REL, REPO_TEST_PYTHON_HOST_REL, REPO_TEST_DOTNET_PACKAGE_REL, type MaterializedHost } from "../../🧱️contract/🟦️.ts";
import {
  type DiscoveredCase,
  type Implementation,
  type OracleHostPackage,
  type TestRole,
  agentCacheRoot,
  digest,
  loadOracleRegistry,
  markOutputDir,
  markRunComplete,
  oracleHostModule,
  oracleHostPackagesFor,
  testCacheDir,
  testTaxonomy,
} from "../../📦️packages/🟦️typescript/🟦️.ts";
import { repoToolCacheEnv, runProbe, testLevelBudgetMs } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { cargoTargetDirectory } from "../../../📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { rustSubjectPackage } from "../../🕸️dependencies/🟨️.mjs";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { delimiter, join, relative, sep } from "node:path";
import { createRequire } from "node:module";

/** 🦀️ Reads Cargo's machine output instead of assuming a target triple, profile directory or executable suffix. */
export function rustHostExecutableFromCargo(stdout: string): string | null {
  let executable: string | null = null;
  for (const line of stdout.split(/\r?\n/)) {
    if (line.trim() === "") continue;
    try {
      const message = JSON.parse(line) as { reason?: string; executable?: unknown; target?: { kind?: unknown; name?: unknown } };
      if (message.reason === "compiler-artifact" && message.target?.name === "host" && Array.isArray(message.target.kind) && message.target.kind.includes("bin") && typeof message.executable === "string") executable = message.executable;
    } catch {}
  }
  return executable;
}

export function hostDirFor(repoRoot: string, discovered: DiscoveredCase, role: TestRole, implementation: Implementation): string {
  const dir = join(testCacheDir(repoRoot, "hosts"), `${discovered.projectName}-${role}-${implementation}`);
  // 🧾️ A generated host is deletable state, so it carries the same ownership marker as every other
  // output root — an unmarked directory is never removed by `clean test`.
  markOutputDir(repoRoot, dir, { testId: `${discovered.owner}::${discovered.case}`, cacheKey: `${role}:${implementation}` });
  markRunComplete(dir);
  return dir;
}

/**
 * 🔬️ Whether this repository actually SHIPS an implementation of the case's owner in
 * `implementation`'s language — the owner root, or the nearest ancestor of it, carrying a package
 * directory for that language.
 *
 * The subject role means "this repository's own implementation, on the same inputs". An adapter file
 * in a language the owner ships no package in exists to HOST a reference library (that is what every
 * `🐍️component.py` in this repository is), so there is no subject there to dispatch — and asking for
 * one produces an `adapter has no subject registration` error for every scenario of the case, which
 * then enters the parity ratio as a null projection and drags a fully-passing case to zero. That is
 * not evidence of anything; it is the coordinator asking a question the taxonomy already answers.
 *
 * This is deliberately NOT "skip an implementation whose adapter registers no subject handlers":
 * an owner that ships a package in a language and whose adapter then forgets a subject registration
 * must still fail, loudly, per scenario. The language dir names come from the taxonomy's own
 * `testImplementationIds`, so no language is named here.
 */
export function ownerShipsImplementation(repoRoot: string, discovered: DiscoveredCase, implementation: Implementation): boolean {
  const languageDir = Object.entries(testTaxonomy(repoRoot).testImplementationIds).find(([, id]) => id === implementation)?.[0];
  if (languageDir === undefined) return false;
  let dir = discovered.owner;
  for (let depth = 0; depth < 16; depth += 1) {
    if (existsSync(join(repoRoot, dir, "📦️packages", languageDir))) return true;
    const parent = dir.split("/").slice(0, -1).join("/");
    if (parent === "" || parent === dir) break;
    dir = parent;
  }
  return false;
}

/**
 * 🧩️ The native oracle packages this case's OWNER contributes, resolved from the discovered
 * contribution manifests. The framework links whatever an owner declares; it never names a package,
 * a plugin or a format, so a new artifact family needs no edit here.
 */
export function contributedOraclePackages(repoRoot: string, discovered: DiscoveredCase, implementation: Implementation): OracleHostPackage[] {
  return oracleHostPackagesFor(loadOracleRegistry(repoRoot), discovered.owner, implementation);
}

/** 🦀️ Materializes a standalone cache-local integration crate that links the adapter and the host support crate by path. */
export function materializeRustHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "rust");
  const adapterAbs = join(repoRoot, discovered.adapters.rust!);
  const sut = rustSubjectPackage(repoRoot, discovered.owner);
  const declared = contributedOraclePackages(repoRoot, discovered, "rust");
  // 🦀️A Cargo dependency is linked by path or it is not linked at all; a crates.io coordinate would
  // be an unreviewed third-party dependency of the generated host, which is what the local-crate
  // rule exists to prevent.
  const oraclePackages = declared.filter((entry) => entry.path !== undefined);
  const problems = declared.filter((entry) => entry.path === undefined).map((entry) => `${discovered.caseDir}: rust oracle host package ${entry.package} declares no path — a Rust host links contributed crates by path`);
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(
    join(dir, "Cargo.toml"),
    [
      "# 🤖️ Generated by `bun ./📜️script.ts <phase>` — safe to delete, never commit.",
      "[workspace]",
      "",
      "[package]",
      `name = "semio-test-host-${discovered.case.replace(/[^a-z0-9]+/g, "-")}"`,
      'version = "0.0.0"',
      'edition = "2021"',
      "",
      "[[bin]]",
      'name = "host"',
      'path = "src/main.rs"',
      "",
      // 🔮️The subject crate is OPTIONAL and reached only through the `sut` feature. §5.3 of the
      // frozen plan requires the oracle-only test to pass WITHOUT invoking the local implementation,
      // so the oracle role must not link — or even compile — the subject. An adapter therefore gates
      // its subject half with `#[cfg(feature = "sut")]`, and this crate turns that feature on for
      // the subject role only.
      ...(sut === null ? [] : ["[features]", `sut = ["dep:${sut.name}"]`, ""]),
      "[dependencies]",
      `semio-repo-test-host = { path = ${JSON.stringify(join(repoRoot, REPO_TEST_RUST_PACKAGE_REL))} }`,
      // 🧩️Whatever the owner contributed, exactly as the owner declared it.
      ...oraclePackages.map(
        (entry) => `${entry.package} = { path = ${JSON.stringify(join(repoRoot, entry.path!))}${(entry.features ?? []).length > 0 ? `, features = [${(entry.features ?? []).map((feature) => JSON.stringify(feature)).join(", ")}]` : ""} }`,
      ),
      ...(sut === null ? [] : [`${sut.name} = { path = ${JSON.stringify(join(repoRoot, sut.path))}, default-features = false, optional = true }`]),
      "",
    ].join("\n"),
  );
  writeFileSync(
    join(dir, "src", "main.rs"),
    [
      "// 🤖️ Generated native entrypoint. The adapter below is the committed, taxonomy-named source.",
      `#[path = ${JSON.stringify(adapterAbs)}]`,
      "mod adapter;",
      "",
      "fn main() -> std::process::ExitCode {",
      "    semio_repo_test_host::run_main(adapter::adapter())",
      "}",
      "",
    ].join("\n"),
  );
  return {
    command: "",
    args: ["--plan", planPath, "--out", outPath],
    cwd: repoRoot,
    env: { ...process.env, CARGO_TARGET_DIR: cargoTargetDirectory(repoRoot) },
    hostDir: dir,
    problems,
    preparation: {
      command: "cargo",
      args: ["build", "--quiet", "--manifest-path", join(dir, "Cargo.toml"), "--message-format", "json-render-diagnostics", ...(sut !== null && role === "subject" ? ["--features", "sut"] : [])],
      executableFromStdout: rustHostExecutableFromCargo,
    },
  };
}

/** 🐹️ Materializes a cache-local Go module whose generated entrypoint delegates to the committed adapter. */
export function materializeGoHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "go");
  const adapterAbs = join(repoRoot, discovered.adapters.go!);
  writeFileSync(
    join(dir, "go.mod"),
    ["// 🤖️ Generated — safe to delete, never commit.", "module semio.test/host", "", "go 1.23", "", "require semio.tech/repo/test v0.0.0", "", `replace semio.tech/repo/test => ${join(repoRoot, REPO_TEST_GO_PACKAGE_REL)}`, ""].join("\n"),
  );
  writeFileSync(join(dir, "adapter.go"), readFileSync(adapterAbs, "utf8").replace(/^package\s+\w+/m, "package main"));
  writeFileSync(join(dir, "main.go"), ["// 🤖️ Generated native entrypoint.", "package main", "", 'import host "semio.tech/repo/test"', "", "func main() {", "\thost.RunMain(Adapter())", "}", ""].join("\n"));
  return { command: "go", args: ["run", ".", "--plan", planPath, "--out", outPath], cwd: dir, env: repoToolCacheEnv(repoRoot, { ...process.env, GOFLAGS: "-mod=mod", GOWORK: "off" }), hostDir: dir, problems: [] };
}

/**
 * 🐍️ The cache-local interpreter the Python host runs under, carrying exactly the external
 * distributions the owners declared.
 *
 * A virtual environment, never the system interpreter: a test host may not mutate the machine it
 * runs on. It is created with `--system-site-packages` so a distribution the machine already
 * provides is REUSED rather than downloaded, which is what keeps a zero-touch checkout working
 * offline; anything still missing is installed INTO the environment, where it stays isolated. The
 * environment is keyed by the declared package set, so it is built once and reused by every run and
 * every case that declares the same set, and rebuilt the moment the declaration changes.
 */
/** 🐍️ The interpreter oracle hosts are provisioned from: `SEMIO_PYTHON`, else the repository's own
 * `.venv` (the only interpreter guaranteed to satisfy `pyproject.toml`'s `requires-python`), else the
 * `python3` on `PATH`. */
export function oracleHostPython(repoRoot: string): string {
  if (process.env.SEMIO_PYTHON) return process.env.SEMIO_PYTHON;
  const venv = join(repoRoot, ".venv", process.platform === "win32" ? "Scripts" : "bin", process.platform === "win32" ? "python.exe" : "python3");
  return existsSync(venv) ? venv : "python3";
}

export function provisionPythonInterpreter(repoRoot: string, base: string, declared: readonly OracleHostPackage[]): { interpreter: string; problems: string[] } {
  const external = declared.filter((entry) => entry.path === undefined);
  if (external.length === 0) return { interpreter: base, problems: [] };
  const specs = [...external]
    .map((entry) => ({ spec: entry.version === undefined ? entry.package : `${entry.package}==${entry.version}`, module: oracleHostModule(entry), package: entry.package, version: entry.version }))
    .sort((a, b) => a.spec.localeCompare(b.spec));
  const signature = specs.map((entry) => entry.spec).join(" ");
  const dir = join(testCacheDir(repoRoot, "hosts"), `python-env-${digest(`${base}\n${signature}`)}`);
  const interpreter = join(dir, process.platform === "win32" ? "Scripts" : "bin", process.platform === "win32" ? "python.exe" : "python3");
  const stampPath = join(dir, "🧾️packages.json");
  const stamp = existsSync(stampPath) ? (JSON.parse(readFileSync(stampPath, "utf8")) as { signature?: string }) : null;
  if (stamp?.signature === signature && existsSync(interpreter)) return { interpreter, problems: [] };

  markOutputDir(repoRoot, dir, { testId: "hosts::python-env", cacheKey: `python-env:${signature}` });
  const problems: string[] = [];
  if (!existsSync(interpreter)) {
    const created = runProbe(base, ["-m", "venv", "--system-site-packages", dir], { cwd: repoRoot, budgetMs: testLevelBudgetMs("long") });
    if ((created.status ?? 1) !== 0 || !existsSync(interpreter)) {
      problems.push(`python oracle host: cannot create the cache-local environment at ${relative(repoRoot, dir).split(sep).join("/")} with \`${base} -m venv\` — ${created.stderr.trim() || `exit ${created.status}`}`);
      return { interpreter: base, problems };
    }
  }
  // 🔎️Importable AND at the declared version. Checking only importability would let a declared pin
  // be silently satisfied by whatever the machine happened to have, which is the same as not
  // declaring one.
  const present = (entry: { spec: string; module: string; package: string; version?: string }): boolean => {
    const probe = runProbe(interpreter, ["-c", `import ${entry.module}, importlib.metadata as meta; print(meta.version(${JSON.stringify(entry.package)}))`], { cwd: repoRoot, budgetMs: testLevelBudgetMs("quick") });
    return (probe.status ?? 1) === 0 && (entry.version === undefined || probe.stdout.trim() === entry.version);
  };
  for (const entry of specs) {
    if (present(entry)) continue;
    const installed = runProbe(interpreter, ["-m", "pip", "install", "--disable-pip-version-check", entry.spec], { cwd: repoRoot, budgetMs: testLevelBudgetMs("exhaustive") });
    if ((installed.status ?? 1) !== 0) {
      problems.push(`python oracle host: ${entry.spec} is neither importable nor installable into ${relative(repoRoot, dir).split(sep).join("/")} — ${installed.stderr.trim().split("\n").slice(-3).join(" ") || `pip exited ${installed.status}`}`);
      continue;
    }
    if (!present(entry)) problems.push(`python oracle host: ${entry.spec} installed but \`import ${entry.module}\` at that version still fails — declare the import name with "module" if it differs from the distribution name`);
  }
  if (problems.length === 0) {
    writeFileSync(stampPath, `${JSON.stringify({ interpreter: base, signature, packages: specs }, null, 2)}\n`);
    markRunComplete(dir);
  }
  return { interpreter, problems };
}

/** 🧭️ Projects the explicit Python host protocol, preserving manifest order for local source roots. */
export function pythonHostArguments(hostPath: string, planPath: string, outPath: string, adapterPath: string, localPaths: readonly string[]): string[] {
  return [hostPath, ...localPaths.flatMap((path) => ["--local-source", path]), "--plan", planPath, "--out", outPath, "--adapter", adapterPath];
}

/** 🐍️ Runs the committed adapter through the owned Python host — never through the compose-scoped root discovery config. */
export function materializePythonHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "python");
  const declared = contributedOraclePackages(repoRoot, discovered, "python");
  const { interpreter, problems } = provisionPythonInterpreter(repoRoot, oracleHostPython(repoRoot), declared);
  // 🧩️A contributed package that DOES carry a path is in-repo source, reached the way Python reaches
  // any source tree: on the import path, never installed.
  const localPaths = declared.filter((entry) => entry.path !== undefined).map((entry) => join(repoRoot, entry.path!));
  return {
    command: interpreter,
    args: pythonHostArguments(join(repoRoot, REPO_TEST_PYTHON_HOST_REL, "🐍️.py"), planPath, outPath, join(repoRoot, discovered.adapters.python!), localPaths),
    cwd: repoRoot,
    env: {
      ...process.env,
      PYTHONDONTWRITEBYTECODE: "1",
      PYTHONPYCACHEPREFIX: join(agentCacheRoot(repoRoot), "pycache"),
      ...(localPaths.length > 0 ? { PYTHONPATH: [...localPaths, process.env.PYTHONPATH ?? ""].filter((value) => value !== "").join(delimiter) } : {}),
    },
    hostDir: dir,
    problems,
  };
}

/**
 * 🟦️ Runs the committed adapter through the owned TypeScript host. Nothing is generated: bun resolves
 * a bare specifier by walking up from the repository root, so a declared npm package is RESOLVED
 * from the checkout's existing `node_modules` rather than installed into a private tree — one
 * install, one lockfile, one version of every library in the repository. What this does add is the
 * check that the declaration is true: an unresolvable package is reported here instead of surfacing
 * as an adapter import error with no mention of the manifest that promised it.
 */
export function materializeTypescriptHost(repoRoot: string, discovered: DiscoveredCase, planPath: string, outPath: string): MaterializedHost {
  const problems = contributedOraclePackages(repoRoot, discovered, "typescript")
    .filter((entry) => entry.path === undefined && !resolvesFromRepoRoot(repoRoot, entry.package))
    .map((entry) => `${discovered.caseDir}: declared typescript oracle package ${entry.package} does not resolve from the repository's node_modules — add it to the root manifest and install it`);
  return {
    command: "bun",
    args: [join(repoRoot, REPO_TEST_DOMAIN_REL, "🖥️host", "🟦️.ts"), "--plan", planPath, "--out", outPath, "--adapter", join(repoRoot, discovered.adapters.typescript!)],
    cwd: repoRoot,
    env: process.env,
    hostDir: null,
    problems,
  };
}

/** 🟦️ Whether a bare specifier resolves from the repository root — the same lookup the host will do. */
export function resolvesFromRepoRoot(repoRoot: string, specifier: string): boolean {
  try {
    createRequire(join(repoRoot, "package.json")).resolve(specifier);
    return true;
  } catch {
    // 🧭️A package whose manifest declares no resolvable entry point still counts as present; the
    // adapter may be reaching a subpath export the root resolver alone cannot answer for.
    return existsSync(join(repoRoot, "node_modules", ...specifier.split("/"), "package.json"));
  }
}

/** 🔷️ Materializes a cache-local .NET test project that links the committed adapter and the host support project. */
export function materializeDotnetHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, planPath: string, outPath: string): MaterializedHost {
  const dir = hostDirFor(repoRoot, discovered, role, "dotnet");
  const adapterAbs = join(repoRoot, discovered.adapters.dotnet!);
  writeFileSync(
    join(dir, "host.csproj"),
    [
      "<!-- 🤖️ Generated — safe to delete, never commit. -->",
      '<Project Sdk="Microsoft.NET.Sdk">',
      "  <PropertyGroup>",
      "    <OutputType>Exe</OutputType>",
      "    <TargetFramework>net8.0</TargetFramework>",
      "    <Nullable>enable</Nullable>",
      "    <ImplicitUsings>enable</ImplicitUsings>",
      "    <LangVersion>latest</LangVersion>",
      "    <EnableDefaultCompileItems>false</EnableDefaultCompileItems>",
      "    <AssemblyName>host</AssemblyName>",
      "    <RootNamespace>Semio.Repo.Test.Host</RootNamespace>",
      "  </PropertyGroup>",
      "  <ItemGroup>",
      `    <Compile Include="${adapterAbs}" />`,
      '    <Compile Include="Program.cs" />',
      `    <ProjectReference Include="${join(repoRoot, REPO_TEST_DOTNET_PACKAGE_REL, "🧪️Semio.Repo.Test.csproj")}" />`,
      "  </ItemGroup>",
      "</Project>",
      "",
    ].join("\n"),
  );
  writeFileSync(
    join(dir, "Program.cs"),
    ["// 🤖️ Generated native entrypoint.", "using Semio.Repo.Test;", "", "internal static class GeneratedHost", "{", "    private static int Main(string[] args) => TestHost.RunMain(Adapter.Create(), args);", "}", ""].join("\n"),
  );
  return {
    command: "dotnet",
    args: ["run", "--project", join(dir, "host.csproj"), "--", "--plan", planPath, "--out", outPath],
    cwd: dir,
    env: { ...process.env, DOTNET_CLI_TELEMETRY_OPTOUT: "1", DOTNET_NOLOGO: "1" },
    hostDir: dir,
    problems: [],
  };
}

/** 🏗️ Resolves the launch recipe for one implementation, materializing a cache-local entrypoint when the native framework needs one. */
export function materializeHost(repoRoot: string, discovered: DiscoveredCase, role: TestRole, implementation: Implementation, planPath: string, outPath: string): MaterializedHost {
  switch (implementation) {
    case "typescript":
      return materializeTypescriptHost(repoRoot, discovered, planPath, outPath);
    case "rust":
      return materializeRustHost(repoRoot, discovered, role, planPath, outPath);
    case "go":
      return materializeGoHost(repoRoot, discovered, role, planPath, outPath);
    case "python":
      return materializePythonHost(repoRoot, discovered, role, planPath, outPath);
    case "dotnet":
      return materializeDotnetHost(repoRoot, discovered, role, planPath, outPath);
  }
}
