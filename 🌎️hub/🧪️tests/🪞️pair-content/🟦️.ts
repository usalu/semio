// #region Header
/**
 * 🪞️ Law of `semio.hub.pair-content/v1`, the content digest the residency watch compares a kind's documents by
 * (`🧪️tests/🧠️residency`, fixture `🧫️fixtures/🪞️pair-content-v1`). The Rust twin
 * `the_pair_content_digest_is_the_fixtures_under_the_production_decoder` (`🛰️lag-rebootstrap`) reads the same streams with
 * the production pair and SPR decoders and hashes them with the repository's own SHA-256; this law hashes with node:crypto,
 * the third-party oracle. Both must reach the fixture's digests.
 */
// #endregion Header

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { pairContentDigest, sprFrames } from "../🧠️residency/🟦️.ts";

type FixturePair = { name: string; kindId: string; documentId: string; streamHex: string; sprFrames: { kind: number; flags: number; rawLength: number | null; payloadLength: number }[]; contentDigest: string };
type Fixture = { pairs: FixturePair[]; sameContent: string[][]; differentContent: string[][] };

const fixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🪞️pair-content-v1/🔣️.json"), "utf8")) as Fixture;
const pair = (name: string) => fixture.pairs.find((entry) => entry.name === name)!;
const stream = (entry: FixturePair) => Buffer.from(entry.streamHex, "hex");

/** 🧭️ The absolute stream offset where the SPR part (data record part 2) starts, and its length. */
function sprSpan(bytes: Buffer): { start: number; length: number } {
  for (let at = 0; at + 4 <= bytes.length; at += 4 + bytes.readUInt32BE(at)) {
    if (bytes[at + 4] === 2 && bytes[at + 5] === 2) return { start: at + 4 + 18, length: bytes.readUInt32BE(at) - 18 };
  }
  throw new Error("no SPR part");
}

function flipped(bytes: Buffer, at: number): Buffer {
  const copy = Buffer.from(bytes);
  copy[at] = copy[at]! ^ 0xff;
  return copy;
}

describe("🪞️ pair content digest", () => {
  it("reads every SPR frame the production decoder reads", () => {
    for (const entry of fixture.pairs) {
      const bytes = stream(entry);
      const { start, length } = sprSpan(bytes);
      expect(sprFrames(bytes.subarray(start, start + length)).map((frame) => ({ kind: frame.kind, flags: frame.flags, rawLength: frame.rawLength, payloadLength: frame.payload.length })), entry.name).toEqual(entry.sprFrames);
    }
  });

  it("is the fixture's digest for every captured pair", () => {
    for (const entry of fixture.pairs) expect(pairContentDigest(stream(entry), entry.documentId), entry.name).toBe(entry.contentDigest);
  });

  it("agrees for two creations of one kind and differs between kinds", () => {
    for (const group of fixture.sameContent) expect(new Set(group.map((name) => pairContentDigest(stream(pair(name)), pair(name).documentId))).size, group.join(" ")).toBe(1);
    for (const [left, right] of fixture.differentContent) expect(pairContentDigest(stream(pair(left!)), pair(left!).documentId)).not.toBe(pairContentDigest(stream(pair(right!)), pair(right!).documentId));
  });

  it("ignores exactly the derived integrity fields and sees every content byte", () => {
    const entry = pair("note-first");
    const bytes = stream(entry);
    const { start, length } = sprSpan(bytes);
    const commit = start + length - 75;
    const digest = (changed: Buffer) => pairContentDigest(changed, entry.documentId);
    expect(digest(flipped(bytes, commit + 3 + 32)), "commit chain hash").toBe(entry.contentDigest);
    expect(digest(flipped(bytes, commit + 67)), "commit CRC-32C").toBe(entry.contentDigest);
    expect(digest(flipped(bytes, start + 32 + 1 + 61)), "first record CRC-32C").toBe(entry.contentDigest);
    expect(digest(flipped(bytes, commit + 3)), "commit sequence").not.toBe(entry.contentDigest);
    expect(digest(flipped(bytes, start + 32 + 1 + 2 + 20)), "first record payload").not.toBe(entry.contentDigest);
    expect(digest(flipped(bytes, start - 18 - 4 - 100)), "pack byte").not.toBe(entry.contentDigest);
  });

  it("refuses a stream whose frame does not echo its own length", () => {
    const bytes = stream(pair("drawing-first"));
    const { start, length } = sprSpan(bytes);
    const spr = Buffer.from(bytes.subarray(start, start + length));
    spr[spr.length - 1] = spr[spr.length - 1]! ^ 0x01;
    expect(() => sprFrames(spr)).toThrow(/echo/u);
  });
});
