/** 🔁️ LAW: a program that mints a fresh document identity on every boot (the sourcing curation editor) is offered its own
 * remembered folder after a reload, and the reconnect adopts the remembered document. Language-agnostic corpus `🔣️.json`.
 * @see ../../🟦️.tsx */
import { describe, expect, it } from "bun:test";
import corpus from "./🔣️.json" with { type: "json" };
import { syncAttachDocumentIdV1 } from "../../../../🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import { localFolderReconnectOfferV1 } from "../../🟦️.tsx";
import type { LocalFolderBinding } from "@semio-tech/framework/config/mutations";

const bindings = { bindings: corpus.bindings as readonly LocalFolderBinding[] };

describe("the reconnect offer of a program with a fresh identity", () => {
  for (const row of corpus.offers) {
    it(row.name, () => {
      const program = "program" in row ? row.program : corpus.program;
      const offer = localFolderReconnectOfferV1(bindings, { documentId: row.identity, ...program }, new Set(row.attached));
      expect(offer?.documentId ?? null).toBe(row.offer);
    });
  }

  it("no identity is offered nothing", () => {
    expect(localFolderReconnectOfferV1(bindings, null, new Set())).toBeNull();
  });
});

describe("the document a sync attach addresses", () => {
  for (const row of corpus.attachTargets) {
    it(row.name, () => {
      const target = "documentId" in row.target ? { kind: "folder" as const, documentId: row.target.documentId } : { kind: "folder" as const };
      expect(syncAttachDocumentIdV1(target, row.identity === null ? null : { parent_document_id: row.identity })).toBe(row.documentId);
    });
  }
});
