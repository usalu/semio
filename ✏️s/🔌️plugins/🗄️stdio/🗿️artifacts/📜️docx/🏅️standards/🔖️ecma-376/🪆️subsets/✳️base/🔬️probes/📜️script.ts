#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External measurement probes for `s.stdio.docx@ecma-376/✳️any`.
//
// Everything here MARSHALS and READS; nothing here applies a mutation or predicts what one should
// produce. `jszip` opens the OPC container, `fast-xml-parser` parses every XML part inside it
// (`[Content_Types].xml`, `word/document.xml`, `word/styles.xml`) — both are already vendored in this
// repo's own `node_modules` (`jszip` 3.10.1, MIT/GPL-3.0-or-later; `fast-xml-parser` 5.11.1, MIT —
// versions read directly off their own `package.json`, not guessed). The projection below implements
// the SAME typed view `../🧪️oracle/🔣️.json`'s `semantic-docx-ecma-376-mutate-v1` comparisonProfile
// documents: `body` (the ordered `w:body` block tree — paragraphs with style ref + ordered runs,
// tables with ordered rows/cells, recursively) and `styles` (the ordered `w:styles` list, id/name/
// basedOn), both order-sensitive; every OTHER real OPC part compared by content-type + digest as an
// unordered path-keyed map; `[Content_Types].xml` and every `*.rels` part excluded entirely as
// regenerated OPC plumbing. This file performs no mutation semantics of its own.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts docx-import  --input <a.docx>
//   bun 📜️script.ts docx-project --input <a.docx>
//   bun 📜️script.ts docx-compare --input <expected.docx> --input <actual.docx>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../../../../💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🔬️probes/📜️script.ts — the sibling
//      probe suite this file's CLI/dispatch shape is mirrored from

//#endregion 🧲️Header

//#region 🔌️Adapters
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { extname } from "node:path";
import JSZip from "jszip";
import { XMLParser } from "fast-xml-parser";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never computes them. */
type ProbeReport = {
  schema: "semio.repository-test.probe-report/v2";
  probe: string;
  probeVersion: string;
  engine: { family: string; implementation: string; version: string };
  status: "ok" | "failed" | "unsupported";
  durationMs: number;
  measurements: Record<string, unknown>;
  diagnostics?: { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
};

const ENGINE = { family: "zip-xml", implementation: "jszip + fast-xml-parser", version: "jszip@3.10.1 + fast-xml-parser@5.11.1" } as const;
const PROBE_VERSION = "jszip@3.10.1 + fast-xml-parser@5.11.1";
//#endregion 🧬️Contract

//#region 📥️Model
type ProjectedRun = { text: string; bold: boolean; italic: boolean; underline: boolean };
type ProjectedParagraph = { kind: "paragraph"; style: string | null; runs: ProjectedRun[] };
type ProjectedTableCell = { blocks: ProjectedBlock[] };
type ProjectedTableRow = { cells: ProjectedTableCell[] };
type ProjectedTable = { kind: "table"; rows: ProjectedTableRow[] };
type ProjectedBlock = ProjectedParagraph | ProjectedTable;
type ProjectedStyle = { id: string; name: string; basedOn: string | null };
type OtherPart = { contentType: string; digest: string; size: number };
type DocxProjection = { body: ProjectedBlock[]; styles: ProjectedStyle[]; otherParts: Record<string, OtherPart> };
//#endregion 📥️Model

//#region 🔓️Parse
// 🌳 `preserveOrder` parsing — the mirror of the generator's `preserveOrder` building. Any other mode
// folds same-tag siblings (`w:p`, `w:p`, …) into one array and loses order relative to a DIFFERENT
// sibling tag (`w:tbl`) at the same nesting level; `w:body` legitimately mixes the two.
// 📌️ `trimValues: false` — `w:t` carries `xml:space="preserve"`-significant text ("Nakagin " with its
// trailing space is a real run boundary), and `format: false` on the generator's builder means no
// incidental pretty-printing whitespace exists anywhere else to strip.
const XML = new XMLParser({ ignoreAttributes: false, attributeNamePrefix: "@_", preserveOrder: true, trimValues: false, ignoreDeclaration: true, textNodeName: "#text" });

type PNode = Record<string, unknown> & { ":@"?: Record<string, string> };

function tagOf(node: PNode): string {
  const found = Object.keys(node).find((key) => key !== ":@");
  if (found === undefined) throw new Error("preserveOrder node carries no tag");
  return found;
}
function kids(node: PNode): PNode[] {
  return (node[tagOf(node)] as PNode[] | undefined) ?? [];
}
function attr(node: PNode, name: string): string | undefined {
  return node[":@"]?.[`@_${name}`];
}
function findChild(node: PNode, tag: string): PNode | undefined {
  return kids(node).find((child) => !("#text" in child) && tagOf(child) === tag);
}
function findChildren(node: PNode, tag: string): PNode[] {
  return kids(node).filter((child) => !("#text" in child) && tagOf(child) === tag);
}
function textOf(node: PNode): string {
  return kids(node)
    .filter((child): child is PNode & { "#text": string } => "#text" in child)
    .map((child) => child["#text"])
    .join("");
}

function digestOf(bytes: Buffer): string {
  return `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
}

function walkRun(node: PNode): ProjectedRun {
  const rPr = findChild(node, "w:rPr");
  const bold = rPr !== undefined && findChild(rPr, "w:b") !== undefined;
  const italic = rPr !== undefined && findChild(rPr, "w:i") !== undefined;
  const underline = rPr !== undefined && findChild(rPr, "w:u") !== undefined;
  const t = findChild(node, "w:t");
  return { text: t !== undefined ? textOf(t) : "", bold, italic, underline };
}

function walkParagraph(node: PNode): ProjectedParagraph {
  const pPr = findChild(node, "w:pPr");
  const pStyle = pPr !== undefined ? findChild(pPr, "w:pStyle") : undefined;
  const style = pStyle !== undefined ? (attr(pStyle, "w:val") ?? null) : null;
  return { kind: "paragraph", style, runs: findChildren(node, "w:r").map(walkRun) };
}

function walkCell(node: PNode): ProjectedTableCell {
  return { blocks: kids(node).filter((child) => !("#text" in child) && (tagOf(child) === "w:p" || tagOf(child) === "w:tbl")).map(walkBlock) };
}
function walkRow(node: PNode): ProjectedTableRow {
  return { cells: findChildren(node, "w:tc").map(walkCell) };
}
function walkTable(node: PNode): ProjectedTable {
  return { kind: "table", rows: findChildren(node, "w:tr").map(walkRow) };
}
function walkBlock(node: PNode): ProjectedBlock {
  return tagOf(node) === "w:tbl" ? walkTable(node) : walkParagraph(node);
}

function walkStyle(node: PNode): ProjectedStyle {
  const name = findChild(node, "w:name");
  const basedOn = findChild(node, "w:basedOn");
  return { id: attr(node, "w:styleId") ?? "", name: name !== undefined ? (attr(name, "w:val") ?? "") : "", basedOn: basedOn !== undefined ? (attr(basedOn, "w:val") ?? null) : null };
}

type ContentTypes = { defaults: Map<string, string>; overrides: Map<string, string> };

function parseContentTypes(xml: string): ContentTypes {
  const root = XML.parse(xml)[0] as PNode;
  const defaults = new Map<string, string>();
  const overrides = new Map<string, string>();
  for (const node of findChildren(root, "Default")) {
    const extension = attr(node, "Extension");
    const contentType = attr(node, "ContentType");
    if (extension !== undefined && contentType !== undefined) defaults.set(extension.toLowerCase(), contentType);
  }
  for (const node of findChildren(root, "Override")) {
    const partName = attr(node, "PartName");
    const contentType = attr(node, "ContentType");
    if (partName !== undefined && contentType !== undefined) overrides.set(partName, contentType);
  }
  return { defaults, overrides };
}

function resolveContentType(contentTypes: ContentTypes, path: string): string {
  const override = contentTypes.overrides.get(`/${path}`);
  if (override !== undefined) return override;
  const ext = extname(path).replace(/^\./, "").toLowerCase();
  return contentTypes.defaults.get(ext) ?? "application/octet-stream";
}

/** 📎️ Every real OPC part beyond `word/document.xml`/`word/styles.xml` — `[Content_Types].xml` and
 *  every `*.rels` part are excluded entirely, exactly as `semantic-docx-ecma-376-mutate-v1` documents:
 *  both sides regenerate them deterministically from the typed content-types/relationships tables, and
 *  no mutation in the 13-kind vocabulary ever targets them directly. */
function isExcludedFromOtherParts(path: string): boolean {
  return path === "[Content_Types].xml" || path.endsWith(".rels");
}

async function readDocx(path: string): Promise<DocxProjection> {
  const bytes = readFileSync(path);
  const zip = await JSZip.loadAsync(bytes);
  const entries = Object.values(zip.files).filter((entry) => !entry.dir);
  const byName = new Map(entries.map((entry) => [entry.name, entry]));

  const contentTypesEntry = byName.get("[Content_Types].xml");
  const contentTypes = contentTypesEntry !== undefined ? parseContentTypes(await contentTypesEntry.async("string")) : { defaults: new Map(), overrides: new Map() };

  let body: ProjectedBlock[] = [];
  const documentEntry = byName.get("word/document.xml");
  if (documentEntry !== undefined) {
    const root = XML.parse(await documentEntry.async("string"))[0] as PNode;
    const bodyNode = findChild(root, "w:body");
    if (bodyNode !== undefined) body = kids(bodyNode).filter((child) => !("#text" in child) && (tagOf(child) === "w:p" || tagOf(child) === "w:tbl")).map(walkBlock);
  }

  let styles: ProjectedStyle[] = [];
  const stylesEntry = byName.get("word/styles.xml");
  if (stylesEntry !== undefined) {
    const root = XML.parse(await stylesEntry.async("string"))[0] as PNode;
    styles = findChildren(root, "w:style").map(walkStyle);
  }

  const otherParts: Record<string, OtherPart> = {};
  for (const entry of entries) {
    if (entry.name === "word/document.xml" || entry.name === "word/styles.xml" || isExcludedFromOtherParts(entry.name)) continue;
    const bytes2 = await entry.async("nodebuffer");
    otherParts[entry.name] = { contentType: resolveContentType(contentTypes, entry.name), digest: digestOf(bytes2), size: bytes2.length };
  }

  return { body, styles, otherParts };
}
//#endregion 🔓️Parse

//#region ⚖️Compare
/** ⚖️ `body`/`styles` are order-sensitive per `semantic-docx-ecma-376-mutate-v1` — plain positional
 *  deep-equal, never set comparison. `otherParts` is an unordered path-keyed map by construction (a
 *  plain object), so iterating its keys is already an unordered comparison. */
function diffAt(path: string, expected: unknown, actual: unknown, diffs: string[]): void {
  if (Array.isArray(expected) || Array.isArray(actual)) {
    const e = Array.isArray(expected) ? expected : [];
    const a = Array.isArray(actual) ? actual : [];
    if (e.length !== a.length) diffs.push(`${path}: length ${e.length} ≠ ${a.length}`);
    for (let index = 0; index < Math.max(e.length, a.length); index += 1) diffAt(`${path}[${index}]`, e[index], a[index], diffs);
    return;
  }
  if (typeof expected === "object" && expected !== null && typeof actual === "object" && actual !== null) {
    const keys = new Set([...Object.keys(expected as object), ...Object.keys(actual as object)]);
    for (const key of keys) diffAt(`${path}.${key}`, (expected as Record<string, unknown>)[key], (actual as Record<string, unknown>)[key], diffs);
    return;
  }
  if (expected !== actual) diffs.push(`${path}: ${JSON.stringify(expected)} ≠ ${JSON.stringify(actual)}`);
}

function compareDocs(expected: DocxProjection, actual: DocxProjection): { equal: boolean; diffCount: number; diffs: string[] } {
  const diffs: string[] = [];
  diffAt("$.body", expected.body, actual.body, diffs);
  diffAt("$.styles", expected.styles, actual.styles, diffs);
  diffAt("$.otherParts", expected.otherParts, actual.otherParts, diffs);
  return { equal: diffs.length === 0, diffCount: diffs.length, diffs: diffs.slice(0, 50) };
}
//#endregion ⚖️Compare

//#region 🔬️Probes
function requireInputs(inputs: readonly string[], count: number, probe: string): void {
  if (inputs.length < count) throw new Error(`${probe} requires ${count} --input path(s), got ${inputs.length}`);
}

type ProbeResult = { status: "ok" | "failed" | "unsupported"; measurements: Record<string, unknown>; diagnostics?: ProbeReport["diagnostics"] };

const PROBES: Record<string, (inputs: readonly string[]) => Promise<ProbeResult>> = {
  "docx-import": async (inputs) => {
    requireInputs(inputs, 1, "docx-import");
    const results = await Promise.allSettled(inputs.map((input) => readDocx(input)));
    const perInput = results.map((result, index) => ({ path: inputs[index], ok: result.status === "fulfilled", error: result.status === "rejected" ? String((result.reason as Error).message ?? result.reason) : undefined }));
    return { status: "ok", measurements: { bothImport: perInput.every((entry) => entry.ok), perInput } };
  },
  "docx-project": async (inputs) => {
    requireInputs(inputs, 1, "docx-project");
    const doc = await readDocx(inputs[0]!);
    return { status: "ok", measurements: { blockCount: doc.body.length, styleCount: doc.styles.length, otherPartCount: Object.keys(doc.otherParts).length, projection: doc } };
  },
  "docx-compare": async (inputs) => {
    requireInputs(inputs, 2, "docx-compare");
    const expected = await readDocx(inputs[0]!);
    const actual = await readDocx(inputs[1]!);
    const verdict = compareDocs(expected, actual);
    return { status: "ok", measurements: { ...verdict, expected, actual } };
  },
};
//#endregion 🔬️Probes

//#region 🚀️Entry
function parseArgv(argv: readonly string[]): { probe: string; inputs: string[] } {
  const [probe = "", ...rest] = argv;
  const inputs: string[] = [];
  for (let i = 0; i < rest.length; i += 1) if (rest[i] === "--input") inputs.push(rest[i + 1] ?? "");
  return { probe, inputs };
}

async function main(argv: readonly string[]): Promise<number> {
  const { probe, inputs } = parseArgv(argv);
  const started = Date.now();
  const emit = (report: ProbeReport): number => {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return report.status === "failed" ? 1 : 0;
  };
  const budgetMs = Number(process.env.SEMIO_PROBE_TIMEOUT_MS ?? 60_000);
  const watchdog = new Promise<never>((_, reject) => setTimeout(() => reject(new Error(`probe exceeded ${budgetMs} ms`)), budgetMs).unref?.());
  const run = PROBES[probe];
  if (!run) return emit({ schema: "semio.repository-test.probe-report/v2", probe: probe || "(none)", probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${probe}`, detail: `known: ${Object.keys(PROBES).join(", ")}` }] });
  try {
    const result = await Promise.race([run(inputs), watchdog]);
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics ? { diagnostics: result.diagnostics } : {}) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
