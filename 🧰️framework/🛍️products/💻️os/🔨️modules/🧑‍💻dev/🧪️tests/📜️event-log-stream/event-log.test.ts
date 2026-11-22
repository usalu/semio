import { expect, test } from "bun:test";
import { appendFileSync, mkdtempSync, rmSync, statSync, truncateSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import largeLaw from "../../../📡️spr/🧵️channel/🧪️tests/🧫️fixtures/📦️large-archive/🔣️.json";
import { appendArchive, eventLogChecksum, eventLogPath, readLatestArchive } from "../../🔌️vite-plugins/🗄️backbone-storage/📜️event-log/🟦️.ts";
import { encodeDocumentArchiveBytes } from "../../../../../../🔨️modules/🪢️wire-contracts/🟦️.ts";

const MASK = (1n << 64n) - 1n;
const splitmix = (seed: string, length: number) => {
  const out = new Uint8Array(Math.ceil(length / 8) * 8), view = new DataView(out.buffer);
  let state = BigInt(seed);
  for (let at = 0; at < out.length; at += 8) {
    state = (state + 0x9E3779B97F4A7C15n) & MASK;
    let z = ((state ^ (state >> 30n)) * 0xBF58476D1CE4E5B9n) & MASK;
    z = ((z ^ (z >> 27n)) * 0x94D049BB133111EBn) & MASK;
    view.setBigUint64(at, (z ^ (z >> 31n)) & MASK, true);
  }
  return out.subarray(0, length);
};

test("the streamed folder event log hashes, appends and reads a large archive exactly as the neutral law frames it", () => {
  const row = largeLaw.cases[0]!;
  const archive = encodeDocumentArchiveBytes({ parent_pack: splitmix(row.packSeed, row.packBytes), parent_spr: splitmix(row.sprSeed, row.sprBytes), members: [] });
  expect(eventLogChecksum(archive).toString(16).padStart(16, "0")).toBe(row.fnv1a64);
  const folder = mkdtempSync(join(tmpdir(), "semio-event-log-"));
  try {
    const path = eventLogPath(folder);
    appendArchive(path, "doc-1", "s.test", archive);
    expect(readLatestArchive(path, "doc-1")).toEqual(archive);
    const small = encodeDocumentArchiveBytes({ parent_pack: [1, 2, 3], parent_spr: [4], members: [] });
    appendArchive(path, "doc-2", "s.test", small);
    expect(readLatestArchive(path, "doc-2")).toEqual(small);
    expect(readLatestArchive(path, "doc-1")).toEqual(archive);
    expect(readLatestArchive(path, "absent")).toBeNull();
    const intact = statSync(path).size;
    appendFileSync(path, Buffer.from("SEMIOEL1-interrupted"));
    appendArchive(path, "doc-2", "s.test", small);
    expect(readLatestArchive(path, "doc-1")).toEqual(archive);
    expect(statSync(path).size).toBeGreaterThan(intact);
    truncateSync(path, intact - 5);
    expect(readLatestArchive(path, "doc-1")).toEqual(archive);
  } finally {
    rmSync(folder, { recursive: true, force: true });
  }
});
