// #region 🧲️Header

// .elements/ui/.storybook/story/elements/Ribbon.stories.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// #endregion 🧲️Header

import { ButtonGroup, ButtonGroupItem, Ribbon, RibbonDivider, RibbonGroup, RibbonItem, RibbonZone, ToggleGroup, type RibbonRow } from "@semio-tech/ui-react";
import { createIconComponent } from "@semio-tech/ui-react";
import type { Meta, StoryObj } from "../../🧪️story";
import { useState, type ComponentType } from "react";

// #region 🌙️Ribbon

const Cloud = createIconComponent("cloud");
const Eye = createIconComponent("eye");
const EyeOff = createIconComponent("eye-off");
const FileJson = createIconComponent("file-json");
const Folder = createIconComponent("folder");
const Hand = createIconComponent("hand");
const Maximize2 = createIconComponent("maximize2");
const MousePointer = createIconComponent("mouse-pointer");
const Move = createIconComponent("move");
const RotateCcw = createIconComponent("rotate-ccw");
const RotateCw = createIconComponent("rotate-cw");
const ZoomIn = createIconComponent("zoom-in");
const ZoomOut = createIconComponent("zoom-out");

const meta = {
  title: "🖱️ui⚛️react/Ribbon",
  component: RibbonZone,
  parameters: {
    layout: "centered",
  },
  tags: ["autodocs"],
} satisfies Meta<typeof RibbonZone>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    children: null,
  },
  render: () => (
    <RibbonZone>
      <RibbonGroup>
        <RibbonItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Move className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </RibbonItem>
      </RibbonGroup>
      <RibbonDivider />
      <RibbonGroup>
        <RibbonItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<ZoomIn className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<ZoomOut className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Maximize2 className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </RibbonItem>
      </RibbonGroup>
    </RibbonZone>
  ),
};

export const WithUndoRedo: Story = {
  args: { children: null },
  render: () => (
    <RibbonZone>
      <RibbonGroup>
        <RibbonItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<RotateCcw className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<RotateCw className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </RibbonItem>
      </RibbonGroup>
      <RibbonDivider />
      <RibbonGroup>
        <RibbonItem>
          <ButtonGroup>
            <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
            <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
          </ButtonGroup>
        </RibbonItem>
      </RibbonGroup>
      <RibbonDivider />
      <RibbonGroup>
        <RibbonItem>
          <ToggleGroup
            kind="multiple"
            defaultValue={["visible"]}
            items={[
              { value: "visible", icon: <Eye className="size-tiny" aria-hidden /> },
              { value: "hidden", icon: <EyeOff className="size-tiny" aria-hidden /> },
            ]}
          />
        </RibbonItem>
      </RibbonGroup>
    </RibbonZone>
  ),
};

export const MultipleZones: Story = {
  args: { children: null },
  render: () => (
    <div className="flex gap-single items-stretch">
      <RibbonZone>
        <RibbonGroup>
          <RibbonItem>
            <ToggleGroup
              kind="single"
              defaultValue="selection"
              items={[
                { value: "selection", id: "ui.ribbon.group.selection", icon: <MousePointer className="size-tiny" aria-hidden />, text: "Selection" },
                { value: "view", id: "ui.ribbon.group.view", icon: <Eye className="size-tiny" aria-hidden />, text: "View" },
              ]}
            />
          </RibbonItem>
        </RibbonGroup>
      </RibbonZone>
      <RibbonZone>
        <RibbonGroup>
          <RibbonItem>
            <ButtonGroup>
              <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
              <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
            </ButtonGroup>
          </RibbonItem>
        </RibbonGroup>
      </RibbonZone>
      <RibbonZone>
        <RibbonGroup>
          <RibbonItem>
            <ButtonGroup>
              <ButtonGroupItem icon={<ZoomIn className="size-tiny" aria-hidden />} />
              <ButtonGroupItem icon={<ZoomOut className="size-tiny" aria-hidden />} />
            </ButtonGroup>
          </RibbonItem>
        </RibbonGroup>
      </RibbonZone>
    </div>
  ),
};

export const RibbonLevels: Story = {
  name: "Ribbon Levels (inline vs. up)",
  args: { children: null },
  render: () => {
    const rows: RibbonRow[] = [
      {
        key: "base",
        content: (
          <RibbonZone>
            <RibbonGroup>
              <RibbonItem>
                <ButtonGroup>
                  <ButtonGroupItem icon={<MousePointer className="size-tiny" aria-hidden />} />
                  <ButtonGroupItem icon={<Hand className="size-tiny" aria-hidden />} />
                </ButtonGroup>
              </RibbonItem>
            </RibbonGroup>
          </RibbonZone>
        ),
      },
      {
        key: "nested",
        content: (
          <RibbonZone>
            <RibbonGroup>
              <RibbonItem>
                <ButtonGroup>
                  <ButtonGroupItem icon={<ZoomIn className="size-tiny" aria-hidden />} />
                  <ButtonGroupItem icon={<ZoomOut className="size-tiny" aria-hidden />} />
                </ButtonGroup>
              </RibbonItem>
            </RibbonGroup>
          </RibbonZone>
        ),
      },
    ];
    return (
      <div className="flex items-end gap-double">
        <div className="flex flex-col items-center gap-single">
          <span className="text-xs text-muted-foreground">inline (footer)</span>
          <Ribbon direction="inline" rows={rows} />
        </div>
        <div className="flex flex-col items-center gap-single">
          <span className="text-xs text-muted-foreground">up (window utility bar)</span>
          <Ribbon direction="up" rows={rows} />
        </div>
      </div>
    );
  },
};

// #region 🗂️RecursiveCategoryGroups

type DemoUtilityLeaf = { readonly id: string; readonly icon: ComponentType<{ size?: number; className?: string }> };
type DemoUtilityNode =
  | { readonly id: string; readonly label: string; readonly icon: DemoUtilityLeaf["icon"]; readonly kind: "leaves"; readonly leaves: readonly DemoUtilityLeaf[] }
  | { readonly id: string; readonly label: string; readonly icon: DemoUtilityLeaf["icon"]; readonly kind: "group"; readonly children: readonly DemoUtilityNode[] };

/** @emoji 🪟️ Window-scoped categories only (selection / utilities) — what belongs in a window's own bottom-left panel. Mode-wide categories like actions/history/sync don't: they're shared across every window in the mode, so they live once in the footer instead (see {@link ModeWideFooterCategories}). */
const WINDOW_CATEGORY_DEMO_TREE: readonly DemoUtilityNode[] = [
  {
    id: "selection",
    label: "Selection",
    icon: MousePointer,
    kind: "leaves",
    leaves: [
      { id: "direct", icon: MousePointer },
      { id: "marquee", icon: Hand },
    ],
  },
  {
    id: "utilities",
    label: "Utilities",
    icon: Move,
    kind: "group",
    children: [
      {
        id: "transform",
        label: "Transform",
        icon: Move,
        kind: "leaves",
        leaves: [
          { id: "move", icon: Move },
          { id: "rotate", icon: RotateCw },
          { id: "zoom", icon: ZoomIn },
        ],
      },
      {
        id: "view",
        label: "View",
        icon: Eye,
        kind: "leaves",
        leaves: [
          { id: "show", icon: Eye },
          { id: "hide", icon: EyeOff },
        ],
      },
    ],
  },
];

const FOOTER_CATEGORY_DEMO_TREE: readonly DemoUtilityNode[] = [
  {
    id: "actions",
    label: "Actions",
    icon: RotateCcw,
    kind: "leaves",
    leaves: [
      { id: "format", icon: RotateCcw },
      { id: "lint", icon: Maximize2 },
    ],
  },
  {
    id: "history",
    label: "History",
    icon: RotateCcw,
    kind: "leaves",
    leaves: [
      { id: "undo", icon: RotateCcw },
      { id: "redo", icon: RotateCw },
    ],
  },
  {
    id: "sync",
    label: "Sync",
    icon: Cloud,
    kind: "leaves",
    leaves: [
      { id: "file", icon: FileJson },
      { id: "folder", icon: Folder },
      { id: "remote", icon: Cloud },
    ],
  },
];

/** @emoji 🗂️ Builds `up`-stacked ribbon rows from a demo utility tree: at most one active group per level, activating a level appends its children as another line, recursing until a leaves group is reached or nothing is active. Mirrors {@link buildUtilityRibbonSegments} in `@semio-tech/framework-renderer-react` for storybook without pulling in the full renderer package. */
function buildRecursiveCategoryRows(tree: readonly DemoUtilityNode[], activePath: readonly string[], onActivate: (depth: number, value: string) => void): RibbonRow[] {
  const rows: RibbonRow[] = [];
  let level = tree;
  let depth = 0;
  while (true) {
    rows.push({
      key: `picker-${depth}`,
      content: (
        <RibbonZone>
          <RibbonGroup>
            <RibbonItem>
              <ToggleGroup
                kind="single"
                value={activePath[depth] ?? ""}
                onValueChange={(value) => onActivate(depth, value)}
                items={level.map((node) => ({ value: node.id, id: `ui.ribbon.demo.group.${node.id}`, icon: <node.icon className="size-tiny" aria-hidden />, text: node.label }))}
              />
            </RibbonItem>
          </RibbonGroup>
        </RibbonZone>
      ),
    });
    const active = level.find((node) => node.id === activePath[depth]);
    if (!active) break;
    if (active.kind === "leaves") {
      rows.push({
        key: `leaves-${depth}`,
        content: (
          <RibbonZone>
            <RibbonGroup>
              <RibbonItem>
                <ButtonGroup>
                  {active.leaves.map((leaf) => (
                    <ButtonGroupItem key={leaf.id} icon={<leaf.icon className="size-tiny" aria-hidden />} />
                  ))}
                </ButtonGroup>
              </RibbonItem>
            </RibbonGroup>
          </RibbonZone>
        ),
      });
      break;
    }
    level = active.children;
    depth += 1;
  }
  return rows;
}

const CategoryGroupsDemo = ({ tree, direction }: { readonly tree: readonly DemoUtilityNode[]; readonly direction: "up" | "inline" }) => {
  const [activePath, setActivePath] = useState<readonly string[]>([]);
  const onActivate = (depth: number, value: string) => {
    setActivePath((previous) => (value ? [...previous.slice(0, depth), value] : previous.slice(0, depth)));
  };
  return <Ribbon id="ui.ribbon.demo" direction={direction} rows={buildRecursiveCategoryRows(tree, activePath, onActivate)} />;
};

export const RecursiveCategoryGroups: Story = {
  name: "Window Panel: Recursive Category Groups (selection / utilities)",
  args: { children: null },
  render: () => (
    <div className="flex flex-col items-start gap-double">
      <span className="text-xs text-muted-foreground">
        A window's own bottom-left panel: only window-scoped categories (selection, utilities). Click a category to expand a line above it; click the active one again to collapse it. Only one group is active per level, and "Utilities" recurses into
        a second picker.
      </span>
      <CategoryGroupsDemo tree={WINDOW_CATEGORY_DEMO_TREE} direction="up" />
    </div>
  ),
};

export const ModeWideFooterCategories: Story = {
  name: "Footer: Mode-Wide Categories (actions / history / sync)",
  args: { children: null },
  render: () => (
    <div className="flex flex-col items-start gap-double">
      <span className="text-xs text-muted-foreground">
        Categories that apply regardless of which window has focus (actions, history, sync — e.g. File/Folder/Remote grouped under "Sync") render once in the shared footer instead of being duplicated into every window's panel. Same recursive,
        one-active-per-level picker, laid out horizontally.
      </span>
      <CategoryGroupsDemo tree={FOOTER_CATEGORY_DEMO_TREE} direction="inline" />
    </div>
  ),
};

// #endregion 🗂️RecursiveCategoryGroups

// #endregion 🌙️Ribbon
