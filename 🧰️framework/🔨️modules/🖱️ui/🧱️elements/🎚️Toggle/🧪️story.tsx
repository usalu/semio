// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Toggle.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Toggle } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";

// 🔷️#region 🗡️Toggle
const Box = createIconComponent("box");
const List = createIconComponent("list");
const Lock = createIconComponent("lock");
const Network = createIconComponent("network");
const Plus = createIconComponent("plus");
const Settings = createIconComponent("settings");

const meta = {
  title: "🖱️ui⚛️react/Toggle",
  component: Toggle,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Toggle>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "toggle-default",
  defaultPressed: true,
  icon: <Lock />,
  showLabel: true,
};

export const Default: Story = {
  args: defaultArgs,
};

const withActionArgs = {
  id: "toggle-action",
  kind: "withAction" as const,
  defaultPressed: false,
  icon: <Settings />,
  actionIcon: <Plus />,
  onActionClick: () => console.log("Action clicked"),
  actionId: "toggle-action-button",
  showLabel: true,
};

export const WithAction: Story = {
  args: withActionArgs,
};

const dropdownArgs = {
  id: "toggle-dropdown",
  kind: "dropdown" as const,
  defaultValue: "option1",
  items: [
    { value: "option1", icon: <Box /> },
    { value: "option2", icon: <Network /> },
    { value: "option3", icon: <List /> },
  ],
  dropdownId: "toggle-dropdown-action",
  showLabel: true,
};

export const Dropdown: Story = {
  args: dropdownArgs,
};

// #endregion 🗡️Toggle
