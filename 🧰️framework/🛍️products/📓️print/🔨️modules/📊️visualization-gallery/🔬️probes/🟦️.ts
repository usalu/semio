/** 🔏️ The PDF measurement tool of the gallery cases: it expands a document's compressed streams,
 * strips the metadata a second run of the same source rewrites (dates, ids, producer, XMP), and
 * digests what is left, so two renders can be compared for sameness and two kinds for distinctness.
 * It lives here because a digest is evidence a test gathers — the gallery module itself neither
 * hashes nor inflates anything.
 */
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { join } from "node:path";
import { inflateRawSync, inflateSync } from "node:zlib";
import { getWorkspaceRoot } from "../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { VIZ_GALLERY_THEMES, VIZ_LANGUAGES, type VizGalleryRenderEvidence, type VizGalleryTheme, type VizGalleryVariantMeasurement, type VizLanguage } from "../../../🧬️schema/🟦️.ts";
import { compilePrintTexOnce } from "../../🖨️tectonic-template-compilation/🟦️.ts";
import { loadVizCatalog, visualizationTemplates } from "../🟦️.ts";


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

//#region 🖼️GalleryMatrix
const workspaceRoot = getWorkspaceRoot();
const productRoot = join(workspaceRoot, "🧰️framework", "🛍️products", "📓️print");

/** 🖼️ The committed evidence file of the `🖼️gallery-render` case. */
export const PRINT_GALLERY_EVIDENCE_PATH = join(productRoot, "🧫️fixtures", "🖼️gallery-render", "🖼️gallery-render.json");

/** 🖼️ The committed evidence shape, as `🧬️schema/🔣️.json#/$defs/GalleryRenderEvidence` declares it. */
export type PrintGalleryFixture = VizGalleryRenderEvidence;

/** 🧭️ One cell of the gallery matrix: a generated section rendered in one theme and one language. */
export type PrintGalleryVariant = Readonly<{ id: string; section: string; texPath: string; theme: VizGalleryTheme; language: VizLanguage }>;

/** 🧭️ Every generated gallery section x {light, dark} x {en, de}, optionally narrowed to the named sections. */
export function printGalleryMatrix(sections?: readonly string[]): readonly PrintGalleryVariant[] {
  const galleries = visualizationTemplates().filter(({ texPath }) => texPath.startsWith("🧾️template/📊️viz-gallery/"));
  const unknown = (sections ?? []).filter((section) => !galleries.some(({ id }) => id === section));
  if (unknown.length > 0) throw new Error(`unknown gallery section(s): ${unknown.join(", ")}`);
  return galleries
    .filter(({ id }) => sections === undefined || sections.length === 0 || sections.includes(id))
    .flatMap(({ id, texPath }) => VIZ_GALLERY_THEMES.flatMap((theme) => VIZ_LANGUAGES.map((language) => ({ id: `${id}/${theme}/${language}`, section: id, texPath, theme, language }))));
}

/** 📝️ The section source with its document class naming this variant's theme and language, so no default ever applies. */
export function printGallerySource(variant: PrintGalleryVariant): string {
  const source = readFileSync(join(productRoot, variant.texPath), "utf8");
  const options = /\\documentclass\[([^\]]*)\]\{semio\}/.exec(source);
  if (options === null) throw new Error(`${variant.texPath}: no \\documentclass[…]{semio} line to derive the variant from`);
  const kept = options[1]!.split(",").map((option) => option.trim()).filter((option) => option !== "" && !/^(theme|language)=/.test(option));
  return source.replace(options[0], `\\documentclass[${[...kept, `theme=${variant.theme}`, `language=${variant.language}`].join(",")}]{semio}`);
}

/** 🏷️ The catalogue kinds a section draws, in document order, with their titles in the variant's language. */
function sectionKinds(source: string, language: VizLanguage): readonly { slug: string; title: string }[] {
  const titles = new Map(loadVizCatalog().kinds.map((kind) => [kind.slug, kind.title[language]]));
  return [...source.matchAll(/\\SemioVizChart\{([^}]+)\}/g)].map((match) => {
    const title = titles.get(match[1]!);
    if (title === undefined) throw new Error(`gallery draws kind ${match[1]} that the catalogue does not declare`);
    return { slug: match[1]!, title };
  });
}

function normalizedText(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

/** 📖️ The text of every page, read through PDF.js — a reader independent of the renderer. */
async function pdfPageTexts(pdfPath: string): Promise<readonly string[]> {
  const canvas = createRequire(join(workspaceRoot, "node_modules/pdfjs-dist/legacy/build/pdf.mjs"))("@napi-rs/canvas") as typeof import("@napi-rs/canvas");
  (globalThis as { DOMMatrix?: typeof canvas.DOMMatrix }).DOMMatrix ??= canvas.DOMMatrix;
  const { getDocument } = await import("pdfjs-dist/legacy/build/pdf.mjs");
  const pdf = await getDocument({ data: new Uint8Array(readFileSync(pdfPath)) }).promise;
  try {
    const texts: string[] = [];
    for (let index = 1; index <= pdf.numPages; index++) {
      const page = await pdf.getPage(index);
      texts.push(normalizedText((await page.getTextContent()).items.map((item) => ("str" in item ? item.str : "")).join(" ")));
      page.cleanup();
    }
    return texts;
  } finally {
    await pdf.destroy();
  }
}

/** 📏️ Renders one variant in a cache-local work directory and measures it: page count, the page and page text of every kind the
 * section draws (a kind whose title appears on no page fails the measurement), and the rebuild-stable hash of the PDF. */
export async function measurePrintGalleryVariant(variant: PrintGalleryVariant, signal?: AbortSignal): Promise<VizGalleryVariantMeasurement> {
  const workDirectory = join(workspaceRoot, ".🧬semio", "🦑️repo", "⚡️cache", "tests", "gallery-render", variant.id.replaceAll("/", "-"));
  const outDirectory = join(workDirectory, "out");
  const jobName = variant.id.replaceAll("/", "-");
  rmSync(workDirectory, { recursive: true, force: true });
  mkdirSync(outDirectory, { recursive: true });
  const source = printGallerySource(variant);
  const texPath = join(workDirectory, `${jobName}.tex`);
  writeFileSync(texPath, source);
  await compilePrintTexOnce(texPath, outDirectory, workDirectory, signal);
  const pdfPath = join(outDirectory, `${jobName}.pdf`);
  const pages = await pdfPageTexts(pdfPath);
  const kinds: Record<string, { page: number; text: string }> = {};
  for (const { slug, title } of sectionKinds(source, variant.language)) {
    const index = pages.findIndex((text) => text.includes(normalizedText(title)));
    if (index < 0) throw new Error(`${variant.id}: kind ${slug} ("${title}") appears on no page`);
    kinds[slug] = { page: index + 1, text: pages[index]! };
  }
  const measurement = { pages: pages.length, kinds, hash: pdfStableHash(pdfPath) };
  rmSync(workDirectory, { recursive: true, force: true });
  return measurement;
}

/** 🖼️ Regenerates the committed evidence: measures the matrix (or the named sections of it, keeping every other committed
 * variant) and writes the fixture with sorted keys. Progress is one line per measured variant; the signal cancels between
 * and inside compilations. */
export async function writePrintGalleryEvidence(sections: readonly string[], signal?: AbortSignal): Promise<PrintGalleryFixture> {
  const committed = JSON.parse(readFileSync(PRINT_GALLERY_EVIDENCE_PATH, "utf8")) as PrintGalleryFixture;
  const matrix = printGalleryMatrix(sections);
  const variants: Record<string, VizGalleryVariantMeasurement> = sections.length === 0 ? {} : { ...committed.variants };
  for (const [index, variant] of matrix.entries()) {
    signal?.throwIfAborted();
    variants[variant.id] = await measurePrintGalleryVariant(variant, signal);
    console.log(`[gallery] ${index + 1}/${matrix.length} ${variant.id}: ${variants[variant.id]!.pages} pages`);
  }
  const fixture: PrintGalleryFixture = { schemaVersion: 1, generatedBy: committed.generatedBy, variants: Object.fromEntries(Object.keys(variants).sort().map((id) => [id, variants[id]!])) };
  writeFileSync(PRINT_GALLERY_EVIDENCE_PATH, `${JSON.stringify(fixture, null, 2)}\n`);
  return fixture;
}
//#endregion 🖼️GalleryMatrix
