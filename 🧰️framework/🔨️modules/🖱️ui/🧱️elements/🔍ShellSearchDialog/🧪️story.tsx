// #region 🧲️Header

// 🥼️ .storybook/stories/ui/ShellSearchDialog.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

// #region 🔌️Adapters
import { ShellSearchDialog, type ShellCommandResult } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "@storybook/react";
import { useMemo, useState } from "react";
// #endregion 🔌️Adapters

// 🔎️#region 🔎️ShellSearchDialog
const ALL_RESULTS: ShellCommandResult[] = [
  { id: "cmd.new-design", label: "New Design", icon: "file-code", hotkey: "⌘️N" },
  { id: "cmd.open-kit", label: "Open Kit…", icon: "folder-open", hotkey: "⌘️O" },
  { id: "cmd.export", label: "Export…", icon: "download" },
  { id: "cmd.settings", label: "Open Settings", icon: "settings" },
];

const meta = {
  title: "🖱️ui⚛️react/ShellSearchDialog",
  component: ShellSearchDialog,
  parameters: {
    layout: "fullscreen",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof ShellSearchDialog>;

export default meta;

type Story = StoryObj<typeof meta>;

function ShellSearchDialogDemo({ initialQuery = "" }: { readonly initialQuery?: string }) {
  const [open, setOpen] = useState(true);
  const [query, setQuery] = useState(initialQuery);
  const results = useMemo(() => ALL_RESULTS.filter((result) => result.label.toLowerCase().includes(query.toLowerCase())), [query]);
  return <ShellSearchDialog open={open} query={query} onQueryChange={setQuery} results={results} onPick={() => setOpen(false)} onClose={() => setOpen(false)} />;
}

export const Default: Story = {
  render: () => <ShellSearchDialogDemo />,
};

export const Filtered: Story = {
  name: "Pre-filtered query",
  render: () => <ShellSearchDialogDemo initialQuery="export" />,
};

export const Empty: Story = {
  name: "No matching results",
  render: () => <ShellSearchDialogDemo initialQuery="zzz" />,
};
// #endregion 🔎️ShellSearchDialog
