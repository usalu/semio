// #region 🧲️Header

// .elements/ui/.storybook/story/elements/window/Diagram.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲️Header

import { DiagramSkeleton } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️🐨️story.ts";

// #region 🧫️Diagram

const meta = {
  title: "🖱️ui⚛️react/Diagram",
  component: DiagramSkeleton,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof DiagramSkeleton>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Skeleton: Story = {
  args: {
    nodeCount: 5,
    edgeCount: 4,
  },
  render: (args) => (
    <div className="h-96 w-full">
      <DiagramSkeleton {...args} />
    </div>
  ),
};

export const LargeSkeleton: Story = {
  args: {
    nodeCount: 12,
    edgeCount: 10,
  },
  render: (args) => (
    <div className="h-96 w-full">
      <DiagramSkeleton {...args} />
    </div>
  ),
};

// #endregion 🧫️Diagram
