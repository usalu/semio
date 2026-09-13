import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join, relative, resolve } from "node:path";
import { createArtifactRegistry, type ArtifactRegistry } from "../../📦️artifacts/📇️registry/🟦️.ts";
import { readInventoryGraph, type InventoryGraph, type InventoryProject } from "../🟦️.ts";
import { slash, sourceFiles } from "../../🔍️discovery/📂️source/🟦️.ts";

export const CACHE_BOOTSTRAP_EXECUTABLE = fileURLToPath(new URL("../../🚀️bootstrap/📜️script.ts", import.meta.url));

export interface CacheInventoryFinding {
  readonly rule: string;
  readonly path: string;
  readonly line: number;
  readonly entry_point: string;
  readonly evidence: string;
  readonly replacement: string;
  readonly severity: string;
  readonly status: string;
}

export interface CacheInventory {
  readonly projects: InventoryProject[];
  readonly commands: any[];
  readonly artifacts: any[];
  readonly artifactRegistry: ArtifactRegistry;
  readonly violations: CacheInventoryFinding[];
}

export interface CacheInventoryOperations {
  readText(path: string): string;
  readonly bootstrapExecutable?: string;
}

/** 🧮️ Joins resolved project commands, output owners, consumers and policy findings from one captured graph. */
export function composeInventory(root: string, files: readonly string[], graph: InventoryGraph, operations: CacheInventoryOperations = { readText: (path) => readFileSync(path, "utf8") }): CacheInventory {
  const { projects, sources } = graph;
  const commands: any[] = [];
  const artifacts: any[] = [];
  const violations: CacheInventoryFinding[] = [];
  const report = (rule: string, path: string, entryPoint: string, evidence: string, replacement: string): void => {
    const text = operations.readText(join(root, path));
    const at = text.indexOf(JSON.stringify(entryPoint.split(":").at(-1)));
    violations.push({ rule, path, line: at < 0 ? 1 : text.slice(0, at).split("\n").length, entry_point: entryPoint, evidence, replacement, severity: "error", status: "open" });
  };
  for (const project of projects) {
    for (const [name, target] of Object.entries(project.targets)) {
      const path = sources[project.name]![name]!;
      const identity = `${project.name}:${name}`;
      commands.push({ project: project.name, target: name, file: path, executor: target.executor, configurations: target.configurations, cwd: target.options?.cwd ?? root, command: target.options?.command, cache: target.cache === true, continuous: target.continuous === true, inputs: target.inputs, outputs: target.outputs ?? [], dependsOn: target.dependsOn ?? [] });
      if (typeof target.options?.command === "string" && (!/^bun(?: --watch)? (?:"[^"\n]*📜️script\.ts"|[^\s]*📜️script\.ts) [^\n]+$/.test(target.options.command) || /(?:&&|\|\|)/.test(target.options.command))) report("ORCH-01", path, identity, target.options.command, "Invoke one script and declare prerequisite ordering in dependsOn");
      if ((target.cache || /^(build(?:-|$)|wasm$|native-build$|package$|extension-package$)/.test(name)) && !Object.hasOwn(target, "outputs")) report("CACHE-06", path, identity, "Target still needs an explicit output contract", "Declare complete owned deliverables, or outputs: [] for a verified read-only task");
      for (const output of target.outputs ?? []) {
        const ownedPath = output.replaceAll("{workspaceRoot}", root).replaceAll("{projectRoot}", join(root, project.root));
        artifacts.push({ owner: identity, path: slash(relative(root, resolve(root, ownedPath))), category: "deliverable", cacheability: Boolean(target.cache), retention: "replace-on-next-success", producer: identity });
        if (/(?:^|\/)(?:node_modules|target|\.venv|\.nx)(?:\/|$)/.test(ownedPath) || resolve(root, ownedPath) === root) report("CACHE-04", path, identity, `Mutable or broad output: ${output}`, "Stage only this target's deliverables");
      }
    }
  }
  for (const file of files.filter((file) => file.endsWith("package.json"))) {
    let json;
    try {
      json = JSON.parse(operations.readText(join(root, file)));
    } catch {
      continue;
    }
    if (Object.keys(json.scripts ?? {}).length && projects.some((project) => project.root === dirname(file) || (project.root === "." && file === "package.json")) && JSON.stringify(json.nx?.includedScripts) !== "[]") report("ORCH-05", file, "nx.includedScripts", "Nx re-infers forwarding scripts and replaces their implementation targets", "Set nx.includedScripts to [] for forwarding package scripts");
    for (const [name, command] of Object.entries(json.scripts ?? {})) {
      commands.push({ file, script: name, command, cwd: dirname(file) });
      const bootstrap = operations.bootstrapExecutable ?? CACHE_BOOTSTRAP_EXECUTABLE;
      if (file === "package.json" && name === "nx" && command === `bun ./${slash(relative(root, bootstrap))} nx`) continue;
      if (!/^(?:bun )?nx\b/.test(String(command))) report("ORCH-01", file, name, String(command), "Forward the public command to an independently implemented Nx target");
    }
  }
  const consumers = new Map<string, Set<string>>();
  for (const command of commands.filter((command) => command.target))
    for (const dependency of command.dependsOn) {
      const target = typeof dependency === "string" ? dependency : dependency.target;
      const selectors = typeof dependency === "object" ? dependency.projects : undefined;
      const selected = selectors ? (Array.isArray(selectors) ? selectors : [selectors]) : [command.project];
      if (typeof target !== "string" || target.startsWith("^") || /[*?{}]/.test(target)) continue;
      for (const project of selected) {
        if (!projects.some((candidate) => candidate.name === project)) continue;
        const producer = target.includes(":") ? target : `${project}:${target}`;
        if (!consumers.has(producer)) consumers.set(producer, new Set());
        consumers.get(producer)!.add(`${command.project}:${command.target}`);
      }
    }
  const artifactRegistry = createArtifactRegistry(artifacts.map((artifact) => ({ ...artifact, consumers: [...(consumers.get(artifact.owner) ?? [])] })));
  for (const finding of artifactRegistry.findings) {
    const command = commands.find((command) => `${command.project}:${command.target}` === finding.owner);
    if (command) report(finding.rule, command.file, finding.owner, finding.evidence, "Declare one exclusive deliverable owner and use explicit target prerequisites for consumers");
  }
  return { projects, commands, artifacts: artifactRegistry.entries, artifactRegistry, violations };
}

/** 🧭️ Audits the complete resolved Nx graph and its source entry points. */
export function inventory(root: string): CacheInventory {
  return composeInventory(root, sourceFiles(root), readInventoryGraph(root));
}
