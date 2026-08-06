// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/Table/component.tsx
/** @emoji 📊️ `Table` — the tabular data scene host: column/row/selection/sort parsing, stepper and
 * row-action-button cell renderers, row drag source wiring, row drop targets, and the per-row
 * context menu (program-supplied items merged with row-scoped button-placement actions). */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { useCallback, useContext, useMemo, useState } from "react";
import { Button, ContextMenuController, Icon, Input, Table, useLabel, useShellScopeOptional, type ContextMenuItem, type IconName, type TableColumn } from "@semio-tech/ui-react";
import { type ActionDescriptor, type ComponentSceneHostProps } from "@semio-tech/framework-core";
import { openSurfaceContextMenu, parseSceneJsonField, useShellContextMenuFallback } from "../Interpreter/🟦️component.tsx";
import { WindowInstanceIdContext } from "../World3dHost/🟦️component.tsx";
import { useMapContextMenuSpecs } from "../ShellHost/🟦️component.tsx";
// #endregion 🔌️Adapters

//#region 🔖️TableHost
//#region TableHost
//#region Types
type TableColumnRecord = { readonly id: string; readonly label: string; readonly sortable?: boolean };
type TableCellButton = { readonly iconId: IconName; readonly label?: string; readonly action: ActionDescriptor; readonly placement?: "row" | "menu" };
type TableCellRecord =
  | { readonly kind: "text"; readonly value: string }
  | { readonly kind: "number"; readonly value: number }
  | { readonly kind: "stepper"; readonly value: number; readonly min: number; readonly max: number; readonly step: number; readonly action: ActionDescriptor }
  | { readonly kind: "buttons"; readonly buttons: readonly TableCellButton[] };
type TableRowRecord = Record<string, unknown> & { readonly id?: string; readonly _drag?: Record<string, unknown> };
//#endregion Types

//#region Helpers
function isTableCellRecord(value: unknown): value is TableCellRecord {
  return typeof value === "object" && value !== null && "kind" in value;
}

function dispatchCellAction(onAction: (action: ActionDescriptor) => void, descriptor: ActionDescriptor, patch: Record<string, unknown>): void {
  onAction({
    ...descriptor,
    args: { ...(typeof descriptor.args === "object" && descriptor.args != null ? descriptor.args : {}), ...patch },
  });
}

function tableRowMenuPlacementItems(row: TableRowRecord, onAction: (action: ActionDescriptor) => void): ContextMenuItem[] {
  const items: ContextMenuItem[] = [];
  for (const value of Object.values(row)) {
    if (!isTableCellRecord(value) || value.kind !== "buttons") continue;
    for (const [index, button] of value.buttons.entries()) {
      if ((button.placement ?? "row") !== "menu") continue;
      items.push({
        id: `table-row-action-${button.iconId}-${index}`,
        label: button.label,
        icon: button.iconId,
        onSelect: () => dispatchCellAction(onAction, button.action, {}),
      });
    }
  }
  return items;
}

function renderTableCell(cell: TableCellRecord, onAction: (action: ActionDescriptor) => void): React.ReactNode {
  switch (cell.kind) {
    case "text":
      return cell.value;
    case "number":
      return String(cell.value);
    case "stepper":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, cell.action, { delta: -cell.step })} disabled={cell.value <= cell.min} type="button" variant="outline">
            −
          </Button>
          <Input className="h-medium w-14 min-w-0 text-center font-mono text-xs" readOnly value={String(cell.value)} />
          <Button className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, cell.action, { delta: cell.step })} disabled={cell.value >= cell.max} type="button" variant="outline">
            +
          </Button>
        </div>
      );
    case "buttons":
      return (
        <div className="flex min-w-0 items-center gap-1" onClick={(event) => event.stopPropagation()}>
          {cell.buttons
            .filter((button) => (button.placement ?? "row") === "row")
            .map((button, index) => (
            <Button key={index} className="h-medium shrink-0 px-2" onClick={() => dispatchCellAction(onAction, button.action, {})} title={button.label} type="button" variant="outline">
              <Icon icon={button.iconId} size="small" />
            </Button>
          ))}
        </div>
      );
  }
}
//#endregion Helpers

//#region Component
export function TableHost({ node, onAction, requestContextMenu }: ComponentSceneHostProps) {
  const scene = node.table;
  // 🐚️ Optional — this host is also unit-tested standalone, outside any `ShellScopeProvider`.
  const shellScope = useShellScopeOptional();
  const windowInstanceId = useContext(WindowInstanceIdContext);
  const emptySceneLabel = useLabel("ui.host.emptyScene");
  const contextMenuTitleLabel = useLabel("ui.surfaceContextMenu.row");
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly items: readonly ContextMenuItem[] } | null>(null);
  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );
  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();
  const columns = useMemo(() => {
    if (!scene) return [] as TableColumnRecord[];
    try {
      return parseSceneJsonField<TableColumnRecord[]>(scene.columnsJson);
    } catch {
      return [];
    }
  }, [scene]);
  const rows = useMemo(() => {
    if (!scene) return [] as TableRowRecord[];
    try {
      return parseSceneJsonField<TableRowRecord[]>(scene.rowsJson);
    } catch {
      return [];
    }
  }, [scene]);
  const selectedRows = useMemo(() => {
    if (!scene?.selectionJson) return undefined;
    try {
      const parsed = parseSceneJsonField<{ readonly selectedIds?: readonly string[] }>(scene.selectionJson);
      return new Set(parsed.selectedIds ?? []);
    } catch {
      return undefined;
    }
  }, [scene]);
  const sort = useMemo(() => {
    if (!scene?.sortJson) return undefined;
    try {
      return parseSceneJsonField<{ readonly columnId?: string; readonly direction?: "asc" | "desc" }>(scene.sortJson);
    } catch {
      return undefined;
    }
  }, [scene]);
  const tableColumns = useMemo<TableColumn<TableRowRecord>[]>(
    () =>
      columns.map((column) => ({
        id: column.id,
        header: column.label,
        sortable: column.sortable,
        accessor: (row) => {
          const value = row[column.id];
          if (isTableCellRecord(value)) return renderTableCell(value, onAction);
          return String(value ?? "");
        },
      })),
    [columns, onAction],
  );

  if (!scene) return <div className="semio-table-empty">{emptySceneLabel}</div>;

  const rowDragMime = scene.rowDragMime;
  const dropAction = scene.dropAction;

  return (
    <div
      className="semio-table-host h-full min-h-0 w-full"
      data-surface-id={node.surfaceId}
      onDragOver={
        dropAction
          ? (event) => {
              event.preventDefault();
              event.dataTransfer.dropEffect = "copy";
            }
          : undefined
      }
      onDrop={
        dropAction
          ? (event) => {
              event.preventDefault();
              const encoded = [...event.dataTransfer.types].filter((kind) => kind.startsWith("application/x-semio-")).map((kind) => event.dataTransfer.getData(kind))[0];
              if (!encoded?.trim()) return;
              try {
                dispatchCellAction(onAction, dropAction, JSON.parse(encoded) as Record<string, unknown>);
              } catch {
                return;
              }
            }
          : undefined
      }
    >
      <Table
        className="h-full w-full"
        columns={tableColumns}
        data={rows}
        getRowId={(row, index) => String(row.id ?? row.pluginId ?? index)}
        selectedRows={selectedRows}
        sortColumn={sort?.columnId}
        sortDirection={sort?.direction}
        onSort={(columnId, direction) =>
          onAction({
            controllerId: node.controllerId,
            action: "sortTable",
            args: { surfaceId: node.surfaceId, columnId, direction },
          })
        }
        rowDragProps={
          rowDragMime
            ? (row) =>
                row._drag
                  ? {
                      draggable: true,
                      onDragStart: (event) => {
                        event.dataTransfer.setData(rowDragMime, JSON.stringify(row._drag));
                        event.dataTransfer.effectAllowed = "copy";
                      },
                    }
                  : {}
            : undefined
        }
        onRowClick={(row) =>
          onAction({
            controllerId: node.controllerId,
            action: "selectRow",
            args: { surfaceId: node.surfaceId, row },
          })
        }
        onRowContextMenu={(row, index, event) => {
          if (!requestContextMenu) return;
          event.preventDefault();
          event.stopPropagation();
          const rowId = String(row.id ?? row.pluginId ?? index);
          void (async () => {
            const items = await openSurfaceContextMenu(
              requestContextMenu,
              {
                menu: { id: "table" },
                surface: {
                  surfaceId: node.surfaceId,
                  kind: "table",
                  hits: [{ domain: "row", id: rowId }],
                  selection: selectedRows && selectedRows.size > 0 ? [{ domain: "row", ids: [...selectedRows] }] : [],
                },
                windowInstanceId: windowInstanceId ?? undefined,
                point: { x: event.clientX, y: event.clientY },
              },
              mapContextMenu,
              shellContextMenuFallback,
            );
            const menuActions = tableRowMenuPlacementItems(row, onAction);
            setContextMenu({
              x: event.clientX,
              y: event.clientY,
              items: menuActions.length ? [...items, ...(items.length ? [{ id: "table-row-action-separator", separator: true } as ContextMenuItem] : []), ...menuActions] : items,
            });
          })();
        }}
      />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion Component
//#endregion TableHost
//#endregion 🔖️TableHost
