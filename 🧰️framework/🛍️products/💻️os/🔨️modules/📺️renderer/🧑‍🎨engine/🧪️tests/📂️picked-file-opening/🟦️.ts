import { describe, expect, it } from "vitest";
import fixture from "../../../../🚪️io/🧪️tests/🧫️fixtures/🧭️format-resolution/🔣️.json";
import { pickedFileRefusalCodeV1, pickedFileRefusalTextV1, routePickedFileV1, type PickedFileRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts";
import vocabulary from "../../🧱️elements/🏛️ShellHost/📂️picked-file-opening/🔣️.json";
import { REPLAY_REFUSAL_LABELS_V1, localizedLabelTextV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";
import { DOCUMENT_TRANSFER_NOTICE_LABELS_V1, documentTransferNoticeTextV1, type DocumentTransferNoticeV1 } from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { resolveFormatV1, type FormatClaimsV1 } from "../../🧱️elements/🛠️ShellHelpers/🧭️format-resolution/🟦️.ts";

const noHead = async (): Promise<Uint8Array> => new Uint8Array();
const descriptors = fixture.descriptors as readonly FormatClaimsV1[];

describe("picked file routing replays T01 format-resolution fixture", () => {
  for (const row of fixture.cases) {
    it(`matches the Rust law: ${row.id}`, () => {
      const query = row.query as { explicit?: string; fileName?: string; mime?: string };
      if ("error" in row) return expect(() => resolveFormatV1(descriptors, query)).toThrow();
      const resolved = resolveFormatV1(descriptors, query);
      expect(resolved?.format.kindId ?? null).toBe(row.kindId);
      if (row.kindId !== null) expect(resolved?.basis).toBe(row.basis);
      expect(resolved?.mimeDisagreesWith ?? undefined).toBe((row as { mimeDisagreesWith?: string }).mimeDisagreesWith);
    });
  }

});

describe("routePickedFileV1", () => {
  const catalog = { descriptors: () => descriptors, artifactRefOf: (kindId: string) => (kindId === "f.docx" ? "s.stdio.docx@ecma-376/*" : undefined) };

  it("routes a file to its owner dialect without choosing an editor first", async () => {
    expect(await routePickedFileV1(catalog, { name: "Letter.DOCX", type: "application/zip" }, noHead)).toMatchObject({ kind: "routed", artifactRef: "s.stdio.docx@ecma-376/*" });
  });

  it("refuses unresolvable, ownerless, ambiguous and catalog-less files accurately", async () => {
    expect(await routePickedFileV1(catalog, { name: "mystery.q1" }, noHead)).toEqual({ kind: "refused", reason: "format-unresolved" });
    expect(await routePickedFileV1(catalog, { name: "plan.json" }, noHead)).toEqual({ kind: "refused", reason: "owner-unavailable" });
    expect(await routePickedFileV1({ ...catalog, descriptors: () => fixture.ambiguousDescriptors as FormatClaimsV1[] }, { name: "x.dup" }, noHead)).toEqual({ kind: "refused", reason: "format-ambiguous" });
    expect(await routePickedFileV1(undefined, { name: "a.png" }, noHead)).toEqual({ kind: "refused", reason: "format-catalog-unavailable" });
  });

  it("words every refusal key in en and de with the file name and a stable code", () => {
    for (const reason of Object.keys(vocabulary.reasons) as PickedFileRefusalReasonV1[]) {
      const en = pickedFileRefusalTextV1(reason, "a.q1", "en");
      const de = pickedFileRefusalTextV1(reason, "a.q1", "de");
      expect(en).toContain("a.q1");
      expect(de).toContain("a.q1");
      expect(en).not.toBe(de);
      expect(pickedFileRefusalTextV1(reason, "a.q1", "fr")).toBe(`${en} | ${de}`);
      expect(pickedFileRefusalCodeV1(reason)).toBe(`shell.openFile.${reason}`);
    }
  });
});

describe("revision sniff from the shared fixture", () => {
  const sniffs = (fixture as unknown as { revisionSniffs: { revisions: { kindId: string; window: number; contains: string[] }[]; cases: { id: string; owner: string; head: number[]; kindId: string }[] } }).revisionSniffs;
  for (const row of sniffs.cases) {
    it(`routes ${row.id} to the editor of ${row.kindId} and reads at most 4096 bytes`, async () => {
      const heads: number[] = [];
      const catalog = {
        descriptors: () => [{ kindId: row.owner, shortId: "x", aliases: [], mimes: [], extensions: [".sniff"] }] as readonly FormatClaimsV1[],
        artifactRefOf: (kindId: string) => `ref:${kindId}`,
        revisionsOf: () => sniffs.revisions,
      };
      const route = await routePickedFileV1(catalog, { name: "model.sniff" }, async (bytes) => { heads.push(bytes); return Uint8Array.from(row.head); });
      expect(route).toMatchObject({ kind: "routed", artifactRef: `ref:${row.kindId}` });
      expect(heads).toEqual([4096]);
    });
  }

  it("never reads the file when the format has a single revision", async () => {
    let reads = 0;
    const catalog = { descriptors: () => descriptors, artifactRefOf: () => "ref", revisionsOf: () => [] };
    await routePickedFileV1(catalog, { name: "plan.json" }, async () => { reads += 1; return new Uint8Array(); });
    expect(reads).toBe(0);
  });
});

describe("locale matching", () => {
  it("picks the row by the primary language subtag and renders every language for an unknown one", () => {
    const label = { en: "Open", de: "Öffnen" };
    expect(localizedLabelTextV1(label, "de-DE")).toBe("Öffnen");
    expect(localizedLabelTextV1(label, "EN_us")).toBe("Open");
    expect(localizedLabelTextV1(label, "fr-CH")).toBe("Open | Öffnen");
  });

  it("words the open-cancelled notice distinctly from the failure", () => {
    expect(documentTransferNoticeTextV1("open-cancelled", "a.png", "en")).not.toBe(documentTransferNoticeTextV1("file-open-failed", "a.png", "en"));
  });
});

describe("no default language: an unknown locale renders every language, joined", () => {
  it("replay refusals, for every key", () => {
    for (const reason of Object.keys(REPLAY_REFUSAL_LABELS_V1) as ReplayRefusalReasonV1[]) {
      const label = REPLAY_REFUSAL_LABELS_V1[reason];
      expect(replayRefusalNoticeTextV1(reason, "en")).toBe(label.en);
      expect(replayRefusalNoticeTextV1(reason, "de")).toBe(label.de);
      expect(replayRefusalNoticeTextV1(reason, "fr")).toBe(`${label.en} | ${label.de}`);
    }
  });

  it("document transfer notices, for every key", () => {
    for (const notice of Object.keys(DOCUMENT_TRANSFER_NOTICE_LABELS_V1) as DocumentTransferNoticeV1[]) {
      const label = DOCUMENT_TRANSFER_NOTICE_LABELS_V1[notice];
      expect(documentTransferNoticeTextV1(notice, "f.txt", "de")).toBe(label.de.replaceAll("{file}", "f.txt"));
      expect(documentTransferNoticeTextV1(notice, "f.txt", "fr")).toBe(`${label.en} | ${label.de}`.replaceAll("{file}", "f.txt"));
    }
  });
});
