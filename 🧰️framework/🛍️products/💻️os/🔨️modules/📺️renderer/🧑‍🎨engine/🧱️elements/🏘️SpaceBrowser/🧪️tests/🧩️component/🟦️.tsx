// #region 🧲️Header
/** @emoji 🏘️ Laws for the end-user spaces surface (ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice AU2). Independent oracles: Ajv over the owned
 * fixture schema, and — for every command this surface can raise — the *production*
 * `parseDirectoryCommandV1`/`sealDirectoryCommandRequestV1` from `@semio-tech/framework-os`, which
 * re-serializes and byte-compares the command. A builder emitting a different field order fails
 * there, not silently at the hub. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen, waitFor } from "@semio-tech/ui-react/test";
import { setUiLocale } from "@semio-tech/ui-react";
import Ajv from "ajv";
import { useEffect, useState, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { directoryCommandRequestJson, parseDirectoryCommandReceiptV1, parseDirectoryCommandV1, sealDirectoryCommandRequestV1, type DirectoryCommandReceiptV1, type DirectorySpaceListEntryV1, type DirectoryCommand } from "@semio-tech/framework-os";
import fixture from "../../../../../../📇️directory/🏘️spaces/🔣️.json";
import receiptFixture from "../../../../../../../🧫️fixtures/📇️directory/🧾️command-receipt-v1.json";
import fixtureSchema from "../../../../../../📇️directory/🏘️spaces/🧬️.schema.json";
import {
  INVITE_TTL_CHOICES_SECS_V1,
  archiveSpaceCommandV1,
  createInviteCommandV1,
  createSpaceCommandV1,
  directoryCommandAnsweredV1,
  filterSpaceRowsV1,
  inviteLinkV1,
  inviteRedeemPathV1,
  inviteRedemptionErrorFromStatusV1,
  parseInviteTokenV1,
  spaceBrowserRowsUsableV1,
  spaceMemberPresenceV1,
  spaceRowInvitableV1,
  spaceRowWritableV1,
  spaceRowsAfterEventsV1,
  spaceRowsV1,
  type InviteRedemptionErrorCodeV1,
  type SpaceMemberPresenceV1,
  type SpaceRowV1,
} from "../../../../../../📇️directory/🏘️spaces/🟦️.ts";
import { emptyDirectoryReadModel, foldAll } from "../../../../../../📇️directory/🟦️.ts";
import type { DirectoryEvent } from "../../../../../../📇️directory/🧬️schema/🟦️.ts";
import { SpaceBrowser, spaceBrowserPresencePeersV1 } from "../../🟦️.tsx";
import { createHubConnectionFetchPortV1, parseHubSpaceMemberRowsV1, useHubConnection, type HubConnectionPortV1, type HubSpaceMemberRowV1 } from "../../../🔗️HubConnection/🟦️.tsx";
import { type HubConnectionStorageV1, type HubSignInTransportV1 } from "../../../../../../📇️directory/🔐️sign-in/🟦️.ts";
// #endregion 🔌️Adapters

afterEach(cleanup);
beforeEach(async () => {
  await setUiLocale("en");
});

//#region 🧫️Doubles
const ORIGIN = "http://127.0.0.1:7777";
const INVITE_TOKEN = "abcDEF012";

function memberEntry(id: string, name: string, role: "author" | "spectator", updatedAtMs: number, access: "author" | "member"): DirectorySpaceListEntryV1 {
  return { access, space: { id, name, kind: "atelier", visibility: "private", ownerUserId: "usr_ada", role, memberCount: 2, documentCount: 3, activeConnections: 1, createdAtMs: 1, updatedAtMs } };
}

function publicEntry(id: string, name: string, updatedAtMs: number): DirectorySpaceListEntryV1 {
  return { access: "public", space: { id, name, kind: "studio", visibility: "public", memberCount: 9, documentCount: 4, createdAtMs: 1, updatedAtMs } };
}

const ENTRIES: readonly DirectorySpaceListEntryV1[] = [
  publicEntry("space-public", "Commons", 900),
  memberEntry("space-member", "Shared atelier", "spectator", 700, "member"),
  memberEntry("space-mine", "My atelier", "author", 500, "author"),
  memberEntry("space-mine-new", "Newer atelier", "author", 800, "author"),
];

/** 👥️ The roster the fake hub's space-administration page serves, in the exact
 * `DirectorySpaceAdministrationMemberRowV1` shape the real port reads back. */
const MEMBER_ROWS: readonly HubSpaceMemberRowV1[] = [
  { userId: "usr_bo", displayName: "Bo", email: "bo@example.invalid", role: "spectator", owner: false },
  { userId: "usr_ada", displayName: "Ada", email: "ada@example.invalid", role: "author", owner: true },
];

const MEMBERS: readonly SpaceMemberPresenceV1[] = spaceMemberPresenceV1(
  [
    { userId: "usr_bo", displayName: "Bo", email: "bo@example.invalid", role: "spectator", owner: false },
    { userId: "usr_ada", displayName: "Ada", email: "ada@example.invalid", role: "author", owner: true },
    { userId: "usr_cy", displayName: "", email: "cy@example.invalid", role: "author", owner: false },
  ],
  ["usr_ada", "usr_cy"],
);

function memoryStorage(): HubConnectionStorageV1 {
  const entries = new Map<string, string>();
  return { getItem: (key) => entries.get(key) ?? null, setItem: (key, value) => void entries.set(key, value) };
}

const IDLE_TRANSPORT: HubSignInTransportV1 = {
  mint: async () => ({ status: 401, body: "", retryAfterHeader: null }),
  read: async () => ({ status: 401, body: "", retryAfterHeader: null }),
  end: async () => ({ status: 204, body: "", retryAfterHeader: null }),
};

async function receiptFor(command: DirectoryCommand, inviteToken?: string): Promise<DirectoryCommandReceiptV1> {
  const request = sealDirectoryCommandRequestV1("1".repeat(32), command);
  return {
    schema: "semio.directory.command-receipt.v1",
    requestId: request.requestId,
    commandSha256: "a".repeat(64),
    outcome: "accepted",
    events: [],
    result: inviteToken === undefined ? { kind: "none" } : { kind: "invite", inviteToken },
    receiptSha256: "b".repeat(64),
  };
}

function recordingPort(options: { readonly inviteToken?: string; readonly redeemStatus?: number; readonly clipboard?: "ok" | "fail" } = {}): HubConnectionPortV1 & { readonly commands: DirectoryCommand[]; readonly sealed: string[]; readonly redeemed: string[] } {
  const commands: DirectoryCommand[] = [];
  const sealed: string[] = [];
  const redeemed: string[] = [];
  return {
    commands,
    sealed,
    redeemed,
    signIn: IDLE_TRANSPORT,
    storage: memoryStorage(),
    bootstrapOrigin: ORIGIN,
    deviceInstanceId: "device-au2",
    clientClass: "browser",
    listSpaces: async () => ENTRIES,
    readSpaceMembers: async () => MEMBER_ROWS,
    submitCommand: async (_origin, command) => {
      commands.push(command);
      sealed.push(JSON.stringify(sealDirectoryCommandRequestV1("1".repeat(32), command).command));
      return receiptFor(command, options.inviteToken);
    },
    redeemInvite: async (_origin, token) => {
      redeemed.push(token);
      return { status: options.redeemStatus ?? 200 };
    },
    writeClipboard: options.clipboard === undefined ? undefined : async () => {
      if (options.clipboard === "fail") throw new Error("denied");
    },
    listAgentDelegations: async () => [],
    createAgentDelegation: async () => {
      throw new Error("unused");
    },
    revokeAgentDelegation: async () => undefined,
  };
}

type Deferred<T> = Readonly<{ promise: Promise<T>; resolve(value: T): void }>;

function deferred<T>(): Deferred<T> {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((accept) => { resolve = accept; });
  return { promise, resolve };
}

const HUB_B_ORIGIN = "http://127.0.0.1:8888";
const SESSION_TOKEN = "session.v1.0123456789abcdef0123456789abcdef.0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

function mintResponse(userId: string) {
  return { status: 200, body: JSON.stringify({ token: SESSION_TOKEN, user_id: userId }), retryAfterHeader: null };
}

function authorityResponse(userId: string) {
  return {
    status: 200,
    body: JSON.stringify({ schema: "semio.directory.session-authority.v1", sessionBindingSha256: "c".repeat(64), authorizationGeneration: 2, userId, email: `${userId}@example.invalid`, displayName: userId, expiresAt: 4_102_444_800_000, sessionKind: "external" }),
    retryAfterHeader: null,
  };
}

function racePort() {
  const endA = deferred<{ status: number; body: string; retryAfterHeader: null }>();
  const mintA = deferred<{ status: number; body: string; retryAfterHeader: null }>();
  const commandA = deferred<DirectoryCommandReceiptV1>();
  const listCalls: string[] = [];
  let deferMintA = false;
  let deferCommandA = false;
  let deferEndA = false;
  const signIn: HubSignInTransportV1 = {
    mint: async (origin) => origin === ORIGIN && deferMintA ? mintA.promise : mintResponse(origin === ORIGIN ? "usr_a" : "usr_b"),
    read: async (origin) => authorityResponse(origin === ORIGIN ? "usr_a" : "usr_b"),
    end: async (origin) => origin === ORIGIN && deferEndA ? endA.promise : { status: 204, body: "", retryAfterHeader: null },
  };
  const port: HubConnectionPortV1 = {
    signIn,
    storage: memoryStorage(),
    bootstrapOrigin: ORIGIN,
    deviceInstanceId: "device-race",
    clientClass: "browser",
    listSpaces: async (origin) => {
      listCalls.push(origin);
      return [memberEntry(origin === ORIGIN ? "space-a" : "space-b", origin === ORIGIN ? "A row" : "B row", "author", 1, "author")];
    },
    readSpaceMembers: async () => [],
    submitCommand: async (origin, command) => origin === ORIGIN && deferCommandA ? commandA.promise : receiptFor(command, "inviteB012"),
    redeemInvite: async () => ({ status: 200 }),
    listAgentDelegations: async () => [],
    createAgentDelegation: async () => {
      throw new Error("unused");
    },
    revokeAgentDelegation: async () => undefined,
  };
  return {
    port,
    endA,
    mintA,
    commandA,
    listCalls,
    deferMintA: () => { deferMintA = true; },
    deferCommandA: () => { deferCommandA = true; },
    deferEndA: () => { deferEndA = true; },
  };
}
//#endregion 🧫️Doubles

//#region 🧬️Contract
describe("spaces surface contract", () => {
  it("validates its own fixture with an independent schema compiler", () => {
    expect(new Ajv({ strict: true, allErrors: true }).compile(fixtureSchema)(fixture)).toBe(true);
    expect(INVITE_TTL_CHOICES_SECS_V1.slice()).toStrictEqual(fixture.inviteTtlChoicesSecs);
    expect(inviteRedeemPathV1(INVITE_TOKEN)).toBe(`/directory/invites/${INVITE_TOKEN}/redeem`);
  });

  it("orders my spaces before shared ones before public ones, newest first inside each group", () => {
    const rows = spaceRowsV1(ENTRIES);
    expect(rows.map((row) => row.id)).toStrictEqual(["space-mine-new", "space-mine", "space-member", "space-public"]);
    expect(rows.map((row) => row.access)).toStrictEqual(fixture.accessOrder.flatMap((access) => (access === "author" ? [access, access] : [access])));
    expect(rows[3]!.role).toBeNull();
    expect(rows[3]!.activeConnections).toBe(0);
  });

  it("folds one command receipt's events into my rows exactly as the shared fixture states (read-your-writes)", () => {
    for (const fold of fixture.receiptFolds) {
      expect(spaceRowsAfterEventsV1(fold.rows as SpaceRowV1[], fold.events as DirectoryEvent[], fold.userId), fold.id).toStrictEqual(fold.expected);
    }
  });

  it("agrees with the directory read model's view of the caller on the golden log", () => {
    const golden = fixture.receiptFolds.find((fold) => fold.id === "the-golden-directory-log-folds-to-the-read-models-view-of-its-owner")!;
    const model = foldAll(emptyDirectoryReadModel(), golden.events as DirectoryEvent[]);
    const rows = spaceRowsAfterEventsV1([], golden.events as DirectoryEvent[], golden.userId);
    const mine = [...model.spaces.values()].filter((space) => space.members.some((member) => member.userId === golden.userId));
    expect(rows.map((row) => row.id)).toStrictEqual(mine.map((space) => space.view.id));
    for (const space of mine) {
      const row = rows.find((candidate) => candidate.id === space.view.id)!;
      const role = space.members.find((member) => member.userId === golden.userId)!.role;
      expect([row.name, row.kind, row.visibility, row.memberCount, row.documentCount, row.role, row.updatedAtMs]).toStrictEqual([space.view.name, space.view.kind, space.view.visibility, space.view.memberCount, space.view.documentCount, role, space.view.updatedAtMs]);
    }
  });

  it("filters case-insensitively over name and id and keeps everything for an empty query", () => {
    const rows = spaceRowsV1(ENTRIES);
    expect(filterSpaceRowsV1(rows, "").length).toBe(4);
    expect(filterSpaceRowsV1(rows, "   ").length).toBe(4);
    expect(filterSpaceRowsV1(rows, "ATELIER").map((row) => row.id)).toStrictEqual(["space-mine-new", "space-mine", "space-member"]);
    expect(filterSpaceRowsV1(rows, "space-public").map((row) => row.id)).toStrictEqual(["space-public"]);
    expect(filterSpaceRowsV1(rows, "nothing")).toHaveLength(0);
  });

  it("derives write and invite authority from the row, never from membership alone", () => {
    const [mineNew, , member, publicRow] = spaceRowsV1(ENTRIES);
    expect(spaceRowWritableV1(mineNew!)).toBe(true);
    expect(spaceRowInvitableV1(mineNew!)).toBe(true);
    expect(spaceRowWritableV1(member!)).toBe(false);
    expect(spaceRowInvitableV1(member!)).toBe(false);
    expect(spaceRowInvitableV1(publicRow!)).toBe(false);
    expect(spaceBrowserRowsUsableV1("loading", 4)).toBe(false);
    expect(spaceBrowserRowsUsableV1("stale", 4)).toBe(true);
    expect(spaceBrowserRowsUsableV1("ready", 0)).toBe(false);
  });

  it("builds every command in the canonical field order the production parser re-serializes", async () => {
    const create = createSpaceCommandV1("  New atelier  ", "atelier", "private");
    expect(Object.keys(create)).toStrictEqual(fixture.commandFieldOrder["create-space"]);
    expect(parseDirectoryCommandV1(create)).toStrictEqual(create);
    expect((create as { name: string }).name).toBe("New atelier");
    const invite = createInviteCommandV1("space-mine", "author", INVITE_TTL_CHOICES_SECS_V1[2]);
    expect(Object.keys(invite)).toStrictEqual(fixture.commandFieldOrder["create-invite"]);
    expect(parseDirectoryCommandV1(invite)).toStrictEqual(invite);
    const archive = archiveSpaceCommandV1("space-mine");
    expect(Object.keys(archive)).toStrictEqual(fixture.commandFieldOrder["archive-space"]);
    expect(sealDirectoryCommandRequestV1("1".repeat(32), archive).command).toStrictEqual(archive);
    expect(() => createSpaceCommandV1("   ", "atelier", "private")).toThrow("directory.spaces.invalid-name");
    expect(() => createSpaceCommandV1("badname", "atelier", "private")).toThrow("directory.spaces.invalid-name");
    expect(() => createInviteCommandV1("space-mine", "author", 0)).toThrow("directory.spaces.invalid-ttl");
    await Promise.resolve();
  });

  it("accepts a pasted invitation link or a bare code and refuses anything else", () => {
    for (const row of fixture.inviteTokens) {
      if (row.token === null) expect(() => parseInviteTokenV1(row.typed)).toThrow("directory.spaces.invalid-invite");
      else expect(parseInviteTokenV1(row.typed)).toBe(row.token);
    }
    expect(inviteLinkV1("https://hub.example.invalid", INVITE_TOKEN)).toBe(`https://hub.example.invalid/#semio-invite=${INVITE_TOKEN}`);
    expect(() => parseInviteTokenV1("a".repeat(300))).toThrow("directory.spaces.invalid-invite");
  });

  it("classifies every redemption status in the fixture's table", () => {
    for (const row of fixture.redemptionStatusCodes) expect(inviteRedemptionErrorFromStatusV1(row.status)).toBe(row.code as InviteRedemptionErrorCodeV1);
  });

  it("joins members with live presence, owners first, and falls back to the email as a display name", () => {
    expect(MEMBERS.map((member) => member.userId)).toStrictEqual(["usr_ada", "usr_cy", "usr_bo"]);
    expect(MEMBERS[0]!.online).toBe(true);
    expect(MEMBERS[2]!.online).toBe(false);
    expect(MEMBERS[1]!.displayName).toBe("cy@example.invalid");
    expect(spaceBrowserPresencePeersV1(MEMBERS).map((peer) => peer.userId)).toStrictEqual(["usr_ada", "usr_cy"]);
  });
});
//#endregion 🧬️Contract

//#region 🖥️Surface
function BrowserHarness(props: Partial<Parameters<typeof SpaceBrowser>[0]> = {}): ReactElement {
  const [search, setSearch] = useState(props.search ?? "");
  return (
    <SpaceBrowser
      rows={filterSpaceRowsV1(spaceRowsV1(ENTRIES), search)}
      activeSpaceId={props.activeSpaceId ?? "space-mine"}
      phase={props.phase ?? "ready"}
      search={search}
      members={props.members ?? MEMBERS}
      invite={props.invite ?? null}
      redemption={props.redemption ?? { phase: "idle", error: null }}
      signedIn={props.signedIn ?? true}
      onSearch={props.onSearch ?? setSearch}
      onOpenSpace={props.onOpenSpace ?? (() => undefined)}
      onRefresh={props.onRefresh ?? (() => undefined)}
      onCreateSpace={props.onCreateSpace ?? (() => undefined)}
      onArchiveSpace={props.onArchiveSpace ?? (() => undefined)}
      onCreateInvite={props.onCreateInvite ?? (() => undefined)}
      onCopyInvite={props.onCopyInvite ?? (() => undefined)}
      onDismissInvite={props.onDismissInvite ?? (() => undefined)}
      onRedeemInvite={props.onRedeemInvite ?? (() => undefined)}
    />
  );
}

describe("space browser surface", () => {
  it("lists every reachable space as a semantic list and marks the open one with aria-current", () => {
    const opened: string[] = [];
    const view = render(<BrowserHarness onOpenSpace={(id) => opened.push(id)} />);
    const items = view.container.querySelectorAll("li[data-space-id]");
    expect(items).toHaveLength(4);
    const current = view.container.querySelectorAll('button[aria-current="true"]');
    expect(current).toHaveLength(1);
    expect(current[0]!.getAttribute("aria-label")?.includes("My atelier")).toBe(true);
    fireEvent.click(view.container.querySelector('li[data-space-id="space-public"] button')!);
    expect(opened).toStrictEqual(["space-public"]);
  });

  it("switches spaces through the search box without losing the roster", () => {
    const view = render(<BrowserHarness />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[type="search"]')!, { target: { value: "commons" } });
    expect(view.container.querySelectorAll("li[data-space-id]")).toHaveLength(1);
    expect(view.container.querySelector('li[data-space-id="space-public"]')).toBeTruthy();
    expect(view.container.querySelectorAll("li[data-user-id]")).toHaveLength(3);
  });

  it("announces the load phase in one live region and keeps stale rows usable", () => {
    const view = render(<BrowserHarness phase="stale" />);
    const status = view.container.querySelector('[data-semio-hub-spaces-status]')!;
    expect(status.getAttribute("aria-live")).toBe("polite");
    expect(status.textContent).toBe("Showing the spaces from your last connection.");
    expect(view.container.querySelectorAll("li[data-space-id]")).toHaveLength(4);
  });

  it("shows presence per member and only puts connected members on the presence bar", () => {
    const view = render(<BrowserHarness />);
    expect(view.container.querySelector('li[data-user-id="usr_ada"]')?.getAttribute("data-online")).toBe("true");
    expect(view.container.querySelector('li[data-user-id="usr_bo"]')?.getAttribute("data-online")).toBe("false");
    expect(view.container.querySelectorAll('[data-row-id^="peer:"]')).toHaveLength(2);
  });

  it("raises a create-space intent only for a non-empty name and hides the form when signed out", () => {
    const created: unknown[] = [];
    const view = render(<BrowserHarness onCreateSpace={(name, kind, visibility) => created.push([name, kind, visibility])} />);
    const form = screen.getByRole("form", { name: "Create a space" });
    const name = form.querySelector<HTMLInputElement>('input[name="spaceName"]')!;
    expect((screen.getByRole("button", { name: "Create space" }) as HTMLButtonElement).disabled).toBe(true);
    fireEvent.change(name, { target: { value: "Atelier Two" } });
    fireEvent.change(form.querySelectorAll("select")[0]!, { target: { value: "studio" } });
    fireEvent.change(form.querySelectorAll("select")[1]!, { target: { value: "public" } });
    fireEvent.click(screen.getByRole("button", { name: "Create space" }));
    expect(created).toStrictEqual([["Atelier Two", "studio", "public"]]);
    view.unmount();
    const signedOut = render(<BrowserHarness signedIn={false} />);
    expect(signedOut.container.querySelector('input[name="spaceName"]')).toBeNull();
  });

  it("offers invitation creation only on a space I author, and shows the one-shot link with copy and discard", () => {
    const invites: unknown[] = [];
    const view = render(<BrowserHarness onCreateInvite={(spaceId, role, ttl) => invites.push([spaceId, role, ttl])} />);
    const form = screen.getByRole("form", { name: "Invite someone" });
    fireEvent.change(form.querySelectorAll("select")[0]!, { target: { value: "author" } });
    fireEvent.change(form.querySelectorAll("select")[1]!, { target: { value: String(INVITE_TTL_CHOICES_SECS_V1[0]) } });
    fireEvent.click(screen.getByRole("button", { name: "Create invitation" }));
    expect(invites).toStrictEqual([["space-mine", "author", INVITE_TTL_CHOICES_SECS_V1[0]]]);
    view.unmount();
    const spectator = render(<BrowserHarness activeSpaceId="space-member" />);
    expect(spectator.container.textContent?.includes("Create invitation")).toBe(false);
    spectator.unmount();
    let copies = 0;
    let dismissals = 0;
    const withLink = render(
      <BrowserHarness
        invite={{ spaceId: "space-mine", link: `${ORIGIN}/#semio-invite=${INVITE_TOKEN}`, copy: "idle" }}
        onCopyInvite={() => { copies += 1; }}
        onDismissInvite={() => { dismissals += 1; }}
      />,
    );
    expect(withLink.container.querySelector('[data-semio-hub-invite="space-mine"]')?.textContent?.includes(INVITE_TOKEN)).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "Copy invitation link" }));
    fireEvent.click(screen.getByRole("button", { name: "Discard invitation link" }));
    expect([copies, dismissals]).toStrictEqual([1, 1]);
  });

  it("names every redemption failure class rather than showing a status code", () => {
    for (const [code, text] of [
      ["invalid-invite", "That invitation is not valid."],
      ["expired-invite", "That invitation has expired. Ask for a new one."],
      ["already-member", "You are already a member of that space."],
      ["unauthorized", "Sign in to the hub before redeeming an invitation."],
      ["unreachable", "The hub could not be reached. Try again in a moment."],
      ["hub-refused", "The hub refused that invitation."],
    ] as const) {
      const view = render(<BrowserHarness redemption={{ phase: "failed", error: code }} />);
      const alert = view.container.querySelector("[data-semio-hub-redemption-error]")!;
      expect(alert.getAttribute("data-semio-hub-redemption-error")).toBe(code);
      expect(alert.textContent).toBe(text);
      view.unmount();
    }
  });

  it("raises a redeem intent with whatever the human pasted and shows a busy control while joining", () => {
    const redeemed: string[] = [];
    const view = render(<BrowserHarness onRedeemInvite={(typed) => redeemed.push(typed)} />);
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[name="invitation"]')!, { target: { value: `${ORIGIN}/#semio-invite=${INVITE_TOKEN}` } });
    fireEvent.click(screen.getByRole("button", { name: "Join space" }));
    expect(redeemed).toStrictEqual([`${ORIGIN}/#semio-invite=${INVITE_TOKEN}`]);
    view.unmount();
    const busy = render(<BrowserHarness redemption={{ phase: "redeeming", error: null }} />);
    expect(screen.getByRole("button", { name: "Joining…" }).getAttribute("aria-busy")).toBe("true");
    expect(busy.container.querySelector<HTMLInputElement>('input[name="invitation"]')!.disabled).toBe(true);
  });

  it("renders every label in German without any English leaking through", async () => {
    await setUiLocale("de");
    const view = render(<BrowserHarness />);
    expect(screen.getByRole("button", { name: "Spaces aktualisieren" })).toBeTruthy();
    expect(screen.getByRole("form", { name: "Space erstellen" })).toBeTruthy();
    expect(screen.getByRole("form", { name: "Einladung einlösen" })).toBeTruthy();
    expect(view.container.textContent?.includes("Create a space")).toBe(false);
    expect(view.container.textContent?.includes("Members")).toBe(false);
    await setUiLocale("en");
  });

  it("lays out without a fixed pixel width so a phone viewport never scrolls sideways", () => {
    const view = render(<BrowserHarness />);
    for (const element of Array.from(view.container.querySelectorAll<HTMLElement>("input, select, button, section, ul, li, aside"))) {
      const className = element.getAttribute("class") ?? "";
      expect(/\b(w|min-w|max-w)-\[\d+px\]/u.test(className)).toBe(false);
      expect(element.style.width === "" || element.style.width.endsWith("%")).toBe(true);
    }
    expect(view.container.querySelector("aside")?.getAttribute("class")?.includes("w-full")).toBe(true);
  });
});
//#endregion 🖥️Surface

//#region 🔗️Hook
function HookHarness({ port }: { readonly port: HubConnectionPortV1 }): ReactElement {
  const hub = useHubConnection(port);
  return (
    <div>
      <SpaceBrowser
        rows={hub.rows}
        activeSpaceId="space-mine"
        phase={hub.spacesPhase}
        search={hub.search}
        members={MEMBERS}
        invite={hub.invite}
        redemption={hub.redemption}
        signedIn
        onSearch={hub.setSearch}
        onOpenSpace={() => undefined}
        onRefresh={hub.refreshSpaces}
        onCreateSpace={hub.createSpace}
        onArchiveSpace={hub.archiveSpace}
        onCreateInvite={hub.createInvite}
        onCopyInvite={hub.copyInvite}
        onDismissInvite={hub.dismissInvite}
        onRedeemInvite={hub.redeemInvite}
      />
      <output data-testid="phase">{hub.spacesPhase}</output>
      <output data-testid="rows">{String(hub.rows.length)}</output>
    </div>
  );
}

// 👥️ ticket 26/09/18 slice AU3 — AU2 §6 gap 3: `ShellHost` passed `members={[]}`, so the roster
// below the prop was tested but never fed. The hook now reads the hub's own space-administration
// page and joins it with the shell's presence lane.
function MembersHarness({ port, online }: { readonly port: HubConnectionPortV1; readonly online: readonly string[] }): ReactElement {
  const hub = useHubConnection(port, online);
  const { watchSpaceMembers } = hub;
  useEffect(() => watchSpaceMembers("space-mine"), [watchSpaceMembers]);
  return (
    <ul data-testid="roster">
      {hub.members.map((member) => (
        <li key={member.userId} data-member-id={member.userId} data-member-online={String(member.online)}>
          {member.displayName}
        </li>
      ))}
    </ul>
  );
}

describe("space members from the hub's own projection", () => {
  it("reads the roster for the watched space and marks exactly the presence lane's hub identities online", async () => {
    const port = recordingPort();
    const view = render(<MembersHarness port={port} online={["usr_ada", "usr_absent"]} />);
    await waitFor(() => expect(view.container.querySelectorAll("li[data-member-id]")).toHaveLength(2));
    const rows = [...view.container.querySelectorAll("li[data-member-id]")].map((row) => [row.getAttribute("data-member-id"), row.getAttribute("data-member-online")]);
    expect(rows).toStrictEqual([
      ["usr_ada", "true"],
      ["usr_bo", "false"],
    ]);
  });

  // 🪟️ `members` is a `DirectorySpaceAdministrationWindowV1` — `{ rows, nextCursor? }` — and an
  // `access: "public"` page carries no `members` key at all. This slice's first implementation read
  // it as a bare array, which only the live hub caught; these vectors are the shape the running hub
  // actually served (`🗑️generated/au3-live-transcript-*.txt`).
  it("reads the administration page's member WINDOW and refuses rows the hub could not have served", () => {
    expect(parseHubSpaceMemberRowsV1(JSON.stringify({ access: "member", members: { rows: [{ userId: "usr_ok", displayName: "Ok", email: "ok@example.invalid", role: "author", owner: true }] } }))).toHaveLength(1);
    expect(parseHubSpaceMemberRowsV1(JSON.stringify({ access: "member", members: { rows: [], nextCursor: "c" } }))).toHaveLength(0);
    expect(parseHubSpaceMemberRowsV1(JSON.stringify({ access: "public", documents: { rows: [] } }))).toHaveLength(0);
    expect(parseHubSpaceMemberRowsV1(JSON.stringify({ members: [{ userId: "usr_ok", displayName: "Ok", email: "ok@example.invalid", role: "author", owner: true }] }))).toHaveLength(0);
    expect(parseHubSpaceMemberRowsV1("<!doctype html><title>proxy</title>")).toHaveLength(0);
    const hostile = {
      members: {
        rows: [
          { userId: "", displayName: "Empty", email: "a@b.invalid", role: "author", owner: true },
          { userId: "usr_role", displayName: "Role", email: "a@b.invalid", role: "root", owner: true },
          { userId: "usr_ctrl", displayName: "Line\nbreak", email: "a@b.invalid", role: "author", owner: true },
          { userId: "usr_long", displayName: "d".repeat(129), email: "a@b.invalid", role: "author", owner: true },
          { userId: "usr_keep", displayName: "Keep", email: "a@b.invalid", role: "spectator", owner: false },
        ],
      },
    };
    expect(parseHubSpaceMemberRowsV1(JSON.stringify(hostile)).map((row) => row.userId)).toStrictEqual(["usr_keep"]);
  });
});

// 📮️ ticket 26/09/23 slice S15 — the hub answers an accepted directory command `202 Accepted` with its canonical receipt
// (`🌎️hub/🏗️bootstrap` `post_directory_command`, and the receipt fixture's transport trace retries into a 202). The real
// fetch port refused every status but 200, so creating a space in the React shell always read "The hub did not accept
// that" while the hub had created it (measured on hub 8040). The platform's own `Response.ok` is the 2xx oracle.
describe("the hub fetch port's directory command lane", () => {
  const request = receiptFixture.requests.find((row) => row.name === "create-space")!;
  const accepted = receiptFixture.receipts.find((row) => row.name === "accepted-create-space")!;
  const portAnswering = (status: number) =>
    createHubConnectionFetchPortV1({
      request: async () => ({ status, headers: { get: () => null }, text: async () => accepted.canonical }),
      storage: null,
      bootstrapOrigin: ORIGIN,
      deviceInstanceId: "s15-command-lane-device-0000000000",
      clientClass: "browser",
      parseSpaces: () => [],
      sealCommand: (command) => {
        const sealed = sealDirectoryCommandRequestV1(request.requestId, command);
        return { body: directoryCommandRequestJson(sealed), parseReceipt: (text) => parseDirectoryCommandReceiptV1(text, sealed) };
      },
    });
  const command = request.command as DirectoryCommand;

  it("reads the receipt of every 2xx answer the transport trace delivers, the hub's 202 Accepted first", async () => {
    const delivered = (receiptFixture.transport.traces as readonly { readonly attempts?: readonly { readonly status: number }[] }[]).flatMap((trace) => trace.attempts ?? []).map((attempt) => attempt.status).filter((status) => new Response(null, { status }).ok);
    expect(delivered).toContain(202);
    for (const status of [...new Set([...delivered, 200])]) {
      const receipt = await portAnswering(status).submitCommand(ORIGIN, command, new AbortController().signal);
      expect(receipt.outcome).toBe("accepted");
      expect(receipt.requestId).toBe(request.requestId);
    }
  });

  it("refuses every status the transport table names, before reading any body", async () => {
    for (const row of receiptFixture.transport.statusCodes) await expect(portAnswering(row.status).submitCommand(ORIGIN, command, new AbortController().signal)).rejects.toThrow("hub.command.refused");
  });

  it("classifies every status exactly as the platform's Response.ok does", () => {
    for (let status = 200; status <= 599; status += 1) expect(directoryCommandAnsweredV1(status)).toBe(new Response(null, { status }).ok);
  });
});

describe("useHubConnection spaces lane", () => {
  it("loads the hub's space list on mount and projects it into ordered rows", async () => {
    const port = recordingPort();
    const view = render(<HookHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    expect(view.container.querySelectorAll("li[data-space-id]")[0]?.getAttribute("data-space-id")).toBe("space-mine-new");
  });

  it("sends create-space through the command path as a sealed canonical command, then reloads", async () => {
    const port = recordingPort();
    const view = render(<HookHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[name="spaceName"]')!, { target: { value: "Atelier Two" } });
    fireEvent.click(screen.getByRole("button", { name: "Create space" }));
    await waitFor(() => expect(port.commands).toHaveLength(1));
    expect(port.commands[0]).toStrictEqual({ kind: "create-space", name: "Atelier Two", spaceKind: "atelier", visibility: "private" });
    expect(port.sealed[0]).toBe(JSON.stringify(port.commands[0]));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("ready"));
  });

  it("turns an accepted create-invite receipt into a one-shot link and copies it through the injected clipboard", async () => {
    const port = recordingPort({ inviteToken: INVITE_TOKEN, clipboard: "ok" });
    const view = render(<HookHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    fireEvent.click(screen.getByRole("button", { name: "Create invitation" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-invite]")).toBeTruthy());
    expect(view.container.querySelector("[data-semio-hub-invite]")?.textContent?.includes(`#semio-invite=${INVITE_TOKEN}`)).toBe(true);
    fireEvent.click(screen.getByRole("button", { name: "Copy invitation link" }));
    await waitFor(() => expect(screen.getByText("Invitation link copied.")).toBeTruthy());
    fireEvent.click(screen.getByRole("button", { name: "Discard invitation link" }));
    expect(view.container.querySelector("[data-semio-hub-invite]")).toBeNull();
  });

  it("reports a clipboard refusal instead of claiming the link was copied", async () => {
    const port = recordingPort({ inviteToken: INVITE_TOKEN, clipboard: "fail" });
    const view = render(<HookHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    fireEvent.click(screen.getByRole("button", { name: "Create invitation" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-invite]")).toBeTruthy());
    fireEvent.click(screen.getByRole("button", { name: "Copy invitation link" }));
    await waitFor(() => expect(screen.getByRole("alert").textContent).toBe("Could not reach the clipboard. Select the link and copy it."));
  });

  it("redeems a pasted invitation link by its bare capability and reports the hub's refusal class", async () => {
    const accepted = recordingPort({ redeemStatus: 200 });
    const view = render(<HookHarness port={accepted} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    fireEvent.change(view.container.querySelector<HTMLInputElement>('input[name="invitation"]')!, { target: { value: `${ORIGIN}/#semio-invite=${INVITE_TOKEN}` } });
    fireEvent.click(screen.getByRole("button", { name: "Join space" }));
    await waitFor(() => expect(accepted.redeemed).toStrictEqual([INVITE_TOKEN]));
    await waitFor(() => expect(screen.getByText("You joined the space.")).toBeTruthy());
    view.unmount();
    const refused = recordingPort({ redeemStatus: 409 });
    const second = render(<HookHarness port={refused} />);
    await waitFor(() => expect(second.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    fireEvent.change(second.container.querySelector<HTMLInputElement>('input[name="invitation"]')!, { target: { value: INVITE_TOKEN } });
    fireEvent.click(screen.getByRole("button", { name: "Join space" }));
    await waitFor(() => expect(second.container.querySelector("[data-semio-hub-redemption-error]")?.getAttribute("data-semio-hub-redemption-error")).toBe("already-member"));
  });

  it("keeps the last projection usable and reports stale when the hub stops answering", async () => {
    let reachable = true;
    const port: HubConnectionPortV1 = {
      ...recordingPort(),
      listSpaces: async () => {
        if (!reachable) throw new Error("offline");
        return ENTRIES;
      },
    };
    const view = render(<HookHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="rows"]')?.textContent).toBe("4"));
    reachable = false;
    fireEvent.click(screen.getByRole("button", { name: "Refresh spaces" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("stale"));
    expect(view.container.querySelectorAll("li[data-space-id]")).toHaveLength(4);
  });

  it("retires a delayed A sign-out before B authority, rows, and invitation become current", async () => {
    const { act: hookAct, renderHook } = await import("@testing-library/react");
    const race = racePort();
    const hook = renderHook(() => useHubConnection(race.port));
    await hookAct(async () => { hook.result.current.addRemoteHub(HUB_B_ORIGIN, "B"); });
    const aId = hook.result.current.book.connections.find((row) => row.origin === ORIGIN)!.id;
    const bId = hook.result.current.book.connections.find((row) => row.origin === HUB_B_ORIGIN)!.id;
    await hookAct(async () => { hook.result.current.selectConnection(aId); });
    await hookAct(async () => { hook.result.current.signIn({ email: "a@example.invalid", password: "password-a" }); });
    await waitFor(() => expect(hook.result.current.session.phase).toBe("signed-in"));
    race.deferEndA();
    await hookAct(async () => { hook.result.current.signOut(); });
    await hookAct(async () => { hook.result.current.selectConnection(bId); });
    await hookAct(async () => { hook.result.current.signIn({ email: "b@example.invalid", password: "password-b" }); });
    await waitFor(() => expect(hook.result.current.session.userId).toBe("usr_b"));
    await hookAct(async () => { hook.result.current.createInvite("space-b", "author", 3600); });
    await waitFor(() => expect(hook.result.current.invite?.link).toContain("inviteB012"));
    await hookAct(async () => { race.endA.resolve({ status: 204, body: "", retryAfterHeader: null }); });
    expect(hook.result.current.connection.id).toBe(bId);
    expect(hook.result.current.session.phase).toBe("signed-in");
    expect(hook.result.current.session.userId).toBe("usr_b");
    expect(hook.result.current.rows.map((row) => row.id)).toEqual(["space-b"]);
    expect(hook.result.current.invite?.link).toContain("inviteB012");
  });

  it("retires a delayed A command without invoking its receipt or refreshing B", async () => {
    const { act: hookAct, renderHook } = await import("@testing-library/react");
    const race = racePort();
    const hook = renderHook(() => useHubConnection(race.port));
    await hookAct(async () => { hook.result.current.addRemoteHub(HUB_B_ORIGIN, "B"); });
    const aId = hook.result.current.book.connections.find((row) => row.origin === ORIGIN)!.id;
    const bId = hook.result.current.book.connections.find((row) => row.origin === HUB_B_ORIGIN)!.id;
    await hookAct(async () => { hook.result.current.selectConnection(aId); });
    race.deferCommandA();
    await hookAct(async () => { hook.result.current.createInvite("space-a", "author", 3600); });
    await hookAct(async () => { hook.result.current.selectConnection(bId); });
    await hookAct(async () => { hook.result.current.signIn({ email: "b@example.invalid", password: "password-b" }); });
    await waitFor(() => expect(hook.result.current.session.userId).toBe("usr_b"));
    await hookAct(async () => { hook.result.current.createInvite("space-b", "author", 3600); });
    await waitFor(() => expect(hook.result.current.invite?.link).toContain("inviteB012"));
    const bReads = race.listCalls.filter((origin) => origin === HUB_B_ORIGIN).length;
    await hookAct(async () => { race.commandA.resolve(await receiptFor({ kind: "create-invite", spaceId: "space-a", role: "author", ttlSecs: 3600 }, "inviteA012")); });
    expect(hook.result.current.invite?.link).toContain("inviteB012");
    expect(race.listCalls.filter((origin) => origin === HUB_B_ORIGIN)).toHaveLength(bReads);
    expect(hook.result.current.rows.map((row) => row.id)).toEqual(["space-b"]);
  });

  it("rejects an aborted A sign-in normalized failure after B is signed in", async () => {
    const { act: hookAct, renderHook } = await import("@testing-library/react");
    const race = racePort();
    const hook = renderHook(() => useHubConnection(race.port));
    await hookAct(async () => { hook.result.current.addRemoteHub(HUB_B_ORIGIN, "B"); });
    const aId = hook.result.current.book.connections.find((row) => row.origin === ORIGIN)!.id;
    const bId = hook.result.current.book.connections.find((row) => row.origin === HUB_B_ORIGIN)!.id;
    await hookAct(async () => { hook.result.current.selectConnection(aId); });
    race.deferMintA();
    await hookAct(async () => { hook.result.current.signIn({ email: "a@example.invalid", password: "password-a" }); });
    await hookAct(async () => { hook.result.current.selectConnection(bId); });
    await hookAct(async () => { hook.result.current.signIn({ email: "b@example.invalid", password: "password-b" }); });
    await waitFor(() => expect(hook.result.current.session.userId).toBe("usr_b"));
    await hookAct(async () => { race.mintA.resolve({ status: 503, body: "", retryAfterHeader: null }); });
    expect(hook.result.current.connection.id).toBe(bId);
    expect(hook.result.current.session.phase).toBe("signed-in");
    expect(hook.result.current.session.userId).toBe("usr_b");
    expect(hook.result.current.session.error).toBeNull();
  });
});
//#endregion 🔗️Hook
