// #region 🧲️Header
// 💻️ 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎓️HubFirstRun/📖️stories/🧪️.story.tsx
// Specs: Every entry point into the hub walkthrough — a person who has not signed in, one signed in
// with no space, one already in a space, a viewer who may not invite, and the German render.
// Summary: `HubFirstRun` is purely presentational; each story is one `HubFirstRunStateV1`, which
// decides only which step the tour opens on. No hub, no transport and no storage are involved.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import type { Meta, StoryObj } from "@storybook/react-vite";
import { HubFirstRun } from "@semio-tech/framework-renderer-react";
import type { HubFirstRunStateV1 } from "@semio-tech/framework-os";
// #endregion 🔌️Adapters

// #region 🎓️HubFirstRun
function state(overrides: Partial<HubFirstRunStateV1> = {}): HubFirstRunStateV1 {
  return { signedIn: false, spaceCount: 0, openSpaceId: null, canInvite: false, ...overrides };
}

const meta = {
  title: "💻️os⚛️react/HubFirstRun",
  component: HubFirstRun,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
  args: { locale: "en", state: state(), onDismiss: () => undefined },
} satisfies Meta<typeof HubFirstRun>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🚪️ The very first thing a new person sees when they open the hub surface. */
export const Welcome: Story = {
  name: "Welcome (forced first step)",
  args: { initialStepIndex: 0 },
};

/** 🔐️ Signed out — the tour opens on sign-in. */
export const SignedOut: Story = {
  args: { state: state() },
};

/** 🏘️ Signed in, no space yet — the tour opens on creating or joining one. */
export const NoSpaceYet: Story = {
  args: { state: state({ signedIn: true }) },
};

/** ✉️ Already collaborating — the tour opens on inviting the rest of the team. */
export const InASpace: Story = {
  args: { state: state({ signedIn: true, spaceCount: 2, openSpaceId: "spc_studio", canInvite: true }) },
};

/** 👀️ A viewer who may not invite — the tour skips the invite step rather than teaching a control
 * that would refuse them. */
export const ViewerInASpace: Story = {
  args: { state: state({ signedIn: true, spaceCount: 1, openSpaceId: "spc_studio", canInvite: false }) },
};

/** 🇩🇪 The German render — both owned locales are first-class, neither is a fallback. */
export const German: Story = {
  args: { locale: "de", state: state() },
};
// #endregion 🎓️HubFirstRun
