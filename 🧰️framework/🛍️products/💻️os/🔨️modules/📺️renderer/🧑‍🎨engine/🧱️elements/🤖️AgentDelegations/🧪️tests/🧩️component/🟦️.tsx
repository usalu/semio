// #region 🧲️Header
/** @emoji 🤖️ Laws for the agent-delegation surface (ticket
 * `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END` slice M6b, spec §6 of
 * `📓️m6-agent-principal-in-hub-space.md`). Independent oracles: the *production* credential-file
 * renderer is held against the exact shape `semio-os-mcp --credential-file` decodes
 * (`🌉️mcp/🤖️agent-credential/🦀️.rs`'s `AgentCredentialV1::decode` — schema, the five keys, the
 * delegation token's 111-byte grammar), and the request builder against the hub's own
 * `CreateAgentDelegationRequestV1::verify` bounds. A builder that drifts from either fails here, not
 * at the hub. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { cleanup, fireEvent, render, screen, waitFor } from "@semio-tech/ui-react/test";
import { setUiLocale } from "@semio-tech/ui-react";
import { useEffect, type ReactElement } from "react";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  AGENT_CREDENTIAL_SCHEMA_V1,
  AGENT_DELEGATION_MAX_TTL_SECS_V1,
  AGENT_DELEGATION_MIN_TTL_SECS_V1,
  AGENT_DELEGATION_TTL_CHOICES_SECS_V1,
  AGENT_LABEL_MAX_BYTES_V1,
  actorDisplayFromRosterV1,
  actorDisplayV1,
  agentCredentialCommandV1,
  agentCredentialFileNameV1,
  agentCredentialFileV1,
  agentDelegationErrorFromResponseV1,
  agentDelegationErrorFromStatusV1,
  agentDelegationListPathV1,
  agentDelegationRevocableV1,
  agentDelegationRevokePathV1,
  agentDelegationRowsV1,
  agentDelegationStateV1,
  agentPrincipalIdV1,
  createAgentDelegationBodyV1,
  isAgentDelegationTokenV1,
  isAgentPrincipalV1,
  parseAgentDelegationListV1,
  parseAgentDelegationReceiptV1,
  agentCredentialInstallFileNameV1,
  agentCredentialInstallRequestV1,
  agentMcpClientConfigJsonV1,
  agentMcpClientConfigV1,
  parseAgentCredentialInstallReceiptV1,
  type AgentCredentialInstallRequestV1,
  type AgentDelegationReceiptV1,
  type AgentDelegationRowV1,
  type AgentDelegationSummaryV1,
} from "../../../../../../📇️directory/🤖️delegations/🟦️.ts";
import { AgentDelegations, agentDelegationInstantV1 } from "../../🟦️.tsx";
import Ajv from "ajv";
import directorySchema from "../../../../../../📇️directory/🧬️schema/🔣️.json" with { type: "json" };
import mcpClientLaw from "../../../../../../📇️directory/🤖️delegations/🧫️fixtures/🔌️mcp-client-config.json" with { type: "json" };
import { useHubConnection, type HubConnectionPortV1 } from "../../../🔗️HubConnection/🟦️.tsx";
import { type HubConnectionStorageV1 } from "../../../../../../📇️directory/🔐️sign-in/🟦️.ts";
// #endregion 🔌️Adapters

afterEach(cleanup);
beforeEach(async () => {
  await setUiLocale("en");
});

//#region 🧫️Doubles
const ORIGIN = "http://127.0.0.1:7777";
const SPACE = "space-mine";
const NOW = Date.UTC(2026, 8, 20, 9, 0, 0);
const TOKEN = `delegation.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
const IDLE_MCP = { phase: "idle", config: null } as const;
const LAUNCHER = { command: "/usr/local/bin/bun", args: ["/home/ada/semio/📜️script.ts", "dev", "mcp", "stdio", "os"] } as const;

function summary(overrides: Partial<AgentDelegationSummaryV1> = {}): AgentDelegationSummaryV1 {
  return {
    delegationId: "dlg_one",
    agentPrincipalId: "agent:dlg_one",
    agentLabel: "Drafting agent",
    spaceId: SPACE,
    audience: "edit",
    createdAtMs: NOW - 86_400_000,
    expiresAtMs: NOW + 86_400_000,
    revoked: false,
    lastUsedAtMs: NOW - 3_600_000,
    ...overrides,
  };
}

function receipt(overrides: Partial<AgentDelegationReceiptV1> = {}): AgentDelegationReceiptV1 {
  return { delegationId: "dlg_one", agentPrincipalId: "agent:dlg_one", agentLabel: "Drafting agent", spaceId: SPACE, audience: "edit", expiresAtMs: NOW + 86_400_000, token: TOKEN, ...overrides };
}

function receiptBody(overrides: Record<string, unknown> = {}): string {
  return JSON.stringify({ schema: "semio.hub.auth.agent-delegation-receipt/v1", delegationId: "dlg_one", agentPrincipalId: "agent:dlg_one", agentLabel: "Drafting agent", spaceId: SPACE, audience: "edit", expiresAtMs: NOW + 86_400_000, token: TOKEN, ...overrides });
}

function rows(...summaries: readonly AgentDelegationSummaryV1[]): readonly AgentDelegationRowV1[] {
  return agentDelegationRowsV1(summaries.length === 0 ? [summary()] : summaries, NOW);
}

function memoryStorage(): HubConnectionStorageV1 {
  const entries = new Map<string, string>();
  return { getItem: (key) => entries.get(key) ?? null, setItem: (key, value) => void entries.set(key, value) };
}

/** 🧫️ A port that records every delegation call the hook makes and answers from a live list, so a
 * create/revoke law observes the listing the human would see afterwards. */
function delegationPort(options: { readonly save?: "ok" | "fail"; readonly install?: "ok" | "fail" } = {}): HubConnectionPortV1 & {
  readonly created: string[];
  readonly revoked: string[];
  readonly listed: string[];
  readonly saved: { fileName: string; contents: string }[];
  readonly installed: AgentCredentialInstallRequestV1[];
  readonly uninstalled: string[];
  readonly copied: string[];
} {
  const created: string[] = [];
  const revoked: string[] = [];
  const listed: string[] = [];
  const saved: { fileName: string; contents: string }[] = [];
  const installed: AgentCredentialInstallRequestV1[] = [];
  const uninstalled: string[] = [];
  const copied: string[] = [];
  let live: AgentDelegationSummaryV1[] = [summary()];
  return {
    created,
    revoked,
    listed,
    saved,
    installed,
    uninstalled,
    copied,
    writeClipboard: async (text) => {
      copied.push(text);
    },
    ...(options.install === undefined
      ? {}
      : {
          installAgentCredential: async (request: AgentCredentialInstallRequestV1) => {
            if (options.install === "fail") throw new Error("host refused");
            installed.push(request);
            return { schema: "semio.os.agent-credential-install-receipt/v1" as const, credentialPath: `/home/ada/.semio/agent/credentials/${agentCredentialInstallFileNameV1(request.delegationId)}`, launcher: LAUNCHER };
          },
          uninstallAgentCredential: async (delegationId: string) => {
            uninstalled.push(delegationId);
          },
        }),
    signIn: {
      mint: async () => ({ status: 401, body: "", retryAfterHeader: null }),
      read: async () => ({ status: 401, body: "", retryAfterHeader: null }),
      end: async () => ({ status: 204, body: "", retryAfterHeader: null }),
    },
    storage: memoryStorage(),
    bootstrapOrigin: ORIGIN,
    deviceInstanceId: "device-m6b",
    clientClass: "browser",
    listSpaces: async () => [],
    readSpaceMembers: async () => [],
    submitCommand: async () => {
      throw new Error("unused");
    },
    redeemInvite: async () => ({ status: 200 }),
    listAgentDelegations: async (_origin, spaceId) => {
      listed.push(spaceId);
      return live;
    },
    createAgentDelegation: async (_origin, body) => {
      created.push(body);
      live = [summary({ delegationId: "dlg_new", agentPrincipalId: "agent:dlg_new", agentLabel: "New agent", lastUsedAtMs: null }), ...live];
      return receipt({ delegationId: "dlg_new", agentPrincipalId: "agent:dlg_new", agentLabel: "New agent" });
    },
    revokeAgentDelegation: async (_origin, delegationId) => {
      revoked.push(delegationId);
      live = live.map((row) => (row.delegationId === delegationId ? { ...row, revoked: true } : row));
    },
    saveFile: options.save === undefined ? undefined : async (file) => {
      if (options.save === "fail") throw new Error("denied");
      saved.push({ fileName: file.fileName, contents: file.contents });
    },
  };
}

/** 🔎️ The owned `screen` has no `queryByRole`, so an "is NOT offered" law reads the accessible names
 * off the rendered tree instead of asserting on a query that throws when it finds nothing. */
function namedButtons(container: HTMLElement, name: string): readonly Element[] {
  return Array.from(container.querySelectorAll("button")).filter((button) => button.getAttribute("aria-label") === name);
}

function Pane(props: Partial<Parameters<typeof AgentDelegations>[0]> = {}): ReactElement {
  return (
    <AgentDelegations
      spaceId={SPACE}
      rows={rows()}
      phase="ready"
      error={null}
      credential={null}
      signedIn
      canDelegate
      onRefresh={() => undefined}
      onCreate={() => undefined}
      onDownloadCredential={() => undefined}
      onDismissCredential={() => undefined}
      onInstallMcpClient={() => undefined}
      onCopyMcpClientConfig={() => undefined}
      onRevoke={() => undefined}
      {...props}
    />
  );
}
//#endregion 🧫️Doubles

//#region 🧬️Contract
describe("agent MCP client configuration law", () => {
  it("replays every fixture case through the product and holds each against the owned schema", () => {
    const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(directorySchema);
    const law = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/AgentMcpClientConfigLawV1` });
    const config = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/AgentMcpClientConfigV1` });
    const request = ajv.compile({ $ref: `${directorySchema.$id}#/$defs/AgentCredentialInstallRequestV1` });
    expect(law(mcpClientLaw), JSON.stringify(law.errors)).toBe(true);
    for (const row of mcpClientLaw.cases) {
      const receiptRow = row.receipt as AgentDelegationReceiptV1;
      const produced = agentMcpClientConfigV1(receiptRow, row.hubOrigin, parseAgentCredentialInstallReceiptV1(JSON.stringify(row.installed)));
      expect(produced, row.name).toEqual(row.expected);
      expect(config(produced), `${row.name}: ${JSON.stringify(config.errors)}`).toBe(true);
      expect(agentMcpClientConfigJsonV1(produced)).not.toContain(receiptRow.token);
      expect(agentCredentialInstallFileNameV1(receiptRow.delegationId)).toBe(row.fileName);
      const sealed = agentCredentialInstallRequestV1(receiptRow, row.hubOrigin);
      expect(request(sealed), JSON.stringify(request.errors)).toBe(true);
      expect(JSON.parse(sealed.contents)).toEqual({ schema: AGENT_CREDENTIAL_SCHEMA_V1, hubOrigin: row.hubOrigin, spaceId: receiptRow.spaceId, audience: receiptRow.audience, token: receiptRow.token });
    }
    for (const refusal of mcpClientLaw.refusals) {
      expect(() => agentCredentialInstallFileNameV1(refusal.delegationId), refusal.name).toThrow(refusal.error);
      expect(request({ schema: "semio.os.agent-credential-install/v1", delegationId: refusal.delegationId, contents: "{}" }), refusal.name).toBe(false);
    }
    expect(() => parseAgentCredentialInstallReceiptV1(JSON.stringify({ ...mcpClientLaw.cases[0]!.installed, credentialPath: "relative/path.json" }))).toThrow("directory.delegations.invalid-install-receipt");
  });
});

describe("agent delegation contract", () => {
  it("builds exactly the hub's create body and refuses everything outside its bounds", () => {
    const body = JSON.parse(createAgentDelegationBodyV1(SPACE, "  Drafting agent  ", "edit", AGENT_DELEGATION_TTL_CHOICES_SECS_V1[1])) as Record<string, unknown>;
    expect(body).toEqual({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: SPACE, agentLabel: "Drafting agent", audience: "edit", ttlSecs: 604800 });
    expect(Object.keys(body)).toEqual(["schema", "spaceId", "agentLabel", "audience", "ttlSecs"]);
    expect(() => createAgentDelegationBodyV1("", "a", "read", 3600)).toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "   ", "read", 3600)).toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "a b", "read", 3600)).toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "x".repeat(AGENT_LABEL_MAX_BYTES_V1 + 1), "read", 3600)).toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "a", "read", AGENT_DELEGATION_MIN_TTL_SECS_V1 - 1)).toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "a", "read", AGENT_DELEGATION_MAX_TTL_SECS_V1 + 1)).toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "a", "read", AGENT_DELEGATION_MIN_TTL_SECS_V1)).not.toThrow();
    expect(() => createAgentDelegationBodyV1(SPACE, "a", "read", AGENT_DELEGATION_MAX_TTL_SECS_V1)).not.toThrow();
  });

  it("renders the credential file `semio-os-mcp --credential-file` decodes, and nothing else", () => {
    const file = agentCredentialFileV1(receipt(), "https://hub.example.org");
    const decoded = JSON.parse(file.contents) as Record<string, unknown>;
    expect(Object.keys(decoded)).toEqual(["schema", "hubOrigin", "spaceId", "audience", "token"]);
    expect(decoded["schema"]).toBe(AGENT_CREDENTIAL_SCHEMA_V1);
    expect(decoded["hubOrigin"]).toBe("https://hub.example.org");
    expect(decoded["spaceId"]).toBe(SPACE);
    expect(decoded["audience"]).toBe("edit");
    expect(isAgentDelegationTokenV1(String(decoded["token"]))).toBe(true);
    expect(file.mediaType).toBe("application/json");
    expect(file.fileName).toBe("semio-agent-drafting-agent.json");
    expect(agentCredentialFileNameV1(receipt({ agentLabel: "///" }))).toBe("semio-agent-delegation.json");
    expect(agentCredentialCommandV1(receipt(), ORIGIN)).toBe(`semio-os-mcp stdio --hub ${ORIGIN} --space ${SPACE} --credential-file <path>`);
    expect(() => agentCredentialFileV1(receipt({ token: `${TOKEN}c` }), ORIGIN)).toThrow();
    expect(() => agentCredentialFileV1(receipt(), "")).toThrow();
  });

  it("pins the delegation token grammar the hub mints, one byte either way", () => {
    expect(isAgentDelegationTokenV1(TOKEN)).toBe(true);
    expect(TOKEN.length).toBe(111);
    expect(isAgentDelegationTokenV1(`${TOKEN}a`)).toBe(false);
    expect(isAgentDelegationTokenV1(TOKEN.slice(0, -1))).toBe(false);
    expect(isAgentDelegationTokenV1(TOKEN.toUpperCase())).toBe(false);
    expect(isAgentDelegationTokenV1(`session.v1.${"a".repeat(32)}.${"b".repeat(64)}`)).toBe(false);
  });

  it("refuses every receipt that is not exactly one, including a principal that does not match its id", () => {
    expect(parseAgentDelegationReceiptV1(receiptBody()).delegationId).toBe("dlg_one");
    for (const malformed of [
      "not json",
      JSON.stringify([]),
      receiptBody({ schema: "semio.hub.auth.agent-delegation-receipt/v2" }),
      receiptBody({ token: "nope" }),
      receiptBody({ audience: "admin" }),
      receiptBody({ agentPrincipalId: "agent:somebody-else" }),
      receiptBody({ expiresAtMs: "soon" }),
      receiptBody({ agentLabel: "" }),
    ]) {
      expect(() => parseAgentDelegationReceiptV1(malformed)).toThrow();
    }
  });

  it("drops a corrupt listing row instead of hiding every other delegation the human owns", () => {
    const listing = JSON.stringify({
      schema: "semio.hub.auth.agent-delegation-list/v1",
      delegations: [
        { delegationId: "dlg_a", agentLabel: "A", spaceId: SPACE, audience: "edit", createdAtMs: 1, expiresAtMs: 2, revoked: false, lastUsedAtMs: 3 },
        { delegationId: "dlg_b", agentLabel: "B", spaceId: SPACE, audience: "sideways", createdAtMs: 1, expiresAtMs: 2, revoked: false },
        { delegationId: "dlg_c", agentLabel: "C", spaceId: SPACE, audience: "read", createdAtMs: 4, expiresAtMs: 5, revoked: true },
      ],
    });
    const parsed = parseAgentDelegationListV1(listing);
    expect(parsed.map((row) => row.delegationId)).toEqual(["dlg_a", "dlg_c"]);
    expect(parsed[0]?.agentPrincipalId).toBe("agent:dlg_a");
    expect(parsed[0]?.lastUsedAtMs).toBe(3);
    expect(parsed[1]?.lastUsedAtMs).toBeNull();
    expect(parseAgentDelegationListV1("not json")).toEqual([]);
    expect(parseAgentDelegationListV1(JSON.stringify({ delegations: "no" }))).toEqual([]);
  });

  it("orders live delegations before expired and revoked ones, newest first", () => {
    const projected = agentDelegationRowsV1(
      [
        summary({ delegationId: "dlg_revoked", revoked: true, createdAtMs: NOW }),
        summary({ delegationId: "dlg_expired", expiresAtMs: NOW - 1, createdAtMs: NOW }),
        summary({ delegationId: "dlg_old", createdAtMs: NOW - 10_000 }),
        summary({ delegationId: "dlg_new", createdAtMs: NOW - 1_000 }),
      ],
      NOW,
    );
    expect(projected.map((row) => row.delegationId)).toEqual(["dlg_new", "dlg_old", "dlg_expired", "dlg_revoked"]);
    expect(projected.map((row) => row.state)).toEqual(["live", "live", "expired", "revoked"]);
    expect(agentDelegationStateV1(summary({ expiresAtMs: NOW }), NOW)).toBe("expired");
    expect(agentDelegationRevocableV1(projected[0]!)).toBe(true);
    expect(agentDelegationRevocableV1(projected[2]!)).toBe(true);
    expect(agentDelegationRevocableV1(projected[3]!)).toBe(false);
  });

  it("maps every hub status and error body to one closed code", () => {
    expect(agentDelegationErrorFromStatusV1(400)).toBe("malformed-request");
    expect(agentDelegationErrorFromStatusV1(401)).toBe("invalid-delegation");
    expect(agentDelegationErrorFromStatusV1(403)).toBe("forbidden");
    expect(agentDelegationErrorFromStatusV1(429)).toBe("rate-limited");
    expect(agentDelegationErrorFromStatusV1(503)).toBe("directory-unavailable");
    expect(agentDelegationErrorFromStatusV1(418)).toBe("unreachable");
    expect(agentDelegationErrorFromResponseV1(403, JSON.stringify({ schema: "semio.hub.auth.agent-error/v1", error: "delegation-revoked" }))).toBe("delegation-revoked");
    expect(agentDelegationErrorFromResponseV1(403, JSON.stringify({ schema: "semio.hub.auth.agent-error/v1", error: "invented" }))).toBe("forbidden");
    expect(agentDelegationErrorFromResponseV1(403, "<html>proxy</html>")).toBe("forbidden");
    expect(agentDelegationListPathV1("space a")).toBe("/auth/agent-delegations?space=space%20a");
    expect(agentDelegationRevokePathV1("dlg/1")).toBe("/auth/agent-delegations/dlg%2F1/revoke");
    expect(() => agentDelegationRevokePathV1("")).toThrow();
  });

  it("names an agent by its delegation label while leaving the actor identity untouched", () => {
    expect(agentPrincipalIdV1("dlg_one")).toBe("agent:dlg_one");
    expect(isAgentPrincipalV1("agent:dlg_one")).toBe(true);
    expect(isAgentPrincipalV1("user:usr_ada#s1")).toBe(false);
    const opaque = `hub.v1.${"3".repeat(64)}`;
    expect(actorDisplayV1(opaque, "Drafting agent", true)).toBe("agent:Drafting agent");
    expect(actorDisplayV1(opaque, "Kisho Kurokawa", false)).toBe("Kisho Kurokawa");
    expect(actorDisplayV1(opaque, null, false)).toBe(opaque);
    expect(actorDisplayV1(opaque, "   ", true)).toBe(`agent:${opaque}`);
    const roster = [
      { actor: opaque, label: "Drafting agent", isAgent: true },
      { actor: `hub.v1.${"4".repeat(64)}`, label: "Kisho Kurokawa" },
    ];
    expect(actorDisplayFromRosterV1(opaque, roster)).toBe("agent:Drafting agent");
    expect(actorDisplayFromRosterV1(`hub.v1.${"4".repeat(64)}`, roster)).toBe("Kisho Kurokawa");
    expect(actorDisplayFromRosterV1(`hub.v1.${"5".repeat(64)}`, roster)).toBe(`hub.v1.${"5".repeat(64)}`);
  });

  it("renders a timestamp two devices agree on", () => {
    expect(agentDelegationInstantV1(Date.UTC(2026, 8, 20, 9, 5, 0))).toBe("2026-09-20 09:05 UTC");
    expect(agentDelegationInstantV1(Number.NaN)).toBe("—");
  });
});
//#endregion 🧬️Contract

//#region 🖥️Surface
describe("agent delegation surface", () => {
  it("shows every delegation with its state, audience, principal and last use", () => {
    const view = render(<Pane rows={rows(summary(), summary({ delegationId: "dlg_never", agentPrincipalId: "agent:dlg_never", agentLabel: "Survey reader", audience: "read", lastUsedAtMs: null, createdAtMs: NOW - 200_000_000 }))} />);
    const listed = view.container.querySelectorAll("li[data-delegation-id]");
    expect(listed).toHaveLength(2);
    expect(listed[0]?.getAttribute("data-delegation-state")).toBe("live");
    expect(listed[0]?.getAttribute("data-delegation-audience")).toBe("edit");
    expect(listed[0]?.textContent).toContain("agent:dlg_one");
    expect(listed[0]?.textContent).toContain("Last used 2026-09-20 08:00 UTC");
    expect(listed[1]?.textContent).toContain("Never used");
    expect(screen.getByRole("list", { name: "Agents with access" })).toBeTruthy();
  });

  it("offers the create form only to an author with a space open, and never to a signed-out human", () => {
    const authored = render(<Pane />);
    expect(authored.container.querySelector("form[aria-label='Give an agent access']")).toBeTruthy();
    cleanup();
    const spectator = render(<Pane canDelegate={false} />);
    expect(spectator.container.querySelector("form[aria-label='Give an agent access']")).toBeNull();
    expect(spectator.container.textContent).toContain("Only an author of this space can give an agent access to it.");
    cleanup();
    const noSpace = render(<Pane spaceId={null} rows={[]} />);
    expect(noSpace.container.textContent).toContain("Open a space to manage the agents that work in it.");
    expect(noSpace.container.querySelector("li[data-delegation-id]")).toBeNull();
    cleanup();
    const signedOut = render(<Pane signedIn={false} rows={[]} />);
    expect(signedOut.container.textContent).toContain("Sign in to the hub to give an agent access.");
  });

  it("discloses the credential once, with its permission warning and the command that uses it", () => {
    const disclosed: string[] = [];
    const view = render(
      <Pane
        credential={{ receipt: receipt(), file: agentCredentialFileV1(receipt(), ORIGIN), command: agentCredentialCommandV1(receipt(), ORIGIN), save: "idle", mcpClient: IDLE_MCP }}
        onDownloadCredential={() => disclosed.push("download")}
      />,
    );
    const block = view.container.querySelector("[data-semio-hub-agent-credential]");
    expect(block?.getAttribute("data-semio-hub-agent-credential")).toBe("dlg_one");
    expect(block?.getAttribute("role")).toBe("status");
    expect(block?.textContent).toContain("It is shown once");
    expect(block?.textContent).toContain("chmod 600");
    expect(block?.textContent).toContain("Acts as agent:dlg_one");
    expect(view.container.querySelector("[data-semio-hub-agent-command]")?.textContent).toContain("--credential-file <path>");
    // 🔐️ The raw token is never rendered while the download lane is healthy.
    expect(view.container.textContent).not.toContain(TOKEN);
    fireEvent.click(screen.getByRole("button", { name: "Download credential file" }));
    expect(disclosed).toEqual(["download"]);
  });

  it("falls back to showing the file text only when the browser refused to save it", () => {
    const view = render(<Pane credential={{ receipt: receipt(), file: agentCredentialFileV1(receipt(), ORIGIN), command: agentCredentialCommandV1(receipt(), ORIGIN), save: "failed", mcpClient: IDLE_MCP }} />);
    expect(view.container.querySelector("[role='alert']")?.textContent).toContain("Could not save the file");
    expect(view.container.querySelector("[data-semio-hub-agent-credential-file]")?.textContent).toContain(TOKEN);
  });

  it("withdraws only behind a confirmation that says what withdrawal costs", () => {
    const revoked: string[] = [];
    render(<Pane onRevoke={(id) => revoked.push(id)} />);
    fireEvent.click(screen.getByRole("button", { name: "Withdraw Drafting agent" }));
    expect(revoked).toEqual([]);
    const confirmation = screen.getByRole("group", { name: "Withdraw access from Drafting agent?" });
    expect(confirmation.textContent).toContain("Its open sessions end at once");
    fireEvent.click(screen.getByRole("button", { name: "Keep access" }));
    expect(revoked).toEqual([]);
    fireEvent.click(screen.getByRole("button", { name: "Withdraw Drafting agent" }));
    fireEvent.click(screen.getByRole("button", { name: "Withdraw access — Withdraw access from Drafting agent?" }));
    expect(revoked).toEqual(["dlg_one"]);
  });

  it("never offers withdrawal for an already-withdrawn delegation", () => {
    const view = render(<Pane rows={rows(summary({ revoked: true }))} />);
    expect(view.container.querySelector("li[data-delegation-id]")?.getAttribute("data-delegation-state")).toBe("revoked");
    expect(namedButtons(view.container, "Withdraw Drafting agent")).toHaveLength(0);
  });

  it("names the cause of a refusal in an alert rather than a status code", () => {
    const view = render(<Pane phase="failed" error="forbidden" />);
    const alert = view.container.querySelector("[data-semio-hub-agent-error]");
    expect(alert?.getAttribute("data-semio-hub-agent-error")).toBe("forbidden");
    expect(alert?.getAttribute("role")).toBe("alert");
    expect(alert?.textContent).toContain("You may not manage agents in this space.");
    expect(view.container.querySelector("[data-semio-hub-agent-status]")?.getAttribute("aria-live")).toBe("polite");
  });

  it("is reachable by keyboard and by name, in en and de", async () => {
    const view = render(<Pane />);
    for (const control of Array.from(view.container.querySelectorAll("button, input, select"))) {
      expect(control.hasAttribute("disabled") || (control as HTMLElement).tabIndex >= 0).toBe(true);
    }
    expect(screen.getByRole("button", { name: "Refresh agent delegations" })).toBeTruthy();
    expect(view.container.querySelector("section")?.getAttribute("class")).toContain("w-full");
    expect(view.container.querySelector("section")?.getAttribute("class")).toContain("min-w-0");
    cleanup();
    await setUiLocale("de");
    const german = render(<Pane credential={{ receipt: receipt(), file: agentCredentialFileV1(receipt(), ORIGIN), command: agentCredentialCommandV1(receipt(), ORIGIN), save: "idle", mcpClient: IDLE_MCP }} />);
    expect(german.container.textContent).toContain("KI-Agenten");
    expect(german.container.textContent).toContain("chmod 600");
    expect(german.container.textContent).not.toContain("AI agent");
    expect(namedButtons(german.container, "Drafting agent zurückziehen")).toHaveLength(1);
    await setUiLocale("en");
  });
});
//#endregion 🖥️Surface

//#region 🔗️Hook
function DelegationHarness({ port, spaceId = SPACE }: { readonly port: HubConnectionPortV1; readonly spaceId?: string | null }): ReactElement {
  const hub = useHubConnection(port);
  const { watchSpaceMembers } = hub;
  useEffect(() => watchSpaceMembers(spaceId), [spaceId, watchSpaceMembers]);
  return (
    <div>
      <AgentDelegations
        spaceId={spaceId}
        rows={hub.delegations}
        phase={hub.delegationPhase}
        error={hub.delegationError}
        credential={hub.agentCredential}
        signedIn
        canDelegate
        onRefresh={hub.refreshDelegations}
        onCreate={hub.createDelegation}
        onDownloadCredential={hub.downloadAgentCredential}
        onDismissCredential={hub.dismissAgentCredential}
        onInstallMcpClient={hub.installAgentMcpClient}
        onCopyMcpClientConfig={hub.copyAgentMcpClientConfig}
        onRevoke={hub.revokeDelegation}
      />
      <output data-testid="phase">{hub.delegationPhase}</output>
      <output data-testid="count">{String(hub.delegations.length)}</output>
    </div>
  );
}

describe("agent delegation lane", () => {
  it("loads the open space's delegations, mints a new one, and shows it in the listing afterwards", async () => {
    const port = delegationPort({ save: "ok" });
    const view = render(<DelegationHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="count"]')?.textContent).toBe("1"));
    expect(port.listed).toEqual([SPACE]);
    fireEvent.change(view.container.querySelector("input[data-element-alias='os.hub.agent.name']")!, { target: { value: "New agent" } });
    fireEvent.click(screen.getByRole("button", { name: "Create delegation" }));
    await waitFor(() => expect(view.container.querySelector('[data-testid="count"]')?.textContent).toBe("2"));
    expect(JSON.parse(port.created[0] ?? "{}")).toEqual({ schema: "semio.hub.auth.agent-delegation-create/v1", spaceId: SPACE, agentLabel: "New agent", audience: "edit", ttlSecs: AGENT_DELEGATION_TTL_CHOICES_SECS_V1[1] });
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-credential]")).toBeTruthy());
    fireEvent.click(screen.getByRole("button", { name: "Download credential file" }));
    await waitFor(() => expect(port.saved).toHaveLength(1));
    expect(port.saved[0]?.fileName).toBe("semio-agent-new-agent.json");
    expect(JSON.parse(port.saved[0]?.contents ?? "{}")).toMatchObject({ schema: AGENT_CREDENTIAL_SCHEMA_V1, hubOrigin: ORIGIN, spaceId: SPACE, audience: "edit" });
  });

  it("re-reads the listing after a withdrawal, so the row shows as withdrawn without a reload", async () => {
    const port = delegationPort();
    const view = render(<DelegationHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="count"]')?.textContent).toBe("1"));
    fireEvent.click(screen.getByRole("button", { name: "Withdraw Drafting agent" }));
    fireEvent.click(screen.getByRole("button", { name: "Withdraw access — Withdraw access from Drafting agent?" }));
    await waitFor(() => expect(port.revoked).toEqual(["dlg_one"]));
    await waitFor(() => expect(view.container.querySelector("li[data-delegation-id]")?.getAttribute("data-delegation-state")).toBe("revoked"));
    expect(namedButtons(view.container, "Withdraw Drafting agent")).toHaveLength(0);
  });

  it("turns a fresh delegation into a working MCP client configuration and removes its credential on withdrawal", async () => {
    const port = delegationPort({ install: "ok" });
    const view = render(<DelegationHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="count"]')?.textContent).toBe("1"));
    fireEvent.change(view.container.querySelector("input[data-element-alias='os.hub.agent.name']")!, { target: { value: "New agent" } });
    fireEvent.click(screen.getByRole("button", { name: "Create delegation" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("idle"));
    fireEvent.click(screen.getByRole("button", { name: "Set up MCP client" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("ready"));
    expect(port.installed).toEqual([{ schema: "semio.os.agent-credential-install/v1", delegationId: "dlg_new", contents: agentCredentialFileV1(receipt({ delegationId: "dlg_new", agentPrincipalId: "agent:dlg_new", agentLabel: "New agent" }), ORIGIN).contents }]);
    const shown = view.container.querySelector("[data-semio-hub-agent-mcp-config]")?.textContent ?? "";
    const config = JSON.parse(shown);
    expect(config).toEqual({ mcpServers: { "semio-new-agent": { type: "stdio", command: LAUNCHER.command, args: [...LAUNCHER.args, "--hub", ORIGIN, "--space", SPACE, "--credential-file", "/home/ada/.semio/agent/credentials/semio-agent-dlg_new.json", "--scopes", "workspace.read,artifact.write,inference.execute"] } } });
    expect(shown, "the configuration names the credential file, never the secret").not.toContain(TOKEN);
    fireEvent.click(screen.getByRole("button", { name: "Copy MCP configuration" }));
    await waitFor(() => expect(port.copied).toEqual([shown]));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("copied"));
    fireEvent.click(screen.getByRole("button", { name: "Withdraw New agent" }));
    fireEvent.click(screen.getByRole("button", { name: "Withdraw access — Withdraw access from New agent?" }));
    await waitFor(() => expect(port.uninstalled).toEqual(["dlg_new"]));
    expect(view.container.querySelector("[data-semio-hub-agent-mcp-config]")).toBeNull();
  });

  it("says so when the host cannot install a credential, and when an install fails", async () => {
    const plain = render(<Pane credential={{ receipt: receipt(), file: agentCredentialFileV1(receipt(), ORIGIN), command: agentCredentialCommandV1(receipt(), ORIGIN), save: "idle", mcpClient: { phase: "unavailable", config: null } }} />);
    expect(plain.container.querySelector("[data-semio-hub-agent-mcp]")?.textContent).toContain("cannot install the credential for you");
    expect(namedButtons(plain.container, "Set up MCP client")).toHaveLength(0);
    cleanup();
    const port = delegationPort({ install: "fail" });
    const view = render(<DelegationHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="count"]')?.textContent).toBe("1"));
    fireEvent.change(view.container.querySelector("input[data-element-alias='os.hub.agent.name']")!, { target: { value: "New agent" } });
    fireEvent.click(screen.getByRole("button", { name: "Create delegation" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")).toBeTruthy());
    fireEvent.click(screen.getByRole("button", { name: "Set up MCP client" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("failed"));
    expect(view.container.querySelector("[data-semio-hub-agent-mcp] [role='alert']")?.textContent).toContain("could not be installed");
    expect(view.container.querySelector("[data-semio-hub-agent-mcp-config]")).toBeNull();
  });

  it("ends an install at its terminal phase when the host answers late, after the shell's session passed through no space and back", async () => {
    const answers: { resolve: (receipt: Awaited<ReturnType<NonNullable<HubConnectionPortV1["installAgentCredential"]>>>) => void }[] = [];
    const base = delegationPort({ install: "ok" });
    const port: HubConnectionPortV1 = {
      ...base,
      installAgentCredential: (request) => new Promise((resolve) => answers.push({ resolve: (receipt) => resolve({ ...receipt, credentialPath: `/home/ada/.semio/agent/credentials/${agentCredentialInstallFileNameV1(request.delegationId)}` }) })),
    };
    const view = render(<DelegationHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="count"]')?.textContent).toBe("1"));
    fireEvent.change(view.container.querySelector("input[data-element-alias='os.hub.agent.name']")!, { target: { value: "New agent" } });
    fireEvent.click(screen.getByRole("button", { name: "Create delegation" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("idle"));
    fireEvent.click(screen.getByRole("button", { name: "Set up MCP client" }));
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("installing"));
    view.rerender(<DelegationHarness port={port} spaceId={null} />);
    expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp"), "a session re-established under the pane keeps the credential").toBe("installing");
    view.rerender(<DelegationHarness port={port} />);
    expect(answers).toHaveLength(1);
    answers[0]!.resolve({ schema: "semio.os.agent-credential-install-receipt/v1", credentialPath: "", launcher: LAUNCHER });
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-mcp]")?.getAttribute("data-semio-hub-agent-mcp")).toBe("ready"));
    expect(JSON.parse(view.container.querySelector("[data-semio-hub-agent-mcp-config]")?.textContent ?? "{}").mcpServers["semio-new-agent"].args).toContain(SPACE);
    view.rerender(<DelegationHarness port={port} spaceId="space-other" />);
    await waitFor(() => expect(view.container.querySelector("[data-semio-hub-agent-credential]"), "another space's pane never shows this delegation's credential").toBeNull());
  }, 30_000);

  it("names every MCP client control in en and de", async () => {
    const ready = { phase: "ready" as const, config: "{}\n" };
    for (const [locale, install, copy, title] of [["en", "Set up MCP client", "Copy MCP configuration", "Connect an AI client"], ["de", "MCP-Client einrichten", "MCP-Konfiguration kopieren", "KI-Client verbinden"]] as const) {
      await setUiLocale(locale);
      const idle = render(<Pane credential={{ receipt: receipt(), file: agentCredentialFileV1(receipt(), ORIGIN), command: agentCredentialCommandV1(receipt(), ORIGIN), save: "idle", mcpClient: IDLE_MCP }} />);
      expect(namedButtons(idle.container, install)).toHaveLength(1);
      expect(idle.container.querySelector("[data-semio-hub-agent-mcp]")?.textContent).toContain(title);
      cleanup();
      const shown = render(<Pane credential={{ receipt: receipt(), file: agentCredentialFileV1(receipt(), ORIGIN), command: agentCredentialCommandV1(receipt(), ORIGIN), save: "idle", mcpClient: ready }} />);
      expect(namedButtons(shown.container, copy)).toHaveLength(1);
      cleanup();
    }
    await setUiLocale("en");
  });

  it("reports an unreachable hub rather than an empty listing that looks like no agents at all", async () => {
    const port: HubConnectionPortV1 = {
      ...delegationPort(),
      listAgentDelegations: async () => {
        throw new Error("offline");
      },
    };
    const view = render(<DelegationHarness port={port} />);
    await waitFor(() => expect(view.container.querySelector('[data-testid="phase"]')?.textContent).toBe("failed"));
    expect(view.container.querySelector("[data-semio-hub-agent-error]")?.getAttribute("data-semio-hub-agent-error")).toBe("unreachable");
  });
});
//#endregion 🔗️Hook
