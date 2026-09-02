#!/usr/bin/env bun
//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// 🏭️ Third-party fixture generator for `s.stdio.docx@ecma-376/✳️any`.
//
// Every recipe below is a BEFORE and (where the outcome is legal) an AFTER docx, each one built
// DIRECTLY by `jszip`+`fast-xml-parser`'s `XMLBuilder` — never by "applying" a mutation in code. Both
// states of every recipe are independently authored here as typed body/style trees; nothing in this
// file re-derives one from the other by executing this repository's own mutation semantics, which is
// the whole reason this counts as independent evidence rather than a reflection of `crate::artifacts::docx`.
//
// A DOCX is an OPC package (a real ZIP archive) of real XML parts: `[Content_Types].xml`, `_rels/.rels`,
// `word/_rels/document.xml.rels`, `word/document.xml`, `word/styles.xml` and (for `set-part`/`remove-part`
// recipes) `docProps/core.xml`. Every part is written by `fast-xml-parser`'s `XMLBuilder` in
// `preserveOrder` mode — the ONLY mode that keeps sibling elements of DIFFERENT tag names (e.g. a `w:p`
// followed by a `w:tbl`) in the order this file puts them, rather than folding same-tag siblings into
// one array and losing cross-tag order, which the simpler object mode BCF's own generator uses would do.
//
// Generation and execution are SEPARATE operations, same shape as the `bcf@2.1/✳️any` generator this
// file's CLI is mirrored from: a normal test run must never be able to rewrite the expectation it is
// measured against.
//
//   bun 📜️script.ts generate [--only <fixture-id>]
//
// @see ../../../../../💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🏭️generator/📜️script.ts — the working
//      reference this file's CLI shape, FIXED_DATE handling and recipe-table shape are mirrored from
// @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/

//#endregion 🧲️Header

//#region 🔌️Adapters
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import JSZip from "jszip";
import { XMLBuilder } from "fast-xml-parser";
//#endregion 🔌️Adapters

//#region 🧬️Contract
/** 🌳 One `preserveOrder`-shaped XML node: exactly one tag key naming its children (or `#text`), plus
 *  an optional `:@` attribute bag. This is the ONLY builder mode that keeps sibling elements of
 *  DIFFERENT tag names in the order they were appended — `word/document.xml`'s body legitimately mixes
 *  `w:p` and `w:tbl` at the same nesting level, and the plain object-keyed mode folds same-tag
 *  siblings into one array, silently losing cross-tag order. */
type PNode = Record<string, unknown> & { ":@"?: Record<string, string> };

const BUILD = new XMLBuilder({ ignoreAttributes: false, attributeNamePrefix: "@_", format: false, preserveOrder: true, suppressEmptyNode: true });

function el(tag: string, attrs?: Record<string, string>, children: PNode[] = []): PNode {
  return attrs === undefined ? { [tag]: children } : { [tag]: children, ":@": attrs };
}
function text(value: string): PNode {
  return { "#text": value } as PNode;
}
function xmlDoc(root: PNode): string {
  return `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>${BUILD.build([root])}`;
}

const W_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const CT_NS = "http://schemas.openxmlformats.org/package/2006/content-types";
const REL_NS = "http://schemas.openxmlformats.org/package/2006/relationships";
const CP_NS = "http://schemas.openxmlformats.org/package/2006/metadata/core-properties";
const DC_NS = "http://purl.org/dc/elements/1.1/";

const CT_MAIN = "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml";
const CT_STYLES = "application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml";
const CT_RELS = "application/vnd.openxmlformats-package.relationships+xml";
const CT_XML = "application/xml";
export const CT_CORE_PROPS = "application/vnd.openxmlformats-package.core-properties+xml";

const REL_OFFICE_DOCUMENT = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const REL_STYLES = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const REL_CORE_PROPS = "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties";

export type DocxRun = { text: string; bold?: boolean; italic?: boolean; underline?: boolean };
export type DocxParagraph = { kind: "paragraph"; style?: string; runs: DocxRun[] };
export type DocxTableCell = { blocks: DocxBlock[] };
export type DocxTableRow = { cells: DocxTableCell[] };
export type DocxTable = { kind: "table"; rows: DocxTableRow[] };
export type DocxBlock = DocxParagraph | DocxTable;
export type DocxStyleRecipe = { id: string; name: string; basedOn?: string };
/** 📎️ A raw OPC part beyond the typed body/styles layer — `SetPart`/`RemovePart`'s own target shape
 *  (`path`, `content_type`, `bytes`). `xml` is this part's already-serialized content. */
export type ExtraPart = { path: string; contentType: string; xml: string };
export type DocxModel = { body: DocxBlock[]; styles: DocxStyleRecipe[]; extraParts?: ExtraPart[] };
//#endregion 🧬️Contract

//#region 📜️Serializers
function runXml(run: DocxRun): PNode {
  const rPr: PNode[] = [];
  if (run.bold === true) rPr.push(el("w:b"));
  if (run.italic === true) rPr.push(el("w:i"));
  if (run.underline === true) rPr.push(el("w:u", { "@_w:val": "single" }));
  const children: PNode[] = [];
  if (rPr.length > 0) children.push(el("w:rPr", undefined, rPr));
  children.push(el("w:t", { "@_xml:space": "preserve" }, [text(run.text)]));
  return el("w:r", undefined, children);
}

function paragraphXml(p: DocxParagraph): PNode {
  const children: PNode[] = [];
  if (p.style !== undefined) children.push(el("w:pPr", undefined, [el("w:pStyle", { "@_w:val": p.style })]));
  for (const run of p.runs) children.push(runXml(run));
  return el("w:p", undefined, children);
}

function tableXml(t: DocxTable): PNode {
  return el(
    "w:tbl",
    undefined,
    t.rows.map((row) => el("w:tr", undefined, row.cells.map((cell) => el("w:tc", undefined, cell.blocks.map(blockXml))))),
  );
}

function blockXml(b: DocxBlock): PNode {
  return b.kind === "paragraph" ? paragraphXml(b) : tableXml(b);
}

function documentXmlBytes(model: DocxModel): string {
  return xmlDoc(el("w:document", { "@_xmlns:w": W_NS }, [el("w:body", undefined, model.body.map(blockXml))]));
}

function stylesXmlBytes(model: DocxModel): string {
  const styleNodes = model.styles.map((s) => {
    const children: PNode[] = [el("w:name", { "@_w:val": s.name })];
    if (s.basedOn !== undefined) children.push(el("w:basedOn", { "@_w:val": s.basedOn }));
    return el("w:style", { "@_w:type": "paragraph", "@_w:styleId": s.id }, children);
  });
  return xmlDoc(el("w:styles", { "@_xmlns:w": W_NS }, styleNodes));
}

function contentTypesXmlBytes(model: DocxModel): string {
  const overrides = [el("Override", { "@_PartName": "/word/document.xml", "@_ContentType": CT_MAIN }), el("Override", { "@_PartName": "/word/styles.xml", "@_ContentType": CT_STYLES })];
  for (const part of model.extraParts ?? []) overrides.push(el("Override", { "@_PartName": `/${part.path}`, "@_ContentType": part.contentType }));
  return xmlDoc(el("Types", { "@_xmlns": CT_NS }, [el("Default", { "@_Extension": "rels", "@_ContentType": CT_RELS }), el("Default", { "@_Extension": "xml", "@_ContentType": CT_XML }), ...overrides]));
}

function packageRelsXmlBytes(model: DocxModel): string {
  const rels = [el("Relationship", { "@_Id": "rId1", "@_Type": REL_OFFICE_DOCUMENT, "@_Target": "word/document.xml" })];
  if ((model.extraParts ?? []).some((part) => part.path === "docProps/core.xml")) rels.push(el("Relationship", { "@_Id": "rId2", "@_Type": REL_CORE_PROPS, "@_Target": "docProps/core.xml" }));
  return xmlDoc(el("Relationships", { "@_xmlns": REL_NS }, rels));
}

function documentRelsXmlBytes(): string {
  return xmlDoc(el("Relationships", { "@_xmlns": REL_NS }, [el("Relationship", { "@_Id": "rId1", "@_Type": REL_STYLES, "@_Target": "styles.xml" })]));
}

/** 📎️ The one extra OPC part `set-part`/`remove-part` recipes exercise — real `docProps/core.xml`
 *  core-properties content, serialized the same way every other part is. */
export function corePropsXml(title: string): string {
  return xmlDoc(el("cp:coreProperties", { "@_xmlns:cp": CP_NS, "@_xmlns:dc": DC_NS }, [el("dc:title", undefined, [text(title)])]));
}

// 📌️ Fixed epoch for every zip entry: `jszip` stamps each entry's DOS date/time from `Date.now()` by
// default, AND auto-creates an implicit parent-folder entry for any nested path, stamping THAT folder
// entry with `new Date()` too and ignoring the child's own `date` option — confirmed empirically per
// ../../../../../💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️markup/🏭️generator/📜️script.ts's own comment. Every
// directory level therefore gets its own explicit dated entry BEFORE the file that lives in it.
const FIXED_DATE = new Date(Date.UTC(2026, 0, 1, 0, 0, 0));

async function buildDocx(model: DocxModel): Promise<Buffer> {
  const zip = new JSZip();
  zip.file("_rels/", null, { dir: true, date: FIXED_DATE });
  zip.file("_rels/.rels", packageRelsXmlBytes(model), { date: FIXED_DATE });
  zip.file("word/", null, { dir: true, date: FIXED_DATE });
  zip.file("word/document.xml", documentXmlBytes(model), { date: FIXED_DATE });
  zip.file("word/styles.xml", stylesXmlBytes(model), { date: FIXED_DATE });
  zip.file("word/_rels/", null, { dir: true, date: FIXED_DATE });
  zip.file("word/_rels/document.xml.rels", documentRelsXmlBytes(), { date: FIXED_DATE });
  zip.file("[Content_Types].xml", contentTypesXmlBytes(model), { date: FIXED_DATE });
  for (const part of model.extraParts ?? []) {
    const slash = part.path.lastIndexOf("/");
    if (slash >= 0) zip.file(`${part.path.slice(0, slash)}/`, null, { dir: true, date: FIXED_DATE });
    zip.file(part.path, part.xml, { date: FIXED_DATE });
  }
  return zip.generateAsync({ type: "nodebuffer", compression: "DEFLATE" });
}
//#endregion 📜️Serializers

//#region 🍳️Recipes
function para(runs: DocxRun[], style?: string): DocxParagraph {
  return style === undefined ? { kind: "paragraph", runs } : { kind: "paragraph", style, runs };
}
function withBody(model: DocxModel, edit: (body: DocxBlock[]) => DocxBlock[]): DocxModel {
  return { ...model, body: edit([...model.body]) };
}
function withStyles(model: DocxModel, edit: (styles: DocxStyleRecipe[]) => DocxStyleRecipe[]): DocxModel {
  return { ...model, styles: edit([...model.styles]) };
}
function withExtraParts(model: DocxModel, edit: (parts: ExtraPart[]) => ExtraPart[]): DocxModel {
  return { ...model, extraParts: edit([...(model.extraParts ?? [])]) };
}

// 🗼 Same opening paragraph as the committed `set-snapshot/🧪️tests/bolds-the-tower-run-of-the-opening-
// paragraph` fixture's own `📸️snapshot/⬅️before` — reused here on purpose so this independently-built
// DOCX and that hand-authored `DocxSnapshot` JSON describe the same document.
const STYLE_HEADING: DocxStyleRecipe = { id: "Heading1", name: "heading 1" };
const STYLE_BODY: DocxStyleRecipe = { id: "Body", name: "Body Text" };
const STYLE_QUOTE: DocxStyleRecipe = { id: "Quote", name: "Quote", basedOn: "Body" };

const P_TOWER: DocxParagraph = { kind: "paragraph", style: "Heading1", runs: [{ text: "Nakagin " }, { text: "Capsule Tower" }] };
const P_CLOSING: DocxParagraph = { kind: "paragraph", style: "Body", runs: [{ text: "The tower stands over the river." }] };

const BASE_MODEL: DocxModel = { body: [P_TOWER, P_CLOSING], styles: [STYLE_HEADING, STYLE_BODY] };
const CORE_TITLE = "Nakagin Capsule Tower Survey";

export type Recipe = { id: string; mutation: string; outcome: "applied" | "no-op" | "rejected"; before: DocxModel; after?: DocxModel; notes: string };

export const RECIPES: Recipe[] = [
  { id: "no-mutation-no-op", mutation: "set-snapshot", outcome: "no-op", before: BASE_MODEL, after: BASE_MODEL, notes: "The identity element — `DocxMutation::NoMutation` was dropped (`#[derive(dsl::Mutations)]` rejects unit variants), so the no-op identity is now `DocxMutation::SetSnapshot` applied to an identical snapshot, which `diff_set_snapshot` resolves to `DocxDiff::default()` — before and after are the same document, byte for byte." },

  {
    id: "bolds-the-tower-run-of-the-opening-paragraph",
    mutation: "set-snapshot",
    outcome: "applied",
    before: BASE_MODEL,
    after: withBody(BASE_MODEL, (body) => [{ ...P_TOWER, runs: [P_TOWER.runs[0]!, { ...P_TOWER.runs[1]!, bold: true }] }, ...body.slice(1)]),
    notes: "Same scenario id and opening paragraph as the committed set-snapshot/🧪️tests fixture: the second run of the opening paragraph is bolded, nothing else moves.",
  },
  { id: "set-snapshot-no-op-identical-snapshot", mutation: "set-snapshot", outcome: "no-op", before: BASE_MODEL, after: BASE_MODEL, notes: "`set-snapshot`'s own diff leaf explicitly warns `mutation.no-op` and returns `DocxDiff::default()` when the new snapshot is identical to the current one." },

  {
    id: "insert-block-appends-a-pricing-table",
    mutation: "insert-block",
    outcome: "applied",
    before: BASE_MODEL,
    after: withBody(BASE_MODEL, (body) => [
      ...body,
      { kind: "table", rows: [{ cells: [{ blocks: [para([{ text: "Item" }])] }, { blocks: [para([{ text: "Price" }])] }] }, { cells: [{ blocks: [para([{ text: "Capsule" }])] }, { blocks: [para([{ text: "¥1,000,000" }])] }] }] },
    ]),
    notes: "Appends a 2×2 table as the body's third (final) block — also the corpus's one exercise of table/row/cell serialization and projection.",
  },
  { id: "insert-block-rejected-invalid-index", mutation: "insert-block", outcome: "rejected", before: BASE_MODEL, notes: "Inserting at index 99 of a 2-block body is outside the final collection — `apply_indexed`'s `mutation.apply.invalid-index`. No legal after-state exists, so only `before.docx` is carried." },

  { id: "remove-block-drops-the-closing-paragraph", mutation: "remove-block", outcome: "applied", before: BASE_MODEL, after: withBody(BASE_MODEL, (body) => body.slice(0, 1)), notes: "Removes the second (closing) paragraph, leaving the opening paragraph alone." },
  { id: "remove-block-rejected-missing-index", mutation: "remove-block", outcome: "rejected", before: BASE_MODEL, notes: "Removing block index 5 of a 2-block body does not exist — `apply_indexed`'s `mutation.apply.missing-target`. Before-only." },

  {
    id: "set-block-content-replaces-the-closing-paragraph",
    mutation: "set-block-content",
    outcome: "applied",
    before: BASE_MODEL,
    after: withBody(BASE_MODEL, (body) => [body[0]!, para([{ text: "The tower has stood since 1972." }], "Body")]),
    notes: "Replaces the closing paragraph's full content with different text via the same style.",
  },
  { id: "set-block-content-no-op-unchanged-content", mutation: "set-block-content", outcome: "no-op", before: BASE_MODEL, after: BASE_MODEL, notes: "Replacing a block with structurally identical content: `diff_block(old,new)` returns `None`, so `diff_set_block_content` returns `DocxDiff::default()`." },

  { id: "set-run-text-rewrites-the-closing-paragraph", mutation: "set-run-text", outcome: "applied", before: BASE_MODEL, after: withBody(BASE_MODEL, (body) => [body[0]!, para([{ text: "The tower has stood since 1972." }], "Body")]), notes: "Rewrites the closing paragraph's single run's literal text." },
  { id: "set-run-text-no-op-identical-text", mutation: "set-run-text", outcome: "no-op", before: BASE_MODEL, after: BASE_MODEL, notes: "`diff_set_run_text` explicitly checks `run.text == text` and returns `DocxDiff::default()` when the new text is identical to the current one." },

  { id: "set-run-formatting-italicizes-the-closing-paragraph", mutation: "set-run-formatting", outcome: "applied", before: BASE_MODEL, after: withBody(BASE_MODEL, (body) => [body[0]!, para([{ text: "The tower stands over the river.", italic: true }], "Body")]), notes: "Sets the closing paragraph's single run to italic; bold/underline stay false." },
  { id: "set-run-formatting-no-op-identical-flags", mutation: "set-run-formatting", outcome: "no-op", before: BASE_MODEL, after: BASE_MODEL, notes: "`diff_set_run_formatting` returns `DocxDiff::default()` when bold/italic/underline all already match the requested flags." },

  { id: "insert-style-adds-a-quote-style", mutation: "insert-style", outcome: "applied", before: BASE_MODEL, after: withStyles(BASE_MODEL, (styles) => [...styles, STYLE_QUOTE]), notes: "Inserts a new `Quote` style, based on `Body`, at the end of the style table." },
  {
    id: "insert-style-rejected-duplicate-id",
    mutation: "insert-style",
    outcome: "rejected",
    before: withStyles(BASE_MODEL, (styles) => [...styles, STYLE_QUOTE]),
    notes: "The before-state already carries a `Quote` style; inserting another with the same id is `apply_named`'s `mutation.apply.duplicate-target`. Before-only.",
  },

  { id: "remove-style-drops-the-quote-style", mutation: "remove-style", outcome: "applied", before: withStyles(BASE_MODEL, (styles) => [...styles, STYLE_QUOTE]), after: BASE_MODEL, notes: "Removes the `Quote` style, leaving `Heading1`/`Body`." },
  { id: "remove-style-rejected-missing-id", mutation: "remove-style", outcome: "rejected", before: BASE_MODEL, notes: "The before-state has no `Quote` style; removing it is `apply_named`'s `mutation.apply.missing-target`. Before-only." },

  { id: "set-style-name-renames-the-body-style", mutation: "set-style-name", outcome: "applied", before: BASE_MODEL, after: withStyles(BASE_MODEL, (styles) => styles.map((s) => (s.id === "Body" ? { ...s, name: "Body Copy" } : s))), notes: "Renames the `Body` style from \"Body Text\" to \"Body Copy\"." },
  { id: "set-style-name-rejected-missing-id", mutation: "set-style-name", outcome: "rejected", before: BASE_MODEL, notes: "The before-state has no `Quote` style; renaming it is a named modification against a missing target. Before-only." },

  {
    id: "set-style-based-on-reparents-the-quote-style",
    mutation: "set-style-based-on",
    outcome: "applied",
    before: withStyles(BASE_MODEL, (styles) => [...styles, STYLE_QUOTE]),
    after: withStyles(BASE_MODEL, (styles) => [...styles, { ...STYLE_QUOTE, basedOn: "Heading1" }]),
    notes: "Reparents `Quote` from `basedOn: Body` to `basedOn: Heading1`.",
  },
  { id: "set-style-based-on-rejected-missing-id", mutation: "set-style-based-on", outcome: "rejected", before: BASE_MODEL, notes: "The before-state has no `Quote` style; setting its `basedOn` is a named modification against a missing target. Before-only." },

  { id: "set-part-adds-core-properties", mutation: "set-part", outcome: "applied", before: BASE_MODEL, after: withExtraParts(BASE_MODEL, (parts) => [...parts, { path: "docProps/core.xml", contentType: CT_CORE_PROPS, xml: corePropsXml(CORE_TITLE) }]), notes: "Inserts a real `docProps/core.xml` part (content this typed layer doesn't model) that did not exist before." },
  {
    id: "set-part-no-op-identical-content",
    mutation: "set-part",
    outcome: "no-op",
    before: withExtraParts(BASE_MODEL, (parts) => [...parts, { path: "docProps/core.xml", contentType: CT_CORE_PROPS, xml: corePropsXml(CORE_TITLE) }]),
    after: withExtraParts(BASE_MODEL, (parts) => [...parts, { path: "docProps/core.xml", contentType: CT_CORE_PROPS, xml: corePropsXml(CORE_TITLE) }]),
    notes: "Setting a part to content-type+bytes identical to what is already there: `diff_part` returns `None`, so `diff_set_part` returns `DocxDiff::default()`.",
  },

  { id: "remove-part-drops-core-properties", mutation: "remove-part", outcome: "applied", before: withExtraParts(BASE_MODEL, (parts) => [...parts, { path: "docProps/core.xml", contentType: CT_CORE_PROPS, xml: corePropsXml(CORE_TITLE) }]), after: BASE_MODEL, notes: "Removes the `docProps/core.xml` part that existed in the before-state." },
  { id: "remove-part-rejected-missing-path", mutation: "remove-part", outcome: "rejected", before: BASE_MODEL, notes: "The before-state carries no `docProps/core.xml` part; removing it targets a part that is not in the package. Before-only." },
];
//#endregion 🍳️Recipes

//#region 🚀️Entry
async function generateOne(id: string, outDir: string): Promise<void> {
  const recipe = RECIPES.find((entry) => entry.id === id);
  if (!recipe) throw new Error(`unknown recipe ${id} — known: ${RECIPES.map((entry) => entry.id).join(", ")}`);
  const dir = join(outDir, id);
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, "before.docx"), await buildDocx(recipe.before));
  if (recipe.outcome !== "rejected") {
    if (!recipe.after) throw new Error(`recipe ${id} is declared ${recipe.outcome} but has no after state`);
    writeFileSync(join(dir, "after.docx"), await buildDocx(recipe.after));
  }
}

async function main(argv: readonly string[]): Promise<number> {
  const [command, ...rest] = argv;
  if (command !== "generate") {
    console.error(`usage: bun 📜️script.ts generate [--only <fixture-id>]`);
    return 2;
  }
  const onlyIndex = rest.indexOf("--only");
  const only = onlyIndex >= 0 ? rest[onlyIndex + 1] : undefined;
  const outDir = process.env.SEMIO_FIXTURE_OUT ?? join(process.cwd(), "🧫️fixtures");
  const ids = only ? [only] : RECIPES.map((entry) => entry.id);
  for (const id of ids) {
    await generateOne(id, outDir);
    console.log(`[docx generator] ${id} -> ${join(outDir, id)}`);
  }
  return 0;
}

if (import.meta.main) process.exit(await main(process.argv.slice(2)));
//#endregion 🚀️Entry
