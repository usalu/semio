#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏗️ Third-party fixture generator for `s.stdio.semio@v1/📑️document`.
//
// Every byte this file commits is written BY A THIRD-PARTY LIBRARY, never assembled by hand and never
// produced by this repository's own exporter:
//   docx — `@xmldom/xmldom` serializes the WordprocessingML DOM, `jszip` writes the OPC container.
//   md   — `mdast-util-to-markdown` serializes an mdast tree to CommonMark.
// Nothing here reimplements ZIP, XML or CommonMark, and nothing here predicts what a mutation OUGHT to
// produce: a recipe DESCRIBES a before and an after document, the libraries encode both, and
// `../🔬️probes/📜️script.ts` reads them back with a DIFFERENT set of libraries.
//
// Generation and execution are SEPARATE operations. A normal test run must never be able to rewrite the
// expectation it is measured against, so this is its own command and its output is reviewed before it is
// committed.
//
//   bun 📜️script.ts generate [--out <dir>] [--only <recipe-id>]
//   bun 📜️script.ts manifests                     # emit the testEvidence block for 🔮️oracles
//
// @see ../🔣️oracle.json — the oracles these bytes are attributed to
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️document-subset-oracle.md
// @see ../../🔺️mesh/🏭️generator/📜️script.ts — the pilot this file mirrors in CLI shape and manifest fields

//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { cargoTargetDirectory } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { findWorkspaceRoot } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import JSZip from "jszip";
import { DOMImplementation, XMLSerializer } from "@xmldom/xmldom";
import { toMarkdown } from "mdast-util-to-markdown";
import { currentPlatform } from "../../../../../../../../../../🧰️framework/📦️packages/🟦️typescript/🧪️test/🔌️adapter/🟦️.ts";
import type { Run, Block, Doc, Recipe } from "./📇️document-model/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const SEED = 4815162342;
const DOCX_ORACLE = "jszip-xmldom-docx-carrier";
const MD_ORACLE = "mdast-to-markdown-md-carrier";
const JSZIP_VERSION = "3.10.1";
const XMLDOM_VERSION = "0.9.10";
const MDAST_TO_MARKDOWN_VERSION = "2.1.2";

const FIXTURE_DIRECTORY_BY_ID: Readonly<Record<string, string>> = {
  "insert-block-appends-a-paragraph": "🧱️insert-block-appends-a-paragraph",
  "insert-image": "🖼️insert-image",
  "insert-style-adds-a-named-style": "🧶️insert-style-adds-a-named-style",
  "remove-block-drops-a-paragraph": "🪓️remove-block-drops-a-paragraph",
  "remove-image": "🪦️remove-image",
  "remove-style-drops-a-named-style": "🧽️remove-style-drops-a-named-style",
  "set-block-content-replaces-a-paragraph": "📦️set-block-content-replaces-a-paragraph",
  "set-heading-level-demotes-the-title": "📐️set-heading-level-demotes-the-title",
  "set-image-block-corrects-the-caption": "📷️set-image-block-corrects-the-caption",
  "set-image-bytes": "📀️set-image-bytes",
  "set-list-ordered-numbers-the-list": "🔢️set-list-ordered-numbers-the-list",
  "set-paragraph-style-names-the-body-style": "🪶️set-paragraph-style-names-the-body-style",
  "set-run-style-emphasises-a-run": "🎨️set-run-style-emphasises-a-run",
  "set-run-text-rewrites-the-body-copy": "🧵️set-run-text-rewrites-the-body-copy",
  "set-style-based-on-reparents-a-style": "🧬️set-style-based-on-reparents-a-style",
  "set-style-name-renames-a-style": "🏷️set-style-name-renames-a-style",
};

const FIXTURE_FILENAME_BY_ROLE: Readonly<Record<string, string>> = {
  "before-docx": "⬅️before.docx",
  "after-docx": "➡️after.docx",
  "counterexample-docx": "⚠️counterexample.docx",
  "before-md": "⏮️before.md",
  "after-md": "⏭️after.md",
  "counterexample-md": "🚫️counterexample.md",
};

const CARRIER_KINDS = ["insert-image", "remove-image", "set-image-bytes"] as const;

/** 📎️ Fixture file paths are resolved against the OWNER'S ORACLE directory, never against this fixture
 *  directory — the registry loader stamps `manifestDir` to where `🔣️oracle.json` lives. A bare
 *  `<recipe>/<file>` therefore resolves to a non-existent `🔮️oracles/<recipe>/<file>` and every digest
 *  reads as a mismatch; the mesh pilot lost 369 fixtures to exactly that. */
const FIXTURE_PATH_PREFIX = "../🧪️tests/🧫️fixtures";

/** 📦️ A fixed 1980-01-01 stamp, not a wall clock. JSZip defaults `date` to `new Date()`, which would
 *  make every regenerated container differ from the committed one and `fixture reproduce` fail forever
 *  on a corpus that is in fact byte-identical. MEASURED: with this stamp two runs agree byte for byte. */
const ZIP_EPOCH = new Date(Date.UTC(1980, 0, 1, 0, 0, 0));

const W_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const CT_NS = "http://schemas.openxmlformats.org/package/2006/content-types";
const REL_NS = "http://schemas.openxmlformats.org/package/2006/relationships";
//#endregion 🧬️Contract

//#region 🧱️Model
//#endregion 🧱️Model

//#region 📜️Docx
type XmlDocument = ReturnType<DOMImplementation["createDocument"]>;
type XmlElement = ReturnType<XmlDocument["createElementNS"]>;

function element(doc: XmlDocument, name: string, attributes: readonly (readonly [string, string])[] = []): XmlElement {
  const node = doc.createElementNS(W_NS, name);
  for (const [key, value] of attributes) node.setAttribute(key, value);
  return node;
}

/** 📜️ One paragraph in WordprocessingML: an optional `w:pStyle`, then one `w:r` per run carrying only
 *  the three character properties `DocxRun` has fields for. Size, font, colour and link have no field in
 *  this subset's own docx model, so they are not invented here either. */
function docxParagraph(doc: XmlDocument, styleId: string | undefined, runs: readonly Run[]): XmlElement {
  const paragraph = element(doc, "w:p");
  if (styleId !== undefined) {
    const properties = element(doc, "w:pPr");
    properties.appendChild(element(doc, "w:pStyle", [["w:val", styleId]]));
    paragraph.appendChild(properties);
  }
  for (const run of runs) {
    const node = element(doc, "w:r");
    if (run.bold === true || run.italic === true || run.underline === true) {
      const properties = element(doc, "w:rPr");
      if (run.bold === true) properties.appendChild(element(doc, "w:b"));
      if (run.italic === true) properties.appendChild(element(doc, "w:i"));
      if (run.underline === true) properties.appendChild(element(doc, "w:u", [["w:val", "single"]]));
      node.appendChild(properties);
    }
    const text = element(doc, "w:t", [["xml:space", "preserve"]]);
    text.appendChild(doc.createTextNode(run.text));
    node.appendChild(text);
    paragraph.appendChild(node);
  }
  return paragraph;
}

/** 🧱 The block→docx mapping this subset's own serializer documents: `List` and `Quote` FLATTEN (their
 *  grouping is lost), `Code` keeps only its text, `Image` keeps only its alt, `PageBreak` drops to
 *  nothing. Reproducing the losses faithfully is what makes the fixture a fair carrier, not a
 *  flattering one. */
function docxBlocks(doc: XmlDocument, parent: XmlElement, blocks: readonly Block[]): void {
  for (const block of blocks) {
    switch (block.kind) {
      case "paragraph":
        parent.appendChild(docxParagraph(doc, block.style, block.runs));
        break;
      case "heading":
        parent.appendChild(docxParagraph(doc, block.style ?? `Heading${block.level}`, block.runs));
        break;
      case "list":
        for (const item of block.items) docxBlocks(doc, parent, item);
        break;
      case "quote":
        docxBlocks(doc, parent, block.blocks);
        break;
      case "code":
        parent.appendChild(docxParagraph(doc, undefined, [{ text: block.text }]));
        break;
      case "image":
        parent.appendChild(docxParagraph(doc, undefined, [{ text: block.alt }]));
        break;
      case "page-break":
        break;
    }
  }
}

function serializeXml(doc: XmlDocument): string {
  return `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>${new XMLSerializer().serializeToString(doc)}`;
}

function flatDocument(namespace: string, root: string, children: readonly (readonly [string, readonly (readonly [string, string])[]])[]): string {
  const implementation = new DOMImplementation();
  const doc = implementation.createDocument(namespace, root, null);
  for (const [name, attributes] of children) {
    const node = doc.createElementNS(namespace, name);
    for (const [key, value] of attributes) node.setAttribute(key, value);
    doc.documentElement!.appendChild(node);
  }
  return serializeXml(doc);
}

/** 📜️ A real OOXML package: five parts, written by `@xmldom/xmldom` and zipped by `jszip`. */
export async function docxBytes(source: Doc): Promise<Uint8Array> {
  const implementation = new DOMImplementation();

  const wordDocument = implementation.createDocument(W_NS, "w:document", null);
  const body = wordDocument.createElementNS(W_NS, "w:body");
  docxBlocks(wordDocument, body, source.blocks);
  wordDocument.documentElement!.appendChild(body);

  const stylesDocument = implementation.createDocument(W_NS, "w:styles", null);
  for (const entry of source.styles) {
    const node = stylesDocument.createElementNS(W_NS, "w:style");
    node.setAttribute("w:type", "paragraph");
    node.setAttribute("w:styleId", entry.id);
    const name = stylesDocument.createElementNS(W_NS, "w:name");
    name.setAttribute("w:val", entry.name);
    node.appendChild(name);
    if (entry.basedOn !== undefined) {
      const basedOn = stylesDocument.createElementNS(W_NS, "w:basedOn");
      basedOn.setAttribute("w:val", entry.basedOn);
      node.appendChild(basedOn);
    }
    stylesDocument.documentElement!.appendChild(node);
  }

  const contentTypes = flatDocument(CT_NS, "Types", [
    ["Default", [["Extension", "rels"], ["ContentType", "application/vnd.openxmlformats-package.relationships+xml"]]],
    ["Default", [["Extension", "xml"], ["ContentType", "application/xml"]]],
    ["Override", [["PartName", "/word/document.xml"], ["ContentType", "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"]]],
    ["Override", [["PartName", "/word/styles.xml"], ["ContentType", "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"]]],
  ]);
  const packageRels = flatDocument(REL_NS, "Relationships", [["Relationship", [["Id", "rId1"], ["Type", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"], ["Target", "word/document.xml"]]]]);
  const documentRels = flatDocument(REL_NS, "Relationships", [["Relationship", [["Id", "rId1"], ["Type", "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles"], ["Target", "styles.xml"]]]]);

  const zip = new JSZip();
  for (const [name, body_] of [
    ["[Content_Types].xml", contentTypes],
    ["_rels/.rels", packageRels],
    ["word/_rels/document.xml.rels", documentRels],
    ["word/document.xml", serializeXml(wordDocument)],
    ["word/styles.xml", serializeXml(stylesDocument)],
  ] as const) {
    zip.file(name, body_, { date: ZIP_EPOCH, createFolders: false });
  }
  return await zip.generateAsync({ type: "uint8array", compression: "DEFLATE", compressionOptions: { level: 9 }, platform: "UNIX" });
}
//#endregion 📜️Docx

//#region 📝️Markdown
type MdastNode = { type: string; [key: string]: unknown };

/** ✍️ One run as mdast inline nodes: bold wraps italic wraps link wraps the literal text — the stable
 *  nesting order this subset's own md serializer documents. `underline` has no CommonMark construct and
 *  is dropped rather than invented. */
function mdastInline(run: Run): MdastNode {
  let node: MdastNode = { type: "text", value: run.text };
  if (run.link !== undefined) node = { type: "link", url: run.link, children: [node] };
  if (run.italic === true) node = { type: "emphasis", children: [node] };
  if (run.bold === true) node = { type: "strong", children: [node] };
  return node;
}

/** 🧱 The block→CommonMark mapping this subset's own serializer documents: named styles are dropped
 *  entirely, an `Image` becomes its own paragraph whose URL is the image id, `PageBreak` drops. */
function mdastBlocks(blocks: readonly Block[]): MdastNode[] {
  const out: MdastNode[] = [];
  for (const block of blocks) {
    switch (block.kind) {
      case "paragraph":
        out.push({ type: "paragraph", children: block.runs.map(mdastInline) });
        break;
      case "heading":
        out.push({ type: "heading", depth: block.level, children: block.runs.map(mdastInline) });
        break;
      case "list":
        out.push({ type: "list", ordered: block.ordered, spread: false, children: block.items.map((item) => ({ type: "listItem", spread: false, children: mdastBlocks(item) })) });
        break;
      case "code":
        out.push({ type: "code", lang: block.language ?? null, value: block.text });
        break;
      case "quote":
        out.push({ type: "blockquote", children: mdastBlocks(block.blocks) });
        break;
      case "image":
        out.push({ type: "paragraph", children: [{ type: "image", url: block.imageId, alt: block.alt }] });
        break;
      case "page-break":
        break;
    }
  }
  return out;
}

/** 📝️ The committed bytes are `mdast-util-to-markdown`'s own serialization of the tree above. This
 *  repository writes no CommonMark syntax: what lands on disk is what the library emits. */
export function mdBytes(source: Doc): Uint8Array {
  return new TextEncoder().encode(toMarkdown({ type: "root", children: mdastBlocks(source.blocks) } as never));
}
//#endregion 📝️Markdown

//#region 🧪️Corpus
/** 🧪️ The corpus, assembled from one module per FAMILY — the sharding key CI uses and the unit somebody
 *  extends, reviews or runs in isolation, exactly as the mesh and BRep pilots organise theirs. */
const RECIPES: readonly Recipe[] = [
  ...(await import("./🧱️blocks/📜️script.ts")).RECIPES,
  ...(await import("./🖋️runs/📜️script.ts")).RECIPES,
  ...(await import("./🎨️styles/🟦️.ts")).RECIPES,
];
//#endregion 🧪️Corpus

//#region 🏭️Generate
async function contentDigest(bytes: Uint8Array): Promise<string> {
  const data = new Uint8Array(bytes.length);
  data.set(bytes);
  const hash = await crypto.subtle.digest("SHA-256", data);
  return `sha256:${[...new Uint8Array(hash)].map((byte) => byte.toString(16).padStart(2, "0")).join("")}`;
}

function write(path: string, body: Uint8Array): void {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, body);
}

/** 🔮️ Keeps oracle policy fields while refreshing generated coordinates and byte authority. */
function synchronizeOracleFixtureCoordinates(manifests: readonly Record<string, unknown>[]): void {
  const oraclePath = join(import.meta.dir, "..", "🔮️oracles", "🔣️.json");
  const oracle = JSON.parse(readFileSync(oraclePath, "utf8")) as Record<string, unknown>;
  const generatedById = new Map(manifests.map((manifest) => [String(manifest.id), manifest]));
  oracle.testEvidence = (oracle.testEvidence as Record<string, unknown>[]).map((manifest) => {
    const generated = generatedById.get(String(manifest.id));
    if (generated === undefined) return manifest;
    return { ...manifest, files: generated.files, generator: generated.generator };
  });
  write(oraclePath, new TextEncoder().encode(`${JSON.stringify(oracle, null, 2)}\n`));
}

const CARRIER: Record<"docx" | "md", { mediaType: string; oracle: string; engineFamily: string; engineVersion: string; packageVersion: string; encode: (doc: Doc) => Promise<Uint8Array> }> = {
  docx: { mediaType: "application/vnd.openxmlformats-officedocument.wordprocessingml.document", oracle: DOCX_ORACLE, engineFamily: "jszip", engineVersion: JSZIP_VERSION, packageVersion: `${JSZIP_VERSION} + @xmldom/xmldom ${XMLDOM_VERSION}`, encode: docxBytes },
  md: { mediaType: "text/markdown", oracle: MD_ORACLE, engineFamily: "unified-mdast", engineVersion: MDAST_TO_MARKDOWN_VERSION, packageVersion: MDAST_TO_MARKDOWN_VERSION, encode: async (doc) => mdBytes(doc) },
};

/** 🏭️ One recipe's bundle: the before, the after and — where the recipe ships one — the deliberately
 *  wrong after, encoded once per carrier that can witness the mutation. One fixture manifest is emitted
 *  PER CARRIER, because a fixture's authority is the single library that wrote its bytes. */
async function generateOne(recipe: Recipe, outDir: string): Promise<Record<string, unknown>[]> {
  const fixtureDirectory = FIXTURE_DIRECTORY_BY_ID[recipe.id];
  if (fixtureDirectory === undefined) throw new Error(`No reviewed fixture directory is registered for ${recipe.id}`);
  const dir = join(outDir, fixtureDirectory);
  const manifests: Record<string, unknown>[] = [];
  const outcome = JSON.stringify(recipe.before) === JSON.stringify(recipe.after) ? "no-op" : "applied";
  for (const carrier of recipe.carriers) {
    const spec = CARRIER[carrier];
    const files: { role: string; path: string; mediaType: string; sha256: string; bytes: number }[] = [];
    const variants: [string, Doc][] = [["before", recipe.before], ["after", recipe.after]];
    if (recipe.counterexample !== undefined) variants.push(["counterexample", recipe.counterexample]);
    for (const [variant, doc] of variants) {
      const bytes = await spec.encode(doc);
      const filename = FIXTURE_FILENAME_BY_ROLE[`${variant}-${carrier}`];
      if (filename === undefined) throw new Error(`No reviewed fixture filename is registered for ${variant}-${carrier}`);
      write(join(dir, filename), bytes);
      files.push({ role: `${variant}-${carrier}`, path: `${FIXTURE_PATH_PREFIX}${fixtureDirectory}/${filename}`, mediaType: spec.mediaType, sha256: await contentDigest(bytes), bytes: bytes.length });
    }
    manifests.push({
      id: `${recipe.id}-${carrier}`,
      class: "third-party-generated",
      target: { artifact: "s.stdio.semio", standard: "v1", subset: "document" },
      mutation: recipe.mutation,
      outcome,
      units: { length: "unitless", angle: "radian" },
      files,
      generator: {
        oracle: spec.oracle,
        packageVersion: spec.packageVersion,
        engineFamily: spec.engineFamily,
        engineVersion: spec.engineVersion,
        command: `bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📜️script.ts generate --only ${recipe.id}`,
        seed: SEED,
        platform: `${process.platform === "win32" ? "win32" : process.platform === "darwin" ? "darwin" : "linux"}-${process.arch === "arm64" ? "arm64" : "x64"}`,
      },
      provenance: {
        source: "generated",
        license: carrier === "docx" ? "MIT (jszip) + MIT (@xmldom/xmldom)" : "MIT (mdast-util-to-markdown)",
        attribution: carrier === "docx" ? "OPC container written by JSZip (MIT); WordprocessingML serialized by @xmldom/xmldom (MIT)" : "CommonMark serialized by mdast-util-to-markdown (MIT)",
        security: "scanned-clean",
        privacy: "no-personal-data",
      },
      comparisonProfile: "semantic-document-carrier-v1",
      toleranceProfile: "document-text-exact",
      reproducible: true,
      family: recipe.family,
      notes: `${recipe.notes} Witnessed through the ${recipe.property} property.`,
    });
  }
  return manifests;
}
//#endregion 🏭️Generate

//#region 🚪️Entry
async function main(argv: readonly string[]): Promise<number> {
  const [command = "generate", ...rest] = argv;
  const value = (flag: string): string | null => {
    const index = rest.indexOf(flag);
    return index === -1 ? null : (rest[index + 1] ?? null);
  };
  const only = value("--only");
  const recipes = only === null ? RECIPES : RECIPES.filter((recipe) => recipe.id === only);
  if (recipes.length === 0) {
    console.error(`[generator] no recipe matches ${JSON.stringify(only)} — known: ${RECIPES.map((recipe) => recipe.id).join(", ")}`);
    return 1;
  }
  const registeredIds = [...new Set([...RECIPES.map((recipe) => recipe.id), ...CARRIER_KINDS])];
  if (registeredIds.some((id) => FIXTURE_DIRECTORY_BY_ID[id] === undefined) || new Set(registeredIds.map((id) => FIXTURE_DIRECTORY_BY_ID[id])).size !== registeredIds.length || Object.keys(FIXTURE_DIRECTORY_BY_ID).length !== registeredIds.length) {
    console.error("[generator] reviewed fixture directory authority must contain exactly one unique entry per recipe");
    return 1;
  }
  const canonicalOutDir = join(import.meta.dir, "..", "🧪️tests/🧫️fixtures");
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? value("--out") ?? canonicalOutDir;

  if (command === "generate" || command === "manifests") {
    const manifests: Record<string, unknown>[] = [];
    let failed = 0;
    for (const recipe of recipes) {
      try {
        manifests.push(...(await generateOne(recipe, outDir)));
        console.error(`[generator] ${recipe.id} (${recipe.family}) → ${recipe.carriers.join(", ")}`);
      } catch (error) {
        failed += 1;
        console.error(`[generator] ${recipe.id} FAILED — ${(error as Error).message}`);
      }
    }
    if (command === "manifests") process.stdout.write(`${JSON.stringify(manifests, null, 2)}\n`);
    else {
      const indexPath = join(outDir, "🔣️.json");
      const previous = (() => {
        if (only === null || !existsSync(indexPath)) return [];
        try {
          return JSON.parse(readFileSync(indexPath, "utf8")) as Record<string, unknown>[];
        } catch {
          return [];
        }
      })();
      const produced = new Set(manifests.map((entry) => entry.id as string));
      const merged = [...previous.filter((entry) => !produced.has(entry.id as string)), ...manifests].sort((a, b) => String(a.id).localeCompare(String(b.id)));
      write(indexPath, new TextEncoder().encode(`${JSON.stringify(merged, null, 2)}\n`));
      if (outDir === canonicalOutDir) synchronizeOracleFixtureCoordinates(manifests);
      if (only !== null) console.error(`[generator] merged ${manifests.length} regenerated entr(ies) into ${merged.length} total`);
    }
    console.error(`[generator] ${manifests.length} fixture manifest(s) from ${recipes.length} recipe(s) into ${outDir}${failed > 0 ? `, ${failed} failed` : ""}`);
    return failed > 0 ? 1 : 0;
  }
  if (command === "carrier" || command === "carrier-manifests") {
    const engineDir = join(import.meta.dir, "🧩️json", "📦️packages", "🦀️rust");
    const build = spawnSync("cargo", ["build", "--release", "--offline", "--manifest-path", join(engineDir, "Cargo.toml")], { stdio: "inherit" });
    if (build.status !== 0) throw new Error(`cargo build failed with status ${build.status}`);
    const fixturesDir = join(import.meta.dir, "..", "🧪️tests/🧫️fixtures");
    if (command === "carrier") {
      const run = spawnSync(join(cargoTargetDirectory(findWorkspaceRoot()), "release", "generate"), [fixturesDir], { stdio: "inherit" });
      if (run.status !== 0) return run.status ?? 1;
    }
    const kinds = CARRIER_KINDS;
    const entries = [];
    for (const kind of kinds) {
      const fixtureDirectory = FIXTURE_DIRECTORY_BY_ID[kind];
      if (fixtureDirectory === undefined) throw new Error(`No reviewed fixture directory is registered for ${kind}`);
      const files = [];
      for (const [role, name] of [["expected-before-json", "⬅️before.json"], ["expected-after-json", "➡️after.json"]] as const) {
        const bytes = readFileSync(join(fixturesDir, fixtureDirectory, name));
        files.push({ role, path: `${FIXTURE_PATH_PREFIX}${fixtureDirectory}/${name}`, mediaType: "application/json", sha256: await contentDigest(bytes), bytes: bytes.length });
      }
      entries.push({
        id: `carrier-${kind}`,
        class: "third-party-generated",
        target: { artifact: "s.stdio.semio", standard: "v1", subset: "document" },
        mutation: kind,
        outcome: "applied",
        units: { length: "unitless", angle: "radian" },
        files,
        provenance: { source: "generated", license: "public-domain (synthetic, no third-party content embedded)" },
        generator: { oracle: "json-rust-semio-document-carrier-reader", packageVersion: "0.12", engineFamily: "json-rust", engineVersion: "0.12", command: "bun ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🏭️generator/📜️script.ts carrier", platform: currentPlatform() },
        comparisonProfile: "semantic-semio-document-carrier-v1",
        reproducible: true,
        family: "mechanical",
        notes: `A deterministic two-image document with the ${kind} mutation applied as an edit to the JSON CARRIER and read back through json-rust — never through this repository's own mutation engine. Neither the docx nor the markdown carrier preserves an image's raw bytes. Observability is checked before a pair is written, and a pair that does not move is refused rather than committed.`,
      });
    }
    if (command === "carrier-manifests") process.stdout.write(`${JSON.stringify(entries, null, 2)}\n`);
    else {
      const indexPath = join(fixturesDir, "🔣️.json");
      const previous = JSON.parse(readFileSync(indexPath, "utf8")) as Record<string, unknown>[];
      const produced = new Set(entries.map((entry) => entry.id));
      const merged = [...previous.filter((entry) => !produced.has(String(entry.id))), ...entries].sort((a, b) => String(a.id).localeCompare(String(b.id)));
      write(indexPath, new TextEncoder().encode(`${JSON.stringify(merged, null, 2)}\n`));
      synchronizeOracleFixtureCoordinates(entries);
    }
    return 0;
  }
  console.error(`[generator] unknown command ${JSON.stringify(command)} — expected generate | manifests | carrier | carrier-manifests`);
  return 1;
}

// 🚰️ `exitCode`, never `process.exit()`: an explicit exit cuts a still-draining piped stdout at 64 KiB.
if (import.meta.main) process.exitCode = await main(process.argv.slice(2));
//#endregion 🚪️Entry
