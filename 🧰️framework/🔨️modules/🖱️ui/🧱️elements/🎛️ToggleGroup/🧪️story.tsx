// #region 🧲️Header

// 🥼️ .storybook/stories/ui/ToggleGroup.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Action, ToggleGroup } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story";

// 🔷️#region 🧩️ToggleGroup
const Box = createIconComponent("box");
const List = createIconComponent("list");
const Lock = createIconComponent("lock");
const Network = createIconComponent("network");
const Plus = createIconComponent("plus");
const Settings = createIconComponent("settings");

const meta = {
  title: "🖱️ui⚛️react/ToggleGroup",
  component: ToggleGroup,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ToggleGroup>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultItems = [
  { id: "toggle-default-standard", value: "standard", icon: <Lock /> },
  { id: "toggle-action-settings", value: "settings", icon: <Settings />, action: <Action id="toggle-action-settings-add" icon={<Plus />} /> },
  { id: "toggle-dropdown-box", value: "box", icon: <Box />, action: <Action id="toggle-dropdown-box-action" icon={<Network />} /> },
];

const multipleItems = [
  { id: "toggle-multiple-standard", value: "standard", icon: <Lock /> },
  { id: "toggle-multiple-box", value: "box", icon: <Box /> },
  { id: "toggle-multiple-network", value: "network", icon: <Network /> },
  { id: "toggle-multiple-list", value: "list", icon: <List /> },
  { id: "toggle-multiple-settings", value: "settings", icon: <Settings /> },
  { id: "toggle-multiple-plus", value: "plus", icon: <Plus /> },
];

export const Default: Story = {
  args: {
    id: "toggle-group-default",
    kind: "single",
    defaultValue: "standard",
    showLabel: true,
    items: defaultItems,
  },
};

export const Multiple: Story = {
  args: {
    id: "toggle-group-multiple",
    kind: "multiple",
    defaultValue: ["box"],
    showLabel: true,
    items: multipleItems,
  },
};

// #endregion 🧩️ToggleGroup
