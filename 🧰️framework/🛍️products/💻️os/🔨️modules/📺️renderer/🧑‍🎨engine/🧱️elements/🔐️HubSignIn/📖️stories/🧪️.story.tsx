// #region 🧲️Header
// 💻️ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔐️HubSignIn/📖️stories/🧪️.story.tsx
// Specs: Every state a human can reach while connecting to a hub — signed out, in flight, each
// refusal class, an expired session, offline, and signed in.
// Summary: `HubSignInPane` is purely presentational, so each story is one `HubSessionStateV1` over a
// two-entry connection book; no transport and no hub are involved.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react-vite";
import { HubSignInPane } from "@semio-tech/framework-renderer-react";
import type { HubConnectionBookV1, HubSessionStateV1 } from "@semio-tech/framework-os";
// #endregion 🔌️Adapters

// #region 🔐️HubSignIn
const BOOK: HubConnectionBookV1 = {
  schema: "semio.os.hub-connection-book.v1",
  connections: [
    { id: "local-bootstrap", label: "This device", origin: "http://127.0.0.1:7777", kind: "local-bootstrap", lastUserId: null },
    { id: "remote:https://hub.example.org", label: "Studio hub", origin: "https://hub.example.org", kind: "remote", lastUserId: "usr_ada" },
  ],
  selectedId: "remote:https://hub.example.org",
};

function session(overrides: Partial<HubSessionStateV1> = {}): HubSessionStateV1 {
  return { phase: "signed-out", connectionId: BOOK.selectedId, userId: null, expiresAtMs: null, role: null, error: null, retryAfterSeconds: null, offline: false, ...overrides };
}

const meta = {
  title: "💻️os⚛️react/HubSignIn",
  component: HubSignInPane,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
  args: {
    book: BOOK,
    session: session(),
    locale: "en",
    onSelectConnection: () => undefined,
    onAddHub: () => null,
    onForgetHub: () => undefined,
    onSignIn: () => undefined,
    onCancel: () => undefined,
    onSignOut: () => undefined,
  },
} satisfies Meta<typeof HubSignInPane>;

export default meta;

type Story = StoryObj<typeof meta>;

export const SignedOut: Story = {};

export const SigningIn: Story = { args: { session: session({ phase: "signing-in" }) } };

export const SignedIn: Story = { args: { session: session({ phase: "signed-in", userId: "usr_ada" }) } };

export const WrongCredentials: Story = { args: { session: session({ error: "invalid-credentials" }) } };

export const RateLimited: Story = { args: { session: session({ error: "rate-limited", retryAfterSeconds: 42 }) } };

export const HubUnreachable: Story = { args: { session: session({ error: "unreachable", offline: true }) } };

export const PasswordSignInDisabled: Story = { args: { session: session({ error: "password-sign-in-disabled" }) } };

export const SessionExpired: Story = { args: { session: session({ phase: "expired" }) } };

export const German: Story = { args: { locale: "de", session: session({ error: "invalid-credentials" }) } };

export const Phone: Story = { parameters: { viewport: { defaultViewport: "mobile1" } } };
// #endregion 🔐️HubSignIn
