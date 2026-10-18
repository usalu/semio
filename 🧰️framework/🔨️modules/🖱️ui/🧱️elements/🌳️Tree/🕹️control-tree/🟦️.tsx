import * as React from "react";
import { Stepper } from "../../🪜️Stepper/🟦️.tsx";
import { Slider } from "../../🎚️Slider/🟦️.tsx";
import { Toggle } from "../../🔀️Toggle/🟦️.tsx";
import { CheckIcon, CloseIcon } from "../../🔣️Icons/🟦️.tsx";
import { Input } from "../../✏️Input/🟦️.tsx";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "../../🔽️Select/🟦️.tsx";
import { Textarea } from "../../🔤️Textarea/🟦️.tsx";
import { domSizePx, uiSpacingLen, STYLING_DOM } from "@semio-tech/ui-styling";
import { cn } from "../../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { reactHostPort } from "../../🔌️Ports/🟦️.tsx";
import { TreeContext, TreeAlignedRow } from "../📏️row-alignment/🟦️.tsx";
import { useFlow, FlowProvider } from "../../../🔨️modules/🧭️flow-direction-context/🟦️.tsx";
import { TreeStateProvider, hasNonEmptyChildren, useTreeOpenState } from "../🌿️state/🟦️.tsx";
import { treeFoldChevronIcon, treePresentationLabelStyle } from "../🎨️chrome/🟦️.tsx";
import { TreeBranchContent } from "../🌲️branch/🟦️.tsx";

// Leva-like nested folder+controls tree UI using existing design system components.
// Consumers MUST provide ControlDef[] and optional ControlTreeFolderSettings.

/**
 * Leaf control definition for the ControlTree.
 **/
export interface ControlDef {
  path: string;
  key?: string;
  order?: number;
  controlKind: string;
  value: any;
  onChange: (next: any) => void;
  meta?: Record<string, any>;
}

/**
 * Folder settings for the ControlTree.
 **/
export interface ControlTreeFolderSettings {
  path: string;
  order?: number;
  collapsed?: boolean;
  color?: string;
}

/**
 * Styling classname overrides for ControlTree visual slots.
 **/
export interface ControlTreeClassNames {
  panel?: string;
  folderRow?: string;
  folderTitle?: string;
  folderChevron?: string;
  folderChildren?: string;
  controlRow?: string;
  controlLabel?: string;
  controlBody?: string;
}

interface ControlTreeNode {
  kind: "folder" | "control";
  key: string;
  path: string;
  order: number;
  control?: ControlDef;
  children?: Record<string, ControlTreeNode>;
}

/**
 * Pure function converting flat ControlDef[] paths into a nested tree. Filtering matches leaf keys case-insensitively.
 **/
export function buildControlTree(controls: ControlDef[], filterText: string, folderSettings?: Record<string, ControlTreeFolderSettings>): Record<string, ControlTreeNode> {
  const root: Record<string, ControlTreeNode> = {};
  const lowerFilter = filterText.toLowerCase();
  for (const control of controls) {
    const leafKey = control.key ?? control.path.split("/").pop() ?? control.path;
    if (lowerFilter && !leafKey.toLowerCase().includes(lowerFilter)) continue;
    const segments = control.path.split("/");
    let current = root;
    let pathAccum = "";
    for (let i = 0; i < segments.length - 1; i++) {
      const seg = segments[i];
      if(seg===undefined)throw Error("Actual control path segment required");
      pathAccum = pathAccum ? `${pathAccum}/${seg}` : seg;
      if (!current[seg]) {
        current[seg] = {
          kind: "folder",
          key: seg,
          path: pathAccum,
          order: folderSettings?.[pathAccum]?.order ?? 0,
          children: {},
        };
      }
      const node=current[seg];if(!node?.children)throw Error("Control path cannot traverse a leaf");current=node.children;
    }
    const lastSeg = segments[segments.length - 1];
    if(lastSeg===undefined)throw Error("Actual control leaf required");
    current[lastSeg] = {
      kind: "control",
      key: leafKey,
      path: control.path,
      order: control.order ?? 0,
      control,
    };
  }
  return root;
}

function sortControlTreeNodes(nodes: Record<string, ControlTreeNode>): ControlTreeNode[] {
  return Object.values(nodes).sort((a, b) => {
    if (a.order !== b.order) return a.order - b.order;
    return a.key.localeCompare(b.key);
  });
}

/**
 * Default control renderer mapping controlKind to built-in components.
 **/
export const defaultControlRenderer = (def: ControlDef): React.ReactNode => {
  const controlId = def.path.replace(/\//g, ".");
  switch (def.controlKind) {
    case "number":
      return <Stepper id={controlId} value={def.value} onChange={def.onChange} min={def.meta?.min} max={def.meta?.max} step={def.meta?.step ?? 1} />;
    case "slider":
      return <Slider id={controlId} value={[def.value]} onValueChange={(v) => def.onChange(v[0])} min={def.meta?.min ?? 0} max={def.meta?.max ?? 100} />;
    case "boolean": {
      const labelText = typeof def.meta?.label === "string" ? def.meta.label : def.key;
      return <Toggle id={controlId} pressed={def.value} onPressedChange={def.onChange} icon={def.value ? <CheckIcon className="size-small" /> : <CloseIcon className="size-small" />} text={labelText} />;
    }
    case "string":
      return <Input id={controlId} lazy value={def.value} onLazyChange={def.onChange} />;
    case "color":
      return <Input id={controlId} type="color" value={def.value} onChange={(e) => def.onChange(e.target.value)} />;
    case "select":
      return (
        <Select id={controlId} value={def.value} onValueChange={def.onChange}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {(def.meta?.options ?? []).map((opt: string | { value: string; label: string }) => {
              const v = typeof opt === "string" ? opt : opt.value;
              const l = typeof opt === "string" ? opt : opt.label;
              return (
                <SelectItem key={v} value={v}>
                  {l}
                </SelectItem>
              );
            })}
          </SelectContent>
        </Select>
      );
    case "text":
      return <Textarea id={controlId} lazy value={def.value} onLazyChange={def.onChange} />;
    default:
      return <Input id={controlId} lazy value={String(def.value)} onLazyChange={def.onChange} />;
  }
};

interface ControlTreeFolderProps {
  node: ControlTreeNode;
  folderSettings?: Record<string, ControlTreeFolderSettings>;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
  renderControl: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
}
const controlTreeValueColumnWidthPx = domSizePx("controlValueColumnUiSpacing");
interface ControlTreeRowProps {
  className?: string;
  left: React.ReactNode;
  right?: React.ReactNode;
}
const ControlTreeRow: React.FC<ControlTreeRowProps> = ({ className, left, right }) => (
  <div data-dim data-slot="control-tree-row" className={cn("grid min-w-0 w-full items-center gap-x-tiny min-h-small", className)} style={{ gridTemplateColumns: `minmax(0, 1fr) ${uiSpacingLen(STYLING_DOM.controlValueColumnUiSpacing)}` }}>
    <div data-slot="control-tree-row-left" className="relative min-w-0">
      {left}
    </div>
    <div data-slot="control-tree-row-right" className="min-w-0">
      {right}
    </div>
  </div>
);
interface ControlTreeFolderRowProps {
  node: ControlTreeNode;
  classNames?: ControlTreeClassNames;
  children?: React.ReactNode;
  defaultOpen: boolean;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
}
const ControlTreeFolderRow: React.FC<ControlTreeFolderRowProps> = ({ node, classNames, children, defaultOpen, onToggleFolder }) => {
  const { level, isLastAtLevel, showLines, isTree, indentMultiplier, direction = "down" } = reactHostPort.useContext(TreeContext);
  const { inline } = useFlow();
  const itemId = `control-tree-folder-${node.path}`;
  const { open, setOpen } = useTreeOpenState(itemId, defaultOpen);
  const hasChildren = hasNonEmptyChildren(children);
  const FolderChevron = treeFoldChevronIcon(direction, inline, open);
  return (
    <>
      <ControlTreeRow
        className={cn("hover:bg-hover-interactive-fill select-none overflow-hidden group", classNames?.folderRow)}
        left={
          <TreeAlignedRow
            level={level}
            isLastAtLevel={isLastAtLevel}
            showLines={showLines}
            connectCurrentLevel={level > 0}
            extendBranchStem={open && hasChildren}
            slotOffsetPx={2}
            slot={
              hasChildren ? (
                <button
                  className="flex-shrink-0 p-0 border-0 bg-transparent cursor-foldable"
                  onClick={(event) => {
                    event.preventDefault();
                    event.stopPropagation();
                    const nextOpen = !open;
                    setOpen(nextOpen);
                    onToggleFolder?.(node.path, !nextOpen);
                  }}
                >
                  <FolderChevron className={cn("size-small flex-shrink-0", classNames?.folderChevron)} />
                </button>
              ) : undefined
            }
            contentClassName="flex min-w-0 items-center gap-double"
          >
            <span data-slot="control-tree-folder-label" className={cn("text-xs font-semibold uppercase tracking-wide truncate text-element group-hover:text-emphasized transition-colors", classNames?.folderTitle)} style={treePresentationLabelStyle}>
              {node.key}
            </span>
          </TreeAlignedRow>
        }
      />
      {open && hasChildren && (
        <TreeContext.Provider value={{ level: level + 1, isLastAtLevel: [...isLastAtLevel, false], showLines, isTree, indentMultiplier, direction }}>
          <TreeBranchContent slot="control-tree-folder-content" className={classNames?.folderChildren}>
            {children}
          </TreeBranchContent>
        </TreeContext.Provider>
      )}
    </>
  );
};
interface ControlTreeLeafRowProps {
  node: ControlTreeNode;
  renderControl: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
}
const ControlTreeLeafRow: React.FC<ControlTreeLeafRowProps> = ({ node, renderControl, classNames }) => {
  const { level, isLastAtLevel, showLines } = reactHostPort.useContext(TreeContext);
  return (
    <ControlTreeRow
      className={cn("hover:bg-hover-interactive-fill select-none overflow-hidden group", classNames?.controlRow)}
      left={
        <TreeAlignedRow level={level} isLastAtLevel={isLastAtLevel} showLines={showLines} connectCurrentLevel={level > 0} slotOffsetPx={2} contentClassName="flex min-w-0 items-center gap-double">
          <span data-slot="control-tree-control-label" className={cn("text-xs font-normal truncate text-element group-hover:text-emphasized", classNames?.controlLabel)} style={treePresentationLabelStyle}>
            {node.key}
          </span>
        </TreeAlignedRow>
      }
      right={
        <div data-slot="control-tree-control-body" className={cn("min-w-0", classNames?.controlBody)}>
          {renderControl(node.control!)}
        </div>
      }
    />
  );
};

const ControlTreeFolder: React.FC<ControlTreeFolderProps> = ({ node, folderSettings, onToggleFolder, renderControl, classNames }) => {
  const settings = folderSettings?.[node.path];
  const defaultOpen = !(settings?.collapsed ?? false);
  const sorted = sortControlTreeNodes(node.children ?? {});
  return (
    <ControlTreeFolderRow node={node} classNames={classNames} defaultOpen={defaultOpen} onToggleFolder={onToggleFolder}>
      {sorted.map((child) =>
        child.kind === "folder" ? (
          <ControlTreeFolder key={child.path} node={child} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
        ) : (
          <ControlTreeLeafRow key={child.path} node={child} renderControl={renderControl} classNames={classNames} />
        ),
      )}
    </ControlTreeFolderRow>
  );
};

/**
 * Props interface for the ControlTree component.
 **/
export interface ControlTreeProps {
  controls: ControlDef[];
  filterText?: string;
  folderSettings?: Record<string, ControlTreeFolderSettings>;
  onToggleFolder?: (path: string, collapsed: boolean) => void;
  renderControl?: (def: ControlDef) => React.ReactNode;
  classNames?: ControlTreeClassNames;
  className?: string;
}

/**
 * Leva-like nested folder+controls tree panel using existing design system components.
 **/
export const ControlTree: React.FC<ControlTreeProps> = ({ controls, filterText = "", folderSettings, onToggleFolder, renderControl = defaultControlRenderer, classNames, className }) => {
  const tree = reactHostPort.useMemo(() => buildControlTree(controls, filterText, folderSettings), [controls, filterText, folderSettings]);
  const sorted = reactHostPort.useMemo(() => sortControlTreeNodes(tree), [tree]);
  return (
    <FlowProvider inline="ltr">
      <div data-slot="control-tree" dir="auto" className={cn("w-full min-w-0", classNames?.panel, className)}>
        <TreeStateProvider>
          <TreeContext.Provider value={{ level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 }}>
            {sorted.map((node) =>
              node.kind === "folder" ? (
                <ControlTreeFolder key={node.path} node={node} folderSettings={folderSettings} onToggleFolder={onToggleFolder} renderControl={renderControl} classNames={classNames} />
              ) : (
                <ControlTreeLeafRow key={node.path} node={node} renderControl={renderControl} classNames={classNames} />
              ),
            )}
          </TreeContext.Provider>
        </TreeStateProvider>
      </div>
    </FlowProvider>
  );
};
