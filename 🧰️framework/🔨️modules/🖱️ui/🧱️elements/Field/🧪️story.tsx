// #region 🧲️Header

// 🥼️ .storybook/stories/ui/Field.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Field, Input } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react-vite";

// 🏷️#region 🏷️Field
const meta = {
  title: "🖱️ui⚛️react/Field",
  component: Field,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof Field>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    id: "field-story-name",
    label: "Name",
    children: <Input id="field-story-name" placeholder="Kisho Kurokawa" />,
  },
  render: (args) => (
    <div className="w-70">
      <Field {...args} />
    </div>
  ),
};

export const WithDescriptionAndError: Story = {
  name: "Description & Error",
  args: {
    id: "field-story-email",
    label: "Email",
    description: "Used for design review notifications.",
    required: true,
    error: "Enter a valid email address.",
    children: <Input id="field-story-email" placeholder="you@example.com" />,
  },
  render: (args) => (
    <div className="w-70">
      <Field {...args} />
    </div>
  ),
};

// #endregion 🏷️Field
