import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { basename, join } from "node:path";
import { inflateRawSync, inflateSync } from "node:zlib";
import { getWorkspaceRoot } from "../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const productRoot = join(getWorkspaceRoot(), "🧰️framework/🛍️products/📓️print");
const VIZ_GALLERY_DIR = join(productRoot, "🧾️template/📊️viz-gallery");
const VIZ_TAXONOMY_PATH = join(productRoot, "🖼️assets/📊️viz-taxonomy.md");

/** 📊️ Enumerates canonical visualization gallery documents. */
export function visualizationTemplates(): readonly { readonly id: string; readonly texPath: string }[] {
  return readdirSync(VIZ_GALLERY_DIR).filter((name) => name.endsWith(".tex") && !name.includes("-dark"))
    .sort().map((name) => ({ id: basename(name, ".tex").replace(/^[^a-z]+/i, ""), texPath: `🧾️template/📊️viz-gallery/${name}` }));
}

/** 🍃️ Reads terminal visualization identifiers from the taxonomy source. */
export function parseVizTaxonomyLeaves(md: string): string[] {
  const leaves: string[] = [];
  let section = "";
  for (const line of md.split(/\n/)) {
    const header = line.match(/^##\s+(\d+)/);
    if (header) {
      section = header[1]!;
      continue;
    }
    const leaf = line.match(/^- .+ `([^`]+)` (?:mark|chart|layout|axis|scale)$/);
    if (leaf && section) leaves.push(`${section}/${leaf[1]}`);
  }
  return leaves;
}

function parseVizCovers(dir: string): Set<string> {
  const covers = new Set<string>();
  if (!existsSync(dir)) return covers;
  for (const name of readdirSync(dir).filter((file) => file.endsWith(".tex"))) {
    const source = readFileSync(join(dir, name), "utf8");
    for (const match of source.matchAll(/% viz-covers:\s+(\S+)/g)) covers.add(match[1]!);
  }
  return covers;
}

function assertVizCoverage(): { readonly leaves: number; readonly missing: readonly string[] } {
  const leaves = parseVizTaxonomyLeaves(readFileSync(VIZ_TAXONOMY_PATH, "utf8"));
  const covers = parseVizCovers(VIZ_GALLERY_DIR);
  return { leaves: leaves.length, missing: leaves.filter((leaf) => !covers.has(leaf)) };
}

const VIZ_API_COMMANDS = [
  "\\SemioVizChart",
  "\\SemioVizMark",
  "\\SemioVizPath",
  "\\SemioVizLayout",
  "\\SemioVizAxis",
  "\\SemioVizGrid",
  "\\SemioVizLegend",
  "\\SemioVizTable",
  "\\SemioVizRow",
  "\\SemioVizScale",
  "\\SemioVizChartKind",
  "\\SemioVizDemo",
  "\\SemioVizText",
  "VizFigure",
  "VizSection",
  "VizColumn",
  "\\SemioVizNoteBelow",
] as const;

function assertVizApi(): { readonly missing: readonly string[] } {
  const source = readFileSync(join(productRoot, visualizationTemplates().find((template) => template.id === "viz-api")!.texPath), "utf8");
  return { missing: VIZ_API_COMMANDS.filter((command) => !source.includes(command)) };
}

function inflatePdfStreamBody(body: Buffer): Buffer {
  try {
    return inflateSync(body);
  } catch {
    return inflateRawSync(body);
  }
}

/** 🔏️ Hashes PDF bytes after removing volatile document metadata and expanding streams. */
export function pdfStableHash(pdfPath: string): string {
  const raw = readFileSync(pdfPath).toString("binary");
  const inflated = raw.replace(/stream\r?\n([\s\S]*?)\r?\nendstream/g, (_all, body: string) => {
    try {
      return `stream\n${inflatePdfStreamBody(Buffer.from(body, "binary")).toString("binary")}\nendstream`;
    } catch {
      return `stream\n${body}\nendstream`;
    }
  });
  const text = inflated
    .replace(/\/CreationDate\s*\([^)]*\)/g, "")
    .replace(/\/ModDate\s*\([^)]*\)/g, "")
    .replace(/\/ID\s*\[[^\]]*\]/g, "")
    .replace(/\(D:[0-9+\-'Z]+\)/g, "")
    .replace(/\/Producer\s*\([^)]*\)/g, "")
    .replace(/\/Creator\s*\([^)]*\)/g, "")
    .replace(/<x:xmpmeta[\s\S]*?<\/x:xmpmeta>/g, "");
  return createHash("sha256").update(text, "binary").digest("hex");
}


/** 🧪️ Checks all visualization taxonomy leaves and public API examples against their imported catalog. */
export function verifyVisualizationCoverage(): void {
  const coverage = assertVizCoverage();
  const catalog = JSON.parse(readFileSync(join(productRoot, "🖼️assets/🔣️viz-taxonomy.json"), "utf8")) as { readonly id: string; readonly section: string }[];
  const leaves = parseVizTaxonomyLeaves(readFileSync(VIZ_TAXONOMY_PATH, "utf8"));
  assert.deepEqual(leaves, catalog.map((entry) => entry.id));
  assert.equal(new Set(catalog.map((entry) => entry.section)).size, 80);
  assert.equal(new Set(leaves).size, leaves.length);
  assert.deepEqual(coverage.missing, []);
  assert.deepEqual(assertVizApi().missing, []);
  console.log(`[DEBUG] print: viz coverage ${coverage.leaves}/${coverage.leaves} leaves, API ${VIZ_API_COMMANDS.length}/${VIZ_API_COMMANDS.length}`);
}
