// #region 🧲️Header
// 💻️ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🤖️AgentDelegations/📖️stories/🧪️.story.tsx
// Specs: The agent-delegation surface in every state — delegations at rest, the one-time credential
// disclosure, a download the browser refused, a spectator who may not delegate, no space open, a
// refusal from the hub, and phone width.
// Summary: `AgentDelegations` is purely presentational, so each story is rows plus a phase; no hub,
// no transport and no `useHubConnection` are involved. The credential shown here is a synthetic
// token of the right shape, never a real capability.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react-vite";
import { AgentDelegations } from "@semio-tech/framework-renderer-react";
// #endregion 🔌️Adapters

// #region 🤖️AgentDelegations
const NOW = Date.UTC(2026, 8, 20, 9, 0, 0);
const TOKEN = `delegation.v1.${"a".repeat(32)}.${"b".repeat(64)}`;

const ROWS = [
  { delegationId: "dlg_drafting", agentPrincipalId: "agent:dlg_drafting", agentLabel: "Drafting agent", spaceId: "space-mine", audience: "edit" as const, createdAtMs: NOW - 86_400_000, expiresAtMs: NOW + 518_400_000, revoked: false, lastUsedAtMs: NOW - 3_600_000, state: "live" as const },
  { delegationId: "dlg_survey", agentPrincipalId: "agent:dlg_survey", agentLabel: "Survey reader", spaceId: "space-mine", audience: "read" as const, createdAtMs: NOW - 172_800_000, expiresAtMs: NOW + 86_400_000, revoked: false, lastUsedAtMs: null, state: "live" as const },
  { delegationId: "dlg_old", agentPrincipalId: "agent:dlg_old", agentLabel: "Retired helper", spaceId: "space-mine", audience: "edit" as const, createdAtMs: NOW - 2_592_000_000, expiresAtMs: NOW - 86_400_000, revoked: true, lastUsedAtMs: NOW - 2_000_000_000, state: "revoked" as const },
];

const CREDENTIAL = {
  receipt: { delegationId: "dlg_drafting", agentPrincipalId: "agent:dlg_drafting", agentLabel: "Drafting agent", spaceId: "space-mine", audience: "edit" as const, expiresAtMs: NOW + 518_400_000, token: TOKEN },
  file: { fileName: "semio-agent-drafting-agent.json", contents: `{\n  "schema": "semio.hub.agent-credential/v1",\n  "hubOrigin": "https://hub.example.org",\n  "spaceId": "space-mine",\n  "audience": "edit",\n  "token": "${TOKEN}"\n}\n`, mediaType: "application/json" },
  command: "semio-os-mcp stdio --hub https://hub.example.org --space space-mine --credential-file <path>",
  save: "idle" as const,
};

const meta = {
  title: "💻️os⚛️react/AgentDelegations",
  component: AgentDelegations,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: {
    spaceId: "space-mine",
    rows: ROWS,
    phase: "ready",
    error: null,
    credential: null,
    signedIn: true,
    canDelegate: true,
    onRefresh: () => undefined,
    onCreate: () => undefined,
    onDownloadCredential: () => undefined,
    onDismissCredential: () => undefined,
    onRevoke: () => undefined,
  },
} satisfies Meta<typeof AgentDelegations>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const FirstRun: Story = { args: { rows: [] } };

export const Loading: Story = { args: { rows: [], phase: "loading" } };

export const CredentialShownOnce: Story = { args: { credential: CREDENTIAL } };

export const DownloadRefused: Story = { args: { credential: { ...CREDENTIAL, save: "failed" } } };

export const SpectatorCannotDelegate: Story = { args: { canDelegate: false, rows: [] } };

export const NoSpaceOpen: Story = { args: { spaceId: null, rows: [] } };

export const HubRefused: Story = { args: { phase: "failed", error: "forbidden" } };

export const SignedOut: Story = { args: { signedIn: false, rows: [] } };

export const Phone: Story = { parameters: { viewport: { defaultViewport: "mobile1" } } };
// #endregion 🤖️AgentDelegations
