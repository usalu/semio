#!/usr/bin/env bun
/** 🛡️ `@semio-tech/hub-admin` (nx `os-hub-admin`) router: `bun ./📜️script.ts <dev|build|test> [args…]`. */
import { strict as assert } from "node:assert";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runViteBuild, runViteBunxDev, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

type AdminEntryGraph = { readonly version: number; readonly html: string; readonly manifest: string; readonly entry: string; readonly laws: readonly string[] };

function verifyAdminEntryGraph(root: string): void {
  const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🚪️entry-graph.json"), "utf8")) as AdminEntryGraph;
  const schema = JSON.parse(readFileSync(resolve(root, "🧪️tests/📐️entry-graph.schema.json"), "utf8")) as { readonly required: readonly string[]; readonly properties: { readonly version: { readonly const: number }; readonly laws: { readonly minItems: number; readonly maxItems: number } } };
  assert.equal(fixture.version, schema.properties.version.const);
  assert.deepEqual(Object.keys(fixture).sort(), [...schema.required].sort());
  assert.equal(fixture.laws.length, schema.properties.laws.minItems);
  assert.equal(fixture.laws.length, schema.properties.laws.maxItems);
  assert.equal(new Set(fixture.laws).size, fixture.laws.length);
  const packageRoot = realpathSync(root);
  const inside = (path: string): boolean => { const local = relative(packageRoot, path); return local !== "" && local !== ".." && !local.startsWith(`..${sep}`) && !isAbsolute(local); };
  const htmlPath = resolve(packageRoot, fixture.html);
  const manifestPath = resolve(packageRoot, fixture.manifest);
  assert.equal(inside(htmlPath) && inside(manifestPath), true, "Hub admin entry graph inputs must stay inside the package");
  const html = readFileSync(htmlPath, "utf8");
  const modules = [...html.matchAll(/<script\b[^>]*>/giu)].map(match => match[0]).filter(tag => /\btype\s*=\s*(["'])module\1/iu.test(tag));
  assert.equal(modules.length, 1, "Hub admin HTML must declare exactly one module entry");
  const source = modules[0]!.match(/\bsrc\s*=\s*(["'])([^"']+)\1/iu);
  assert.ok(source, "Hub admin module entry requires a source");
  assert.equal(source[2], fixture.entry, "Hub admin HTML module entry differs from the canonical entry graph");
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { readonly exports?: Readonly<Record<string, unknown>> };
  assert.equal(manifest.exports?.["."], fixture.entry, "Hub admin package export differs from the canonical entry graph");
  const htmlEntry = fileURLToPath(new URL(source[2]!, pathToFileURL(htmlPath)));
  const manifestEntry = fileURLToPath(new URL(manifest.exports["."] as string, pathToFileURL(manifestPath)));
  assert.equal(htmlEntry, manifestEntry);
  assert.equal(inside(htmlEntry), true, "Hub admin module entry escapes the package");
  assert.equal(existsSync(htmlEntry), true, "Hub admin canonical module entry does not exist");
  assert.equal(inside(realpathSync(htmlEntry)), true, "Hub admin module entry resolves outside the package");
  console.log(`[DEBUG] hub admin entry graph oracle: ${fixture.laws.length} laws, ${modules.length} HTML module entry, 1 package export, ${readFileSync(htmlEntry).byteLength} entry bytes`);
}

type AdminStylesheetGraph = { readonly version: number; readonly stylesheet: string; readonly imports: readonly string[]; readonly sources: readonly string[]; readonly shared: { readonly manifest: string; readonly export: string; readonly canonical: string }; readonly laws: readonly string[] };

function verifyAdminStylesheetGraph(root: string): void {
  const fixture = JSON.parse(readFileSync(resolve(root, "🧪️tests/🎨️stylesheet-graph.json"), "utf8")) as AdminStylesheetGraph;
  const schema = JSON.parse(readFileSync(resolve(root, "🧪️tests/🧵️stylesheet-graph.schema.json"), "utf8")) as { readonly required: readonly string[]; readonly properties: { readonly version: { readonly const: number }; readonly laws: { readonly minItems: number; readonly maxItems: number } } };
  assert.equal(fixture.version, schema.properties.version.const);
  assert.deepEqual(Object.keys(fixture).sort(), [...schema.required].sort());
  assert.equal(fixture.laws.length, schema.properties.laws.minItems);
  assert.equal(fixture.laws.length, schema.properties.laws.maxItems);
  assert.equal(new Set(fixture.laws).size, fixture.laws.length);
  const packageRoot = realpathSync(root);
  const repositoryRoot = realpathSync(resolve(packageRoot, "../../../../.."));
  const inside = (authority: string, path: string): boolean => { const local = relative(authority, path); return local !== "" && local !== ".." && !local.startsWith(`..${sep}`) && !isAbsolute(local); };
  const stylesheetPath = resolve(packageRoot, fixture.stylesheet);
  assert.equal(inside(packageRoot, stylesheetPath), true, "Hub admin stylesheet must stay inside the package");
  const stylesheet = readFileSync(stylesheetPath, "utf8");
  const lines = stylesheet.split(/\r?\n/u).map(line => line.trim());
  const imports = lines.flatMap(line => { const match = line.match(/^@import\s+(["'])([^"']+)\1\s*;/u); return match ? [match[2]!] : []; });
  const sources = lines.flatMap(line => { const match = line.match(/^@source\s+(["'])([^"']+)\1\s*;/u); return match ? [match[2]!] : []; });
  assert.deepEqual(imports, fixture.imports, "Hub admin stylesheet imports differ from the canonical dependency graph");
  assert.deepEqual(sources, fixture.sources, "Hub admin Tailwind sources differ from the canonical dependency graph");
  const firstSource = lines.findIndex(line => line.startsWith("@source"));
  const lastImport = lines.findLastIndex(line => line.startsWith("@import"));
  assert.ok(lastImport >= 0 && firstSource > lastImport, "Hub admin imports must precede Tailwind sources");
  for (const source of sources) { const path = resolve(dirname(stylesheetPath), source); assert.equal(existsSync(path) && inside(repositoryRoot, path), true, "Hub admin Tailwind source does not resolve inside the repository"); }
  const canonicalPath = realpathSync(resolve(repositoryRoot, fixture.shared.canonical));
  const immediate = imports.map(specifier => fileURLToPath(new URL(specifier, pathToFileURL(stylesheetPath))));
  for (const path of immediate) { assert.equal(existsSync(path), true, "Hub admin stylesheet import does not exist"); assert.equal(inside(repositoryRoot, path), true, "Hub admin stylesheet import escapes the repository"); }
  assert.deepEqual(immediate.map(path => realpathSync(path)), [canonicalPath]);
  const manifestPath = resolve(repositoryRoot, fixture.shared.manifest);
  assert.equal(existsSync(manifestPath) && inside(repositoryRoot, manifestPath), true, "Shared style manifest does not resolve inside the repository");
  const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { readonly exports?: Readonly<Record<string, unknown>> };
  const exported = manifest.exports?.[fixture.shared.export];
  assert.equal(typeof exported, "string", "Shared style export is missing");
  assert.equal(realpathSync(fileURLToPath(new URL(exported, pathToFileURL(manifestPath)))), canonicalPath, "Shared style export does not resolve to the canonical stylesheet");
  assert.equal(existsSync(resolve(dirname(manifestPath), fixture.shared.export.slice(2))), false, "Shared style export must not rely on a compatibility duplicate");
  const visited = new Set<string>();
  let localImports = 0;
  const visit = (path: string): void => {
    const exact = realpathSync(path); if (visited.has(exact)) return; visited.add(exact);
    for (const match of readFileSync(exact, "utf8").matchAll(/^@import\s+(["'])([^"']+)\1\s*;/gmu)) {
      const specifier = match[2]!; if (!specifier.startsWith(".")) continue;
      const dependency = fileURLToPath(new URL(specifier, pathToFileURL(exact)));
      assert.equal(existsSync(dependency), true, "Shared stylesheet local import does not exist");
      assert.equal(inside(repositoryRoot, dependency), true, "Shared stylesheet local import escapes the repository");
      localImports++; visit(dependency);
    }
  };
  visit(canonicalPath);
  console.log(`[DEBUG] hub admin stylesheet graph oracle: ${fixture.laws.length} laws, ${imports.length} canonical import, ${sources.length} Tailwind sources, ${localImports} resolved shared imports across ${visited.size} stylesheets`);
}

/** 🩺️ Warns that the standalone Vite surface has no administrator authority; authenticated use is
 * exclusively owned by the loopback relay started by `os-hub:dev-secure-admin`. */
async function warnWhenHubIsUnreachable(): Promise<void> {
  const hubUrl = process.env.OS_HUB_URL ?? "http://127.0.0.1:8787";
  try {
    const response = await fetch(`${hubUrl}/admin/api/overview`, { signal: AbortSignal.timeout(2_000) });
    if (response.ok) {
      console.log(`[admin] hub reachable at ${hubUrl}; use bun nx run os-hub:dev-secure-admin for authenticated administration`);
      return;
    }
    console.warn(`[admin] hub at ${hubUrl} answered ${response.status}; use bun nx run os-hub:dev-secure-admin for the authenticated relay.`);
  } catch {
    console.warn(`[admin] no hub reachable at ${hubUrl}; this Vite surface is static-only and has no administrator credential.`);
    console.warn(`[admin] start the protected surface with: bun nx run os-hub:dev-secure-admin`);
  }
}

class DevScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await warnWhenHubIsUnreachable();
    await runViteBunxDev(this.root, ["--config", "⚙️vite.config.ts", ...segments], {
      portEnv: "OS_HUB_ADMIN_DEV_PORT",
      defaultPort: "8790",
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    verifyAdminEntryGraph(this.root);
    verifyAdminStylesheetGraph(this.root);
    runViteBuild(this.root, segments, "⚙️vite.config.ts");
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    verifyAdminEntryGraph(this.root);
    verifyAdminStylesheetGraph(this.root);
    await runVitest(this.root, rest, "🧪️tests/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
