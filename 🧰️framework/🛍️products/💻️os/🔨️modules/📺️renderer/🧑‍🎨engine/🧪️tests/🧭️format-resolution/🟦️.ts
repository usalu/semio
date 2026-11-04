/** 🧭️ The TypeScript resolver agrees with the Rust one on the shared `format-resolution` fixture. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { FormatResolutionErrorV1, pickRevisionV1, resolveFormatV1, type RevisionSniffV1, type FormatClaimsV1, type FormatQueryV1 } from "../../🧱️elements/🛠️ShellHelpers/🧭️format-resolution/🟦️.ts";

type Row = { kindId: string; shortId: string; aliases: string[]; mimes: string[]; extensions: string[] };
type Fixture = {
  descriptors: Row[];
  cases: { id: string; query: Record<string, string | null>; kindId?: string | null; basis?: string; mimeDisagreesWith?: string; error?: string }[];
  ambiguousDescriptors: Row[];
  revisionSniffs: { revisions: RevisionSniffV1[]; cases: { id: string; owner: string; head: number[]; kindId: string }[] };
  ambiguousCase: { query: FormatQueryV1; claim: string; kindIds: string[] };
};
const fixture = JSON.parse(readFileSync(new URL("../../../../🚪️io/🧪️tests/🧫️fixtures/🧭️format-resolution/🔣️.json", import.meta.url), "utf8")) as Fixture;

describe("format resolution twin", () => {
  it("resolves every shared fixture case with the Rust precedence", () => {
    for (const row of fixture.cases) {
      if (row.error !== undefined) {
        let thrown: unknown = null;
        try {
          resolveFormatV1(fixture.descriptors as FormatClaimsV1[], row.query as FormatQueryV1);
        } catch (error) {
          thrown = error;
        }
        expect(thrown, row.id).toBeInstanceOf(FormatResolutionErrorV1);
        expect((thrown as FormatResolutionErrorV1).code, row.id).toBe(row.error);
        continue;
      }
      const resolved = resolveFormatV1(fixture.descriptors as FormatClaimsV1[], row.query as FormatQueryV1);
      expect(resolved?.format.kindId ?? null, row.id).toBe(row.kindId ?? null);
      if (resolved !== null) {
        expect(resolved.basis, row.id).toBe(row.basis);
        expect(resolved.mimeDisagreesWith, row.id).toBe(row.mimeDisagreesWith ?? null);
      }
    }
  });

  it("refuses a claim held by two formats", () => {
    const { query, claim, kindIds } = fixture.ambiguousCase;
    try {
      resolveFormatV1(fixture.ambiguousDescriptors as FormatClaimsV1[], query);
      throw new Error("expected ambiguity");
    } catch (error) {
      expect(error).toBeInstanceOf(FormatResolutionErrorV1);
      expect((error as FormatResolutionErrorV1).claim).toBe(claim);
      expect((error as FormatResolutionErrorV1).kindIds).toEqual(kindIds);
    }
  });

  it("picks a revision by its declared header and otherwise the owner", () => {
    for (const row of fixture.revisionSniffs.cases) expect(pickRevisionV1(row.owner, fixture.revisionSniffs.revisions, Uint8Array.from(row.head)), row.id).toBe(row.kindId);
  });
});
