// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Popover.stories.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Button, Input, Popover, PopoverContent, PopoverTrigger } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story.ts";

// 🔷️#region 🌐️Popover
const Settings = createIconComponent("settings");

const meta = {
  title: "🖱️ui⚛️react/Popover",
  component: Popover,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Popover>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  render: () => (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="default" id="popover-trigger-default">
          <Settings />
          Connection Settings
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-80" side="bottom" align="start">
        <div className="space-y-4">
          <div className="space-y-2">
            <h4 className="font-medium text-sm">Connection Parameters</h4>
            <p className="text-sm text-muted-foreground">Configure spatial relationship between pieces.</p>
          </div>
          <div className="grid gap-double">
            <Input id="gap-input" showLabel defaultValue="0" type="number" />
            <Input id="rotation-input" showLabel defaultValue="0" type="number" min={0} max={360} />
          </div>
          <div className="flex justify-end gap-double">
            <Button variant="default">Cancel</Button>
            <Button>Save</Button>
          </div>
        </div>
      </PopoverContent>
    </Popover>
  ),
};

// #endregion 🌐️Popover
