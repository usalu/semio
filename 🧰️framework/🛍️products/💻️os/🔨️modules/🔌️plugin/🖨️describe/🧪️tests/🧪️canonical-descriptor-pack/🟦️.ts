/** 🔤️ TypeScript oracle of the canonical descriptor pack (`canonical_descriptor_value` / `descriptor_pack` in
 * `🛂️descriptor-emission/🦀️.rs`, design §22.19), checked against the language-agnostic fixture
 * `🧫️fixtures/🧫️canonical-descriptor-pack/🔣️.json` that `🧪️tests/🔬️unit/🦀️.rs` runs through the Rust law.
 * The canonical member order is
 * re-derived here from UTF-8 bytes alone, and the bytes come from the pack encoder every descriptor verifier uses. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { decodePackValue, encodePackValue, packValueToExactJson, type PackValue } from "../../../../../🟦️.ts";

type Case = Readonly<{ id: string; authored: PackValue; canonical: PackValue; expectedPackHex: string }>;
type Fixture = Readonly<{ cases: readonly Case[] }>;

const FIXTURE_ROOT = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧫️fixtures/🧫️canonical-descriptor-pack";

/** 🔃️ `value` with the members of every object in UTF-8 key-byte order, recursively. */
export function canonicalDescriptorValue(value: PackValue): PackValue {
  if (Array.isArray(value)) return (value as readonly PackValue[]).map(canonicalDescriptorValue);
  if (value === null || typeof value !== "object") return value;
  return Object.fromEntries(
    Object.entries(value as Record<string, PackValue>)
      .sort((left, right) => Buffer.compare(Buffer.from(left[0], "utf8"), Buffer.from(right[0], "utf8")))
      .map(([key, entry]) => [key, canonicalDescriptorValue(entry)]),
  );
}

/** 📦️ The pack bytes of `value` as lowercase hex. */
export function descriptorPackHex(value: PackValue): string {
  return Buffer.from(encodePackValue(value)).toString("hex");
}

/** ⚖️ Derives every canonical descriptor case through the independent byte oracle; answers the case count. */
export function canonicalDescriptorPackOracle(repoRoot: string): number {
  const fixture: Fixture = JSON.parse(readFileSync(join(repoRoot, FIXTURE_ROOT, "🔣️.json"), "utf8"));
  for (const row of fixture.cases) {
    assert.notEqual(JSON.stringify(row.authored), JSON.stringify(row.canonical), `${row.id}: the authored member order differs from the canonical one`);
    assert.equal(JSON.stringify(canonicalDescriptorValue(row.authored)), JSON.stringify(row.canonical), `${row.id}: canonical member order`);
    assert.equal(descriptorPackHex(row.authored), row.expectedPackHex, `${row.id}: the authored order packs to the sealed bytes`);
    assert.equal(descriptorPackHex(row.canonical), row.expectedPackHex, `${row.id}: the canonical order packs to the sealed bytes`);
    const decoded = decodePackValue(Buffer.from(row.expectedPackHex, "hex"));
    assert.equal(descriptorPackHex(decoded), row.expectedPackHex, `${row.id}: the sealed bytes are canonical for every verifier`);
    assert.deepEqual(packValueToExactJson(decoded), row.authored, `${row.id}: the sealed bytes carry the authored value`);
    assert.equal(JSON.stringify(packValueToExactJson(decoded)), JSON.stringify(row.canonical), `${row.id}: a decoded pack reads in canonical member order`);
  }
  return fixture.cases.length;
}
