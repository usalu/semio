/** 🔌️ HubConnection contract: the authority recheck bound, the injected port, the delegation refusal, the credential, member, invitation and redemption rows, the connection value and the operation owner. */
import type { DirectoryCommand, DirectoryCommandReceiptV1, DirectorySpaceKind, DirectorySpaceListEntryV1, DirectorySpaceRole, DirectorySpaceVisibility } from "@semio-tech/framework/directory/schema";
import { type HubConnectionBookV1, type HubConnectionStorageV1, type HubConnectionV1, type HubSessionAuthorityV1, type HubSessionStateV1, type HubSignInClientClassV1, type HubSignInErrorCodeV1, type HubSignInTransportV1 } from "@semio-tech/framework/directory/sign-in";
import { type InviteRedemptionErrorCodeV1, type SpaceBrowserPhaseV1, type SpaceMemberPresenceV1, type SpaceRowV1 } from "@semio-tech/framework/directory/spaces";
import { type AgentCredentialInstallReceiptV1, type AgentCredentialInstallRequestV1, type AgentAudienceV1, type AgentDelegationErrorCodeV1, type AgentDelegationPhaseV1, type AgentDelegationReceiptV1, type AgentDelegationRowV1, type AgentDelegationSummaryV1 } from "@semio-tech/framework/directory/delegations";

/** ⏳️ The longest a single re-authentication timer is allowed to sleep. A session TTL may be a year
 * (`MAX_SESSION_TTL_SECS`), which overflows `setTimeout`'s 32-bit delay and fires immediately; the
 * timer re-arms from the remaining deadline on every wake instead. */
export const HUB_SESSION_AUTHORITY_RECHECK_MAX_MS = 30 * 60 * 1000;

//#region 🔖️Port
/** 🔌️ Everything `useHubConnection` needs from the outside world. `signIn` is the auth transport;
 * the three directory calls are the space/command/invite lanes. A fake implementing exactly this
 * drives every test, and the shell's adapter is {@link createHubConnectionFetchPortV1}. */
export interface HubConnectionPortV1 {
  readonly signIn: HubSignInTransportV1;
  readonly storage: HubConnectionStorageV1 | null;
  readonly bootstrapOrigin: string;
  readonly deviceInstanceId: string;
  readonly clientClass: HubSignInClientClassV1;
  /** 🎫️ The session this browsing context holds at the moment of asking — the one a previous page load left behind, or
   * the one an earlier mount of this surface minted. Read on every mount, so a surface reopened after a sign-in starts
   * signed in instead of saying "Not signed in to a hub" above the signed-in human's own spaces (ticket 26/09/23 U5, G10 S4
   * relay: the value used to be captured when the port was built, before anyone had signed in). The hub still has the last
   * word: the hook confirms it with one `GET /auth/sessions/me` before believing it. */
  heldCapability?(): Readonly<{ userId: string }> | null;
  listSpaces(origin: string, signal: AbortSignal): Promise<readonly DirectorySpaceListEntryV1[]>;
  /** 👥️ `GET /directory/spaces/{id}` — the hub's own administration page, whose `members` window is
   * the only authoritative roster. A non-member's hub answers `404`, which reaches the caller as an
   * empty roster rather than an error. */
  readSpaceMembers(origin: string, spaceId: string, signal: AbortSignal): Promise<readonly HubSpaceMemberRowV1[]>;
  submitCommand(origin: string, command: DirectoryCommand, signal: AbortSignal): Promise<DirectoryCommandReceiptV1>;
  redeemInvite(origin: string, token: string, signal: AbortSignal): Promise<Readonly<{ status: number }>>;
  writeClipboard?(text: string): Promise<void>;
  /** 🤖️ `GET /auth/agent-delegations?space=<id>` — this human's agent delegations in one space,
   * revoked ones included so the pane can show what was withdrawn. Never a token, never a selector. */
  listAgentDelegations(origin: string, spaceId: string, signal: AbortSignal): Promise<readonly AgentDelegationSummaryV1[]>;
  /** 🤖️ `POST /auth/agent-delegations` — the one call that ever yields a delegation capability, and
   * it yields it exactly once. The receipt is handed straight to the pane and never stored. */
  createAgentDelegation(origin: string, body: string, signal: AbortSignal): Promise<AgentDelegationReceiptV1>;
  /** 🤖️ `POST /auth/agent-delegations/{id}/revoke` — the withdrawal command, which cascades to every
   * live agent session minted from it. */
  revokeAgentDelegation(origin: string, delegationId: string, signal: AbortSignal): Promise<void>;
  /** 📄️ Hands the human a file to save. Separate from the clipboard port because a credential must
   * end up in a file the agent can read, not in a paste buffer. */
  saveFile?(file: Readonly<{ fileName: string; contents: string; mediaType: string }>): Promise<void>;
  /** 🔌️ Installs one delegation's credential file owner-only where `semio-os-mcp` reads it and
   * answers with its absolute path and this host's MCP launcher. Absent on a host that cannot write
   * files for the human (a plain browser); the pane then offers the download and the command. */
  installAgentCredential?(request: AgentCredentialInstallRequestV1, signal: AbortSignal): Promise<AgentCredentialInstallReceiptV1>;
  /** 🗑️ Removes an installed credential once its delegation is withdrawn. */
  uninstallAgentCredential?(delegationId: string): Promise<void>;
}

/** 🤖️ One refusal carrying the hub's own code, so the pane names a cause instead of a status. */
export class AgentDelegationRefusalV1 extends Error {
  readonly code: AgentDelegationErrorCodeV1;

  constructor(code: AgentDelegationErrorCodeV1) {
    super(`hub.agent-delegation.${code}`);
    this.name = "AgentDelegationRefusalV1";
    this.code = code;
  }
}

/** 🎁️ The one-time delegation capability, held here and nowhere else — the invitation lane's twin.
 * `receipt.token` never enters the connection book, a URL or a log line: it is handed to
 * `🤖️AgentDelegations` only while the human is looking at it, and `dismissDelegation` drops it. */
export interface HubAgentCredentialV1 {
  readonly receipt: AgentDelegationReceiptV1;
  readonly file: Readonly<{ fileName: string; contents: string; mediaType: string }>;
  readonly command: string;
  readonly save: "idle" | "saved" | "failed";
  /** 🔌️ The MCP client configuration for this delegation, once the host installed its credential. */
  readonly mcpClient: HubAgentMcpClientV1;
}

/** 🔌️ Where setting up an MCP client for one delegation stands. `unavailable` is a host that cannot
 * install files (the pane offers the download and command instead); `config` is the exact text a
 * human pastes into their client's configuration. */
export interface HubAgentMcpClientV1 {
  readonly phase: "idle" | "installing" | "ready" | "copied" | "unavailable" | "failed";
  readonly config: string | null;
}

/** 👥️ One roster row as the hub's space-administration page serves it
 * (`DirectorySpaceAdministrationMemberRowV1`). Declared here so the port has no dependency on the
 * whole administration page type. */
export interface HubSpaceMemberRowV1 {
  readonly userId: string;
  readonly displayName: string;
  readonly email: string;
  readonly role: DirectorySpaceRole;
  readonly owner: boolean;
}

/** 🎟️ The one-shot invitation capability, held here and nowhere else. `token` never enters the
 * connection book, a URL, a log line or React's devtools-visible props of any persisted element —
 * it is handed to `🏘️SpaceBrowser` only while the human is looking at it. */
export interface HubInviteCapabilityV1 {
  readonly spaceId: string;
  readonly link: string;
  readonly copy: "idle" | "copied" | "failed";
}

export interface HubRedemptionStateV1 {
  readonly phase: "idle" | "redeeming" | "redeemed" | "failed";
  readonly error: InviteRedemptionErrorCodeV1 | null;
}

/** 🏘️ The whole hub surface state one shell mounts. */
export interface HubConnectionValueV1 {
  readonly book: HubConnectionBookV1;
  readonly connection: HubConnectionV1;
  readonly session: HubSessionStateV1;
  /** 🪪️ The last `GET /auth/sessions/me` answer, or `null` before the first read. */
  readonly authority: HubSessionAuthorityV1 | null;
  readonly members: readonly SpaceMemberPresenceV1[];
  readonly rows: readonly SpaceRowV1[];
  readonly spacesPhase: SpaceBrowserPhaseV1;
  readonly search: string;
  readonly invite: HubInviteCapabilityV1 | null;
  readonly redemption: HubRedemptionStateV1;
  /** 🤖️ The agent delegations of the space currently being watched, newest live first. */
  readonly delegations: readonly AgentDelegationRowV1[];
  readonly delegationPhase: AgentDelegationPhaseV1;
  readonly delegationError: AgentDelegationErrorCodeV1 | null;
  /** 🎁️ The delegation capability minted a moment ago, readable exactly once. */
  readonly agentCredential: HubAgentCredentialV1 | null;
  selectConnection(id: string): void;
  addRemoteHub(typed: string, label: string): HubSignInErrorCodeV1 | "invalid-origin" | null;
  forgetHub(id: string): void;
  signIn(credential: Readonly<{ email: string; password: string }>): void;
  cancelSignIn(): void;
  signOut(): void;
  refreshSpaces(): void;
  /** 👥️ Re-reads the roster for one space; `null` clears it. */
  watchSpaceMembers(spaceId: string | null): void;
  setSearch(query: string): void;
  createSpace(name: string, kind: DirectorySpaceKind, visibility: DirectorySpaceVisibility): void;
  archiveSpace(spaceId: string): void;
  createInvite(spaceId: string, role: DirectorySpaceRole, ttlSecs: number): void;
  copyInvite(): void;
  dismissInvite(): void;
  redeemInvite(typed: string): void;
  refreshDelegations(): void;
  createDelegation(agentLabel: string, audience: AgentAudienceV1, ttlSecs: number): void;
  downloadAgentCredential(): void;
  dismissAgentCredential(): void;
  installAgentMcpClient(): void;
  copyAgentMcpClientConfig(): void;
  revokeDelegation(delegationId: string): void;
}

export interface HubConnectionOperationOwnerV1 {
  readonly generation: number;
  readonly connectionId: string;
  readonly origin: string;
}

/** 🪪️ Accepts a completion only while both its connection identity and selection generation remain current. */
export function hubConnectionOperationOwnerCurrentV1(owner: HubConnectionOperationOwnerV1, generation: number, connectionId: string): boolean {
  return owner.generation === generation && owner.connectionId === connectionId;
}
//#endregion 🔖️Port
