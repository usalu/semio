/** 🧷️ Proves launch seed reconciliation on the language-neutral corpus, with an independent JSONC parser as the oracle. */
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv/dist/2020.js";
import { findNodeAtLocation, getNodeValue, parse, parseTree, type Node, type ParseError } from "jsonc-parser";
import { getWorkspaceRoot } from "../../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { generateLaunchJson, launchContainerRanges, reconcileLaunchSeed } from "../../🟦️.ts";
import fixture from "../../🧫️fixtures/🧫️seed-reconcile/🔣️.json";

type LineEdit = { readonly at: number; readonly remove: number; readonly insert: readonly string[] };
type LaunchEdit = { readonly container: string; readonly after?: string | null; readonly replace?: string; readonly text: readonly string[] };
type Document = { readonly configurations: readonly Record<string, unknown>[]; readonly inputs: readonly Record<string, unknown>[] };
const root = getWorkspaceRoot();
/** 📄️ Joins a line list into file text, applying 1-based line edits from the last line up. */
const text = (lines: readonly string[], edits: readonly LineEdit[] = []): string => {
  const out = [...lines];
  for (const edit of [...edits].sort((left, right) => right.at - left.at)) out.splice(edit.at - 1, edit.remove, ...edit.insert);
  return `${out.join("\n")}\n`;
};
const render = (seed: string): string => generateLaunchJson(root, [], fixture.targets, () => seed);
const rows = (source: string, container: string): Node[] => findNodeAtLocation(parseTree(source, [], { allowTrailingComma: true })!, [container])?.children ?? [];
/** ✋️ Applies one hand edit to a launch file through the independent parser's offsets. */
function handEdit(source: string, edit: LaunchEdit): string {
  const key = edit.container === "inputs" ? "id" : "name", nodes = rows(source, edit.container), body = edit.text.join("\n");
  const find = (value: string): Node => {
    const node = nodes.find((candidate) => candidate.type === "object" && getNodeValue(candidate)[key] === value);
    if (!node) throw new Error(`corpus names no ${edit.container} row ${value}`);
    return node;
  };
  if (edit.replace !== undefined) return source.slice(0, find(edit.replace).offset) + body + source.slice(find(edit.replace).offset + find(edit.replace).length);
  if (edit.after === null || edit.after === undefined) return `${source.slice(0, nodes[0]!.offset)}${body},\n    ${source.slice(nodes[0]!.offset)}`;
  const end = find(edit.after).offset + find(edit.after).length;
  return `${source.slice(0, end)},\n    ${body}${source.slice(end)}`;
}
/** 🔭️ Reads every top-level array and its element ranges through the independent parser. */
function independentRanges(source: string): Record<string, { range: { start: number; end: number }; elements: { start: number; end: number }[] }> {
  const errors: ParseError[] = [], tree = parseTree(source, errors, { allowTrailingComma: true })!;
  expect(errors).toEqual([]);
  return Object.fromEntries((tree.children ?? []).filter((property) => property.children![1]!.type === "array").map((property) => {
    const array = property.children![1]!;
    return [property.children![0]!.value as string, { range: { start: array.offset, end: array.offset + array.length }, elements: (array.children ?? []).map((node) => ({ start: node.offset, end: node.offset + node.length })) }];
  }));
}
const independent = (source: string): Document => {
  const errors: ParseError[] = [], value = parse(source, errors, { allowTrailingComma: true }) as Document;
  expect(errors).toEqual([]);
  return value;
};

test("the seed reconcile corpus matches its schema", () => {
});

test("launch container ranges equal an independent JSONC parser on every corpus document", () => {
  const base = text(fixture.seed), sources = [base, render(base)];
  for (const row of fixture.cases) sources.push(text(fixture.seed, row.seedEdits ?? []), (row.launchEdits as LaunchEdit[]).reduce(handEdit, render(base)));
  for (const source of sources) expect(launchContainerRanges(source)).toEqual(independentRanges(source));
});

test("launch container ranges equal an independent JSONC parser on the repository launch pair", () => {
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const source = readFileSync(join(root, path), "utf8"), ranges = launchContainerRanges(source);
    expect(ranges, path).toEqual(independentRanges(source));
    expect(ranges.configurations!.elements.length, path).toBeGreaterThan(1000);
  }
});

test("every corpus case reconciles to the exact seed bytes, settles, and renders what the launch file carries", () => {
  const base = text(fixture.seed);
  for (const row of fixture.cases) {
    const seed = text(fixture.seed, row.seedEdits ?? []), launch = (row.launchEdits as LaunchEdit[]).reduce(handEdit, render(base));
    if ("error" in row.expect) { expect(() => reconcileLaunchSeed(seed, launch, render, row.adoptEdits), row.id).toThrow(row.expect.error); continue; }
    const { seed: next, ...report } = reconcileLaunchSeed(seed, launch, render, row.adoptEdits), { seedEdits, ...expected } = row.expect;
    expect(report, row.id).toEqual(expected);
    expect(next, row.id).toBe(text(fixture.seed, seedEdits));
    expect(Bun.JSONC.parse(next), row.id).toEqual(independent(next));
    const rendered = independent(render(next)), carried = independent(launch);
    for (const name of [...report.adopted, ...(row.adoptEdits ? report.edited : [])]) expect(rendered.configurations.filter((entry) => entry.name === name), `${row.id} ${name}`).toEqual([carried.configurations.find((entry) => entry.name === name)!]);
    for (const id of report.adoptedInputs) expect(rendered.inputs.filter((entry) => entry.id === id), `${row.id} ${id}`).toEqual([carried.inputs.find((entry) => entry.id === id)!]);
    const lost = carried.configurations.filter((entry) => !rendered.configurations.some((twin) => twin.name === entry.name)).map((entry) => entry.name);
    expect(lost, row.id).toEqual([...report.renamed, ...report.stale.filter((name) => !name.startsWith("projectTarget."))]);
    const again = reconcileLaunchSeed(next, launch, render, row.adoptEdits);
    expect({ seed: again.seed, adopted: again.adopted, adoptedInputs: again.adoptedInputs, misplaced: again.misplaced }, row.id).toEqual({ seed: next, adopted: [], adoptedInputs: [], misplaced: 0 });
  }
});

test("launch generation admits semantic seed formatting without a comment boundary", () => {
  const original = text(fixture.seed), document = Bun.JSONC.parse(original);
  const expected = independent(render(original));
  for (const format of fixture.seedFormatting) {
    const seed = format === "commentless-lines" ? text(fixture.seed.filter((line) => !line.trimStart().startsWith("//"))) : JSON.stringify(document, null, format === "pretty-json" ? 2 : undefined);
    expect(Bun.JSONC.parse(seed), format).toEqual(independent(seed));
    const actual = independent(render(seed));
    expect(actual, format).toEqual(expected);
    expect(Object.hasOwn(actual, "devLaunchers")).toBe(false);
    expect(Object.hasOwn(actual, "projectLaunchers")).toBe(false);
  }
  console.log("[DEBUG] Launch generation preserves declared rows across three neutral seed formats, independently parsed with jsonc-parser");
});
