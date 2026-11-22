import { expect, test } from "bun:test";
import corpus from "../🧫️fixtures/🎒️pack-dynamic-bytes-v1/🔣️.json";
import { decodePackValue, encodePackValue } from "../../../../🔨️modules/🎁️packs/🟦️.ts";

const hex = (bytes: Uint8Array) => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
const bytes = (text: string) => Uint8Array.from(text.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));

test("Pack carries a Uint8Array as TAG_BYTES exactly as the neutral corpus frames it", () => {
  expect(corpus.schema).toBe("semio.pack.dynamic-bytes/v1");
  for (const row of corpus.cases) {
    const payload = bytes(row.bytesHex);
    expect(hex(encodePackValue(payload)), row.id).toBe(row.topLevelWireHex);
    const top = decodePackValue(bytes(row.topLevelWireHex));
    expect(top instanceof Uint8Array && hex(top), row.id).toBe(row.bytesHex);
    expect(hex(encodePackValue({ kind: "x", archive: payload })), row.id).toBe(row.keyedWireHex);
    const keyed = decodePackValue(bytes(row.keyedWireHex)) as { archive: Uint8Array; kind: string };
    expect(keyed.kind).toBe("x");
    expect(hex(keyed.archive), row.id).toBe(row.bytesHex);
  }
});

test("a 9.6 MB archive crosses the Pack wire without expanding into a number array", () => {
  const archive = new Uint8Array(9_600_000).map((_, index) => (index * 31 + 7) & 255);
  const started = performance.now();
  const wire = encodePackValue({ kind: "localDocumentArchive", archive });
  const decoded = decodePackValue(wire) as { archive: Uint8Array };
  expect(decoded.archive instanceof Uint8Array && decoded.archive.length === archive.length && decoded.archive.every((byte, index) => byte === archive[index])).toBe(true);
  expect(performance.now() - started).toBeLessThan(3000);
});
