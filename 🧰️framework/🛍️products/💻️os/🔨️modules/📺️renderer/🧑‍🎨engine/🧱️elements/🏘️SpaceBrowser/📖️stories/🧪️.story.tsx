// #region 🧲️Header
// 💻️ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏘️SpaceBrowser/📖️stories/🧪️.story.tsx
// Specs: The end-user spaces surface in every state — a populated roster, the first-run empty state,
// a stale projection from a lost connection, a live invitation link, a refused redemption, and phone width.
// Summary: `SpaceBrowser` is purely presentational, so each story is rows plus a phase; no hub, no
// transport and no `useHubConnection` are involved.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react-vite";
import { SpaceBrowser } from "@semio-tech/framework-renderer-react";
import { spaceMemberPresenceV1, spaceRowsV1, type DirectorySpaceListEntryV1 } from "@semio-tech/framework-os";
// #endregion 🔌️Adapters

// #region 🏘️SpaceBrowser
const ENTRIES: readonly DirectorySpaceListEntryV1[] = [
  { access: "author", space: { id: "space-mine", name: "Kurokawa atelier", kind: "atelier", visibility: "private", ownerUserId: "usr_ada", role: "author", memberCount: 3, documentCount: 12, activeConnections: 2, createdAtMs: 1, updatedAtMs: 900 } },
  { access: "member", space: { id: "space-shared", name: "Tange studio", kind: "studio", visibility: "private", ownerUserId: "usr_bo", role: "spectator", memberCount: 8, documentCount: 40, activeConnections: 5, createdAtMs: 1, updatedAtMs: 700 } },
  { access: "public", space: { id: "space-commons", name: "Metabolism commons", kind: "studio", visibility: "public", memberCount: 120, documentCount: 300, createdAtMs: 1, updatedAtMs: 500 } },
];

const MEMBERS = spaceMemberPresenceV1(
  [
    { userId: "usr_ada", displayName: "Kisho Kurokawa", email: "kisho@example.invalid", role: "author", owner: true },
    { userId: "usr_bo", displayName: "Kenzo Tange", email: "kenzo@example.invalid", role: "author", owner: false },
    { userId: "usr_cy", displayName: "Fumihiko Maki", email: "fumihiko@example.invalid", role: "spectator", owner: false },
  ],
  ["usr_ada", "usr_bo"],
);

const meta = {
  title: "💻️os⚛️react/SpaceBrowser",
  component: SpaceBrowser,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: {
    rows: spaceRowsV1(ENTRIES),
    activeSpaceId: "space-mine",
    phase: "ready",
    search: "",
    members: MEMBERS,
    invite: null,
    redemption: { phase: "idle", error: null },
    signedIn: true,
    onSearch: () => undefined,
    onOpenSpace: () => undefined,
    onRefresh: () => undefined,
    onCreateSpace: () => undefined,
    onArchiveSpace: () => undefined,
    onCreateInvite: () => undefined,
    onCopyInvite: () => undefined,
    onDismissInvite: () => undefined,
    onRedeemInvite: () => undefined,
  },
} satisfies Meta<typeof SpaceBrowser>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const FirstRun: Story = { args: { rows: [], members: [] } };

export const Loading: Story = { args: { rows: [], members: [], phase: "loading" } };

export const StaleAfterConnectionLoss: Story = { args: { phase: "stale" } };

export const InvitationReady: Story = { args: { invite: { spaceId: "space-mine", link: "https://hub.example.org/#semio-invite=abcDEF012", copy: "idle" } } };

export const RedemptionRefused: Story = { args: { redemption: { phase: "failed", error: "expired-invite" } } };

export const SignedOutSpectator: Story = { args: { signedIn: false, activeSpaceId: "space-commons" } };

export const Phone: Story = { parameters: { viewport: { defaultViewport: "mobile1" } } };
// #endregion 🏘️SpaceBrowser
