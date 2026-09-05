#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🔬️ External reading probes for `s.stdio.semio@v1/📑️document`.
//
// Everything here MARSHALS and INVOKES; nothing here computes a document and nothing here PREDICTS what
// a mutation ought to produce. Every value comes out of a third-party library reading committed bytes:
// `fflate` unzips the OPC container and `fast-xml-parser` decodes the WordprocessingML, `markdown-it`
// parses the CommonMark. The pipeline compares the emitted `measurements`; this file states facts about
// files and nothing else.
//
// The readers are deliberately DIFFERENT LIBRARIES from the writers in `../🏗️generator/📜️script.ts`
// (`jszip` + `@xmldom/xmldom` write docx, `mdast-util-to-markdown` writes md). A reading produced by the
// library that did the writing would confirm its own serializer instead of checking the bytes.
//
// The carrier decides what is checkable, which is why `document-property` can answer `unsupported`.
// docx flattens lists, so it cannot witness `list-ordered`; CommonMark drops named styles, so it cannot
// witness `paragraph-style` or `style-table`; and NEITHER serializes the id-keyed embedded image store,
// so `image-store` is unsupported everywhere. An empty reading returned as `ok` would let such a
// mutation pass against evidence that was never there.
//
// Usage — one probe per invocation, one typed report on stdout:
//   bun 📜️script.ts document-read      --input <a.docx|a.md>
//   bun 📜️script.ts document-property  --property <p> --input <before.ext> --input <after.ext>
//   bun 📜️script.ts document-compare   --input <before.ext> --input <after.ext>
//   bun 📜️script.ts carrier-agreement  --input <a.docx> --input <a.md>
//
// @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️.json — ProbeReport
// @see ../../🔺️mesh/🔬️probes/📜️script.ts — the pilot this file mirrors in report contract and CLI shape
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️document-subset-oracle.md

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { extname } from "node:path";
import { strFromU8, unzipSync } from "fflate";
import { XMLParser } from "fast-xml-parser";
import MarkdownIt from "markdown-it";
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

/** ⚙️ Two carriers, two engine families, and neither is the family that WROTE the bytes it reads. */
const DOCX_ENGINE = { family: "fast-xml-parser", implementation: "fflate unzip + fast-xml-parser OOXML reader", version: "5.11.1" } as const;
const MD_ENGINE = { family: "markdown-it", implementation: "markdown-it CommonMark parser", version: "14.3.0" } as const;
const PROBE_VERSION = "fflate@0.8.3 + fast-xml-parser@5.11.1 + markdown-it@14.3.0";

/** 🔎️ Every document property a mutation of this subset can write. `document-read` reports which of
 *  them the given carrier ENCODES and which it does not; `document-property` refuses the latter. */
const PROPERTIES = ["block-text", "block-count", "paragraph-style", "heading-level", "list-ordered", "run-emphasis", "image-alt", "style-table", "image-store"] as const;
type Property = (typeof PROPERTIES)[number];

type Run = { text: string; bold: boolean; italic: boolean; underline: boolean; link: string | null };
type DocxParagraph = { style: string | null; runs: Run[] };
type DocxStyle = { id: string; name: string; basedOn: string | null };
type MdBlock = { kind: string; level: number; ordered: boolean; info: string; runs: Run[]; url: string; items: string[] };
type Carrier = { id: "docx"; paragraphs: DocxParagraph[]; styles: DocxStyle[] } | { id: "md"; blocks: MdBlock[] };

function emptyRun(): Run {
  return { text: "", bold: false, italic: false, underline: false, link: null };
}
//#endregion 🧬️Contract

//#region 📜️Docx
type XmlNode = Record<string, unknown>;

const XML = new XMLParser({ ignoreAttributes: false, attributeNamePrefix: "@_", preserveOrder: true, trimValues: false, parseTagValue: false });

/** 🏷️ The single element name a `preserveOrder` node carries, or `null` for a text node. */
function tagOf(node: XmlNode): string | null {
  const keys = Object.keys(node).filter((key) => key !== ":@");
  const tag = keys[0];
  return tag === undefined || tag === "#text" ? null : tag;
}

function childrenOf(node: XmlNode): XmlNode[] {
  const tag = tagOf(node);
  return tag === null ? [] : ((node[tag] as XmlNode[] | undefined) ?? []);
}

function attributeOf(node: XmlNode, name: string): string | null {
  const attributes = node[":@"] as Record<string, string> | undefined;
  const value = attributes?.[`@_${name}`];
  return value === undefined ? null : String(value);
}

function textOf(nodes: readonly XmlNode[]): string {
  return nodes.map((node) => (tagOf(node) === null ? String((node as { "#text"?: unknown })["#text"] ?? "") : textOf(childrenOf(node)))).join("");
}

function findAll(nodes: readonly XmlNode[], tag: string, out: XmlNode[] = []): XmlNode[] {
  for (const node of nodes) {
    if (tagOf(node) === tag) out.push(node);
    findAll(childrenOf(node), tag, out);
  }
  return out;
}

function firstChild(node: XmlNode, tag: string): XmlNode | null {
  return childrenOf(node).find((child) => tagOf(child) === tag) ?? null;
}

/** 📜️ One `w:p` as `fast-xml-parser` recovers it: the `w:pStyle` reference and one entry per `w:r`
 *  carrying the three character properties WordprocessingML actually encodes here. */
function docxParagraph(node: XmlNode): DocxParagraph {
  const properties = firstChild(node, "w:pPr");
  const styleNode = properties === null ? null : firstChild(properties, "w:pStyle");
  const runs: Run[] = [];
  for (const runNode of childrenOf(node).filter((child) => tagOf(child) === "w:r")) {
    const runProperties = firstChild(runNode, "w:rPr");
    runs.push({
      text: textOf(childrenOf(runNode).filter((child) => tagOf(child) === "w:t").flatMap(childrenOf)),
      bold: runProperties !== null && firstChild(runProperties, "w:b") !== null,
      italic: runProperties !== null && firstChild(runProperties, "w:i") !== null,
      underline: runProperties !== null && firstChild(runProperties, "w:u") !== null,
      link: null,
    });
  }
  return { style: styleNode === null ? null : attributeOf(styleNode, "w:val"), runs };
}

function readDocx(bytes: Uint8Array): Carrier {
  const entries = unzipSync(bytes);
  const part = (name: string): XmlNode[] => {
    const raw = entries[name];
    if (raw === undefined) throw new Error(`the OPC container has no ${name}`);
    return XML.parse(strFromU8(raw)) as XmlNode[];
  };
  const paragraphs = findAll(part("word/document.xml"), "w:p").map(docxParagraph);
  const styles = findAll(part("word/styles.xml"), "w:style").map((node) => {
    const name = firstChild(node, "w:name");
    const basedOn = firstChild(node, "w:basedOn");
    return { id: attributeOf(node, "w:styleId") ?? "", name: name === null ? "" : (attributeOf(name, "w:val") ?? ""), basedOn: basedOn === null ? null : attributeOf(basedOn, "w:val") };
  });
  return { id: "docx", paragraphs, styles };
}
//#endregion 📜️Docx

//#region 📝️Markdown
type MdToken = { type: string; tag: string; info: string; content: string; markup: string; attrs: [string, string][] | null; children: MdToken[] | null };

/** ✍️ Flattens one `inline` token into runs, tracking the emphasis and link nesting markdown-it opened
 *  and closed around each text span. Empty spans are dropped — markdown-it emits them at wrap
 *  boundaries and they carry no content of their own. */
function inlineRuns(token: MdToken): Run[] {
  const runs: Run[] = [];
  let bold = 0;
  let italic = 0;
  const links: string[] = [];
  for (const child of token.children ?? []) {
    switch (child.type) {
      case "strong_open":
        bold += 1;
        break;
      case "strong_close":
        bold -= 1;
        break;
      case "em_open":
        italic += 1;
        break;
      case "em_close":
        italic -= 1;
        break;
      case "link_open":
        links.push(child.attrs?.find(([key]) => key === "href")?.[1] ?? "");
        break;
      case "link_close":
        links.pop();
        break;
      case "image":
        runs.push({ text: child.content, bold: bold > 0, italic: italic > 0, underline: false, link: child.attrs?.find(([key]) => key === "src")?.[1] ?? "" });
        break;
      case "text":
      case "code_inline":
        if (child.content.length > 0) runs.push({ text: child.content, bold: bold > 0, italic: italic > 0, underline: false, link: links.at(-1) ?? null });
        break;
      default:
        break;
    }
  }
  return runs;
}

function emptyMdBlock(kind: string): MdBlock {
  return { kind, level: 0, ordered: false, info: "", runs: [], url: "", items: [] };
}

/** 📝️ The committed CommonMark as `markdown-it` — a different implementation from the one that wrote
 *  it — recovers it: top-level blocks only, with list items and blockquote bodies folded into their
 *  own entry, which is the same granularity the docx side reports. */
function readMd(bytes: Uint8Array): Carrier {
  const tokens = new MarkdownIt("commonmark").parse(new TextDecoder().decode(bytes), {}) as unknown as MdToken[];
  const blocks: MdBlock[] = [];
  let depth = 0;
  let pending: MdBlock | null = null;
  const nested: string[] = [];
  for (const token of tokens) {
    if (token.type === "blockquote_open" || token.type === "bullet_list_open" || token.type === "ordered_list_open") {
      if (depth === 0) {
        pending = emptyMdBlock(token.type === "blockquote_open" ? "quote" : "list");
        pending.ordered = token.type === "ordered_list_open";
        nested.length = 0;
      }
      depth += 1;
      continue;
    }
    if (token.type === "blockquote_close" || token.type === "bullet_list_close" || token.type === "ordered_list_close") {
      depth -= 1;
      if (depth === 0 && pending !== null) {
        if (pending.kind === "list") pending.items = [...nested];
        else pending.runs = [{ ...emptyRun(), text: nested.join("\n") }];
        blocks.push(pending);
        pending = null;
      }
      continue;
    }
    if (token.type === "inline") {
      const runs = inlineRuns(token);
      if (depth > 0) {
        nested.push(runs.map((run) => run.text).join(""));
        continue;
      }
      const previous = blocks.at(-1);
      if (previous !== undefined && previous.runs.length === 0 && previous.kind !== "code") {
        previous.runs = runs;
        if (runs.length === 1 && (token.children ?? []).some((child) => child.type === "image")) {
          previous.kind = "image";
          previous.url = runs[0]!.link ?? "";
        }
      }
      continue;
    }
    if (depth > 0) continue;
    if (token.type === "heading_open") {
      const block = emptyMdBlock("heading");
      block.level = Number(token.tag.slice(1));
      blocks.push(block);
      continue;
    }
    if (token.type === "paragraph_open") {
      blocks.push(emptyMdBlock("paragraph"));
      continue;
    }
    if (token.type === "fence" || token.type === "code_block") {
      const block = emptyMdBlock("code");
      block.info = token.info.trim();
      block.runs = [{ ...emptyRun(), text: token.content.replace(/\n$/, "") }];
      blocks.push(block);
    }
  }
  return { id: "md", blocks };
}

function mdText(block: MdBlock): string {
  return block.kind === "list" ? block.items.join("\n") : block.runs.map((run) => run.text).join("");
}
//#endregion 📝️Markdown

//#region 📥️Carrier
function readCarrier(path: string): Carrier {
  const extension = extname(path).toLowerCase();
  const bytes = new Uint8Array(readFileSync(path));
  if (extension === ".docx") return readDocx(bytes);
  if (extension === ".md") return readMd(bytes);
  // 🚫️`.pdf` is deliberately absent: `pdfjs-dist` could read one, but no vendored library WRITES one,
  // so no third-party-generated pdf fixture exists for it to read. Registering a reader with no
  // library-authored artifact behind it would be a coverage claim with nothing under it.
  throw new Error(`unsupported carrier extension ${extension} — this subset's registered carriers are .docx and .md`);
}

function engineOf(carrier: Carrier): ProbeReport["engine"] {
  return carrier.id === "docx" ? DOCX_ENGINE : MD_ENGINE;
}

function emphasisEntry(run: Run): string {
  return `${run.text}|b=${run.bold}|i=${run.italic}|u=${run.underline}|link=${run.link ?? "-"}`;
}

/**
 * 🔎️ One property as the carrier's OWN reader recovers it, or `null` when the carrier does not encode
 * it at all. `null` is the load-bearing answer, not a fallback: docx flattens lists so `ordered` is
 * simply not in the bytes, CommonMark drops the named-style table, and neither export path ever writes
 * the id-keyed embedded image store — so `image-store` is `null` for every carrier and the three
 * image-store mutations stay honestly un-oracled.
 */
function propertyOf(carrier: Carrier, property: Property): string[] | null {
  if (property === "image-store") return null;
  if (carrier.id === "docx") {
    const text = (): string[] => carrier.paragraphs.map((paragraph) => paragraph.runs.map((run) => run.text).join(""));
    switch (property) {
      case "block-text":
      case "image-alt":
        return text();
      case "block-count":
        return [String(carrier.paragraphs.length)];
      case "paragraph-style":
        return carrier.paragraphs.map((paragraph) => paragraph.style ?? "-");
      case "heading-level":
        return carrier.paragraphs.map((paragraph) => {
          const level = paragraph.style?.startsWith("Heading") === true ? Number(paragraph.style.slice("Heading".length)) : Number.NaN;
          return Number.isInteger(level) ? String(level) : "-";
        });
      case "run-emphasis":
        return carrier.paragraphs.flatMap((paragraph) => paragraph.runs.map(emphasisEntry));
      case "style-table":
        return carrier.styles.map((entry) => `${entry.id}|${entry.name}|${entry.basedOn ?? "-"}`);
      default:
        return null;
    }
  }
  switch (property) {
    case "block-text":
      return carrier.blocks.map(mdText);
    case "block-count":
      return [String(carrier.blocks.length)];
    case "heading-level":
      return carrier.blocks.map((block) => (block.kind === "heading" ? String(block.level) : "-"));
    case "list-ordered":
      return carrier.blocks.filter((block) => block.kind === "list").map((block) => String(block.ordered));
    case "run-emphasis":
      return carrier.blocks.flatMap((block) => block.runs.map(emphasisEntry));
    case "image-alt":
      return carrier.blocks.filter((block) => block.kind === "image").map((block) => `${mdText(block)}|${block.url}`);
    default:
      // 🚫️`paragraph-style` and `style-table`: the md serializer's own header states that
      // `styles`/`style_id` are dropped entirely, so there is nothing in these bytes to read.
      return null;
  }
}

function differingIndices(left: readonly string[], right: readonly string[]): number[] {
  const out: number[] = [];
  for (let index = 0; index < Math.max(left.length, right.length); index += 1) if (left[index] !== right[index]) out.push(index);
  return out;
}
//#endregion 📥️Carrier

//#region 🔬️Probes
type Probe = (inputs: string[], options: Record<string, string>) => Pick<ProbeReport, "status" | "measurements"> & { diagnostics?: ProbeReport["diagnostics"]; engine?: ProbeReport["engine"] };

function requireInputs(inputs: readonly string[], n: number, probe: string): void {
  if (inputs.length < n) throw new Error(`${probe} needs ${n} --input path(s), got ${inputs.length}`);
}

const PROBES: Record<string, Probe> = {
  "document-read": (inputs) => {
    requireInputs(inputs, 1, "document-read");
    const carrier = readCarrier(inputs[0]!);
    const measurements: Record<string, unknown> = { carrier: carrier.id };
    const supported: string[] = [];
    const unsupported: string[] = [];
    for (const property of PROPERTIES) {
      const value = propertyOf(carrier, property);
      if (value === null) unsupported.push(property);
      else {
        supported.push(property);
        measurements[property] = value;
      }
    }
    measurements.supportedProperties = supported;
    measurements.unsupportedProperties = unsupported;
    return { status: "ok", engine: engineOf(carrier), measurements };
  },
  "document-property": (inputs, options) => {
    requireInputs(inputs, 2, "document-property");
    const property = (options.property ?? "block-text") as Property;
    if (!PROPERTIES.includes(property)) throw new Error(`unknown property ${property} — known: ${PROPERTIES.join(", ")}`);
    const before = readCarrier(inputs[0]!);
    const after = readCarrier(inputs[1]!);
    if (before.id !== after.id) throw new Error(`document-property compares ONE carrier, got ${before.id} and ${after.id}`);
    const left = propertyOf(before, property);
    const right = propertyOf(after, property);
    if (left === null || right === null) {
      // ✘️A carrier that never encoded the property is `unsupported`, never an empty `ok`: an empty
      // reading reported as ok would let the mutation pass against evidence that was never there.
      return { status: "unsupported", engine: engineOf(before), measurements: { carrier: before.id, property, reason: `${before.id} does not encode ${property}` } };
    }
    const differing = differingIndices(left, right);
    return {
      status: "ok",
      engine: engineOf(before),
      measurements: { carrier: before.id, property, before: left, after: right, differingIndices: differing, differingEntries: differing.length, equal: differing.length === 0, beforeEntries: left.length, afterEntries: right.length },
    };
  },
  "document-compare": (inputs) => {
    requireInputs(inputs, 2, "document-compare");
    const before = readCarrier(inputs[0]!);
    const after = readCarrier(inputs[1]!);
    if (before.id !== after.id) throw new Error(`document-compare compares ONE carrier, got ${before.id} and ${after.id}`);
    const properties: Record<string, unknown> = {};
    const unsupported: string[] = [];
    let total = 0;
    for (const property of PROPERTIES) {
      const left = propertyOf(before, property);
      const right = propertyOf(after, property);
      if (left === null || right === null) {
        unsupported.push(property);
        continue;
      }
      const differing = differingIndices(left, right);
      total += differing.length;
      properties[property] = { before: left, after: right, differingIndices: differing, differingEntries: differing.length };
    }
    return { status: "ok", engine: engineOf(before), measurements: { carrier: before.id, properties, unsupportedProperties: unsupported, totalDifferingEntries: total } };
  },
  "carrier-agreement": (inputs) => {
    requireInputs(inputs, 2, "carrier-agreement");
    // 🤝️The cross-family invariant. `fast-xml-parser` and `markdown-it` share no ancestry, so their
    // agreement on a flat document's block text is a real check rather than one library nodding at
    // itself. Documents with lists or quotes legitimately disagree — docx flattens them — so this
    // probe reports the disagreement instead of hiding it.
    const readings = inputs.slice(0, 2).map((input) => {
      const carrier = readCarrier(input);
      const text = propertyOf(carrier, "block-text");
      if (text === null) throw new Error(`${carrier.id} does not encode block text`);
      return { carrier: carrier.id, text };
    });
    const differing = differingIndices(readings[0]!.text, readings[1]!.text);
    return {
      status: "ok",
      measurements: {
        carriers: readings.map((reading) => reading.carrier),
        engineFamilies: [DOCX_ENGINE.family, MD_ENGINE.family],
        text: Object.fromEntries(readings.map((reading) => [reading.carrier, reading.text])),
        differingIndices: differing,
        totalDisagreements: differing.length,
        allEqual: differing.length === 0,
      },
    };
  },
};
//#endregion 🔬️Probes

//#region 🚀️Entry
function parseArgv(argv: readonly string[]): { probe: string; inputs: string[]; options: Record<string, string> } {
  const [probe = "", ...rest] = argv;
  const inputs: string[] = [];
  const options: Record<string, string> = {};
  for (let i = 0; i < rest.length; i += 1) {
    if (rest[i] === "--input") inputs.push(rest[i + 1] ?? "");
    else if (rest[i]?.startsWith("--")) options[rest[i]!.slice(2)] = rest[i + 1] ?? "";
  }
  return { probe, inputs, options };
}

function main(argv: readonly string[]): number {
  const { probe, inputs, options } = parseArgv(argv);
  const started = Date.now();
  const emit = (report: ProbeReport): number => {
    process.stdout.write(`${JSON.stringify(report, null, 2)}\n`);
    return report.status === "failed" ? 1 : 0;
  };
  const run = PROBES[probe];
  if (run === undefined) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe: probe || "(none)", probeVersion: PROBE_VERSION, engine: DOCX_ENGINE, status: "failed", durationMs: 0, measurements: {}, diagnostics: [{ severity: "error", message: `unknown probe ${probe}`, detail: `known: ${Object.keys(PROBES).join(", ")}` }] });
  }
  try {
    const result = run(inputs, options);
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: result.engine ?? DOCX_ENGINE, status: result.status, durationMs: Date.now() - started, measurements: result.measurements, ...(result.diagnostics === undefined ? {} : { diagnostics: result.diagnostics }) });
  } catch (error) {
    return emit({ schema: "semio.repository-test.probe-report/v2", probe, probeVersion: PROBE_VERSION, engine: DOCX_ENGINE, status: "failed", durationMs: Date.now() - started, measurements: {}, diagnostics: [{ severity: "error", message: String((error as Error).message ?? error) }] });
  }
}

if (import.meta.main) process.exit(main(process.argv.slice(2)));
//#endregion 🚀️Entry
