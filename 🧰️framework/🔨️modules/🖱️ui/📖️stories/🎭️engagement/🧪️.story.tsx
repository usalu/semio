// #region 🧲️Header
// 🧪️ 🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️engagement/🧪️.story.tsx
// #endregion 🧲️Header

import { Engagement, Search, uiDataLabel, Window } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";
import * as React from "react";
import { searchStandalonePlay, standalonePlay, withControlPlay } from "../../🧪️tests/🎯️engagement-story-interaction/🟦️.ts";

const meta = {
  title: "🖱️ui⚛️react/Engagement",
  component: Engagement,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof Engagement>;

export default meta;

type Story = StoryObj<typeof meta>;

export const SearchStandalone: Story = {
  render: () => <Search input={{ placeholder: uiDataLabel("Ask or action…"), onSubmit: () => {} }} />,
  play: searchStandalonePlay,
};

export const Standalone: Story = {
  render: () => (
    <Engagement
      options={[
        { id: "snap", label: uiDataLabel("Snap"), icon: "magnet", pressed: true, onPress: () => {} },
        { id: "grid", label: uiDataLabel("Grid"), icon: "grid-3x3", onPress: () => {} },
      ]}
      status={[
        { id: "ready", content: "Ready" },
        { id: "selection", content: "3 selected" },
      ]}
    />
  ),
  play: standalonePlay,
};

export const WithControl: Story = {
  render: () => (
    <Engagement
      sessionActive
      status={[{ id: "engagement-step", content: "Step: Height" }]}
      control={{
        kind: "stepper",
        id: "height",
        label: uiDataLabel("Height"),
        value: 3,
        min: 0,
        step: 0.1,
        unit: "m",
        onChange: () => {},
      }}
    />
  ),
  play: withControlPlay,
};

export const InWindow: Story = {
  render: () => (
    <div className="relative h-[320px] w-[480px]">
      <Window
        id="engagement-window"
        active
        engagement={{
          options: [{ id: "tool-a", label: uiDataLabel("Tool A"), icon: "wrench", onPress: () => {} }],
          status: [{ id: "status", content: "Idle" }],
        }}
        search={{
          input: { placeholder: uiDataLabel("Type an action") },
        }}
      >
        {/* Interior content of the `<Window>` above — its root already fills `ui-surface` at level="window", so this stays bg-transparent. */}
        <div className="flex h-full items-center justify-center bg-transparent">Window body</div>
      </Window>
    </div>
  ),
};
