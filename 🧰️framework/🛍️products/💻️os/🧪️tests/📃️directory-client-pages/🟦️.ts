/** 📃️ Directory client pages: the canonical event page and the space administration page.
 *
 * @see ../🧮️directory-client-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it, vi } from "vitest";
import { DirectoryClient } from "@semio-tech/framework/directory-client";
import { fetchWithTimeout } from "@semio-tech/framework";
import { parseDirectorySpaceAdministrationPageV1 } from "@semio-tech/framework/directory/io/text";
import type { DirectoryEvent } from "../🧮️directory-client-helpers/🟦️.ts";

async function sampleCanonicalDirectoryEventPage(after = 3): Promise<string> {
  const unsigned = {
    schema: "semio.directory.event-page.v1" as const,
    sessionBindingSha256: "a".repeat(64),
    authorizationGeneration: 7,
    afterSeqExclusive: after,
    throughSeqInclusive: after + 2,
    hasMore: true,
    events: [] as DirectoryEvent[],
  };
  const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(unsigned))));
  const receiptSha256 = Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
  return JSON.stringify({ ...unsigned, receiptSha256 });
}

describe("DirectoryClient event page", () => {
  it("preserves canonical response text and rejects a substituted request frontier", async () => {
    const canonical = await sampleCanonicalDirectoryEventPage();
    const text = vi.fn(async () => canonical);
    const request = vi.fn(async (..._args: Parameters<typeof fetchWithTimeout>) => ({ ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => { throw new Error("event pages never use response.json"); }, text }));
    const page = await new DirectoryClient("https://hub.test", { request: request as unknown as typeof fetchWithTimeout }).eventPage(3);
    expect(page.canonicalJson).toBe(canonical);
    expect(page.afterSeqExclusive).toBe(3);
    expect(page.throughSeqInclusive).toBe(5);
    expect(text).toHaveBeenCalledTimes(1);
    expect(request.mock.calls[0]?.[0]).toBe("https://hub.test/directory/event-page/v1?after=3");

    const substituted = await sampleCanonicalDirectoryEventPage(4);
    const hostileRequest = vi.fn(async () => ({ ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => substituted }));
    await expect(new DirectoryClient("https://hub.test", { request: hostileRequest as unknown as typeof fetchWithTimeout }).eventPage(3)).rejects.toThrow("response frontier mismatch");

    const abort = new AbortController();
    const cancelledRequest = vi.fn(async () => ({ ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => ({}), text: async () => { abort.abort(new Error("page owner closed")); return canonical; } }));
    await expect(new DirectoryClient("https://hub.test", { request: cancelledRequest as unknown as typeof fetchWithTimeout }).eventPage(3, { signal: abort.signal })).rejects.toThrow("page owner closed");
  });
});

async function sampleCanonicalAdministrationPage(access: "public" | "author", overrides: Record<string, unknown> = {}): Promise<string> {
  const space = access === "public"
    ? { id: "space-1", name: "Public", kind: "studio", visibility: "public", memberCount: 2, documentCount: 1, createdAtMs: 1, updatedAtMs: 2 }
    : { id: "space-1", name: "Authored", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "author", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 };
  const documents = access === "public"
    ? [{ documentId: "document-public", artifactKind: "note.document", artifactSchema: "note.document@1", owner: { pluginId: "note", packageId: "note", version: "1", packageHash: "11".repeat(32) }, packSchemaHash: "22".repeat(32) }]
    : [];
  const base = {
    access,
    schema: "semio.directory.space-administration-page.v1" as const,
    sessionBindingSha256: access === "public" ? "0".repeat(64) : "a".repeat(64),
    authorizationGeneration: access === "public" ? 0 : 7,
    spaceId: "space-1",
    space,
  };
  const unsigned = access === "public"
    ? { ...base, documents: { rows: documents } }
    : {
        ...base,
        members: { rows: [
          { userId: "user-a", email: "a@example.invalid", displayName: "A", role: "author", owner: true },
          { userId: "user-b", email: "b@example.invalid", displayName: "B", role: "spectator", owner: false },
        ] },
        documents: { rows: documents },
        invites: { rows: [{ inviteId: "invite-1", role: "spectator", createdAtMs: 20, expiresAtMs: 900, revoked: false, accepted: false }] },
        capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true },
      };
  const sealed = { ...unsigned, ...overrides };
  const digest = new Uint8Array(await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(JSON.stringify(sealed))));
  const receiptSha256 = Array.from(digest, (byte) => byte.toString(16).padStart(2, "0")).join("");
  return JSON.stringify({ ...sealed, receiptSha256 });
}

function administrationResponse(canonical: string): unknown {
  return { ok: true, status: 200, statusText: "OK", headers: { get: () => "application/json" }, json: async () => { throw new Error("administration pages never use response.json"); }, text: async () => canonical };
}

describe("DirectorySpaceAdministrationPageV1", () => {
  it("keeps invites and capabilities author-only and rejects a substituted receipt", async () => {
    const authored = await sampleCanonicalAdministrationPage("author");
    const page = await parseDirectorySpaceAdministrationPageV1(authored);
    if (page.access !== "author") throw new Error("author discriminator did not narrow");
    expect(page.members.rows.map((row) => row.userId)).toEqual(["user-a", "user-b"]);
    expect(page.members.rows[0]!.owner).toBe(true);
    expect(page.capabilities.removeMember).toBe(true);
    expect(page.invites.rows).toHaveLength(1);
    expect(authored).not.toContain("secretDigest");
    expect(authored).not.toContain("selector");

    const publicPage = await parseDirectorySpaceAdministrationPageV1(await sampleCanonicalAdministrationPage("public"));
    if (publicPage.access !== "public") throw new Error("public discriminator did not narrow");
    expect("members" in publicPage).toBe(false);
    expect("invites" in publicPage).toBe(false);
    expect("capabilities" in publicPage).toBe(false);

    await expect(parseDirectorySpaceAdministrationPageV1(authored.replace(/"receiptSha256":"[0-9a-f]{64}"/u, `"receiptSha256":"${"b".repeat(64)}"`))).rejects.toThrow("receipt-mismatch");
    await expect(parseDirectorySpaceAdministrationPageV1(`${authored} `)).rejects.toThrow();
    await expect(parseDirectorySpaceAdministrationPageV1(authored.replace('"spaceId":"space-1"', '"spaceId":"space-2"'))).rejects.toThrow();
    await expect(parseDirectorySpaceAdministrationPageV1(authored.replace('{"access":"author"', '{"actor":"user:secret","access":"author"'))).rejects.toThrow();
  });

  it("fetches the exact canonical bytes and refuses a foreign space or a bad cursor", async () => {
    const canonical = await sampleCanonicalAdministrationPage("author");
    const request = vi.fn(async (..._args: Parameters<typeof fetchWithTimeout>) => administrationResponse(canonical));
    const client = new DirectoryClient("https://hub.test", { request: request as unknown as typeof fetchWithTimeout });
    const fetched = await client.spaceAdministrationPage("space-1");
    expect(fetched.canonicalJson).toBe(canonical);
    expect(request.mock.calls[0]?.[0]).toBe("https://hub.test/directory/spaces/space-1");

    await client.spaceAdministrationPage("space-1", "m.6162.deadbeef");
    expect(request.mock.calls[1]?.[0]).toBe("https://hub.test/directory/spaces/space-1?cursor=m.6162.deadbeef");
    await expect(client.spaceAdministrationPage("space-1", "not a cursor")).rejects.toThrow("invalid cursor");
    await expect(client.spaceAdministrationPage("space-9")).rejects.toThrow("response space mismatch");
  });
});
