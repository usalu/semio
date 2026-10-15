/** 🌲 Owns the tree panel source shapes that tab bars and panels exchange. */
import * as React from "react";
import { type TreeDataItem, type TreeDataSection, type TreeDragAndDropController, type TreeSelectionMode } from "../../🌳️Tree/🟦️.tsx";

// Collapsible panel growing from one of the display's eight anchors (four corners plus four edge middles), with tabbed content.
// Consumers MUST provide PanelTabNode entries (see {@link PanelTabNode} in #region 🩻️Ribbon Components).

export interface TreePanelConfig {
  sections: TreeDataSection[];
  dragAndDropController?: TreeDragAndDropController;
  selectionMode?: TreeSelectionMode;
  selectedIds?: string[];
  defaultSelectedIds?: string[];
  onSelectionChange?: (selectedIds: string[], items: TreeDataItem[]) => void;
  highlightedIds?: readonly string[];
  emptyState?: React.ReactNode;
  indentMultiplier?: number;
  className?: string;
  /** ↕️ Override section-reorder grips; defaults to enabled when {@link sections} has more than one entry. */
  sortableSections?: boolean;
  onSectionsReorder?: (orderedIds: readonly string[]) => void;
}

export interface TreePanelDefinition {
  resolveTree(): TreePanelConfig;
  /** 📡️ Present on a LIVE source ({@link liveTreePanelDefinition}): its content changes on its own, `resolveTree`
   * answers one object per change, and the pane re-reads it on every notification. */
  subscribe?(listener: () => void): () => void;
}

export type TreePanelSource = TreePanelConfig | TreePanelDefinition;
