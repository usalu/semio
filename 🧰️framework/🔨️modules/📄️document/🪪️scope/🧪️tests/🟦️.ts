/** 🪪️ Frontier laws of the document scope module, judged by independent Ajv schemas built per fixture row. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json" with { type: "json" };
import { artifactFrontierIsEditedForV1, artifactFrontierIsGenesisForV1, type ArtifactFrontier } from "../🟦️.ts";

type Row = (typeof fixture.rows)[number];
const bytes = (chain: Row["frontier"]["chain"]): number[] => Array.from({ length: chain.length }, (_, index) => (chain.overrides as Record<string, number>)[String(index)] ?? chain.fill);
const frontier = (row: Row): ArtifactFrontier => ({ documentId: row.frontier.documentId, headEditOrdinal: row.frontier.headEditOrdinal, headEditId: row.frontier.headEditId, lastCommitSeq: row.frontier.lastCommitSeq, chainHash: bytes(row.frontier.chain) });
const identifier = { type: "string", minLength: 1, maxLength: 256, pattern: "^[^\\u0000-\\u001f\\u007f-\\u009f]+$" };
const hash = (nonzero: boolean) => ({ type: "array", minItems: 32, maxItems: 32, items: { type: "integer", minimum: 0, maximum: 255 }, ...(nonzero ? { contains: { type: "integer", minimum: 1 } } : { items: { const: 0 } }) });
const oracle = (row: Row, kind: "genesis" | "edited") => new Ajv({ strict: true }).compile({
  type: "object",
  required: ["documentId", "headEditOrdinal", "headEditId", "lastCommitSeq", "chainHash"],
  additionalProperties: false,
  properties: kind === "genesis"
    ? { documentId: { const: row.scope.documentId }, headEditOrdinal: { const: 0 }, headEditId: { const: "" }, lastCommitSeq: { const: 0 }, chainHash: hash(false) }
    : { documentId: { const: row.scope.documentId }, headEditOrdinal: { type: "integer", minimum: 1, maximum: Number.MAX_SAFE_INTEGER }, headEditId: identifier, lastCommitSeq: { type: "integer", minimum: 1, maximum: Number.MAX_SAFE_INTEGER }, chainHash: hash(true) },
});
const scoped = (row: Row): boolean => [row.scope.spaceId, row.scope.documentId].every((value) => new Ajv({ strict: true }).validate(identifier, value));

test("genesis and edited frontiers agree with the expected vectors and an independent Ajv oracle", () => {
  expect(fixture.rows.length).toBeGreaterThan(10);
  for (const row of fixture.rows) {
    const value = frontier(row);
    expect(artifactFrontierIsGenesisForV1(row.scope, value), row.label).toBe(row.genesis);
    expect(artifactFrontierIsEditedForV1(row.scope, value), row.label).toBe(row.edited);
    expect(scoped(row) && oracle(row, "genesis")(value), row.label).toBe(row.genesis);
    expect(scoped(row) && oracle(row, "edited")(value), row.label).toBe(row.edited);
  }
});

test("no vector is both genesis and edited", () => {
  expect(fixture.rows.some((row) => row.genesis && row.edited)).toBe(false);
  expect(fixture.rows.filter((row) => row.genesis).length).toBe(1);
  expect(fixture.rows.filter((row) => row.edited).length).toBe(2);
});
