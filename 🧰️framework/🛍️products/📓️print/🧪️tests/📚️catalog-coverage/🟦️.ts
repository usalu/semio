/** 🧪️ Adapter for `catalog-coverage`: the structural gate in front of every visualization renderer.
 *
 * Subject: `🖼️assets/🔣️viz-catalog.json` and the files `generate viz` derives from it.
 * Oracles: `ajv` (draft 2020-12) for the entry shape, `markdown-it` for the taxonomy leaf set.
 * Both are test-only dependencies declared in the print package's `devDependencies`.
 */
import assert from "node:assert/strict";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import Ajv2020 from "ajv/dist/2020";
import MarkdownIt from "markdown-it";
import { getWorkspaceRoot } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { defineTestAdapter } from "../../../🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
import { loadVizCatalog, loadVizSchema, marksItselfGenerated, registeredVizFamilies, vizCoverageReport, vizGeneratedFiles, vizImplementedFamilyKeys } from "../../🔨️modules/📊️visualization-gallery/🟦️.ts";
import type { LocalizedText } from "../../🧬️schema/🟦️.ts";
import { vizOptionList } from "../../🧬️schema/🟦️.ts";

const repoRoot = getWorkspaceRoot();
const productRoot = join(repoRoot, "🧰️framework/🛍️products/📓️print");
const LATEX_DIR = join(productRoot, "🖋️latex");
const LEAF_KINDS = new Set(["mark", "chart", "layout", "axis", "scale"]);

//#region 🔖️Oracles
/** 🥒️ Reads the taxonomy leaf identifiers out of a markdown-it token stream. */
export function markdownItLeaves(md: string): readonly string[] {
  const tokens = new MarkdownIt().parse(md, {});
  const leaves: string[] = [];
  let section: string | undefined;
  for (let index = 0; index < tokens.length; index += 1) {
    const token = tokens[index]!;
    if (token.type === "heading_open" && token.tag === "h2") section = String(Number.parseInt(tokens[index + 1]!.content, 10));
    if (token.type !== "inline" || section === undefined) continue;
    const children = token.children ?? [];
    const code = children.findIndex((child) => child.type === "code_inline");
    if (code < 0 || !LEAF_KINDS.has(children[code + 1]?.content.trim() ?? "")) continue;
    leaves.push(`${section}/${children[code]!.content}`);
  }
  return leaves;
}

/** 🥒️ Validates the catalogue document against its JSON Schema with ajv in draft 2020-12 mode. */
export function ajvErrors(): readonly string[] {
  const ajv = new Ajv2020({ allErrors: true, strict: false });
  const validate = ajv.compile(loadVizSchema());
  return validate(loadVizCatalog()) ? [] : (validate.errors ?? []).map((error) => `${error.instancePath} ${error.message}`);
}
//#endregion 🔖️Oracles

//#region 🔖️Scenarios
/** 🧪️ `@id-schema`: ajv accepts the catalogue. */
export function scenarioSchema(): void {
  assert.deepEqual(ajvErrors(), []);
}

/** 🧪️ `@id-leaves`: every markdown-it leaf is covered exactly once and nothing else is. */
export function scenarioLeaves(): void {
  const oracle = markdownItLeaves(readFileSync(join(productRoot, "🖼️assets/📊️viz-taxonomy.md"), "utf8"));
  const covered = new Map<string, number>();
  for (const entry of loadVizCatalog().kinds) for (const leaf of entry.covers) covered.set(leaf, (covered.get(leaf) ?? 0) + 1);
  assert.deepEqual(oracle.filter((leaf) => covered.get(leaf) !== 1), []);
  assert.deepEqual([...covered.keys()].filter((leaf) => !oracle.includes(leaf)), []);
}

/** 🧪️ `@id-slugs`: slugs are global, section-free and unique. */
export function scenarioSlugs(): void {
  const slugs = loadVizCatalog().kinds.map((entry) => entry.slug);
  assert.deepEqual(slugs.filter((slug) => /-(?:[0-9]|[1-7][0-9])$/.test(slug)), []);
  assert.equal(new Set(slugs).size, slugs.length);
}

/** 🧪️ `@id-families`: every catalogue family is registered once by a LaTeX package. */
export function scenarioFamilies(): void {
  const occurrences = new Map<string, string[]>();
  for (const name of readdirSync(LATEX_DIR).filter((file) => file.endsWith(".sty"))) {
    for (const match of readFileSync(join(LATEX_DIR, name), "utf8").matchAll(/\\SemioVizFamily\{([a-z0-9-]+)\}/g)) {
      occurrences.set(match[1]!, [...(occurrences.get(match[1]!) ?? []), name]);
    }
  }
  assert.deepEqual([...occurrences].filter(([, packages]) => new Set(packages).size > 1).map(([family]) => family), []);
  const registered = registeredVizFamilies();
  assert.deepEqual([...new Set(loadVizCatalog().kinds.map((entry) => entry.family))].filter((family) => !registered.has(family)), []);
}

/** 🧪️ `@id-options`: every option key and demo table an entry names is declared in the schema, and
 * an entry carries `variant` exactly when its family's schema vocabulary declares the key. */
export function scenarioOptions(): void {
  const report = vizCoverageReport();
  assert.deepEqual(report.unknownOptions, []);
  assert.deepEqual(report.unknownDemoTables, []);
  assert.deepEqual(report.missingVariant, []);
}

/** 🧪️ `@id-implemented-keys-documented`: the schema's per-family vocabulary equals what the
 * LaTeX sources implement, in both directions. */
export function scenarioImplementedKeysDocumented(): void {
  const report = vizCoverageReport();
  assert.deepEqual(report.undocumentedOptions, []);
  assert.deepEqual(report.phantomOptions, []);
}

/** 🧪️ `@id-distinctness`: family plus option list identifies exactly one chart kind. */
export function scenarioDistinctness(): void {
  const signatures = loadVizCatalog().kinds.map((entry) => `${entry.family}|${vizOptionList(entry.options)}`);
  assert.equal(new Set(signatures).size, signatures.length);
}

/** 🧪️ `@id-languages`: both document languages carry a title everywhere. */
export function scenarioLanguages(): void {
  const catalog = loadVizCatalog();
  const schema = loadVizSchema();
  const localized: LocalizedText[] = [
    ...catalog.kinds.map((entry) => entry.title),
    ...(schema["x-semio-taxonomy-sections"] as { title: LocalizedText }[]).map((entry) => entry.title),
    ...(schema["x-semio-taxonomy-groups"] as { title: LocalizedText }[]).map((entry) => entry.title),
  ];
  assert.deepEqual(localized.filter((title) => title.en.length === 0 || title.de.length === 0), []);
  const labels = readFileSync(join(LATEX_DIR, "semio-viz-catalog-labels.sty"), "utf8");
  assert.equal((labels.match(/^\\SemioVizKindLabel\{/gm) ?? []).length, catalog.kinds.length);
}

/** 🧪️ `@id-generated`: the checked-in generated files equal what the generator would write. */
export function scenarioGenerated(): void {
  for (const file of vizGeneratedFiles()) {
    assert.equal(readFileSync(join(repoRoot, file.path), "utf8"), file.content, `stale generated file ${file.path}`);
    assert.ok(marksItselfGenerated(file.path, file.content), `unmarked generated file ${file.path}`);
  }
}

/** 🧪️ Runs the fundamental scenarios: everything that needs no LaTeX package to exist yet. */
export function runCatalogCoverageCase(): void {
  scenarioSchema();
  scenarioLeaves();
  scenarioSlugs();
  scenarioOptions();
  scenarioImplementedKeysDocumented();
  scenarioDistinctness();
  scenarioLanguages();
  scenarioGenerated();
  console.log("print: catalog-coverage — 8 fundamental scenarios passed");
}

/** 🧪️ Runs the long scenario: every catalogue family is registered by a namespace package. */
export function runCatalogFamilyRegistration(): void {
  scenarioFamilies();
  console.log("print: catalog-coverage — family registration complete");
}
//#endregion 🔖️Scenarios

//#region 🔖️Adapter
/** 🧫️ How often each taxonomy leaf is covered, as the counts the two sides compare. */
function coverCounts(leaves: readonly string[]): Record<string, number> {
  const counts: Record<string, number> = {};
  for (const leaf of leaves) counts[leaf] = (counts[leaf] ?? 0) + 1;
  return counts;
}

/** 🧫️ Every leaf identifier the catalogue claims to cover, in catalogue order. */
function catalogLeaves(): string[] {
  return loadVizCatalog().kinds.flatMap((entry) => entry.covers);
}

/** 🧫️ Turns a conformance check into the pair of finding lists the profile compares: the
 * specification says the list is empty, so the oracle side is the empty list by construction. */
function findings(actual: readonly unknown[]): { projection: { findings: readonly unknown[] } } {
  return { projection: { findings: actual } };
}
const NONE = findings([]);

/** 🔍️ The slugs that still carry a taxonomy section suffix, plus any slug used twice. */
function slugFindings(): string[] {
  const slugs = loadVizCatalog().kinds.map((entry) => entry.slug);
  const seen = new Set<string>();
  const duplicates = slugs.filter((slug) => (seen.has(slug) ? true : (seen.add(slug), false)));
  return [...slugs.filter((slug) => /-(?:[0-9]|[1-7][0-9])$/.test(slug)), ...duplicates];
}

/** 🔍️ Every family that two packages register, and every family no package registers. */
function familyFindings(): string[] {
  const occurrences = new Map<string, string[]>();
  for (const name of readdirSync(LATEX_DIR).filter((file) => file.endsWith(".sty"))) {
    for (const match of readFileSync(join(LATEX_DIR, name), "utf8").matchAll(/\\SemioVizFamily\s*\{([a-z0-9-]+)\}/g)) {
      occurrences.set(match[1]!, [...(occurrences.get(match[1]!) ?? []), name]);
    }
  }
  const registered = registeredVizFamilies();
  return [
    ...[...occurrences].filter(([, packages]) => new Set(packages).size > 1).map(([family]) => `duplicate ${family}`),
    ...[...new Set(loadVizCatalog().kinds.map((entry) => entry.family))].filter((family) => !registered.has(family)).map((family) => `unregistered ${family}`),
  ];
}

/** 🔍️ Every option key and demo table an entry names that the schema does not declare, and every
 * entry missing a `variant` its family declares. The schema is the authority on both. */
function optionFindings(): string[] {
  const report = vizCoverageReport();
  return [...report.unknownOptions, ...report.unknownDemoTables, ...report.missingVariant];
}

/** 🔍️ Every key a family implements that the schema is silent about, and every key the schema
 * declares that no family path actually installs. */
function implementedKeyFindings(): string[] {
  const report = vizCoverageReport();
  return [...report.undocumentedOptions.map((entry) => `undocumented ${entry}`), ...report.phantomOptions.map((entry) => `phantom ${entry}`)];
}

/** 🔍️ Every family plus option list that identifies more than one chart kind. */
function distinctnessFindings(): string[] {
  const seen = new Set<string>();
  const duplicates: string[] = [];
  for (const entry of loadVizCatalog().kinds) {
    const signature = `${entry.family}|${vizOptionList(entry.options)}`;
    if (seen.has(signature)) duplicates.push(signature);
    seen.add(signature);
  }
  return duplicates;
}

/** 🔍️ Everything that is missing a title in one of the two document languages. */
function languageFindings(): string[] {
  const catalog = loadVizCatalog();
  const schema = loadVizSchema();
  const localized: { id: string; title: LocalizedText }[] = [
    ...catalog.kinds.map((entry) => ({ id: entry.slug, title: entry.title })),
    ...(schema["x-semio-taxonomy-sections"] as { id: string; title: LocalizedText }[]).map((entry) => ({ id: `section ${entry.id}`, title: entry.title })),
    ...(schema["x-semio-taxonomy-groups"] as { id: string; title: LocalizedText }[]).map((entry) => ({ id: `group ${entry.id}`, title: entry.title })),
  ];
  const labels = readFileSync(join(LATEX_DIR, "semio-viz-catalog-labels.sty"), "utf8");
  const declared = (labels.match(/^\\SemioVizKindLabel\{/gm) ?? []).length;
  return [
    ...localized.filter((entry) => entry.title.en.length === 0 || entry.title.de.length === 0).map((entry) => entry.id),
    ...(declared === catalog.kinds.length ? [] : [`semio-viz-catalog-labels.sty declares ${declared} of ${catalog.kinds.length} kinds`]),
  ];
}

const VIZ_API_ARTIFACT = "🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-api.json";

/** 🔍️ One generated file measured against what the generator would write, right now. */
function generatedFinding(file: { path: string; content: string }): string[] {
  const disk = readFileSync(join(repoRoot, file.path), "utf8");
  if (disk !== file.content) return [`stale ${file.path}`];
  return marksItselfGenerated(file.path, file.content) ? [] : [`unmarked ${file.path}`];
}

/** 🔍️ The catalogue-derived artifacts — the two packages and the gallery documents — out of step. */
function generatedFindings(): string[] {
  return vizGeneratedFiles()
    .filter((file) => file.path !== VIZ_API_ARTIFACT)
    .flatMap((file) => generatedFinding(file));
}

/** 🔍️ The API reference out of step with the LaTeX sources it is derived from. */
function apiReferenceFindings(): string[] {
  const file = vizGeneratedFiles().find((entry) => entry.path === VIZ_API_ARTIFACT);
  if (file === undefined) return [`missing ${VIZ_API_ARTIFACT}`];
  return generatedFinding(file);
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    schema: {
      /** 🔮️ ajv in draft 2020-12 mode is what adjudicates the entry shape; the specification is that it finds nothing. */
      oracle: () => NONE,
      /** 🎯️ The catalogue document as it stands, run through ajv against `🧬️schema/🔣️.json`. */
      subject: () => findings(ajvErrors()),
    },
    leaves: {
      /** 🔮️ The leaf identifiers markdown-it reads out of the taxonomy, each expected exactly once. */
      oracle: () => ({ projection: { covers: coverCounts(markdownItLeaves(readFileSync(join(productRoot, "🖼️assets/📊️viz-taxonomy.md"), "utf8"))) } }),
      /** 🎯️ The leaf identifiers the catalogue's own `covers` lists claim, with their multiplicity. */
      subject: () => ({ projection: { covers: coverCounts(catalogLeaves()) } }),
    },
    slugs: {
      /** 🔮️ Architecture §4: slugs are global, carry no section suffix and are unique. */
      oracle: () => NONE,
      /** 🎯️ The slugs that break that rule. */
      subject: () => findings(slugFindings()),
    },
    families: {
      /** 🔮️ Architecture §4: one family name, one owning package, and every catalogue family registered. */
      oracle: () => NONE,
      /** 🎯️ The families registered twice or not at all. */
      subject: () => findings(familyFindings()),
    },
    options: {
      /** 🔮️ Architecture §4: every option key and demo table an entry names lives in the schema vocabulary. */
      oracle: () => NONE,
      /** 🎯️ The option keys, demo tables and missing `variant` values that are not declared. */
      subject: () => findings(optionFindings()),
    },
    "implemented-keys-documented": {
      /** 🔮️ The specification: the schema's vocabulary of a family is exactly what the family implements. */
      oracle: () => NONE,
      /** 🎯️ The keys the LaTeX sources implement but the schema omits, and the ones it invents. */
      subject: () => findings(implementedKeyFindings()),
    },
    distinctness: {
      /** 🔮️ Architecture §4: family plus option list identifies exactly one chart kind. */
      oracle: () => NONE,
      /** 🎯️ The signatures two kinds share. */
      subject: () => findings(distinctnessFindings()),
    },
    languages: {
      /** 🔮️ Both document languages carry a title everywhere, and the generated labels package declares them all. */
      oracle: () => NONE,
      /** 🎯️ What is untitled in one language, and a label package out of step with the catalogue. */
      subject: () => findings(languageFindings()),
    },
    generated: {
      /** 🔮️ The generated packages and gallery documents equal what `generate viz` would write. */
      oracle: () => NONE,
      /** 🎯️ The generated files that are stale or miss the generated-file header. */
      subject: () => findings(generatedFindings()),
    },
    "api-reference": {
      /** 🔮️ `🔣️viz-api.json` equals what `generate viz` would write from today's LaTeX sources. */
      oracle: () => NONE,
      /** 🎯️ The API reference if it lags the packages, commands and keys it describes. */
      subject: () => findings(apiReferenceFindings()),
    },
  },
});
//#endregion 🔖️Adapter
