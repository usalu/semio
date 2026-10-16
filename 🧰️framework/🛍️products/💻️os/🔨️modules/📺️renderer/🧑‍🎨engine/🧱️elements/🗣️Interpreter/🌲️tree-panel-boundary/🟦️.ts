/** 🌲️ The still-supported manifest tree payload mapped onto the owned panel-tree contract: panel tree items, sections, catalogue drag MIME and the declarative drag controller.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import {
  catalogueTreeDragController,
  renderControlIcon,
  type IconName,
  type TreeDataItem,
  type TreeDataSection,
  type TreeDragAndDropController,
  type TreePanelConfig
} from "@semio-tech/ui-react";
import { type ActionDescriptor } from "@semio-tech/framework";
import { wireLabel } from "../🌉️scene-host-contract/🟦️.ts";

//#region 🔖️UiInterpreter
//#region 🌲️TreePanelBoundary
type PanelTreePresence = {
  readonly status?: "idle" | "loading" | "waiting";
  readonly selected?: boolean;
};

type PanelTreeItem = {
  readonly id: string;
  readonly label: string;
  readonly description?: string;
  readonly iconId?: IconName;
  readonly defaultOpen?: boolean;
  readonly presence?: PanelTreePresence;
  readonly dimmed?: boolean;
  readonly draggable?: boolean;
  readonly dragData?: Record<string, string>;
  readonly items?: readonly PanelTreeItem[];
  readonly action?: ActionDescriptor;
  readonly hoverAction?: ActionDescriptor;
  readonly unhoverAction?: ActionDescriptor;
  readonly actions?: readonly {
    readonly iconId: IconName;
    readonly label?: string;
    readonly placement?: "row" | "menu";
    readonly action: ActionDescriptor;
  }[];
};

type PanelTreeNode = {
  readonly sections: readonly {
    readonly id: string;
    readonly label?: string;
    readonly defaultOpen?: boolean;
    readonly presence?: PanelTreePresence;
    readonly items: readonly PanelTreeItem[];
  }[];
  readonly selectedIds?: readonly string[];
  readonly highlightedIds?: readonly string[];
  readonly selectionChange?: ActionDescriptor;
  readonly dropAction?: ActionDescriptor;
};

function dispatchPanelTreeAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({ ...descriptor, args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch } });
}

function panelTreeItemsToData(items: readonly PanelTreeItem[], onAction: (action: ActionDescriptor) => void): TreeDataItem[] {
  return items.map((item) => ({
    id: item.id,
    label: item.label,
    description: item.description,
    icon: item.iconId ? renderControlIcon(item.iconId, 12) : undefined,
    defaultOpen: item.defaultOpen,
    isSelected: item.presence?.selected,
    loading: item.presence?.status === "loading",
    waiting: item.presence?.status === "waiting",
    isHidden: item.dimmed,
    draggable: item.draggable,
    dragData: item.dragData,
    items: item.items?.length ? panelTreeItemsToData(item.items, onAction) : undefined,
    onClick: item.action ? () => dispatchPanelTreeAction(onAction, item.action!, {}) : undefined,
    onPointerEnter: item.hoverAction ? () => dispatchPanelTreeAction(onAction, item.hoverAction!, {}) : undefined,
    onPointerLeave: item.unhoverAction ? () => dispatchPanelTreeAction(onAction, item.unhoverAction!, {}) : undefined,
    actions: item.actions?.map((action) => ({
      kind: "button" as const,
      icon: action.iconId,
      title: action.label ? wireLabel(action.label) : undefined,
      placement: action.placement ?? "row",
      onClick: () => dispatchPanelTreeAction(onAction, action.action, {}),
    })),
  }));
}

/** 🌲️ Maps the still-supported manifest tree payload onto the owned panel-tree contract. */
export function uiTreeNodeToTreePanelConfig(treeNode: PanelTreeNode, onAction: (action: ActionDescriptor) => void): TreePanelConfig {
  const sections: TreeDataSection[] = treeNode.sections.map((section) => ({
    id: section.id,
    label: section.label ?? "",
    defaultOpen: section.defaultOpen,
    loading: section.presence?.status === "loading",
    waiting: section.presence?.status === "waiting",
    items: panelTreeItemsToData(section.items, onAction),
  }));
  return {
    sections,
    selectedIds: treeNode.selectedIds ? [...treeNode.selectedIds] : undefined,
    highlightedIds: treeNode.highlightedIds,
    onSelectionChange: treeNode.selectionChange ? (selectedIds) => dispatchPanelTreeAction(onAction, treeNode.selectionChange!, { ids: selectedIds }) : undefined,
    sortableSections: Boolean(treeNode.dropAction) && sections.length > 1,
  };
}

/** 🖱️ First catalogue-transfer MIME on a declarative {@link Tree} document, if any row carries drag data. */
export function treeSectionsCatalogueDragMime(sections: readonly TreeDataSection[]): string | undefined {
  const visit = (items: readonly TreeDataItem[]): string | undefined => {
    for (const item of items) {
      if (item.dragData) {
        const mime = Object.keys(item.dragData).find((key) => key === "application/x-semio-catalogue-item");
        if (mime) return mime;
      }
      if (item.items?.length) {
        const nested = visit(item.items);
        if (nested) return nested;
      }
    }
    return undefined;
  };
  for (const section of sections) {
    const mime = visit(section.items ?? []);
    if (mime) return mime;
  }
  return undefined;
}

function panelTreeDragMime(treeNode: PanelTreeNode): string | undefined {
  const visit = (items: readonly PanelTreeItem[]): string | undefined => {
    for (const item of items) {
      const mime = item.dragData ? Object.keys(item.dragData)[0] : undefined;
      if (mime) return mime;
      const nested = item.items?.length ? visit(item.items) : undefined;
      if (nested) return nested;
    }
    return undefined;
  };
  for (const section of treeNode.sections) {
    const mime = visit(section.items);
    if (mime) return mime;
  }
  return undefined;
}

/** 🖱️ Owns manifest-tree drag payload and drop-action routing. */
export function declarativeTreeDragController(treeNode: PanelTreeNode, onAction: (action: ActionDescriptor) => void): TreeDragAndDropController | undefined {
  const mime = panelTreeDragMime(treeNode);
  const source = mime ? catalogueTreeDragController(mime) : undefined;
  if (!treeNode.dropAction) return source;
  return {
    ...(source ?? {}),
    handleDrop: ({ data, target, dropPosition }) => {
      const encoded = Object.entries(data).find(([kind, value]) => kind.startsWith("application/x-semio-") && value.trim())?.[1];
      if (!encoded) return;
      let payload: Record<string, unknown>;
      try {
        payload = JSON.parse(encoded) as Record<string, unknown>;
      } catch {
        return;
      }
      dispatchPanelTreeAction(onAction, treeNode.dropAction!, { ...payload, targetId: target.id, dropPosition: dropPosition ?? "inside" });
    },
  };
}
//#endregion 🌲️TreePanelBoundary
//#endregion 🔖️UiInterpreter
