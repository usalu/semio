// #region 🧲️Header
// 🥼️ .storybook/stories/ui/TableAvatar.stories.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { TableAvatar } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story";
// #endregion 🔌️Adapters

// #region 📻️TableAvatar
const meta = {
  title: "🖱️ui⚛️react/TableAvatar",
  component: TableAvatar,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof TableAvatar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <div className="flex items-center gap-4">
      <TableAvatar name="Kisho Kurokawa" icon="https://github.com/shadcn.png" />
      <TableAvatar name="Kenzo Tange" />
      <TableAvatar name="Fumihiko Maki" isSelected />
      <TableAvatar name="Arata Isozaki" isHovered />
    </div>
  ),
};
// #endregion 📻️TableAvatar
