/**
 * 🧹️ Vite config hygiene. Pinned to the node environment: at `long` the suite default is jsdom, whose
 * `TextEncoder` fails esbuild JS API's `instanceof Uint8Array` invariant, and the graph bundles below are
 * pure Node work with no DOM.
 *
 * @vitest-environment node
 */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { build, type Plugin } from "esbuild";
import picomatch from "picomatch";
import { describe, expect, it } from "vitest";
import { stripExecutableShebang } from "../../🧹️executable-source/🟦️.ts";
import { UNWATCHED_REPOSITORY_SEGMENTS, repositorySourceWatchRoots, semioSourceWatchVitePlugin, unwatchedRepositoryPathMatcher } from "../../🔌️vite-plugins/🟦️.ts";

describe("executable source transformation", () => {
  it.each([
    ["#!/usr/bin/env bun\nexport const value = 1;\n", "export const value = 1;\n"],
    ["#!/usr/bin/env bun\r\nexport const value = 1;\r\n", "export const value = 1;\r\n"],
    ["export const value = '#!/usr/bin/env bun';\n", "export const value = '#!/usr/bin/env bun';\n"],
  ])("removes only a leading Bun shebang before Vite injects imports", (source, expected) => {
    expect(stripExecutableShebang(source)).toBe(expected);
  });
});

const packageDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(packageDir, "../../../../../../..");
const contract = JSON.parse(readFileSync(join(packageDir, "../../🧫️fixtures/⚙️config-graph.json"), "utf8")) as {
  readonly entry: string;
  readonly deny: readonly string[];
  readonly require: readonly string[];
  readonly maxModules: number;
  readonly maxSourceBytes: number;
};
const entryPath = join(repoRoot, contract.entry);

/** @emoji 📦️ Vite's own `externalize-deps` rule for `--configLoader bundle` (`vite/dist/node/chunks/config.js`):
 * every bare specifier is external, so a config's real bundle graph is its relative-import closure. */
const externalizeDeps: Plugin = {
  name: "externalize-deps",
  setup(builder) {
    builder.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true }));
  },
};

const repoRelative = (path: string): string => relative(repoRoot, resolve(repoRoot, path)).replaceAll("\\", "/");

async function esbuildConfigGraph(): Promise<{ readonly modules: readonly string[]; readonly sourceBytes: number }> {
  const result = await build({ entryPoints: [entryPath], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: repoRoot, plugins: [externalizeDeps] });
  return { modules: Object.keys(result.metafile.inputs).map(repoRelative).sort(), sourceBytes: Object.values(result.metafile.inputs).reduce((total, input) => total + input.bytes, 0) };
}

/** @emoji 🔮️ Independent oracle: Bun's own bundler resolves the same entry under the same
 * externalize-every-package rule, and its sourcemap `sources` array is the module set it actually
 * read — a second implementation of the graph this contract bounds, not a second read of esbuild's. */
function bunConfigGraph(): readonly string[] {
  const bun = process.execPath.endsWith("bun") ? process.execPath : "bun";
  const outDir = mkdtempSync(join(tmpdir(), "semio-config-graph-"));
  try {
    const status = spawnSync(bun, ["build", entryPath, "--target=bun", "--packages=external", "--sourcemap=external", "--outdir", outDir], { cwd: repoRoot, encoding: "utf8" });
    if (status.status !== 0) throw new Error(`bun build failed: ${status.stderr || status.stdout}`);
    const sourcemap = readdirSync(outDir).find((name) => name.endsWith(".map"));
    if (!sourcemap) throw new Error("bun build emitted no sourcemap to read the module graph from");
    return (JSON.parse(readFileSync(join(outDir, sourcemap), "utf8")) as { sources: string[] }).sources.map(repoRelative).sort();
  } finally {
    rmSync(outDir, { recursive: true, force: true });
  }
}

describe("vite config module graph", () => {
  it("keeps every denied module out of the bundled config graph", async () => {
    const { modules } = await esbuildConfigGraph();
    expect(modules).toContain(contract.entry);
    for (const denied of contract.deny) expect(modules, `${denied} is reachable from ⚙️vite.config.ts — Vite parses and watches it on every boot`).not.toContain(denied);
  });

  it("keeps every required module in the bundled config graph", async () => {
    const { modules } = await esbuildConfigGraph();
    for (const required of contract.require) expect(modules).toContain(required);
  });

  it("stays inside the declared module and source-byte bounds", async () => {
    const { modules, sourceBytes } = await esbuildConfigGraph();
    expect(modules.length).toBeLessThanOrEqual(contract.maxModules);
    expect(sourceBytes).toBeLessThanOrEqual(contract.maxSourceBytes);
    console.log(`[DEBUG] vite config graph: ${modules.length} modules, ${sourceBytes} source bytes`);
  });

  it("agrees with Bun's independent bundler on the resolved TypeScript module set", async () => {
    const { modules } = await esbuildConfigGraph();
    const oracle = bunConfigGraph();
    expect(oracle).toEqual(modules.filter((module) => /\.[cm]?[jt]sx?$/u.test(module)));
    for (const denied of contract.deny) expect(oracle).not.toContain(denied);
  });
});

const watchPolicy = JSON.parse(readFileSync(join(packageDir, "../../🧫️fixtures/👁️watch-policy.json"), "utf8")) as {
  readonly unwatchedSegments: readonly string[];
  readonly unwatchedGlobs: readonly string[];
  readonly unwatchedPaths: readonly string[];
  readonly watchedPaths: readonly string[];
  readonly requiredRoots: readonly string[];
  readonly forbiddenRoots: readonly string[];
};

/** @emoji 🔮️ Independent oracle: `picomatch` is the glob engine chokidar itself filters with, so the
 * fixture's equivalent ignore globs decide every path through a third-party implementation rather than
 * through a second reading of ours. */
const picomatchUnwatched = (relativePath: string): boolean => watchPolicy.unwatchedGlobs.some((glob) => picomatch(glob, { dot: true })(relativePath));

describe("dev server watch policy", () => {
  it.each(watchPolicy.unwatchedPaths)("keeps %s outside every dev-server watch", (relativePath) => {
    expect(unwatchedRepositoryPathMatcher().test(relativePath)).toBe(true);
    expect(unwatchedRepositoryPathMatcher().test(relativePath.replaceAll("/", "\\"))).toBe(true);
  });

  it.each(watchPolicy.watchedPaths)("keeps %s inside the watched source set", (relativePath) => {
    expect(unwatchedRepositoryPathMatcher().test(relativePath)).toBe(false);
    expect(unwatchedRepositoryPathMatcher().test(relativePath.replaceAll("/", "\\"))).toBe(false);
  });

  it("agrees with picomatch's independent glob engine on every fixture path", () => {
    const matcher = unwatchedRepositoryPathMatcher();
    for (const relativePath of [...watchPolicy.unwatchedPaths, ...watchPolicy.watchedPaths]) {
      expect(matcher.test(relativePath), relativePath).toBe(picomatchUnwatched(relativePath));
    }
  });

  it("declares exactly the fixture's unwatched segments", () => {
    expect([...UNWATCHED_REPOSITORY_SEGMENTS].sort()).toEqual([...watchPolicy.unwatchedSegments].sort());
  });

  it("resolves source roots that cover the products and exclude every store", () => {
    const roots = repositorySourceWatchRoots(repoRoot).map((root) => relative(repoRoot, root).replaceAll("\\", "/"));
    for (const required of watchPolicy.requiredRoots) expect(roots).toContain(required);
    for (const forbidden of watchPolicy.forbiddenRoots) expect(roots).not.toContain(forbidden);
  });

  it("hands Vite no chokidar watcher of its own and mounts the replacement", () => {
    const source = readFileSync(join(repoRoot, contract.entry), "utf8");
    expect(source, "server.watch must stay null — a chokidar watcher here consolidates onto the whole repository").toMatch(/watch:\s*null/u);
    expect(source).toContain("semioSourceWatchVitePlugin({ repoRoot })");
  });

  it("reports source edits and stays silent for every unwatched store", async () => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-watch-policy-"));
    const seen: string[] = [];
    const server = { watcher: { emit: (_event: string, path: string) => (seen.push(relative(sandbox, path).replaceAll("\\", "/")), true) }, httpServer: null };
    try {
      for (const segment of ["🧰️framework", ...watchPolicy.forbiddenRoots, "🧰️framework/dist", "🧰️framework/🤖️generated"]) mkdirSync(join(sandbox, segment), { recursive: true });
      semioSourceWatchVitePlugin({ repoRoot: sandbox }).configureServer(server);
      const written = [join(sandbox, "🧰️framework/🟦️.ts"), join(sandbox, "🧰️framework/dist/🟦️.js"), join(sandbox, "🧰️framework/🤖️generated/🟦️.ts"), ...watchPolicy.forbiddenRoots.map((root) => join(sandbox, root, "noise.txt"))];
      // ⏳️ `fs.watch` arms asynchronously, so the edit is replayed until it lands rather than written once
      // behind an unarmed watcher — the deadline is the failure, never a fixed sleep.
      const deadline = Date.now() + 20_000;
      while (!seen.includes("🧰️framework/🟦️.ts") && Date.now() < deadline) {
        for (const file of written) writeFileSync(file, `export const value = ${Date.now()};\n`);
        await new Promise((resolve$) => setTimeout(resolve$, 100));
      }
      expect(seen, "the source edit must reach Vite").toContain("🧰️framework/🟦️.ts");
      expect(seen.filter((path) => unwatchedRepositoryPathMatcher().test(path))).toEqual([]);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  // 🛰️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B53: the EVENT is the contract, not only the path. macOS
  // reports every write to an existing file — in place and atomic (temp + rename) alike — as `fs.watch`
  // `eventType: "rename"`, and Vite invalidates a transformed module ONLY from its `change` handler
  // (`moduleGraph.onFileChange`); its `add` handler never touches the module graph, and with
  // `SEMIO_VITE_HMR=0` (`hmr: false`) no HMR pass invalidates either. A watcher that answers a modified
  // file with `add` therefore serves the pre-edit transform for the life of the server, which is how a
  // landed host fix measured as absent on `:6013` (`📓️2026-09-13-wave-B53-nakagin-export-full-run.md` §4.2).
  it.each([
    ["an in-place write", (target: string) => writeFileSync(target, `export const value = ${Date.now()};\n`)],
    ["an atomic save", (target: string) => {
      const temporary = `${target}.tmp`;
      writeFileSync(temporary, `export const value = ${Date.now()};\n`);
      renameSync(temporary, target);
    }],
  ])("replays %s over an existing source file as a change, the only event that invalidates Vite's module graph", async (_label, write) => {
    const sandbox = mkdtempSync(join(tmpdir(), "semio-watch-invalidate-"));
    const events: string[] = [];
    const server = { watcher: { emit: (event: string, path: string) => (events.push(`${event}:${relative(sandbox, path).replaceAll("\\", "/")}`), true) }, httpServer: null };
    try {
      mkdirSync(join(sandbox, "🧰️framework"), { recursive: true });
      const target = join(sandbox, "🧰️framework/🟦️.ts");
      const arming = join(sandbox, "🧰️framework/🔎️arm.ts");
      writeFileSync(target, "export const value = 0;\n");
      semioSourceWatchVitePlugin({ repoRoot: sandbox }).configureServer(server);
      // ⏳️ `fs.watch` arms asynchronously, and macOS answers only the FIRST write to a path with `rename`
      // — every later write to the same path in the same session may report `change` on its own. A retry
      // loop over the target would therefore pass on its second write while a real editor's single save
      // stays invisible, so the arming is proven on a SEPARATE file and the target is written exactly once.
      const deadline = Date.now() + 20_000;
      while (!events.some((event) => event.endsWith("🔎️arm.ts")) && Date.now() < deadline) {
        writeFileSync(arming, `export const armed = ${Date.now()};\n`);
        await new Promise((resolve$) => setTimeout(resolve$, 100));
      }
      expect(events.some((event) => event.endsWith("🔎️arm.ts")), "the watcher must arm before the measured write").toBe(true);
      write(target);
      const settle = Date.now() + 10_000;
      while (!events.includes("change:🧰️framework/🟦️.ts") && Date.now() < settle) await new Promise((resolve$) => setTimeout(resolve$, 100));
      expect(events, "one save of a modified module must reach Vite as a change").toContain("change:🧰️framework/🟦️.ts");
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  }, 30_000);
});

const browserContract = JSON.parse(readFileSync(join(packageDir, "../../🧫️fixtures/🌐️browser-graph.json"), "utf8")) as {
  readonly entry: string;
  readonly configPath: string;
  readonly resolveExtensions: readonly string[];
  readonly conditionNames: readonly string[];
  readonly extensionAlias: Readonly<Record<string, readonly string[]>>;
  readonly alias: readonly { readonly find: string; readonly replacement: string }[];
  readonly denyPackages: readonly string[];
  readonly denyModules: readonly string[];
  readonly requireModules: readonly string[];
  readonly ambiguousDirectories: readonly { readonly directory: string; readonly kindBasename: string; readonly extensionlessResolvesTo: string; readonly browserEntry: string }[];
  readonly maxModules: number;
};
const browserAliases = [...browserContract.alias].sort((left, right) => right.find.length - left.find.length);
const SOURCE_MODULE = /\.(?:[cm]?tsx?|[cm]?jsx?)$/u;

/** @emoji 🧭️ Vite's file resolution for one candidate path, in the declared extension order: the exact file, then
 * `<candidate><extension>`, then a directory's own package entry. The ORDER is the contract — `🟦️.mts` sits ahead of
 * `🟦️.ts`/`🟦️.tsx`, so an extensionless `…/🟦️` in a directory that also carries a tool-config entry silently resolves
 * to the tool config rather than to the browser entry beside it. */
function resolveBrowserCandidate(candidate: string): string | null {
  if (existsSync(candidate) && statSync(candidate).isFile()) return candidate;
  // 📎️ A TypeScript source may address its sibling by the emitted `.js`/`.mjs`/`.jsx` name (NodeNext style);
  // every resolver here rewrites that back onto the TypeScript twin before it looks for a real `.js` file.
  for (const [emitted, authored] of Object.entries(browserContract.extensionAlias)) {
    if (!candidate.endsWith(emitted)) continue;
    for (const extension of authored) {
      const twin = `${candidate.slice(0, -emitted.length)}${extension}`;
      if (existsSync(twin) && statSync(twin).isFile()) return twin;
    }
  }
  for (const extension of browserContract.resolveExtensions) {
    const withExtension = `${candidate}${extension}`;
    if (existsSync(withExtension) && statSync(withExtension).isFile()) return withExtension;
  }
  if (!existsSync(candidate) || !statSync(candidate).isDirectory()) return null;
  const manifestPath = join(candidate, "package.json");
  if (existsSync(manifestPath)) {
    const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { exports?: unknown; module?: string; main?: string };
    const exported = typeof manifest.exports === "string" ? manifest.exports : typeof (manifest.exports as Record<string, unknown> | undefined)?.["."] === "string" ? ((manifest.exports as Record<string, string>)["."]) : undefined;
    const entry = exported ?? manifest.module ?? manifest.main;
    if (entry) return resolveBrowserCandidate(resolve(candidate, entry));
  }
  for (const extension of browserContract.resolveExtensions) {
    const indexPath = join(candidate, `index${extension}`);
    if (existsSync(indexPath)) return indexPath;
  }
  return null;
}

/** @emoji ✂️ Every specifier a bundler keeps: comments and fully type-only `import type … from` / `export type … from`
 * statements are erased before any bundler sees them, so they carry no browser edge and are dropped here too. */
function browserSpecifiers(source: string): readonly string[] {
  const body = source
    .replaceAll(/\/\*[\s\S]*?\*\//gu, "")
    .replaceAll(/(^|[^:'"`\\])\/\/[^\n]*/gu, "$1")
    .replaceAll(/\b(?:import|export)\s+type\s[^;]*?\bfrom\s*["'][^"']+["']/gu, "");
  const found = new Set<string>();
  for (const pattern of [/\bfrom\s*["']([^"']+)["']/gu, /\bimport\s*\(\s*["']([^"']+)["']/gu, /\bimport\s+["']([^"']+)["']/gu, /\brequire\(\s*["']([^"']+)["']\)/gu]) {
    for (const match of body.matchAll(pattern)) found.add(match[1]!);
  }
  return [...found];
}

const aliasTarget = (specifier: string): string | null => {
  const hit = browserAliases.find((entry) => specifier === entry.find || specifier.startsWith(`${entry.find}/`));
  return hit ? `${join(repoRoot, hit.replacement)}${specifier.slice(hit.find.length)}` : null;
};

interface BrowserGraph {
  readonly modules: readonly string[];
  readonly packageEdges: ReadonlyMap<string, readonly string[]>;
  readonly resolutions: readonly { readonly importerDirectory: string; readonly specifier: string; readonly resolved: string }[];
  readonly unresolved: readonly string[];
}

/** @emoji 🌐️ The closure Vite serves to the browser: a breadth-first walk of the host entry's relative and aliased
 * imports under the declared resolve order, with every bare specifier recorded as a leaf edge (Vite hands exactly
 * those to its esbuild dependency optimizer, which is where a node-only package's `.node` binding fails the pass). */
function browserGraph(): BrowserGraph {
  const entryPath = join(repoRoot, browserContract.entry);
  const modules = new Set([entryPath]);
  const packageEdges = new Map<string, string[]>();
  const resolutions: { importerDirectory: string; specifier: string; resolved: string }[] = [];
  const unresolved: string[] = [];
  const queue = [entryPath];
  while (queue.length > 0) {
    const file = queue.shift()!;
    if (!SOURCE_MODULE.test(file)) continue;
    for (const specifier of browserSpecifiers(readFileSync(file, "utf8"))) {
      if (/^(?:node:|bun:|virtual:|data:|https?:)/u.test(specifier)) continue;
      const bare = specifier.split("?")[0]!;
      const target = aliasTarget(bare) ?? (bare.startsWith(".") ? resolve(dirname(file), bare) : null);
      const resolved = target === null ? null : resolveBrowserCandidate(target);
      if (resolved === null) {
        // 🧱️ A relative specifier that resolves to nothing is a build artifact this repository has not produced
        // yet (a wasm component wrapper, a generated bundle); it carries no package edge to deny.
        (bare.startsWith(".") ? unresolved : []).push(`${repoRelative(file)} → ${bare}`);
        if (!bare.startsWith(".")) packageEdges.set(bare, [...(packageEdges.get(bare) ?? []), repoRelative(file)]);
        continue;
      }
      if (resolved.includes("node_modules")) {
        packageEdges.set(bare, [...(packageEdges.get(bare) ?? []), repoRelative(file)]);
        continue;
      }
      resolutions.push({ importerDirectory: dirname(file), specifier: bare, resolved });
      if (!modules.has(resolved)) {
        modules.add(resolved);
        queue.push(resolved);
      }
    }
  }
  return { modules: [...modules].map(repoRelative).sort(), packageEdges, resolutions, unresolved };
}

/** @emoji 🔮️ Independent oracle: esbuild resolves and walks the same entry under the same extension order and alias
 * map, with every bare specifier external — a second implementation of the closure, not a second read of ours. */
async function esbuildBrowserGraph(): Promise<{ readonly modules: readonly string[]; readonly packages: readonly string[] }> {
  const externalizeBare: Plugin = {
    name: "browser-graph-alias",
    setup(builder) {
      builder.onResolve({ filter: /.*/ }, ({ path: specifier, kind }) => {
        if (kind === "entry-point") return null;
        const bare = specifier.split("?")[0]!;
        if (bare.startsWith(".")) return null;
        const target = aliasTarget(bare);
        const resolved = target === null ? null : resolveBrowserCandidate(target);
        return resolved === null ? { path: bare, external: true } : { path: resolved };
      });
    },
  };
  const result = await build({ entryPoints: [join(repoRoot, browserContract.entry)], bundle: true, write: false, metafile: true, platform: "browser", format: "esm", logLevel: "silent", absWorkingDir: repoRoot, resolveExtensions: [...browserContract.resolveExtensions], loader: { ".css": "empty", ".wasm": "empty", ".node": "empty" }, plugins: [externalizeBare] });
  const packages = new Set<string>();
  for (const input of Object.values(result.metafile.inputs)) for (const imported of input.imports) if (imported.external) packages.add(imported.path);
  return { modules: Object.keys(result.metafile.inputs).map(repoRelative).sort(), packages: [...packages].sort() };
}

const deniedPackageOf = (specifier: string): string | undefined => browserContract.denyPackages.find((denied) => specifier === denied || specifier.startsWith(`${denied}/`));

describe("browser entry module graph", () => {
  it("keeps every build-tooling module out of the closure Vite serves to the browser", () => {
    const { modules } = browserGraph();
    for (const denied of browserContract.denyModules) expect(modules, `${denied} is browser-served — Vite optimizes its bare imports with esbuild, which cannot load a native .node binding`).not.toContain(denied);
  });

  it("imports no node-only package from any browser-served module", () => {
    const { packageEdges } = browserGraph();
    const violations = [...packageEdges].flatMap(([specifier, importers]) => {
      const denied = deniedPackageOf(specifier);
      return denied === undefined ? [] : [`${specifier} ← ${importers.join(", ")}`];
    });
    expect(violations, "a node-only package reached the browser dependency optimizer").toEqual([]);
  });

  it("still reaches every module the playground boots through and stays inside the declared bound", () => {
    const { modules, packageEdges, unresolved } = browserGraph();
    for (const required of browserContract.requireModules) expect(modules).toContain(required);
    expect(modules.length).toBeLessThanOrEqual(browserContract.maxModules);
    console.log(`[DEBUG] browser entry graph: ${modules.length} modules, ${packageEdges.size} bare packages, ${unresolved.length} unbuilt artifacts`);
  });

  it("agrees with esbuild's independent bundler on the denied modules and packages", async () => {
    const ours = browserGraph();
    const oracle = await esbuildBrowserGraph();
    for (const denied of browserContract.denyModules) expect(oracle.modules, denied).not.toContain(denied);
    expect(oracle.packages.filter((specifier) => deniedPackageOf(specifier) !== undefined)).toEqual([]);
    // 🔁️ The safety-relevant direction: the walk above must not MISS an edge esbuild found — a guard that
    // under-walks is the dangerous one. The reverse is not asserted: esbuild drops an import statement whose
    // bindings are all unused (it assumes they were types), so its input set is legitimately the smaller one.
    expect(oracle.modules.filter((module) => SOURCE_MODULE.test(module) && !ours.modules.includes(module)), "esbuild reached a browser module this walk never visited").toEqual([]);
    console.log(`[DEBUG] browser entry graph: ${ours.modules.length} walked, ${oracle.modules.length} esbuild inputs, ${oracle.packages.length} bare packages`);
  }, 120_000);

  it("agrees with enhanced-resolve's independent resolver on every resolved specifier", async () => {
    const { create } = (await import("enhanced-resolve")).default;
    const resolver = create.sync({ extensions: [...browserContract.resolveExtensions], conditionNames: [...browserContract.conditionNames], mainFields: ["browser", "module", "main"], extensionAlias: { ...browserContract.extensionAlias } as Record<string, string[]>, symlinks: false });
    const disagreements = browserGraph().resolutions.flatMap(({ importerDirectory, specifier, resolved }) => {
      if (!specifier.startsWith(".")) return [];
      let theirs: string | false;
      try {
        theirs = resolver(importerDirectory, specifier);
      } catch {
        return [`${specifier} from ${repoRelative(importerDirectory)}: enhanced-resolve found nothing`];
      }
      return theirs === resolved ? [] : [`${specifier} from ${repoRelative(importerDirectory)}: ${repoRelative(resolved)} vs ${theirs === false ? "false" : repoRelative(theirs)}`];
    });
    expect(disagreements).toEqual([]);
  });

  it("names the extension in every browser import of a directory whose kind basename is also a tool-config entry", async () => {
    const { create } = (await import("enhanced-resolve")).default;
    const resolver = create.sync({ extensions: [...browserContract.resolveExtensions], conditionNames: [...browserContract.conditionNames], mainFields: ["browser", "module", "main"], extensionAlias: { ...browserContract.extensionAlias } as Record<string, string[]>, symlinks: false });
    const { resolutions } = browserGraph();
    for (const ambiguous of browserContract.ambiguousDirectories) {
      const directory = join(repoRoot, ambiguous.directory);
      // 🔎️ The hazard is proven, not assumed: a third-party resolver on Vite's own extension order lands the bare
      // kind basename on the tool-config entry, which is why every browser importer must name its extension.
      expect(repoRelative(resolver(directory, `./${ambiguous.kindBasename}`) as string)).toBe(ambiguous.extensionlessResolvesTo);
      expect(repoRelative(resolver(directory, `./${ambiguous.kindBasename}.tsx`) as string)).toBe(ambiguous.browserEntry);
      const extensionless = resolutions.filter(({ resolved }) => repoRelative(resolved) === ambiguous.extensionlessResolvesTo);
      expect(extensionless.map(({ importerDirectory, specifier }) => `${repoRelative(importerDirectory)} → ${specifier}`)).toEqual([]);
    }
  });

  it("declares the alias map and the resolve order the Vite config actually installs", () => {
    const source = readFileSync(join(repoRoot, browserContract.configPath), "utf8");
    for (const { find, replacement } of browserContract.alias) {
      expect(source, `alias ${find} drifted from the Vite config`).toContain(`{ find: "${find}", replacement: path.resolve(repoRoot, "./${replacement}") }`);
    }
    expect(source, "an explicit resolve.extensions would replace the order this contract is resolved under").not.toMatch(/extensions:\s*\[/u);
  });
});
