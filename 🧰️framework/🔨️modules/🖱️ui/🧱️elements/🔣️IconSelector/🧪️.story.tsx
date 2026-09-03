// #region 🧲️Header

// 🥼️ .storybook/stories/ui/IconSelector.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { IconSelector } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️🐨️story.ts";
import { useState } from "react";

// 🖼️#region 🖼️IconSelector
const meta = {
  title: "🖱️ui⚛️react/IconSelector",
  component: IconSelector,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof IconSelector>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "icon-selector-story-shortcode",
    value: "box",
  },
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return (
      <div className="w-80">
        <IconSelector {...args} value={value} onChange={setValue} />
      </div>
    );
  },
};

export const Emoji: Story = {
  args: {
    id: "icon-selector-story-emoji",
    value: "emoji:🏗️",
  },
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return (
      <div className="w-80">
        <IconSelector {...args} value={value} onChange={setValue} />
      </div>
    );
  },
};

export const Disabled: Story = {
  args: {
    id: "icon-selector-story-disabled",
    value: "box",
    disabled: true,
  },
  render: (args) => {
    const [value, setValue] = useState(args.value);
    return (
      <div className="w-80">
        <IconSelector {...args} value={value} onChange={setValue} />
      </div>
    );
  },
};

// #endregion 🖼️IconSelector
