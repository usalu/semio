/** 🧭️ The shell route classification, actor identity and directory command tests. */
import { expect, it } from "vitest";
import { parseShellRoute, shellActorId, canonicalSurfaceId, reloadRetainsActiveApp, directoryCommandFromAction, mintDirectoryCommandRequestId, retainDirectoryCommandResult, DIRECTORY_COMMAND_RESULT_SLOTS, type DirectoryCommandResultSlotV1, type DirectoryCommandReceiptV1 } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🆔 Registers the shell route, actor id, surface id, reload retention, directory command mapping and correlation nonce tests. */
export function registerShellRoutesAndIdentityTests(): void {
  it("classifies shell routes into landing, space, and notFound", () => {
    expect(parseShellRoute("/")).toEqual({ kind: "landing" });
    expect(parseShellRoute("/spaces/my-studio")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: undefined });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: "inst-1" });
    expect(parseShellRoute("/unknown/path")).toEqual({ kind: "notFound", path: "/unknown/path" });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1/extra")).toEqual({ kind: "notFound", path: "/spaces/my-studio/instances/inst-1/extra" });
  });
  it("shellActorId mints user:{userId}#{sessionId} once identity resolves, else client-{sessionId}", () => {
    expect(shellActorId("sess-1", null)).toBe("client-sess-1");
    expect(shellActorId("sess-1", { userId: "u-1", email: "u1@semio.dev", displayName: "U1", hubBaseUrl: "http://127.0.0.1:8787", issuedAtMs: 0 })).toBe("user:u-1#sess-1");
  });
  it("canonicalSurfaceId formats <kind>@<standard>/<subset>#<role>", () => {
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "editor")).toBe("s.space.space@1/*#editor");
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "viewer")).toBe("s.space.space@1/*#viewer");
  });
  it("reloadRetainsActiveApp accepts extension-only programs and rejects a dropped active app", () => {
    expect(reloadRetainsActiveApp([], undefined)).toBe(true);
    expect(reloadRetainsActiveApp([{ id: "active" }], "active")).toBe(true);
    expect(reloadRetainsActiveApp([], "active")).toBe(false);
  });
  it("directoryCommandFromAction maps all 7 frozen os.directory.* ids, share-link sugaring to create-invite", () => {
    expect(directoryCommandFromAction("os.directory.create-space", { name: "Atelier", spaceKind: "atelier", visibility: "private" })).toEqual({
      kind: "create-space",
      name: "Atelier",
      spaceKind: "atelier",
      visibility: "private",
    });
    expect(directoryCommandFromAction("os.directory.delete-space", { spaceId: "sp-1" })).toEqual({ kind: "delete-space", spaceId: "sp-1" });
    expect(directoryCommandFromAction("os.directory.rename-space", { spaceId: "sp-1", name: "New" })).toEqual({ kind: "rename-space", spaceId: "sp-1", name: "New" });
    expect(directoryCommandFromAction("os.directory.set-visibility", { spaceId: "sp-1", visibility: "public" })).toEqual({ kind: "set-visibility", spaceId: "sp-1", visibility: "public" });
    expect(directoryCommandFromAction("os.directory.upsert-member", { spaceId: "sp-1", email: "a@b.com", role: "author" })).toEqual({
      kind: "upsert-member",
      spaceId: "sp-1",
      email: "a@b.com",
      role: "author",
    });
    expect(directoryCommandFromAction("os.directory.remove-member", { spaceId: "sp-1", userId: "u-1" })).toEqual({ kind: "remove-member", spaceId: "sp-1", userId: "u-1" });
    expect(directoryCommandFromAction("os.directory.share-link", { spaceId: "sp-1", role: "spectator", ttlSecs: 60 })).toEqual({
      kind: "create-invite",
      spaceId: "sp-1",
      role: "spectator",
      ttlSecs: 60,
    });
    expect(directoryCommandFromAction("os.unknownVerb", {})).toBeNull();
  });
  it("mints a 32-hex nonzero correlation and retains bounded, request-id-keyed command results", () => {
    const first = mintDirectoryCommandRequestId();
    expect(first).toMatch(/^(?!0{32}$)[0-9a-f]{32}$/u);
    expect(mintDirectoryCommandRequestId()).not.toBe(first);

    const slots = new Map<string, DirectoryCommandResultSlotV1>();
    const receipt = {
      schema: "semio.directory.command-receipt.v1",
      requestId: first,
      commandSha256: "a".repeat(64),
      outcome: "accepted",
      events: [],
      result: { kind: "invite", inviteToken: "invite.v1.one-shot" },
      receiptSha256: "b".repeat(64),
    } as unknown as DirectoryCommandReceiptV1;
    retainDirectoryCommandResult(slots, first, { kind: "receipt", receipt });
    expect(slots.get(first)).toEqual({ kind: "receipt", receipt });
    retainDirectoryCommandResult(slots, first, { kind: "failed", code: "forbidden" });
    expect(slots.size).toBe(1);
    expect(slots.get(first)).toEqual({ kind: "failed", code: "forbidden" });

    for (let index = 0; index < DIRECTORY_COMMAND_RESULT_SLOTS + 4; index += 1) {
      retainDirectoryCommandResult(slots, index.toString(16).padStart(32, "c"), { kind: "failed", code: "request-conflict" });
    }
    expect(slots.size).toBe(DIRECTORY_COMMAND_RESULT_SLOTS);
    expect(slots.has(first)).toBe(false);
    expect(slots.has((DIRECTORY_COMMAND_RESULT_SLOTS + 3).toString(16).padStart(32, "c"))).toBe(true);
  });
}
