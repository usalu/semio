/** 🛰️ HubConnection fetch port: the member row parser, the capability cells and the `fetch`-backed port that every hub call enters through, plus the bootstrap connection it offers. */
import type { DirectoryCommand, DirectoryCommandReceiptV1, DirectorySpaceListEntryV1 } from "@semio-tech/framework/directory/schema";
import { localBootstrapHubConnectionV1, parseHubSessionMintResultV1, HUB_SESSION_SIGN_OUT_PATH_V1, type HubConnectionStorageV1, type HubSignInClientClassV1 } from "@semio-tech/framework/directory/sign-in";
import { directoryCommandAnsweredV1 } from "@semio-tech/framework/directory/spaces";
import { agentDelegationErrorFromResponseV1, agentDelegationListPathV1, agentDelegationRevokePathV1, parseAgentDelegationListV1, parseAgentDelegationReceiptV1, AGENT_DELEGATION_PATH_V1, type AgentCredentialInstallReceiptV1, type AgentCredentialInstallRequestV1 } from "@semio-tech/framework/directory/delegations";
import { AgentDelegationRefusalV1, type HubConnectionPortV1, type HubSpaceMemberRowV1 } from "../🔌️connection-contract/🟦️.ts";

//#region 🔖️FetchPort
/** 👥️ Reads the `members` window out of the hub's own space-administration page. `members` is a
 * `DirectorySpaceAdministrationWindowV1` — `{ rows, nextCursor? }`, not a bare array — and a
 * `"public"`-access page (the projection a non-member gets) carries no `members` key at all, which
 * is an empty roster rather than an error. Every row is bounds-checked here rather than trusted, so
 * a proxy's HTML or an over-long display name can never be rendered as a roster. */
export function parseHubSpaceMemberRowsV1(source: string): readonly HubSpaceMemberRowV1[] {
  let value: unknown;
  try {
    value = JSON.parse(source);
  } catch {
    return [];
  }
  if (value === null || typeof value !== "object") return [];
  const window = (value as { members?: unknown }).members;
  const members = window !== null && typeof window === "object" ? (window as { rows?: unknown }).rows : undefined;
  if (!Array.isArray(members)) return [];
  const rows: HubSpaceMemberRowV1[] = [];
  for (const entry of members.slice(0, HUB_SPACE_MEMBER_ROWS_MAX)) {
    if (entry === null || typeof entry !== "object") continue;
    const { userId, displayName, email, role, owner } = entry as Record<string, unknown>;
    if (typeof userId !== "string" || userId.length === 0 || userId.length > 256 || /\p{Cc}/u.test(userId)) continue;
    if (typeof email !== "string" || email.length > 320 || /\p{Cc}/u.test(email)) continue;
    if (typeof displayName !== "string" || displayName.length > 128 || /\p{Cc}/u.test(displayName)) continue;
    if (role !== "author" && role !== "spectator") continue;
    rows.push({ userId, displayName, email, role, owner: owner === true });
  }
  return rows;
}

/** 📏️ `SPACE_ADMINISTRATION_PAGE_MAX` on the hub side; a longer array is a wire fault, not a page. */
export const HUB_SPACE_MEMBER_ROWS_MAX = 256;

/** 📨️ Minimal structural response the fetch adapter reads — declared here so this element never
 * requires the ambient `Response` type. */
export interface HubFetchResponseV1 {
  readonly status: number;
  readonly headers: { get(name: string): string | null };
  text(): Promise<string>;
}

/** 🎫️ The one place a hub session capability is held for the fetch port. The shell backs it with the capability it
 * remembers for the browsing context, so a sign-in in the hub pane, the dev serve's own local session and the hub refusing
 * the capability are all one value — the port reads it on every request and never keeps a second copy that could go
 * stale. `write` is the port announcing a mint, a sign-out or a refusal. */
export interface HubConnectionCapabilityCellV1 {
  read(): Readonly<{ token: string; userId: string }> | null;
  write(next: Readonly<{ token: string; userId: string }> | null): void;
}

/** 🎫️ A capability cell that is only this closure — what a port without a shell behind it uses. */
export function hubConnectionCapabilityCellV1(initial: Readonly<{ token: string; userId: string }> | null): HubConnectionCapabilityCellV1 {
  let held = initial;
  return {
    read: () => held,
    write: (next) => {
      held = next;
    },
  };
}

/** 🌐️ Builds the real port over an injected request function. The capability is read from its
 * {@link HubConnectionCapabilityCellV1} on every request and sent as `Authorization: Bearer …` (AU1 §1.1);
 * the port never returns it, and the hook only ever learns whose it is ({@link HubConnectionPortV1.heldCapability}),
 * so React state never holds the token. */
export function createHubConnectionFetchPortV1(options: {
  readonly request: (url: string, init: Readonly<{ method: string; headers: Record<string, string>; body?: string }>, signal: AbortSignal) => Promise<HubFetchResponseV1>;
  readonly storage: HubConnectionStorageV1 | null;
  readonly bootstrapOrigin: string;
  readonly deviceInstanceId: string;
  readonly clientClass: HubSignInClientClassV1;
  readonly parseSpaces: (body: string) => readonly DirectorySpaceListEntryV1[];
  readonly sealCommand: (command: DirectoryCommand) => Readonly<{ body: string; parseReceipt: (body: string) => Promise<DirectoryCommandReceiptV1> }>;
  readonly writeClipboard?: (text: string) => Promise<void>;
  /** 📄️ Hands the human a file to save — the one-time agent credential. Injected rather than reached
   * for, so a headless test drives the same lane a browser download does. */
  readonly saveFile?: (file: Readonly<{ fileName: string; contents: string; mediaType: string }>) => Promise<void>;
  /** 🔌️ Installs a delegation's credential for MCP clients; see {@link HubConnectionPortV1.installAgentCredential}. */
  readonly installAgentCredential?: (request: AgentCredentialInstallRequestV1, signal: AbortSignal) => Promise<AgentCredentialInstallReceiptV1>;
  readonly uninstallAgentCredential?: (delegationId: string) => Promise<void>;
  /** 🎫️ Where the capability lives. Without one the port keeps it in its own closure. */
  readonly capability?: HubConnectionCapabilityCellV1;
}): HubConnectionPortV1 {
  const cell = options.capability ?? hubConnectionCapabilityCellV1(null);
  const announce = (next: Readonly<{ token: string; userId: string }> | null): void => cell.write(next);
  const authorized = (json: boolean): Record<string, string> => {
    const held = cell.read();
    return { ...(json ? { "content-type": "application/json" } : {}), ...(held === null ? {} : { authorization: `Bearer ${held.token}` }) };
  };
  const answer = async (response: HubFetchResponseV1): Promise<{ status: number; body: string; retryAfterHeader: string | null }> => ({
    status: response.status,
    body: await response.text(),
    retryAfterHeader: response.headers.get("retry-after"),
  });
  return {
    bootstrapOrigin: options.bootstrapOrigin,
    storage: options.storage,
    deviceInstanceId: options.deviceInstanceId,
    clientClass: options.clientClass,
    heldCapability: () => {
      const held = cell.read();
      return held === null ? null : { userId: held.userId };
    },
    ...(options.writeClipboard === undefined ? {} : { writeClipboard: options.writeClipboard }),
    signIn: {
      mint: async (origin, body, signal) => {
        const result = await answer(await options.request(`${origin}/auth/sessions`, { method: "POST", headers: { "content-type": "application/json" }, body }, signal));
        if (result.status === 200) {
          try {
            const minted = parseHubSessionMintResultV1(result.body);
            announce({ token: minted.token, userId: minted.userId });
          } catch {
            announce(null);
          }
        }
        return result;
      },
      read: async (origin, signal) => {
        const result = await answer(await options.request(`${origin}/auth/sessions/me`, { method: "GET", headers: authorized(false) }, signal));
        if (result.status === 401) announce(null);
        return result;
      },
      end: async (origin, signal) => {
        const result = await answer(await options.request(`${origin}${HUB_SESSION_SIGN_OUT_PATH_V1}`, { method: "POST", headers: authorized(false) }, signal));
        announce(null);
        return result;
      },
    },
    readSpaceMembers: async (origin, spaceId, signal) => {
      const response = await options.request(`${origin}/directory/spaces/${encodeURIComponent(spaceId)}`, { method: "GET", headers: authorized(false) }, signal);
      if (response.status !== 200) return [];
      return parseHubSpaceMemberRowsV1(await response.text());
    },
    listSpaces: async (origin, signal) => {
      const response = await options.request(`${origin}/directory/spaces`, { method: "GET", headers: authorized(false) }, signal);
      if (response.status !== 200) throw new Error("hub.spaces.unavailable");
      return options.parseSpaces(await response.text());
    },
    submitCommand: async (origin, command, signal) => {
      const sealed = options.sealCommand(command);
      const response = await options.request(`${origin}/directory/commands`, { method: "POST", headers: authorized(true), body: sealed.body }, signal);
      if (!directoryCommandAnsweredV1(response.status)) throw new Error("hub.command.refused");
      return sealed.parseReceipt(await response.text());
    },
    redeemInvite: async (origin, token, signal) => {
      const response = await options.request(`${origin}/directory/invites/${encodeURIComponent(token)}/redeem`, { method: "POST", headers: authorized(false) }, signal);
      return { status: response.status };
    },
    listAgentDelegations: async (origin, spaceId, signal) => {
      const response = await options.request(`${origin}${agentDelegationListPathV1(spaceId)}`, { method: "GET", headers: authorized(false) }, signal);
      const text = await response.text();
      if (response.status !== 200) throw new AgentDelegationRefusalV1(agentDelegationErrorFromResponseV1(response.status, text));
      return parseAgentDelegationListV1(text);
    },
    createAgentDelegation: async (origin, body, signal) => {
      const response = await options.request(`${origin}${AGENT_DELEGATION_PATH_V1}`, { method: "POST", headers: authorized(true), body }, signal);
      const text = await response.text();
      if (response.status !== 201) throw new AgentDelegationRefusalV1(agentDelegationErrorFromResponseV1(response.status, text));
      return parseAgentDelegationReceiptV1(text);
    },
    revokeAgentDelegation: async (origin, delegationId, signal) => {
      const response = await options.request(`${origin}${agentDelegationRevokePathV1(delegationId)}`, { method: "POST", headers: authorized(false) }, signal);
      if (response.status !== 204) throw new AgentDelegationRefusalV1(agentDelegationErrorFromResponseV1(response.status, await response.text()));
    },
    ...(options.saveFile === undefined ? {} : { saveFile: options.saveFile }),
    ...(options.installAgentCredential === undefined ? {} : { installAgentCredential: options.installAgentCredential }),
    ...(options.uninstallAgentCredential === undefined ? {} : { uninstallAgentCredential: options.uninstallAgentCredential }),
  };
}

/** 🏠️ The bootstrap connection this device always offers, exported so a shell and a story build the
 * same initial book. */
export { localBootstrapHubConnectionV1 };
//#endregion 🔖️FetchPort
