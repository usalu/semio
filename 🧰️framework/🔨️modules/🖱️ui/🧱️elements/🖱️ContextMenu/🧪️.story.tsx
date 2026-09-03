// #region 🧲️Header

// 🥼️ .storybook/stories/ui/🖱️ContextMenu.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲️Header

import { Button, ContextMenu, ContextMenuController, uiDataLabel, type ContextMenuItem } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️🐨️story.ts";
import { useState } from "react";

// 🖱️#region 🖱️ContextMenu
const meta = {
  title: "🖱️ui⚛️react/ContextMenu",
  component: ContextMenu,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
  args: {
    title: uiDataLabel("Actions"),
  },
} satisfies Meta<typeof ContextMenu>;

export default meta;

type Story = StoryObj<typeof meta>;

const sampleItems: ContextMenuItem[] = [
  { id: "rename", label: uiDataLabel("Rename"), icon: "pencil", shortcut: "F2" },
  { id: "duplicate", label: uiDataLabel("Duplicate"), icon: "copy", shortcut: "⌘️D" },
  { id: "visible", label: uiDataLabel("Visible"), icon: "eye", checked: true },
  { id: "sep-1", label: uiDataLabel(""), separator: true },
  {
    id: "transform",
    label: uiDataLabel("Transform"),
    icon: "move",
    children: [
      { id: "transform.move", label: uiDataLabel("Move"), icon: "move" },
      { id: "transform.rotate", label: uiDataLabel("Rotate"), icon: "rotate-cw" },
      { id: "transform.scale", label: uiDataLabel("Scale"), icon: "maximize-2", disabled: true },
    ],
  },
  { id: "sep-2", label: uiDataLabel(""), separator: true },
  { id: "delete", label: uiDataLabel("Delete"), icon: "trash-2", destructive: true, shortcut: "⌫️" },
];

export const Default: Story = {
  args: {
    items: sampleItems,
    children: <div className="flex size-40 items-center justify-center border text-sm text-muted-foreground">Right-click me</div>,
  },
};

const groupedItems: ContextMenuItem[] = [
  { id: "rename", label: uiDataLabel("Rename"), icon: "pencil", shortcut: "F2" },
  { id: "duplicate", label: uiDataLabel("Duplicate"), icon: "copy", shortcut: "⌘️D" },
  { id: "sep-header", label: uiDataLabel("Transform"), separator: true },
  {
    id: "menu.group.transform",
    label: uiDataLabel("Transform"),
    icon: "move",
    children: [
      { id: "transform.move", label: uiDataLabel("Move"), icon: "move", shortcut: "V" },
      { id: "transform.rotate", label: uiDataLabel("Rotate"), icon: "rotate-cw" },
      { id: "transform.scale", label: uiDataLabel("Scale"), icon: "maximize-2", disabled: true },
    ],
  },
  {
    id: "menu.group.view",
    label: uiDataLabel("View"),
    icon: "eye",
    children: [
      { id: "view.fit", label: uiDataLabel("Fit to Screen"), icon: "scan" },
      { id: "view.reset", label: uiDataLabel("Reset Camera"), icon: "refresh-ccw" },
    ],
  },
  {
    id: "menu.group.export",
    label: uiDataLabel("Export"),
    icon: "download",
    children: [
      { id: "export.png", label: uiDataLabel("Export as PNG"), icon: "image" },
      { id: "export.svg", label: uiDataLabel("Export as SVG"), icon: "file-code" },
    ],
  },
  { id: "sep-3", label: uiDataLabel(""), separator: true },
  { id: "delete", label: uiDataLabel("Delete"), icon: "trash-2", destructive: true, shortcut: "⌫️" },
];

export const Grouped: Story = {
  name: "Grouped (Header + Submenus + Destructive)",
  args: {
    items: groupedItems,
    children: <div className="flex size-40 items-center justify-center border text-sm text-muted-foreground">Right-click me</div>,
  },
};

export const NoItems: Story = {
  name: "No Items (Native Menu Suppressed)",
  args: {
    items: [],
    children: <div className="flex size-40 items-center justify-center border text-sm text-muted-foreground">Right-click does nothing</div>,
  },
};

export const TextSelection: Story = {
  name: "Text Selection (Copy Menu)",
  args: {
    items: [],
    children: (
      <div className="flex size-48 select-text items-center justify-center border p-single text-sm text-element">
        Select this sentence, then right-click for Copy / Select all.
      </div>
    ),
  },
};

// #endregion 🖱️ContextMenu

// #region 🎯️ContextMenuController
const controllerItems: ContextMenuItem[] = [
  { id: "focus", label: uiDataLabel("Focus"), icon: "focus", shortcut: "F" },
  { id: "select", label: uiDataLabel("Select"), icon: "mouse-pointer", checked: false },
  { id: "sep", label: uiDataLabel(""), separator: true },
  { id: "delete", label: uiDataLabel("Delete"), icon: "trash-2", destructive: true, shortcut: "⌫️" },
];

const numberedPreviewItems: ContextMenuItem[] = [
  { id: "suggestion-0", label: uiDataLabel("Capsule · port"), icon: "box", checked: true },
  { id: "suggestion-1", label: uiDataLabel("Box · port"), icon: "box", checked: false },
  { id: "sep", label: uiDataLabel(""), separator: true },
  { id: "delete", label: uiDataLabel("Delete"), icon: "trash-2", destructive: true, shortcut: "⌫️" },
];

/** @emoji 🎯️ Controlled fixed-position menu — mirrors how puzzle 2d canvas surfaces open a right-click menu at pointer coordinates. */
const ControlledContextMenuDemo = () => {
  const [open, setOpen] = useState(false);
  const [position, setPosition] = useState<{ x: number; y: number } | null>(null);
  return (
    <div className="relative flex size-40 items-center justify-center border text-sm text-muted-foreground">
      <Button
        id="context-menu-controller-story-open"
        text="Open at center"
        onClick={(event) => {
          const rect = event.currentTarget.closest(".relative")?.getBoundingClientRect();
          setPosition({ x: (rect?.left ?? 0) + (rect?.width ?? 0) / 2, y: (rect?.top ?? 0) + (rect?.height ?? 0) / 2 });
          setOpen(true);
        }}
      />
      <ContextMenuController title={uiDataLabel("Actions")} open={open} position={position} items={controllerItems} onOpenChange={setOpen} />
    </div>
  );
};

type ControllerStory = StoryObj<typeof ContextMenuController>;

export const Controlled: ControllerStory = {
  name: "ContextMenuController",
  args: {
    open: false,
    position: null,
    items: controllerItems,
    onOpenChange: () => {},
  },
  render: () => <ControlledContextMenuDemo />,
};

/** @emoji 🔢️ Numbered suggestion rows — press `1`/`2` to preview, Enter to accept the highlighted row. */
const NumberedPreviewContextMenuDemo = () => {
  const [open, setOpen] = useState(true);
  const [checkedId, setCheckedId] = useState("suggestion-0");
  const items = numberedPreviewItems.map((item) => ({
    ...item,
    checked: item.id === checkedId ? true : item.checked === undefined ? undefined : false,
    onHover: item.id.startsWith("suggestion-") ? () => setCheckedId(item.id) : undefined,
    onSelect: item.id.startsWith("suggestion-") ? () => setOpen(false) : undefined,
  }));
  return (
    <div className="relative flex size-40 items-center justify-center border text-sm text-muted-foreground">
      <Button id="context-menu-numbered-story-open" text="Open numbered menu" onClick={() => setOpen(true)} />
      <ContextMenuController open={open} closeOnSelect={false} position={{ x: 120, y: 120 }} items={items} onOpenChange={setOpen} title={uiDataLabel("Suggest")} />
    </div>
  );
};

export const NumberedPreview: ControllerStory = {
  name: "Numbered Preview (Digit + Enter)",
  args: {
    open: true,
    position: { x: 120, y: 120 },
    items: numberedPreviewItems,
    onOpenChange: () => {},
  },
  render: () => <NumberedPreviewContextMenuDemo />,
};

// #endregion 🎯️ContextMenuController
