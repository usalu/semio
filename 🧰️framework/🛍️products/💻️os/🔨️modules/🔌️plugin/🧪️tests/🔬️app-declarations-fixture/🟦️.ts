import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import { inspectRustBindingFacts } from "../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/🔗️binding/🟦️.ts";
import { rustTokens, rustTokenPairs } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

type Channel = Readonly<{ snapshot: string; diff: string; mutation: string; leafOwner: string; command: string; editor: string; viewer: string; dialect: string; schema: string }>;
type Fixture = Readonly<{ version: 1; source: string; originalSource: string; originalSha256: string; canonicalHandles: readonly string[]; channels: readonly Channel[] }>;
const owner = resolve(import.meta.dir, "../.."), read = (path: string): unknown => JSON.parse(readFileSync(resolve(owner, path), "utf8"));
const fixture = read("🧫️fixtures/🏗️fixture-channel-interfaces/🔣️.json") as Fixture;
const current = (): string => readFileSync(resolve(import.meta.dir, fixture.source), "utf8");
const tokens = (source: string): readonly string[] => rustTokens(source).map(token => token.text);

/** 🧩️ Selects one authored item by its paired token boundaries without executing an expansion. */
function item(source: string, prefix: string): string {
  const start = source.indexOf(prefix);
  if (start < 0 || source.indexOf(prefix, start + prefix.length) >= 0) throw Error(`Expected one item: ${prefix}`);
  const stream = rustTokens(source), pairs = rustTokenPairs(stream), opening = stream.findIndex(token => token.start >= start && token.text === "{");
  const closing = pairs.get(opening);
  if (closing === undefined) throw Error(`Unpaired item: ${prefix}`);
  return source.slice(start, stream[closing]!.end);
}

/** 🏷️ Substitutes only the original fixture's explicitly declared channel bindings. */
function bind(source: string, row: Channel): string {
  const values: Readonly<Record<string, string>> = { snapshot: row.snapshot, diff: row.diff, mutation: row.mutation, leaf_owner: row.leafOwner, command: row.command, editor: row.editor, viewer: row.viewer, dialect: row.dialect, schema: JSON.stringify(row.schema) };
  return source.replace(/\$(snapshot|diff|mutation|leaf_owner|command|editor|viewer|dialect|schema)\b/gu, (_, name: string) => values[name]!);
}

test("the closed channel roster preserves all three original concrete identities", () => {
  expect(createHash("sha256").update(fixture.originalSource).digest("hex")).toBe(fixture.originalSha256);
  expect(new Set(fixture.channels.map(row => row.snapshot)).size).toBe(3);
  
});

test("every editor and viewer interface is explicit and retains its entire original implementation", () => {
  const source = current(), facts = inspectRustBindingFacts(source);
  expect(facts.imports.filter(row => row.path.at(-1) === "EngineHandles").map(row => ({ path: row.path, alias: row.alias ?? null, glob: row.glob }))).toEqual([{ path: fixture.canonicalHandles, alias: null, glob: false }]);
  for (const row of fixture.channels) for (const [trait, parameter, name] of [["ArtifactEditor", "$editor", row.editor], ["ArtifactViewer", "$viewer", row.viewer]] as const) {
    expect(facts.declarations.filter(declaration => declaration.name === name && declaration.kind === "type" && declaration.blockScope.length === 0).length, name).toBe(1);
    expect(tokens(item(source, `impl ${trait} for ${name}`)), name).toEqual(tokens(bind(item(fixture.originalSource, `impl ${trait} for ${parameter}`), row)));
    expect(tokens(source).join(" ")).toContain(tokens(`#[derive(Default)] pub(crate) struct ${name};`).join(" "));
  }
  for (const problem of facts.problems.filter(row => row.kind === "unproven-macro-output" && row.span)) {
    const body = Buffer.from(source).subarray(problem.span!.start, problem.span!.end).toString();
    expect(rustTokens(body).some(token => token.text === "EngineHandles"), problem.path).toBe(false);
  }
});

test("only interface ownership changes while fixture codecs and all original downstream laws stay intact", () => {
  const source = current(), marker = "//#region 🔖️FixtureChannel", end = "//#endregion 🔖️FixtureChannel";
  for (const side of ["before", "after"] as const) {
    const outside = (text: string): string => side === "before" ? text.slice(0, text.indexOf(marker)) : text.slice(text.indexOf(end) + end.length);
    expect(tokens(outside(source)), side).toEqual(tokens(outside(fixture.originalSource)));
  }
  let expected = item(fixture.originalSource, "macro_rules! fixture_channel");
  const editor = expected.indexOf("#[derive(Default)]"), viewerEnd = expected.lastIndexOf("\n        };");
  if (editor < 0 || viewerEnd < editor) throw Error("Original channel boundaries are unavailable");
  expected = expected.slice(0, editor) + expected.slice(viewerEnd);
  expected = expected.replace(", $editor:ident, $viewer:ident", "");
  expect(tokens(item(source, "macro_rules! fixture_channel"))).toEqual(tokens(expected));
  for (const row of fixture.channels) expect(tokens(source).join(" ")).toContain(tokens(`fixture_channel!(${row.snapshot}, ${row.diff}, ${row.mutation}, ${row.leafOwner}, ${row.command}, ${row.dialect}, ${JSON.stringify(row.schema)});`).join(" "));
});
