import { createRequire } from "node:module";
import { join, posix } from "node:path";
import { lstat, readFile, readdir, stat as followedStat } from "node:fs/promises";
import { rustSourceDirectionEdges, rustSourceReferences, type RustSourceDirectionEdge } from "../🟦️.ts";
import type { DependencyDirectionRule } from "../../🟦️.ts";
import { inspectRustModuleGraph } from "../../../../🔍️discovery/🟦️.ts";

/** 🛡️ Checks all authored framework Rust literal file dependencies without compiling specific owners. */
export async function verifyRustSourceDirection(root: string): Promise<void> {
  const library = join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
  const taxonomy = JSON.parse(await readFile(join(library, "🔣️taxonomy.json"), "utf8"));
  const policy = createRequire(import.meta.url)(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs"));
  const names = ["framework-no-implementation", ...Object.keys(taxonomy.dependencyDirections.rules)];
  const rules: DependencyDirectionRule[] = policy.forbidden.filter((rule: DependencyDirectionRule) => names.includes(rule.name));
  if (rules.length !== names.length || rules.some((rule) => rule.severity !== "error")) throw new Error("Rust source direction requires every declared strict framework rule");
  const ignore: string[] = taxonomy.implementationLeafPolicy.ignoredPathPatterns.map((path: string) => path.replace(/^\*\*\//u, ""));
  const excluded: string[] = Object.values(taxonomy.pathExclusions).map((value: any) => value.path.replace(/\/$/u, ""));
  const edges: RustSourceDirectionEdge[] = [];
  const sources = new Map<string, string>();
  const unsupported: string[] = [];
  let files = 0, references = 0, stopped = false;
  const stop = (): void => { stopped = true; };
  process.once("SIGINT", stop); process.once("SIGTERM", stop);
  const check = (): void => { if (stopped) throw new Error("Rust source direction scan canceled"); };
  const walk = async (path: string): Promise<void> => {
    check();
    if (ignore.some((part) => path === part || path.endsWith(`/${part}`)) || excluded.some((part) => path === part || path.startsWith(`${part}/`))) return;
    const stat = await lstat(join(root, path));
    if (stat.isSymbolicLink()) {
      if (path.endsWith(".rs") || path.endsWith("/Cargo.toml") || (await followedStat(join(root, path))).isDirectory()) throw new Error(`Rust source direction cannot inventory linked input: ${path}`);
      return;
    }
    if (stat.isDirectory()) {
      for (const entry of await readdir(join(root, path))) await walk(`${path}/${entry}`);
    } else if (stat.isFile() && (path.endsWith(".rs") || path.endsWith("/Cargo.toml"))) sources.set(path, await readFile(join(root, path), "utf8"));
  };
  try {
    for (const [area, layer] of Object.entries(taxonomy.areaLayers)) if (layer === "framework") await walk(area);
    const compileReferences = new Map<string, ReturnType<typeof rustSourceReferences>>();
    for (const [path, source] of sources) {
      check();
      if (!path.endsWith(".rs")) continue;
      try { compileReferences.set(path, rustSourceReferences(source)); } catch (error) { unsupported.push(`${path}: ${(error as Error).message}`); }
    }
    const graph = inspectRustModuleGraph([...sources.keys()], (path) => sources.get(path), { checkCancellation: check, compileReferences });
    for (const [path] of sources) {
      check();
      if (!path.endsWith(".rs")) continue;
      const refs = compileReferences.get(path) ?? [];
      references += refs.length; files++;
      const contexts = graph.contexts.get(path), manifestPaths: string[] = [];
      for (let directory = posix.dirname(path); directory !== "."; directory = posix.dirname(directory)) {
        const manifest = `${directory}/Cargo.toml`;
        if (sources.has(manifest)) { manifestPaths.push(manifest); break; }
      }
      try { edges.push(...rustSourceDirectionEdges(path, refs, rules, { contexts, manifestPaths: contexts?.some((context) => context.manifestPath) ? [] : manifestPaths })); } catch (error) { unsupported.push((error as Error).message); }
      if (files % 250 === 0) console.log(`[rust-source-direction] progress; files=${files}; references=${references}`);
    }
    if (!files) throw new Error("Rust source direction requires a nonempty authored framework source inventory");
    for (const edge of edges) console.error(`[rust-source-direction] ${edge.rule}: ${edge.from}:${edge.line} → ${edge.to}; kind=${edge.kind}`);
    for (const error of unsupported) console.error(`[rust-source-direction] unsupported: ${error}`);
    if (edges.length || unsupported.length) throw new Error(`Rust source direction failed: ${edges.length} strict compile-time boundary violations and ${unsupported.length} unsupported source inputs across ${files} files and ${references} authored references`);
    console.log(`[rust-source-direction] passed; files=${files}; authoredReferences=${references}; scope=all-configurations-and-macro-templates`);
  } finally { process.off("SIGINT", stop); process.off("SIGTERM", stop); }
}
