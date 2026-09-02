// #region 🧲️Header
// 💻️ framework/ui/elements/📊️Table/component.tsx
// 2026 Ueli Saluz <ueli@semio-tech.com>
// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

// #region 🔌️Adapters
import * as React from "react";
import { closestCenter, DndContext, PointerSensor, useDraggable, useDroppable, useSensor, useSensors } from "@dnd-kit/core";
import { type UiLabel } from "../🏷️UiLabel/🟦️.tsx";
import { reactHostPort } from "../🔌️Ports/🟦️.tsx";
import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";
import { Scrollable } from "../📜️Scrollable/🟦️.tsx";
import { borderNormalBottomClass } from "../../🔨️modules/📏️border-presentation/🟦️.ts";
import { surfaceClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";
import { useLabel } from "../🏷️Label/🟦️.tsx";
import { interactiveActiveFillClass, interactiveControlTransitionClass, interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";
import { useUiDriverDragSurface, useNativeDragArm } from "../🚗️UiDriver/🟦️.tsx";
import { DragHandle } from "../🧱️DragHandle/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 🛎️Table
// Sortable, hierarchical data table with drag-drop support.
// Consumers MUST provide columns and data arrays.

/** @emoji 📊️ Private interactive table-row presentation. */
const tableRowInteractiveClass = cn("text-element", interactiveControlTransitionClass, interactiveHoverClass);

/** @emoji 📊️ Private selected table-row presentation. */
const tableRowSelectedClass = interactiveActiveFillClass;

/**
 * Union type for ascending or descending sort order.
 **/
export type SortDirection = "asc" | "desc";

/**
 * Configuration interface for a table column definition.
 **/
export interface TableColumn<T = unknown> {
  id: string;
  header: React.ReactNode;
  accessor: (row: T) => React.ReactNode;
  width?: string;
  className?: string;
  headerClassName?: string;
  sortable?: boolean;
  visible?: boolean | ((data: T[]) => boolean);
}

/**
 * Interface for hierarchical row data with parent/child relations.
 **/
export interface HierarchicalRowData {
  readonly id: string;
  readonly level?: number;
  readonly parentId?: string | null;
  readonly hasChildren?: boolean;
  readonly isExpanded?: boolean;
}

/**
 * Configuration interface for table drag-and-drop behavior.
 **/
export interface DragDropConfig {
  enabled?: boolean;
  /** @emoji ⏱️ Delay (ms) before pointer drag activates so double-click can reach the row. */
  pointerActivationDelayMs?: number;
  /** @emoji ↔ Pointer movement tolerance (px) while waiting for {@link DragDropConfig.pointerActivationDelayMs}. */
  pointerActivationTolerancePx?: number;
  /** @emoji ↔ Immediate drag after pointer movement (px); ignored when {@link DragDropConfig.pointerActivationDelayMs} is set. */
  pointerActivationDistancePx?: number;
  onDragStart?: (rowId: string) => void;
  onDragEnd?: (event: { active: string; over: string | null }) => void;
  canDrag?: (rowId: string) => boolean;
  canDrop?: (draggedId: string, targetId: string) => boolean;
  renderDragOverlay?: (rowId: string) => React.ReactNode;
}

/**
 * Props interface for the Table component.
 **/
export interface TableProps<T = unknown> {
  columns: TableColumn<T>[];
  data: T[];
  onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowContextMenu?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowDoubleClick?: (row: T, index: number) => void;
  onRowMouseEnter?: (row: T, index: number) => void;
  onRowMouseLeave?: (row: T, index: number) => void;
  rowClassName?: (row: T, index: number) => string;
  rowKey?: (row: T, index: number) => string;
  emptyMessage?: UiLabel;
  className?: string;
  sortColumn?: string;
  sortDirection?: SortDirection;
  onSort?: (columnId: string, direction: SortDirection) => void;
  selectedRows?: Set<string> | string[];
  getRowId?: (row: T) => string;
  stickyHeader?: boolean;
  headerClassName?: string;
  rowHeight?: "compact" | "normal" | "comfortable";
  focusedItemId?: string;
  onFocusComplete?: () => void;
  renderMobileRow?: (row: T, index: number, isSelected: boolean, onClick: (e: React.MouseEvent) => void, onDoubleClick: () => void) => React.ReactNode;
  isMobile?: boolean;
  hierarchical?: boolean;
  onToggleRow?: (rowId: string) => void;
  renderDocumentControls?: (row: T & HierarchicalRowData) => React.ReactNode;
  dragDrop?: DragDropConfig;
  /** @emoji 🖱️ Native (cross-window) HTML5 drag attributes for a row — for `declarativeTreeDragController`-style dataTransfer drags; independent of {@link TableProps.dragDrop}'s dnd-kit reordering. */
  rowDragProps?: (row: T, index: number) => React.HTMLAttributes<HTMLTableRowElement>;
  wrapperComponent?: React.ComponentType<{ children: React.ReactNode }>;
}

interface TableDraggableRowProps<T> {
  row: T;
  rowId: string;
  index: number;
  isSelected: boolean;
  customRowClassName: string;
  activeId: string | null;
  rowHeightClass: string;
  visibleColumns: TableColumn<T>[];
  dragDrop?: DragDropConfig;
  onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowContextMenu?: (row: T, index: number, event: React.MouseEvent) => void;
  onRowDoubleClick?: (row: T, index: number) => void;
  onRowMouseEnter?: (row: T, index: number) => void;
  onRowMouseLeave?: (row: T, index: number) => void;
}

/** @emoji 🖱 Native HTML5 table-row drag that honors the UI driver (handle vs surface). */
function TableHtml5DragRow<T>({
  row,
  rowId,
  index,
  isSelected,
  customRowClassName,
  isDragging,
  rowHeightClass,
  visibleColumns,
  rowDragProps,
  onRowClick,
  onRowContextMenu,
  onRowDoubleClick,
  onRowMouseEnter,
  onRowMouseLeave,
}: {
  readonly row: T;
  readonly rowId: string;
  readonly index: number;
  readonly isSelected: boolean;
  readonly customRowClassName: string;
  readonly isDragging: boolean;
  readonly rowHeightClass: string;
  readonly visibleColumns: TableColumn<T>[];
  readonly rowDragProps?: (row: T, index: number) => React.HTMLAttributes<HTMLTableRowElement>;
  readonly onRowClick?: (row: T, index: number, event: React.MouseEvent) => void;
  readonly onRowContextMenu?: (row: T, index: number, event: React.MouseEvent) => void;
  readonly onRowDoubleClick?: (row: T, index: number) => void;
  readonly onRowMouseEnter?: (row: T, index: number) => void;
  readonly onRowMouseLeave?: (row: T, index: number) => void;
}) {
  const driverSurfaceDrag = useUiDriverDragSurface();
  const { armed, arm } = useNativeDragArm();
  const dragProps = rowDragProps?.(row, index) ?? {};
  const wantsDrag = Boolean(dragProps.draggable || dragProps.onDragStart);
  const effectiveDraggable = wantsDrag && (driverSurfaceDrag || armed);
  const baseRowClassName = cn(
    borderNormalBottomClass,
    rowHeightClass,
    tableRowInteractiveClass,
    isSelected && tableRowSelectedClass,
    wantsDrag && driverSurfaceDrag && "cursor-grab active:cursor-grabbing",
    isDragging && "opacity-50",
    onRowClick && "cursor-selectable",
    customRowClassName,
  );
  const { draggable: _ignoredDraggable, className: dragClassName, ...restDragProps } = dragProps;
  return (
    <tr
      className={cn(baseRowClassName, dragClassName)}
      draggable={effectiveDraggable || undefined}
      {...restDragProps}
      onClick={(e) => {
        if (e.detail >= 2) {
          onRowDoubleClick?.(row, index);
          return;
        }
        onRowClick?.(row, index, e);
      }}
      onContextMenu={(e) => {
        onRowContextMenu?.(row, index, e);
      }}
      onMouseEnter={() => onRowMouseEnter?.(row, index)}
      onMouseLeave={() => onRowMouseLeave?.(row, index)}
      role={onRowClick ? "button" : undefined}
      tabIndex={onRowClick ? 0 : undefined}
      data-row-id={rowId}
    >
      {visibleColumns.map((column, columnIndex) => (
        <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
          <div className="flex items-center h-full min-w-0 gap-1">
            {wantsDrag && !driverSurfaceDrag && columnIndex === 0 ? (
              <DragHandle labelId="ui.tree.drag.transfer" iconKind="move" onPointerDown={arm} onClick={(event) => event.stopPropagation()} />
            ) : null}
            {column.accessor(row)}
          </div>
        </td>
      ))}
    </tr>
  );
}

function TableDraggableRow<T>({ row, rowId, index, isSelected, customRowClassName, activeId, rowHeightClass, visibleColumns, dragDrop, onRowClick, onRowContextMenu, onRowDoubleClick, onRowMouseEnter, onRowMouseLeave }: TableDraggableRowProps<T>) {
  const driverSurfaceDrag = useUiDriverDragSurface();
  const canDragRow = !dragDrop?.canDrag || dragDrop.canDrag(rowId);
  const {
    attributes,
    listeners,
    setNodeRef: setDraggableRef,
    transform,
    isDragging: isDraggingHook,
  } = useDraggable({
    id: rowId,
    disabled: !canDragRow,
    data: { row },
  });
  const { setNodeRef: setDroppableRef, isOver } = useDroppable({
    id: rowId,
    data: { row },
  });
  const style = transform ? { transform: `translate3d(${transform.x}px, ${transform.y}px, 0)` } : undefined;
  const combinedRef = (node: HTMLElement | null) => {
    setDraggableRef(node);
    setDroppableRef(node);
  };
  const baseRowClassName = cn(borderNormalBottomClass, rowHeightClass, tableRowInteractiveClass, isSelected && tableRowSelectedClass, isOver && !isSelected && "bg-hover-interactive-fill text-emphasized ring-2 ring-active");
  const isDragging = activeId === rowId || isDraggingHook;
  return (
    <tr
      ref={combinedRef}
      style={style}
      className={`${baseRowClassName} ${customRowClassName} ${isDragging ? "opacity-50" : ""} ${onRowClick ? "cursor-selectable" : ""}`}
      {...(canDragRow && driverSurfaceDrag ? { ...attributes, ...listeners } : canDragRow ? { ...attributes } : {})}
      onClick={(e) => {
        if (e.detail >= 2) {
          onRowDoubleClick?.(row, index);
          return;
        }
        onRowClick?.(row, index, e);
      }}
      onContextMenu={(e) => {
        onRowContextMenu?.(row, index, e);
      }}
      onMouseEnter={() => onRowMouseEnter?.(row, index)}
      onMouseLeave={() => onRowMouseLeave?.(row, index)}
      role={onRowClick ? "button" : undefined}
      tabIndex={onRowClick ? 0 : undefined}
      data-row-id={rowId}
    >
      {visibleColumns.map((column, columnIndex) => (
        <td key={column.id} className={`${rowHeightClass} px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
          <div className="flex items-center h-full min-w-0 gap-1">
            {canDragRow && !driverSurfaceDrag && columnIndex === 0 ? (
              <DragHandle labelId="ui.tree.drag.sort" attributes={attributes} listeners={listeners} onClick={(event) => event.stopPropagation()} />
            ) : null}
            {column.accessor(row)}
          </div>
        </td>
      ))}
    </tr>
  );
}

/**
 * Table holds the data fields for a Table record.
 **/
const Table = <T,>({
  columns,
  data,
  onRowClick,
  onRowContextMenu,
  onRowDoubleClick,
  onRowMouseEnter,
  onRowMouseLeave,
  rowClassName,
  rowKey,
  emptyMessage,
  className = "",
  sortColumn,
  sortDirection,
  onSort,
  selectedRows,
  getRowId,
  stickyHeader = true,
  headerClassName = "",
  rowHeight = "normal",
  focusedItemId,
  onFocusComplete,
  renderMobileRow,
  isMobile = false,
  hierarchical = false,
  onToggleRow,
  renderDocumentControls,
  dragDrop,
  rowDragProps,
  wrapperComponent: WrapperComponent,
}: TableProps<T>) => {
  const noDataLabel = useLabel("ui.common.noData");
  const resolvedEmptyMessage = emptyMessage ?? noDataLabel;
  const selectedSet = selectedRows instanceof Set ? selectedRows : new Set(selectedRows || []);
  const scrollAreaRef = reactHostPort.useRef<HTMLDivElement>(null);
  const [activeId, setActiveId] = reactHostPort.useState<string | null>(null);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint:
        dragDrop?.pointerActivationDelayMs != null
          ? {
              delay: dragDrop.pointerActivationDelayMs,
              tolerance: dragDrop.pointerActivationTolerancePx ?? 5,
            }
          : {
              distance: dragDrop?.pointerActivationDistancePx ?? 8,
            },
    }),
  );

  reactHostPort.useEffect(() => {
    if (focusedItemId && scrollAreaRef.current) {
      const rowElements = scrollAreaRef.current.querySelectorAll(isMobile ? "[data-row]" : "tbody tr");
      let focusedIndex = -1;

      data.forEach((row, index) => {
        const rowId = getRowId ? getRowId(row) : rowKey ? rowKey(row, index) : index.toString();
        if (rowId === focusedItemId) {
          focusedIndex = index;
        }
      });

      if (focusedIndex >= 0 && rowElements[focusedIndex]) {
        rowElements[focusedIndex].scrollIntoView({ behavior: "smooth", block: "center" });
        if (onFocusComplete) {
          setTimeout(() => onFocusComplete(), 600);
        }
      }
    }
  }, [focusedItemId, data, getRowId, rowKey, onFocusComplete, isMobile]);

  const rowHeightClass = {
    compact: "h-medium",
    normal: "h-medium",
    comfortable: "h-medium",
  }[rowHeight];

  const visibleColumns = columns.filter((col) => {
    if (col.visible === undefined) return true;
    if (typeof col.visible === "boolean") return col.visible;
    return col.visible(data);
  });

  const handleDragStart = (event: any) => {
    const id = event.active.id;
    setActiveId(id);
    dragDrop?.onDragStart?.(id);
  };

  const handleDragEnd = (event: any) => {
    const { active, over } = event;
    setActiveId(null);
    if (dragDrop?.onDragEnd) {
      dragDrop.onDragEnd({ active: active.id, over: over?.id || null });
    }
  };

  const renderTableContent = () => {
    if (isMobile && renderMobileRow) {
      return (
        <Scrollable ref={scrollAreaRef} className={`h-full w-full ${className}`}>
          <div className="flex flex-col">
            {data.length === 0 ? (
              <div className="p-small text-center text-muted-foreground">{resolvedEmptyMessage}</div>
            ) : (
              data.map((row, index) => {
                const key = rowKey ? rowKey(row, index) : index.toString();
                const rowId = getRowId ? getRowId(row) : key;
                const isSelected = selectedSet.has(rowId);
                return (
                  <div key={key} data-row onMouseEnter={() => onRowMouseEnter?.(row, index)} onMouseLeave={() => onRowMouseLeave?.(row, index)}>
                    {renderMobileRow(
                      row,
                      index,
                      isSelected,
                      (e) => onRowClick?.(row, index, e),
                      () => onRowDoubleClick?.(row, index),
                    )}
                  </div>
                );
              })
            )}
          </div>
        </Scrollable>
      );
    }

    return (
      <Scrollable ref={scrollAreaRef} className={`h-full w-full ${className}`}>
        <table className="w-full border-collapse">
          <thead className={cn(surfaceClass, borderNormalBottomClass, stickyHeader && "sticky top-0 z-panel", headerClassName)}>
            <tr className="h-large">
              {visibleColumns.map((column) => (
                <th key={column.id} className={`text-start p-single font-medium h-large text-element ${column.headerClassName || column.className || ""}`} style={{ width: column.width }}>
                  {column.header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {data.length === 0 ? (
              <tr>
                <td colSpan={visibleColumns.length} className="p-small text-center text-muted-foreground">
                  {resolvedEmptyMessage}
                </td>
              </tr>
            ) : (
              data.map((row, index) => {
                const key = rowKey ? rowKey(row, index) : index.toString();
                const rowId = getRowId ? getRowId(row) : key;
                const isSelected = selectedSet.has(rowId);
                const customRowClassName = rowClassName ? rowClassName(row, index) : "";

                if (dragDrop?.enabled) {
                  return (
                    <TableDraggableRow
                      key={key}
                      row={row}
                      rowId={rowId}
                      index={index}
                      isSelected={isSelected}
                      customRowClassName={customRowClassName}
                      activeId={activeId}
                      rowHeightClass={rowHeightClass}
                      visibleColumns={visibleColumns}
                      dragDrop={dragDrop}
                      onRowClick={onRowClick}
                      onRowContextMenu={onRowContextMenu}
                      onRowDoubleClick={onRowDoubleClick}
                      onRowMouseEnter={onRowMouseEnter}
                      onRowMouseLeave={onRowMouseLeave}
                    />
                  );
                }

                const isDragging = activeId === rowId;

                return (
                  <TableHtml5DragRow
                    key={key}
                    row={row}
                    rowId={rowId}
                    index={index}
                    isSelected={isSelected}
                    customRowClassName={customRowClassName}
                    isDragging={isDragging}
                    rowHeightClass={rowHeightClass}
                    visibleColumns={visibleColumns}
                    rowDragProps={rowDragProps}
                    onRowClick={onRowClick}
                    onRowContextMenu={onRowContextMenu}
                    onRowDoubleClick={onRowDoubleClick}
                    onRowMouseEnter={onRowMouseEnter}
                    onRowMouseLeave={onRowMouseLeave}
                  />
                );
              })
            )}
          </tbody>
        </table>
      </Scrollable>
    );
  };

  const content = renderTableContent();

  if (dragDrop?.enabled) {
    return (
      <DndContext sensors={sensors} collisionDetection={closestCenter} onDragStart={handleDragStart} onDragEnd={handleDragEnd}>
        {WrapperComponent ? <WrapperComponent>{content}</WrapperComponent> : content}
      </DndContext>
    );
  }

  return WrapperComponent ? <WrapperComponent>{content}</WrapperComponent> : content;
};

export { Table };

/**
 * Props interface for the TableSkeleton component.
 **/
export interface TableSkeletonProps {
  columns: TableColumn[];
  rowCount?: number;
  className?: string;
}

/**
 * Skeleton loading placeholder for a table.
 **/
export const TableSkeleton: React.FC<TableSkeletonProps> = ({ columns, rowCount = 5, className = "" }) => (
  <Scrollable className={`h-full w-full ${className}`}>
    <table className="w-full border-collapse">
      <thead className={cn(surfaceClass, "sticky top-0 z-panel", borderNormalBottomClass)}>
        <tr className="h-large">
          {columns.map((column) => (
            <th key={column.id} className={`text-start p-single text-sm font-medium h-large ${column.className || ""}`} style={{ width: column.width }}>
              {column.header}
            </th>
          ))}
        </tr>
      </thead>
      <tbody>
        {Array.from({ length: rowCount }).map((_, index) => (
          <tr key={index} className={cn(borderNormalBottomClass, "h-medium")}>
            {columns.map((column) => (
              <td key={column.id} className={`h-medium px-single py-0 align-middle text-sm [&_svg:not([class*='size-'])]:size-small [&_img]:size-small ${column.className || ""}`}>
                <div className="flex items-center h-full min-w-0">
                  <div className="h-small bg-muted-foreground/20 rounded animate-pulse w-full" />
                </div>
              </td>
            ))}
          </tr>
        ))}
      </tbody>
    </table>
  </Scrollable>
);

// #endregion 🛎️Table
