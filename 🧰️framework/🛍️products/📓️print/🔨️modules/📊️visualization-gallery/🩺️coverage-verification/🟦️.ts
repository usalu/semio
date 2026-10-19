/** 🩺️ The coverage of the visualization catalogue: the registered families, the printed key findings, the coverage report and the verification of the whole catalogue. */
import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { basename, join } from "node:path";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { vizOptionList, type VizFamilyOptions } from "../../../🧬️schema/🟦️.ts";
import { GENERATED_NOTE, loadVizCatalog, loadVizSchema, marksItselfGenerated, parseVizTaxonomyLeaves, productRoot, VIZ_GALLERY_DIR, VIZ_LATEX_DIR, VIZ_TAXONOMY_PATH } from "../🟦️.ts";
import { type VizApiKey, vizApiReference, vizDocumentHelperScopes, vizDocumentKeyReader, vizDocumentKeyScopes } from "../🧾️api-reference/🟦️.ts";
import { vizImplementedFamilyKeys, vizImplementedFamilyKeyScopes, vizKeysModel } from "../🔑️implemented-keys/🟦️.ts";
import { assertVizApi, VIZ_API_COMMANDS } from "../🗂️api-commands/🟦️.ts";
import { vizPrintedKeyRows, type VizPrintedKeySpec, vizPrintedKeyTables, vizPrintedSections, vizPrintedSource } from "../📋️printed-key-tables/🟦️.ts";

//#region 🔖️Coverage
function parseVizCovers(dir: string): Set<string> {
  const covers = new Set<string>();
  if (!existsSync(dir)) return covers;
  for (const name of readdirSync(dir).filter((file) => file.endsWith(".tex"))) {
    const source = readFileSync(join(dir, name), "utf8");
    for (const match of source.matchAll(/% viz-covers:\s+(\S+)/g)) covers.add(match[1]!);
  }
  return covers;
}

/** 👨‍👩‍👧 Every family name any LaTeX package registers with `\SemioVizFamily`. A registration that
 * stands inside a comment is prose about the command, not a call to it — `semio-viz-family.sty`
 * documents its own signature as `\SemioVizFamily{name}{code}` — so commented lines are skipped. */
export function registeredVizFamilies(): ReadonlySet<string> {
  const families = new Set<string>();
  const commented = (source: string, at: number): boolean => source.slice(source.lastIndexOf("\n", at) + 1, at).includes("%");
  for (const name of readdirSync(VIZ_LATEX_DIR).filter((file) => file.endsWith(".sty"))) {
    const source = readFileSync(join(VIZ_LATEX_DIR, name), "utf8");
    for (const match of source.matchAll(/\\SemioVizFamily\s*\{\s*([a-z0-9-]+)\s*\}/g)) if (!commented(source, match.index)) families.add(match[1]!);
    // 🏷️ A kernel package that is loaded before the registry registers from a begindocument hook,
    // which reaches the same registry through its internal entry point.
    for (const match of source.matchAll(/\\semio_viz_family_define:nn\s*\{\s*([a-z0-9-]+)\s*\}/g)) if (!commented(source, match.index)) families.add(match[1]!);
  }
  return families;
}

/** 🔑️ Rejects a key documented only in another owner's table, prose or a comment. */
export function vizPrintedKeyFindings(source: string, required: Readonly<Record<string, readonly string[]>>): readonly string[] {
  const tables = vizPrintedKeyTables(source);
  return Object.entries(required).flatMap(([owner, keys]) => keys.filter(key => !tables[owner]?.includes(key)).map(key => owner + ":" + key)).sort();
}

/** 📚️ Admits inherited rows only through explicit, source-reachable printed references. */
export function vizPrintedScopeFindings(
  source: string,
  required: Readonly<Record<string, Readonly<Record<string, VizPrintedKeySpec>>>>,
  reachable: Readonly<Record<string, Readonly<Record<string, readonly string[]>>>>,
  bindings: Readonly<Record<string, string>>,
): readonly string[] {
  const text = vizPrintedSource(source), tables = vizPrintedKeyRows(text);
  const sections = vizPrintedSections(text);
  const declared = new Map([...text.matchAll(/\\ApiScopeSource\{([a-z0-9-]+)\}\{([^}]+)\}/g)].map(match => [match[1]!, match[2]!.replaceAll("\\_", "_")]));
  const findings: string[] = [], aliases: Record<string, string> = { fp: "number", int: "integer", bool: "boolean", tl: "string", clist: "string", mm: "number", deg: "number", pt: "number", rad: "number", name: "string", colour: "string", list: "string", choice: "string", column: "string", "TikZ style": "string", text: "string", pattern: "string", path: "string", records: "string", "TeX font": "string", "light|dark": "string", spec: "string", anchor: "string", "x,y": "string", "dx,dy": "string", "lo,hi": "string", dimension: "string", "agg:column": "string", "table:left:right": "string", "key list": "string", "key:value": "string", "kind:column:size": "string", "kind:x:y": "string", "x:series:value": "string", "fp predicate": "string", "id:c1;c2;…": "string", data: "string", rule: "string" };
  const type = (value: string): string => aliases[value.trim()] ?? value.split("|").map(part => (aliases[part.trim()] ?? part.trim()).toLowerCase()).sort().join("|");
  const scalar = (value: string): string => value.trim().replace(/\s*,\s*/g, ",").toLowerCase();
  for (const [family, options] of Object.entries(required)) {
    const section = sections.get(family);
    if (section === undefined) { findings.push(family + ":missing-section"); continue; }
    const rows = [...(vizPrintedKeyRows(section)[family] ?? [])];
    for (const link of section.matchAll(/\\ApiKeyScope\{([a-z0-9-]+)\}/g)) {
      const table = link[1]!, scope = bindings[table];
      const accepted = scope === "registry" ? ["variant"] : reachable[family]?.[scope ?? ""];
      if (!accepted || !tables[table]) { findings.push(family + ":invalid-scope:" + table); continue; }
      if (declared.get(table) !== scope) { findings.push(family + ":invalid-binding:" + table); continue; }
      for (const row of tables[table]!) {
        for (const key of row.names) if (!accepted.includes(key)) findings.push(family + ":unreachable:" + table + ":" + key);
        rows.push({ ...row, names: row.names.filter(key => accepted.includes(key)) });
      }
    }
    for (const [key, spec] of Object.entries(options)) {
      const row = rows.find(candidate => candidate.names.includes(key));
      if (!row) { findings.push(family + ":" + key); continue; }
      if (type(row.type) !== type(spec.type)) findings.push(family + ":" + key + ":type");
      if (spec.default !== undefined && scalar(row.default) !== scalar(spec.default)) findings.push(family + ":" + key + ":default");
      for (const language of ["en", "de"] as const) {
        if (!row.meaning[language].trim()) findings.push(family + ":" + key + ":" + language);
        else if (spec.unit && !row.meaning[language].toLowerCase().includes(spec.unit.toLowerCase())) findings.push(family + ":" + key + ":" + language + ":unit");
      }
    }
  }
  return [...new Set(findings)].sort();
}

/** 📚️ Verifies every family option using the authored schema and actual source forwarding graph. */
export function vizPrintedFamilyFindings(source: string, bindings: Readonly<Record<string, string>>): readonly string[] {
  const required = Object.fromEntries(vizApiReference().families.map(family => [family.name, Object.fromEntries(family.options.map(option => [option.name, { type: option.type, ...(option.default ? { default: option.default } : {}) }]))]));
  return vizPrintedScopeFindings(source, required, vizImplementedFamilyKeyScopes(), bindings);
}

/** 📦️ Every emitted package key requires a rendered row in its actual l3keys owner scope. */
export function vizPrintedPackageFindings(source: string, implementations?: Readonly<Record<string, string>>): readonly string[] {
  const packages = implementations ?? Object.fromEntries(readdirSync(VIZ_LATEX_DIR).filter(name => name.startsWith("semio-viz") && name.endsWith(".sty")).sort().map(name => [basename(name, ".sty"), readFileSync(join(VIZ_LATEX_DIR, name), "utf8")]));
  const model = implementations ? vizKeysModel(Object.values(implementations)) : vizKeysModel(), read = vizDocumentKeyReader(source, model), helpers = vizDocumentHelperScopes(source), findings: string[] = [];
  for (const [pack, implementation] of Object.entries(packages)) for (const [scope, entry] of vizDocumentKeyScopes(implementation, model, helpers)) {
    let rows: ReadonlyMap<string, VizApiKey>;
    try { rows = read(scope); } catch (error) { findings.push(pack + ":" + scope + ":" + String(error)); continue; }
    for (const name of entry.names) if (!rows.has(name)) findings.push(pack + ":" + scope + ":" + name + ":row");
  }
  return [...new Set(findings)].sort();
}

/** 🧪️ The full structural report of the catalogue against the taxonomy, the schema and the LaTeX packages. */
export function vizCoverageReport(): {
  readonly leaves: number;
  readonly kinds: number;
  readonly uncovered: readonly string[];
  readonly overcovered: readonly string[];
  readonly unknownFamilies: readonly string[];
  readonly unknownOptions: readonly string[];
  readonly missingVariant: readonly string[];
  readonly duplicateSignatures: readonly string[];
  readonly untitled: readonly string[];
  readonly unknownDemoTables: readonly string[];
  readonly undocumentedOptions: readonly string[];
  readonly phantomOptions: readonly string[];
} {
  const md = readFileSync(VIZ_TAXONOMY_PATH, "utf8");
  const leaves = parseVizTaxonomyLeaves(md);
  const catalog = loadVizCatalog();
  const schema = loadVizSchema();
  const familyOptions = schema["x-semio-family-options"] as Record<string, VizFamilyOptions>;
  const demoTables = new Set(schema["x-semio-demo-tables"].map((table) => table.name));
  const registered = registeredVizFamilies();

  const coverCount = new Map<string, number>();
  for (const entry of catalog.kinds) for (const leaf of entry.covers) coverCount.set(leaf, (coverCount.get(leaf) ?? 0) + 1);

  const signatures = new Map<string, string[]>();
  const unknownOptions: string[] = [];
  const missingVariant: string[] = [];
  const untitled: string[] = [];
  const unknownDemoTables: string[] = [];
  for (const entry of catalog.kinds) {
    const signature = `${entry.family}|${vizOptionList(entry.options)}`;
    signatures.set(signature, [...(signatures.get(signature) ?? []), entry.slug]);
    const declared = familyOptions[entry.family]?.options ?? {};
    for (const key of Object.keys(entry.options)) if (!(key in declared)) unknownOptions.push(`${entry.slug}:${key}`);
    if ("variant" in declared && typeof entry.options.variant !== "string") missingVariant.push(`${entry.slug}:variant`);
    if (entry.title.en.length === 0 || entry.title.de.length === 0) untitled.push(entry.slug);
    if (!demoTables.has(entry.data)) unknownDemoTables.push(`${entry.slug}:${entry.data}`);
  }

  const implemented = vizImplementedFamilyKeys();
  const undocumentedOptions: string[] = [];
  const phantomOptions: string[] = [];
  for (const [family, keys] of Object.entries(implemented)) {
    const documented = familyOptions[family]?.options;
    if (documented === undefined) {
      undocumentedOptions.push(`${family}:*`);
      continue;
    }
    for (const key of keys) if (!(key in documented)) undocumentedOptions.push(`${family}:${key}`);
    for (const key of Object.keys(documented)) if (!keys.includes(key)) phantomOptions.push(`${family}:${key}`);
  }
  for (const family of Object.keys(familyOptions)) if (implemented[family] === undefined) phantomOptions.push(`${family}:*`);

  return {
    leaves: leaves.length,
    kinds: catalog.kinds.length,
    uncovered: leaves.filter((leaf) => (coverCount.get(leaf) ?? 0) === 0),
    overcovered: leaves.filter((leaf) => (coverCount.get(leaf) ?? 0) > 1),
    unknownFamilies: [...new Set(catalog.kinds.map((entry) => entry.family))].filter((family) => !registered.has(family)),
    unknownOptions,
    missingVariant,
    duplicateSignatures: [...signatures].filter(([, slugs]) => slugs.length > 1).map(([signature]) => signature),
    untitled,
    unknownDemoTables,
    undocumentedOptions,
    phantomOptions,
  };
}

/** 🔓️ Compares machine API metadata with the current owned generator's exact bytes. */
export function vizApiReferenceFindings(metadata?: string): readonly string[] {
  const path = "🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-api.json";
  const expected = `${JSON.stringify({ _comment: GENERATED_NOTE, ...vizApiReference() }, null, 2)}\n`;
  const actual = metadata ?? readFileSync(join(findWorkspaceRoot(), path), "utf8");
  if (actual !== expected) return [`stale ${path}`];
  return marksItselfGenerated(path, actual) ? [] : [`unmarked ${path}`];
}

/** 🧪️ Checks the whole visualization catalogue: taxonomy coverage, gallery, schema and public API examples. */
export function verifyVisualizationCoverage(metadata?: string): void {
  assert.deepEqual(vizApiReferenceFindings(metadata), []);
  const report = vizCoverageReport();
  const leaves = parseVizTaxonomyLeaves(readFileSync(VIZ_TAXONOMY_PATH, "utf8"));
  const taxonomy = JSON.parse(readFileSync(join(productRoot, "🖼️assets/🔣️viz-taxonomy.json"), "utf8")) as { readonly id: string; readonly section: string }[];
  assert.deepEqual(leaves, taxonomy.map((entry) => entry.id));
  assert.equal(new Set(taxonomy.map((entry) => entry.section)).size, 80);
  assert.equal(new Set(leaves).size, leaves.length);
  assert.deepEqual(report.uncovered, []);
  assert.deepEqual(report.overcovered, []);
  assert.deepEqual(report.unknownOptions, []);
  assert.deepEqual(report.missingVariant, []);
  assert.deepEqual(report.duplicateSignatures, []);
  assert.deepEqual(report.untitled, []);
  assert.deepEqual(report.unknownDemoTables, []);
  assert.deepEqual(leaves.filter((leaf) => !parseVizCovers(VIZ_GALLERY_DIR).has(leaf)), []);
  assert.deepEqual(assertVizApi().missing, []);
  console.log(`[TRACE] print: viz coverage ${report.leaves}/${report.leaves} leaves through ${report.kinds} kinds, API ${VIZ_API_COMMANDS.length}/${VIZ_API_COMMANDS.length}`);
  if (report.unknownFamilies.length > 0) console.log(`[TRACE] print: ${report.unknownFamilies.length} families still awaiting a \\SemioVizFamily registration: ${report.unknownFamilies.slice(0, 8).join(" ")}…`);
}

/** 🧪️ Long-level check: every catalogue family is actually registered by a LaTeX package. */
export function verifyVisualizationFamilies(): void {
  const report = vizCoverageReport();
  assert.deepEqual(report.unknownFamilies, []);
  console.log(`[TRACE] print: every viz family of ${report.kinds} kinds is registered`);
}
//#endregion 🔖️Coverage
