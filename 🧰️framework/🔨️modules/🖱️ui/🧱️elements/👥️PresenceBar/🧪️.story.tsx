// #region 🧲️Header
// 🥼️ .storybook/stories/ui/PresenceBar.stories.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import { PresenceBar } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️🐨️story.ts";
// #endregion 🔌️Adapters

// #region 👥️PresenceBar
const meta = {
  title: "🖱️ui⚛️react/PresenceBar",
  component: PresenceBar,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof PresenceBar>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <PresenceBar
      peers={[
        { actor: "user:kisho#a", label: "Kisho Kurokawa", role: "author" },
        { actor: "user:kenzo#b", label: "Kenzo Tange", role: "spectator" },
        { actor: "user:fumihiko#c", label: "Fumihiko Maki", role: "author" },
      ]}
    />
  ),
};

export const Overflow: Story = {
  render: () => (
    <PresenceBar
      max={3}
      peers={[
        { actor: "user:kisho#a", label: "Kisho Kurokawa", role: "author" },
        { actor: "user:kenzo#b", label: "Kenzo Tange", role: "spectator" },
        { actor: "user:fumihiko#c", label: "Fumihiko Maki", role: "author" },
        { actor: "user:arata#d", label: "Arata Isozaki", role: "spectator" },
        { actor: "user:toyo#e", label: "Toyo Ito", role: "author" },
      ]}
    />
  ),
};

export const Empty: Story = {
  render: () => <PresenceBar peers={[]} />,
};
// #endregion 👥️PresenceBar
