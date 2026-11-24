/** 🗄️ A staged child reads the receipts its parent adopted into the dataset and derives only what no receipt answers; its own writes stay in its staging. */
import {expect, test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync, mkdtempSync, readdirSync, rmSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createCargoResolutionContextV1, crateIdentityV1, flushCargoReceiptsV1, readCargoResolutionPolicyV1} from "../../🟦️.ts";
import {cargoReceiptNameV1, createCargoReceiptWriterV1} from "../../💾️receipt/🟨️.mjs";
import policyDocument from "../../🔣️.json" with {type: "json"};
import fixture from "../🔣️.json" with {type: "json"};

const policy = readCargoResolutionPolicyV1(policyDocument), queries = fixture.queries.map((query) => [query.slice(0, -2), query.endsWith(":t")] as [string, boolean]);
const host = (files: Record<string, string>, log: string[]) => ({read: async (path: string) => { log.push(path); return path in files ? {value: Bun.TOML.parse(files[path]!) as Record<string, unknown>, sha: createHash("sha256").update(files[path]!).digest("hex")} : null; }, digest: async (path: string) => (path in files ? createHash("sha256").update(files[path]!).digest("hex") : null)});
const publish = (_path: string, _plan: unknown, write: () => void) => { write(); return true; };
const run = async (files: Record<string, string>, directory: string, archive?: string) => {
  const log: string[] = [], context = createCargoResolutionContextV1(host(files, log), policy, createCargoReceiptWriterV1(directory, publish, archive)), ids: Record<string, string> = {};
  for (const [crate, tests] of queries) ids[`${crate}:${tests ? "t" : "b"}`] = await crateIdentityV1(context, crate, tests);
  return {ids, derived: context.counts.derived, parsed: context.counts.parsed, written: await flushCargoReceiptsV1(context), log};
};
const receipts = (directory: string) => existsSync(join(directory, "rr")) ? readdirSync(join(directory, "rr")).sort() : [];

test("a child over an empty staging derives nothing the dataset's receipts answer, writes nothing, and returns the identities the Python-checked fixture names", async () => {
  const dataset = mkdtempSync(join(tmpdir(), "rr-dataset-")), staging = mkdtempSync(join(tmpdir(), "rr-staging-"));
  try {
    const seed = await run(fixture.files, dataset);
    expect(seed.ids).toEqual(fixture.expect.base);
    expect(seed.written).toBeGreaterThan(0);
    const before = receipts(dataset), child = await run(fixture.files, staging, dataset);
    expect({ids: child.ids, derived: child.derived, parsed: child.parsed, written: child.written, read: child.log}).toEqual({ids: fixture.expect.base, derived: 0, parsed: 0, written: 0, read: []});
    expect(receipts(staging)).toEqual([]);
    const alone = await run(fixture.files, mkdtempSync(join(tmpdir(), "rr-alone-")));
    expect(alone.derived).toBe(seed.derived);
    expect(receipts(dataset)).toEqual(before);
  } finally { rmSync(dataset, {recursive: true, force: true}); rmSync(staging, {recursive: true, force: true}); }
});

test("an edit re-derives the moved crate into the staging only; a corrupt archived receipt is no receipt; the staging's own receipt wins", async () => {
  const dataset = mkdtempSync(join(tmpdir(), "rr-dataset-")), staging = mkdtempSync(join(tmpdir(), "rr-staging-"));
  try {
    await run(fixture.files, dataset);
    const before = receipts(dataset), edited = {...fixture.files, "c/Cargo.toml": fixture.files["c/Cargo.toml"] + "\n# edited\n"};
    const moved = await run(edited, staging, dataset);
    expect(moved.derived).toBeGreaterThan(0);
    expect(moved.written).toBeGreaterThan(0);
    expect(receipts(staging).length).toBe(moved.written);
    expect(receipts(dataset)).toEqual(before);
    expect((await run(edited, staging, dataset)).derived).toBe(0);
    writeFileSync(join(dataset, cargoReceiptNameV1("a")), "not a receipt");
    const fresh = mkdtempSync(join(tmpdir(), "rr-staging-")), damaged = await run(fixture.files, fresh, dataset);
    expect(damaged.ids).toEqual(fixture.expect.base);
    expect(damaged.derived).toBeGreaterThan(0);
    expect(receipts(fresh)).toContain(cargoReceiptNameV1("a").slice(3));
    rmSync(fresh, {recursive: true, force: true});
  } finally { rmSync(dataset, {recursive: true, force: true}); rmSync(staging, {recursive: true, force: true}); }
});
