import { describe, expect, it } from "vitest";
import type { DocumentArchivePack } from "@semio-tech/framework/wire-contracts";
import { readOwnedDocumentArchiveV1, type ArchiveOwnerEntryV1 } from "../../🧱️elements/🏛️ShellHost/🗄️shell-uri-documents/🔸️open-document/🔏️archive-owner/🟦️.ts";

const archive = (name: string): DocumentArchivePack => ({ parent_pack: Array.from(new TextEncoder().encode(name)), parent_spr: [], members: [] });

describe("a folder archive is persisted only for the document the instance shows", () => {
  const a: ArchiveOwnerEntryV1 = { documentId: "doc-a", session: { instanceId: 7 } };
  const b: ArchiveOwnerEntryV1 = { documentId: "doc-b", session: { instanceId: 7 } };
  it("reads the archive of the document the instance holds", async () => {
    const plugin = { readAppDocumentArchive: async () => archive("a"), readAppDocumentIdentity: async () => ({ parentDocumentId: "doc-a" }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", a]]))).toEqual(archive("a"));
  });
  it("two documents on one instance never write one archive under the other id", async () => {
    let reads = 0;
    const plugin = { readAppDocumentArchive: async () => { reads += 1; return archive("b"); }, readAppDocumentIdentity: async () => ({ parentDocumentId: "doc-b" }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", a]]))).toBeNull();
    expect(reads).toBe(0);
    expect(await readOwnedDocumentArchiveV1({ readAppDocumentArchive: plugin.readAppDocumentArchive }, a, "doc-a", new Map([["doc-a", a], ["doc-b", b]]))).toBeNull();
    expect(reads).toBe(0);
  });
  it("fails closed when the instance changes document while the archive is read", async () => {
    let identity = "doc-a";
    const plugin = { readAppDocumentArchive: async () => { identity = "doc-b"; return archive("b"); }, readAppDocumentIdentity: async () => ({ parentDocumentId: identity }) };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", a]]))).toBeNull();
  });
  it("fails closed when the entry was replaced", async () => {
    const plugin = { readAppDocumentArchive: async () => archive("a") };
    expect(await readOwnedDocumentArchiveV1(plugin, a, "doc-a", new Map([["doc-a", { ...a }]]))).toBeNull();
  });
  it("refuses without an archive reader", async () => {
    await expect(readOwnedDocumentArchiveV1({}, a, "doc-a", new Map([["doc-a", a]]))).rejects.toThrow("document-backbone.archive-reader-unavailable");
  });
});
