// #region 🧲️Header

// 🥼️ .storybook/stories/ui/HistoryTable.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { HistoryTable, type HistoryColumn } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

// 🗄️#region 🗄️HistoryTable
const meta = {
  title: "🖱️ui⚛️react/HistoryTable",
  component: HistoryTable,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof HistoryTable>;

export default meta;

type Story = StoryObj<typeof meta>;

const branchingColumns: HistoryColumn[] = [
  { checkpointId: "c6", timestamp: "2026-07-19T09:00:00Z", labels: ["main"], authors: [{ id: "ueli", name: "Ueli Saluz" }], parentCheckpointId: "c5", description: "Merge balcony variant back into main", lane: 0, alternativeIds: [] },
  { checkpointId: "c5", timestamp: "2026-07-18T17:20:00Z", labels: [], authors: [{ id: "kk", name: "Kisho Kurokawa" }], parentCheckpointId: "c3", description: "Add balcony geometry", lane: 1, alternativeIds: [] },
  { checkpointId: "c4", timestamp: "2026-07-18T15:05:00Z", labels: [], authors: [{ id: "ueli", name: "Ueli Saluz" }], parentCheckpointId: "c3", description: "Adjust capsule spacing", lane: 0, alternativeIds: [] },
  { checkpointId: "c3", timestamp: "2026-07-17T11:40:00Z", labels: ["v0.2"], authors: [{ id: "kk", name: "Kisho Kurokawa" }], parentCheckpointId: "c2", description: "Split capsule variants into a branch", lane: 0, alternativeIds: ["release-0.2"] },
  { checkpointId: "c2", timestamp: "2026-07-15T09:12:00Z", labels: [], authors: [{ id: "kt", name: "Kenzo Tange" }], parentCheckpointId: "c1", description: "Rename tower kit pieces", lane: 0, alternativeIds: [] },
  { checkpointId: "c1", timestamp: "2026-07-14T08:00:00Z", labels: ["v0.1"], authors: [{ id: "ueli", name: "Ueli Saluz" }], lane: 0, alternativeIds: [] },
];

export const Default: Story = {
  args: {
    id: "storybook.historyTable.default",
    columns: branchingColumns,
  },
  render: (args) => (
    <div className="w-[720px] p-single">
      <HistoryTable {...args} />
    </div>
  ),
};

export const Empty: Story = {
  args: {
    id: "storybook.historyTable.empty",
    columns: [],
  },
  render: (args) => (
    <div className="w-[720px] p-single">
      <HistoryTable {...args} />
    </div>
  ),
};

export const Selectable: Story = {
  args: {
    id: "storybook.historyTable.selectable",
    columns: branchingColumns,
    onSelectCheckpoint: (checkpointId: string) => {
      // eslint-disable-next-line no-console
      console.log("selected checkpoint", checkpointId);
    },
  },
  render: (args) => (
    <div className="w-[720px] p-single">
      <HistoryTable {...args} />
    </div>
  ),
};

// #endregion 🗄️HistoryTable
