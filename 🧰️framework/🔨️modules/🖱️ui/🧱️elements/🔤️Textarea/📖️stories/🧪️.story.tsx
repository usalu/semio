// #region 🧲️Header

// 🥼️ 🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔤️Textarea/📖️stories/🧪️.story.tsx

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Textarea } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../../🧪️tests/📚️storybook-types/🟦️.ts";

const NAKAGIN_DESCRIPTION = "The digital shadow of the former Nakagin Capsule Tower — a mixed-use residential and office tower designed by architect Kisho Kurokawa in Shimbashi, Tokyo. Completed in 1972, it exemplified Japanese Metabolism.";

// 📝️#region 🎏️Textarea
const meta = {
  title: "🖱️ui⚛️react/Textarea",
  component: Textarea,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Textarea>;

export default meta;

type Story = StoryObj<typeof meta>;

const defaultArgs = {
  id: "textarea-default",
  placeholder: "Describe the design configuration, spatial relationships, and architectural intent...",
  placeholderId: "textarea.placeholder",
  defaultValue: NAKAGIN_DESCRIPTION.slice(0, 140),
  rows: 4,
  lazy: true,
  showLabel: true,
  disabled: false,
  "aria-invalid": false,
  className: "w-96",
};

export const Default: Story = {
  args: defaultArgs,
};

// #endregion 🎏️Textarea
