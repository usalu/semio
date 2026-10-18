/** 🧷️ Proves optional member slug patterns resolve identically in both taxonomy engines and an independent emoji-regex oracle. */
import { afterAll, expect, test } from "bun:test";
import { mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import emojiRegex from "emoji-regex";
import { semanticDirectoryKindId, validateTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { type Taxonomy } from "../../🔍️discovery/📑️taxonomy-document/🟦️.ts";
import { matchDirectoryKind } from "../../🧹️normalization/🏷️file-kind-matching/🟦️.ts";
import { loadTaxonomy } from "../../🧹️normalization/🔣️taxonomy/🟦️.ts";

type Row = Readonly<{ ownerKindIds: readonly string[]; memberNames: readonly string[]; memberSlugPattern?: unknown; source: "registry" }>;
type Kind = Readonly<{ emoji: string; slugPattern: string; allowEmojiOnly: boolean; parentKindIds?: readonly string[] }>;
type Case = Readonly<{ id: string; name: string; parentKindId?: string; ancestorKindIds: readonly string[]; kindId: string | null; ambiguous: readonly string[] }>;
type Vector = Readonly<{ schemaVersion: number; contractId: string; overlay: { semanticDirectoryKinds: Record<string, Kind>; semanticDirectoryMemberKinds: Record<string, Row> }; cases: Case[]; invalidRows: { id: string; row: Row; problem: string }[] }>;
type Document = Record<string, unknown> & { semanticDirectoryKinds: Record<string, Kind>; semanticDirectoryMemberKinds: Record<string, Row> };

const library = resolve(import.meta.dir, "../..");
const vector: Vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🧷️member-slug-pattern/🔣️.json"), "utf8"));
const roots: string[] = [];
afterAll(() => { for (const root of roots) rmSync(root, { recursive: true, force: true }); });

function merged(extra: Record<string, Row> = {}): Document {
  const document = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8")) as Document;
  Object.assign(document.semanticDirectoryKinds, vector.overlay.semanticDirectoryKinds);
  Object.assign(document.semanticDirectoryMemberKinds, vector.overlay.semanticDirectoryMemberKinds, extra);
  return document;
}

function stored(document: Document): { repoRoot: string; taxonomyPath: string } {
  const repoRoot = realpathSync(mkdtempSync(join(tmpdir(), "slugpat-")));
  roots.push(repoRoot);
  writeFileSync(join(repoRoot, "🔣️taxonomy.json"), JSON.stringify(document));
  return { repoRoot, taxonomyPath: "🔣️taxonomy.json" };
}

const document = merged();
const discovery = document as unknown as Taxonomy;
const normalization = loadTaxonomy(stored(document));
const fold = (value: string): string => value.normalize("NFC").replaceAll("️", "");

function nearest(rows: readonly [string, Row][], contexts: readonly string[]): { kindId: string | null; ambiguous: string[] } | null {
  const ranked = rows.map(([id, row]) => ({ id, distance: contexts.findIndex((context) => row.ownerKindIds.includes(context)) })).filter((entry) => entry.distance >= 0).sort((left, right) => left.distance - right.distance || left.id.localeCompare(right.id));
  if (ranked.length === 0) return null;
  const closest = ranked.filter((entry) => entry.distance === ranked[0]!.distance);
  return closest.length === 1 ? { kindId: closest[0]!.id, ambiguous: [] } : { kindId: null, ambiguous: closest.map((entry) => entry.id) };
}

function oracle(row: Case): { kindId: string | null; ambiguous: string[] } {
  const lead = emojiRegex().exec(row.name);
  if (!lead || lead.index !== 0) return { kindId: null, ambiguous: [] };
  const emoji = row.name.startsWith("\uFE0F", lead[0].length) ? `${lead[0]}\uFE0F` : lead[0];
  const slug = row.name.slice(emoji.length), contexts = [row.parentKindId, ...row.ancestorKindIds].filter((id): id is string => Boolean(id));
  const structural = Object.entries(document.semanticDirectoryKinds).filter(([, kind]) => fold(kind.emoji) === fold(emoji) && new RegExp(kind.slugPattern, "u").test(slug) && (!kind.parentKindIds || (row.parentKindId !== undefined && kind.parentKindIds.includes(row.parentKindId))));
  if (structural.length === 1) return { kindId: structural[0]![0], ambiguous: [] };
  const members = Object.entries(document.semanticDirectoryMemberKinds);
  const explicit = nearest(members.filter(([, member]) => member.memberNames.some((name) => fold(name) === fold(row.name))), contexts);
  if (explicit) return explicit;
  const pattern = nearest(members.filter(([, member]) => typeof member.memberSlugPattern === "string" && new RegExp(member.memberSlugPattern, "u").test(slug)), contexts);
  return pattern ?? { kindId: null, ambiguous: [] };
}

test("the language-neutral vector has closed identity and unique case ids", () => {
  expect(vector.schemaVersion).toBe(1);
  expect(vector.contractId).toBe("taxonomy-member-slug-pattern-v1");
  expect(new Set(vector.cases.map((row) => row.id)).size).toBe(vector.cases.length);
  expect(new Set(vector.invalidRows.map((row) => row.id)).size).toBe(vector.invalidRows.length);
  expect(validateTaxonomy(discovery)).toEqual([]);
});

test("pattern rows resolve by scope, precedence and ambiguity in the discovery resolver", () => {
  for (const row of vector.cases) expect(semanticDirectoryKindId(row.name, discovery, { parentKindId: row.parentKindId, ancestorKindIds: row.ancestorKindIds }), row.id).toBe(row.kindId);
});

test("pattern rows resolve by scope, precedence and ambiguity in the normalization matcher", () => {
  for (const row of vector.cases) {
    const match = matchDirectoryKind(row.name, normalization, row.parentKindId, row.ancestorKindIds);
    expect(match.kind?.id ?? null, row.id).toBe(row.kindId);
    expect([...match.ambiguous].sort(), row.id).toEqual([...row.ambiguous]);
  }
});

test("an independent emoji-regex oracle reproduces every vector verdict", () => {
  for (const row of vector.cases) expect(oracle(row), row.id).toEqual({ kindId: row.kindId, ambiguous: [...row.ambiguous] });
});

test("the loader refuses malformed or colliding pattern rows with the declared discovery problem", () => {
  for (const invalid of vector.invalidRows) expect(() => loadTaxonomy(stored(merged({ "slugpat-invalid": invalid.row }))), invalid.id).toThrow(invalid.problem);
}, 120_000);
