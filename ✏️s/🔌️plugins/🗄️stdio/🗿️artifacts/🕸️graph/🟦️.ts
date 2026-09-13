import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { CARGO_COMPOSITION_NAME, CARGO_CONTRACT_NAME, COMPOSITION_RUST_NAME, COMPOSITION_TYPESCRIPT_NAME, JsonMap, NX_CONTRACT_NAME, StdioArtifactPackageRecord, canonicalStdioArtifactNames, slashStdioPath } from "../../📇️inventory/🟦️.ts";

async function runCaptured(command: string, args: string[], cwd: string, timeoutMs: number): Promise<string> {
  console.log(`[stdio-package-contract] running ${command} ${args.join(" ")}`);
  const child = spawn(command, args, { cwd, env: { ...process.env, NX_DAEMON: "false", NX_ISOLATE_PLUGINS: "false" }, stdio: ["ignore", "pipe", "pipe"], windowsHide: true });
  let stdout = "";
  let stderr = "";
  let cancelled = "";
  const maximum = 64 * 1024 * 1024;
  const append = (current: string, chunk: Buffer): string => {
    const next = current + chunk.toString("utf8");
    if (Buffer.byteLength(next) > maximum) {
      cancelled = "output limit";
      child.kill("SIGTERM");
    }
    return next;
  };
  child.stdout?.on("data", (chunk: Buffer) => stdout = append(stdout, chunk));
  child.stderr?.on("data", (chunk: Buffer) => stderr = append(stderr, chunk));
  const cancel = (signal: NodeJS.Signals): void => {
    cancelled = signal;
    child.kill("SIGTERM");
  };
  const interrupt = (): void => cancel("SIGINT");
  const terminate = (): void => cancel("SIGTERM");
  process.once("SIGINT", interrupt);
  process.once("SIGTERM", terminate);
  const started = Date.now();
  const progress = setInterval(() => console.log(`[stdio-package-contract] ${command} running elapsedMs=${Date.now() - started}`), 10_000);
  const timeout = setTimeout(() => {
    cancelled = `timeout ${timeoutMs}ms`;
    child.kill("SIGTERM");
  }, timeoutMs);
  try {
    const result = await new Promise<{ code: number | null; signal: NodeJS.Signals | null }>((resolveExit, rejectExit) => {
      child.once("error", rejectExit);
      child.once("exit", (code, signal) => resolveExit({ code, signal }));
    });
    assert(!cancelled, `${command} ${cancelled}\n${stderr}`);
    assert.equal(result.code, 0, stderr || `${command} exited with ${result.signal ?? result.code}`);
    console.log(`[stdio-package-contract] completed ${command} elapsedMs=${Date.now() - started}`);
    return stdout;
  } finally {
    clearInterval(progress);
    clearTimeout(timeout);
    process.off("SIGINT", interrupt);
    process.off("SIGTERM", terminate);
  }
}

function assertActualCargoDag(metadata: JsonMap, artifactCargoNames: ReadonlySet<string>): void {
  const packages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  const artifactGraph = new Map<string, string[]>();
  for (const name of artifactCargoNames) {
    const cargo = packages.get(name);
    assert(cargo, `Cargo metadata misses ${name}`);
    artifactGraph.set(name, (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name)).filter((dependency) => artifactCargoNames.has(dependency)));
  }
  const complete = new Set<string>();
  const visitArtifacts = (name: string, route: string[]): void => {
    const repeated = route.indexOf(name);
    if (repeated >= 0) throw new Error(`Cargo artifact dependency cycle: ${[...route.slice(repeated), name].join(" -> ")}`);
    if (complete.has(name)) return;
    for (const dependency of artifactGraph.get(name) ?? []) visitArtifacts(dependency, [...route, name]);
    complete.add(name);
  };
  const visitComposition = (name: string, route: string[], visited: Set<string>): void => {
    if (name === CARGO_COMPOSITION_NAME) throw new Error(`Cargo artifact reaches stdio composition: ${route.join(" -> ")}`);
    if (visited.has(name)) return;
    visited.add(name);
    const cargo = packages.get(name);
    if (!cargo) return;
    for (const dependency of cargo.dependencies as JsonMap[]) visitComposition(String(dependency.name), [...route, String(dependency.name)], visited);
  };
  for (const name of artifactCargoNames) {
    visitArtifacts(name, []);
    visitComposition(name, [name], new Set());
  }
}

/** 🦀️ Reads Cargo's own package graph without compiling Stdio. */
export async function stdioCargoMetadata(repoRoot: string): Promise<JsonMap> {
  return JSON.parse(await runCaptured("cargo", ["metadata", "--locked", "--no-deps", "--format-version", "1"], repoRoot, 120_000)) as JsonMap;
}

/** 🕸️ Compares the admitted artifact contract with Cargo's actual native metadata graph. */
export function assertStdioArtifactCargoMetadata(repoRoot: string, contract: { packages: StdioArtifactPackageRecord[] }, metadata: JsonMap): void {
  const byName = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  for (const entry of contract.packages) {
    const cargo = byName.get(entry.rust.cargoName);
    assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
    assert.equal(slashStdioPath(cargo.manifest_path), slashStdioPath(join(repoRoot, entry.rust.manifest)));
    const dependencies = new Set((cargo.dependencies as JsonMap[]).map((dependency) => dependency.name));
    assert(!dependencies.has(CARGO_COMPOSITION_NAME));
    assert(dependencies.has(CARGO_CONTRACT_NAME), `${entry.rust.cargoName} misses ${CARGO_CONTRACT_NAME}`);
  }
  assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
  console.log(`[stdio-package-contract] Cargo metadata packages=${contract.packages.length}`);
}

/** 🔬️ Compares the admitted artifact dependency projection with Nx's current project graph. */
export async function assertStdioArtifactNxGraph(repoRoot: string, contract: { packages: StdioArtifactPackageRecord[] }, metadata: JsonMap): Promise<void> {
  assertActualCargoDag(metadata, new Set(contract.packages.map((entry) => entry.rust.cargoName)));
  const output = await runCaptured(process.execPath, ["run", "nx", "graph", "--print"], repoRoot, 180_000);
  const start = output.indexOf("{");
  assert(start >= 0, "Nx graph emitted no JSON");
  const graph = JSON.parse(output.slice(start)) as JsonMap;
  const dependencies = graph.graph?.dependencies ?? graph.dependencies;
  const nodes = graph.graph?.nodes ?? graph.nodes;
  const cargoPackages = new Map((metadata.packages as JsonMap[]).map((entry) => [String(entry.name), entry]));
  for (const entry of contract.packages) {
    assert(nodes?.[entry.rust.nxName], `Nx graph misses ${entry.rust.nxName}`);
    assert(nodes?.[entry.typescript.nxName], `Nx graph misses ${entry.typescript.nxName}`);
    const rustEdges = new Set((dependencies?.[entry.rust.nxName] ?? []).map((edge: JsonMap) => edge.target));
    const typescriptEdges = new Set((dependencies?.[entry.typescript.nxName] ?? []).map((edge: JsonMap) => edge.target));
    assert(!rustEdges.has(COMPOSITION_RUST_NAME), `${entry.rust.nxName} has a composition back-edge`);
    assert(rustEdges.has(NX_CONTRACT_NAME), `${entry.rust.nxName} misses Nx edge ${NX_CONTRACT_NAME}`);
    const cargo = cargoPackages.get(entry.rust.cargoName);
    assert(cargo, `Cargo metadata misses ${entry.rust.cargoName}`);
    const expectedRustEdges = (cargo.dependencies as JsonMap[]).map((dependency) => String(dependency.name)).filter((name) => name.startsWith("semio-s-artifact-stdio-") && name !== CARGO_CONTRACT_NAME).map((name) => canonicalStdioArtifactNames(name.slice("semio-s-artifact-stdio-".length)).rustNx);
    for (const target of expectedRustEdges) assert(rustEdges.has(target), `${entry.rust.nxName} misses Nx edge ${target}`);
    const typescript = JSON.parse(await Bun.file(join(repoRoot, entry.typescript.manifest)).text()) as JsonMap;
    for (const target of Object.keys(typescript.dependencies ?? {})) assert(typescriptEdges.has(target), `${entry.typescript.nxName} misses Nx edge ${target}`);
    const ownerRoot = slashStdioPath(dirname(entry.source));
    for (const node of [nodes[entry.rust.nxName], nodes[entry.typescript.nxName]]) {
      const defaultInputs = node.data?.namedInputs?.default ?? [];
      assert(defaultInputs.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${ownerRoot}/`)), `${node.name} does not hash ${ownerRoot}`);
    }
  }
  assert(nodes?.[NX_CONTRACT_NAME], `Nx graph misses ${NX_CONTRACT_NAME}`);
  assert(nodes?.[COMPOSITION_TYPESCRIPT_NAME], `Nx graph misses ${COMPOSITION_TYPESCRIPT_NAME}`);
  console.log(`[stdio-package-contract] Nx graph rust=${contract.packages.length} typescript=${contract.packages.length}`);
}
