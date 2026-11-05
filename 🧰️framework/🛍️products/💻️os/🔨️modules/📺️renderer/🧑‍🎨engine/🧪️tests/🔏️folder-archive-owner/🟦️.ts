import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import fixture from "./🧫️fixtures/🔣️.json";
import type { DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { readOwnedDocumentArchiveV1, type ArchiveOwnerEntryV1 } from "../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔸️open-document/🔏️archive-owner/🟦️.ts";

const archive = (name: string): DocumentArchivePack => ({ parent_pack: Array.from(new TextEncoder().encode(name)), parent_spr: [], members: [] });

describe("a folder archive is persisted only for the document the instance shows", () => {
  const a: ArchiveOwnerEntryV1 = { documentId: "doc-a", session: { pluginId: "stdio", instanceId: 7 } };
  const b: ArchiveOwnerEntryV1 = { documentId: "doc-b", session: { pluginId: "stdio", instanceId: 7 } };
  it("reads the archive of the document the instance holds", async () => {
    const plugin = { readAppDocumentArchive: async () => archive("a"), readAppDocumentIdentity: async () => ({ parent_document_id: "doc-a" }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", a]]))).toEqual(archive("a"));
  });
  it("two documents on one instance never write one archive under the other id", async () => {
    let reads = 0;
    const plugin = { readAppDocumentArchive: async () => { reads += 1; return archive("b"); }, readAppDocumentIdentity: async () => ({ parent_document_id: "doc-b" }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", a]]))).toBeNull();
    expect(reads).toBe(0);
    expect(await readOwnedDocumentArchiveV1({ readAppDocumentArchive: plugin.readAppDocumentArchive, readAppDocumentIdentity: plugin.readAppDocumentIdentity }, a, "doc-a", new Map([["doc-a", a], ["doc-b", b]]))).toBeNull();
    expect(reads).toBe(0);
  });
  it("fails closed when the instance changes document while the archive is read", async () => {
    let identity = "doc-a";
    const plugin = { readAppDocumentArchive: async () => { identity = "doc-b"; return archive("b"); }, readAppDocumentIdentity: async () => ({ parent_document_id: identity }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", a]]))).toBeNull();
  });
  it("fails closed when the entry was replaced", async () => {
    const plugin = { readAppDocumentArchive: async () => archive("a"), readAppDocumentIdentity: async () => ({ parent_document_id: "doc-a" }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", { ...a }]]))).toBeNull();
  });
  it("agrees with neutral ownership cases and the independent schema validator", async () => {
    expect(new Ajv().validate(fixture.schema, fixture.cases)).toBe(true);
    for (const row of fixture.cases) {
      const plugin = { readAppDocumentArchive: async () => archive("a"), readAppDocumentIdentity: async () => ({ parent_document_id: row.identity }) };
      const sessions = new Map<string, ArchiveOwnerEntryV1>([["doc-a", a]]);
      if (row.otherPlugin !== null) sessions.set("doc-b", { documentId: "doc-b", session: { pluginId: row.otherPlugin, instanceId: 7 } });
      const value = await readOwnedDocumentArchiveV1(plugin, a, "doc-a", sessions);
      expect(value !== null, row.name).toBe(row.expected);
    }
  });
  it("rechecks the captured entry after asynchronous identity lookup", async () => {
    const sessions = new Map([["doc-a", a]]);
    let reads = 0;
    const plugin = { readAppDocumentArchive: async () => { reads += 1; return archive("a"); }, readAppDocumentIdentity: async () => { sessions.set("doc-a", { ...a }); return { parent_document_id: "doc-a" }; } };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", sessions)).toBeNull();
    expect(reads).toBe(0);
  });
  it("refuses without an archive reader", async () => {
    await expect(readOwnedDocumentArchiveV1({ readAppDocumentIdentity: async () => ({ parent_document_id: "doc-a" }) }, a, "doc-a", new Map([["doc-a", a]]))).rejects.toThrow("document-backbone.archive-reader-unavailable");
  });
});
