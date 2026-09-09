import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { setUiLocale } from "@semio-tech/ui-react";
import { afterEach, describe, expect, it, vi } from "vitest";
import Ajv from "ajv";
import deepEqual from "fast-deep-equal";
import directorySchema from "../../../../📇️directory/🧬️schema/🔣️.json";
import propertiesFixture from "../../🧱️elements/🛂️SpaceAdministration/🧫️fixtures/⚙️properties/🔣️.json";
import deleteFixture from "../../🧱️elements/🛂️SpaceAdministration/🧫️fixtures/🗑️delete-space/🔣️.json";
import admissionFixture from "../../../../📇️directory/🧬️schema/🏛️administration/🧫️fixtures/🛂️command-admission/🔣️.json";
import admissionFixtureSchema from "../../../../📇️directory/🧬️schema/🏛️administration/🧫️fixtures/🛂️command-admission/🧬️schema/🔣️.json";
import propertiesFixtureSchema from "../../🧱️elements/🛂️SpaceAdministration/🧫️fixtures/⚙️properties/🧬️schema/🔣️.json";
import deleteFixtureSchema from "../../🧱️elements/🛂️SpaceAdministration/🧫️fixtures/🗑️delete-space/🧬️schema/🔣️.json";
import {
  decodeBackboneWorkerRequest,
  decodeBackboneWorkerResponse,
  directoryAdministrationCommandAllowedV1,
  encodeBackboneWorkerRequest,
  encodeBackboneWorkerResponse,
  type BackboneWorkerRequest,
  type BackboneWorkerResponse,
  type DirectorySpaceAdministrationPageV1,
  type DirectoryCommand,
} from "@semio-tech/framework-os";
import {
  copyDirectoryInviteCapabilityV1,
  reduceShellSpaceAdministrationState,
  shellSpaceAdministrationOpening,
  shellSpaceAdministrationRequest,
  type ShellSpaceAdministrationStateV1,
} from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import {
  SpaceAdministrationPane,
  spaceAdministrationCapabilities,
  spaceAdministrationDispatchable,
  spaceAdministrationInviteRevocable,
  spaceAdministrationMemberRemovable,
  type SpaceAdministrationIntentV1,
} from "../../🧱️elements/🛂️SpaceAdministration/🟦️.tsx";

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
  it("validates the neutral administration fixtures and refuses unknown envelope and row fields", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(directorySchema);
    for (const [schema, fixture] of [[propertiesFixtureSchema, propertiesFixture], [deleteFixtureSchema, deleteFixture], [admissionFixtureSchema, admissionFixture]] as const) {
      const valid = ajv.compile(schema);
      expect(valid(fixture), JSON.stringify(valid.errors)).toBe(true);
      expect(valid({ ...fixture, extra: true })).toBe(false);
    }
    const valid = ajv.compile(propertiesFixtureSchema);
    expect(valid({ ...propertiesFixture, cases: propertiesFixture.cases.map((row, index) => index === 0 ? { ...row, extra: true } : row) })).toBe(false);
  });

  it("admits only schema-owned administration commands bound to the current page and capability", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(directorySchema);
    const valid = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/DirectoryCommand` });
    for (const row of admissionFixture.allowed) {
      const command = row.command as DirectoryCommand;
      expect(valid(command), row.capability).toBe(true);
      expect(directoryAdministrationCommandAllowedV1(authorPage(), SPACE, command), row.capability).toBe(true);
      for (const refusal of admissionFixture.refused) {
        const page = authorPage();
        if (page.access !== "author") throw new Error("author fixture");
        if (refusal === "foreign-page") page.spaceId = "another-space";
        if (refusal === "foreign-space-view") page.space.id = "another-space";
        if (refusal === "withdrawn-capability") page.capabilities = { ...page.capabilities, [row.capability]: false };
        const captured = refusal === "missing-page" ? null : refusal === "member-page" ? memberPage() : page;
        const target = refusal === "foreign-command" ? { ...command, spaceId: "another-space" } as DirectoryCommand : command;
        expect(directoryAdministrationCommandAllowedV1(captured, SPACE, target), `${row.capability}:${refusal}`).toBe(false);
      }
    }
    for (const command of admissionFixture.unrelated) {
      expect(valid(command), command.kind).toBe(true);
      expect(directoryAdministrationCommandAllowedV1(authorPage(), SPACE, command as DirectoryCommand)).toBe(false);
    }
    for (const command of admissionFixture.malformed) expect(directoryAdministrationCommandAllowedV1(authorPage(), SPACE, command as DirectoryCommand)).toBe(false);
    console.info(`[DEBUG] administration command admission: ${admissionFixture.allowed.length} scoped capabilities with ${admissionFixture.refused.length} refusals each`);
  });

  it("maps the neutral space-property commands only under exact current page capability", () => {
    const ajv = new Ajv({ strict: false });
    ajv.addSchema(directorySchema);
    const valid = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/DirectoryCommand` });
    for (const row of propertiesFixture.cases) {
      const intent = row.intent as SpaceAdministrationIntentV1;
      const current = state(authorPage());
      const request = shellSpaceAdministrationRequest(current, intent, "1".repeat(32));
      expect(request?.kind, row.id).toBe("directory-administration-submit");
      if (request?.kind !== "directory-administration-submit") throw new Error(row.id);
      expect(valid(request.command), row.id).toBe(true);
      expect(deepEqual(request.command, row.command), row.id).toBe(true);
      for (const refusal of propertiesFixture.refused) {
        const page = authorPage();
        if (page.access !== "author") throw new Error("author fixture");
        if (refusal === "capability-withdrawn") page.capabilities = { ...page.capabilities, renameSpace: false, setVisibility: false };
        if (refusal === "foreign-page") page.spaceId = "another-space";
        const denied = refusal === "member" ? state(memberPage()) : state(page, refusal === "capability-withdrawn" || refusal === "foreign-page" ? "ready" : refusal as ShellSpaceAdministrationStateV1["phase"]);
        expect(shellSpaceAdministrationRequest(denied, intent, "1".repeat(32)), `${row.id}:${refusal}`).toBeNull();
      }
    }
    for (const name of propertiesFixture.invalidNames) expect(shellSpaceAdministrationRequest(state(authorPage()), { kind: "rename-space", name } as SpaceAdministrationIntentV1, "1".repeat(32))).toBeNull();
    console.info(`[DEBUG] space administration properties: ${propertiesFixture.cases.length} canonical commands, ${propertiesFixture.refused.length} authority refusals each`);
  });

  it("maps only the exact Home manage-space effect into one canonical administration opening", () => {
    expect(shellSpaceAdministrationOpening("os.directory.open-administration", { spaceId: SPACE }, 7)).toEqual({
      state: { operationEpoch: 7, spaceId: SPACE, phase: "loading", page: null },
      request: { kind: "directory-administration-open", operationEpoch: 7, spaceId: SPACE },
    });
    expect(shellSpaceAdministrationOpening("os.directory.open-administration", { spaceId: "" }, 7)).toBeNull();
    expect(shellSpaceAdministrationOpening("os.directory.open", { spaceId: SPACE }, 7)).toBeNull();
    expect(shellSpaceAdministrationOpening("os.directory.open-administration", { spaceId: SPACE }, -1)).toBeNull();
  });

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

  it("never resurrects a closed administration operation after late page verification success or failure", async () => {
    for (const value of propertiesFixture.retiredVerificationPhases) {
      const phase = value as "ready" | "failed";
      let current: ShellSpaceAdministrationStateV1 | null = state(authorPage(), "loading");
      const pending = Promise.resolve().then(() => { current = reduceShellSpaceAdministrationState(current, workerState({ phase }), 1, phase === "ready" ? authorPage() : null); });
      current = null;
      await pending;
      expect(current, phase).toBeNull();
    }
  });

  it("carries a receipt only when the worker reports one and never invents authority", () => {
    const receipt = reduceShellSpaceAdministrationState(state(authorPage(), "submitting"), workerState({ phase: "receipt", receiptSha256: "d".repeat(64), inviteCapabilityPending: true, inviteCapabilityStatus: "available" }), 1, authorPage());
    expect(receipt).toMatchObject({ phase: "receipt", receiptSha256: "d".repeat(64), inviteCapabilityPending: true, inviteCapabilityStatus: "available" });
    const submitting = reduceShellSpaceAdministrationState(state(authorPage()), workerState({ phase: "submitting" }), 1, authorPage());
    expect(submitting?.receiptSha256).toBeUndefined();
  });

  it("admits delete only under the exact current capability and retains only an accepted receipt terminal", () => {
    const author = state(authorPage());
    expect(shellSpaceAdministrationRequest(author, { kind: "delete-space" }, "4".repeat(32))).toEqual({
      kind: "directory-administration-submit",
      operationEpoch: 1,
      requestId: "4".repeat(32),
      command: { kind: "delete-space", spaceId: SPACE },
    });
    for (const phase of deleteFixture.refusedPhases) expect(shellSpaceAdministrationRequest(state(authorPage(), phase as ShellSpaceAdministrationStateV1["phase"]), { kind: "delete-space" }, "4".repeat(32)), phase).toBeNull();
    const withdrawn = authorPage();
    withdrawn.capabilities = { ...withdrawn.capabilities, deleteSpace: false };
    expect(shellSpaceAdministrationRequest(state(withdrawn), { kind: "delete-space" }, "4".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(state(memberPage()), { kind: "delete-space" }, "4".repeat(32))).toBeNull();
    const receiptSha256 = "d".repeat(64);
    for (const outcome of deleteFixture.acceptedOutcomes) {
      const deleted = reduceShellSpaceAdministrationState(author, workerState({ phase: "deleted", receiptSha256, outcome }), 1, null);
      expect(deleted, outcome).toEqual({ operationEpoch: 1, spaceId: SPACE, phase: "deleted", page: null, receiptSha256 });
    }
    expect(reduceShellSpaceAdministrationState(author, workerState({ phase: "deleted", receiptSha256, outcome: "secret-undeliverable" }), 1, null)).toBe(author);
    expect(reduceShellSpaceAdministrationState(author, workerState({ phase: "deleted", outcome: "accepted" }), 1, null)).toBe(author);
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
    expect(shellSpaceAdministrationRequest({ ...state(memberPage()), inviteCapabilityPending: true, inviteCapabilityStatus: "available" }, { kind: "copy-invite-capability" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest({ ...author, inviteCapabilityPending: true, inviteCapabilityStatus: "available" }, { kind: "copy-invite-capability" }, "1".repeat(32))).toMatchObject({ kind: "directory-administration-capability-request" });
    expect(shellSpaceAdministrationRequest(state(authorPage(), "submitting"), { kind: "create-invite", role: "spectator" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(state(null, "denied"), { kind: "create-invite", role: "spectator" }, "1".repeat(32))).toBeNull();
    expect(shellSpaceAdministrationRequest(state(null, "denied"), { kind: "close" }, "1".repeat(32))).toMatchObject({ kind: "directory-administration-close" });
  });
});

describe("SpaceAdministrationPane", () => {
  it("retains drafts during receipt refresh and replaces them only with the confirmed page", () => {
    const page = authorPage();
    const intents: SpaceAdministrationIntentV1[] = [];
    const { rerender } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={page} onIntent={(intent) => intents.push(intent)} />);
    const name = screen.getByRole("textbox", { name: "Space name" }) as HTMLInputElement;
    fireEvent.change(name, { target: { value: "   " } });
    expect(name.getAttribute("aria-invalid")).toBe("true");
    expect(screen.getByRole("button", { name: "Rename space" }).hasAttribute("disabled")).toBe(true);
    fireEvent.change(name, { target: { value: "Draft" } });
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="refreshing" page={page} onIntent={() => {}} />);
    expect((screen.getByRole("textbox", { name: "Space name" }) as HTMLInputElement).value).toBe("Draft");
    const confirmed = authorPage();
    confirmed.receiptSha256 = "e".repeat(64);
    confirmed.space.name = "Server confirmed";
    confirmed.space.visibility = "public";
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={confirmed} onIntent={() => {}} />);
    expect((screen.getByRole("textbox", { name: "Space name" }) as HTMLInputElement).value).toBe("Server confirmed");
    expect((screen.getByRole("combobox", { name: "Visibility" }) as HTMLSelectElement).value).toBe("public");
    expect(intents).toEqual([]);
    console.info("[DEBUG] space properties: drafts stayed local through refresh and replaced only by a new receipt");
  });

  it("edits properties in both locales without optimistic publication and withdraws controls with authority", async () => {
    for (const labels of propertiesFixture.locales) {
      await setUiLocale(labels.locale);
      try {
        const intents: SpaceAdministrationIntentV1[] = [];
        const page = authorPage();
        const { rerender } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={page} onIntent={(intent) => intents.push(intent)} />);
        const name = screen.getByRole("textbox", { name: labels.name }) as HTMLInputElement;
        fireEvent.change(name, { target: { value: "Research" } });
        fireEvent.click(screen.getByRole("button", { name: labels.rename }));
        expect(intents).toEqual([{ kind: "rename-space", name: "Research" }]);
        expect(page.space.name).toBe("Administered");
        const visibility = screen.getByRole("combobox", { name: labels.visibility }) as HTMLSelectElement;
        fireEvent.change(visibility, { target: { value: "public" } });
        fireEvent.click(screen.getByRole("button", { name: labels.applyVisibility }));
        expect(intents.at(-1)).toEqual({ kind: "set-visibility", visibility: "public" });
        expect(page.space.visibility).toBe("private");
        rerender(<SpaceAdministrationPane spaceId={SPACE} phase="submitting" page={page} onIntent={() => {}} />);
        expect(screen.getByRole("button", { name: labels.rename }).hasAttribute("disabled")).toBe(true);
        expect(screen.getByRole("button", { name: labels.applyVisibility }).hasAttribute("disabled")).toBe(true);
        rerender(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={memberPage()} onIntent={() => {}} />);
        expect(() => screen.getByRole("textbox", { name: labels.name })).toThrow();
        expect(() => screen.getByRole("combobox", { name: labels.visibility })).toThrow();
      } finally {
        cleanup();
        await setUiLocale("en");
      }
    }
  });

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
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="refreshing" page={authorPage()} inviteCapabilityPending inviteCapabilityStatus="available" onIntent={(intent) => intents.push(intent)} />);
    expect(screen.getByRole("button", { name: "Copy invitation link" }).hasAttribute("disabled")).toBe(true);
    rerender(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={authorPage()} onIntent={(intent) => intents.push(intent)} />);
    expect(container.querySelector(`[aria-label="Copy invitation link"]`)).toBeNull();
  });

  it("requires an explicit accessible confirmation in both locales and never deletes optimistically", async () => {
    for (const labels of deleteFixture.locales) {
      await setUiLocale(labels.locale);
      try {
        const intents: SpaceAdministrationIntentV1[] = [];
        const page = authorPage();
        const { container, rerender } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={page} onIntent={(intent) => intents.push(intent)} />);
        fireEvent.click(screen.getByRole("button", { name: labels.delete }));
        const confirmation = screen.getByRole("alertdialog", { name: labels.delete });
        expect(confirmation.getAttribute("aria-describedby")).not.toBeNull();
        expect(screen.getByText(labels.warning)).toBeTruthy();
        expect(intents).toEqual([]);
        fireEvent.click(screen.getByRole("button", { name: labels.cancel }));
        expect(intents).toEqual([]);
        expect(document.activeElement).toBe(screen.getByRole("button", { name: labels.delete }));
        fireEvent.click(screen.getByRole("button", { name: labels.delete }));
        fireEvent.click(screen.getByRole("button", { name: labels.confirm }));
        expect(intents).toEqual([{ kind: "delete-space" }]);
        expect(page.space.id).toBe(SPACE);
        rerender(<SpaceAdministrationPane spaceId={SPACE} phase="deleted" page={null} receiptSha256={"d".repeat(64)} onIntent={(intent) => intents.push(intent)} />);
        expect(screen.getByRole("status").textContent).toContain(labels.deleted);
        expect(container.querySelector(`[aria-label="${labels.delete}"]`)).toBeNull();
        expect(document.activeElement?.id).toBe("os-space-administration-title");
      } finally {
        cleanup();
        await setUiLocale("en");
      }
    }
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

  it("renders the German spectator status and restores keyboard focus without exposing an invite", async () => {
    await setUiLocale("de");
    try {
      const { container, rerender } = render(<SpaceAdministrationPane spaceId={SPACE} phase="ready" page={memberPage()} onIntent={() => {}} />);
      expect(screen.getByRole("status").textContent).toContain("Verwaltungsseite ist aktuell.");
      expect(screen.getByText("Du kannst diesen Space ansehen, aber nicht verwalten.")).toBeTruthy();
      expect(container.querySelector(`[aria-label="Einladung ausstellen"]`)).toBeNull();
      expect(container.textContent).not.toContain("invite.v1");
      rerender(<SpaceAdministrationPane spaceId={SPACE} phase="denied" page={null} code="forbidden" onIntent={() => {}} />);
      expect(screen.getByRole("status").textContent).toContain("Der Zugriff auf diesen Space wurde entzogen.");
      expect(document.activeElement?.id).toBe("os-space-administration-title");
    } finally {
      await setUiLocale("en");
    }
  });
});
