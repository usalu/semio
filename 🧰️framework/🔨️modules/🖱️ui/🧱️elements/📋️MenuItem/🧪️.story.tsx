// #region 🔌️Adapters
import { MenuItem } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️🐨️story.ts";
// #endregion 🔌️Adapters

// #region 📋️MenuItemMatrix
const meta = {
  title: "🖱️ui⚛️react/MenuItem",
  component: MenuItem,
  parameters: { layout: "centered" },
  tags: ["autodocs"],
} satisfies Meta<typeof MenuItem>;

export default meta;
type Story = StoryObj<typeof meta>;

export const StateMatrix: Story = {
  render: () => (
    <div role="menu" className="w-layout-floating-menu-sm rounded-md border p-single">
      <MenuItem>Ready</MenuItem>
      <MenuItem data-active="true">Active</MenuItem>
      <MenuItem data-selected="true">Selected</MenuItem>
      <MenuItem disabled>Disabled</MenuItem>
    </div>
  ),
};
// #endregion 📋️MenuItemMatrix
