#!/usr/bin/env bun
// 🪢️ Replayable third-party generator and verifier of the OPC-plumbing pairs of `s.stdio.xlsx@ecma-376/🧱️base`: `set-relationship`, `remove-relationship`, `set-content-type` and
// `remove-content-type`. Every `before`/`after` pair is built from the artifact's COMMITTED real package by `jszip` (container) and `fast-xml-parser` (`[Content_Types].xml`, `*.rels`),
// both declared in `✏️s/package.json` and resolved from `✏️s/node_modules`; nothing here imports this repository's codec or mutation semantics. Each recipe states its effect twice, once as
// the XML edit that writes the `after` and once as a model-level expectation, and the script refuses a pair whose re-read plumbing differs from the expectation.
//
//   repo tools exec bun -- <this file> verify      re-derives every committed pair and compares it byte for byte
//   repo tools exec bun -- <this file> generate    rewrites the pair fixtures (their testEvidence rows are kept)
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import JSZip from "jszip";
import { XMLBuilder, XMLParser } from "fast-xml-parser";

const ARTIFACT = resolve(import.meta.dir, "../..");
const SUBSET = "🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base";
const FIXED_DATE = new Date(Date.UTC(2026, 0, 1, 0, 0, 0));
const REL_HYPERLINK = "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
const REL_CUSTOM = "http://example.invalid/relationships/oracle-retyped";

const REAL = "📕️reuse-marketplaces.xlsx";

//#region 🧬️Model
type Rel = { id: string; type: string; target: string; external: boolean };
type Plumbing = { defaults: [string, string][]; overrides: [string, string][]; rels: Record<string, Rel[]> };
type Step =
  | { op: "set-relationship"; owner: string; rel: Rel; index?: number }
  | { op: "remove-relationship"; owner: string; id: string }
  | { op: "set-content-type"; override: boolean; name: string; contentType: string; index?: number }
  | { op: "remove-content-type"; override: boolean; name: string };
type Recipe = { id: string; dir: string; mutation: Step["op"]; outcome: "applied" | "rejected"; before: Step[]; step: Step; notes: string };
//#endregion 🧬️Model

//#region 📖️Plumbing
const PARSE = new XMLParser({ ignoreAttributes: false, attributeNamePrefix: "@_", preserveOrder: true, trimValues: false, ignoreDeclaration: true });
const BUILD = new XMLBuilder({ ignoreAttributes: false, attributeNamePrefix: "@_", preserveOrder: true, format: false, suppressEmptyNode: true });
type PNode = Record<string, unknown> & { ":@"?: Record<string, string> };
const tagOf = (node: PNode): string => Object.keys(node).find((key) => key !== ":@")!;
const rowsOf = (root: PNode): PNode[] => (root[tagOf(root)] as PNode[]).filter((node) => !("#text" in node));
const attr = (node: PNode, name: string): string => node[":@"]?.[`@_${name}`] ?? "";
const ownerOf = (path: string): string | undefined => {
  const match = /^(?:(.*)\/)?_rels\/([^/]*)\.rels$/.exec(path);
  return match === null ? undefined : `${match[1] === undefined ? "" : `${match[1]}/`}${match[2]}`;
};
const relsPathOf = (owner: string): string => {
  const slash = owner.lastIndexOf("/");
  return slash < 0 ? `_rels/${owner}.rels` : `${owner.slice(0, slash)}/_rels/${owner.slice(slash + 1)}.rels`;
};

async function readPlumbing(zip: JSZip): Promise<Plumbing> {
  const types = PARSE.parse(await zip.file("[Content_Types].xml")!.async("string"))[0] as PNode;
  const rows = rowsOf(types);
  const plumbing: Plumbing = { defaults: [], overrides: [], rels: {} };
  for (const row of rows) {
    if (tagOf(row) === "Default") plumbing.defaults.push([attr(row, "Extension"), attr(row, "ContentType")]);
    if (tagOf(row) === "Override") plumbing.overrides.push([attr(row, "PartName"), attr(row, "ContentType")]);
  }
  for (const name of Object.keys(zip.files).filter((entry) => !zip.files[entry]!.dir)) {
    const owner = ownerOf(name);
    if (owner === undefined) continue;
    const root = PARSE.parse(await zip.file(name)!.async("string"))[0] as PNode;
    plumbing.rels[owner] = rowsOf(root).map((row) => ({ id: attr(row, "Id"), type: attr(row, "Type"), target: attr(row, "Target"), external: attr(row, "TargetMode") === "External" }));
  }
  return plumbing;
}

/** 🧮️ The model-level effect of one step: an independent restatement of what the XML edit below writes. */
function expect(model: Plumbing, step: Step): Plumbing {
  const next: Plumbing = { defaults: model.defaults.map((entry) => [...entry]), overrides: model.overrides.map((entry) => [...entry]), rels: Object.fromEntries(Object.entries(model.rels).map(([owner, rels]) => [owner, rels.map((rel) => ({ ...rel }))])) };
  const place = <T>(list: T[], value: T, index: number | undefined) => list.splice(Math.min(index ?? list.length, list.length), 0, value);
  if (step.op === "set-relationship") {
    const list = (next.rels[step.owner] ??= []);
    const at = list.findIndex((rel) => rel.id === step.rel.id);
    if (at >= 0) list[at] = step.rel;
    else place(list, step.rel, step.index);
  } else if (step.op === "remove-relationship") {
    const list = next.rels[step.owner];
    const at = list?.findIndex((rel) => rel.id === step.id) ?? -1;
    if (list === undefined || at < 0) throw new Error(`no relationship ${step.id} of ${step.owner || "(root)"}`);
    list.splice(at, 1);
    if (list.length === 0) delete next.rels[step.owner];
  } else {
    const list = step.override ? next.overrides : next.defaults;
    const name = step.name;
    const at = list.findIndex(([key]) => key === name);
    if (step.op === "set-content-type") {
      if (!step.override && list.some(([key]) => key !== name && key.toLowerCase() === name.toLowerCase())) throw new Error("default extension exists in another letter case");
      if (at >= 0) list[at] = [name, step.contentType];
      else place(list, [name, step.contentType], step.index);
    } else {
      if (at < 0) throw new Error(`no ${step.override ? "override" : "default"} ${name}`);
      list.splice(at, 1);
    }
  }
  return next;
}
//#endregion 📖️Plumbing

//#region ✍️Edit
const rowNode = (tag: string, attrs: Record<string, string>): PNode => ({ [tag]: [], ":@": Object.fromEntries(Object.entries(attrs).map(([key, value]) => [`@_${key}`, value])) }) as PNode;

/** ✍️ The XML edit of one step, written with `fast-xml-parser`'s tree and `jszip`'s container. Rejected steps raise before anything is written. */
async function edit(zip: JSZip, step: Step): Promise<void> {
  const insertAmong = (rows: PNode[], tag: string, row: PNode, index: number | undefined) => {
    const slots = rows.map((node, at) => [node, at] as const).filter(([node]) => tagOf(node) === tag).map(([, at]) => at);
    const physical = index !== undefined && index < slots.length ? slots[index]! : slots.length > 0 ? slots[slots.length - 1]! + 1 : tag === "Default" ? 0 : rows.length;
    rows.splice(physical, 0, row);
  };
  const rewrite = (name: string, root: PNode, rows: PNode[]) => {
    (root as Record<string, unknown>)[tagOf(root)] = rows;
    zip.file(name, `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>${BUILD.build([root])}`, { date: FIXED_DATE, createFolders: false });
  };
  if (step.op === "set-relationship" || step.op === "remove-relationship") {
    const name = relsPathOf(step.owner);
    const exists = zip.file(name) !== null;
    const root = exists ? (PARSE.parse(await zip.file(name)!.async("string"))[0] as PNode) : ({ Relationships: [], ":@": { "@_xmlns": "http://schemas.openxmlformats.org/package/2006/relationships" } } as PNode);
    const rows = rowsOf(root);
    const at = rows.findIndex((row) => attr(row, "Id") === (step.op === "set-relationship" ? step.rel.id : step.id));
    if (step.op === "set-relationship") {
      const row = rowNode("Relationship", { Id: step.rel.id, Type: step.rel.type, Target: step.rel.target, ...(step.rel.external ? { TargetMode: "External" } : {}) });
      if (at >= 0) rows[at] = row;
      else insertAmong(rows, "Relationship", row, step.index);
      rewrite(name, root, rows);
    } else {
      if (!exists || at < 0) throw new Error(`rejected: no relationship ${step.id} of ${step.owner || "(root)"}`);
      rows.splice(at, 1);
      if (rows.length === 0) zip.remove(name);
      else rewrite(name, root, rows);
    }
    return;
  }
  const root = PARSE.parse(await zip.file("[Content_Types].xml")!.async("string"))[0] as PNode;
  const rows = rowsOf(root);
  const tag = step.override ? "Override" : "Default";
  const key = step.override ? "PartName" : "Extension";
  const name = step.name;
  const at = rows.findIndex((row) => tagOf(row) === tag && attr(row, key) === name);
  if (step.op === "set-content-type") {
    if (!step.override && rows.some((row) => tagOf(row) === "Default" && attr(row, key) !== name && attr(row, key).toLowerCase() === name.toLowerCase())) throw new Error("rejected: default extension exists in another letter case");
    const row = rowNode(tag, { [key]: name, ContentType: step.contentType });
    if (at >= 0) rows[at] = row;
    else insertAmong(rows, tag, row, step.index);
  } else {
    if (at < 0) throw new Error(`rejected: no ${tag} ${name}`);
    rows.splice(at, 1);
  }
  rewrite("[Content_Types].xml", root, rows);
}
//#endregion ✍️Edit

//#region 🍳️Recipes
const external = (id: string): Rel => ({ id, type: REL_HYPERLINK, target: "https://example.invalid/oracle", external: true });

function recipes(model: Plumbing): Recipe[] {
  const root = model.rels[""]!;
  const last = root[root.length - 1]!;
  const extension = model.defaults.find(([name]) => name === "xml")?.[0] ?? model.defaults[0]![0];
  const addDefault: Step = { op: "set-content-type", override: false, name: "zzoracle", contentType: "application/x-oracle", index: 0 };
  return [
    { id: "set-relationship-inserts-an-external-hyperlink-first", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: external("rIdOracleLink"), index: 0 }, notes: "A new external relationship is inserted at position 0 of the package root's relationships." },
    { id: "set-relationship-appends-an-external-hyperlink", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: external("rIdOracleLink") }, notes: "A new external relationship without a position is appended after the last relationship of the package root." },
    { id: "set-relationship-retypes-the-last-root-relationship", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: { ...last, type: REL_CUSTOM } }, notes: "An existing id is changed in place: the last root relationship keeps its id, target and position and takes another type." },
    { id: "set-relationship-no-op-identical-relationship", dir: "📎", mutation: "set-relationship", outcome: "applied", before: [], step: { op: "set-relationship", owner: "", rel: { ...last } }, notes: "Writing a relationship identical to the stored one changes nothing." },
    { id: "remove-relationship-drops-the-last-root-relationship", dir: "🧷", mutation: "remove-relationship", outcome: "applied", before: [], step: { op: "remove-relationship", owner: "", id: last.id }, notes: "The last relationship of the package root is removed; the others keep their order." },
    { id: "remove-relationship-drops-an-added-hyperlink", dir: "🧷", mutation: "remove-relationship", outcome: "applied", before: [{ op: "set-relationship", owner: "", rel: external("rIdOracleLink"), index: 0 }], step: { op: "remove-relationship", owner: "", id: "rIdOracleLink" }, notes: "The relationship added at position 0 is removed again; the package root is back to its original relationships." },
    { id: "remove-relationship-rejected-missing-id", dir: "🧷", mutation: "remove-relationship", outcome: "rejected", before: [], step: { op: "remove-relationship", owner: "", id: "rIdNothing" }, notes: "The package root owns no relationship rIdNothing. Before-only." },
    { id: "set-content-type-inserts-a-default-first", dir: "📇", mutation: "set-content-type", outcome: "applied", before: [], step: addDefault, notes: "A new extension default is inserted at position 0 of the content-types defaults." },
    { id: "set-content-type-retypes-an-existing-default", dir: "📇", mutation: "set-content-type", outcome: "applied", before: [], step: { op: "set-content-type", override: false, name: extension, contentType: "application/x-oracle-retyped" }, notes: "An existing default keeps its position and takes another content type." },
    { id: "set-content-type-no-op-identical-entry", dir: "📇", mutation: "set-content-type", outcome: "applied", before: [], step: { op: "set-content-type", override: false, name: model.defaults[0]![0], contentType: model.defaults[0]![1] }, notes: "Writing the stored content type again changes nothing." },
    { id: "set-content-type-rejected-default-in-another-case", dir: "📇", mutation: "set-content-type", outcome: "rejected", before: [addDefault], step: { op: "set-content-type", override: false, name: "ZZORACLE", contentType: "application/x-oracle-other" }, notes: "The extension default zzoracle exists in lower case; ZZORACLE would be a second default in another letter case. Before-only." },
    { id: "remove-content-type-drops-an-added-default", dir: "🧺", mutation: "remove-content-type", outcome: "applied", before: [addDefault], step: { op: "remove-content-type", override: false, name: "zzoracle" }, notes: "The default added at position 0 is removed again." },
    { id: "remove-content-type-rejected-missing-entry", dir: "🧺", mutation: "remove-content-type", outcome: "rejected", before: [], step: { op: "remove-content-type", override: false, name: "nothing" }, notes: "The content types hold no default for the extension nothing. Before-only." },
  ];
}
//#endregion 🍳️Recipes

//#region 🚀️Entry
const dirName = (recipe: Recipe): string => `${recipe.dir}\uFE0F${recipe.id}`;
const same = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);
const fixtures = join(ARTIFACT, SUBSET, "🧪️tests", "🧫️fixtures");

async function build(recipe: Recipe): Promise<{ before: Buffer; after?: Buffer }> {
  const zip = await JSZip.loadAsync(readFileSync(join(fixtures, REAL)));
  let model = await readPlumbing(zip);
  for (const step of recipe.before) {
    await edit(zip, step);
    model = expect(model, step);
  }
  const generate = (target: JSZip) => target.generateAsync({ type: "nodebuffer", compression: "DEFLATE" });
  const before = await generate(zip);
  if (!same(await readPlumbing(await JSZip.loadAsync(before)), model)) throw new Error(`${recipe.id}: the before package does not carry its recipe's plumbing`);
  if (recipe.outcome === "rejected") {
    let refused = false;
    try {
      await edit(await JSZip.loadAsync(before), recipe.step);
      expect(model, recipe.step);
    } catch {
      refused = true;
    }
    if (!refused) throw new Error(`${recipe.id}: a rejected recipe was accepted`);
    return { before };
  }
  const afterZip = await JSZip.loadAsync(before);
  await edit(afterZip, recipe.step);
  const after = await generate(afterZip);
  if (!same(await readPlumbing(await JSZip.loadAsync(after)), expect(model, recipe.step))) throw new Error(`${recipe.id}: the after package differs from the model-level expectation`);
  return { before, after };
}

async function generate(): Promise<void> {
  const model = await readPlumbing(await JSZip.loadAsync(readFileSync(join(fixtures, REAL))));
  for (const recipe of recipes(model)) {
    const built = await build(recipe);
    const directory = join(fixtures, dirName(recipe));
    mkdirSync(directory, { recursive: true });
    writeFileSync(join(directory, "⬅️before.xlsx"), built.before);
    if (built.after !== undefined) writeFileSync(join(directory, "➡️after.xlsx"), built.after);
    console.log(`[xlsx plumbing] ${recipe.id} -> ${directory}`);
  }
}

async function verify(): Promise<number> {
  let failures = 0;
  let byteIdentical = 0;
  const model = await readPlumbing(await JSZip.loadAsync(readFileSync(join(fixtures, REAL))));
  for (const recipe of recipes(model)) {
    try {
      const built = await build(recipe);
      const directory = join(fixtures, dirName(recipe));
      const pairs: [string, Buffer | undefined][] = [["⬅️before.xlsx", built.before], ["➡️after.xlsx", built.after]];
      for (const [name, bytes] of pairs) {
        if (bytes === undefined) continue;
        const path = join(directory, name);
        if (!existsSync(path)) throw new Error(`${name} is not committed`);
        const committed = readFileSync(path);
        if (committed.equals(bytes)) byteIdentical += 1;
        else if (!same(await readPlumbing(await JSZip.loadAsync(committed)), await readPlumbing(await JSZip.loadAsync(bytes)))) throw new Error(`${name}: the committed plumbing differs from the re-derived one`);
        else console.log(`${recipe.id}: ${name} has the same plumbing but other container bytes`);
      }
    } catch (error) {
      failures += 1;
      console.error(`${recipe.id}: ${(error as Error).message}`);
    }
  }
  console.log(failures === 0 ? `every plumbing pair re-derives (${byteIdentical} files byte-identical)` : `${failures} pair(s) failed`);
  return failures === 0 ? 0 : 1;
}

async function main(argv: readonly string[]): Promise<number> {
  const [command] = argv;
  if (command === "verify") return verify();
  if (command === "generate") return (await generate(), 0);
  console.error("usage: <this file> generate | verify");
  return 2;
}

if (import.meta.main) process.exitCode = await main(process.argv.slice(2));
//#endregion 🚀️Entry
