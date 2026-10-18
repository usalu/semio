import * as React from "react";
import { reactHostPort } from "../../🔌️Ports/🟦️.tsx";
import { CATALOGUE_DRAG_MIME, type TreeDragAndDropController, catalogueTreeDragController } from "../🧲️drag-drop/🟦️.tsx";
import type { TreeDataSection } from "../📋️data/🟦️.ts";
import { Tree } from "../🟦️.tsx";

export interface CatalogueItem {
  readonly id: string;
  readonly label: React.ReactNode;
  readonly icon?: React.ReactNode;
  readonly payload: Record<string, unknown>;
}

export interface CatalogueProps {
  readonly title: React.ReactNode;
  readonly items: readonly CatalogueItem[];
  readonly mime?: string;
  readonly className?: string;
  readonly dragController?: TreeDragAndDropController;
}

/** 🗂️ Draggable catalogue palette rendered as a tree section. */
export const Catalogue: React.FC<CatalogueProps> = ({ title, items, mime = CATALOGUE_DRAG_MIME, className, dragController }) => {
  const sections = reactHostPort.useMemo<TreeDataSection[]>(
    () => [
      {
        id: "catalogue",
        label: title,
        items: items.map((item) => ({
          id: item.id,
          label: item.label,
          icon: item.icon,
          draggable: true,
          dragData: { [mime]: JSON.stringify(item.payload) },
        })),
      },
    ],
    [items, mime, title],
  );
  return <Tree className={className} sections={sections} dragAndDropController={dragController ?? catalogueTreeDragController(mime)} />;
};
