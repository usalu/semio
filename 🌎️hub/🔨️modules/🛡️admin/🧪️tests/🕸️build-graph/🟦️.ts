import { strict as assert } from "node:assert";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import Ajv from "ajv";

const ADMIN_SCHEMA_MODULE = "../../🧬️schema/🔣️.json";
const ADMIN_SCHEMA_ID = "https://semio.tech/schema/hub/admin/schema.json";
const ADMIN_ENTRY_GRAPH_LAWS = ["html-module-entry-exists", "package-export-matches-html", "entry-contained-by-package", "single-canonical-entry"] as const;
const ADMIN_STYLESHEET_GRAPH_LAWS = ["imports-before-tailwind-sources", "repository-imports-resolve", "repository-imports-contained", "shared-export-is-canonical", "no-compatibility-duplicate"] as const;

/** 🧬️ Compiles one `hub.admin` export from the scope-owned draft-07 module next to the module itself. */
function adminSchemaExport(root: string, exportId: string): (value: unknown) => boolean {
  const document = JSON.parse(readFileSync(resolve(root, ADMIN_SCHEMA_MODULE), "utf8")) as { readonly $schema: string; readonly $id: string };
  assert.equal(document.$schema, "http://json-schema.org/draft-07/schema#", "hub.admin module must declare the draft-07 dialect");
  assert.equal(document.$id, ADMIN_SCHEMA_ID, "hub.admin module must declare its scope $id");
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${ADMIN_SCHEMA_ID}#/$defs/${exportId}`);
  assert.ok(validate, `hub.admin exports no ${exportId}`);
  return (value: unknown): boolean => validate(value) as boolean;
}

type AdminEntryGraph = { readonly version: number; readonly html: string; readonly manifest: string; readonly entry: string; readonly laws: readonly string[] };

/** 🚪️ Verifies the browser entry against the owned graph example and schema. */
export function verifyAdminEntryGraph(root: string): void {
  const fixture = JSON.parse(readFileSync(resolve(root, "../../🧫️fixtures/🕸️build-graph/🚪️entry-graph.json"), "utf8")) as AdminEntryGraph;
  const validate = adminSchemaExport(root, "AdminEntryGraphV1");
  assert.equal(validate(fixture), true, "Hub admin entry graph violates schema://hub.admin/AdminEntryGraphV1");
  assert.equal(validate({ ...fixture, locator: "private" }), false, "Hub admin entry graph must reject unknown members");
  assert.equal(validate({ ...fixture, laws: fixture.laws.slice(1) }), false, "Hub admin entry graph must enumerate every law");
  assert.deepEqual([...fixture.laws].sort(), [...ADMIN_ENTRY_GRAPH_LAWS].sort());
  assert.equal(new Set(fixture.laws).size, ADMIN_ENTRY_GRAPH_LAWS.length);
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

/** 🎨️ Verifies the stylesheet against the owned graph example and schema. */
export function verifyAdminStylesheetGraph(root: string): void {
  const fixture = JSON.parse(readFileSync(resolve(root, "../../🧫️fixtures/🕸️build-graph/🎨️stylesheet-graph.json"), "utf8")) as AdminStylesheetGraph;
  const validate = adminSchemaExport(root, "AdminStylesheetGraphV1");
  assert.equal(validate(fixture), true, "Hub admin stylesheet graph violates schema://hub.admin/AdminStylesheetGraphV1");
  assert.equal(validate({ ...fixture, locator: "private" }), false, "Hub admin stylesheet graph must reject unknown members");
  assert.equal(validate({ ...fixture, shared: { ...fixture.shared, export: "./🎨️.css" } }), false, "Hub admin stylesheet graph must pin the shared canonical export");
  assert.deepEqual([...fixture.laws].sort(), [...ADMIN_STYLESHEET_GRAPH_LAWS].sort());
  assert.equal(new Set(fixture.laws).size, ADMIN_STYLESHEET_GRAPH_LAWS.length);
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

