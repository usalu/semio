import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  decodeBackboneWorkerRequest,
  decodeBackboneWorkerResponse,
  encodeBackboneWorkerRequest,
  encodeBackboneWorkerResponse,
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type DirectorySpaceAdministrationPageV1,
} from "@semio-tech/framework-os";
import {
  copyDirectoryInviteCapabilityV1,
  reduceShellSpaceAdministrationState,
  shellSpaceAdministrationRequest,
  type ShellSpaceAdministrationStateV1,
} from "../../../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import {
  SpaceAdministrationPane,
  spaceAdministrationCapabilities,
  spaceAdministrationDispatchable,
  spaceAdministrationInviteRevocable,
  spaceAdministrationMemberRemovable,
  type SpaceAdministrationIntentV1,
} from "../../../../🧱️elements/🛂️SpaceAdministration/🟦️.tsx";

afterEach(cleanup);

const SPACE = "space-admin-01";

const authorPage = (): DirectorySpaceAdministrationPageV1 => ({
  access: "author",
  schema: "semio.directory.space-administration-page.v1",
  sessionBindingSha256: "a".repeat(64),
  authorizationGeneration: 5,
  spaceId: SPACE,
  space: { id: SPACE, name: "Administered", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "author", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 },
  members: { rows: [
    { userId: "user-a", email: "a@example.invalid", displayName: "Ada", role: "author", owner: true },
    { userId: "user-b", email: "b@example.invalid", displayName: "Bo", role: "spectator", owner: false },
  ] },
  documents: { rows: [] },
  invites: { rows: [
    { inviteId: "invite-live", role: "spectator", createdAtMs: 400, expiresAtMs: 900000, revoked: false, accepted: false },
    { inviteId: "invite-dead", role: "spectator", createdAtMs: 200, expiresAtMs: 900000, revoked: true, accepted: false },
  ] },
  capabilities: { renameSpace: true, setVisibility: true, deleteSpace: true, upsertMember: true, removeMember: true, createInvite: true, revokeInvite: true },
  receiptSha256: "b".repeat(64),
});

const memberPage = (): DirectorySpaceAdministrationPageV1 => ({
  access: "member",
  schema: "semio.directory.space-administration-page.v1",
  sessionBindingSha256: "a".repeat(64),
  authorizationGeneration: 5,
  spaceId: SPACE,
  space: { id: SPACE, name: "Administered", kind: "studio", visibility: "private", ownerUserId: "user-a", role: "spectator", memberCount: 2, documentCount: 0, activeConnections: 0, createdAtMs: 1, updatedAtMs: 2 },
  members: { rows: [{ userId: "user-b", email: "b@example.invalid", displayName: "Bo", role: "spectator", owner: false }] },
  documents: { rows: [] },
  receiptSha256: "c".repeat(64),
});

const state = (page: DirectorySpaceAdministrationPageV1 | null, phase: ShellSpaceAdministrationStateV1["phase"] = "ready"): ShellSpaceAdministrationStateV1 => ({ operationEpoch: 1, spaceId: SPACE, phase, page });

const workerState = (patch: Partial<Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }>> = {}): Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }> =>
  ({ kind: "directory-administration-state", operationEpoch: 1, spaceId: SPACE, phase: "ready", ...patch }) as Extract<BackboneWorkerResponse, { kind: "directory-administration-state" }>;

describe("ShellHost space administration state", () => {
  it("keeps a superseded operation's state and erases page and receipt on every terminal phase", () => {
    const current = state(authorPage());
    expect(reduceShellSpaceAdministrationState(current, workerState({ operationEpoch: 2 }), 1, authorPage())).toBe(current);
    for (const phase of ["cancelled", "denied", "stale", "failed"] as const) {
      const next = reduceShellSpaceAdministrationState(current, workerState({ phase, code: "forbidden" }), 1, authorPage());
      expect(next).toMatchObject({ phase, page: null, code: "forbidden" });
      expect(next?.receiptSha256).toBeUndefined();
      expect(next?.inviteCapabilityPending).toBeUndefined();
    }
  });

  it("carries a receipt only when the worker reports one and never invents authority", () => {
    const receipt = reduceShellSpaceAdministrationState(state(authorPage(), "submitting"), workerState({ phase: "receipt", receiptSha256: "d".repeat(64), inviteCapabilityPending: true, inviteCapabilityStatus: "available" }), 1, authorPage());
    expect(receipt).toMatchObject({ phase: "receipt", receiptSha256: "d".repeat(64), inviteCapabilityPending: true, inviteCapabilityStatus: "available" });
    const submitting = reduceShellSpaceAdministrationState(state(authorPage()), workerState({ phase: "submitting" }), 1, authorPage());
    expect(submitting?.receiptSha256).toBeUndefined();
  });

  it("reports unavailable, rejected, and successful clipboard writes without exposing the capability", async () => {
    const writeText = vi.fn(async (_text: string) => {});
    expect(await copyDirectoryInviteCapabilityV1("invite.v1.secret", undefined)).toBe(false);
    expect(await copyDirectoryInviteCapabilityV1("invite.v1.secret", { writeText })).toBe(true);
    expect(writeText).toHaveBeenCalledTimes(1);
    expect(writeText).toHaveBeenCalledWith("invite.v1.secret");
    expect(await copyDirectoryInviteCapabilityV1("invite.v1.secret", { writeText: async () => { throw new Error("denied"); } })).toBe(false);
  });

  it("round trips the operation and transfer epochs across the typed worker wire", () => {
    const requests: readonly BackboneWorkerRequest[] = [
      { kind: "directory-administration-capability-request", operationEpoch: 7 },
      { kind: "directory-administration-capability-result", operationEpoch: 7, transferEpoch: 2, copied: false },
    ];
    const responses: readonly BackboneWorkerResponse[] = [
      { kind: "directory-administration-capability", operationEpoch: 7, transferEpoch: 2, inviteToken: "invite.v1.secret" },
      { kind: "directory-administration-capability-rejected", operationEpoch: 7, transferEpoch: 2, code: "mismatch" },
    ];
    expect(requests.map((request) => decodeBackboneWorkerRequest(encodeBackboneWorkerRequest(request)))).toEqual(requests);
    expect(responses.map((response) => decodeBackboneWorkerResponse(encodeBackboneWorkerResponse(response)))).toEqual(responses);
  });

  it("refuses every intent the canonical page does not authorize and never double-dispatches", () => {
    const author = state(authorPage());
    const remove = shellSpaceAdministrationRequest(author, { kind: "remove-member", userId: "user-b" }, "1".repeat(32));
    expect(remove).toMatchObject({ kind: "directory-administration-submit", command: { kind: "remove-member", spaceId: SPACE, userId: "user-b" } });
    expect(shellSpaceAdministrationRequest(author, { kind: "remove-member", userId: "user-a" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(author, { kind: "remove-member", userId: "user-ghost" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(author, { kind: "revoke-invite", inviteId: "invite-dead" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(author, { kind: "revoke-invite", inviteId: "invite-live" }, "1".repeat(32))).toMatchObject({ kind: "directory-administration-submit" });
    expect(shellSpaceAdministrationRequest(state(memberPage()), { kind: "create-invite", role: "spectator" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(state(authorPage(), "submitting"), { kind: "create-invite", role: "spectator" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(state(null, "denied"), { kind: "create-invite", role: "spectator" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(state(null, "denied"), { kind: "close" }, "1".repeat(32))).toMatchObject({ kind: "directory-administration-close" });
  });
});

describe("SpaceAdministrationPane", () => {
  it("derives every affordance from the server capability flags and disables owner removal", () => {
    const page = authorPage();
    expect(spaceAdministrationCapabilities(page)?.removeMember).toBe(true);
    expect(spaceAdministrationCapabilities(memberPage())).toBeNull();
    expect(spaceAdministrationMemberRemovable(page.access === "author" ? page.members.rows[0]! : ({} as never), spaceAdministrationCapabilities(page))).toBe(false);
    expect(spaceAdministrationMemberRemovable(page.access === "author" ? page.members.rows[1]! : ({} as never), spaceAdministrationCapabilities(page))).toBe(true);
    expect(spaceAdministrationInviteRevocable(page.access === "author" ? page.invites.rows[1]! : ({} as never), spaceAdministrationCapabilities(page))).toBe(false);
    expect(spaceAdministrationDispatchable("submitting")).toBe(false);
    expect(spaceAdministrationDispatchable("ready")).toBe(true);
  });

  it("renders a labelled live region, semantic controls, and dispatches one intent per activation", () => {
    const intents: SpaceAdministrationIntentV1[] = [];
    const { container } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={authorPage()} onIntent={(intent) => intents.push(intent)} />);
    const status = screen.getByRole("status");
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.textContent).toContain("Administration page is current.");
    const remove = screen.getByRole("button", { name: "Remove member: user-b" });
    expect(remove.hasAttribute("disabled")).toBe(false);
    expect(screen.getByRole("button", { name: "Remove member: user-a" }).hasAttribute("disabled")).toBe(true);
    fireEvent.click(remove);
    expect(intents).toEqual([{ kind: "remove-member", userId: "user-b" }]);
    const select = container.querySelector<HTMLSelectElement>("#os-space-administration-role-user-b");
    expect(select).not.toBeNull();
    expect(container.querySelector(`label[for="os-space-administration-role-user-b"]`)?.textContent).toBe("Role");
    fireEvent.change(select as HTMLSelectElement, { target: { value: "author" } });
    expect(intents.at(-1)).toEqual({ kind: "set-role", userId: "user-b", role: "author" });
    expect(screen.getByRole("button", { name: "Revoke invitation: invite-dead" }).hasAttribute("disabled")).toBe(true);
    expect(screen.getByRole("button", { name: "Revoke invitation: invite-live" }).hasAttribute("disabled")).toBe(false);
    expect(container.querySelector(`[aria-label="Copy invitation link"]`)).toBeNull();
    expect(container.querySelector("#os-space-administration-title")?.getAttribute("tabindex")).toBe("-1");
  });

  it("shows the member notice without any administration control and no invite window", () => {
    const intents: SpaceAdministrationIntentV1[] = [];
    const { container } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={memberPage()} onIntent={(intent) => intents.push(intent)} />);
    expect(screen.getByText(/You can view this space but not administer it\./u)).toBeTruthy();
    expect(container.querySelector(`[aria-label="Issue invitation"]`)).toBeNull();
    expect(screen.getByRole("button", { name: "Remove member: user-b" }).hasAttribute("disabled")).toBe(true);
    expect((container.querySelector<HTMLSelectElement>("#os-space-administration-role-user-b"))?.disabled).toBe(true);
  });

  it("offers the one-shot copy only while the worker still holds the capability", () => {
    const intents: SpaceAdministrationIntentV1[] = [];
    const { container, rerender } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={authorPage()} inviteCapabilityPending inviteCapabilityStatus="failed" onIntent={(intent) => intents.push(intent)} />);
    expect(screen.getByRole("status").textContent).toContain("Clipboard unavailable or denied. Try copying again.");
    fireEvent.click(screen.getByRole("button", { name: "Copy invitation link" }));
    expect(intents).toEqual([{ kind: "copy-invite-capability" }]);
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={authorPage()} inviteCapabilityPending inviteCapabilityStatus="copying" onIntent={(intent) => intents.push(intent)} />);
    expect(screen.getByRole("button", { name: "Copy invitation link" }).hasAttribute("disabled")).toBe(true);
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={authorPage()} onIntent={(intent) => intents.push(intent)} />);
    expect(container.querySelector(`[aria-label="Copy invitation link"]`)).toBeNull();
  });

  it("erases every administration control the moment the phase settles denied", () => {
    const { container, rerender } = render(<SpaceAdministrationPane spaceId={SPACE} phase="loading" page={null} onIntent={() => {}} />);
    expect(screen.getByRole("status").textContent).toContain("Loading the administration page…");
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="denied" page={null} code="forbidden" onIntent={() => {}} />);
    expect(screen.getByRole("status").textContent).toContain("Access to this space was withdrawn.");
    expect(screen.getByRole("status").textContent).toContain("forbidden");
    expect(container.querySelector(`[aria-label="Issue invitation"]`)).toBeNull();
    expect(container.querySelectorAll("select")).toHaveLength(0);
    expect(document.activeElement?.id).toBe("os-space-administration-title");
  });
});
