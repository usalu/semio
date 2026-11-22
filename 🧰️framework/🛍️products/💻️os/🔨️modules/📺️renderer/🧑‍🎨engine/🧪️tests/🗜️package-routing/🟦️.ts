// @vitest-environment node
import { strToU8, unzipSync, zipSync, strFromU8 } from "fflate";
import { describe, expect, it } from "vitest";
import fixture from "./🧫️fixtures/🔣️.json";
import { openPickedFileV1, pickedFileRefusalCodeV1, pickedFileRefusalTextV1, readPickedFileRangeV1, routePickedFileV1, type PickedFileFlowPortV1, type PickedFileOutcomeV1 } from "../../🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts";
import { inspectPackageV1, PACKAGE_SNIFF_LIMITS_V1, type PackageClaimV1, type PackageSourceV1 } from "../../🧱️elements/🛠️ShellHelpers/🗜️package-sniff/🟦️.ts";
import type { FormatClaimsV1 } from "../../🧱️elements/🛠️ShellHelpers/🧭️format-resolution/🟦️.ts";

type Case = { id: string; entries?: Record<string, string>; raw?: string; padPart?: { path: string; bytes: number }; cutTail?: number; verdict: { kind: string; artifactRef?: string | null } };
const claim = fixture.claim as PackageClaimV1;
const cases = fixture.cases as readonly Case[];
const sourceOf = (bytes: Uint8Array): PackageSourceV1 => ({ size: bytes.length, read: async (start, end) => bytes.slice(start, end) });
const idle = new AbortController().signal;

function archiveOf(row: Case, level: 0 | 9): Uint8Array {
  if (row.raw !== undefined) return Uint8Array.from(Array.from(row.raw, (character) => character.charCodeAt(0)));
  const entries = Object.fromEntries(Object.entries(row.entries ?? {}).map(([name, text]) => [name, strToU8(row.padPart?.path === name ? `${text}<!-- ${"x".repeat(row.padPart.bytes)} -->` : text)]));
  const bytes = zipSync(entries, { level });
  return row.cutTail === undefined ? bytes : bytes.slice(0, bytes.length - row.cutTail);
}

describe("package content check against the fflate reference writer", () => {
  for (const level of [0, 9] as const) {
    for (const row of cases) {
      it(`${row.id} (${level === 0 ? "stored" : "deflated"})`, async () => {
        const bytes = archiveOf(row, level);
        if (row.entries !== undefined && row.cutTail === undefined) {
          const reference = unzipSync(bytes);
          expect(Object.keys(reference).sort()).toEqual(Object.keys(row.entries).sort());
          for (const [name, text] of Object.entries(row.entries)) expect(strFromU8(reference[name]!).startsWith(text.slice(0, 40))).toBe(true);
        }
        expect(await inspectPackageV1(sourceOf(bytes), claim, idle)).toEqual(row.verdict);
      });
    }
  }

  it("reads only the zip tail, the central directory and two entries, never the whole archive", async () => {
    const row = cases.find((candidate) => candidate.padPart !== undefined)!;
    const bytes = archiveOf(row, 0);
    const reads: number[] = [];
    const source: PackageSourceV1 = { size: bytes.length, read: async (start, end) => { reads.push(end - start); return bytes.slice(start, end); } };
    expect(await inspectPackageV1(source, claim, idle)).toEqual(row.verdict);
    expect(bytes.length).toBeGreaterThan(200000);
    expect(reads.reduce((sum, length) => sum + length, 0)).toBeLessThan(PACKAGE_SNIFF_LIMITS_V1.tailBytes + PACKAGE_SNIFF_LIMITS_V1.rootBytes + 4096);
  });

  it("refuses content types larger than the bound instead of reading on", async () => {
    const huge = `<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><!-- ${"x".repeat(PACKAGE_SNIFF_LIMITS_V1.contentTypesBytes + 1)} --></Types>`;
    const bytes = zipSync({ "[Content_Types].xml": strToU8(huge), "xl/workbook.xml": strToU8("<workbook/>") }, { level: 9 });
    expect(await inspectPackageV1(sourceOf(bytes), claim, idle)).toEqual({ kind: "mismatch" });
  });

  it("honours cancellation instead of reporting a mismatch", async () => {
    const abort = new AbortController();
    abort.abort(new DOMException("cancelled", "AbortError"));
    await expect(inspectPackageV1(sourceOf(archiveOf(cases[0]!, 9)), claim, abort.signal)).rejects.toThrow("cancelled");
  });
});

describe("routePickedFileV1 with a package claim", () => {
  const descriptors: readonly FormatClaimsV1[] = [{ kindId: "f.sheet", shortId: "sheet", aliases: [], mimes: ["application/x-sheet"], extensions: [".sheet"] }];
  const catalog = { descriptors: () => descriptors, artifactRefOf: () => "owner@ecma/*", packageOf: (kindId: string) => (kindId === "f.sheet" ? claim : undefined) };
  const route = (row: Case) => routePickedFileV1(catalog, { name: "Budget.SHEET" }, async () => new Uint8Array(), { source: sourceOf(archiveOf(row, 9)), signal: idle });
  const byId = (id: string) => cases.find((candidate) => candidate.id === id)!;

  it("routes Strict and Transitional workbooks to their own subset editor and an undeclared namespace to the owner", async () => {
    expect(await route(byId("strict workbook"))).toMatchObject({ kind: "routed", artifactRef: "s.stdio.xlsx@ecma-376/strict" });
    expect(await route(byId("transitional workbook"))).toMatchObject({ kind: "routed", artifactRef: "s.stdio.xlsx@ecma-376/transitional" });
    expect(await route(byId("workbook in an undeclared namespace is still a workbook"))).toMatchObject({ kind: "routed", artifactRef: "owner@ecma/*" });
  });

  it("refuses content that contradicts the name before anything is created", async () => {
    for (const id of ["a word processing package is no workbook", "plain text is no zip", "zip without content types"]) expect(await route(byId(id))).toEqual({ kind: "refused", reason: "content-mismatch" });
  });

  it("keeps routing by owner when the host supplies no content", async () => {
    expect(await routePickedFileV1(catalog, { name: "Budget.sheet" }, async () => new Uint8Array())).toMatchObject({ kind: "routed", artifactRef: "owner@ecma/*" });
  });

  it("never reads content for a format without a package claim", async () => {
    let reads = 0;
    const plain = { ...catalog, packageOf: () => undefined };
    await routePickedFileV1(plain, { name: "Budget.sheet" }, async () => new Uint8Array(), { source: { size: 1, read: async () => { reads += 1; return new Uint8Array(); } }, signal: idle });
    expect(reads).toBe(0);
  });

  it("words the content refusal in en and de with the file name, a stable code and a distinct text per language", () => {
    const en = pickedFileRefusalTextV1("content-mismatch", "Budget.xlsx", "en");
    const de = pickedFileRefusalTextV1("content-mismatch", "Budget.xlsx", "de");
    expect(en).toContain("Budget.xlsx");
    expect(de).toContain("Budget.xlsx");
    expect(en).not.toBe(de);
    expect(pickedFileRefusalTextV1("content-mismatch", "Budget.xlsx", "fr")).toBe(`${en} | ${de}`);
    expect(pickedFileRefusalCodeV1("content-mismatch")).toBe("shell.openFile.content-mismatch");
  });
});

describe("the open flow on a content mismatch", () => {
  it("creates no instance, refuses with the content reason and releases the slot", async () => {
    const outcomes: PickedFileOutcomeV1[] = [];
    const log: string[] = [];
    const picked = { name: "Fake.xlsx" };
    const port: PickedFileFlowPortV1<{ id: string }, typeof picked> = {
      select: async () => picked,
      claim: () => true,
      release: () => { log.push("release"); },
      track: () => ({ signal: idle, progress: () => undefined, finish: () => { log.push("finish"); } }),
      route: async () => ({ kind: "refused", reason: "content-mismatch" }),
      resolveOwner: async () => { log.push("resolveOwner"); return { id: "o" }; },
      importInto: async () => { log.push("create"); return 1; },
      commit: () => true,
      destroy: async () => undefined,
      report: () => undefined,
      notify: (outcome) => { outcomes.push(outcome); },
    };
    await openPickedFileV1(port);
    expect(outcomes).toEqual([{ kind: "refused", reason: "content-mismatch", name: "Fake.xlsx" }]);
    expect(log).toEqual(["finish", "release"]);
  });
});

describe("readPickedFileRangeV1", () => {
  it("reads exactly the requested range and settles at once on cancel", async () => {
    const blob = new Blob([Uint8Array.from([0, 1, 2, 3, 4, 5])]);
    expect([...await readPickedFileRangeV1(blob, 2, 5, idle)]).toEqual([2, 3, 4]);
    const abort = new AbortController();
    abort.abort(new DOMException("stop", "AbortError"));
    await expect(readPickedFileRangeV1(blob, 0, 1, abort.signal)).rejects.toThrow("stop");
  });
});
